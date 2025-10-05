use mlua::{Lua, Result, Table};
use std::fs;
use std::path::Path;

use crate::api::{base, io as api_io, os as api_os, http as api_http, json as api_json, toml as api_toml, dotenv as api_dotenv, yaml as api_yaml};

pub fn execute_script(file: &str, safe_mode: &bool) -> Result<()> {
    if !Path::new(file).exists() {
        eprintln!("Error: File '{}' not found!", file);
        return Ok(());
    }

    let script = fs::read_to_string(file)
        .map_err(|e| mlua::Error::external(format!("Error while reading: {}", e)))?;

    let lua = Lua::new();

    if *safe_mode {
        println!("Safe mode not yet implemented.");
        return Ok(());
    }

    let dapi = base::register(&lua)?;
    let dapi_io = api_io::register(&lua)?;
    let dapi_os = api_os::register(&lua)?;
    let dapi_http = api_http::register(&lua)?;
    let dapi_json = api_json::register(&lua)?;
    let dapi_toml = api_toml::register(&lua)?;
    let dapi_dotenv = api_dotenv::register(&lua)?;
    let dapi_yaml = api_yaml::register(&lua)?;

    let globals = lua.globals();
    let package: Table = globals.get("package")?;
    let preload: Table = package.get("preload")?;

    preload.set("dapi", lua.create_function(move |_, ()| Ok(dapi.clone()))?)?;
    preload.set("dapi_io", lua.create_function(move |_, ()| Ok(dapi_io.clone()))?)?;
    preload.set("dapi_os", lua.create_function(move |_, ()| Ok(dapi_os.clone()))?)?;
    preload.set("dapi_http", lua.create_function(move |_, ()| Ok(dapi_http.clone()))?)?;
    preload.set("dapi_json", lua.create_function(move |_, ()| Ok(dapi_json.clone()))?)?;
    preload.set("dapi_toml", lua.create_function(move |_, ()| Ok(dapi_toml.clone()))?)?;
    preload.set("dapi_dotenv", lua.create_function(move |_, ()| Ok(dapi_dotenv.clone()))?)?;
    preload.set("dapi_yaml", lua.create_function(move |_, ()| Ok(dapi_yaml.clone()))?)?;

    lua.load(&script).exec()?;
    Ok(())
}
