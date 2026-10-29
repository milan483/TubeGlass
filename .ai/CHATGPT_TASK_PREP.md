# ChatGPT-to-Codex task preparation

## Goal

Use ChatGPT for repository reconnaissance, external research, requirement clarification, and implementation planning before handing a task to Codex. Codex should spend its context on local-state checks, implementation, tests, and fixing real problems—not repeating research already completed.

## Prepare the task

1. **Clarify the outcome.** Resolve material ambiguities before handoff. State observable acceptance criteria, edge cases, constraints, and non-goals.
2. **Inspect the relevant repository context.** Record the target repository, branch, and exact base commit SHA. Follow the affected call paths and inspect relevant types, existing tests, UI/documentation, and nearby conventions. Avoid scanning unrelated parts of the repository.
3. **Research only where useful.** Prefer primary sources such as official documentation, specifications, source code, release notes, and relevant project issues/PRs. Record concise conclusions with direct links. If no external research is needed, omit it.
4. **Make decisions before handoff.** Select a reasonable implementation approach and explain constraints that affect implementation. Do not leave avoidable design decisions for Codex to rediscover. Leave unresolved only questions that genuinely require user input.
5. **Write a compact task packet.** Use the Codex issue template. Include exact file/function references and a source snapshot, but not a transcript of searches or every explored file. Separate verified findings from assumptions.
6. **Create or update the task location.** Prefer a GitHub issue when available. If Issues are disabled, put the packet in `.ai/tasks/<task>.md`, link it from a draft PR, and tell Codex to update that same PR.
7. **Hand off with a short prompt.** Point Codex to the packet and `AGENTS.md`; do not repeat the packet in the prompt.

## Reusable Codex handoff prompt

> Implement the task in [issue/task packet URL]. Follow `AGENTS.md`. The packet contains researched codebase context and design decisions; verify the referenced local code and current source snapshot, then implement without repeating broad repository exploration or external research unless findings are stale, missing, or contradicted. Preserve existing local changes. For a well-specified, low-risk task, proceed directly; stop for approval if a risk gate in `AGENTS.md` applies. Run focused tests and required validation, report exact outcomes, and update the existing task PR. Do not create a duplicate PR or merge. Report the summary, changed files, validation results, branch/commit SHA, PR URL, and unresolved risks.

## Review after implementation

- Inspect the actual PR diff against the packet's acceptance criteria and non-goals.
- Check relevant CI results and whether claimed local validation actually ran.
- Review only the changed code and necessary surrounding context; expand repository exploration if the diff reveals a concrete concern.
- Send Codex a focused list of actionable defects or missing criteria, then review the resulting changes.
- Do not approve or merge on the user's behalf.

## Division of responsibility

ChatGPT's GitHub view does not establish the state of a local checkout or uncommitted changes. Codex must verify the local branch and working tree and run the required checks. The optimization is to avoid duplicated discovery, not to skip implementation verification.
