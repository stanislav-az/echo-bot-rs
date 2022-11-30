use super::config::Config;

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

pub fn respond_to_user(conf: &Config, state: &mut ConsoleBotState, user_input: String) -> String {
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
        conf.repeat_msg.clone()
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

fn mk_failed_repeat_msg(conf: &Config) -> String {
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
        let msg = String::from("Hi!");
        let mut state = ConsoleBotState::new();
        let conf = Config {
            help_msg: String::from("help_msg"),
            repeat_msg: String::from("repeat_msg"),
            default_repeat_number: 1,
        };
        let response = respond_to_user(&conf, &mut state, msg.clone());
        assert_eq!(response, msg);
    }

    #[test]
    fn should_send_special_help_msg() {
        let msg = String::from("/help");
        let mut state = ConsoleBotState::new();
        let help_msg = String::from("help_msg");
        let conf = Config {
            help_msg: help_msg.clone(),
            repeat_msg: String::from("repeat_msg"),
            default_repeat_number: 1,
        };
        let response = respond_to_user(&conf, &mut state, msg.clone());
        assert_eq!(response, help_msg);
    }
}
