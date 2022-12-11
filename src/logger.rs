use std::fs::File;
use std::io::Write;

use crate::config::LoggerSettings;

pub struct Logger {
    log_to_file: Option<File>,
    log_to_stderr: bool,
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
        }
    }

    pub fn log(&mut self, msg: String) {
        if self.log_to_stderr {
            eprintln!("{}", &msg);
        }
        if let Some(file) = &mut self.log_to_file {
            writeln!(file, "{}", &msg).expect("Could not append to log file");
        }
    }
}
