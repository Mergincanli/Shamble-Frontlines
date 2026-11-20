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

### Block 4: FP-5 (i)-(v) and ADV-U4FF1H-1's correction, in both records (DONE: all found)
Where: the -20 L1 record is u4f-report.md:646-680 (item 5's last sub-bullet; the item's heading line :592-593 says "Re-run
on U4f-F1's merge: PASS (46ca143; -20's L1 record, at this item's end; F1's report u4f-f1-report.md)"). F1's report is
u4f-f1-report.md:57-78 (its FP-5 section) and :44-46 (the correction).
| Line | Source | -20 L1 record | F1's report |
|---|---|---|---|
| (i) Block 1's sentence | r1:67-72 (inside :65-72) | :663-668: word for word (a python compare, whitespace folded: equal) | :58-63: word for word (equal) |
| (ii) Wasm | r1:142-144 | :669-670: the same words ("Wasm, by reading only: ...") | :64-66, plus RR-F1-4 = TE-F1-4's reason |
| (iii) the throw's rate state | r1:166-169 | :671-672: the same words | :67-69, plus the v2 rule to the readings PR |
| (iv) SA-F1-1 open as C-U6-19; RR-F1-1 no action | r1:136-139 (RR-F1-1), :131-133 (SA-F1-1) | :673-675: SA-F1-1 (Medium) open until C-U6-19, before -19; RR-F1-1 no action with its reason, shortened (it drops "or a bites build" and the T2 (f) reason) | :70-73 with the whole reason; and its own Security section :97-101 |
| (v) the premise corrected; the hash | r1:44-47; pass :175-176 | :676-678: both | :74-78: both, with blob b757470 and the cite |
| ADV-U4FF1H-1 | head check :119-121 | :679-680 | :44-46; 46ca143's message carries it too (`git log -1 46ca143`) |
- The record's "no -20, -21 or -22 client rejoins or throws" holds by reading: -20's bots press fire only
  (bots/src/lib.rs:643-646 at 1faf894: `presses[PRESS_FIRE]` alone), -22's press nothing, -21's forgers neither throw nor
  rejoin (r1:90-93, u4f-brief.md:533, :1402, which I did not re-open).
- NC's "High then" (r1:72-73) does not fire: neither record offers the PASS as run-scale evidence of the sim's take. Both
  say the opposite ((i); F1's report :115-116, "What was not verified").
- One gap, a Low: r1:132-133 has "F1's record and U4's report name it [SA-F1-1] in a line of its own". F1's report does
  (:97-101). U4's report "folds" u4f-report.md (its :8), whose own Security section (:467-468, "for PR #40's report")
  still reads "No security finding stays open on U4f"; SA-F1-1 appears there only inside item 5's (iv). See
  ADV-U4FF1C-1.
- Code cites in F1's report spot-checked at 1faf894: seam.rs:73-80 (FP-1's doc, `edges_taken` at :79), world.rs:1338
  `run_command` with the rule at :1348, room.rs:1333 `run_one` with the rule at :1343, room.rs:352-357 `presses()`,
  room.rs:1327-1331, commands.rs:617-618 (T1), sim/tests/world.rs:1825-1921 (T2), input.rs:287 (`if let Some(prev)`):
  all as stated. sim/tests/seam.rs's FP-2 test runs :581-593, not "582-596" (:594-596 are the next item's blank line
  and doc); its assert at :591 is right. ADV-U4FF1C-2.
- Other cites read: r1:223-239 (the ruling table), pass :181 (the verdict), history.md:6818-6819 (the 2026-11-17 choice):
  as stated.
- The run's start: c-u4f-r2.mjs:116-123's `committed()` refuses any uncommitted tracked file and reads HEAD once at the
  start. HEAD 96de6ac (02:21:50 UTC); the card's "running" save 9f288d1 came at 02:22:21 and the run folder at 02:22:42,
  so the runner started between them and the card was committed while it built: only .checkpoints/ moved, the spike
  tree did not (Block 1). As the record says.

### Block 5: condition 10, U4f2's brief rev 1.2 (DONE: met as written, no new check)
Condition 10 (plan :222-223): "U4f2's brief rebases on F1's merge (Q3): its base and its count baseline
(u4f2-brief.md:15, :128), and it takes condition 7's item." With r1's two carries and my pass check's condition 4.
- **The base:** section 0 (:21-26) names 1faf894 (tree d4f6cd41...) and says outside results/ it is 25d316e plus F1's six
  files, +239 -24. `git diff --stat 25d316e 1faf894 -- spikes/netcode-proof ':(exclude)…/results'`: exactly those six
  files, 239 insertions, 24 deletions; crates/bots, lab, client and ops/ untouched. `<base>` filled at launch with the
  tree outside results/ named equal to 1faf894's or each difference listed: right, since the lane head now also carries
  -20's new results (46ca143).
- **The count baseline:** section 4 (:158-160): deltas on 1faf894's 642 measured and 739 bites (m5-check.log), that is
  634/731 plus F1's +8/+8, "which count in neither U4f's share nor U4f2's". Matches my head check's figures (642/739).
- **Condition 7's item:** r1 (:98-127) moved it to U6 as C-U6-19 and left U4f2 a record line, C-U4F2-9; section 3
  (:133-135) says so, citing r1 :98-122. Met as revised by my own ruling.
