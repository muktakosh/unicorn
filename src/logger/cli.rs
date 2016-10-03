//! Console logger

use log::*;

/// Logs to console
pub struct CLILogger;

impl CLILogger {
    pub fn init<'a>(level: &'a str) -> Result<(), SetLoggerError> {
        set_logger(|max_log_level| {
            max_log_level.set(match level {
                "info" => LogLevelFilter::Info,
                "debug" => LogLevelFilter::Debug,
                "error" => LogLevelFilter::Error,
                "warn" => LogLevelFilter::Warn,
                "trace" => LogLevelFilter::Trace,
                _ => LogLevelFilter::Info
            });
            Box::new(CLILogger)
        })
    }
}

impl Log for CLILogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= max_log_level()
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            if record.level() == LogLevel::Info {
                println!("{}", record.args());
            } else {
                println!("[{}] - {}", record.level(), record.args());
            }
        }
    }
}
