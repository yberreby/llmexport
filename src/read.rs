use git2::Repository;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::types::*;

pub fn get_recent_commits(repo: &Repository, count: usize) -> Result<Vec<CommitInfo>> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    revwalk
        .take(count)
        .map(|oid| -> Result<CommitInfo> {
            let oid = oid?;
            let commit = repo.find_commit(oid)?;
            Ok(CommitInfo {
                message: commit.message().unwrap_or("").to_string(),
                time: commit.time(),
            })
        })
        .collect()
}

pub fn tracked_files(
    repo: &Repository,
    subdir: Option<&Path>,
) -> Result<Vec<(PathBuf, SystemTime)>> {
    let mut files = Vec::new();
    let base_path = repo
        .workdir()
        .ok_or("Not a working directory")?
        .to_path_buf();

    for entry in repo.index()?.iter() {
        let path = PathBuf::from(std::str::from_utf8(&entry.path[..])?);

        if let Some(subdir) = subdir {
            if !path.starts_with(subdir) {
                continue;
            }
        }

        let full_path = base_path.join(&path);
        if let Ok(metadata) = fs::metadata(&full_path) {
            if let Ok(modified) = metadata.modified() {
                files.push((path, modified));
            }
        }
    }

    files.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(files)
}
