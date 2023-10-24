use crate::Error;
use std::fs;

pub fn kifi_exists() -> Result<(), Error> {
    match fs::read_dir("./kifi") {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::KifiNotInitialised),
    }
}
