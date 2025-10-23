use ini;

use crate::utils::json_utils;

use mlua::{Error, Lua, Result, Table, Value};
use serde_json;

use std::collections::HashMap;

type IniMap = HashMap<String, HashMap<String, Option<String>>>;

/// Hilfsfunktion: Konvertiert eine mlua Lua Table zurück in HashMap
fn lua_table_to_ini(table: &Table) -> Result<IniMap> {
    let mut map = HashMap::new();

    for pair in table.clone().pairs::<String, Table>() {
        let (section, sub_table) = pair?;
        let mut props = HashMap::new();

        for pair2 in sub_table.pairs::<String, Value>() {
            let (key, value) = pair2?;
            let opt_val = match value {
                Value::Nil => None,
                Value::String(s) => Some(s.to_str()?.to_string()),
                // Optional: Andere Typen konvertieren wir auch in Strings
                Value::Integer(i) => Some(i.to_string()),
                Value::Number(n) => Some(n.to_string()),
                Value::Boolean(b) => Some(b.to_string()),
                _ => None,
            };
            props.insert(key.to_lowercase(), opt_val); // optional: keys lowercase machen
        }
        map.insert(section.to_lowercase(), props);
    }

    Ok(map)
}

fn map_to_ini_string(map: &HashMap<String, HashMap<String, Option<String>>>) -> String {
    let mut result = String::new();

    for (section, values) in map {
        // Skip "default" if you want no section header
        if section.to_lowercase() != "default" {
            result.push_str(&format!("[{}]\n", section));
        }

        for (key, value_opt) in values {
            match value_opt {
                Some(value) => result.push_str(&format!("{} = {}\n", key, value)),
                None => result.push_str(&format!("{}\n", key)), // key with no value
            }
        }

        result.push('\n'); // Add blank line between sections
    }

    result
}

pub fn register(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;

    // Start with the content of a Ini File, returns a Lua Table
    let parse_ini = lua.create_function(|lua, value: String| {
        let ini_value: &str = &value;
        let mut ini_map = ini::inistr!(ini_value);

        // 👇 Patch: ensure flag keys are not dropped
        for (_section, entries) in ini_map.iter_mut() {
            for (k, v) in entries.clone() {
                if v.is_none() {
                    entries.insert(k.clone(), Some(String::new()));
                }
            }
        }

        let json_str = serde_json::to_string_pretty(&ini_map)
            .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;
        let json_value: serde_json::Value =
            serde_json::from_str(&json_str).map_err(Error::external)?;
        json_utils::json_to_lua(lua, &json_value)
    })?;

    // Covernts a Lua Table to ini
    let create_ini = lua.create_function(|_, value: Table| {
        let map = lua_table_to_ini(&value)?;
        Ok(map_to_ini_string(&map))
    })?;

    table.set("parse", parse_ini)?;
    table.set("convert", create_ini)?;

    Ok(table)
}

#[cfg(test)]
mod tests {
    use mlua::Lua;

    const SAMPLE_INI: &str = r#"
[server]
host = localhost
port = 8080

[database]
user = admin
password = secret
"#;

    #[test]
    fn test_lua_table_to_ini_roundtrip() {
        let lua = Lua::new();

        let table = lua.create_table().unwrap();
        let server_tbl = lua.create_table().unwrap();
        server_tbl.set("host", "localhost").unwrap();
        server_tbl.set("port", 8080).unwrap();
        table.set("server", server_tbl).unwrap();

        let db_tbl = lua.create_table().unwrap();
        db_tbl.set("user", "admin").unwrap();
        db_tbl.set("password", "secret").unwrap();
        table.set("database", db_tbl).unwrap();

        let ini_str = super::map_to_ini_string(&super::lua_table_to_ini(&table).unwrap());
        assert!(ini_str.contains("[server]"));
        assert!(ini_str.contains("host = localhost"));
        assert!(ini_str.contains("port = 8080"));
        assert!(ini_str.contains("[database]"));
        assert!(ini_str.contains("password = secret"));
    }

    #[test]
    fn test_full_cycle_parse_and_convert() {
        let lua = Lua::new();
        let ini_table = super::register(&lua).unwrap();

        let parse_func: mlua::Function = ini_table.get("parse").unwrap();
        let table: mlua::Table = parse_func.call(SAMPLE_INI.to_string()).unwrap();

        let convert_func: mlua::Function = ini_table.get("convert").unwrap();
        let ini_output: String = convert_func.call(table).unwrap();

        assert!(ini_output.contains("[server]"));
        assert!(ini_output.contains("host = localhost"));
        assert!(ini_output.contains("[database]"));
        assert!(ini_output.contains("user = admin"));
    }

