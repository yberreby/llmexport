use git2::Time;
use std::path::PathBuf;
use std::time::SystemTime;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct SkippedFile {
    pub path: PathBuf,
    pub reason: SkipReason,
}

#[derive(Debug)]
pub enum SkipReason {
    InvalidUtf8,
    GlobExcluded(String),
    ReadError(String),
}

pub struct FormattedFile {
    pub path: PathBuf,
    pub content: String,
    pub modified_time: SystemTime,
}

pub struct CommitInfo {
    pub message: String,
    pub time: Time,
}
