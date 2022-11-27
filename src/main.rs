use echo_bot_rs::config::Config;
use echo_bot_rs::run_console_bot;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!(
            "\
Problem parsing configuration: {}
Expected default path for it: config/settings.local.yaml
Or --config param",
            err
        );
        process::exit(1);
    });

    run_console_bot();
}
