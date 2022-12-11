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

    pub fn log(&mut self, level: LogLevel, msg_text: String) {
        if level >= self.log_level_starting_from
            && (self.log_to_stderr || self.log_to_file.is_some())
        {
            let msg = LoggerMsg {
                text: msg_text,
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
}
