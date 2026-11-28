# Test-engineer: the read of commit 5e563b9's 15 marks.json lines (C62H-REVIEW, workflow wf_4ce9b8cd-ad2), 2026-11-15

Written by script, word for word, from the workflow's journal. Nothing below is the main session's. Its notes: c62h-review-test.md. The questions are Mustafa's, in history.md, "## Decision: Mustafa, 2026-11-15 13:35 Amsterdam".

- **Seat:** test-engineer (C62H-REVIEW (a): a read of the 15 lines that commit 5e563b9 added to spikes/netcode-proof/marks/marks.json; read only; notes in /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/c62h-review-test.md)
- **marks.json at HEAD, sha256:** 9fb5be578b27d4c378a890d297c5ba636af0ac239bb6d50eae157ed44884a3c5

## Question 1

- **Answer:** partly

Partly. The check matches the advisor's later rulings and the plan's readings line word for word. It does not match carry 62 (h)'s own text word for word. And PM-0001-17's row in ADR 0001, the plan's pass mark, says nothing about lost groups at all.

(1) Against the readings line that waits for Mustafa's go before R5 (u4-build-plan.md:1094-1100, committed at a36babd) and the JSON in the advisor's r2-run ruling (advisor-u4r-r2run-full.md:9): equal. All seven fields match (name, series tick.s.snap_incomplete, stat sum, window measured, op ==, value 0, parts ["L1 P0"]), and a JSON comparison returns True. The bite entry is equal too.

(2) Against carry 62 (h) as written (u4s-carries.md:305-309): two differences. The name has lost its 'setup: ' prefix, and '"setup": true' is gone. The other six fields are identical. The carry's own amendment line (u4s-carries.md:314-316) still says 'the check's JSON stays word for word'. Both differences come from the advisor's build ruling 4 (advisor-u4r-build4-full.md:32). That ruling found that the setup form could never show a bite: a failed setup check makes the run INVALID (judge eval.rs:1189-1208), so the tick16 and whole-as-lost bite runs would read INVALID and -17's P0 part could never count. The plain form is also stricter: a lost group on P0 FAILs -17 instead of voiding the run. The carry file was never updated to say so (TE-C62H-2).

(3) The bite's text, 'a client writer that files a tick it decoded whole as lost: null and incomplete 1' (marks.json:4011), equals the advisor's amended wording in build ruling 1, point 4, character for character (advisor-u4r-build1-full.md:33). Its id is PM-0001-17/whole-as-lost.

(4) ADR 0001:1605, PM-0001-17's pass-mark row, asks for 0 mismatches and names two bites: a 16-bit tick and client rounding. It has no words about incomplete ticks or lost groups. The row hash pinned in marks.json (3075f638...) still equals the row, so the row is unchanged. The check is a stricter reading of -17, built on the advisor's rulings, and is still waiting for Mustafa's go on the readings PR. plan.md's -17 rows (:462, :475, :721) say nothing about it either.

**Evidence:**

git show 5e563b9 -- spikes/netcode-proof/marks/marks.json (+11 check lines at marks.json:3971-3981, +4 bite lines at :4009-4012). git log 5e563b9..HEAD -- marks.json crates/judge: empty, so nothing has changed them since. python3 -I JSON comparison: the committed check equals the r2-run ruling's JSON (True), and its bite equals the ruling's bite (True). Against carry 62 (h)'s JSON the differences are exactly {name: 'setup: L1 P0 loses no group (no client-tick left incomplete)' -> 'L1 P0 loses no group (no client-tick left incomplete)', setup: True -> absent}. The ADR row's line sha256 is 3075f6384860068622268fc3efff47db2b8a29624d063882d72beab37eedffea, equal to marks.json's draft.row_sha256 for PM-0001-17. The row contains none of 'incomplete', 'loses no group' or 'lost'.

## Question 2

- **Answer:** yes

Yes, and in this bug class only this check fails.

Where the bite acts: npt-lab's client writer (bites build only), lab/src/logs.rs:317-330, `bitten`. On every client slot it takes the first tick decoded whole in the measured phase (hash not 0) and files it as Res::Lost (:362-366). The writer then writes decoded_hash null and snap_incomplete 1 for that slot (:441-449). That makes one tick per client, so 48 on a 48-client run.

How the judge catches it: the bite run matches -17's L1 P0 part, so the new check, whose parts are ["L1 P0"], applies to it (eval.rs:1451-1458). The check is a plain check (:1209-1211): it sums the series (:282) over measured-phase records (:123). A failing check on a valid bite run marks the bite 'shown' (:1477-1478). An empty series would be a gap, not a pass (:313-327), so the check cannot pass on missing data.

