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
    - Active effects the content bounds on their target (its own picks, its own abilities' and weapon's effects,
      spawn protection): the whole-set check reserves their worst case in every target's rows, so they never give way.
      Reason: their count follows from the content alone, and a pick or a pressed ability that silently fails to apply
      takes away what the player earned (pillar 2).
    - Active effects that other soldiers, vehicles or structures apply (one row per application under "no stacking", so
      play sets their count, not the content): they share the rest of the target's rows, up to a cap in data set once
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

### FIX GD-7 (rust-reviewer's routed note: does AB-1 have an operation for spawn protection?): the pipeline's immunity rule (AB-2)
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

### FIX GD-7: the immunity rule's need (AB-2)
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

### FIX GD-7: spawn protection written in the model (a new AB-6 row after the Javelin's)
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

### FIX GD-7 (the new OQ-0005-AB14) + GD-PICK-VIS (the new OQ-0005-AB15) + POOL-FULL (where the proposal sits): the open questions' opening lines
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
The first three are the ones Mustafa named; OQ-0005-AB4 to AB11 are what his rules and LOOP_V2 leave open beside each
other. None of AB1 to AB11 has a pick here: they are Mustafa's ("list them as open questions, don't decide them"), and
the data model carries every option listed.
AFTER:
The first three are the ones Mustafa named; OQ-0005-AB4 to AB11 and AB15 are what his rules and LOOP_V2 leave open
beside each other, and OQ-0005-AB14 is what LOOP_V2 §2's spawn protection leaves open. None of AB1 to AB11, AB14 or
AB15 has a pick here: they are Mustafa's (for the upgrade questions, his own words: "list them as open questions, don't
decide them"), and the data model carries every option listed, except where an option says it needs a reviewed Rust
addition. Which row gives way when a pool is full is ADR 0004's OQ-0004-11, with the game-designer's proposal per kind
in AB-1 for Mustafa to confirm.
END

### FIX GD-8 (rust-reviewer's routed note: OQ-0005-AB8 says what a full offer queue does)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  - Why it matters: a live fight has no pause. The open offer is state on the player's record (ADR 0004), the answer
    is a command the authority checks (ADR 0001), and ADR 0003 draws it.
  - Options: (a) at once, in play, answerable any time (unanswered offers wait, in a queue whose length is data); the
    fastest power, a distraction mid-fight. (b) Only at the buy terminal, where everyone spawns (LOOP_V2 §2); ties picks
    to a visit to the barracks. (c) While dead or respawning; gives the respawn wait a use, and picks lag behind levels.
    With each: are the three candidates always different, and what happens when a track has fewer than three left.
AFTER:
  - Why it matters: a live fight has no pause. The open offers are state on the player's record (ADR 0004 part 18: a
    bounded queue per track, its length and the offer's size both data), the answer is a command the authority checks
    (ADR 0001), and ADR 0003 draws it.
  - Options: (a) at once, in play, answerable any time (unanswered offers wait, in a queue whose length is data); the
    fastest power, a distraction mid-fight. (b) Only at the buy terminal, where everyone spawns (LOOP_V2 §2); ties picks
    to a visit to the barracks. (c) While dead or respawning; gives the respawn wait a use, and picks lag behind levels.
    With each: are the candidates always different, and what happens when a track has fewer left than the offer's size
    (3). Also with each, what a full queue does when the track levels again: (i) the level is kept and its offer drawn
    once a pick frees a place, so nothing earned is lost and the queue's length only limits what is shown; (ii) the
    oldest open offer gives way to the new one, so the player loses a choice they earned (pillar 2); (iii) the track
    stops earning until a pick, a push to pick and lost progress for a player who cannot stop. Under (b) and (c) the
    queue fills fastest, since offers wait for a visit or a death.
END

