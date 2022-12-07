use serde::{Deserialize, Serialize};
use std::fmt;

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

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelegramUpdate {
    pub update_id: u64, // TODO why could not deserialize u128?
    pub message: Option<TelegramMessage>,
    pub callback_query: Option<TelegramCallbackQuery>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelegramMessage {
    pub chat: TelegramChat,
    pub text: Option<String>,
    pub sticker: Option<TelegramSticker>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelegramCallbackQuery {
    pub id: String,
    pub message: Option<TelegramMessage>,
    pub data: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelegramChat {
    pub id: u64,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelegramSticker {
    pub file_id: String,
}

pub type TelegramUpdates = Vec<TelegramUpdate>;

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
