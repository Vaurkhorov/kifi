use fs_extra::error::Error as dirError;
use std::io::Error as ioError;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug)]
pub enum Error {
    KifiNotInitialised,
    Canonicalize(ioError),
    CreateDirectory(ioError),
    CreateFile(ioError),
    ReadFile(ioError),
    GetCurrentDirectory(ioError),
    CBORWriter(serde_cbor::Error),
    CBORReader(serde_cbor::Error),
    FileCopy(PathBuf, PathBuf, ioError),
    DirCopy(PathBuf, PathBuf, dirError),
    FileNotFoundInCache(PathBuf), // String is the path to the file
    ReservedFilenameNotAvailable(PathBuf),
    PreviewWithoutSnapshots,
    InvalidEmail,
    InvalidConfigDir,
    UserNotRegistered,
    TrackIgnoredFile(PathBuf),
    InvalidTime(SystemTime),
}

impl Error {
    pub fn handle(&self) {
        match self {
            Error::KifiNotInitialised => {
                eprintln!("No repository accessible at the current working directory.");
                eprintln!("Run `kifi init` to initialise the repository.");
            }
            Error::Canonicalize(io_error) => {
                eprintln!("Could not canonicalize path: {:?}", io_error);
            }
            Error::CreateDirectory(io_error) => {
                eprintln!("Failed to create directory: {:?}", io_error);
            }
            Error::CreateFile(io_error) => {
                eprintln!("Failed to create file: {:?}", io_error);
            }
            Error::ReadFile(io_error) => {
                eprintln!("Failed to read file: {:?}", io_error);
            }
            Error::GetCurrentDirectory(io_error) => {
                eprintln!(
                    "Failed to get details about current directory: {:?}",
                    io_error
                );
            }
            Error::CBORWriter(cbor_err) => {
                eprintln!("Could not write CBOR to file: {:?}", cbor_err);
            }
            Error::CBORReader(cbor_err) => {
                eprintln!("Could not read CBOR from file: {:?}", cbor_err);
            }
            Error::FileCopy(from, to, io_error) => {
                eprintln!(
                    "Failed to copy {} to {}: {:?}",
                    from.display(),
                    to.display(),
                    io_error
                );
            }
            Error::DirCopy(from, to, dir_error) => {
                eprintln!(
                    "Failed to copy {} to {}: {:?}",
                    from.display(),
                    to.display(),
                    dir_error
                );
            }
            Error::FileNotFoundInCache(file_path) => {
                eprintln!("File not found in cache: {}", file_path.display());
            }
            Error::ReservedFilenameNotAvailable(file_name) => {
                eprintln!("{:?} is a reserved file name, and isn't available. Is there a directory with the same name?", file_name);
            }
            Error::PreviewWithoutSnapshots => {
                eprintln!("No previous snapshots exist to preview from.");
            }
            Error::InvalidEmail => {
                eprintln!("Invalid email provided.");
            }
            Error::InvalidConfigDir => {
                eprintln!("Could not fetch config directory.");
            }
            Error::UserNotRegistered => {
                eprintln!("No registered user was found.");
                eprintln!("Use `kifi register` to register the current user.");
            }
            Error::TrackIgnoredFile(file) => {
                eprintln!("File {:?} has been ignored.", file);
                eprintln!("Use -f to force tracking.");
            }
            Error::InvalidTime(time) => {
                eprintln!("Could not parse time {:?}.", time);
            }
        }
    }
}
