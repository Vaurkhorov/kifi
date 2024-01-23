use fs_extra::error::Error as dirError;
use std::io::Error as ioError;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::output::Output;

#[derive(Debug)]
pub enum Error {
    KifiNotInitialised,
    Canonicalize(ioError, PathBuf),
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
    pub fn handle(&self, output: &mut dyn Output) {
        match self {
            Error::KifiNotInitialised => {
                output.add_str("No repository accessible at the current working directory.");
                output.add_str("Run `kifi init` to initialise the repository.");
            }
            Error::Canonicalize(io_error, path) => {
                output.add(format!("Could not canonicalize {:?}: {:?}", path, io_error));
            }
            Error::CreateDirectory(io_error) => {
                output.add(format!("Failed to create directory: {:?}", io_error));
            }
            Error::CreateFile(io_error) => {
                output.add(format!("Failed to create file: {:?}", io_error));
            }
            Error::ReadFile(io_error) => {
                output.add(format!("Failed to read file: {:?}", io_error));
            }
            Error::GetCurrentDirectory(io_error) => {
                output.add(format!(
                    "Failed to get details about current directory: {:?}",
                    io_error
                ));
            }
            Error::CBORWriter(cbor_err) => {
                output.add(format!("Could not write CBOR to file: {:?}", cbor_err));
            }
            Error::CBORReader(cbor_err) => {
                output.add(format!("Could not read CBOR from file: {:?}", cbor_err));
            }
            Error::FileCopy(from, to, io_error) => {
                output.add(format!(
                    "Failed to copy {} to {}: {:?}",
                    from.display(),
                    to.display(),
                    io_error
                ));
            }
            Error::DirCopy(from, to, dir_error) => {
                output.add(format!(
                    "Failed to copy {} to {}: {:?}",
                    from.display(),
                    to.display(),
                    dir_error
                ));
            }
            Error::FileNotFoundInCache(file_path) => {
                output.add(format!("File not found in cache: {}", file_path.display()));
            }
            Error::ReservedFilenameNotAvailable(file_name) => {
                output.add(format!("{:?} is a reserved file name, and isn't available. Is there a directory with the same name?", file_name));
            }
            Error::PreviewWithoutSnapshots => {
                output.add("No previous snapshots exist to preview from.".to_string());
            }
            Error::InvalidEmail => {
                output.add("Invalid email provided.".to_string());
            }
            Error::InvalidConfigDir => {
                output.add("Could not fetch config directory.".to_string());
            }
            Error::UserNotRegistered => {
                output.add("No registered user was found.".to_string());
                output.add("Use `kifi register` to register the current user.".to_string());
            }
            Error::TrackIgnoredFile(file) => {
                output.add(format!("File {:?} has been ignored.", file));
                output.add("Use -f to force tracking.".to_string());
            }
            Error::InvalidTime(time) => {
                output.add(format!("Could not parse time {:?}.", time));
            }
        }
    }
}
