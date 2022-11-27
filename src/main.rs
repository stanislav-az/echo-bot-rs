use std::io;

fn main() {
    run_console_bot();
}

fn run_console_bot() {
    loop {
        console_bot_cycle()
    }
}

fn console_bot_cycle() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    println!("{input}")
}
