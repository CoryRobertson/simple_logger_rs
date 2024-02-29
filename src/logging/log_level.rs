use std::fmt::{Display, Formatter};

/// The type of a log entry
/// Error represents a log message that needs attention
/// Warn represents a log message that might or might not matter
/// Info represents simple logging info for record keeping purposes
#[derive(Clone,Copy,Debug)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
}

impl Display for LogLevel {
    #[tracing::instrument(skip_all)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => {
                write!(f, "Error")
            }
            Self::Warn => {
                write!(f, "Warn")
            }
            Self::Info => {
                write!(f, "Info")
            }
        }
    }
}
