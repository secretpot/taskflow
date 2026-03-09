use anyhow::{Result, anyhow, Context};
use chrono::Local;
use std::fs;
use std::path::PathBuf;
use regex::Regex;
use crate::git;
use crate::config;

pub fn get_changelog_path() -> Result<PathBuf> {
    let mut path = std::env::current_dir()?;
    path.push("docs");
    path.push("CHANGELOG.md");
    Ok(path)
}

// --- Init ---

pub fn init_parallel(branch: Option<&str>) -> Result<()> {
    let root = config::get_git_root()?;

    // Verify .git is a directory (we are in the main worktree, not a linked one)
    let dot_git = root.join(".git");
    if !dot_git.is_dir() {
        return Err(anyhow!("Must run 'init' from the main git repository root, not from a linked worktree."));
    }

    let config_path = root.join(".agent/config.toml");
    if config_path.exists() {
        return Err(anyhow!("Already initialized. Config exists at {:?}", config_path));
    }

    // Resolve base branch
    let base_branch = match branch {
        Some(b) => {
            println!("Using specified base branch: {}", b);
            b.to_string()
        }
        None => {
            let detected = git::get_current_branch()?;
            println!("Detected base branch: {}. Using it as the base branch.", detected);
            detected
        }
    };

    // Create .agent directory if it doesn't exist
    let agent_dir = root.join(".agent");
    if !agent_dir.exists() {
        fs::create_dir_all(&agent_dir)?;
    }

    // Write config
    let config_content = format!(
        "mode = \"parallel\"\nbase_branch = \"{}\"\n",
        base_branch
    );
    fs::write(&config_path, config_content)?;
    println!("Created config: {:?}", config_path);

    // Create tasks directory
    let tasks_dir = root.join("tasks");
    if !tasks_dir.exists() {
        fs::create_dir_all(&tasks_dir)?;
    }
    println!("Created tasks directory: {:?}", tasks_dir);

    // Create base branch worktree
    let base_worktree_path = root.join(&base_branch);
    if !base_worktree_path.exists() {
        // WHY: Git prevents a branch from being checked out in two places.
        // Detach HEAD in the management root so the base branch is free.
        println!("Detaching HEAD in management root...");
        git::detach_head()?;

        println!("Creating worktree for base branch '{}'...", base_branch);
        git::worktree_add_existing(&base_worktree_path, &base_branch)?;
    } else {
        println!("Base branch worktree already exists at {:?}", base_worktree_path);
    }

    // Update .gitignore
    let gitignore_path = root.join(".gitignore");
    let mut gitignore = if gitignore_path.exists() {
        fs::read_to_string(&gitignore_path)?
    } else {
        String::new()
    };

    let entries_to_add = vec![
        format!("/{}/", base_branch),
        "/tasks/".to_string(),
    ];

    let mut modified = false;
    for entry in &entries_to_add {
        if !gitignore.contains(entry.as_str()) {
            if !gitignore.ends_with('\n') && !gitignore.is_empty() {
                gitignore.push('\n');
            }
            gitignore.push_str(entry);
            gitignore.push('\n');
            modified = true;
        }
    }

    if modified {
        fs::write(&gitignore_path, gitignore)?;
        println!("Updated .gitignore");
    }

    println!("\n--- Parallel mode initialized ---");
    println!("Management root: {:?}", root);
    println!("Base branch:     {}", base_branch);
    println!("Dev worktree:    {:?}", base_worktree_path);
    println!("Tasks directory: {:?}", tasks_dir);
    println!("\nPlease open {:?} as your IDE workspace root to see all worktrees.", root);

    Ok(())
}

// --- Open ---

pub fn open_task(description: &str, topic: Option<&str>, lang: Option<&str>) -> Result<()> {
    let mode = config::detect_mode()?;
    match mode {
        config::TaskflowMode::Classic => open_task_classic(description, topic, lang),
        config::TaskflowMode::Parallel => open_task_parallel(description, topic, lang),
    }
}

