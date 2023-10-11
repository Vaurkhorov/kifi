#[cfg(target_os = "windows")]
const DIR_SEPARATOR: char = '\\';
#[cfg(not(target_os = "windows"))]
const DIR_SEPARATOR: char = '/';

use crate::commands::FileCache;
use crate::commands::KIFI_FILECACHE;
use crate::Error;
use serde_cbor::to_writer;
use std::format;
use std::fs;
use std::fs::copy;
use std::fs::DirEntry;
use std::path::PathBuf;
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
                read_direntry(f, file_list)?;
            }
            Err(e) => {
                return Err(Error::ReadFile(e));
            }
        }
    }

    Ok(())
}

fn read_direntry(f: DirEntry, file_list: &mut FileCache) -> Result<(), Error> {
    if f.file_type().map_err(Error::ReadFile)?.is_dir() {
        match fs::read_dir(f.path()).map_err(Error::GetCurrentDirectory) {
            Ok(files) => {
                get_name_from_fileentries(files, file_list)?;
            }
            Err(e) => {
                return Err(e);
            }
        }
    } else {
        let file_str = &f
            .path()
            .into_os_string()
            .into_string()
            .map_err(Error::ConvertToString)?;
        file_list.add_file(file_str.to_owned());
    }

    Ok(())
}

pub fn snap_file(file_name: &String) -> Result<(), Error> {
    let snap_dir = format!(".kifi{}{}", DIR_SEPARATOR, gen_name());
    fs::create_dir_all(&snap_dir).map_err(Error::CreateDirectory)?;

    let mut destination_dir = PathBuf::from(file_name);
    destination_dir = destination_dir
        .parent()
        .expect("a path's parent directory should always be atleast './'.")
        .to_path_buf();
    fs::create_dir_all(destination_dir).map_err(Error::CreateDirectory)?;

    match copy(
        file_name,
        format!("{}{}{}", &snap_dir, DIR_SEPARATOR, "main.go"),
    ) {
        Ok(_) => Ok(()),
        Err(io_error) => {
            println!("{:#?}", file_name);
            Err(Error::FileCopy(io_error))
        }
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
