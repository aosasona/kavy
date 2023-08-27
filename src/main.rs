mod args;
mod kv;
mod query;
mod repl;

use args::Args;
use clap::Parser as _; // for side-effects
use kv::{Opts, Store};

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
        repl::run_repl(&args, &store);
    } else {
        println!("No mode selected. Exiting...");
    }
}
