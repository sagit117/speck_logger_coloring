#![crate_type = "lib"]
#![crate_name = "speck_logger_coloring"]

use chrono::Local;
pub use speck_logger::{Logger, LogLevel, LogWriter};
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
pub struct ColorizedConsoleLogger {
    logger: Logger
}

impl ColorizedConsoleLogger {
    pub fn new(level: LogLevel) -> Self {
        ColorizedConsoleLogger {
            logger: Logger {
                out: Box::new(std::io::stdout()),
                level,
                formatter: |message: &str, level: LogLevel| -> String { format!("{} [{: <16}] {}\n", date_format_now(), color_level(level), message) }
            }
        }
    }
}

impl LogWriter<()> for ColorizedConsoleLogger {
    fn info(&mut self, message: &str) {
        self.logger.info(message).expect("Ошибка записи лога!");
    }

    fn debug(&mut self, message: &str) {
        self.logger.debug(message).expect("Ошибка записи лога!");
    }

    fn warning(&mut self, message: &str) {
        self.logger.warning(message).expect("Ошибка записи лога!");
    }

    fn error(&mut self, message: &str) {
        self.logger.error(message).expect("Ошибка записи лога!");
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