### FIX GD-8 (my review's finding 12, the game-designer's half, and netcode-architect's routed note from 75615d1: E for a pickup against E for a steal joins OQ-0005-AB9's list)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
    teams, not one); or anyone. Also Mustafa's, each a value in data: how long a pickup stays; whether a gun a player
    swaps out becomes a pickup that anyone can take; which of two claims on one pickup in the same tick wins (ADR 0004
    keeps the order deterministic whichever is chosen).
  - Decider: Mustafa, with the game-designer. What decides it: a design session, with OQ-0005-AB1. Research ids: none;
    whether a pickup is predicted is ADR 0001's (Quake III predicts item pickups: R2-SYNC-F17, r2-netcode.md, Findings,
    lines 230–236).
AFTER:
    teams, not one); or anyone. Also Mustafa's, each a value in data: how long a pickup stays; whether a gun a player
    swaps out becomes a pickup that anyone can take; which of two claims on one pickup in the same tick wins (ADR 0004
    keeps the order deterministic whichever is chosen); and which act a press means where a pickup lies within reach of
    an uplink or a cache, since a steal is holding E (LOOP_V2 §4 step 4), ADR 0001 part 12 has E place a cache too, and
    a pickup is asked for through the same button edges: the pickup wins, the steal or the placing wins, or a tap takes
    the pickup and a hold steals. A full pickup pool follows AB-1's proposal for OQ-0004-11.
  - Decider: Mustafa, with the game-designer. What decides it: a design session, with OQ-0005-AB1; the press precedence
    in the uplinks change-set's playtests, where fights cluster at uplinks and drops lie there. Research ids: none;
    whether a pickup is predicted is ADR 0001's (Quake III predicts item pickups: R2-SYNC-F17, r2-netcode.md, Findings,
    lines 230–236).
END

### FIX GD-8 (my review's findings 15 and 21, the game-designer's halves) + rust-reviewer's routed note: OQ-0005-AB10 cites OQ-0004-10 and OQ-0004-12
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
    empty; fair to the joiner, and the team loses the bot's picks at once. (c) Bots have no tracks; simplest, and teams
    with more bots are weaker, which matters in three-team modes with uneven bot counts. How a bot picks is bot
    behaviour (ADR 0004).
  - Decider: Mustafa. What decides it: a design session, then sims with mixed humans and bots. Research ids: none.
AFTER:
    empty; fair to the joiner, and the team loses the bot's picks at once. (c) Bots have no tracks; simplest, and teams
    with more bots are weaker, which matters in three-team modes with uneven bot counts. How a bot picks is bot
    behaviour (ADR 0004).
  - Asked beside it in ADR 0004, both Mustafa's: whether a player who rejoins after ADR 0001's grace window gets their
    record back, tracks and money included, or the slot's bot keeps it (OQ-0004-10, decided with this question; the
    record's key, slot or player, follows the answer); and whether bots take world pickups at all (OQ-0004-12), which
    changes what OQ-0005-AB9's drop rules do to balance when most slots are bots.
  - Decider: Mustafa. What decides it: a design session, with OQ-0004-10 and OQ-0004-12, then sims with mixed humans
    and bots (3 runs per build, two-team and three-team modes separately). Research ids: none.
END

