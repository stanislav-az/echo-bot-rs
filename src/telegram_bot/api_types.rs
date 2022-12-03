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
