use clap::{Parser, Subcommand};
use anyhow::Result;

mod task;
mod git;

#[derive(Parser)]
#[command(name = "taskflow")]
#[command(about = "Manage task lifecycle", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Open a new task
    Open {
        /// Task description
        description: String,
        /// Optional branch slug
        slug: Option<String>,
        /// Optional language preference for prompt coaching (e.g. zh, en)
        #[arg(long)]
        lang: Option<String>,
    },
    /// Close the current task
    Close {
        /// Commit type (feat, fix, etc.)
        r#type: String,
        /// Scope of changes (use "" for no scope)
        scope: String,
        /// Brief subject
        subject: String,
        /// Detailed body
        body: String,
        /// Optional footer
        #[arg(default_value = "")]
        footer: String,
        /// Disable auto-staging (default: true)
        #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
        auto_stage: bool,
    },
    /// Release a new version
    Release {
        /// New version string (e.g. 0.1.0)
        version: String,
    },
    /// Get current task status
    Status,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Open { description, slug, lang } => {
            task::open_task(&description, slug.as_deref(), lang.as_deref())?;
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
    }

    Ok(())
}
