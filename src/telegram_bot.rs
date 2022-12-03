pub mod error;
pub mod api_types;

pub use error::TelegramBotError;
use api_types::*;

pub fn one_cycle(bot_token: &String) -> Result<(), TelegramBotError> {
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
  let json: TelegramUpdates = resp
      .into_json::<TelegramResponse<TelegramUpdates>>()
      .map_err(TelegramBotError::Serialization)?
      .into_result()
      .map_err(TelegramBotError::Api)?;
  dbg!(&json);
  Ok(())
}

pub fn mk_telegram_api_url(bot_token: &String, method_name: &str) -> String {
  let mut url = String::from("https://api.telegram.org/bot");
  url.push_str(bot_token);
  url.push('/');
  url.push_str(method_name);
  url
}
