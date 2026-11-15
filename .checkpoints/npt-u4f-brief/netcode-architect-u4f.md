Task: night item 7 (npt-u4f-brief), 2026-11-16: netcode-architect drafts rev 1 of U4f's build brief (clock and commands) into .checkpoints/npt-u4f-brief/u4f-brief-draft.md; these are its notes.

Authority: the approved plan at 781212774986118183a7c6521c34cca31b5fe2a0:.checkpoints/netcode-proof-test/night-2026-11-16.md ("Rules for the item" 1-7, "Item 7"). Inputs read at 11fd190 in /home/user/Shamble-Frontlines/.gate/npt-spike-1116 (read only). N/ = that worktree's .checkpoints/netcode-proof-test/.

## Plan (blocks, in order)
B1. Read the plan. B2. Scout map. B3. Build plan. B4. Carries. B5. Rulings. B6. Card. B7. ADR, marks.json, README, plan.md, catalogue, scout-netcode. B8. N/u4r-brief.md rev 3.2 (form). B9. Code lane at 11fd190.
B10. Draft sections 0-2 (save). B11. Draft sections 3-5 (save, with 2.10 count fixes). B12. Draft sections 6-12 + drafting findings (save).
B13. Final counts and DONE.

## Findings (one line each: id, severity, file:line, scenario)
- NA-1 Low: N/u4s-carries.md:535-537 carry 86 has no quoted text (others quote it, :9-10), so the fill script cannot tell label from rule; proof: the fill script asserts one quoted span per entry.
- NA-2 Low: marks/SERIES.md:93 names npt-server as `second.v.rss_bytes`'s writer; on L1 the room log is npt-lab's, so -22's report (marks.json:5182-5190) has no named L1 writer; proof: grep rss_bytes under crates/.
- NA-3 Medium: N/u4-build-plan.md:335's cap formula with EVQ-LOAD's 460-560/s gives about 730-1,008 (margins to 0.7 s), at or below the 1,024 that overflowed in BRAWL-1's P3 window without a burst; proof: BRAWL-1 P3 s1 high-water marks.
- NA-4 High (estimate): server/src/pump.rs:458-467, lab/src/scenario.rs:459: the room's Pong carries only the next tick (hold about 0), so server-time samples are quantised to 33.3 ms; p99 about 6.0 ms at P2 and 10 pings/s > -47's 5 ms; proof: arithmetic model or the builder's first D-step.
- NA-5 Medium (estimate): U4f's judged runs about 3,750 MB vs 3,593 MiB above the floor; KEPT-LOGS (card :521) is set before U4g's build, not U4f's runs; proof: df -BM and the runner's projection.
- NA-6 Medium: room/src/input.rs:80-89, room.rs:795: a non-forward sequence is a counted `Repeat`, so -21's sequence-forward kind cannot be rejected and counted apart from honest copies; proof: unit test.
- NA-7 Medium: room/src/input.rs:107-116: option B counts an equal/older forged tick as `late`, and there is no lead bound (8 far-ahead commands fill the buffer); proof: unit test.
- NA-8 Medium: bots/src/lib.rs:74-76 FAR_CM 32,000 cm outside the map (sim/src/greybox.rs:62, :151, ±12,800 cm): with the aim-bounds check the clamp points are refused, failing "clamp: never refused or counted" (marks.json:4966-4977); proof: unit test of the cheat's points.
- NA-9 Medium: marks.json:5086-5088, :5110-5112: no-cooldown's and refuse-not-clamp's sites are npt-sim's (World::BITES sim/src/world.rs:937-938), outside U4f's row (plan :806-809) and §4.14; no site without a lane ruling; proof: grep the ids.
- NA-10 Low: the night plan cites TE-Q2's after-clause at N/advisor-te-m3.md:126-127; at 11fd190 it is :127-128; proof: the fill's first/last-word assert.
- NA-11 Low: plan :810's bite words vs marks.json's ids PM-0001-47/newest-sample and /tick-quantised (:9097, :9101); proof: grep at merge.
- NA-12 Medium: plan :335 and carry 57 bound the cap from below only; carry 50 (carries :180-183; room/src/events.rs:19-20) expires a never-acker's events at 256 ticks, so a cap above rate x 8.5 s (about 3,900 at 460/s) is never reached and -22's headroom check (marks.json:5145-5155) fails a correct build; proof: arithmetic plus U4e's test room/tests/room.rs:1302 at the chosen cap.

## Counts in the draft (kept current)
- 2.10: measured +60, bites +74 (14 bites-only unit proofs), negative cases 44 (rows marked neg), T-6 +5 conditional.
- Judged runs: 36 measured + 12 twins + 17 bites = 65.
- Carries placed: 14 (46, 80, 86, 57, 74, 21, 27, 18, 39, 50, 61, 58, 76, 77); 63, 64, 67 conditional.
- Questions U4F-Q1 to Q12.

## Block log
- B1-B9 done. B10 done (sections 0-2 written). Next: B11 (sections 3-5, rewriting the file whole with 2.10's count fixes: measured +60, bites +74, negatives 44; f32 renamed f30; cross-refs 2.11/2.12).
