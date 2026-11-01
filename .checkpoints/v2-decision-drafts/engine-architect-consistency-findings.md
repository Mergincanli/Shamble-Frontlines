# Consistency findings on the v2 decision drafts (plan step 8, engine-architect as consistency critic, 2026-11-01)

Ranks: H (a reader gets a different decision, number or interface depending on which document they read; must change
before review), M (a seam, owner or term disagrees but the decision itself is the same; change in the fix round),
L (form: precision, citation form, code collisions). Line numbers are against the files as of 14:40 Amsterdam (ADR 0006
after its fix round, 09803a9). The ranked index and the known items' outcomes follow the findings.
Each finding: documents and lines; the texts that disagree; the proposed consistent outcome; the owner who changes it.

## Findings

### F1 (H). Development reload: ADR 0004 states the mid-match swap as decided; ADR 0005 proposes the next match
- Documents: docs/adr/0004-simulation-core.md:412–413 (part 6/16 text); docs/adr/0005-content-abilities-assets.md:255,
  364–370, 525–526, 606–613 (CD-7, OQ-0005-CD4), 1888–1889; docs/adr/0001-multiplayer-netcode.md:488 (`ContentReload`);
  docs/SPIKE_PLAN.md:158 ("hot reload into the Worker").
- Texts: 0004: "a development reload swaps them between ticks, never inside one (R1-DATA-REC5, ADR 0005's)". 0005 CD-7:
  "simulation numbers apply from the next match (proposed; Mustafa and the Advisor decide, OQ-0005-CD4)"; the swap is
  option (b). 0001: `ContentReload` "only if ADR 0005 keeps development reload over the game connection (R1-DATA-REC5)".
- Outcome: ADR 0005 owns reload (seam S5); the open question OQ-0005-CD4 is the one place it is decided. 0004 part 16
  becomes: "Definitions are read by id (I-1). In development they change at a match boundary under ADR 0005's CD-7
  (proposed); if OQ-0005-CD4 chooses (b), a reload swaps them between ticks, never inside one (R1-DATA-REC5)." 0001's
  row keeps the message but its condition reads "only if OQ-0005-CD4 chooses option (b) (mid-match swap)", since under
  (a) nothing in a running match changes and no message is sent. SPIKE_PLAN:158 says "content files reaching the Worker
  in development (PM-0002-31, R1-DATA-OQ8)" instead of "hot reload into the Worker" under parts 2, 4 and 7 (ADR 0002 has
  no reload text in parts 2, 4, 7; its only reload mark is PM-0002-31, line 1034).
