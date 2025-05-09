use anyhow::{anyhow, Context, Result};
use git2::{Repository, StatusOptions};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Find the root directory of the git repository
pub fn get_git_repo_path() -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .context("Failed to execute git command. Is git installed?")?;
    
    if !output.status.success() {
        return Err(anyhow!("Not in a git repository"));
    }
    
    let path = String::from_utf8(output.stdout)
        .context("Failed to parse git output")?
        .trim()
        .to_string();
    
    Ok(PathBuf::from(path))
}

/// Get the diff of all uncommitted changes
pub fn get_uncommitted_diff() -> Result<String> {
    // Try to get both staged and unstaged changes
    let output = Command::new("git")
        .args(["diff", "HEAD"])
        .output()
        .context("Failed to execute git diff command")?;
    
    if !output.status.success() {
        return Err(anyhow!("Failed to get git diff"));
    }
    
    Ok(String::from_utf8(output.stdout)
        .context("Failed to parse git diff output")?)
}

/// Check if there are uncommitted changes in the repository
pub fn has_uncommitted_changes() -> Result<bool> {
    let repo_path = get_git_repo_path()?;
    let repo = Repository::open(repo_path)?;
    
    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true);
    status_options.include_ignored(false);
    status_options.include_unmodified(false);
    
    let statuses = repo.statuses(Some(&mut status_options))?;
    
    Ok(!statuses.is_empty())
}
