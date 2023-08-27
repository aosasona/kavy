use crate::{args::Args, kv::Store, query::Parser};

use std::io::{self, Write};

pub fn run_repl(_: &Args, store: &Store) {
    println!("Welcome to KavyDB! All your data are belong to us!");

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
        let _ = match parser.parse() {
            Ok(cmd) => cmd,
            Err(error) => {
                println!("\x1b[31mParser error: {}\x1b[0m", error);
                continue;
            }
        };
    }
}
