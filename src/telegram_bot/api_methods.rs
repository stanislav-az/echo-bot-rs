use serde::de::DeserializeOwned;

use super::api_types::{TelegramMessage, TelegramResponse, TelegramUpdates};
use crate::TelegramBotError;

pub fn get_updates(
    bot_token: &String,
    offset: &Option<u64>,
) -> Result<TelegramUpdates, TelegramBotError> {
    let offset_str = offset.map_or(String::new(), |o| o.to_string());
    let resp = ureq::get(&mk_telegram_api_url(bot_token, "getUpdates"))
        .query("offset", &offset_str)
        .call()
        .map_err(TelegramBotError::HttpClient)?;
    parse_response(resp)
}

pub fn send_message(
    bot_token: &String,
    chat_id: u64,
    text: &str,
) -> Result<TelegramMessage, TelegramBotError> {
    let resp = ureq::post(&mk_telegram_api_url(bot_token, "sendMessage"))
        .send_json(ureq::json!({
            "chat_id": chat_id,
            "text": text,
        }))
        .map_err(TelegramBotError::HttpClient)?;
    parse_response(resp)
}

pub fn send_keyboard(
    bot_token: &String,
    chat_id: u64,
    text: &str,
    keyboard: serde_json::Value,
) -> Result<TelegramMessage, TelegramBotError> {
    let resp = ureq::post(&mk_telegram_api_url(bot_token, "sendMessage"))
        .send_json(ureq::json!({
            "chat_id": chat_id,
            "text": text,
            "reply_markup": keyboard
        }))
        .map_err(TelegramBotError::HttpClient)?;
    parse_response(resp)
}

pub fn send_sticker(
    bot_token: &String,
    chat_id: u64,
    file_id: String,
) -> Result<TelegramMessage, TelegramBotError> {
    let resp = ureq::post(&mk_telegram_api_url(bot_token, "sendSticker"))
        .send_json(ureq::json!({
            "chat_id": chat_id,
            "sticker": file_id
        }))
        .map_err(TelegramBotError::HttpClient)?;
    parse_response(resp)
}

pub fn mk_telegram_api_url(bot_token: &String, method_name: &str) -> String {
    let mut url = String::from("https://api.telegram.org/bot");
    url.push_str(bot_token);
    url.push('/');
    url.push_str(method_name);
    url
}

pub fn parse_response<T: DeserializeOwned>(resp: ureq::Response) -> Result<T, TelegramBotError> {
    resp.into_json::<TelegramResponse<T>>()
        .map_err(TelegramBotError::Serialization)?
        .into_result()
        .map_err(TelegramBotError::Api)
}
