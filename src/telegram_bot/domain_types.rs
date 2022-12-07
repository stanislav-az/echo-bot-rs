use super::api_types::TelegramUpdate;

#[derive(Debug, PartialEq, Eq)]
pub enum Update {
    Message {
        update_id: u64,
        chat_id: u64,
        contents: MessageContents,
    },
    CallbackQuery {
        query_id: String,
        update_id: u64,
        chat_id: u64,
        data: String,
    },
    Ignored {
        update_id: u64,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum MessageContents {
    TextMessage { text: String },
    Sticker { file_id: String },
    HelpCommand,
    RepeatCommand,
}

impl Update {
    pub fn new(u: TelegramUpdate) -> Update {
        match (u.message, u.callback_query) {
            (Some(msg), None) => match (msg.text, msg.sticker) {
                (Some(t), None) => {
                    if t == String::from("/help") {
                        return Update::Message {
                            update_id: u.update_id,
                            chat_id: msg.chat.id,
                            contents: MessageContents::HelpCommand,
                        };
                    }
                    if t == String::from("/repeat") {
                        return Update::Message {
                            update_id: u.update_id,
                            chat_id: msg.chat.id,
                            contents: MessageContents::RepeatCommand,
                        };
                    } else {
                        return Update::Message {
                            update_id: u.update_id,
                            chat_id: msg.chat.id,
                            contents: MessageContents::TextMessage { text: t },
                        };
                    }
                }

                (None, Some(s)) => Update::Message {
                    update_id: u.update_id,
                    chat_id: msg.chat.id,
                    contents: MessageContents::Sticker { file_id: s.file_id },
                },
                _ => Update::Ignored {
                    update_id: u.update_id,
                },
            },
            (None, Some(query)) => match (query.data, query.message) {
                (Some(d), Some(m)) => Update::CallbackQuery {
                    query_id: query.id,
                    update_id: u.update_id,
                    chat_id: m.chat.id,
                    data: d,
                },
                _ => Update::Ignored {
                    update_id: u.update_id,
                },
            },
            _ => Update::Ignored {
                update_id: u.update_id,
            },
        }
    }
}
