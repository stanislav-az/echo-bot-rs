use std::io;

use super::api_types::TelegramApiError;

#[derive(Debug)]
pub enum TelegramBotError {
    Api(TelegramApiError),
    HttpClient(ureq::Error),
    Serialization(io::Error),
}
