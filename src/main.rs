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
    Track {
        file_name: String,
        #[arg(short = 'f')]
        /// force tracking ignored files
        forced: bool,
    },
    /// shows diffs from the last snapshot
    Preview,
    /// takes a snapshot of tracked files
    Klick,
    /// shows previous snapshots
    Log,
    /// reverts to a specific snapshot
    Revert { name: String },
    #[cfg(debug_assertions)]
    /// prints contents of metadata files
    Debug,
    /// registers user name and email
    Register { username: String, email: String },
}

fn main() {
    let cli = Cli::parse();

    let mut output = ConsoleOutput::new();

    let exit_status: Result<(), Error> = match &cli.command {
        Some(Commands::Init) => commands::initialise(&mut output, None),
        Some(Commands::Track { file_name, forced }) => {
            commands::track(file_name, forced, &mut output, None)
        }
        Some(Commands::Preview) => commands::preview(&mut output, None),
        Some(Commands::Klick) => commands::snapshot(None),
        Some(Commands::Log) => commands::log(&mut output, None),
        Some(Commands::Revert { name }) => commands::revert(&mut output, name.to_owned(), None),
        #[cfg(debug_assertions)]
        Some(Commands::Debug) => commands::debug_meta(&mut output, None),
        Some(Commands::Register { username, email }) => commands::register(username, email),
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
