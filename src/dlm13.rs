// Custom Modules for Luajit
// IDK what the name means ...

use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;

use crate::VERSION;
use crate::lua_script::execute_script;

#[derive(Debug, Deserialize)]
struct ModuleConfig {
    name: String,
    version: String,
    description: String,
    author: String,
    license: String,
    link: String,
    entrypoint: String,
    luajitversion: Luajitversion,
    edition: i16,
}

#[derive(Debug, Deserialize)]
struct Luajitversion {
    min: String,
    max: String,
}

// Function to start running a Module
pub fn start_module(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Not Implemented yet!");
    return Ok(());

    let mut modulepath = "";

    let mut args_iter = args.iter().peekable();

    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            a if a.starts_with("-path=") => {
                modulepath = &a["-path=".len()..];
            }
            "" => {
                // leerer String – ignorieren oder Fehler melden
            }
            _ => {
                // println!("Unbekanntes Argument: {}", arg);
            }
        }
    }

    // Module Index File
    let index_file: String = "dlm13.yml".to_string();

    let full_path = Path::new(modulepath).join(index_file.clone());

    if !full_path.exists() {
        eprintln!("Error: File '{}' not found!", index_file);
        return Err("Index file not found".into());
    }

    let yaml_str = fs::read_to_string(&full_path)?;
    let config: ModuleConfig = serde_yaml::from_str(&yaml_str)?;

    // Check if the edition is correct
    match config.edition {
        2025 => {
            // Check for 2025 Modules
        }
        _ => {
            return Err("Wrong Version".into());
        }
    };

    //
    //
    // More Module Checks
    //
    // If Name is the same as the folder
    //

    // TODO
    //
    // Update Version Checking
    //
    // Stop when the version is not fitting
    if !is_newer_version(VERSION, config.luajitversion.min.clone())
        && is_newer_version(VERSION, config.luajitversion.max.clone())
    {
        return Err("Error the Version is not fitting".into());
    }

    // Run the entrypoint File
    let _ = handle_module_execution(modulepath, config);

    Ok(())
}

// TODO
// add a function to create a new Module

// TODO
// add this function
// Function to run a Module
// Führt das Modul aus
fn handle_module_execution(basepath: &str, config: ModuleConfig) -> Result<(), Box<dyn Error>> {
    let full_entry_path = Path::new(basepath).join(&config.entrypoint);

    if !full_entry_path.exists() {
        eprintln!(
            "Error: Entrypoint '{}' not found!",
            full_entry_path.display()
        );
        return Err(format!("Entrypoint '{}' not found", config.entrypoint).into());
    }

    println!("Starting Module: {}", config.name);
    println!("Running: {}", full_entry_path.display());

    // TODO: Hier eigentlichen Modulstart integrieren (z.B. LuaJIT aufrufen)
    let args: Vec<String> = Vec::new();

    let file_path = full_entry_path.to_str().ok_or("Invalid path")?;

    match execute_script(file_path, &false, args) {
        Ok(()) => {}
        Err(e) => return Err(format!("Script execution failed: {}", e).into()),
    };

    Ok(())
}

// TODO
// Maybe use the other function
// Function to check if the current version is newer than another
fn is_newer_version(current_version: &str, the_version: String) -> bool {
    let v1 = parse_version(current_version);
    let v2 = parse_version(&the_version);

    v1 > v2
}

// TODO
// Change this Version parsin
// Numbers are not allowed to above 255!
fn parse_version(input: &str) -> [u8; 3] {
    let parts: Vec<u8> = input
        .split(".")
        .take(3)
        .map(|s| s.parse::<u8>())
        .collect::<Result<_, _>>()
        .expect("Version syntax is wrong, it does fit in u8 (numbers from 0 to 255)");

    match parts.as_slice() {
        [a, b, c] => [*a, *b, *c],
        _ => panic!("Version must contain 3 numbers!"),
    }
}

// TODO
// Ideas
// add Option to create a shortcut for executing a module
// Because flua run module ... is very long

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version_valid() {
        assert_eq!(parse_version("1.2.3"), [1, 2, 3]);
        assert_eq!(parse_version("0.0.0"), [0, 0, 0]);
        assert_eq!(parse_version("255.255.255"), [255, 255, 255]);
    }

    #[test]
    #[should_panic]
    fn test_parse_version_too_large() {
        parse_version("256.0.0"); // >255 → panict
    }

    #[test]
    #[should_panic]
    fn test_parse_version_invalid_format() {
        parse_version("1.2"); // zu kurz
    }

    #[test]
    #[should_panic]
    fn test_parse_version_non_numeric() {
        parse_version("a.b.c");
    }

    #[test]
    fn test_is_newer_version_true() {
        assert!(is_newer_version("2.0.0", "1.9.9".to_string()));
        assert!(is_newer_version("1.2.5", "1.2.4".to_string()));
    }

    #[test]
    fn test_is_newer_version_false() {
        assert!(!is_newer_version("1.2.3", "2.0.0".to_string()));
        assert!(!is_newer_version("1.2.3", "1.2.3".to_string())); // gleich ist nicht "newer"
    }

    #[test]
    fn test_deserialize_valid_config() {
        let yaml = r#"
name: "testmodule"
version: "0.1.0"
description: "A test module"
author: "You"
license: "MIT"
link: "https://example.com"
entrypoint: "main.lua"
luajitversion:
  min: "2.1.0"
  max: "2.1.999"
edition: 2025
"#;

        let config: ModuleConfig = serde_yaml::from_str(yaml).expect("Valid YAML should parse");
        assert_eq!(config.name, "testmodule");
        assert_eq!(config.luajitversion.min, "2.1.0");
        assert_eq!(config.edition, 2025);
    }

    #[test]
    #[should_panic]
    fn test_deserialize_invalid_yaml() {
        let invalid_yaml = r#"
name: "broken"
entrypoint: 123  # falscher Typ
"#;
        let _: ModuleConfig = serde_yaml::from_str(invalid_yaml).unwrap(); // sollte fehlschlagen
    }

    use std::env::temp_dir;
    use std::fs::write;

    #[test]
    fn test_start_module_with_valid_config() {
        // Setup: temp dir + fake config
        let temp_path = temp_dir().join("testmodule");
        std::fs::create_dir_all(&temp_path).unwrap();

        let config_path = temp_path.join("dlm13.yml");
        write(
            &config_path,
            r#"
name: "testmod"
version: "0.1.0"
description: "desc"
author: "you"
license: "MIT"
link: "none"
entrypoint: "main.lua"
luajitversion: { min: "0.0.1", max: "9.9.9" }
edition: 2025
"#,
        )
        .unwrap();

        // Fake entrypoint
        write(temp_path.join("main.lua"), "-- entrypoint").unwrap();

        let arg = format!("-path={}", temp_path.to_string_lossy());
        let args = vec![arg];

        let result = start_module(args);

        assert!(result.is_ok(), "start_module failed: {:?}", result);
    }
}
