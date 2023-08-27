use clap::Parser as ArgParser;

#[derive(ArgParser)]
#[command(
    name = "KavyDB",
    version = "0.1.0",
    about = "An in-memory key-value store"
)]

pub struct Args {
    #[arg(short, long, default_value_t = 1)]
    pub psize: usize,

    #[arg(short, long, default_value_t = true)]
    pub repl: bool,
}
