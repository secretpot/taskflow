# Project Overview: Taskflow (Rust)

## Purpose
`taskflow` is a CLI tool designed to manage the development lifecycle of agents. It ensures that every code change is tracked, documented in a CHANGELOG, and committed to Git with a standardized format.

## Core Features
- **Task Serialization**: Prevents multiple overlapping tasks to maintain a clean history.
- **Automated CHANGELOG**: Generates and updates `docs/CHANGELOG.md` relative to the current working directory.
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
├── scripts/            # Build and maintenance scripts
│   ├── build.sh        # Unix build/distribute script
│   └── build.bat       # Windows build/distribute script
├── docs/
│   └── CHANGELOG.md    # Project task log
└── dist/               # Final packaged distribution
```
