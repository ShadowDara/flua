use std::env;
use tokio;

mod api;
mod helper;
mod lua_script;
mod utils;

mod dlm13;

#[cfg(windows)]
mod windows_utf8;

pub const VERSION: &str = "0.2.1";

use crate::helper::print::{END, GREEN, RED, YELLOW};
use crate::helper::exit;

#[tokio::main]
async fn main() {
    // Windows UTF-8 Support aktivieren
    #[cfg(windows)]
    let _ = windows_utf8::enable_utf8();

    // Array of Programm Arguments
    let args: Vec<String> = env::args().collect();

    // TODO
    // Refactor wait on exit for the timer
    let mut wait_on_exit = true;

    if args.len() < 2 {
        println!("{}Flua v{}{}", YELLOW, VERSION, END);
        eprintln!(
            "{}No Path provided! Run with -h or --help for more information.{}",
            RED, END
        );
        exit(wait_on_exit, true);
    }

    // TODO
    // Refactor the Argument Parsing

    //
    // Argumente parsen
    //
    // Config Args
    let mut safe = false;
    let mut info = true;

    // Config
    let mut load_config = true;

    // Usage Args
    let mut version = false;
    let mut help = false;
    let mut helpconfig = false;

    // Modules
    let mut config = false;
    let mut module_init = false;

    let mut args_iter = args.iter().peekable();
    let mut lua_args: Vec<String> = Vec::new();
    let mut collect_lua_args = false;

    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            //
            // IMPORTANT BOOL ARGUMENTS
            //
            // To Suppress a wait Message
            "-nw" => wait_on_exit = false,
            "-no-config" => load_config = false,
            "--no-info" => info = false,
            //
            // ARGUMENTS WHICH CLOSE AFTER RUNNING
            //
            // Safe Mode
            "--safe" => {
                safe = true;
                eprintln!("{}[ERROR] Safe is not implemented yet{}", RED, END);
                exit(wait_on_exit, true);
            }
            // Showing the version
            "--version" | "-v" => version = true,
            // Seding a Help Message
            "--help" | "-h" | "h" => help = true,
            "--help-config" => helpconfig = true,
            // CREATING A MODULE
            "init" => module_init = false,
            // Config Stuff
            "config" => config = true,
            //
            // OTHER ARGUMENTS
            //
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

    // EXECUTION ORDER

    // 1. LOAD THE CONFIG

    let configvalue = helper::config::loadconfig(load_config);
    if !configvalue.show_info {
        info = false;
    }

    // 2. Run Version, Help, Module Init

    if version {
        println!("{}", VERSION);
        exit(wait_on_exit, false);
    }
    if help {
        helper::help();
        exit(wait_on_exit, false);
    }
    if helpconfig {
        helper::config_help();
        exit(wait_on_exit, false);
    }
    // Showing options for config
    if config {
        helper::config::configstuff(args_iter.clone(), wait_on_exit)
    }
    if module_init {
        println!("Not implemnted!");
        exit(wait_on_exit, true);
    }

    match args.get(1).map(String::as_str) {
        // Run a Action command here
        Some("run") => {
            if let Err(e) = handle_run_command(&args).await {
                eprintln!("{}[ERROR] {}{}", RED, e, END);
                exit(wait_on_exit, true);
            }
        }
        // Run a Lua Script here
        Some(script_path) => {
            if let Err(e) = handle_script_execution(script_path, safe, info, lua_args).await {
                eprintln!("{}[FLUA-ERROR] {}{}", RED, e, END);
                exit(wait_on_exit, true);
            }
        }
        None => {
            eprintln!("{}[ERROR] No command or script provided.{}", RED, END);
            exit(wait_on_exit, true);
        }
    }
}

// Function to handle these commands
async fn handle_run_command(args: &Vec<String>) -> Result<(), String> {
    // Create the List
    let mut action_args: Vec<String> = Vec::new();
    let mut args_iter = args.iter().enumerate().peekable();

    // Collect all args starting from index 3
    while let Some((i, next_arg)) = args_iter.peek() {
        if *i < 3 {
            args_iter.next(); // einfach überspringen
            continue;
        }

        if next_arg.starts_with('#') {
            break;
        }

        // clone & push
        let (_, arg) = args_iter.next().unwrap();
        action_args.push(arg.clone());
    }

    // Run the Modules or actions
    match args.get(2).map(String::as_str) {
        Some("update") => {
            helper::update::update().map_err(|e| format!("Update failed: {}", e))?;
        }
        Some("install") => {
            helper::update::install().map_err(|e| format!("Installation failed: {}", e))?;
        }
        Some(_) | None => {
            println!("Starting module...");
            dlm13::start_module(action_args).map_err(|e| format!("Module start failed: {}", e))?;
        }
    }

    Ok(())
}

// Function to run a Lua script -> returns a Error
async fn handle_script_execution(
    path: &str,
    safe: bool,
    info: bool,
    lua_args: Vec<String>,
) -> Result<(), String> {
    if info {
        println!("{}[LUAJIT-INFO] Running script: {}{}", GREEN, path, END);
    }

    let path = path.to_string();
    let path2 = path.clone();

    let join_result = tokio::task::spawn_blocking(move || {
        lua_script::execute_script(&path, &safe, lua_args)
            .map_err(|e| format!("Script error: {}", e))
    })
    .await
    .map_err(|e| format!("Join error: {}", e))?; // 1. await + JoinError behandeln

    // 2. Jetzt das innere Result behandeln
    join_result?; // Wenn Err(String), wird es hier korrekt nach außen gereicht

    if info {
        println!(
            "{}[LUAJIT-INFO] Finished executing: {}{}",
            GREEN, path2, END
        );
    }

    Ok(())
}
