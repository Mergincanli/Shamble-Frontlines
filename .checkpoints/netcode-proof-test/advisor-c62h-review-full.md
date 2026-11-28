# Advisor: the ruling on C62H-REVIEW's read (workflow wf_4ce9b8cd-ad2), 2026-11-15

Written by script, word for word, from the workflow's journal. Nothing below is the main session's. Its notes: advisor-c62h-review.md.

## Summary

I agree with all three of the test-engineer's answers. I checked each one against the files myself.
(1) The new check matches the advisor's rulings and the plan's readings line word for word. It does not match carry 62 (h)'s own text: two fields were changed on purpose and the carry file never recorded it. The pass mark's row in ADR 0001 says nothing about lost groups, so the check is a stricter reading that Mustafa has not answered yet.
(2) The check can fail. On the record, the planted bug whole-as-lost makes it fail with 48 (needs 0), while the mismatch check passes with 0, so this check alone catches it.
(3) On P2, P3 and B1 nothing limits how many client-ticks may be filed as left incomplete.
Findings: TE-C62H-1 is Medium and goes to Mustafa as decision C62H-BOUND. TE-C62H-2 and TE-C62H-3 are Low and are fixed now. I add two of my own: ADV-C62H-1 (Low) and ADV-C62H-2 (Medium: PB-1's wording has three flaws).
Two decisions go to Mustafa:
- C62H-KEEP: keep or remove the loss-free check that was built at 5e563b9.
- C62H-BOUND: whether the lossy networks get a limit, and in what form.
Nothing is built on this ruling alone. Read only: I wrote only my notes file, /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-c62h-review.md.

## The read's three answers

Q1, word for word: I agree with "partly". What I checked:
- `git show 5e563b9 -- marks.json` adds exactly 15 lines: 11 for the check and 4 for the planted bug. HEAD's marks.json has sha256 9fb5be57…. `git log 5e563b9..HEAD -- marks.json crates/judge` is empty.
- The committed check equals the JSON in the advisor's r2-run ruling (advisor-u4r-r2run-full.md:9). It also equals the readings line that waits for Mustafa's go before R5 (u4-build-plan.md:1094-1100).
- Carry 62 (h) at u4s-carries.md:305-307 still holds the setup form: the name starts "setup: " and the JSON has "setup": true. Its amendment line at :314-316 still says "the check's JSON stays word for word".
- Both differences come from build ruling 4 (advisor-u4r-build4-full.md:32). In the setup form, a failing setup check makes the run INVALID (judge eval.rs:1189-1211). The tick16 and whole-as-lost runs would then read INVALID, and neither planted bug could ever be shown.
- The planted bug's text equals build ruling 1's amended wording (advisor-u4r-build1-full.md:33).
- One point the read touched only in passing: the carry's original planted bug was "a client that writes null and incomplete 1 for a refused group". Build ruling 1 changed it to "a tick decoded whole, filed as lost", because P0 never refuses anything (0 sentinels in three runs). The amended carry records that change.
- ADR 0001:1605 hashes, without its newline, to 3075f638…, equal to marks.json's pin. The row contains none of "incomplete", "lost", "group" or "loses no group". plan.md's -17 rows (:462, :475, :721) say nothing about it either.
- So the check is a stricter reading of the row, built on the advisor's rulings, and still waits for Mustafa's go.

Q2, can it fail: I agree with "yes".
- The record folder .gate/netcode-proof-test/c62h-5e563b9589d3-record/ holds a README.txt. It names commit 5e563b9589d3…, marks.json 9fb5be57…, the judge 65acacfe… (`sha256sum` of the binary agrees) and `--only PM-0001-2,PM-0001-17`.
- Each verdict compares equal (`cmp` exit 0) to its file in -diag-with-2/: sha256 0ee24942… (whole-as-lost) and da62fe7f… (tick16).
- whole-as-lost-verdict.json:157-178: the planted bug is "shown", run u4r-r2-whole-as-lost-1 is valid, the mismatch check passes at 0, and "L1 P0 loses no group" fails at 48 (needs == 0).
- tick16: shown, with 2,880 mismatches and 720 on the new check, both failing.
- Where the bug acts: lab/src/logs.rs:317-330 (`bitten`: a hash other than 0, at each client slot's first measured tick), :362-366 (filed as lost) and :441-449 (null and incomplete 1).
- The unit test is scenario.rs:2684-2694: 16 bitten, 0 clean.
- The judged P0 set at 93509dfd7fd1 shows it again. In verdict-bites.json, whole-as-lost reads 48 (fail) with mismatches 0 (pass). Its clean runs s1-s3 read 0 (pass).
- One note of mine: the planted bug sits in npt-lab's log writer, not in npt-client. It proves that the check and the judge catch the outcome. That is the right level for a check that reads the logs.

Q3, any limit off P0: I agree with "not bounded". What I checked:
- The check's parts are ["L1 P0"] only. A grep of marks.json finds no other mark with a check on snap_incomplete.
- The judge's coverage rule (figures.rs:466-481) counts an incomplete tick as resolved. Its "empty" floor applies only when the whole run compared nothing (:547-551).
- The client files a tick as lost unless it filed something else first (client/src/lib.rs:580-607).
- -57 counts only ticks filed with a hash other than the sentinel (logs.rs:15-17).
- -2's 'everyone in view' comes from the room's log (figures.rs:24-40).
- In the B1 set at 64f3307d5c7f, -17 L1 B1 and -18 L1 B1 are both INVALID.
- npt-lab's runs.txt lines give the measured counts: P2 17,260 / 17,089 / 17,532 and P3 43,295 / 43,210 / 43,454, out of 864,000 measured client-ticks each. On every run stale = 0 and sentinels = 0.
- Two corrections to the read:
  - B1 does have counts: npt-lab measured 31,346 / 31,279 / 31,635. Only the judge has no -17 value for B1.
  - -58's L1 P0 wrap part also has no such check; the read named only its L1 P3 part.
- My estimate, not a check (it assumes one datagram per client-tick): the profiles' designed loss predicts 17,280 on P2 (standard deviation about 130), 43,200 on P3 (about 200), and about 31,400 on B1 (2 % outside ten 1-second bursts, plus 48 × 10 × 30 inside them). Every run is within 2 standard deviations, so no misfiling shows today.
- The read's list of what could hide stands. A decode or apply bug filed as lost instead of the sentinel; a bookkeeping slip that falls into 'lost unless filed'; stale drops that are too eager; a bad recovery after a B1 burst (-18 times only the first decode); or one client filing nearly everything as lost. Each would go unseen by -17, -57 and the judge, as long as one hash in the run matches.

## Each finding

- **TE-C62H-1** (Medium): follow-up; home: Decisions needed C62H-BOUND (his answer decides whether a limit is added, and its form; nothing is built on this ruling). Until he answers, these report lines, recorded: u4r-report.md's P2 and P3 lines and the card, and PR #40's report when it is written. Each run's measured incomplete count goes beside the share the profile's loss predicts, with the plain sentence that -17 off P0 does not limit this count. B1's line comes with U4g's judged B1 runs.. I confirmed it from the files: figures.rs:466-481 and :547-551, client lib.rs:580-607, logs.rs:15-17, and a grep of marks.json. A misfiled tick is a decoded snapshot that the ADR row's '100 % of decoded snapshots' never compares, so the row's own words have a hole off P0. It stays Medium, not High. No result is wrong today: sentinels and stale are 0 on every judged run, and the counts fit the designed loss within 2 standard deviations (my estimate).
- **TE-C62H-2** (Low): fix now; home: u4s-carries.md, one line under carry 62 (h) after :316: the read's line word for word, plus 'Mustafa's answer to C62H-KEEP decides whether it stays'. It rides the commit that records this ruling, with no push of its own.. It is true (u4s-carries.md:305-307 and :314-316 against advisor-u4r-build4-full.md:32 and advisor-u4r-r2run-full.md:9). It is a one-line notes fix. It also stops the next reader from finding two unexplained differences.
- **TE-C62H-3** (Low): fix now; home: The report to Mustafa that names this read: Decisions needed C62H-KEEP. Wherever the card and u4r-report.md name the check, they say 'built on the advisor's rulings, awaiting his answer'.. It is true: ADR 0001:1605 is unchanged and has no such words, and the judged P0 set already applies the check. It was not a breach. The check was built on 2026-11-14, before his rule of 13:35, on the route then standing: the readings-PR list for his go before R5 (u4-build-plan.md:1094-1100). Under his new words it now goes to him as a decision, and is never shown as decided.
- **ADV-C62H-1** (Low): recorded; home: The report words (this ruling): B1's npt-lab counts and -58's L1 P0 wrap part go into the Q3 answer.. The read's Q3 left out two things: npt-lab's B1 counts (31,346 / 31,279 / 31,635 in results/u4r/64f3307d5c7f/B1/runs.txt), and -58's L1 P0 wrap part, which also has no incomplete check. Its conclusion still stands.
- **ADV-C62H-2** (Medium): recorded; home: C62H-BOUND option (a) carries these as design conditions. The netcode-architect designs the exact check, the advisor and a test-engineer read it, and its exact wording goes to Mustafa before anything is built.. PB-1 as worded would be a wrong rule, for three reasons. (1) It excuses a tick the client 'counted as a stale drop' on the client-under-test's own counter. That reopens the read's own hiding place (c); staleness must come from the link's record, never from the client's counters. (2) Ticks the room sent no snapshot for are filed as lost by design: the gap after a FullState until it is acknowledged (NC-L10), and after a B1 burst. Under PB-1 they would read 'unexplained' and fail clean runs. (3) The fate log has no tick field (l1/src/lib.rs:308-321) and is kept only as its sha256 (scenario.rs:771-776). So npt-lab needs a new record of which client and tick each snapshot datagram carries, and the judged P2 and P3 sets must be re-run; old logs cannot serve. Nothing is built, so this is recorded, not fixed.

## The proposed bars

- **PB-1 (the read's proposed limit on client-ticks left incomplete, on -17's L1 P2, P3 and B1 parts)**: to Mustafa: yes. It is a new pass bar, so it goes to Mustafa under his words of 13:35, as C62H-BOUND. It is not built on this ruling. I recommend the direction, an exact limit derived from the simulated network, but not PB-1's wording as it stands: ADV-C62H-2 lists its three flaws. The exact check comes back to him word for word before anything is built.
- **5e563b9's check 'L1 P0 loses no group (no client-tick left incomplete)' with planted bug PM-0001-17/whole-as-lost (already built)**: to Mustafa: yes. It is a stricter bar, built on the advisor's rulings, that Mustafa has not answered (TE-C62H-3). It goes to him now as C62H-KEEP (keep or remove) and is never shown as decided. The readings PR still carries its words for his go before R5, as planned.

## Decisions for Mustafa

### C62H-KEEP

- **Question:** Do you keep the stricter loss-free check on the mark 'every decoded snapshot is the server's state' (PM-0001-17)? It fails the mark if, on the perfect network, any client files a snapshot as 'lost'. It went into the pass-bar file on 2026-11-14 (commit 5e563b9) on my rulings alone, together with a planted bug that proves it works.
- **Why it matters:** It is the only check that catches a client hiding snapshots it decoded as 'lost' (the mismatch check passes at 0 on that bug). But it is stricter than the ADR's row, and you have not said yes to it.
- **Options:** (a) Keep it as built. It already applies to the judged loss-free set: all three runs pass with 0, and the planted bug is caught with 48. Its words still go in the readings pull request for your go before R5. (b) Remove the check and its planted bug: a 15-line pass-bar file change with a reviewer's read, after which the loss-free set is re-judged (its clean runs pass either way).
- **Recommendation:** (a) Keep. It costs nothing on clean runs, and it closes the one way that a client bug could hide on the perfect network.

### C62H-BOUND

- **Question:** On the lossy networks (P2 with 2 % loss, P3 with 5 %, and B1 with bursts), should the same mark also get a limit on how many snapshots a client may file as 'lost'?
- **Why it matters:** Today a client that wrongly files received snapshots as lost hides inside the 2 to 5 % that the network really drops, on exactly the code paths most likely to be wrong (deltas after loss, reorder, recovery after a burst), and nothing fails. Today's counts show no such excess, but only by eye.
- **Options:** (a) An exact limit. The lab, which runs the simulated network, records which client and tick each snapshot packet carries. A tick filed as lost passes only if the network dropped a piece of it, delivered it too late (judged from the network's record, never from the client's own counters), or the server sent nothing for it. The check is 0 unexplained on L1 P2, P3 and B1, with a planted bug on a short P2 run. Cost: one lab change, one pass-bar check and one series definition, each with a reviewer's read, plus re-running the judged P2 and P3 sets (about 35 minutes of runs; B1 comes with U4g's judged B1 runs). (b) A count limit. Per run, the snapshots filed as lost may not exceed the number of snapshot packets the network dropped, plus the server's known gaps. It is cheaper, but weaker: it has slack wherever one tick loses several packets. (c) No new check. The report shows each run's count beside the share the profile's loss predicts, and says plainly that this count has no limit off the perfect network.
- **Recommendation:** (a). Its exact wording, designed by the netcode-architect and read by me and the test-engineer, comes back to you word for word for a yes before anything is built. Build it before U4g's judged B1 runs. Meanwhile (c)'s report lines go in now in any case. If the netcode-architect finds (a) cannot be built cleanly, (b) comes back to you, stated as weaker.

## Units ahead that change marks.json or the judge

Mustafa's rule of 2026-11-15 13:35: every change to marks.json or to the judge (spikes/netcode-proof/crates/judge/) gets a reviewer's read before its unit closes, and the report names it in its own line with what covered it. The test at each unit's close: `git diff --stat <unit base>..<head> -- spikes/netcode-proof/marks/marks.json spikes/netcode-proof/crates/judge`. If it is not empty, a read is owed. Inside a unit's review round, the read can be the test-engineer's review, if its notes name the marks.json and judge lines read and marks.json's sha256 before and after. A change made outside a round (a main-session commit like 5e563b9) gets its own short read, like this one.
Coming units that change them:
1. JDG-ONLY-1's fix (the judge's main.rs and eval.rs, with cli.rs fixtures). Certain. It is npt-judge's next change, before PR #40 is marked ready at the latest. Readers: test-engineer, plus rust-reviewer for the judge code.
2. C62H-BOUND, if he picks (a) or (b): marks.json (one check and one planted bug), npt-lab, SERIES.md, and the judge only if a new figure is needed. Readers: test-engineer and netcode-architect.
3. C62H-KEEP, if he picks remove: marks.json loses the 15 lines. Reader: test-engineer.
4. U4f moves -47, -20, -21 and -22. Its brief must say whether marks.json changes (planted bugs, notes, values). If it does, the read is owed.
5. U4g moves -41, -42, -60, -45, -61, -62, -55, -56 (6) and -58. Its plan also says how a B1 building entry reads in -17, and carry 58's high-water marks come before -18's judged B1 runs. marks.json changes are likely.
6. U4h: certain judge changes. It adds the figures view_hit_registered_share, false_hit_share, rule_cap_set_by_rule and throw_desync_count, with fixtures, and -48's test cap.
7. The readings PR, only if it changes an ADR 0001 row that marks.json pins (draft.row_sha256), because the pins then change.
Recommended, though his words do not name them: the same read for marks/SERIES.md (its :320 cell and the :368 correction, at SERIES.md's next edit) and marks/README.md. README.md:6 says that contract changes only together with npt-judge and its fixtures.
Not covered unless they touch those paths: step (7) U4's report fold, TE-B1-R2-2's runner edit, ADV-PT-L3's change to check.mjs, RATES-1 and RUNS-LINE in c-u4r-runs.mjs, and KEPT-LOGS.

## The report words

1. Its own line, under his rule: "Pass-bar file and judge changes: 5e563b9 (2026-11-14 16:14 UTC; marks.json +15 lines: PM-0001-17's check 'L1 P0 loses no group (no client-tick left incomplete)' and the planted bug PM-0001-17/whole-as-lost). Covered by: the test-engineer's read C62H-REVIEW (a) (notes c62h-review-test.md; marks.json sha256 9fb5be57…) and the advisor's ruling on it (advisor-c62h-review.md). Before that, the change was covered only by the advisor's rulings that designed it (build ruling 4, the r2-run ruling, the c62h ruling) and by check.mjs's 11 steps; no reviewer had read it. Nothing has changed marks.json or crates/judge since (git log 5e563b9..HEAD: empty)."
2. The answers in plain words:
- Q1: partly. The check matches the advisor's rulings and the plan's readings line. It differs from carry 62 (h)'s text in two fields that build ruling 4 changed on purpose ('setup' removed, so the planted bugs can be shown). ADR 0001's row has no such words, so the check is a stricter reading that awaits his answer.
- Q2: yes. On the record, whole-as-lost fails the check with 48 (needs 0) while the mismatch check passes at 0, and the judged P0 set shows the same.
- Q3: not bounded. Measured incomplete client-ticks out of 864,000 per run:
  - P2: 17,260 / 17,089 / 17,532, against the 17,280 that 2 % predicts.
  - P3: 43,295 / 43,210 / 43,454, against 43,200 (5 %).
  - B1 (npt-lab's count; the judge's -17 part is INVALID): 31,346 / 31,279 / 31,635, against about 31,400.
  These are the advisor's estimates, assuming one datagram per client-tick; no check makes the comparison. Stale = 0 and sentinels = 0 on every run. -17 does not limit this count off P0, and neither do -58's two wrap parts.
3. The findings:
- TE-C62H-1 (Medium): to C62H-BOUND.
- TE-C62H-2 (Low): one line in the carries file.
- TE-C62H-3 (Low): to C62H-KEEP.
- ADV-C62H-1 (Low): recorded here.
- ADV-C62H-2 (Medium): PB-1's wording has three flaws, carried as design conditions in C62H-BOUND (a).
4. Where the lines go: the Q3 lines go into u4r-report.md's P2 and P3 lines, on the card, and into PR #40's report.
5. Decisions needed: C62H-KEEP and C62H-BOUND, in the advisor's form, with the recommendations (a) and (a).
6. Nothing is built on this ruling.

## For Mustafa

The read is done and I have ruled on it.

**What the read found:**
- The new pass-bar check matches the advisor's rulings and the plan's readings line. It differs from the carry's text in two fields, changed on purpose, and the carry file never recorded that change. It now gets one line.
- The ADR's row for that mark says nothing about lost data, so the check is stricter than the row, and you have not said yes to it.
- The check works: on the planted bug it fails with 48 (needs 0), while the 'snapshots match' check passes. Nothing else would catch that bug.
- On the lossy networks nothing limits how much data a client may file as 'lost'. A client bug there would hide inside the 2 to 5 % the network really drops. Today's counts fit that loss closely, so nothing seems hidden now, but only an eye comparison says so.

**Decisions needed:**
- **C62H-KEEP:** do you keep the stricter loss-free check on 'every decoded snapshot is the server's state' (PM-0001-17)? It was added to the pass-bar file on 2026-11-14 on my rulings alone.
  - Why it matters: it is the only check that catches a client hiding good snapshots as 'lost', but it goes beyond the ADR's wording.
  - Options: (a) keep it as built (all three clean loss-free runs pass); (b) remove it and its planted bug.
  - My recommendation: (a).
- **C62H-BOUND:** on the lossy networks (P2, P3, and B1 with bursts), should the same mark limit how many snapshots a client may file as 'lost'?
  - Why it matters: today a client bug can hide there unseen, on the code most likely to be wrong.
  - Options:
    - (a) An exact limit, worked out from the simulated network's own record: every 'lost' snapshot must be explained by real loss, lateness, or the server sending nothing. It needs one lab change, and the two judged lossy sets re-run (about 35 minutes).
    - (b) A cheaper count limit, which is weaker.
    - (c) No new check: the report only shows each count beside what the network's loss predicts.
  - My recommendation: (a). Its exact wording comes back to you for a yes before anything is built. Meanwhile, (c)'s report lines go in now.

**For Mustafa:**
1. Answer C62H-KEEP and C62H-BOUND (above).
