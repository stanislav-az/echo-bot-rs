use serde::{Deserialize, Serialize};
use std::io;

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

pub fn run_telegram_bot(bot_token: &String) {
    let resp = ureq::get(&mk_telegram_api_url(bot_token, "getMe"))
        .call()
        .unwrap();
    dbg!(&resp);
    let json: TelegramResponse<serde_json::Value> = resp.into_json().unwrap();
    dbg!(&json);
    let resp = ureq::get(&mk_telegram_api_url(bot_token, "getUpdates"))
        .call()
        .unwrap();
    dbg!(&resp);
    let json: TelegramResponse<serde_json::Value> = resp.into_json().unwrap();
    dbg!(&json);
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
    Error(TelegramErrResponse),
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelegramOkResponse<T> {
    pub result: T,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelegramErrResponse {
    pub error_code: u32,
    pub description: String,
}
