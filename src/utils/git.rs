use std::path::Path;
use std::process::{Command, Stdio};
use std::str;

use log::error;

pub struct Remote {
    pub name: String,
    pub git_url: String,
    pub url: String,
}

pub fn get_remotes(dir: &str) -> Vec<Remote> {
    if !Path::new(&dir).join(".git").exists() {
        return vec![];
    }

    let output = match Command::new("git").current_dir(&dir).arg("remote").arg("-v").stdout(Stdio::piped()).spawn() {
        Ok(child) => child.wait_with_output().map_or(None, |x| Some(x)),
        Err(_) => {
            error!("Failed to run git!");
            return vec![];
        }, 
    }.unwrap();
    
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
    use crate::utils::git::get_remotes;

    #[test]
    fn get_remotes_should_return_the_remotes() {
        assert_eq!(get_remotes(".").len(), 1);
    }

    #[test]
    fn get_remotes_should_return_an_empty_list_if_not_in_a_git_repo() {
        assert_eq!(get_remotes("../").len(), 0);
    }
}