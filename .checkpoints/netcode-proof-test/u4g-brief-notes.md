# U4g brief drafting: notes (agent, read-only except this file and u4g-brief.md)

## Plan
1. Read history.md:2234-2282 (TEST-FIRST rule 3) and :2048-2051 (SIZE-CHECK).
2. Read u4-build-plan.md §5 U4g (:881-895), §4.8 (Interp), §4.14 (bites), :1117-1119.
3. Read u4f2-brief.md (shape), u4f-brief.md sections 1, 4, 6, 7 (pointers only).
4. Read u4s-carries.md:554-585; history.md:2216, :2477-2478, :6425, :6791-6806; u4f-brief.md:128, :1302, :1333;
   advisor-u4f-stop.md:126-134; u4f-report.md item 5 / "The other figures" / "After the merge".
5. Find marks.json; read the marks -41 -42 -60 -45 -61 -62 -55 -56 -58.
6. Survey spikes/netcode-proof/crates: client, sim predict, room snapshot packing, lab suites/figures.
7. Runs.txt under spikes/netcode-proof/results/u4f/ for per-run size and time.
8. Write the brief (~200-300 lines), then the final reply.

## Log (one line per source read / finding)
- u4g-brief-carry.md exists (14 lines): BRAWL-1 B1-burst words from advisor-brawl1-r1-full.md:279-288 to go into U4g's brief (door-gap reading in -17 before judged B1 runs). Carry it by pointer.
- (restart 04:22; lines below re-logged from the read before it)
- history.md:2236-2282 read: TEST-FIRST rule 3 "A brief says what is built, how it is tested and what is still open. It points ... does not repeat them." Rule 2: pass mark/judge/marks.json/money/main wait for him.
- history.md:2048-2051 (SIZE-CHECK, his item 2 of 2026-11-14): M = about 3,000; "well past" = more than twice; stop and tell him in three lines.
- u4-build-plan.md read: U4g :881-895; §4.7 events :335-367 (cap 1,600; margin under 1.5 named for U4g :362-364); §4.8 :369-372 (Interp 7 slots at 30 Hz; integer ns API); §4.14 bites :484-508 (-41 x2, -45, -58 x6 incl. lt-tick-offset's U4g half, -62 x2); §4.13 scenes :458-483; §6 :954-970 (-42/smoothing-off carried, R7); §7.1 :973-985; §8 risk 9 :1011-1013; §9 run count :1025-1029 (-41 30, -42 18, -60 3, -45 9, -61 3, -62 3, -55 3, -58 6); Q23/Q24 :1124-1134; Lanes :949-950 (U4f, U4f2, U4g, U4h in order).
- marks.json (spikes/netcode-proof/marks/marks.json, 12,630 lines): -41 :7719 (5 L1 CO parts min 3, 2 checks on tick.v.untraced_corrections_count, 2 bites); -42 :7863 (6 L1 SS brawl parts P2-P4 x A/B min 3; L1 check only P3 share<0.01 of second.s.snap_corr_over_5cm; P2/P4 report only; bite smoothing-off); -60 :8071 (suite own-change L1 P2 min 3; 4 checks event.own_change.*; bite own-state-missing); -45 :8598 (suite extrapolation L1 P2/P3/B1 min 3; tick.v.block_extrapolated, block_extrap_over_interval; bite buffer-one-short); -61 :8936 (suite events L1 P2 min 3; tick.v.remote_event_misplaced_count, own_event_misplaced_count; bite on-arrival); -62 :10467 (suite cues L1 P2 min 3; event.own_cue.plays_count x3, second.v.own_cosmetic_sent_count; bites cues-from-replay, server-echo); -55 :10694 (suite beside-vehicle L1 P2 min 3; second.s.snap_corr_over_5cm share<0.02; bite vehicle-snap; readings R-11, R-12); -56 :10745 (suite record, item (6)); -58 :4148 (suite wrap L1 P0/P3 min 3; 10 checks incl. untraced corrections; 6 bites; reading R-5).
- u4f2-brief.md read (257 lines): shape = header, 0 stamps/base, 1 blocks, 2 tests, 3 carries, 4 rules, 6 size table, 7 reviewers, 8 disk/time, "What goes to Mustafa".
