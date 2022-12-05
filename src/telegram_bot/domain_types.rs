use super::api_types::TelegramUpdate;

#[derive(Debug, PartialEq, Eq)]
pub enum Update {
    Message {
        update_id: u64,
        chat_id: u64,
        contents: UpdateContents,
    },
    Ignored {
        update_id: u64,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum UpdateContents {
    TextMessage { text: String },
    Sticker { file_id: String },
    HelpCommand,
    RepeatCommand,
}

impl Update {
    pub fn new(u: TelegramUpdate) -> Update {
        match u.message {
            None => Update::Ignored {
                update_id: u.update_id,
            },
            Some(msg) => match (msg.text, msg.sticker) {
                (Some(t), None) => {
                    if t == String::from("/help") {
                        return Update::Message {
                            update_id: u.update_id,
                            chat_id: msg.chat.id,
                            contents: UpdateContents::HelpCommand,
                        };
                    }
                    if t == String::from("/repeat") {
                        return Update::Message {
                            update_id: u.update_id,
                            chat_id: msg.chat.id,
                            contents: UpdateContents::RepeatCommand,
                        };
                    } else {
                        return Update::Message {
                            update_id: u.update_id,
                            chat_id: msg.chat.id,
                            contents: UpdateContents::TextMessage { text: t },
                        };
                    }
                }

                (None, Some(s)) => Update::Message {
                    update_id: u.update_id,
                    chat_id: msg.chat.id,
                    contents: UpdateContents::Sticker { file_id: s.file_id },
                },
                _ => Update::Ignored {
                    update_id: u.update_id,
                },
            },
        }
    }
}
