# Start Here

Use this when asked to execute a task. `AGENTS.md` contains persistent project rules; the assigned issue or linked task specification contains the requirements.

## Copy-paste prompt for an issue

> Read `AGENTS.md` and GitHub issue #<NUMBER>, including relevant comments and links. Inspect the current branch, working tree, and relevant code/tests. State a concise plan, then implement the smallest complete change that meets the issue's acceptance criteria. If the task triggers a planning gate in `AGENTS.md`, stop before editing and ask for approval. Run relevant tests and report exact results. Commit and push a dedicated task branch and open a PR; do not merge. Finish with a summary, key files, validation results, branch/commit SHA, PR URL, and unresolved risks.

## When GitHub Issues are disabled

Use the linked task specification, normally under `.ai/tasks/`. If it is attached to an existing draft PR, continue on that PR's head branch, update the same PR with implementation and validation, and do not create a duplicate PR.

Send the resulting PR URL to ChatGPT for review against the task's acceptance criteria and available CI results.
