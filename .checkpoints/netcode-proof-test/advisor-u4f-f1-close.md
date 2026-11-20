# Advisor: U4f-F1's close, -20's L1 re-run held to the expected figures (2026-11-20)

Seat: the advisor agent, day session, read-only (main's advisor.md, read with `git show origin/main:.claude/agents/advisor.md`).
Branch claude/netcode-proof-test-plan-hcky5g at e51d1ef; nothing switched, nothing committed.
Why: my head check's condition 3 (advisor-u4f-f1-head.md:109-113) and my pass check's condition 3
(advisor-u4f-f1-pass.md:192-197): hold -20's L1 re-run to my expected figures; any other difference stops for me; read
FP-5 (i)-(v) in the -20 L1 record and F1's report. Plus condition 10 of my plan review (advisor-u4f-f1-plan.md:222-225).

## Plan
1. The tree: `git rev-parse 96de6ac:spikes/netcode-proof 1faf894:spikes/netcode-proof` (both d4f6cd41...); 46ca143
   touches only the results folder; the run header names HEAD 96de6ac and that tree.
2. The committed results (46ca143): runs.txt against 350677f's, line by line, wall and binary sha256s aside; verdict.json
   and verdict-bites.json leaf by leaf (jdiff.py, or my own python), every changed leaf named.
3. The expected list, item by item: summary lines; per-tick `h` (compare_20.py on both raw-log folders, re-run by me in
   the background, or spot-checked); lost 0, doubled 0; presses B 432,000 x3, A 432,003 / 432,003 / 432,009; -2 SS PASS
   6/6; -20 A and B PASS 3/3; L2 NOT-RUN; the bite at 0.894544 with lost 3,614 to 0; the reorder row "not available".
4. FP-5 (i)-(v) and ADV-U4FF1H-1's correction, in the -20 L1 record (u4f-report.md item 5) and in F1's report, each
   against its source line in r1 and the pass check.
5. Condition 10: u4f2-brief.md rev 1.2 (base 1faf894, counts 642/739, C-U4F2-9 and -10 as ruled, block 4's T1 line,
   section 6's line), against u4s-carries.md's filed texts and r1 :126-127, :159-165.
6. Findings (ADV-U4FF1C-n), verdict, what I did not check.

## Blocks
(A container restart at about 04:22 UTC stopped me after Blocks 1-3's checks had run, before they were written down;
their figures below are from those runs, read back from my session. Resumed 04:25.)

### Block 1: the tree and the commits (DONE)
- `git rev-parse 96de6ac:spikes/netcode-proof 1faf894:spikes/netcode-proof`: both d4f6cd41b8d789a7d9ad00f729f3a32224d123c4.
  350677f's is f6e5f9a5... (the defect's tree, as its runs.txt:2 says).
- 1faf894..46ca143: 2e20d29, d02612e, 02032ee, 96de6ac, 9f288d1, 8a8a4be touch only .checkpoints/ (the card, history.md,
  card_trim_1120d.py, advisor-u4f-f1-head.md); 46ca143 adds only the three result files under
  results/u4f/96de6acf8223/pm-0001-20/ (+1,140 lines). So the run's spike tree is 1faf894's, as my head check's
  condition 2 asks.
- runs.txt:2 (new): "commit 96de6acf8223..., spikes/netcode-proof tree d4f6cd41b8d789a7d9ad00f729f3a32224d123c4; target:
  the main folder's own cache" (CARGO_TARGET_DIR unset reads so). Every raw log header's `build.commit` is 96de6ac.
- The run folder's times: marks.json and the first run folder 02:22:42, the last log 03:07:39, verdict-bites.json
  03:07:46 (`ls --time-style`): the record's "about 02:22 to 03:07 UTC" holds.
- The committed verdict.json and verdict-bites.json are byte-equal (`cmp`) to the run folder's
  (.gate/netcode-proof-test/runs/u4f-20-96de6acf8223/); marks.json there is c1935a5b..., the same as 350677f's run
  folder and runs.txt:6.

### Block 2: the results against 350677f's (DONE)
- **runs.txt** (`diff`; then a python compare of the seven summary lines with only " (wall N s)" stripped): the seven
  lines EQUAL, all 7. The other differences: :1 the date; :2 commit and tree; :3 the free space (10,568 against 11,492
  MiB); :6 npt-lab's two sha256s (6dab8d1b..., bites c611bb3b...; npt-judge d8b8c91e... and marks.json c1935a5b...
  unchanged); :10 the mark's headline FAIL -> NOT-RUN "L2 P3 option A: 0 of 1 valid runs"; :13 option B FAIL -> PASS and
  its three "0 presses lost = 3 / 1 / 2" lines gone. The judge's log sizes (371.8 and 61.1 MiB) are equal.
