use echo_bot_rs::config::BotToRun;
use echo_bot_rs::config::Config;
use echo_bot_rs::logger::LogLevel;
use echo_bot_rs::logger::Logger;
use echo_bot_rs::run_console_bot;
use echo_bot_rs::run_telegram_bot;
use echo_bot_rs::TelegramBotError;
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

    let mut logger = Logger::initialize(&config.logger_settings);

    logger.log(LogLevel::Debug, "Started logger".to_string());

    match config.bot_to_run {
        BotToRun::Console => run_console_bot(&config.static_bot_options),
        BotToRun::Telegram => {
            run_telegram_bot(&config.telegram_bot_token, &config.static_bot_options).unwrap_or_else(
                |err| {
                    match err {
                        TelegramBotError::Api(e) => {
                            eprintln!("Telegram API responded with error:\n  {}", e)
                        }
                        TelegramBotError::HttpClient(e) => eprintln!("HTTP client error:\n  {}", e),
                        TelegramBotError::Serialization(e) => {
                            eprintln!("Could not (de)serialize:\n  {}", e)
                        }
                        TelegramBotError::Parsing(e) => {
                            eprintln!("Could not parse data:\n  {}", e)
                        }
                    }
                    process::exit(1);
                },
            )
        }
    }
}
