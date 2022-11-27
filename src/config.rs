use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub help_msg: String,
    pub repeat_msg: String,
    pub default_repeat_number: u8,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, Box<dyn Error>> {
        let args_path = if args.get(1) == Some(&String::from("--config")) {
            args.get(2)
        } else {
            None
        };
        let default_path = String::from("config/settings.local.yaml");
        
        let config_path = args_path.unwrap_or(&default_path);

        let yaml = read_to_string(config_path)?;

        serde_yaml::from_str(&yaml).map_err(|err| err.into())
    }
}
