// Lua Logger

use crate::helper::logger::Logger;
use mlua::{Lua, Result, Table};

/// Register a Lua logger for a specific Logger instance
pub fn register(lua: &Lua, logger: &Logger) -> Result<Table> {
    let table = lua.create_table()?;

    let logger_ref = logger as *const Logger; // Zeiger, damit Lua den Logger referenzieren kann

    let info_fn = lua.create_function(move |_, msg: String| {
        unsafe { (*logger_ref).info(&msg) };
        Ok(())
    })?;
    table.set("info", info_fn)?;

    let warn_fn = lua.create_function(move |_, msg: String| {
        unsafe { (*logger_ref).warn(&msg) };
        Ok(())
    })?;
    table.set("warn", warn_fn)?;

    let debug_fn = lua.create_function(move |_, msg: String| {
        unsafe { (*logger_ref).debug(&msg) };
        Ok(())
    })?;
    table.set("debug", debug_fn)?;

    let error_fn = lua.create_function(move |_, msg: String| {
        unsafe { (*logger_ref).error(&msg) };
        Ok(())
    })?;
    table.set("error", error_fn)?;

    Ok(table)
}

#[cfg(test)]
mod tests {
    use mlua::Lua;
    use std::fs;
    use tempfile::TempDir;

    use crate::api::lua_logger;
    use crate::helper::logger::{LogLevel, Logger};

    // Hilfsfunktion: temporären Logger erzeugen
    fn temp_logger(level: LogLevel) -> (Logger, TempDir) {
        let dir = TempDir::new().unwrap();
        let mut logger = Logger::new_with_path(dir.path());
        logger.set_level(level);
        (logger, dir)
    }

    // Hilfsfunktion: Log-Datei leeren
    fn clear_log_file(logger: &Logger) {
        let log_path = logger.log_dir().join("flua.log");
        if log_path.exists() {
            fs::write(&log_path, "").unwrap();
        }
    }

    // Hilfsfunktion: Log-Datei auslesen
    fn read_log_file(logger: &Logger) -> String {
        let log_path = logger.log_dir().join("flua.log");
        fs::read_to_string(log_path).unwrap_or_default()
    }

    #[test]
    fn lua_logger_info_creates_log_entry() {
        let lua = Lua::new();
        let (logger, _dir) = temp_logger(LogLevel::All);

        let table = lua_logger::register(&lua, &logger).unwrap();
        lua.globals().set("logger", table).unwrap();

        clear_log_file(&logger);

        lua.load(r#"logger.info("Hello from Lua!")"#)
            .exec()
            .unwrap();

        let content = read_log_file(&logger);
        assert!(content.contains("Hello from Lua!"));
        assert!(content.contains("[Info]"));
    }

    #[test]
    fn lua_logger_warn_creates_log_entry() {
        let lua = Lua::new();
        let (logger, _dir) = temp_logger(LogLevel::All);

        let table = lua_logger::register(&lua, &logger).unwrap();
        lua.globals().set("logger", table).unwrap();

        clear_log_file(&logger);

        lua.load(r#"logger.warn("Warning from Lua!")"#)
            .exec()
            .unwrap();

        let content = read_log_file(&logger);
        assert!(content.contains("Warning from Lua!"));
        assert!(content.contains("[Warn]"));
    }

    #[test]
    fn lua_logger_warn_and_error_work() {
        let lua = Lua::new();
        let (logger, _dir) = temp_logger(LogLevel::All);

        let table = lua_logger::register(&lua, &logger).unwrap();
        lua.globals().set("logger", table).unwrap();

        clear_log_file(&logger);

        lua.load(
            r#"
            logger.warn("Lua warning")
            logger.error("Lua error")
        "#,
        )
        .exec()
        .unwrap();

        let content = read_log_file(&logger);
        assert!(content.contains("[Warn]"));
        assert!(content.contains("[Error]"));
        assert!(content.contains("Lua warning"));
        assert!(content.contains("Lua error"));
    }

    #[test]
    fn lua_logger_debug_respects_level_filter() {
        let lua = Lua::new();
        let (logger, _dir) = temp_logger(LogLevel::Warn);

        let table = lua_logger::register(&lua, &logger).unwrap();
        lua.globals().set("logger", table).unwrap();

        clear_log_file(&logger);

        lua.load(r#"logger.debug("This should NOT appear")"#)
            .exec()
            .unwrap();

        let content = read_log_file(&logger);
        assert!(!content.contains("This should NOT appear"));
    }
}
