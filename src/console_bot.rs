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

pub fn console_bot_cycle(conf: &Config, state: &mut ConsoleBotState, user_input: String) {
    if state.is_awaiting_repeat_number {
        let new_repeat = user_input.parse::<u8>();
        // TODO what if repeat number is 0?
        match new_repeat {
            Err(_) => {
                println!("Failed to parse an integer number.");
                println!("{}", conf.repeat_msg);
            }
            Ok(n) => {
                state.custom_repeat_number = Some(n);
                state.tweak_is_awaiting();
                println!("Repeat number changed to: {}", n);
            }
        }
        return;
    }
    if user_input == "/help" {
        println!("{}", conf.help_msg);
        return;
    }
    if user_input == "/repeat" {
        state.tweak_is_awaiting();
        println!("{}", conf.repeat_msg);
    } else {
        let repeat_number = state
            .custom_repeat_number
            .unwrap_or(conf.default_repeat_number);
        for _ in 0..repeat_number {
            println!("{user_input}");
        }
    }
}
