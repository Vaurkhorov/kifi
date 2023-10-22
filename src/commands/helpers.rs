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
use std::io::{BufRead, BufReader};
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
    let current_file = read_lines(file_name)?;

    let snapped_file_path = ".kifi\\".to_string() + &last_snapshot.name + "\\" + file_name;
    let snapped_file = read_lines(&snapped_file_path)?;

    let changes = slice_diff_patch::lcs_diff(&snapped_file, &current_file);

    // To debug:
    #[cfg(debug_assertions)]
    println!("{:?}", &changes);

    // Might want to check if there are any differences before calling display_diffs
    display_diffs(snapped_file, changes)?;

    Ok(())
}

fn read_lines(path: &String) -> Result<Vec<String>, Error> {
    let file = fs::File::open(path).map_err(Error::ReadFile)?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        lines.push(line.map_err(Error::ReadFile)?);
    }

    Ok(lines)
}

fn display_diffs(
    mut snapped_file: Vec<String>,
    changes: Vec<slice_diff_patch::Change<String>>,
) -> Result<(), Error> {

    let mut line_numbers: Vec<usize> = (1..=snapped_file.len()).collect();

    for change in changes {
        println!();
        match change {
            slice_diff_patch::Change::Remove(index) => {
                println!("\x1B[91m- {}\t|{}\x1B[0m", line_numbers.remove(index), snapped_file.remove(index));
            }
            slice_diff_patch::Change::Insert((index, element)) => {
                println!("\x1B[32m+ {}\t|{}\x1B[0m", (&index + 1), &element);
                
                // Anything can be inserted here, this is just tracking the line number where lines exist.
                // So the index is important, not the element. 0 is just a placeholder.
                // There could be an enum instead, but there really isn't any need for it.
                line_numbers.insert(index, 0);
                snapped_file.insert(index, element);
            }
            slice_diff_patch::Change::Update((index, element)) => {
                println!(
                    "\x1B[91m- {}\t|{}\x1B[0m",
                    line_numbers
                    .get(index)
                    .expect("Diffs were just calculated, this index should exist."),
                    snapped_file
                        .get(index)
                        .expect("Diffs were just calculated, this index should exist.")
                );
                println!("\x1B[32m+ {}\t|{}\x1B[0m", (&index + 1), &element);
                
                // Setting the element to zero has no use, but it could be helpful while debugging.
                // line_numbers[index] = 0;
                snapped_file[index] = element;
            }
        }
    }

    Ok(())
}
