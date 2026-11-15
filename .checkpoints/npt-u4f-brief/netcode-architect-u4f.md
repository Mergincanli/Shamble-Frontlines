Task: night item 7 (npt-u4f-brief), 2026-11-16: netcode-architect drafts rev 1 of U4f's build brief (clock and commands) into .checkpoints/npt-u4f-brief/u4f-brief-draft.md; these are its notes.

Authority: the approved plan at 781212774986118183a7c6521c34cca31b5fe2a0:.checkpoints/netcode-proof-test/night-2026-11-16.md ("Rules for the item" 1-7, "Item 7"). Inputs read at 11fd190 in /home/user/Shamble-Frontlines/.gate/npt-spike-1116 (read only). N/ = that worktree's .checkpoints/netcode-proof-test/.

## Plan (blocks, in order)
B1. Read the plan. B2. Scout map. B3. Build plan. B4. Carries. B5. Rulings. B6. Card. B7. ADR, marks.json, README, plan.md, catalogue, scout-netcode. B8. N/u4r-brief.md rev 3.2 (form). B9. Code lane at 11fd190.
B10. Draft sections 0-2 (save). B11. Draft sections 3-5 (save). B12. Draft sections 6-12 + drafting findings (save).
B13. Final counts and DONE.

## Findings (one line each: id, severity, file:line, scenario)
- NA-1 Low: N/u4s-carries.md:535-537 carry 86 has no quoted text (others quote it, :9-10), so the fill script cannot tell label from rule; proof: the fill script asserts one quoted span per entry.
- NA-2 Low: marks/SERIES.md:93 names npt-server as `second.v.rss_bytes`'s writer; on L1 the room log is npt-lab's, so -22's report (marks.json:5182-5190) has no named L1 writer; proof: grep rss_bytes under crates/.
- NA-3 Medium: N/u4-build-plan.md:335's cap formula with EVQ-LOAD's 460-560/s (N/advisor-brawl1-sets-p3-full.md:158) gives about 730-1,008 (margins to 0.7 s), at or below the 1,024 that overflowed in BRAWL-1's P3 window without a burst (queue drains at the full window's pace); a formula cap resyncs clients in judged runs; proof: BRAWL-1 P3 s1 high-water marks against 560 x (1 s + RTT).
- NA-4 High (estimate): crates/server/src/pump.rs:458-467, crates/lab/src/scenario.rs:459: the room's Pong carries only the next tick and hold_us (about 0), so a server-time sample is quantised to 33.3 ms; at P2 (crates/l1/src/lib.rs:185-188) and 10 pings/s (catalogue :91) 20 samples in 2 s give sigma about 2.34 ms, p99 about 6.0 ms > -47's 5 ms (marks.json:9079-9085); ClockSync on the Pong as built fails -47 clean or a filter tuned past it may hide the tick-quantised bite; proof: an arithmetic model or the builder's first D-step.
- NA-5 Medium (estimate): U4f's judged runs at 56 MB each (c-u4r-runs.mjs:153): about 3,750 MB vs 3,593 MiB above the 4 GiB floor at 7,689 MiB free (scout Disk), before builds, r2's extension and the cap measurement; nothing packed or deleted before the KEPT-LOGS kit PR (N/history.md:2088-2111); card :521 puts KEPT-LOGS before U4g's build, not before U4f's runs; proof: df -BM on the day and the runner's projection.
- NA-6 Medium: crates/room/src/input.rs:80-89, room.rs:795: a non-forward sequence is `Repeat` (counted with every honest redundant copy, never refused), so -21's sequence-forward kind (ADR 0001:234; marks.json:5071-5072) cannot be rejected and counted apart; the clean forged runs' unrejected check (marks.json:4932-4943) cannot pass for it as built; proof: unit test honest repeat vs forged old-sequence with other content.
- NA-7 Medium: crates/room/src/input.rs:107-116: under option B an equal/older-tick forgery is `Late` (the honest late counter), and no lead bound exists (8 far-ahead commands fill the buffer, every later honest command `Full`); proof: unit test 8 far-ahead commands then an honest one.
- NA-8 Medium: crates/bots/src/lib.rs:74-76 (FAR_CM 32,000 cm) vs the map's half-width 12,800 cm (crates/sim/src/greybox.rs:62, :151): once U4f adds the aim-inside-the-map check, U4r's clamp script's points are refused and counted, failing "clamp: never refused or counted" (marks.json:4966-4977); the clamp cheat needs points beyond the 6,000 cm reach (crates/sim/src/scene.rs:66) yet inside the map; proof: a unit test of the cheat's points against the room's bounds.
- NA-9 Medium: marks.json:5086-5088 (no-cooldown) and :5110-5112 (refuse-not-clamp): both sites are npt-sim's shot phase (World::BITES, crates/sim/src/world.rs:937-938, holds 4 other ids), outside the files the plan's U4f row names (N/u4-build-plan.md:806-809) and absent from §4.14 (:457-476); without a lane ruling the two bites have no site in U4f's lane (risk 3: a bite that cannot bite); proof: grep both ids under crates/ (none today).
- NA-10 Low: the night plan's "Read first" cites TE-Q2's after-clause at N/advisor-te-m3.md:126-127; at 11fd190 it is :127-128 (:126 is "Keeping the rule costs nothing ..."); a fill by the plan's range takes the wrong first line; proof: the fill script's first/last-word assert.
- NA-11 Low: N/u4-build-plan.md:810 names -47's bites "newest-raw-sample" and "tick-quantised receive times", marks.json's ids are PM-0001-47/newest-sample and PM-0001-47/tick-quantised (:9097, :9101); a builder spelling the plan's words mints ids no bite entry matches (bite NOT-RUN); proof: grep the ids in npt_client::BITES against marks.json at the merge.
- NA-12 Medium: N/u4-build-plan.md:335 and carry 57 (N/u4s-carries.md:223-228) bound the event cap from below only; carry 50 (N/u4s-carries.md:180-183; crates/room/src/events.rs:19-20) drops a never-acker's events at age 256 ticks, so a cap above its rate x 255/30 s (about 3,900 at 460/s) is never reached and -22's headroom min == 0 check (marks.json:5145-5155) fails a correct build; proof: arithmetic plus U4e's clean unbounded test (crates/room/tests/room.rs:1302) at the chosen cap.

