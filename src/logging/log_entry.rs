use crate::logging::log_date::LogDate;
use crate::logging::log_level::LogLevel;
use crate::logging::log_message::LogMessage;
use std::fmt::{Display, Formatter};

/// An item that is to be written to a file
pub struct LogEntry {
    log_level: LogLevel,
    log_message: LogMessage,
    log_date: LogDate,
}

impl From<&str> for LogEntry {
    fn from(value: &str) -> Self {
        Self {
            log_level: LogLevel::Info,
            log_message: value.into(),
            log_date: LogDate::default(),
        }
    }
}

impl From<String> for LogEntry {
    fn from(value: String) -> Self {
        Self {
            log_level: LogLevel::Info,
            log_message: value.into(),
            log_date: LogDate::default(),
        }
    }
}

impl LogEntry {
    pub fn new(log_message: LogMessage, log_level: LogLevel) -> Self {
        Self {
            log_level,
            log_message,
            log_date: LogDate::new(),
        }
    }
}

impl Display for LogEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "[{}]\t[{}]\t{}",
            self.log_level, self.log_date, self.log_message
        )
    }
}
