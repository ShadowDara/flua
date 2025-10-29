// Rust File for the Lua API

use mlua::{Lua, Table};

use crate::api::{
    base, data_parsing, http as api_http, io as api_io, lua_logger, net as api_net, os as api_os,
    time as api_time,
};

use flualogger::logger;

// Funktion zum Hinzufügen aller APIs
pub fn add_api(lua: &Lua) -> mlua::Result<()> {
    // All the Register functions from the different Lua Modules
    let dapi = base::register(lua)?;
    let dapi_io = api_io::register(lua)?;
    let dapi_os = api_os::register(lua)?;
    let dapi_http = api_http::http::register(lua)?;
    let dapi_json = data_parsing::json::register(lua)?;
    let dapi_toml = data_parsing::toml::register(lua)?;
    let dapi_dotenv = data_parsing::dotenv::register(lua)?;
    let dapi_yaml = data_parsing::yaml::register(lua)?;
    let dapi_ini = data_parsing::ini_parser::register(lua)?;
    let dapi_base64 = data_parsing::base64_api::register(lua)?;
    let dapi_xml = data_parsing::xml::register(lua)?;
    let dapi_http_async = api_http::async_server::register(lua)?;
    let dapi_net = api_net::net::register(lua)?;
    let dapi_time = api_time::register(lua)?;
    let dapi_api_async = api_http::async_api_server::register(lua)?;
    let dapi_sqlite = data_parsing::sqlite_api::register(lua)?;
    let dapi_logger = lua_logger::register(lua, &logger())?;

    // Set the Modules as Lua Globals
    let globals = lua.globals();
    let package: Table = globals.get("package")?;
    let preload: Table = package.get("preload")?;

    let apis = [
        ("dapi", dapi),
        ("dapi_io", dapi_io),
        ("dapi_os", dapi_os),
        ("dapi_http", dapi_http),
        ("dapi_json", dapi_json),
        ("dapi_toml", dapi_toml),
        ("dapi_dotenv", dapi_dotenv),
        ("dapi_yaml", dapi_yaml),
        ("dapi_ini", dapi_ini),
        ("dapi_base64", dapi_base64),
        ("dapi_xml", dapi_xml),
        ("dapi_http_async", dapi_http_async),
        ("dapi_net", dapi_net),
        ("dapi_time", dapi_time),
        ("dapi_api_async", dapi_api_async),
        ("dapi_sqlite", dapi_sqlite),
        ("dapi_logger", dapi_logger),
    ];

    for (name, api) in apis.into_iter() {
        preload.set(name, lua.create_function(move |_, ()| Ok(api.clone()))?)?;
    }

    Ok(())
}
