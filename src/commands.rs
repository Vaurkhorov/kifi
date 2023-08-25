const KIFI_DIR: &str = ".kifi";
const KIFI_META: &str = ".kifi/META.kifi";
const KIFI_TRACKED: &str = ".kifi/TRACKED.kifi";
const KIFI_COMMITS: &str = ".kifi/COMMITS.kifi";

#[cfg(target_os = "windows")]
const DIR_SEPARATOR: char = '\\';
#[cfg(not(target_os = "windows"))]
const DIR_SEPARATOR: char = '/';


use std::io::Write;
use std::env::current_dir;
use std::fs;


/// Initialises a kifi
pub fn initialise() {
    fs::create_dir_all(KIFI_DIR).expect("Current directory should not be read-only");

    let mut metadata =
    fs::File::create(KIFI_META).expect(".kifi should be writeable by the user");
    fs::File::create(KIFI_TRACKED).expect(".kifi should be writeable by the user");
    fs::File::create(KIFI_COMMITS).expect(".kifi should be writeable by the user");

    let current_directory_path = current_dir().expect("Could not get current directory.");
    let current_directory_path_str = current_directory_path.to_str().expect("Could not convert current directory pathbuf to &str");

    let current_directory_index = match current_directory_path_str.rfind(DIR_SEPARATOR) {
        Some(i) => i + 1,
        None => 0,
    };

    let dir_name = &current_directory_path_str[current_directory_index..];
    let current_directory_bytes = dir_name.as_bytes();

    match metadata.write(current_directory_bytes) {
        Ok(n) => {
            println!("{} bytes written.", n);
        }
        Err(e) => panic!("{:?}", e),
    }

    cache_files()
}


pub fn cache_files() {
    let mut file_list = String::new();

    if let Ok(files) = fs::read_dir(".") {
            get_name_from_fileentries(files, &mut file_list);
    }

    let mut tracked_file = fs::File::create(KIFI_TRACKED).expect(".kifi should be writeable");
    tracked_file.write(file_list.as_bytes()).expect("Could not write to file");
}

fn get_name_from_fileentries(files: fs::ReadDir, file_list: &mut String) {
    for file in files {
        match file {
            Ok(f) => {
                let file_str = &f.file_name()
                    .into_string()
                    .expect("test");

                file_list.push_str(&file_str);
                file_list.push(' ');
            },
            Err(e) => {
                panic!("Error reading directory: {:?}", e);
            },
        }
    }
}
