use std::fs;
use std::path::Path;
use std::vec::Vec;
use errors::CliError;

pub fn list_repos(dir: &Path, git_dir: &str) -> Result<Vec<String>, CliError> {
    let mut repos: Vec<String> = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if path.join(git_dir).exists() {
                repos.push(path.to_str().unwrap().to_string());
            } else {
                let mut children = list_repos(&path, git_dir)?;
                repos.append(&mut children);
            }
        }
    }

    Ok(repos)
}
