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
