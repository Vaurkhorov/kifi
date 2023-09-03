use crate::commands::FileCache;
use crate::commands::KIFI_FILECACHE;
use crate::Error;
use serde_cbor::to_writer;
use std::format;
use std::fs;
use std::fs::copy;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

/// Generates a vector of files and stores it
pub fn create_file_cache() -> Result<(), Error> {
    let mut file_list = FileCache::new();

    match fs::read_dir(".").map_err(Error::GetCurrentDirectory) {
        Ok(files) => {
            get_name_from_fileentries(files, &mut file_list)?;
        }
        Err(e) => {
            return Err(e);
        }
    }

    let cache_file = fs::File::create(KIFI_FILECACHE).map_err(Error::CreateFile)?;
    to_writer(cache_file, &file_list).map_err(Error::CBORWriter)?;

    Ok(())
}

/// Loops through files and adds them to the cache vector
fn get_name_from_fileentries(files: fs::ReadDir, file_list: &mut FileCache) -> Result<(), Error> {
    for file in files {
        match file {
            Ok(f) => {
                let file_str = &f
                    .file_name()
                    .into_string()
                    .map_err(Error::ConvertToString)?;
                file_list.add_file(file_str.to_string());
            }
            Err(e) => {
                return Err(Error::ReadFile(e));
            }
        }
    }

    Ok(())
}

pub fn snap_file_if_tracked(file_name: &String, cache: &FileCache) -> Result<(), Error> {
    let snap_dir = format!(".kifi/{}", gen_name());

    fs::create_dir_all(&snap_dir).map_err(Error::CreateDirectory)?;

    if cache.has_tracked_file(file_name) {
        match copy(file_name, format!("{}/{}", &snap_dir, file_name)) {
            Ok(_) => Ok(()),
            Err(io_error) => Err(Error::FileCopy(io_error)),
        }
    } else {
        Ok(())
    }
}

fn gen_name() -> String {
    // check username and email, if registered, here
    let user = String::from("testuser");
    // let email = String::from("test@testing.com");

    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Right now is before 1970? Check the system clock.")
        .as_secs();

    format!("{}_{}", user, current_timestamp)
}

// TODO
// pub fn update_file_cache() -> Result<(), Error> {

//     Ok(())
// }
