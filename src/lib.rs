use std::io;
use std::{thread, time};

pub mod config;
pub mod logger;
pub mod console_bot;
pub mod telegram_bot;
use config::StaticBotSettings;
use console_bot::ConsoleBotState;
use telegram_bot::api_client as telegram_client;
pub use telegram_bot::TelegramBotError;
use telegram_bot::TelegramBotState;

pub fn run_console_bot(conf: &StaticBotSettings) {
    let mut init_state = ConsoleBotState::new();
    io::stdin().lines().for_each(|elem| {
        let input = elem.expect("Failed to read user input");
        let response = console_bot::respond_to_user(&conf, &mut init_state, input);
        println!("{response}");
    });
}

pub fn run_telegram_bot(
    bot_token: &String,
    conf: &StaticBotSettings,
) -> Result<(), TelegramBotError> {
    let mut bot_state = TelegramBotState::new();
    let delay = time::Duration::from_millis(100);
    let client = telegram_client::HttpIO;

    loop {
        telegram_bot::one_communication_cycle(bot_token, conf, &client, &mut bot_state)?;
        thread::sleep(delay);
    }
}
