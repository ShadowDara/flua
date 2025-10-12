use either::Either;
use mlua::{Function, Lua, LuaSerdeExt, Result, Table, Value};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use warp::any;
use warp::filters::BoxedFilter;
use warp::http::Response;
use warp::http::StatusCode;
use warp::hyper::Body;
use warp::{Filter, Rejection, Reply};

use crate::utils::json_utils::lua_to_json;

pub fn register(lua: &Lua) -> Result<Table> {
    let server_controls: Arc<Mutex<HashMap<u16, oneshot::Sender<()>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let table = lua.create_table()?;
    let lua = lua.clone();

    // Start API server
    let start_api_server = {
        let server_controls = Arc::clone(&server_controls);

        lua.create_function(move |_, (port, handlers): (u16, Table)| {
            let (shutdown_tx, shutdown_rx) = oneshot::channel();
            server_controls.lock().unwrap().insert(port, shutdown_tx);

            let lua = lua.clone();
            let mut route_map: HashMap<String, Function> = HashMap::new();
            for pair in handlers.pairs::<String, Function>() {
                let (route, func) = pair?;
                route_map.insert(route, func);
            }

            tokio::spawn(async move {
                let lua = Arc::new(lua);

                // Im Tokio-Thread

                let mut filters = Vec::new();

                for (route_name, lua_func) in route_map {
                    let lua = Arc::clone(&lua);
                    let expected = route_name.clone();
                    let func = lua_func.clone();

                    let route = warp::path!("api" / String)
                        .and(warp::path::end())
                        .and(warp::get())
                        .and_then(move |actual_path: String| {
                            let lua = Arc::clone(&lua);
                            let func = func.clone();
                            let expected = expected.clone();

                            async move {
                                if actual_path != expected {
                                    return Ok::<_, Rejection>(
                                        warp::reply::with_status(
                                            "Not found",
                                            warp::http::StatusCode::NOT_FOUND,
                                        )
                                        .into_response(),
                                    );
                                }

                                let result: std::result::Result<_, mlua::Error> = (|| {
                                    let lua_value: Value = func.call(())?;
                                    let json: JsonValue = lua_to_json(&lua_value)
                                        .expect("Failed to convert Lua value to JSON");
                                    Ok(json)
                                })(
                                );

                                match result {
                                    Ok(json) => Ok(warp::reply::json(&json).into_response()),
                                    Err(err) => {
                                        eprintln!("Lua error: {:?}", err);
                                        Ok(warp::reply::with_status(
                                            "Lua execution error",
                                            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                                        )
                                        .into_response())
                                    }
                                }
                            }
                        });

                    filters.push(route); // **kein `.boxed()` hier!**
                }

                let not_found = warp::any()
                    .map(|| warp::reply::with_status("Not found", StatusCode::NOT_FOUND))
                    .boxed();

                let filters = filters
                    .into_iter()
                    .map(|f| f.map(|_| ()).boxed())
                    .collect::<Vec<_>>();

                let combined = filters
                    .into_iter()
                    .reduce(|a, b| a.or(b))
                    .unwrap_or(not_found)
                    .boxed();

                println!(
                    "Lua API server running on http://0.0.0.0:{}/api/<endpoint>",
                    port
                );

                let (_addr, server) = warp::serve(combined.with(warp::log("lua_api")))
                    .bind_with_graceful_shutdown(([0, 0, 0, 0], port), async move {
                        shutdown_rx.await.ok();
                        println!("API server on port {} stopped", port);
                    });

                server.await;
            });

            Ok(())
        })?
    };

    // Stop API server
    let stop_api_server = {
        let server_controls = Arc::clone(&server_controls);
        lua.create_function(move |_, port: u16| {
            let mut controls = server_controls.lock().unwrap();
            if let Some(sender) = controls.remove(&port) {
                let _ = sender.send(()); // Stop signal
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