### FIX GD-2's data half (netcode-architect's routed note from 75615d1: ADR 0001 part 12's Buy{unitId} is checked against rule data; AB-5 says where the data names what may be bought)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  - What: the team's tier is team state the sim keeps (ADR 0004). An ability requires its tier's tag, and a unit or
    vehicle is buyable at its tier (its tier and cost are unit data, AB-4). The T1 class abilities require Tech 1
    (LOOP_V2 §4, `[DECIDED]`); T2 and T3 units and vehicles require their tier (the `[DEFAULT]` ladder; LOOP_V2 §8: "the
    team's tech tier still decides when a unit or vehicle becomes buyable"). The tier thresholds and the minute-5 floor
    are tuning numbers in data, run by the economy and objective phases (ADR 0004). The authority refuses a press or a
    purchase before its tier; whether the client predicts the press is ABL-PREDICT.
  - Why: R1-ABL-REC1 names "team tech tiers" among the tag gates, and legacy checks no tier (GE4, GC11).
  - Research: R1-ABL-REC1 (lines 2343–2345).
AFTER:
  - What: the team's tier is team state the sim keeps (ADR 0004). An ability requires its tier's tag, and a unit or
    vehicle is buyable at its tier (its tier and cost are unit and vehicle data, AB-4). The T1 class abilities require
    Tech 1 (LOOP_V2 §4, `[DECIDED]`); T2 and T3 units and vehicles require their tier (the `[DEFAULT]` ladder; LOOP_V2
    §8: "the team's tech tier still decides when a unit or vehicle becomes buyable"). The tier thresholds and the
    minute-5 floor are tuning numbers in data, run by the economy and objective phases (ADR 0004). The authority refuses
    a press or a purchase before its tier; whether the client predicts the press is ABL-PREDICT.
  - What may be bought is data, and ADR 0001's `Buy{unitId}` (part 12) is checked against it: the faction definition
    lists the units and vehicles its team can buy (AB-4's faction kind; R1-ABL-REC5: "which units and vehicles"), the
    mode's rules table may change that per mode (R2-ROOM-REC7), and each unit and vehicle definition holds its tier and
    its price ($100 for each T1 class, LOOP_V2 §5 `[DECIDED]`). The rise in a team's prices after it loses a power plant
    (LOOP_V2 §3 `[DEFAULT]`, run by ADR 0004 part 17) and the buy terminal's reach are numbers in the same tables. The
    Recruit is free and comes with every death (LOOP_V2 §5 `[DEFAULT]`), so its price in data is 0. A purchase gives the
    unit's own weapons and nothing else (AB-8).
  - Why: R1-ABL-REC1 names "team tech tiers" among the tag gates, and legacy checks no tier (GE4, GC11).
  - Research: R1-ABL-REC1 (lines 2343–2345); R1-ABL-REC5 (lines 2448–2452) for the faction's list; R2-ROOM-REC7
    (r2-netcode.md, Recommendations, lines 2857–2867) for the mode's rules table.
  - Need: VISION_V2 §6.5; the faction's list: §6.7.
END

### FIX GD-PICK-VIS (my review's question; the coordinator's note: a number beside OQ-0005-AB7): AB7 points to the new OQ-0005-AB15
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  - Decider: Mustafa. What decides it: a design session, then the two measurements of OQ-0005-AB3. Research ids: none.
AFTER:
  - Decider: Mustafa. What decides it: a design session, then the two measurements of OQ-0005-AB3; who may see a
    player's picks is OQ-0005-AB15, decided with this question, since raw power that nobody can see has no answer.
    Research ids: none.
END

### FIX GD-7 (a new open question, OQ-0005-AB14, for what LOOP_V2 §2's spawn protection leaves open) + GD-PICK-VIS (a new open question, OQ-0005-AB15), added after OQ-0005-AB13
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
  - Proposed: (a), and (b) whenever a copy appears. Decider: the Advisor (the source). What decides it: the content and
    ability spike's marks 1 and 2 (the trigger). Research ids: R1-ABL-OQ2, R1-ABL-OQ9.
AFTER:
  - Proposed: (a), and (b) whenever a copy appears. Decider: the Advisor (the source). What decides it: the content and
    ability spike's marks 1 and 2 (the trigger). Research ids: R1-ABL-OQ2, R1-ABL-OQ9.
- **OQ-0005-AB14. What does spawn protection cover, and what ends it early?**
  - Why it matters: LOOP_V2 §2 (`[DEFAULT]`) gives "3 s of spawn protection" and no more. AB-2's immunity rule stops
    damage for the effect's whole length; whether it also stops other effects, and whether firing ends it, decides
    whether a fresh spawn is merely safe or can fight with no risk. On legacy numbers a Warrior empties its 36-round SMG
    magazine in 35 × 0.072 = 2.52 s, inside the 3 s, for up to 36 × 8 = 288 damage, nearly two Peacekeepers' worth
    (2 × 150 = 300) (LEGACY_TUNING.md, `WEAPONS` and `UNIT_TYPES`, lines 167–230; arithmetic), and nothing can answer
    it while it lasts (pillar 3). Barracks interiors are closed to enemies (LOOP_V2 §2), so this plays out at the
    openings and against blasts thrown in.
  - Options for what it covers: (a) damage only (AB-2's rule as written); (b) damage and every effect another team
    applies (slows, marks, knockback), which needs one more reviewed Rust check: an effect's application refused on a
    target holding the tag. Options for what ends it early: (i) nothing, the full 3 s; (ii) the player's first shot or
    ability press, which needs a trigger that removes the effect on the soldier's own fire, of the kind R1-ABL-REC1's
    examples name ("on kill by source, run effect E", line 2347); (iii) leaving the barracks interior, read from the
    interior's flag (ADR 0004 parts 11 and 17). The length stays data with every option.
  - Decider: Mustafa (the 3 s is the Advisor's `[DEFAULT]`; what it covers and what ends it are new rules). What
    decides it: a design session, then sims (3 runs per build, two-team and three-team modes separately) counting
    deaths before a soldier leaves its barracks and the damage protected soldiers deal, then playtests. Research ids:
    none (no research finding on spawn protection).
- **OQ-0005-AB15. Who sees a player's upgrade picks?** (GD-PICK-VIS)
  - Why it matters: ADR 0001 part 12 sends track state and picks to their owner only, as its starting class. Teammates
    plan around each other's power (pillar 4: "Teams must split up and talk"); winners should "feel strong and can see
    it" (pillar 2); and enemies need to read a strong player to answer it (pillar 3, "every strong thing has an
    answer").
  - Options: (a) the owner only (ADR 0001 today); (b) the team too; (c) everyone, through the scoreboard or a visible
    cue on the soldier. Each is one of R2-INT-REC5's visibility classes (r2-netcode.md, Recommendations, line 2502;
    ADR 0001's), so the answer changes a class in ADR 0001's state table, not the data model; a visible cue in (c) is
    also ADR 0003's to draw and LOOK_V2's to style.
  - Decider: Mustafa. What decides it: a design session, with OQ-0005-AB7 (if picks may add raw power, the answer
    pillar 3 needs is easier to give when enemies can see them), then playtests. Research ids: none.
END

### FIX TE-8 (ability item 1: the proof is behaviour; the id search is a smell check)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
1. **AB-1, AB-2 (R1-ABL-REC1's marks, lines 2364–2378, as written):** the test content (AMK, Bagpipe, Shabzon, Retro
   Lancer, Beam Katana, Torque Bow, frag grenade, Molotov, Riot Shield, Banzai, Bayonet, the Juggernaut cannon) runs
   with no ability-specific Rust code (zero matches for those ids in the sim crate's non-test code); a test-only
   combination (the Shield's block on a vehicle, the Banzai speed buff on another class) behaves as its parts, with
   zero Rust changes; the two §6.4 primitives work in a fixture; one number changed in data changes the next match
   with zero Rust changes.
AFTER:
1. **AB-1, AB-2 (R1-ABL-REC1's marks, lines 2364–2378; the first mark's proof proposed):** the test content (AMK,
   Bagpipe, Shabzon, Retro Lancer, Beam Katana, Torque Bow, frag grenade, Molotov, Riot Shield, Banzai, Bayonet, the
   Juggernaut cannon) runs with no ability-specific Rust code. The proof is behaviour, not a search: a test-only
   combination (the Shield's block on a vehicle, the Banzai speed buff on another class) behaves as its parts, with
   zero Rust changes, and a scripted scenario over the test content gives the same event log (ids mapped back) with
   every test id renamed and the definitions reordered in the data alone (proposed: a search for the ids, the
   research's measure, misses code keyed by an enum variant or by a numeric handle per ability; it stays as a smell
   check, zero matches for those ids in the sim crate's non-test code). The two §6.4 primitives work in a fixture; one
   number changed in data changes the next match with zero Rust changes.
END

### FIX TE-8 (u1, the same as mark 1)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
   - u1: no track-specific Rust code (zero matches for the test track ids in the sim crate's non-test code, as mark 1);
AFTER:
   - u1: no track-specific Rust code, proven as mark 1 is: u2 to u6 pass on test-only tracks written in data alone,
     and give the same logs with the test track ids renamed and the definitions reordered; zero matches for those ids
     in the sim crate's non-test code is the smell check;
END

### FIX GD-8 (my review's finding 8, the game-designer's half: the triangle's third leg) + GD-7 (the immunity rule's case), ability item 5
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
5. **AB-6, AB-7 (proposed golden cases):** from data alone, the CD13 matchups reproduce: the Warrior kills a Conscript
   in 0.86 s within one tick (R1-ABL-REC3's mark, lines 2416–2418, ADR 0004's); the Peacekeeper kills a Warrior in
   0.85 s when 11 of 16 pellets land; a Peacekeeper behind the Shield takes 63 SMG body hits. The knockback operation
   moves a target with no Javelin-specific code. Reason: these are the numbers the counter check uses (AB-7).
AFTER:
5. **AB-6, AB-7 (proposed golden cases):** from data alone, the CD13 matchups reproduce: the Warrior kills a Conscript
   in 0.86 s within one tick (R1-ABL-REC3's mark, lines 2416–2418, ADR 0004's); the Peacekeeper kills a Warrior in
   0.85 s when 11 of 16 pellets land; a Peacekeeper behind the Shield takes 63 SMG body hits. The triangle's third leg
   (the Conscript beats the Peacekeeper, AB-7): a Molotov blast deals a Peacekeeper the same damage with the Shield up
   as with it down (a blast skips the frontal block, GC7), and at 35 m a Bagpipe pellet deals 0 while an AMK body hit
   deals 34 (ranges 34 m and 300 m, AMK damage 34: LEGACY_TUNING.md, `WEAPONS`, lines 173 and 181). The knockback
   operation moves a target with no Javelin-specific code. AB-2's immunity rule: a soldier inside its spawn protection
   takes 0 damage from a body hit and from a blast, and full damage once the effect's 90 ticks end (3 × 30 at 30 Hz),
   with no protection-specific code; this holds under every option of OQ-0005-AB14. Reason: these are the numbers the
   counter check uses (AB-7), and spawn protection (LOOP_V2 §2) is the one pipeline rule the research does not list.
END

### FIX GD-8 (my review's finding 13, as it applies to my own u4: a lost gun, not a voluntary drop, OQ-0005-AB1 and AB9 open)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
   - u4, a lost gun: the player drops W and takes another W from the world, and the player's W picks apply to it; a
     second player who takes the first gun gets their own W picks and none of the first player's;
AFTER:
   - u4, a lost gun: the player holds a looted weapon of type X, loses it on death (dropped by the fixture's drop rule,
     a test rule while OQ-0005-AB9 is open), and takes another X the fixture places, and the player's X picks apply to
     it; a second player who takes the first gun gets their own X picks and none of the first player's;
END

### FIX GD-8 (the hard-coded three, in u6)
FILE: docs/adr/0005-content-abilities-assets.md
BEFORE:
   - u6, offers: the authority draws three candidates; a pick that was not offered, or for a closed offer, is refused in
     100 % of 1,000 forged picks; the same seed and the same inputs give byte-identical offer logs on two runs (and on
     native and Wasm wherever the stream is one the client holds, ADR 0004).
AFTER:
   - u6, offers: the authority draws an offer of the data's size (3); a pick that was not offered, or for a closed
     offer, is refused in 100 % of 1,000 forged picks; the same seed and the same inputs give byte-identical offer logs
     on two runs, and on native and Wasm: the offers stream is authority-only (ADR 0004 part 8), but in local play the
     authority is the local server in the browser's Web Worker (ADR 0002, Decision).
END

### NOTE RR-4 (the 0005 half) + POOL-FULL
Fixed in AB-1 ("When a pool is full"): the effect pool's overflow points to OQ-0004-11 (no second question), with the
game-designer's proposal per kind for Mustafa to confirm where a player could notice: active effects the content bounds
on their target (own picks, own abilities and weapon, spawn protection) are reserved and never give way; active effects
others apply share the rest up to a cap in data and the oldest gives way (b), because refusing (a) would make a
saturated target immune to new slows and marks (pillar 3); live projectiles are sized to the content's worst case
(legacy Molotov: floor(1.4 / 1.05) + 1 = 2 per thrower, 96 at N = 48), so (a) is only a bug net; pickups (b), the
oldest gives way, because drops are the only source of other weapons (UPGRADE-SCOPE). Routed:
- ADR 0004 OQ-0004-11 (rust-reviewer): "What decides it" may add "ADR 0005 AB-1 holds the game-designer's proposal per
  kind"; part 1's "until it is answered the spikes refuse the new row as a test fixture" stays true.
- ADR 0005 CD-2 (engine-architect): the whole-set check reports, per target, the reserved worst case of content-bounded
  effects (picks per track level for the tracks that apply at once, the unit's own ability and weapon effects, spawn
  protection), and names the missing cap for effects from others and for pickups.
- PM-0004-6 and the PM-0005 rows (test-engineer): once Mustafa confirms, one full-pool row per kind (own effects never
  refused at a full pool; the oldest other-source row ends early; the projectile overflow counter stays 0 at the
  content's worst case; the oldest pickup ends early).
For "Decisions needed" (POOL-FULL, Mustafa confirms): the recommendation is the proposal as written in AB-1.
END

### NOTE TE-8
Fixed in ability item 1 and u1: the proof is behaviour (the test-only combination with zero Rust changes, and the same
event log with every test id renamed and the definitions reordered in data alone); the id search stays as a smell
check. Routed: PM-0005-18 (test-engineer) already has the rename test; it may also reorder the definitions, so a
constant keyed on a handle bites whatever order handles are given in; PM-0005-26 follows for the test tracks.
END

### NOTE GD-7 (spawn protection: rust-reviewer's routed check)
Checked: AB-1's closed set did not hold it. No AB-2 stage reads a target-side immunity (the target-tag multiplier is
set per weapon, and the frontal-arc block skips blasts), so spawn protection needs one reviewed Rust addition: AB-2's
immunity rule (a tag named once in the pipeline's data; a hit on a target holding it deals no damage, blasts included).
Fixed in AB-2, AB-6 (a new row: a timed effect, 3 s = 90 ticks at 30 Hz), ability item 5 (its golden case), and a new
open question, OQ-0005-AB14 (what it covers beyond damage, what ends it early; Mustafa). Routed:
- ADR 0004 part 17 (rust-reviewer): "an effect in ADR 0005's data" may read "an effect in ADR 0005's data (AB-6, read by
  AB-2's immunity rule; its scope is OQ-0005-AB14)".
- docs/adr/README.md (engine-architect): the "Needs outside VISION_V2 §6" list gains "ADR 0005: AB-2's immunity rule for
  spawn protection (LOOP_V2 §2 only)"; register rows for OQ-0005-AB14 (Mustafa) and OQ-0005-AB15 (Mustafa); ADR 0005's
  row count +2 (AB1–AB15). ADR 0005's frame "Open questions" table (engine-architect) gains the same two rows after
  OQ-0005-AB13, with "The ability and effect system" as their section.
- PM-0005-25 (test-engineer): the immunity case as ability item 5 states it (0 damage from a body hit and a blast inside
  the 90 ticks, full damage after, no protection-specific code).
For "Decisions needed" (OQ-0005-AB14, Mustafa): recommended (a) damage only, with (ii) ending at the player's first shot
or ability press. Reason: on legacy numbers a protected Warrior can empty its 36-round SMG (35 × 0.072 = 2.52 s, inside
3 s; 36 × 8 = 288 damage) with no answer possible (pillar 3); ending at the first shot keeps protection a safety feature,
not a weapon; (ii) costs one trigger, the same kind as R1-ABL-REC1's "on kill by source, run effect E". Then sims, 3 runs
per build, two-team and three-team modes separately.
END

### NOTE GD-PICK-VIS (the coordinator's note)
Given a number: OQ-0005-AB15 (who sees a player's upgrade picks; Mustafa), after OQ-0005-AB14, with OQ-0005-AB7's decider
line pointing to it. Routed: ADR 0001 part 12 (netcode-architect): "who else sees picks is GD-PICK-VIS (Mustafa's)" reads
"who else sees picks is ADR 0005's OQ-0005-AB15 (GD-PICK-VIS, Mustafa's)". For "Decisions needed": recommended (a) the
owner only to start, decided with OQ-0005-AB7: if picks may add raw power, (c) everyone is the answer pillar 3 needs.
END

### NOTE GD-2 (the purchase as data; netcode-architect's routed note from 75615d1)
Fixed in AB-5: what may be bought is the faction's list (AB-4's faction kind), changed per mode by its rules table
(R2-ROOM-REC7); each unit and vehicle holds its tier and price; the power-plant price rise and the buy terminal's reach
are numbers in the same tables; the Recruit's price is 0. Routed: ADR 0001 part 12 and the `Buy` row of its message
catalogue (netcode-architect) may cite "ADR 0005 AB-5" for "the conditions are rule data (ADR 0005)". A low note on the
same paragraph: "(E also steals and places, LOOP_V2 §4)" cites LOOP_V2 §4 for placing with E, but LOOP_V2 names E only
for steals (step 4: "holding E"); step 3 names no key for placing. Suggested: "(a steal is holding E, LOOP_V2 §4 step 4,
and placing a cache is a press too)".
END

### NOTE GD-8 (the lows of my review that name the game-designer, and rust-reviewer's routed AB8 and AB10 notes)
Fixed: OQ-0005-AB8 lists what a full offer queue does ((i) to (iii), no pick) and AB-8's rows read the queue and the
offer's size as data (ADR 0004 part 18) and name ADR 0004's authority-only offers stream (part 8); OQ-0005-AB9 lists
the E-press precedence (finding 12; netcode-architect's routed note) and points a full pickup pool to AB-1; OQ-0005-AB10
cites OQ-0004-10 and OQ-0004-12 (findings 15 and 21) and AB-8's PlayerState row names OQ-0004-10 as the key's owner;
AB-6's active-reload row points to OQ-0001-7, which now names the tap (finding 18, 75615d1); ability item 5 gains the
triangle's third leg (finding 8: a Molotov blast through the Shield; a Bagpipe pellet at 35 m deals 0, an AMK hit 34);
u4 tests a lost looted gun, not a voluntary drop (finding 13, as it applies to my own text); u6 draws an offer of the
data's size and asks for equal offer logs on native and Wasm, since in local play the authority runs in the Worker.
Routed (test-engineer): PM-0005-25 adds the third leg; PM-0005-29 follows u4; PM-0005-31 reads "an offer of the data's
size" and asks for equal offer logs on native and Wasm, not "only if ADR 0004 puts the offers in a stream the client
holds" (the offers stream is authority-only, but the local server draws it in Wasm).
Not changed: AB-8's "the authority draws ... with a sim random stream" wording beyond the stream's name, and the "three"
in Mustafa's own quoted words.
END

### NOTE checks (done before handing back)
All 21 FIX blocks apply in order to ADR 0005 at b88714d (scripts/apply_fixes.py on a scratch copy: all applied; each
BEFORE once, whole lines, copied by script). cite_check.py: the same 2 pre-existing range flags before and after
(R1-ABL-OQ1 at the method note, R1-ABL-REC6 beside the glossary lines), 0 new; ids cited 471 -> 480. Glossary check on
the copy: exact 0 -> 0; in context 77 -> 85 (active effects, vehicles, "2 × 48 = 96 at N = 48"). No sentence sits
directly under a table. Not checked: the other authors' texts the routes name, as they stand after their own batches.
END
