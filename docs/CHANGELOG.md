# CHANGELOG

## [20260310-101147] Optimize prompt-coaching taskflow feature
- **开始时间**: 2026-03-10 10:11:47
- **完成时间**: 2026-03-10 10:17:39
- **类型**: feat(prompt-coaching)
- **描述**: Refactor prompt coaching for human focus and YYYYMMDD path prefix

## [20260310-093708] Refactor configuration to unify modes and implement smart branch resolution, fixing hardcoded branch names and Cargo.lock sync in release
- **开始时间**: 2026-03-10 09:37:08
- **完成时间**: 2026-03-10 09:37:31
- **类型**: refactor(config)
- **描述**: Unify config mode and implement smart branch detection for release

## [20260309-191327] Fix parallel close commands running in deleted worktree context
- **开始时间**: 2026-03-09 19:13:27
- **完成时间**: 2026-03-09 19:13:55
- **类型**: fix(parallel)
- **描述**: Fix git commands failing in deleted worktree context during parallel close

## [20260309-175537] Refactor init command to use orphan branch for clean management root
- **开始时间**: 2026-03-09 17:55:37
- **完成时间**: 2026-03-09 17:58:17
- **类型**: refactor(init)
- **描述**: Refactor init command to use orphan branch for clean management root

## [20260309-154524] Add git worktree integration for parallel multi-agent development
- **开始时间**: 2026-03-09 15:45:24
- **完成时间**: 2026-03-09 17:02:04
- **类型**: feat(parallel)
- **描述**: Add git worktree integration for parallel multi-agent development

## [1.0.4] - 2026-03-09
### Added
- **Semantic Naming**: Branches and Prompt Coaching reports now use `<topic>-<task-id>` format for better traceability.
- **Enhanced Metadata**: CHANGELOG metadata now persists `topic` and `lang` preferences.
- **Improved Installation**: Release packages now include a `scripts/` directory with standalone installers supporting relative path (`../bin/`) lookup.
- **Multi-platform Distribution**: Added `build.sh --all` to generate artifacts for macOS, Linux, and Windows.

### Fixed
- **Atomic Hard Gate**: Repositioned the Prompt Coaching check to occur BEFORE modifying the CHANGELOG, ensuring task state integrity on failure.

## [20260309-141943] Optimize installation scripts for release package structure
- **开始时间**: 2026-03-09 14:19:43
- **完成时间**: 2026-03-09 14:21:12
- **类型**: feat(installation)
- **描述**: optimize installation for release package structure

## [20260309-135916] Sync coaching report filename with branch pattern <topic>-<task-id>
- **开始时间**: 2026-03-09 13:59:16
- **完成时间**: 2026-03-09 14:10:26
- **类型**: feat(naming)
- **描述**: sync coaching report naming with branch pattern

## [20260309-135103] Optimize task naming (topic vs slug) and add --version support
- **开始时间**: 2026-03-09 13:51:03
- **完成时间**: 2026-03-09 13:52:12
- **类型**: feat(naming)
- **描述**: refine task naming and add version support

## [20260309-112409] Global installation with bilingual help support and agent self-healing logic
- **开始时间**: 2026-03-09 11:24:09
- **完成时间**: 2026-03-09 13:14:13
- **类型**: feat(installation)
- **描述**: global installation and bilingual help support

## [20260306-192346] Add multi-language support to prompt coaching with --lang flag and metadata storage
- **开始时间**: 2026-03-06 19:23:46
- **完成时间**: 2026-03-06 19:24:57
- **类型**: feat(coaching)
- **描述**: add multi-language support to prompt coaching

## [20260306-174351] Add prompt coaching feature with Hard Gate enforcement
- **开始时间**: 2026-03-06 17:43:51
- **完成时间**: 2026-03-06 17:47:15
- **类型**: feat(coaching)
- **描述**: add prompt coaching with hard gate enforcement

## [20260127-181716] Fix GitHub Actions release permissions
- **开始时间**: 2026-01-27 18:17:16
- **完成时间**: 2026-01-27 18:18:00
- **类型**: feat(ci)
- **描述**: Fix GitHub Actions release permissions

## [20260127-180856] Fix arm64 CI linking and refine distribution packaging
- **开始时间**: 2026-01-27 18:08:56
- **完成时间**: 2026-01-27 18:11:31
- **类型**: feat(ci)
- **描述**: Fix arm64 CI linking and optimize skill package

## [20260127-174839] Fix GitHub Actions musl-cross installation
- **开始时间**: 2026-01-27 17:48:39
- **完成时间**: 2026-01-27 17:52:21
- **类型**: feat(ci)
- **描述**: Refactor release workflow to use matrix strategy for stability

## [20260127-173755] Finalize release workflow and add bilingual README
- **开始时间**: 2026-01-27 17:37:55
- **完成时间**: 2026-01-27 17:42:18
- **类型**: feat(git)
- **描述**: Fix release sync and add comprehensive README

## [20260127-165006] Optimize SKILL.md for AI assistants
- **开始时间**: 2026-01-27 16:50:06
- **完成时间**: 2026-01-27 17:25:01
- **类型**: feat(docs)
- **描述**: Refine SKILL.md lifecycle and documentation rules

## [20260127-161237] Enhance bootstrapping and remote robustness
- **开始时间**: 2026-01-27 16:12:37
- **完成时间**: 2026-01-27 16:16:18
- **类型**: feat(git)
- **描述**: Enhance bootstrapping and remote robustness

## [20260127-155654] Improve robustness and error handling
- **开始时间**: 2026-01-27 15:56:54
- **完成时间**: 2026-01-27 16:03:39
- **类型**: fix(git)
- **描述**: Improve robustness and error handling

## [20260127-153331] Support remote sync, changelog safety, and GitHub Actions
- **开始时间**: 2026-01-27 15:33:31
- **完成时间**: 2026-01-27 15:50:49
- **类型**: feat(git)
- **描述**: Refine remote sync, safety, and documentation

## [20260127-145625] Standardize Git workflow and enhance taskflow
- **开始时间**: 2026-01-27 14:56:25
- **完成时间**: 2026-01-27 14:58:55
- **类型**: feat(git)
- **描述**: Standardize Git workflow and enhance taskflow

## [20260127-142403] Support linux/windows arm64 builds and manage .cargo config tracking
- **开始时间**: 2026-01-27 14:24:03
- **完成时间**: 2026-01-27 14:35:53
- **类型**: feat(build)
- **描述**: Support linux_arm64 and individual platform builds

## [20260127-132955] Finalize taskflow migration and record status
- **开始时间**: 2026-01-27 13:29:55
- **完成时间**: 2026-01-27 14:08:54
- **类型**: feat(skill)
- **描述**: Finalize Taskflow Rust Migration