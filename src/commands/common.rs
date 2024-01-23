use super::metafiles::User;
use crate::commands::metafiles::Paths;
use crate::errors::Error;
use dirs::config_local_dir;
use serde_cbor::from_reader;
use std::{fs, path::PathBuf};

/// Checks if a repository already exists in the current working directory
pub fn get_kifi(provided_path: &Option<PathBuf>) -> Result<Paths, Error> {
    let provided_path = match provided_path {
        Some(p) => {
            if p.ends_with(".kifi") {
                p.clone()
            } else {
                p.join(".kifi")
            }
        }
        None => PathBuf::from("./.kifi"),
    };

    if fs::read_dir(&provided_path).is_ok() {
        return Paths::from_path_buf(
            provided_path
                .parent()
                .expect("'.kifi' was joined, and should be able to be removed here.")
                .to_path_buf(),
        );
    }

    let mut path =
        fs::canonicalize(&provided_path).map_err(|e| Error::Canonicalize(e, provided_path))?;
    let mut new_path = path.parent();

    while new_path.is_some() {
        path = new_path
            .expect("new_path has already been checked to be a Some(...) variant.")
            .to_path_buf();
        match fs::read_dir(path.join(".kifi")) {
            Ok(_) => return Paths::from_path_buf(path),
            Err(_) => {
                new_path = path.parent();
            }
        }
    }

    Err(Error::KifiNotInitialised)
}

/// Get user data
pub fn get_user() -> Result<User, Error> {
    let mut config = config_local_dir().ok_or_else(|| Error::InvalidConfigDir)?;
    config.push("kifi");
    config.push(".kificonfig");
    let config_file = fs::read(config).map_err(|_| Error::UserNotRegistered)?;
    from_reader(&config_file[..]).map_err(Error::CBORReader)
}
