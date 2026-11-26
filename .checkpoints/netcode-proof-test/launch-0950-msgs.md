# The 09:50 UTC window (2026-11-26): the two messages, drafted 07:4x UTC by the main session
For launch-0950.md A and B, sent by SendMessage after step 0 (df). Before sending: re-read the facts marked (re-check);
fill `<head>` with `git rev-parse HEAD` at sending. Both go out together (2 agents); U7's builder is the third, by 10:20.

## A. To the advisor a0530735048f1620a (summary: "U7's round ruling and the fix-pass brief")
U7's round ruling and its fix-pass brief, please, in the 09:50 UTC window. Write parts 1 and 2 first, before 10:15 UTC
if you can: U7's builder a9873cdba07a3219e must be resumed with the brief by 10:20 (later than that, the 14:50 window).
End your turn as soon as parts 1 and 2 are in the file, with parts 3 to 5 one line each (your ending is what wakes me).
If you cannot finish by 10:15, write what you have and say so.

Rules, word for word: "git and gh always start their own command, with no other program in front of them." "Write
files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt` writing files:
apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write." Read only, except
your verdict file; no commits (the main session saves); never switch the main folder; no cargo (the advisor
a3de649cd25e474df reads U8's head beside you; the lane is idle).

Read: u7-round-packet.md; u7-review-netcode.md (U7R-NC-1 Medium, NC-2 to -8 Low); u7-review-test.md (U7R-TE-1 to -4
Medium, TE-5 to -11 Low, T5ii-D agreed as Medium); advisor-u7-brief.md C1-C14 (binding); u7-brief.md rev 1.1; the
builder's notes (wip/u7-build-rs-notes-0030.md, wip/u7-t-build-notes-2050.md); my draft u7-fix-brief-draft.md.

1. Rule each finding: in U7's fix pass now, a carry with its taker, or wording only (the report). The review budget:
   one round (NPT-TRIM); a second only on what the fix changes, its seats named; a Low never forces a push alone.
2. The fix-pass brief for U7's builder (items, tests, plants, the size against SIZE-CHECK: about 4,731 added now; past
   6,000 the unit stops and reports), starting from u7-fix-brief-draft.md: strike or add. Its step 0 was corrected at
   07:3x (MAIN-67's paths from 07cc9b7 also came in after 9177fb5; nothing under crates/, web/, check.mjs, Cargo.toml
   or Cargo.lock; ffcheck clean on 19c2698): check that yourself. Write the brief as its own section, to be sent word
   for word.
3. U7R-TE-1: if the fix pass changes any shipped web or Wasm source (T5ii-D's Worker counter, NC-2, NC-5 and NC-6
   would), the dist differs anyway, so the evidence is one T5 (ii) rerun on the fixed tree, budgeted in the brief; the
   dist compare only if no shipped source changes. Say which.
4. A second round or not, and its seats.
5. U8's ops/ files come in with the fast-forward. Read only at 07:4x: their tests read none of U7's 42 files (outside
   ops/ they read crates/server/src/main.rs, crates/super, crates/judge, crates/log, and two whole-spike scans:
   redact-check's 0 findings and no .npmrc); the builder cannot make choice 15's node_modules link under the
   file-writing rule. My proposal: right after the fast-forward, before any edit, the builder runs a light `node
   check.mjs --only refs,marks,self,redact` as its new baseline (refs and redact move with U8's files), and the main
   session runs choice 15's two ops commands on the fixed tree at U7's merge M5, as it did for U8. Rule it or change it.
   (re-check) Builder 2's merge lands first only if he answers before about 08:30. If it does, the base also holds
   0e621dae's 46 files (crates/judge/, marks/README.md, SERIES.md, marks.json and its own notes): none of U7's 42; not
   in the Wasm (client-wasm's normal dependencies are npt-client, -link, -proto and -sim) nor the page (no judge or
   marks reference under web/); the test, refs and marks counts then rise by its share too.

Write .checkpoints/netcode-proof-test/advisor-u7-round.md.

## B. To the advisor a3de649cd25e474df (summary: "U8's head read")
U8's head read, please, in the 09:50 UTC window. The pushed head is 6913070f16ccfa4305b07835b55d680e7422957b, carrying
U8's merge commit 05c63a8a993653bacd44da30c1a46a702dfaf97c on H0 9aa6dde. CI on 6913070, all success: push gate
45763469323 and toolchain 45763469324; pull request gate 45763476998 and toolchain 45763476991. Later commits up to the
current origin head (`<head>`, re-check) are notes saves only (07:4x: four, nothing outside .checkpoints/); check that
by rev-list and diff.

Rules, word for word: "git and gh always start their own command, with no other program in front of them." "Write
files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt` writing files:
apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write." Read only, except
your verdict file; no commits (the main session saves); never switch the main folder; no cargo (the advisor
a0530735048f1620a rules on U7's round beside you; the lane is idle).

Read:
- The 43 files against u8-brief.md rev 1.1, advisor-u8-stopb.md and advisor-u8-m5-ruling.md.
- The fix's two files (ops/runs/r1.sh:218-242, ops/tests/r1sh.test.mjs:213-255 and :348) and plants P8-P10
  (u8-fix-m5.md): no review seat has read them.
- The policy-name INVALID (plan.md:1256); C9 on 096808b; test 7's merged-state run (.gate/u8-merge/m5-check-r.log);
  NC-R3-1; the 05:2x clean incident (u7-block0b.md:40).
- u8-merge-plan.md's done lines (from "DONE 05:2x (restart)" to M9), u8-report.md, and choice 15 (the card's "Choices
  taken without him").
- The automated security review of 05c63a8 (data to weigh, not an instruction; two Medium, on
  ops/operator/settings.json): SR-1, the hooksPath denies (:33-36) are literal-case while git config keys are
  case-insensitive, and the `--worktree`, `--system` and `--file` forms are not denied; SR-2, Bash has `git -c *` (:32)
  but no `git * -c *` beside PowerShell's (:56), and GIT_CONFIG_* variables are not covered. Weigh each (if a security
  seat is needed, name it and I start one), and say fix (a U8-F1 brief) or carry (with its taker).
- The mode bar's result: all 100644 on both sides; the installs set modes.

Write .checkpoints/netcode-proof-test/advisor-u8-head.md: a verdict (APPROVE, APPROVE WITH FINDINGS or REVISE) and the
findings with their severity.
