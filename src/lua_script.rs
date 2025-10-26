use mlua::{Lua, Result, Table};
use std::fs;
use std::path::Path;

use crate::custom_flua_api::add_api;

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

    let globals = lua.globals();

    // Add Arguments as a Lua table
    globals.set("arg", lua_arg)?;

    // Add the script path as a Lua path table named SCRIPT_FULL_PATH
    set_script_paths(&lua, file)?;

    // Register all APIs
    add_api(&lua)?;

    // Execute the script
    lua.load(&script).exec()?;
    Ok(())
}

// Function which collects the paths of the Lua script
fn set_script_paths(lua: &Lua, file: &str) -> Result<()> {
    let globals = lua.globals();

    let full_path = std::fs::canonicalize(Path::new(file))
        .map_err(|e| mlua::Error::external(format!("Path does not work: {}", e)))?;

    globals.set("SCRIPT_FULL_PATH", full_path.to_string_lossy().to_string())?;

    if let Some(script_dir) = full_path.parent() {
        globals.set("SCRIPT_DIRECTORY", script_dir.to_string_lossy().to_string())?;
    } else {
        eprintln!("Could not determine script directory");
    }

    Ok(())
}

//
//
// TODO
// Refactor a lot of Code here from imports between multiple scripts
//
//
//

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_set_script_paths_valid_file() {
        let lua = Lua::new();

        // Temporäres Verzeichnis + Datei
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_script.lua");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "-- dummy lua script").unwrap();

        // Funktion aufrufen
        set_script_paths(&lua, file_path.to_str().unwrap()).unwrap();

        // Lua-Variablen überprüfen
        let globals = lua.globals();

        let script_path: String = globals.get("SCRIPT_FULL_PATH").unwrap();
        let script_dir: String = globals.get("SCRIPT_DIRECTORY").unwrap();

        let expected_path = std::fs::canonicalize(&file_path).unwrap();
        let expected_dir = expected_path.parent().unwrap();

        assert_eq!(script_path, expected_path.to_string_lossy());
        assert_eq!(script_dir, expected_dir.to_string_lossy());

        // Optional: Prüfe, dass \\?\ enthalten ist
        //
        // // Maybe add this function to strip \\?\ from the path
        // fn strip_extended_prefix(path: &Path) -> String {
        //     path.to_string_lossy().replace(r"\\?\", "")
        // }
        //
        #[cfg(target_os = "windows")]
        {
            assert!(script_path.starts_with(r"\\?\"));
            assert!(script_dir.starts_with(r"\\?\"));
        }

        #[cfg(not(target_os = "windows"))]
        {
            assert!(!script_path.starts_with(r"\\?\"));
            assert!(!script_dir.starts_with(r"\\?\"));
        }

        // Lua kann Pfad verwenden
        let code = r#"
            assert(type(SCRIPT_FULL_PATH) == "string")
            assert(type(SCRIPT_DIRECTORY) == "string")
            assert(#SCRIPT_DIRECTORY > 0)
        "#;

        assert!(lua.load(code).exec().is_ok());
    }

    #[test]
    fn test_set_script_paths_invalid_path() {
        let lua = Lua::new();
        let non_existent_file = "this_file_should_not_exist.lua";
        let result = set_script_paths(&lua, non_existent_file);

        assert!(result.is_err()); // ← korrektes Verhalten
    }

    #[test]
    fn test_set_script_paths() {
        let lua = Lua::new();

        // Temporäres Verzeichnis + Datei
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_script.lua");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "-- dummy lua script").unwrap();

        // Funktion aufrufen
        set_script_paths(&lua, file_path.to_str().unwrap()).unwrap();

        // Lua-Variablen überprüfen
        let globals: Table = lua.globals();

        let script_path: String = globals.get("SCRIPT_FULL_PATH").unwrap();
        let script_dir: String = globals.get("SCRIPT_DIRECTORY").unwrap();

        // Überprüfen, ob Pfade korrekt gesetzt sind
        let expected_path = std::fs::canonicalize(&file_path).unwrap();
        let expected_dir = expected_path.parent().unwrap();

        assert_eq!(script_path, expected_path.to_string_lossy());
        assert_eq!(script_dir, expected_dir.to_string_lossy());
    }
}
