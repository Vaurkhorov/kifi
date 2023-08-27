#[cfg(target_os = "windows")]
const DIR_SEPARATOR: char = '\\';
#[cfg(not(target_os = "windows"))]
const DIR_SEPARATOR: char = '/';

use serde_derive::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    repo_name: String,
}

impl Metadata {
    pub fn from_pathbuf(value: PathBuf) -> Self {
        let path = value.to_str().expect("test"); // TODO Write a better message before replacing all the expects
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
enum FileStatus {
    Ignored,
    Untracked,
    Tracked,
}

#[derive(Debug, Serialize, Deserialize)]
struct RepoFile {
    status: FileStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileCache {
    // files: Vec<String>,
    files: HashMap<String, RepoFile>,
}

impl FileCache {
    pub fn new() -> Self {
        FileCache { files: HashMap::new() }
    }

    pub fn add_file(&mut self, file_name: String) {
        self.files.insert(
                file_name,
                RepoFile { status: FileStatus::Untracked },
            );
    }
}
