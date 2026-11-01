# Consistency findings on the v2 decision drafts (plan step 8, engine-architect as consistency critic, 2026-11-01)

Ranks: H (a reader gets a different decision, number or interface depending on which document they read; must change
before review), M (a seam, owner or term disagrees but the decision itself is the same; change in the fix round),
L (form: precision, citation form, code collisions). Line numbers are against the files as of 09:20 Amsterdam.
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
- Owners: rust-reviewer (0004); engine-architect and game-designer (0005 AB9); engine-architect synthesis (README).

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
