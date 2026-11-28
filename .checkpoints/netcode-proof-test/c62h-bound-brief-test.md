# test-engineer: read of c62h-bound-brief.md (C62H-BOUND build brief)

Seat: test-engineer. Read only: this notes file is the only file I write (rewritten whole with Write: no Edit tool).
Brief read: 540 lines, sha256 ba6bdeb02e61aa83... (the drafter's notes give the same).

## Plan
1. Read Mustafa's words (history.md 2113-2129, 2131-2146, 2163-2175).
2. Read the brief whole (c62h-bound-brief.md) and its notes.
3. Read the design (c62h-bound-design-full.json), advisor C1-C11 + Q1-Q5, TE-CB-1..10, advisor r2 (te_rulings, replacements, planned_size, other_changes item 6).
4. Read frozen texts v3 + freeze.txt; verify sha256 of each frozen block and the paste.
5. Read card NOW, KEPT-LOGS, C62H-BOUND blocks, MH-L1 note.
6. Read code: spikes/netcode-proof crates l1, lab, judge; marks/; ops/runs/c-u4r-runs.mjs; check.mjs; runners u4r-run.sh, main-run.sh (sha256).
7. Check per item: tests + fixtures with assertions; planted bugs with home test and expected values; lab tally identities; TE-CB-7 expected lines; TE condition 6 bite rule (advisor-strengthened); freeze check; P0 re-judge comparison.
8. Findings with brief line + source line; structured result.
ALL STEPS DONE.

## What I verified (no file written; in-memory scripts only)
- Hashes on disk equal the brief's: texts-v3 3f21c9b7, freeze 9cb8be17, paste cfac7502, u4r-run.sh e9437ae2, main-run.sh
  8df2dee3, peak.py dfacdcc0, c-u4r-runs.mjs 99dd070e, marks.json 9fb5be57, SERIES.md 3411eee7, README.md b49f3441.
- All 8 frozen blocks (freeze script's own regex) hash to freeze.txt's values. No frozen text changed by the brief.
- The insert algorithm (brief section 2) reproduced in memory: marks.json c1935a5b (json: exactly 2 checks + 1 bite added,
  everything else equal; each block occurs once), SERIES part 1 89295b57, SERIES full 6898e119, README f3c3c804 (lines
  67-77 refilled to 16 lines, max 120; sentence 73 words). All four equal brief :101, :109, :116, :121.
- His words quoted correctly at brief :10-16. Stop rules :509-517 hold every rule of the paste (paste :120-124) plus
  TE-cond-6's validity and N == A.
- P2/P3 expected numbers (:466-468, :477-479) equal fc411116c1f1/P2/runs.txt:24-26 and fd4612050cd6/P3/runs.txt:24-26.
- `git log fc411116c1f1..442a284`, `fd4612050cd6..`, `93509dfd7fd1..` -- crates, marks: empty (no crate or mark change).
- Raw P0 folder u4r-P0-93509dfd7fd1 holds verdict.json e926b812 and verdict-bites.json b472784f (byte-equal to the
  committed results) and marks.json 9fb5be57.
- B-2 checked against every -17 L1 P2 fixture: base, lost-group, hashed-and-incomplete as brief says; none-compared A 6
  stays FAIL; orphan's t 65539 is a hash, no new gap; short-lists replaces p2-room.jsonl without the series, still
  INVALID. README.md:6 ("changes only together with npt-judge and its fixtures") supports B-2 in this lane.
- B-1 holds: contract.rs:123-145 (b) fails any written name without a SERIES row; r2's doctored case (3) still finds
  the :320 row by its prefix after the row changes.
- npt-judge exits 0 whatever the verdict (main.rs:52-123): main-run.sh will not cut the second re-judge call.
- Runner import is guarded (c-u4r-runs.mjs:601): the dry-print harness can import it safely.
- No test imports the runner (C6's "helper test" does not exist): the dry print is its only test.
- bite-home check (check.mjs:926-958) reads only crates/*/src outside test modules: the new id lives in npt-lab only. OK.
- Lab runs are simulated time (no Instant/SystemTime/sleep in scenario.rs, logs.rs): no timing-dependent test.

## Findings
- TE-CBB-1 Medium: the three evidence scripts (built check :126-135, re-judge compare :398-404, set check :446-448)
  have no negative run; nothing shows each can print FAIL. Precedent: U4R-Q11 (u4r-brief.md:1675-1680).
- TE-CBB-2 Medium: gap fixture :247-251 asserts Outcome::Gap on both figures but not which gap; gap 1 ("no room log
  gives the series") and gap 2 ("a room tick record without it") shadow each other; gap 4 names four lists, one case.
- TE-CBB-3 Medium: bite summary lines :470-473 "s1's line whole" cannot hold literally: the line starts with the run
  name (scenario.rs:1621-1624 `s.meta.run`); P0 precedent 93509dfd7fd1/P0/runs.txt:37 vs :42 differ in exactly
  name, client hashes, measured incomplete, wall.
- TE-CBB-4 Medium: :299 records test counts with no expected values; :333 claims the lab bite tests run in every bites
  step with nothing proving it. Expected: c1 +7/+9, c2 +6/+6, c3 +1/+1 (B-2) else 0, c4 0; final 552/632 (551/631).
- TE-CBB-5 Low: :399-401 "at most one -17 why line": should be exactly one per file (eval.rs:1696, :1704-1708: -17 stays
  NOT-RUN in both files, so both gain the line; verdict-bites.json's -17 why today has no bite line).
- TE-CBB-6 Low: dry print :284-291 asserts "about 552", "about 1,530": exact 552 and 1530 (toFixed(0) of
  302.2 + 2.3403 x MiB); run names of C6 (l1-p2-s1..s3, bite-whole-as-lost, bite-link-lost-overstated) not asserted.
- TE-CBB-7 Low: plants missing for: l1 carries filter + from slice (CB-P1 only dir); l1 expected numbers must come
  from an independent count (canonical bytes), B1 has 2 % random loss; the canonical-bytes-equal assert cannot fail
  (&self on a Vec); figure A's E1 and E2 branches; figure B's window (CB-P8 is A's); the run's Err wiring at the call
  site; Summary's L from counts in the short link-lost-overstated test.
- TE-CBB-8 Low: frozen row 2 says "A client hash at a tick the room log has no record of is not read here": no fixture.
- TE-CBB-9 Low: stop rule :513-514 ("..., on a valid run") reads either way; write it as TE-cond-6's conjunction.
- TE-CBB-10 Low: CB-P5 :320-321 expects two asserts; only the first fires: name it.
- TE-CBB-11 Low: B-3 / MH-L1: re-judging both P0 folders (two more judge calls each, no runs) removes the reading and
  doubles the "not changed for" evidence; the design's claim "old P2 and P3 logs ... read INVALID" (design bites[3])
  has no step: one judge call on u4r-P2-fc411116c1f1 or drop the claim.
- TE-CBB-12 Low: B-2 changes an existing judge test's answer (hashed-and-incomplete INVALID -> FAIL): the 13:35 read
  line in the report (:491-493) should name it beside SAME ANSWERS.

## Verdict
APPROVE WITH CONDITIONS: close TE-CBB-1 to -4 in the brief before the build; carry the Lows or defer each in writing.
