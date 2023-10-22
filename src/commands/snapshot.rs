#[cfg(target_os = "windows")]
const DIR_SEPARATOR: char = '\\';
#[cfg(not(target_os = "windows"))]
const DIR_SEPARATOR: char = '/';

use crate::Error;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn snap_file(file_name: &String, snap_dir: &String) -> Result<(), Error> {
    fs::create_dir_all(snap_dir).map_err(Error::CreateDirectory)?;

    let mut destination_dir = PathBuf::from(file_name);
    destination_dir = destination_dir
        .parent()
        .expect("a path's parent directory should always be atleast './'.")
        .to_path_buf();
    fs::create_dir_all(destination_dir).map_err(Error::CreateDirectory)?;

    match fs::copy(
        file_name,
        format!("{}{}{}", snap_dir, DIR_SEPARATOR, file_name),
    ) {
        Ok(_) => Ok(()),
        Err(io_error) => {
            println!("{:#?}", file_name);
            Err(Error::FileCopy(io_error))
        }
    }
}

pub fn gen_name() -> String {
    // check username and email, if registered, here
    let user = String::from("testuser");
    // let email = String::from("test@testing.com");

    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Right now is before 1970? Check the system clock.")
        .as_secs();

    format!("{}_{}", user, current_timestamp)
}