- **compare_20.py re-run by me** (python3 -I, 7.4 s; its output): all seven "summary line (wall aside) EQUAL"; ticks old
  18335 new 18335, common 18335, h mismatches 0, on every run; presses seen (all ticks) 439,628 / 439,643 / 439,619 (A),
  439,565 / 439,586 / 439,561 (B), 439,563 (bite), each equal old to new; lost 0->0 x3 (A), 3->0, 1->0, 2->0 (B), 3683->0
  (bite); doubled 0->0 everywhere. The main session's output is reproduced exactly.
- **My own, stronger check** (python3 -I from stdin, no file written, 8 s): every line of all 21 raw logs (room, client,
  lab x 7 runs) compared with 350677f's, line by line: equal line counts (room 18,997, client 18,332-18,333, lab 56); the
  only differing lines are (a) each file's header line, with exactly four leaves changed: `build.commit`,
  `build.binary_sha256`, `date`, `machine.kernel` (6.18.44-fc-v653 -> -v726: a newer container kernel, which the record
  does not name; the logs hold equal anyway), and (b) room `tick` lines whose only changed leaf is `v.press_lost_count`
  1 -> 0: b-s1 3 lines (the first at tick 13594), b-s2 1 (tick 4441), b-s3 2 (first tick 4446), the bite 3,331 (first at
  tick 36). No other byte of any log differs: client hashes, fates, shots, the `second` and `event` records, and the
  footers are equal. Every tick record carries an `h` on both sides (128,345 each, none missing), so compare_20.py's
  `r.get("h")` never compared None with None.
- **The verdicts** (jdiff.py re-run by me): verdict.json 14 of 593 leaves differ: option B's three runs' check 3
  (`outcome` fail->pass, `value` 3/1/2->0), the part's verdict FAIL->PASS and its three `why` lines gone, the mark's
  verdict FAIL->NOT-RUN and its three `why` lines now "L2 P3 option A: 0 of 1 valid runs", "L2 P3 option B: 0 of 1 valid
  runs", "bite PM-0001-20/redundancy-1: no run yet". verdict-bites.json 2 of 65: the bite run's check 3 (fail->pass,
  3614->0). All of it the lost check and its reasons, as the record says.

### Block 3: my expected list, item by item (DONE: every item met; no other difference)
| Expected (plan :119-140; r1 :191-205; head check cond. 3) | Found (46ca143) | Met |
|---|---|---|
| summary lines equal to runs.txt:23-29 but wall and the binaries' sha256s | 7 of 7 equal; header :6 npt-lab x2 new | yes |
| per-tick `h` equal to 350677f's kept logs (measured/, bites/) | 18,335 x 7, 0 mismatches; every log byte equal but headers and lost | yes |
| lost 0 (by construction), doubled 0 | all ticks, all runs: 0 and 0 | yes |
| presses B 432,000 each; A 432,003 / 432,003 / 432,009 | B 432000 x3; A 432003, 432003, 432009 | yes |
| -2 "SS runs" PASS, 6 of 6 valid | PASS, a-s1..b-s3 all valid | yes |
| -20 A PASS 3/3, every figure as at 350677f | PASS; on time 0.999843 / 0.99987 / 0.999861, missing 0.000157 / 0.00013 / 0.000139; jdiff 0 leaves | yes |
| -20 B PASS 3/3, on-time and missing as at 350677f (= A's per seed) | PASS; the same six figures as A per seed | yes |
| L2 parts NOT-RUN | L2 P3 A and B NOT-RUN; headline NOT-RUN | yes |
| the bite shown at 0.894544, lost 3,614 -> 0 | status shown; 0.894544 fail; missing 0.105456; lost 3614 -> 0 | yes |
| the reorder row "not available" | the row's series is `header.params.reorder_permille` (marks/marks.json:4787-4790 at the tree); all seven new room headers carry only `reorder_step_permille`, and the headers differ from 350677f's in four leaves only (Block 2), so the row reads "not available" by the same cause | yes, by reading (the judge's report call not run by me) |
- Seen beyond the list, unchanged from 350677f (so not a difference): the bite's "10,000 scripted presses" reads
  431,998 (pass), the same at both commits. It is the bite, not an option-B run, so (i)'s "432,000 per option-B run"
  is untouched; I name it so no reader takes the bite's figure for a missing press.
- The kernel change (Block 2) is outside my list (header lines, not summary lines) and moves nothing.
