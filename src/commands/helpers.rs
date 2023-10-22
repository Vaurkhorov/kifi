#[cfg(target_os = "windows")]
const DIR_SEPARATOR: char = '\\';
#[cfg(not(target_os = "windows"))]
const DIR_SEPARATOR: char = '/';

use super::metafiles::Snapshot;
use crate::commands::{FileCache, KIFI_FILECACHE};
use crate::Error;
use serde_cbor::to_writer;
use std::format;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

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

pub fn snap_file(file_name: &String, snap_dir: &String) -> Result<(), Error> {
    fs::create_dir_all(snap_dir).map_err(Error::CreateDirectory)?;

    let mut destination_dir = PathBuf::from(file_name);
    destination_dir = destination_dir
        .parent()
        .expect("a path's parent directory should always be atleast './'.")
        .to_path_buf();
    fs::create_dir_all(destination_dir).map_err(Error::CreateDirectory)?;

    match fs::copy(
        file_name,
        format!("{}{}{}", snap_dir, DIR_SEPARATOR, file_name),
    ) {
        Ok(_) => Ok(()),
        Err(io_error) => {
            println!("{:#?}", file_name);
            Err(Error::FileCopy(io_error))
        }
    }
}

pub fn gen_name() -> String {
    // check username and email, if registered, here
    let user = String::from("testuser");
    // let email = String::from("test@testing.com");

    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Right now is before 1970? Check the system clock.")
        .as_secs();

    format!("{}_{}", user, current_timestamp)
}

pub fn diffs(file_name: &String, last_snapshot: &Snapshot) -> Result<(), Error> {
    let current_file = fs::read(file_name).map_err(Error::ReadFile)?;

    let snapped_file_path = ".kifi\\".to_string() + &last_snapshot.name + "\\" + file_name;
    let snapped_file = fs::read(snapped_file_path).map_err(Error::ReadFile)?;

    let changes = slice_diff_patch::lcs_diff(&snapped_file, &current_file);

    // To debug:
    #[cfg(debug_assertions)]
    println!("{:?}", &changes);

    // Might want to check if there are any differences before calling display_diffs
    display_diffs(snapped_file, changes)?;

    Ok(())
}

enum LastChange {
    Remove,
    Insert,
    Update,
    None,
}

fn display_diffs(
    mut snapped_file: Vec<u8>,
    changes: Vec<slice_diff_patch::Change<u8>>,
) -> Result<(), Error> {
    let mut last_change = LastChange::None;
    let mut plus_line: Vec<u8> = Vec::new();
    let mut minus_line: Vec<u8> = Vec::new();

    for change in changes {
        match change {
            slice_diff_patch::Change::Remove(i) => match last_change {
                LastChange::Remove => {
                    minus_line.push(snapped_file.remove(i));
                }
                x => {
                    print_changes(&x, &plus_line, &minus_line)?;

                    plus_line.clear();
                    minus_line.clear();

                    minus_line.push(snapped_file[i]);
                    snapped_file.remove(i);
                    last_change = LastChange::Remove;
                }
            },
            slice_diff_patch::Change::Insert((i, c)) => match last_change {
                LastChange::Insert => {
                    plus_line.push(c);
                    snapped_file.insert(i, c);
                }
                x => {
                    print_changes(&x, &plus_line, &minus_line)?;

                    plus_line.clear();
                    minus_line.clear();

                    plus_line.push(c);
                    snapped_file.insert(i, c);
                    last_change = LastChange::Insert;
                }
            },
            slice_diff_patch::Change::Update((i, c)) => match last_change {
                LastChange::Update => {
                    plus_line.push(c);
                    minus_line.push(snapped_file[i]);
                    snapped_file.remove(i);
                    snapped_file.insert(i, c);
                }
                x => {
                    print_changes(&x, &plus_line, &minus_line)?;

                    plus_line.clear();
                    minus_line.clear();

                    plus_line.push(c);
                    minus_line.push(snapped_file[i]);
                    snapped_file.remove(i);
                    snapped_file.insert(i, c);
                    last_change = LastChange::Update;
                }
            },
        }
    }

    print_changes(&last_change, &plus_line, &minus_line)?;

    Ok(())
}

fn print_changes(
    last_change: &LastChange,
    plus_line: &[u8],
    minus_line: &[u8],
) -> Result<(), Error> {
    match last_change {
        LastChange::Remove => {
            println!(
                "\x1B[91m- {}\x1B[0m",
                String::from_utf8(minus_line.to_owned()).map_err(Error::InvalidUTF8)?
            );
        }
        LastChange::Insert => {
            println!(
                "\x1B[32m+ {}\x1B[0m",
                String::from_utf8(plus_line.to_owned()).map_err(Error::InvalidUTF8)?
            )
        }
        LastChange::Update => {
            println!(
                "\x1B[32m+ {}\x1B[0m",
                String::from_utf8(plus_line.to_owned()).map_err(Error::InvalidUTF8)?
            );
            println!(
                "\x1B[91m- {}\x1B[0m",
                String::from_utf8(minus_line.to_owned()).map_err(Error::InvalidUTF8)?
            );
        }
        LastChange::None => (),
    };

    Ok(())
}
