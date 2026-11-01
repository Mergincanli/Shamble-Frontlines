Task: engine-architect synthesis of the v2 decision drafts (plan step 6): ENGINE_OVERVIEW, SPIKE_PLAN, adr/README, GLOSSARY §8, claims ledger. Started 2026-11-01 09:01 Amsterdam.

## Progress
- started 09:01 Amsterdam; no prior notes. Reading inputs.
- (step) inputs read (brief, plan rules/step 6, advisor conditions, all six spike-entry parts, map parts d-j, OQ list extracted to scratchpad oqs.tsv). Writing SPIKE_PLAN next.
- (step) SPIKE_PLAN written (engine-architect-spikeplan-draft.md, 10 spikes + foundation + asset research). Next: overview.
- (step) ENGINE_OVERVIEW written (133 lines, Mermaid diagram + words, §6 table). Next: README.
- (step) README: drafts, decisions (72 rows), design questions (38) written. Next: loose ends, needs outside §6, codes.
- (step) README written whole. Next: glossary8, claims ledger, consistency notes.
- (step) glossary8 written (placement correction: append after line 743, the unterminated last line). Next: claims ledger + consistency list.
- (step) claims ledger written (engine-architect-synthesis-claims.md). All five outputs done.

## Outputs
- engine-architect-spikeplan-draft.md (→ docs/SPIKE_PLAN.md), 385 lines: rules, shared fixtures, Mermaid order + words, summary table, 10 spikes + the foundation's checks + the asset-pipeline research, later milestones.
- engine-architect-overview-draft.md (→ docs/ENGINE_OVERVIEW.md), 133 lines with Mermaid diagram.
- engine-architect-adr-readme-draft.md (→ docs/adr/README.md), 337 lines; "Design questions" has exactly 38 id rows.
- engine-architect-glossary8.md: J1 text verbatim (map lines 1066–1118) under a corrections header.
- engine-architect-synthesis-claims.md: the ledger.

## For the consistency pass (disagreements between drafts; not resolved here)
1. 60 Hz marks at two precisions: PM-0001-39/-40 (0001:1134–1135) "sim step and server-only p99 at most 4.2 ms", "client share 6.7 ms", "max 16.7 ms"; PM-0004-71 (0004:1014) "≤ 4.17 ms", "≤ 6.67 ms", "≤ 16.67 ms". Same marks; one form should hold.
2. R1-TICK-REC4: ADR 0001's spike section (0001:915–994) never names it; ADR 0004's PM-0004-67 (0004:1008) places S17 in the netcode proof test. SPIKE_PLAN item 1 lists it.
3. R2-SYNC-REC6 mark 1 and R5-LAYOUT-REC4: map part g puts them in the sim core spike; no draft's pass-mark table names them (0004:822–843; 0001 and 0002 adopt the recommendations). SPIKE_PLAN item 4 places them.
4. R1-BND-REC6: map part g lists it under the netcode proof test; ADR 0002 spike item 6 (0002:913) holds it in the boundary spike. SPIKE_PLAN follows ADR 0002.
5. R3-MEASURE-REC1/REC4: map part g in the look proof test; ADR 0003 (0003:838–883) in the bench. SPIKE_PLAN follows ADR 0003.
6. The look proof test's inputs: ADR 0005's share needs the content crate's whole-set check and the ported kit builders (passmarks-0005; 0005:1498, :1901); ADR 0003's asset list (part 11; 0003:806–837) names only authored test assets.
7. Safari version: ADR 0001's key (0001:1017–1019) names Safari 27 (R2-TRN-REC1 mark 1) and "stable on the day"; ADR 0003 (8 hits), 0004 (2) and 0005 (4) say "Safari 26".
8. Upgrade questions twice: OQ-0004-5 (drops and pickups, 0004:773) and OQ-0004-6 (bots' player records, 0004:784) overlap OQ-0005-AB9 (0005:1188) and AB10 (0005:1197).
9. Development reload: ADR 0005 CD-7 / OQ-0005-CD4 (next match, proposed) vs ADR 0001's `ContentReload` message (0001:488) and ADR 0004 part 16 "swaps them between ticks" (0004:413), which hold only under the mid-match option (checkpoint open finding).
10. One Wasm instance or two: OQ-0002-3 (0002:822) and OQ-0004-7 (0004:792) are both decided by the boundary spike; the drafts should cite one another (they do partly).
11. Code collisions: ADR 0002 numbers its gate checks G1–G13 (0002:433 on) while ADR 0003 and ADR 0005 cite the map's research gaps as "gap G-n"; ADR 0004 numbers its spike claims S1–S20 while ADR 0004 (0004 "seam S1") and ADR 0006 cite the map's seams S-n. README defines the map's codes with "gap"/"seam" in front; a reader of ADR 0002 or 0004 may confuse them.
12. Map vs drafts on who decides: R3-BUDGET-OQ2, OQ3, OQ6 (map part g: spikes) are open questions OQ-0003-10, -12, -4 in ADR 0003 (checkpoint open finding); R1-TICK-OQ5 (map part g: the netcode proof test) is the sim core spike's in ADR 0004 (0004:1148). SPIKE_PLAN follows the drafts.
13. Audio voices: map N48 says 32 on both tiers; ADR 0003 follows r3:1246 (32 Low, 48 High) (checkpoint open finding). README's N48 row follows ADR 0003.
14. GLOSSARY §8's 60 Hz margin entry: its "Depends on" states one reading, while GLOSSARY:30–31 asks every reading to be stated; OQ-0001-1 lists option (b). Not changed (condition 3's text).
15. TICK-30's replaced lines: ADR 0001 replaces four lines (checkpoint open finding), condition 2 lists three.

## Could not place
- Nothing unplaced among the spikes. The "Decider" of OQ-0002-7 and OQ-0003-14 is a reading by an agent (no row in "Decisions the drafts ask for").
- The legacy rows ED5 and ED7 (Character Lab, Anim Lab): listed by no draft except as OQ-0005-4 (in README's decisions table).

- Note: the step lines above carried guessed clock times; corrected to (step). All five outputs were done by 09:14 Amsterdam (date check); verification pass follows.
- (step) verification pass: Mermaid edges normalized to `-->|"label"|`; overview §6 refs fixed (0003 parts 10 and 5.i; CD-8; 0004's §6.9 share); README need lines re-pointed to their parts, CD-4 dropped; design section holds exactly the 38 DESIGN ids (diff with the id table: same); every table's columns consistent (script); ledger records the corrections. Finished about 09:25 Amsterdam.

DONE
