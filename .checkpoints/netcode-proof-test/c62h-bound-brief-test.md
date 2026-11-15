# test-engineer: read of c62h-bound-brief.md (C62H-BOUND build brief)

Seat: test-engineer. Read only: this notes file is the only file I write (rewritten whole with Write: no Edit tool).

## Plan
1. Read Mustafa's words (history.md 2113-2129, 2131-2146, 2163-2175).
2. Read the brief whole (c62h-bound-brief.md) and its notes.
3. Read the design (c62h-bound-design-full.json), advisor C1-C11 + Q1-Q5, TE-CB-1..10, advisor r2 (te_rulings, replacements, planned_size, other_changes item 6).
4. Read frozen texts v3 + freeze.txt; verify sha256 of each frozen block and the paste.
5. Read card NOW, KEPT-LOGS, C62H-BOUND blocks, MH-L1 note.
6. Read code: spikes/netcode-proof crates l1, lab, judge; marks/; ops/runs/c-u4r-runs.mjs; check.mjs; runners u4r-run.sh, main-run.sh (sha256).
7. Check per item: tests + fixtures with assertions; planted bugs with home test and expected values; lab tally identities; TE-CB-7 expected lines; TE condition 6 bite rule (advisor-strengthened); freeze check; P0 re-judge comparison.
8. Findings with brief line + source line; structured result.

## Notes (one line per finding as found)
- Steps 1-3 done. His words read (history 2113-2175). Brief 540 lines, sha256 ba6bdeb0...
- Hashes verified on disk: texts-v3 3f21c9b7, freeze 9cb8be17, paste cfac7502, u4r-run e9437ae2, main-run 8df2dee3,
  peak dfacdcc0, c-u4r-runs 99dd070e, marks 9fb5be57, SERIES 3411eee7, README b49f3441: all equal the brief's.
- TE-CB-1..10, advisor C1-C11, r2 te_rulings read (TE-cond-6 strengthened: valid, own 48, other figure and
  mismatch 0, gaps empty, lab N == judge A).
- Candidate: C6 says "Update the helper test with it" (SPLIT_LINE); brief 3.4 item 2 changes SPLIT_LINE, says nothing of
  a helper test; brief :70 bars check.test.mjs. Check whether any test pins SPLIT_LINE.
- Candidate: C6 dry plan "lists exactly 5 runs (l1-p2-s1 to s3, bite-whole-as-lost, bite-link-lost-overstated)";
  brief :286 asserts counts and bite ids, not run names.
