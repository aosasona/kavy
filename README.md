# Kavy

An in-memory key-value store with a simple Redis-like query language.

## Usage

You can either build and run the REPL like this:

```bash
cargo build

./target/debug/kavy
```

You can also run it directly with Cargo:

```bash
cargo run
```

## Should I use this?

Probably not, this is mostly to learn Rust

## Might add

- [ ] WAL journal to reconstruct in-memory storage on startup
- [ ] HTTP server (why not?)
