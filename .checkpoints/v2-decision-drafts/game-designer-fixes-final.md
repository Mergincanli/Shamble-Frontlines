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

### FIX GD-7 (rust-reviewer's routed note: does AB-1 have an operation for spawn protection?), part 1 of 3: the pipeline's immunity rule (AB-2)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  - What: the stages are the locational multiplier, range falloff, the frontal-arc block, a target-tag multiplier (for
    counters such as "strong against vehicles") and the owner and team rules. Each hit becomes an instant damage effect
    that passes through them.
  - Why: the outcome logic GAS gives to execution code becomes fixed, reviewed stages, and each number has one home
    (CS3 shows the cost of two homes).
  - Research: R1-ABL-REC1 (lines 2350–2352), R1-ABL-REC2 (lines 2384–2385); R1-ABL-F9.
  - Need: VISION_V2 §6.3 ("combos and counters can emerge").
AFTER:
  - What: the stages are the locational multiplier, range falloff, the frontal-arc block, a target-tag multiplier (for
    counters such as "strong against vehicles") and the owner and team rules. Each hit becomes an instant damage effect
    that passes through them. Beside the owner and team rules sits one rule the research does not list, for spawn
    protection (LOOP_V2 §2 `[DEFAULT]`; AB-6): a hit on a target that holds the immunity tag the pipeline's data names
    deals no damage, a blast included (unlike the frontal-arc block, it reads no arc). It is one reviewed Rust addition,
    like the knockback operation (AB-6). Proposed as a tag, not as a damage scale of 0 on an attribute, so that no
    modifier order (AB-1's aggregation) can undo it, and with the tag named once in the pipeline's data rather than as
    a target-tag multiplier in every weapon (two homes, CS3).
  - Why: the outcome logic GAS gives to execution code becomes fixed, reviewed stages, and each number has one home
    (CS3 shows the cost of two homes).
  - Research: R1-ABL-REC1 (lines 2350–2352), R1-ABL-REC2 (lines 2384–2385); R1-ABL-F9; the immunity rule: none (no
    research finding on spawn protection).
END

### FIX GD-7, part 2 of 3: the immunity rule's need (AB-2)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
- **AB-3. Weapons keep their own bookkeeping and share the damage pipeline (option B1).**
AFTER:
  - Need: VISION_V2 §6.3 ("combos and counters can emerge"); the immunity rule: LOOP_V2 §2 only.
END

### FIX GD-8 (my review's finding 18, the game-designer's half: the active-reload tap at the window's edge, AB-6's row)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  | Active reload (§7 `[DECIDED]`: "tap R at the right moment for a faster reload and a short damage boost") | the reload state machine's window (AB-3); a tap inside it shortens the reload and applies a timed effect on the weapon's damage attribute; the authority judges the tap from the command's tick (SLICE-ORDER, condition 2); the timing is own weapon state, inside the prediction scope; the boost is damage, which only the authority applies | none in legacy | the window, the faster reload and the boost's size and length (R1-ABL-OQ6) |
AFTER:
  | Active reload (§7 `[DECIDED]`: "tap R at the right moment for a faster reload and a short damage boost") | the reload state machine's window (AB-3); a tap inside it shortens the reload and applies a timed effect on the weapon's damage attribute; the authority judges the tap from the command's tick (SLICE-ORDER, condition 2), so near the window's edge the 33.3 ms tick (1000 ÷ 30) decides while the page draws the window every frame, and a tap the bar shows inside can be judged outside: whether the tap carries its moment inside a tick is asked beside shots (OQ-0001-7); the timing is own weapon state, inside the prediction scope; the boost is damage, which only the authority applies | none in legacy | the window, the faster reload and the boost's size and length (R1-ABL-OQ6) |
END

### FIX GD-7, part 3 of 3: spawn protection written in the model (a new AB-6 row after the Javelin's)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  | Javelin (§5 `[DECIDED]`: "strong against vehicles and aircraft, weak against infantry"; knockback `[DEFAULT]`; §7 `[DECIDED]`: "Targets react: flinch and knockback") | a projectile weapon; target-tag multipliers in the pipeline (above 1 against vehicle tags, below 1 against infantry); knockback as one new operation, a velocity change on the target, which the research reads as "a velocity change on the kinematic soldier" (R4-PHYS-REC3, ADR 0004); the flinch is presentation (the feel kit) | none (the Javelin is new; LOOP_V2 §11 leaves it out of the greybox v0) | the numbers |
AFTER:
  | Javelin (§5 `[DECIDED]`: "strong against vehicles and aircraft, weak against infantry"; knockback `[DEFAULT]`; §7 `[DECIDED]`: "Targets react: flinch and knockback") | a projectile weapon; target-tag multipliers in the pipeline (above 1 against vehicle tags, below 1 against infantry); knockback as one new operation, a velocity change on the target, which the research reads as "a velocity change on the kinematic soldier" (R4-PHYS-REC3, ADR 0004); the flinch is presentation (the feel kit) | none (the Javelin is new; LOOP_V2 §11 leaves it out of the greybox v0) | the numbers |
  | Spawn protection (§2 `[DEFAULT]`: "3 s of spawn protection"; ADR 0004 part 17 applies it at every spawn) | a timed effect on the new soldier that grants the immunity tag AB-2's immunity rule reads; that rule is one new pipeline rule (a reviewed Rust change, like the knockback); the effect's length is data, and its rows are among those the content bounds (AB-1, full pools) | 3 s (LOOP_V2 §2), 90 ticks at 30 Hz (3 × 30, arithmetic); none in legacy, whose respawn waits 2.0 s (LEGACY_INVENTORY.md, CD4, line 85) | what it covers beyond damage, and what ends it early (OQ-0005-AB14) |
END

### FIX GD-7 (the AB-6 need line gains LOOP_V2 §2)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  - Need: VISION_V2 §6.3, §6.4; the rows' rules: LOOP_V2 §4–§7.
AFTER:
  - Need: VISION_V2 §6.3, §6.4; the rows' rules: LOOP_V2 §2 (spawn protection) and §4–§7.
END

### FIX GD-8 (rust-reviewer's routed note: the offer's size and the open-offer queue are data in ADR 0004 part 18; AB-8's first row follows)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  | "Every unit and every weapon has its own roguelike upgrade track (pick one of three per level), owned by the player." | a track per unit type and per weapon type (AB-4), listing the candidates it can offer at each level; a candidate is an effect or a granted ability, with optional requirements (tags, the track's level); the offer size is a data value, 3, so no number is hard-coded | the player's record holds, per track, its level, its picks and any open offer | the pick command, and the visibility class of offers and track state (R2-INT-REC5's classes: public, team, owner, spatial) |
AFTER:
  | "Every unit and every weapon has its own roguelike upgrade track (pick one of three per level), owned by the player." | a track per unit type and per weapon type (AB-4), listing the candidates it can offer at each level; a candidate is an effect or a granted ability, with optional requirements (tags, the track's level); the offer size is a data value, 3, so no number is hard-coded | the player's record holds, per track, its level, its picks and its open offers, in a bounded queue whose length is data (ADR 0004 part 18; when offers appear, how they wait and what a full queue does are OQ-0005-AB8's) | the pick command, and the visibility class of offers and track state (R2-INT-REC5's classes: public, team, owner, spatial) |
END

### FIX GD-8 (my review's finding 15 and rust-reviewer's routed note: the record's key follows OQ-0004-10; AB-8's PlayerState row)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  | "In Unreal terms, the tracks live on the PlayerState, not the Pawn." | none | the record is per player slot, beside the soldier tables | none |
AFTER:
  | "In Unreal terms, the tracks live on the PlayerState, not the Pawn." | none | the record is per player slot, beside the soldier tables; whether a player who rejoins after the grace window gets it back, and so whether it is keyed by slot or by player, is OQ-0004-10 | none |
END

### FIX GD-8 (the hard-coded three, in AB-8's offer row; ADR 0004 part 8 already names the stream)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  | "pick one of three" (the offer) | the candidates a track offers at a level, and their requirements | the authority draws the three with a sim random stream; which stream is ADR 0004's (a [shared stream](../research/GLOSSARY.md#shared-stream-and-authority-only-stream) would let the owner's client foresee its next offers, the reasoning that kept weapon spread in an authority-only stream, SEC-SPREAD (a)) | a pick that was not offered, or for an offer that is no longer open, is refused |
AFTER:
  | "pick one of three" (the offer) | the candidates a track offers at a level, and their requirements | the authority draws the offer (its size from data, 3) from ADR 0004's authority-only offers stream (part 8): a [shared stream](../research/GLOSSARY.md#shared-stream-and-authority-only-stream) would let the owner's client foresee its next offers, the reasoning that kept weapon spread in an authority-only stream, SEC-SPREAD (a) | a pick that was not offered, or for an offer that is no longer open, is refused |
END

