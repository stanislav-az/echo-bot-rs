use super::config::StaticBotSettings;

#[derive(Debug, PartialEq, Eq)]
pub struct ConsoleBotState {
    pub is_awaiting_repeat_number: bool,
    pub custom_repeat_number: Option<u8>,
}

impl ConsoleBotState {
    pub fn new() -> ConsoleBotState {
        ConsoleBotState {
            is_awaiting_repeat_number: false,
            custom_repeat_number: None,
        }
    }
    pub fn tweak_is_awaiting(&mut self) {
        self.is_awaiting_repeat_number = !self.is_awaiting_repeat_number;
    }
}

pub fn respond_to_user(
    conf: &StaticBotSettings,
    state: &mut ConsoleBotState,
    user_input: String,
) -> String {
    if state.is_awaiting_repeat_number {
        let new_repeat = user_input.parse::<u8>();
        let repeat_msg = match new_repeat {
            Ok(n) => {
                if n > 0 {
                    state.custom_repeat_number = Some(n);
                    state.tweak_is_awaiting();
                    format!("Repeat number changed to: {}", n)
                } else {
                    mk_failed_repeat_msg(conf)
                }
            }
            _ => mk_failed_repeat_msg(conf),
        };
        return repeat_msg;
    }
    if user_input.trim() == "/help" {
        return conf.help_msg.clone();
    }
    if user_input.trim() == "/repeat" {
        state.tweak_is_awaiting();
        mk_repeat_msg(conf, state)
    } else {
        let repeat_number = state
            .custom_repeat_number
            .unwrap_or(conf.default_repeat_number);
        let mut repeat_user_input = user_input.clone();
        for _ in 1..repeat_number {
            repeat_user_input.push('\n');
            repeat_user_input.push_str(&user_input);
        }
        repeat_user_input
    }
}

fn mk_repeat_msg(conf: &StaticBotSettings, state: &ConsoleBotState) -> String {
    let mut repeat_msg = conf.repeat_msg.clone();
    let repeat_number = state
        .custom_repeat_number
        .unwrap_or(conf.default_repeat_number);
    repeat_msg.push('\n');
    repeat_msg.push_str(&format!("Current repeat number is {}", repeat_number));
    repeat_msg
}

fn mk_failed_repeat_msg(conf: &StaticBotSettings) -> String {
    let mut failed_repeat_msg =
        String::from("Failed to parse an integer number, that is greater than zero");
    failed_repeat_msg.push('\n');
    failed_repeat_msg.push_str(&conf.repeat_msg);
    failed_repeat_msg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_repeat_user_messages() {
        let mut state = ConsoleBotState::new();
        let conf = StaticBotSettings {
            help_msg: String::from("help_msg"),
            repeat_msg: String::from("repeat_msg"),
            default_repeat_number: 1,
        };
        let msg = String::from("Hi!");
        let response = respond_to_user(&conf, &mut state, msg.clone());
        assert_eq!(response, msg);
    }

    #[test]
    fn should_send_special_help_msg() {
        let mut state = ConsoleBotState::new();
        let help_msg = String::from("help_msg");
        let conf = StaticBotSettings {
            help_msg: help_msg.clone(),
            repeat_msg: String::from("repeat_msg"),
            default_repeat_number: 1,
        };
        let msg = String::from("/help");
        let response = respond_to_user(&conf, &mut state, msg.clone());
        assert_eq!(response, help_msg);
    }

    #[test]
    fn should_send_special_repeat_msg() {
        let mut state = ConsoleBotState::new();
        let repeat_msg = String::from("repeat_msg\nCurrent repeat number is 1");
        let conf = StaticBotSettings {
            help_msg: String::from("help_msg"),
            repeat_msg: String::from("repeat_msg"),
            default_repeat_number: 1,
        };
        let msg = String::from("/repeat");
        let response = respond_to_user(&conf, &mut state, msg.clone());
        assert_eq!(response, repeat_msg);
    }

    #[test]
    fn should_change_repeat_number() {
        let mut state = ConsoleBotState::new();
        let conf = StaticBotSettings {
            help_msg: String::from("help_msg"),
            repeat_msg: String::from("repeat_msg"),
            default_repeat_number: 1,
        };
        let msg1 = String::from("/repeat");
        let _response1 = respond_to_user(&conf, &mut state, msg1);
        let msg2 = String::from("2");
        let response2 = respond_to_user(&conf, &mut state, msg2);
        assert_eq!(response2, "Repeat number changed to: 2");
    }

    #[test]
    fn should_repeat_user_messages_with_customized_repeat() {
        let mut state = ConsoleBotState::new();
        let conf = StaticBotSettings {
            help_msg: String::from("help_msg"),
            repeat_msg: String::from("repeat_msg"),
            default_repeat_number: 1,
        };
        let msg1 = String::from("/repeat");
        let _response1 = respond_to_user(&conf, &mut state, msg1);
        let msg2 = String::from("2");
        let _response2 = respond_to_user(&conf, &mut state, msg2);
        let msg3 = String::from("hey");
        let response3 = respond_to_user(&conf, &mut state, msg3);
        assert_eq!(response3, "hey\nhey");
    }
}
