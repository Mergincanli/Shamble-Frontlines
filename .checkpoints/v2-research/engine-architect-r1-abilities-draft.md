# R1 unit abilities: a data-driven ability and effect system

Brief question (quoted): "a **data-driven ability and effect system** in the spirit of UE's Gameplay Ability System"
(`docs/V2_PHASE0_BRIEF.md` §4, row R1).

Author: engine-architect (R1 unit "abilities", wave 1), 2026-10-27. Status: draft, not verified.
Method note: quotes marked "verbatim" were returned by WebFetch when it was asked to copy the sentence character for
character; the verifier should re-open them at the source. Claims seen only in a WebFetch summary are marked "inferred".
The official Unreal Engine documentation was not read (outside the allow list), nor was the engine source (private
repository, HTTP 404), so every GAS fact below comes from one community write-up (tranek's GASDocumentation, tier
"other"), which describes GAS as of UE 5.3.

VISION_V2 needs this unit serves: §6.3 (abilities and effects from data, combos and counters emerge, tuning without
code changes), §6.4 (vehicles with their own weapons and abilities: leech, marking), §6.5 (economy and objectives
server-side and tunable: tech tiers gate abilities), §6.6 (telemetry events), §6.7 (faction content as data).

## Findings

### GAS concepts (from the community write-up; tier other)
Source for F1–F20: tranek, "GASDocumentation", README.md, https://raw.githubusercontent.com/tranek/GASDocumentation/master/README.md
(the README says it is current with UE 5.3; no page date; read 2026-10-27).

