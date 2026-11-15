# Advisor: BRAWL-1 review round 1 ruling — notes (2026-11-15)

## Plan
1. Read advisor.md (main), the round file, plan review (conditions 1-18), points ruling, U4r r1 ruling form. DONE
2. Read the four seats' notes, build notes/report, packet sections as needed.
3. Check each finding against the read copy (.claude/worktrees/brawl1): file:line.
4. Run (through u4r-run.sh only, logs in .gate/u4r/brawl1-advisor-r1/) only if a claim needs it (e.g. TE-B1-1's bite claim; TE-B1-3's walk phase).
5. Rule: fix pass in order, pins, round 2, merge step (disk order), U4g brief words, size, For Mustafa, each finding.

## Findings log (one line each as checked)
- Read copy: HEAD 352a245, 10 files M, `git diff HEAD | sha256sum` = ee2973aa... = wip/brawl1.patch. numstat 875+/123- = 998.
- TE-B1-3 CONFIRMED by my own splitmix64: slot 5 seed 1 phase 7; walk k4-7 = [-127,0] = steer at (1450,0); blocked asserts at k4-6. toward_centre(1450,200) = [-125,-17] (no MOVES entry). 1401 -> [-127,0]. None/dead state and R_OUT boundary untested: confirmed.
- TE-B1-2 / RR-B1-1 CONFIRMED: logs.rs:584-599 untested; summary_line scenario.rs:1612-1690 untested. Note: npt-log Writer::tick refuses a tick <= last (log/src/lib.rs:467-474), so TE's "same tick, measured then warm-up" would fail: the test needs two room ticks. Logs::open(None,..) works (sinks); a folder + finish needed to read room.jsonl (scenario tests::scratch, tests::read). Summary derives Default; tests::settings gives Settings. Room::script + Room::tick (room.rs:617, :805) apply teleports; World teleport only for alive soldiers (world.rs:1147-1151); soldiers start alive (world.rs:745). area_of: x0 <= x < x1 (greybox.rs:176-183): (2200,0) -> area 1.
- TE-B1-1 CONFIRMED as a gap (no leash-off run in any log). sim_run's `now.len() == n` assert sound: write_records pushes one record per soldier slot always (world.rs:1816-1818).
- Dead-bit cluster (TE-B1-6, NA-B1-3, RR-B1-3, EA-B1-1) CONFIRMED: client lib.rs:247-248 literals; bots test :479 literal; world.rs:92/:120 private; death sets health 0 + flags = FLAG_DEAD (world.rs:1630-1634), revive sets flags 0 + health full (:1176-1188): `dead == (health == 0)` is an exact tie.
- Client wrap cluster (TE-B1-4 (b), NA-B1-1, RR-B1-5, EA-B1-3 (b)) CONFIRMED: ticks 100-106 only. One wrap case (u32::MAX in slot 31, 0 in slot 0) kills both the plain-max mutant AND the ring-order (last-in-iteration) mutant, so TE's 127/128 case is not needed.
- TE-B1-4 (a) CONFIRMED: sentinel() sets only filed (client lib.rs:626-637); a complete-then-sentinel tick is never built in the test.
- EA-B1-3 (a) CONFIRMED: slot_for resets on a newer tick (client lib.rs:564-574).
- TE-B1-5 PARTLY REFUTED: its "obstacle at 18.3 m / 14 m vehicle" reading is wrong: sim_run uses StandIns::OFF (bots lib.rs:496); no solid within 1.6 m of the three maxima (building 0 SW corner (1770,-430) is 166 cm from (1734,-592); crates <= 950 cm; wall at y 1180-1220). Maxima all at a burst's end (k 12030 = burst 6 end; k 15630 = burst 8 end). Its kinematic bound (~2,000 cm) and conclusion (B1 can enter a door) hold.
- NA-B1-2 CONFIRMED in substance (Skips s2 1,720; Bursts 1,832 x3: final-regen-tests.log:84-91).
- Reviewers' reruns: TE t2-full-release.log and NA run1.log brawl1 prints identical to final-regen-tests.log (my diff).
- RR-B1-2 CONFIRMED unreachable (16-bit wire); ruling: doc route (domain stated), no code change.
- RR-B1-4 CONFIRMED: run's u32 cap (scenario.rs:1435) before the burst 64-bit check (:1507-1515); fp3 :2576 comment wrong (u32::MAX odd, never a multiple of 30); burst bound's accepted side untested. RR's case arithmetic checked: (143,165,566 x 30) + 300 = 4,294,967,280 ok; next second 4,294,967,310 refused. burst_starts has no in-run check, so the accepted case is ok.
- RR-B1-6 / EA-B1-5: already ruled (points ruling, Point 1, ADV-BP-4, ADV-PT-L3). No action.
- RR-B1-7 CONFIRMED (cast and doc). (1) is polish; deferred to keep size and keep production logs code out of the fix pass; (2) doc rides.
- EA-B1-2 CONFIRMED: only caller scenario.rs:390 (grep crates/ and check.mjs).
- EA-B1-4 CONFIRMED: logs.rs:6-7 vs :597-598.
- TE-B1-7 CONFIRMED: u4r-run.sh:66 / main-run.sh:41 log HEAD only. Fix by the main session (FP-0 form).
- TE-B1-8 CONFIRMED (no glue test); recorded, the 10-minute runs are the glue's proof.
- Disk now: free 6,566 MiB; shared target 6,442 MiB (debug 6,008 of which npt-* deps 1,026 and incremental 3,171; release 250; doc 15; wasm32 172). A main-folder rebuild adds new workspace artifacts + incremental dirs (~4 GiB est.) -> would cross the 4 GiB floor mid-run. Clean before the counted run: yes, but only the part-cleans (B)+(C); the folder-level `cargo clean --target-dir .gate/shared-target` is NOT approved (W1-TARGET item 3 as amended, advisor-u4d2-m9.md:252-257).
- Pins: no ruled fix touches library behaviour on the recording path (bots/client lib: docs only; logs.rs room_tick not run on --logs 0 and not changed now; scenario.rs: visibility, comments, tests; hash.rs: test).

