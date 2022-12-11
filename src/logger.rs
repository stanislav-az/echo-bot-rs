use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

use crate::config::LoggerSettings;

pub struct Logger {
    log_to_file: Option<File>,
    log_to_stderr: bool,
    log_level_starting_from: LogLevel,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoggerMsg {
    pub text: String,
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl Logger {
    pub fn initialize(config: &LoggerSettings) -> Self {
        let file = if let Some(path) = &config.log_to_file {
            let f = File::options()
                .append(true)
                .create(true)
                .open(path)
                .expect(&format!("Could not open log file on path {}", path));
            Some(f)
        } else {
            None
        };
        Logger {
            log_to_file: file,
            log_to_stderr: config.log_to_stderr,
            log_level_starting_from: config.log_level_starting_from,
        }
    }

    pub fn disable_logs() -> Self {
        Logger {
            log_to_file: None,
            log_to_stderr: false,
            log_level_starting_from: LogLevel::Warn,
        }
    }

    pub fn log(&mut self, level: LogLevel, msg_text: &str) {
        if level >= self.log_level_starting_from
            && (self.log_to_stderr || self.log_to_file.is_some())
        {
            let msg = LoggerMsg {
                text: msg_text.to_string(),
                level,
                timestamp: Utc::now(),
            };
            let json = serde_json::to_string(&msg).expect("Could not serialize log message");
            if self.log_to_stderr {
                eprintln!("{}", &json);
            }
            if let Some(file) = &mut self.log_to_file {
                writeln!(file, "{}", &json).expect("Could not append to log file");
            }
        }
    }

    pub fn log_debug(&mut self, msg_text: &str) {
        self.log(LogLevel::Debug, msg_text)
    }
    pub fn log_info(&mut self, msg_text: &str) {
        self.log(LogLevel::Info, msg_text)
    }
    pub fn log_warn(&mut self, msg_text: &str) {
        self.log(LogLevel::Warn, msg_text)
    }
    pub fn log_error(&mut self, msg_text: &str) {
        self.log(LogLevel::Error, msg_text)
    }
}
