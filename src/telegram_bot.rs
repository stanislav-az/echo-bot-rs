pub mod api_types;
pub mod api_methods;
pub mod domain_types;
pub mod error;

use api_types::*;
use self::api_methods::{get_updates, send_sticker, send_message};
use domain_types::*;
pub use error::TelegramBotError;

pub fn one_communication_cycle(bot_token: &String) -> Result<(), TelegramBotError> {
    get_updates(bot_token)?
        .into_iter()
        .try_for_each(|u| handle_update(bot_token, u))?;
    Ok(())
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
            UpdateContents::Sticker { file_id } => {
                send_sticker(bot_token, chat_id, file_id).map(|_| ())
            }
            UpdateContents::TextMessage { text } => {
                send_message(bot_token, chat_id, text).map(|_| ())
            }
        },
    }
}
