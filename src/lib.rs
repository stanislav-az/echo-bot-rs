use std::io;

pub mod config;
pub mod console_bot;
pub mod telegram_bot;
pub use telegram_bot::TelegramBotError;
use config::StaticBotSettings;
use console_bot::ConsoleBotState;

pub fn run_console_bot(conf: &StaticBotSettings) {
    let mut init_state = ConsoleBotState::new();
    io::stdin().lines().for_each(|elem| {
        let input = elem.expect("Failed to read user input");
        let response = console_bot::respond_to_user(&conf, &mut init_state, input);
        println!("{response}");
    });
}

pub fn run_telegram_bot(bot_token: &String) -> Result<(), TelegramBotError> {
    telegram_bot::one_communication_cycle(bot_token)
}