## Ruling skeleton (decided)
- Fix pass: FP-0 runner header (main session); FP-1 bots tests; FP-2 logs.rs positive control (two room ticks); FP-3 client cases; FP-4 docs; FP-5 small scenario/hash items; FP-6 plants (leash-off with conditional door-entry distance bar; figure; two client); FP-7 end checks; FP-8 report words.
- Round 2: test-engineer + rust-reviewer, one batch, xhigh. netcode + engine sit out.
- Merge: M1 bytes; M2 (B)+(C) part-cleans right before; M3 counted cold run (+ full test by name); M4 gate; M5 CI; M6 my review; M7 (B)+(C) again before the 10-minute runs.
- Size: about 1,270 (1,200-1,340), under 1,400.
- For Mustafa: nothing (KEPT-LOGS unchanged).

## Checks behind the exact words (done)
- Unit-test insertions re-sequenced by my splitmix64: k12 (1350 after None) walk [-89,-89] != steer; k15 (1400) walk [-89,-89]; k16 (1401) walk [-49,-117] != steer; k17 diagonal still steers [-89,89].
- Client sentinel case: a second, different group for tick 105 -> refused, sentinel(105), out.hashes [(105,0)], own_soldier falls back to tick 101 (`first`).
- Lab control: Server::build (scenario.rs:373) pub(crate), Server.room pub(crate) (:347), Bites and Summary derive Default (:173, :739), Logs::open(Some) needs the folder to exist (logs.rs:77-88, File::create_new), `--logs` default 1 (scenario.rs:1463-1467), Ended::{Clean,Crash,Stopped} (log lib.rs:256-260), Room::last_tick = next_tick - 1 (room.rs:573-575).
- No cargo run by me: nothing needed a run that the reviewers had not already reproduced; plants need edits I may not make.

## Status: ruling returned by StructuredOutput (2026-11-15).
