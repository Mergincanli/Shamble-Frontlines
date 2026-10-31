## The ability and effect system (game-designer)

This section decides how abilities, effects and weapons are written as data, and what the engine runs for them: a
reduced version of Unreal's Gameplay Ability System (GAS), written as data over a closed set of operations in Rust.
Weapons keep their own bookkeeping, and every hit goes through one damage pipeline. The section shows how LOOP_V2's
abilities, the team's tech tiers and the counters ride on that model. It also shows how the model carries Mustafa's
in-match upgrade tracks (UPGRADE-SCOPE) without designing any of them. It designs no upgrade, ability, unit or rule:
those are Mustafa's. Every question his rules leave open is listed for him, with options and no pick.

### Context (this section)

Short citations: r1, r2 and r4 are `docs/research/r1-simulation-core.md`, `r2-netcode.md` and
`r4-physics-navigation.md`; r1:1373–1376 are r1's Findings §5, "Legacy facts used (unit `abilities`)"; lines are at
9e7db28.

**What it serves.**
- VISION_V2 §6.3: "A data-driven ability and effect system, like UE's Gameplay Ability System. Units, vehicles, weapons
  and abilities are built from data, so combos and counters can emerge and tuning needs no code changes."
- VISION_V2 §6.4 (vehicles with their own weapons and abilities: "leech, marking"; seats `[OPEN]`), §6.5 (economy and
  objectives server-side and tunable: the tech gates), §6.6 (balance telemetry: pick and win rates), §6.7 (faction
  content as data).
- VISION_V2 §2, pillar 3 ("every strong thing has an answer") and pillar 4 ("Every choice costs something"). Pillars 1
  and 2 pull against each other on purpose, and their mechanics are `[OPEN]` (VISION_V2 §2, §7).
