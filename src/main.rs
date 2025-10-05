use std::env;

mod api;
mod script;
mod helper;
mod utils;

#[cfg(windows)]
mod windows_utf8;

pub const VERSION: &str = "0.1.11";

use crate::helper::print::{
    END,
    RED,
    GREEN
};

fn main() {
    // Must be add the start IMPORTANT
    #[cfg(windows)]
    let _ = windows_utf8::enable_utf8();

    let args: Vec<String> = env::args().collect();

    // Check if filename
    if args.len() < 2 {
        eprintln!("{}No Path provided! Run with -h or --help for more information.{}", RED, END);
        std::thread::sleep(std::time::Duration::from_secs(5));
        return;
    }

    // Help
    if args[1] == "-h" || args[1] == "--help" {
        println!("LuaAPI-Rust {}", VERSION);
        println!("Usage: luaapi-rust <script.lua> [-safe]");
        println!(" -safe: Run in safe mode (limited API, no OS access)");
        std::thread::sleep(std::time::Duration::from_secs(5));
        return;
    }

    let mut safe = false;

    if args.len() >= 3 {
        // Save Mode
        if args[2] == "-safe" {
            safe = true;
            eprintln!("{}[ERROR] Safe is not implemnted yet{}", RED, END);
            std::thread::sleep(std::time::Duration::from_secs(5));
            return;
        } else {
            if args[1] == "run" {
                // Update
                if args[2] == "update" {
                    if let Err(e) = helper::update::update() {
                        eprintln!("Update error: {}", e);
                    }
                }
                // Install
                else if args[2] == "install" {
                    if let Err(e) = helper::update::install() {
                        eprintln!("Installation error: {}", e);
                    }
                }

                std::thread::sleep(std::time::Duration::from_secs(5));
                return;
            }
        }
    }

    let path = &args[1];

    println!("{}[LUAJIT-INFO] running script: {}{}", GREEN, path, END);

    if let Err(e) = script::execute_script(path, &safe) {
        eprintln!("{}[LUAJIT-ERROR] Script error: {}{}", RED, e, END);
        std::thread::sleep(std::time::Duration::from_secs(5));
        return;
    }

    println!("{}[LUAJIT-INFO] finished script executing: {}{}", GREEN, path, END);
}
