use crate::Error;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn snap_file(file_name: &PathBuf, snap_dir: &PathBuf) -> Result<(), Error> {
    fs::create_dir_all(snap_dir).map_err(Error::CreateDirectory)?;

    if let Some(dir) = file_name.parent() {
        fs::create_dir_all(snap_dir.join(dir)).map_err(Error::CreateDirectory)?;
    }

    match fs::copy(file_name, snap_dir.join(file_name)) {
        Ok(_) => Ok(()),
        Err(io_error) => Err(Error::FileCopy(
            file_name.to_owned(),
            snap_dir.join(file_name).to_owned(),
            io_error,
        )),
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
