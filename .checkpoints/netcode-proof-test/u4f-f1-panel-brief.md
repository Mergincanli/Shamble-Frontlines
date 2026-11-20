# U4f-F1's review round (one round; 2026-11-20): the shared brief for the four seats

## What changed
- **The fix** (ADV-U4F20-1, High: advisor-u4f-20.md:114-140, :162): npt-sim took one edge per changed press counter
  (crates/sim/src/world.rs:1340-1346 at cb7e374), so after option B's late drops a command two edges on lost a press
  that ADR 0001 part 5 keeps (docs/adr/0001-multiplayer-netcode.md:225-237). Now the sim takes every edge through its
  own checks, and the room's seen/lost/doubled counts come from the same rule.
- **The diff:** 5 files, 205 added and 24 removed lines, all in spikes/netcode-proof/crates/sim and crates/room. Read
  it as the patch `.checkpoints/netcode-proof-test/wip/u4f-f1-build.patch` (sha256 37d87a10bed52b81…, 17,495 B, base
  cb7e374), or in the builder's worktree `/home/user/Shamble-Frontlines/.claude/worktrees/agent-ad5f72ed146da5f93`
  (from that folder: `git diff cb7e374 -- spikes/`).
- **The spec:** `.checkpoints/netcode-proof-test/u4f-f1-plan.md` (rev 1.1) and the advisor's plan review
  `.checkpoints/netcode-proof-test/advisor-u4f-f1-plan.md`: its conditions 1-9 (:186-221) bind the build; its Blocks
  1-3 (:16-140) give the code facts (every `edge` use, the throw, the fire interval, the twins, today's multi-edge
  cases, the expected re-run).
- **The builder's record:** `.checkpoints/netcode-proof-test/u4f-f1-build.md` (where each part sits, what a bite does
  on an extra edge, a session's first command, each plant and its restore).
- **The evidence** (logs, read only): `/home/user/Shamble-Frontlines/.gate/u4f/f1-*.log`: b1-sim, b1-room, b1-bites,
  b2-t1, b2-plant, b2-restored, b3-t2, b3-plant1, b3-plant2, b3-fmt, b3-clippy, b4-check1 and b4-check2 (check.mjs
  twice: all 11 steps pass, 641 measured and 738 bites tests, +7 each on cb7e374's 634 and 731; the wasm step 27,000
  ticks with 0 mismatches).
- **Not done yet (the main session's, after this round):** the merge into the lane, the gate, CI, the advisor's head
  check, and -20's L1 re-run compared tick by tick with 350677f's kept logs (condition 5).

## The four seats (each reviews the whole diff, with its own focus)
- **rust-reviewer:** Rust correctness and safety; identical results on wasm32 and native; the seam fn integer-only;
  `wrapping_sub` across the u8 wrap; the loops' bounds (up to 255 edges per button) and their cost in `run_command`
  (a hot path: no allocation per edge); clippy; no panic or overflow path.
- **netcode-architect:** the reading of ADR 0001 part 5 (each edge through the server's own state); the room's counts
  and SERIES.md:42's meaning; options A and B; the expected -20 figures (advisor-u4f-f1-plan.md:119-140); a session's
  first command (ADV-U4FF1-3), carried to U4f2.
- **test-engineer:** whether T1 (room/tests/commands.rs:618) and T2 (sim/tests/world.rs:1813-1920, cases a-f) prove
  conditions 3 and 4; whether each plant bites by name; the +7 counts; what stays unproven until the re-run.
- **security-auditor** (scoped by condition 6 to hostile input): the loops' bounds; a session's first command (an
  unchecked advance up to 255 per button: throws fill the free pool, the rest refused); the path with
  PM-0001-21/no-press-edge bitten (room/src/input.rs:293).

## Rules for every seat
- Read and search only: git show/diff/grep/log, sed -n, wc, sha256sum. Do not run cargo, check.mjs, npt-lab, npt-judge
  or run scripts, and write nothing under spikes/ or .gate/.
- Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt` writing
  files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write.
- Your one file: `.checkpoints/netcode-proof-test/u4f-f1-review-<seat>.md` (seat: rust, netcode, test or security).
  Write its plan first, then each finished block as you go (a cut may stop you; a restart continues after the last
  complete block).
- Findings on CLAUDE.md's one scale (Critical, High, Medium, Low), each with an id (RR-F1-n, NC-F1-n, TE-F1-n or
  SA-F1-n), file:line, what goes wrong, the fix. Verdict: APPROVE, APPROVE WITH FINDINGS or REQUEST CHANGES.
- Anything that would touch a pass mark, the judge, marks.json, money or main: name it (Mustafa's, under TEST-FIRST).
- Return (your final text, at most 15 lines): the verdict, each finding's id, severity and one line, your file's path.
