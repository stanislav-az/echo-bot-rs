use super::{
    api_methods,
    api_types::{TelegramMessage, TelegramUpdates},
};
use crate::TelegramBotError;

pub trait TelegramApiClient {
    fn get_updates(
        &self,
        bot_token: &String,
        offset: &Option<u64>,
    ) -> Result<TelegramUpdates, TelegramBotError>;
    fn send_message(
        &self,
        bot_token: &String,
        chat_id: u64,
        text: &str,
    ) -> Result<TelegramMessage, TelegramBotError>;
    fn send_keyboard(
        &self,
        bot_token: &String,
        chat_id: u64,
        text: &str,
        keyboard: serde_json::Value,
    ) -> Result<TelegramMessage, TelegramBotError>;
    fn send_sticker(
        &self,
        bot_token: &String,
        chat_id: u64,
        file_id: &String,
    ) -> Result<TelegramMessage, TelegramBotError>;
    fn answer_callback_query(
        &self,
        bot_token: &String,
        query_id: &String,
        text: &str,
    ) -> Result<bool, TelegramBotError>;
}

pub struct HttpIO;

impl TelegramApiClient for HttpIO {
    fn get_updates(
        &self,
        bot_token: &String,
        offset: &Option<u64>,
    ) -> Result<TelegramUpdates, TelegramBotError> {
        api_methods::get_updates(bot_token, offset)
    }
    fn send_message(
        &self,
        bot_token: &String,
        chat_id: u64,
        text: &str,
    ) -> Result<TelegramMessage, TelegramBotError> {
        api_methods::send_message(bot_token, chat_id, text)
    }
    fn send_keyboard(
        &self,
        bot_token: &String,
        chat_id: u64,
        text: &str,
        keyboard: serde_json::Value,
    ) -> Result<TelegramMessage, TelegramBotError> {
        api_methods::send_keyboard(bot_token, chat_id, text, keyboard)
    }
    fn send_sticker(
        &self,
        bot_token: &String,
        chat_id: u64,
        file_id: &String,
    ) -> Result<TelegramMessage, TelegramBotError> {
        api_methods::send_sticker(bot_token, chat_id, file_id)
    }
    fn answer_callback_query(
        &self,
        bot_token: &String,
        query_id: &String,
        text: &str,
    ) -> Result<bool, TelegramBotError> {
        api_methods::answer_callback_query(bot_token, query_id, text)
    }
}
