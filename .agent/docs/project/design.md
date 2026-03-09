# Design Documentation: Taskflow (Rust)

## Architecture
The tool is built as a modular Rust CLI application.

### Logic (task.rs)
- **Path Resolution**: Uses `std::env::current_dir()` to ensure all operations happen relative to the project where the command is executed.
- **State Management**: Uses HTML comments in `docs/CHANGELOG.md` (`<!-- CURRENT_TASK: ID -->`) as the source of truth for the active task.

### Build & Distribution (build.sh / GitHub Actions)
- **Multi-layered Build**: Local machine builds via `build.sh`; cloud-based release builds via GitHub Actions.
- **Packaging**: The scripts handle compiling the binary and packaging it with `SKILL.md` into a structured `dist/` hierarchy.
- **Cross-Compilation**: `build.sh --all` leverages `rustup` targets and cross-compilation toolchains (GCC) for macOS, Linux, and Windows.

## Key Decisions
- **Branching Model (Main/Dev)**: Adopted a standard branching model where `main` is the stable release branch and `dev` is for active development.
- **Immediate Task Locking**: `open` command performs an immediate `chore` commit to prevent changelog data loss across branch switches.
- **Automated Publishing**: GitHub Actions triggered by tags reduce manual build errors and ensure consistent release quality.
- **Prompt Coaching (Soft Guide + Hard Gate)**: The prompt coaching analysis is performed by the AI agent (which has conversation context), not the Rust binary. The binary enforces a **hard gate** in `close_task` by checking for the report's existence. It also supports **multi-language alignment** by storing an optional language preference in the CHANGELOG metadata (`[lang:zh]`) during `open_task`, which the agent then follows.
- **Bilingual & Detailed Help**: CLI help text is explicitly bilingual (EN/ZH) and detailed to serve as a self-documenting "AI primitive" within agentic loops.
- **Global Deployment & Agent Self-Healing**: Installation scripts (`install.sh`/`install.bat`) are provided to normalize the CLI path. `SKILL.md` includes logic for the Agent to auto-install the tool if it's missing, ensuring environmental robustness.
- **PWD over Script Dir**: Solves the "skill-to-project" mismatch.
- **Asset Separation**: `SKILL.md` is moved to `assets/` to keep the root directory clean.
