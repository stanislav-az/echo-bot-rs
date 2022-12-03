use serde::{Deserialize, Serialize};
use std::io;
use std::fmt;

pub mod config;
pub mod console_bot;
use config::StaticBotSettings;
use console_bot::ConsoleBotState;

pub fn run_console_bot(conf: &StaticBotSettings) {
    let mut init_state = ConsoleBotState::new();
    io::stdin().lines().for_each(|elem| {
        let input = elem.expect("Failed to read user input");
        let response = console_bot::respond_to_user(&conf, &mut init_state, input);
        println!("{response}");
    });
}

pub fn run_telegram_bot(bot_token: &String) -> Result<(), TelegramBotError> {
    let resp = ureq::get(&mk_telegram_api_url(bot_token, "getMe"))
        .call()
        .map_err(TelegramBotError::HttpClient)?;
    dbg!(&resp);
    let json: serde_json::Value = resp
        .into_json::<TelegramResponse<serde_json::Value>>()
        .map_err(TelegramBotError::Serialization)?
        .into_result()
        .map_err(TelegramBotError::Api)?;
    dbg!(&json);
    let resp = ureq::get(&mk_telegram_api_url(bot_token, "getUpdates"))
        .call()
        .map_err(TelegramBotError::HttpClient)?;
    dbg!(&resp);
    let json: serde_json::Value = resp
        .into_json::<TelegramResponse<serde_json::Value>>()
        .map_err(TelegramBotError::Serialization)?
        .into_result()
        .map_err(TelegramBotError::Api)?;
    dbg!(&json);
    Ok(())
}

#[derive(Debug)]
pub enum TelegramBotError {
    Api(TelegramApiError),
    HttpClient(ureq::Error),
    Serialization(io::Error),
}

pub fn mk_telegram_api_url(bot_token: &String, method_name: &str) -> String {
    let mut url = String::from("https://api.telegram.org/bot");
    url.push_str(bot_token);
    url.push('/');
    url.push_str(method_name);
    url
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TelegramResponse<T> {
    Success(TelegramOkResponse<T>),
    Error(TelegramApiError),
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelegramOkResponse<T> {
    pub result: T,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelegramApiError {
    pub error_code: u16,
    pub description: String,
}

impl fmt::Display for TelegramApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, error code {}", self.description, self.error_code)
    }
}

impl<T> TelegramResponse<T> {
    pub fn into_result(self) -> Result<T, TelegramApiError> {
        match self {
            TelegramResponse::Error(err) => Err(err),
            TelegramResponse::Success(resp) => Ok(resp.result),
        }
    }
}
