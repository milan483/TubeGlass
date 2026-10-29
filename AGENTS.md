# Codex instructions

## Project context
- TubeGlass is a Rust 2024 desktop client built with eframe/egui. Read `README.md` and `Cargo.toml` before making project-wide assumptions.
- The GitHub issue or explicit task specification is the source of truth for scope and acceptance criteria.

## Working rules
- Read this file, `CODEX_START_HERE.md`, the assigned issue, and the relevant source/tests before editing.
- Inspect the current branch and working tree first. Never overwrite or discard existing user changes.
- Work on a dedicated task branch, not the default branch. Keep the diff focused; avoid unrelated refactors and new dependencies unless the task justifies them.
- Follow existing architecture and naming. Each Rust source file should begin with a `//! ` module description, and functions should have doc comments, consistent with the project documentation.
- If a requirement is materially ambiguous or conflicts with the existing implementation, explain the conflict and ask before making a broad assumption.
- Do not change release versions, packaging, update behavior, dependency policy, or security-sensitive code unless it is in scope.
- Never put credentials, tokens, or other secrets in issues, commits, logs, or reports.

## Validation
- For normal code changes, run `cargo fmt --check`, `cargo build --release`, and `cargo test --release` where the environment permits.
- The ignored tests can contact YouTube and require network access. Run `cargo test --release -- --ignored` only when relevant to the task and explicitly justified.
- Report each command as passed, failed, or not run. Do not claim a check passed unless it actually ran successfully. Explain environmental blockers and distinguish local test results from GitHub CI results.

## Git and delivery
- Use focused commits with messages that state what changed and why.
- For an implementation task, commit and push the task branch after validation, then open or update a pull request when the available GitHub tooling permits.
- Never push directly to the default branch, force-push, merge a pull request, publish a release, or change repository settings without explicit user approval.
- Finish with a concise report: summary, important files changed, validation results, branch/commit, PR link if available, and unresolved risks.
