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
- Steps 1-5 done. His words read (history 2113-2175). Brief 540 lines, sha256 ba6bdeb0...
- Hashes verified on disk: texts-v3 3f21c9b7, freeze 9cb8be17, paste cfac7502, u4r-run e9437ae2, main-run 8df2dee3,
  peak dfacdcc0, c-u4r-runs 99dd070e, marks 9fb5be57, SERIES 3411eee7, README b49f3441: all equal the brief's.
- All 8 frozen blocks' sha256 (parsed with the freeze script's regex) equal freeze.txt.
- Reproduced in memory (no file written) the brief's insert algorithm: marks.json c1935a5b (json: exactly 2 checks + 1
  bite added, everything else equal), SERIES part 1 89295b57, SERIES full 6898e119, README f3c3c804 (refilled lines
  67-77 -> 16 lines, max 120; sentence 73 words). All four equal the brief's :101, :109, :116, :121.
- TE-CB-1..10, advisor C1-C11, r2 te_rulings read (TE-cond-6 strengthened: valid, own 48, other figure and
  mismatch 0, gaps empty, lab N == judge A). Brief :509-517 carries his stop rules and TE-cond-6.
- DROPPED candidate: C6's "Update the helper test" (SPLIT_LINE): no test imports the runner (grep check.test.mjs, ops/runs:
  none), so the dry-print harness is the only test; fine.
- F (Low): C6 wants the dry plan to list exactly 5 runs BY NAME (l1-p2-s1..s3, bite-whole-as-lost,
  bite-link-lost-overstated); brief :286 asserts counts + bite ids, not run names; :286-288 also asserts "about 552"
  and "about 1,530": an assert needs exact values (552 and 1530 by toFixed(0): 302.2+2.3403x).
- F (Medium): bite summary-line comparison (:470-473 "s1's line whole") cannot hold literally: the line's first field
  is the run name (scenario.rs:1621 s.meta.run; P0 precedent 93509dfd7fd1/P0/runs.txt:37 vs :42 differ in name, client
  hashes, incomplete, wall). Set-check must state what it strips (run name, wall, new phrase) or it false-FAILs.
- F (Medium): gap fixture (:247-251) asserts Outcome::Gap on both figures but not WHICH gap; "series missing from every
  room log" and "a room tick without it" shadow each other (any missing-series log trips both), so deleting one
  gap's code leaves the test green. Each case must assert its own note text (a distinct substring).
- F (Low): l1 plant set covers only the dir filter (CB-P1); no plant drops the `carries == Datagram` filter or the
  `from` slice; the test (:152-156) holds down acks/stream pieces only if it says so: state it, add CB plant.
- F (Low): CB-P5 expected "sum 0 and unexplained above 0": a Rust test stops at its first failed assert, so only the
  first shows; name the assert that fires.
- B-2 checked against fixtures: base/lost-group/hashed-and-incomplete as the brief says; none-compared A 6 (slots 0,1
  x ticks 1-3) stays FAIL; orphan's t 65539 is a hash (not incomplete) so no new gap; short-lists replaces p2-room.jsonl
  without the series -> new figures gap too, still INVALID with its old why line. Brief right. README.md:6 ("changes
  only together with npt-judge and its fixtures") supports B-2 in this lane.
- P0 re-judge: verdict.json -17 why holds "bite ... whole-as-lost: no run yet" (so the new line appears there);
  verdict-bites.json -17 why has no bite lines (all three ran), so the new bite's "no run yet" line ALSO appears there.
  Brief :399-401 "at most one -17 why line" should be "exactly one in each file" (a missing line would pass).
- npt-judge exits 0 on any verdict (main.rs:52-123), so main-run.sh's stop-at-first-failure will not cut the second
  re-judge call. OK.
