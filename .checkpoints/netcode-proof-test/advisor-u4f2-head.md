# Advisor: check of pushed head b4e1f43 (U4f2's merge, PM-0001-21's forged-command test)

## Plan (written first)
Seat: day-session advisor agent, read only (caller's rules: no builds, no runs). Main's advisor.md governs; NPT-TRIM
(history.md:7561-7593) keeps this head review. Steps: (1) `git show b4e1f43 --stat`, parent, patch sha256, files' hashes
head vs worktree, spikes/ vs a289def; (2) R6 logs a/b, M5's header and tail, CI check runs; (3) round-2 reviews and
u4f2-build.md's dispositions, FP-10 and TE-U4F2-R2-4 in the code; (4) parta :102-105 and `kind_7_left_out`'s call
sites; (5) F-B-COUNT (a) against advisor-u4f2-r1.md:404; (6) what -21's set must hold. Then verdict and findings.

## Verdict: APPROVE WITH FINDINGS (two Lows; neither needs a push)
b4e1f43 is the reviewed tree (14 files byte-equal to R6's worktree); M5 passed on its exact diff; CI is green; round 2's
Medium is fixed; F-B-COUNT (a) is built as his words say and reported per forged run. Departure accepted (point 4).
Next: c-u4f-r2.mjs whole, then -21's set (point 6). Risk tier 1 (spikes/ only); nothing goes to main here.

## What I checked myself (read only)
1. **Tree.** 14 files, all under spikes/netcode-proof, +2,674 -194; parent 82583f8. `git diff --stat a289def 82583f8
   -- spikes/` is empty (results/ included). The patch's sha256 is f7d79ebd…a86f. The 14 blob ids in
   `git diff --raw 82583f8 b4e1f43` equal the patch's index lines, and `git hash-object` on **all 14** worktree files
   (agent-aa62d31b6fa5b9807) gives the same ids, so they are byte-equal. The worktree's base 0376c76 has a289def's
   spikes/. Every mode is 100644. b4e1f43..654fe28 touches only .checkpoints/.
2. **M5.** The header's diff 445b8f9de3e3…1aaf equals `git diff 82583f8 b4e1f43 -- spikes/netcode-proof | sha256sum`
   (I computed it). Untracked is the empty hash (e3b0c442…), and its HEAD ec85f12 has 82583f8's spikes/. All 11 steps
   pass; test 1,070.2 s; 709/806; wasm 0 mismatching.
3. **R6** (u4f2-r6a.log, -r6b.log; tree 93fcd1f7…, forged.rs untracked 07a4f4f5…): all 11 steps pass in both; test
   1,155.5 s and 1,112.7 s against 1,207 s; 709 and 806, 0 failed or skipped (+67 on a289def's 642/739,
   u4f2-build.md:195); wasm 27,000 ticks, 0 mismatching. The empty provenance section matches U4f-F1's M5.
4. **CI** (`gh api …/check-runs`): gate, smoke and toolchain all success, for both push and PR (runs 44969638312/-333,
   44969642476/-481). The diff touches no .github/ or gate file, so the green stands.
5. **Code.** `kind_7_left_out` appears only at bots lib.rs:180 (the definition), :2082 (T-K7, test module) and lab
   forged.rs:107 (in `Attribution::tick`, which exists only on forged runs that are not twins: scenario.rs:1368).
   Outside that arm there is only the empty `dead_before` (:1373). I read forged.rs whole, f16 (scenario.rs:4498-4525),
   the FP-2 and FP-10 diffs (snapshot patches f0da659→096a49b→82583f8) and SERIES.md's diff.
- **Not checked by me:** no gate, cargo or runs (the caller's rules): the gate --no-smoke PASS is the main session's,
  CI's gate and smoke stand in here; R2, R3, the lag proof and the plants read through the reviewers' citations. The PR
  review to main runs the gate itself.

## Findings
| Id | Sev | Where | What is wrong | Fix |
|---|---|---|---|---|
| ADV-U4F2H-1 | Low | c-u4f-r2.mjs:100-106 (`kind7Line`); c-u4f-runs.mjs:147 | (a) Any folder with no kind-7 `forged_kind` record prints "a twin forges nothing", so a forged run or bite that stopped or failed before its end records reads like a twin. (b) A run stopped at RUN_LIMIT_MS is SIGKILLed (check.mjs:1923-1927) and can leave a torn last line (logs.rs writes through a BufWriter). JSON.parse then throws in the run loop and ends the set before its remaining runs, both judge calls and runs.txt, which breaks its own rule, "a stopped call is a result". Reach: stopped or failed runs only (the limit is 3,600 s; a run takes about 5 min). | Follow-up, about 4 lines: parse each line in try/catch; name a twin only for a `-twin` folder, else "no forged_kind record: the run did not finish". No push now (NPT-TRIM, budget). Until it is fixed, -21's report treats that line on a forged run that is not a twin as a failed run. |
| ADV-U4F2H-2 | Low | advisor-u4f2-r1.md:404 (my FP-2 row); the reading, history.md:7551 | Both say "an item whose d-1 never ran is not left out". The code (bots lib.rs:175-182, forged.rs:105-107) leaves it out when d itself ran dead ("then d alone decides"), as his words say ("dead as that command or the one before it ran"). The code is right and the wording is loose. The f33 case covers d-1 never run only with d alive (item 10). | No code. -21's reading line words it as the code does: "an item whose d - 1 never ran is decided by d alone". The missing case (d-1 never ran, d dead: left out) joins TE-U4F2-R2-3's follow-up. |

## Point 4, the departure. Ruling: ACCEPT; nothing re-runs
- My wording (parta :103-104) was a proxy for "FP-2 changes no code a non-forged line runs". The bots lib.rs part is
  one pure `pub const fn` (no state, RNG, loop or I/O). Its only caller outside tests is forged.rs:107, inside the
  Attribution, which exists only on forged runs that are not twins; the rest is T-K7's lines in the test module. A
  function nobody calls cannot change another's behaviour. The evidence agrees: the lag-switch A proof (fate log
  0576dea2… and 481 tick hashes equal), R5b's forged lines keeping their early fate logs, and R6 and M5 running every
  non-forged suite's unit tests on the final tree. So R4 and R5's non-forged lines stand.
- For FP-10 (parta :106: I name the lines): it changes no code a lab run executes (f16 is test code, the forged.rs
  change is a doc comment, the scripts change only output lines and a `part` tag). Nothing re-runs. The r2 rows and
  -21's set are the first runs of the new script lines.
- U4f2's report records it in a line of its own (one predicate beyond the letter, so one plant fails both tests).

## Point 2, the commit before M5 ended: acceptable
The commit (01:03:34 UTC) followed R6 twice and the gate, on R6's bytes; its message said M5 "runs" (true); nothing used
b4e1f43 before M5 passed (01:22:52) on its exact diff. Condition: the report states the order, M5's result and its hash.

## Point 3, round 2 and its dispositions: they hold
- No Critical or High in round 2, so no third round (NPT-TRIM; budget); round 1's open High justified round 2.
- TE-U4F2-R2-1: fixed. `kind7Line` runs in c-u4f-r2.mjs (p1, p7, p14, p17) and in c-u4f-runs.mjs (every forged-part
  run, twins and the 11 bites included); `--kind7` reads without running anything. The dry read of R2's folder gives
  18/1,505/0 and 8/1,515/0, equal to R2's records. The 15 R5b figures are recorded (u4f2-build.md:605-607).
  NC-U4F2-R2-1 is met at the run lines; pairing them with each run's check value is the report's job.
- TE-U4F2-R2-4: fixed (the zero check now comes before the clamp return; exemptions only when `suite == "forged"`);
  f16 24/24 (u4f2-fp10-tests.log), and in R6 and M5. NC-U4F2-R2-2's comment: fixed (forged.rs:50-53).
- Follow-ups (u4f2-build.md:613-618): NC-U4F2-R2-2's case, NC-U4F2-R2-3, TE-U4F2-R2-2, -3 and -5, named in U4f2's
  report and STATUS; TE-U4F2-R2-6's record lines go into the report.

## Point 5, F-B-COUNT (a): built as his words and the FP-2 row say
The dead bit is the server World's own: forger slots whose health reads 0 in the records after tick t-1, which is the
state entering tick t's command phase (scenario.rs:1465-1474). It is read at each kind-7 command's first run
(forged.rs:100-115). Left-out items leave `sent_count` (:185) and `unrejected_count` (:154). `dead_left_out_count` is on
every record, 0 for the other kinds (:189). One new SERIES.md row; no existing row, no marks.json and no judge change.
Its acceptance (the f33 case's five cases, :445-465; T-K7 extended; one plant failing both; R2 with every kind's
unrejected 0 under A and B) is as the row asks. FP-10 reports it per run; the report's pairing and reading line are
still owed.

## Point 6: what must hold in c-u4f-r2.mjs whole (21 rows) and -21's set (49 runs, judged in two calls)
1. **Order:** run from a commit whose spikes/netcode-proof equals b4e1f43's outside results/ (654fe28 does). D2's
   reading (c-u4f-runs.mjs:7-8), then the 21 rows, then -21's set. A row that reads other than its `expect` comes to
   me first (u4f2-brief.md:109-111). p14's equality with p1 (fate log and tick hashes) is the main session's read.
2. **Disk:** -21's start floor needs 4 GiB plus its projection. That is 29,470 measured seconds x 65,188,828 B / 600 s,
   about 3.20 GB, so about 7.50 GB free before its builds (in the main folder's own cache). Free was 8.21 GB before the
   r2 rows: run a P27 clean first. A refusal at the floor is a safe stop, not a result.
3. **Which sets:** NPT-TRIM's KEEP 1 replaces the header's -47, -20, -22 order; the report says which sets ran.
4. **Per forged run in the report** (6 forged measured runs, 6 twins, 11 forged bites; p1, p7, p14, p17): kind 7's
   `dead_left_out_count`, `sent_count` and `unrejected_count` beside the judge's "every forged command rejected and
   counted" value for that run (u4f2-review-r2-test.md:127-133). One line that every other kind's left-out count is 0;
   twins "no forged_kind record (a twin …)"; clamp, fast-clock and lag-switch runs "not applicable". A forged run that
   is not a twin and has no record has failed (ADV-U4F2H-1).
5. **Expected readings:** on every forged measured run, every kind unrejected 0, dropped 0, twin mismatch 0, kind 7
   sent at least 1,000, `counted_count` = `sent_count` for kinds 8, 9 and 11, in a building 0, and PM-0001-2's band.
   Each bite fails only its own kind's check. Comes to me before the report: a left-out count above 76 on any 600 s run
   (5 % of about 1,523; R2 read 18 and 8), a left-out count on a kind other than 7, or a forged run unrejected above 0.
6. **Lines of their own:** the reading (his words quoted; worded per ADV-U4F2H-2; it narrows ADR 0001:1611 and
   SERIES.md:158's `sent_count` for kind 7; reviewed by round 2's netcode and test seats and this check; on the
   readings PR's list); the departure with this ruling; the commit-before-M5 order; the follow-ups by id.

**Decisions needed:** none. **For Mustafa:** nothing.

## Ruling on the 21 rows (c-u4f-r2.mjs on 92ed9a8, r2.txt): -21's set MAY START; p12's expect is a follow-up
- Checked: `git diff --stat b4e1f43 92ed9a8 -- spikes/netcode-proof` is empty (point 6.1 holds). p12-verdict.json:109-137:
  "99.9 % reach" fails at 0.894861 and "missing inputs" fails at 0.105139, so the bite reaches its own target. "0
  presses lost" passes at 0, "0 presses doubled" passes at 0, presses 14,398. The other 20 rows I take from the
  coordinator's reading (not re-read by me).
- p12 is -20's row, not -21's, and nothing in -21's set reads it. "0 presses lost" passing is by design since U4f-F1
  (1faf894, 2026-11-20): the room counts presses with the rule the sim runs, as advisor-u4f-f1-r1.md:65-73 bound it.
  So the expect is wrong, not the run. The stale text was F5 (c) (advisor-u4f-r2.md:201, written before F1), carried
  into p12's row by U4f2's merge after F1 had landed. Neither round 2 nor this head check caught it.
- **ADV-U4F2H-3 (Low):** c-u4f-r2.mjs p12's `expect` reads "'0 presses lost' fails too". Fix: "'0 presses lost' passes
  by design after U4f-F1 (advisor-u4f-f1-r1.md:65-73)". It touches no code a run executes, and nothing re-runs. It
  rides the next commit to the run scripts (with ADV-U4F2H-1's fix) or becomes a follow-up by id. U4f2's report shows
  p12 as read against the corrected text.
- Record lines: the -20 record must not cite this PASS as run-scale evidence of the sim's take (NC-F1-1's "High then").
  Presses 14,398 against 14,400 scripted is window-edge variation, not loss: p5 reads 14,402 and p6 14,400.
- -21's set starts after point 6.2's disk check, as point 6 says. Nothing else holds it.
