use clap::{Parser, Subcommand};
use anyhow::Result;

mod task;
mod git;
mod config;

#[derive(Parser)]
#[command(name = "taskflow")]
#[command(version)]
#[command(about = "Manage task lifecycle (管理任务生命周期)", 
    long_about = "A CLI tool for agents to manage task branches, changelogs, and git commits. (用于 Agent 自动化管理任务分支、变更日志及 Git 提交的命令行工具。)")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Open a new task (开启一个新任务)
    Open {
        /// Task description (任务描述)
        description: String,
        /// Optional topic/short-summary (可选的任务主题/简短摘要)
        topic: Option<String>,
        /// Optional language preference for prompt coaching (e.g. zh, en) (可选的指令教练语言偏好，如 zh/en)
        #[arg(long)]
        lang: Option<String>,
    },
    /// Close the current task (关闭当前任务)
    Close {
        /// Commit type (feat, fix, etc.) (提交类型，如 feat/fix 等)
        r#type: String,
        /// Scope of changes (use "" for no scope) (变更范围，无范围请使用 "")
        scope: String,
        /// Brief subject (简短的主题描述)
        subject: String,
        /// Detailed body (详细的变更描述)
        body: String,
        /// Optional footer (可选的页脚信息)
        #[arg(default_value = "")]
        footer: String,
        /// Disable auto-staging (default: true) (禁用自动暂存，默认开启)
        #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
        auto_stage: bool,
    },
    /// Release a new version (发布新版本)
    Release {
        /// New version string (e.g. 0.1.0) (新版本号字符串，如 0.1.0)
        version: String,
    },
    /// Get current task status (获取当前任务状态)
    Status,
    /// Initialize parallel task management (初始化并行任务管理)
    Init {
        /// Base branch name, auto-detected from current branch if omitted (基础分支名称，省略则自动从当前分支检测)
        #[arg(long)]
        branch: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Open { description, topic, lang } => {
            task::open_task(&description, topic.as_deref(), lang.as_deref())?;
        }
        Commands::Close { r#type, scope, subject, body, footer, auto_stage } => {
            task::close_task(&r#type, Some(scope), &subject, &body, Some(footer), auto_stage)?;
        }
        Commands::Release { version } => {
            task::release_task(&version)?;
        }
        Commands::Status => {
            task::status()?;
        }
        Commands::Init { branch } => {
            task::init_parallel(branch.as_deref())?;
        }
    }

    Ok(())
}
