# game-designer: fix round 2 for ADR 0005, "The ability and effect system" (2026-11-01)

Sources: engine-architect-consistency-findings.md F2 and F14; verifier-adr0005-verdicts.md A246; verifier-skeptic-3-results.md
(A246 upheld: ADR 0006 has 0 hits for AB5, unlock or OQ-0005; its nearest lines are 0006:34, accounts `[LATER]`, and
0006:608, legacy row GE15, "account progression stays `[LATER]`"). Every BEFORE was copied by script from
docs/adr/0005-content-abilities-assets.md as it is now (lines 1160 and 1188–1196) and occurs once there.

### FIX F14-A246
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
    data model rules none out (a unit definition names its weapons, AB-4). ADR 0006 cites this.
AFTER:
    data model rules none out (a unit definition names its weapons, AB-4). ADR 0006 builds no account store (accounts
    are `[LATER]` there, its legacy row GE15 included), so the unlock catalogue's weapon question is asked only here.
END

### FIX F2
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
- **OQ-0005-AB9. Which drop and pickup rules does UPGRADE-SCOPE leave unstated?**
  - Why it matters: ADR 0004 builds pickups as sim entities that read these values from data; until they exist, the
    spike uses placeholders.
  - Options for what a killed enemy drops: (a) its class weapon; (b) the gun in its hands; (c) every weapon it carried.
    Also Mustafa's, each a value in data: how long a pickup stays; whether a gun a player swaps out becomes a pickup that
    anyone can take; whether only the killer's team, or anyone, may take a drop; whether a teammate's death drops
    anything (his words name "killed enemies' drops" only).
  - Decider: Mustafa. What decides it: a design session, with OQ-0005-AB1. Research ids: none; whether a pickup is
    predicted is ADR 0001's (Quake III predicts item pickups: R2-SYNC-F17, r2-netcode.md, Findings, lines 230–236).
AFTER:
- **OQ-0005-AB9. Which drop and pickup rules does UPGRADE-SCOPE leave unstated?**
  - Why it matters: ADR 0004 builds pickups as sim entities that read these values from data; until they exist, the
    spike uses placeholders. This is the one place the question is asked: ADR 0004's OQ-0004-5 points here and keeps
    only the mechanism (two claims on one pickup in one tick resolve in its phase's fixed, deterministic order).
  - Options for which deaths drop: (i) any death; (ii) only a death to an enemy (his words name "killed enemies' drops"
    only). Options for what a dead soldier drops: (a) its class weapon; (b) the gun in its hands; (c) every weapon it
    carried. Options for who may take a drop: the killer's team; any enemy of the dead soldier (in three-team modes two
    teams, not one); or anyone. Also Mustafa's, each a value in data: how long a pickup stays; whether a gun a player
    swaps out becomes a pickup that anyone can take; which of two claims on one pickup in the same tick wins (ADR 0004
    keeps the order deterministic whichever is chosen).
  - Decider: Mustafa, with the game-designer. What decides it: a design session, with OQ-0005-AB1. Research ids: none;
    whether a pickup is predicted is ADR 0001's (Quake III predicts item pickups: R2-SYNC-F17, r2-netcode.md, Findings,
    lines 230–236).
END
