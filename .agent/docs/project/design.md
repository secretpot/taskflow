# Design Documentation: Taskflow (Rust)

## Architecture
The tool is built as a modular Rust CLI application.

### Modules
- **main.rs**: CLI entry point using `clap`. Dispatches to task functions.
- **task.rs**: Core task lifecycle logic. Contains dual-mode dispatch for `open`, `close`, `status`, and `init_parallel`.
- **git.rs**: Git command wrappers. Includes standard operations (checkout, merge, push) and worktree management (add, remove, list, detach_head).
- **config.rs**: Mode detection and configuration parsing. Reads `.agent/config.toml` to determine Classic vs Parallel mode and resolve the management root.

### Dual-Mode Architecture
- **Classic mode** (default): Branch-switching workflow. `open` creates a `task/` branch, `close` merges to `dev` and deletes the branch. Behavior unchanged from v1.x.
- **Parallel mode** (opt-in via `taskflow init`): Worktree-based workflow. `open` creates a new worktree under `tasks/`, `close` merges to the configured base branch and removes the worktree. Multiple tasks can run concurrently.

### Mode Detection (config.rs)
- `detect_mode()` checks if `.agent/config.toml` exists at the git root. If yes, returns `Parallel`; otherwise `Classic`.
- `get_management_root()` resolves the real git root via `git rev-parse --git-common-dir`, working correctly from both the main repo and linked worktrees.
- `load_config()` parses the TOML config to extract `base_branch`.

### State Management (task.rs)
- **Path Resolution**: Uses `std::env::current_dir()` to ensure all operations happen relative to the project where the command is executed.
- **Task Marker**: Uses HTML comments in `docs/CHANGELOG.md` (`<!-- CURRENT_TASK: ID -->`) as the source of truth for the active task. In Parallel mode, each worktree has its own CHANGELOG with its own marker.

### Build & Distribution (build.sh / GitHub Actions)
- **Multi-layered Build**: Local machine builds via `build.sh`; cloud-based release builds via GitHub Actions.
- **Packaging**: The scripts handle compiling the binary and packaging it with `SKILL.md` into a structured `dist/` hierarchy.
- **Cross-Compilation**: `build.sh --all` leverages `rustup` targets and cross-compilation toolchains (GCC) for macOS, Linux, and Windows.

## Key Decisions
- **Dual-Mode Design**: Preserves backward compatibility for single-agent users (Classic) while enabling parallel development (Parallel) via `taskflow init`. Mode is detected automatically from config presence.
- **Detached HEAD in Management Root**: During `init`, the management root's HEAD is detached to free the base branch for worktree use. The management root becomes a "control plane" that is not directly used for development.
- **Auto-detected Base Branch**: `init` auto-detects the current branch as `base_branch` (with explicit user confirmation), or accepts `--branch` for explicit specification. Stored in `.agent/config.toml`.
- **Config in .agent/**: Configuration is stored in `.agent/config.toml` alongside existing `.agent/docs/` project documentation, maintaining a single management directory.
- **Branching Model (Main/Dev)**: Adopted a standard branching model where `main` is the stable release branch and `dev` (or user-configured base branch) is for active development.
- **Immediate Task Locking**: `open` command performs an immediate `chore` commit to prevent changelog data loss across branch switches.
- **Semantic Branch & Report Naming**: Branches and Prompt Coaching reports are named `<topic>-<task-id>` for semantic clarity and full traceability, falling back to `<task-id>` if no topic is provided.
- **Automated Publishing**: GitHub Actions triggered by tags reduce manual build errors and ensure consistent release quality.
- **Prompt Coaching (Soft Guide + Hard Gate)**: The prompt coaching analysis is performed by the AI agent (which has conversation context), not the Rust binary. The binary enforces a **hard gate** in `close_task` by checking for the report's existence. It also supports **multi-language alignment** by storing an optional language preference in the CHANGELOG metadata (`[lang:zh]`) during `open_task`, which the agent then follows.
- **Bilingual & Detailed Help**: CLI help text is explicitly bilingual (EN/ZH) and detailed to serve as a self-documenting "AI primitive" within agentic loops.
- **Global Deployment & Agent Self-Healing**: Installation scripts (`install.sh`/`install.bat`) are provided to normalize the CLI path. `SKILL.md` includes logic for the Agent to auto-install the tool if it's missing, ensuring environmental robustness.
- **PWD over Script Dir**: Solves the "skill-to-project" mismatch.
- **Asset Separation**: `SKILL.md` is moved to `assets/` to keep the root directory clean.
