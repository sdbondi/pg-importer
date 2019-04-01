#[macro_use]
extern crate derive_error;

mod cli;
mod commands;
mod types;

fn main() {
    cli::run_cli();
}
