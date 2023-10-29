use std::{ffi::OsString, io::Error as ioError};

#[derive(Debug)]
pub enum Error {
    KifiNotInitialised,
    CreateDirectory(ioError),
    CreateFile(ioError),
    ReadFile(ioError),
    GetCurrentDirectory(ioError),
    CBORWriter(serde_cbor::Error),
    CBORReader(serde_cbor::Error),
    ConvertToString(OsString),
    FileCopy(String, ioError),
    FileNotFoundInCache(String), // String is the path to the file
    ReservedFilenameNotAvailable(String),
}

impl Error {
    pub fn handle(&self) {
        match self {
            Error::KifiNotInitialised => {
                eprintln!("No repository accessible at the current working directory.");
                eprintln!("Run `kifi init` to initialise the repository.");
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
            Error::ConvertToString(os_string) => {
                eprintln!(
                    "Failed to convert OsString to String, possibly due to invalid Unicode: {:?}",
                    os_string
                );
            }
            Error::FileCopy(path, io_error) => {
                eprintln!("Failed to copy {}: {:?}", path, io_error);
            }
            Error::FileNotFoundInCache(file_path) => {
                eprintln!("File not found in cache: {}", file_path);
            }
            Error::ReservedFilenameNotAvailable(file_name) => {
                eprintln!("{} is a reserved file name, and isn't available. Is there a directory with the same name?", file_name);
            },
        }
    }
}
