use anyhow::{Result, anyhow, Context};
use chrono::Local;
use std::fs;
use std::path::PathBuf;
use regex::Regex;
use crate::git;

pub fn get_changelog_path() -> Result<PathBuf> {
    let mut path = std::env::current_dir()?;
    path.push("docs");
    path.push("CHANGELOG.md");
    Ok(path)
}

pub fn open_task(description: &str) -> Result<()> {
    let changelog_path = get_changelog_path()?;
    let docs_dir = changelog_path.parent().unwrap();
    
    if !docs_dir.exists() {
        fs::create_dir_all(docs_dir)?;
    }

    if !changelog_path.exists() {
        fs::write(&changelog_path, "# CHANGELOG\n\n")?;
    }

    let content = fs::read_to_string(&changelog_path)?;
    if content.contains("<!-- CURRENT_TASK:") {
        return Err(anyhow!("A task is already OPEN! Please close it before opening a new one."));
    }

    let task_id = Local::now().format("%Y%m%d-%H%M%S").to_string();
    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let mut new_content = format!(
        "# CHANGELOG\n<!-- CURRENT_TASK: {} -->\n\n## [{}] {}\n- **开始时间**: {}\n- **完成时间**: (进行中)\n- **类型**: (待定)\n- **描述**: (待补充)\n\n",
        task_id, task_id, description, current_time
    );

    // Skip the old header if it exists
    let body = if content.starts_with("# CHANGELOG") {
        content.lines().skip(1).collect::<Vec<_>>().join("\n")
    } else {
        content
    };
    new_content.push_str(body.trim_start());

    fs::write(&changelog_path, new_content)?;

    println!("✓ Task OPENED successfully");
    println!("Task ID: {}", task_id);
    println!("Description: {}", description);
    println!("CHANGELOG updated: {:?}", changelog_path);

    Ok(())
}

pub fn close_task(
    commit_type: &str,
    scope: Option<String>,
    subject: &str,
    body: &str,
    footer: Option<String>,
    auto_stage: bool,
) -> Result<()> {
    let changelog_path = get_changelog_path()?;
    let content = fs::read_to_string(&changelog_path)
        .context("CHANGELOG.md not found. Have you opened a task?")?;

    let re_marker = Regex::new(r"<!-- CURRENT_TASK: (.*) -->")?;
    let task_id = re_marker.captures(&content)
        .ok_or_else(|| anyhow!("No active task found in CHANGELOG.md"))?
        .get(1).unwrap().as_str().to_string();

    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let type_scope = if let Some(s) = scope.as_ref() {
        if s.is_empty() {
            commit_type.to_string()
        } else {
            format!("{}({})", commit_type, s)
        }
    } else {
        commit_type.to_string()
    };

    // Update CHANGELOG content
    let mut updated_lines = Vec::new();
    let mut in_task = false;
    
    for line in content.lines() {
        if line.starts_with(&format!("## [{}]", task_id)) {
            in_task = true;
            updated_lines.push(line.to_string());
            continue;
        }

        if in_task {
            if line.contains("- **完成时间**:") {
                updated_lines.push(format!("- **完成时间**: {}", current_time));
                continue;
            }
            if line.contains("- **类型**:") {
                updated_lines.push(format!("- **类型**: {}", type_scope));
                continue;
            }
            if line.contains("- **描述**:") {
                updated_lines.push(format!("- **描述**: {}", subject));
                in_task = false;
                continue;
            }
        }

        if !line.starts_with("<!-- CURRENT_TASK:") {
            updated_lines.push(line.to_string());
        }
    }

    fs::write(&changelog_path, updated_lines.join("\n"))?;

    // Build commit message
    let scope_filtered = scope.filter(|s| !s.is_empty());
    let footer_filtered = footer.filter(|s| !s.is_empty());

    let scope_str = scope_filtered.as_ref().map(|s| format!("({})", s)).unwrap_or_default();
    let mut commit_msg = format!("{}{}: [{}] {}\n\n{}", commit_type, scope_str, task_id, subject, body);
    if let Some(f) = footer_filtered {
        commit_msg.push_str("\n\n");
        commit_msg.push_str(&f);
    }

    if auto_stage {
        println!("Auto-staging all modified files (git add .)...");
        git::git_add_all()?;
    }

    if git::has_staged_changes()? {
        git::git_commit(&commit_msg)?;
        println!("✓ Git commit created successfully");
    } else {
        println!("Warning: No staged changes found. Skipping git commit.");
    }

    println!("✓ Task CLOSED successfully");
    println!("Task ID: {}", task_id);
    println!("CHANGELOG updated: {:?}", changelog_path);

    Ok(())
}

pub fn status() -> Result<()> {
    let changelog_path = get_changelog_path()?;
    let content = fs::read_to_string(&changelog_path)
        .context("CHANGELOG.md not found.")?;

    let re_marker = Regex::new(r"<!-- CURRENT_TASK: (.*) -->")?;
    let task_id = re_marker.captures(&content)
        .ok_or_else(|| anyhow!("No active task"))?
        .get(1).unwrap().as_str();

    let re_desc = Regex::new(&format!(r"## \[{}\] (.*)", regex::escape(task_id)))?;
    let description = re_desc.captures(&content)
        .map(|c| c.get(1).unwrap().as_str())
        .unwrap_or("Unknown");

    println!("Task ID:     {}", task_id);
    println!("Description: {}", description);

    Ok(())
}
