use std::fs::{OpenOptions};
use std::io::Write;
use std::path::Path;
use std::string::ToString;
use chrono::{Datelike, DateTime, Local, Timelike};

pub static mut FILE_NAME: &str = "log.txt";

pub enum LogLevel {
    Error,
    Warning,
    Log,
}

impl LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Error => {"Error".to_string()}
            LogLevel::Warning => {"Warn".to_string()}
            LogLevel::Log => {"Log".to_string()}
        }
    }
}

pub fn log(text: &str, level: LogLevel) {
    let path = Path::new(unsafe { FILE_NAME });
    let display = path.display();
    let date: DateTime<Local> = Local::now();

    // let mut file = match File::create(&path) {
    //     Ok(f) => {f}
    //     Err(e) => {panic!("{}, {}",e, display);}
    // };

    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(unsafe { FILE_NAME }) {
        Ok(f) => {f}
        Err(e) => {panic!("{}, {}", e, display);}
    };

    let am_pm = match date.hour12().0 {
        true => {"PM"}
        false => {"AM"}
    };

    let time_format = format!("{}:{:02}:{:02}{}", date.hour12().1, date.minute(), date.second(), am_pm);

    let full_text = format!("[{}]\t[{}-{}-{}: {}]\t{} \n", level.to_string(), date.year(), date.month(), date.day(), time_format, text);

    match file.write_all(full_text.as_bytes()) {
        Ok(_) => {}
        Err(e) => {panic!("{}, {}",e, display);}
    };
}


#[cfg(test)]
mod tests {
    use super::*;

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
        unsafe { FILE_NAME = "other_log.txt"; }
        log("this is a different log file test", LogLevel::Log);
        log("this is a different log file test", LogLevel::Warning);
        log("this is a different log file test", LogLevel::Error);
        unsafe { FILE_NAME = "log.txt"; }
    }


}
