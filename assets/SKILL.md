---
name: taskflow
description: Manage task lifecycle with automatic ID generation, CHANGELOG updates, and git commits following project conventions.
---

# Task Flow Management Skill
This skill provides complete task lifecycle management, ensuring every change is tracked, documented, and follows project standards.

## When to Use
Activate this skill at the beginning of every technical task:
1. **New Requirement**: Before writing code for a feature or improvement.
2. **Bug Fix**: Before starting the debugging or fixing process.
3. **Refactor**: Before restructuring existing code.

**CRITICAL RULE**: **NEVER generate code or execute shell commands unless they are attached to an Active Task ID**. Working without an active task violates project traceability standards.

## Usage Instructions
Use the absolute path to the binary appropriate for your operating system (e.g., `.agent/skills/taskflow/bin/taskflow`).

### 1. Open a New Task
**Intent**: Initialize the development context for a new piece of work.
**Project Impact**: Switches to a task-specific branch and initializes a tracked CHANGELOG entry.

```bash
<path_to_binary> open "Brief task description" [optional-slug]
```

### 2. Close a Task
**Intent**: Finalize work and integrate it into the main development branch.
**Project Impact**: Commits staged changes, merges to `dev`, deletes the local task branch, and pushes to origin.

**MANDATORY PRE-CLOSE CHECKLIST**:
1. **Update Documentation**: **ALL** relevant files in `.agent/docs/project/` MUST be updated to reflect current implementation.
2. **Verification**: Confirm all tests pass and lints are clean.
3. **Staging**: Stage relevant files (auto-staging is enabled by default).

```bash
<path_to_binary> close "<type>" "<scope>" "<subject>" "<body>" "[footer]"
```

### 3. Release a Version
**Intent**: Promote stable code from `dev` to `main` for distribution.
**Project Impact**: Creates a production version, tags the commit, and triggers CI/CD via GitHub Actions.

```bash
<path_to_binary> release <version>
```

### 4. Get Current Task Status
**Intent**: Verify the active developmental context.
**Project Impact**: Returns the current Task ID and branch name. Always returns code 0 if the tool is initialized.

```bash
<path_to_binary> status
```

## Side Effects & Remote Sync
The tool automatically manages remote synchronization:
- **✅ Success**: Changes successfully synced with origin.
- **ℹ️ Information**: Remote not configured; push skipped gracefully.
- **⚠️ Warning**: Push failed due to network or authentication issues.

## Git Commit Message Format
All commits follow this format:
```
<type>(<optional-scope>): [<task-id>] <subject>

<body>

<optional-footer>
```

### Commit Body Rules
- Explains **WHY** the change was made (the intent).
- **MUST** use a bulleted list (using `-`).
- Each point **MUST** be on a new line.
- Wrap at 72 characters per line.