The record made under the advisor's carry-62 (h) ruling (.gate/netcode-proof-test/c62h-5e563b9589d3-record/):
- README.txt names commit 5e563b9589d3..., a marks.json sha256 equal to HEAD's, r2's judge (sha256 65acacfe...) and the scripts' form of the command, `--only PM-0001-2,PM-0001-17`.
- whole-as-lost-verdict.json:157-178: bite PM-0001-17/whole-as-lost 'shown'; run u4r-r2-whole-as-lost-1 valid; mismatch figure 'pass', value 0; 'L1 P0 loses no group (no client-tick left incomplete)' 'fail', value 48, needs '== 0'.
- tick16-verdict.json: bite shown with 2,880 mismatches (fail) and the new check at 720 (fail).
- Both verdicts are byte-equal to the labelled diagnostic in -diag-with-2/: cmp exits 0, sha256 0ee24942... and da62fe7f.... The first try (c62h-5e563b9589d3/) read INVALID through the judge's --only defect (JDG-ONLY-1) and is not the record.

The same bite was shown again on the judged 10-minute P0 set at 93509dfd7fd1 (results/u4r/93509dfd7fd1/P0/verdict-bites.json): new check 48 fail, mismatches 0 pass. The three clean runs there read 0 and pass.

Why this matters: on the whole-as-lost run the mismatch figure passes at 0. Without this check, -17 would be blind to the bug even on the perfect network.

**Evidence:**

.gate/netcode-proof-test/c62h-5e563b9589d3-record/whole-as-lost-verdict.json:157-178 ('status': 'shown'; check 'L1 P0 loses no group (no client-tick left incomplete)', outcome 'fail', value '48', needs '== 0'; mismatch figure outcome 'pass', value '0'). README.txt in the same folder: marks.json sha256 9fb5be57..., equal to `sha256sum` of HEAD's marks.json. cmp against c62h-5e563b9589d3-diag-with-2/: exit 0 for both files. spikes/netcode-proof/results/u4r/93509dfd7fd1/P0/verdict-bites.json: whole-as-lost shown with ('L1 P0 loses no group', 'fail', '48') and ('100 % of decoded sna...', 'pass', '0'). Unit test: lab/src/scenario.rs:2684-2694 asserts that the bitten short P0 run's measured sum equals the client count (16), and :2655-2677 that the clean run's sum is 0.

## Question 3

- **Answer:** not bounded

Nothing bounds the count on L1 P2, P3 or B1.

- The new check's parts are ["L1 P0"] only (marks.json:3978-3980). On every other part, -17 only reports the sum (window all, :3993-3998), and nothing passes or fails on it.
- The mismatch figure's coverage rule accepts a tick as resolved by snap_incomplete 1 just as by a hash (judge figures.rs:466-481). Its one floor is 'empty' when no hash at all was compared in the whole run (:547). That floor is per run, not per client, so a single client can file every tick as lost and the run still passes.
- No other mark fills the gap:
  - -57 takes its errors only from ticks filed with a real hash (lab/src/logs.rs:15-17), so a tick filed as lost drops out of -57 too.
  - PM-0001-2's 'everyone in view' is counted from the room's log, the server side.
  - -18 on B1 bounds only how long the first decode after each burst takes, not how ticks are filed afterwards, and its L1 B1 part is INVALID today.
  - -58's L1 P3 wrap suite and the L2 parts have no incomplete check either.

What was measured on the judged sets (u4r-report.md:992 and :1002, from the re-run sets):
- P2: 17,260 / 17,089 / 17,532 incomplete client-ticks in the measured window, which the judge counts as 17,739 / 17,566 / 18,027 over the whole run.
- P3: 43,295 / 43,210 / 43,454 measured (44,395 / 44,253 / 44,535 over the whole run).
- That is about 2.0 % and 5.0 % of roughly 864,000 measured client-ticks (48 × 18,000), which matches the profiles' designed loss (l1/src/lib.rs:185-192: P2 2 %, P3 5 %).
- B1 has no judged -17 value at all: all three runs in 64f3307d5c7f are INVALID by pre:PM-0001-2, and -17's L1 B1 part waits for U4g.

The only comparison of these counts with the designed loss was the advisor's by-eye check in its r2-run ruling. No check makes it.

**Evidence:**

