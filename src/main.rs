use std::env;
use tokio;

mod api;
mod helper;
mod lua_script;
mod utils;

mod dlm13;

#[cfg(windows)]
mod windows_utf8;

pub const VERSION: &str = "0.2.0";

use crate::helper::print::{BLUE, BOLD, END, GREEN, RED, YELLOW};

#[tokio::main]
async fn main() {
    // Windows UTF-8 Support aktivieren
    #[cfg(windows)]
    let _ = windows_utf8::enable_utf8();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "{}No Path provided! Run with -h or --help for more information.{}",
            RED, END
        );
        std::thread::sleep(std::time::Duration::from_secs(5));
        std::process::exit(1);
    }

    // Argumente parsen
    let mut safe = false;
    let mut info = true;

    let mut args_iter = args.iter().peekable();
    let mut lua_args: Vec<String> = Vec::new();
    let mut collect_lua_args = false;

    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "--safe" => {
                safe = true;
                eprintln!("{}[ERROR] Safe is not implemented yet{}", RED, END);
                std::thread::sleep(std::time::Duration::from_secs(5));
                std::process::exit(1);
            }
            "--no-info" => {
                info = false;
            }
            "--version" | "-v" => {
                println!("{}", VERSION);
                return;
            }
            "--help" | "-h" => {
                print_help();
                return;
            }
            "lua-args" | "l" => {
                // Starte das Sammeln der Lua-Argumente
                collect_lua_args = true;
                // Sammle alle folgenden Argumente, bis eins mit '-' beginnt
                while let Some(next_arg) = args_iter.peek() {
                    if next_arg.starts_with('-') {
                        break;
                    }
                    lua_args.push(args_iter.next().unwrap().clone());
                }
            }
            _ => {
                if collect_lua_args {
                    // Sollte theoretisch nie hier landen, da oben alles gesammelt wird
                    // Kann aber zur Sicherheit leer gelassen werden
                }
            }
        }
    }

    match args.get(1).map(String::as_str) {
        Some("run") => {
            handle_run_command(&args).await;
        }
        Some(script_path) => {
            handle_script_execution(script_path, safe, info, lua_args).await;
        }
        None => {
            eprintln!("{}[ERROR] No command or script provided.{}", RED, END);
            std::thread::sleep(std::time::Duration::from_secs(5));
            std::process::exit(1);
        }
    }
}

async fn handle_run_command(args: &[String]) {
    match args.get(2).map(String::as_str) {
        Some("update") => {
            if let Err(e) = helper::update::update() {
                eprintln!("{}[ERROR] Update failed: {}{}", RED, e, END);
                std::thread::sleep(std::time::Duration::from_secs(5));
                std::process::exit(1);
            }
        }
        Some("install") => {
            if let Err(e) = helper::update::install() {
                eprintln!("{}[ERROR] Installation failed: {}{}", RED, e, END);
                std::thread::sleep(std::time::Duration::from_secs(5));
                std::process::exit(1);
            }
        }
        Some(_) | None => {
            println!("Starting module...");
            if let Err(e) = dlm13::start() {
                eprintln!("{}[ERROR] Module start failed: {}{}", RED, e, END);
                std::thread::sleep(std::time::Duration::from_secs(5));
                std::process::exit(1);
            }
        }
    }
}

async fn handle_script_execution(path: &str, safe: bool, info: bool, lua_args: Vec<String>) {
    if info {
        println!("{}[LUAJIT-INFO] Running script: {}{}", GREEN, path, END);
    }

    let path = path.to_string(); // move into closure
    let path2 = path.clone();
    let result = tokio::task::spawn_blocking(move || {
        lua_script::execute_script(&path, &safe, lua_args).map_err(|e| e.to_string())
    })
    .await;

    match result {
        Ok(Ok(())) => {
            if info {
                println!(
                    "{}[LUAJIT-INFO] Finished executing: {}{}",
                    GREEN, path2, END
                );
            }
        }
        Ok(Err(e)) => {
            eprintln!("{}[LUAJIT-ERROR] Script error: {}{}", RED, e, END);
            std::thread::sleep(std::time::Duration::from_secs(5));
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("{}[LUAJIT-ERROR] Join error: {}{}", RED, e, END);
            std::thread::sleep(std::time::Duration::from_secs(5));
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!("{}Luajit Help:{}", GREEN, END);
    println!("Using Version {}{}v{}{}", BOLD, GREEN, VERSION, END);
    println!(
        "\nUsage for scripts: {}<luajit>{} <script.lua> {}[SCRIPTOPTIONS]{}",
        GREEN, END, YELLOW, END
    );
    println!("\n{}[SCRIPTOPTIONS]{}", YELLOW, END);
    println!(
        "{}  --safe:        {}Run in safe mode (limited API, no OS access)",
        BLUE, END
    );
    println!(
        "{}  --no-info:     {}Suppress start and end info messages",
        BLUE, END
    );
    println!(
        "{}  l, lua-args    {}submit arguments after lua-args for the lua file which will be run, stop the collectiong when it sees an argument which start with -",
        BLUE, END
    );
    println!(
        "\nUsage for Modules: {}<luajit>{} run {}<action>{}",
        GREEN, END, YELLOW, END
    );
    println!(
        "\n{}<action>{}:   'update', 'install', or path to module directory",
        YELLOW, END
    );
    println!(
        "\nOther Usage: {}<luajit>{} {}[OPTIONS]{}",
        GREEN, END, YELLOW, END
    );
    println!("\n{}[OPTIONS]{}", YELLOW, END);
    println!(
        "{}  -v, --version   {}Function which prints the version in the terminal",
        BLUE, END
    );
    println!("\nFor more info about the Lua API, see:");
    println!("{}https://github.com/ShadowDara/LuaAPI-Rust{}", BLUE, END);
}
