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

use crate::helper::print::{BLUE, BOLD, CYAN, END, GREEN, PURPLE, RED, YELLOW};

#[tokio::main]
async fn main() {
    // Windows UTF-8 Support aktivieren
    #[cfg(windows)]
    let _ = windows_utf8::enable_utf8();

    // Array of Programm Arguments
    let args: Vec<String> = env::args().collect();

    let mut wait_on_exit = true;

    if args.len() < 2 {
        println!("{}Flua v{}{}", YELLOW, VERSION, END);
        eprintln!(
            "{}No Path provided! Run with -h or --help for more information.{}",
            RED, END
        );
        exit(wait_on_exit);
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
                exit(wait_on_exit);
            }
            "--no-info" => {
                info = false;
            }
            // To Suppress a wait Message
            "-nw" => {
                wait_on_exit = false;
            }
            "--version" | "-v" => {
                println!("{}", VERSION);
                return;
            }
            "--help" | "-h" | "h" => {
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
        // Run a Action command here
        Some("run") => {
            if let Err(e) = handle_run_command(&args).await {
                eprintln!("{}[ERROR] {}{}", RED, e, END);
                exit(wait_on_exit);
            }
        }
        // Run a Lua Script here
        Some(script_path) => {
            if let Err(e) = handle_script_execution(script_path, safe, info, lua_args).await {
                eprintln!("{}[FLUA-ERROR] {}{}", RED, e, END);
                exit(wait_on_exit);
            }
        }
        None => {
            eprintln!("{}[ERROR] No command or script provided.{}", RED, END);
            exit(wait_on_exit);
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

// Function to wait some to read the command in an open Terminal Window
// when an Error appears
//
// TODO
// Make this Interuptable with pressing Enter
fn exit(wait: bool) {
    if wait {
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
    std::process::exit(1);
}

// Function for a detailed INFO help Message
fn print_help() {
    //[GENERALL-OPTIONS]
    let sco = PURPLE;

    //[SCRIPTOPTIONS]
    let sc = YELLOW;

    //[OPTIONS]
    let opt = CYAN;

    //<action>
    let ac = RED;

    //<custom-action-options>
    let aco = CYAN;

    // OptionList
    let op = BLUE;

    // Links
    let link = BLUE;

    // Start
    println!("{}Flua Help:{}", GREEN, END);
    println!("Using Version {}{}v{}{}", BOLD, GREEN, VERSION, END);
    // Lua Scripts
    println!(
        "\nUsage for Lua scripts: {}<flua>{} <script.lua> {}[SCRIPTOPTIONS]{} {}[GENERALL-OPTIONS]{}",
        GREEN, END, sc, END, sco, END
    );
    //[SCRIPTOPTIONS]
    println!("\n{}[SCRIPTOPTIONS]{}", sc, END);
    println!(
        "{}  --safe:        {}Run in safe mode (limited API, no OS access)",
        op, END
    );
    println!(
        "{}  --no-info:     {}Suppress start and end info messages",
        op, END
    );
    println!(
        "{}  l, lua-args    {}submit arguments after lua-args for the lua file which will be run, stop the collectiong when it sees an argument which starts with: {}'-'{}",
        op, END, RED, END
    );
    // Modules
    println!(
        "\nUsage for Modules: {}<flua>{} run {}<actions>{} {}<custom-action-options>{} {}[GENERALL-OPTIONS]{}",
        GREEN, END, ac, END, aco, END, sco, END
    );
    println!("\n{}<actions>{}:   'update', 'install'", ac, END);
    println!(
        "{}  module         {}Argument to run a {}dlm13{} Module",
        op, END, RED, END
    );
    println!(
        "\n{}<custom-action-options>{}: Ends the arg collecting when a path starts with #",
        aco, END
    );
    println!("{}  -path=    {}Add the Module path after the '='", op, END);
    // Other
    println!(
        "\nOther Usage: {}<flua>{} {}[OPTIONS]{} {}[GENERALL-OPTIONS]{}",
        GREEN, END, opt, END, sco, END
    );
    //[OPTIONS]
    println!("\n{}[OPTIONS]{}", opt, END);
    println!(
        "{}  -v, --version  {}Function which prints the version in the terminal",
        op, END
    );
    println!(
        "{}  h, -h, --help  {}Function which prints this help message in the terminal",
        op, END
    );
    //[GENERALL-OPTIONS]
    println!("\n{}[GENERALL-OPTIONS]{}", sco, END);
    println!(
        "{}  -nw            {}No exit -> The Programm closes instantly after an error and showing an error message without waiting for some Time.",
        op, END
    );
    // More
    println!("\nFor more info about Flua and the Lua API, see:");
    println!("{}https://github.com/ShadowDara/LuaAPI-Rust{}", link, END);
    println!("{}https://shadowdara.github.io/flua/{}", link, END);
}
