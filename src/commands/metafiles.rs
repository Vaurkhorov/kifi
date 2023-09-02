#[cfg(target_os = "windows")]
const DIR_SEPARATOR: char = '\\';
#[cfg(not(target_os = "windows"))]
const DIR_SEPARATOR: char = '/';

use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Serialize, Deserialize)]
/// Contains information about the repository as a whole
pub struct Metadata {
    repo_name: String,
}

impl Metadata {
    pub fn from_pathbuf(value: PathBuf) -> Self {
        let path = value.to_str().expect("test");
        let name = match path.rfind(DIR_SEPARATOR) {
            Some(i) => {
                String::from_str(&path[i + 1..]).expect("test2") // Whatever is after the last '/' or '\'
            }
            None => String::from_str(path).expect("test3"),
        };

        Metadata { repo_name: name }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FileStatus {
    /// These files are not included in snapshots or previews
    Ignored,
    /// These files are not included in snapshots, but can be added. These show up in previews.
    Untracked,
    /// These files are included in snapshots, their changes will be tracked and shown during previews.
    Tracked,
}

#[derive(Debug, Serialize, Deserialize)]
struct RepoFile {
    status: FileStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileCache {
    files: HashMap<String, RepoFile>,
}

impl FileCache {
    pub fn new() -> Self {
        FileCache {
            files: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, file_name: String) {
        self.files.insert(
            file_name,
            RepoFile {
                status: FileStatus::Untracked,
            },
        );
    }

    pub fn change_status(&mut self, file: &String, status: FileStatus) {
        // TODO update cache
        match self.files.contains_key(file) {
            true => {
                self.files.insert(file.to_string(), RepoFile { status });
            }
            false => {
                println!("File {:?} not found.", file); // Replace this with an error later.
            }
        }
    }

    pub fn has_tracked_file(&self, file_name: &String) -> bool {
        match self.files.get(file_name) {
            Some(tracked) => {
                match tracked.status {
                    FileStatus::Ignored => false,
                    FileStatus::Untracked => false,
                    FileStatus::Tracked => true,
                }
            },
            None => false,
        }
    }
}
