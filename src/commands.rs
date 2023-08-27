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


use std::{env::current_dir, todo};
use std::fs;
use metafiles::Metadata;
use serde_cbor::{to_writer, from_reader};

use self::metafiles::FileCache;

/// Initialises a kifi repo
pub fn initialise() {
    fs::create_dir_all(KIFI_DIR).expect("Current directory should not be read-only");
    let metadata_file = fs::File::create(KIFI_META).expect(".kifi should be writeable by the user");
    fs::File::create(KIFI_TRACKED).expect(".kifi should be writeable by the user");
    fs::File::create(KIFI_COMMITS).expect(".kifi should be writeable by the user");

    let current_directory_path = current_dir().expect("Could not get current directory.");
    let metadata = Metadata::from_pathbuf(current_directory_path);

    to_writer(metadata_file, &metadata).expect("failed to write to metafile");

    cache_files()
}

pub fn cache_files() {
    let mut file_list = FileCache::new();

    if let Ok(files) = fs::read_dir(".") {
        get_name_from_fileentries(files, &mut file_list);
    }

    let tracked_file = fs::File::create(KIFI_FILECACHE).expect(".kifi should be writeable");
    to_writer(tracked_file, &file_list).expect("failed to write to metafile");
}

fn get_name_from_fileentries(files: fs::ReadDir, file_list: &mut FileCache) {
    for file in files {
        match file {
            Ok(f) => {
                let file_str = &f.file_name().into_string().expect("test");
                file_list.add_file(file_str.to_string());
            }
            Err(e) => {
                panic!("Error reading directory: {:?}", e);
            }
        }
    }
}

pub fn debug_meta() -> Result<(), Box<dyn std::error::Error>> {
    let metadata_file = fs::read(KIFI_META)?;
    let cache_file = fs::read(KIFI_FILECACHE)?;

    let metadata: Metadata = from_reader(&metadata_file[..])?;
    let cache: FileCache = from_reader(&cache_file[..])?;

    println!("{:?}", metadata);
    println!("{:?}", cache);

    Ok(())
}

pub fn track(file_name: &String) {
    println!("{:?}", file_name);
    todo!();
}
