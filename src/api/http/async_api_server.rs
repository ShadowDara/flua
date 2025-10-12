use mlua::{Function, Lua, Result, Table, Value as LuaValue};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, oneshot};
use warp::filters::BoxedFilter;
use warp::http::{Response, StatusCode};
use warp::hyper::Body;
use warp::{Filter, Rejection, Reply};

use crate::utils::json_utils::lua_to_json;

enum LuaRequest {
    Call {
        route: String,
        resp_tx: oneshot::Sender<std::result::Result<JsonValue, String>>,
    },
}

pub fn register(lua: &Lua) -> Result<Table> {
    let server_controls: Arc<Mutex<HashMap<u16, oneshot::Sender<()>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let table = lua.create_table()?;
    let lua = Arc::new(lua.clone()); // Arc<Lua> für sichere Multi-Thread Nutzung

    // --- Start API Server ---
    let start_api_server = {
        let server_controls = Arc::clone(&server_controls);
        let lua2 = lua.clone();

        lua2.create_function(move |_, (port, handlers_table): (u16, Option<Table>)| {
            let lua = Arc::clone(&lua); // lokale clone für Closure
            let (shutdown_tx, shutdown_rx) = oneshot::channel();
            server_controls.lock().unwrap().insert(port, shutdown_tx);

            // Lua-Routen sammeln
            let mut handlers: HashMap<String, Function> = HashMap::new();
            if let Some(table) = handlers_table {
                for pair in table.pairs::<String, LuaValue>() {
                    let (route, value) = pair?;
                    let func: Function = match value {
                        LuaValue::Function(f) => f,
                        LuaValue::String(s_val) => {
                            let src: &str = &s_val.to_str()?; // Direkt zu &str
                            if src.trim_start().starts_with("function") {
                                match lua.load(src).eval()? {
                                    // jetzt funktioniert load
                                    LuaValue::Function(f) => f,
                                    other => {
                                        return Err(mlua::Error::external(format!(
                                            "Route '{}' must return a function; got {:?}",
                                            route, other
                                        )));
                                    }
                                }
                            } else {
                                let wrapped = format!("return ({});", src);
                                let val: LuaValue = lua.load(&wrapped).eval()?;
                                lua.create_function(move |_, ()| Ok(val.clone()))?
                            }
                        }

                        LuaValue::Nil => lua.create_function(|_, ()| Ok(LuaValue::Nil))?,
                        other => {
                            return Err(mlua::Error::external(format!(
                                "Route '{}' invalid type: {:?}",
                                route, other
                            )));
                        }
                    };
                    handlers.insert(route, func);
                }
            }

            let (lua_tx, mut lua_rx) = mpsc::unbounded_channel::<LuaRequest>();
            let handlers_clone = handlers.clone();

            // Dispatcher auf Tokio-Task
            let local_set = tokio::task::LocalSet::new();
            local_set.spawn_local(async move {
                while let Some(req) = lua_rx.recv().await {
                    if let LuaRequest::Call { route, resp_tx } = req {
                        let res: std::result::Result<JsonValue, String> = (|| {
                            if let Some(func) = handlers_clone.get(&route) {
                                let val: LuaValue = func.call(())?;
                                let json = lua_to_json(&val)?;
                                Ok(json)
                            } else {
                                Ok(serde_json::json!({"error": "route not found"}))
                            }
                        })(
                        )
                        .map_err(|e: mlua::Error| format!("{:?}", e));
                        let _ = resp_tx.send(res);
                    }
                }
            });

            // Warp Filter pro Route
            let mut filters: Vec<BoxedFilter<(Response<Body>,)>> = Vec::new();
            let handlers_arc = Arc::new(handlers);

            for route_name in handlers_arc.keys() {
                let lua_tx_clone = lua_tx.clone();
                let route_name_clone = route_name.clone();

                let route = warp::path("api")
                    .and(warp::path(route_name_clone.clone()))
                    .and(warp::path::end())
                    .and(warp::get())
                    .and_then(move || {
                        let lua_tx_clone = lua_tx_clone.clone();
                        let route_name_clone = route_name_clone.clone();

                        async move {
                            let (resp_tx, resp_rx) = oneshot::channel();
                            lua_tx_clone
                                .send(LuaRequest::Call {
                                    route: route_name_clone,
                                    resp_tx,
                                })
                                .map_err(|_| warp::reject())?;

                            match resp_rx.await {
                                Ok(Ok(json)) => Ok::<Response<Body>, Rejection>(
                                    warp::reply::json(&json).into_response(),
                                ),
                                Ok(Err(err_msg)) => Ok(warp::reply::with_status(
                                    err_msg,
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                )
                                .into_response()),
                                Err(_) => Ok(warp::reply::with_status(
                                    "Lua task dropped",
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                )
                                .into_response()),
                            }
                        }
                    })
                    .boxed();

                filters.push(route);
            }

            // Default 404
            let not_found: BoxedFilter<(Response<Body>,)> = warp::any()
                .map(|| {
                    warp::reply::with_status("Not found", StatusCode::NOT_FOUND).into_response()
                })
                .boxed();

            let combined: BoxedFilter<(Response<Body>,)> = filters
                .into_iter()
                .reduce(|a, b| a.or(b).unify().boxed())
                .unwrap_or(not_found);

            println!(
                "Lua API server running on http://0.0.0.0:{}/api/<endpoint>",
                port
            );

            tokio::spawn(async move {
                warp::serve(combined)
                    .bind_with_graceful_shutdown(([0, 0, 0, 0], port), async move {
                        shutdown_rx.await.ok();
                        println!("API server on port {} stopped", port);
                    })
                    .1
                    .await;
            });

            Ok(())
        })?
    };

    // --- Stop API Server ---
    // let stop_api_server = {
    //     let server_controls = Arc::clone(&server_controls);
    //     lua.create_function(move |_, port: Option<u16>| {
    //         if let Some(port) = port {
    //             let mut controls = server_controls.lock().unwrap();
    //             if let Some(sender) = controls.remove(&port) {
    //                 let _ = sender.send(());
    //                 println!("Server on port {} stopped", port);
    //             } else {
    //                 println!("No server on port {}", port);
    //             }
    //         } else {
    //             // Stoppe alle Server
    //             let mut controls = server_controls.lock().unwrap();
    //             for (port, sender) in controls.drain() {
    //                 let _ = sender.send(());
    //                 println!("Server on port {} stopped", port);
    //             }
    //         }
    //         Ok(())
    //     })?
    // };

    table.set("start_api_server", start_api_server)?;
    // table.set("stop_api_server", stop_api_server)?;
    Ok(table)
}
