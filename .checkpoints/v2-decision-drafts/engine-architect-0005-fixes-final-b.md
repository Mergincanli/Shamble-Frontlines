# engine-architect, ADR 0005 follow-up batch b (frame and content data): fixes on d57dc5a

Routed from game-designer's ability-section batch (d57dc5a; its NOTEs RR-4 and GD-7). BEFORE blocks are whole
lines copied by script from docs/adr/0005 at d57dc5a; each occurs exactly once and no two overlap.

### FIX RR-4 (routed by game-designer: CD-2 reports the reserved worst case per target)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  - The pools' worst case (for ADR 0004 part 1, which gives every sim pool a fixed capacity at match start from the
    mode and the content set, so that the step never grows one and nothing a client sends sizes it): the whole-set
    check reports, for each mode in the set, the worst case of each pool (live projectiles, active effects, weapon
    pickups, big vehicles and their parts), from the content's numbers (fire intervals and projectile lifetimes,
    pellets per shot, effect durations and stack limits, drops per death and pickup lifetimes), the mode's team count
    and team size, and the game-designer's caps where they exist; the report travels with the frozen set. Where the
    content alone sets no bound, the report names the pool and the missing cap instead of a number, and nothing fills
    it with a default: "Caps on vehicles and live projectiles per team are not in VISION_V2 or LOOP_V2", and until the
    game-designer sets them every topic's stress marks use the stress scene's 15 big vehicles and 300 live projectiles
    (R2-BUD-OQ5, r2-netcode.md, Open questions, Brief question 2, lines 2982–2985). No research sizes a pool, so each
    pool's arithmetic is written beside its kind and reviewed like the bounds above (proposed); which row gives way
    when a pool is full is ADR 0004's OQ-0004-11.
AFTER:
  - The pools' worst case (for ADR 0004 part 1, which gives every sim pool a fixed capacity at match start from the
    mode and the content set, so that the step never grows one and nothing a client sends sizes it): the whole-set
    check reports, for each mode in the set, the worst case of each pool (live projectiles, active effects, weapon
    pickups, big vehicles and their parts), from the content's numbers (fire intervals and projectile lifetimes,
    pellets per shot, effect durations and stack limits, drops per death and pickup lifetimes), the mode's team count
    and team size, and the game-designer's caps where they exist; the report travels with the frozen set. For active
    effects it reports per target, in the two shares AB-1 proposes ("When a pool is full"): the reserved worst case of
    the effects the content bounds on that target (the pick effects of the tracks that can apply to it at once, one
    per track level; its unit's own ability and weapon effects; spawn protection), which every target's rows reserve;
    and, for the effects other soldiers, vehicles and structures apply, whose count play sets, the missing cap. Where
    the content alone sets no bound (those effects, weapon pickups, big vehicles), the report names the pool and the
    missing cap instead of a number, and nothing fills it with a default: "Caps on vehicles and live projectiles per
    team are not in VISION_V2 or LOOP_V2", and until the game-designer sets them every topic's stress marks use the
    stress scene's 15 big vehicles and 300 live projectiles (R2-BUD-OQ5, r2-netcode.md, Open questions, Brief
    question 2, lines 2982–2985); once those caps are set, a set whose worst case exceeds one fails the check (AB-1).
    No research sizes a pool, so each pool's arithmetic is written beside its kind and reviewed like the bounds above
    (proposed); which row gives way when a pool is full is ADR 0004's OQ-0004-11, and AB-1 holds the game-designer's
    proposal per kind for it.
END

### FIX GD-7 (routed by game-designer: the frame's needs outside VISION_V2 §6, Decision part 5)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
   carried (AB-9); the step's cost mark and a proposed curve (AB-10). *Need:* VISION_V2 §6.3, §6.4, §6.5, §6.7; the
   cost mark: VISION_V2 §3 and §4 only.
AFTER:
   carried (AB-9); the step's cost mark and a proposed curve (AB-10). *Need:* VISION_V2 §6.3, §6.4, §6.5, §6.7; the
   cost mark and the full-pool rules: VISION_V2 §3 and §4 only; AB-2's immunity rule for spawn protection: LOOP_V2 §2
   only.
END

### FIX RR-4 (routed: the content spike's item 2 follows CD-2)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
   default depth limit (R1-DATA-OQ4) and runs the same tests with garde on wasm32 (R1-DATA-OQ5). Proposed (reason:
   ADR 0004 part 1 sizes its pools from this report): for each test mode, every pool's reported worst case equals a
   count by hand from the test content's numbers, and every pool the content leaves without a bound is named as such.
AFTER:
   default depth limit (R1-DATA-OQ4) and runs the same tests with garde on wasm32 (R1-DATA-OQ5). Proposed (reason:
   ADR 0004 part 1 sizes its pools from this report): for each test mode, every pool's reported worst case, and for
   active effects the reserved share per target, equals a count by hand from the test content's numbers, and every
   pool the content leaves without a bound is named as such.
END

### FIX GD-7, GD-PICK-VIS (routed by game-designer: the frame's list gains OQ-0005-AB14 and AB15)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
| OQ-0005-AB13 | When does scripting (option A3) come back? (proposed: if the spike's ability mark 1 or 2 fails, or new operations are needed often; the talk is read when a copy appears) | the Advisor | The ability and effect system |
AFTER:
| OQ-0005-AB13 | When does scripting (option A3) come back? (proposed: if the spike's ability mark 1 or 2 fails, or new operations are needed often; the talk is read when a copy appears) | the Advisor | The ability and effect system |
| OQ-0005-AB14 | What does spawn protection cover beyond damage, and what ends it early? | Mustafa | The ability and effect system |
| OQ-0005-AB15 | Who sees a player's upgrade picks? | Mustafa | The ability and effect system |
END

### NOTE RR-4, GD-7 (what follows elsewhere)
routed: test-engineer's CD-2 row (PM-0005-4 or -5) takes the per-target share in item 2's leg (the reserved worst case
per target equals a count by hand). The synthesis (engine-architect's own batch): README's "Needs outside VISION_V2
§6" list gains "ADR 0005: AB-2's immunity rule for spawn protection (LOOP_V2 §2 only)", and the register gains rows
for OQ-0005-AB14 and OQ-0005-AB15 (Mustafa). The frame has no separate "outside VISION_V2 §6" line; its Decision
parts' *Need:* lines carry those traces, so part 5's line takes the immunity rule (and the full-pool rules, whose own
Need is VISION_V2 §3 and §4, AB-1).
END
