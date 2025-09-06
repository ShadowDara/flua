use std::env;

mod lua_api;

pub const VERSION: &str = "v0.1.7";

fn main() /* -> Result<(), e> */{
    // get Run Arguments
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
        }
    }

    let path = &args[1];

    match lua_api::execute_script(&path, &safe) {
        Ok(()) => {println!("-- Script executed!")},
        Err(e) => {eprintln!("Error: {}", e)}
    }
}