- **R1-ABL-F1.** A GAS attribute holds two values: a permanent BaseValue and a CurrentValue that is the BaseValue plus the
  temporary changes from active effects. · §4.3.2 · other · read 2026-10-27 · confirmed in the source (verbatim:
  "An `Attribute` is composed of two values - a `BaseValue` and a `CurrentValue`." … "The `BaseValue` is the permanent
  value of the `Attribute` whereas the `CurrentValue` is the `BaseValue` plus temporary modifications from
  `GameplayEffects`.").
- **R1-ABL-F2.** Gameplay effects have three duration types: Instant (a permanent BaseValue change that applies no tags),
  Duration (temporary CurrentValue changes and tags, removed on expiry) and Infinite (never expires by itself). · §4.5
  (duration table) · other · read 2026-10-27 · confirmed in the source (verbatim: Instant "For immediate permanent changes
  to `Attribute's` `BaseValue`. `GameplayTags` will not be applied, not even for a frame."; Duration "For temporary
  changes to `Attribute's` `CurrentValue` and to apply `GameplayTags`"; Infinite "These will never expire on their own and
  must be manually removed by an ability or the `ASC`.").
- **R1-ABL-F3.** Duration and Infinite effects can be periodic: they re-apply their modifiers and executions every Period
  seconds, and periodic effects are not predicted. · §4.5, §4.10 · other · read 2026-10-27 · confirmed in the source
  (verbatim: "`Periodic Effects` that apply its `Modifiers` and `Executions` every `X` seconds as defined by its
  `Period`."; "`Periodic Effects` cannot be [predicted](#concepts-p)."; not-predicted list: "GameplayEffect periodic
  effects (dots ticking)").
- **R1-ABL-F4.** Modifiers aggregate as ((base + sum of adds) × multiply) / divide, and by default all Multiply (and all
  Divide) modifiers are summed before they are applied; an Override replaces the result. · §4.5.4 · other · read
  2026-10-27 · confirmed in the source for the formula and the summing (verbatim: "The formula for how `Modifiers` are
  aggregated is defined as follows" `((InlineBaseValue + Additive) * Multiplicitive) / Division`; "By default, all
  `Multiply` and `Divide` `Modifiers` are added together before multiplying or dividing"); the Override rule ("last
  applied wins") is inferred (summary only).
- **R1-ABL-F5.** Stacking applies only to Duration and Infinite effects, aggregated either per source or per target, with
  policies for expiration, duration refresh and period reset that the write-up says are explained only by editor
  tooltips. · §4.5.5 · other · read 2026-10-27 · confirmed in the source (verbatim: "Stacking only works for `Duration`
  and `Infinite` `GameplayEffects`."; "There is a separate instance of stacks per Source `ASC` on the Target. Each Source
  can apply X amount of stacks."; "There is only one instance of stacks on the Target regardless of Source."; "Stacks
  also have policies for expiration, duration refresh, and period reset."; they "have helpful hover tooltips in the
  `GameplayEffect` Blueprint.").
- **R1-ABL-F6.** Tags are hierarchical names (Parent.Child.Grandchild), and the ability system counts how many times each
  tag is held. · §4.2 · other · read 2026-10-27 · confirmed in the source (verbatim: "`FGameplayTags` are hierarchical
  names in the form of `Parent.Child.Grandchild...`"; "`GameplayTags` stored in `FGameplayTagCountContainer` have a
  `TagMap` that stores the number of instances").
- **R1-ABL-F7.** An effect can grant tags to its target for as long as it is active, and can carry ongoing tag
  requirements that switch it on or off while it is applied. · §4.5.7 · other · read 2026-10-27 · confirmed in the source
  (verbatim: Granted Tags "Tags that live on the `GameplayEffect` but are also given to the `ASC` that the
  `GameplayEffect` is applied to."; Ongoing Tag Requirements "Once applied, these tags determine whether the
  `GameplayEffect` is on or off.").
- **R1-ABL-F8.** Duration and Infinite effects can grant abilities, with a removal policy when the effect ends. · §4.5.6 ·
  other · read 2026-10-27 · confirmed in the source (verbatim: "`GameplayEffects` can grant new
  [`GameplayAbilities`](#concepts-ga) to `ASCs`."; "Only `Duration` and `Infinite` `GameplayEffects` can grant
  abilities."; removal policies "Cancel Ability Immediately", "Remove Ability on End", "Do Nothing").
- **R1-ABL-F9.** Execution calculations (custom code that computes an effect's outcome) are not predicted and must be
  written in C++; only attribute modifiers predict. · §4.5.12, §4.10 · other · read 2026-10-27 · confirmed in the source
  (verbatim: "The downside to this power and flexibility is that they can not be predicted and they must be implemented
  in C++."; "Attribute modification (EXCEPTIONS: Executions do not currently predict, only attribute modifiers)").
- **R1-ABL-F10.** A cost is an Instant effect, and a cooldown is a Duration effect with no modifiers whose granted tag the
  ability checks. · §4.5.14, §4.5.15 · other · read 2026-10-27 · confirmed in the source (verbatim: "This `Cost GE` should
  be an `Instant` `GameplayEffect`"; "This `Cooldown GE` should be a `Duration` `GameplayEffect` with no `Modifiers`";
  "The `GA` actually checks for the presence of the `Cooldown Tag` instead of the presence of the `Cooldown GE`.").
- **R1-ABL-F11.** Abilities gate and cancel each other through tag sets: required tags (all), blocked tags (any) and
  cancel-with-tag; a stun is built by putting the stun tag in every ability's blocked tags. · §4.6.9, §5.1 · other · read
  2026-10-27 · confirmed in the source (verbatim: "This `GameplayAbility` can only be activated if the owner has **all**
  of these `GameplayTags`."; "This `GameplayAbility` cannot be activated if the owner has **any** of these
  `GameplayTags`."; "Other `GameplayAbilities` that have these `GameplayTags` in their `Ability Tags` will be canceled";
  "the GameplayAbilities are given the stun GameplayTag in their Activation Blocked Tags GameplayTagContainer").
- **R1-ABL-F12.** A "Local Predicted" ability activates on the owning client first, then on the server. · §4.6.8 · other ·
  read 2026-10-27 · confirmed in the source (verbatim: "`Local Predicted` `GameplayAbilities` activate first on the
  owning client and then on the server."). That the server's run corrects the client is inferred (summary only).
- **R1-ABL-F13.** GAS predicts ability activation, triggered events, effect application (attribute modifiers and tag
  changes), cue events, montages and movement; it does not predict effect removal or periodic ticks. · §4.10 · other ·
  read 2026-10-27 · confirmed in the source (verbatim lists: "Ability activation", "Triggered Events", "GameplayEffect
  application:", "Attribute modification (EXCEPTIONS: Executions do not currently predict, only attribute modifiers)",
  "GameplayTag modification", "Gameplay Cue events (both from within predictive gameplay effect and on their own)",
  "Montages", "Movement (built into UE's UCharacterMovement)"; not predicted: "GameplayEffect removal",
  "GameplayEffect periodic effects (dots ticking)").
- **R1-ABL-F14.** GAS ties predicted effects to server confirmation with a prediction key: the client sends the key with
  its activation request, and removes the effects it created under a key once that key comes back from the server;
  predicted effects with no matching server version were mispredicted. · §4.10 · other · read 2026-10-27 · confirmed in
  the source (verbatim: "Client sends this prediction key to the server with CallServerTryActivateAbility()."; "Client
  removes all GameplayEffects that it created with the now stale replicated prediction key."; "Any GameplayEffects that
  the client added and didn't receive a matching replicated version from the server were mispredicted.").
- **R1-ABL-F15.** Paragon and Fortnite (both Epic's shipped games) do not predict damage, and Epic's stated guidance is to
  keep prediction to a minimum. · §4.10 · other (second-hand report of shipped-game practice) · read 2026-10-27 ·
  confirmed in the source (verbatim: "For example, Paragon and Fortnite do not predict damage."; "We still feel player
  prediction is best kept to a minimum"; "predict the minimum amount of stuff you can get away with"). The attribution of
  the last two to Epic's Dave Ratti is inferred (summary only).
- **R1-ABL-F16.** Gameplay cues are for non-gameplay output (sound, particles, camera shake) and are unreliable, so unfit
  for anything that affects gameplay. · §4.8, §4.8.1 · other · read 2026-10-27 · confirmed in the source (verbatim:
  "`GameplayCues` (`GC`) execute non-gameplay related things like sound effects, particle effects, camera shakes,
  etc."; "GameplayCues in general should be considered unreliable and thus unsuited for anything that directly affects
  gameplay.").
- **R1-ABL-F17.** Fortnite tracks weapon fire rates with its own bookkeeping instead of cooldown effects; the answer comes
  in reply to a question about the server overwriting the cooldown of locally predicted abilities. · §11.1.1 "Community
  Questions 1", question 7 · other · read 2026-10-27 · confirmed in the source (verbatim question start: "Is there a way
  to stop the Server from overwriting the cooldown duration of locally predicted abilities"; answer: "Fortnite does its
  own bookkeeping for weapon firing rates: it does not use GEs for cooldowns on weapons."). That the question is about
  high-latency players re-activating later is inferred (summary only).
- **R1-ABL-F18.** The ability's activation prediction key is a 16-bit integer known to both client and server, which the
  write-up offers as a shared random seed. · §5.5 · other · read 2026-10-27 · confirmed in the source for the key
  (verbatim: "is an int16 guaranteed to be synchronized and available in both the client and server"); that it restarts
  at zero each game and that a seed sent by the client can be forged are inferred (summary only).
- **R1-ABL-F19.** Paragon's slows did not stack: only the strongest slow applied. · §5.7 · other · read 2026-10-27 ·
  confirmed in the source (verbatim: "Slow effects in Paragon did not stack."; "but only the greatest magnitude slow
  effect actually affected the Character").
- **R1-ABL-F20.** Background, no recommendation rests on these (summary only, inferred): GAS replicates effects in three
  modes (Full to all clients; Mixed, owner only, for players; Minimal, none, for AI), with cues still multicast to all
  (§7.3); Fortnite replaces attribute replication on simulated proxies by a proxy struct on the pawn (§7.4); lifesteal
  and critical hits are written inside the damage execution, switched on by tags such as `Effect.CanLifesteal` (§5.4,
  §5.6); abilities run in one frame and use latent "ability tasks" for anything over time (§4.7.1); ability batching
  merges activate, target data and end messages into one, mainly for hitscan guns (§4.6.15); predicted gameplay
  projectiles need extra work (Unreal Tournament's dummy projectile, §4.10.3).

### A shipped game's weapon code (Quake III Arena)
Source for F21–F22: id Software, Quake III Arena source (GPL release), `code/game/bg_pmove.c`,
https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/bg_pmove.c (master; no page date; read
2026-10-27).

- **R1-ABL-F21.** Quake III runs the same player code in both client and server ("both games"): it maps a player state and
  a user command to a new player state, and weapon firing is part of it. · file header and `PM_Weapon` · shipped game ·
  read 2026-10-27 · confirmed in the source (verbatim: "// bg_pmove.c -- both games player movement code"; "// takes a
  playerstate and a usercmd as input and returns a modifed playerstate"; inside PM_Weapon: "PM_AddEvent( EV_FIRE_WEAPON
  );").
- **R1-ABL-F22.** Quake III's weapon cooldown is an integer millisecond counter: it is decremented by the frame's
  milliseconds, firing adds the weapon's interval to it (so an overshoot carries into the next shot while the trigger is
  held), and releasing the trigger resets it to 0; ammo is decremented in the same function; the per-weapon intervals are
  code constants in a switch, not data. · `PM_Weapon` · shipped game · read 2026-10-27 · confirmed in the source
  (verbatim: "if ( pm->ps->weaponTime > 0 ) {" "pm->ps->weaponTime -= pml.msec;" … "if ( pm->ps->weaponTime > 0 ) {"
  "return;" … "// check for fire" "if ( ! (pm->cmd.buttons & BUTTON_ATTACK) ) {" "pm->ps->weaponTime = 0;" … "case
  WP_MACHINEGUN:" "addTime = 100;" … "pm->ps->weaponTime += addTime;"; ammo: "pm->ps->ammo[ pm->ps->weapon ]--;"). The
  carry-over reading of `+=` after the decrement is inferred from the code.

### Rust ability crates (tier other)
- **R1-ABL-F23.** leafwing_abilities 0.13.0 (released 2026-09-27 per its page) is built for Bevy and covers cooldowns,
  charges and resource pools; damage, cast times, range checking and resource management are listed as planned; its
  cooldown ticks by a `Duration` and is built from `f32` seconds. · https://docs.rs/leafwing_abilities/latest/leafwing_abilities/
  (crate root) and …/cooldown/struct.Cooldown.html · other · read 2026-10-27 · confirmed in the source (verbatim: "A
  convenient, well-tested ability management suite. Built for the Bevy game engine."; "track and automatically tick
  cooldowns"; "store multiple charges of abilities"; Planned: "resource management (health, mana, energy etc)",
  "damage", "cast times", "range checking"; `pub fn tick(&mut self, delta_time: Duration, charges: Option<&mut
  Charges>)`; `pub fn from_secs(max_time: f32) -> Cooldown`). Its dependency on Bevy ^0.19 and the page's silence on
  attributes, effects, tags, fixed ticks and networking are inferred (summary only).
- **R1-ABL-F24.** bevy_gauge 0.5.1 (released 2026-07-06 per its page) is an attribute and stat system for Bevy:
  attributes are expressions over other attributes, with tagged modifiers that can be applied and removed as sets. ·
  https://docs.rs/crate/bevy_gauge/latest · other · read 2026-10-27 · inferred (summary, with short quotes "A flexible
  attribute and stat system for Bevy" and "Define attributes as expressions, not update systems"; no verbatim request
  made). The page's silence on duration, stacking, determinism and fixed ticks is inferred.
- **R1-ABL-F25.** bevy_alchemy 0.6.0 models each status effect as its own entity linked to its target, with merge modes
  Stack, Insert and Merge, a Lifetime timer and a repeating Delay timer for periodic effects. ·
  https://docs.rs/crate/bevy_alchemy/latest/source/README.md · other · read 2026-10-27 · inferred (summary, with the short
  quote "An experimental, status effects-as-entities system for Bevy").

### Rust standard library (official)
- **R1-ABL-F26.** Rust's standard `HashMap` is randomly seeded by default and iterates in arbitrary order. ·
  https://doc.rust-lang.org/std/collections/struct.HashMap.html (page shows Rust 1.99.1, 2026-10-13), overview and
  `iter` · official · read 2026-10-27 · confirmed in the source (verbatim: "The algorithm is randomly seeded, and a
  reasonable best-effort is made to generate this seed from a high quality, secure source of randomness provided by the
  host without blocking the program."; "An iterator visiting all key-value pairs in arbitrary order.").

## Legacy facts used
From `docs/LEGACY_INVENTORY.md` (by row id) and `docs/LEGACY_TUNING.md` / `docs/legacy-tuning.json` (by table). Marks
are working recommendations (R7-MARKS (a)); rows marked "look brief pending" stay pending.

| Fact | Where | What it tells v2 |
|---|---|---|
| Seven weapons as one data table: fire rate, magazine, reserve, pellets, five spread fields, damage, range, recoil, reload, tracer; special fields for falloff (Shabzon 15–35 m, min 0.2), burst (Lancer 3 at 0.06 s), melee arc (Katana 120°, impact 0.13 s, swing 0.4 s) and charge (Torque Bow) | CD12; `WEAPONS` table | The test arsenal. Sim fields and client feel fields (recoil, tracer) are mixed today and must split. Special-case fields per weapon are the pressure that a generic shape must absorb. |
| Four classes as data: weapon, ability, cooldowns, HP 100/150/120/110, speed and sprint multipliers, bot-only fields, a colour | CD13; `UNIT_TYPES` table | The unit definition's test case; bot fields and the colour do not belong in sim unit data. CD13's computed matchups (Warrior kills Conscript in 0.86 s, 10.5 s the other way; Peacekeeper kills Warrior in 0.85 s only if 11 of 16 pellets land) are golden numbers for the spike. Bots fire a separate, weaker model today: v2 needs one weapon model for both. |
| Juggernaut as data (tier 2, cost 2500, 1500 HP, one seat) and its cannon (three 130-damage shells 80 ms apart, 3.4 s cooldown, 130 m) | CD14, GC17; `VEHICLE_TYPES`, `JUG`, `JUG_PILOT`, `JUG_CANNON` | The vehicle-weapon test case. Its HP is never reduced and its shells start at an animated node: in v2 the mount points are sim data. |
| Class abilities: Molotov (a grenade, building ×1.8, fuse 1.4 s), Riot Shield (70 % frontal block within arc cos 0.30, move ×0.72), Banzai (Katana for 10 s, 45 s cooldown, 1.3× speed), Bayonet (1.3 s charge, kills within 2.4 m, 10 s cooldown) | GC11, CD8; `UNIT_TYPES` (banzaiCd/Dur, bayonetCd/Dur), `T` (shield fields) | The ability test cases: a timed equip with a speed buff (Banzai), a toggled stance with a conditional damage reduction (Shield), a throwable (Molotov), a dash with contact kills (Bayonet). LOOP_V2 §4 locks the three T1 abilities until Tech 1; the Bayonet is not in LOOP_V2. Legacy abilities check no tech tier. |
| Damage path: locational multipliers head 2.6 / body 1.0 / legs 0.65 (Bagpipe head 1.2), the shield block (not for blasts), team filter, kill credit | GC7, CS3, CD8; `T`, `PART` | One damage pipeline with fixed stages; the multipliers live twice today (PART unread, T live): one source per number in v2. |
| Frag grenade 140 damage in 6.5 m with linear falloff, owner ×0.5, fuse 1.5 s, buildings 240, max 3; bolt and Molotov share the blast | GC14, CD8; `T`, `T_CHARGE` | Area damage with falloff and owner/team rules as data; blast reach by line of sight is still open (NC-7). |
| Torque Bow: charge-scaled speed, gravity flight with a swept ray, sticks, fuse 0.55 s, building ×1.5 | GC15, GC13; `T_CHARGE.bow` | A fly-stick-fuse projectile primitive (the Javelin needs projectile flight too). Flight integrates with frame dt today, so landing shifts with frame rate. |
| Weapon state: ammo, reserve, switch and fire cooldowns, timed reloads, spread that grows and decays; normal guns have unlimited reserve by a quirk | GC4 | Weapon bookkeeping the server decides; LOOP_V2 §7 adds active reload (faster reload plus a short damage boost). |
| Fire cooldowns are reset, not carried: the Shabzon fires 10.0 / 12.0 / 13.1 shots per second at 30 / 60 / 144 Hz against 13.9 from its 0.072 s table value | GC5 | On a fixed tick the table values hold only if cooldowns carry their remainder (compare F22). |
| Melee hits through walls and ignores height; the arc uses the render object's facing | GC12 | Impact timing and facing must come from sim state; the line-of-sight rule is R4's. |
| Health regen 3.5 s delay, 16 HP/s, player only | GC19 | A candidate periodic effect, but whether v2 keeps regen is Mustafa's decision (LOOP_V2 is silent). |
| Gun sockets and mass are cosmetic; a missing gun key silently falls back to the AMK | CC9 | The feel kit stays client-side; definitions must be validated at load, never silently defaulted. |
| Shield and blade props only show the ability; the shield block flash is broken | CC12 | Presentation follows sim events (cues); nothing animated decides a hit. |
| Input command groundwork (seq, tick, button bitmask) declared but never wired | CS1 | The sequence number is the natural prediction key (see REC4). |
| Hitscan resolution is a pure query returning a small typed hit record | CS7 | Weapons call one hit query and feed its result into the damage pipeline. |
| Era gating and purchase status enum (owned / era / level / poor / buy) | GE4, GE13 | Tech-tier gates become activation and purchase requirements read from team state. |
| AMK reload tiers (5 / 3.5 / 2.5 s) | `RIFLE_TIERS` | Variants of one weapon as data; LOOP_V2 §8 wants sidegrade variants, not raw power. |

## Options

### Question A: how abilities and effects are expressed

| Option | What it is | Complexity and effort | Performance | Risk | Fit with VISION_V2 §6 |
|---|---|---|---|---|---|
| A1. Full GAS model | Attributes, all effect types, stacking policies, execution hooks, ability tasks, instancing policies, replication modes, prediction keys per effect | Highest: GAS's own write-up needs 11 concept sections (F1–F20) and several concepts exist for Unreal's object and replication model (instancing policy, replication modes, F20) | Unknown here | Builds generic-engine features "for later" (CLAUDE.md: REVISE); executions are unpredictable code anyway (F9) | Over-serves §6.3 |
| A2. Reduced GAS subset as data, with a closed set of operations written in Rust | Attributes (base and current, F1), effects (instant, duration, infinite, periodic; F2–F3), counted hierarchical tags (F6–F7), abilities with tag gates, costs and cooldowns (F10–F11), effect-granted abilities (F8), cues as events (F16). Content picks and parameterizes operations; it cannot add new code paths | Medium: one generic interpreter over plain data; every new operation is a reviewed Rust change | Bounded by data size; no script VM; testable for zero allocation | A needed mechanic may lack an operation (then it is a Rust change, which is visible and reviewed) | Serves §6.3, §6.4, §6.5, §6.7 directly; each operation traces to a legacy or LOOP_V2 test case |
| A3. Embedded scripting language for abilities | Abilities as scripts (Overwatch's Statescript is the shipped example, but the talk was not read: OQ2) | High: a second language and runtime in both the Wasm and native builds | Not researched | Determinism and Wasm support of a script runtime not researched (OQ10) | Serves §6.3 but adds a runtime no §6 item asks for |
| A4. Hard-coded Rust per ability, numbers in tables | The legacy shape (CD12, CD13, GC11): each ability is code, only tuning is data | Low at first, grows per ability | Best | Combos and counters need code per pairing; content work blocks on programmers | Meets "tuning without code changes" but not "built from data" (§6.3) |
| A5. Adopt Bevy crates (leafwing_abilities, bevy_gauge, bevy_alchemy) | Assemble from existing pieces | Medium, plus glue | Not measured | Each is Bevy-bound (F23–F25), covers one slice (cooldowns, stats or status effects), and leafwing's cooldowns tick by `Duration` from `f32` seconds (F23); none documents fixed ticks, determinism or prediction (inferred) | Partial; ties this unit to the ECS unit's choice |

### Question B: weapons on the effect system or beside it
- **B1. Weapons keep their own bookkeeping** (fire timer, ammo, burst, charge, reload, spread) as a small state machine
  driven by the weapon definition, and send hits into the shared damage pipeline as instant effects. Fits F17 (Fortnite)
  and F22 (Quake III); keeps per-shot cost low; one more mechanism to learn.
- **B2. Weapons as abilities with cooldown effects per shot** (F10's cooldown pattern for every shot). One mechanism,
  but F17's question points at the predicted-cooldown problem this creates, and per-shot effect instances would add
  bookkeeping at up to 13.9 shots per second per soldier (CD12 Shabzon; the cost is inferred, not measured).

### Question C: the time base for durations, periods and cooldowns
- **C1. Integer time on the fixed tick**: durations converted from authored seconds at load into an integer unit in which
  the tick step is exact, decremented per tick, with the remainder carried (F22's `+=`). Same result on every machine
  and frame rate; needs a load-time conversion and a rounding report.
- **C2. Floating-point seconds advanced by a delta** (leafwing's `Duration` and `f32`, F23; legacy's frame dt, GC5,
  GC15). Simple, but legacy shows the rate drifting with the frame rate (GC5) and bit-identical results on Wasm and
  native are the determinism unit's question.

### Question D: what the client predicts
- **D1. Nothing** (server only): simplest; every ability press waits a round trip, against LOOP_V2 §7's "the shot feels
  instant".
- **D2. The local player's own activation, cost, cooldown, tags and movement changes, plus cues; never damage, kills or
  effects on other players.** Matches Epic's practice (F13, F15) and LOOP_V2 §7's split feedback (instant shot, hit
  feedback on confirmation).
- **D3. Everything, damage included.** F15: Epic's own games avoid it, and the write-up warns of health jumping back.

### Question E: the shape of the definitions
- **E1. One table per kind** (unit, vehicle, weapon, ability, effect, tag registry, feel kit), linked by string ids that
  are resolved to dense integer handles at load and validated (an unknown id is a load error). Matches the legacy
  tables (CD12–CD14) and fixes CC9's silent fallback.
- **E2. Nested, self-contained definitions** (each unit embeds its weapons and abilities). Simpler to read; duplicates
  shared content (the three T1 classes are shared by every faction, LOOP_V2 §5) and blocks reuse across factions
  (§6.7).

## Recommendations

**R1-ABL-REC1. A reduced GAS subset, as data, over a closed set of Rust operations (option A2).**
- The model: attributes with a base and a current value; effects that are instant, timed or permanent, optionally
  periodic; counted hierarchical tags; abilities gated by required and blocked tags (team tech tiers included, LOOP_V2
  §4), with a cost and a cooldown; effects that grant tags or abilities for their lifetime; cues as presentation events
  only. Content chooses and parameterizes operations (for example "apply effect E to targets in radius R with falloff
  F", "grant ability A while active", "on kill by source, run effect E"); it cannot run arbitrary code. Leave out what
  exists for Unreal's object and replication model (instancing policies, replication modes, ability tasks as a general
  framework) and anything no legacy or LOOP_V2 test case needs.
- Damage and similar outcome logic is a fixed pipeline of Rust stages parameterized by data (locational multiplier,
  range falloff, frontal-arc block, target-tag multiplier for counters such as "strong against vehicles", owner and team
  rules), the role GAS gives to execution code (F9), which GAS does not predict either.
- Aggregation order is fixed and documented per attribute (F4 shows that the default GAS rule sums multipliers, which
  is a rule the designers must know). Stacking is an explicit per-effect policy chosen from a short list (no stacking,
  refresh duration, add a stack up to a limit, strongest only as in F19), never an implicit default (F5 says GAS's
  own policies are only documented in tooltips).
- Reason: §6.3 asks for GAS-like data; the subset covers every legacy ability and weapon (Legacy facts) and the
  primitives §6.4 names (an effect bound to its source's life for leech, an effect that grants a tag for marking),
  without generic-engine features.
- Rests on: F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F16, F19.
- Serves: VISION_V2 §6.3, §6.4, §6.5 (tech gates), §6.7.
- A spike must prove (proposed pass marks):
  1. The test content runs with no ability-specific Rust code: AMK, Bagpipe, Shabzon, Retro Lancer (burst), Beam Katana
     (melee arc), Torque Bow (charge, stick, 0.55 s fuse), frag grenade, Molotov, Riot Shield, Banzai, Bayonet and the
     Juggernaut cannon, all as data. Measure: zero matches for those content ids in the simulation crate's non-test
     code.
  2. A new combination needs data only: a test-only definition (not a game rule) that puts the Riot Shield's block on
     a vehicle and the Banzai speed buff on a different class loads and behaves as its parts do, with zero Rust
     changes.
  3. The two §6.4 primitives work in a test fixture: an effect removed when its source dies (LOOP_V2 §6: "Stolen items
     only last until the Hammer dies"), and an on-kill trigger that moves a granted ability from victim to source. The
     item and marking rules themselves stay undesigned ([LATER], Mustafa's).
  4. Changing one number in data (for example Shabzon damage 8 to 9) changes the next match with zero Rust changes
     (the hot-reload path is the data-formats unit's).

**R1-ABL-REC2. Weapons keep their own bookkeeping and share the damage pipeline (option B1).**
- A weapon definition holds the sim fields only (fire interval, fire mode auto, semi or burst, magazine, reserve,
  pellets, spread hip, ADS, per shot, max and decay, range, falloff, damage, reload, charge curve, projectile kind);
  the feel kit (recoil, tracer, flash, sound, kick) is a separate client definition keyed by the same id (LOOP_V2 §7,
  CC9). The weapon's state machine (timer, ammo, burst count, charge, reload, active-reload window) is per-soldier sim
  state; each hit becomes an instant damage effect through REC1's pipeline. Abilities may grant or swap weapons (Banzai
  equips the Katana; Molotov is a thrown weapon) and apply effects to weapon attributes (active reload's short damage
  boost, LOOP_V2 §7).
- Reason: F17 says Fortnite keeps weapon fire rates out of the effect system and F22 shows a shipped shooter's
  per-player integer weapon timer; both keep the hottest path (13.9 shots per second for the Shabzon, CD12) out of
  effect bookkeeping.
- Rests on: F17, F22, F9.
- Serves: VISION_V2 §6.3 (weapons built from data), §6.4 (vehicle weapons on mounts).
- A spike must prove (proposed): for each of the seven legacy weapons and the Juggernaut cannon, per-shot results equal
  the legacy tables: damage per body part (head 2.6, body 1.0, legs 0.65; Bagpipe head 1.2), Shabzon falloff at 10, 15,
  25, 35 and 50 m against the legacy `falloffMult` formula, pellet count, magazine, reload and burst timing; integer
  results exactly, floating-point results within 1e-6 relative (the bound is proposed); and one weapon model for bots
  and humans (CD13).

**R1-ABL-REC3. Integer time on the fixed tick, with carried remainders and a stable order (option C1).**
- Authors write seconds; the loader converts every duration, period, cooldown, fuse and fire interval into an integer
  time unit in which the tick step is exact, and reports the rounding of each value. Timers carry their remainder
  (F22's `+=`), so table rates hold on any tick rate. Effects start, tick and expire only on tick boundaries; the order
  of application within a tick is fixed (for example by entity index, then effect start tick, then a per-match sequence
  number), and no per-tick logic iterates a randomly seeded `HashMap` (F26). Random rolls (spread) come from the
  simulation's seeded RNG, never from the activation key alone (F18 is a 16-bit key that both sides know).
- Reason: GC5 measured the legacy reset-not-carry rule losing up to 28 % of the Shabzon's rate at 30 Hz
  (10.0 vs 13.9 shots per second); F22 shows the carry; F26 shows that `HashMap` order is not stable across runs.
- Rests on: F22, F26, F3.
- Serves: VISION_V2 §6.3 (tuning that means the same thing everywhere), §6.6 (telemetry numbers comparable across
  matches).
- A spike must prove (proposed): at the tick rate the fixed-tick unit recommends, the Shabzon's sustained rate is within
  1 % of 1 / 0.072 s = 13.9 shots per second, and CD13's Warrior-kills-Conscript time (0.86 s from the table values) is
  reproduced within one tick; a 120 s scripted scenario that uses every test ability, run with 3 seeds, gives
  byte-identical logs of damage, effect start and end, and cooldown start and end on native and Wasm.

**R1-ABL-REC4. Predict only the local player's own abilities and weapons; never damage (option D2).**
- The client predicts its own activation, cost, cooldown, granted tags, movement changes (Banzai speed, Shield slow),
  weapon timer and ammo, and the cues of those; damage, kills and effects on other players wait for the server. The
  prediction key is the input command's sequence number (the field CS1 already declared), echoed by the server with
  accept or reject; a rejected activation is rolled back. Cues are events keyed by (entity, ability or weapon,
  command sequence), so a re-simulated or confirmed activation does not play twice. Effect removal and periodic ticks
  are not predicted (as in GAS, F13).
- Reason: F13 and F15 (Epic's shipped practice, second-hand), F14's key-and-confirm model, F16 (cues never decide
  gameplay), LOOP_V2 §7 (split feedback). Because the client runs the same Rust code in Wasm, prediction of the local
  player can be the netcode's re-simulation from the last confirmed state instead of per-effect keys: that is an
  inference, and R2 decides the prediction model (OQ4).
- Rests on: F12, F13, F14, F15, F16, F21.
- Serves: VISION_V2 §6.3, §4 ("matches feel fair up to about 100 ms of ping").
- A spike must prove (proposed): with 100 ms injected round trip (plus jitter and loss as R2 sets), the local player's
  ability press changes its own cooldown, cost and speed in the same client tick as the input; a rejected press (for
  example before Tech 1) is undone within one round trip plus one tick; every accepted activation plays its cue exactly
  once (count over 1,000 scripted presses); no damage number or kill on another soldier is shown before the server's
  event.

**R1-ABL-REC5. One table per kind, linked by validated ids (option E1).**
- Kinds: tag registry; attribute set per unit kind; effect; ability; weapon (sim); feel kit (client only); unit (class:
  attributes, weapons, abilities, tier, cost, tags); vehicle (parts, each with HP and tags; weapon mounts as fixed
  points; seats, with the count left open by VISION_V2 §6.4 [OPEN]); faction (which units and vehicles, uniforms by
  reference, §6.7). String ids resolve to dense handles at load; every reference, tag and number range is checked, and
  a failure names the file and field; no silent defaults (CC9). The file format, schema tooling and hot reload are the
  data-formats unit's.
- Reason: the legacy tables already have this shape (CD12–CD14) and their faults are mixing (bot and colour fields in
  unit data, CD13), duplication (multipliers in two places, CS3) and silent fallbacks (CC9).
- Rests on: F6 (tags as the shared vocabulary between kinds), F8 (abilities granted by effects need referenceable ids).
- Serves: VISION_V2 §6.3, §6.4, §6.7, §6.6 (telemetry keys pick and win rates by these ids).
- A spike must prove (proposed): the legacy `WEAPONS`, `UNIT_TYPES`, `VEHICLE_TYPES` and the ability fields convert
  into these tables with every legacy number present once (a script lists any number with no home); a definition with
  an unknown id, tag or out-of-range value fails at load with file and field named; 10,000 fuzzed definitions never
  panic the simulation.

**R1-ABL-REC6. Cost budget for the ability and effect step (proposed).**
- The ability, effect and weapon step for 48 soldiers (VISION_V2 §3, 24v24), each with up to 8 active effects (a
  proposed load), costs at most 1 ms per tick on the proposed reference server core and on the VISION_V2 §4 minimum
  laptop running Wasm in a Worker (both budgets proposed), and allocates nothing on the heap in steady state (counted
  with a counting allocator).
- Reason: the brief's own example pass mark ("48 simulated players at a 30 Hz server tick within X ms"); this unit's
  share of X must be measured, not assumed.
- Rests on: no finding (a budget to measure; every number in it is proposed).
- Serves: VISION_V2 §3 (scale), §4 (minimum hardware).
- A spike must prove: the numbers above, measured with the method R5 sets.

## Open questions
- **R1-ABL-OQ1.** The official GAS documentation (dev.epicgames.com, outside the allow list) and the engine source
  (private, HTTP 404) were not read: every GAS fact here is one community write-up's. What decides it: allowing the
  Epic documentation domain for one verifier pass, or accepting the write-up as the reference.
- **R1-ABL-OQ2.** Overwatch's "Networking Scripted Weapons and Abilities" (GDC 2017, per a search result) is the closest
  shipped team-shooter precedent for scripted, networked abilities, and was not read (GDC Vault redirected to a
  maintenance page outside the allow list; the talk is video). It would show whether a scripting layer (A3) paid off in
  a shipped game. What decides it: reading the talk or its slides.
- **R1-ABL-OQ3.** Valve's developer wiki pages on prediction and on Dota 2's data-driven abilities returned HTTP 403:
  how Source suppresses repeated effects during re-prediction, and how Dota 2 shapes modifiers as data, are unread. What
  decides it: another fetch route to those pages, or leave REC4's cue rule as an inference to be tested in the spike.
- **R1-ABL-OQ4.** Per-effect prediction keys (GAS, F14) or re-simulation of the local player from the last confirmed
  state: which one depends on R2's prediction model and on the Worker boundary unit. What decides it: R2's
  recommendation.
- **R1-ABL-OQ5.** Attribute math in floats or fixed-point, and the exact aggregation order (F4): the determinism unit
  decides whether floats are bit-identical on Wasm and native.
- **R1-ABL-OQ6.** Rules the system must support but not decide (Mustafa's): stacking of slows and marks; whether health
  regen stays (GC19); whether the Bayonet and the Torque Bow stay (GC11, GC15); active reload numbers (LOOP_V2 §7);
  the Hammer's "item" and leech details ([LATER]); what marking does (§6.4); seats per vehicle ([OPEN]); one weapon
  model for bots and humans (CD13). The spike uses legacy behaviour as its test case only.
- **R1-ABL-OQ7.** Fire intervals shorter than one tick: fire several shots in one tick or cap at one per tick? Legacy's
  fastest intervals (Shabzon 0.072 s, Lancer burst 0.06 s) are longer than a 30 Hz tick (0.033 s) but not a 10 Hz one.
  What decides it: the tick-rate choice.
- **R1-ABL-OQ8.** Who sees which tags: marking and the Spy's disguise (LOOP_V2 §5, [LATER]) need tags visible to some
  teams only. What decides it: R2's interest management.
- **R1-ABL-OQ9.** Whether a Rust-embeddable script runtime compiles to Wasm and runs deterministically was not
  researched; it matters only if A3 comes back after OQ2.
- **R1-ABL-OQ10.** Effects as their own entities (bevy_alchemy's approach, F25) or as a list on the target: the ECS unit's
  choice decides it.

## Sources read
| URL | Title | Version or page date | Read |
|---|---|---|---|
| https://github.com/tranek/GASDocumentation | tranek, GASDocumentation (repository page) | no date shown | 2026-10-27 |
| https://raw.githubusercontent.com/tranek/GASDocumentation/master/README.md | GASDocumentation README (community; tier other) | says current with UE 5.3; no page date; read in windows 0–331,640 (the last 6,640 characters, around the changelog, not read) | 2026-10-27 |
| https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/bg_pmove.c | Quake III Arena source, bg_pmove.c (shipped game) | master; no page date | 2026-10-27 |
| https://docs.rs/leafwing_abilities/latest/leafwing_abilities/ | leafwing_abilities crate docs | 0.13.0, released 2026-09-27 per page | 2026-10-27 |
| https://docs.rs/leafwing_abilities/latest/leafwing_abilities/cooldown/struct.Cooldown.html | leafwing_abilities `Cooldown` | 0.13.0 (first 100,000 characters read) | 2026-10-27 |
| https://docs.rs/crate/bevy_gauge/latest | bevy_gauge crate page | 0.5.1, released 2026-07-06 per page | 2026-10-27 |
| https://docs.rs/crate/bevy_alchemy/latest/source/README.md | bevy_alchemy README | 0.6.0; no date | 2026-10-27 |
| https://doc.rust-lang.org/std/collections/struct.HashMap.html | Rust std `HashMap` | Rust 1.99.1 (2026-10-13) | 2026-10-27 |

## Sources not read
| URL | Reason |
|---|---|
| https://dev.epicgames.com/documentation (Unreal Engine Gameplay Ability System pages) | outside the allow list |
| https://github.com/EpicGames/UnrealEngine/blob/release/Engine/Plugins/Runtime/GameplayAbilities/Source/GameplayAbilities/Public/GameplayPrediction.h | HTTP 404 (private repository: needs an Epic-linked GitHub account) |
| https://developer.valvesoftware.com/wiki/Dota_2_Workshop_Tools/Scripting/Abilities_Data_Driven | HTTP 403 |
| https://developer.valvesoftware.com/wiki/Prediction | HTTP 403 |
| https://gdcvault.com/play/1024041/Networking-Scripted-Weapons-and-Abilities (and the www. form) | redirected to gdconf.com/gdc-vault-maintenance/, outside the allow list; the talk is video only |
| https://gdcvault.com/play/1024001/-Overwatch-Gameplay-Architecture-and | not tried: same host in maintenance; video only |
| https://ikrima.dev/ue4guide/gameplay-programming/gameplay-ability-system/epic-technical-brief/ | outside the allow list (search lead only) |
| https://www.gamedeveloper.com/design/attend-gdc-and-see-how-blizzard-scripts-i-overwatch-i-s-weapons-and-powers | outside the allow list (search lead only) |
| https://crates-io.ganymede-static.progval.net/crates/leafwing_abilities/0.1.1 and https://rust-digger.code-maven.com/crates/bevy_gauge | outside the allow list (search leads only; docs.rs read instead) |

## Cross-unit notes
- ECS or plain data: effects as entities or as a list on the target (OQ10); the crates in F23–F25 assume Bevy.
- Fixed tick rate: REC3's integer time unit must make the tick step exact; shots faster than a tick (OQ7); GC5's
  measured drift.
- Identical results Wasm and native: attribute math and aggregation order (OQ5); no `HashMap` iteration in per-tick
  logic (F26).
- Rust ↔ TypeScript boundary and Worker: cues and damage events cross as a bulk event buffer, not a call per event;
  definition ids and the feel-kit keys need generated TypeScript types; the feel kit stays client-side.
- Data formats and hot reload: REC5 gives only the shape of the definitions; the file format, schema, validation tooling
  and hot reload are that unit's.
- R2: the prediction model (OQ4), the prediction key as the command sequence (CS1), which tags other teams see (OQ8),
  how much effect state is replicated (F20 for GAS's own modes).
- R4: line of sight for blasts (NC-7, GC14) and melee (GC12); the shield's frontal arc uses sim facing.
- R6 telemetry (§6.6): damage and effect events carry the source ability or weapon id, which pick and win rates need.
- R3: cues drive effects and the spectacle budget (§6.9); the look is not decided.
