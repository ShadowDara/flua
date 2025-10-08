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

    let start_static_server = {
        let server_controls = Arc::clone(&server_controls);
        lua.create_function(move |_, (directory, port): (String, u16)| {
            let (shutdown_tx, shutdown_rx) = oneshot::channel();
            server_controls.lock().unwrap().insert(port, shutdown_tx);

            let dir = directory.clone();

            // Warp-Server async im Hintergrund starten
            tokio::spawn(async move {
                let static_files = warp::fs::dir(dir.clone());
                let routes = warp::any()
                    .and(static_files)
                    .with(warp::log::custom(|info| {
                        println!(
                            "Request: method={} path={} status={}",
                            info.method(),
                            info.path(),
                            info.status()
                        );
                    }));

                println!(
                    "Static server running on http://0.0.0.0:{} serving directory '{}'",
                    port, dir
                );

                let (_addr, server) = warp::serve(routes).bind_with_graceful_shutdown(
                    ([0, 0, 0, 0], port),
                    async move {
                        shutdown_rx.await.ok();
                        println!("Server on port {} stopped", port);
                    },
                );

                server.await;
            });

            Ok(())
        })?
    };

    let stop_static_server = {
        let server_controls = Arc::clone(&server_controls);
        lua.create_function(move |_, port: u16| {
            let mut controls = server_controls.lock().unwrap();
            if let Some(shutdown) = controls.remove(&port) {
                let _ = shutdown.send(()); // Signal an den Server
                println!("Server on port {} was signaled to stop.", port);
            } else {
                println!("No server running on port {}", port);
            }
            Ok(())
        })?
    };

    table.set("start_static_server", start_static_server)?;
    table.set("stop_static_server", stop_static_server)?;

    Ok(table)
}
