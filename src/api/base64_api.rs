use base64::{Engine as _, engine::general_purpose};
use mlua::{Error, Lua, Result, Value};

pub fn register(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?; // base64-Tabelle

    // base64.encode(string) -> base64_string
    let encode =
        lua.create_function(|_, input: String| Ok(general_purpose::STANDARD.encode(input)))?;

    // base64.decode(base64_string) -> string
    let decode = lua.create_function(|_, b64: String| {
        let bytes = general_purpose::STANDARD
            .decode(b64)
            .map_err(Error::external)?;

        let s = String::from_utf8(bytes).map_err(Error::external)?;
        Ok(s)
    })?;

    table.set("encode", encode)?;
    table.set("decode", decode)?;

    Ok(table)
}
