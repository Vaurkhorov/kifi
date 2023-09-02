mod commands;
mod errors;

use std::todo;

use crate::errors::Error;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// initialises a repository
    Init,
    /// tracks the given file
    Track { file_name: String },
    /// takes a snapshot of tracked files
    Klick,
    #[cfg(debug_assertions)]
    /// prints contents of metadata files
    Debug,
}

fn main() {
    let cli = Cli::parse();

    let exit_status: Result<(), Error> = match &cli.command {
        Some(Commands::Init) => commands::initialise(),
        Some(Commands::Track { file_name }) => commands::track(file_name),
        Some(Commands::Klick) => commands::snapshot(),
        #[cfg(debug_assertions)]
        Some(Commands::Debug) => commands::debug_meta(),
        None => {
            todo!("implement help");
        }
    };

    match exit_status {
        Ok(_) => (),
        Err(ref e) => {
            e.handle();
        }
    }

    println!("{:?}", exit_status);
}
