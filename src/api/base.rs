use mlua::{Lua, Result};
use std::fs::File;
use std::io::copy;
use std::sync::Arc;

use mlua::Error as LuaError;

use crate::VERSION;

pub fn register(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;

    let greet = lua.create_function(|_, name: String| {
        println!("Hello from Rust, {}!", name);
        Ok(())
    })?;

    let add = lua.create_function(|_, (a, b): (i64, i64)| Ok(a + b))?;

    let version = lua.create_function(|_, ()| Ok(VERSION))?;

    let download = lua.create_function(|_, (url, destination): (String, String)| {
        let mut resp = reqwest::blocking::get(url)
            .map_err(|e| LuaError::ExternalError(Arc::new(e)))?;
        let mut out = File::create(destination)?;
        copy(&mut resp, &mut out)?;
        Ok(())
    })?;

    table.set("greet", greet)?;
    table.set("add", add)?;
    table.set("version", version)?;
    table.set("download", download)?;

    Ok(table)
}
