use crate::commands::{get_kifi, FileCache};
use crate::Error;
use glob::Pattern;
use serde_cbor::{from_reader, to_writer};
use std::fs;
use std::path::{Path, PathBuf};

use super::common::get_user;

/// Generates a vector of files and stores it
pub fn update_file_cache(provided_path: Option<PathBuf>) -> Result<(), Error> {
    let path = get_kifi(&provided_path)?;
    let kignore = get_kignore(path.root());

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
            for file in files {
                match file {
                    Ok(f) => get_name_from_fileentries(
                        f,
                        &mut file_list,
                        &old_file_list,
                        &path.root(),
                        &kignore,
                    )?,
                    Err(e) => return Err(Error::ReadFile(e)),
                }
            }
        }
        Err(e) => {
            return Err(e);
        }
    }

    let cache_file = fs::File::create(path.filecache()).map_err(Error::CreateFile)?;
    to_writer(cache_file, &file_list).map_err(Error::CBORWriter)?;

    Ok(())
}

/// Get ignore patterns
fn get_kignore(root: PathBuf) -> Vec<Pattern> {
    let mut kignore: Vec<Pattern> = Vec::new();

    if let Ok(local_patterns) = fs::read(root.join(".kignore")) {
        let local_patterns_cow = String::from_utf8_lossy(&local_patterns[..]);
        let local_patterns_strings = local_patterns_cow.lines().map(String::from);
        let local_glob_result = local_patterns_strings.map(|s| Pattern::new(&s));
        let mut local_glob_patterns = local_glob_result.filter_map(|p| p.ok()).collect();

        kignore.append(&mut local_glob_patterns);
    }

    if let Ok(user) = get_user() {
        if let Some(global_kignore) = user.kignore() {
            if let Ok(global_patterns) = fs::read(global_kignore).map_err(Error::ReadFile) {
                let global_patterns_cow = String::from_utf8_lossy(&global_patterns[..]);
                let global_patterns_strings = global_patterns_cow.lines().map(String::from);
                let global_glob_result = global_patterns_strings.map(|s| Pattern::new(&s));
                let mut global_glob_patterns = global_glob_result.filter_map(|p| p.ok()).collect();

                kignore.append(&mut global_glob_patterns);
            }
        }
    }

    kignore.push(Pattern::new(".kifi/*").expect("This should be a valid pattern."));

    kignore
}

/// Loops through files and adds them to the cache vector
fn get_name_from_fileentries(
    file: fs::DirEntry,
    file_list: &mut FileCache,
    old_file_list: &FileCache,
    root: &PathBuf,
    kignore: &Vec<Pattern>,
) -> Result<(), Error> {
    if file.file_type().map_err(Error::ReadFile)?.is_dir() {
        match fs::read_dir(file.path()).map_err(Error::GetCurrentDirectory) {
            Ok(files) => {
                for file in files {
                    match file {
                        Ok(f) => {
                            get_name_from_fileentries(f, file_list, old_file_list, root, kignore)?
                        }
                        Err(e) => return Err(Error::ReadFile(e)),
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    } else {
        // This step turns the paths from absolute to relative to the root
        let file_path = &file
            .path()
            .strip_prefix(root)
            .expect("Files checked here must be contained within root")
            .to_owned();

        if old_file_list.get_keys().contains(&file_path) {
            file_list.add_file_from_existing(file_path.to_owned(), old_file_list.get_status(file_path).expect("Keys were fetched from the cache and immediately used, so the corresponding value should exist.").to_owned());
        } else if file_is_ignored(file_path, kignore) {
            file_list.add_file(file_path.to_owned(), super::metafiles::FileStatus::Ignored);
        } else {
            file_list.add_file(
                file_path.to_owned(),
                super::metafiles::FileStatus::Untracked,
            );
        }
    }

    Ok(())
}

fn file_is_ignored(file: &Path, kignore: &Vec<Pattern>) -> bool {
    for pattern in kignore {
        if pattern.matches_path(file) {
            return true;
        }
    }

    false
}
