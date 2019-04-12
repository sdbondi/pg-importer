#[macro_use]
extern crate derive_error;

mod cli;
mod commands;
mod dump_reader;
mod types;

fn main() {
    cli::run_cli();
}
