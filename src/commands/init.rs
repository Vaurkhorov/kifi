use crate::commands::{FileCache, KIFI_FILECACHE};
use crate::Error;
use serde_cbor::to_writer;
use std::fs;

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

fn read_direntry(f: fs::DirEntry, file_list: &mut FileCache) -> Result<(), Error> {
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
