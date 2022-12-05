pub mod api_methods;
pub mod api_types;
pub mod domain_types;
pub mod error;

use std::collections::HashMap;

use self::api_methods::{get_updates, send_keyboard, send_message, send_sticker};
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
    state: &mut TelegramBotState,
) -> Result<(), TelegramBotError> {
    let offset = state.last_handled_update_id.map(|o| o + 1);
    get_updates(bot_token, &offset)?
        .into_iter()
        .try_for_each(|u| {
            let update_id = handle_update(bot_token, conf, state, u)?;
            state.last_handled_update_id = Some(update_id);
            Ok(())
        })
}

pub fn handle_update(
    bot_token: &String,
    conf: &StaticBotSettings,
    state: &mut TelegramBotState,
    update: TelegramUpdate,
) -> Result<u64, TelegramBotError> {
    let u = Update::new(update);
    match u {
        Update::Ignored { update_id } => Ok(update_id),
        Update::Message {
            update_id,
            chat_id,
            contents,
        } => match contents {
            UpdateContents::Sticker { file_id } => {
                send_sticker(bot_token, chat_id, file_id)?;
                Ok(update_id)
            }
            UpdateContents::TextMessage { text } => {
                send_message(bot_token, chat_id, &text)?;
                Ok(update_id)
            }
            UpdateContents::HelpCommand => {
                send_message(bot_token, chat_id, &conf.help_msg)?;
                Ok(update_id)
            }
            UpdateContents::RepeatCommand => {
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
                let repeat_msg = format!("{}\nCurrent repeat number is {}", conf.repeat_msg, repeat_number);
                send_keyboard(bot_token, chat_id, &repeat_msg, keyboard)?;
                Ok(update_id)
            }
        },
    }
}
