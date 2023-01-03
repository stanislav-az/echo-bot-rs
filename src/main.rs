use echo_bot_rs::config::BotToRun;
use echo_bot_rs::config::Config;
use echo_bot_rs::logger::LogLevel;
use echo_bot_rs::logger::Logger;
use echo_bot_rs::run_console_bot;
use echo_bot_rs::run_telegram_bot;
use echo_bot_rs::TelegramBotError;
use std::env;
use std::process;

fn add_p(p: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + p)
}

fn do_add(f: impl Fn(i32) -> i32, arg: i32) -> i32 {
    f(arg)
}

fn do_add_pointer(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg)
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let fun = add_p(2);
    let x = do_add(fun, 23);
    let y = do_add_pointer(add_one, 41);
    println!("do_add: {x}");
    println!("do_add_pointer: {y}");
}
