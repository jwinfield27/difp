use git2::{Repository, RepositoryState, Status, string_array::StringArray};
use std::{collections::HashMap, path::Path};

struct LocalRepo {
    repo: Repository,
    state: RepositoryState,

}

impl LocalRepo {

    pub fn get_file_status(&self, file_path: &Path) -> Status {
        match self.repo.status_file(file_path) {
            Ok(status) => status,
            Err(_) => panic!("path given was not a valid file in repo")
        }
    }

    pub fn get_remotes(&self) -> StringArray {
        match self.repo.remotes() {
            Ok(remotes) => remotes,
            Err(_) => panic!("could not find any remotes associated to local repo")
        }
    }
}

pub fn get_repo() -> Repository {
    match Repository::open_from_env() {
        Ok(repo) => repo, 
        Err(_) => panic!("could not find repository in based off of current directory")
    }
}
pub fn file_git_diff() {

}

pub fn map_status_res(res: git2::Statuses) -> HashMap<String, String> {
    let mut file_status: HashMap<String, String> = HashMap::new();
    for entry in res.iter() {
        let status = entry.status();
        let path = String::from(entry.path().unwrap_or("unknown file path?"));
        let status_str: String = match_status(status);
        file_status.insert(path, status_str);
    }
    file_status
}

fn match_status(status: Status) -> String {
    match status {
        s if s.contains(Status::INDEX_MODIFIED) || s.contains(Status::WT_MODIFIED) => String::from("Modified"),
        s if s.contains(Status::INDEX_DELETED) || s.contains(Status::WT_DELETED) => String::from("Deleted"),
        s if s.contains(Status::INDEX_RENAMED) || s.contains(Status::WT_RENAMED) => String::from("Renamed"),
        s if s.contains(Status::INDEX_TYPECHANGE) || s.contains(Status::WT_TYPECHANGE) => String::from("TypeChanged"),
        s if s.contains(Status::IGNORED) =>  String::from("Ignored"),
        _ =>  String::from("Unknown"),
    }
}

#[cfg(test)]
mod tests {

    use git2::{BranchType, StatusOptions};

    use super::*;

    #[test]
    fn get_repo_test() {
        let repo: Repository = get_repo();
        let branch_name = repo.find_branch("main", BranchType::Local)
        .unwrap()
        .name()
        .unwrap()
        .ok_or("error")
        .unwrap()
        .to_owned();
        assert_eq!(String::from("main"), branch_name);
    }

    #[test]
    fn get_repo_path_test() {
        let repo: Repository = get_repo();
        let mut status_options: StatusOptions = StatusOptions::new();
        let res: git2::Statuses = repo.statuses(Some(&mut status_options))
        .expect("Failed to get status");
        let file_statuses: HashMap<String, String> = map_status_res(res);
        for (file, status) in file_statuses {
            println!("{},{}", file, status);
        }
        assert!(true);
    }
}