use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub bot_to_run: BotToRun,
    pub static_bot_options: StaticBotSettings,
    pub telegram_bot_token: String,
    pub logger_settings: LoggerSettings,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaticBotSettings {
    pub help_msg: String,
    pub repeat_msg: String,
    pub default_repeat_number: u8,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BotToRun {
    Telegram,
    Console,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoggerSettings {
    pub log_to_file: Option<String>,
    pub log_to_stderr: bool,
}

impl Config {
    pub fn build(default_path: &String, args: &[String]) -> Result<Config, Box<dyn Error>> {
        let args_path = if args.get(1) == Some(&String::from("--config")) {
            args.get(2)
        } else {
            None
        };

        let config_path = args_path.unwrap_or(default_path);

        let yaml = read_to_string(config_path)?;

        serde_yaml::from_str(&yaml).map_err(|err| err.into())
    }
}
