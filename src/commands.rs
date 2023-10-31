mod common;
mod init;
mod metafiles;
mod preview;
mod snapshot;

#[cfg(target_os = "windows")]
const DIR_SEPARATOR: char = '\\';
#[cfg(not(target_os = "windows"))]
const DIR_SEPARATOR: char = '/';
/// Directory containing metadata
const KIFI_DIR: &str = ".kifi";
/// File containing metadata about the repository itself
const KIFI_META: &str = ".kifi/META.kifi";
/// File containing paths of currently tracked files
const KIFI_TRACKED: &str = ".kifi/TRACKED.kifi";
/// File containing metadata about individual commits
const KIFI_SNAPS: &str = ".kifi/SNAPSHOTS.kifi";
/// File containing paths of all files in the repo's root directory, tracked or otherwise
const KIFI_FILECACHE: &str = ".kifi/FILECACHE.kifi";

use crate::commands::common::kifi_exists;
use crate::commands::init::update_file_cache;
use crate::commands::preview::{generate_diffs, read_lines};
use crate::commands::snapshot::{gen_name, snap_file};
use crate::errors::Error;
use crate::output::Output;
use metafiles::{FileCache, FileStatus, Metadata, Snapshots};
use serde_cbor::{from_reader, to_writer};
use std::env::current_dir;
use std::fs;

/// Initialises a kifi repo
pub fn initialise() -> Result<(), Error> {
    fs::create_dir(KIFI_DIR).map_err(Error::CreateDirectory)?;
    let metadata_file = fs::File::create(KIFI_META).map_err(Error::CreateFile)?;
    fs::File::create(KIFI_TRACKED).map_err(Error::CreateFile)?;

    let snapshots_file = fs::File::create(KIFI_SNAPS).map_err(Error::CreateFile)?;
    to_writer(snapshots_file, &Snapshots::new()).map_err(Error::CBORWriter)?;

    let current_directory_path = current_dir().map_err(Error::GetCurrentDirectory)?;
    let metadata = Metadata::from_pathbuf(current_directory_path);

    to_writer(metadata_file, &metadata).map_err(Error::CBORWriter)?;

    update_file_cache()
}

#[cfg(debug_assertions)]
/// Outputs contents of files from the .kifi directory
pub fn debug_meta(output: &mut dyn Output) -> Result<(), Error> {
    kifi_exists()?;

    let metadata_file = fs::read(KIFI_META).map_err(Error::ReadFile)?;
    let cache_file = fs::read(KIFI_FILECACHE).map_err(Error::ReadFile)?;

    let metadata: Metadata = from_reader(&metadata_file[..]).map_err(Error::CBORReader)?;
    let cache: FileCache = from_reader(&cache_file[..]).map_err(Error::CBORReader)?;

    output.add(format!("{:?}", metadata));

    output.add_str("FileCache {{");

    output.add_str("\tfiles: {{");
    for file in cache.get_keys() {
        output.add(format!("\t\t{}", file));
        let status = cache.get_status(file).expect("Keys were fetched from the cache and immediately used, so the corresponding value should exist.");
        output.add(format!("\t\t\tStatus: {:?}", status));
        output.add_str("");
    }
    output.add_str("\t}}");
    output.add_str("}}");

    Ok(())
}

/// Changes status of file to FileStatus::Tracked, see `metafiles`
pub fn track(file_name: &String, output: &mut dyn Output) -> Result<(), Error> {
    kifi_exists()?;
    update_file_cache()?;

    let file_path = format!(".{}{}", DIR_SEPARATOR, file_name);

    let cache_file = fs::read(KIFI_FILECACHE).map_err(Error::ReadFile)?;
    let mut cache: FileCache = from_reader(&cache_file[..]).map_err(Error::CBORReader)?;

    match cache.change_status(&file_path, FileStatus::Tracked) {
        Ok(()) => {
            output.add(format!("Tracking {}", file_path));
        }
        Err(e) => {
            return Err(e);
        }
    };

    let cache_file = fs::File::create(KIFI_FILECACHE).map_err(Error::CreateFile)?;
    to_writer(cache_file, &cache).map_err(Error::CBORWriter)?;

    Ok(())
}

/// Shows diffs
pub fn preview(output: &mut dyn Output) -> Result<(), Error> {
    kifi_exists()?;
    update_file_cache()?;

    let cache_file = fs::read(KIFI_FILECACHE).map_err(Error::ReadFile)?;
    let cache: FileCache = from_reader(&cache_file[..]).map_err(Error::CBORReader)?;

    let snapshots_file = fs::read(KIFI_SNAPS).map_err(Error::ReadFile)?;
    let snapshots: Snapshots = from_reader(&snapshots_file[..]).map_err(Error::CBORReader)?;

    let last_snapshot = snapshots.get_last()?;

    for file in cache.get_keys() {
        if let FileStatus::Tracked = cache.get_status(file).expect("Keys were fetched from the cache and immediately used, so the corresponding value should exist.") {
            output.add(file.to_string());

            let current_file = match read_lines(file) {
                Ok(v) => v,
                Err(_) => Vec::new(),
            };
        
            let snapped_file_path = ".kifi\\".to_string() + &last_snapshot.name + "\\" + file;
            let snapped_file = match read_lines(&snapped_file_path) {
                Ok(v) => v,
                Err(_) => Vec::new(),
            };
            
            generate_diffs(snapped_file, current_file, output)?;
        }
    }

    Ok(())
}

/// Takes a snapshot
pub fn snapshot() -> Result<(), Error> {
    kifi_exists()?;
    update_file_cache()?;

    let cache_file = fs::read(KIFI_FILECACHE).map_err(Error::ReadFile)?;
    let cache: FileCache = from_reader(&cache_file[..]).map_err(Error::CBORReader)?;

    let snapshots_file = fs::read(KIFI_SNAPS).map_err(Error::ReadFile)?;
    let mut snapshots: Snapshots = from_reader(&snapshots_file[..]).map_err(Error::CBORReader)?;

    let snap_name = gen_name();
    let snap_dir = format!(".kifi{}{}", DIR_SEPARATOR, snap_name);
    snapshots.new_snap(&snap_name);

    for file in cache.get_tracked_files() {
        snap_file(file, &snap_dir)?;
    }

    let snapshots_file = fs::File::create(KIFI_SNAPS).map_err(Error::CreateFile)?;
    to_writer(snapshots_file, &snapshots).map_err(Error::CBORWriter)?;

    Ok(())
}
