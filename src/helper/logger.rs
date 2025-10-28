use chrono::Local;
use dirs_next;
use once_cell::sync::Lazy;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum LogLevel {
    Off = 0,
    Error = 1,
    Warn = 2,
    Debug = 3,
    Info = 4,
    All = 5,
}

pub struct Logger {
    level: u8,
    dir: PathBuf,
}

impl Logger {
    // Make a new Logger
    fn new() -> Self {
        let mut path = dirs_next::config_dir().expect("no config dir");
        path.push("@shadowdara");
        path.push("flua");
        path.push("logs");
        fs::create_dir_all(&path).unwrap();

        Self {
            level: LogLevel::Info as u8,
            dir: path,
        }
    }

    // Logging Function
    fn log(&self, level: LogLevel, msg: &str) {
        if (level as u8) > self.level || level == LogLevel::Off {
            return;
        }

        // <- Sicherstellen, dass der Log-Ordner existiert
        if let Err(e) = fs::create_dir_all(&self.dir) {
            eprintln!("Logger: could not create log dir: {e}");
            return;
        }

        let logfile = self.dir.join("flua.log");
        let mut file = match OpenOptions::new().create(true).append(true).open(&logfile) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Cannot open log file: {e}");
                return;
            }
        };

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let line = format!("[{}] [{:?}] {}\n", timestamp, level, msg);
        if let Err(e) = file.write_all(line.as_bytes()) {
            eprintln!("Logger write failed: {e}");
        }
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

    // Getter for log dir
    pub fn log_dir(&self) -> &PathBuf {
        &self.dir
    }

    // Getter for Log Level
    pub fn level(&self) -> LogLevel {
        match self.level {
            0 => LogLevel::Off,
            1 => LogLevel::Error,
            2 => LogLevel::Warn,
            3 => LogLevel::Debug,
            4 => LogLevel::Info,
            5 => LogLevel::All,
            _ => LogLevel::All,
        }
    }

    // Setter for LogLevel
    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level as u8;
    }
}

static LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| Mutex::new(Logger::new()));

/// Globale Zugriffsfunktion
pub fn logger() -> std::sync::MutexGuard<'static, Logger> {
    LOGGER.lock().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;
    use std::thread;
    use std::time::Duration;

    /// Erstellt eine Logger-Instanz mit temporärem Verzeichnis
    fn temp_logger(level: LogLevel) -> Logger {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().to_path_buf();
        fs::create_dir_all(&dir).unwrap(); // <-- sicherstellen, dass existiert

        Logger {
            level: level as u8,
            dir,
        }
    }

    /// Liest Dateiinhalt in String
    fn read_log(logger: &Logger) -> String {
        let logfile = logger.dir.join("flua.log");
        if !logfile.exists() {
            return String::new();
        }
        let mut buf = String::new();
        let mut file = fs::File::open(logfile).unwrap();
        file.read_to_string(&mut buf).unwrap();
        buf
    }

    #[test]
    fn creates_log_directory_and_file() {
        let logger = temp_logger(LogLevel::Info);
        logger.info("init");
        let logfile = logger.dir.join("flua.log");
        assert!(logfile.exists(), "logfile must be created");
    }

    #[test]
    fn writes_message_to_file() {
        let logger = temp_logger(LogLevel::Info);
        logger.info("Hello Logger");
        let content = read_log(&logger);
        assert!(content.contains("Hello Logger"));
        assert!(content.contains("[Info]"));
    }

    #[test]
    fn filters_by_level_correctly() {
        let logger = temp_logger(LogLevel::Warn);
        logger.debug("debug hidden");
        logger.info("info hidden");
        logger.warn("warn visible");
        logger.error("error visible");

        let content = read_log(&logger);
        assert!(content.contains("warn visible"));
        assert!(content.contains("error visible"));
        assert!(!content.contains("info hidden"));
        assert!(!content.contains("debug hidden"));
    }

    #[test]
    fn appends_multiple_logs() {
        let logger = temp_logger(LogLevel::All);
        logger.info("line 1");
        logger.info("line 2");
        let content = read_log(&logger);
        assert!(content.contains("line 1"));
        assert!(content.contains("line 2"));
        assert!(content.lines().count() >= 2);
    }

    #[test]
    fn includes_timestamp_and_level() {
        let logger = temp_logger(LogLevel::Info);
        logger.info("timestamp test");
        let content = read_log(&logger);
        assert!(content.contains("[Info]"));
        assert!(content.contains("timestamp test"));
        assert!(content.chars().filter(|&c| c == '[').count() >= 2);
    }

    #[test]
    fn error_and_warn_logged_even_when_info_disabled() {
        let logger = temp_logger(LogLevel::Warn);
        logger.info("ignore me");
        logger.error("error!");
        logger.warn("warn!");

        let content = read_log(&logger);
        assert!(content.contains("error!"));
        assert!(content.contains("warn!"));
        assert!(!content.contains("ignore me"));
    }

    #[test]
    fn thread_safety_test() {
        let logger = temp_logger(LogLevel::Info);
        let shared = std::sync::Arc::new(std::sync::Mutex::new(logger));

        let handles: Vec<_> = (0..5)
            .map(|i| {
                let l = shared.clone();
                thread::spawn(move || {
                    let msg = format!("thread-{i}");
                    l.lock().unwrap().info(&msg);
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        // warten, bis alle schreiben
        thread::sleep(Duration::from_millis(100));
        let content = read_log(&shared.lock().unwrap());
        for i in 0..5 {
            assert!(content.contains(&format!("thread-{i}")));
        }
    }

    #[test]
    fn off_level_disables_all_logging() {
        let logger = temp_logger(LogLevel::Off);
        logger.error("nothing");
        logger.warn("nothing");
        let content = read_log(&logger);
        assert!(
            content.is_empty(),
            "no logs should be written when level=Off"
        );
    }

    #[test]
    fn all_level_logs_everything() {
        let logger = temp_logger(LogLevel::All);
        logger.debug("debug");
        logger.info("info");
        logger.warn("warn");
        logger.error("error");

        let content = read_log(&logger);
        assert!(content.contains("debug"));
        assert!(content.contains("info"));
        assert!(content.contains("warn"));
        assert!(content.contains("error"));
    }
}
