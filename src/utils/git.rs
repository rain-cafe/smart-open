use std::io::Result;
use std::path::Path;
use std::process::{Command, Stdio, Output};
use std::str;

use log::error;

pub struct Remote {
    pub name: String,
    pub git_url: String,
    pub url: String,
}

fn is_git_dir(dir: &str) -> bool {
    return Path::new(&dir).join(".git").exists();
}

fn exec(dir: &str, args: Vec<&str>) -> Result<Output> {
    let mut command = Command::new("git");
    
    for arg in args.iter() {
        command.arg(arg);
    }

    match command.current_dir(&dir).stdout(Stdio::piped()).spawn() {
        Ok(child) => {
            return child.wait_with_output();
        },
        Err(e) => {
            error!("Failed to run git!");
            return Err(e);
        },
    };
}

pub fn get_remotes(dir: &str) -> Vec<Remote> {
    if !is_git_dir(&dir) {
        return vec![];
    }

    let output = exec(&dir, vec!["remote", "-v"]);

    if output.is_err() {
        return vec![];
    }

    let output = output.unwrap();
    
    let remotes = str::from_utf8(&output.stdout).unwrap();

    return remotes.lines()
        .filter(|x| x.contains("(fetch)"))
        .map(|remote| {
            let mut iter = remote.split_whitespace();
            let name = iter.next();
            let url = iter.next();

            if name.is_some() && url.is_some() {
                let git_url = url.unwrap().to_string();
                return Some(Remote {
                    name: name.unwrap().to_string(),
                    url: git_url.clone().replace(":", "/").replace("git@", "https://"),
                    git_url: git_url,
                });
            }
            
            None
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();
}

#[cfg(test)]
mod tests {
    use crate::utils::git::{exec, get_remotes};

    #[test]
    fn get_remotes_should_return_the_remotes() {
        exec(".", vec!["init", ".test-git"]).expect("Failed to initialize git directory");
        exec(".test-git", vec!["remote", "add", "origin", "git@github.com:rain-cafe/smart-open.git"]).expect("Failed to initialize git directory");

        // I hate this, but NixOS does shenanigans to the git directory
        // Ideally we should setup a test git directory
        assert_eq!(get_remotes(".test-git").len(), 1);
    }

    #[test]
    fn get_remotes_should_return_an_empty_list_if_not_in_a_git_repo() {
        assert_eq!(get_remotes("../").len(), 0);
    }
}