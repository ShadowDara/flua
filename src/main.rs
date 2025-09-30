use std::env;

mod api;
mod script;
mod update;
mod utils;

#[cfg(windows)]
mod windows_utf8;

pub const VERSION: &str = "0.1.8";

pub fn deprecated(name: &str, version: &str) {
    println!("[DEPRECATED-WARNING] '{}' is deprectaed since Version: {}", name, version);
}

fn main() {
    #[cfg(windows)]
    let _ = windows_utf8::enable_utf8();

    let args: Vec<String> = env::args().collect();

    // Check if filename
    if args.len() < 2 {
        eprintln!("No Path provided! Run with -h or --help for more information.");
        std::thread::sleep(std::time::Duration::from_secs(5));
        return;
    }

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
            eprintln! {"Safe is not implemnted yet"}
            std::thread::sleep(std::time::Duration::from_secs(5));
            return;
        } else {
            if args[1] == "run" {
                // Update
                if args[2] == "update" {
                    if let Err(e) = update::update() {
                        eprintln!("Update error: {}", e);
                    }
                }
                // Install
                else if args[2] == "install" {
                    if let Err(e) = update::install() {
                        eprintln!("Installation error: {}", e);
                    }
                }

                std::thread::sleep(std::time::Duration::from_secs(5));
                return;
            }
        }
    }

    let path = &args[1];

    println!("[LUAJIT-INFO] running script: {}", path);

    if let Err(e) = script::execute_script(path, &safe) {
        eprintln!("Script error: {}", e);
        std::thread::sleep(std::time::Duration::from_secs(5));
        return;
    }
}
