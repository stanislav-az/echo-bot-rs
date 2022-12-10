pub mod api_client;
pub mod api_methods;
pub mod api_types;
pub mod domain_types;
pub mod error;

use std::collections::HashMap;

use self::api_client::TelegramApiClient;
use crate::config::StaticBotSettings;
use api_types::*;
use domain_types::*;
pub use error::TelegramBotError;

#[derive(Debug, PartialEq, Eq)]
pub struct TelegramBotState {
    last_handled_update_id: Option<u64>,
    repeat_number_for_chat_id: HashMap<u64, u8>,
}

impl TelegramBotState {
    pub fn new() -> Self {
        TelegramBotState {
            last_handled_update_id: None,
            repeat_number_for_chat_id: HashMap::new(),
        }
    }
}

pub fn one_communication_cycle(
    bot_token: &String,
    conf: &StaticBotSettings,
    client: &impl TelegramApiClient,
    state: &mut TelegramBotState,
) -> Result<(), TelegramBotError> {
    let offset = state.last_handled_update_id.map(|o| o + 1);
    client
        .get_updates(bot_token, &offset)?
        .into_iter()
        .try_for_each(|u| {
            let update_id = handle_update(bot_token, conf, client, state, u)?;
            state.last_handled_update_id = Some(update_id);
            Ok(())
        })
}

pub fn handle_update(
    bot_token: &String,
    conf: &StaticBotSettings,
    client: &impl TelegramApiClient,
    state: &mut TelegramBotState,
    update: TelegramUpdate,
) -> Result<u64, TelegramBotError> {
    let u = Update::new(update);
    match u {
        Update::Ignored { update_id } => Ok(update_id),
        Update::CallbackQuery {
            query_id,
            update_id,
            chat_id,
            data,
        } => {
            let chosen_repeat = data.parse::<u8>().map_err(TelegramBotError::Parsing)?;
            state
                .repeat_number_for_chat_id
                .insert(chat_id, chosen_repeat);
            client.answer_callback_query(
                bot_token,
                &query_id,
                &format!("Number of repeats was changed to {}", chosen_repeat),
            )?;
            Ok(update_id)
        }
        Update::Message {
            update_id,
            chat_id,
            contents,
        } => match contents {
            MessageContents::Sticker { file_id } => {
                let repeat_number = state
                    .repeat_number_for_chat_id
                    .get(&chat_id)
                    .copied()
                    .unwrap_or(conf.default_repeat_number);
                for _ in 0..repeat_number {
                    client.send_sticker(bot_token, chat_id, &file_id)?;
                }
                Ok(update_id)
            }
            MessageContents::TextMessage { text } => {
                let repeat_number = state
                    .repeat_number_for_chat_id
                    .get(&chat_id)
                    .copied()
                    .unwrap_or(conf.default_repeat_number);
                for _ in 0..repeat_number {
                    client.send_message(bot_token, chat_id, &text)?;
                }
                Ok(update_id)
            }
            MessageContents::HelpCommand => {
                client.send_message(bot_token, chat_id, &conf.help_msg)?;
                Ok(update_id)
            }
            MessageContents::RepeatCommand => {
                let buttons: Vec<serde_json::Value> = vec![1, 2, 3, 4, 5]
                    .into_iter()
                    .map(|n| {
                        ureq::json!({
                            "text": format!("{n}"),
                            "callback_data": format!("{n}"),
                        })
                    })
                    .collect();
                let keyboard = ureq::json!({ "inline_keyboard": [buttons] });
                let repeat_number = state
                    .repeat_number_for_chat_id
                    .get(&chat_id)
                    .copied()
                    .unwrap_or(conf.default_repeat_number);
                let repeat_msg = format!(
                    "{}\nCurrent repeat number is {}",
                    conf.repeat_msg, repeat_number
                );
                client.send_keyboard(bot_token, chat_id, &repeat_msg, keyboard)?;
                Ok(update_id)
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct Message {
        chat_id: u64,
        text: String,
    }
    #[derive(Debug, PartialEq, Eq)]
    struct Sticker {
        chat_id: u64,
        file_id: String,
    }
    #[derive(Debug, PartialEq, Eq)]
    struct CallbackAnswer {
        query_id: String,
        text: String,
    }

    struct MockClient {
        updates_to_receive: TelegramUpdates,
        sent_messages: RefCell<Vec<Message>>,
        sent_stickers: RefCell<Vec<Sticker>>,
        sent_callback_answers: RefCell<Vec<CallbackAnswer>>,
    }

    impl TelegramApiClient for MockClient {
        fn get_updates(
            &self,
            _bot_token: &String,
            offset: &Option<u64>,
        ) -> Result<TelegramUpdates, TelegramBotError> {
            let resp = self.updates_to_receive.clone();
            Ok(resp)
        }
        fn send_message(
            &self,
            _bot_token: &String,
            chat_id: u64,
            text: &str,
        ) -> Result<TelegramMessage, TelegramBotError> {
            let msg = Message {
                chat_id,
                text: text.to_string(),
            };
            self.sent_messages.borrow_mut().push(msg);
            Ok(TelegramMessage {
                chat: TelegramChat { id: chat_id },
                text: Some(String::from(text)),
                sticker: None,
            })
        }
        fn send_keyboard(
            &self,
            _bot_token: &String,
            chat_id: u64,
            text: &str,
            _keyboard: serde_json::Value,
        ) -> Result<TelegramMessage, TelegramBotError> {
            let msg = Message {
                chat_id,
                text: text.to_string(),
            };
            self.sent_messages.borrow_mut().push(msg);
            Ok(TelegramMessage {
                chat: TelegramChat { id: chat_id },
                text: Some(String::from(text)),
                sticker: None,
            })
        }
        fn send_sticker(
            &self,
            _bot_token: &String,
            chat_id: u64,
            file_id: &String,
        ) -> Result<TelegramMessage, TelegramBotError> {
            let sticker = Sticker {
                file_id: file_id.clone(),
                chat_id,
            };
            self.sent_stickers.borrow_mut().push(sticker);
            Ok(TelegramMessage {
                chat: TelegramChat { id: chat_id },
                text: None,
                sticker: Some(TelegramSticker {
                    file_id: file_id.clone(),
                }),
            })
        }
        fn answer_callback_query(
            &self,
            _bot_token: &String,
            query_id: &String,
            text: &str,
        ) -> Result<bool, TelegramBotError> {
            let answer = CallbackAnswer {
                query_id: query_id.clone(),
                text: text.to_string(),
            };
            self.sent_callback_answers.borrow_mut().push(answer);
            Ok(true)
        }
    }
}
