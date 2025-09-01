use mlua::{Lua, Result};
use mlua::Table;
use std::fs;
use std::path::Path;

// LUA REPL
/*
pub fn main() -> Result<()> {
    let lua = Lua::new();

    loop {
        let mut input = String::new();
        print!("D-Terminal Lua> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Error while reading");

        let input = input.trim();
        if input == "exit" || input == "quit" {
            break;
        }

        match lua.load(input).eval::<mlua::Value>() {
            Ok(result) => println!("{:?}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}
*/

// Execute an Lua Script
pub fn execute_script(file: &str) -> Result<()> {
    // Datei existiert?
    if !Path::new(file).exists() {
        eprintln!("Error: File '{}' not found!", file);
        return Ok(());
    }

    // Dateiinhalt lesen
    let script = fs::read_to_string(file)
        .map_err(|e| mlua::Error::external(format!("Error while reading: {}", e)))?;

    // Lua-VM erstellen
    let lua = Lua::new();

    //
    //
    //
    // Registriere das API-Modul als Lua-Tabelle
    let module = lua.create_table()?;

    let greet = lua.create_function(|_, name: String| {
        println!("Hello from Rust, {}!", name);
        Ok(())
    })?;
    module.set("greet", greet)?;

    let add = lua.create_function(|_, (a, b): (i64, i64)| Ok(a + b))?;
    module.set("add", add)?;

    // Modul beim Lua package.preload registrieren
    let globals = lua.globals();
    let package: Table = globals.get("package")?;
    let preload: Table = package.get("preload")?;

    preload.set(
        "dapi",
        lua.create_function(move |_, ()| Ok(module.clone()))?,
    )?;
    //
    //
    //

    // Skript ausführen
    lua.load(&script).exec()?;

    Ok(())
}
