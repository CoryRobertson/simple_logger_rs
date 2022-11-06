use chrono::{DateTime, Datelike, Local, Timelike};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::string::ToString;
use std::sync::Mutex;

pub static FILE_NAME: Mutex<&str> = Mutex::new("log.log");

pub enum LogLevel {
    Error,
    Warning,
    Log,
}

impl LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Error => "Error".to_string(),
            LogLevel::Warning => "Warn".to_string(),
            LogLevel::Log => "Log".to_string(),
        }
    }
}

pub fn log(text: &str, level: LogLevel) {
    let path = { Path::new(*FILE_NAME.lock().unwrap()) };
    let display = path.display();
    let date: DateTime<Local> = Local::now();

    let file_name = { *FILE_NAME.lock().unwrap() };

    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_name)
    {
        Ok(f) => f,
        Err(e) => {
            panic!("{}, {}", e, display);
        }
    };

    let am_pm = match date.hour12().0 {
        true => "PM",
        false => "AM",
    };

    let time_format = format!(
        "{}:{:02}:{:02}{}",
        date.hour12().1,
        date.minute(),
        date.second(),
        am_pm
    );

    let full_text = format!(
        "[{}]\t[{}-{}-{}: {}]\t{} \n",
        level.to_string(),
        date.year(),
        date.month(),
        date.day(),
        time_format,
        text
    );

    match file.write_all(full_text.as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            panic!("{}, {}", e, display);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn log_test_log() {
        log("this is a log test", LogLevel::Log);
    }

    #[test]
    fn log_test_warn() {
        log("this is a warning test", LogLevel::Warning);
    }

    #[test]
    fn log_test_error() {
        log("this is an error test", LogLevel::Error);
    }

    #[test]
    fn test_diff_filename() {
        *FILE_NAME.lock().unwrap() = "other_log.log";
        log("this is a different log file test", LogLevel::Log);
        sleep(Duration::from_secs(1));
        log("this is a different log file test", LogLevel::Warning);
        sleep(Duration::from_secs(1));
        log("this is a different log file test", LogLevel::Error);
        *FILE_NAME.lock().unwrap() = "log.log";
    }
}
