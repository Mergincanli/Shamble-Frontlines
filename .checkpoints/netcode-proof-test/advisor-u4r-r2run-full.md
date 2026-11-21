# Advisor: the ruling on U4r's r2 record run and carry 62 (h) (workflow wf_ec319c2d-f45), 2026-11-14

Written by script, word for word, from the workflow's journal (advisor-u4r-r2run's StructuredOutput result). The run: .gate/netcode-proof-test/r2-e59ece9ecafc/r2.txt. Its notes: advisor-u4r-r2run.md. Nothing below is the main session's.

## Carry 62 (h)

The condition holds: -17's plain check goes into marks.json. Which suites count: -17's L1 P0 part selects layer L1, scene SS, profile P0 and 48 clients (marks.json). The clean short runs in r2 that it selects, measured role and no bite, are p0, forged, clamp, fast-clock, evstart and tick16-clean. I recomputed each one's measured-phase sum of tick.s.snap_incomplete from its client.jsonl: 0, 0, 0, 0, 0 and 0. Their warm-up sums are 0 too, and each figure equals the contract's printed line. This is the same set that build ruling 4's further finding named (it also listed the builder's p0-s1c), and that ruling left lag-switch out. Outside the condition: es (scene ES; its sum is 0 anyway, so no reading changes the outcome); p2, p3 and b1 (profiles P2, P3 and B1); tick16 (720) and whole-as-lost (48), which are bite runs whose sums are the bites' own. lag-switch: a -21 option-B stand-in on profile P1 (50 ms, 1 % loss), not P0, with marks PM-0001-2 only. Its 300 is 1.04 % of 28,800 measured client-ticks, which is P1's designed 1 % loss, on the same scale as P2's 2.03 % and P3's 5.16 %. It was the same 300 in the builder's copies 3 and 5. So it is outside the condition, and no cause needs finding. Raising it with me before acting was right. Under Mustafa's limit, the main session itself does the change and reads all of it: marks.json is its lane, so no agent edits it. The exact change: in PM-0001-17's checks, append {"name": "L1 P0 loses no group (no client-tick left incomplete)", "series": "tick.s.snap_incomplete", "stat": "sum", "window": "measured", "op": "==", "value": 0, "parts": ["L1 P0"]}, with no "setup" and no "setup: " prefix. In its bites, append {"id": "PM-0001-17/whole-as-lost", "what": "a client writer that files a tick it decoded whole as lost: null and incomplete 1"}. Nothing else in marks.json changes. Add one line to u4-build-plan.md §11's readings-PR list, for his go before R5. I checked that the judge accepts this check: Rule::Pre reads the whole marks file whatever --only says (eval.rs:958-996), and the schema takes a measured-window series check without setup (marks.rs:274-283, :505-540). Expected from npt-judge --only PM-0001-17 with the committed marks.json: on r2's whole-as-lost folder, bite PM-0001-17/whole-as-lost is shown, its run is valid, the new check reads 48 (needs == 0, FAIL) and mismatches 0. On r2's tick16 folder, bite PM-0001-17/tick16 is shown, its run is valid, mismatches are 2,880 and the new check reads 720, both FAIL. Anything else (INVALID, missed, a run invalid, the check not applied) comes to me whole, and marks.json is not changed again until I rule.

## Next steps

