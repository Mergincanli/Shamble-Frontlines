# rust-reviewer: the final fix round on ADR 0004 (all but "### Pass marks"), at 47eadd9

Author: rust-reviewer, 2026-11-02. Every BEFORE below is copied by script from `git show 47eadd9:docs/adr/0004-simulation-core.md` (whole lines, non-overlapping ranges, in file order); the session's `scripts/apply_fixes.py` applies them. Notes and routes follow the fixes.

### FIX RR-4 + GD-8 (the pickup pool) + POOL-FULL
FILE: docs/adr/0004-simulation-core.md
BEFORE:
   target: owner id, definition id, timers, stacks); per-team tables; the player records (part 18); the static colliders
   with their ids; the random streams' state; the tick counter; the tick's event list. Phases are plain functions called
AFTER:
   target: owner id, definition id, timers, stacks); per-team tables; the player records (part 18); the static colliders
   with their ids; the random streams' state; the tick counter; the tick's event list. Every pool has a fixed capacity,
   set at match start from the mode and the content set and allocated then, so the step never grows one (part 6) and
   nothing a client sends sizes it: each pool's worst case follows from the content's numbers (fire intervals and
   projectile lifetimes, pellets per shot, effect durations and stack limits, drops per death and pickup lifetimes)
   and from the game-designer's caps where they exist (R2-BUD-OQ5), and ADR 0005's whole-set check reports it. A full
   pool applies one fixed, documented overflow rule, identical on every target, and counts every overflow in a
   per-pool counter; which rule each kind gets (refuse the new row, or end the oldest early) decides who wins a
   pile-up, so it is OQ-0004-11's, and until it is answered the spikes refuse the new row as a test fixture, not a
   rule. Phases are plain functions called
END

