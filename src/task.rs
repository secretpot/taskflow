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

pub fn open_task(description: &str, topic: Option<&str>, lang: Option<&str>) -> Result<()> {
    let current_branch = git::get_current_branch()?;
    if current_branch.starts_with("task/") {
        return Err(anyhow!("You are already on a task branch ({})! Please close it before opening a new one.", current_branch));
    }

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
        return Err(anyhow!("A task is already OPEN (in CHANGELOG)! Please close it before opening a new one."));
    }

    // Git operations: Checkout dev and pull (Skip if repo is empty)
    if git::is_empty_repo()? {
        println!("ℹ️  Empty repository detected. Skipping dev sync.");
    } else {
        println!("🚀 Switching to dev and pulling latest changes...");
        if let Err(e) = git::checkout("dev") {
            println!("ℹ️  'dev' branch not found. Staying on current branch. ({})", e);
        } else {
            let _ = git::pull("origin", "dev");
        }
    }

    let task_id = Local::now().format("%Y%m%d-%H%M%S").to_string();
    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Create branch
    let branch_name = if let Some(t) = topic {
        format!("task/{}-{}", t, task_id)
    } else {
        format!("task/{}", task_id)
    };
    println!("🌿 Creating and switching to branch {}...", branch_name);
    git::create_branch(&branch_name)?;

    let lang_suffix = lang.map(|l| format!(" [lang:{}]", l)).unwrap_or_default();
    let topic_suffix = topic.map(|t| format!(" [topic:{}]", t)).unwrap_or_default();
    let mut new_content = format!(
        "# CHANGELOG\n<!-- CURRENT_TASK: {}{}{} -->\n\n## [{}] {}\n- **开始时间**: {}\n- **完成时间**: (进行中)\n- **类型**: (待定)\n- **描述**: (待补充)\n\n",
        task_id, lang_suffix, topic_suffix, task_id, description, current_time
    );

    // Skip the old header if it exists
    let body = if content.starts_with("# CHANGELOG") {
        content.lines().skip(1).collect::<Vec<_>>().join("\n")
    } else {
        content
    };
    new_content.push_str(body.trim_start());

    fs::write(&changelog_path, new_content)?;

    // Immediate commit on task branch to "lock" the task
    println!("💾 Securing task in git...");
    git::git_add_all()?;
    let _ = git::git_commit(&format!("chore: open task [{}]", task_id));

    println!("✓ Task OPENED successfully");
    println!("Task ID: {}", task_id);
    println!("Branch:  {}", branch_name);
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
    let current_branch = git::get_current_branch()?;
    if !current_branch.starts_with("task/") {
        return Err(anyhow!("You are not on a task branch ({})! Use 'git checkout' to switch to your task branch.", current_branch));
    }

    let changelog_path = get_changelog_path()?;
    let content = fs::read_to_string(&changelog_path)
        .context("CHANGELOG.md not found. Have you opened a task?")?;

    let re_marker = Regex::new(r"<!-- CURRENT_TASK: (.*) -->")?;
    let marker_content = re_marker.captures(&content)
        .ok_or_else(|| anyhow!("No active task found in CHANGELOG.md"))?
        .get(1).unwrap().as_str().to_string();

    // Parse task_id and optional metadata
    // Format: "ID" or "ID [lang:zh] [topic:my-topic]"
    let re_id = Regex::new(r"^([^\s]+)")?;
    let task_id = re_id.captures(&marker_content)
        .ok_or_else(|| anyhow!("Invalid task marker format"))?
        .get(1).unwrap().as_str().to_string();

    let re_topic = Regex::new(r"\[topic:([^\]]+)\]")?;
    let topic = re_topic.captures(&marker_content)
        .map(|c| c.get(1).unwrap().as_str());

    // Hard Gate: Verify prompt coaching document exists before allowing close.
    // We check this BEFORE modifying any files to ensure atomicity.
    let coaching_filename = if let Some(t) = topic {
        format!("{}-{}.md", t, task_id)
    } else {
        format!("{}.md", task_id)
    };

    let coaching_file = {
        let mut p = std::env::current_dir()?;
        p.push("docs");
        p.push("prompt-coaching");
        p.push(&coaching_filename);
        p
    };
    if !coaching_file.exists() {
        return Err(anyhow!(
            "Prompt coaching document not found: {:?}\n\
             You MUST generate a prompt coaching report before closing the task.\n\
             Expected path: docs/prompt-coaching/{}",
            coaching_file, coaching_filename
        ));
    }

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

    // Git workflow: Merge to dev and cleanup
    println!("🚀 Merging task branch into dev...");
    if let Err(e) = git::checkout("dev") {
        println!("ℹ️  'dev' branch not found. Creating it... ({})", e);
        git::create_branch("dev")?;
    }
    git::merge(&current_branch)?;
    
    println!("🧹 Deleting task branch {}...", current_branch);
    git::delete_branch(&current_branch)?;

    // Push dev to remote
    println!("📤 Syncing dev with remote...");
    git::push("origin", "dev", false)?;

    println!("✓ Task CLOSED successfully");
    println!("Task ID: {}", task_id);
    println!("CHANGELOG updated: {:?}", changelog_path);

    Ok(())
}

