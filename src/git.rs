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
