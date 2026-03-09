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
Invoke the tool directly using the `taskflow` command.

### 0. Installation & Self-Healing
If the `taskflow` command is not found in your environment, you MUST attempt to install it before proceeding:
- **Unix (macOS/Linux)**: Run `bash scripts/install.sh`
- **Windows**: Run `scripts\install.bat`

Once installed, use the command as described below.

### 1. Open a New Task
**Intent**: Initialize the development context for a new piece of work.
**Project Impact**:
- **Classic mode**: Switches to a task-specific branch and initializes a tracked CHANGELOG entry.
- **Parallel mode**: Creates a new worktree under `tasks/` and outputs `WORKTREE_PATH:`. You MUST `cd` to that path for all subsequent work.

```bash
taskflow open "Brief task description" [topic] [--lang <lang-code>]
```

> [!TIP]
> Use `taskflow --version` to check the current tool version.

### 2. Close a Task
**Intent**: Finalize work and integrate it into the main development branch.
**Project Impact**:
- **Classic mode**: Commits staged changes, merges to `dev`, deletes the task branch, and pushes to origin.
- **Parallel mode**: Commits, merges to the configured base branch, removes the worktree, and pushes. Return to the management root after close.

**MANDATORY PRE-CLOSE CHECKLIST**:
1. **Update Documentation**: **ALL** relevant files in `.agent/docs/project/` MUST be updated to reflect current implementation.
2. **Verification**: Confirm all tests pass and lints are clean.
3. **Prompt Coaching**: Generate a prompt coaching report at `docs/prompt-coaching/<topic>-<task-id>.md` (see [Prompt Coaching](#prompt-coaching)). **The `close` command will REJECT the operation if this file is missing.**
4. **Staging**: Stage relevant files (auto-staging is enabled by default).

```bash
taskflow close "<type>" "<scope>" "<subject>" "<body>" "[footer]"
```

### 3. Release a Version
**Intent**: Promote stable code from `dev` to `main` for distribution.
**Project Impact**: Creates a production version, tags the commit, and triggers CI/CD via GitHub Actions.

```bash
taskflow release <version>
```

### 4. Get Current Task Status
**Intent**: Verify the active developmental context.
**Project Impact**: In Classic mode, returns the current Task ID and branch. In Parallel mode, lists all active task worktrees.

```bash
taskflow status
```

### 5. Initialize Parallel Mode
**Intent**: Convert a repository into a management root for multi-agent parallel development.
**Project Impact**: Creates `.agent/config.toml`, a base branch worktree, and a `tasks/` directory. One-time setup.

```bash
taskflow init [--branch <base-branch-name>]
```

If `--branch` is omitted, the current branch is auto-detected and used as the base branch.

## Parallel Mode: Agent Guidance

When working in a project initialized with `taskflow init`:

1. **After `taskflow open`**: The output contains a `WORKTREE_PATH:` line. You MUST change your working directory to that path. All file edits, test runs, and commits happen inside the worktree.
2. **After `taskflow close`**: The worktree is removed. Return to the management root directory.
3. **File paths**: Always use paths relative to the worktree root, not the management root.

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

## Prompt Coaching

**Purpose**: Analyze the user's instruction quality throughout the conversation and generate actionable feedback to improve their context engineering skills. The goal is to reduce future conversation turns, cognitive load, and LLM token consumption.

**Output Path**: `docs/prompt-coaching/<topic>-<task-id>.md` (or `docs/prompt-coaching/<task-id>.md` if no topic was provided)

**CRITICAL**: The `close` command performs a **hard gate check** on this file. If the file does not exist, the close operation will fail. You MUST generate this document before running `close`.

### Analysis Dimensions

Evaluate the user's prompts across these four dimensions:

| Dimension | Description | Key Questions |
|---|---|---|
| Context Completeness | Whether sufficient background was provided | Was critical context missing? Were assumptions required? Was information disclosed progressively and logically? |
| Instruction Clarity | Semantic precision of the task directives | Were instructions unambiguous? Was structured formatting used (lists, headings)? Were success criteria defined? |
| Requirement Evolution | How requirements changed during the conversation | Were changes abrupt or well-motivated? Did communication gaps cause rework? |
| Conversation Efficiency | Whether unnecessary clarification rounds occurred | Could any back-and-forth have been avoided with better upfront context? |

### Grading Rubric

Use discrete letter grades for each dimension:

| Grade | Meaning |
|---|---|
| **A** | Excellent. No improvement needed. |
| **B** | Good. Minor improvements possible. |
| **C** | Adequate. Notable gaps that caused friction. |
| **D** | Poor. Significant issues that led to wasted effort. |

### Output Template

Use the following markdown structure:

```markdown
# Prompt Coaching: [task-id]

## Conversation Overview
<!-- 1-2 sentence summary of what was accomplished -->

## Evaluation

| Dimension | Grade | Summary |
|---|---|---|
| Context Completeness | _ | ... |
| Instruction Clarity | _ | ... |
| Requirement Evolution | _ | ... |
| Conversation Efficiency | _ | ... |

## Detailed Analysis
<!-- For each dimension graded B or below, provide:
     1. What happened (specific example from conversation)
     2. What could have been done differently
     3. Suggested rewrite or approach -->

## Actionable Takeaways
<!-- Bulleted list of 2-5 concrete, specific improvements.
     Each must be actionable (start with a verb). -->
```

### Rules

1. **User-scope only**: Evaluate only what the user could have controlled. Do NOT critique the agent's own failures.
2. **Be specific**: Every piece of feedback must reference a concrete moment in the conversation. Avoid generic advice.
3. **Be constructive**: Frame all feedback as opportunities, not criticisms.
4. **Proportional response**: If the conversation was short (1-2 turns) and instructions were clear, output a brief positive acknowledgment rather than forcing analysis.
5. **No fluff**: Do not pad the document with boilerplate praise. Every sentence must carry information.
6. **Language Alignment**: Check the `CHANGELOG.md` for the current task's language metadata (e.g., `[lang:zh]`).
    - If specified, generate the report in that language.
    - If not specified, detect and use the primary language used by the user in the current conversation.
    - Fallback to **English** if the language is unclear or not detected.