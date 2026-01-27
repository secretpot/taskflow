---
name: taskflow
description: Manage task lifecycle with automatic ID generation, CHANGELOG updates, and git commits following project conventions.
---

# Task Flow Management Skill
This skill provides complete task lifecycle management, including task ID generation, CHANGELOG updates, and Git commits following standardized conventions.

## When to Use
**CRITICAL RULE**: **NEVER generate code or execute shell commands unless they are attached to an Active Task ID**. For tasks that do not involve code modification or system changes (e.g., code interpretation), a task ID is not required.

Use this skill when:
- Agent starts implementing a new requirement → **Open** a new task
- Task is completed → **Close** the task (Generate CHANGELOG entry and create Git commit)
- Need to check current active task ID
- Need to view task history

## How It Works
### Task ID Format
Task IDs use timestamp format to ensure chronological ordering:
```
YYYYMMDD-HHMMSS
```
Example: `20260121-173243`
### Three Core Functions
1. **Open Task** - Generate new task ID and save to CHANGELOG
2. **Close Task** - Update CHANGELOG and create Git commit
3. **Get Current Task** - Display current active task ID and description

## Usage Instructions
### 1. Open a New Task
When Agent begins implementing a new requirement:

```bash
.agent/skills/taskflow/bin/taskflow open "Brief task description"
```
**Example**:
```bash
.agent/skills/taskflow/bin/taskflow open "Implement user authentication system"
```
**Actions Performed**:
- Checks if a task is already in progress (enforces serialization)
- Generates task ID (e.g., `20260121-173243`)
- Adds task entry to top of `docs/CHANGELOG.md` (relative to current working directory)
- Saves task ID as HTML comment in CHANGELOG header

### 2. Close a Task

#### Step 1: Prepare Before Closing
Before running the close script, complete the following:

1. **Update Project Documentation** (CRITICAL): If the task introduced new features, changed architecture, or modified APIs, update the relevant files in `.agent/docs/project/`:
   - `overview.md`: Update project structure, features, or usage if changed
   - `design.md`: Update architecture decisions or design patterns if changed
   - `background.md`: Update if project goals or context change
   - **Record current state**: Ensure the documentation accurately reflects the project's current state so work can be resumed seamlessly.

2. **Verify Your Work**: Ensure all tests pass and linter issues are resolved.

#### Step 2: Run the Close Command
```bash
.agent/skills/taskflow/bin/taskflow close "<type>" "<scope>" "<subject>" "<body>" "[footer]"
```

**Parameters**:
| Parameter | Required | Description                                          |
| --------- | -------- | ---------------------------------------------------- |
| `type`    | Yes      | Commit type: `feat`, `fix`, `refactor`, `docs`, etc. |
| `scope`   | No       | Scope of changes (use `""` for no scope)             |
| `subject` | Yes      | Brief description (≤50 characters)                   |
| `body`    | Yes      | Detailed explanation (intent and approach)           |
| `footer`  | No       | Optional: `Closes #123`, `BREAKING CHANGE: ...`      |

**Example**:
```bash
bash .agent/skills/taskflow/scripts/close_task.sh \
  "feat" \
  "auth" \
  "Implement user authentication system" \
  "Added JWT-based authentication with login, registration, and token refresh. Uses bcrypt for password hashing to ensure secure storage."
```

#### Step 3: What the Script Does
The script automatically performs:
1. Updates task entry in `.agent/docs/CHANGELOG.md` (adds completion time)
2. Removes task ID comment from CHANGELOG header
3. Stages all files with `git add .` (disable with `TASKFLOW_AUTO_STAGE=false`)
4. Creates Git commit following the standard format

**Disable Auto-Staging** (optional):
```bash
TASKFLOW_AUTO_STAGE=false bash .agent/skills/taskflow/scripts/close_task.sh ...

### 3. Get Current Task Status
```bash
.agent/skills/taskflow/bin/taskflow status
```
**Output**: Current active task ID and description, or error if none exists.

## File Structure
```
.agent/skills/taskflow/
├── SKILL.md                    # This file
└── bin/
    └── taskflow               # Binary for the current OS
```

## CHANGELOG Format
`.agent/docs/CHANGELOG.md` follows this format:
```markdown
# CHANGELOG
<!-- CURRENT_TASK: 20260121-173243 -->

## [20260121-173243] Implement user authentication system
- **开始时间**: 2026-01-21 17:32:43
- **完成时间**: 2026-01-21 18:45:20
- **类型**: feat(auth)
- **描述**: Added JWT-based authentication system

## [20260121-143022] Fix database connection pool issue
- **开始时间**: 2026-01-21 14:30:22
- **完成时间**: 2026-01-21 15:10:45
- **类型**: fix(db)
- **描述**: Fixed connection pool exhaustion problem
```

## Git Commit Message Format

All commits must follow this standardized format:

```
<type>(<optional-scope>): [<task-id>] <subject>

