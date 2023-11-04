use crate::commands::{get_kifi, FileCache};
use crate::Error;
use serde_cbor::{from_reader, to_writer};
use std::fs;
use std::path::PathBuf;

/// Generates a vector of files and stores it
pub fn update_file_cache() -> Result<(), Error> {
    let path = get_kifi()?;
    let old_file_list = match fs::metadata(path.filecache()) {
        Ok(metadata) => {
            if metadata.is_file() {
                let existing_cache_file = fs::read(path.filecache()).map_err(Error::ReadFile)?;
                from_reader(&existing_cache_file[..]).map_err(Error::CBORReader)?
            } else {
                return Err(Error::ReservedFilenameNotAvailable(path.filecache()));
            }
        }
        Err(_) => FileCache::new(),
    };

    let mut file_list = FileCache::new();

    match fs::read_dir(path.root()).map_err(Error::GetCurrentDirectory) {
        Ok(files) => {
            get_name_from_fileentries(files, &mut file_list, &old_file_list, &path.root())?;
        }
        Err(e) => {
            return Err(e);
        }
    }

    let cache_file = fs::File::create(path.filecache()).map_err(Error::CreateFile)?;
    to_writer(cache_file, &file_list).map_err(Error::CBORWriter)?;

    Ok(())
}

/// Loops through files and adds them to the cache vector
fn get_name_from_fileentries(
    files: fs::ReadDir,
    file_list: &mut FileCache,
    old_file_list: &FileCache,
    root: &PathBuf,
) -> Result<(), Error> {
    for file in files {
        match file {
            Ok(f) => {
                read_direntry(f, file_list, old_file_list, root)?;
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
    root: &PathBuf,
) -> Result<(), Error> {
    if f.file_type().map_err(Error::ReadFile)?.is_dir() {
        match fs::read_dir(f.path()).map_err(Error::GetCurrentDirectory) {
            Ok(files) => {
                get_name_from_fileentries(files, file_list, old_file_list, root)?;
            }
            Err(e) => {
                return Err(e);
            }
        }
    } else {
        // This step turns the paths from absolute to relative to the root
        let file_path = &f
            .path()
            .strip_prefix(root)
            .expect("Files checked here must be contained within root")
            .to_owned();

        if old_file_list.get_keys().contains(&file_path) {
            file_list.add_file_from_existing(file_path.to_owned(), old_file_list.get_status(file_path).expect("Keys were fetched from the cache and immediately used, so the corresponding value should exist.").to_owned())
        } else {
            file_list.add_file(file_path.to_owned());
        }
    }

    Ok(())
}
