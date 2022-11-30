use echo_bot_rs::config::Config;
use echo_bot_rs::run_console_bot;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_path = String::from("config/settings.local.yaml");

    let config = Config::build(&default_path, &args).unwrap_or_else(|err| {
        eprintln!(
            "\
Problem parsing configuration: {}
Expected default path for it: {}
Or --config param",
            err, default_path,
        );
        process::exit(1);
    });

    run_console_bot(config);
}
