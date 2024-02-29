use crate::logging::log_entry::LogEntry;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::{fs, io};

#[derive(Debug)]
pub struct Logger {
    log_path: Mutex<PathBuf>,
}

impl Logger {

    #[tracing::instrument]
    pub fn new(log_path: PathBuf) -> Result<Self, io::Error> {
        if !log_path.exists() {
            fs::create_dir_all(log_path.parent().unwrap())?;
        }

        Ok(Self {
            log_path: Mutex::new(log_path),
        })
    }

    #[tracing::instrument]
    pub fn get_log_file_path(&self) -> PathBuf {
        self.log_path.lock().unwrap().clone()
    }

    #[tracing::instrument]
    fn get_log_file(path: &Path) -> io::Result<File> {
        OpenOptions::new().create(true).append(true).open(path)
    }

    #[tracing::instrument]
    pub fn log(&self, log_entry: &LogEntry) -> io::Result<usize> {
        let path = self.log_path.lock().unwrap();
        match Self::get_log_file(path.as_path()) {
            Ok(mut log_file) => log_file.write(log_entry.to_string().as_bytes()),
            Err(err) => Err(err),
        }
    }
}
