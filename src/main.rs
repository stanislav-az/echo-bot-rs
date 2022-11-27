use std::io;

fn main() {
    run_console_bot();
}

fn run_console_bot() {
    io::stdin().lines().for_each(|elem| {
        let input = elem.expect("Failed to read user input");
        console_bot_cycle(input);
    });
}

fn console_bot_cycle(user_input: String) {
    println!("{user_input}");
}
