use chrono::{DateTime, Datelike, Local, Timelike};
use std::fmt::{Display, Formatter};

/// Date and time that will be added to a log entry when logged to a file
#[derive(Debug,Clone)]
pub struct LogDate(DateTime<Local>);

impl LogDate {
    #[tracing::instrument]
    pub fn new() -> Self {
        Self(Local::now())
    }
}

impl Default for LogDate {
    #[tracing::instrument]
    fn default() -> Self {
        Self::new()
    }
}

impl Display for LogDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ptr = &self.0;
        write!(
            f,
            "{}/{}/{} {}:{:02} {}",
            ptr.month(),
            ptr.day(),
            ptr.year(),
            ptr.hour12().1,
            ptr.minute(),
            {
                if ptr.hour12().0 {
                    "PM"
                } else {
                    "AM"
                }
            }
        )
    }
}