pub fn release_task(version: &str) -> Result<()> {
    println!("📦 Starting release process for version v{}...", version);

    // 1. Ensure we are on dev and clean
    git::checkout("dev")?;
    if git::has_staged_changes()? {
        return Err(anyhow!("You have staged changes on dev. Please commit or stash them before releasing."));
    }

    // 2. Update Cargo.toml version
    let mut cargo_path = std::env::current_dir()?;
    cargo_path.push("Cargo.toml");
    let cargo_content = fs::read_to_string(&cargo_path)?;
    let re_version = Regex::new(r#"version = "(.*)""#)?;
    let updated_cargo = re_version.replace(&cargo_content, format!(r#"version = "{}""#, version));
    fs::write(&cargo_path, updated_cargo.to_string())?;
    println!("✓ Cargo.toml updated to v{}", version);

    // 3. Commit version bump on dev and push
    git::git_add_all()?;
    git::git_commit(&format!("chore: release v{}", version))?;
    println!("📤 Syncing dev with remote...");
    git::push("origin", "dev", false)?;

    // 4. Merge dev into main
    println!("🚀 Merging dev into main...");
    git::checkout("main")?;
    git::merge("dev")?;

    // 5. Create tag
    println!("🏷️  Creating tag v{}...", version);
    let tag_name = format!("v{}", version);
    let status = std::process::Command::new("git")
        .arg("tag")
        .arg("-a")
        .arg(&tag_name)
        .arg("-m")
        .arg(format!("Release {}", tag_name))
        .status()?;
    
    if !status.success() {
        return Err(anyhow!("Failed to create git tag {}", tag_name));
    }

    // 6. Push main and tags
    println!("📤 Syncing main and tags with remote...");
    git::push("origin", "main", true)?;

    // 7. Return to dev
    git::checkout("dev")?;

    println!("✅ Release v{} complete!", version);
    println!("GitHub Actions will now automatically build and publish the release.");

    Ok(())
}

pub fn status() -> Result<()> {
    let changelog_path = get_changelog_path()?;
    let content = fs::read_to_string(&changelog_path)
        .context("CHANGELOG.md not found.")?;

    let re_marker = Regex::new(r"<!-- CURRENT_TASK: (.*) -->")?;
    let captures = re_marker.captures(&content);
    
    if captures.is_none() {
        println!("No active task found.");
        return Ok(());
    }
    
    let marker_content = captures.unwrap().get(1).unwrap().as_str();
    
    // Parse task_id and optional metadata
    // Format: "ID [lang:zh] [topic:my-topic]"
    let re_id = Regex::new(r"^([^\s]+)")?;
    let id_caps = re_id.captures(marker_content)
        .ok_or_else(|| anyhow!("Invalid task marker format"))?;
    let task_id = id_caps.get(1).unwrap().as_str();

    let re_lang = Regex::new(r"\[lang:([^\]]+)\]")?;
    let language = re_lang.captures(marker_content)
        .map(|c| c.get(1).unwrap().as_str());

    let re_topic = Regex::new(r"\[topic:([^\]]+)\]")?;
    let topic = re_topic.captures(marker_content)
        .map(|c| c.get(1).unwrap().as_str());

    let re_desc = Regex::new(&format!(r"## \[{}\] (.*)", regex::escape(task_id)))?;
    let description = re_desc.captures(&content)
        .map(|c| c.get(1).unwrap().as_str())
        .unwrap_or("Unknown");

    let branch = git::get_current_branch().unwrap_or_else(|_| "Unknown".to_string());

    println!("Task ID:     {}", task_id);
    println!("Branch:      {}", branch);
    println!("Description: {}", description);
    if let Some(l) = language {
        println!("Language:    {}", l);
    }
    if let Some(t) = topic {
        println!("Topic:       {}", t);
    }

    Ok(())
}
