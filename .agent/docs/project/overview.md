# Project Overview: Taskflow (Rust)

## Purpose
`taskflow` is a CLI tool designed to manage the development lifecycle of agents. It ensures that every code change is tracked, documented in a CHANGELOG, and committed to Git with a standardized format.

- **Unified Architecture**: Supports both Classic (single-branch) and Parallel (worktree-based) task management via a single `.agent/config.toml` configuration.
- **Smart Release Engine**: The `release` command automatically detects the mode. In **Classic mode**, it handles branch switching. In **Parallel mode**, it targets the `dev/` worktree for versioning while performing the merge and tagging at the management root. It ensures `Cargo.lock` is atomically synchronized and version tags are pushed to remote.
- **Parallel Multi-Agent Development**: `init` command converts a repo into a management root with worktree-based isolation.
- **Automated Branching**: `open` command handles branching/worktree creation and immediate changelog "locking".
- **Global Installation**: Provides `install.sh` (Unix) and `install.bat` (Windows) to install `taskflow` as a native CLI command.
- **Bilingual CLI Help**: Full English and Chinese support in `--help` output for improved readability by both humans and agents.
- **Remote Synchronization**: `close` and `release` commands automatically push changes to remote repositories.
- **Prompt Coaching**: `taskflow coach` subcommand provides a simplified pipe/CLI interface for agents to generate human-centric coaching reports without needing to resolve internal repository structures. These reports focus on **meta-level context engineering advice** to help human users optimize their future prompts. The `close` command enforces the generation of this report at `docs/prompt-coaching/<YYYYMMDD>/<HHMMSS>-<topic>.md`.
- **GitHub-Integrated Releases**: Automated CI/CD (GitHub Actions) for cross-platform binary distribution on Tag creation.
- **Structured Git Commits**: Enforces Conventional Commits with Task IDs integrated into the message.
- **Streamlined Distribution**: 
    - `scripts/build.sh` (macOS/Linux) and `scripts/build.bat` (Windows) handle both local and cross-platform builds.
    - Final output is organized by platform in the `dist/` directory (e.g., `dist/macos_arm64/taskflow/`).

## Directory Structure
```
.
├── src/                # Rust source code
│   ├── main.rs         # CLI entry point (clap)
│   ├── task.rs         # Task lifecycle (open/close/init/status/release)
│   ├── git.rs          # Git wrappers (branching, merging, worktrees)
│   └── config.rs       # Unified config system & smart branch probing
├── .agent/             # Management directory
│   ├── config.toml     # v1.1.1 Unified config (mode, branches)
│   └── docs/project/   # Internal project documentation (AI/Agent context)
├── assets/
│   └── SKILL.md        # Skill definition template
├── scripts/            # Build and installation scripts
│   ├── build.sh        # Unix build/distribute script
│   └── install.sh      # Unix installation script
├── docs/
│   └── CHANGELOG.md    # Project task log
└── dist/               # Final packaged distribution
    └── <platform>/     # e.g., macos_arm64/
        └── taskflow/
            ├── bin/    # Executable
            ├── scripts/# Installers (install.sh/bat)
            └── SKILL.md# Documentation
```

## Parallel Mode Structure
When `taskflow init` is used, the project adopts this layout:
```
project/                # Management root (.git/ lives here, taskflow-root branch)
├── .agent/config.toml  # Parallel mode configuration (base_branch, main_branch)
├── <base_branch>/      # Main development worktree (e.g., dev/)
└── tasks/              # Active task worktrees
    ├── <topic>-<id>/   # Task A (git worktree)
    └── <id>/           # Task B (git worktree)
```
