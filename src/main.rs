use std::env;

mod api;
mod helper;
mod lua_script;
mod utils;

mod dlm13;

#[cfg(windows)]
mod windows_utf8;

pub const VERSION: &str = "0.1.12";

use crate::helper::print::{END, GREEN, RED};

fn main() {
    // Must be add the start IMPORTANT
    #[cfg(windows)]
    let _ = windows_utf8::enable_utf8();

    let args: Vec<String> = env::args().collect();

    // Check if filename
    if args.len() < 2 {
        eprintln!(
            "{}No Path provided! Run with -h or --help for more information.{}",
            RED, END
        );
        std::thread::sleep(std::time::Duration::from_secs(5));
        return;
    }

    // Help Message
    if args[1] == "-h" || args[1] == "--help" {
        println!("Luajit v{}", VERSION);
        println!("\nUsage for scripts: <luajit> <script.lua> [OPTIONS]");
        println!("\n[OPTIONS]");
        println!("  --safe:     Run in safe mode (limited API, no OS access)");
        println!("  --no-info:  Run a script and dont the start and end INFO message from luajit");
        println!("\nUsage for Modules: <luajit> run <path>");
        println!(
            "  <path>:     Directory where the module is located but the argument is optional"
        );
        println!("\nFor more Infos about the Lua API open the docs here");
        println!("https://github.com/ShadowDara/LuaAPI-Rust");
        return;
    }

    if let (Some(cmd), Some(action)) = (args.get(1), args.get(2)) {
        if cmd == "run" {
            match action.as_str() {
                "update" => {
                    if let Err(e) = helper::update::update() {
                        eprintln!("Update error: {}", e);
                        std::thread::sleep(std::time::Duration::from_secs(5));
                        return;
                    }
                }
                "install" => {
                    if let Err(e) = helper::update::install() {
                        eprintln!("Installation error: {}", e);
                        std::thread::sleep(std::time::Duration::from_secs(5));
                        return;
                    }
                }
                _ => {
                    // println!("run");
                }
            }
        }
    }

    // Boolean Values
    let mut safe = false;
    let mut info = true;

    for arg in &args {
        // Save Mode
        if arg == "--safe" {
            safe = true;
            eprintln!("{}[ERROR] Safe is not implemnted yet{}", RED, END);
            std::thread::sleep(std::time::Duration::from_secs(5));
            return;
        } else if arg == "--no-info" {
            info = false;
        }
    }

    let path = &args[1];

    if path == "run" {
        // Execute the Module

        // TODO
        // Parse the mainfile of the Module and read the data of it

        println!("Starting Module");

        let _ = dlm13::start();
    } else {
        // Execute the Lua Script
        if info {
            println!("{}[LUAJIT-INFO] running script: {}{}", GREEN, path, END);
        }

        if let Err(e) = lua_script::execute_script(path, &safe) {
            eprintln!("{}[LUAJIT-ERROR] Script error: {}{}", RED, e, END);
            std::thread::sleep(std::time::Duration::from_secs(5));
            return;
        }

        if info {
            println!(
                "{}[LUAJIT-INFO] finished script executing: {}{}",
                GREEN, path, END
            );
        }
    }
}