- LOOP_V2 §4 (class abilities locked until Tech 1, with a minute-5 floor, `[DECIDED]`; the tier ladder `[DEFAULT]`),
  §5 (the T1 classes and their abilities `[DECIDED]`; the counter triangle `[DEFAULT]`; the Javelin `[DECIDED]`, its
  knockback `[DEFAULT]`; the Spy `[DECIDED]` `[LATER]`, its cost and rules `[OPEN]`), §6 (the Juggernaut `[DECIDED]`;
  the Hammer's leech `[DECIDED]` `[LATER]`), §7 (the feel kit, split feedback, flinch and knockback, active reload, the
  AMK's rhythm: all `[DECIDED]`), §8 (the three unlock gates `[DECIDED]`; the catalogue `[LATER]`), and its header: every
  number is a starting value, kept in one data table (LOOP_V2 line 8).

**Settled inputs.**
- UPGRADE-SCOPE (Mustafa, 2026-10-31), his words:
  > - Every unit and every weapon has its own roguelike upgrade track (pick one of three per level), owned by the player.
  > - Tracks survive death and switching (switch back and the track is still there). They reset at the start of every match.
  > - A class always comes with its own weapon: no weapon choice when buying.
  > - Other weapons come only from killed enemies' drops; dropped weapons are world pickups.
  > - Progress belongs to the weapon type, not the physical gun: when a gun is lost, its track waits for the next gun of that type.
  > - In Unreal terms, the tracks live on the PlayerState, not the Pawn.
  >
  > Still open; list them as open questions, don't decide them: carry slots, ammo for looted weapons, and how fast tracks level (including catch-up when switching).
- ABL-PREDICT: open, Mustafa decides after the Feel Lab test. SEC-SPREAD (a): weapon spread stays in an
  [authority-only stream](../research/GLOSSARY.md#shared-stream-and-authority-only-stream). MODES-3 (a):
  [the largest mode](../research/GLOSSARY.md#the-largest-mode-and-its-n) is 24v24, N = 48. TICK-30: a 30 Hz server
  tick is the baseline. SLICE-ORDER, condition 2: movement and shooting are input-driven and replayable from day one.
  SPIKE-SCOPE (a): spikes are throwaway.

**The findings it turns on.**
- R1 recommends a reduced GAS subset as data over a closed set of Rust operations (option A2), weapons with their own
  bookkeeping on a shared damage pipeline (option B1), and one table per kind linked by validated ids (option E1)
  (r1-simulation-core.md, Recommendations §5, R1-ABL-REC1, REC2, REC5, lines 2342–2463).
- Every GAS fact R1 uses comes from one community write-up (tranek's GASDocumentation, current with UE 5.3); Epic's
  own documentation and the engine source were not read (r1-simulation-core.md, Findings §5, method note, lines
  1185–1189; R1-ABL-OQ1, lines 2756–2758).
- Weapons: Fortnite "does its own bookkeeping for weapon firing rates"; with cooldowns as effects that the server
  overwrites, Paragon's "Higher latency connections would have a lower ROF with basic attacks" (r1-simulation-core.md,
  Findings §5, R1-ABL-F17, lines 1286–1295). Quake III's weapon timer is an integer counter that carries its overshoot
  into the next shot (R1-ABL-F22, lines 1322–1330).
- Upgrade tracks: the research has no finding on them (no topic file mentions a roguelike track, an offer of three or
  in-match upgrades; its "playerstate" hits are Quake III's player state, r1:1320, r2:199–203). What this section says
  about tracks rests on UPGRADE-SCOPE and on R1-ABL-REC1's vocabulary only. Anything more is an open question or a
  proposed spike mark (plan rule 2).

**What it takes from and gives to the other drafts.**
- ADR 0004 executes it: the closed set of operations and the damage pipeline inside
  [the sim step](../research/GLOSSARY.md#sim-step-time); the one time representation (integer time with carried
  remainders, R1-ABL-REC3); attribute maths (R1-ABL-OQ5); effects as rows or as entities (R1-ABL-OQ10); the 1 ms
  [p99](../research/GLOSSARY.md#time-statistics) part of
  [the sim step share](../research/GLOSSARY.md#the-sim-step-share-and-the-server-only-share) (R1-ABL-REC6). ADR 0004
  also decides the player record that holds the tracks, the random stream that draws the offers of three, and weapon
  drops as world pickup entities.
- ADR 0001 decides [the prediction scope](../research/GLOSSARY.md#the-prediction-scope) (R1-ABL-REC4, ABL-PREDICT),
  who sees which tags (R1-ABL-OQ8), and [the authority](../research/GLOSSARY.md#the-authority)'s checks, the messages
  and the visibility classes for offers, picks, drops and pickups.
- ADR 0006 decides the match record's fields for picks (it owns R6-TEL-REC1). ADR 0003 draws the cues and the HUD,
  offers included.
- This record's content data section decides the file format, the schema, validation, hot reload (R1-DATA) and the
  feel kit's format.

**What the legacy game proves** (LEGACY_INVENTORY.md, by row).
- Weapons and classes are already data: seven weapons in one table, with special-case fields for falloff, burst, melee
  and charge (CD12, line 93); four classes with HP 100 / 150 / 120 / 110 and their abilities (CD13, line 94); the
  Juggernaut (CD14, line 95). Their faults are what v2 fixes: bot and colour fields mixed into unit data (CD13), the
  body-part multipliers in two places (CS3, line 171), a missing gun key that silently falls back to the AMK (CC9,
  line 138).
- One damage path with a frontal shield block that skips blasts (CD8, line 89; GC7, line 246); four class abilities
  in code, none of which checks a tech tier (GC11, line 250; GE4, line 290); a purchase status enum (GE13, line 299);
  fire cooldowns that are reset, not carried (GC5, line 244).
- No live in-match weapon progression exists to port: legacy's gun-game ladder (`RIFLE_TIERS`) is dead code, marked
  drop (GE17, line 303).

### Decision (this section's parts)

Abilities, effects and weapons are data over a closed set of Rust operations: a reduced GAS subset. Weapons keep their
own bookkeeping and send every hit through one damage pipeline. Definitions sit in one table per kind, linked by
validated ids, with an upgrade-track kind added for UPGRADE-SCOPE. The team's tech tier gates abilities and purchases as
tags. Mustafa's upgrade tracks ride on the same model: a track is data; the player's progress lives on the player's
record (ADR 0004), not on the soldier; a pick becomes effects or granted abilities on the player's current unit and
weapon.

- **AB-1. The model: a reduced GAS subset, as data, over a closed set of Rust operations (option A2).**
  - What: attributes with a base value and a current value; effects that are instant, timed or permanent, optionally
    periodic; counted hierarchical tags; abilities gated by required and blocked tags, with a cost and a cooldown;
    effects that grant tags or abilities for their lifetime; cues as presentation events only. Content chooses and
    parameterizes operations. It cannot run arbitrary code: every new operation is a reviewed Rust change. The
    aggregation order is fixed and documented per attribute (GAS's default sums its multipliers, a rule designers must
    know). Stacking is an explicit policy per effect, from a short list: no stacking, refresh the duration, add a stack
    up to a limit, strongest only. Left out: what exists for Unreal's object and replication model (instancing
    policies, replication modes, ability tasks as a framework), and anything no legacy or LOOP_V2 test case needs.
  - Why: §6.3 asks for GAS-like data, and the subset covers every legacy ability and weapon and the primitives §6.4
    names, with no generic-engine feature.
  - Research: R1-ABL-REC1 (r1-simulation-core.md, Recommendations §5, lines 2342–2363), resting on R1-ABL-F1–F11, F16,
    F19.
  - Need: VISION_V2 §6.3, §6.4, §6.5, §6.7.
- **AB-2. One damage pipeline: fixed Rust stages, parameterized by data.**
  - What: the stages are the locational multiplier, range falloff, the frontal-arc block, a target-tag multiplier (for
    counters such as "strong against vehicles") and the owner and team rules. Each hit becomes an instant damage effect
    that passes through them.
  - Why: the outcome logic GAS gives to execution code becomes fixed, reviewed stages, and each number has one home
    (CS3 shows the cost of two homes).
  - Research: R1-ABL-REC1 (lines 2350–2352), R1-ABL-REC2 (lines 2384–2385); R1-ABL-F9.
  - Need: VISION_V2 §6.3 ("combos and counters can emerge").
- **AB-3. Weapons keep their own bookkeeping and share the damage pipeline (option B1).**
  - What: a weapon definition holds the sim fields only (fire interval, fire mode, magazine, reserve, pellets, the
    spread fields, range, falloff, damage, reload, charge curve, projectile kind). The weapon's state machine is
    per-soldier sim state: timer, ammo, burst count, charge, reload and the active-reload window. The feel kit is a
    separate client definition keyed by the same id. Abilities may grant or swap weapons (the Banzai blade equips the
    Katana; the Molotov is a thrown weapon) and apply effects to weapon attributes (active reload's short damage
    boost). One weapon model for bots and humans fits this shape; whether v2 keeps it is Mustafa's (R1-ABL-OQ6, a
    design question).
  - Why: with cooldowns as effects that the server overwrites, high-latency players fire more slowly (F17); a shipped
    shooter runs an integer weapon timer inside the player code both sides run (F21, F22).
  - Research: R1-ABL-REC2 (lines 2380–2395).
  - Need: VISION_V2 §6.3 (weapons built from data), §6.4 (vehicle weapons on mounts); active reload itself: LOOP_V2 §7
    only (ADR 0004 executes it).
- **AB-4. One table per kind, linked by validated ids (option E1), with two kinds added for UPGRADE-SCOPE.**
  - What: the kinds are the tag registry; an attribute set per unit kind; effect; ability; weapon (sim); feel kit
    (client only); unit (a class: attributes, weapons, abilities, tier, cost, tags); vehicle (parts, each with HP and
    tags; weapon mounts as fixed points; seats, their count left open by VISION_V2 §6.4 `[OPEN]`); faction. String ids
    resolve to dense handles at load; every reference, tag and number range is checked; a failure names the file and
    the field; nothing defaults silently.
  - Added for UPGRADE-SCOPE: an **upgrade track** kind (what a track can offer at each level, as references to effects
    and abilities), with every unit and weapon definition naming its track; and the **drop and pickup values** that
    ADR 0004's pickup entities read (their values are Mustafa's, OQ-0005-AB9). The unit kind names the class's own
    weapon or weapons, and a purchase gives exactly those ("A class always comes with its own weapon: no weapon choice
    when buying").
  - Not here: the file format, the schema tooling, how definitions sit in files and hot reload are the content data
    section's (R1-DATA; R1-XU-OQ9).
  - Why: the legacy tables already have this shape, and their faults are mixing, duplication and silent fallbacks
    (CD13, CS3, CC9).
  - Research: R1-ABL-REC5 (lines 2448–2459).
  - Need: VISION_V2 §6.3, §6.4, §6.6 (ids key the telemetry), §6.7; the track kind and the drop and pickup values: need
    UPGRADE-SCOPE.
- **AB-5. Tech tiers gate abilities and purchases as tags.**
  - What: the team's tier is team state the sim keeps (ADR 0004). An ability requires its tier's tag, and a unit or
    vehicle is buyable at its tier (its tier and cost are unit data, AB-4). The T1 class abilities require Tech 1
    (LOOP_V2 §4, `[DECIDED]`); T2 and T3 units and vehicles require their tier (the `[DEFAULT]` ladder; LOOP_V2 §8: "the
    team's tech tier still decides when a unit or vehicle becomes buyable"). The tier thresholds and the minute-5 floor
    are tuning numbers in data, run by the economy and objective phases (ADR 0004). The authority refuses a press or a
    purchase before its tier; whether the client predicts the press is ABL-PREDICT.
  - Why: R1-ABL-REC1 names "team tech tiers" among the tag gates, and legacy checks no tier (GE4, GC11).
  - Research: R1-ABL-REC1 (lines 2343–2345).
  - Need: VISION_V2 §6.5.
- **AB-6. LOOP_V2's abilities, written in the model.** Each row uses AB-1 to AB-5; a row that needs a new operation
  says so. The legacy numbers are starting values (LEGACY_TUNING.md; LOOP_V2 line 8), never decisions.

  | Item (LOOP_V2 §, tag) | Written as | Starting values (legacy) | Stays Mustafa's |
  |---|---|---|---|
  | Molotov (§5 `[DECIDED]`; locked until Tech 1) | the Conscript's ability: a thrown weapon (AB-3) whose projectile is swept per tick (R4-PHYS-REC3, ADR 0004) and ends in an area blast through the pipeline (radius, falloff, owner and team rules; a blast skips the frontal block, as in legacy) | fuse 1.4 s, building ×1.8; the shared blast is 140 damage in 6.5 m with linear falloff, owner ×0.5 (r1:1373, r1:1375; GC14); no fire area (GC11) | whether it leaves a burning area: legacy has none and LOOP_V2 is silent; an area that lingers would be a new operation |
  | Riot Shield (§5 `[DECIDED]`; Tech 1) | a toggled ability that applies a permanent effect while it is on: a granted tag and a movement modifier; the pipeline's frontal-arc stage reads the tag | block 70 % within arc cos 0.30, move ×0.72 (LEGACY_TUNING.md, `T`, lines 132–135) | how the block meets a one-hit kill (OQ-0005-AB11) |
  | Banzai blade (§5 `[DECIDED]`: "one-hit kill while active"; Tech 1) | a timed ability: a timed effect that grants the melee weapon and a speed modifier, with a cooldown; the kill is the weapon's damage through the pipeline | Katana for 10 s, 45 s cooldown, speed ×1.3 (r1:1373); Katana damage 9999, arc 120°, impact 0.13 s, swing 0.4 s, range 3.6 m (LEGACY_TUNING.md, `WEAPONS`, lines 167–203) | its answer (OQ-0005-AB11); whether melee is judged at the attacker's view time is ADR 0001's (R2-SYNC-OQ9) |
  | Active reload (§7 `[DECIDED]`: "tap R at the right moment for a faster reload and a short damage boost") | the reload state machine's window (AB-3); a tap inside it shortens the reload and applies a timed effect on the weapon's damage attribute; the authority judges the tap from the command's tick (SLICE-ORDER, condition 2); the timing is own weapon state, inside the prediction scope; the boost is damage, which only the authority applies | none in legacy | the window, the faster reload and the boost's size and length (R1-ABL-OQ6) |
  | The AMK's rhythm (§7 `[DECIDED]`: "a small magazine, the active reload, or both") | the magazine is a weapon field and active reload is the row above, so each of the three fixes is a data change | magazine 1, reload 3.5 s, interval 0.2 s (`WEAPONS`) | which fix |
  | Javelin (§5 `[DECIDED]`: "strong against vehicles and aircraft, weak against infantry"; knockback `[DEFAULT]`; §7 `[DECIDED]`: "Targets react: flinch and knockback") | a projectile weapon; target-tag multipliers in the pipeline (above 1 against vehicle tags, below 1 against infantry); knockback as one new operation, a velocity change on the target, which the research reads as "a velocity change on the kinematic soldier" (R4-PHYS-REC3, ADR 0004); the flinch is presentation (the feel kit) | none (the Javelin is new; LOOP_V2 §11 leaves it out of the greybox v0) | the numbers |
  | The Hammer's leech (§6 `[DECIDED]` `[LATER]`) | not built; kept possible by two primitives VISION_V2 §6.4 needs anyway: an effect removed when its source dies, and a trigger on the last hit or tick that moves the victim's item to the source (R1-ABL-REC1 mark 3, lines 2372–2376) | none | the item and the leech rules |
  | Marking (VISION_V2 §6.4) | an effect that grants a tag (the research's reading, R1-ABL-REC1, lines 2359–2361); who sees the tag is ADR 0001's interest management (R1-ABL-OQ8) | none | what marking does (R1-ABL-OQ6) |
  | The Spy's disguise and hack (§5 `[DECIDED]` `[LATER]`; cost, what breaks the disguise, hack length, the amount stolen and the warnings `[OPEN]`) | not built; kept possible: the disguise as a tag-granting effect whose shown identity differs per team (an enemy client receives the disguise as the identity it sees: R2-INT-REC5, ADR 0001); the hack as a timed ability with cancel tags; the cost as an instant effect on a named attribute, so the player's money and the team's tech both stay possible | none | all of it (R2-INT-OQ8, a design question) |
  | Legacy Bayonet, Torque Bow, health regen (not in LOOP_V2) | test cases only (R1-ABL-REC1 mark 1) | Bayonet 1.3 s charge, kills within 2.4 m, 10 s cooldown (r1:1373); Torque Bow fuse 0.55 s (r1:1376); regen after 3.5 s at 16 HP/s (LEGACY_TUNING.md, `T`, lines 126–127) | whether each stays (R1-ABL-OQ6) |

  - Research: R1-ABL-REC1, REC2; R4-PHYS-REC3 (r4-physics-navigation.md, Recommendations, lines 747–751).
  - Need: VISION_V2 §6.3, §6.4; the rows' rules: LOOP_V2 §4–§7.
- **AB-7. Combos and counters as data (pillar 3).** Any effect, tag or weapon can be put on any unit or vehicle by data
  alone (R1-ABL-REC1 mark 2, lines 2369–2371). The ids of AB-4 key the telemetry: every kill names its source weapon or
  ability id (R6-TEL-REC1, ADR 0006's), so counters get tuned on evidence (VISION_V2 §6.6). The counters LOOP_V2 states,
  and how the data carries them:

  | Strong thing | Its answer (LOOP_V2) | Carried by | Legacy numbers (starting values) |
  |---|---|---|---|
  | Imperial Warrior (SMG, strong up close) | the Peacekeeper (the triangle, §5 `[DEFAULT]`) | weapon numbers; the Riot Shield's frontal-arc stage | the Peacekeeper kills a Warrior in 0.85 s only if 11 of its 16 pellets land, else 3.45 s, against the Warrior's 1.30 s, so that side "leans on the Riot Shield", locked until Tech 1 (CD13, line 94); with the Shield up, an SMG hit deals 30 %: 63 hits to kill instead of 19 (arithmetic below) |
  | The Banzai blade (Tech 1) | not stated | the frontal-arc stage and target-tag rules can carry any answer | a blocked hit keeps 30 %, so the blade's 9999 still kills through the Shield: 9999 × 0.3 = 2999.7, above 150 HP (shamble-game.js:1514–1515; OQ-0005-AB11) |
  | Conscript (AMK) | the Warrior | weapon numbers | the Warrior kills a Conscript in 0.86 s, 10.5 s the other way (CD13; R1's verifier row R1-ABL-N6, r1:3487) |
  | Peacekeeper (shotgun, Shield) | the Conscript | weapon range; a blast skips the frontal block | the Conscript wins only beyond the Bagpipe's 34 m (CD13); the Molotov's blast is not blocked by the Shield (GC7: the block is "not to blasts") |
  | The Juggernaut and other vehicles | the Javelin, "strong against vehicles and aircraft" (§5 `[DECIDED]`) | a target-tag multiplier (R1-ABL-REC1, line 2351) | none: legacy never reduces the Juggernaut's 1500 HP (GC17, line 256) |
  | The Javelin | infantry ("weak against infantry", §5) | a target-tag multiplier below 1 | none |
  | The Hammer's leech, the Spy (`[LATER]`) | not designed | primitives kept possible (AB-6) | none |
  | A player with many picks (UPGRADE-SCOPE) | not designed: track content and levelling are Mustafa's (OQ-0005-AB3, AB7) | picks are effects, so any answer is data too | none |

  - The arithmetic (legacy table values: SMG damage 8, Shield block 0.7, Peacekeeper HP 150, Katana damage 9999):
    a blocked SMG hit deals 8 × (1 − 0.7) = 2.4, so ceil(150 ÷ 2.4) = ceil(62.5) = 63 hits; unblocked, ceil(150 ÷ 8) =
    ceil(18.75) = 19 hits; a blocked blade hit deals 9999 × (1 − 0.7) = 2999.7. These are sums on starting values, not
    balance evidence: CD13 itself asks for sims of each matchup (3 or more runs) before the values are kept.
  - Research: R1-ABL-REC1 (lines 2350–2352, 2369–2371), R1-ABL-REC5 (line 2459).
  - Need: VISION_V2 §6.3, §6.6; pillar 3 (VISION_V2 §2).
- **AB-8. Mustafa's upgrade tracks ride on the model (UPGRADE-SCOPE).** Each clause of his words, and who carries it.
  The rules, the track content and every number are his; this section adds only the track kind (AB-4) and its load
  checks, and no new operation: a pick is an effect or a granted ability (AB-1).

  | Mustafa's words | Data (this section) | State and execution (ADR 0004 decides) | Authority, messages, visibility (ADR 0001 decides) |
  |---|---|---|---|
  | "Every unit and every weapon has its own roguelike upgrade track (pick one of three per level), owned by the player." | a track per unit type and per weapon type (AB-4), listing the candidates it can offer at each level; a candidate is an effect or a granted ability, with optional requirements (tags, the track's level); the offer size is a data value, 3, so no number is hard-coded | the player's record holds, per track, its level, its picks and any open offer | the pick command, and the visibility class of offers and track state (R2-INT-REC5's classes: public, team, owner, spatial) |
  | "Tracks survive death and switching (switch back and the track is still there)." | nothing changes in the definitions | the record, not the soldier, holds every track; on each spawn (as a Recruit, or after a purchase) the picks of that unit type's track apply to the new unit | respawns and purchases are commands the authority checks |
  | "They reset at the start of every match." | nothing is saved: tracks need no persistence (accounts are `[LATER]`, VISION_V2 §6.10) | the record starts empty at every match start (with ADR 0001's room lifecycle) | none |
  | "A class always comes with its own weapon: no weapon choice when buying." | the unit definition names its weapon or weapons (AB-4) | a purchase gives exactly those | the purchase names a unit, never a weapon |
  | "Other weapons come only from killed enemies' drops; dropped weapons are world pickups." | the drop and pickup values (AB-4; OQ-0005-AB9) | a drop is a world pickup entity | the check of who may take what; a pickup the authority did not place is refused; the drops' visibility class |
  | "Progress belongs to the weapon type, not the physical gun: when a gun is lost, its track waits for the next gun of that type." | the weapon definition names its track (AB-4) | the picks of a weapon type's track apply to whichever gun of that type the player holds and leave the gun when the player loses it; a world pickup carries none of its last holder's picks; the track keeps its state while no gun of that type is held | none |
  | "In Unreal terms, the tracks live on the PlayerState, not the Pawn." | none | the record is per player slot, beside the soldier tables | none |
  | "pick one of three" (the offer) | the candidates a track offers at a level, and their requirements | the authority draws the three with a sim random stream; which stream is ADR 0004's (a [shared stream](../research/GLOSSARY.md#shared-stream-and-authority-only-stream) would let the owner's client foresee its next offers, the reasoning that kept weapon spread in an authority-only stream, SEC-SPREAD (a)) | a pick that was not offered, or for an offer that is no longer open, is refused |

  - Telemetry: which pick and track fields the match record holds is ADR 0006's (its record shape, R6-TEL-REC1); they
    are keyed by AB-4's ids.
  - If a pick changes something inside the prediction scope (own movement, own weapon state), the client's predicting
    sim needs its owner's track state; whether and how it gets it is ADR 0001's.
  - Research: none on tracks; R1-ABL-REC1 and REC5 give the vocabulary.
  - Need: UPGRADE-SCOPE (a settled input outside VISION_V2 §6, LOOP_V2, EDITOR_V2 and LOOK_V2), on §6.3's system.
- **AB-9. ABL-PREDICT stays open; the model carries both answers.** If yes, the client predicts its own ability
  activation, cost, cooldown, granted tags and movement changes, with the command's sequence number as the prediction
  key and cues keyed so that a replayed activation plays once (R1-ABL-REC4, lines 2421–2433, ADR 0001's). If no,
  ability presses wait for the server's confirmation. Nothing in AB-1 to AB-8 depends on the answer: the definitions
  are the same either way; only the prediction scope changes (ADR 0001). Need: VISION_V2 §6.3.
- **AB-10. The per-tick cost (R1-ABL-REC6, owned by ADR 0004), and one proposed addition.**
  - The mark as the research sets it: the ability, effect and weapon step in
    [the stress scene](../research/GLOSSARY.md#the-stress-scene) (N = 48 soldiers, each with up to 8
    [active effects](../research/GLOSSARY.md#active-effect-and-visual-effect), so 48 × 8 = 384) takes at most 1 ms
    per tick at p99, on [one vCPU](../research/GLOSSARY.md#the-measuring-unit-one-vcpu) of
    [the reference server](../research/GLOSSARY.md#the-reference-server) and on the minimum laptop in Wasm in a
    Worker, and allocates nothing on the heap in steady state (r1-simulation-core.md, Recommendations §5,
    R1-ABL-REC6, lines 2465–2472; GLOSSARY.md, §3 The stress scene, lines 397–418).
  - At the 30 Hz baseline, 1 ms is 12 % of the sim step share (1 ÷ 8.33 = 0.12); at 60 Hz it would be 24 % of 4.17 ms
    (1 ÷ 4.17 = 0.24). The 1 ms itself is the same at both rates.
  - Proposed addition: picks are long-lived effects, and how many a soldier holds is not designed yet (OQ-0005-AB3).
    So the spike runs the step at 8, 16 and 32 active effects per soldier (384, 768 and 1,536 at N = 48) and reports
    the count at which the step reaches 1 ms p99. Reason: the stress scene's 8 per soldier predates UPGRADE-SCOPE, and
    the curve tells Mustafa how much room track content has before a number is designed.
  - Need: VISION_V2 §3 and §4 only (a budget, not a feature: R1-ABL-REC6, line 2478).

### Options considered (this section)
Questions C (the time base for durations) and D (what the client predicts) are other drafts': C is ADR 0004's
(R1-ABL-REC3), D is ADR 0001's (R1-ABL-REC4; ABL-PREDICT open). Who owns the tracks and how long they live is settled
by UPGRADE-SCOPE. The research has no option for how tracks are carried, so AB-8 only shows the fit; a separate upgrade
system beside the ability system is not researched and is not proposed.

**Question A: how abilities and effects are expressed** (r1-simulation-core.md, Options §5, Question A, lines 1792–1800).

Option A1: the full GAS model.
| Dimension | Assessment |
|---|---|
| Complexity | High: GAS's write-up needs 11 concept sections (F1–F20), several only for Unreal's object and replication model |
| Cost (money, time, usage) | the highest effort of the five |
| Scalability / performance | unknown (not measured) |
| Security & anti-cheat | execution code is not predicted anyway (F9); otherwise not researched |
| Effect on the art direction / feel | not researched |
| Effort from the current code | legacy has none of it (CD12, CD13, GC11 are tables plus code) |

**Pros:** covers everything GAS does. **Cons:** builds generic-engine features "for later" (CLAUDE.md: REVISE);
over-serves §6.3.

Option A2: a reduced GAS subset as data, over a closed set of Rust operations (proposed: AB-1).
| Dimension | Assessment |
|---|---|
| Complexity | Medium: one generic interpreter over plain data; every new operation a reviewed Rust change |
| Cost (money, time, usage) | medium |
| Scalability / performance | bounded by data size; no script VM; testable for zero allocation; held to 1 ms p99 (AB-10) |
| Security & anti-cheat | content picks and parameterizes operations and cannot add code paths; definitions are validated at load (AB-4) |
| Effect on the art direction / feel | a number change needs no code (R1-ABL-REC1 mark 4) |
| Effort from the current code | the legacy tables convert (R1-ABL-REC5's mark); each operation traces to a legacy or LOOP_V2 test case |

**Pros:** serves §6.3, §6.4, §6.5 and §6.7 directly. **Cons:** a needed mechanic may lack an operation; it then waits
for a Rust change, which is visible and reviewed (the Javelin's knockback is one, AB-6).

Option A3: an embedded scripting language for abilities.
| Dimension | Assessment |
|---|---|
| Complexity | High: a second language and runtime in both the Wasm and the native build |
| Cost (money, time, usage) | not researched |
| Scalability / performance | not researched |
| Security & anti-cheat | the determinism and Wasm support of a script runtime were not researched (R1-ABL-OQ9, line 2781; the options table at line 1798 points to "OQ10", which is the effects-as-entities question) |
| Effect on the art direction / feel | not researched |
| Effort from the current code | no legacy precedent (legacy abilities are code, GC11) |

**Pros:** serves §6.3; Overwatch's Statescript is a shipped example (the talk was not read: R1-ABL-OQ2). **Cons:** adds a
runtime no §6 item asks for.

Option A4: hard-coded Rust per ability, numbers in tables.
| Dimension | Assessment |
|---|---|
| Complexity | Low at first; grows with every ability |
| Cost (money, time, usage) | low at first |
| Scalability / performance | the best of the five |
| Security & anti-cheat | not researched |
| Effect on the art direction / feel | combos and counters need code per pairing, so trying a combo waits for a programmer |
| Effort from the current code | the legacy shape (CD12, CD13, GC11): the least change |

**Pros:** simple and fast. **Cons:** meets "tuning needs no code changes" but not "built from data" (§6.3).

Option A5: adopt Bevy crates (leafwing_abilities, bevy_gauge, bevy_alchemy).
| Dimension | Assessment |
|---|---|
| Complexity | Medium, plus glue |
| Cost (money, time, usage) | not measured |
| Scalability / performance | not measured |
| Security & anti-cheat | not researched |
| Effect on the art direction / feel | leafwing's cooldowns tick by a `Duration` built from `f32` seconds (F23), not on integer ticks (ADR 0004) |
| Effort from the current code | each crate is Bevy-bound and covers one slice (F23–F25); ties this choice to the ECS choice |

**Pros:** existing code. **Cons:** none documents fixed ticks, determinism or prediction (inferred, line 1800).

**Question B: weapons on the effect system or beside it** (Options §5, Question B, lines 1802–1808).

Option B1: weapons keep their own bookkeeping (proposed: AB-3).
| Dimension | Assessment |
|---|---|
| Complexity | one more mechanism to learn |
| Cost (money, time, usage) | low |
| Scalability / performance | keeps the per-shot cost low |
| Security & anti-cheat | the authority decides ammo, cooldowns and reloads (GC4's mark: "the server must decide them") |
| Effect on the art direction / feel | avoids Paragon's lower rate of fire at high latency (F17) |
| Effort from the current code | legacy weapon state is already a per-soldier state machine (GC4, line 243) |

**Pros:** Fortnite's and Quake III's shape (F17, F22). **Cons:** a second mechanism beside effects.

Option B2: weapons as abilities, with a cooldown effect per shot.
| Dimension | Assessment |
|---|---|
| Complexity | one mechanism for everything |
| Cost (money, time, usage) | not researched |
| Scalability / performance | an effect instance per shot at up to 13.9 shots per second per soldier (the cost is inferred, not measured) |
| Security & anti-cheat | not researched |
| Effect on the art direction / feel | the predicted-cooldown problem F17 points at: high-latency players fire more slowly |
| Effort from the current code | not researched |

**Pros:** one mechanism. **Cons:** the latency failure Epic's own games show (F17).

**Question E: the shape of the definitions** (Options §5, Question E, lines 1826–1832).

Option E1: one table per kind, linked by validated ids (proposed: AB-4).
| Dimension | Assessment |
|---|---|
| Complexity | low: string ids resolved to dense integer handles at load |
| Cost (money, time, usage) | low |
| Scalability / performance | integer handles at run time |
| Security & anti-cheat | an unknown id is a load error; no silent fallback (CC9) |
| Effect on the art direction / feel | none |
| Effort from the current code | matches the legacy tables (CD12–CD14) |

**Pros:** shared content is written once (the three T1 classes serve every faction, LOOP_V2 §5). **Cons:** harder to
read than E2, which the research calls "simpler to read" (line 1830).

Option E2: nested, self-contained definitions (each unit embeds its weapons and abilities).
| Dimension | Assessment |
|---|---|
| Complexity | simpler to read |
| Cost (money, time, usage) | shared content is duplicated (the three T1 classes are shared by every faction, LOOP_V2 §5) |
| Scalability / performance | not researched |
| Security & anti-cheat | not researched |
| Effect on the art direction / feel | none |
| Effort from the current code | blocks reuse across factions (§6.7) |

**Pros:** one file tells the whole unit. **Cons:** duplication, and no reuse across factions.

### Trade-off analysis (this section)
- **Why A2, B1 and E1:** together they are the smallest model that writes every legacy weapon and ability and every
  LOOP_V2 ability as data (AB-6) and still keeps counters and combos in data (AB-7). A4 is cheaper today but turns every
  combo into code; A1 and A3 add machinery no §6 item asks for; A5 ties abilities to Bevy and to float time. B1 keeps
  the latency problem Paragon had out of the weapons (F17). E1 keeps one definition, and so one track id, for a class
  every faction shares (LOOP_V2 §5).
- **The strongest argument against:** every GAS fact behind AB-1 comes from one community write-up (R1-ABL-OQ1), and
  the closest shipped team-shooter precedent, Overwatch's scripted abilities, was not read (R1-ABL-OQ2). So the case
  against scripting rests on cost and on "no §6 item asks for a runtime", not on proof that a closed set is enough for
  a live game. And a closed set means each new mechanic Mustafa designs that lacks an operation waits for a reviewed
  Rust change. The content and ability spike's marks 1 and 2 test exactly this; OQ-0005-AB13 names the trigger.
- **Pillars 1 and 2, both sides:** the model takes neither side. Tracks that survive death let a player's power grow
  through a match (pillar 2, power that feels earned); how fast tracks level, and whether a switched-to track catches
  up, decide whether a trailing player keeps a real way back (pillar 1). No track numbers exist, so neither side can be
  measured yet; OQ-0005-AB3 names the two measurements.
- **Two teams and three teams:** everything here is data, so a mode can carry its own values (a room is a mode id plus
  allowed overrides: R2-ROOM-REC7, r2-netcode.md, Recommendations, lines 2857–2867, ADR 0001's). Balance is checked
  per mode, separately (VISION_V2 §3: "Two-team and three-team battles are both tested").

### Open questions (this section)
The first three are the ones Mustafa named; OQ-0005-AB4 to AB11 are what his rules and LOOP_V2 leave open beside each
other. None of AB1 to AB11 has a pick here: they are Mustafa's ("list them as open questions, don't decide them"), and
the data model carries every option listed.

- **OQ-0005-AB1. Carry slots: how many weapons can a player carry besides the class weapon, and what happens to the
  one in hand when they take a pickup?**
  - Why it matters: it decides whether loot replaces or adds, and how many weapon tracks act at once (each held gun's
    picks apply, AB-8).
  - Options: (a) one weapon: a pickup replaces the class weapon, which drops as a pickup; simplest to read and balance,
    but the class's identity in the triangle (LOOP_V2 §5) can be lost mid-life, and one weapon track acts at a time.
    (b) The class weapon plus one looted slot; keeps the class, adds a choice; two weapon tracks act, and the switch
    time is a tuning number. (c) The class weapon plus N looted slots, N as data; the most loot freedom; more tracks act
    at once (more active effects per soldier, AB-10); harder to read from the top-down camera. In every option, a class
    that already comes with two weapons (the Javelin's rockets and Luger sidearm, LOOP_V2 §5) counts both as its own.
  - Decider: Mustafa. What decides it: a design session, then playtests with bots (sims, 3 runs per build).
  - Research ids: none (UPGRADE-SCOPE; the research has no finding on loot).
- **OQ-0005-AB2. Ammo for looted weapons: what does a looted gun come with, and can it be refilled?**
  - Why it matters: it decides how long a looted gun matters against the class's own weapon. LOOP_V2 does not say how
    class weapons refill either; in legacy, normal guns have unlimited reserve by a quirk (GC4, line 243).
  - Options: (a) what its last holder had left, magazine and reserve, never refilled; loot is a short burst, a gun can
    drop empty, and the victim's state matters. (b) A fixed share of a full load (as data), never refilled; predictable
    value, less "what did the victim have". (c) Refilled the same way as class weapons; loot can replace the class for a
    whole life, the strongest pull away from class identity.
  - Decider: Mustafa. What decides it: a design session, together with how class weapons refill. Research ids: none.
- **OQ-0005-AB3. How fast tracks level: what earns a level, how fast, and does a track the player switches to catch
  up?**
  - Why it matters: it sets the snowball inside a match (VISION_V2 §2, pillars 1 and 2, `[OPEN]`; DESIGN_NOTES open
    question 2, line 52: "Snowball control within a match, given that Momentum survives death") and the cost of
    switching.
  - Options for what earns a level: (a) the current unit's and the current weapon's tracks earn from what the player
    does with them (kills, damage, objectives); rewards mastery, makes switching costly. (b) The player earns one pool
    and puts it into any track; switching costs nothing, and a player's power becomes one number. (c) Time in the match,
    the same for everyone; no snowball from skill, the least "earned". (d) Team events (tier-ups, cashouts); ties
    personal power to objective play, and a team shut out of uplinks falls behind twice.
  - Options for catch-up on switching: (i) none: a new track starts at level 0; switching costs the most. (ii) A new
    track starts at a level derived from the player's other tracks (for example a share of the highest). (iii) Lower
    tracks earn faster (a multiplier as data). (ii) and (iii) keep counter-picking (pillar 3) cheap, which the triangle
    needs, and weaken "power that feels earned".
  - Both sides measured, once numbers exist: sims (3 runs per build, two-team and three-team modes separately) and
    match telemetry measure (1) earned power: the time to kill between a fully levelled and a fresh player, both ways,
    per triangle matchup; (2) the way back: how long a fresh player or a trailing team takes to close that gap
    (catch-up speed). Nothing can be computed now: no track content exists.
  - Decider: Mustafa. What decides it: the snowball design session VISION_V2 §2 and §7 call for, then the measurements
    above. Research ids: none (the match record that carries the measurements is ADR 0006's, R6-TEL-REC1).
- **OQ-0005-AB4. What is one track: which definitions share one?**
  - Why it matters: UPGRADE-SCOPE ties progress to "the weapon type". LOOP_V2 §5 gives the same Luger to the Recruit
    and to the Javelin as its sidearm; abilities hand out weapons (the Banzai blade's Katana, the Molotov as a thrown
    weapon, AB-3); vehicles carry mounted weapons (the Juggernaut's cannon, CD14, GC17); LOOP_V2 §8 plans "sidegrade
    variants"; and a vehicle with several seats (`[OPEN]`, VISION_V2 §6.4) would need a rule for whose vehicle track
    applies.
  - Options: (a) every unit, weapon and vehicle definition has its own track; most tracks, and the Luger's progress is
    split by class. (b) Definitions may share a track (the Luger in two classes; the variants of one weapon); progress
    follows the gun the player sees, and needs a rule for which definitions share. (c) Weapons an ability grants for a
    few seconds, and vehicles, have no track; fewer tracks, but against a literal reading of "every weapon". The data
    carries all three: each definition names its track, or none (AB-4).
  - Decider: Mustafa. What decides it: a design session, before track content is written. Research ids: R2-SYNC-OQ7
    (seats, a design question).
- **OQ-0005-AB5. WEAPON-UNLOCKS: how does an account-unlocked weapon reach a match, when buying offers no weapon choice
  and other weapons come only from drops?**
  - Why it matters: LOOP_V2 §8's third gate ("In a match you buy it each life with match money, at a low price") and
    VISION_V2 §1's "Finally unlocked that weapon. Can't wait to use it against them." sit beside UPGRADE-SCOPE's "no
    weapon choice when buying" and "Other weapons come only from killed enemies' drops".
  - Options: (a) a weapon unlock is a class variant that comes with that weapon, bought each life (LOOP_V2 §8:
    "sidegrade variants"); keeps both rules, and the catalogue grows in classes. (b) An unlocked weapon reaches a match
    only as loot; keeps buying choice-free, but needs drops beyond the victim's own weapons, which UPGRADE-SCOPE does not
    describe, and an unlocker may never hold it. (c) Weapons leave the unlock catalogue, and VISION_V2 §1's line is read
    as unlocking a class; narrows pillar 7. (d) An account loadout sets a class's weapon before the match, not when
    buying; keeps the buy step choice-free, but adds a weapon choice UPGRADE-SCOPE's words do not cover.
  - Nothing is built now: the catalogue is `[LATER]` (LOOP_V2 §8) and accounts are `[LATER]` (VISION_V2 §6.10). The
    data model rules none out (a unit definition names its weapons, AB-4). ADR 0006 cites this.
  - Decider: Mustafa, with the Advisor (LOOP_V2 §8 is theirs). What decides it: the unlock catalogue's design session,
    before any weapon unlock is built. Research ids: none.
- **OQ-0005-AB6. How do the player's tracks sit beside the team's tech tiers?**
  - Why it matters: LOOP_V2 §12 and DESIGN_NOTES (lines 19–20) call the team's tiers the answer to in-match
    progression ("Momentum"); UPGRADE-SCOPE adds per-player tracks beside them.
  - Options: (a) independent: two progressions for the player to read. (b) Tiers gate track levels or candidates (a
    candidate can require a tier tag, AB-5); the tier stays the big beat and tracks grow inside it; a shut-out team's
    players level within lower tiers. (c) Tier-ups feed track levels; objective play feeds personal power (pillar 4),
    and a team shut out of uplinks falls behind twice (pillar 1).
  - Decider: Mustafa. What decides it: a design session, with OQ-0005-AB3. Research ids: none.
- **OQ-0005-AB7. May a pick add raw power (more damage, more health), or only change how a unit or weapon plays?**
  - Why it matters: Mustafa decided that tiers are "sophistication, not damage" (DESIGN_NOTES, lines 27–28) and that
    unlocks "give breadth, never raw power" (LOOP_V2 §8); the Advisor's suggestion, not decided, was to bias any deck
    "toward sidegrades over raw power" (DESIGN_NOTES, lines 40–41). None of these lines names in-match picks.
  - Options: (a) sidegrades only; protects the triangle and new players (LOOP_V2 §8: every counter role from level 1),
    with a weaker sense of earned power. (b) Raw power in small steps, with a cap per track as data (a stacking limit,
    AB-1); a strong pillar 2, a risk to pillar 1 and to counters. (c) Mixed, chosen per track; flexible, the hardest to
    balance.
  - Decider: Mustafa. What decides it: a design session, then the two measurements of OQ-0005-AB3. Research ids: none.
- **OQ-0005-AB8. The offer: when does it appear, and when can the player answer it?**
  - Why it matters: a live fight has no pause. The open offer is state on the player's record (ADR 0004), the answer
    is a command the authority checks (ADR 0001), and ADR 0003 draws it.
  - Options: (a) at once, in play, answerable any time (unanswered offers wait, in a queue whose length is data); the
    fastest power, a distraction mid-fight. (b) Only at the buy terminal, where everyone spawns (LOOP_V2 §2); ties picks
    to a visit to the barracks. (c) While dead or respawning; gives the respawn wait a use, and picks lag behind levels.
    With each: are the three candidates always different, and what happens when a track has fewer than three left.
  - Decider: Mustafa. What decides it: a design session, then the Feel Lab or playtests. Research ids: none.
- **OQ-0005-AB9. Which drop and pickup rules does UPGRADE-SCOPE leave unstated?**
  - Why it matters: ADR 0004 builds pickups as sim entities that read these values from data; until they exist, the
    spike uses placeholders.
  - Options for what a killed enemy drops: (a) its class weapon; (b) the gun in its hands; (c) every weapon it carried.
    Also Mustafa's, each a value in data: how long a pickup stays; whether a gun a player swaps out becomes a pickup that
    anyone can take; whether only the killer's team, or anyone, may take a drop; whether a teammate's death drops
    anything (his words name "killed enemies' drops" only).
  - Decider: Mustafa. What decides it: a design session, with OQ-0005-AB1. Research ids: none; whether a pickup is
    predicted is ADR 0001's (Quake III predicts item pickups: R2-SYNC-F17, r2-netcode.md, Findings, lines 230–236).
- **OQ-0005-AB10. Do bots have tracks and make picks, and what does a human get when taking a bot's slot?**
  - Why it matters: bots fill empty slots (LOOP_V2 §2 `[DECIDED]`; VISION_V2 §3), and a joining human takes a bot's
    slot (R2-ROOM-REC6, r2-netcode.md, Recommendations, lines 2842–2846, ADR 0001's); tracks are owned by the player.
  - Options: (a) bots level and pick like humans, and a human taking the slot inherits its tracks; seamless for the
    team, but the joiner holds picks they did not choose. (b) Bots level and pick, and a human taking the slot starts
    empty; fair to the joiner, and the team loses the bot's picks at once. (c) Bots have no tracks; simplest, and teams
    with more bots are weaker, which matters in three-team modes with uneven bot counts. How a bot picks is bot
    behaviour (ADR 0004).
  - Decider: Mustafa. What decides it: a design session, then sims with mixed humans and bots. Research ids: none.
- **OQ-0005-AB11. Does the Riot Shield answer the Banzai blade, and is the Peacekeeper's side of the triangle strong
  enough before Tech 1?**
  - Why it matters: pillar 3 ("every strong thing has an answer") and the triangle (LOOP_V2 §5, `[DEFAULT]`: the
    Peacekeeper beats the Warrior). On legacy numbers, before Tech 1 the Peacekeeper wins only if 11 of 16 pellets land
    (CD13); at Tech 1 the Shield cuts SMG hits to 30 % (63 hits instead of 19), but a blocked blade hit still deals
    2999.7 against 150 HP (AB-7).
  - Options: (a) the frontal block treats the blade like any hit (legacy: the blade kills through it). (b) The Shield
    stops a one-hit kill from the front (a target-tag rule in the pipeline). (c) The Peacekeeper's side is tuned by
    numbers alone (pellets, range, HP), the Shield aside. The data carries all three.
  - Decider: Mustafa. What decides it: sims of each matchup at Tech 0 and at Tech 1 (3 or more runs, as CD13 asks),
    then the Feel Lab. Research ids: R1-ABL-OQ6 (a design question); R2-SYNC-OQ9 (melee timing, ADR 0001's).
- **OQ-0005-AB12. Is tranek's write-up accepted as the GAS reference, or does a verifier read Epic's documentation
  first?**
  - Why it matters: every GAS concept AB-1 borrows comes from one community write-up (R1-ABL-OQ1, lines 2756–2758).
  - Options: (a) accept it: the subset is defined by our own test cases (legacy and LOOP_V2) and proven by the content
    and ability spike, so no part rests on GAS internals beyond the concept list. (b) Allow Epic's documentation domain
    for one verifier pass before acceptance (SRC-ACCESS (a): official-docs domains such as Epic's can be added in the
    next kit PR).
  - Proposed: (a), with (b) as a cheap check if the kit adds the domain. Decider: the Advisor. What decides it: whether
    the next kit PR adds Epic's domain. Research ids: R1-ABL-OQ1.
- **OQ-0005-AB13. When does scripting (option A3) come back?**
  - Why it matters: A3 is set aside on cost and need; the shipped precedent (Overwatch's "Networking Scripted Weapons
    and Abilities", GDC 2017) was not read (R1-ABL-OQ2, lines 2759–2762).
  - Options: (a) only if the content and ability spike's mark 1 or 2 fails (a test ability needs ability-specific Rust
    code, or a combination needs a Rust change), or if new operations turn out to be needed often; (b) read the talk
    first, when a copy turns up (SRC-ACCESS (a)).
  - Proposed: (a), and (b) whenever a copy appears. Decider: the Advisor (the source). What decides it: the content and
    ability spike's marks 1 and 2 (the trigger). Research ids: R1-ABL-OQ2, R1-ABL-OQ9.

### What a spike must prove (this section's part)
All in the content and ability spike (a working name: `docs/SPIKE_PLAN.md` names the spikes and places them). The
test-engineer writes the final pass marks; the candidates below are the research's, or marked proposed with a reason.

1. **AB-1, AB-2 (R1-ABL-REC1's marks, lines 2364–2378, as written):** the test content (AMK, Bagpipe, Shabzon, Retro
   Lancer, Beam Katana, Torque Bow, frag grenade, Molotov, Riot Shield, Banzai, Bayonet, the Juggernaut cannon) runs
   with no ability-specific Rust code (zero matches for those ids in the sim crate's non-test code); a test-only
   combination (the Shield's block on a vehicle, the Banzai speed buff on another class) behaves as its parts, with
   zero Rust changes; the two §6.4 primitives work in a fixture; one number changed in data changes the next match
   with zero Rust changes.
2. **AB-3 (R1-ABL-REC2's mark, lines 2396–2400):** for each of the seven legacy weapons and the Juggernaut cannon,
   per-shot results equal the legacy tables (body-part multipliers; Shabzon falloff at 10, 15, 25, 35 and 50 m; pellets,
   magazine, reload and burst timing), integers exactly and floats within 1e-6 relative (proposed in the research); one
   weapon model for bots and humans.
3. **AB-4 (R1-ABL-REC5's mark, lines 2460–2463):** the legacy `WEAPONS`, `UNIT_TYPES`, `VEHICLE_TYPES` and ability
   fields convert with every legacy number present once; an unknown id, tag or out-of-range value fails at load with
   the file and field named; 10,000 fuzzed definitions never panic the simulation. Proposed: the fuzzing and the load
   checks include track definitions (a track naming an unknown effect fails at load), because AB-4 adds the kind.
4. **AB-5 (proposed):** an ability press before the team's Tech 1, and a T2 purchase before Tech 2, are refused by the
   authority in 100 % of 1,000 scripted attempts, and accepted after the tier. Reason: LOOP_V2 §4 `[DECIDED]`, and legacy
   checks no tier (GE4); 1,000 is the count R1-ABL-REC4's cue mark uses (line 2444).
5. **AB-6, AB-7 (proposed golden cases):** from data alone, the CD13 matchups reproduce: the Warrior kills a Conscript
   in 0.86 s within one tick (R1-ABL-REC3's mark, lines 2416–2418, ADR 0004's); the Peacekeeper kills a Warrior in
   0.85 s when 11 of 16 pellets land; a Peacekeeper behind the Shield takes 63 SMG body hits. The knockback operation
   moves a target with no Javelin-specific code. Reason: these are the numbers the counter check uses (AB-7).
6. **AB-8 (proposed; the research has no mark; reason: UPGRADE-SCOPE), with test-only tracks that are not game
   content, and 1,000 as the count R1-ABL-REC4 uses:**
   - u1: no track-specific Rust code (zero matches for the test track ids in the sim crate's non-test code, as mark 1);
   - u2, death: with picks on unit type U and weapon type W, the unit dies, respawns as a Recruit and buys U again; the
     new unit and its W carry exactly the same pick effects (compared effect lists) in 100 % of 1,000 scripted cycles;
   - u3, switching: U, then V, then U again: U's track is unchanged, V's is its own;
   - u4, a lost gun: the player drops W and takes another W from the world, and the player's W picks apply to it; a
     second player who takes the first gun gets their own W picks and none of the first player's;
   - u5, reset: at the first tick of a new match, every track is empty;
   - u6, offers: the authority draws three candidates; a pick that was not offered, or for a closed offer, is refused in
     100 % of 1,000 forged picks; the same seed and the same inputs give byte-identical offer logs on two runs (and on
     native and Wasm wherever the stream is one the client holds, ADR 0004).
7. **AB-10:** R1-ABL-REC6's mark (ADR 0004's), measured with the method ADR 0002 sets (R1-XT-OQ15); plus the proposed
   curve at 8, 16 and 32 active effects per soldier (AB-10).
8. **AB-9 (ABL-PREDICT):** no spike before acceptance. The Feel Lab test, once abilities are playable (the spike plan
   places it), compares predicted and server-confirmed Molotov and Shield; Mustafa judges the feel, and the
   test-engineer adds his checklist. Ability feel and, later, upgrade picks are Mustafa's to judge in playtests.

### Appendix A rows (for the frame)
| id | disposition | where or why |
|---|---|---|
| R1-ABL-REC1 | adopted | AB-1, AB-2, AB-5, AB-6, AB-7; its marks are "What a spike must prove" 1 |
| R1-ABL-REC2 | adopted | AB-3; its mark is "What a spike must prove" 2 |
| R1-ABL-REC5 | changed: for UPGRADE-SCOPE (Mustafa, 2026-10-31, after the research), two kinds are added (the upgrade track; the drop and pickup values) and the unit kind names its class's own weapons; the rest as written | AB-4, AB-8; "What a spike must prove" 3 |
| R1-ABL-OQ1 | open: OQ-0005-AB12 | the Advisor decides the reference |
| R1-ABL-OQ2 | open: OQ-0005-AB13 | the source, or the spike's trigger |
| R1-ABL-OQ3 | spike: the cue-once rule is tested where a replay happens: own weapon cues in [the netcode proof test](../research/GLOSSARY.md#the-netcode-proof-test) (the prediction scope), own ability cues in the Feel Lab test if ABL-PREDICT says yes; the mark is R1-ABL-REC4's (1,000 scripted presses), ADR 0001's | AB-9 |
| R1-ABL-OQ9 | answered: it matters only if option A3 returns; A3 is not adopted (AB-1), and OQ-0005-AB13 names the trigger | AB-1; Options, Question A |

### For the frame
- **The decision in three lines:** abilities, effects and weapons are data over a closed set of Rust operations (a
  reduced GAS subset); weapons keep their own bookkeeping on one damage pipeline; one table per kind with validated
  ids, tech tiers as tag gates, and Mustafa's upgrade tracks as data on the player's record, picks applied as effects.
- **Parts to point to:** AB-1 to AB-10. Open questions: OQ-0005-AB1 to AB13 (AB1 to AB11 are Mustafa's, with no pick;
  AB12 and AB13 the Advisor's, with a proposal).
- **For `docs/adr/README.md`'s list of features whose need is outside VISION_V2 §6:** the upgrade-track kind and the
  drop and pickup values (need: UPGRADE-SCOPE); active reload's weapon fields (need: LOOP_V2 §7 only; ADR 0004 runs
  it); the AB-10 cost curve, a measurement (need: VISION_V2 §3 and §4 only, as R1-ABL-REC6).
- **Consequences.** Easier: combos and counters by data; tuning with no code change; content work within the closed
  set without a programmer; telemetry keyed by the same ids. Harder: every mechanic outside the closed set is a
  reviewed Rust change (knockback is the first); designers must know the aggregation order and the stacking policies;
  tracks add long-lived effects per soldier, which the 1 ms part must absorb. Revisit when: the spike's mark 1 or 2
  fails (OQ-0005-AB13); track content exists (restate the stress scene's active effects per soldier with ADR 0004);
  ABL-PREDICT is answered; the Spy or the Hammer leaves `[LATER]`; seats are decided.
- **Hard constraints in CLAUDE.md that change:** none because of this section. Tracks reset every match, so they need
  no persistence; cues stay presentation only, so nothing animated decides a hit (AB-1).
- **Legacy inventory rows:** no mark changes. The marks this section relies on stand: CD8, CD12, CD13, CD14, GC4, GC5,
  GC7, GC11, GC14, GC15, GC17 (rewrite in Rust); CS3 (drop); CC9 (port to TypeScript); GE13 (redesign); GE17 (drop: its
  dead gun-game ladder is no track and is not ported). The "look brief pending" notes on GC11, GC14, GC15, CC9 and CC12
  concern presentation, ADR 0003's.
- **Migration (proposed places in SLICE-ORDER):** slice 1 (move and shoot): weapons as data (AB-3, AB-4); slice 2 (bots
  and three teams, played locally): the T1 classes as unit definitions, one weapon model for bots and humans if Mustafa
  keeps it; slice 4 (the loop): tech tiers gating abilities and purchases, the three T1 abilities, and the upgrade tracks
  once OQ-0005-AB1 to AB3 are answered; slice 6 (scale to 48): R1-ABL-REC6 measured with real content.
- **Test plan:** the spike's marks become CI goldens (the CD13 matchups, the legacy-table conversion, 10,000 fuzzed
  definitions); the determinism log of R1-ABL-REC3 (a 120 s scenario, 3 seeds, native and Wasm, in ADR 0004's suite);
  balance by sims, 3 runs per build, in two-team and three-team modes separately; match telemetry (ADR 0006).
- **Rollback:** if mark 1 or 2 fails for one mechanic, that mechanic becomes a new reviewed operation (A2's own route);
  if many fail, those mechanics fall back to A4 with E1's tables kept, and OQ-0005-AB13 reopens A3; if picks cannot be
  written as effects and granted abilities (u1 fails), tracks get their own operations, a reviewed Rust change.
- **Seams the other drafts carry:** ADR 0004: the player record, picks applied on spawn and on holding a gun, pickup
  entities (a new entity kind, which R1-ECS-OQ7 asks ADR 0004 to weigh), the offer stream, the active effects per
  soldier. ADR 0001: the pick command, the visibility of offers and track state, the pickup check and whether pickups
  are predicted (R2-SYNC-F17), a knocked-back local player's predicted movement being corrected. ADR 0006: the pick
  fields of the match record. ADR 0003: the offer on the page.
- **Ids:** the map gave this section seven ids (R1-ABL-REC1, REC2, REC5, OQ1, OQ2, OQ3, OQ9), and all belong here.
