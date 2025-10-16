use mlua::{Lua, Result};
use std::fs::File;
use std::io::copy;
use std::thread;
use std::time::Duration;

use crate::deprecated;

use crate::VERSION;

use crate::helper::update::version_checker;

use crate::helper::print::{
    BG_BLACK, BG_BLUE, BG_BRIGHT_BLACK, BG_BRIGHT_BLUE, BG_BRIGHT_CYAN, BG_BRIGHT_GREEN,
    BG_BRIGHT_PURLPE, BG_BRIGHT_RED, BG_BRIGHT_WHITE, BG_BRIGHT_YELLOW, BG_CYAN, BG_GREEN,
    BG_PURPLE, BG_RED, BG_WHITE, BG_YELLOW, BLACK, BLUE, BOLD, BRIGHT_BLACK, BRIGHT_BLUE,
    BRIGHT_CYAN, BRIGHT_GREEN, BRIGHT_PURLPE, BRIGHT_RED, BRIGHT_WHITE, BRIGHT_YELLOW, CYAN, END,
    GREEN, ITALIC, NOT_UNDERLINED, POSITIVE_TEXT, PURPLE, RED, REVERSE_TEXT, UNDERLINED, WHITE,
    YELLOW, clear_terminal,
};

pub fn register(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;

    // a Simple greet function which will be removed soon!
    let greet = lua.create_function(|_, name: String| {
        deprecated!("dapi.greet", "0.1.8", "Why do you even want to use it?");
        println!("Hello from Rust, {}!", name);
        Ok(())
    })?;

    // a Simple add fucntion which will be removed probably soon
    let add = lua.create_function(|_, (a, b): (i64, i64)| {
        deprecated!("dapi.add", "0.1.8", "Its completely useless bro");
        Ok(a + b)
    })?;

    // Get the Version of Current Luajit
    // Returns the Version as a String
    let version = lua.create_function(|_, ()| Ok(VERSION))?;

    // Check if the right Version is used
    // Returns a Boolean and a warning message when the correct version is not used
    let check_version = lua.create_function(
        |_, (version, warning_opt, break_wrong_version): (String, Option<bool>, Option<bool>)| {
            let warning = warning_opt.unwrap_or(true);
            let break_script = break_wrong_version.unwrap_or(false);

            let result = match version_checker(&version, VERSION) {
                true => {
                    if warning {
                        println!("{}[INFO] Using the right Version for LUAJIT!{}", GREEN, END);
                    }
                    Ok(true)
                }
                false => {
                    if warning {
                        println!(
                            "{}[WARNING] Not the right version for luajit is used!{}",
                            YELLOW, END
                        );
                        println!(
                            "{}[WARNING] Required Version: {} => Your Version: {}{}",
                            YELLOW, version, VERSION, END
                        );
                    }
                    // To interupt the script
                    if break_script {
                        // check if the 2nd number is different
                        // check if the 3rd number is not newer
                        return Err(mlua::Error::external(
                            "Wrong Version used! Interrupting Script!",
                        ));
                    }
                    Ok(false)
                }
            };
            result
        },
    )?;

    // Lua Function to throw an Error and Interrupt the Script
    let throw_error = lua.create_function(|_, msg: String| {
        return Err::<(), mlua::Error>(mlua::Error::external(msg));
    })?;

    // Function to download a file
    let download = lua.create_function(|_, (url, destination): (String, String)| {
        deprecated!(
            "dapi.greet",
            "0.1.13",
            "Use dapi_net.download_file() instead!"
        );
        match reqwest::blocking::get(&url) {
            Ok(mut resp) => {
                match File::create(&destination) {
                    Ok(mut out) => {
                        if copy(&mut resp, &mut out).is_ok() {
                            Ok(true) // Erfolgreich heruntergeladen
                        } else {
                            Ok(false) // Fehler beim Schreiben
                        }
                    }
                    Err(_) => Ok(false), // Fehler beim Datei erstellen
                }
            }
            Err(_) => Ok(false), // Fehler beim HTTP-GET
        }
    })?;

    let wait = lua.create_function(|_, time: u64| {
        deprecated!("dapi.greet", "0.1.13", "Use dapi_time.wait() instead!");
        thread::sleep(Duration::from_millis(time));
        Ok(())
    })?;

    // TODO
    // Move the function to a own Rust file for Terminal Stuff
    //
    // Function to clear the terminal content
    let clear = lua.create_function(|_, (): ()| {
        clear_terminal();
        Ok(())
    })?;

    // TODO
    // Move to another Rust file for Terminal Stuff
    //
    // Function to get the ANSI Color Codes to print colored Text
    // Returns a Lua Table
    let get_colors = lua.create_function(|lua, ()| {
        let table = lua.create_table()?;

        table.set("end", END)?;
        table.set("bold", BOLD)?;

        table.set("italic", ITALIC)?;
        table.set("underlined", UNDERLINED)?;

        table.set("reverse_text", REVERSE_TEXT)?;

        table.set("not_underlined", NOT_UNDERLINED)?;

        table.set("positive_text", POSITIVE_TEXT)?;

        table.set("black", BLACK)?;
        table.set("red", RED)?;
        table.set("green", GREEN)?;
        table.set("yellow", YELLOW)?;
        table.set("blue", BLUE)?;
        table.set("purple", PURPLE)?;
        table.set("cyan", CYAN)?;
        table.set("white", WHITE)?;

        table.set("bg_black", BG_BLACK)?;
        table.set("bg_red", BG_RED)?;
        table.set("gb_green", BG_GREEN)?;
        table.set("bg_yellow", BG_YELLOW)?;
        table.set("bg_blue", BG_BLUE)?;
        table.set("bg_purple", BG_PURPLE)?;
        table.set("bg_cyan", BG_CYAN)?;
        table.set("bg_white", BG_WHITE)?;

        table.set("bright_black", BRIGHT_BLACK)?;
        table.set("bright_red", BRIGHT_RED)?;
        table.set("bright_green", BRIGHT_GREEN)?;
        table.set("bright_yellow", BRIGHT_YELLOW)?;
        table.set("bright_blue", BRIGHT_BLUE)?;
        table.set("bright_purple", BRIGHT_PURLPE)?;
        table.set("bright_cyan", BRIGHT_CYAN)?;
        table.set("bright_white", BRIGHT_WHITE)?;

        table.set("bg_bright_black", BG_BRIGHT_BLACK)?;
        table.set("bg_bright_red", BG_BRIGHT_RED)?;
        table.set("bg_bright_green", BG_BRIGHT_GREEN)?;
        table.set("bg_bright_yellow", BG_BRIGHT_YELLOW)?;
        table.set("bg_bright_blue", BG_BRIGHT_BLUE)?;
        table.set("bg_bright_purple", BG_BRIGHT_PURLPE)?;
        table.set("bg_bright_cyan", BG_BRIGHT_CYAN)?;
        table.set("bg_bright_white", BG_BRIGHT_WHITE)?;

        Ok(table)
    })?;

    table.set("greet", greet)?;
    table.set("add", add)?;
    table.set("version", version)?;
    table.set("check_version", check_version)?;
    table.set("throw_error", throw_error)?;
    table.set("download", download)?;
    table.set("wait", wait)?;
    table.set("clear", clear)?;
    table.set("get_colors", get_colors)?;

    Ok(table)
}
