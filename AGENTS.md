# Codex project instructions

## Project context
- TubeGlass is a Rust 2024 desktop client built with eframe/egui. Read `README.md` and `Cargo.toml` before making project-wide assumptions.
- The assigned GitHub issue or explicitly linked task specification is the source of truth for scope and acceptance criteria.
- If GitHub Issues are disabled, a task may be specified in `.ai/tasks/<task>.md` and linked from the task PR.

## Before editing
- Inspect the current branch, working tree, task specification, and relevant source/tests. Preserve existing user changes; never discard or overwrite them.
- State a concise implementation plan before editing.
- For a clear, low-risk task, proceed after the brief plan. If requirements are materially ambiguous, or the work is architectural/cross-cutting, security- or privacy-sensitive, changes persistent data formats, migrations, dependencies, packaging, or release behavior, stop after planning and ask for approval before editing.
- When executing a task that already has a task-spec PR, continue on that PR's head branch and update the same PR; do not open a duplicate PR.
- Make the smallest complete change. Avoid unrelated refactors and new dependencies unless the task justifies them.
- Follow the existing architecture and naming. Keep Rust module descriptions and function documentation consistent with nearby code.
- Never put credentials, tokens, or other secrets in issues, commits, logs, or reports.

## Validation
- Run relevant checks and add or update tests where appropriate.
- For ordinary Rust code changes, run `cargo fmt --check`, `cargo build --release`, and `cargo test --release` where the environment permits.
- Ignored tests can contact YouTube and require network access. Run `cargo test --release -- --ignored` only when relevant and justified by the task.
- Report each command as **passed**, **failed**, or **not run**. Never claim a check passed unless it actually ran successfully. Explain environmental blockers and distinguish local results from GitHub CI results.

## Git and delivery
- Use a dedicated task branch and focused commits. Never push directly to the default branch or force-push.
- After implementation and validation, push the task branch and open or update a pull request unless the user asks otherwise. Never merge a PR, publish a release, or change repository settings without explicit user approval.
- End with a compact handoff report containing: summary, key files changed, exact validation results, branch and commit SHA, PR URL, and unresolved risks.
