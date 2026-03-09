use std::process::Command;
use anyhow::{Result, anyhow};

pub fn is_empty_repo() -> Result<bool> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()?;
    
    Ok(!output.status.success())
}

pub fn git_add_all() -> Result<()> {
    let status = Command::new("git")
        .arg("add")
        .arg(".")
        .status()?;
    
    if !status.success() {
        return Err(anyhow!("git add . failed"));
    }
    Ok(())
}

pub fn git_commit(message: &str) -> Result<()> {
    let status = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .status()?;
    
    if !status.success() {
        return Err(anyhow!("git commit failed"));
    }
    Ok(())
}

pub fn has_staged_changes() -> Result<bool> {
    let output = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .arg("--quiet")
        .status()?;
    
    Ok(!output.success())
}

pub fn get_current_branch() -> Result<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()?;
    
    if !output.status.success() {
        // Handle empty repository by trying to get the default branch name
        let default_branch = Command::new("git")
            .arg("config")
            .arg("--get")
            .arg("init.defaultBranch")
            .output()?;
        
        let branch = if default_branch.status.success() {
            String::from_utf8(default_branch.stdout)?.trim().to_string()
        } else {
            "main".to_string()
        };
        
        if branch.is_empty() {
            return Ok("main".to_string());
        }
        return Ok(branch);
    }
    
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

pub fn checkout(branch: &str) -> Result<()> {
    let status = Command::new("git")
        .arg("checkout")
        .arg(branch)
        .status()?;
    
    if !status.success() {
        return Err(anyhow!("git checkout {} failed", branch));
    }
    Ok(())
}

pub fn create_branch(branch: &str) -> Result<()> {
    let status = Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg(branch)
        .status()?;
    
    if !status.success() {
        return Err(anyhow!("git checkout -b {} failed", branch));
    }
    Ok(())
}

pub fn pull(remote: &str, branch: &str) -> Result<()> {
    let status = Command::new("git")
        .arg("pull")
        .arg(remote)
        .arg(branch)
        .status()?;
    
    if !status.success() {
        // Ignored if remote doesn't exist or pull fails due to no remote track
        println!("Warning: git pull {}:{} failed. Continuing...", remote, branch);
    }
    Ok(())
}

pub fn merge(branch: &str) -> Result<()> {
    let status = Command::new("git")
        .arg("merge")
        .arg(branch)
        .status()?;
    
    if !status.success() {
        return Err(anyhow!("git merge {} failed", branch));
    }
    Ok(())
}

pub fn delete_branch(branch: &str) -> Result<()> {
    let status = Command::new("git")
        .arg("branch")
        .arg("-d")
        .arg(branch)
        .status()?;
    
    if !status.success() {
        println!("Warning: Failed to delete local branch {}", branch);
    }
    Ok(())
}

pub fn push(remote: &str, branch: &str, include_tags: bool) -> Result<()> {
    // Check if remote exists
    let remote_check = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg(remote)
        .output()?;
    
    if !remote_check.status.success() {
        println!("ℹ️  Remote '{}' not found. Skipping push.", remote);
        return Ok(());
    }

    let mut cmd = Command::new("git");
    cmd.arg("push").arg(remote).arg(branch);
    
    if include_tags {
        cmd.arg("--tags");
    }

    let status = cmd.status()?;
    
    if !status.success() {
        println!("Warning: git push {} {} failed. Your internet connection might be down.", remote, branch);
    }
    Ok(())
}

// --- Worktree management functions ---

/// Detach HEAD so the current branch can be used by a worktree.
/// WHY: Git doesn't allow a branch to be checked out in two places.
/// By detaching HEAD, we free the branch for worktree use.
pub fn detach_head() -> Result<()> {
    let status = Command::new("git")
        .arg("checkout")
        .arg("--detach")
        .status()?;

    if !status.success() {
        return Err(anyhow!("git checkout --detach failed"));
    }
    Ok(())
}

/// Run a git command in a specific working directory.
fn run_git_in(dir: &std::path::Path, args: &[&str]) -> Result<()> {
    let status = Command::new("git")
        .current_dir(dir)
        .args(args)
        .status()?;

    if !status.success() {
        return Err(anyhow!("git {} failed in {:?}", args.join(" "), dir));
    }
    Ok(())
}