1. 1. Record this ruling word for word by script (advisor-u4r-r2run-full.md). On the card: r2 at e59ece9 is the record, all 14 suites as expected, the three ruled fails (b1, es, b1-nofallback) recorded and never re-run; carry 62 (h)'s condition is met by the six clean short L1 P0 suites (all 0); lag-switch's 300 is P1's designed 1 % loss and outside -17's L1 P0 part. I checked these myself: spikes/ is the same at 71ee13d and e59ece9 (`git diff --quiet` exits 0, tree b57488ad... at both, and the five commits between touch only .checkpoints/); the binaries' and the copies' sha256s equal r2.txt:2-5; r2.txt's suite blocks equal r2.log; no 'stopped at' or 'exit by' line, and every exit is 0; no commit landed between 15:40:04 and 15:45:51 UTC, so INF-R2-1's interim rule held.
2. 2. The marks.json edit, made by the main session itself with Edit and read in full: exactly the check and the bite given in carry62h, in PM-0001-17's entry, and nothing else. In the same edit, add one line to u4-build-plan.md §11's readings-PR list: 'PM-0001-17's L1 P0 part gains a plain check, "L1 P0 loses no group (no client-tick left incomplete)": the measured sum of tick.s.snap_incomplete == 0, so a lost group on P0 FAILs -17 (stricter; it was a report only), with its bite PM-0001-17/whole-as-lost (carry 62 (h); the advisor's U4r build ruling 4, further finding; r2 at e59ece9 shows 0 on the six clean short L1 P0 suites)'. Do not change SERIES.md now: its row :320 Marks cell, '(the coverage rule; -17 reports it)', gains 'and checks its measured sum on L1 P0 (carry 62 (h))' at SERIES.md's next edit, together with ruling 3 point 8's correction of row :368 (Low).
3. 3. Before the commit, on the edited tree: (a) `npt-judge --marks spikes/netcode-proof/marks/marks.json --check-marks` with r2's binary (.gate/netcode-proof-test/bin/r2-e59ece9ecafc/npt-judge, sha256 65acacfe...) must exit 0. (b) `node check.mjs` whole, through main-run.sh (lane lock, offline), because tests read the real marks.json (the judge's fixtures.rs and engine.rs, room/tests/bites.rs:22, server/tests/schedule.rs:355) and the marks step runs --check-marks. Expected: all 11 steps pass, test 531 and 609 with 0 failed, marks 81 with the ADR rows unchanged. A FAIL comes to me whole and is never retried; TE2-5's naming rule applies to any failed test.
4. 4. Commit (marks/marks.json, the plan's line and the card), for example 'carry 62 (h): -17's L1 P0 plain check and PM-0001-17/whole-as-lost in marks.json (the advisor's U4r build ruling 4; r2 at e59ece9: 0 on the six clean short L1 P0 suites)', and push. First confirm `git diff --quiet e59ece9 HEAD -- spikes/netcode-proof/crates` exits 0, so r2's judge binary is still the commit's judge.
5. 5. The record of step 12, from that commit: run r2's npt-judge with the committed marks.json and `--only PM-0001-17` on .gate/netcode-proof-test/r2-e59ece9ecafc/whole-as-lost and on .../tick16. Write --out into a new folder, .gate/netcode-proof-test/c62h-<commit12>/ (whole-as-lost-verdict.json, tick16-verdict.json), never into r2's folder, and keep a text file there naming the commit, marks.json's sha256 and the judge's sha256. Read both verdicts in full. Expected as in carry62h: both bites shown, never INVALID. Anything else comes to me whole before step 13.
6. 6. Record the result on the card and in u4r-report.md's r2 section, in the same edit that folds MRG-L1 to MRG-L3 and r2's 14 suites (expected beside what came). Write lag-switch's 300 as P1's designed loss and the tick16 'PM-0001-2 passes' as the bite run's validity. No new review round, and no push of its own.
7. 7. Then step 13, U4r-F1, as ruled. One Low rides it (RUN-L1). r2.log's provenance block is empty: c-u4r-r2.mjs's sh() keeps cargo's output, so no 'Compiling npt-*' line reaches the runner log, and c-u4r-runs.mjs has the same sh(). Both scripts print cargo's 'Compiling/Fresh npt-* (path)' lines from stderr into the runner log. The record still holds, because the clean-tree check, the commit and the binaries' sha256s in r2.txt identify the build.
8. 8. Steps 15 and 17 now count the new bite. c-u4r-runs.mjs's bitesOf (:170-180) reads -17's bites from marks.json at run time, so the P0 set gains bite-whole-as-lost: about 56 MB projected and about 11 minutes. The disk plan counts it. P0's expected lines become: whole-as-lost shown by the new check (48); the 10-minute tick16 bite shown by both the mismatch figure and the new check (720 = 48 x 15, its resyncs about 48 at about wrap + 60, all in the after phase, as build ruling 4 says); every clean judged L1 P0 run reads 0 on the new check. A non-zero result there is a FAIL, recorded under condition 5 and never re-run, and it comes to me with its cause before -17's verdict is reported.
9. 9. Nothing goes to Mustafa now. The marks.json change reaches him on the readings-PR list before R5, and PR #40's report carries r2's story: B1's blindness after loss bursts, ES's event rate, and -18's no-fallback bite, all ruled already.

## For Mustafa

