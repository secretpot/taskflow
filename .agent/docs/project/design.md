# Design Documentation: Taskflow (Rust)

## Architecture
The tool is built as a modular Rust CLI application.

### Logic (task.rs)
- **Path Resolution**: Uses `std::env::current_dir()` to ensure all operations happen relative to the project where the command is executed.
- **State Management**: Uses HTML comments in `docs/CHANGELOG.md` (`<!-- CURRENT_TASK: ID -->`) as the source of truth for the active task.

### Build & Distribution (build.sh / build.bat)
- **Centralized Logic**: Redundant `setup.rs` was removed in favor of shell/batch scripts for better flexibility across platforms.
- **Packaging**: The scripts handle compiling the binary and packaging it with `SKILL.md` into a structured `dist/` hierarchy.
- **Cross-Compilation**: `build.sh --all` leverages `rustup` targets to produce binaries for macOS (Intel/ARM), Linux, and Windows.

## Key Decisions
- **PWD over Script Dir**: Solves the "skill-to-project" mismatch.
- **Asset Separation**: `SKILL.md` is moved to `assets/` to keep the root directory clean and clearly separate source from distribution templates.
- **Simplified CLI**: Position-based arguments for the `close` command were refined for a smoother developer experience.
