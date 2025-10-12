use mlua::{Function, Lua, Result, Table, Value as LuaValue};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::sync::{mpsc, oneshot};
use warp::filters::BoxedFilter;
use warp::http::{Response, StatusCode};
use warp::hyper::Body;
use warp::{Filter, Rejection, Reply};

use crate::utils::json_utils::lua_to_json;

// Nachrichtentyp für Dispatcher (Warp -> Lua-Thread)
enum LuaRequest {
    Call {
        route: String,
        resp_tx: oneshot::Sender<std::result::Result<JsonValue, String>>,
    },
}

pub fn register(lua: &Lua) -> Result<Table> {
    // shutdown control per port
    let server_controls: Arc<Mutex<HashMap<u16, oneshot::Sender<()>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let table = lua.create_table()?;
    // We won't move the provided `lua` into the worker thread; the worker will create its own Lua VM.
    let _lua_ref = lua.clone();

    // start_api_server(port, handlers_opt)
    let start_api_server = {
        let server_controls = Arc::clone(&server_controls);

        lua.create_function(move |_, (port, handlers_table): (u16, Option<Table>)| {
            // create shutdown channel for the warp server
            let (shutdown_tx, shutdown_rx) = oneshot::channel();
            server_controls.lock().unwrap().insert(port, shutdown_tx);

            // Collect handler sources (strings) from Lua table, if provided.
            // handlers_map: route -> lua source string
            let mut handler_sources: HashMap<String, String> = HashMap::new();
            if let Some(table) = handlers_table {
                for pair in table.pairs::<String, LuaValue>() {
                    let (route, val) = pair?;
                    match val {
                        LuaValue::String(s) => {
                            handler_sources.insert(route, s.to_str()?.to_string());
                        }
                        // If the user passed an actual function value, produce a clear error
                        LuaValue::Function(_) => {
                            return Err(mlua::Error::external(format!(
                                "handler for route '{}' must be a string (source), function values are not supported in this setup",
                                route
                            )));
                        }
                        _ => {
                            return Err(mlua::Error::external(format!(
                                "handler for route '{}' must be a string containing Lua code",
                                route
                            )));
                        }
                    }
                }
            }

            // Channel from Warp (async) to Lua-thread (single-thread)
            let (lua_tx, mut lua_rx) = mpsc::unbounded_channel::<LuaRequest>();

            // Move handler_sources into worker thread
            let handler_sources_for_thread = handler_sources.clone();

            // Spawn a native thread which will own its own Lua VM and compiled functions.
            thread::spawn(move || {
                // build a single-threaded tokio runtime so we can await on the mpsc receiver
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("failed to build local runtime for lua thread");

                rt.block_on(async move {
                    // Create a Lua VM in this thread (this VM is NOT moved to other threads)
                    let lua_vm = Lua::new();

                    // Compile handler sources into Functions inside this lua_vm
                    let mut lua_handlers: HashMap<String, Function> = HashMap::new();
                    for (route, src) in handler_sources_for_thread.into_iter() {
                        // Try to interpret src:
                        // - If it looks like "function(...)" produce that function
                        // - Otherwise wrap it into "return (src)" so expression works.
                        let compile_result: std::result::Result<Function, mlua::Error> = (|| {
    if src.trim_start().starts_with("function") {
        // load chunk expecting the chunk yields a function value
        let chunk = lua_vm.load(&src);
        let v = chunk.eval()?;
        match v {
            LuaValue::Function(f) => Ok(f),
            other => Err(mlua::Error::external(format!(
                "handler chunk for route '{}' did not return a function; got: {:?}",
                route, other
            ))),
        }
    } else {
        // treat as expression -> wrap to return expression, then create a function that returns it
        // Only wrap with return if src is not empty and does not already start with return
        let wrapped = if src.trim().is_empty() {
            "return nil".to_string()
        } else if src.trim_start().starts_with("return") {
            src.to_string() // already a return statement
        } else {
            format!("return ({})", src)
        };

        let chunk = lua_vm.load(&wrapped);
        let v: LuaValue = chunk.eval()?; // evaluate the expression

        // create a function that returns this value when called
        // Note: to return a cloned value each call, we clone using mlua::Value's clone trick
        let f = lua_vm.create_function(move |_, ()| Ok(v.clone()))?;
        Ok(f)
    }
})();


                        match compile_result {
                            Ok(f) => {
                                lua_handlers.insert(route, f);
                            }
                            Err(e) => {
                                eprintln!("Failed to compile handler for route '{}': {:?}", route, e);
                            }
                        }
                    }

                    // Serve incoming requests from warp (via channel)
                    while let Some(req) = lua_rx.recv().await {
                        if let LuaRequest::Call { route, resp_tx } = req {
                            let result: std::result::Result<JsonValue, String> = (|| {
                                if let Some(func) = lua_handlers.get(&route) {
                                    let val = func.call(())?;
                                    let json = lua_to_json(&val)?;
                                    Ok(json)
                                } else {
                                    Ok(serde_json::json!({ "error": "route not found" }))
                                }
                            })()
                            .map_err(|e: mlua::Error| format!("{:?}", e));

                            let _ = resp_tx.send(result);
                        }
                    }

                    // channel closed → exit thread
                });
            });

            // Build warp filters (these run in your application's Tokio runtime)
            let mut filters: Vec<BoxedFilter<(Response<Body>,)>> = Vec::new();

            for route in handler_sources.keys() {
                let lua_tx_clone = lua_tx.clone();
                let route_clone = route.clone();

                let route_filter = warp::path("api")
                    .and(warp::path(route_clone.clone()))
                    .and(warp::path::end())
                    .and(warp::get())
                    .and_then(move || {
                        let lua_tx2 = lua_tx_clone.clone();
                        let route2 = route_clone.clone();

                        async move {
                            let (resp_tx, resp_rx) =
                                oneshot::channel::<std::result::Result<JsonValue, String>>();

                            let req = LuaRequest::Call {
                                route: route2,
                                resp_tx,
                            };

                            // send to Lua thread
                            lua_tx2.send(req).map_err(|_| warp::reject())?;

                            // wait for result
                            match resp_rx.await {
                                Ok(Ok(json)) => Ok::<Response<Body>, Rejection>(
                                    warp::reply::json(&json).into_response(),
                                ),
                                Ok(Err(err_msg)) => Ok::<Response<Body>, Rejection>(
                                    warp::reply::with_status(err_msg, StatusCode::INTERNAL_SERVER_ERROR)
                                        .into_response(),
                                ),
                                Err(_) => Ok::<Response<Body>, Rejection>(
                                    warp::reply::with_status("Lua thread dropped", StatusCode::INTERNAL_SERVER_ERROR)
                                        .into_response(),
                                ),
                            }
                        }
                    })
                    .boxed();

                filters.push(route_filter);
            }

            // 404 fallback
            let not_found: BoxedFilter<(Response<Body>,)> = warp::any()
                .map(|| warp::reply::with_status("Not found", StatusCode::NOT_FOUND).into_response())
                .boxed();

            // combine
            let combined: BoxedFilter<(Response<Body>,)> = filters
                .into_iter()
                .reduce(|a, b| a.or(b).unify().boxed())
                .unwrap_or(not_found);

            println!("Lua API server running on http://0.0.0.0:{}/api/<endpoint>", port);

            // start warp server using your app's runtime
            tokio::spawn(async move {
                let (_addr, server_future) = warp::serve(combined).bind_with_graceful_shutdown(
                    ([0u8, 0, 0, 0], port),
                    async move {
                        let _ = shutdown_rx.await;
                        println!("API server on port {} stopped", port);
                    },
                );
                server_future.await;
            });

            Ok(())
        })?
    };

    // stop fn
    let stop_api_server = {
        let server_controls = Arc::clone(&server_controls);
        lua.create_function(move |_, port: u16| {
            let mut controls = server_controls.lock().unwrap();
            if let Some(tx) = controls.remove(&port) {
                let _ = tx.send(());
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
