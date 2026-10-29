# Start Here

Use this guide when executing a task from a GitHub issue or a linked task specification. `AGENTS.md` contains the persistent project rules; the issue or task packet contains the task-specific requirements and research.

## Copy-paste prompt for Codex

> Implement the task described in [issue/task packet link]. Read `AGENTS.md` and the task specification. Verify the current branch, working tree, and HEAD against the packet's source snapshot; preserve existing user changes. Reuse the packet's codebase findings and research: inspect the referenced local code and necessary dependencies, but do not repeat broad repository exploration or external research unless the packet is stale, incomplete, or contradicted by the checkout. If the task is well-specified and low-risk, implement directly. If it triggers an approval gate in `AGENTS.md`, stop before editing and ask for approval. Run focused tests and the required validation commands, report exact outcomes, and update the existing task PR when one is linked. Do not create a duplicate PR or merge. Finish with a summary, changed files, validation results, branch/commit SHA, PR URL, and unresolved risks.

## When GitHub Issues are disabled

Keep the task specification in `.ai/tasks/<task>.md` and link it from a draft PR. Codex should continue on that PR's head branch and update the same PR with implementation and validation results.
