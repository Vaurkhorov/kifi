use dirs::config_local_dir;
use serde_cbor::from_reader;

use crate::Error;
use std::fs;
use super::metafiles::User;

/// Checks if a repository already exists in the current working directory
pub fn kifi_exists() -> Result<(), Error> {
    match fs::read_dir("./.kifi") {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::KifiNotInitialised),
    }
}

/// Get user data
pub fn get_user() -> Result<User, Error> {
    let mut config = config_local_dir().ok_or_else(|| Error::InvalidConfigDir)?;
    config.push("kifi");
    config.push(".kificonfig");
    let config_file = fs::read(config).map_err(|_| Error::UserNotRegistered)?;
    Ok(from_reader(&config_file[..]).map_err(Error::CBORReader)?)
}