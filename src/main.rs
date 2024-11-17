use clap::Parser;
use git2::Repository;
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::io::{self, Write};

use cli::Cli;
use format::*;
use read::*;
use types::*;
mod cli;
mod format;
mod read;
mod types;

fn build_glob_set(patterns: &[&str]) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        builder.add(Glob::new(pattern)?);
    }
    Ok(builder.build()?)
}

fn main() -> types::Result<()> {
    let cli = Cli::parse();

    let mut ignore_patterns = vec!["*.lock", "*.log", "**/*.csv", "**/*.mat"];
    ignore_patterns.extend(cli.ignore.iter().map(|s| s.as_str()));
    let _glob_set = build_glob_set(&ignore_patterns)?;

    let repo = Repository::discover(".")?;
    let repo_root = repo.workdir().ok_or("Not a working directory")?;
    let recent_commits = get_recent_commits(&repo, cli.commits)?;
    let tracked = tracked_files(&repo, cli.directory.as_deref())?;

    let mut skipped_files = Vec::new();
    let mut formatted_files = Vec::new();

    for (path, _modified_time) in &tracked {
        if let Some(pattern) = ignore_patterns.iter().find(|&p| {
            let glob = Glob::new(p).unwrap();
            glob.compile_matcher().is_match(path)
        }) {
            skipped_files.push(SkippedFile {
                path: path.clone(),
                reason: SkipReason::GlobExcluded(pattern.to_string()),
            });
            continue;
        }

        match format_file(repo_root, path) {
            Ok(formatted) => formatted_files.push(formatted),
            Err(reason) => skipped_files.push(SkippedFile {
                path: path.clone(),
                reason,
            }),
        }
    }

    let file_listing = format_file_listing(&tracked);
    let full_output = format_full_output(
        formatted_files,
        file_listing,
        &recent_commits,
        &skipped_files,
    );

    println!("{}", full_output);
    io::stdout().flush()?;
    Ok(())
}
