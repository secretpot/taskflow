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

**Purpose**: Generate a coaching document tailored for the *human user* to help them improve their prompts in the future. The feedback should abstract away from the specific bug/feature and focus on *meta-level* advice on Context Engineering and Instruction formulation. The goal is to answer for the user: "Next time I assign a similar task, what should I provide upfront and how should I phrase my request so the agent can nail it in one try?"

**Output Location Rules**: 
A task ID looks like `YYYYMMDD-HHMMSS`. You must parse the task ID into date (`YYYYMMDD`) and time (`HHMMSS`).
**Output Path**: `docs/prompt-coaching/<YYYYMMDD>/<HHMMSS>-<topic>.md` (or `docs/prompt-coaching/<YYYYMMDD>/<HHMMSS>.md` if no topic was provided). Example: `docs/prompt-coaching/20260310/093708-unify-config.md`.

**CRITICAL**: The `close` command performs a **hard gate check** on this file. If the file does not exist at the exact expected path, the close operation will fail. You MUST generate this document and its parent directories before running `close`.

### Analysis Focus

Evaluate the user's methodology across these areas:
- **Context Completeness**: Are there hidden assumptions the human made? Did they point to the right initial files or architecture docs? Did the agent have to fish for context?
- **Instruction Clarity**: Was the human's intention unambiguous? Did it prevent the agent from heading down the wrong technical path? 
- **Course Correction**: When the agent got stuck, did the human provide effective unblocking hints, or did they provide vague directives?

### Output Template

Use the following human-centric markdown structure:

```markdown
# Prompt Coaching: [topic or task-id]

## 1. What Happened
<!-- 1-2 sentence objective summary of the task and where any friction occurred. -->

## 2. Abstractions & Meta-Advice
<!-- Abstract the frictions into general categories. E.g., instead of "You didn't give me user.rs", say "When refactoring database schemas, always provide the struct definition files." Provide high-level advice on how the human should reason about supplying context. -->

## 3. Actionable "Next Time" Checklist
<!-- 2-4 concrete bullet points for the human. Start with strong verbs.
  - e.g., "Provide the specific CLI command output when reporting a crash."
  - e.g., "Explicitly define the accepted file paths when requesting refactoring."
-->
```

### Rules
1. **Target the Human**: Address the user directly as a collaborator. Answer "How can *you* help *me* help *you* better?".
2. **Be Constructive and Meta**: Do not just list "what I did". Generalize the experience into reusable prompt engineering rules.
3. **Praise Good Practice**: If the human's initial prompt was perfect and the task was finished without friction, acknowledge what they did right so they can reproduce that success.
4. **Language Alignment**: Check the `CHANGELOG.md` for the current task's language metadata (e.g., `[lang:zh]`).
    - If specified, generate the report in that language.
    - If not specified, detect and use the primary language used by the user in the current conversation.
    - Fallback to **English** if the language is unclear or not detected.