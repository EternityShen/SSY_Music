use core::panic;
use std::{
    fs::{File, OpenOptions},
    io::Write,
};

pub struct Logger {
    file_handle: File,
    _file_path: String,
}

enum Level {
    Info,
    Wran,
    Debug,
    Error,
}

fn get_level(level: Level) -> String {
    match level {
        Level::Info => "INFO".to_string(),
        Level::Wran => "WARN".to_string(),
        Level::Debug => "DEBUG".to_string(),
        Level::Error => "ERROR".to_string(),
    }
}

fn get_time() -> String {
    let now = chrono::Local::now();
    now.format("%d/%m/%Y %H:%M").to_string()
}

impl Logger {
    pub fn new(file_path: &str) -> Self {
        let result = OpenOptions::new().create(true).append(true).open(file_path);

        match result {
            Ok(f) => Self {
                file_handle: f,
                _file_path: file_path.to_string(),
            },
            Err(e) => {
                eprintln!("无法创建Logger对象:{}", e);
                panic!()
            }
        }
    }

    fn log(&mut self, level: Level, msg: String) {
        let message = format!("{} [{}] {}{}", get_time(), get_level(level), msg, "\n");
        let result = self.file_handle.write_all(message.as_bytes());
        match result {
            Ok(_) => {}
            Err(e) => {
                eprintln!("无法写入log 错误:{}", e);
            }
        }
    }

    pub fn info(&mut self, message: String) {
        self.log(Level::Info, message);
    }
    pub fn warn(&mut self, message: String) {
        self.log(Level::Wran, message);
    }
    pub fn debug(&mut self, message: String) {
        self.log(Level::Debug, message);
    }
    pub fn error(&mut self, message: String) {
        self.log(Level::Error, message);
    }

    pub fn clear(&mut self) {
        self.file_handle.set_len(0).unwrap();
    }
}
