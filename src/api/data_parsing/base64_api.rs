use base64::{Engine as _, engine::general_purpose};
use mlua::{Error, Lua, Result};

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

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::{Function, Lua};

    #[test]
    fn test_base64_encode() {
        let lua = Lua::new();
        let base64_table = register(&lua).expect("Failed to register base64 table");

        let encode_func: Function = base64_table.get("encode").unwrap();
        let result: String = encode_func.call("Hello, World!").unwrap();

        assert_eq!(result, "SGVsbG8sIFdvcmxkIQ==");
    }

    #[test]
    fn test_base64_decode() {
        let lua = Lua::new();
        let base64_table = register(&lua).expect("Failed to register base64 table");

        let decode_func: Function = base64_table.get("decode").unwrap();
        let result: String = decode_func.call("SGVsbG8sIFdvcmxkIQ==").unwrap();

        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let lua = Lua::new();
        let base64_table = register(&lua).expect("Failed to register base64 table");

        let encode_func: Function = base64_table.get("encode").unwrap();
        let decode_func: Function = base64_table.get("decode").unwrap();

        let original = "Das ist ein Test!";
        let encoded: String = encode_func.call(original).unwrap();
        let decoded: String = decode_func.call(encoded).unwrap();

        assert_eq!(decoded, original);
    }

    #[test]
    fn test_invalid_base64_decode() {
        let lua = Lua::new();
        let base64_table = register(&lua).expect("Failed to register base64 table");

        let decode_func: Function = base64_table.get("decode").unwrap();

        let result: mlua::Result<String> = decode_func.call("!!!not_base64@@@");
        assert!(result.is_err());
    }
}
