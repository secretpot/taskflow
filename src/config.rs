use anyhow::{Result, anyhow, Context};
use std::fs;
use std::path::PathBuf;

// WHY: Dual-mode design allows backward compatibility (Classic) while
// enabling parallel multi-agent development (Parallel) via git worktrees.
#[derive(Debug, PartialEq)]
pub enum TaskflowMode {
    Classic,
    Parallel,
}

pub struct Config {
    pub mode: TaskflowMode,
    pub base_branch: String,
}

const CONFIG_RELATIVE_PATH: &str = ".agent/config.toml";

/// Locate the git repository root by walking up from current_dir().
pub fn get_git_root() -> Result<PathBuf> {
    let output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("Not inside a git repository."));
    }

    let root = String::from_utf8(output.stdout)?.trim().to_string();
    Ok(PathBuf::from(root))
}

/// Resolve the management root: the directory containing `.git/` (the actual
/// git directory, not a worktree `.git` file).
/// In a linked worktree, `git rev-parse --show-toplevel` returns the worktree
/// path, but the real `.git/` directory is in the main worktree. We use
/// `--git-common-dir` to find the shared object store, then derive the root.
pub fn get_management_root() -> Result<PathBuf> {
    let output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--git-common-dir")
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("Not inside a git repository."));
    }

    let common_dir = String::from_utf8(output.stdout)?.trim().to_string();
    let common_path = PathBuf::from(&common_dir);

    // WHY: --git-common-dir returns the path to the shared .git directory.
    // Its parent is the management root (the main worktree).
    // For a regular repo, it returns ".git" (relative), so parent is ".".
    // For an absolute path, parent is the repo root.
    if common_path.is_absolute() {
        Ok(common_path.parent()
            .ok_or_else(|| anyhow!("Cannot determine management root"))?
            .to_path_buf())
    } else {
        // Relative path (e.g., ".git") - resolve from current git toplevel
        let toplevel = get_git_root()?;
        let resolved = toplevel.join(&common_path);
        Ok(resolved.parent()
            .ok_or_else(|| anyhow!("Cannot determine management root"))?
            .to_path_buf())
    }
}

/// Return the config file path at the management root.
fn get_config_path() -> Result<PathBuf> {
    let root = get_management_root()?;
    Ok(root.join(CONFIG_RELATIVE_PATH))
}

/// Detect the operating mode by checking for the config file.
pub fn detect_mode() -> Result<TaskflowMode> {
    let config_path = get_config_path()?;
    if config_path.exists() {
        Ok(TaskflowMode::Parallel)
    } else {
        Ok(TaskflowMode::Classic)
    }
}

/// Parse the config file and return a Config struct.
/// Only valid in Parallel mode (file must exist).
pub fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;
    let content = fs::read_to_string(&config_path)
        .context("Failed to read .agent/config.toml. Is taskflow init completed?")?;

    let base_branch = parse_toml_value(&content, "base_branch")
        .ok_or_else(|| anyhow!("Missing 'base_branch' in .agent/config.toml"))?;

    Ok(Config {
        mode: TaskflowMode::Parallel,
        base_branch,
    })
}

/// Return the tasks directory path for parallel mode.
pub fn get_tasks_dir() -> Result<PathBuf> {
    let root = get_management_root()?;
    Ok(root.join("tasks"))
}

// WHY: Avoid pulling in a full TOML parser crate for two simple key-value pairs.
// The config file is machine-generated and has a predictable format.
fn parse_toml_value(content: &str, key: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(key) {
            if let Some(value_part) = trimmed.split('=').nth(1) {
                let value = value_part.trim().trim_matches('"').to_string();
                if !value.is_empty() {
                    return Some(value);
                }
            }
        }
    }
    None
}
