// Lua Logger

use mlua::{Lua, Result, Value};
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;

// Add Logger for Lua
pub fn register(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;

    // Logger Info
    let info = lua.create_function(|_, msg: String| {})?;

    // Logger Warn
    let warn = lua.create_function(|_, msg: String| {})?;

    // Logger Debug
    let debug = lua.create_function(|_, msg: String| {})?;

    // Logger Error
    let error = lua.create_function(|_, msg: String| {})?;

    // Logger Custom
    let custom = lua.create_function(|_, msg: String| {})?;

    table.set("info", info)?;
    table.set("warn", warn)?;
    table.set("debug", debug)?;
    table.set("error", error)?;
    table.set("custom", custom)?;

    Ok(table)
}
