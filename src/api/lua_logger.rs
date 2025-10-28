// Lua Logger

use mlua::{Lua, Result, Value};
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;

use crate::helper::logger::logger;

// Add Logger for Lua
pub fn register(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;

    // Logger Info
    let info = lua.create_function(|_, msg: String| {
        logger().info(&msg);
        Ok(())
    })?;

    // Logger Warn
    let warn = lua.create_function(|_, msg: String| {
        logger().warn(&msg);
        Ok(())
    })?;

    // Logger Debug
    let debug = lua.create_function(|_, msg: String| {
        logger().debug(&msg);
        Ok(())
    })?;

    // Logger Error
    let error = lua.create_function(|_, msg: String| {
        logger().error(&msg);
        Ok(())
    })?;

    // Logger Custom
    let custom = lua.create_function(|_, msg: String| {
        // logger().
        Ok(())
    })?;

    table.set("info", info)?;
    table.set("warn", warn)?;
    table.set("debug", debug)?;
    table.set("error", error)?;
    table.set("custom", custom)?;

    Ok(table)
}

#[cfg(test)]
mod tests {
    use mlua::Lua;
    use std::{fs, thread, time::Duration};
    use tempfile::tempdir;

    use super::*;
    use crate::api::lua_logger;
    use crate::helper::logger::LogLevel;
    use crate::helper::logger::Logger;

    fn clear_log_file(logger: &Logger) {
        let log_path = logger.log_dir().join("flua.log");
        if log_path.exists() {
            fs::remove_file(&log_path).unwrap();
        }
    }

    fn read_log_file(logger: &Logger) -> String {
        let log_path = logger.log_dir().join("flua.log");
        fs::read_to_string(&log_path).unwrap_or_default()
    }

    #[test]
    fn lua_logger_info_creates_log_entry() {
        let lua = Lua::new();

        // Lua-Logger registrieren
        let table = lua_logger::register(&lua).unwrap();
        lua.globals().set("logger", table).unwrap();

        let logger = crate::helper::logger::logger();
        clear_log_file(&logger);
        drop(logger);

        // Lua-Code ausführen
        lua.load(r#"logger.info("Hello from Lua!")"#)
            .exec()
            .unwrap();

        thread::sleep(Duration::from_millis(100)); // kurz warten für Dateischreibvorgang
        let content = {
            let logger = crate::helper::logger::logger();
            read_log_file(&logger)
        };

        assert!(content.contains("Hello from Lua!"));
        assert!(content.contains("[Info]"));
    }

    #[test]
    fn lua_logger_warn_and_error_work() {
        let lua = Lua::new();
        let table = lua_logger::register(&lua).unwrap();
        lua.globals().set("logger", table).unwrap();

        {
            let logger = crate::helper::logger::logger();
            clear_log_file(&logger);
        }

        lua.load(
            r#"
        logger.warn("Lua warning")
        logger.error("Lua error")
        "#,
        )
        .exec()
        .unwrap();

        thread::sleep(Duration::from_millis(100));

        let content = {
            let logger = crate::helper::logger::logger();
            read_log_file(&logger)
        };

        assert!(content.contains("[Warn]"));
        assert!(content.contains("[Error]"));
        assert!(content.contains("Lua warning"));
        assert!(content.contains("Lua error"));
    }

    #[test]
    fn lua_logger_debug_respects_level_filter() {
        let lua = Lua::new();
        let table = lua_logger::register(&lua).unwrap();
        lua.globals().set("logger", table).unwrap();

        // Temporär LogLevel hochsetzen
        {
            let mut logger = crate::helper::logger::logger();
            logger.set_level(LogLevel::Warn); //= LogLevel::Warn as u8; // nur Warnungen+Errors loggen
            clear_log_file(&logger);
        }

        lua.load(r#"logger.debug("This should NOT appear")"#)
            .exec()
            .unwrap();

        thread::sleep(Duration::from_millis(100));

        let content = {
            let logger = crate::helper::logger::logger();
            read_log_file(&logger)
        };

        assert!(!content.contains("This should NOT appear"));
    }
}
