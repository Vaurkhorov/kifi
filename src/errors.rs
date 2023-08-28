use std::{io::Error as ioError, ffi::OsString};
use serde_cbor;

#[derive(Debug)]
pub enum Error {
    CreateDirectoryFailed(ioError),
    CreateFileFailed(ioError),
    ReadFileFailed(ioError),
    GetCurrentDirectoryFailed(ioError),
    CBORWriterFailed(serde_cbor::Error),
    CBORReaderFailed(serde_cbor::Error),
    ConvertToStringFailed(OsString),
}

impl Error {
    pub fn handle(&self) {
        match self {
            Error::CreateDirectoryFailed(io_error) => {
                eprintln!("Failed to create directory: {:?}", io_error);
            },
            Error::CreateFileFailed(io_error) => {
                eprintln!("Failed to create file: {:?}", io_error);
            },
            Error::ReadFileFailed(io_error) => {
                eprintln!("Failed to read file: {:?}", io_error);
            },
            Error::GetCurrentDirectoryFailed(io_error) => {
                eprintln!("Failed to get details about current directory: {:?}", io_error);
            },
            Error::CBORWriterFailed(cbor_err) => {
                eprintln!("Could not write CBOR to file: {:?}", cbor_err);
            },
            Error::CBORReaderFailed(cbor_err) => {
                eprintln!("Could not read CBOR from file: {:?}", cbor_err);
            },
            Error::ConvertToStringFailed(os_string) => {
                eprintln!("Failed to convert OsString to String, possibly due to invalid Unicode: {:?}", os_string);
            },
        }
    }
}
