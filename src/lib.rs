#![crate_type = "lib"]
#![crate_name = "speck_logger_coloring"]
use chrono::Utc;
use cpeck_logger::{LogLevel, Logger};
use cpeck_text_coloring::{black, blue, purple, red, yellow};

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
    pub fn new(level: LogLevel) -> Logger {
        Logger {
            out: Box::new(std::io::stdout()),
            level,
            formatter: |message: &str, level: LogLevel| -> String { format!("{} [{: <16}] {}\n", date_format_now(), color_level(level), message) }
        }
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
    let now = Utc::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}