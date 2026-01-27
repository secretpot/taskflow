use std::process::Command;
use anyhow::{Result, anyhow};

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
