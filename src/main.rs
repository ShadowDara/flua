use std::env;
use tokio;

mod api;
mod helper;
mod lua_script;
mod utils;

mod dlm13;

#[cfg(windows)]
mod windows_utf8;

pub const VERSION: &str = "0.1.13";

use crate::helper::print::{END, GREEN, RED};

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

    for arg in &args {
        match arg.as_str() {
            // Save Mode to prevent file access and other Stuff
            "--safe" => {
                safe = true;
                eprintln!("{}[ERROR] Safe is not implemented yet{}", RED, END);
                std::thread::sleep(std::time::Duration::from_secs(5));

                // Return an Error Message because it is not implemented yet
                std::process::exit(1);
            }
            // Disable Message for starting and ending a script
            "--no-info" => info = false,
            // Show Version Info
            "--version" | "-v" => {
                println!("{}", VERSION);
                return;
            }
            // Show Help
            "--help" | "-h" => {
                print_help();
                return;
            }
            // Else do nothing
            _ => {}
        }
    }

    match args.get(1).map(String::as_str) {
        Some("run") => {
            handle_run_command(&args).await;
        }
        Some(script_path) => {
            handle_script_execution(script_path, safe, info).await;
        }
        None => {
            eprintln!("{}[ERROR] No command or script provided.{}", RED, END);
            std::thread::sleep(std::time::Duration::from_secs(5));
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!("Luajit Help:");
    println!("Using Version v{}", VERSION);
    println!("\nUsage for scripts: <luajit> <script.lua> [SCRIPTOPTIONS]");
    println!("\n[SCRIPTOPTIONS]");
    println!("  --safe:     Run in safe mode (limited API, no OS access)");
    println!("  --no-info:  Suppress start and end info messages");
    println!("\nUsage for Modules: <luajit> run <action>");
    println!("  <action>:   'update', 'install', or path to module directory");
    println!("\nOther Usage: <luajit> [OPTIONS]");
    println!("\n[OPTIONS]");
    println!("  --VERSION   Function which prints the version in the terminal");
    println!("\nFor more info about the Lua API, see:");
    println!("https://github.com/ShadowDara/LuaAPI-Rust");
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

async fn handle_script_execution(path: &str, safe: bool, info: bool) {
    if info {
        println!("{}[LUAJIT-INFO] Running script: {}{}", GREEN, path, END);
    }

    let path = path.to_string(); // move into closure
    let path2 = path.clone();
    let result = tokio::task::spawn_blocking(move || {
        lua_script::execute_script(&path, &safe).map_err(|e| e.to_string())
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
