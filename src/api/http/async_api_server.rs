use mlua::{Lua, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use warp::Filter;

pub fn register(lua: &Lua) -> Result<mlua::Table> {
    // Arc-Mutex für Zugriff aus Lua auf aktive Server
    let server_controls: Arc<Mutex<HashMap<u16, oneshot::Sender<()>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let table = lua.create_table()?;

    let start_api_server = {
        let server_controls = Arc::clone(&server_controls);
        lua.create_function(move |_, port: u16| {
            let (shutdown_tx, shutdown_rx) = oneshot::channel();
            server_controls.lock().unwrap().insert(port, shutdown_tx);

            tokio::spawn(async move {
                // Beispielroute /api/hello
                let hello = warp::path!("api" / "hello").and(warp::get()).map(|| {
                    warp::reply::json(&serde_json::json!({
                        "message": "Hello from API"
                    }))
                });

                let routes = hello.with(warp::log("api"));

                println!("API server running on http://0.0.0.0:{}/api/hello", port);

                let (_addr, server) = warp::serve(routes).bind_with_graceful_shutdown(
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

    table.set("start_api_server", start_api_server)?;

    Ok(table)
}
