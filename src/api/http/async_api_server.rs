use mlua::{Function, Lua, Result, Table, Value};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, oneshot};
use warp::Rejection;
use warp::filters::BoxedFilter;
use warp::http::{Response, StatusCode};
use warp::hyper::Body;
use warp::{Filter, Reply};

use crate::utils::json_utils::lua_to_json;

// Nachrichtentyp für Dispatcher
enum LuaRequest {
    Call {
        route: String,
        resp_tx: oneshot::Sender<std::result::Result<JsonValue, String>>,
    },
}

pub fn register(lua: &Lua) -> Result<Table> {
    // Steuerelemente für Server-Shutdown
    let server_controls: Arc<Mutex<HashMap<u16, oneshot::Sender<()>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let table = lua.create_table()?;
    let lua = lua.clone();

    // --- Start server ---
    let start_api_server = {
        let server_controls = Arc::clone(&server_controls);

        lua.create_function(move |_, (port, handlers_table): (u16, Option<Table>)| {
            let (shutdown_tx, shutdown_rx) = oneshot::channel();
            server_controls.lock().unwrap().insert(port, shutdown_tx);

            // Sammele alle Lua-Routen, falls vorhanden
            let mut handlers: HashMap<String, Function> = HashMap::new();
            if let Some(table) = handlers_table {
                for pair in table.pairs::<String, Function>() {
                    let (route, func) = pair?;
                    handlers.insert(route, func);
                }
            }

            // Channel für Dispatcher
            let (lua_tx, mut lua_rx) = mpsc::unbounded_channel::<LuaRequest>();

            let handlers2 = handlers.clone();

            // Spawn Dispatcher-Task
            tokio::task::spawn_local(async move {
                let lua_routes = handlers2;

                while let Some(req) = lua_rx.recv().await {
                    match req {
                        LuaRequest::Call { route, resp_tx } => {
                            let res: std::result::Result<JsonValue, String> = (|| {
                                if let Some(func) = lua_routes.get(&route) {
                                    let val: Value = func.call(())?;
                                    let json = lua_to_json(&val)?;
                                    Ok(json)
                                } else {
                                    Ok(serde_json::json!({"error": "route not found"}))
                                }
                            })(
                            )
                            .map_err(|e: mlua::Error| format!("{:?}", e));

                            let _ = resp_tx.send(res.map_err(|e| format!("{:?}", e)));
                        }
                    }
                }
            });

            // Warp Filter pro Route
            let mut filters: Vec<BoxedFilter<(Response<Body>,)>> = Vec::new();
            let handlers = Arc::new(handlers);

            for route_name in handlers.keys() {
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
                            let req = LuaRequest::Call {
                                route: route_name_clone,
                                resp_tx,
                            };
                            lua_tx_clone.send(req).map_err(|_| warp::reject())?;

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

            // Alle Filter kombinieren
            let combined: BoxedFilter<(Response<Body>,)> = filters
                .into_iter()
                .reduce(
                    |a: BoxedFilter<(Response<Body>,)>, b: BoxedFilter<(Response<Body>,)>| {
                        a.or(b).unify().boxed()
                    },
                )
                .unwrap_or(not_found);

            println!(
                "Lua API server running on http://0.0.0.0:{}/api/<endpoint>",
                port
            );

            // Warp Server starten
            tokio::task::spawn(async move {
                let (_addr, server) = warp::serve(combined).bind_with_graceful_shutdown(
                    ([0, 0, 0, 0], port),
                    async move {
                        shutdown_rx.await.ok();
                        println!("API server on port {} stopped", port);
                    },
                );

                server.await;
            });

            Ok(())
        })?
    };

    // --- Stop server ---
    let stop_api_server = {
        let server_controls = Arc::clone(&server_controls);

        lua.create_function(move |_, port: u16| {
            let mut controls = server_controls.lock().unwrap();
            if let Some(sender) = controls.remove(&port) {
                let _ = sender.send(());
                println!("Server on port {} stopped", port);
            } else {
                println!("No server on port {}", port);
            }
            Ok(())
        })?
    };

    table.set("start_api_server", start_api_server)?;
    table.set("stop_api_server", stop_api_server)?;
    Ok(table)
}
