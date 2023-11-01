use crate::Error;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;
use regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
/// Contains information about the repository as a whole
pub struct Metadata {
    repo_name: String,
}

impl Metadata {
    pub fn from_pathbuf(path: PathBuf) -> Result<Self, Error> {
        let canonical_path = path.canonicalize().map_err(Error::Canonicalize)?;
        let name = canonical_path
            .file_name()
            .expect("kifi must be running in a directory, and so it should have a name.");

        Ok(Metadata {
            repo_name: name.to_string_lossy().to_string(),
        })
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
    files: HashMap<PathBuf, RepoFile>,
}

impl FileCache {
    pub fn new() -> Self {
        FileCache {
            files: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, file_path: PathBuf) {
        self.files.entry(file_path).or_insert(RepoFile {
            status: FileStatus::Untracked,
        });
    }

    pub fn add_file_from_existing(&mut self, file_path: PathBuf, old_file_status: FileStatus) {
        self.files.insert(
            file_path,
            RepoFile {
                status: old_file_status,
            },
        );
    }

    pub fn get_keys(&self) -> Vec<&PathBuf> {
        self.files.keys().collect()
    }

    pub fn get_status(&self, key: &PathBuf) -> Option<&FileStatus> {
        match self.files.get(key) {
            Some(repo_file) => Some(&repo_file.status),
            None => None,
        }
    }

    pub fn change_status(&mut self, file_path: &PathBuf, status: FileStatus) -> Result<(), Error> {
        // TODO update cache
        if self.files.contains_key(file_path) {
            self.files.insert(file_path.to_owned(), RepoFile { status });
            Ok(())
        } else {
            Err(Error::FileNotFoundInCache(file_path.clone()))
        }
    }

    pub fn get_tracked_files(&self) -> Vec<&PathBuf> {
        let mut files = self.get_keys();
        files.retain(|&k| self.has_tracked_file(k));
        files
    }

    pub fn has_tracked_file(&self, file_path: &PathBuf) -> bool {
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

    pub fn new_snap(&mut self, name: &String, user: &User) {
        let snap = Snapshot::new(name, user);
        self.list.insert(0, snap);
    }

    pub fn get_last(&self) -> Result<&Snapshot, Error> {
        self.list
            .get(0)
            .ok_or_else(|| Error::PreviewWithoutSnapshots)
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// Stores data about individual snapshots
pub struct Snapshot {
    pub name: String,
    pub author: String,
    pub author_email: String,
    pub created: SystemTime,
}

impl Snapshot {
    fn new(name: &String, user: &User) -> Snapshot {
        Snapshot {
            name: name.to_owned(),
            author: user.name().to_owned(),
            author_email: user.email().to_owned(),
            created: { SystemTime::now() },
        }
    }
}

/// Stores information about the user
#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    email: String,
}

impl User {
    pub fn new(name: &String, email: &String) -> Result<Self, Error> {
        if !Self::is_valid(email) {
            return Err(Error::InvalidEmail)
        }

        Ok(User { name: name.to_owned(), email: email.to_owned() })
    }

    fn is_valid(email: &String) -> bool {
        let re = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();
        re.is_match(email)
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn email(&self) -> &String {
        &self.email
    }
}