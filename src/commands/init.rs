use crate::commands::{FileCache, KIFI_FILECACHE};
use crate::Error;
use serde_cbor::{from_reader, to_writer};
use std::fs;
use std::path::PathBuf;

/// Generates a vector of files and stores it
pub fn update_file_cache() -> Result<(), Error> {
    let old_file_list = match fs::metadata(KIFI_FILECACHE) {
        Ok(metadata) => {
            if metadata.is_file() {
                let existing_cache_file = fs::read(KIFI_FILECACHE).map_err(Error::ReadFile)?;
                from_reader(&existing_cache_file[..]).map_err(Error::CBORReader)?
            } else {
                return Err(Error::ReservedFilenameNotAvailable(
                    KIFI_FILECACHE.to_string(),
                ));
            }
        }
        Err(_) => FileCache::new(),
    };

    let mut file_list = FileCache::new();

    match fs::read_dir(".").map_err(Error::GetCurrentDirectory) {
        Ok(files) => {
            get_name_from_fileentries(files, &mut file_list, &old_file_list)?;
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
fn get_name_from_fileentries(
    files: fs::ReadDir,
    file_list: &mut FileCache,
    old_file_list: &FileCache,
) -> Result<(), Error> {
    for file in files {
        match file {
            Ok(f) => {
                read_direntry(f, file_list, old_file_list)?;
            }
            Err(e) => {
                return Err(Error::ReadFile(e));
            }
        }
    }

    Ok(())
}

fn read_direntry(
    f: fs::DirEntry,
    file_list: &mut FileCache,
    old_file_list: &FileCache,
) -> Result<(), Error> {
    if f.file_type().map_err(Error::ReadFile)?.is_dir() {
        match fs::read_dir(f.path()).map_err(Error::GetCurrentDirectory) {
            Ok(files) => {
                get_name_from_fileentries(files, file_list, old_file_list)?;
            }
            Err(e) => {
                return Err(e);
            }
        }
    } else {
        // The file is going to contain './' at the beginning, which we strip.
        let file_path = &f
            .path()
            .strip_prefix(PathBuf::from("."))
            .expect("Should start with its first ancestor.")
            .to_owned();

        if old_file_list.get_keys().contains(&file_path) {
            file_list.add_file_from_existing(file_path.to_owned(), old_file_list.get_status(file_path).expect("Keys were fetched from the cache and immediately used, so the corresponding value should exist.").to_owned())
        }
        file_list.add_file(file_path.to_owned());
    }

    Ok(())
}
