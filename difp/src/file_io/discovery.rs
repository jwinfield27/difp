use std::path::{Path, PathBuf};
use std::fs::{self, ReadDir};

use super::file_tracker::FileTracker;

const IGNORED_DIRECTORIES: [&'static str; 3] = [
            "./Cargo.toml",
            "./Cargo.lock",
            "./target"
];

pub fn is_file(dir_path: &String) -> bool {
    Path::new(dir_path).is_file()
}

pub fn is_directory(dir_path: &String) -> bool {
    Path::new(dir_path).is_dir()
}

pub fn create_file_map(dir_path: &String) -> FileTracker {
    let mut file_tracker= FileTracker::new();
    find_all_files(dir_path, &mut file_tracker);
    file_tracker
}

fn find_all_files(dir_path: &String, file_tracker: &mut FileTracker) {
    if IGNORED_DIRECTORIES.contains(&dir_path.as_str()) {
        return
    }
    if is_file(dir_path){
        file_tracker.add_file(PathBuf::from(dir_path));
        return
    } else {
        match fs::read_dir(dir_path.as_str()) {
            Ok(dir) => parse_dirs(dir, file_tracker),
            Err(_) => println!("error reading {}", dir_path.as_str())
        }
    }
}

fn parse_dirs(dir: ReadDir, file_tracker: &mut FileTracker) {
    for path in dir {
        let path_str = match path {
            Ok(entry) => entry,
            Err(_) => continue
        };
        let path_str: String = path_str.path().to_str().unwrap().to_owned();
        find_all_files(&path_str, file_tracker);
    }
}

fn test_get_files_and_directories(dir_path: &String) -> ReadDir {
    let fp = fs::read_dir(dir_path.as_str());
    if fp.is_ok() {
        fp.unwrap()
    } else {
        panic!("error reading directory {}", dir_path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_directory_test() {
        let crate_file_paths: [&'static str; 4] = [
            "./Cargo.toml",
            "./Cargo.lock",
            "./src",
            "./target"
        ];
        let res: ReadDir = test_get_files_and_directories(&String::from("."));
        for (ndx, path) in res.enumerate() {
            let path_str: String = path.unwrap().path().to_str().unwrap().to_owned();
            assert_eq!(path_str, crate_file_paths[ndx]);
        }
    }

    #[test]
    fn find_all_files_test() {
        let dir_path = String::from(".");
        let mut file_tracker: FileTracker = FileTracker::new();
        find_all_files(&dir_path, &mut file_tracker);
        assert!(compare_source_files(&file_tracker));
    }
    fn compare_source_files(file_tracker: &FileTracker) -> bool {
        let crate_file_paths: [&'static str; 9] = [
            "./src/main.rs",
            "./src/cli/scan_args.rs",
            "./src/cli/mod.rs",
            "./src/file_io/discovery.rs",
            "./src/file_io/file_tracker.rs",
            "./src/file_io/mod.rs",
            "./src/git_io/funcs.rs",
            "./src/git_io/mod.rs",
            "./src/lib.rs",
        ];
        let files_found: &Vec<PathBuf> = file_tracker.get_files();
        for ndx in 0..crate_file_paths.len() {
            let temp_pb = PathBuf::from(crate_file_paths[ndx]);
            if !files_found.contains(&temp_pb){
                return false
            }
        }
        return true
    }
}