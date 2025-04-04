#![crate_type = "lib"]
#![crate_name = "speck_logger_coloring"]
use chrono::Local;
use speck_logger::{LogLevel, Logger};
use speck_text_coloring::{black, blue, purple, red, yellow};

/// Функционал для создания цветного консольного логгера.
///
/// #Example
///
/// ```
/// let mut logger = speck_logger_coloring::ColorizedConsoleLogger::new(cpeck_logger::LogLevel::ALL);
/// logger.info("info");
/// logger.debug("debug");
/// logger.warning("warning");
/// logger.error("error");
/// ```
pub struct ColorizedConsoleLogger;

impl ColorizedConsoleLogger {
    pub fn new(level: Level) -> Logger {
        Logger {
            out: Box::new(std::io::stdout()),
            level: level_to_log_level(level),
            formatter: |message: &str, level: LogLevel| -> String { format!("{} [{: <16}] {}\n", date_format_now(), color_level(level), message) }
        }
    }
}

pub enum Level {
    ALL,
    DEBUG,
    INFO,
    WARNING,
    ERROR
}

impl Level {
    /// Метод воссоздаст Level из строки или вернет Level::ALL, если операция не будет успешной.
    pub fn from_string(str: &str) -> Level {
        let level = str.to_uppercase();

        if level.eq("DEBUG") {
            Level::DEBUG
        } else if level.eq("ONFO") {
            Level::INFO
        } else if level.eq("WARNING") {
            Level::WARNING
        } else if level.eq("ERROR") {
            Level::ERROR
        } else {
            Level::ALL
        }
    }
}

fn level_to_log_level(level: Level) -> LogLevel {
    match level {
        Level::ALL => LogLevel::ALL,
        Level::DEBUG => LogLevel::DEBUG,
        Level::INFO => LogLevel::INFO,
        Level::WARNING => LogLevel::WARNING,
        Level::ERROR => LogLevel::ERROR,
    }
}

fn color_level(level: LogLevel) -> String {
    match  level {
        LogLevel::ALL => black(&level.to_string()),
        LogLevel::DEBUG => purple(&level.to_string()),
        LogLevel::INFO => blue(&level.to_string()),
        LogLevel::WARNING => yellow(&level.to_string()),
        LogLevel::ERROR => red(&level.to_string()),
    }
}

fn date_format_now() -> String {
    let now = Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}