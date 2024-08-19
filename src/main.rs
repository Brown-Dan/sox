use clap::Parser;

use qc::{add, list, remove, rename, search, update};

use crate::models::Command;

pub mod models;

#[derive(Parser)]
struct Cli {
    instruction: Command,
    alias: Option<String>,
    command: Option<String>,
}

fn main() {
    let args = Cli::parse();

    match args.instruction {
        Command::List => list(),
        Command::Add => add(args.alias, args.command),
        Command::Remove => remove(args.alias),
        Command::Rename => rename(args.alias, args.command),
        Command::Update => update(args.alias, args.command),
        Command::Search => search(args.alias),
    }
}
