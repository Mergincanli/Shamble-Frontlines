# test-engineer, review round 2 of foundation 2.2 (PR #48): 4d78ad6..4bccf55 (excluding .checkpoints/)

Angle: is it proven? Only what changed since round 1's head 4d78ad6.
Scratch and builds: /home/user/Shamble-Frontlines/.gate/test-engineer-r2/ (nothing in the worktree but this file).
Agent memory: none in this environment (no .claude/agent-memory-local/); round-1 notes read
(.checkpoints/foundation-toolchain/test-engineer-review-r1.md).

## Plan (blocks; each marked DONE with its evidence when finished)
1. Read the diff 4d78ad6..4bccf55 (code: Cargo.toml, 4 crates' doc comments; docs: report, register row OVERFLOW-ALL,
   REPO_MAP, ROADMAP, STATUS, STATUS_ARCHIVE); the ruling advisor-ruling-d1.md (conditions 5, 7, 7e); ADR 0002 near
   :1319 (PM-0002-53) and the overflow parts; round-1 dedupe and verdicts for my own findings' dispositions.
2. M7: 297d48a touches only the release setting; 8d622ac exact revert (git diff 297d48a^ 8d622ac empty); CI runs
   43415093334 (job 131850863497) push and 43415101662 pull_request: which step is red, which binary, which message;
   compare with the report's M7 row word by word; does it claim to distinguish route A from part 4's blocks?
   Also: CI on 8d622ac and the head 4bccf55 (green? cancelled?).
3. Relabelled evidence: M5 and step 2's control marked historical; grep report/STATUS/register for every use of M5,
   "control", "step 2" and check nothing still leans on them for a current claim.
4. E12: read xcrate-check.sh and both outputs; check "before" really is the pre-fix tree (what did the script archive,
   when; compare the archive's Cargo.toml with 4d78ad6/1de98a9's); re-run on a scratch copy of 1de98a9 and of 4bccf55,
   native and wasm32; check the probe can fail (a caller crate's inline/generic sim code, debug vs release,
   wasm32 run or build only).
5. OVERFLOW-XCRATE-CI anchor: wording in report/register/STATUS; is there any mechanism (CI, gate, checklist) that
   fires on "the first change that adds a dependency on sim"; who notices.
6. PM-0002-53 placement: ADR 0002:1319 text vs the report's and the register row's text; testable as written?
7. Remaining new rows and dispositions (round-1 findings TE-1..TE-3 and others' that changed text); facts in E12/M7.
8. Findings, claim/proof/gap table, verdict; advisor tool before finalising.

## Findings (a line each, as found)

### Block 1 DONE (diff read)
- Code: Cargo.toml:21-29 (two route-A tables + comment), four crates' doc comments only (1de98a9). The worktree HEAD is
  4bccf55; commits aee92cc, 8c397ac between 1de98a9 and 4bd9dd5 touch only .checkpoints/advisor-ruling-d1.md.
- ADR 0002 part 3 (:186) says `protocol` depends on "`sim` and `content` types only"; room, client-endpoint, sim-wasm,
  server, navbake depend on sim. Today no crate depends on anything (six manifests: no [dependencies]).

### Block 2 DONE (M7; logs under /home/user/Shamble-Frontlines/.gate/test-engineer-r2/ci/; gh api, read-only: the
### GitHub MCP tools were not loadable in this agent, no ToolSearch)
- 297d48a touches only Cargo.toml: removes `[profile.release]` + `overflow-checks = true` (+ blank line), keeps "*".
  `git diff 297d48a^ 8d622ac` is empty even with .checkpoints/ (297d48a^ = 8c82be2). Exact revert: holds.
- Push 43415093334 (job 131850863497) and pull_request 43415101662 (job 131850893215), both on 297d48a, attempt 1,
  failure at step 7 inside the cut (uid 1001 nogroup only, only lo, curl (7)); dev `cargo test --workspace` 6 x
  "1 passed"; then release: protocol "overflow_checks_are_on - should panic ... FAILED", "note: test did not panic as
  expected at crates/protocol/src/lib.rs:10:8", exit 101. sim's release binary never ran (cargo stops at the first
  failing binary). Gate push 43415093339 / PR 43415101832 green on 297d48a.
- Revert 8d622ac: toolchain push 43415458569 (job 131852089087) and PR 43415465726 (job 131852114268) green; inside the
  cut both release binaries ran: protocol ... ok, sim ... ok; 16 "test result: ok"; Playwright 1 passed; tsc 6.0.3.
  Gate 43415458525 / 43415465755 green. Every run id in the M7 row matches its SHA, event and conclusion.
- CI on later heads: 8a4f9b0 toolchain 43415740925 / 43415748074, gate 43415740367 / 43415748089; 4bccf55 toolchain
  43424298848 / 43424306336, gate 43424299180 / 43424306390: all green, attempt 1. No cancelled run since 4d78ad6.

### Block 4 DONE (E12; my copies under /home/user/Shamble-Frontlines/.gate/test-engineer-r2/orig-before|orig-after)
- "Before" is the pre-fix tree: the session's folder .gate/xcrate-before-1de98a9/ws has the per-package blocks and file
  mtimes 01:10:10 = 1de98a9's commit time (git archive stamps the commit time); after-4bd9dd5 has route A, 01:31:43 =
  4bd9dd5's. The early run .gate/xcrate-fixed (01:31:21) archived 8c397ac (HEAD before the fix commit; mtime 01:30:01)
  and so tested the OLD blocks under a "fixed" name; it is cited nowhere (report, register, STATUS: grep below).
  1de98a9..4bd9dd5 differs only in Cargo.toml (non-checkpoint files), so before/after isolates the setting.
- The unmodified script re-run by me on 1de98a9 and 4bd9dd5: both outputs byte-identical to the recorded files (diff).
- Raw (unfiltered) results: before, release/bench/relcustom each "2 passed; 4 failed", the 4 = "test did not panic as
  expected" (room lib.rs:9, 15, 21, 27), never_inline ok; after, each "6 passed; 0 failed". The recorded files show
  neither "ok" nor the summary line (grep '^test tests' + --nocapture pushes "ok" to another line): a reader must read
  a blank as a pass. Format only; the claim holds.
- wasm32 control (my wasm-control.js, not in the script): non-overflowing calls (1+1, tick 5) return 2 and 6 in all
  four builds, before and after; overflowing calls trap in dev (both) and release-after, return 0 in release-before.
  So the release traps after the fix are the overflow check, not some other trap. The script has no such control.
- rustc's flag per crate after a clean: after, native release --workspace AND wasm32 release (sim, protocol, room,
  client-endpoint, sim-wasm, wasmcheck): every member on (client_endpoint, devtools, protocol, room, sim, sim_wasm,
  wasmcheck), extdep off. Before: sim and protocol only, native and wasm32. E12's three-crate flag list understates it;
  "every workspace crate" holds for all six real members, both targets.
