use std::io::Write;
use std::env::current_dir;
use std::fs;

// use std::path::PathBuf;
use clap::{Parser, Subcommand};


const KIFI_DIR: &str = ".kifi";
const KIFI_META: &str = ".kifi/META.kifi";

#[cfg(target_os = "windows")]
const DIR_SEPARATOR: char = '\\';
#[cfg(not(target_os = "windows"))]
const DIR_SEPARATOR: char = '/';



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
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Init) => {
            fs::create_dir_all(KIFI_DIR).unwrap();
            let mut metadata = fs::File::create(KIFI_META).unwrap();

            let current_directory_path = current_dir().expect("Could not get current directory.");
            let current_directory_path_str = current_directory_path.to_str().expect("Could not convert current directory pathbuf to &str");

            let current_directory_index = match current_directory_path_str.rfind(DIR_SEPARATOR) {
                Some(i) => i + 1,
                None => 0,
            };

            let dir_name = &current_directory_path_str[current_directory_index..];
            let current_directory_bytes = dir_name.as_bytes();

            match metadata.write(current_directory_bytes) {
                Ok(n) => {
                    println!("{} bytes written.", n);
                }
                Err(e) => panic!("{:?}", e),
            }
        }
        None => {}
    }



    // Continued program logic goes here...
}
