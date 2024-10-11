use crate::profile::{GitConfig, RepoItem, RepoItemData};
use crate::tool::{get_current_dir, run_command};
use std::path::Path;

pub fn sync(items: &[RepoItem]) {
    sync_512(items, false);
}

pub fn sync_force(items: &[RepoItem]) {
    sync_512(items, true);
}

fn sync_512(items: &[RepoItem], force: bool) {
    let cwd = get_current_dir();
    match cwd {
        Ok(cwd) => {
            items.iter().for_each(|item| {
                println!("Syncing {}...", item.name);
                let target_path = cwd.join(&item.path);
                if force {
                    if let Err(e) = run_command(&["rm", "-rf", target_path.to_str().unwrap()]) {
                        println!("Error while clear: {}", e);
                        return;
                    }
                }
                match &item.data {
                    RepoItemData::Git(it) => {
                        sync_git(&target_path, it);
                    }
                    RepoItemData::Http(_it) => {
                        println!("no realization!")
                    }
                }
            });
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn sync_git(path: &Path, config: &GitConfig) {
    if path.exists() {
        if path.is_dir() {
            println!("target directory does exist, maybe synced, skipping");
            return;
        }
        println!("target path is exist and not a directory, skipping");
        return;
    }
    let mut cmd = vec!["git", "clone", &config.url];
    if !config.hash.is_empty() {
        cmd.push("-b");
        cmd.push(&config.hash);
    }
    cmd.push(path.to_str().unwrap());
    println!("{:?}", &cmd);
    if let Err(e) = run_command(&cmd) {
        println!("Error: {}", e);
        return;
    }
    if let Err(e) = run_command(&["rm", "-rf", path.join(".git").to_str().unwrap()]) {
        println!("Error: {}", e);
    }
}