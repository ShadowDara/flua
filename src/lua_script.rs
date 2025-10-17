use mlua::{Lua, Result, Table};
use std::fs;
use std::path::Path;

use crate::api::{
    base, data_parsing, http as api_http, io as api_io, net as api_net, os as api_os,
    time as api_time,
};

pub fn execute_script(file: &str, safe_mode: &bool, lua_args: Vec<String>) -> Result<()> {
    if *safe_mode {
        println!("Safe mode not yet implemented.");
        return Ok(());
    }

    if !Path::new(file).exists() {
        eprintln!("Error: File '{}' not found!", file);
        return Ok(());
    }

    let script = fs::read_to_string(file)
        .map_err(|e| mlua::Error::external(format!("Error while reading: {}", e)))?;

    let lua = Lua::new();

    // Collect Arguments for Lua
    let lua_arg = lua.create_table()?;
    for (i, arg) in lua_args.iter().enumerate() {
        lua_arg.set(i + 1, arg.clone())?;
    }

    let dapi = base::register(&lua)?;
    let dapi_io = api_io::register(&lua)?;
    let dapi_os = api_os::register(&lua)?;
    let dapi_http = api_http::http::register(&lua)?;
    let dapi_json = data_parsing::json::register(&lua)?;
    let dapi_toml = data_parsing::toml::register(&lua)?;
    let dapi_dotenv = data_parsing::dotenv::register(&lua)?;
    let dapi_yaml = data_parsing::yaml::register(&lua)?;
    let dapi_ini = data_parsing::ini_parser::register(&lua)?;
    let dapi_base64 = data_parsing::base64_api::register(&lua)?;
    let dapi_xml = data_parsing::xml::register(&lua)?;
    let dapi_http_async = api_http::async_server::register(&lua)?;
    let dapi_net = api_net::net::register(&lua)?;
    let dapi_time = api_time::register(&lua)?;
    let dapi_api_async = api_http::async_api_server::register(&lua)?;

    let globals = lua.globals();

    // Add Arguments as a arg Lua Table
    globals.set("arg", lua_arg)?;

    // Add the script path as a Lua path Table named SCRIPT_FULL_PATH
    let full_path = std::fs::canonicalize(Path::new(file)).expect("Path does not work!");
    let _ = globals.set("SCRIPT_FULL_PATH", full_path.to_string_lossy().to_string());

    let package: Table = globals.get("package")?;
    let preload: Table = package.get("preload")?;

    preload.set("dapi", lua.create_function(move |_, ()| Ok(dapi.clone()))?)?;
    preload.set(
        "dapi_io",
        lua.create_function(move |_, ()| Ok(dapi_io.clone()))?,
    )?;
    preload.set(
        "dapi_os",
        lua.create_function(move |_, ()| Ok(dapi_os.clone()))?,
    )?;
    preload.set(
        "dapi_http",
        lua.create_function(move |_, ()| Ok(dapi_http.clone()))?,
    )?;
    preload.set(
        "dapi_json",
        lua.create_function(move |_, ()| Ok(dapi_json.clone()))?,
    )?;
    preload.set(
        "dapi_toml",
        lua.create_function(move |_, ()| Ok(dapi_toml.clone()))?,
    )?;
    preload.set(
        "dapi_dotenv",
        lua.create_function(move |_, ()| Ok(dapi_dotenv.clone()))?,
    )?;
    preload.set(
        "dapi_yaml",
        lua.create_function(move |_, ()| Ok(dapi_yaml.clone()))?,
    )?;
    preload.set(
        "dapi_ini",
        lua.create_function(move |_, ()| Ok(dapi_ini.clone()))?,
    )?;
    preload.set(
        "dapi_base64",
        lua.create_function(move |_, ()| Ok(dapi_base64.clone()))?,
    )?;
    preload.set(
        "dapi_xml",
        lua.create_function(move |_, ()| Ok(dapi_xml.clone()))?,
    )?;
    preload.set(
        "dapi_http_async",
        lua.create_function(move |_, ()| Ok(dapi_http_async.clone()))?,
    )?;
    preload.set(
        "dapi_net",
        lua.create_function(move |_, ()| Ok(dapi_net.clone()))?,
    )?;
    preload.set(
        "dapi_time",
        lua.create_function(move |_, ()| Ok(dapi_time.clone()))?,
    )?;
    preload.set(
        "dapi_api_async",
        lua.create_function(move |_, ()| Ok(dapi_api_async.clone()))?,
    )?;

    // Execute the Script
    lua.load(&script).exec()?;
    Ok(())
}
