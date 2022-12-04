use serde::de::DeserializeOwned;

pub mod api_types;
pub mod domain_types;
pub mod error;

use api_types::*;
use domain_types::*;
pub use error::TelegramBotError;

pub fn one_communication_cycle(bot_token: &String) -> Result<(), TelegramBotError> {
    get_updates(bot_token)?
        .into_iter()
        .try_for_each(|u| handle_update(bot_token, u))?;
    Ok(())
}

pub fn get_updates(bot_token: &String) -> Result<TelegramUpdates, TelegramBotError> {
    let resp = ureq::get(&mk_telegram_api_url(bot_token, "getUpdates"))
        .call()
        .map_err(TelegramBotError::HttpClient)?;
    parse_response(resp)
}

pub fn send_message(
    bot_token: &String,
    chat_id: u64,
    text: String,
) -> Result<TelegramMessage, TelegramBotError> {
    let resp = ureq::post(&mk_telegram_api_url(bot_token, "sendMessage"))
        .send_json(ureq::json!({
            "chat_id": chat_id,
            "text": text
        }))
        .map_err(TelegramBotError::HttpClient)?;
    parse_response(resp)
}

pub fn handle_update(bot_token: &String, update: TelegramUpdate) -> Result<(), TelegramBotError> {
    let u = Update::new(update);
    match u {
        Update::Ignored { update_id } => Ok(()),
        Update::Message {
            update_id,
            chat_id,
            contents,
        } => match contents {
            UpdateContents::Sticker { file_id } => Ok(()),
            UpdateContents::TextMessage { text } => {
                send_message(bot_token, chat_id, text).map(|_| ())
            }
        },
    }
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
