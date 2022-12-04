pub mod api_methods;
pub mod api_types;
pub mod domain_types;
pub mod error;

use self::api_methods::{get_updates, send_message, send_sticker};
use api_types::*;
use domain_types::*;
pub use error::TelegramBotError;

pub type TelegramBotState = Option<u64>;

pub fn one_communication_cycle(
    bot_token: &String,
    state: &mut TelegramBotState,
) -> Result<(), TelegramBotError> {
    let offset = state.map(|o| o + 1);
    get_updates(bot_token, &offset)?
        .into_iter()
        .try_for_each(|u| {
            let update_id = handle_update(bot_token, u)?;
            *state = Some(update_id);
            Ok(())
        })
}

pub fn handle_update(bot_token: &String, update: TelegramUpdate) -> Result<u64, TelegramBotError> {
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
                send_message(bot_token, chat_id, text)?;
                Ok(update_id)
            }
        },
    }
}
