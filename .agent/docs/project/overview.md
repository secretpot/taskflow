# Project Overview: Taskflow (Rust)

## Purpose
`taskflow` is a CLI tool designed to manage the development lifecycle of agents. It ensures that every code change is tracked, documented in a CHANGELOG, and committed to Git with a standardized format.

- **Dual-Mode Architecture**: Supports both Classic (single-branch) and Parallel (worktree-based) task management modes.
- **Parallel Multi-Agent Development**: `init` command converts a repo into a management root with worktree-based isolation, enabling multiple agents to work concurrently.
- **Automated Branching**: `open` command handles branching/worktree creation and immediate changelog "locking".
- **Global Installation**: Provides `install.sh` (Unix) and `install.bat` (Windows) to install `taskflow` as a native CLI command.
- **Bilingual CLI Help**: Full English and Chinese support in `--help` output for improved readability by both humans and agents.
- **Remote Synchronization**: `close` and `release` commands automatically push changes to remote repositories.
- **Prompt Coaching**: `close` command enforces generation of a prompt coaching report (`docs/prompt-coaching/<YYYYMMDD>/<HHMMSS>-<topic>.md`) tailored for human users, helping them improve their prompt and context engineering skills globally.
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
│   ├── task.rs         # Task lifecycle logic (open/close/init/status/release)
│   ├── git.rs          # Git command wrappers (incl. worktree helpers)
│   └── config.rs       # Mode detection and config parsing (.agent/config.toml)
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
project/                # Management root (.git/ lives here, HEAD detached)
├── .agent/config.toml  # Mode and base_branch configuration
├── <base_branch>/      # Main development worktree (e.g., dev/, develop/)
└── tasks/              # Active task worktrees
    ├── fix-bug-123/    # Task A (independent git worktree)
    └── add-feature/    # Task B (independent git worktree)
```