### FIX EA-3
FILE: docs/adr/0004-simulation-core.md
BEFORE:
   prediction scope needs it; under R2-SYNC-REC3's scope (own movement, own weapon state, own vehicle) it is dropped
   (R1-XT-OQ7 is ADR 0001's).
   - Why: the client replays its own commands from the server's state on every snapshot, and the suite must show that a
     restored run continues byte for byte.
   - Ids: R1-ECS-REC1 marks 2–4, R1-ECS-REC4, R1-ECS-OQ2, R1-ECS-OQ5 (r1-simulation-core.md, lines 1932–1945,
     1993–2005, 2614–2618, 2627–2628); R4-PHYS-REC2 (r4-physics-navigation.md, lines 721–745).
AFTER:
   prediction scope needs it; under R2-SYNC-REC3's scope (own movement, own weapon state, own vehicle) it is dropped
   (R1-XT-OQ7 is ADR 0001's). The sim crate therefore has two entry points over the same phase functions. The
   authority step runs every phase in the fixed order (each room on the server; the Worker's local server). The
   prediction step advances one slot under ADR 0001's prediction scope (its part 4: own movement, own weapon state
   with reload and active reload, the vehicle the player pilots, and the cosmetic shot events these make): it runs
   only those phases, for that slot and its vehicle, against the static world and the movers as last decoded from the
   server's snapshot, which it reads and never advances; it runs no bots, navigation, economy, objectives, moments or
   other soldiers' weapons, and it draws from no random stream (part 8), so a predicting instance holds no
   authority-only stream's state. The Worker's client endpoint calls it for each replayed and each new own command; a
   phase joins it only when ADR 0001's scope changes (ABL-PREDICT may add the player's own abilities).
   - Why: the client replays its own commands from the server's state on every snapshot, the suite must show that a
     restored run continues byte for byte, and both the replay's agreement with the authority and part 8's stream rule
     rest on one named prediction step.
   - Ids: R1-ECS-REC1 marks 2–4, R1-ECS-REC4, R1-ECS-OQ2, R1-ECS-OQ5 (r1-simulation-core.md, lines 1932–1945,
     1993–2005, 2614–2618, 2627–2628); R4-PHYS-REC2 (r4-physics-navigation.md, lines 721–745); cites R2-SYNC-REC3
     (r2-netcode.md, Recommendations, lines 2068–2105; ADR 0001's) for the prediction step.
END

### FIX NA-8 (part 5)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
   mark (W3 needs shared memory, so cross-origin isolation: ADR 0002's call, R1-XU-OQ4). `requestAnimationFrame` is the
   page's render clock only. After a long gap (a hidden tab, a sleeping laptop) the browser does not simulate the gap:
   in local play the match pauses; online the Worker resyncs from the server's state through ADR 0001's path, with
   catch-up per frame bounded at 7 ticks (the same rule, proposed). Pause, shop and hit-stop never stop the shared tick
AFTER:
   mark (W3 needs shared memory, so cross-origin isolation: ADR 0002's call, R1-XU-OQ4). What sets when the next tick
   is due depends on the mode: in local play, the plain accumulator; online, ADR 0001's rule for keeping commands just
   in time (OQ-0001-6), which runs the Worker's tick ahead of the server's on the client endpoint's server-time
   estimate (ADR 0001 part 5). The Worker's step call (ADR 0002 part 5) therefore also returns when its next tick is
   due, and the Worker's host sets its timer from that, never from a count of its own; the sim's own step still takes
   no time (part 4). `requestAnimationFrame` is the
   page's render clock only. After a long gap (a hidden tab, a sleeping laptop) the browser does not simulate the gap:
   in local play the match pauses; online the Worker resyncs from the server's state through ADR 0001's path, with
   catch-up per wake of the Worker bounded at 7 ticks (the same rule, proposed; R1-TICK-REC5 says "per frame", and the
   Worker has no frames). Pause, shop and hit-stop never stop the shared tick
END

### FIX GD-8 (money's unit, part 7)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
   lockstep, no fixed point. Inside the sim: f32 throughout (up to a map 2 km across, R4-PHYS-REC5); only `+ − × ÷`,
AFTER:
   lockstep, no fixed point. Inside the sim: f32 for every real-valued quantity (up to a map 2 km across,
   R4-PHYS-REC5), while time, counts and money are integers (parts 4 and 17); only `+ − × ÷`,
END

### FIX SA-15
FILE: docs/adr/0004-simulation-core.md
BEFORE:
8. **Random streams.** The sim draws only from `rand_chacha`'s `ChaCha8Rng`, pinned, seeded by the authority per match,
   one stream per purpose. The sim maps raw generator output with its own few sampling functions and pinned test
AFTER:
8. **Random streams.** The sim draws only from `rand_chacha`'s `ChaCha8Rng`, pinned, seeded by the authority per match,
   one stream per purpose. Each match's seed comes from the operating system's secure random source on the server
   (in local play, the Worker's `crypto.getRandomValues`): the host draws it and passes it in at match start, so the sim
   reads no random source itself. The seed is never sent to a client, never logged and never written into ADR 0006's
   match record; outside the live state only the recorded match (I-16) keeps it, and no recording reaches a client
   while its match runs (proposed: the sim crate, and with it the generator, ships in every client's Wasm, so a seed a
   client could derive, from the match id, the room or the start time, would let it compute the authority-only
   streams: the spread SEC-SPREAD (a) keeps on the server, and the offers). The sim maps raw generator output with its
   own few sampling functions and pinned test
END

### FIX GD-8 (the hard-coded three, part 8)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
   upgrade offers of three (part 18). Offers are authority-only for SEC-SPREAD's reason: a seed the client holds lets a
AFTER:
   upgrade offers (part 18). Offers are authority-only for SEC-SPREAD's reason: a seed the client holds lets a
END

### FIX FB-10
FILE: docs/adr/0004-simulation-core.md
BEFORE:
      toolchain, R4-NAV-F8, and R4-NAV-REC1 reads linking it as not a supported path), no Rust navigation crate
      promises determinism, baking once makes generation's floats shared data, and Quake III shipped bots on a
      precomputed area graph.
AFTER:
      toolchain, R4-NAV-F8, and R4-NAV-REC1 reads linking it as not a supported path), no Rust navigation crate read
      promises determinism (R4-NAV-F16), baking once makes generation's floats shared data, and Quake III shipped bots
      on a precomputed area graph.
END

### FIX GD-BOT-LOOT (part 15)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
    mix); no research covers a bot AI architecture, so its structure is OQ-0004-1. Bots act on the present while humans
AFTER:
    mix; whether bots also take world pickups, which UPGRADE-SCOPE added after that list, is OQ-0004-12); no research
    covers a bot AI architecture, so its structure is OQ-0004-1. Bots act on the present while humans
END

### FIX GD-4 + FC-1 + GD-7 + GD-8 (money's unit and the rebuild split) + GD-STEAL + GD-CARRIER (part 17)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
17. **Economy and objectives as sim phases.** Money and harvester deliveries, team-funded rebuilds, uplinks (cache,
    carry, place, countdown, steal, rotation), tech tiers, power-plant pricing and the points race run as sim phases in
    the fixed order at the cadences of part 4, with their numbers in ADR 0005's tables and their rules exactly as LOOP_V2
    §3, §4 and §8 state them. Team filtering (who a shot or blast can hurt, GC1) and presence counting for steals (GE8)
    are sim code. What each team may see of another team's state is ADR 0001's visibility classes. A tier reached or
    money paid is an event (part 19), never a HUD write from the sim (legacy's `addTech`).
    - Why: LOOP_V2's economy is server-side and tunable, and its outcomes must replay like everything else.
    - Ids: none of its own (no research finding on economy phases); cites R1-TICK-REC3 (cadences).
    - Need: VISION_V2 §6.5.
AFTER:
17. **Spawning, economy and objectives as sim phases.** Money and harvester deliveries, team-funded rebuilds, uplinks
    (cache, carry, place, countdown, steal, rotation), tech tiers, power-plant pricing and the points race run as sim
    phases in the fixed order at the cadences of part 4, with their numbers in ADR 0005's tables and their rules
    exactly as LOOP_V2 §3, §4 and §8 state them. Money is an integer, never a float (proposed: whole dollars, as every
    LOOP_V2 §3 and §5 amount is, so a split gives the same shares on every target), and it lives on the player record
    (part 18), since a death keeps it (LOOP_V2 §5); a rebuild's equal shares (LOOP_V2 §3) are integer divisions whose
    rounding and remainder rule are data in ADR 0005's tables, applied in a fixed slot order so every target gives the
    same shares. A steal is holding E for a few seconds (LOOP_V2 §4 step 4), one channel for bots and humans alike,
    since bots send the same commands (part 15); GE8's presence count (every team's live members in the uplink's
    circle) is kept as a primitive the steal may read. LOOP_V2 does not say whether a defender inside the circle pauses
    or blocks a steal, or what happens when two enemy teams steal one uplink at once, so the presence rule is a data
    switch whose setting is OQ-0004-8's; whether a carried cache's position is public is OQ-0004-9's (its visibility
    class then ADR 0001's). Spawning runs as LOOP_V2 §2 and §5 state it: everyone spawns inside their own barracks, in
    front of the buy terminal; spawn protection lasts 3 s (`[DEFAULT]`; an effect in ADR 0005's data); a death
    respawns the slot as the Recruit (`[DEFAULT]`); a purchase is a command the authority checks (ADR 0001). "Enemies
    cannot enter a barracks interior" (`[DEFAULT]`) is a team mask on a collider's movement flag (part 13): colliders
    across a barracks' openings block every soldier but its own team's, and the bake gives the nodes behind them that
    team's flag (part 11), so no other team's route runs through them. Team filtering (who a shot or blast can hurt,
    GC1) is sim code. What each team may see of another team's state is ADR 0001's visibility classes. A tier reached
    or money paid is an event (part 19), never a HUD write from the sim (legacy's `addTech`).
    - Why: LOOP_V2's economy is server-side and tunable, and its outcomes must replay like everything else; integer
      money keeps a split identical on every target; one team mask that movement and the bake both read keeps bots and
      humans under one rule.
    - Ids: none of its own (no research finding on economy phases or spawning); cites R1-TICK-REC3 (cadences).
    - Need: VISION_V2 §6.5; spawning, spawn protection, the Recruit respawn and the barracks' team mask: LOOP_V2 §2
      and §5 only.
END

### FIX GD-8 (the pending offer of three; the record keyed by slot) + GD-REJOIN (part 18, first lines)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
18. **The player record, weapon drops and pickups (UPGRADE-SCOPE).** Each player slot has a player record, the
    PlayerState, in a table beside the soldier tables and apart from the soldier entity, the Pawn. For the match it
    holds one track per unit type and one per weapon type (level, picks taken, the pending offer of three), keyed by the
AFTER:
18. **The player record, weapon drops and pickups (UPGRADE-SCOPE).** Each player slot has a player record, the
    PlayerState, in a table beside the soldier tables and apart from the soldier entity, the Pawn; it is keyed by slot,
    and whether a player who rejoins after ADR 0001's grace window gets it back is OQ-0004-10 (the key follows the
    answer). For the match it holds the player's money (part 17) and one track per unit type and one per weapon type
    (level, picks taken, and its open offers: a bounded queue whose length, like the offer's size, is data, so no
    number is hard-coded; when offers appear, how they wait and what a full queue does are OQ-0005-AB8's), keyed by the
END

### FIX GD-8 (money on the record: its starting amount, part 18)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
    start the sim builds every slot's record empty, with the soldier tables, so nothing carries over from a previous
AFTER:
    start the sim builds every slot's record afresh (every track empty, money at LOOP_V2 §3's starting $300), with the
    soldier tables, so nothing carries over from a previous
END

### FIX GD-8 (the offer of three; the pickup pool) + GD-6 (part 18) + GD-REJOIN
FILE: docs/adr/0004-simulation-core.md
BEFORE:
    on one pickup in one tick always resolve the same way on every machine. The offer of three is drawn on the authority
    from the authority-only offers stream (part 8) when a track levels; the server refuses a pick it did not offer and a
    pickup it did not place (ADR 0001). Picks, offers shown, weapons looted and time per weapon type are events for
    telemetry (part 19; ADR 0006's fields). The rules stay open: carry slots, ammo for looted weapons and how fast tracks
    level (ADR 0005's open questions); which deaths drop which weapons, how long a drop lies, who may pick it up and
    which order breaks a tie (OQ-0005-AB9); and what bots and slot takeovers do with records (OQ-0005-AB10).
AFTER:
    on one pickup in one tick always resolve the same way on every machine. When a track levels, the authority draws an
    offer (UPGRADE-SCOPE's one of three, its size from data) from the authority-only offers stream (part 8) into that
    track's queue; the server refuses a pick it did not offer and a pickup it did not place (ADR 0001). Picks, offers
    shown and weapons looted are tick-stamped events (part 19), from which time per weapon type follows, and ADR 0006's
    record needs them with their ticks. A full pickup pool follows part 1's overflow rule (OQ-0004-11). The rules stay
    open: carry slots, ammo for looted weapons and how fast tracks level (ADR 0005's open questions); which deaths drop
    which weapons, how long a drop lies, who may pick it up and which order breaks a tie (OQ-0005-AB9); what bots and
    slot takeovers do with records (OQ-0005-AB10); and whether a late rejoiner gets their record back (OQ-0004-10).
END

### FIX GD-6 (part 19)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
    purchases, uplink cache events, harvester deliveries and kills, structure kills and stage changes, tech tiers,
    upgrade picks and pickups. The event list is part of the state and its hash; ADR 0002 places it in the output block,
AFTER:
    purchases, uplink cache events, harvester deliveries and kills, structure kills and stage changes, tech tiers,
    upgrade picks (slot, track, level, choice) and pickups (slot, weapon type). Every event carries its tick, so the
    match record can place each pick and pickup on the timeline beside the kills, not only count them: OQ-0005-AB3's
    two measurements (the time to kill between a fully levelled and a fresh player, and how long the way back takes)
    need each player's levels at each kill (VISION_V2 §6.6: "Snowball and counters get tuned on evidence"). The event
    list is part of the state and its hash; ADR 0002 places it in the output block,
END

### FIX RR-3 (0004 half) + RR-9 (part 20)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
    R1-DET-F14). On the server a panic stops that room's match (each room is its own process, R2-ROOM-REC2); in the
    browser `-Cpanic=abort` ends the sim's instance. Online, after a trap, the Worker's host starts a fresh instance and
    rejoins through ADR 0001's resync path, and the trap is reported (R6-OBS-REC3, ADR 0006); the panic strategy is ADR
    0002's. In local play the trapped instance was the authority; recovery there is not researched (OQ-0004-7).
    - Why: a panic on the server is a denial of service for a whole room, and in the browser it ends the match for that
      player.
    - Ids: R1-RR-OQ3, R1-XT-OQ12, R2-XT-OQ12, R6-XT-OQ6 (r1-simulation-core.md, lines 2936–2940, 3084–3087;
      r2-netcode.md, lines 3395–3398; r6-infrastructure.md, lines 1021–1024).
AFTER:
    R1-DET-F14). On the server a panic in the tick ends that room's match and its process (each room is its own
    process, R2-ROOM-REC2). The room keeps `panic = "unwind"`, so that a panic in one connection's work closes only that
    connection (R2-ROOM-REC2's fix pass); but an uncaught panic ends only its own thread (R2-ROOM-F8), so a tick thread
    left to die would leave a room that still holds its connections and sends nothing. The room therefore catches a
    panic at the top of its tick loop (`catch_unwind`, R2-ROOM-F8) and treats it as fatal: it reports it (ADR 0006),
    tells its clients the match ended where it can (ADR 0001's room lifecycle), and exits non-zero, so the supervisor
    records the exit and the VM's other rooms run on. The tick thread and the network runtime share only bounded
    channels, never a lock the tick unwraps (a lock poisoned by a panicking connection would turn that panic into the
    room's). Whether a room may instead resume from a snapshot is the Advisor's question in ADR 0001. In the browser
    `-Cpanic=abort` ends the sim's instance, and the trap alone carries no panic message, only `unreachable` and
    frames; so the Wasm build installs a small panic hook of its own (`std::panic::set_hook`, no extra crate) that
    writes the message and location into a fixed buffer in linear memory before the abort, and the Worker's host reads
    it after the trap and attaches it to the report (proposed: a native scratch build on rustc 1.97.0 ran such a hook
    under `panic = "abort"` and kept the message; the Wasm build is S20's to check). Online, after a trap, the Worker's
    host starts a fresh instance and rejoins through ADR 0001's resync path, and the trap is reported (R6-OBS-REC3, ADR
    0006); the panic strategy is ADR 0002's. In local play the trapped instance was the authority; recovery there is
    not researched (OQ-0004-7).
    - Why: a panic on the server is a denial of service for a whole room, and in the browser it ends the match for that
      player; a room that outlives its tick, or a trap without its message, hides the fault.
    - Ids: R1-RR-OQ3, R1-XT-OQ12, R2-XT-OQ12, R6-XT-OQ6 (r1-simulation-core.md, lines 2936–2940, 3084–3087;
      r2-netcode.md, lines 3395–3398; r6-infrastructure.md, lines 1021–1024); cites R2-ROOM-REC2 and R2-ROOM-F8
      (r2-netcode.md, lines 2762–2768, 1459–1467).
END

### FIX GD-4 + FC-1 (rows GE9 and GE10)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| GE9 (tech boxes, line 295) | redesign, look brief pending | redesign; look brief pending lifted | LOOP_V2 §4 `[DEFAULT]` replaces tech boxes with uplinks, a sim phase (part 17: cache, carry, place, countdown, steal by GE8's presence count, rotation) whose cache events part 19 lists; the two lessons the row names are met: if the next site is drawn at random, the draw comes from a seeded sim stream, never `Math.random` (part 8), and "can't shoot while carrying" binds bots as it binds humans, since bots send the same commands (part 15); the note is lifted because the box's mesh, world bar and HUD go with the box while that `[DEFAULT]` stands, not because LOOK_V2 sets them; the uplink's world marker (GE7, line 293) and its HUD are the presentation half, ADR 0005's and ADR 0003's |
| GE10 (silos, line 296) | redesign, look brief pending | redesign; look brief pending lifted | LOOP_V2 §4 `[DEFAULT]` removes silos, so no silo rule reaches the sim; the row keeps the presence count (GE8, the steals of part 17) and hands the "income never zero" role to the safe near-base field (LOOP_V2 §3), a money rule of part 17; the note is lifted because the pads and the HUD's silo dots and rate go with the silos while that `[DEFAULT]` stands, not because LOOK_V2 sets them |
AFTER:
| GE9 (tech boxes, line 295) | redesign, look brief pending | redesign; look brief pending lifted | LOOP_V2 §4 `[DEFAULT]` replaces tech boxes with uplinks, a sim phase (part 17: cache, carry, place, countdown, steal by holding E (LOOP_V2 §4 step 4), with GE8's presence count kept as a primitive whose part in a steal is OQ-0004-8, rotation) whose cache events part 19 lists; the two lessons the row names are met: if the next site is drawn at random, the draw comes from a seeded sim stream, never `Math.random` (part 8), and "can't shoot while carrying", and whatever the steal channel blocks, binds bots as it binds humans, since bots send the same commands (part 15); the note is lifted because the box's mesh, world bar and HUD go with the box while that `[DEFAULT]` stands, not because LOOK_V2 sets them; the uplink's world marker (GE7, line 293) and its HUD are the presentation half, ADR 0005's and ADR 0003's |
| GE10 (silos, line 296) | redesign, look brief pending | redesign; look brief pending lifted | LOOP_V2 §4 `[DEFAULT]` removes silos, so no silo rule reaches the sim; the row keeps the presence count (GE8, a primitive part 17's steal may read, OQ-0004-8) and hands the "income never zero" role to the safe near-base field (LOOP_V2 §3), a money rule of part 17; the note is lifted because the pads and the HUD's silo dots and rate go with the silos while that `[DEFAULT]` stands, not because LOOK_V2 sets them |
END

### FIX GD-STEAL + GD-CARRIER + GD-REJOIN + POOL-FULL + GD-BOT-LOOT (new open questions OQ-0004-8 to -12)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
  - What decides it: whether the skeleton keeps a copy of state or inputs outside the sim's instance.
  - Research ids: R1-RR-OQ3, partly (r1-simulation-core.md, lines 2936–2940).
AFTER:
  - What decides it: whether the skeleton keeps a copy of state or inputs outside the sim's instance.
  - Research ids: R1-RR-OQ3, partly (r1-simulation-core.md, lines 2936–2940).
- **OQ-0004-8. How does an uplink steal meet defenders, and two enemy teams?**
  - Why it matters: LOOP_V2 §4 step 4 (`[DECIDED]`) says only that enemies steal an uplink by holding E for a few
    seconds. What defenders standing on it do, and who wins when two enemy teams try at once (three-team modes only),
    decide how safe a lead is (VISION_V2 §2, pillars 1 and 2). Legacy's contest rules belong to the tech boxes and silos
    that LOOP_V2 §4 replaces (`docs/LEGACY_INVENTORY.md`, row GE8).
  - Options: (a) holding E only, presence ignored (LOOP_V2's words as written); (b) a defender inside the circle pauses
    the steal; (c) a defender blocks it and its progress decays (legacy's tech-box rule, GE8). For two enemy teams: the
    first to finish takes the uplink, or each pauses the other. Part 17 builds the presence rule as a data switch, so
    each option is data, not code.
  - Decider: Mustafa, with the game-designer.
  - What decides it: the uplinks change-set (LOOP_V2 §11, step 5), from sims of (a) and (b), 3 runs each in the
    three-team mode, reading steals and late steals among LOOP_V2 §11's objective counters.
  - Research ids: none (a design question).
- **OQ-0004-9. Is a carried tech cache's position public?**
  - Why it matters: LOOP_V2 §4 step 1 tells everyone where a cache spawns, not whether its carrier stays visible. A
    carrier hidden like any soldier ends the chase that makes steals possible; a public carrier, slow and unable to
    shoot (step 2), may rarely reach an uplink.
  - Options: (a) public while carried; (b) public at spawn and at placement only.
  - Decider: Mustafa.
  - What decides it: a design session, or the uplinks change-set's sims (cashouts and steals per match), with the carry
    speed as tuning in ADR 0005's tables; the answer sets the carried cache's visibility class in ADR 0001's state
    table.
  - Research ids: none (a design question); either answer is one of R2-INT-REC5's classes (r2-netcode.md, line 2502;
    ADR 0001's).
- **OQ-0004-10. Does a player who rejoins after the grace window get their player record back?**
  - Why it matters: UPGRADE-SCOPE says the tracks are owned by the player, but part 18 keys the record by slot. A
    player who returns after ADR 0001's reconnect grace window joins as a new player, with no claim on the old slot's
    record, which the bot that took the slot over now holds, its tracks and its money (R2-ROOM-REC4: "after the window
    the slot follows the bot-fill rule").
  - Options: (a) the record stays with the slot (part 18 as written); (b) the room keeps a departed player's record by
    player id until the match ends and restores it on rejoin, the slot's bot starting a record of its own; its cost is
    one record per departed player, in a table whose capacity is data set at match start, so no client can grow it
    (part 18's rule).
  - Decider: Mustafa, with OQ-0005-AB10 (what a human gets on taking a bot's slot).
  - What decides it: a design session with OQ-0005-AB10; the record's key (slot or player id) follows the answer.
  - Research ids: none (a design question); it cites R2-ROOM-REC4 (r2-netcode.md, lines 2812–2815; ADR 0001's).
- **OQ-0004-11. When a pool is full, which row gives way?**
  - Why it matters: a step with no heap allocation in steady state (part 6) needs a fixed capacity per pool (part 1).
    When a burst fills one (live projectiles, active effects, weapon pickups), its rule decides who wins a pile-up:
    whose grenade, whose slow, whose dropped gun gives way. The rule must be the same on every target.
  - Options: (a) the new row is refused and counted (nothing that already exists changes); (b) the oldest row, by
    start tick and then sequence number (part 4), ends early. Each kind may take its own: active effects beside ADR
    0005 AB-1's stacking policy, pickups beside OQ-0005-AB9's lifetime.
  - Decider: the game-designer proposes per kind; Mustafa confirms where a player could notice.
  - What decides it: each pool's worst case from the content set (ADR 0005's whole-set check) and the game-designer's
    caps (R2-BUD-OQ5), after which the stress scene's counts gain pickups; until it is answered the spikes use (a) as
    a test fixture, not a rule.
  - Research ids: none (no research sizes a pool); it cites R1-ABL-REC6 (r1-simulation-core.md, lines 2465–2480) and
    R2-BUD-OQ5 (r2-netcode.md, lines 2982–2985).
- **OQ-0004-12. Do bots take world pickups?**
  - Why it matters: bots fill most slots in the start modes (VISION_V2 §3), and LOOP_V2 §10's behaviour list was
    written before UPGRADE-SCOPE added loot; looting bots change what the drop rules do to balance.
  - Options: (a) no, bots keep their class weapons; (b) yes, by the human rules.
  - Decider: Mustafa.
  - What decides it: the slice 2 plan's behaviour set (OQ-0004-1), revisited with OQ-0005-AB9 and AB10.
  - Research ids: none (a design question).
END

### FIX TE-1 (S1, and S2 by the same reasoning)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| S1 | The sim fits the sim step share at N = 48 | 6 | p99 sim step time ≤ 8.33 ms at 30 Hz (≤ 4.17 ms for the 60 Hz margin run); mean, p50, p95, p99 and max reported per N at N = 12, 24, 36, 48 and the N = 64 [headroom](../research/GLOSSARY.md#the-design-point-and-headroom) run; 36,000 ticks per run (20 minutes of game time at 30 Hz, 10 at 60 Hz: arithmetic), 3 runs; each part of the share reported on its own against its ceiling (R1-TICK-REC2 P1, r1-simulation-core.md lines 2044–2053; R1-ECS-REC1 mark 1, lines 1923–1931; GLOSSARY.md lines 53–69) | the sim core spike |
| S2 | The browser runs the whole sim within the client's sim share | 6 | p99 sim step time ≤ 13.33 ms at 30 Hz in Wasm in a dedicated Worker on the minimum laptop while the page holds 30+ fps on Low for the whole run; Wasm ≤ 2 × native (R1-TICK-REC2 P2, lines 2054–2057; R1-ECS-REC1 mark 1); N for the pass from R1-TICK-OQ7 (a design question), the scale steps reported meanwhile | the sim core spike's sim in the boundary spike's Worker |
AFTER:
| S1 | The sim fits the sim step share at N = 48 | 6 | p99 sim step time ≤ 8.33 ms at 30 Hz (≤ 4.17 ms for the 60 Hz margin run); mean, p50, p95, p99 and max reported per N at N = 12, 24, 36, 48 and the N = 64 [headroom](../research/GLOSSARY.md#the-design-point-and-headroom) run; 36,000 ticks per run (20 minutes of game time at 30 Hz, 10 at 60 Hz: arithmetic), 3 runs; each part of the share reported on its own against its ceiling (R1-TICK-REC2 P1, r1-simulation-core.md lines 2044–2053; R1-ECS-REC1 mark 1, lines 1923–1931; GLOSSARY.md lines 53–69); the sim core spike builds no ability and effect step (S15 follows it) and no economy, objectives, moments or events (slice 4), so its run stands in for each: 384 rows in the effect pool (so the snapshot, the hash and memory carry them) with the effect step's 1 ms ceiling burned per tick, and the 2.80 ms remainder topped up per tick after the built projectiles and bot decisions, each stand-in named in the run's record and scaled by N / 48 at the other scale steps (proposed: a pass with the stand-ins still holds once the real parts meet their ceilings; a run that fails only through them is reported so, and the share's verdict then waits for S15's step); the built parts are judged against their own ceilings either way; S1 on the real crate is migration phase 8's; the 60 Hz margin run's load is ADR 0001's (OQ-0001-2, TICK-60-SETUP), and at 60 Hz the same two stand-ins would be 1 ms and 0.30 ms (4.17 − 3.87, arithmetic) | the sim core spike |
| S2 | The browser runs the whole sim within the client's sim share | 6 | p99 sim step time ≤ 13.33 ms at 30 Hz in Wasm in a dedicated Worker on the minimum laptop while the page holds 30+ fps on Low for the whole run; Wasm ≤ 2 × native (R1-TICK-REC2 P2, lines 2054–2057; R1-ECS-REC1 mark 1); N for the pass from R1-TICK-OQ7 (a design question), the scale steps reported meanwhile; the wave-1 run lacks the same parts as S1's, and the client's sim share has no per-part split to burn, so it reports the built sim's time and its pass waits for S15's step as well | the sim core spike's sim in the boundary spike's Worker |
END

### FIX GD-8 (money on the record, S16)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| S16 | The player record behaves as UPGRADE-SCOPE says | 18 | no research mark; proposed (UPGRADE-SCOPE's sentences as checks): over scripted deaths and unit switches, 0 track entries lost or changed; a weapon type's track resumes when the next gun of that type is picked up; every record is empty at the start of every match; pickups keep their ids through snapshot and restore; two claims on one pickup in one tick resolve the same way on native and Wasm (the state hash). test-engineer sets the numbers | the content and ability spike |
AFTER:
| S16 | The player record behaves as UPGRADE-SCOPE says | 18 | no research mark; proposed (UPGRADE-SCOPE's sentences as checks): over scripted deaths and unit switches, 0 track entries lost or changed; a weapon type's track resumes when the next gun of that type is picked up; every record's tracks are empty and its money at the starting amount at the start of every match; pickups keep their ids through snapshot and restore; two claims on one pickup in one tick resolve the same way on native and Wasm (the state hash). test-engineer sets the numbers | the content and ability spike |
END

### FIX NA-8 (S18)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| S18 | The Worker paces the tick in every browser | 5 | on Chrome, Edge, Firefox and Safari: foreground tick start jitter p99 ≤ 4 ms over 10 minutes; per-frame displacement of a constant-speed unit with a standard deviation ≤ 5 % of its mean on 60, 120 and 144 Hz displays; after 60 s hidden, local play ran no tick and resumes where it stopped, and online the client is back in step within 0.5 s and never runs more than 7 ticks in one frame (proposed restatement of 15 at 60 Hz) (R1-TICK-REC5, lines 2109–2114); the same after more than 5 minutes hidden (R1-BND-OQ2, lines 2724–2727) | the boundary spike (ADR 0002) |
AFTER:
| S18 | The Worker paces the tick in every browser | 5 | on Chrome, Edge, Firefox and Safari: foreground tick start jitter p99 ≤ 4 ms over 10 minutes, read online against the schedule the server-time estimate sets (part 5); per-frame displacement of a constant-speed unit with a standard deviation ≤ 5 % of its mean on 60, 120 and 144 Hz displays; after 60 s hidden, local play ran no tick and resumes where it stopped, and online the client is back in step within 0.5 s and never runs more than 7 ticks in one wake of the Worker (proposed restatement of 15 at 60 Hz; the research's "per frame") (R1-TICK-REC5, lines 2109–2114); the same after more than 5 minutes hidden (R1-BND-OQ2, lines 2724–2727) | the boundary spike (ADR 0002) |
END

### FIX RR-9 (S20)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| S20 | No panic from input or content; recovery online | 20 | zero panics over the fuzzed content corpus (ADR 0005's mark, R1-RR-OQ3) and over fuzzed commands fed to the sim (proposed: part 20's rule needs its own test, the decoders being ADR 0001's); a trap injected into the client's Worker ends in a fresh instance back in step through ADR 0001's resync mark, and the trap reaches error reporting (R6-OBS-REC3) | the sim core spike with the boundary spike |
AFTER:
| S20 | No panic from input or content; recovery online | 20 | zero panics over the fuzzed content corpus (ADR 0005's mark, R1-RR-OQ3) and over fuzzed commands fed to the sim (proposed: part 20's rule needs its own test, the decoders being ADR 0001's); a trap injected into the client's Worker ends in a fresh instance back in step through ADR 0001's resync mark, and the trap reaches error reporting with the panic message and location from part 20's hook (R6-OBS-REC3; the hook proposed in part 20) | the sim core spike with the boundary spike |
END

### FIX TE-6 (migration phase 6)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
| 6 | Slice 4, the loop: economy and objective phases, the ability system's execution, player records, pickups and offers (after ADR 0005's upgrade questions, OQ-0005-AB9 and AB10 among them), events and moments, structure states and damage stages (after OQ-0004-4) | S13, S15 and S16 on the real crate; a recorded full match replays with identical per-tick hashes on every target |
AFTER:
| 6 | Slice 4, the loop: economy and objective phases, the ability system's execution, player records, pickups and offers (after ADR 0005's upgrade questions, OQ-0005-AB9 and AB10 among them), events and moments, structure states and damage stages (after OQ-0004-4) | S13, S15 and S16 on the real crate; a recorded full match replays with identical per-tick hashes on every target; scripted cases for the rules LOOP_V2 §2 to §5 state, held as v2's own snapshots (ADR 0002): the $300 start, a delivery paying every player on the team, a rebuild's integer split and its remainder, power-plant pricing, the minute-5 Tech 1 floor (9,000 ticks at 30 Hz: 5 × 60 × 30, arithmetic), a steal by holding E under each setting of OQ-0004-8's switch, 3 s of spawn protection (90 ticks at 30 Hz: 3 × 30, arithmetic), an enemy stopped at a barracks opening, a death respawning as the Recruit with money kept; GE8's presence count against its legacy golden (ADR 0002) |
END

### FIX TE-6 (test plan)
FILE: docs/adr/0004-simulation-core.md
BEFORE:
- **The determinism suite** (part 9) on every sim change, with ADR 0002's tools (R5-TEST-REC2): recorded matches run
  natively twice in separate processes, as Wasm in Node.js, and as Wasm in Chromium, Firefox and WebKit, with real
  Safari through its WebDriver; a small gate set on every change and a nightly 10-minute bot match; the first mismatch
  reported with a field-level diff; the injected-divergence tests.
AFTER:
- **The determinism suite** (part 9) on every sim change, with ADR 0002's tools (R5-TEST-REC2): recorded matches run
  natively twice in separate processes, as Wasm in Node.js, and as Wasm in Chromium, Firefox and WebKit, with real
  Safari through its WebDriver; a small gate set on every change and a nightly 10-minute bot match; the first mismatch
  reported with a field-level diff; the injected-divergence tests, among them one that changes only one event (a kill
  credited to the wrong slot) and one that changes only one economy value (one player's money, by $1), each caught at
  the tick it happens (proposed: they show that the state hash covers the event list and the economy, parts 17 and
  19).
END
## Notes and routes (one per finding; every finding above is fixed, none deferred)

### NOTE EA-3
Fixed in part 3 (the authority step and the prediction step over the same phase functions; the prediction step draws
from no stream). Routed: PM-0004-29 (test-engineer) names "the prediction step (part 3)" for its instance in place of
"the client-mode instance built as ADR 0001's prediction scope makes it"; ADR 0001 part 4 (netcode-architect) may write
"replays the newer commands through the sim crate's prediction step (ADR 0004 part 3)".
END

### NOTE GD-4 + FC-1
Fixed in part 17 and rows GE9 and GE10 with FC-1's smallest fix (hold E per LOOP_V2 §4 step 4; "whatever the steal
channel blocks" binds bots) and GD-4's (GE8's count kept as a primitive; the defender and two-team cases open, as
OQ-0004-8). The presence rule is built as a data switch, so no option needs code.
END

### NOTE GD-6
Fixed in part 19 (picks with slot, track, level and choice; pickups with slot and weapon type; every event with its
tick) and part 18 (time per weapon type follows from the events). Routed: ADR 0006 part 12 (infra-engineer): its
tick-stamped event list adds "upgrade picks (slot, track, level, choice) and pickups (slot, weapon type)"; the per-slot
totals may stay as a summary.
END

### NOTE GD-7
Fixed in part 17 (spawning inside the barracks, 3 s spawn protection as an effect, the Recruit respawn, and a team mask
on a collider's movement flag that movement and the bake read). Routed: docs/adr/README.md (engine-architect), the list
of features whose need lies outside VISION_V2 §6, gains: "- **ADR 0004:** spawning in the barracks, spawn protection,
the Recruit respawn and the barracks' team mask for movement and the bake (part 17): LOOP_V2 §2 and §5 only." ADR 0005
(game-designer): AB-1's closed set should hold what spawn protection needs (damage immunity, or a damage scale of 0)
for 3 s; if it does not, that is one reviewed Rust operation.
END

### NOTE GD-8 (the 0004 items)
Fixed: the offer's size and the open-offer queue are data (parts 8 and 18); money is an integer on the player record,
the rebuild split's rounding and remainder are data applied in slot order (parts 7, 17), the record starts with the
$300 (part 18, S16); the record keyed by slot is OQ-0004-10; the pickup pool's capacity and full-pool rule are part 1
and OQ-0004-11. Routed: OQ-0005-AB8 (game-designer) adds "and what a full offer queue does" beside its queue option;
PM-0004-65 (test-engineer) reads "empty tracks and money at its starting amount" for "empty records", as S16 now does.
GD-8's ADR 0001 items are netcode-architect's.
END

### NOTE SA-15
Fixed in part 8 (the OS secure random source, drawn by the host and passed in; never sent, logged or written into the
match record). Routed: ADR 0006 part 12 (infra-engineer): the match record never holds the seed. Optional for
test-engineer: a bite in PM-0004-29 with a seed derived from the room id.
END

### NOTE FB-10
Fixed: "no Rust navigation crate read promises determinism (R4-NAV-F16)", as 0004's Context already says.
END

### NOTE NA-8
Fixed in part 5 and S18 (the online schedule from the server-time estimate; catch-up per wake of the Worker). Routed:
ADR 0002 part 5 (engine-architect): the step call also returns when its next tick is due (online from the client
endpoint's server-time estimate, in local play from the accumulator); PM-0004-68 (test-engineer): online, start jitter
is read against that schedule.
END

### NOTE RR-3 (the 0004 half)
Fixed in part 20, on R2-ROOM-F8 (r2:1459–1467: an uncaught panic ends only its own thread) and R2-ROOM-REC2's fix pass
(r2:2765–2768: the room keeps `panic = "unwind"`). Routed: ADR 0001 (netcode-architect) holds ROOM-CRASH as its open
question for the Advisor; part 20's "the Advisor's question in ADR 0001" then takes its number. The mark beside
PM-0001-26 is test-engineer's (unit A): a panic injected into the sim step ends the process with a non-zero exit, every
client gets the match's end or a close, and the supervisor logs the exit; bite: a tick thread whose panic leaves the
process running.
END

### NOTE RR-9
Fixed in part 20 and S20 (our own small hook, no extra crate, into a fixed buffer). Evidence: a scratch build
(`.gate/rr-final-panichook`, rustc 1.97.0, x86-64 Linux, `panic = "abort"`): the hook wrote "panicked at
src/main.rs:35:21: index out of bounds: the len is 3 but the index is 6" (83 bytes) into a 256-byte static buffer
through a fixed-buffer `fmt::Write` (no heap buffer of its own), then the process aborted (exit 134). Not checked: the wasm32 target (not installed here), and that the
host can read linear memory after a trap. Routed: ADR 0002 part 4 (engine-architect): the Worker's host reads the
hook's buffer after a trap and attaches it to the report; PM-0004-61 and PM-0002-33 (test-engineer) check that the
message arrives.
END

### NOTE RR-4 + POOL-FULL
Fixed in part 1 (capacities from the mode and content at match start, one overflow rule per pool, a counter) and
OQ-0004-11 (the rule per kind: the game-designer proposes, Mustafa confirms). Routed: ADR 0005 AB-1 (game-designer):
the effect pool's overflow points to OQ-0004-11 rather than a second question; ADR 0005 CD-2 (engine-architect): the
whole-set check reports each pool's worst case; PM-0004-6 (test-engineer): the burst bite (one second at twice the
stress scene's projectile and effect counts: 0 allocations, equal hashes on native and Wasm) and a full-pool row once
OQ-0004-11 is answered.
END

### NOTE TE-1 (the 0004 part)
Fixed in S1 (stand-ins for the parts the sim core spike does not build, named in the run's record) and S2 (no per-part
split to burn, so its pass waits for S15's step). Disagree with one number in the finding's fix: "half of each at
60 Hz". Part 6 sets the ability, effect and weapon part at 1 ms at both rates (its 60 Hz sum counts it at 1 ms:
1 + 0.2 + 1 + 0.83 + 0.83 = 3.87 of 4.17 ms), and the 60 Hz remainder is 4.17 − 3.87 = 0.30 ms, not half of 2.80
(1.40 ms, which would overrun the share by itself). Routed: SPIKE_PLAN §2 and §4 (engine-architect) and OQ-0001-2 (a)
(netcode-architect) use 1 ms and 0.30 ms at 60 Hz if the Advisor picks the stand-ins (the test-engineer's Q1);
PM-0004-2 (test-engineer) counts the 384 stand-in effect rows as the stress scene's effects in the wave-1 run.
END

### NOTE TE-6
Fixed in the test plan (one divergence only in an event, one only in an economy value, each caught at its tick) and
migration phase 6 (scripted cases for LOOP_V2 §2 to §5's rules). No S claim is added: no spike builds parts 17 and 19,
which arrive in slice 4. Routed: the pass marks' "What has no measurable mark here" lines for parts 17 and 19
(test-engineer) point to these two.
END

### NOTE GD-STEAL, GD-CARRIER, GD-REJOIN, GD-BOT-LOOT (questions)
Carried as OQ-0004-8 (steals: Mustafa with the game-designer), OQ-0004-9 (a carried cache's position: Mustafa),
OQ-0004-10 (a late rejoiner's record: Mustafa, with OQ-0005-AB10) and OQ-0004-12 (bots and pickups: Mustafa). The
reviewers' recommendations, for "Decisions needed" (not written into the draft): GD-STEAL, build the switch and decide
from sims of (a) and (b); GD-CARRIER (a); GD-REJOIN (b); GD-BOT-LOOT (a) for the slice 2 greybox. Routed:
docs/adr/README.md (engine-architect): five register rows, OQ-0004-8 to -12, and its total line +5 (numbered and open);
netcode-architect: its GD-REJOIN points to OQ-0004-10 instead of a second question (ADR 0001 keeps the grace window), and
the uplinks row of ADR 0001's state table names OQ-0004-9 for the carried cache; game-designer: OQ-0005-AB10 may cite
OQ-0004-10 and -12.
END