marks.json PM-0001-17 at HEAD: the checks are the mismatch figure (no parts), the S30 setup check (parts ['every S30']) and the new check (parts ['L1 P0']); the report is the sum with window 'all'. Judged verdicts: results/u4r/fc411116c1f1/P2/verdict.json (PASS; incomplete over the whole run 17,739 / 17,566 / 18,027; compared 861,996 / 862,011 / 861,731); results/u4r/fd4612050cd6/P3/verdict.json (PASS; 44,395 / 44,253 / 44,535; compared 835,238 / 835,419 / 835,088); results/u4r/64f3307d5c7f/B1/verdict.json (all three runs INVALID: everyone in view 0.129944 / 0.268056 / 0.3075, needs >= 0.99; no -17 report). Judge: figures.rs:466-481 (coverage), :547-551 (empty only when compared == 0). Client: client/src/lib.rs:580-607 (a tick leaving the ring 'lost unless filed'). Lab: logs.rs:15-17 (-57's errors only on ticks filed with a non-sentinel hash). marks.json PM-0001-18: the heal checks read event.burst_heal.after_link_ns, the first decode after the link returns (SERIES.md:134).

**What a client bug of that kind could hide:**

On P2, P3 and B1, a client that files ticks as lost when it should file a hash or the sentinel, beyond the 2-5 % the network really drops, goes unseen by -17, by -57 and by the judge. That holds as long as at least one hash in the whole run is compared and matches. What could hide there:

(a) A decode or apply bug, filed as lost instead of the sentinel. This is the hiding place the advisor's R2.4 named: 'a refused group can hide only as lost and counted'. The paths where a refusal is likely run only under loss: a delta against a baseline the client lost, a baseline-age or group-count disagreement after a dropped acknowledgement, a reordered group past the newest tick on P3, and resync after a burst on B1. The P0 check never exercises these paths, because P0 drops nothing. They are exactly the code most likely to be wrong.

(b) A bookkeeping slip that never files a tick it did decode. The client's default is 'lost unless filed' when a tick leaves its frame ring (client/src/lib.rs:580-607). An early return, a miscounted group total or a lost completion flag therefore turns silently into an incomplete tick, not a mismatch.

(c) A client that drops groups as stale too eagerly on P3's reorder. By SERIES.md:320, a stale drop is counted as incomplete by design, and it is bounded nowhere on -17.

(d) A client that recovers badly after a B1 burst, filing ticks as lost long after its first fresh decode. -18 times only that first decode.

(e) In the extreme: one client, or many, filing almost every tick as lost. The coverage rule accepts it, and the empty rule only needs one compared hash in the whole run.

Scale: about 8,640 extra lost client-ticks per run (1 % of 864,000) would move P2's measured count from about 17,300 to about 25,900. That is still a quiet number in a report line, with -17 PASS. Each such tick is also one decoded snapshot that -17 and -57 never compare. The same gap holds on -58's L1 P3 wrap suite: a wrap bug that leaves ticks lost rather than mis-hashed would hide there. It also holds on -17's L2 parts, P0 included, which R3 runs.

## Findings

### TE-C62H-1 (Medium)

- **Where:** spikes/netcode-proof/marks/marks.json:3972-3981 (the new check's parts ["L1 P0"] only) and :3993-3998 (on every other part, the count is a report only); spikes/netcode-proof/crates/judge/src/figures.rs:466-481 and :547
- **What is wrong:** On -17's L1 P2, L1 P3 and L1 B1 parts, nothing bounds how many client-ticks may be filed as left incomplete. The bug class the new check exists for, a decoded or refused tick filed as lost (R2.4: 'a refused group can hide only as lost and counted'), is caught only where nothing is lost. The paths where refusals and filing slips are likely (deltas after loss, reorder past the newest tick, resync after bursts) run only on the lossy profiles. There the misfiling blends into 2-5 % of designed loss, and -17 and -57 still PASS.
- **Evidence:** The judged sets PASS -17 with P2 measured incomplete 17,260 / 17,089 / 17,532 and P3 43,295 / 43,210 / 43,454 (u4r-report.md:992, :1002; results/u4r/fc411116c1f1/P2 and fd4612050cd6/P3 verdict.json). These are about 2.0 % and 5.0 % of about 864,000 measured client-ticks, the profiles' designed loss (l1/src/lib.rs:185-192). No check compares them with anything. The coverage rule accepts snap_incomplete 1 as a resolution (figures.rs:466-481); the empty floor is 0 compared per run (:547). The client defaults to 'lost unless filed' (client/src/lib.rs:580-607). -57 errors come only from ticks filed with a hash (lab/src/logs.rs:15-17). B1 has no judged -17 value yet (results/u4r/64f3307d5c7f/B1: all three runs INVALID by pre:PM-0001-2). This is Medium, not High: it is a missing bound, not a wrong result. Today's client files the sentinel on every refusal path the advisor listed (client/src/lib.rs:473-749), and the counts match designed loss by eye.
- **Fix:** No change on the advisor's ruling alone: this is a new bar, so it goes to Mustafa under Decisions needed (proposed bar PB-1 gives its exact wording). Until he decides, the advisor rules that the main session records, in PR #40's report and on -17's L1 P2, P3 and (when judged) B1 lines, each run's measured incomplete count beside the share the profile's loss predicts. The report also states plainly that -17 off P0 is unbounded on this count.

### TE-C62H-2 (Low)

- **Where:** /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4s-carries.md:305-309 and :314-316
- **What is wrong:** Carry 62 (h) still holds the setup-form JSON, '"name": "setup: L1 P0 loses no group ..."' with '"setup": true'. Its amendment line still says 'the check's JSON stays word for word'. It has no pointer to the advisor's build ruling 4 and r2-run ruling, which changed the check to the plain form now in marks.json. A reader checking marks.json against the carry, as this read was asked to, finds two unexplained differences.
- **Evidence:** JSON comparison: carry 62 (h) against the committed check differs in exactly name ('setup: ' prefix) and setup (True vs absent). The change is ruled in advisor-u4r-build4-full.md:32 ('without "setup": true and without the 'setup: ' prefix') and advisor-u4r-r2run-full.md:9 ('with no "setup" and no "setup: " prefix'); u4r-report.md:53 notes it. The carries file does not.
- **Fix:** At the main session's next edit of u4s-carries.md, add one line under carry 62 (h), after :316: '**(h) amended again (the advisor's U4r build ruling 4, Medium, advisor-u4r-build4-full.md:32; its r2-run ruling, advisor-u4r-r2run-full.md:9):** the check is a plain check, with no "setup" and no "setup: " prefix, as committed at 5e563b9: {"name": "L1 P0 loses no group (no client-tick left incomplete)", "series": "tick.s.snap_incomplete", "stat": "sum", "window": "measured", "op": "==", "value": 0, "parts": ["L1 P0"]}.' Nothing in marks.json changes.

### TE-C62H-3 (Low)

- **Where:** spikes/netcode-proof/marks/marks.json:3971-3981, :4009-4012 (commit 5e563b9); /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4-build-plan.md:1094-1100
- **What is wrong:** The check and its bite are a stricter pass bar for -17. They were built into marks.json on the advisor's rulings, and Mustafa's go is still pending on the readings-PR list before R5. The judged P0 verdict at 93509dfd7fd1 already applies the bar. Under his words of 13:35 ('A new bar, or a change to one, comes to me under Decisions needed; it is not built on the ruling alone'), a report could read this bar as decided when his go is still open.
- **Evidence:** ADR 0001:1605 is unchanged (row sha256 3075f638... equals marks.json's pin) and has no such check or bite. u4-build-plan.md:1094-1100 lists it for his go before R5. results/u4r/93509dfd7fd1/P0/verdict.json applies 'L1 P0 loses no group' to l1-p0-s1..s3 (pass, 0). It loosens nothing: the mismatch figure passes on those runs too.
- **Fix:** The report to Mustafa that names this read lists 5e563b9's check and bite as a stricter bar built and awaiting his go. It goes under Decisions needed with the options keep (as the readings PR would) or remove, and the recommendation keep. It is not shown as decided. No file changes on this finding.

## Proposed bars (proposals only: to the advisor, then to Mustafa)

### PB-1 (proposal only; to the advisor, then to Mustafa under Decisions needed; nothing built on it)

- **Wording:** PM-0001-17 gains one check on its L1 P2, L1 P3 and L1 B1 parts: {"name": "every client-tick left incomplete off P0 is the link's loss (none unexplained)", "series": "tick.s.snap_incomplete_unexplained", "stat": "sum", "window": "measured", "op": "==", "value": 0, "parts": ["L1 P2", "L1 P3", "L1 B1"]}. A new SERIES.md row defines the series: written by npt-lab on L1 beside tick.s.snap_incomplete, one per client slot, 1 when that client filed the tick incomplete and L1's fate log shows no datagram of that client's snapshot for that tick dropped or burst-lost, and the client counted no stale drop of it; else 0. -17's bites gain a run of PM-0001-17/whole-as-lost on a short L1 P2 run, which must read 48 on the new check.
- **Why:** TE-C62H-1. The new P0 check proves the client files nothing as lost where nothing is lost. Off P0, the same misfiling hides inside 2-5 % of designed loss, on exactly the code paths where refusals happen. L1 is seeded and logs every datagram's fate (npt-l1's fate log, whose sha256 each run already records: lab/src/scenario.rs:771-776, :1158), so the bound can be exact (0 unexplained) rather than a loose percentage that would itself hide about a percent of misfiling. Its bite on a lossy run shows the check catches the bug where it hides. The netcode-architect should confirm that npt-lab can map each dropped datagram to its (client, tick) before the wording is final. If it cannot, the fallback is a ratio bound on measured incomplete share against the profile's loss, which is weaker and should be stated as such to Mustafa.
