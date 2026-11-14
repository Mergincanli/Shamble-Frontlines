# U4r build 3: facts for the advisor (fact-checker, read-only, 2026-11-14)

## Plan
1. Read questions (done), builder notes :571-906, brief, carries, rulings (for the words cited).
2. Point 1: b1 verdict + logs (everyone_in_view per window, burst_eviction, datagram sizes); marks.json PM-0001-2, -17 B1, -18 B1; history.md floor.
3. Point 2: es verdict; marks.json band; hit share from logs; mirror.rs view; brawl_aim.
4. Point 3: tick16c verdict; where everyone_in_view goes to 0; carry 73 words.
5. Point 4: no-fallback heal records vs clean b1; brief 2.4 r2 words.
6. Point 5: r6 depth readings at the recording's end.
7. Point 6: contract.rs, notes :590-605, brief 2.7, ruling 2 point 5.
8. Point 7: after_capped in scenario.rs and logs; mirror tick lookup; clean P0 re-run equality.
9. Point 8: SERIES.md rows, point 33 labels, per-profile bytes.

## Items (a line per item; done marked)
- [done] P1 verdict: r2/b1-verdict.json -2 FAIL 0.273333 (SS runs part); -17 L1 B1 INVALID, -18 L1 B1 INVALID by pre:PM-0001-2; -57 checks pass (6974/2750/88060), overall NOT-RUN 1 of 3. Held.
- [done] P1 figure: judge/src/figures.rs:24-54 least per-slot share of measured ticks; b1 room.jsonl: least slot 246 of 900 = 0.273333 (max slot 569). Held.
- [done] P1 windows from 64186: notes' 47,0,0,0,0,0,4,12,23,31,40,44,45,46 = per-window MEAN of slots holding, truncated (means 47.9,0.5,0,0,0,0.6,4.1,12.4,23.1,31.9,40.1,44.2,45.8,46.3). Held with that reading (unstated in notes).
- [done] P1 burst_eviction: 96 events, all evicted 1 baseline_age 0, ticks 64217 (burst 0) and 64667 (burst 1), 48 each. Held.
- [done] P1 datagrams: 48 per tick, each 996-1000 B, ~47.9 KB/tick before (mean 47,954), during (47,966), after (47,953 / 47,939). Held. Cap Limits::SNAPSHOT 1,000 (proto/src/limits.rs:25).
- [done] P1 class_audit breaches_count 0 (lab.jsonl:1315). Held. Heals: 96, first ~48 ms after link return.
- [done] P1 history.md: Mustafa's only PM-0001-2 word = "The 9.5 floor (PM-0001-2) does not move" (history.md:338-339, 2026-11-11 ~11:50 UTC) = ES reliable events; 0.99 everyone-in-view is "proposed share" in ADR 0001 row :1589. "holds his floor" true of the mark, but not of the failing check.
- [done] P1 per-burst cost: burst 0 window (64187-64666, 480 ticks) zeros per slot min 145, mean 237.1, max 397; recovery to first 1: 119-320 ticks after eviction; before burst 0 all 48 hold on 151/151 ticks. 10 bursts in 18,000 measured ticks -> least share <= 1 - 1450/18000 = 0.919; even 1 burst: worst slot 397 > 180 (1 %).
- [done] P1 code: ack.rs:66-69,180-205 ring 32 ticks, age 32 evicted -> defaults (room.rs:1160-1170); B1 = P2 + 1 s total loss (l1/src/lib.rs:197-201).
- [done] P2 es-verdict.json: -2 FAIL, ES part: 12 checks, 11 pass, reliable <=10.5 fails at 50.05 (>=9.5 passes); shots 72, projectiles 32.383333, spawns 21.1, everyone 1. Held.
- [done] P2 logs: measured 20 s: reliable 1001, shots 1440 -> 50.05/s, 72/s; reliable = Hit (2) + Kill (3) (logs.rs:115-117, world.rs:2005-2011); kills <= 14 over run (1426/101) -> hit share 68.5-69.5 %. "about 70 %" held.
- [done] P2 code: bots/src/lib.rs:226-245 brawl_aim = target's position in view (+100 cm); view = mirror.view of client's newest tick (scenario.rs:959-960; mirror.rs:212-230). Band 9.5-10.5 marks.json PM-0001-2 checks. Carry 59 (u4s-carries.md:274-283) words checked.
- [done] P3 verdicts: tick16c-verdict.json -17 INVALID, bite tick16 status invalid, invalid_why only pre:PM-0001-2 = 0; tick16b same; tick16 (first try) invalid by 3 missing footers AND pre:PM-0001-2 = 0 (notes :712 name only the cut logs).
- [done] P3 where 0: tick16c room.jsonl flags: 48 to 65566; 0 65567-65604; 48 65605-65636; 0 65637-65682; 26-31 65683-65714; 0 from 65715 to 66555 (end). Notes' "from tick 65,746" NOT HELD: 65,715 (same in tick16, tick16b). Measured 65806-66405 all 0 (held).
- [done] P3 judge: eval.rs:1408-1409 bite run takes validity of matched parts -> L1 P0 pre:PM-0001-2; carry 73 words (u4s-carries.md:328-337) held; brief :1161 judged tick16 crosses wrap 1,800 ticks in (held).
- [done] P3 tick16b vs c summaries (c3-r2-tick16b.log:46, -c.log:19): equal except projectiles per set 12.65 -> 12.33 and audit counts (434460/187272/31646/5828 -> 459038/264878/33677/0).
- [done] P7 clean re-run: c3-r2-p0.log:20 vs c3-r2-tick16c.log:50 summary text equal after run id; p0-s1 vs p0-s1b logs equal record for record (header run/build blanked) except lab run_lengths gains after_capped false. Span 63736-64635: does not cross the wrap.
- [done] P7 (a) code: scenario.rs:1034-1049 cap -> error if s.bites.id is none, else after_capped; run_lengths.after_capped scenario.rs:1102; summary text :1468-1472; default after-max-s 30 (scenario.rs:1318). carry 62 (e)/(c) words u4s-carries.md:289-312.
