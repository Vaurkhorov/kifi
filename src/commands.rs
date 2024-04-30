mod common;
mod init;
mod metafiles;
mod preview;
mod snapshot;

use crate::commands::common::{get_kifi, get_user};
use crate::commands::init::update_file_cache;
use crate::commands::metafiles::Paths;
use crate::commands::preview::{generate_diffs, read_lines};
use crate::commands::snapshot::{gen_name, snap_file};
use crate::errors::Error;
use crate::output::Output;
use dirs::config_local_dir;
use fs_extra::dir;
use metafiles::{FileCache, FileStatus, Metadata, Snapshots, User};
use serde_cbor::{from_reader, to_writer};
use std::fs;
use std::path::PathBuf;

/// Initialises a kifi repo
pub fn initialise(output: &mut dyn Output, provided_path: Option<PathBuf>) -> Result<(), Error> {
    let path = match get_kifi(&provided_path) {
        Ok(path) => {
            fs::remove_dir_all(path.kifi())
                .expect(".kifi was just confirmed to exist already. kifi should have sufficient permissions to remove its contents.");
            output.add_str("Reinitialising kifi");
            path
        }
        Err(Error::KifiNotInitialised) => Paths::from_path_buf(PathBuf::from("."))?,
        Err(e) => return Err(e),
    };

    fs::create_dir(path.kifi()).map_err(Error::CreateDirectory)?;
    let metadata_file = fs::File::create(path.meta()).map_err(Error::CreateFile)?;
    fs::File::create(path.tracked()).map_err(Error::CreateFile)?;

    let snapshots_file = fs::File::create(path.snaps()).map_err(Error::CreateFile)?;
    to_writer(snapshots_file, &Snapshots::new()).map_err(Error::CBORWriter)?;

    let metadata = Metadata::from_pathbuf(path.root())?;

    to_writer(metadata_file, &metadata).map_err(Error::CBORWriter)?;

    update_file_cache(provided_path)
}

/// Shows information about a repository
pub fn meta(output: &mut dyn Output, provided_path: Option<PathBuf>) -> Result<(), Error> {
    let path = get_kifi(&provided_path)?;
    output.add(format!("Repository at {}", path.root().display()));

    let metadata_file = fs::read(path.meta()).map_err(Error::ReadFile)?;
    let cache_file = fs::read(path.filecache()).map_err(Error::ReadFile)?;

    let metadata: Metadata = from_reader(&metadata_file[..]).map_err(Error::CBORReader)?;
    let cache: FileCache = from_reader(&cache_file[..]).map_err(Error::CBORReader)?;

    output.add(format!("Name: {}", metadata.name()));
    output.add_str("");

    output.add_str("Files:");
    let files_in_cache = cache.get_keys();
    for file in files_in_cache {
        match cache.get_status(file) {
            Some(FileStatus::Tracked) => {
                output.add(format!("{}: Tracked", file.display()));
            }
            Some(FileStatus::Untracked) => {
                output.add(format!("{}: Untracked", file.display()));
            }
            _ => {}
        }
    }

    Ok(())
}

#[cfg(debug_assertions)]
/// Outputs contents of files from the .kifi directory
pub fn debug_meta(output: &mut dyn Output, provided_path: Option<PathBuf>) -> Result<(), Error> {
    let path = get_kifi(&provided_path)?;
    output.add(format!("{:?}", path.root()));

    let metadata_file = fs::read(path.meta()).map_err(Error::ReadFile)?;
    let cache_file = fs::read(path.filecache()).map_err(Error::ReadFile)?;

    let metadata: Metadata = from_reader(&metadata_file[..]).map_err(Error::CBORReader)?;
    let cache: FileCache = from_reader(&cache_file[..]).map_err(Error::CBORReader)?;

    output.add(format!("{:?}", metadata));

    output.add_str("FileCache {{");

    output.add_str("\tfiles: {{");
    for file in cache.get_keys() {
        output.add(format!("\t\t{}", file.display()));
        let status = cache.get_status(file).expect("Keys were fetched from the cache and immediately used, so the corresponding value should exist.");
        output.add(format!("\t\t\tStatus: {:?}", status));
        output.add_str("");
    }
    output.add_str("\t}}");
    output.add_str("}}");

    Ok(())
}

/// Changes status of file to FileStatus::Tracked, see `metafiles`
pub fn track(
    file_name: &String,
    forced: &bool,
    output: &mut dyn Output,
    provided_path: Option<PathBuf>,
) -> Result<(), Error> {
    let path = get_kifi(&provided_path)?;
    update_file_cache(provided_path)?;

    let file_path = PathBuf::from(file_name);

    let cache_file = fs::read(path.filecache()).map_err(Error::ReadFile)?;
    let mut cache: FileCache = from_reader(&cache_file[..]).map_err(Error::CBORReader)?;

    match cache.change_status(&file_path, FileStatus::Tracked, forced) {
        Ok(()) => {
            output.add(format!("Tracking {}", file_path.display()));
        }
        Err(e) => {
            return Err(e);
        }
    };

    let cache_file = fs::File::create(path.filecache()).map_err(Error::CreateFile)?;
    to_writer(cache_file, &cache).map_err(Error::CBORWriter)?;

    Ok(())
}

