use mlua::{Lua, Result};
use std::fs::File;
use std::io::copy;
use std::sync::Arc;

use mlua::Error as LuaError;

use crate::VERSION;
use crate::deprecated;

pub fn register(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;

    // a Simple greet function which will be removed soon!
    let greet = lua.create_function(|_, name: String| {
        deprecated("dapi.greet", "0.1.8");
        println!("Hello from Rust, {}!", name);
        Ok(())
    })?;

    // a Simple add fucntion which will be removed probably soon
    let add = lua.create_function(|_, (a, b): (i64, i64)| {
        deprecated("dapi.add", "0.1.8");
        Ok(a + b)
    })?;

    // Get the Version of Current Luajit
    // Returns the Version as a String
    let version = lua.create_function(|_, ()| Ok(VERSION))?;

    // Check if the right Version is used
    // Returns a Boolean and a warning message when the correct version is not used
    let check_version = lua.create_function(|_, (version, warning_opt): (String, Option<bool>)| {
        let warning = warning_opt.unwrap_or(true);

        let result = match version == VERSION {
            true => Ok(true),
            false => {
                if warning {
                    println!("[WARNING] Not the right version for luajit is used!");
                    println!("[WARNING] Use Version: {}", VERSION);
                }
                Ok(false)
            }
        };
        result
    })?;

    let download = lua.create_function(|_, (url, destination): (String, String)| {
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

    table.set("greet", greet)?;
    table.set("add", add)?;
    table.set("version", version)?;
    table.set("check_version", check_version)?;
    table.set("download", download)?;

    Ok(table)
}