fn open_task_classic(description: &str, topic: Option<&str>, lang: Option<&str>) -> Result<()> {
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

fn open_task_parallel(description: &str, topic: Option<&str>, lang: Option<&str>) -> Result<()> {
    let cfg = config::load_config()?;
    let _management_root = config::get_management_root()?;
    let tasks_dir = config::get_tasks_dir()?;

    let task_id = Local::now().format("%Y%m%d-%H%M%S").to_string();
    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let (branch_name, worktree_dir_name) = if let Some(t) = topic {
        (format!("task/{}-{}", t, task_id), format!("{}-{}", t, task_id))
    } else {
        (format!("task/{}", task_id), task_id.clone())
    };

    let worktree_path = tasks_dir.join(&worktree_dir_name);

    // Fetch latest base branch from remote (non-blocking, no checkout)
    println!("🔄 Fetching latest {} from origin...", cfg.base_branch);
    git::fetch("origin", &cfg.base_branch)?;

    // Determine start point: prefer remote tracking branch, fall back to local
    let start_point = format!("origin/{}", cfg.base_branch);
    // WHY: Check if remote tracking branch exists. If not (e.g., no remote),
    // fall back to local base branch.
    let check_remote = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--verify")
        .arg(&start_point)
        .output()?;

    let actual_start = if check_remote.status.success() {
        start_point
    } else {
        println!("ℹ️  Remote tracking branch not found. Using local '{}'.", cfg.base_branch);
        cfg.base_branch.clone()
    };

    // Create worktree
    println!("🌿 Creating worktree at {:?}...", worktree_path);
    git::worktree_add(&worktree_path, &branch_name, &actual_start)?;

    // Initialize CHANGELOG inside the worktree
    let wt_docs_dir = worktree_path.join("docs");
    if !wt_docs_dir.exists() {
        fs::create_dir_all(&wt_docs_dir)?;
    }

    let wt_changelog = wt_docs_dir.join("CHANGELOG.md");
    let existing_content = if wt_changelog.exists() {
        fs::read_to_string(&wt_changelog)?
    } else {
        "# CHANGELOG\n\n".to_string()
    };

    let lang_suffix = lang.map(|l| format!(" [lang:{}]", l)).unwrap_or_default();
    let topic_suffix = topic.map(|t| format!(" [topic:{}]", t)).unwrap_or_default();
    let mut new_content = format!(
        "# CHANGELOG\n<!-- CURRENT_TASK: {}{}{} -->\n\n## [{}] {}\n- **开始时间**: {}\n- **完成时间**: (进行中)\n- **类型**: (待定)\n- **描述**: (待补充)\n\n",
        task_id, lang_suffix, topic_suffix, task_id, description, current_time
    );

    let body = if existing_content.starts_with("# CHANGELOG") {
        existing_content.lines().skip(1).collect::<Vec<_>>().join("\n")
    } else {
        existing_content
    };
    new_content.push_str(body.trim_start());

    fs::write(&wt_changelog, new_content)?;

    // Lock the task with an immediate commit inside the worktree
    println!("💾 Securing task in git...");
    git::commit_in(&worktree_path, &format!("chore: open task [{}]", task_id))?;

    println!("✓ Task OPENED successfully (Parallel mode)");
    println!("Task ID: {}", task_id);
    println!("Branch:  {}", branch_name);
    println!("Description: {}", description);
    println!("WORKTREE_PATH: {}", worktree_path.display());
    println!("\nPlease switch your working directory to the worktree path above.");

    Ok(())
}

// --- Close ---

pub fn close_task(
    commit_type: &str,
    scope: Option<String>,
    subject: &str,
    body: &str,
    footer: Option<String>,
    auto_stage: bool,
) -> Result<()> {
    let mode = config::detect_mode()?;
    match mode {
        config::TaskflowMode::Classic => close_task_classic(commit_type, scope, subject, body, footer, auto_stage),
        config::TaskflowMode::Parallel => close_task_parallel(commit_type, scope, subject, body, footer, auto_stage),
    }
}

fn close_task_classic(
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
    let re_id = Regex::new(r"^([^\s]+)")?;
    let task_id = re_id.captures(&marker_content)
        .ok_or_else(|| anyhow!("Invalid task marker format"))?
        .get(1).unwrap().as_str().to_string();

    let re_topic = Regex::new(r"\[topic:([^\]]+)\]")?;
    let topic = re_topic.captures(&marker_content)
        .map(|c| c.get(1).unwrap().as_str());

    // Hard Gate: Verify prompt coaching document exists
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

fn close_task_parallel(
    commit_type: &str,
    scope: Option<String>,
    subject: &str,
    body: &str,
    footer: Option<String>,
    auto_stage: bool,
) -> Result<()> {
    let cfg = config::load_config()?;
    let management_root = config::get_management_root()?;

    // We must be inside a task worktree
    let current_branch = git::get_current_branch()?;
    if !current_branch.starts_with("task/") {
        return Err(anyhow!(
            "You are not in a task worktree (branch: {})! Navigate to a task worktree first.",
            current_branch
        ));
    }

    let worktree_root = config::get_git_root()?;

    let changelog_path = {
        let mut p = worktree_root.clone();
        p.push("docs");
        p.push("CHANGELOG.md");
        p
    };
    let content = fs::read_to_string(&changelog_path)
        .context("CHANGELOG.md not found in this worktree.")?;

    let re_marker = Regex::new(r"<!-- CURRENT_TASK: (.*) -->")?;
    let marker_content = re_marker.captures(&content)
        .ok_or_else(|| anyhow!("No active task found in CHANGELOG.md"))?
        .get(1).unwrap().as_str().to_string();

    let re_id = Regex::new(r"^([^\s]+)")?;
    let task_id = re_id.captures(&marker_content)
        .ok_or_else(|| anyhow!("Invalid task marker format"))?
        .get(1).unwrap().as_str().to_string();

    let re_topic = Regex::new(r"\[topic:([^\]]+)\]")?;
    let topic = re_topic.captures(&marker_content)
        .map(|c| c.get(1).unwrap().as_str());

    // Hard Gate: prompt coaching check
    let coaching_filename = if let Some(t) = topic {
        format!("{}-{}.md", t, task_id)
    } else {
        format!("{}.md", task_id)
    };

    let coaching_file = {
        let mut p = worktree_root.clone();
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
        if s.is_empty() { commit_type.to_string() }
        else { format!("{}({})", commit_type, s) }
    } else {
        commit_type.to_string()
    };

    // Update CHANGELOG
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

    // Merge into base branch from the management root
    println!("🚀 Merging task branch into {}...", cfg.base_branch);
    git::checkout_in(&management_root, &cfg.base_branch)?;
    git::merge_in(&management_root, &current_branch)?;

    // Remove worktree and delete branch
    println!("🧹 Removing worktree and cleaning up...");
    git::worktree_remove(&worktree_root)?;
    git::delete_branch(&current_branch)?;

    // Push base branch
    println!("📤 Syncing {} with remote...", cfg.base_branch);
    git::push("origin", &cfg.base_branch, false)?;

    println!("✓ Task CLOSED successfully (Parallel mode)");
    println!("Task ID: {}", task_id);

    Ok(())
}

// --- Release ---

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

    println!("Release v{} complete!", version);
    println!("GitHub Actions will now automatically build and publish the release.");

    Ok(())
}

// --- Status ---

pub fn status() -> Result<()> {
    let mode = config::detect_mode()?;
    match mode {
        config::TaskflowMode::Classic => status_classic(),
        config::TaskflowMode::Parallel => status_parallel(),
    }
}

fn status_classic() -> Result<()> {
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

    println!("Mode:        Classic");
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

fn status_parallel() -> Result<()> {
    let cfg = config::load_config()?;
    let tasks_dir = config::get_tasks_dir()?;

    println!("Mode:        Parallel");
    println!("Base branch: {}", cfg.base_branch);

    // List active worktrees under tasks/
    let worktrees = git::worktree_list()?;
    let tasks_dir_str = tasks_dir.to_string_lossy().to_string();

    let task_worktrees: Vec<&(String, String)> = worktrees.iter()
        .filter(|(path, _)| path.starts_with(&tasks_dir_str))
        .collect();

    if task_worktrees.is_empty() {
        println!("\nNo active task worktrees.");
    } else {
        println!("\nActive tasks ({}):", task_worktrees.len());
        for (path, branch) in &task_worktrees {
            // Try to read task description from the worktree's CHANGELOG
            let wt_changelog = PathBuf::from(path).join("docs/CHANGELOG.md");
            let desc = if wt_changelog.exists() {
                if let Ok(content) = fs::read_to_string(&wt_changelog) {
                    let re_marker = Regex::new(r"<!-- CURRENT_TASK: ([^\s]+)").ok();
                    let task_id = re_marker
                        .and_then(|re| re.captures(&content))
                        .map(|c| c.get(1).unwrap().as_str().to_string());
                    
                    if let Some(id) = &task_id {
                        let re_desc = Regex::new(&format!(r"## \[{}\] (.*)", regex::escape(id))).ok();
                        re_desc
                            .and_then(|re| re.captures(&content))
                            .map(|c| c.get(1).unwrap().as_str().to_string())
                            .unwrap_or_else(|| "-".to_string())
                    } else {
                        "-".to_string()
                    }
                } else {
                    "-".to_string()
                }
            } else {
                "-".to_string()
            };

            println!("  - Branch: {}  Path: {}  Desc: {}", branch, path, desc);
        }
    }

    // Show current worktree context if we're inside one
    let current_dir = std::env::current_dir()?;
    let current_str = current_dir.to_string_lossy().to_string();
    if current_str.starts_with(&tasks_dir_str) {
        println!("\nYou are currently in a task worktree.");
    }

    Ok(())
}
