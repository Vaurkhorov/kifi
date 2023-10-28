mod commands;
mod errors;
mod output;

use crate::errors::Error;
use crate::output::ConsoleOutput;
use clap::{Parser, Subcommand};
use output::Output;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
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
    /// shows diffs from the last snapshot
    Preview,
    /// takes a snapshot of tracked files
    Klick,
    #[cfg(debug_assertions)]
    /// prints contents of metadata files
    Debug,
}

fn main() {
    let cli = Cli::parse();

    let mut output = ConsoleOutput::new();

    let exit_status: Result<(), Error> = match &cli.command {
        Some(Commands::Init) => commands::initialise(),
        Some(Commands::Track { file_name }) => commands::track(file_name, &mut output),
        Some(Commands::Preview) => commands::preview(&mut output),
        Some(Commands::Klick) => commands::snapshot(),
        #[cfg(debug_assertions)]
        Some(Commands::Debug) => commands::debug_meta(&mut output),
        None => {
            // This will not execute as long as the flag 'arg_required_else_help' is set to 'true'.
            unreachable!();
        }
    };

    match exit_status {
        Ok(()) => {
            output.print();
            0
        }
        Err(ref e) => {
            e.handle();
            1
        }
    };
}
