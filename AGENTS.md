# Codex project instructions

## Project context
- TubeGlass is a Rust 2024 desktop client built with eframe/egui. Read `README.md` and `Cargo.toml` when project-wide assumptions or build details matter.
- The assigned GitHub issue or linked task specification is the source of truth for scope and acceptance criteria.
- `.ai/CHATGPT_TASK_PREP.md` describes the ChatGPT-side research and handoff workflow. The task packet should contain the relevant findings; do not read that guide for every implementation task unless the workflow itself is being changed.

## Before editing
- Read this file and the assigned issue or task packet. Verify the repository, current branch, working tree, and HEAD against the packet's source snapshot. Preserve existing user changes; never reset, overwrite, or discard them.
- Treat the packet's codebase findings, design decisions, and research as precomputed context. Verify the referenced local code and any directly necessary dependencies, but do not repeat broad repository exploration, full-file reading, or external research when the packet already answers the question.
- Expand investigation only when the source snapshot is stale, local code contradicts the packet, a required implementation detail is missing, or implementation reveals a material issue. Keep additional exploration targeted. Report material contradictions or new decisions instead of silently changing scope.
- For a well-specified, low-risk task, implement directly; do not add a separate user-facing planning round just to restate the packet. If a short plan helps coordinate a genuinely multi-step change, state it and proceed. If requirements are materially ambiguous, or the work is architectural/cross-cutting, security- or privacy-sensitive, changes persistent data formats, migrations, dependencies, packaging, or release behavior, summarize the proposed approach and ask for approval before editing.
- Make the smallest complete change. Avoid unrelated refactors and new dependencies unless the task justifies them. Follow existing architecture and naming.
- Never put credentials, tokens, or other secrets in issues, commits, logs, or reports.

## Validation
- Add or update focused tests where appropriate; run targeted tests first when they give fast feedback.
- For ordinary Rust code changes, run `cargo fmt --check`, `cargo build --release --locked`, and `cargo test --release --locked` where the environment permits. Use the repository's documented commands if they differ.
- Do not run broad ignored or network-dependent tests unless relevant and justified. Run any task-specific checks needed to verify the change.
- Report each command as **passed**, **failed**, or **not run**. Never claim a check passed unless it actually ran successfully. Explain environmental blockers and distinguish local results from GitHub CI results.

## Git and delivery
- Use a dedicated task branch and focused commits. Never push directly to the default branch or force-push.
- If the task packet points to an existing task PR, continue on that PR's head branch and update the same PR. Do not create a duplicate branch or PR for the same task.
- After implementation and validation, push the task branch and open or update a pull request unless the user asks otherwise. Never merge a PR, publish a release, or change repository settings without explicit user approval.
- End with a compact handoff report containing: summary, key files changed, exact validation results, branch and commit SHA, PR URL, and unresolved risks.
