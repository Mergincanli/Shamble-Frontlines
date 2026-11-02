# The final fix round: findings to batch per author (the session's tracking file, 2026-11-02)

Collected from fixcheck-a, -b, -c, the skeptics and the step-10 reviews; one round per author at the end.

## From verifier fixcheck-b (verifier-fixcheck-b-verdicts.md; PASS WITH NOTES, 2026-11-02 about 09:25)
- test-engineer: finding 1 (PM-0002-17, -42: the `enum` bite required again; skeptic-4 first), finding 3 (PM-0002-45
  cannot fail G7 in the foundation; skeptic-4 first), finding 9 (PM-0002-23: "(2 × 1000 / 30 = 66.67 ms)"), finding 12
  (PM-0005-42 "faction definition" vs CD-13's client definition). Findings 2, 4, 6: fixed in round 3 (81d2a65); fixcheck-c
  checks them.
- engine-architect (0005 frame): finding 7 (0005:1759 "Revisit when" lost its verb), finding 11 (0005:5 author line
  reads as if graphics-engineer wrote the pass marks), finding 13 (0005:1693 "until the spikes have measured it" vs
  ADR 0002's "until the Advisor sets the first limit").
- engine-architect (0002): finding 8 (OQ-0002-8's ids line: add R5-XT-OQ6; "no answer on one side" fits R4-XT-OQ5 only).
- rust-reviewer: finding 10 (0004:339–340 "no Rust navigation crate read promises determinism"); finding 5 (0004:805–807,
  "listed once in README": true once the synthesis adds the fragments row; skeptic-4 first).
- The synthesis (sent 09:30, queued): README rows 69–70, 72, 82, 109; OQ-0002-8 row; fragments row; SPIKE_PLAN:30, spike 4
  table (PM-0004-73, -74), :176.
- For the report: R5-TSLINT-F12's construct list (frozen in r5 as "general knowledge, unverified") is now confirmed at the
  source (typescriptlang.org/tsconfig, "Erasable Syntax Only"); the six pending source checks are confirmed; of the five
  claims on unread sources, four confirmed and 0004 row 138 supported (Unsafe Code Guidelines glossary).

## From verifier fixcheck-a (verifier-fixcheck-a-verdicts.md; PASS WITH NOTES, 2026-11-02 about 09:35)
109 rows (0001 39, 0003 43, 0006 27): 104 confirmed, 0 wrong, 4 unsupported (A8; B14, B18, B38), 1 stale (B25), 1 rule
(L3); 114 blocks, 109 resolved, 5 partly; the fix files applied in commit order to 80a27b3 give 57c43c2 byte for byte;
21 citation flags (0001 3, 0006 18), all script artifacts; 3 fetches, all 200.
- netcode-architect: L1 = A8 (0001:230–231 says "the events of ADR 0004 part 19" but leaves out blasts, harvester
  deliveries and kills, structure kills and stage changes; skeptic-4 first). Optional note: 0001:809–811 could cite
  PM-0002-25 as the row reporting the Worker's extra cost.
- graphics-engineer: L2 = B25 (0003:707, GW17 "pending removed", while its camera-player half is now Mustafa's open
  call under OQ-0003-15; the other OQ-0003-15 rows keep a pending note).
- test-engineer: L3 (0006:884 "the proof test's load"): fixed in round 3 (81d2a65); fixcheck-c confirms.
- The synthesis (sent about 09:36, queued): M1 (the fragments row), M2 (README:63 OQ-0003-11's old question; OQ-0006-5
  row; SPIKE_PLAN item 1 "Depends on" OQ-0006-5; SPIKE_PLAN:74 and :97 "PM-0001-1 to -56" → -57).
- Skeptic-4 (added about 09:36): A8; B14/B18/B38.

## From the step-10 review by engine-architect (review-engine-architect.md; 2026-11-02 about 09:45)
Verdicts: ADR 0001 REVISE (small: one high gap); 0003, 0004, 0006, 0002's pass marks, 0005's CD/AP pass-mark rows:
APPROVE WITH NOTES. Findings (its numbering; owners as the review names them):
1. High: no timing rule for events (0001:445, 255–262; 0002:258–262): spatial events play ahead of the delayed poses;
   R2-SYNC-REC6 (r2:2210, blasts "on the timeline of the entity they hit") is marked adopted but not carried. Fix: a
   part 7 rule (the Worker releases spatial events at the delayed time); ADR 0002 part 5 and PM-0002-9 follow.
   Owners: netcode-architect, engine-architect (0002), test-engineer.
2. Medium: the 30 Hz input cost understated (0001:649, 807–808, 1165): the local player is drawn one tick behind its
   prediction; no mark measures input-to-screen. Its Q1 (decision).
3. Medium: the client's prediction step is not defined as a sim crate entry point (0004:151–156, :977). rust-reviewer.
4. Medium: the aim-ray seam is untested (0002:1056, 0001:1170). test-engineer.
5. Medium: no production release model (ADR 0006 with 0001:560): a refused client reloads the same bundle and loops.
   Its Q2 (decision: an open question in 0006 now).
6. Medium: ADR 0005 hands the archive of shipped content sets to ADR 0006, which has none (0005:388–389, 0006:1022).
7. Medium: the 8-rooms-per-VM mark contradicts PM-0001-1 (one tick thread per vCPU) and PM-0006-13 (0001:1118, 1146;
   0006:880). test-engineer.
8. Medium: the client endpoint lives only in `sim-wasm`, while ADR 0001's tests run native headless clients (0002:183,
   its own text). engine-architect (0002).
9. Low: PM-0005-10 tests only the native dev server (0005:2034). 10. Low: no endpoint named for client field data or
   crash reports (0006:321, 0001:497). 11. Lows in ADR 0003 (370–374 audio occlusion, 928–931 bench Worker load,
   276–277 wording, 381 stale hedge). 12. Lows in ADR 0002, its own text (191–192 supervisor crate; 255–262 the block's
   owner section omits the floor layer and area id of 0004:387–388).
Questions: Q1 own-movement delay at 30 Hz (Mustafa by feel after the netcode proof test; add the absolute measurement
now); Q2 the release model (an open question in 0006 now; decider the Advisor).
