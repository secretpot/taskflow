---
name: taskflow
description: Manage task lifecycle with automatic ID generation, CHANGELOG updates, and git commits following project conventions.
---

# Task Flow Management Skill
This skill provides complete task lifecycle management, including task ID generation, CHANGELOG updates, and Git commits following standardized conventions.

## When to Use
**CRITICAL RULE**: **NEVER generate code or execute shell commands unless they are attached to an Active Task ID**. For tasks that do not involve code modification or system changes (e.g., code interpretation), a task ID is not required.

## Usage Instructions

### 1. Open a New Task
When Agent begins implementing a new requirement:

```bash
.agent/skills/taskflow/bin/taskflow open "Brief task description" [optional-slug]
```

**Actions**:
- Switches to `dev` branch and runs `git pull`.
- Generates task ID (e.g., `20260121-173243`).
- Creates and switches to a new task branch: `task/<ID>[-slug]`.
- Adds task entry to `.agent/docs/CHANGELOG.md` and **immediately commits it** to lock the task.

### 2. Close a Task

#### Step 1: Prepare Before Closing
1. **Update Documentation**: Update files in `.agent/docs/project/`.
2. **Verify Work**: Pass tests and lint.

#### Step 2: Run Close Command
```bash
.agent/skills/taskflow/bin/taskflow close "<type>" "<scope>" "<subject>" "<body>" "[footer]"
```

**Actions**:
1. Commits changes to the task branch.
2. Switches to `dev` and merges the task branch.
3. **Automatically pushes `dev` to origin**.
4. Deletes the local task branch.

### 3. Release a Version
Standardize the release process from `dev` to `main` with automated CI/CD.

```bash
.agent/skills/taskflow/bin/taskflow release <version>
```

**Prerequisites**:
- Must be on `dev` branch.
- Working directory must be clean.
- Remote `origin` must be configured.

**Actions**:
1. Merges `dev` into `main`.
2. Bumps version in `Cargo.toml`.
3. Creates a Git tag (e.g., `v0.1.1`).
4. **Automatically pushes `main` and Tags to origin**.
5. Returns to `dev`.
6. **GitHub Action** triggers to build and publish cross-platform binaries.

### 4. Get Current Task Status
```bash
.agent/skills/taskflow/bin/taskflow status
```

## Bootstrapping a New Project
If you are starting a brand new project (empty repository):

1. **First Open**: Run `taskflow open "initial task"`. It will detect the empty repo and skip the `dev` branch check.
2. **First Close**: Run `taskflow close ...`. It will detect the missing `dev` branch and create it for you from your first task's commit.

## Remote Synchronization Behavior
The tool provides visual feedback for remote operations:
- **ℹ️ Information**: Remote not configured for the repository. The tool will skip pushing without erroring.
- **⚠️ Warning**: Remote exists but communication failed (e.g., authentication or network issues).
- **✅ Success**: Changes successfully synced with the remote.

## File Structure
```
.agent/skills/taskflow/
├── SKILL.md                    # This file
└── bin/
    └── taskflow               # Binary for the current OS
```

## Git Commit Message Format
All commits follow this format:
```
<type>(<optional-scope>): [<task-id>] <subject>

<body>

<optional-footer>
```

### Commit Body Rules
- Explains **WHY** the change was made.
- **MUST** use a bulleted list (using `-`).
- Each point **MUST** be on a new line.
- **NEVER** use numbered lists.
- Wrap at 72 characters per line.