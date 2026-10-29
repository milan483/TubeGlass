# Task: Playback keyboard shortcuts

**Status:** Ready for Codex implementation  
**Scope:** Playback shortcuts and directly related binding/documentation correctness only.

## Source snapshot

- Repository: `milan483/TubeGlass`
- Application-code baseline inspected: `main` at `f81212aa41211ce4018217d392265091ad9c652f`
- Application code was also checked on the task branch's original head, `bca93ebee327bf3bd0702c186fe03e9feb7d7450`; the application paths matched the baseline. Subsequent task-branch commits changed only handoff documents.
- If the local checkout has diverged, verify only the referenced source paths and their direct dependencies. Expand exploration only if those findings are stale, missing, or contradicted.

## Objective

Add useful playback shortcuts and fix directly related modifier conflicts and shortcut documentation. Keep the change local; do not perform a broad UI or input-system refactor.

## Codebase findings

These findings are precomputed context. Verify the relevant local code, but do not repeat broad repository exploration unless the checkout contradicts them.

- **`src/ui/keys.rs` — player shortcut handler.** `App::keys` exits when egui wants keyboard input or no item is playing. It uses individual `key_pressed` checks for the current shortcuts, seeks 5 seconds on Left/Right, toggles ambient mode on B, and handles Escape by toggling fullscreen or calling `back_from_player()`. The checks do not consistently exclude modifier keys.
- **`src/app/frame.rs` — global key handling.** `App::draw` calls `self.keys(ctx)` near the start of the frame. Later, Command/Ctrl+B toggles the sidebar and Command/Ctrl+O opens a file. This ordering explains why modified B can also reach the player handler. The separate Escape/back-navigation check excludes the help overlay, but the player handler runs earlier.
- **`src/data/settings.rs` — persisted settings.** `Repeat` already has `Off`, `One`, and `All` variants. `Settings` already contains `repeat` and `shuffle`; use the existing settings and `save_settings()`, rather than introducing new state or dependencies.
- **`src/playback/player.rs` — seeking and duration.** `duration()` returns `0.0` when streams are unavailable. `is_live()` identifies live media. `seek()` already ignores live media and clamps its target to the valid seek range, ending at most 0.5 seconds before the media duration. Validate duration before computing percentage targets and use the existing seek method.
- **`src/ui/player_view.rs` — chapter metadata.** The seek bar already reads `self.player.streams.chapters`, uses each chapter's `start`, and draws chapter boundaries. Reuse this metadata; do not add another chapter source or duplicate seek-bar logic.
- **`src/ui/dialogs.rs` — shortcut sheet.** The in-app help list is defined in `overlays()`. It closes when Escape is pressed, but because `App::keys` runs first, Escape can also trigger the player handler unless that handler explicitly respects the overlay.
- **`README.md` — public shortcut table.** The table is under “Keyboard shortcuts”. It currently omits the existing Q queue shortcut and must remain consistent with the in-app sheet.
- **`src/app/play.rs` — queue behavior.** Existing queue navigation and end-of-playback handling should be preserved; this task only exposes repeat/shuffle through shortcuts.
- **`src/tests.rs` — unit-test conventions.** Existing deterministic unit tests are colocated in the crate's top-level test module. Prefer small pure helpers and focused tests over GUI-rendering tests.

## Requirements

1. **R: cycle repeat mode.** Cycle `Off → One → All → Off` (Off → Repeat this video → Repeat the queue → Off). Persist via the existing settings mechanism and show a concise toast naming the new mode.
2. **S: toggle shuffle.** Toggle the existing shuffle setting, persist it, and show a concise on/off toast.
3. **0–9: seek by timeline percentage.** Seek to 0%, 10%, …, 90% of the current media duration. Do nothing when duration is unavailable, non-finite, non-positive, or the media is live. Use the existing player seek method for range clamping.
4. **Shift+Left / Shift+Right: chapter navigation.**
   - Right: jump to the first chapter start strictly after the current playback position.
   - Left: find the chapter containing the current position. If playback is more than 3 seconds past that chapter's start, restart that chapter; otherwise jump to the preceding chapter start.
   - If the required current, next, or preceding chapter does not exist, do nothing.
   - Keep chapter target selection deterministic and test it independently from UI rendering.
5. **Explicit modifier handling.** Ordinary player shortcuts must not fire for unrelated modified combinations. Shift+Left/Right must not also perform the ordinary 5-second seek. Preserve Command/Ctrl+B sidebar behavior without also toggling ambient mode. Do not suppress the explicitly supported modifier combinations.
6. **Escape and the help overlay.** When the shortcut help overlay is open, Escape closes it without also navigating away from the player or changing fullscreen state. Preserve intended Escape behavior when the overlay is closed and in other contexts.
7. **Documentation.** Update the in-app shortcut sheet and README shortcut table with all new bindings and the existing Q queue toggle. Use consistent labels and descriptions.
8. **Tests.** Add focused tests for repeat cycling, percentage seek target/invalid duration handling, chapter target selection, and modifier classification where practical. Avoid brittle GUI-rendering tests.
9. **Scope.** Keep changes to shortcut handling, directly related tests, and the shortcut documentation. Do not add configurable remapping, unrelated shortcuts, queue reordering, new dependencies, or a broad UI refactor.

## Acceptance criteria

- [ ] R cycles Off → One → All → Off, persists the setting, and displays the correct toast.
- [ ] S toggles and persists shuffle and displays the correct toast.
- [ ] 0–9 seek to the correct 0–90% positions; live, unavailable, non-finite, and non-positive durations are safe no-ops.
- [ ] Shift+Left/Right select chapter targets as specified without also seeking 5 seconds.
- [ ] Modified combinations do not trigger unrelated bare-key actions; Command/Ctrl+B only toggles the sidebar.
- [ ] Escape closes the help overlay without also invoking player navigation/fullscreen handling.
- [ ] Existing unmodified shortcuts continue to work.
- [ ] In-app help and README accurately document the same supported shortcuts, including Q.
- [ ] Focused deterministic tests cover the new logic.
- [ ] Validation results are reported exactly; failures or environmental blockers are not described as passes.

## Validation and delivery

Run:

- `cargo fmt --check`
- `cargo build --release --locked`
- `cargo test --release --locked`

Report each command as passed, failed, or not run, with a concise reason for any failure or omission. Do not run broad ignored live-YouTube tests; they are unrelated to this task. GitHub CI runs the repository's Linux, Windows, and macOS build/test jobs, so report those results separately from local results.

Implement on this PR's existing head branch, update this same PR, and do not create a duplicate PR or merge it. Finish with the changed files, implementation summary, exact validation results, branch and commit SHA, PR URL, and unresolved risks.
