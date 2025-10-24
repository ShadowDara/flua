use dirs_next;
use mlua::Lua;
use std::env;
use std::fs;
use std::path::PathBuf;
use tokio;

mod api;
mod helper;
mod lua_script;
mod utils;

mod dlm13;

#[cfg(windows)]
mod windows_utf8;

pub const VERSION: &str = "0.2.1";

use crate::helper::print::{BLUE, BOLD, CYAN, END, GREEN, PURPLE, RED, YELLOW};

struct FluaConfig {
    // CONFIG VALUES
    wait_time: u64,
}

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
    let mut help = false;
    let mut version = false;

    // Modules
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
            "-nw" => {
                wait_on_exit = false;
            }
            "-no-config" => {
                load_config = false;
            }
            "--no-info" => {
                info = false;
            }
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
            "--version" | "-v" => {
                version = true;
            }
            // Seding a Help Message
            "--help" | "-h" | "h" => {
                help = true;
            }
            // CREATING A MODULE
            "init" => {
                module_init = false;
            }
            // Config Stuff
            "config" => {
                match args_iter.peek().map(|s| s.as_str()) {
                    Some("generate") => {
                        args_iter.next(); // consume the "generate" argument
                        println!("Generating config...");
                        // Generate code here
                    }
                    Some("open") => {
                        args_iter.next(); // consume the "open" argument
                        println!("Opening config...");
                        // Open code here
                    }
                    _ => {
                        println!("No subcommand specified for config.");
                    }
                }
                // Interrupt after opening the Config File
                break;
            }
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

    let configvalue = loadconfig(load_config);

    // 2. Run Version, Help, Module Init

    if help {
        print_help();
        exit(wait_on_exit, false);
    }
    if version {
        println!("{}", VERSION);
        exit(wait_on_exit, false);
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

// Function to wait some to read the command in an open Terminal Window
// when an Error appears
//
// TODO
// Make this Interuptable with pressing Enter
fn exit(wait: bool, error: bool) {
    if wait {
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
    if error {
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
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
    println!(
        "{}  -no-config     {}The Programm will not search or load the config file. Standard Option is true, if there is no config file, the step will be skipped. A config file will not be created automaticly!",
        op, END
    );
    // More
    println!("\nFor more info about Flua and the Lua API, see:");
    println!("{}https://github.com/ShadowDara/LuaAPI-Rust{}", link, END);
    println!("{}https://shadowdara.github.io/flua/{}", link, END);
}

fn loadconfig(doload: bool) -> FluaConfig {
    if !doload {
        return FluaConfig { wait_time: 3 };
    }

    let mut path: PathBuf = dirs_next::config_dir().expect("could not find config_dir()");

    path.push("@shadowdara");
    path.push("flua");
    path.push("config.lua");

    let contents: String = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => {
            println!("Config file not found, using default Config.");
            return FluaConfig { wait_time: 3 };
        }
    };

    let lua = Lua::new();

    // Lua ausführen
    lua.load(contents).exec().expect("Failed to exec Lua");

    // Jetzt aus Rust die Lua-Tabelle auslesen
    let globals = lua.globals();
    let config_table = globals
        .get::<mlua::Table>("config")
        .expect("No 'config' table found");

    let wait_time: u64 = config_table.get("wait_time").unwrap_or(0);
    FluaConfig { wait_time }
}
