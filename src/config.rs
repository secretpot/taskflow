use anyhow::{anyhow, Result};
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
    pub base_branch: Option<String>,
    pub main_branch: Option<String>,
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
        Ok(common_path
            .parent()
            .ok_or_else(|| anyhow!("Cannot determine management root"))?
            .to_path_buf())
    } else {
        // Relative path (e.g., ".git") - resolve from current git toplevel
        let toplevel = get_git_root()?;
        let resolved = toplevel.join(&common_path);
        Ok(resolved
            .parent()
            .ok_or_else(|| anyhow!("Cannot determine management root"))?
            .to_path_buf())
    }
}

/// Return the config file path at the management root.
fn get_config_path() -> Result<PathBuf> {
    let root = get_management_root()?;
    Ok(root.join(CONFIG_RELATIVE_PATH))
}

/// Detect the operating mode.
pub fn detect_mode() -> Result<TaskflowMode> {
    Ok(load_config()?.mode)
}

/// Parse the config file and return a Config struct.
/// If config file doesn't exist, returns default Classic mode config.
pub fn load_config() -> Result<Config> {
    let mut mode = TaskflowMode::Classic;
    let mut base_branch = None;
    let mut main_branch = None;

    if let Ok(config_path) = get_config_path() {
        if config_path.exists() {
            if let Ok(content) = fs::read_to_string(&config_path) {
                if let Some(m) = parse_toml_value(&content, "mode") {
                    if m == "parallel" {
                        mode = TaskflowMode::Parallel;
                    }
                }
                base_branch = parse_toml_value(&content, "base_branch");
                main_branch = parse_toml_value(&content, "main_branch");
            }
        }
    }

    Ok(Config {
        mode,
        base_branch,
        main_branch,
    })
}

pub struct ReleaseBranches {
    pub base_branch: String,
    pub main_branch: String,
}

/// Resolve exactly what branches to use for release based on config and smart fallback.
pub fn resolve_release_branches() -> Result<ReleaseBranches> {
    let cfg = load_config()?;

    let main_branch = if let Some(m) = cfg.main_branch {
        m
    } else if crate::git::branch_exists("main") {
        "main".to_string()
    } else if crate::git::branch_exists("master") {
        "master".to_string()
    } else {
        "main".to_string()
    };

    let base_branch = if let Some(b) = cfg.base_branch {
        b
    } else if crate::git::branch_exists("dev") {
        "dev".to_string()
    } else if crate::git::branch_exists("develop") {
        "develop".to_string()
    } else {
        crate::git::get_current_branch().unwrap_or_else(|_| "dev".to_string())
    };

    Ok(ReleaseBranches {
        base_branch,
        main_branch,
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