/// Shows diffs
pub fn preview(output: &mut dyn Output, provided_path: Option<PathBuf>) -> Result<(), Error> {
    let path = get_kifi(&provided_path)?;
    update_file_cache(provided_path)?;

    let cache_file = fs::read(path.filecache()).map_err(Error::ReadFile)?;
    let cache: FileCache = from_reader(&cache_file[..]).map_err(Error::CBORReader)?;

    let snapshots_file = fs::read(path.snaps()).map_err(Error::ReadFile)?;
    let snapshots: Snapshots = from_reader(&snapshots_file[..]).map_err(Error::CBORReader)?;

    let last_snapshot = snapshots.get_last()?;

    for file in cache.get_keys() {
        if let FileStatus::Tracked = cache.get_status(file).expect("Keys were fetched from the cache and immediately used, so the corresponding value should exist.") {
            output.add(file.display().to_string());
            let file = path.root().join(file);

            let current_file = match read_lines(&file) {
                Ok(v) => v,
                Err(_) => Vec::new(),
            };

            let snapped_file_path = path.kifi().join(&last_snapshot.name);
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
pub fn snapshot(provided_path: Option<PathBuf>) -> Result<(), Error> {
    let path = get_kifi(&provided_path)?;
    update_file_cache(provided_path)?;

    let cache_file = fs::read(path.filecache()).map_err(Error::ReadFile)?;
    let cache: FileCache = from_reader(&cache_file[..]).map_err(Error::CBORReader)?;

    let snapshots_file = fs::read(path.snaps()).map_err(Error::ReadFile)?;
    let mut snapshots: Snapshots = from_reader(&snapshots_file[..]).map_err(Error::CBORReader)?;

    let snap_name = gen_name()?;
    let snap_dir = PathBuf::from(".kifi").join(&snap_name);

    fs::create_dir_all(&snap_dir).map_err(Error::CreateDirectory)?;

    let user = get_user()?;
    snapshots.new_snap(&snap_name, &user);

    for file in cache.get_tracked_files() {
        snap_file(file, &snap_dir)?;
    }

    let snapshots_file = fs::File::create(path.snaps()).map_err(Error::CreateFile)?;
    to_writer(snapshots_file, &snapshots).map_err(Error::CBORWriter)?;

    Ok(())
}

/// Shows previous commits stored in Snapshots
pub fn log(output: &mut dyn Output, provided_path: Option<PathBuf>) -> Result<(), Error> {
    let path = get_kifi(&provided_path)?;

    let snapshots_file = fs::read(path.snaps()).map_err(Error::ReadFile)?;
    let snapshots: Snapshots = from_reader(&snapshots_file[..]).map_err(Error::CBORReader)?;

    for snap in snapshots.into_iter() {
        output.add(format!("snapshot {}", snap.name));
        output.add(format!(
            "Created by:\n{}\n{}",
            snap.author, snap.author_email
        ));
        output.add(format!(
            "Created on {} UTC",
            time::OffsetDateTime::from(snap.created)
                .format(
                    &time::format_description::parse(
                        "[year]-[month]-[day] [hour]:[minute]:[second]"
                    )
                    .map_err(|_| Error::InvalidTime(snap.created))?
                )
                .map_err(|_| Error::InvalidTime(snap.created))?
        ));
        output.add_str("");
    }

    Ok(())
}

/// Restore snapshot
pub fn revert(
    output: &mut dyn Output,
    name: String,
    provided_path: Option<PathBuf>,
) -> Result<(), Error> {
    let path = get_kifi(&provided_path)?;

    let snapshots_file = fs::read(path.snaps()).map_err(Error::ReadFile)?;
    let snapshots: Snapshots = from_reader(&snapshots_file[..]).map_err(Error::CBORReader)?;

    match snapshots.find(name) {
        metafiles::SearchResults::FoundExact(snapshot) => {
            let snap_dir = path.kifi().join(snapshot.name);

            for file in fs::read_dir(snap_dir).map_err(Error::GetCurrentDirectory)? {
                let file = file.map_err(Error::ReadFile)?;
                if file.metadata().map_err(Error::ReadFile)?.is_dir() {
                    dir::copy(file.path(), ".", &dir::CopyOptions::new().overwrite(true)).map_err(
                        |e| Error::DirCopy(file.path(), PathBuf::from(file.file_name()), e),
                    )?;
                } else {
                    fs::copy(file.path(), "file.file_name()").map_err(|e| {
                        Error::FileCopy(file.path(), PathBuf::from(file.file_name()), e)
                    })?;
                }
            }
        }
        metafiles::SearchResults::FoundSimilar(matching_snapshots) => {
            output.add_str("Snapshot not found. Did you mean one of these?");
            for snap in matching_snapshots {
                output.add(format!(
                    "{} | {} - {} | {}",
                    snap.name,
                    snap.author,
                    snap.author_email,
                    time::OffsetDateTime::from(snap.created)
                        .format(
                            &time::format_description::parse(
                                "[year]-[month]-[day] [hour]:[minute]:[second]"
                            )
                            .map_err(|_| Error::InvalidTime(snap.created))?
                        )
                        .map_err(|_| Error::InvalidTime(snap.created))?
                ));
            }
        }
        metafiles::SearchResults::NotFound => output.add_str("Snapshot not found."),
    }

    Ok(())
}

/// Register a user, to reflect them as the author in later commits
pub fn register(name: &String, email: &String) -> Result<(), Error> {
    let user = User::new(name, email)?;

    let mut config = config_local_dir().ok_or_else(|| Error::InvalidConfigDir)?;
    config.push("kifi");
    fs::create_dir_all(&config).map_err(Error::CreateDirectory)?;
    config.push(".kificonfig");
    let config_file = fs::File::create(config).map_err(Error::CreateFile)?;
    to_writer(config_file, &user).map_err(Error::CBORWriter)?;

    Ok(())
}