    #[test]
    fn test_empty_and_flag_values() {
        let lua = Lua::new();
        let ini_table = super::register(&lua).unwrap();

        let sample = r#"
[flags]
enabled
disabled = 
checked = true
count = 42
"#;

        let parse_func: mlua::Function = ini_table.get("parse").unwrap();
        let table: mlua::Table = parse_func.call(sample.to_string()).unwrap();

        let flags: mlua::Table = table.get("flags").unwrap();

        // "enabled" ist ein key ohne "=" → sollte Nil oder None sein
        let enabled: Option<String> = flags.get("enabled").ok();
        assert!(enabled.is_none() || enabled.as_deref() == Some(""));

        // "disabled" hat ein leeres "=" → sollte leeren String liefern
        let disabled: Option<String> = flags.get("disabled").ok();
        assert!(disabled.is_none() || disabled.as_deref() == Some(""));

        assert_eq!(flags.get::<String>("checked").unwrap(), "true");
        assert_eq!(flags.get::<String>("count").unwrap(), "42");

        // Roundtrip: zurück in INI konvertieren
        let convert_func: mlua::Function = ini_table.get("convert").unwrap();
        let ini_output: String = convert_func.call(table).unwrap();

        assert!(ini_output.contains("checked = true"));
        assert!(ini_output.contains("count = 42"));
        assert!(ini_output.contains("enabled"));
    }

    #[test]
    fn test_default_section_handling() {
        let lua = Lua::new();
        let ini_table = super::register(&lua).unwrap();

        let sample = r#"
username = guest
password = test123
[server]
host = 127.0.0.1
"#;

        let parse_func: mlua::Function = ini_table.get("parse").unwrap();
        let table: mlua::Table = parse_func.call(sample.to_string()).unwrap();

        let default_section: mlua::Table = table.get("default").unwrap();
        assert_eq!(default_section.get::<String>("username").unwrap(), "guest");
        assert_eq!(
            default_section.get::<String>("password").unwrap(),
            "test123"
        );

        let server: mlua::Table = table.get("server").unwrap();
        assert_eq!(server.get::<String>("host").unwrap(), "127.0.0.1");

        // zurück in INI-Text
        let convert_func: mlua::Function = ini_table.get("convert").unwrap();
        let ini_str: String = convert_func.call(table).unwrap();

        // default keys sollten ohne [default] erscheinen
        assert!(ini_str.contains("username = guest"));
        assert!(ini_str.contains("[server]"));
    }

    #[test]
    fn test_case_insensitivity_of_keys() {
        let lua = Lua::new();
        let ini_table = super::register(&lua).unwrap();

        let sample = r#"
[MySection]
Host = localhost
PORT = 9000
"#;

        let parse_func: mlua::Function = ini_table.get("parse").unwrap();
        let table: mlua::Table = parse_func.call(sample.to_string()).unwrap();

        let sec: mlua::Table = table.get("mysection").unwrap(); // wird kleingeschrieben
        assert_eq!(sec.get::<String>("host").unwrap(), "localhost");
        assert_eq!(sec.get::<String>("port").unwrap(), "9000");

        // Roundtrip prüfen
        let convert_func: mlua::Function = ini_table.get("convert").unwrap();
        let ini_output: String = convert_func.call(table).unwrap();

        assert!(ini_output.contains("[mysection]"));
        assert!(ini_output.contains("port = 9000"));
    }

    #[test]
    fn test_numeric_and_boolean_values_roundtrip() {
        let lua = Lua::new();
        let ini_table = super::register(&lua).unwrap();

        let sample = r#"
[settings]
enabled = true
threshold = 0.75
count = 100
"#;

        let parse_func: mlua::Function = ini_table.get("parse").unwrap();
        let table: mlua::Table = parse_func.call(sample.to_string()).unwrap();

        let settings: mlua::Table = table.get("settings").unwrap();
        assert_eq!(settings.get::<String>("enabled").unwrap(), "true");
        assert_eq!(settings.get::<String>("threshold").unwrap(), "0.75");
        assert_eq!(settings.get::<String>("count").unwrap(), "100");

        let convert_func: mlua::Function = ini_table.get("convert").unwrap();
        let output: String = convert_func.call(table).unwrap();

        assert!(output.contains("enabled = true"));
        assert!(output.contains("threshold = 0.75"));
        assert!(output.contains("count = 100"));
    }

    #[test]
    fn test_complex_roundtrip_multiple_sections() {
        let lua = Lua::new();
        let ini_table = super::register(&lua).unwrap();

        let sample = r#"
[app]
name = testapp
version = 1.2.3

[logging]
level = debug
path = /tmp/app.log

[network]
timeout = 30
ssl = false
"#;

        let parse_func: mlua::Function = ini_table.get("parse").unwrap();
        let table: mlua::Table = parse_func.call(sample.to_string()).unwrap();

        let app: mlua::Table = table.get("app").unwrap();
        assert_eq!(app.get::<String>("name").unwrap(), "testapp");
        assert_eq!(app.get::<String>("version").unwrap(), "1.2.3");

        let net: mlua::Table = table.get("network").unwrap();
        assert_eq!(net.get::<String>("timeout").unwrap(), "30");
        assert_eq!(net.get::<String>("ssl").unwrap(), "false");

        // Rückwandlung prüfen
        let convert_func: mlua::Function = ini_table.get("convert").unwrap();
        let out: String = convert_func.call(table).unwrap();

        assert!(out.contains("[app]"));
        assert!(out.contains("[logging]"));
        assert!(out.contains("[network]"));
        assert!(out.contains("ssl = false"));
        assert!(out.contains("version = 1.2.3"));
    }
}
