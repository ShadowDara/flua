use mlua::{Lua, Result};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use warp::Filter;

pub fn register(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;

    // Shared mutable state for additional API handlers (future use)
    // For now unused, aber vorbereitet für spätere Erweiterung
    let api_handlers: Arc<Mutex<Vec<Box<dyn Fn() + Send + Sync>>>> =
        Arc::new(Mutex::new(Vec::new()));

    // Funktion zum Starten eines Static File Servers
    // Argument: Verzeichnis, Port
    let start_static_server = {
        let api_handlers = Arc::clone(&api_handlers);
        lua.create_function(move |_, (directory, port): (String, u16)| {
            let dir = directory.clone();

            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
                rt.block_on(async move {
                    let static_files = warp::fs::dir(dir);

                    // Middleware fürs Logging aller Requests
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
                        "Static server running on 0.0.0.0:{} serving directory '{}'",
                        port, directory
                    );

                    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
                });
            });

            // Jetzt blockieren wir den aktuellen Thread, damit das Programm nicht direkt exitet:
            // z.B. auf Tastendruck warten (du kannst hier auch etwas Eleganteres machen)
            println!("Press Enter to stop the server...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            Ok(())
        })?
    };

    table.set("start_static_server", start_static_server)?;

    Ok(table)
}
