use mlua::{Lua, Result};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use warp::Filter;
use warp::filters::BoxedFilter;
use warp::hyper::Body;
use warp::reply::Response;

pub fn register(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;

    // Shared mutable state for additional API handlers (future use)
    // For now unused, aber vorbereitet für spätere Erweiterung
    let api_handlers: Arc<Mutex<Vec<Box<dyn Fn() + Send + Sync>>>> =
        Arc::new(Mutex::new(Vec::new()));

    // // Funktion zum Starten eines Static File Servers
    // // Argument: Verzeichnis, Port
    // let start_static_server = {
    //     let api_handlers = Arc::clone(&api_handlers);
    //     lua.create_function(move |_, (directory, port): (String, u16)| {
    //         let dir = directory.clone();

    //         std::thread::spawn(move || {
    //             let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    //             rt.block_on(async move {
    //                 let static_files = warp::fs::dir(dir);

    //                 // Middleware fürs Logging aller Requests
    //                 let routes = warp::any()
    //                     .and(static_files)
    //                     .with(warp::log::custom(|info| {
    //                         println!(
    //                             "Request: method={} path={} status={}",
    //                             info.method(),
    //                             info.path(),
    //                             info.status()
    //                         );
    //                     }));

    //                 println!(
    //                     "Static server running on 0.0.0.0:{} serving directory '{}'",
    //                     port, directory
    //                 );

    //                 warp::serve(routes).run(([0, 0, 0, 0], port)).await;
    //             });
    //         });

    //         // Jetzt blockieren wir den aktuellen Thread, damit das Programm nicht direkt exitet:
    //         // z.B. auf Tastendruck warten (du kannst hier auch etwas Eleganteres machen)
    //         println!("Press Enter to stop the server...");
    //         let mut input = String::new();
    //         std::io::stdin().read_line(&mut input).unwrap();

    //         Ok(())
    //     })?
    // };

    // "/" is the System Root Directory
    let start_static_server = {
        let api_handlers = Arc::clone(&api_handlers);
        lua.create_function(move |_, (directory, port): (String, u16)| {
            let dir = directory.clone();

            // // Verzeichnisinhalt beim Start auflisten
            // fn list_files_recursively<P: AsRef<Path>>(path: P, prefix: String) {
            //     if let Ok(entries) = fs::read_dir(path) {
            //         for entry in entries.flatten() {
            //             let path = entry.path();
            //             let display_path =
            //                 format!("{}/{}", prefix, entry.file_name().to_string_lossy());

            //             if path.is_dir() {
            //                 list_files_recursively(&path, display_path);
            //             } else {
            //                 println!("{}", display_path);
            //             }
            //         }
            //     } else {
            //         println!("Verzeichnis '{}' konnte nicht gelesen werden.", prefix);
            //     }
            // }

            // Verzeichnisinhalt (nur oberste Ebene) beim Start auflisten
            fn list_files_direct<P: AsRef<Path>>(path: P) {
                if let Ok(entries) = fs::read_dir(path) {
                    for entry in entries.flatten() {
                        print!("{}\t", entry.path().display());
                    }
                } else {
                    println!("Could not read the directory!");
                }
            }

            // Debug to Check to Directory Content
            println!("Content from '{}':", dir);
            list_files_direct(&dir);
            println!("");

            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
                rt.block_on(async move {
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
                        "Static http server runs on http://0.0.0.0:{} and runs on directory '{}'",
                        port, dir
                    );

                    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
                });
            });

            // Blockiere den aktuellen Thread (optional, wenn du nicht sofort beenden willst)
            println!("Press Enter to stop the server...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            Ok(())
        })?
    };

    table.set("start_static_server", start_static_server)?;

    Ok(table)
}
