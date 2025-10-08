use mlua::{Lua, Result, Table};
use std::fs;
use std::path::Path;

use crate::api::{base, data_parsing, http as api_http, io as api_io, os as api_os};

pub fn execute_script(file: &str, safe_mode: &bool) -> Result<()> {
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

    let globals = lua.globals();
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

    lua.load(&script).exec()?;
    Ok(())
}
