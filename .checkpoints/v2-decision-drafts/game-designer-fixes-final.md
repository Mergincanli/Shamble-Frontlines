# game-designer: the final fix round, ADR 0005 "The ability and effect system" (BEFOREs from b88714d)

### FIX RR-4 (the 0005 half) + POOL-FULL (rust-reviewer's routed note: AB-1 points to OQ-0004-11; the game-designer proposes per kind)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
    know). Stacking is an explicit policy per effect, from a short list: no stacking, refresh the duration, add a stack
    up to a limit, strongest only. Left out: what exists for Unreal's object and replication model (instancing
    policies, replication modes, ability tasks as a framework), and anything no legacy or LOOP_V2 test case needs.
  - Why: §6.3 asks for GAS-like data, and the subset covers every legacy ability and weapon and the primitives §6.4
AFTER:
    know). Stacking is an explicit policy per effect, from a short list: no stacking, refresh the duration, add a stack
    up to a limit, strongest only; what a full pool does to a new row sits beside it (the next item). Left out: what
    exists for Unreal's object and replication model (instancing policies, replication modes, ability tasks as a
    framework), and anything no legacy or LOOP_V2 test case needs.
  - When a pool is full (OQ-0004-11: the game-designer proposes per kind, and Mustafa confirms where a player could
    notice; ADR 0004 part 1 fixes each pool's capacity at match start and counts every overflow; CD-2's whole-set
    check reports each pool's worst case, or names the missing cap). Proposed, per kind:
    - Effects the content bounds on their target (its own picks, its own abilities' and weapon's effects, spawn
      protection): the whole-set check reserves their worst case in every target's rows, so they never give way.
      Reason: their count follows from the content alone, and a pick or a pressed ability that silently fails to apply
      takes away what the player earned (pillar 2).
    - Effects that other soldiers, vehicles or structures apply (one row per application under "no stacking", so play
      sets their count, not the content): they share the rest of the target's rows, up to a cap in data, set once
      AB-10's curve shows what the step can carry; when that share is full, the oldest such row on the target, by start
      tick and then sequence number (ADR 0004 part 4), ends early (OQ-0004-11's option (b)). Reason: refusing the new
      row would make a saturated target immune to every further slow or mark, so cheap buffs stacked by teammates could
      shut out the enemy's answers (pillar 3); ending the oldest keeps every new application landing, at the cost of an
      older effect ending early, which the overflow counter shows to tuning.
    - Live projectiles: the pool holds the content's worst case at the mode's N, so honest play cannot fill it. Per
      shooter that is at most ⌊lifetime ÷ fire interval⌋ + 1 live shots of each projectile weapon it can carry at once
      (carry slots are OQ-0005-AB1's), times the weapon's pellets; legacy's Molotov (fuse 1.4 s, r1:1373; fire interval
      1.05 s, LEGACY_TUNING.md, `WEAPONS`, lines 167–203) gives ⌊1.4 ÷ 1.05⌋ + 1 = 2 per thrower, and 2 × 48 = 96 at
      N = 48 (arithmetic). Refusing the new row (option (a)), counted, stays as the safety net, and a non-zero count is
      a content or capacity bug, not a rule a player meets. Reason: a shot that never leaves the gun and a projectile
      that vanishes in flight would both be seen, so the pool is sized never to need either; R2-BUD-OQ5's caps, once
      set, bound the content (a set whose worst case exceeds them fails the check), not the play.
    - Weapon pickups: the pickup that has lain longest ends early (option (b)), whatever lifetime OQ-0005-AB9 sets.
      Reason: UPGRADE-SCOPE makes drops the only source of other weapons, so the drop from the fight just won matters
      more than one nobody has taken, and refusing it would leave a kill with no drop.
    - Research: none (no research sizes a pool); R2-BUD-OQ5 (r2-netcode.md, Open questions, Brief question 2, lines
      2982–2985) for the caps; R1-ABL-REC6's "allocates nothing on the heap in steady state" (r1, Recommendations §5,
      lines 2471–2472). Need: VISION_V2 §3 and §4 (a budget, not a feature, as AB-10).
END

