use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::types::*;

pub fn format_file_listing(files: &[(PathBuf, SystemTime)]) -> String {
    let mut listing = String::from("Repository Files (sorted by last modified):\n");
    listing.push_str("----------------------------------------\n");

    for (path, modified) in files {
        let modified_duration = modified
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let modified_str = chrono::DateTime::from_timestamp(modified_duration as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Unknown time".to_string());

        listing.push_str(&format!("{} ({})\n", path.display(), modified_str));
    }

    listing
}

pub fn format_commits(commits: &[CommitInfo]) -> String {
    let mut output = String::from("\nRecent Commits:\n");
    output.push_str("-------------\n");

    for commit in commits {
        let timestamp = chrono::DateTime::from_timestamp(commit.time.seconds(), 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Unknown time".to_string());

        output.push_str(&format!("{}: {}\n", timestamp, commit.message.trim()));
    }

    output
}
pub fn format_file(
    repo_root: &Path,
    file_path: &Path,
) -> std::result::Result<FormattedFile, SkipReason> {
    let full_path = repo_root.join(file_path);
    let metadata = fs::metadata(&full_path).map_err(|e| SkipReason::ReadError(e.to_string()))?;
    let modified_time = metadata
        .modified()
        .map_err(|e| SkipReason::ReadError(e.to_string()))?;

    let file = fs::File::open(&full_path).map_err(|e| SkipReason::ReadError(e.to_string()))?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => lines.push(line),
            Err(_) => return Err(SkipReason::InvalidUtf8),
        }
    }

    let max_line_num_width = lines.len().to_string().len();
    let formatted_content = lines
        .into_iter()
        .enumerate()
        .map(|(i, line)| format!("{:>width$} │ {}", i + 1, line, width = max_line_num_width))
        .collect::<Vec<_>>()
        .join("\n");

    Ok(FormattedFile {
        path: file_path.to_path_buf(),
        content: formatted_content,
        modified_time,
    })
}

pub fn format_skipped_files(skipped: &[SkippedFile]) -> String {
    if skipped.is_empty() {
        return String::from("\nNo files were skipped.");
    }

    let mut output = String::from("\nSkipped Files:\n");
    output.push_str("-------------\n");

    for SkippedFile { path, reason } in skipped {
        let reason_str = match reason {
            SkipReason::InvalidUtf8 => "Invalid UTF-8 encoding",
            SkipReason::GlobExcluded(pattern) => pattern,
            SkipReason::ReadError(err) => err,
        };
        output.push_str(&format!("{}: {}\n", path.display(), reason_str));
    }

    output
}

pub fn format_full_output(
    files: Vec<FormattedFile>,
    file_listing: String,
    commits: &[CommitInfo],
    skipped_files: &[SkippedFile],
) -> String {
    let mut output = String::new();
    output.push_str(&file_listing);
    output.push_str(&format_skipped_files(skipped_files));
    output.push_str(&format_commits(commits));
    output.push_str("\nFile Contents:\n=============\n\n");

    let delimiter = "```";
    output.push_str(
        &files
            .into_iter()
            .map(|file| {
                let modified = file
                    .modified_time
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                let modified_str = chrono::DateTime::from_timestamp(modified as i64, 0)
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_else(|| "Unknown time".to_string());

                format!(
                    "File: {} (Last modified: {})\n{}\n{}\n{}",
                    file.path.display(),
                    modified_str,
                    delimiter,
                    file.content,
                    delimiter
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n"),
    );

    output
}
