use std::sync::Mutex;

pub mod logging;
pub static FILE_NAME: Mutex<&str> = Mutex::new("log.log");

pub mod prelude {
    pub use crate::logging::logger::Logger;
    pub use crate::FILE_NAME;
}

#[cfg(test)]
mod tests {
    use crate::logging::logger::Logger;
    use std::path::PathBuf;

    #[test]
    fn test_log() {
        let logger = Logger::new(PathBuf::from("test.log")).unwrap();
        logger.log(&("test123".into())).unwrap();
        println!("{:?}", logger.get_log_file_path().canonicalize().unwrap());
        logger.log(&("test123".into())).unwrap();
        logger.log(&("test123".into())).unwrap();
        logger.log(&"test123".into()).unwrap();
        logger.log(&("test123".into())).unwrap();
        logger.log(&("test123".into())).unwrap();
        assert!(true);
    }
}
