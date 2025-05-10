use core::fmt;
use std::path::PathBuf;

pub struct FileTracker {
    files: Vec<PathBuf>
}

impl fmt::Display for FileTracker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Found files: {:#?}", self.files)
    }
}

impl FileTracker {

    pub fn new() -> FileTracker {
        FileTracker {
            files: Vec::new()
        }
    }
    pub fn add_file(&mut self, file: PathBuf) {
        self.files.push(file);
    }

    pub fn get_files(&self) -> &Vec<PathBuf> {
        &self.files
    }
}