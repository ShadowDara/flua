use mlua::{Lua, Result, Error};
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
    let get = lua.create_function(|_, key: String| Ok(env::var(&key).ok()))?;

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
    let set = lua.create_function(|_, (key, value): (String, String)| {
        // Sanity check (optional, je nach Plattform)
        if key.contains('\0') || value.contains('\0') {
            return Err(Error::external("Key or value contains null byte"));
        }

        // Rust 1.77+: set_var ist unsafe, also block drumherum
        unsafe {
            env::set_var(&key, &value);
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
