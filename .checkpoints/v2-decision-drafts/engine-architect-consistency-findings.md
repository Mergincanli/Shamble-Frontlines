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
