use git2::{Repository, RepositoryState, Status, string_array::StringArray};
use std::path::Path;

pub struct LocalRepo {
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

#[cfg(test)]
mod tests {

    use git2::BranchType;

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
}