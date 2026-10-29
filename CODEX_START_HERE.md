# Start Here

Use this when asked to execute a GitHub issue. `AGENTS.md` contains the persistent project rules; the issue contains the task requirements.

## Copy-paste prompt for Codex

> Read `AGENTS.md` and GitHub issue #<NUMBER>, including relevant comments and links. Inspect the current branch, working tree, and relevant code/tests. State a concise plan, then implement the smallest complete change that meets the issue's acceptance criteria. If the task triggers a planning gate in `AGENTS.md`, stop before editing and ask for approval. Run relevant tests and report exact results. Commit and push a dedicated task branch and open a PR; do not merge. Finish with a summary, key files, validation results, branch/commit SHA, PR URL, and unresolved risks.

Replace `<NUMBER>` with the actual issue number. Send the resulting PR URL to ChatGPT for review against the issue and available CI results.