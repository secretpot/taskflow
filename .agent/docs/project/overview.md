# Project Overview: Taskflow (Rust)

## Purpose
`taskflow` is a CLI tool designed to manage the development lifecycle of agents. It ensures that every code change is tracked, documented in a CHANGELOG, and committed to Git with a standardized format.

- **Automated Branching**: `open` command handles branching from `dev` and immediate changelog "locking".
- **Global Installation**: Provides `install.sh` (Unix) and `install.bat` (Windows) to install `taskflow` as a native CLI command.
- **Bilingual CLI Help**: Full English and Chinese support in `--help` output for improved readability by both humans and agents.
- **Remote Synchronization**: `close` and `release` commands automatically push changes to remote repositories.
- **Prompt Coaching**: `close` command enforces generation of a prompt coaching report (`docs/prompt-coaching/<topic>-<task-id>.md`) that analyzes the user's instruction quality, helping users improve their context engineering skills over time.
- **GitHub-Integrated Releases**: Automated CI/CD (GitHub Actions) for cross-platform binary distribution on Tag creation.
- **Structured Git Commits**: Enforces Conventional Commits with Task IDs integrated into the message.
- **Streamlined Distribution**: 
    - `scripts/build.sh` (macOS/Linux) and `scripts/build.bat` (Windows) handle both local and cross-platform builds.
    - Final output is organized by platform in the `dist/` directory (e.g., `dist/macos_arm64/taskflow/`).

## Directory Structure
```
.
├── src/                # Rust source code
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
