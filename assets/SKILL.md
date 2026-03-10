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

| Dimension | Description | Key Questions |
|---|---|---|
| Context Completeness | Are there hidden assumptions the human made? | Did they point to the right initial files or architecture docs? Did the agent have to fish for context? |
| Instruction Clarity | Semantic precision of the task directives | Was the human's intention unambiguous? Did it prevent the agent from heading down the wrong technical path? |
| Requirement Evolution | How requirements changed during the conversation | Were changes abrupt or well-motivated? Did communication gaps cause rework? |
| Course Correction | Effectiveness of human intervention | When the agent got stuck, did the human provide effective unblocking hints, or did they provide vague directives? |

### Grading Rubric

Use discrete letter grades for each dimension:

| Grade | Meaning |
|---|---|
| **A** | Excellent. Optimal context engineering. |
| **B** | Good. Minor friction. |
| **C** | Adequate. Notable gaps that caused rework. |
| **D** | Poor. Significant issues leading to wasted effort. |

### Output Template

Use the following human-centric markdown structure:

```markdown
# Prompt Coaching: [topic or task-id]

## 1. 发生了什么 (What Happened)
<!-- 1-2 sentence objective summary of the task and where any friction occurred. -->

## 2. 评分 (Evaluation)

| Dimension | Grade | Summary |
|---|---|---|
| Context Completeness | _ | ... |
| Instruction Clarity | _ | ... |
| Requirement Evolution | _ | ... |
| Course Correction | _ | ... |

## 3. 需要优化的部分 (Areas for Improvement)
<!-- Abstract the frictions into general categories. Provide high-level advice on how the human should reason about supplying context. 
  - DO NOT praise the user in this section. Only list what needs to be improved based on the friction that occurred.
  - E.g., "When refactoring database schemas, always provide the struct definition files." 
-->

## 4. 可以继续保持的习惯 (What to Keep Doing)
<!-- 1-3 concrete bullet points highlighting things the user did *exceptionally well* that actively helped the agent.
  - CRITICAL RULE: You MUST verify the conversation history. DO NOT list hypothetical or generic best practices here. ONLY list actions the user *actually performed* in this specific task.
  - If the user did nothing exceptionally helpful, OMIT this section entirely or write "无特别值得提取的习惯".
-->
```

### Rules
1. **Target the Human**: Address the user directly as a collaborator. Answer "How can *you* help *me* help *you* better?".
2. **Be Constructive and Meta**: Do not just list "what I did". Generalize the experience into reusable prompt engineering rules.
3. **No Unnecessary Praise**: The "Areas for Improvement" section must strictly focus on optimization opportunities without padding it with praise.
4. **Strict Factual Validation**: The "What to Keep Doing" section must *only* reflect actual actions taken by the user. Never hallucinate user actions.
5. **Language Alignment**: Check the `CHANGELOG.md` for the current task's language metadata (e.g., `[lang:zh]`).
    - If specified, generate the report in that language.
    - If not specified, detect and use the primary language used by the user in the current conversation.
    - Fallback to **English** if the language is unclear or not detected.