/// Create a new worktree with a new branch based on a start point.
pub fn worktree_add(path: &std::path::Path, branch: &str, start_point: &str) -> Result<()> {
    let status = Command::new("git")
        .arg("worktree")
        .arg("add")
        .arg("-b")
        .arg(branch)
        .arg(path)
        .arg(start_point)
        .status()?;

    if !status.success() {
        return Err(anyhow!(
            "git worktree add -b {} {:?} {} failed",
            branch, path, start_point
        ));
    }
    Ok(())
}

/// Create a new worktree linked to an already-existing branch.
/// WHY: `git worktree add -b` fails if the branch already exists.
/// This variant uses `git worktree add <path> <branch>` without `-b`.
pub fn worktree_add_existing(path: &std::path::Path, branch: &str) -> Result<()> {
    let status = Command::new("git")
        .arg("worktree")
        .arg("add")
        .arg(path)
        .arg(branch)
        .status()?;

    if !status.success() {
        return Err(anyhow!(
            "git worktree add {:?} {} failed",
            path, branch
        ));
    }
    Ok(())
}
pub fn worktree_remove(path: &std::path::Path) -> Result<()> {
    let status = Command::new("git")
        .arg("worktree")
        .arg("remove")
        .arg("--force")
        .arg(path)
        .status()?;

    if !status.success() {
        println!("Warning: Failed to remove worktree at {:?}", path);
    }
    Ok(())
}

/// List all linked worktrees. Returns a list of (path, branch) pairs.
pub fn worktree_list() -> Result<Vec<(String, String)>> {
    let output = Command::new("git")
        .arg("worktree")
        .arg("list")
        .arg("--porcelain")
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("git worktree list failed"));
    }

    let text = String::from_utf8(output.stdout)?;
    let mut result = Vec::new();
    let mut current_path = String::new();

    for line in text.lines() {
        if let Some(path) = line.strip_prefix("worktree ") {
            current_path = path.to_string();
        } else if let Some(branch_ref) = line.strip_prefix("branch ") {
            // branch_ref is like "refs/heads/task/xxx"
            let branch = branch_ref
                .strip_prefix("refs/heads/")
                .unwrap_or(branch_ref)
                .to_string();
            result.push((current_path.clone(), branch));
        }
    }

    Ok(result)
}

/// Check if the current directory is inside a linked worktree (not the main worktree).
/// WHY: In a linked worktree, `.git` is a file (not a directory) containing
/// a gitdir pointer. In the main worktree, `.git` is a directory.
#[allow(dead_code)]
pub fn is_worktree() -> Result<bool> {
    let git_root = crate::config::get_git_root()?;
    let dot_git = git_root.join(".git");
    Ok(dot_git.is_file())
}

/// Fetch a specific branch from a remote without checking it out.
pub fn fetch(remote: &str, branch: &str) -> Result<()> {
    let remote_check = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg(remote)
        .output()?;

    if !remote_check.status.success() {
        println!("ℹ️  Remote '{}' not found. Skipping fetch.", remote);
        return Ok(());
    }

    let status = Command::new("git")
        .arg("fetch")
        .arg(remote)
        .arg(branch)
        .status()?;

    if !status.success() {
        println!("Warning: git fetch {} {} failed. Continuing...", remote, branch);
    }
    Ok(())
}

/// Stage all changes and commit in a specific directory.
pub fn commit_in(dir: &std::path::Path, message: &str) -> Result<()> {
    run_git_in(dir, &["add", "."])?;
    run_git_in(dir, &["commit", "-m", message])?;
    Ok(())
}

/// Merge a branch into the current branch, executed from a specific directory.
pub fn merge_in(dir: &std::path::Path, branch: &str) -> Result<()> {
    run_git_in(dir, &["merge", branch])
}

/// Checkout a branch from a specific directory.
pub fn checkout_in(dir: &std::path::Path, branch: &str) -> Result<()> {
    run_git_in(dir, &["checkout", branch])
}
