mod args;
mod db;
mod kv;
mod query;

use args::Args;
use clap::Parser;
use kv::{Opts, Store};
use std::sync::Arc;

fn main() {
    let args = Args::parse();

    let store = match Store::new(Opts {
        num_partitions: Some(args.psize),
    }) {
        Ok(store) => Arc::new(store),
        Err(error) => {
            println!("failed to create store: {}", error);
            return;
        }
    };

    let engine = kv::Engine::new(store.clone());
    if args.repl {
        db::run_repl(&args, &engine);
    } else {
        println!("No mode selected. Exiting...");
    }
}
