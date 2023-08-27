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
    /// Prints contents of metadata files
    Debug,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Init) => {
            commands::initialise();
        },
        Some(Commands::Track {file_name}) => {
            commands::track(file_name);
        },
        Some(Commands::Debug) => {
            commands::debug_meta().expect("test");
        },
        None => {}
    }

    // Continued program logic goes here...
}
