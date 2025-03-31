use speck_logger_coloring::ColorizedConsoleLogger;

fn main() {
    let mut logger = ColorizedConsoleLogger::new(cpeck_logger::LogLevel::ALL);

    logger.info("info");
    logger.debug("debug");
    logger.warning("warning");
    logger.error("error");
}
