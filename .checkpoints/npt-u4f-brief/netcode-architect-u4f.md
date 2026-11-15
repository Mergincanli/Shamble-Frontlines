Task: night item 7 (npt-u4f-brief), 2026-11-16: netcode-architect drafts rev 1 of U4f's build brief (clock and commands) into .checkpoints/npt-u4f-brief/u4f-brief-draft.md; these are its notes.

Authority: the approved plan at 781212774986118183a7c6521c34cca31b5fe2a0:.checkpoints/netcode-proof-test/night-2026-11-16.md ("Rules for the item" 1-7, "Item 7"). Inputs read at 11fd190 in /home/user/Shamble-Frontlines/.gate/npt-spike-1116 (read only). N/ = that worktree's .checkpoints/netcode-proof-test/.

## Plan (blocks, in order)
B1. Read the plan. B2. Scout map. B3. Build plan. B4. Carries. B5. Rulings. B6. Card. B7. ADR, marks.json, README, plan.md, catalogue, scout-netcode. B8. N/u4r-brief.md rev 3.2 (form). B9. Code lane at 11fd190.
B10. Draft sections 0-2 (save). B11. Draft sections 3-5 (save). B12. Draft sections 6-12 + drafting findings (save).
B13. Final counts and DONE.

## Findings (one line each: id, severity, file:line, scenario)
- NA-4 High (estimate): server/src/pump.rs:458-467, lab/src/scenario.rs:459: the room's Pong carries only the next tick (hold about 0), so ClockSync's server stamps are quantised to 33.3 ms; at P2 and 10 pings/s p99 near 6 ms > -47's 5 ms (marks.json:9079-9085), or a filter tuned past it hides the tick-quantised bite; proof: D4's model or the builder's first P2 clock run.
- NA-3 Medium: N/u4-build-plan.md:335's cap formula with EVQ-LOAD's 460-560/s (N/advisor-brawl1-sets-p3-full.md:158) gives at most about 1,008, while 1,024 overflowed in BRAWL-1's P3 window without a burst; a formula cap resyncs clients in judged runs; proof: D5.
- NA-12 Medium: plan :335 and carry 57 (N/u4s-carries.md:223-228) bound the cap from below only; carry 50 (:180-183; room/src/events.rs:19-20) expires a never-acker's events at 256 ticks, so a cap above about 3,900 is never reached and -22's headroom check (marks.json:5145-5155) fails a correct build; proof: arithmetic plus room/tests/room.rs:1302 at the chosen cap.
- NA-5 Medium (estimate): U4f's judged sets about 3,750 MB vs 3,593 MiB above the 4 GiB floor (scout Disk; c-u4r-runs.mjs:153); KEPT-LOGS is set before U4g's build (card :521), not U4f's runs; proof: D2.
- NA-6 Medium: room/src/input.rs:80-89, room.rs:795: a backward sequence is a counted Repeat like honest copies, so -21's kind 3 cannot be rejected and counted apart (marks.json:4932-4943); proof: unit test f12.
- NA-7 Medium: room/src/input.rs:107-116: option B counts a forged old tick as late; no lead bound exists, so 8 far-ahead commands fill the buffer and every later honest command is refused Full; proof: unit test f13.
- NA-8 Medium: bots/src/lib.rs:74-76 (FAR_CM 32,000 cm) vs the +-12,800 cm map (sim/src/greybox.rs:62, :151): with the aim check the clamp points are refused, failing "clamp: never refused or counted" (marks.json:4966-4977); proof: f18.
- NA-9 Medium: marks.json:5086-5088, :5110-5112: no-cooldown's and refuse-not-clamp's sites are npt-sim's (sim/src/world.rs:937-938), outside U4f's row (plan :806-809) and §4.14; without a lane ruling they cannot bite; proof: grep the ids under crates/ (none today).
- NA-1 Low: N/u4s-carries.md:535-537: carry 86 has no quoted text, so a fill script cannot find where its rule begins; proof: the fill's assert.
- NA-2 Low: marks/SERIES.md:93: rss_bytes's writer is npt-server, which runs no process on L1; -22's report line (marks.json:5182-5190) has no L1 writer; proof: grep rss_bytes under crates/.
- NA-10 Low: the night plan's cite of TE-Q2's after-clause (N/advisor-te-m3.md:126-127) is :127-128 at 11fd190; proof: the fill's first-word assert.
- NA-11 Low: N/u4-build-plan.md:810's bite words differ from marks.json's ids (:9097, :9101); a builder spelling the plan's words mints ids no bite entry matches; proof: grep at merge.
None is carried in N/u4s-carries.md.

## Final counts (the draft, 837 lines)
- Sections 0-12 all present, plus the drafting findings table after section 12.
- Field tables in §2: 9 (2.1 ClockSync, 2.2 controller, 2.3 option A/B, 2.4 checks, 2.6 cheats, 2.7 series, 2.9 client carries, 2.12 twins, 2.13 public items); plus 2.10 tests table and 2.11 bites table.
- Tests (deltas): measured +60, bites +74 (14 bites-only unit proofs); 44 negative cases; T-6 +5 in both builds if contract.rs is edited.
- Bites: 17 (14 new unit proofs; 3 existing: no-trailing, no-channel, unbounded).
- Judged-run rows: 8 part rows in 5.1; 36 measured + 12 twins + 17 bites = 65 runs.
- Carries placed: 14 CARRY placeholders (46, 80, 86, 57, 74, 21, 27, 18, 39, 50, 61, 58, 76, 77); 63, 64, 67 conditional (not placed); 62 (h) routed to C62H-KEEP.
- WORDS placeholders: 15 (carries :9-10; b1-p3 :158; m9 :252-256, :259-264; d2-r2 :304-308; df-ruling :462-464, :465-468; j-r2 :298; te-m3 :127-128; history :292-293; c62h :20; r-b2 :28; r-r1 :122; u4r-report :1137-1141, :1162).
- Recheck rows (§8): 29.
- Open items (§9): 15 from the scout: 12 routed (3 of them open with Mustafa: C62H-BOUND, C62H-KEEP, KL-YES), 3 left (check.mjs follow-up, EA-U4R-4 group, RR-5 bench), each with why.
- U4F-Q questions: 12 (Q1 lane, Q2 run budget, Q3 option-A depth, Q4 logs keying, Q5 TE2-5 stop clause, Q6 ClockSync stamp, Q7 controller form and with_lead, Q8 lead bound, Q9 per-kind attribution, Q10 bite options, Q11 suites/profiles/references, Q12 event cap value).
- Needs a run (day): 10 (D1-D10).

## Not done / limits
- No build, test or run (night rules); NA-4 and NA-5 are arithmetic estimates.
- client/src/lib.rs:810-839 (full_state), lab/src/logs.rs:495, lab/src/contract.rs:1-25 and bots/src/lib.rs:404 cited from the scout's map, not read at night.
- D5's exact command not writable: the BRAWL-1 P3 folder name was not read.
- History line ranges taken from the plan and the scout; the decision headings at :1927, :1990, :2039, :2088, :2113 and :291-293 checked.

## Block log
- B1-B9 done. B10 done. B11 done. B12 done (sections 6-12 and the findings table written; draft 837 lines). B13 done.

DONE