- **C-U4F2-9** (:136-137): word for word r1:126-127 (python compare, whitespace folded: equal).
- **C-U4F2-10** (:138-145): word for word r1:159-165; the only change is the inner quote marks ('a refusal record of its
  own kind' inside the outer quote), the right nesting. Its note that r1's ":44-46" is rev 1.1's block-6 line is right.
  Block 6 (:63-66) carries the case and its plant, +1 test; section 2 (:94) names it among the extras; section 6 (:219-220)
  keeps the total and the figure, as r1:128-130 ruled ("no size change", "U4F2-SIZE ... does not change").
- **Block 4's T1 line** (:51-53): at 1faf894 `git grep -c -w Refusal` in crates/room/src gives input.rs 22, room.rs 16,
  lib.rs 1 (39), and room/tests/commands.rs:636 is `npt_sim::Refusal {`: as stated; the rename leaves it alone.
- **u4s-carries.md's U4f2 section** (:548-552) holds both numbers with their r1 cites: the brief's section 0 and 3
  stamps point there.
- Nothing in rev 1.2 touches the lane, the stop, a pass mark, marks.json or the judge; U4F2-SIZE stays his and pending.
So condition 10 is met as written; no new check of rev 1.2 is needed (plan :224-225).

### Block 6: findings (CLAUDE.md's scale)
- **ADV-U4FF1C-1, Low (due before U4's report is assembled)**, u4f-report.md:467-468: its Security section ("for PR #40's
  report"; U4's report folds this file, :8) still reads "No security finding stays open on U4f". After F1, SA-F1-1
  (Medium, kind 5's first-command bypass at input.rs:287, which F1 widened) is open until C-U6-19, and my ruling asked
  that "F1's record and U4's report name it in a line of its own" (r1:132-133). Today it sits only inside item 5's (iv)
  and F1's own Security section. Fix by Edit at the next save: one line under :467-472, e.g. "After U4f-F1 (1faf894):
  SA-F1-1 (Medium) stays open until C-U6-19, before -19's runs: a session's first command is not edge-checked
  (input.rs:287); PR #40 stays needs-mustafa (u4f-f1-report.md, Security)." No effect on -20's verdict or F1's close;
  PR #40 is needs-mustafa either way.
- **ADV-U4FF1C-2, Low**, u4f-f1-report.md:29: "sim/tests/seam.rs:582-596" should be :582-593 (the test's fn line to its
  close; :594-596 are the next item's). Fix by Edit.
- **ADV-U4FF1C-3, Low**, u4f-report.md:657-658 ("the scratchpad's compare_20.py"), with jdiff.py behind the leaf count:
  the session scratchpad can be cleared by a reboot, so the record's method could not be re-run from the record (this
  morning's restart spared them). Fix: copy both read-only scripts into .checkpoints/netcode-proof-test/
  (CHECKPOINTS.md §3: small scripts in the notes folder; compare_20.py sha256 65e29b45...c47d7, 51 lines; jdiff.py
  3c836ef8...fd06, 25 lines) and point the record's line there.
- Observations, no finding: (a) the raw log headers record a newer container kernel (6.18.44-fc-v653 -> -v726) and the
  logs are byte-equal anyway, which adds to the determinism the expected list rests on; the record need not name it.
  (b) The bite's scripted presses read 431,998 at both commits (Block 3): unchanged, not an option-B run.
None is a High or a Medium; none needs a push of its own or a re-check by me; the main session fixes the three by Edit
at its next checkpoint save (ADV-U4FF1C-1 before U4's report).

## Verdict
**APPROVE WITH FINDINGS: -20's L1 verdict counts, and U4f-F1 closes.**
- The run is tied to F1's merge by tree (d4f6cd41... at 96de6ac and 1faf894) and every result file to the run folder
  (byte-equal).
- Every item of my expected list is met, and no other difference exists: every byte of the 21 raw logs equals
  350677f's except the header lines (commit, binary sha256, date, kernel) and `press_lost_count` on the lost ticks (3, 1,
  2 and the bite's 3,331 lines); the verdicts differ in 14 of 593 and 2 of 65 leaves, all the lost check and its reasons.
- PM-0001-2's SS part PASS 6 of 6; PM-0001-20 L1 options A and B PASS 3 of 3 each, its headline NOT-RUN (L2 not run);
  the bite shown at 0.894544, lost 3,614 to 0 by design.
- FP-5 (i)-(v) and ADV-U4FF1H-1's correction are in both records; (i) word for word; NC's "High then" does not fire.
- Condition 10 is met as written (Block 5).
- PR #40 stays a draft and needs-mustafa: SA-F1-1 is an open security finding until C-U6-19 (advisor.md step 4).
- The three Lows are record fixes by Edit (ADV-U4FF1C-1 before U4's report is assembled).

## Not checked by me
- I ran no cargo, check.mjs, npt-lab, judge or gate (not asked, and nothing but results and records changed since my
  head check of 1faf894). The reorder row's "not available" is by reading the row's series against the headers, not by
  the judge's report call.
- The run's exit 0 and the lane lock: no console log of the run is kept that I found; the verdicts written last
  (03:07:46), the full logs and 46ca143 stand for a run that ended. CARGO_TARGET_DIR unset: by runs.txt:2's "the main
  folder's own cache" only.
- FP-5 (ii)'s Wasm line is by reading (as ruled); u4f-brief.md:533 and :1402 behind (iv)'s "no client rejoins or throws"
  were not reopened (r1 read them); -20's bots pressing fire only was read at bots/src/lib.rs:643-646.
- The card (.checkpoints/netcode-proof-test.md, modified in the working tree) was not re-read beyond its F1 lines.
