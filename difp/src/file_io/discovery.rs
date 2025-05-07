use std::path::Path;
use std::fs::{self, ReadDir};


pub fn is_file(dir_path: &String) -> bool {
    Path::new(dir_path).is_file()
}

pub fn is_directory(dir_path: &String) -> bool {
    Path::new(dir_path).is_dir()
}

pub fn get_files_and_directories(dir_path: &String) -> ReadDir {
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
        let crate_file_paths = [
            "./Cargo.toml",
            "./Cargo.lock",
            "./src",
            "./target"
        ];
        let res: ReadDir = get_files_and_directories(&String::from("."));
        for (ndx, path) in res.enumerate() {
            let path_str: String = path.unwrap().path().to_str().unwrap().to_owned();
            assert_eq!(path_str, crate_file_paths[ndx]);
        }
    }
}