#[cfg(target_os = "windows")]
const DIR_SEPARATOR: char = '\\';
#[cfg(not(target_os = "windows"))]
const DIR_SEPARATOR: char = '/';

use crate::Error;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

    pub fn add_file(&mut self, file_path: String) {
        if !self.files.contains_key(&file_path) {
            self.files.insert(
                file_path,
                RepoFile {
                    status: FileStatus::Untracked,
                },
            );
        }
    }

    pub fn add_file_from_existing(&mut self, file_path: String, old_file_status: FileStatus) {
        self.files.insert(file_path, RepoFile { status: old_file_status });
    }

    pub fn get_keys(&self) -> Vec<&String> {
        self.files.keys().collect()
    }

    pub fn get_status(&self, key: &String) -> Option<&FileStatus> {
        match self.files.get(key) {
            Some(repo_file) => Some(&repo_file.status),
            None => None,
        }
    }

    pub fn change_status(&mut self, file_path: &String, status: FileStatus) -> Result<(), Error> {
        // TODO update cache
        match self.files.contains_key(file_path) {
            true => {
                self.files
                    .insert(file_path.to_string(), RepoFile { status });
                Ok(())
            }
            false => Err(Error::FileNotFoundInCache(file_path.clone())),
        }
    }

    pub fn get_tracked_files(&self) -> Vec<&String> {
        let mut files = self.get_keys();
        files.retain(|&k| self.has_tracked_file(k));
        files
    }

    pub fn has_tracked_file(&self, file_path: &String) -> bool {
        match self.files.get(file_path) {
            Some(tracked) => match tracked.status {
                FileStatus::Ignored => false,
                FileStatus::Untracked => false,
                FileStatus::Tracked => true,
            },
            None => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// Stores a list of snapshots
pub struct Snapshots {
    list: Vec<Snapshot>,
}

impl Snapshots {
    pub fn new() -> Snapshots {
        Snapshots { list: Vec::new() }
    }

    pub fn new_snap(&mut self, name: &String) {
        let snap = Snapshot::new(name);
        self.list.insert(0, snap);
    }

    pub fn get_last(&self) -> Result<&Snapshot, Error> {
        self.list.get(0).ok_or_else(|| Error::PreviewWithoutSnapshots)
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// Stores data about individual snapshots
pub struct Snapshot {
    pub name: String,
    pub created: SystemTime,
}

impl Snapshot {
    fn new(name: &String) -> Snapshot {
        Snapshot {
            name: name.to_owned(),
            created: { SystemTime::now() },
        }
    }
}
