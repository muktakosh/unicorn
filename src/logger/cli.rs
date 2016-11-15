//! Console logger

use log::*;

/// Logs to console
pub struct CLILogger;

impl CLILogger {
    pub fn init(level: &str) -> Result<(), SetLoggerError> {
        set_logger(|max_log_level| {
            max_log_level.set(match level {
                "debug" => LogLevelFilter::Debug,
                "error" => LogLevelFilter::Error,
                "warn" => LogLevelFilter::Warn,
                "trace" => LogLevelFilter::Trace,
                "info" | _ => LogLevelFilter::Info,
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
            println!("[{}] - {}", record.level(), record.args());
        }
    }
}
