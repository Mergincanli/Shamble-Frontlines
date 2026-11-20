# Advisor check of U4f-F1's fix pass (2026-11-20), from main's advisor.md

In place of a second round (advisor-u4f-f1-r1.md:173-189, :241-255). Read only: git show/diff/grep/log, sed -n, wc,
sha256sum. Nothing run from the change; nothing written under spikes/ or .gate/. Builder's worktree:
.claude/worktrees/agent-ad5f72ed146da5f93 (read from that folder).

## Plan
1. The diff: the final snapshot's sha256 and size; equal to `git diff cb7e374 -- spikes/` in the worktree; the
   interdiff against the reviewed patch 37d87a10 (5 files); in src only `///` lines differ (FP-1: seam.rs:73-78,
   room.rs:353-357, :1328-1332); the sixth file named.
2. FP-1: the three doc comments read against FP-1's text.
3. FP-2 and FP-3: the tests at their lines; pass, plant and restore logs (f1-b5-*.log); the restored seam.rs hash
   901bb546 (after FP-1) against my table's 63057544 (before FP-1): which is right.
4. FP-4: t2_f's new asserts; the `word_if` helper and the pub(crate) departure: does it meet FP-4's intent.
5. The rustfmt layout in t2_f (f1-b5-pass2.log).
6. The end checks: f1-b5-check1.log (cut), -check1b, -check2: 11 steps, 642/739, wasm 0 mismatches.
7. The size: +239 -24, 6 files, against the 300 stop.
8. The builder's B5 notes.
9. FP-5: the carries (u4s-carries.md U4f2 C-U4F2-5..-10, U4g, U4h, U5 lines; plan.md:1253 C-U6-19), the readings-PR
   list and "After the close" on the card; each against my ruling.
10. Verdict, findings, the merge's steps.