- Owners: rust-reviewer (0004:412–413); netcode-architect (0001:488's condition); engine-architect synthesis (SPIKE_PLAN:158).

### F2 (M). Drops, pickups and bot records asked twice, with different options and deciders
- Documents: docs/adr/0004-simulation-core.md:773–791 (OQ-0004-5, OQ-0004-6), 443, 1039, 1068; docs/adr/0005-content-
  abilities-assets.md:1188–1205 (OQ-0005-AB9, AB10); docs/adr/README.md:69–70 and 82–83 (four rows for two questions).
- Texts: OQ-0004-5 "who may pick it up (the dead soldier's enemies only, or anyone); how two claims in one tick are
  broken", Decider "Mustafa with the game-designer"; OQ-0005-AB9 "whether only the killer's team, or anyone, may take a
  drop", "whether a gun a player swaps out becomes a pickup", Decider "Mustafa". OQ-0004-6 options are "whether bots
  level ... whether a human ... gets that slot's record or a fresh one"; OQ-0005-AB10 has (a)/(b)/(c) with (c) "bots
  have no tracks". In three-team modes "the dead soldier's enemies" and "the killer's team" are different sets.
- Outcome: one question each, in ADR 0005 (UPGRADE-SCOPE's data model is 0005's, seam S5; I-21). AB9 absorbs 0004's two
  extra options (the tie rule in one tick; "enemies of the dead soldier" as a third pick-up option beside "the killer's
  team" and "anyone") and takes one decider line: "Mustafa, with the game-designer". AB10 is unchanged. ADR 0004 deletes
  OQ-0004-5 and OQ-0004-6 and cites "OQ-0005-AB9, AB10" at 443, 1039 and 1068 (the tie rule's mechanism, a fixed
  deterministic order, stays 0004's part 18). README drops rows 69–70.
- Also: 0004 part 18 (docs/adr/0004-simulation-core.md:435–436) states a rule the same paragraph then calls open: "When a
  soldier dies, its weapon becomes a pickup entity" pre-empts AB9's options (a)/(b)/(c) and "any death" vs "a death to
  an enemy". Proposed: "When a soldier dies, the weapons the drop rule names (OQ-0005-AB9) become pickup entities".
  ADR 0001 part 12 (docs/adr/0001-multiplayer-netcode.md:386–387) lists the open rules as "carry slots, ammo for looted
  weapons and how fast tracks level"; it should add "the drop rules and bots' records (OQ-0005-AB9, AB10)".
- Owners: rust-reviewer (0004); engine-architect and game-designer (0005 AB9); engine-architect synthesis (README). netcode-architect (0001:386–387).

### F3 (H). The same tick-budget marks at two precisions: a run between them passes one draft and fails the other
- Documents: docs/adr/0001-multiplayer-netcode.md:125–126 (part 2, "4.2 ms each"), :1134 (PM-0001-39: "sim step time p99
  and server-only work p99 at most 4.2 ms each; max under 16.7 ms"), :1135 (PM-0001-40: "at most 4.2 ms", "max at most
  16.7 ms", "p99 at most 6.7 ms"); docs/adr/0004-simulation-core.md:823 (claim S1: "≤ 8.3 ms at 30 Hz (≤ 4.2 ms for the
  60 Hz margin run)"), :824 (S2: "≤ 13.3 ms"), :942 (PM-0004-3: "≤ 8.33 ms (25 % of 33.33 ms)"), :947 (PM-0004-8:
  "≤ 13.33 ms"), :1014 (PM-0004-71: "≤ 4.17 ms (25 % of 16.67 ms), max ≤ 16.67 ms", "≤ 6.67 ms"); GLOSSARY §8:783
  and condition 2 ("8.33 ms", "6.25 ms").
- Disagreement: a p99 of 4.18 ms passes 0001's 60 Hz mark and fails 0004's; 8.31 ms passes 0004's S1 claim and fails
  PM-0004-3. ADR 0004 disagrees with itself (S1/S2 vs PM-0004-3/-8/-71). (Synthesis list item 1, widened to the 30 Hz rows.)
- Outcome: every pass mark states the share at two decimals, the form TICK-60-HEADROOM's ruling uses (8.33, 6.25):
  30 Hz: 16.67 / 8.33 / 8.33 / 13.33 ms, max under 33.33 ms; 60 Hz: 8.33 / 4.17 / 4.17 / 6.67 ms, max under 16.67 ms.
  Prose and the GLOSSARY's one-decimal sums may keep "about 8.3 ms". The owner of each share states it (N4 and N6:
  0001; N5 and N11: 0004); the other cites.
- Owners: test-engineer for the PM rows it wrote in 0001 (PM-0001-39, -40, and every 30 Hz row with "8.3"/"16.7") with
  netcode-architect for part 2's prose; rust-reviewer for 0004's claim table S1, S2 (lines 823–824).

### F4 (M). The Mac test machine is defined five ways
- Documents: docs/SPIKE_PLAN.md:30 ("MAC, a macOS machine with Safari"), :167 ("MAC with the latest Safari");
  docs/adr/0002-code-structure-build-toolchain.md:949 ("with the latest Safari"); docs/adr/0001-multiplayer-netcode.md:
  1017–1019 ("the stable releases on the day of the run, recorded (R2-TRN-REC1 mark 1 names ... Safari 27"); docs/adr/
  0003-rendering.md:822, :897, :1143 ("Safari 26"); docs/adr/0004-simulation-core.md:865 ("a macOS machine with Safari
  26"); docs/adr/0005-content-abilities-assets.md:1932 ("Safari 26"); docs/adr/0006-infrastructure-operations.md:817
  ("a macOS machine with Safari").
- Disagreement: one shared machine (SPIKE_PLAN "Shared fixtures") pinned to Safari 26 in three drafts, "latest" in two,
  27 in one. Safari 26.0 is WebGPU's floor (R3-WGPU-F19, r3:171–174); WebTransport needs 26.4 (r2:1008–1021); R2-TRN-
  REC1 mark 1 names 27 (r2:2586). A Safari 26.0 machine fails 0001's transport marks by construction.
- Outcome: SPIKE_PLAN:30 defines MAC once: "a macOS machine with the stable Safari on the day of the run, its version
  recorded (WebGPU from 26.0, R3-WGPU-F19; WebTransport from 26.4)". 0003, 0004, 0005 replace "Safari 26" in their
  machine keys and matrices with "MAC (SPIKE_PLAN)"; 0001 and 0002 cite it.
- Owners: engine-architect synthesis (SPIKE_PLAN:30); graphics-engineer (0003); rust-reviewer (0004); the 0005 frame
  author (engine-architect) for 0005:1932.

### F5 (M). A "Retire (exact)" phrase in two pass marks
- Documents: docs/adr/0001-multiplayer-netcode.md:1097 (PM-0001-4: "the whole tick timed directly on the tick thread"),
  :1134 (PM-0001-39: "the whole tick timed directly, never summed from its phases"); GLOSSARY "Server tick time",
  Retire (exact) "whole tick" (the session's glossary script, consistency-scripts.txt: exact hits 2).
- Outcome: "the server tick time measured directly on the tick thread" (PM-0001-4) and "the server tick time measured
  directly, never summed from its phases" (PM-0001-39).
- Owner: test-engineer (its rows in ADR 0001).

### F6 (M). The look proof test: ADR 0003 owns it but never names ADR 0005's share
- Documents: docs/adr/0003-rendering.md:806–837 (the look proof test and part 11's asset list; 0 mentions of PM-0005,
  OQ-0005-AP2 or the kit builders in the whole file); docs/adr/0005-content-abilities-assets.md:1498, :1901 (its share:
  PM-0005-33 to -37, OQ-0005-AP2's families checklist, the content crate's whole-set check, ported kit builders beside
  authored models); docs/SPIKE_PLAN.md:212–234 (lists both, and says "ADR 0003's own asset list names only authored
  assets"). Synthesis list item 6.
- Disagreement: an implementer reading the owning draft builds a look test that cannot run PM-0005-33 to -37 (no kit
  builders, no minimal content loader) and Mustafa's per-family pick (OQ-0005-AP2) has no slot in 0003's checklist.
- Outcome: ADR 0003's look proof test gains one bullet: "ADR 0005's share runs in the same test: PM-0005-33 to PM-0005-37,
  the families checklist for OQ-0005-AP2; inputs added: the ported kit builders beside the authored models, and a
  minimal content loader running the content crate's whole-set check." Part 11's asset list adds the kit builders.
- Owner: graphics-engineer (0003).

### F7 (M). Two research marks placed in the sim core spike by SPIKE_PLAN only; no draft owns a row for them
- Documents: docs/SPIKE_PLAN.md:204–207 ("no draft's table names them ... R2-SYNC-REC6 mark 1 (throws bit-identical on
  Wasm and native) and R5-LAYOUT-REC4 (one feature set for `sim` on every target)"); docs/adr/0004-simulation-core.md:
  822–843 (spike claims S1–S20) and :940–1016 (PM-0004 rows: neither id); 0001 and 0002 adopt the recommendations.
  Synthesis list item 3.
- Disagreement: SPIKE_PLAN may place spikes but not write marks ("Pass marks ... are never restated here", SPIKE_PLAN:9);
  as written, the sim core spike carries two marks with no owning pass-mark row, so nobody's table judges them.
- Outcome: ADR 0004 adds two pass-mark rows to the sim core spike: "throws bit-identical on Wasm and native (R2-SYNC-
  REC6 mark 1; the id is ADR 0001's)" and "one feature set for `sim` on every target (R5-LAYOUT-REC4; the id is ADR
  0002's, re-proven later as a gate check)", with their PM numbers cited back in SPIKE_PLAN:204–207.
- Owners: rust-reviewer with test-engineer (0004 rows); engine-architect synthesis (SPIKE_PLAN cites the new numbers).

### F8 (H). The netcode proof test's WebSocket leg needs a certificate only ADR 0006 knows about (new with 0006's fix round, 09803a9)
- Documents: docs/adr/0006-infrastructure-operations.md:655–672 (OQ-0006-5, now: "which certificate does the WebSocket
  fallback present on development and staging servers?"; "A WebSocket cannot pin a certificate hash ... the netcode
  proof test's WebSocket leg needs one first (PM-0006-15)"; Decider "... the development and staging case before the
  netcode proof test starts (the Advisor, from a short read)"), :871 (PM-0006-15: 20 of 20 `wss` sessions from each of
  the four browsers); docs/adr/0001-multiplayer-netcode.md:334–337 ("development and preview servers use self-signed,
  pinned certificates valid at most 14 days, whose hash the page fetches"), :946–947 (the spike's certificate mark names
  only the pinned self-signed certificate); docs/SPIKE_PLAN.md:104–110 (item 1 "Depends on": "the development
  certificates (ADR 0006)" only); docs/adr/README.md:94–103 (no OQ-0006-5 row, though it now routes a part to the Advisor).
- Disagreement: read alone, ADR 0001 and SPIKE_PLAN start the proof test with pinned self-signed certificates on both
  transports; ADR 0006 says the `wss` leg cannot work that way (R2-ROOM-F12, r2:1494–1500) and makes a decision a
  precondition. The first spike would stall on its WebSocket marks (0001's R2-TRN-REC3 leg, PM-0006-15).
- Outcome: 0001:336 reads "development and preview servers use self-signed, pinned certificates for WebTransport ...
  (R2-TRN-REC5); the WebSocket fallback cannot pin a hash (R2-ROOM-F12) and presents the certificate ADR 0006's
  OQ-0006-5 names for development and staging". SPIKE_PLAN item 1 "Depends on" adds "OQ-0006-5's development and staging
  answer (the Advisor, before the test starts)". README "Decisions the drafts ask for" adds the OQ-0006-5 row (the
  Advisor for development and staging; security-auditor for production).
- Owners: netcode-architect (0001:336, and 0001:946 if it should name the `wss` leg's certificate); engine-architect
  synthesis (SPIKE_PLAN, README).

### F9 (M). README's line pointers into ADR 0006 are stale after 0006's fix round
- Documents: docs/adr/README.md:94–103. Script check of all 68 "line N" pointers in README against the current drafts:
  10 miss, all into ADR 0006 (OQ-0006-1 at 598 is now 613; -2 608→625; -3 615→632; -4 626→643; -6 647→673; -7 658→684;
  -8 668→694; -9 677→703; -10 686→712; -12 701→727). The other 58 hold (four rows that name a decision code with its OQ
  in brackets match the OQ's line).
- Outcome: re-point the ten rows after every draft's fix round lands (a last script pass before the PR is marked ready).
- Owner: engine-architect synthesis (README).

### F10 (M). Line citations into STATUS are stale (STATUS was rewritten by 549490f); plan rule C12 says cite the code
- Documents and what the cited STATUS line holds today: docs/adr/0003-rendering.md:45 (STATUS:133–134, carry-forward 3:
  now an unrelated line), :46 (STATUS:94–101, SLICE-ORDER: now the "Taken by Mustafa" header), :50 (STATUS:108–109,
  SEC-SPREAD: now the foundation's text; SEC-SPREAD is at STATUS:151–152), :52 (STATUS:118–120, TICK-30: now a header;
  TICK-30 is at STATUS:161), :59 (STATUS:187–188, SPIKE-SCOPE: now at :235), :60 (STATUS:159–160, R7-MARKS: now at
  :207), :141 and :1182 (STATUS:202, ADR-003: now at :252); docs/adr/0001-multiplayer-netcode.md:97 (STATUS.md line
  199, ADR-001's inputs: now at :249); docs/adr/README.md:208–214 (STATUS:198, 199, 200–201, 202, 257, 262, 273: ADR-001..3
  now at :246–252, SEC-MDL-2 at :314, CDN-SRI at :319, WALL-CLOCK at :330) and :333 (J2's ":257, :262, :273").
- Disagreement: a reader following the pointer lands on another decision. Plan rule C12 (plan.md:79): cite settled
  inputs "by the STATUS decision code or the GLOSSARY §8 entry".
- Outcome: the drafts cite the codes alone (SEC-SPREAD (a), TICK-30, SLICE-ORDER, SPIKE-SCOPE (a), R7-MARKS (a),
  carry-forward 3); README's loose-ends rows cite the STATUS section and item name ("STATUS, ADR-001", "STATUS, Deferred:
  SEC-MDL-2") instead of line numbers, since docs-keeper edits STATUS again before ready.
- Owners: graphics-engineer (0003:45–60, 141, 1182); netcode-architect (0001:97); engine-architect synthesis (README).

### F11 (M). One Wasm file, two procedures for its size budget
- Documents: docs/adr/0002-code-structure-build-toolchain.md:233–236 (part 4: "the boundary spike and the sim core spike
  measure the `sim-wasm` file ... and the budget file (part 9) takes the measured value as its first limit"), :922–925
  (spike item 8), :1292 (R1-RR-OQ5: "the budget file takes the measurement"); docs/adr/0004-simulation-core.md:1149
  ("the budget is ADR 0002's (R1-RR-OQ5)"); docs/adr/0005-content-abilities-assets.md, OQ-0005-CD2 ("What load time and
  Wasm size may content cost ... Decider: the content and ability spike's measurement, then the Advisor sets the
  budgets").
- Disagreement: the content crate (serde_json, garde) lives in the same `sim-wasm` file (0002 part 3: `sim-wasm` = `sim` +
  `room` + `protocol` + `content`). Under 0002 the first limit is fixed by the boundary and sim core spikes, which run
  before the content and ability spike (SPIKE_PLAN order), so content's bytes would fail the limit the day they land;
  under 0005 the Advisor sets a content Wasm budget later. Two deciders and two dates for one number.
- Outcome: ADR 0002 owns the `sim-wasm` size budget (N50); its first limit is set after the content and ability spike
  has measured the content crate's share (until then the size is recorded, not gated), and the Advisor sets it from both
  measurements. OQ-0005-CD2 keeps content load time and the content share's size as inputs to that one budget and cites
  ADR 0002 part 4 for the decider.
- Owners: engine-architect (0002 part 4 and item 8; also the 0005 content-section author for CD2's decider line).

### F12 (L). Codes without a lasting home, and code families that collide
- Documents: docs/adr/0002-code-structure-build-toolchain.md:318 ("decision map S11": README's seam table has S1, S3, S4,
  S6, S12 only), :918, :936, :1034 ("the decision map's part g", "map part g, 'Not throwaway spikes'", "the map's part
  g": README carries no part g); docs/SPIKE_PLAN.md:354 ("decision map part h": README's "Research gaps" is that part,
  under another name); docs/adr/0003-rendering.md:1275 (lists the decision map's checkpoint file among sources). The
  map is deleted before the PR is marked ready.
- Collisions (synthesis list item 11): bare G-n and S-n are used for four different families: ADR 0002's gate checks
  G1–G13 (0002:433 on, e.g. "G5 | dependency rules"), ADR 0006's option labels G1–G4 (hosting, 0006 Options) and S1–S3
  (record storage), ADR 0004's spike claims S1–S20 (0004:822–843), and README's gaps G1–G15 and seams S1–S12 (cited
  with "gap"/"seam" in front). "G5" is dependency rules in 0002 and visual effects as "gap G5".
- Outcome: 0002:318 cites "seam S11" and README adds the S11 row (offline and local play: DESIGN decides whether offline
  ships; 0002 and 0004 build local play from slice 2), or 0002 drops the code and cites R1-BND-OQ1 (DESIGN). 0002's
  "part g" references become "SPIKE_PLAN item 3" / "SPIKE_PLAN, the foundation's acceptance checks". SPIKE_PLAN:354 says
  "(README, research gaps)". 0003:1275 drops the checkpoint file from its sources or names README instead. README's
  codes section adds one line: "ADR 0002's gate checks G1–G13, ADR 0004's spike claims S1–S20 and ADR 0006's option
  labels (G1–G4, S1–S3, K1–K4, B1–B11) are local to their draft; register codes always carry their word (gap, seam)".
- Owners: engine-architect (0002), engine-architect synthesis (SPIKE_PLAN, README), graphics-engineer (0003:1275).

### F13 (L). Terms: the same thing under different words
- Event names: ADR 0004 part 19 emits the events (I-10, I-20; docs/adr/0004-simulation-core.md:448–452: "hits, deaths
  and kills ..., blasts ..., cosmetic shot events, purchases, uplink cache events, harvester deliveries and kills,
  structure kills and stage changes, tech tiers, upgrade picks and pickups"); docs/adr/0001-multiplayer-netcode.md:230
  ("hits, kills, deaths, purchases, captures, tier changes, pick results, ..."), docs/adr/0002-code-structure-build-
  toolchain.md:253 ("captures, pick results, drops") and docs/ENGINE_OVERVIEW.md:57 ("captures") name a "capture" event
  that neither 0004 nor LOOP_V2 defines (LOOP_V2 has no "captur" at all). Outcome: the three use 0004's words ("uplink
  cache events", "tech tiers", "upgrade picks and pickups") or say "the events of ADR 0004 part 19". Owners:
  netcode-architect (0001:230), engine-architect (0002:253), engine-architect synthesis (ENGINE_OVERVIEW:57).
- "The proof test": GLOSSARY §8 marks the term by "the netcode proof test" only; ADR 0001 shortens it 23 times, and the
  short form also appears in docs/SPIKE_PLAN.md:369, docs/adr/README.md:228 and docs/adr/0006-infrastructure-operations.md:
  550, beside "the look proof test". Outcome: the full name outside ADR 0001 (SPIKE_PLAN, README, 0006); ADR 0001 may
  keep the short form after defining it once ("the netcode proof test (below: the proof test)"). Owners: synthesis
  (SPIKE_PLAN, README), infra-engineer (0006:550), netcode-architect (0001, one definition line).
- Glossary in-context hits judged wrong or marginal (of 489; see the table below): docs/adr/0005-content-abilities-
  assets.md:2027 (PM-0005-32, "points 4 effects per soldier apart": a bare "effects" in a count, retired in context;
  read "4 active effects per soldier") (owner: test-engineer, its row); docs/adr/0006-infrastructure-operations.md:869
  (PM-0006-13 "8 rooms of 48 on each machine": "48" alone for the largest mode's N; read "8 rooms at N = 48") and :873
  (PM-0006-17 "V = 15 moving vehicles": the stress scene's big vehicles; read "15 big vehicles" unless it quotes R2's
  symbol) (owner: test-engineer, its rows).

### F14 (L). Small seam and cross-reference slips
- R5-TEST-OQ2 (the WebTransport crates' interop): docs/adr/0002-code-structure-build-toolchain.md:1351 disposes it as
  "spike: the netcode proof test (... ADR 0001)"; ADR 0001 never names the id (0 hits), though its spike covers
  interop (0001:404, R2-TRN-REC1 and REC3). Outcome: ADR 0001's "What the proof test decides" (0001:994 on) adds
  R5-TEST-OQ2. Owner: netcode-architect.
- OQ-0001-1 option (b) "another reading of TICK-30's '25 % headroom'" (docs/adr/0001-multiplayer-netcode.md:843) names no
  reading, while GLOSSARY:30–31 asks a "Depends on" to state every reading, and GLOSSARY §8's entry (:790) states only
  the ruled one (synthesis list item 14). GLOSSARY §8 is the Advisor's text (condition 3) and does not change. Outcome:
  OQ-0001-1 (b) reads "not confirmed: Mustafa and the Advisor name the base, and part 2's mark and GLOSSARY §8 follow".
  Owner: netcode-architect.
- OQ-0005-AB5 ends "ADR 0006 cites this" (docs/adr/0005-content-abilities-assets.md:1160); ADR 0006 does not (0 hits for
  AB5, WEAPON-UNLOCKS or unlock), and needs not: accounts are `[LATER]` (0006:34). Outcome: drop the sentence, or 0006:34
  adds "the unlock catalogue's weapon question is OQ-0005-AB5". Owner: game-designer (AB section).
- Telemetry timeline: docs/adr/0004-simulation-core.md:460 states "The timeline is sampled every 10 s of game time", a
  part of the record's shape, which seam S6 gives ADR 0006 (0006:341–342 states the same 10 s). Same value; 0004 should
  cite it ("ADR 0006's record samples ... every 10 s"). Owner: rust-reviewer.
- Author lines: ADR 0001, 0002 and 0006 credit test-engineer for the "Pass marks" table on line 5; ADR 0003, 0004 and
  0005 carry test-engineer's PM rows too but do not credit them (each file's line 5). Outcome: one form on all six,
  "(the 'Pass marks' table: test-engineer, plan step 4)". Owners: graphics-engineer, rust-reviewer, the 0005 frame
  author (engine-architect).

## The known items (brief, check 6): one outcome each
| Item | Outcome | Who changes what |
|---|---|---|
| ADR 0005 CD-7 vs ADR 0001 `ContentReload` and ADR 0004 part 16 | F1: OQ-0005-CD4 is the one place; 0004 part 16 conditional on CD4 (b); 0001's row conditional on CD4 (b) | rust-reviewer (0004:412–413), netcode-architect (0001:488), synthesis (SPIKE_PLAN:158) |
| OQ-0004-5, OQ-0004-6 vs OQ-0005-AB9, AB10 | F2: one question each, in ADR 0005; 0004 deletes its two and cites; part 18's "its weapon becomes a pickup" made conditional | rust-reviewer, the 0005 ability authors, synthesis (README:69–70) |
| Audio voices 32 Low / 48 High vs the map's N48 | consistent: ADR 0003:360, :991 and README:327 follow r3:1244–1245; only the checkpoint map said 32 on both tiers, and it is deleted before ready | nobody |
| R1-TICK-OQ1 and R2-INT-OQ5: 0001 or 0004 | stay in ADR 0001 (0001:1325, :1404); ADR 0004 cites 0001 (0004:243, :1166) and measures the pass (PM-0004-52) | nobody |
| R5-TEST-OQ2 | stays in ADR 0002 (0002:1351) as "spike: the netcode proof test"; ADR 0001 must name it (F14) | netcode-architect |
| R5-TEST-OQ3 | stays OQ-0002-6 (a design question for the game-designer with Mustafa; README:55); consistent with 0004's reach (PM-0004-59) | nobody |
| R5-PREV-OQ5, R5-PREV-OQ6 | stay in ADR 0002 as spikes; ADR 0006 decides them (PM-0006-2, PM-0006-10); SPIKE_PLAN:262 agrees | nobody |
| R5-PREV-REC3 | stays in ADR 0006 (policy); ADR 0002:510, :524 holds the close and sweep jobs and cites 0006 (seam S3) | nobody |
| R1-RR-OQ7, R5-GATE-OQ4 | stay in ADR 0006 (OQ-0006-6, B1); ADR 0002:472–474 defines the advisories job and cites the baseline (seam S3 holds) | nobody |
| R2-ROOM-OQ11 | stays in ADR 0006 (OQ-0006-6, interim HttpOnly cookie); ADR 0001:366, :1380 cites 0006 | nobody |
| ADR 0001's four replaced lines vs condition 2's three | ADR 0001's four are the consistent text: R2-BUD-REC1's tick half (r2:2302) states the same rule TICK-30 replaces; leaving it would leave a contradicting research line. No draft change; the session's report names it as an addition to condition 2 for the Advisor | the session (report) |
| Decision-map codes (I-n, S-n, G-n, LE-x, J-n, N-n) | every code the drafts cite has a README row (I-1 to I-21; seams S1, S3, S4, S6, S12; gaps G1–G5, G7–G9, G11, G15; LE-B, LE-N; J1–J6; the twelve N-codes), except "decision map S11" and "part g"/"part h" (F12) | engine-architect (0002), synthesis |
| ADR 0006's goal cost at two densities | consistent: 0006:260 and :264 (about 361 EUR at 8 rooms per VM, about 534 EUR at 4), :548–551 (why both), :1053 ledger; no other document restates a goal cost; 0001's PM-0001-27 runs the 8-room test both use | nobody |

## Ranked index
- **H (3):** F1 development reload (0004 part 16 vs 0005 CD-7); F3 tick-budget marks at two precisions (0001 vs 0004,
  and inside 0004); F8 the netcode proof test's WebSocket leg needs OQ-0006-5's certificate (0006 fix round vs 0001,
  SPIKE_PLAN, README).
- **M (8):** F2 drops and bots' records asked twice; F4 the Mac machine defined five ways (Safari 26 / 27 / latest);
  F5 "whole tick" (Retire exact) in PM-0001-4, -39; F6 ADR 0003's look proof test omits ADR 0005's share; F7 two marks
  with no owning row (R2-SYNC-REC6 mark 1, R5-LAYOUT-REC4); F9 README's ten stale pointers into ADR 0006; F10 stale
  STATUS line citations (0001, 0003, README); F11 two procedures for the one `sim-wasm` size budget.
- **L (3, several items each):** F12 codes without a home and colliding code families; F13 terms (captures, "the proof
  test", three glossary hits); F14 small slips (R5-TEST-OQ2, OQ-0001-1 (b), AB5, the 10 s timeline, author lines).

## Checked and found consistent
| Check | What was compared | Result |
|---|---|---|
| Shared numbers: network | snapshot cap 1,000 / 1,150 / 1,067 B, 160 / 256 / 48 kbps, interpolation delay, rewind cap, profiles P0–P4 | stated only in ADR 0001; P0–P4 restated with 0001:393's values in 0002:967, 0003:1149, 0005:1970, 0006:835 |
| Shared numbers: rooms, hosts, CI | 64 MB per room, room start-up under 2 s (0001:1109 and 0006 PM rows), 14-day certificates (0006 only), 25 MiB (0002, 0005 cite 0006), previews 5 min / 24 h / 7 days, CI 10 / 20 min, logs 14 days, metrics 15 days, goal costs | same everywhere, owner states, others cite |
| Shared numbers: client budgets | 512 MB, 4 draw calls per soldier, draw caps 800 / 1,100 / 1,500, page's cost of the sim (p99 in 0003 PM-0003-20, p95 difference in 0002 PM-0002-4, "both gate" in both) | consistent (LE-B handled the same way in both) |
| Shared numbers: tick | 30 Hz baseline, 6.25 / 8.33 ms margin, 36,000 ticks = 20 minutes, Worker jitter 4 ms, server jitter 2 ms | consistent except the precision (F3) |
| Seams S1–S12 | S1 (0001 adds 0004's share, 0001:528, 0004:84, :215); S2 (block: 0002:248–262 vs 0001 fields, 0004 part 19); S3 (0002 jobs, 0006 policy); S4 (0006:88 gives VMs, supervisor, firewall); S5 (except F1, F11); S6 (except the 10 s line, F14); S7 (audio occlusion stays on the page: 0003:366, :1178; 0004 has no audio part); S8 (bots on the input path: 0001 part 11, 0004 part 15); S9 (tree style open); S10 (OQ-0002-1 keeps EDITOR_V2 §5 as option (a)); S11 (DESIGN; local play from slice 2); S12 (baseline cited by 0001, 0002, 0005) | consistent apart from the findings named |
| Interfaces | crate and package names (0002 part 3 list; 0001, 0004, 0005, ENGINE_OVERVIEW use the same), message names (0005 uses `ContentReload`, `HostCommand`, `JoinAccepted`, `JoinRejected`, all in 0001:481–490), aim point I-19 (0001 part 5, 0002:268, 0003:382, 0004:302), block contents, `packages/types`, OQ-0002-3 with OQ-0004-7 | consistent (event names: F13) |
| ENGINE_OVERVIEW | every part reference into 0001 and 0004 (part numbers match the drafts' headings), the data flow, state ownership (vs 0001 "State ownership", 0001:445–466), the §6 table, the settled list | consistent (one word, "captures": F13) |
| Settled inputs | N = 48 (the 244 "48" hits screened), TICK-30 and the 60 Hz margin (0001 part 2, GLOSSARY §8, 0002:47, 0004:46, 0006:51), SEC-SPREAD (a) (0001:178–181, :450; 0003:50–51, :238; 0004:260–266), PREVIEW-WALL (a) (0002:49–50, :511–512; 0006 part 2 after its fix), BASELINE-WHEN (b), UPGRADE-SCOPE (0001 part 12, 0004 part 18, 0005), ABL-PREDICT open (0001 Q2 option D "ABL-PREDICT decides", 0004:261, 0005:742), LOOK_V2's `[OPEN]` tree style (0003:12, :443, OQ-0003-9) | no settled input weighed as an option; 0004 part 18's drop sentence pre-empts a design question (F2) |
| Open questions | all 112 OQ titles; overlaps: OQ-0002-4 / OQ-0006-12 (split by R1-BND-OQ5, cross-cited), OQ-0002-5 / OQ-0006-7 (jobs vs cost, cross-cited), OQ-0002-3 / OQ-0004-7 (cross-cited), OQ-0005-CD2 vs 0002 (F11), OQ-0004-5/-6 vs AB9/AB10 (F2) | the rest ask distinct questions |
| Design ids | the 38 DESIGN ids the dispositions script left open | all 38 have a README "Design questions" row |
| GLOSSARY | 489 in-context hits from glossary-hits-drafts.tsv, all judged: 124 read one by one (tick budget, goal modes, client host, access wall, "always", "its hash", time statistics, single phrases); 244 "48" hits screened by script, the 57 occurrences without a marking word read; 49 "effects", 37 "vehicles", 37 "players" screened, flagged ones read. Wrong: the 2 exact hits (F5), 1 bare "effects" count; marginal: 2 (F13). "headroom": every use is the N = 64 run or a quote of TICK-30 | otherwise right |
| Spike names | drafts use SPIKE_PLAN's names (netcode proof test 108, look proof test 72, sim core spike 56, boundary spike 63, render budget bench 33, content and ability spike 31, asset spike 26, Feel Lab test 16, preview hosting check 15); "the build spike" 0 | consistent ("the proof test": F13) |
| Not checked (time) | none of the brief's checks was skipped; not re-read: every pass-mark row's arithmetic (the verifiers' and test-engineer's ground), and the 0001–0005 fix rounds that land after this pass | — |
