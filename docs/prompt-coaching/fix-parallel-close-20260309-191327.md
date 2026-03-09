# Prompt Coaching: 20260309-191327

## Conversation Overview
While executing the end-to-end test for parallel mode `init`, `open`, and `close`, we uncovered a critical bug in `close_task_parallel`. After removing the worktree, the Git commands (`merge`, `branch -D`, `push`) were still executing from the deleted worktree's path, causing `git checkout develop failed` and other fatal errors because the current working directory no longer existed. We fixed this by introducing `merge_in`, `delete_branch_in`, and `push_in` to explicitly run those final cleanup and sync commands from the valid management root directory.

## Evaluation

| Dimension | Grade | Summary |
|---|---|---|
| Context Completeness | A | We perfectly identified the root cause: running git commands in a destroyed directory path. |
| Issue Isolation | A | We isolated the problem to `get_current_branch` failing within `delete_branch` due to CWD destruction, and then correctly traced the issue to `push` doing the same thing. |
| API Design | A | We added `_in` variants of the git functions to allow explicit directory context targeting (`run_git_in`). |
| Testing Thoroughness | A+ | The creation of the E2E bash script caught this bug automatically before release, proving the value of end-to-end sandbox testing. |

## Actionable Takeaways
- Always be mindful of the Current Working Directory (CWD) when designing CLI applications, especially when the application's core feature is destroying directories!
- Use explicit directory targeting (`Command::current_dir()`) instead of relying on inherited environments whenever possible to avoid race conditions and CWD disappearance bugs.
