mod metafiles;

/// Directory containing metadata
const KIFI_DIR: &str = ".kifi";
/// File containing metadata about the repository itself
const KIFI_META: &str = ".kifi/META.kifi";
/// File containing paths of currently tracked files
const KIFI_TRACKED: &str = ".kifi/TRACKED.kifi";
/// File containing metadata about individual commits
const KIFI_COMMITS: &str = ".kifi/COMMITS.kifi";
/// File containing paths of all files in the repo's root directory, tracked or otherwise
const KIFI_FILECACHE: &str = ".kifi/FILECACHE.kifi";

use crate::errors::Error;
use metafiles::{FileCache, FileStatus, Metadata};
use serde_cbor::{from_reader, to_writer};
use std::env::current_dir;
use std::fs;

/// Initialises a kifi repo
pub fn initialise() -> Result<(), Error> {
    fs::create_dir(KIFI_DIR).map_err(Error::CreateDirectory)?;
    let metadata_file =
        fs::File::create(KIFI_META).map_err(Error::CreateFile)?;
    fs::File::create(KIFI_TRACKED).map_err(Error::CreateFile)?;
    fs::File::create(KIFI_COMMITS).map_err(Error::CreateFile)?;

    let current_directory_path =
        current_dir().map_err(Error::GetCurrentDirectory)?;
    let metadata = Metadata::from_pathbuf(current_directory_path);

    to_writer(metadata_file, &metadata).map_err(Error::CBORWriter)?;

    cache_files()
}

/// Generates a vector of files and stores it
fn cache_files() -> Result<(), Error> {
    let mut file_list = FileCache::new();

    match fs::read_dir(".").map_err(Error::GetCurrentDirectory) {
        Ok(files) => {
            get_name_from_fileentries(files, &mut file_list)?;
        }
        Err(e) => {
            return Err(e);
        }
    }

    let cache_file =
        fs::File::create(KIFI_FILECACHE).map_err(Error::CreateFile)?;
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
                panic!("Error reading directory: {:?}", e);
            }
        }
    }

    Ok(())
}

#[cfg(debug_assertions)]
/// Outputs contents of files from the .kifi directory
pub fn debug_meta() -> Result<(), Error> {
    let metadata_file = fs::read(KIFI_META).map_err(Error::ReadFile)?;
    let cache_file = fs::read(KIFI_FILECACHE).map_err(Error::ReadFile)?;

    let metadata: Metadata =
        from_reader(&metadata_file[..]).map_err(Error::CBORReader)?;
    let cache: FileCache =
        from_reader(&cache_file[..]).map_err(Error::CBORReader)?;

    println!("{:?}", metadata);
    println!("{:?}", cache);

    Ok(())
}

/// Changes status of file to FileStatus::Tracked, see `metafiles`
pub fn track(file_name: &String) -> Result<(), Error> {
    let cache_file = fs::read(KIFI_FILECACHE).map_err(Error::ReadFile)?;
    let mut cache: FileCache =
        from_reader(&cache_file[..]).map_err(Error::CBORReader)?;

    cache.change_status(file_name, FileStatus::Tracked);
    println!("Tracking {:?}", file_name);

    let cache_file =
        fs::File::create(KIFI_FILECACHE).map_err(Error::CreateFile)?;
    to_writer(cache_file, &cache).map_err(Error::CBORWriter)?;

    Ok(())
}