Nothing for you to decide or do now. The record run of the netcode proof test's short suites, run from U4r's merge, came out exactly as ruled. I checked all 14 suites against their logs. 11 pass, and the 3 known failures came out as expected. Those 3 are already explained in U4r's report. After each short loss burst, every player loses sight of the others for a few seconds; the room server causes this, and the U4g unit fixes it. The busy test scene's bots hit far more often than its target allows, which the U4h unit fixes. A planted defect in the burst recovery is caught only by the bursts' visibility check, until U4g's fix lands. One open question is now settled. Before adding a stricter pass rule, we had to show that the clean, loss-free runs drop no snapshot data. They drop none: 0 on all six. The one run that dropped some, 300 incomplete snapshots, uses a network profile with 1 % packet loss on purpose, so that loss is expected. That run is not covered by the rule. So the main session now adds the stricter rule to the marks: on the loss-free network, a single lost piece of snapshot data fails the 'every decoded snapshot matches the server' mark. It also adds a planted defect that proves the rule catches the problem. This tightens one of your pass marks, and loosens nothing. Like every change to the marks, it goes on the list of readings that waits for your go before R5. You will see it there with the rest. Decisions needed: none. For Mustafa: nothing.

## Each suite

### p0 (matches)

- **Expected:** contract PASS; PM-0001-2's SS checks, -17 (0 mismatches) and -57 pass on the run
- **Came:** exit 0 on all three commands; contract PASS (74 names, 0 without a row, 0 breaches, 48 admits); per run (p0-verdict.json): -2 11 of 11 pass, -17 L1 P0 0 mismatches, -57 6,981 / 2,750 / 88,044 pass; run valid; measured incomplete 0. The judge's NOT-RUN lines only mean '1 of 3 valid runs'.
- **Ruling:** Matches. Recorded as the record.

### p2 (matches)

- **Expected:** contract PASS; PM-0001-2, -17 and -57 pass
- **Came:** contract PASS; -2 11 of 11, -17 (L1 P2 part) 0 mismatches, -57 6,974 / 2,750 / 88,061; figure for figure equal to the builder's copies 3 and 5 (38,311 / late 112 / 6,194 / incomplete 585).
- **Ruling:** Matches. Its 585 is P2's 2 % loss (585 of 28,800 client-ticks = 2.03 %); P2 is not an L1 P0 run, so it is not carry 62 (h)'s condition.

### p3 (matches)

- **Expected:** contract PASS; PM-0001-2, -17 and -57 pass (carry 62 (g): P3, the reorder step on)
- **Came:** contract PASS; -2 11 of 11, -17 (L1 P3) 0 mismatches with no gap, -57 6,952 / 2,750 / 88,062; equal to copies 3 and 5 (35,169 / 73 / 9,139 / 1,485).
- **Ruling:** Matches; carry 62 (g) shown on the record. 1,485 = 5.16 % against P3's 5 % loss.

### b1 (matches)

- **Expected:** contract PASS; PM-0001-2 FAIL (everyone in view 0.273333), so -17 and -18 L1 B1 INVALID by pre:PM-0001-2, recorded and not counted; -57 passes (ruling 3, point 1)
- **Came:** contract PASS, 0 breaches; -2 FAIL 0.273333 (I recounted room.jsonl: worst slot 246 of 900 measured ticks); -17 and -18 L1 B1 INVALID by pre:PM-0001-2; -57 L1 B1 6,974 / 2,750 / 88,060 pass; 96 heals, all within 500 ms (max 129.6 ms).
- **Ruling:** Matches the ruled fail. Recorded under ruling 3, point 1; never re-run. The room's fix and -17's and -18's L1 B1 evidence stay U4g's.

### forged (matches)

- **Expected:** contract PASS; PM-0001-2 passes
- **Came:** contract PASS; -2 11 of 11; measured incomplete 0. Its room and client logs are line for line those of fast-clock; its lab log differs only by one press counter_wrap event (tick 513).
- **Ruling:** Matches. Note, not a finding: the presses script is wired (the wrap event), and with fire held its press edges change no game state, so this stand-in shows only that the load holds, which is all its expected line claims.

### clamp (matches)

- **Expected:** contract PASS; PM-0001-2 passes
- **Came:** contract PASS; -2 11 of 11 (shots 700.1); measured incomplete 0.
- **Ruling:** Matches.

### fast-clock (matches)

- **Expected:** contract PASS; PM-0001-2 passes
- **Came:** contract PASS; -2 11 of 11; measured incomplete 0.
- **Ruling:** Matches.

### lag-switch (matches)

