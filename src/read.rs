// src/read.rs

use git2::Repository;
use std::collections::HashSet;
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

fn is_path_under_any(path: &Path, bases: &[PathBuf]) -> bool {
    bases.iter().any(|base| {
        path.strip_prefix(base).is_ok()
            || base
                .file_name()
                .and_then(|f| Some(Path::new(f)))
                .map(|f| path == f)
                .unwrap_or(false)
    })
}

pub fn tracked_files_multiple(
    repo: &Repository,
    paths: &[PathBuf],
) -> Result<Vec<(PathBuf, SystemTime)>> {
    let mut files = Vec::new();
    let base_path = repo
        .workdir()
        .ok_or("Not a working directory")?
        .to_path_buf();

    // Resolve and validate all input paths
    let resolved_paths: Vec<PathBuf> = paths
        .iter()
        .map(|p| {
            if p.is_absolute() {
                p.clone()
            } else {
                base_path.join(p)
            }
        })
        .collect();

    for entry in repo.index()?.iter() {
        let path = PathBuf::from(std::str::from_utf8(&entry.path[..])?);

        // Check if the file matches any of the input paths
        if !paths.is_empty() && !is_path_under_any(&path, paths) {
            continue;
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
