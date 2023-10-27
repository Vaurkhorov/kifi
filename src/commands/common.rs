use crate::Error;
use std::fs;

/// Checks if a repository already exists in the current working directory
pub fn kifi_exists() -> Result<(), Error> {
    match fs::read_dir("./.kifi") {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::KifiNotInitialised),
    }
}
