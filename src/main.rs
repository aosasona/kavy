mod query;

use query::Parser;
use std::io::{self, Write};

fn main() {
    println!("Welcome to KavyDB! Type in your commands below.");
    loop {
        let mut input = String::new();

        print!("> ");
        io::stdout().flush().expect("failed to flush stdout");

        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(error) => {
                println!("failed to read input: {}", error);
                break;
            }
        }

        if input.trim() == "exit" || input.trim() == "exit;" {
            break;
        }

        let mut parser = Parser::new(input);
        match parser.parse() {
            Ok(_) => {}
            Err(error) => {
                println!("\x1b[31mParser error: {}\x1b[0m", error);
            }
        }
    }
}
