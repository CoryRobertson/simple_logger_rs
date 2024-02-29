use std::fmt::{Display, Formatter};

/// The text component to a log entry
#[derive(Debug,Clone)]
pub struct LogMessage(String);

impl LogMessage {
    #[tracing::instrument]
    pub fn new(text: &str) -> Self {
        Self(text.to_string())
    }
}

impl From<&str> for LogMessage {
    #[tracing::instrument]
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for LogMessage {
    #[tracing::instrument]
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for LogMessage {
    #[tracing::instrument(skip_all)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
