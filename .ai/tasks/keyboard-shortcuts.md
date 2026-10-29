# Task: Playback keyboard shortcuts

**Status:** Ready for Codex implementation  
**Scope:** Playback shortcuts and directly related binding/documentation correctness only.

## Objective

Add a focused set of useful playback shortcuts and fix the small modifier conflicts exposed by them. Avoid a broad UI refactor.

## Current implementation

- Playback shortcuts: `src/ui/keys.rs`
- In-app shortcut sheet: `src/ui/dialogs.rs`
- Documented shortcuts: `README.md`
- Repeat and shuffle settings already exist.
- Chapter metadata is available through `self.player.streams.chapters` and chapter boundaries are rendered on the seek bar.
- `Q` opens/closes the queue but is missing from the help sheet and README.
- Modifier matching needs explicit handling: `Ctrl/Cmd+B` toggles the sidebar in `src/app/frame.rs`, while the player handler can also treat it as bare `B` (ambient mode). Escape may also trigger player navigation while the help overlay is open.

Inspect the current code and tests before implementation; these paths are starting points, not an exhaustive list.

## Requirements

1. **R: cycle repeat mode** in this order: Off → Repeat this video → Repeat the queue → Off. Persist the setting and show a concise toast identifying the new mode.
2. **S: toggle shuffle.** Persist the setting and show a concise toast indicating whether shuffle is on or off.
3. **0–9: timeline jump** to 0%, 10%, …, 90% of the current finite media duration. Clamp to a valid seek position. Do nothing if duration is unavailable/invalid or the media is live.
4. **Shift+Left / Shift+Right: chapter navigation**, when chapter metadata exists:
   - Right jumps to the next chapter start after the current playback position.
   - Left restarts the current chapter if playback is more than 3 seconds past its start; otherwise it jumps to the preceding chapter start.
   - If no matching chapter exists, do nothing.
5. Make modifier handling explicit. Shift+Left/Right must not also trigger the ordinary 5-second seek; modified key combinations must not accidentally trigger unrelated bare-key shortcuts. Preserve intended Ctrl/Cmd+B sidebar behavior without also toggling ambient mode.
6. Escape closes the keyboard-shortcut help overlay without also navigating away from the player. Preserve intended Escape behavior in other contexts.
7. Update both the in-app shortcut help and README shortcut table. Include existing Q queue behavior and all new bindings with consistent labels.
8. Keep changes local to shortcut handling and its directly related UI/docs. Do not add configurable key remapping, unrelated navigation shortcuts, queue reordering, or a broad UI refactor.
9. Add focused tests for deterministic shortcut logic (repeat cycling, percentage seek target, chapter target selection) and modifier handling where practical. Avoid brittle GUI-rendering tests.

## Acceptance criteria

- [ ] R, S, 0–9, and Shift+Left/Right behave as specified.
- [ ] Existing unmodified playback shortcuts continue to work.
- [ ] Modified shortcuts do not trigger unrelated bare-key actions; Ctrl/Cmd+B only toggles the sidebar.
- [ ] Escape dismisses the help overlay without also leaving the player.
- [ ] The help sheet and README accurately document all supported shortcuts, including Q.
- [ ] Focused tests cover deterministic shortcut logic.
- [ ] `cargo fmt --check`, `cargo build --release`, and `cargo test --release` pass, or any failure/environmental blocker is reported precisely.

## Validation and delivery

Run the normal checks from `AGENTS.md`. Do not run ignored live YouTube tests unless relevant and justified. Work on this PR's branch, report exact validation outcomes, and do not merge the PR.
