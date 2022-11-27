use echo_bot_rs::run_console_bot;
use echo_bot_rs::config::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing configuration: {err}");
        process::exit(1);
    });

    dbg!(config);
    run_console_bot();
}
