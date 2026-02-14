// src/main.rs

mod cli;
mod config;
mod commands;
mod content;
mod context;
mod utils;

use std::env;
use utils::output;

fn main() {
    let args_vec: Vec<String> = env::args().collect();
    let excutable_name = args_vec[0].clone();

    if let Err(e) = cli::dispatch::run(args_vec) {
        output::error(&format!("{}", e));

        if matches!(e, cli::dispatch::CliError::ParseError(_)) {
            output::info(&format!("Run '{} help' to see available commands.", excutable_name));
        }

        std::process::exit(1);
    }
}