- **Expected:** contract PASS; PM-0001-2 passes
- **Came:** Header: layer L1, scene SS, profile P1, behaviour steady, marks PM-0001-2 only. contract PASS; -2 11 of 11 (everyone in view 1.0, 600 of 600); measured incomplete 300 (warm-up 219, after 15), recomputed by me from client.jsonl; equal to copies 3 and 5 (42,771 / 222 / 1,811 / 300).
- **Ruling:** Matches its expected line. Its 300 is not a carry 62 (h) item: the run is P1 (50 ms, +-10 ms, 1 % loss each way), outside -17's L1 P0 part. 300 of 28,800 client-ticks is 1.04 %, against 1 % loss; the same scale holds for P2 (2.03 % against 2 %) and P3 (5.16 % against 5 %). So it is P1's designed loss: lost groups leave ticks incomplete, as SERIES.md's row says.

### es (matches)

- **Expected:** contract PASS; PM-0001-2 FAIL by ES's reliable events (50.05 a second against 9.5-10.5), the other 11 ES checks pass; no ES run counts at U4r
- **Came:** contract PASS; -2 FAIL on 'at most +5 %' only: 50.05 against 10.5; the other 11 checks pass (shots 72, projectiles 32.38, spawns 21.1, everyone in view 1); measured incomplete 0.
- **Ruling:** Matches the ruled fail. Recorded under ruling 3, point 2 (the brawl's ES hit rule is U4h's); never re-run; counts for nothing.

### evstart (matches)

- **Expected:** contract PASS; PM-0001-2 and -17 (0 mismatches) pass (carry 75)
- **Came:** contract PASS; -2 11 of 11; -17 L1 P0 0 mismatches; measured incomplete 0.
- **Ruling:** Matches; carry 75 shown on the record.

### tick16 (matches)

- **Expected:** contract PASS (0 mirror breaches); PM-0001-2 passes; -17's bite run FAILs by decoded_hash_mismatch_count with at least 48 mismatches; after_capped as it comes; measured incomplete is the bite's own (720 = 48 x 15)
- **Came:** contract PASS, 0 breaches; bite PM-0001-17/tick16 'shown' with 2,880 mismatches; the run is valid (everyone in view 1.0 on 600 of 600, recounted); after_capped false (46 after-phase ticks, below the 150 cap); all 48 overflow resyncs at 65,596 (wrap + 60), in the after phase; measured 64,951-65,550, so the wrap is at measured tick 585; measured incomplete 720 = 48 x 15. Equal to the builder's tick16d.
- **Ruling:** Matches. Note for reading: the judge's PM-0001-2 line says NOT-RUN because no -2 part takes a bite run. 'PM-0001-2 passes' is shown by the bite run's validity, because pre:PM-0001-2 holds. Never INVALID: carry 73 is met on the record.

### tick16-clean (matches)

- **Expected:** contract PASS (0 mirror breaches); PM-0001-2, -17 (0 mismatches) and -57 pass; the after phase resolves without the cap
- **Came:** contract PASS, 0 breaches; -2 11 of 11, -17 0 mismatches, -57 6,981 / 2,750 / 88,044; 0 after-phase ticks, after_capped false; measured incomplete 0 across the wrap (at measured tick 300).
- **Ruling:** Matches; ruling 3, point 7 (b) is shown across the wrap on the record.

### b1-nofallback (matches)

- **Expected:** contract PASS (0 mirror breaches); -18's max check FAILs (a heal later than 500 ms); INVALID by pre:PM-0001-2 is expected while npt-room is unfixed
- **Came:** contract PASS, 0 breaches; heals 96, 25 within 500 ms, max 1,688,286,323 ns (I read it from lab.jsonl: above 500,000,000, so the max check fails); 71 overflow resyncs, each heal after them carrying full_state 1; bite INVALID by pre:PM-0001-2 at 0.453333 (worst slot 408 of 900, recounted). Equal to the builder's b1-nofallback2.
- **Ruling:** Matches the ruled fail (ruling 3, points 4 and 8 (A)). An invalid run lists no checks, so the max FAIL is read from the heal records, as the builder did. Recorded; never re-run. -18's bitten evidence stays U4g's.

### whole-as-lost (matches)

- **Expected:** carry 62 (h)'s bite: measured incomplete equal to the client count (48); -17's verdict as it comes until marks.json holds ruling 4's plain check
- **Came:** contract PASS, 0 breaches; measured incomplete 48 (every client's first measured tick, 64,036, filed null), client hashes 43,104 = 43,152 - 48; everyone in view 1.0; -17 NOT-RUN, because the bite id is not in marks.json yet.
- **Ruling:** Matches. This is the folder for step 12's -17 check once marks.json holds the plain check and the bite.
