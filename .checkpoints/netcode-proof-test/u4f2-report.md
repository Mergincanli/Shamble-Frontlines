# U4f2 report: -21's forged-command test (merged at b4e1f43) and -21's L1 set (2026-11-22)

## In plain words
- **What U4f2 is:** the forged-command half of PM-0001-21 ("forged commands and lag switches change nothing"). Eleven
  cheating clients each forge one kind of bad command; the server must refuse and count every one, and a twin run
  without the forgeries must match the real run tick for tick. U4f2 also finished the code for -21's other three parts
  (clamp, fast clock, lag switch) under option B.
- **Result:** -21's whole L1 set ran on 2026-11-22 (49 runs, 01:48-05:30 UTC). **Every part passes under both
  options on all three seeds, and all 13 planted bugs are caught.** -21 is judged only at L1 in the re-plan
  (replan-marks.md:40), so its judged runs are done.
- **His F-B-COUNT (a) at work:** kind 7 (firing or reloading too early) presses made while the forger's soldier was
  dead are left out of the count and printed beside the check: 5 to 18 a run, every other kind 0.
- **Size:** about 2,674 added lines, under his 3,000 stop. Nothing is stopped on him.

## -21's L1 set
- **How it ran:** `c-u4f-runs.mjs 21 --date 2026-11-22`, detached, from f591dcf, whose spikes/netcode-proof equals
  b4e1f43's outside results/ (`git diff --stat` empty), in the main folder's own cache under the lane lock; builds Fresh.
  Only -21's set ran: NPT-TRIM's KEEP 1 replaces the header's -47, -20, -22 order (those three ran in U4f:
  u4f-report.md:547-700). Hashes, checked again before each judge call: npt-lab 81eda1f2…, npt-lab (bites) ed7d77af…,
  npt-judge d8b8c91e…, marks.json c1935a5b….
- **Disk:** 7,824 MiB free before the builds (the advisor asked about 7.50 GB); projected 3,054 MiB, logged 3,048.1 MiB
  (2,238.6 measured, 809.5 bites); 4,997,857,280 B free after.
- **Where:** results spikes/netcode-proof/results/u4f/f591dcf71ca6/pm-0001-21/ (runs.txt, verdict.json,
  verdict-bites.json); raw logs .gate/netcode-proof-test/runs/u4f-21-f591dcf71ca6/ (3.0 GiB, packed only on his word).
- **Verdicts, two calls by design** (runs.txt's STOP_NOTE): the measured call gives PM-0001-2's SS runs PASS on all 24
  runs (shots 685.77-695.66 a measured second against the band 665-735) and **all 8 of -21's parts PASS** (forged,
  clamp, fast clock, lag switch; options A and B; 3 valid runs each). Each mark reads NOT-RUN in that call by design:
  -2 has no ES run in this set, and -21's bites are in the other call. The bites call: **all 13 bites shown**, every
  part "0 of n" by design. Read together: PM-0001-21 PASSES at L1.

### Kind 7 beside "every forged command rejected and counted" (F-B-COUNT (a): every forged run)
| Run (seed) | kind 7 left out | kind 7 sent | kind 7 unrejected | the judge's value | outcome |
|---|---|---|---|---|---|
| forged A s1 / s2 / s3 | 18 / 17 / 16 | 1,505 / 1,506 / 1,507 | 0 / 0 / 0 | 0 / 0 / 0 | PASS |
| forged B s1 / s2 / s3 | 8 / 15 / 12 | 1,515 / 1,508 / 1,511 | 0 / 0 / 0 | 0 / 0 / 0 | PASS |
| bite no-lead-bound / no-newer-tick | 10 / 8 | 1,513 / 1,515 | 0 / 0 | 1,523 (kind 1) / 1,523 (kind 2) | shown |
| bite no-seq-forward / no-move-length | 12 / 15 | 1,511 / 1,508 | 0 / 0 | 1,523 (kind 3) / 1,523 (kind 4) | shown |
| bite no-press-edge / no-aim-bounds | 5 / 11 | 1,518 / 1,512 | 0 / 0 | 1,523 (kind 5) / 1,523 (kind 6) | shown |
| bite no-cooldown | 12 | 1,511 | 1,325 | 1,325 (kind 7) | shown |
| bite no-ack-sent / no-trailing | 8 / 11 | 1,515 / 1,512 | 0 / 0 | 1,523 (kind 8) / 1,523 (kind 9) | shown |
| bite no-catchup-cap / no-channel | 6 / 18 | 1,517 / 1,505 | 0 / 0 | 1,656 (kind 10) / 1,523 (kind 11) | shown |
| r2 rows p1 / p7 / p14 / p17 (92ed9a8) | 0 each | 73 each | 0 each | 0 / 73 (p7's bite, its own kind) / 0 / 0 | as expected |
- **The 6 twins:** "no forged_kind record (a twin forges nothing; its twin check covers every press, the left-out ones
  included)"; each forged run's twin check ("0 effect beyond the valid part") reads 0.
