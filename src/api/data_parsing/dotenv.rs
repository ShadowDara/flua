use mlua::{Error, Lua, Result};
use std::env;

/// Registers the `.env` module in Lua, providing access to environment variables.
///
/// This module exposes the following functions to Lua:
/// - `env.load([filename])`
/// - `env.get(key)`
/// - `env.set(key, value)`
pub fn register(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?; // .env-Tabelle

    /// Gets the value of an environment variable.
    ///
    /// # Lua
    /// ```lua
    /// local value = env.get("DATABASE_URL")
    /// if value then
    ///     print("Found:", value)
    /// else
    ///     print("Not set")
    /// end
    /// ```
    ///
    /// # Returns
    /// - `string`: the value if found
    /// - `nil`: if the variable is not set
    let get = lua.create_function(|_, key: String| match env::var(&key) {
        Ok(val) => Ok(Some(val)),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(e) => Err(Error::external(e)),
    })?;

    /// Sets an environment variable (unsafe in multi-threaded contexts).
    ///
    /// This uses `std::env::set_var`, which is `unsafe` as of Rust 1.77.
    /// Only use this in single-threaded scenarios.
    ///
    /// # Lua
    /// ```lua
    /// env.set("MY_VAR", "123")
    /// print(env.get("MY_VAR")) --> "123"
    /// ```
    ///
    /// # Safety
    /// This function uses an `unsafe` block because modifying environment variables
    /// is not thread-safe across all platforms.
    ///
    /// # Errors
    /// Returns a Lua error if key or value contain null bytes (`\0`), which are invalid.
    let set = lua.create_function(|_, (key, value): (String, Option<String>)| {
        if key.contains('\0') {
            return Err(Error::external("Key contains null byte"));
        }

        if let Some(val) = value {
            if val.contains('\0') {
                return Err(Error::external("Value contains null byte"));
            }
            unsafe {
                env::set_var(&key, &val);
            }
        } else {
            unsafe {
                env::remove_var(&key);
            }
        }

        Ok(())
    })?;

    /// Loads environment variables from a `.env` file into the process environment.
    ///
    /// # Lua
    /// ```lua
    /// env.load()             -- loads from ".env" by default
    /// env.load("custom.env") -- loads from a custom file
    /// ```
    ///
    /// # Errors
    /// Returns a Lua error if the file could not be found or parsed.
    let load = lua.create_function(|_, path: Option<String>| {
        let path = path.unwrap_or_else(|| ".env".to_string());
        dotenv::from_filename(path).map_err(mlua::Error::external)?;
        Ok(())
    })?;

    table.set("get", get)?;
    table.set("set", set)?;
    table.set("load", load)?;

    Ok(table)
}

#[cfg(test)]
mod tests {
    use mlua::{Lua, Value};
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;

    // Importiere deine Funktion
    use super::*; // ← anpassen an dein Crate

    #[test]
    fn test_env_get_and_set() -> mlua::Result<()> {
        let lua = Lua::new();
        let env_mod = register(&lua)?;
        lua.globals().set("env", env_mod)?;

        // Setze eine Variable
        lua.load(r#"env.set("FOO", "BAR")"#).exec()?;

        // Lese sie wieder
        let val: Option<String> = lua.load(r#"return env.get("FOO")"#).eval()?;
        assert_eq!(val, Some("BAR".to_string()));

        // Entferne sie wieder
        lua.load(r#"env.set("FOO", nil)"#).exec()?;
        let val: Option<String> = lua.load(r#"return env.get("FOO")"#).eval()?;
        assert_eq!(val, None);

        Ok(())
    }

    #[test]
    fn test_env_get_nonexistent() -> mlua::Result<()> {
        let lua = Lua::new();
        let env_mod = register(&lua)?;
        lua.globals().set("env", env_mod)?;

        // Variable, die es nicht gibt
        let val: Value = lua
            .load(r#"return env.get("SOMETHING_THAT_DOESNT_EXIST")"#)
            .eval()?;
        assert!(matches!(val, Value::Nil));

        Ok(())
    }

    #[test]
    fn test_env_set_with_null_bytes() {
        let lua = Lua::new();
        let env_mod = register(&lua).unwrap();
        lua.globals().set("env", env_mod).unwrap();

        // Null-Byte im Key
        let result: mlua::Result<()> = lua.load(r#"env.set("BAD\0KEY", "VAL")"#).exec();
        assert!(result.is_err());

        // Null-Byte im Value
        let result: mlua::Result<()> = lua.load(r#"env.set("KEY", "VAL\0UE")"#).exec();
        assert!(result.is_err());
    }

    #[test]
    fn test_env_load_default() -> mlua::Result<()> {
        // Erzeuge temporäre .env-Datei
        let path = ".env";
        let mut file = File::create(path).unwrap();
        writeln!(file, "MY_TEST_VAR=123").unwrap();

        let lua = Lua::new();
        let env_mod = register(&lua)?;
        lua.globals().set("env", env_mod)?;

        lua.load(r#"env.load()"#).exec()?;
        assert_eq!(env::var("MY_TEST_VAR").unwrap(), "123");

        // Aufräumen
        fs::remove_file(path).unwrap();
        unsafe {
            env::remove_var("MY_TEST_VAR");
        }

        Ok(())
    }

    #[test]
    fn test_env_load_custom_file() -> mlua::Result<()> {
        let path = "custom.env";
        let mut file = File::create(path).unwrap();
        writeln!(file, "ANOTHER_VAR=hello_world").unwrap();

        let lua = Lua::new();
        let env_mod = register(&lua)?;
        lua.globals().set("env", env_mod)?;

        lua.load(&format!(r#"env.load("{}")"#, path)).exec()?;
        assert_eq!(env::var("ANOTHER_VAR").unwrap(), "hello_world");

        // Aufräumen
        fs::remove_file(path).unwrap();
        unsafe {
            env::remove_var("ANOTHER_VAR");
        }

        Ok(())
    }
}
