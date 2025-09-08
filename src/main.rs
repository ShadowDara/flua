use std::env;

mod api;
mod script;
mod update;
mod utils;

pub const VERSION: &str = "1.0.0";

fn main() {
    #[cfg(windows)]
    windows_utf8::enable_utf8();

    let args: Vec<String> = env::args().collect();

    // Check if filename
    if args.len() < 2 {
        eprintln!("No Path provided! Run with -h or --help for more information.");
        return;
    }

    if args[1] == "-h" || args[1] == "--help" {
        println!("LuaAPI-Rust {}", VERSION);
        println!("Usage: luaapi-rust <script.lua> [-safe]");
        println!(" -safe: Run in safe mode (limited API, no OS access)");
        return;
    }

    let mut safe = false;

    if args.len() >= 3 {
        if args[2] == "-safe" {
            safe = true;
        } else {
            if args[1] == "run" {
                // Update
                if args[2] == "update" {
                    if let Err(e) = update::update() {
                        eprintln!("Update error: {}", e);
                    }
                } else if args[2] == "install" {
                    if let Err(e) = update::install() {
                        eprintln!("Installation error: {}", e);
                    }
                }
                
                return;
            }
        }
    }

    let path = &args[1];

    if let Err(e) = script::execute_script(path, &safe) {
        eprintln!("Script error: {}", e);
    }
}
