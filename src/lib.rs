use std::io;

pub mod console_bot;
pub mod config;
use console_bot::console_bot_cycle;

pub fn run_console_bot() {
    io::stdin().lines().for_each(|elem| {
        let input = elem.expect("Failed to read user input");
        console_bot_cycle(input);
    });
}
