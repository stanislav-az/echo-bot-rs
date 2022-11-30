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
        match new_repeat {
            Ok(n) => {
                if n > 0 {
                    state.custom_repeat_number = Some(n);
                    state.tweak_is_awaiting();
                    println!("Repeat number changed to: {}", n);
                } else {
                    println!("Failed to parse an integer number, that is greater than zero");
                    println!("{}", conf.repeat_msg);
                }
            }
            _ => {
                println!("Failed to parse an integer number, that is greater than zero");
                println!("{}", conf.repeat_msg);
            }
        }
        return;
    }
    if user_input.trim() == "/help" {
        println!("{}", conf.help_msg);
        return;
    }
    if user_input.trim() == "/repeat" {
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
