mod commands;

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
    Track {
        file_name: String,
    },
    #[cfg(debug_assertions)]
    /// prints contents of metadata files
    Debug,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init) => {
            commands::initialise();
        },
        Some(Commands::Track {file_name}) => {
            commands::track(file_name);
        },
        #[cfg(debug_assertions)]
        Some(Commands::Debug) => {
            commands::debug_meta().expect("test");
        },
        None => {}
    }
}