- **Every other kind's left-out count is 0** on every forged run, measured and bites.
- **Not applicable** (no forged_kind record): the 18 clamp, fast-clock and lag-switch runs, the 6 clamp twins, and the
  bites refuse-not-clamp and not-older-than. No forged run lacks its record, so ADV-U4F2H-1's failed-run reading is
  never met.
- Read by a scratch reader (setread.mjs, not committed) from each run's lab.jsonl and both verdict files; all 17
  non-twin rows equal runs.txt's kind 7 lines.

### The advisor's expected readings (advisor-u4f2-head.md point 6.5): each met
- **Every forged measured run:** every kind unrejected 0; refusals dropped 0; twin mismatch 0; kind 7 sent at least
  1,000 (1,505-1,515); `counted_count` = `sent_count` for kinds 8, 9 and 11 (1,523 each); in a building 0 slot-ticks;
  PM-0001-2's band (685-691 on these six).
- **Each bite fails only its own kind's check:** the bitten kind's unrejected count is its whole sent count (kind 10
  1,656; kind 7 1,325 of 1,511, as R5b's short run read 31 of 36), every other kind 0. The twin check reads GAP on
  bite runs because bites run without twins, as the 21 rows' p1, p7, p14 and p17 do. refuse-not-clamp fails "never
  refused or counted" at 18,007; not-older-than fails both fast-clock checks (168 commands over, 33,600,000 µm).
- **Stop conditions: none.** Left out at most 18 against the line of 76; no other kind left out; no forged measured
  run unrejected above 0; no forged run without its record.
- **The other parts** (the same value on all three runs unless a range is shown): clamp A/B 18,297/18,295 points
  beyond reach, never refused or counted 0, twin hash 0; fast clock A/B 10 episodes, commands over ticks plus lead -732/-12,
  movement -146,400,000/-2,400,000 µm; lag switch A/B 101 bursts, cap -200,000..0/-1,397,516 µm. Option B's own
  counters read late 404-410 (lag switch) and buffer full 22 (fast clock); both parts pass.

## Lines of their own (point 6.6, FP-9)
1. **The reading (F-B-COUNT (a)).** His words (history.md:7544-7551): "F-B-COUNT: (a). Leave kind 7's presses out of
   that count when the server's own state shows its soldier dead as that command or the one before it ran; report the
   number left out beside the check on every run." As built: a kind-7 item is left out of `sent_count` and
   `unrejected_count` when the server World shows its forger's soldier dead at the run of its command d or of d - 1;
   an item whose d - 1 never ran is decided by d alone (bots lib.rs:175-182, forged.rs:100-115; ADV-U4F2H-2's wording).
   It narrows ADR 0001:1611's "every forged command" and what SERIES.md:158's `sent_count` counts for kind 7. No pass
   mark, judge rule, marks.json text or existing SERIES.md row changed: one new row, `dead_left_out_count`. Reviewed by
   round 2's netcode and test seats and the advisor's head check; on the readings PR's list. The case "d - 1 never
   ran, d dead: left out" has no test yet: it joins TE-U4F2-R2-3's follow-up.
2. **The departure.** FP-2 also touches bots lib.rs (the pure `kind_7_left_out` and T-K7's lines), one predicate wider
   than the ruling's letter (advisor-u4f2-parta.md:103-104), chosen before the build so one plant fails both tests.
   The advisor: accepted, nothing re-runs (advisor-u4f2-head.md point 4: its one caller outside tests is
   forged.rs:107, on forged runs that are not twins; the lag-switch A proof and R5b's forged lines kept their fate logs).
3. **The commit before M5 ended.** b4e1f43 was committed at 01:03:34 UTC (the Stop hook asked for it), after R6 twice
   and the gate, on R6's bytes. M5 (check.mjs in the main folder on b4e1f43's exact diff, 445b8f9de3e3…1aaf) passed at
   01:22:52: all 11 steps, test 1,070.2 s, 709/806, wasm 0 mismatching. Nothing used b4e1f43 before M5 passed; the
   advisor ruled the order acceptable with this line.
4. **`HOLD_STEPS`:** `CATCHUP_HOLD` was renamed to pass check.mjs:760's name rule (a gate file under KIT-FREEZE-2, so
   the name changed, not the rule). It counts client steps: `npt_proto::Limits::REDUNDANCY_CAP as u32 - 2`, 6 at 30 Hz
   and at 60 Hz, at bots lib.rs:186 (156 at the build). The netcode and rust seats read it as a step count
   (advisor-u4f2-r1.md:238-246).
5. **p12, read against the corrected text (ADV-U4F2H-3).** Its `expect` ("'0 presses lost' fails too") predates U4f-F1.
   Against "'0 presses lost' passes by design after U4f-F1 (advisor-u4f-f1-r1.md:65-73)" it reads as expected: "99.9 %
   reach" fails at 0.894861 and "missing inputs" at 0.105139 (the bite reaches its target); "0 presses lost" passes at
   0; presses 14,398 of 14,400 scripted (window edge, not loss: p5 14,402, p6 14,400). The -20 record cites no
   run-scale proof from this pass. The text fix rides the next run-script commit with ADV-U4F2H-1's.
6. **The five departures as ruled** (advisor-u4f2-r1.md:219-250): f16's shared pairs (with FP-7 naming the culprit);
   the forgers on fixed-lead endpoints; C-U4F2-5 by route (c) (below); kind 7 rebuilt so no-cooldown reaches it (F-D);
   `HOLD_STEPS` (item 4).
7. **C-U4F2-5, point 1, from the diff** (`git diff 25d316e b4e1f43 -- …/bots/src/lib.rs` outside the test module,
   which starts at 25d316e's line 698 and b4e1f43's 1,143: 23 hunks, +486 -41). In the script's arms one line is new,
   `Kind::Steady => [0, 0],` with its three comment lines (bots lib.rs:1096-1099). Fire, aim, presses, MOVES,
   TURN_EVERY and the Brawl, Lane, NoFire, Presses and Clamp arms are as at 25d316e. The lane walk's way moved from
   `Fast::west` to `Behaviour::lane_west`, with the new `lane_walk`: that is FP-4's fix for F-A (the lag client walks
   the lane), not C-U4F2-5's. The rest is forger code. C-U4F2-5's short copy passed (b)'s 1/32 walk that the full test
   failed (u4f2-b7-walk-fix.log:30, :52); the full test reads 0 in all 12 (seed, pattern) pairs (u4f2-b7-stand.log).
8. **C-U4F2-9:** kind 5's press-edge compare runs only once a command has been accepted (room input.rs:288, `if let
   Some(prev) = &self.accepted`; :287 at 1faf894), so a client's first command after joining is not compared. No -21
   forger rejoins (one connect each: connect-order digest e826cbd37d20d3ae on every run) or throws (the scripts press
   only fire, and kind 7 reload). Its closure is C-U6-19, kept in the cut U6 (replan-units.md:48).
9. **The rest of FP-9:** each plant fails by name and was restored by Edit with its sha256 equal (u4f2-build.md:154-186,
   :284-288, :310-311, :319-326, :332-335, :343-345, :357-359, :383-386, :417-418, :521-524, each with the file and
   the failing test's file:line). The forgers stamp at P0's fixed lead of 2 (`lead_now_count` -1 by design) and kind
   10's burst relies on P0's in-order arrival; `--suite forged` refuses another profile or `--lead`. `skip_command`'s
   controller half is untested (no -21 client skips on a controlled endpoint): TE-U4F2-13. F-C's shot margin:
   measured runs 685.77-695.66; bites 673-696 by their summary lines (PM-0001-2 is not judged on bite runs), the
   thinnest bite-no-lead-bound at 673, as in R3.

## Carries, by number (u4f2-brief.md section 3)
- **46's lab half** (-21's twins, f16): met (12 twins in the set, each twin check 0; f16's 24 cases). **86** (the forged
  suite's option-A rows and -21's 5.3 record from U4f2's merge commit): met: the 21 rows at 92ed9a8 read their expect
  (p12 as item 5), and every PM-0001-2 check passes on all 24 measured runs of the set.
- **C-U4F2-1** (the parser half): `--forge-every` below 2, not a number or above 1,000,000 is a usage error; its proof
  f16 kinds 2 and 5 at the judged 12. **C-U4F2-SEC-1:** `refusals_dropped_count` on every record (0 in every run).
  **C-U4F2-2:** `skip_command`'s unit test, `twin_left_out`'s doc and row. **C-U4F2-3:** npt-room's `CommandRefusal`
  with `RefusalRecord { slot, seq, kind }`. **C-U4F2-4:** FP8's line reworded. **ADV-U4FP-3:** the client docs rewrapped.
- **C-U4F2-5:** item 7. **C-U4F2-6:** p19's `expect` on section 5's reading; -20's report names it
  (u4f-report.md:628). **C-U4F2-7:** the clock suite's band "not applied", two cases, T-6's five. **C-U4F2-8:** the
  header's `params.cmd_timing` from the live room, pinned by f6 (plant 6); plan.md §4.1's npt-room row was owed in
  the merge and missed: written now (plan.md:378-381). **C-U4F2-9:** item 8. **C-U4F2-10:** c_u4f2_10, its plant
  ("any kind matches") failing it by name.
- **Record lines:** F3, p20 read its expect at 92ed9a8; that its episode drops nothing late is the advisor's reading
  (F1's every-10th stamp closing the idled bank at once; not verified: per-command stamps are not logged), and f38
  stays W's proof; F5 (a) p4 "5 bursts in 30 s", (b) p12 "89.5 % on U4f's early read", (c) superseded by item 5.
- **RR-CB-2 and RR-B1-7 (1):** left, not taken (the record names neither; the count stood near the stop), still with
  their takers.

## Tests (2.10's minimums)
- **U4f2: +67 measured and +67 bites** (642 to 709, 739 to 806: R6 twice and M5). 2.10's share +51 (f15 6, f16 24,
  f22 2, f23 1, f33 13, T-6 5), 9 extras at the build's end (c_u4f2_1, c_u4f2_2, c_u4f2_5_sim_short, c_u4f2_7 x2,
  f28 B x2, q4, c_u4f2_10; c_u4f2_5_sim_full is ignored and runs in release by name), and the fix pass's 7 (T-K7, fp2,
  fp3, fp5, fp6, the two fp8 cases).
- **With U4f's:** 82 + 67 = 149 measured and 99 + 67 = 166 bites, against 2.10's +129 and +144 (its +124 and +139,
  plus 5 since contract.rs was edited): met. **Negatives:** 22 (f15 6, f22 2, f23 1, f33 13; q4 and c_u4f2_10 beyond)
  + U4f's 46 = 68 against 67: met, by one.

## Reviews and checks
- **Round 1** (netcode, test, rust, security): REQUEST CHANGES (advisor-u4f2-r1.md; FP-1 to FP-9, FP-2 on his
  answer). **Fix pass:** parts A, A2, B and B2 (u4f2-build.md Blocks 9-10). **Round 2** (netcode and test; the rust seat
  dropped by NPT-TRIM's default, named in msg-2026-11-22.md): APPROVE WITH FINDINGS each; TE-U4F2-R2-1 (Medium) fixed
  by FP-10.
- **R6** A and B PASS (all 11 steps; test 1,155.5 s and 1,112.7 s against 1,207; 709/806, 0 failed or skipped; wasm
  27,000 ticks, 0 mismatching). **Merge b4e1f43:** 14 files +2,674 -194, each byte-equal to R6's tree; gate --no-smoke
  PASS; CI green on push and PR (gate 44969638312, 44969642476; toolchain 44969638333, 44969642481); M5 PASS (item 3).
  **The advisor's head check:** APPROVE WITH FINDINGS (ADV-U4F2H-1 to -3, Low).
- No marks.json, judge, marks/README.md or existing SERIES.md row changed, so no reviewer's read of one is owed.
  Security: U4f2 is lab code only; round 1's SA-U4F2-1 to -3 are fixed (FP-3, FP-8); SA-F1-1 stays open until
  C-U6-19, so PR #40 stays needs-mustafa (it is anyway).
- **Size:** about 2,674 added (+2,198 -194 tracked, forged.rs 476 new) against U4F2-SIZE (a)'s about 1,700 to 2,250
  (about 2,700 at 1.2) and his stop past 3,000; the advisor's 2,800 line not reached.

## Follow-ups, by id (none blocks; for STATUS at PR #40's closing docs pass)
- ADV-U4F2H-1 (kind7Line: try/catch per line; a twin only for a `-twin` folder) and ADV-U4F2H-3 (p12's text): the
  next run-script commit. ADV-U4F2H-2's missing case: with TE-U4F2-R2-3.
- NC-U4F2-R2-2's case, NC-U4F2-R2-3, TE-U4F2-R2-2, -3 and -5 (u4f2-build.md:613-618); TE-U4F2-13.
- TE-U4F2-R2-6's record lines (u4f2-build.md:608-612): snapshot sizes print 12 B low; R2's PM-0001-2 NOT-RUN by
  design; option B's re-run proxy covers the measured ticks only; R3's 673 is the summary line's whole number.

## What was not verified
- The extras' `--list` lines were not produced (no debug build for it, the disk near its floor); check.mjs's counted
  test step gives the counts (709/806, three times).
- No death series is logged: the left-out items rest on the World's dead flags as the attribution reads them
  (scenario.rs:1465-1474). R2 showed seed 1's left-out counts equal its pre-FP-2 unrejected counts; seeds 2 and 3 have
  no such earlier run.
- No single judge call prints PASS for PM-0001-21: the mark is read from the two calls, by design.
- The bites' shot figures are the summary lines' whole numbers, not judged.

**Decisions needed:** none.

**For Mustafa:** nothing for U4f2. The re-plan's four answers (RE-PLAN-OK, MARK-NOTES, LAPTOP, PACK-P0) and R1's day
stay asked in the re-plan message.
