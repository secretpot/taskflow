# Design Documentation: Taskflow (Rust)

## Architecture
The tool is built as a modular Rust CLI application.

### Modules
- **main.rs**: CLI entry point using `clap`. Dispatches to task functions.
- **task.rs**: Core task lifecycle logic. Contains dual-mode dispatch for `open`, `close`, `status`, and `init_parallel`.
- **git.rs**: Git command wrappers. Includes standard operations (checkout, merge, push) and worktree management (add, remove, list, detach_head).
- **config.rs**: Unified configuration system. Manages mode detection (Classic vs Parallel) and smart branch resolution (`resolve_release_branches`). Allows explicit overrides via `.agent/config.toml`.

### Dual-Mode Architecture
- **Classic mode** (default): Branch-switching workflow. `open` creates a `task/` branch, `close` merges to `dev` and deletes the branch. Behavior unchanged from v1.x.
- **Parallel mode** (opt-in via `taskflow init`): Worktree-based workflow. `open` creates a new worktree under `tasks/` using `git worktree add`. `close` merges the task branch into the base branch worktree and removes the worktree. Multiple tasks can run concurrently.
- **Coach Subcommand**: `taskflow coach [--content "text"]` allows writing coaching reports from a string or stdin. It automatically resolves the correct path in the format `docs/prompt-coaching/<YYYYMMDD>/<HHMMSS>-<topic>.md` by parsing the active task marker from CHANGELOG.md.

### Unified Configuration & Mode Detection (config.rs)
- `detect_mode()`: Now part of the unified `load_config()` flow. It parses `.agent/config.toml` for a `mode` field ("classic" or "parallel"). If the config is missing or the field is unset, it defaults to **Classic mode**.
- `load_config()`: Parses optional `.agent/config.toml`. Returns a unified `Config` struct containing `mode`, optional `base_branch`, and optional `main_branch`.
- `resolve_release_branches()`: The core of the smart release logic. It implements a hierarchical resolution:
    1. **Explicit Config**: Uses `base_branch`/`main_branch` from `.agent/config.toml`.
    2. **Smart Probe**: Probes for standard Git branches (`main`/`master`, `dev`/`develop`).
    3. **Current Fallback**: Uses the current branch as the base branch if all else fails.
- `get_management_root()`: Resolves the real git root via `git rev-parse --git-common-dir`, working correctly from both the main repo and linked worktrees.

### State Management (task.rs)
- **Path Resolution**: Uses `std::env::current_dir()` to ensure all operations happen relative to the project where the command is executed. In Parallel mode, the management root is resolved via `git rev-parse --git-common-dir`.
- **Persistence**: The tool serializes `topic` and `lang` metadata into the marker during `open`, allowing `close` and `coach` to reconstruct the intended file paths and branching logic.
- **Smart Release & Lock Sync**: `release_task` leverages `resolve_release_branches()` to figure out where to bump versions and merge. It automatically runs `cargo update -p taskflow` to ensure `Cargo.lock` is atomically synchronized with `Cargo.toml`.

### Build & Distribution (build.sh / GitHub Actions)
- **Multi-layered Build**: Local machine builds via `build.sh`; cloud-based release builds via GitHub Actions.
- **Packaging**: The scripts handle compiling the binary and packaging it with `SKILL.md` into a structured `dist/` hierarchy.
- **Cross-Compilation**: `build.sh --all` leverages `rustup` targets and cross-compilation toolchains (GCC) for macOS, Linux, and Windows.

## Key Decisions
- **Unified Configuration Strategy**: Instead of basing mode solely on file existence, v1.1.1 uses an explicit `mode` field in `.agent/config.toml`. This allows using a configuration file even in Classic mode to define custom release branches.
- **Smart Branch Resolution (Smart Probe)**: To maximize "out-of-the-box" usability, the tool uses a smart probing strategy for releases, automatically finding `main`/`master` and `dev`/`develop` without requiring manual configuration in most projects.
- **Atomic Release Commit**: By integrating `cargo update` into the `release` command, the tool ensures that version bumps are always consistent between `Cargo.toml` and `Cargo.lock` in a single Git commit.
- **Orphan Branch Management Root**: During `init`, a new orphan branch (`taskflow-root`) is created, and all tracked source files are removed from the management root. The management root becomes a clean "control plane" that is not directly used for development, only containing config and `.gitignore`.
- **Auto-detected Base Branch**: `init` auto-detects the current branch as `base_branch` (with explicit user confirmation), or accepts `--branch` for explicit specification. Stored in `.agent/config.toml`.
- **Config in .agent/**: Configuration is stored in `.agent/config.toml` alongside existing `.agent/docs/` project documentation, maintaining a single management directory.
- **Branching Model (Main/Dev)**: Adopted a standard branching model where `main` is the stable release branch and `dev` (or user-configured base branch) is for active development.
- **Immediate Task Locking**: `open` command performs an immediate `chore` commit to prevent changelog data loss across branch switches.
- **Semantic Branch & Report Naming**: Branches and Prompt Coaching reports are named `<topic>-<task-id>` for semantic clarity and full traceability, falling back to `<task-id>` if no topic is provided.
- **Automated Publishing**: GitHub Actions triggered by tags reduce manual build errors and ensure consistent release quality.
- **Prompt Coaching (Human-Centric & Hard Gate)**: The analysis is performed by the AI agent and targeted at the **human user** to provide meta-level context engineering advice. The binary enforces a **hard gate** in `close_task` and provides a `coach` subcommand for agents to safely write reports. Key rules include:
    - **Meta-Advice Focus**: Moving beyond specific fixes to abstract prompt engineering rules.
    - **Strict Factual Validation**: Positive feedback in "What to Keep Doing" must be verified against actual user actions in the conversation (anti-hallucination).
    - **No Fluff**: Negative feedback is isolated in "Areas for Improvement" without defensive padding.
    - **Multi-language Alignment**: Persists `[lang:zh]` metadata in CHANGELOG to ensure report language consistency.
- **Bilingual & Detailed Help**: CLI help text is explicitly bilingual (EN/ZH) and detailed to serve as a self-documenting "AI primitive" within agentic loops.
- **Global Deployment & Agent Self-Healing**: Installation scripts (`install.sh`/`install.bat`) are provided to normalize the CLI path. `SKILL.md` includes logic for the Agent to auto-install the tool if it's missing, ensuring environmental robustness.
- **PWD over Script Dir**: Solves the "skill-to-project" mismatch.
- **Asset Separation**: `SKILL.md` is moved to `assets/` to keep the root directory clean.
