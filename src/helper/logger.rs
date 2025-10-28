// Logger Functions
// can be configured soon in the Config File

use chrono::Local;
use dirs_next;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf; // für Zeitstempel

pub struct Logger {
    // LoggerLevel
    // 0 -> off
    // 1 -> errors only
    // 2 -> + warnings
    // 3 -> + debug
    // 4 -> + Info
    // 5 -> ALL Stuff
    loggerlevel: u8, // Can be values from 0 to 5
    loggerdir: PathBuf,
}

impl Default for Logger {
    fn default() -> Self {
        let mut path: PathBuf = dirs_next::config_dir().expect("could not find config_dir()");
        path.push("@shadowdara");
        path.push("flua");
        path.push("logs");

        Logger {
            loggerlevel: 2,
            loggerdir: path,
        }
    }
}

// Enum for available Log Levels
#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub enum LogLevel {
    Off = 0,
    Error = 1,
    Warn = 2,
    Debug = 3,
    Info = 4,
    All = 5,
}

// Display funtion for the Logger
impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Logger {
    pub fn log(&self, level: LogLevel, msg: &str) {
        if (level as u8) > self.loggerlevel || level == LogLevel::Off {
            return;
        }

        // Stelle sicher, dass der Pfad existiert
        fs::create_dir_all(&self.loggerdir).expect("Failed to create log directory");

        let logfile = self.loggerdir.join("flua.log");
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&logfile)
            .expect("Failed to open log file");

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let line = format!("[{}] [{}] {}\n", timestamp, level, msg);
        file.write_all(line.as_bytes())
            .expect("Failed to write log");
    }

    pub fn error(&self, msg: &str) {
        self.log(LogLevel::Error, msg);
    }

    pub fn warn(&self, msg: &str) {
        self.log(LogLevel::Warn, msg);
    }

    pub fn debug(&self, msg: &str) {
        self.log(LogLevel::Debug, msg);
    }

    pub fn info(&self, msg: &str) {
        self.log(LogLevel::Info, msg);
    }

    pub fn custom(&self, msg: &str) {
        self.log(LogLevel::All, msg);
    }

    pub fn luaerror(&self, msg: &str) {
        self.log(LogLevel::Error, msg);
    }

    pub fn luawarn(&self, msg: &str) {
        self.log(LogLevel::Warn, msg);
    }

    pub fn luadebug(&self, msg: &str) {
        self.log(LogLevel::Debug, msg);
    }

    pub fn luainfo(&self, msg: &str) {
        self.log(LogLevel::Info, msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;
    use tempfile::tempdir; // für temporäre Testverzeichnisse

    fn setup_logger(level: u8) -> Logger {
        let tmp_dir = tempdir().unwrap();
        Logger {
            loggerlevel: level,
            loggerdir: tmp_dir.path().to_path_buf(),
        }
    }

    fn read_logfile(logger: &Logger) -> String {
        let logfile = logger.loggerdir.join("flua.log");
        let mut content = String::new();
        let mut file = std::fs::File::open(&logfile).unwrap();
        file.read_to_string(&mut content).unwrap();
        content
    }

    #[test]
    fn creates_log_directory_and_file() {
        let logger = setup_logger(5);
        logger.info("Init test log");

        let logfile = logger.loggerdir.join("flua.log");
        assert!(logfile.exists(), "Log file should exist after writing");
    }

    #[test]
    fn writes_message_to_file() {
        let logger = setup_logger(5);
        logger.info("Test message");

        let content = read_logfile(&logger);
        assert!(
            content.contains("Test message"),
            "Log file should contain message"
        );
    }

    #[test]
    fn includes_log_level_and_timestamp() {
        let logger = setup_logger(5);
        logger.warn("Something happened");

        let content = read_logfile(&logger);
        assert!(content.contains("[Warn]"), "Log level tag missing");
        assert!(content.contains("["), "Missing timestamp brackets");
    }

    #[test]
    fn respects_log_level_filter() {
        let logger = setup_logger(1); // only errors
        logger.info("Should not log this");
        logger.error("Should log this");

        let logfile = logger.loggerdir.join("flua.log");
        let content = fs::read_to_string(logfile).unwrap();

        assert!(
            !content.contains("Should not log this"),
            "Info should not appear"
        );
        assert!(content.contains("Should log this"), "Error should appear");
    }

    #[test]
    fn appends_logs_to_existing_file() {
        let logger = setup_logger(5);
        logger.debug("First message");
        logger.debug("Second message");

        let content = read_logfile(&logger);
        assert!(content.contains("First message"));
        assert!(content.contains("Second message"));
    }

    #[test]
    fn handles_multiple_levels_correctly() {
        let logger = setup_logger(3); // up to debug
        logger.error("err1");
        logger.warn("warn1");
        logger.debug("debug1");
        logger.info("info1"); // should NOT be logged

        let content = read_logfile(&logger);

        assert!(content.contains("err1"));
        assert!(content.contains("warn1"));
        assert!(content.contains("debug1"));
        assert!(!content.contains("info1"));
    }
}