## Key facts
(as before, see the draft's sections for the cites; kept here in short)
- Disk: 7,689 MiB free; 4 GiB floor; 56 MB/run; judged run 18,335 ticks, 355.4 s longest; tps SS P0 69.6, P2 51.6, P3 54.7, B1 58.4.
- Profiles: P0 0; P1 50+-10 1 %; P2 100+-20 2 %; P3 150+-30 5 %; P4 200+-40 5 %; B1 = P2 + bursts (crates/l1/src/lib.rs:177-202).
- check.mjs:39 LEAF includes npt-room and npt-client (clippy-wasm32 covers them); :1957 WASM_CRATES.
- history decisions: :280 (Decisions taken), :1927, :1990, :2013, :2039, :2088, :2113; builders' limits :292-293 (USAGE-SAVINGS :291).
- j-r2 B6 :298; df-ruling T4 :461-468; te-m3 after-clause :127-128.
- Existing bites: proto quant.rs:34-42 (no-trailing, no-channel), room lib.rs:38-46 (unbounded), client lib.rs:28-32 (3), sim world.rs:937-938 (4), server schedule.rs:252-257 (4), lab scenario.rs:204.
- Carry 21's room test: crates/room/tests/room.rs:235; unit test room.rs:1402. -22/unbounded tests: tests/room.rs:1302, :1367.
- Map ±12,800 cm (greybox.rs:62, :151); reach 6,000 cm (scene.rs:66).
- No -20/-21/-22/-47 series is written today (grep in crates/lab/src: none).

## Counts planned for the draft
- Judged runs: 36 measured + 12 twins + 17 bites = 65 (-20 6+1, -21 24+12+13, -22 3+1, -47 3+2).
- Tests: about 61 measured, 14 bites-only unit proofs (bites +75), negative cases listed in 2.10; T-6 +5 if contract.rs opens.
- Questions U4F-Q1 to Q12.

## Block log
- B1-B9 done. Next: B10 (draft sections 0-2).
