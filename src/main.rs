mod kv;
mod query;

use clap::Parser as ArgParser;
use kv::{Opts, Store};
use query::Parser;
use std::io::{self, Write};

#[derive(ArgParser)]
#[command(
    name = "KavyDB",
    version = "0.1.0",
    about = "An in-memory key-value store"
)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    psize: usize,

    #[arg(short, long, default_value_t = true)]
    repl: bool,
}

fn main() {
    let args = Args::parse();

    let store = match Store::new(Opts {
        num_partitions: Some(args.psize),
    }) {
        Ok(store) => store,
        Err(error) => {
            println!("failed to create store: {}", error);
            return;
        }
    };

    if args.repl {
        run_repl(&args, &store);
    } else {
        println!("No mode selected. Exiting...");
    }
}

fn run_repl(args: &Args, store: &Store) {
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
        match parser.parse() {
            Ok(e) => {
                println!("{:?}", e);
            }
            Err(error) => {
                println!("\x1b[31mParser error: {}\x1b[0m", error);
            }
        }
    }
}