<body>

<optional-footer>
```
### Commit Type
Allowed types:
- `feat`: New feature
- `fix`: Bug fix
- `perf`: Performance optimization
- `refactor`: Code refactoring
- `docs`: Documentation updates
- `style`: Code formatting (no logic changes)
- `test`: Test-related changes
- `build`: Build system changes
- `revert`: Revert previous commit
- `ci`: CI/CD configuration
- `chore`: Miscellaneous tasks
- `release`: Version release
- `workflow`: Workflow changes
### Commit Scope
Optional. Indicates the area of change (e.g., `auth`, `api`, `db`, `ui`).
### Commit Subject
- Maximum 50 characters
- Brief, imperative description
- No period at the end
### Commit Body
- Explains **WHY** the change was made and **HOW** it was approached
- For multiple points or features, **MUST** use a bulleted list (using `-`)
- Each point **MUST** be on a new line
- **NEVER** use numbered lists (e.g., `1.`, `2.`)
- Does NOT describe implementation details
- Wrap at 72 characters per line
### Commit Footer
Optional. Used for:
- Issue references: `Closes #123`, `Fixes #456`
- Breaking changes: `BREAKING CHANGE: description`
- Co-authors: `Co-authored-by: Name <email>`
### Complete Commit Example
```
feat(auth): [20260121-173243] Implement user authentication system

- Added JWT-based authentication system including login and registration.
- Implemented token refresh functionality.
- Uses bcrypt for password hashing to ensure secure password storage.
- Implements role-based access control for different user types.

Closes #123
```
### More Examples
```
fix(db): [20260121-143022] Fix connection pool exhaustion

- Modified connection pool configuration to properly release connections after use.
- Increased max pool size and added connection timeout handling.

Fixes #456
```

```
refactor(api): [20260121-095530] Simplify error handling middleware

Consolidated duplicate error handling logic into a single middleware
function. Improves maintainability and reduces code duplication.
```

```
docs: [20260121-112015] Update API documentation

Added missing endpoint descriptions and request/response examples.
Clarified authentication requirements for protected routes.
```

## Workflow Example
### Complete Task Lifecycle
```bash
# 1. Open new task
.agent/skills/taskflow/bin/taskflow open "Add user profile page"
# Output: Task ID: 20260121-173243

# 2. Agent performs development work...
# (Write code, run tests, docs update, etc.)

# 3. Close task (auto-staging is enabled by default)
.agent/skills/taskflow/bin/taskflow close \
  "feat" \
  "profile" \
  "Add user profile page" \
  "Implemented user profile viewing and editing functionality, including avatar upload and basic information modification. Uses React Hook Form for form validation."
```

## Error Handling
The scripts perform the following checks:
- **Open task**: If an active task exists, errors and prevents overwriting unless manually cleared
- **Close task**: If no active task exists, exits with error
- **Git commit**: Checks for staged changes before committing
- **Missing parameters**: Displays usage instructions and exits

## Important Notes
1. **Task ID Uniqueness**: Based on timestamp; avoid opening multiple tasks within the same second
2. **CHANGELOG Location**: Always at `.agent/docs/CHANGELOG.md`; created automatically if missing
3. **Task ID Storage**: Stored as HTML comment (`<!-- CURRENT_TASK: ID -->`) in CHANGELOG header
4. **Git Staging**: Files are auto-staged by default with `git add .`; disable with `TASKFLOW_AUTO_STAGE=false` if needed
5. **Auto-Stage Behavior**: Enabled by default; uses `git add .` to stage all modified and new files
6. **File Exclusion**: Use `.gitignore` to exclude files from tracking; use `git rm` to untrack specific files
7. **Local Only**: All operations are local; no automatic push to remote repository
8. **Work Locally Only**: Remote pushes and PRs are handled manually by the user to ensure full control over repository state

## Environment Variables
- `TASKFLOW_AUTO_STAGE`: Set to `false` to disable automatic staging of files with `git add .` before commit (default: `true`, auto-staging enabled)

## Commit Guidelines Summary
### Before Committing
- Ensure all tests pass
- Run linter and fix any issues
- Stage only relevant changes
- Write meaningful commit messages
### Commit Message Rules
- Use imperative mood ("Add feature" not "Added feature")
- Subject line ≤50 characters
- Body wraps at 72 characters
- Separate subject and body with blank line
- Include task ID in subject line
- Explain WHY, not WHAT (code shows WHAT)
### After Committing
- Verify commit message format
- Check that CHANGELOG was updated
- Ensure task ID comment was removed from CHANGELOG header