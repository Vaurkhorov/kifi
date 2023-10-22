use std::{ffi::OsString, io::Error as ioError, string::FromUtf8Error};

#[derive(Debug)]
pub enum Error {
    CreateDirectory(ioError),
    CreateFile(ioError),
    ReadFile(ioError),
    GetCurrentDirectory(ioError),
    CBORWriter(serde_cbor::Error),
    CBORReader(serde_cbor::Error),
    ConvertToString(OsString),
    FileCopy(ioError),
    InvalidUTF8(FromUtf8Error),
    FileNotFoundInCache(String), // &str is the path to the file
}

impl Error {
    pub fn handle(&self) {
        match self {
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
            Error::FileCopy(io_error) => {
                eprintln!("Failed to copy file: {:?}", io_error);
            }
            Error::InvalidUTF8(utf8_error) => {
                eprintln!(
                    "File being previewed has invalid UTF-8, which is currently unsupported: {:?}",
                    utf8_error
                )
            }
            Error::FileNotFoundInCache(file_path) => {
                eprintln!("File not found in cache: {:?}", file_path)
            }
        }
    }
}
