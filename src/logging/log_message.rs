use std::fmt::{Display, Formatter};

/// The text component to a log entry
pub struct LogMessage(String);

impl LogMessage {
    pub fn new(text: &str) -> Self {
        Self(text.to_string())
    }
}

impl From<&str> for LogMessage {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for LogMessage {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for LogMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
