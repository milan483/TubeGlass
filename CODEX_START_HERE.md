# Start Here: ChatGPT-planned work, Codex execution

This repository uses GitHub as the durable handoff between planning and implementation. ChatGPT prepares requirements and issues; Codex owns local code changes, tests, commits, and task-branch delivery.

## When starting a task
1. Read `AGENTS.md`, this file, and the repository's relevant documentation.
2. Identify the exact GitHub issue or task specification. If the user says `finish #N`, read issue `#N` including relevant comments and linked context. If no task or issue is identified, ask rather than inventing work.
3. Inspect the current branch, working tree, and relevant implementation before editing. Preserve all existing user changes.
4. Give a short plan and note only material uncertainties. Then implement the smallest complete change that satisfies the acceptance criteria.
5. Run the relevant checks listed in `AGENTS.md`; add or update tests when appropriate.
6. Commit and push the task branch, then open or update a pull request when possible. Never merge it.

## Standard Codex prompt

> Read `CODEX_START_HERE.md` and `AGENTS.md`. Implement GitHub issue #<NUMBER> according to its requirements and acceptance criteria. Inspect the current repository state first, outline a concise plan, make the smallest complete change, run the required checks, commit and push a dedicated task branch, and open a pull request if possible. Do not merge. Report the branch, commit SHA, PR URL, exact validation results, and unresolved risks. Do not claim tests passed unless they actually ran.

Replace `<NUMBER>` with the real issue number.

## Handoff back to ChatGPT
Send ChatGPT the PR URL after pushing. ChatGPT will review the actual diff against the issue's acceptance criteria and available CI results, then return specific findings or a recommendation to merge. A pushed branch or successful local test report is not, by itself, approval to merge.
