# ADR 0001, the final fix round (netcode-architect, plan step 10's fix loop), 2026-11-02

Doc: `docs/adr/0001-multiplayer-netcode.md` at 47eadd9 (unchanged at HEAD; `git diff 47eadd9 HEAD` on it is empty).
Every BEFORE was copied by script from the doc's whole lines at 47eadd9; each occurs exactly once in the doc, the
ranges do not overlap, and applying the blocks in file order on a copy succeeds (each BEFORE is still unique when its
turn comes). No block edits "### Pass marks" (lines 1018–1270), which is test-engineer's. The doc grows by about 150
lines; pointers into it are anchored to parts and ids. No new sentence sits directly under a table. The NOTE blocks
after the FIX blocks give, per finding id, what was done, what is deferred and what other authors' text must follow.
Includes the coordinator's routed items: from rust-reviewer's ADR 0004 batch (b815c85): ROOM-CRASH as OQ-0001-18,
GD-REJOIN pointing to OQ-0004-10, the uplinks state row naming OQ-0004-9, OQ-0001-2 (a) worded as ADR 0004's S1
stand-ins, part 4 naming ADR 0004 part 3's prediction step; from infra-engineer's ADR 0006 batch (8fa7173): the key
per environment, the registrable domain (OQ-0006-15), the aim side file as ADR 0006 part 9 sizes it, one report
endpoint on the lobby, the room directory's build field (OQ-0006-14), part 10 pointing to OQ-0006-16 (so ADR 0001
adds no port question of its own); from engine-architect's ADR 0002 batch (not yet applied): the event-release rule,
the arrival stamps, previous send time and current time as plain values, the ping's hold time, the newest input
posted once per drawn frame, the timers in `room` and in `server`, staging certificates per OQ-0006-5, and the
`client-endpoint` crate. New open questions: OQ-0001-15 to OQ-0001-18.

### FIX NA-12
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
   Snapshots go out on every tick (30 Hz); inputs are sampled and sent every tick. 60 or 64 Hz is asked only if the
AFTER:
   Snapshots go out 30 times a second: on every tick at 30 Hz, and on every second tick in the 60 Hz margin run
   (R2-BUD-REC1's snapshot half, below); inputs are sampled and sent every tick. 60 or 64 Hz is asked only if the
END

### FIX NA-3, NA-10 (its 0001 half), GD-3, GD-2 (purchases never predicted), routed: ADR 0004's prediction step, ADR 0002's client-endpoint crate
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
4. **The prediction scope.** The client's Worker predicts, for the local player only,
   [the prediction scope](../research/GLOSSARY.md#the-prediction-scope): its own movement, its own weapon state (ammo,
   fire timers, reload and active reload), the vehicle it pilots, and cosmetic shot feedback. On every snapshot it
   resets the local player (and its vehicle) to the server's state for the last command the server ran and replays
   the newer commands through the same Rust code (Quake III's method); it skips the replay when the server's state
   matches the stored prediction. The replay covers 200 ms of game time (6 ticks at 30 Hz) within 1 ms p99 on the
   minimum laptop (R1-TICK-REC2 P3). Nothing else is re-simulated: there is no whole-world replay, so ADR 0004 may drop
   R1-ECS-REC1 mark 3. A correction fades over a data-set time (proposed 100 ms), snaps above a data-set distance
   (proposed 2 m), and clears on respawn or teleport. The local player's own state is sent unquantised, or the server
   quantises its own copy the same way each tick. Never predicted: damage, deaths and hits on others; the uplink
   countdown and steals; upgrade offers and picks; weapon pickups (interim: OQ-0001-10); the player's own abilities
AFTER:
4. **The prediction scope.** The client's Worker predicts, for the local player only,
   [the prediction scope](../research/GLOSSARY.md#the-prediction-scope): its own movement, its own weapon state (ammo,
   fire timers, reload and active reload), the vehicle it pilots, and cosmetic shot feedback. It does so in the client
   endpoint, ADR 0002's `client-endpoint` crate (no bindings, transport or clock), which the native headless test
   clients run too. On every snapshot it
   resets the local player (and its vehicle) to the server's state for the last command the server ran and replays
   the newer commands through the same Rust code (Quake III's method), ADR 0004 part 3's prediction step: only the
   prediction scope's phases, for the local slot and its vehicle, against the static world and the movers as last
   decoded, drawing from no random stream; it skips the replay when the server's state
   matches the stored prediction. A replay emits no event its first run already emitted: the player's own cues are
   keyed by (entity, weapon, command sequence), so a replayed shot plays its muzzle flash and sound once ("Cues are
   events keyed by (entity, ability or weapon, command sequence)": r1-simulation-core.md, Recommendations, unit
   abilities, R1-ABL-REC4, lines 2427–2428), and the server never sends a client its own cosmetic shot events (part
   6). The replay covers 200 ms of game time (6 ticks at 30 Hz) within 1 ms p99 on the minimum laptop (R1-TICK-REC2
   P3); its real depth is the round trip plus the input buffer plus one tick, 5 to 7 ticks at P2 with a 1–3 tick
   buffer ((100 + 33.3 + 33.3) ÷ 33.3 to (100 + 100 + 33.3) ÷ 33.3, arithmetic) and deeper at P3 and P4, so the proof
   test records the depth per profile. Nothing else is re-simulated: there is no whole-world replay, so ADR 0004 may
   drop R1-ECS-REC1 mark 3. A correction fades over a data-set time (proposed 100 ms), snaps above a data-set distance
   (proposed 2 m), and clears on respawn or teleport. The snapshot's own-state section carries everything the replay
   reads for the local soldier and its vehicle: position and velocity, its weapon state, and the active effect rows
   (or the attribute values they produce) that own movement and own weapon state read, upgrade picks and ability
   effects included (ADR 0005 leaves how the predicting sim gets them to this record), so a pick or an effect that
   changes move speed or a fire interval causes no correction; that section is sent unquantised, or the server
   quantises its own copy the same way each tick, and like every field it is sent when it changes (its real size is
   measured, R2-XU-OQ15). Never predicted: damage, deaths and hits on others; the uplink countdown and steals;
   purchases; upgrade offers and picks; weapon pickups (interim: OQ-0001-10); the player's own abilities
END

### FIX GD-1, SA-8, NA-9, PA-3, NA-4 (its 0001 half), RR-2 (part 5)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
5. **Input commands, time and the aim point.** One command per tick, sent every tick in a datagram that repeats every
   command the server has not acknowledged, up to a cap (data, proposed 8). A command holds: its tick and sequence; the
   move vector; the button bitfield; a press counter per button (a resent command can neither lose nor double a
   press); the quantised world aim point (at most 80 bits); the view time (the interpolation tick plus a fraction).
   Each input datagram also carries the newest snapshot the client decoded, the acknowledgement deltas are built on.
   The server checks every command (every limit is data): its tick is not too far ahead of the server's and not older
   than the last command run; sequence numbers only move forward, wrap-around handled; the move vector's length is at
   most 1; a press counter advances by at most one edge per button for each tick the command covers; the aim point lies
   inside the map bounds and within the weapon's range; fire, ability and reload edges meet the server's own cooldown,
   ammunition and reload state; the acknowledged snapshot is one the server sent; a datagram with trailing bytes or
   unknown fields is dropped whole; the commands it runs per client per tick are capped, the catch-up cap no larger
   than the redundancy cap (the lag-switch guard). *Time:* the client estimates server time from its own ping samples
   (the browsers' `getStats()` is missing on Chrome and Edge and throws on Firefox), drifting at most 1–2 ms per frame,
   halving errors over 100 ms and resetting over 500 ms (Quake III's rules); the server keeps a per-client input
   buffer of 1–3 ticks (proposed), adapted per client; each command is stamped with the server tick it is meant for.
   When the server runs commands and how the client keeps them just in time are the proof test's to decide
   (OQ-0001-6); sub-tick fire stamps are OQ-0001-7. *The aim point:* the page posts, with its per-tick command, its
   cursor ray from the camera and the view time it drew (how they cross the boundary is ADR 0002's); the Worker casts
   that ray with the sim crate's ray query (ADR 0004) against the static world and the interpolated remote entities it
   published for that view time, and quantises the hit point into the network command. The server rebuilds the ray
   from its own fire origin at the command's tick (the `playerFireOrigin` rule, GC5) toward that point, extends it to
   the weapon's range and resolves it with part 8. Throws and charged shots take their heading and pitch from the same
   point. If gentle aim help onto bodies is adopted (LOOP_V2 §9 [OPEN]), the server applies it, never the client.
   *Why:* redundancy removes retransmission; a point keeps the player's intent at height and lets the server check it;
   the Worker holds the interpolated state, so it casts against what is drawn while the page makes no Wasm calls
   (R1-BND-REC1 mark 2). *Research:* R2-SYNC-REC2, R2-SYNC-REC8, R2-BUD-REC4 (the input buffer and the clock),
   R2-BUD-REC1 (changed: redundancy as R2-SYNC-REC2), R2-XU-OQ4, R1-XT-OQ1 = R2-XT-OQ1 = R4-XT-OQ1, R1-XT-OQ5 =
   R2-XT-OQ5, R1-TICK-OQ4, R1-TICK-OQ8; R4-LOS-REC3 (the same queries on both sides, ADR 0004's). *Need:* VISION_V2
   §6.2 (aiming at height), §6.3 (abilities as input edges), §4.
AFTER:
5. **Input commands, time and the aim point.** One command per tick, sent every tick in a datagram that repeats every
   command the server has not acknowledged, up to a cap (data, proposed 8). The repeats are delta-coded against the
   newer command beside them: option U2, which R2-BUD-REC1 takes up "once measured" (r2-netcode.md, Recommendations,
   Brief question 2, lines 2302–2303), adopted from the start because this command, with its aim point, is about twice
   R2's 8 B input, and sent whole at the cap it would break the upstream mark (the bandwidth estimate); the proof test
   measures both forms (R2-XU-OQ4). A command holds: its tick and sequence; the move vector; the button bitfield; a
   press counter per button (a resent command can neither lose nor double a press); the quantised world aim point (at
   most 80 bits); the view time (the interpolation tick plus a fraction). Each input datagram also carries the newest
   snapshot the client decoded, the acknowledgement deltas are built on. Each snapshot acknowledges two commands: the
   newest one the server received, which ends its repeats, and the last one it ran, the state the client's replay
   starts from (part 4); with only the second, the repeats would also span the input buffer's 1–3 ticks.
   The server checks every command (every limit is data): its tick is not too far ahead of the server's and strictly
   newer than the last command run, so a client whose clock runs fast never runs more commands than the server has
   ticked plus that lead bound (equal ticks would let it run the per-tick cap on every tick: the security review,
   2026-11-02); sequence numbers only move forward, wrap-around handled; the move vector's length is at most 1; a
   press counter advances by at most one edge per button for each tick the command covers; the aim point lies inside
   the map bounds (a point beyond the weapon's reach is clamped, not refused: below); fire, ability and reload edges
   meet the server's own cooldown, ammunition and reload state; the acknowledged snapshot is one the server sent; a
   datagram with trailing bytes or unknown fields is dropped whole; the commands it runs per client per tick are
   capped, the catch-up cap no larger than the redundancy cap (the lag-switch guard). *Time:* the client estimates
   server time from its own ping samples (the browsers' `getStats()` is missing on Chrome and Edge and throws on
   Firefox), drifting at most 1–2 ms per frame, halving errors over 100 ms and resetting over 500 ms (Quake III's
   rules). Every sample, and every jitter sample of part 7, is timed by its datagram's arrival: the Worker's host
   passes in, with each tick's input call, every datagram received since the last tick with the time it arrived, the
   time it sent the previous step's datagrams and its current time, all as plain values (ADR 0002 part 5), since
   timing a datagram at the tick that reads it would add up to one tick period (33.3 ms) of false jitter. The client
   endpoint reads no clock itself (std's clock panics on the browser's Wasm target: the Rust review of 2026-11-02,
   reading std's source). The server answers pings on its I/O runtime as they arrive; the client answers the server's
   at its next tick, and its pong carries the hold time, from the ping's arrival to the host's time at that step, which
   the server subtracts up to one tick period (more would only shrink the client's own RTT; holding a pong longer is
   the latency faking part 8 accepts). The server keeps a per-client input buffer of 1–3
   ticks (proposed), adapted per client; each command is stamped with the server tick it is meant for. When the server
   runs commands and how the client keeps them just in time are the proof test's to decide (OQ-0001-6); sub-tick fire
   stamps are OQ-0001-7. *The aim point:* the page posts its newest input once per drawn frame, with its cursor ray from
   the camera and the view time of the frame it drew (ADR 0002 part 5), and each tick the Worker builds the one network
   command from the newest post (running press totals, so a superseded post loses no press); it casts that ray with the
   sim crate's ray query (ADR 0004) against the static world and the interpolated remote entities it published for that view
   time, and quantises the hit point into the network command. The server rebuilds the ray from its own fire origin at
   the command's tick (the `playerFireOrigin` rule, GC5) toward that point, extends it to or cuts it at the weapon's
   range and resolves it with part 8. A point beyond the weapon's reach is clamped, never refused: a top-down cursor
   names points about 28 m from the look point, 44 m with the ADS pan (r2-netcode.md, Findings, R2-INT-F18, lines
   847–851), against legacy's Beam Katana range of 3.6 m and Molotov range of 0 (LEGACY_TUNING.md:181), and a clamped
   point gives a cheat nothing, since the server builds the ray from its own origin. Throws and charged shots take
   their heading and pitch from the same point; a melee swing and a throw stop at their own reach, and what a throw
   aimed past it does is rule data (ADR 0005). If gentle aim help onto bodies is adopted (LOOP_V2 §9 [OPEN]), the
   server applies it, never the client.
   *Why:* redundancy removes retransmission; a point keeps the player's intent at height and lets the server check it;
   the Worker holds the interpolated state, so it casts against what is drawn while the page makes no Wasm calls
   (R1-BND-REC1 mark 2). *Research:* R2-SYNC-REC2 (changed: an aim point beyond reach is clamped, a command's tick
   must be strictly newer), R2-SYNC-REC8, R2-BUD-REC4 (the input buffer and the clock), R2-BUD-REC1 (changed:
   redundancy as R2-SYNC-REC2, the repeats delta-coded as its U2), R2-XU-OQ4, R1-XT-OQ1 = R2-XT-OQ1 = R4-XT-OQ1,
   R1-XT-OQ5 = R2-XT-OQ5, R1-TICK-OQ4, R1-TICK-OQ8; R4-LOS-REC3 (the same queries on both sides, ADR 0004's). *Need:*
   VISION_V2 §6.2 (aiming at height), §6.3 (abilities as input edges), §4.
END

### FIX FA-L1 = A8, PA-2, NA-3, NA-13, NA-14, NA-16 (part 6)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
6. **Snapshots, encoding and events.** Each snapshot is a delta against the last snapshot that client acknowledged:
   unchanged entities cost nothing, new ones are coded from per-entity-type defaults, removals are explicit; with no
   usable baseline the server codes against the per-entity defaults. The server keeps one shared ring of per-tick world
   states and, per client, a ring of the snapshots it sent (data, proposed 32, about 1.07 s at 30 Hz). Encoding E3:
   positions to 1 cm in 16, 16 and 12 bits (±327.68 m across, 0–40.95 m of height), yaw in 16 bits, pitch in 10 bits,
   changed-entity lists. Projectiles are trajectory records (type, start tick, start position, velocity, fuse) re-sent
   only when the trajectory changes, evaluated by one shared trajectory function in the sim crate; a client gets a
   projectile only from the tick it becomes relevant, re-based to that tick. If the proof test counts any desync of
   client-side flight against the server, projectiles ride in every snapshot instead (E3b, about +42 kbps at N = 48).
   A per-client hard cap (proposed 256 kbps, counted at the UDP payload plus 28 B per datagram: about 1,067 B per
   snapshot interval at 30 Hz) is enforced through the priority accumulator (part 9). *Datagrams:* a snapshot datagram
   never exceeds min(the smallest `maxDatagramSize` the four browsers report, 1,150 B of WebTransport payload); the
   proof test's load uses 1,000 B until that is measured. A snapshot that does not fit splits by entity groups into
   datagrams that each decode alone, deltas against the same baseline, acknowledged per group. The first full state,
   at join and after a resync, goes over a reliable stream; deltas resume once the client acknowledges it. *Events:*
   tick-stamped gameplay events (the events of ADR 0004 part 19: hits, deaths and kills, purchases, uplink cache
   events, tech tiers, upgrade picks and pickups) ride in the
   snapshot's event section, filtered per client by visibility class, and are resent until the client's acknowledgement
   covers them; the per-client queue of unacknowledged events is bounded (data), and on overflow the server forces a
   full resync of that client or disconnects it. Cosmetic events (shots, muzzle flashes, sounds) go once. The shooter's
   hit confirmation is a gameplay event (LOOP_V2 §7). *Remote soldiers* carry position, yaw, pitch, health, flag bits
   for stance and action states (crouch, sprint, aiming down sights, reloading, carrying a cache) and the weapon id;
   action starts (fire, reload, throw, melee, an ability's cue) are tick-stamped cosmetic events. That is what the page
   animates remote soldiers from (what animation reads: ADR 0003; the block layout: ADR 0002), and it fits the
   estimate's 90-bit full and 60-bit delta soldier records, whose 8 flag bits change rarely. Vehicles carry hull and
   turret transforms and health per part; parts and seated players travel with their parent. *Wire widths:* entities
   are named by the sim's own id (kind, index, generation; never a library id); a record names kind and index, and the
   generation travels in the record that brings the entity into a client's set. The snapshot header carries the full
AFTER:
6. **Snapshots, encoding and events.** Each snapshot is a delta against the last snapshot that client acknowledged:
   unchanged entities cost nothing, new ones are coded from per-entity-type defaults, removals are explicit; with no
   usable baseline the server codes against the per-entity defaults. The server keeps one shared ring of per-tick world
   states and, per client, a ring of the snapshots it sent (data, proposed 32, about 1.07 s at 30 Hz). Encoding E3:
   positions to 1 cm in 16, 16 and 12 bits (±327.68 m across, 0–40.95 m of height), yaw in 16 bits, pitch in 10 bits,
   changed-entity lists. Projectiles are trajectory records (type, start tick, start position, velocity, fuse) re-sent
   only when the trajectory changes, evaluated by one shared trajectory function in the sim crate; a client gets a
   projectile only from the tick it becomes relevant, re-based to that tick. A desync of client-side flight against
   the server is a defect of that function (ADR 0004's PM-0004-73 holds it bit-identical on every target), fixed
   there, not a reason to change the encoding: whole, E3b (projectiles in every snapshot) cannot pass the budgets
   (about 188.3 kbps at N = 48 in R2's estimate, above the 160 kbps mean: r2-netcode.md line 1779; in the stress
   scene 300 live projectiles × 5.5 B = 1,650 B per snapshot, above the 1,067 B cap before any soldier: arithmetic on
   line 1755), so it is kept only per projectile kind: a kind whose flight still desyncs rides in every snapshot, its
   live count × 5.5 B shown against the cap.
   A per-client hard cap (proposed 256 kbps, counted at the UDP payload plus 28 B per datagram: about 1,067 B per
   snapshot interval at 30 Hz) is enforced through the priority accumulator (part 9). *Datagrams:* a snapshot datagram
   never exceeds min(the smallest `maxDatagramSize` the four browsers report, 1,150 B of WebTransport payload); the
   proof test's load uses 1,000 B until that is measured. A snapshot that does not fit splits by entity groups into
   datagrams that each decode alone, deltas against the same baseline, acknowledged per group. The first full state,
   at join and after a resync, goes over a reliable stream; deltas resume once the client acknowledges it. The server
   forces a resync when a client's event queue overflows (below); a client that has lost its state (a Worker restart,
   a long hidden tab) asks for one with `Resync` (message catalogue). *Events:* tick-stamped gameplay events (the
   gameplay events of ADR 0004 part 19: hits, deaths and kills, purchases, uplink cache events, harvester deliveries
   and kills, structure kills and stage changes, tech tiers, upgrade picks and pickups) ride in the snapshot's event
   section, filtered per client by visibility class (part 9 (e); which economy events are team-private is
   R2-INT-OQ3), and are resent until the client's acknowledgement covers them; the per-client queue of unacknowledged
   events is bounded (data), and on overflow the server forces a full resync of that client or disconnects it.
   Cosmetic events (shots, muzzle flashes, sounds) go once, and never to the client whose own shots they are (part 4).
   Blasts are not cosmetic, since a lost one would hide an explosion: a blast that ends a projectile is that
   projectile's final trajectory change (its explosion, with tick and position), which the delta coding repeats until
   the client acknowledges it, so it needs no event of its own; any other blast (a vehicle's or a structure's) is a
   gameplay event, resent like one; beyond a projectile's relevance its blast reaches a client only as a cosmetic
   sound event inside part 9 (f)'s sound radius. When each event plays is part 7's rule. The shooter's hit
   confirmation is a gameplay event (LOOP_V2 §7). *Remote soldiers* carry position, yaw, pitch, health, flag bits
   for stance and action states (crouch, sprint, aiming down sights, reloading, carrying a cache) and the weapon id;
   action starts (fire, reload, throw, melee, an ability's cue) are tick-stamped cosmetic events. That is what the page
   animates remote soldiers from (what animation reads: ADR 0003; the block layout: ADR 0002), and it fits the
   estimate's 90-bit full and 60-bit delta soldier records, whose 8 flag bits change rarely. Vehicles carry hull and
   turret transforms and health per part; parts, and the pilot, travel with their parent. *Wire widths:* entities
   are named by the sim's own id (kind, index, generation; never a library id); a record names kind and index, and the
   generation travels in the record that brings the entity into a client's set; a generation change at an index the
   client holds (a respawn in the same slot, ADR 0004 part 18) is sent as a removal plus an addition, which also tells
   the client to place that soldier, not slide it (part 7). The snapshot header carries the full
END

### FIX EA-1, NA-1 (a), NA-4 (part 7)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
7. **Interpolation and the clock.** Remote entities are drawn at "server time minus the interpolation delay" from a
   snapshot buffer kept in the Worker: the Worker publishes already-delayed remote poses in each per-tick block, and
   the page interpolates between the two newest blocks (R1-TICK-REC5, ADR 0004; drawing, ADR 0003). The delay starts at
   3 snapshot intervals (100 ms at 30 Hz), widens with measured jitter and narrows slowly on a clean link, within 2 to
   5 intervals (66.7 to 166.7 ms at 30 Hz), as data. A late snapshot is extrapolated for at most one interval, then
   held. Respawns, teleports and first appearances are placed, not interpolated. Linear first; Hermite only if the
   proof test shows artefacts on fast movers at 30 Hz. Remote projectiles are evaluated on the same delayed timeline,
   so a frag lands where the interpolated target is drawn. The client announces its delay on the control stream; the
   server clamps it to the mode's range and lets it change by at most one interval per second. *Why:* three intervals
   is Fiedler's measured rule for 2–5 % loss, and with the buffer in the Worker the view time, the aim ray and the
   command are built in one place. *Research:* R2-SYNC-REC4 (changed: the buffer's side, one bound), R2-BUD-REC4
   (changed), R2-XU-OQ3, R1-XT-OQ3 = R2-XT-OQ2 = R3-XT-OQ1 = R5-XT-OQ2, R2-SYNC-OQ12 = R2-BUD-OQ7. *Need:* VISION_V2
   §4; §6.4 (big vehicles drawn smoothly).
AFTER:
7. **Interpolation and the clock.** Remote entities are drawn at the delayed time, "server time minus the
   interpolation delay", from a snapshot buffer kept in the Worker. Server time here is the client's own reading: the
   newest snapshot's server tick, advanced by the local clock since that snapshot arrived (its arrival stamp, part 5),
   so the delayed time trails the server's present by the downlink's one-way delay plus the interpolation delay. The
   Worker publishes already-delayed remote poses in each per-tick block, and the page interpolates between the two
   newest blocks (R1-TICK-REC5, ADR 0004; drawing, ADR 0003), so what the page draws trails the newest block by up to
   one tick; that fraction is inside the view time the page reports (part 5). The delay starts at 3 snapshot
   intervals (100 ms at 30 Hz), widens with jitter measured from the arrival stamps and narrows slowly on a clean
   link, within 2 to 5 intervals (66.7 to 166.7 ms at 30 Hz), as data. A late snapshot is extrapolated for at most one
   interval, then held. Respawns, teleports and first appearances are placed, not interpolated. Linear first; Hermite
   only if the proof test shows artefacts on fast movers at 30 Hz. Remote projectiles are evaluated on the same
   delayed timeline, so a frag lands where the interpolated target is drawn. *Events* follow the timeline of what they
   belong to: the Worker writes each spatial event (a remote soldier's action start or cosmetic shot, a hit on or the
   death of a remote entity, a blast, a structure kill) into the first block whose delayed time reaches the event's
   tick, so it plays with the drawn poses it belongs to ("Blasts are server events with a tick, played on the timeline
   of the entity they hit": r2-netcode.md, R2-SYNC-REC6, line 2210); the local player's own and predicted events,
   owner events (offers, pick and purchase results, the shooter's hit confirmation) and events tied to no entity on
   the delayed timeline (tech tiers, the uplink countdown) go into the next block at once, online and in a local match
   alike, since both run the client endpoint. The client announces its
   delay on the control stream; the server clamps it to the mode's range and lets it change by at most one interval
   per second. *Why:* three intervals is Fiedler's measured rule for 2–5 % loss, and with the buffer in the Worker the
   view time, the aim ray and the command are built in one place; an event played on arrival would run one
   interpolation delay ahead of its entity, so a remote death would start 0.9–1.5 m from the drawn soldier at 9 m/s
   (100–166.7 ms × 9 m/s, arithmetic). *Research:* R2-SYNC-REC4 (changed: the buffer's side, one bound),
   R2-BUD-REC4 (changed), R2-SYNC-REC6 (its blast rule), R2-XU-OQ3, R1-XT-OQ3 = R2-XT-OQ2 = R3-XT-OQ1 = R5-XT-OQ2,
   R2-SYNC-OQ12 = R2-BUD-OQ7. *Need:* VISION_V2 §4; §6.4 (big vehicles drawn smoothly); §6.8 (events stamped with
   ticks).
END

### FIX NA-1 (b, c), SA-5, SA-6 (part 8)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
8. **Lag-compensated hitscan.** The server rewinds to the shooter's view time as a pure query on a per-tick history of
   sim-owned hit volumes: soldiers (position, facing, stance, alive, Riot Shield up), vehicles and big units (position,
   hull and turret transforms, alive per part) and harvesters; no animation is stored. The ray leaves the server's own
   fire origin at the command's tick toward the command's aim point, and is tested against movers at the view time and
   the static world at present. The view time is accepted if it lies within a tolerance of the server's own estimate
   (the RTT plus that client's interpolation delay), the tolerance at most one snapshot interval (33.3 ms at 30 Hz,
   data); otherwise the estimate is used. The RTT is the server's own: quinn's `rtt()`, or over WebSocket the lowest
   recent sample of the server's own pings. The cap bounds the final rewind on every path: data, 200 or 250 ms
   (OQ-0001-5); beyond it, shooters lead by the excess. The history keeps the cap plus a margin (data), at most 1 MB
   for the stress scene. Filters: targets the shooter could hit (friendly fire from mode data), targets sent to and
   acknowledged by the shooter, targets within the ray's reach; a target that died or teleported stops at that record.
   Bots shoot at present state, with no rewind. The server logs each player's view-time offset and aim per command for
   cheat detection later (detection is the security baseline's, ADR 0006). On the round-trip reading of R2's profiles,
   a P2 player with a 100–150 ms delay needs a rewind of 200–250 ms (R2-SYNC-REC5's arithmetic), so a 200 ms cap clips
   some shots at the latency VISION_V2 §4 names; the proof test records the clipped share. *Why:* without it, a 100 ms
   player leads a sprinting soldier by about 1.8 m; with it, the shooter's view is the truth up to the cap.
   *Research:* R2-SYNC-REC5, R2-BUD-REC4 (changed: history and cap follow R2-SYNC-REC5), R2-XU-OQ1, R2-XU-OQ2,
   R2-XU-OQ7, R2-SYNC-OQ2, R2-SYNC-OQ8, R2-SYNC-OQ9, R2-SYNC-OQ15. *Need:* VISION_V2 §4; LOOP_V2 §7; §6.4 (hits on
   each part of a multi-part vehicle).
AFTER:
8. **Lag-compensated hitscan.** The server rewinds to the shooter's view time as a pure query on a per-tick history of
   sim-owned hit volumes: soldiers (position, facing, stance, alive, Riot Shield up), vehicles and big units (position,
   hull and turret transforms, alive per part) and harvesters; no animation is stored. The ray leaves the server's own
   fire origin at the command's tick toward the command's aim point, and is tested against movers at the view time and
   the static world at present. The view time is accepted if it lies within a tolerance of the server's own estimate,
   the tolerance at most one snapshot interval (33.3 ms at 30 Hz, data); otherwise the estimate is used. The estimate
   is R2's "RTT plus that client's interpolation delay" (r2-netcode.md, R2-SYNC-REC5, lines 2145–2146), which is
   Source's model, where a command runs on arrival, plus the two waits that model has not got: the time this command
   waited in the server's input buffer (part 5), which the server knows, and half a tick for the page's blend of two
   blocks (part 7: 0 to 1 tick, so half a tick centres the estimate; proposed). The RTT is the server's own: quinn's
   `rtt()`, or over WebSocket the lowest recent sample of the server's own pings. Neither holds against a cheat: a
   client can delay its replies (r2-netcode.md, lines 2150–2152) or, outside a browser, its QUIC acknowledgements, and
   so claim rewind up to the cap, an accepted risk. The cap bounds the final rewind on every path: data, its value
   OQ-0001-5; beyond it, shooters lead by the excess. The history keeps the cap plus a margin (data), at most 1 MB
   for the stress scene. Filters: targets the shooter could hit (friendly fire from mode data), targets sent to and
   acknowledged by the shooter, targets within the ray's reach; a target that died or teleported stops at that record.
   Bots shoot at present state, with no rewind. For cheat detection later (detection is the security baseline's, ADR
   0006), the server records each player's view-time offset and aim per command, and each client's quinn `rtt()`
   beside its own ping RTT once a second (proposed: a gap between the two is a detection signal), as raw entries in a
   per-match side file written beside the match record, keyed by slot and never by guest id, never in the `tracing`
   log stream: 48 × 30 = 1,440 entries a second, 1,728,000 in a 20-minute match (LOOP_V2 §8), about 518 MB as log
   lines at A-9's 300 B against its 200 lines a match (r6-infrastructure.md, lines 568–569) and about 27.6 MB at 16 B a
   raw entry (a slot, a tick, a view-time offset and a two-axis aim point: ADR 0006 part 9's scale figure; arithmetic).
   Raw entries, not per-player summaries (proposed: until the detection policy, item B7 of ADR 0006, says what it
   needs, nothing is summarised away); ADR 0006 part 9 uploads the file with the record and keeps it 14 days. On the
   round-trip reading of R2's profiles, a P2 player with a 100–150 ms delay needs a rewind of 200–250 ms by R2's
   formula (R2-SYNC-REC5's arithmetic), and 233–383 ms once the buffer (33.3–100 ms at 1–3 ticks) and the blend
   (0–33.3 ms) are added (arithmetic), so a 200 ms cap clips every honest shot at the latency VISION_V2 §4 names, and
   a 250 ms cap all but those with the shortest buffer and blend; the proof test records each shot's RTT, delay,
   buffer wait and blend, and the share each candidate cap clips (OQ-0001-5). *Why:* without it, a 100 ms player
   leads a sprinting soldier by about 1.8 m; with it, the shooter's view is the truth up to the cap.
   *Research:* R2-SYNC-REC5 (changed: the estimate adds the buffer wait and the blend), R2-BUD-REC4 (changed: history
   and cap follow R2-SYNC-REC5), R2-XU-OQ1, R2-XU-OQ2, R2-XU-OQ7, R2-SYNC-OQ2, R2-SYNC-OQ8, R2-SYNC-OQ9, R2-SYNC-OQ15.
   *Need:* VISION_V2 §4; LOOP_V2 §7; §6.4 (hits on each part of a multi-part vehicle).
END

### FIX NA-16 (part 9 (f))
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
   viewer's region; big units are relevant by their bounds plus a per-type visual radius; parts and seated players
AFTER:
   viewer's region; big units are relevant by their bounds plus a per-type visual radius; parts, and the pilot,
END

### FIX IE-7 (part 10, the ports)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
    HTTP/2. Over WebSocket the server sends only the newest snapshot and drops any unsent older one. Inputs, snapshots
    and pings travel as datagrams (binary messages on the fallback); one bidirectional control stream per session
    carries room control; the first full state goes on its own unidirectional stream. A thin transport interface on
AFTER:
    HTTP/2. Over WebSocket the server sends only the newest snapshot and drops any unsent older one. Inputs, snapshots
    and pings travel as datagrams (binary messages on the fallback); one bidirectional control stream per session
    carries room control; the first full state goes on its own unidirectional stream. *Ports:* the port plan is
    OQ-0006-16 (infra-engineer with netcode-architect, confirmed by the Advisor); until it is answered, each room
    process listens on one UDP port for WebTransport and one TCP port for the WebSocket fallback, taken from the VM's
    two firewall ranges (ADR 0006 part 5; that question's option (a)), and the lobby hands the client the host and both
    ports (one process per room gives one port per room: r2-netcode.md, Findings, R2-ROOM-F18, line 1562); whether the
    fallback must also answer on TCP 443 is the same question's. A thin transport interface on
END

### FIX SA-9, SA-7 (part 10, the join deadline and per-address limits)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
    dropped; per-connection rate limits; `Join` within 2 s (until then one stream, datagrams dropped); a modest cap on
    connections per IP address (above 1); new sessions rate-limited, with quinn's retry or refusal under load and
AFTER:
    dropped; per-connection rate limits; `Join` within 2 s, counted from the TCP accept or the QUIC handshake's first
    packet, so TLS and the WebSocket upgrade fall inside it (until then one stream, datagrams dropped), and a cap on
    handshakes in progress; a modest cap on connections per address (above 1), where an address is an IPv4 /32 or an
    IPv6 /64 (the prefix length is data: an IPv6 interface id is 64 bits, so one subnet gives a client 2^64
    addresses: RFC 4291 §2.5.1, read by the security review on 2026-11-02, not in the research), and every
    per-address limit of parts 10 and 11 keys the same way; new sessions rate-limited, with quinn's retry or
    refusal under load and
END

### FIX IE-4 as routed from ADR 0002 (part 10, the staging certificate)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
    carry-forward 4); development and preview servers use self-signed, pinned certificates for WebTransport, valid at
    most 14 days, whose hash the page fetches from its own HTTPS origin (R2-TRN-REC5); the WebSocket fallback cannot
AFTER:
    carry-forward 4); development servers and the netcode proof test use self-signed, pinned certificates for
    WebTransport, valid at most 14 days, whose hash the page fetches from its own HTTPS origin (R2-TRN-REC5); whether
    the staging server that multiplayer previews use presents such a pinned certificate or the browser-trusted one on
    both transports (a hash written into a static preview goes stale) is OQ-0006-5's, with ADR 0002 part 10's two
    shapes; the WebSocket fallback cannot
END

### FIX RR-2, RR-3, SA-2, SA-3, SA-12, EA-5 + IE-1, IE-11, SA Q2, GD-REJOIN (part 11)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
11. **Rooms, the lobby and bots.** A room lives through Created → Open → Countdown → Running → Ended → Closed, with a
    timeout in each state (activation, an empty room, the mode's match length) and a health check for the supervisor;
    a match never pauses or restarts for one player, and every match starts from a new sim instance, which also resets
    every player record, upgrade tracks included (UPGRADE-SCOPE; the record is ADR 0004's). One room per OS process to
    start: the sim on its own thread, network I/O on an async runtime, `panic = "unwind"`, each connection's work
    isolated from the tick; many rooms per process only if the proof test shows per-process memory or start-up cost
    too high. *Join:* a short-lived token from the lobby, checked by the room offline (claims: player id, guest flag,
    display name, room id as audience, team or slot if pre-assigned, expiry 30 s after issue with at most 5 s of leeway,
    a unique id), sent in the first message of the control stream and never in a URL. The signature follows the
    security review's answer to R2-ROOM-OQ2: an asymmetric signature (the lobby signs; rooms hold only the public key),
    one pinned algorithm, an explicit token type and a key id for rotation, the signature verified before the replay
    cache is touched, a bounded cache that refuses new tokens when full, and the room re-checking any pre-assigned team
    or slot against the mode; key storage waits on the security baseline (ADR 0006). The room, not the client, assigns
    team and squad. *Reconnect:* slot, team and squad are held for a grace window (data, proposed 60 s); the client
    fetches a fresh token; the newest connection for a player id wins and the older one is closed. *Private rooms:* a
    short code that only locates the room, from a secure random source, with lookups limited per client, per address
    and globally and guest creation limited per address; the creator is the host, and the room checks every host
    command (start, kick, lock, team and faction settings) against the host's player id; settings are fixed at start
    and carry a faction per team (VISION_V2 §3 [DECIDED]). *Bots:* the slot table marks each slot human, bot or empty;
    bots per team = team size − humans on that team; a joining human takes a bot's slot, keeping its sim id; bots run on
    the server (in the Worker in a local match) and produce the same input commands a client sends. *Modes:* a room is
    a mode id plus the overrides the mode allows; the mode file's format and validation are ADR 0005's. *Guests:* the
    lobby issues a guest id and a secret of at least 64 bits in an HttpOnly, Secure, SameSite=Strict cookie (where the
    secret finally lives is R2-ROOM-OQ11, ADR 0006's); the room only ever sees the join token. The room writes the
    match record before Closed (its shape is ADR 0006's, its events ADR 0004's). Where the lobby and supervisor run, the
    room directory and the guest store are ADR 0006's. *Why:* shipped hosting services model a session as a bounded
    life, one process each; offline token checks keep the lobby off the join path; bots through the human input path
    keep one sim path. *Research:* R2-ROOM-REC1 to R2-ROOM-REC8, R2-ROOM-OQ1, R2-ROOM-OQ2, R2-ROOM-OQ8, R2-ROOM-OQ10.
    *Need:* VISION_V2 §6.1; §3 ("Bots fill empty slots"); §6.6 (the record at Ended); §6.10 [LATER] prepared by the
    guest id, not built.
AFTER:
11. **Rooms, the lobby and bots.** A room lives through Created → Open → Countdown → Running → Ended → Closed, with a
    timeout in each state (activation, an empty room, the mode's match length) and a health check for the supervisor.
    `room` reads no clock and no OS randomness, since it also runs as the local server in the browser's Worker, where
    std's clock panics (the Rust review of 2026-11-02, reading std's source): its timers count ticks, their lengths in
    data (the state timeouts and the reconnect grace window, 60 s = 1,800 ticks at 30 Hz), while the `server` crate
    keeps the wall-time ones on its I/O side (the `Join` deadline and the handshake cap, per-connection and
    per-address rate limits, the ping schedule; part 10), and the host passes time in as a value. A match never
    pauses or restarts for one player, and every match starts from a new sim instance, which also resets every player
    record, upgrade tracks included (UPGRADE-SCOPE; the record is ADR 0004's). One room per OS process to start: the
    sim on its own thread, network I/O on an async runtime, `panic = "unwind"`, each connection's work isolated from
    the tick; many rooms per process only if the proof test shows per-process memory or start-up cost too high. The
    tick thread and the I/O runtime share only bounded channels, never a lock the tick unwraps (a connection that
    panics while holding a lock would poison it for the tick: std's `Mutex`, the Rust review). A panic on the tick
    thread is fatal to the room: the loop catches it at its top (`catch_unwind`), reports it (ADR 0006 part 10), sends
    `RoomEnded` with a crash reason where it can, marks the match record as crashed and exits the process non-zero, so
    the supervisor records the exit and the VM's other rooms go on; under `unwind` an uncaught panic "ends only its
    own thread" (r2-netcode.md, Findings, R2-ROOM-F8, lines 1459–1462) and would leave a room that holds its
    connections and sends nothing (ADR 0004 part 20 says the same; whether a match should ever resume instead is
    OQ-0001-18). *Join:* a
    short-lived token from the lobby, checked by the room offline (claims: player id, guest flag, display name, room id
    as audience, team or slot if pre-assigned, expiry 30 s after issue with at most 5 s of leeway, a unique id), sent
    in the first message of the control stream and never in a URL. The lobby validates display names, and the room
    re-checks the length before the roster: bounds as data (a length in bytes and in code points), no Unicode control
    or format characters (categories Cc and Cf), NFC (proposed; the security review's). The signature follows the
    security review's answer to R2-ROOM-OQ2: an asymmetric signature (the lobby signs; rooms hold only the public key),
    one pinned algorithm, an explicit token type and a key id for rotation, and a separate signing key per environment,
    each with its own key id, so staging rooms take only staging-key tokens and a production room refuses every staging
    or preview token whatever its Origin (ADR 0006 part 4 and item B5), the signature
    verified before the replay cache is touched, a bounded cache that refuses new tokens when full, and the room
    re-checking any pre-assigned team or slot against the mode; key storage waits on the security baseline (ADR 0006).
    The room, not the client, assigns team and squad. *Reconnect:* slot, team and squad are held for a grace window
    (data, proposed 60 s); the client fetches a fresh token; the newest connection for a player id wins and the older
    one is closed; whether a player who returns after the window gets their record (tracks, money) back is ADR 0004's
    OQ-0004-10 (GD-REJOIN: Mustafa's, with OQ-0005-AB10; the record is ADR 0004's). *Private rooms:* a short code
    that only locates the room, from a secure random source, with lookups limited per client, per address and
    globally and guest creation limited per address (addresses keyed as in part 10); the creator is the host, and the
    room checks every host
    command (start, kick, lock, team and faction settings) against the host's player id (whether a kick also bars that
    player for the rest of the match is OQ-0001-17); settings are fixed at start and carry a faction per team
    (VISION_V2 §3 [DECIDED]). *Bots:* the slot table marks each slot human, bot or empty; bots per team = team size −
    humans on that team; a joining human takes a bot's slot, keeping its sim id; bots run on the server (in the Worker
    in a local match) and produce the same input commands a client sends. *Modes:* a room is a mode id plus the
    overrides the mode allows; the mode file's format and validation are ADR 0005's. *Guests:* the lobby issues a
    guest id and a secret of at least 64 bits in an HttpOnly, Secure, SameSite=Strict cookie (where the secret finally
    lives is R2-ROOM-OQ11, ADR 0006's); SameSite=Strict sends it only with requests from the site that set it, a site
    being the registrable domain (MDN, Set-Cookie and Glossary "Site", read by the security review on 2026-11-02), so
    the client host and the lobby sit on one registrable domain (ADR 0006 part 6 and item B4; the domain is
    OQ-0006-15); the room
    only ever sees the join token. The room writes the match record before Closed (its shape is ADR 0006's, its events
    ADR 0004's). Where the lobby and supervisor run, the room directory and the guest store are ADR 0006's; the
    directory records each room's build, protocol version and content hash, so the lobby can send a client only to
    rooms that match it (needed if releases run live, harmless in a release window: how releases reach running rooms
    is OQ-0006-14); the lobby asks a VM's supervisor for a new room and learns its host and ports (message catalogue;
    the link is ADR 0006 part 6's). *Why:* shipped hosting services model a session as a bounded
    life, one process each; offline token checks keep the lobby off the join path; bots through the human input path
    keep one sim path. *Research:* R2-ROOM-REC1 to R2-ROOM-REC8, R2-ROOM-OQ1, R2-ROOM-OQ2, R2-ROOM-OQ8, R2-ROOM-OQ10.
    *Need:* VISION_V2 §6.1; §3 ("Bots fill empty slots"); §6.6 (the record at Ended); §6.10 [LATER] prepared by the
    guest id, not built.
END

### FIX GD-2, GD-8 (its 0001 items), GD-19, GD-3 (part 12)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
12. **In-match upgrades, weapon drops and pickups (UPGRADE-SCOPE, the network side).** The server is the authority over
    every player's upgrade tracks, the offers of three, the picks, the weapon drops and the world pickups. Offers reach
    only their owner (owner class), in the owner's section of the snapshot, each with an offer id. A pick is
    `PickUpgrade{offerId, choice}` on the control stream; the authority applies it at its next tick and reports the
    result to the owner as an event; it refuses a pick whose offer id is not an open offer to that player, a choice
    outside the three, or a second pick of one offer, and counts the refusal like a forged command. Track state (levels
    per unit type and per weapon type) is owner class and is sent to the owner when it changes. A dropped weapon is a
    sim entity (ADR 0004) replicated like any spatial entity: relevant through the region and the enclosure test, and
    sent from the tick it becomes relevant to a client. A pickup is asked for through the command's existing button
    edges, with no message of its own; the authority picks the pickup from its own state, so a command can never name
    a pickup into existence, and it refuses one it did not place, one already taken, or one outside the pickup rule
    (rule data: ADR 0005). Neither offers, picks nor pickups are predicted (pickups interim: OQ-0001-10). The room writes
    pick and pickup telemetry into the match record (fields: ADR 0006). No rule is decided here: carry slots, ammo for
    looted weapons, how fast tracks level, the drop rules and bots' records are Mustafa's (ADR 0005 lists them; the
    last two are OQ-0005-AB9 and OQ-0005-AB10). *Why:* the tracks belong to the
    player, so they are owner data the server holds, and offers that are never predicted need no client copy of the
    random stream that draws them (its kind is ADR 0004's). *Research:* no research id concerns upgrades; the parts used
    are R2-INT-REC5's classes, R2-SYNC-REC2's validation and R2-SYNC-F17 (Quake III's predicted pickups, the
    alternative OQ-0001-10 weighs). *Need:* UPGRADE-SCOPE.
AFTER:
12. **In-match upgrades, purchases, weapon drops and pickups (UPGRADE-SCOPE, the network side).** The server is the
    authority over every player's upgrade tracks, the offers, the picks, the purchases, the weapon drops and the world
    pickups. Offers (three choices by default; the offer's size is data, ADR 0005 AB-8) reach only their owner (owner
    class), in the owner's section of the snapshot, each with an offer id. A pick is `PickUpgrade{offerId, choice}`
    on the control stream; the authority applies it at its next tick and reports the result to the owner as an event;
    it refuses a pick whose offer id is not an open offer to that player, a choice outside the offer, or a second pick
    of one offer. It counts a malformed pick (an offer id it never issued, a choice out of range) like a forged
    command, and a stale or repeated one (a double click, or a pick in flight when the offer closed) apart, so honest
    races are not counted with cheats. Track state (levels and picks per unit type and per weapon type) is owner class
    and is sent to the owner when it changes; owner is the starting class, and who else sees picks is GD-PICK-VIS
    (Mustafa's); what a pick changes inside the prediction scope reaches the owner in the own-state section (part 4).
    A purchase is `Buy{unitId}` on the control stream, for a unit or a vehicle ("the purchase names a unit, never a
    weapon": ADR 0005 AB-8's table); the authority applies it at its next tick only if the id exists in the
    mode's content, the team has reached its tier (ADR 0005 AB-5), the player can pay the price (personal money,
    LOOP_V2 §3, power-plant pricing included) and stands at the team's own buy terminal (LOOP_V2 §2, §5), these
    conditions being rule data (ADR 0005); it reports the result to the owner as an event, counts refusals as it
    counts picks, and never predicts a purchase. A dropped weapon is a sim entity (ADR 0004) replicated like any
    spatial entity: relevant through the region and the enclosure test, and sent from the tick it becomes relevant to
    a client. A pickup is asked for through the command's existing button edges, with no message of its own; which act
    a press means where a pickup lies within reach of an uplink or a cache (E also steals and places, LOOP_V2 §4) is
    rule data (ADR 0005, OQ-0005-AB9's list). The authority picks the pickup from its own state, so a command can never
    name a pickup into existence, and it refuses one it did not place, one already taken, or one outside the pickup
    rule (rule data: ADR 0005). Neither offers, picks, purchases nor pickups are predicted (pickups interim:
    OQ-0001-10). The room writes pick and pickup telemetry into the match record (fields: ADR 0006). No rule is
    decided here: carry slots, ammo for looted weapons, how fast tracks level, the drop rules and bots' records are
    Mustafa's (ADR 0005 lists them; the last two are OQ-0005-AB9 and OQ-0005-AB10). *Why:* the tracks belong to the
    player, so they are owner data the server holds, and offers that are never predicted need no client copy of the
    random stream that draws them (its kind is ADR 0004's); a purchase changes money, the authority's alone. *Research:*
    no research id concerns upgrades; the parts used are R2-INT-REC5's classes, R2-SYNC-REC2's validation and
    R2-SYNC-F17 (Quake III's predicted pickups, the alternative OQ-0001-10 weighs); R2's estimate already counts buys
    among the reliable gameplay events (r2-netcode.md, worked bandwidth estimate, line 1760). *Need:* UPGRADE-SCOPE;
    purchases: VISION_V2 §6.5 (the economy server-side).
END

### FIX NA Q2 (part 13)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
    Brief question 1, lines 1987–1992). Every R2 mark is restated against one: the budgets unit's "100 ms RTT, ±10 ms
AFTER:
    Brief question 1, lines 1987–1992). Each direction carries half the profile's latency and a uniform jitter of
    ± half its jitter, so the round trip keeps the profile's mean and range (proposed; OQ-0001-16). Every R2 mark is
    restated against one: the budgets unit's "100 ms RTT, ±10 ms
END

### FIX NA-7 and EA-8 as routed from ADR 0002 (the components diagram)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
 input, camera, draw  --> per-tick command + cursor ray      |  network I/O runtime (async, other threads):
 HUD, audio (ADR 0003)    client endpoint:                   |   WebTransport on quinn | WebSocket (tokio-
AFTER:
 input, camera, draw  --> newest input + ray, each frame     |  network I/O runtime (async, other threads):
 HUD, audio (ADR 0003)    client endpoint (client-endpoint): |   WebTransport on quinn | WebSocket (tokio-
END

### FIX EA-1, NA-7 as routed (the client tick)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  match); advance the interpolation clock; build this tick's command from the page's posted input, casting the aim ray
  against the state published for the page's view time; send the input datagram; write the block: the predicted local
  player, remote entities at the delayed time, the tick's events.
AFTER:
  match); advance the interpolation clock; build this tick's command from the page's newest post, casting the aim ray
  against the state published for the page's view time; send the input datagram; write the block: the predicted local
  player, remote entities at the delayed time, and the events due at this block (part 7).
END

### FIX NA-4 as routed from ADR 0002 (catalogue: Ping and Pong)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| `Ping` / `Pong` | both ways | datagram | n/a | a few per second (proposed) | sender's time; on the answer, the receiver's tick | rate limit |
AFTER:
| `Ping` / `Pong` | both ways | datagram | n/a | a few per second (proposed) | sender's time; on the answer, the receiver's tick and the hold time from the ping's arrival to the answer (part 5) | rate limit; a reported hold time counted up to one tick period |
END

### FIX GD-20 (GD-CARRIER pointer), coordinator item 3 (OQ-0004-9)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| Uplinks, countdown, steals, cache location | room sim · local server | never | everyone | public |
AFTER:
| Uplinks, countdown, steals, cache location | room sim · local server | never | everyone | public (whether a carried cache's position stays public is ADR 0004's OQ-0004-9, GD-CARRIER: Mustafa's) |
END

### FIX NA-9, PA-3, GD-3, NA-3, EA-1 (catalogue: Input, Snapshot)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| `Input` | client → room | datagram | n/a | every tick (30 Hz) | newest snapshot decoded (ack, per entity group); every unacknowledged command up to the cap: tick, sequence, move, buttons, press counters, aim point, view time | part 5's list; size ≤ command size × cap; rate limit |
| `Snapshot` (one or more datagrams per tick) | room → client | datagram | per field | every tick (30 Hz) | header (32-bit server tick, baseline tick, last processed command, group index); own-state section (owner); spatial records; team and public sections; event section (gameplay events until acknowledged, cosmetic once) | n/a (the client drops what it cannot decode) |
AFTER:
| `Input` | client → room | datagram | n/a | every tick (30 Hz) | newest snapshot decoded (ack, per entity group); every unacknowledged command up to the cap, the repeats delta-coded (part 5): tick, sequence, move, buttons, press counters, aim point, view time | part 5's list; size ≤ command size × cap; rate limit |
| `Snapshot` (one or more datagrams per tick) | room → client | datagram | per field | every tick (30 Hz) | header (32-bit server tick, baseline tick, newest command received, last processed command, group index); own-state section (owner: part 4's list); spatial records; team and public sections; event section (gameplay events until acknowledged, cosmetic once and never the client's own shots; written into the block at part 7's time) | n/a (the client drops what it cannot decode) |
END

### FIX NA-13 (catalogue: Resync)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| `FullState` | room → client | unidirectional stream | per field | at join and on resync | the client's whole relevant world at one tick | n/a |
AFTER:
| `FullState` | room → client | unidirectional stream | per field | at join and on resync | the client's whole relevant world at one tick | n/a |
| `Resync` | client → room | control stream | n/a | when the client has lost its state (a Worker restart, a long hidden tab) | none | rate limit (proposed: one in flight, since a full state is the largest message a client can make the server send); answered with `FullState` |
END

### FIX GD-2, GD-8 (catalogue: PickUpgrade, Buy)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| `PickUpgrade` | client → room | control stream | n/a | when an offer is open | offer id, choice (one of three) | open offer to this player, choice in range, not already picked |
AFTER:
| `PickUpgrade` | client → room | control stream | n/a | when an offer is open | offer id, choice (an index into the offer; the offer's size is data) | open offer to this player, choice in range, not already picked; malformed picks counted like forged commands, stale or repeated ones apart (part 12) |
| `Buy` | client → room | control stream | n/a | when the player buys | unit or vehicle id | the id exists in the mode's content; the team has its tier; the player can pay the price; the player stands at the team's own buy terminal (part 12; the conditions are rule data, ADR 0005); refusals counted as for picks |
END

### FIX EA-10, SA-7, IE-11 (catalogue: lobby calls, supervisor calls, lobby to supervisor)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| Lobby calls (HTTPS) | browser → lobby | HTTPS | n/a | per visit or room | guest create (sets the cookie), private-room create, code lookup, token request | Origin check; rate limits per client, per address and global |
| Supervisor calls | supervisor ↔ room | local to the VM (ADR 0006) | n/a | per room | spawn with room id, mode id, overrides and the lobby's public key; health check; exit; record upload | ADR 0006 |
AFTER:
| Lobby calls (HTTPS) | browser → lobby | HTTPS | n/a | per visit or room | guest create (sets the cookie), private-room create, code lookup, token request; crash reports and client field data (one report endpoint on the lobby: ADR 0006 part 10) | Origin check; rate limits per client, per address (part 10's IPv4 /32 and IPv6 /64) and global |
| Supervisor calls | supervisor ↔ room | local to the VM (ADR 0006) | n/a | per room | spawn with room id, mode id, overrides and the lobby's public key; health check; exit; record upload | ADR 0006 |
| Lobby ↔ supervisor | both ways | between the lobby and each game VM (ADR 0006 part 6) | n/a | per room, and when a VM's state changes | the supervisor registers its VM, its free room slots and the build and protocol version of the room binaries it holds; the lobby asks it to start a room (room id, mode id, overrides); it answers with the host and the room's ports (part 10) or a refusal; it reports Closed, an abnormal exit and draining | ADR 0006 part 6 (the link authenticated, item B5; not researched) |
END

### FIX GD-2 (catalogue note)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
Upgrade offers, pick results, track changes, hit confirmations, kills, purchases and drops are not separate message
types: they are gameplay events or owner-section fields inside `Snapshot`, each with its class.
AFTER:
Upgrade offers, pick and purchase results, track changes, hit confirmations, kills and drops are not separate message
types: they are gameplay events or owner-section fields inside `Snapshot`, each with its class.
END

### FIX PA-1 (its 0001 half: the downstream bullet)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  proof test measures them, R2-XU-OQ15). IPv6 adds 20 B per datagram: +4.8 kbps at 30 Hz.
AFTER:
  proof test measures them, R2-XU-OQ15). IPv6 adds 20 B per datagram: +4.8 kbps at 30 Hz. The estimate is R2's own
  scene, not the stress scene (GLOSSARY.md:415–416): the proof test's load fires 700 shots a second (R2-SYNC-REC5
  mark 4), which alone adds (700 − 72) × 4 B × 8 ≈ 20.1 kbps (R2's shot events: 1.5 per player per second at 4 B,
  r2-netcode.md line 1759, so 72 a second at N = 48; arithmetic), so the 160 kbps mean is judged in R2's estimate
  scene and the stress scene is held by the cap (the budgets table).
END

### FIX PA-3, NA-9 (the upstream bullet and the room's upstream)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **Upstream per client:** with the estimate's 8 B commands, newest plus 3 repeats, sending at 30 Hz costs about 22.3 kbps
  (option U1's 44.6 kbps at 60 sends per second, halved; r2-netcode.md, Options, Brief question 2, table C, lines
  1704–1710). The mark is at most 48 kbps (IPv4: R2-BUD-REC1), which at 30 sends per second leaves 200 B per datagram
  (48,000 ÷ 8 ÷ 30), 143 B after the 57 B of IPv4 transport overhead (r2-netcode.md, line 1742), for the header and
  the repeated commands with their aim points (at most 80 bits each) and view times. The real command size is
  measured (R2-XU-OQ4).
- **One room of 48 humans:** about 48 × 146 kbps ≈ 7.0 Mbit/s out and about 3.2 GB per match-hour (r2-netcode.md,
  line 1786); about 48 × 22.3 kbps ≈ 1.07 Mbit/s in (arithmetic). Bots cost no network traffic. R6 prices 3.16 GB per
AFTER:
- **Upstream per client:** R2's estimate assumes 8 B commands, newest plus 3 repeats: about 22.3 kbps at 30 sends per
  second (option U1's 44.6 kbps at 60 sends per second, halved; r2-netcode.md, Options, Brief question 2, table C,
  lines 1704–1710). Part 5's command is larger: its aim point alone is up to 80 bits, beside the tick, sequence, move
  vector, buttons, press counters and view time, about 15–20 B in all (the perf review's estimate, 2026-11-02; the
  real size is measured, R2-XU-OQ4). With the newest-received acknowledgement (part 5), a datagram carries the
  commands of one round trip plus up to two ticks of waiting (the snapshot's and the client's): 80–186.7 ms at P2's
  100 ± 20 ms, so 3 to 6 commands ((80 to 186.7) ÷ 33.3, rounded up; arithmetic). Sent whole (U1), with a header of
  about 6 B (an assumption) and the 57 B of IPv4 transport overhead (r2-netcode.md, line 1742), at 30 datagrams a
  second (240 bits a second per byte of datagram), that is (3 × 15 + 63) × 240 ≈ 25.9 to (6 × 20 + 63) × 240 ≈
  43.9 kbps at P2, and at the cap of 8, reached at P3 and P4 or after a loss, (8 × 15 + 63) × 240 ≈ 43.9 to
  (8 × 20 + 63) × 240 ≈ 53.5 kbps, against a mark of at most 48 kbps (IPv4: R2-BUD-REC1), 200 B per datagram
  (48,000 ÷ 8 ÷ 30). So the repeats are delta-coded (U2, part 5), and the proof test records the real datagram at P2
  and at the cap; QUIC's acknowledgements riding on input packets add about 1.9 kbps (r2-netcode.md, line 1785:
  about 3.8 kbps at 60 Hz, an assumption there, halved).
- **One room of 48 humans:** about 48 × 146 kbps ≈ 7.0 Mbit/s out and about 3.2 GB per match-hour (r2-netcode.md,
  line 1786); about 48 × 25.9 to 48 × 43.9 kbps ≈ 1.2 to 2.1 Mbit/s in at P2 before delta coding (arithmetic). Bots
  cost no network traffic. R6 prices 3.16 GB per
END

### FIX PA-1 (its 0001 half: the budgets row)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| Downstream per client | mean ≤ 160 kbps and p99 of 1-second windows ≤ 256 kbps, N = 48, everyone in view, 30 Hz, counted at UDP payload + 28 B per datagram; hard cap 256 kbps (proposed); 0 datagrams above the cap rule | R2-BUD-REC2 (r2-netcode.md:2326–2344); R2-BUD-REC3 |
AFTER:
| Downstream per client | in R2's estimate scene, where R2-BUD-REC2 set the mark (48 soldiers and 15 big vehicles all in view, a 10-minute bot match): mean ≤ 160 kbps and p99 of 1-second windows ≤ 256 kbps, 30 Hz, counted at UDP payload + 28 B per datagram; in the stress scene: every 1-second window ≤ 256 kbps (the hard cap, proposed) and 0 datagrams above the cap rule, its mean reported (its shots alone add about 20.1 kbps to R2's 146.1: the bandwidth estimate) | R2-BUD-REC2 (r2-netcode.md:2326–2344; its scene, lines 2339–2340); R2-BUD-REC3; GLOSSARY.md:415–416 |
END

### FIX RR-3, NA-13, EA-5 + IE-1, TE-4 (failure modes)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| A panic in one connection's handler | that connection closes; the room's tick and other connections go on | R2-ROOM-REC2 mark 6 |
| A room process dies | that match ends; other rooms on the VM are unaffected | R2-ROOM-REC2 mark 4 |
| A trap in the client's Wasm sim | the Worker restarts (ADR 0004) and resyncs through `FullState` | the join mark (What a spike must prove, item 2) |
| A hidden tab or a sleeping laptop | online, the client does not simulate the gap; it resyncs | back in step within 0.5 s (R1-TICK-REC5, ADR 0004) |
| A player drops | slot held for the grace window; fresh token; newest connection wins | 100 % rejoin the same slot (R2-ROOM-REC4) |
| The lobby is down | no new joins; running rooms go on (tokens are checked offline) | none |
| Version or content mismatch | `JoinRejected` with the reason; the client reloads the bundle | R1-DATA-REC6's mark (ADR 0005) |
| A tick counter wraps | wire ticks are 32-bit in the header, 16-bit offsets elsewhere | the 60 Hz margin run's 30-minute soak crosses 65,536 ticks |
AFTER:
| A panic in one connection's handler | that connection closes; the room's tick and other connections go on (connections share only bounded channels with the tick, never a lock it unwraps: part 11) | R2-ROOM-REC2 mark 6 |
| A panic on the room's tick thread | the match ends: `RoomEnded` with a crash reason where possible, the match record marked crashed, the process exits non-zero and the supervisor records the exit (part 11; OQ-0001-18) | the process gone within 1 s and every client told or closed (proposed: a mark beside PM-0001-26, test-engineer's) |
| A room process dies | that match ends; other rooms on the VM are unaffected | R2-ROOM-REC2 mark 4 |
| A trap in the client's Wasm sim | the Worker restarts (ADR 0004) and asks for a `FullState` with `Resync` | the join mark (What a spike must prove, item 2) |
| A hidden tab or a sleeping laptop | online, the client does not simulate the gap; it resyncs (`Resync`) | back in step within 0.5 s (R1-TICK-REC5, ADR 0004) |
| A player drops | slot held for the grace window; fresh token; newest connection wins | 100 % rejoin the same slot (R2-ROOM-REC4) |
| The lobby is down | no new joins; running rooms go on (tokens are checked offline) | none |
| Version or content mismatch | the lobby can send a client only to rooms of its own build and content hash (part 11); a `JoinRejected` for a mismatch reloads the client once, and only if the lobby names a newer build than its own; otherwise the client shows the reason and returns to the lobby, so a refused client never loops (proposed) | R1-DATA-REC6's mark (ADR 0005); the release path is OQ-0006-14 |
| A tick or sequence counter wraps | wire ticks are 32-bit in the header, 16-bit offsets elsewhere; sequence numbers wrap, handled (part 5) | a deterministic L1 gate test that starts every counter just below its wrap (Test plan); the soaks are a second check |
END

### FIX SA-8, SA-6 (anti-cheat: commands, lag compensation)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- Command checks and the lag-switch cap (part 5); forged commands are rejected and counted, with no effect beyond
  their valid part (R2-SYNC-REC2 mark 3).
- Lag compensation: the view-time tolerance of one snapshot interval, the final cap and the server's own RTT stop a
  client from buying extra rewind; view-time offsets and aims are logged for later detection (part 8).
AFTER:
- Command checks, strictly newer command ticks (the fast-clock guard) and the lag-switch cap (part 5); forged
  commands are rejected and counted, with no effect beyond their valid part (R2-SYNC-REC2 mark 3).
- Lag compensation: the view-time tolerance of one snapshot interval and the final cap bound the rewind any client can
  claim; the server's own RTT does not, since a client can delay its pongs or, outside a browser, its QUIC
  acknowledgements, so a client faking latency gets rewind up to the cap (an accepted risk; OQ-0001-5); view-time
  offsets and aims are logged per command, and quinn's RTT beside the ping RTT per client, for later detection
  (part 8).
END

### FIX GD-2 (anti-cheat: purchases)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- Upgrades and pickups: a pick the server did not offer, or a pickup it did not place, is refused (part 12).
AFTER:
- Upgrades, purchases and pickups: a pick the server did not offer, a purchase the purchase rule does not allow, or a
  pickup it did not place, is refused (part 12).
END

### FIX SA-2, SA-7 (anti-cheat: admission)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- Admission: asymmetric join tokens checked before the replay cache, single use; room codes only locate; host
  commands checked against the host's id; the guest secret out of script's reach (part 11).
AFTER:
- Admission: asymmetric join tokens checked before the replay cache, single use, signed with a key per environment;
  room codes only locate; host commands checked against the host's id; the guest secret out of script's reach;
  per-address limits keyed by IPv4 /32 and IPv6 /64 (parts 10 and 11).
END

### FIX SA-6 (Q5, option B)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| **B. Server rewind to the view time, capped (chosen)** | medium | ≤ 20 µs per shot; ≤ 1 MB of history | grows with shots per second | the tolerance, cap and own RTT bound the abuse | the shooter's view is the truth up to the cap | `resolveShot` is a pure query built for this (CS7) | Source's shipped model | the victim's side unmeasured (R2-SYNC-OQ15) |
AFTER:
| **B. Server rewind to the view time, capped (chosen)** | medium | ≤ 20 µs per shot; ≤ 1 MB of history | grows with shots per second | the tolerance and the cap bound the abuse (a client can inflate the RTT up to the cap: part 8) | the shooter's view is the truth up to the cap | `resolveShot` is a pure query built for this (CS7) | Source's shipped model | the victim's side unmeasured (R2-SYNC-OQ15) |
END

### FIX PA-2 (Q6, E3b)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| E3b: projectiles in every snapshot | low | about 188 kbps | +42 kbps | same | always matches | none | no client flight needed | more bytes: the fallback if the proof test counts a desync |
AFTER:
| E3b: projectiles in every snapshot | low | about 188 kbps | +42 kbps | same | always matches | none | no client flight needed | more bytes: whole, it breaks the mean and, in the stress scene, the cap (part 6); kept per projectile kind only |
END

### FIX EA-2 (Q8, T2)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| **T2. 30 Hz tick, snapshots and input (chosen, TICK-30's baseline)** | low | lowest step cost | 146 kbps down; 9.18 of 16.67 ms | as T1 | +8 ms average input wait; fire intervals quantise harder (remainders carried) | none | the baseline TICK-30 sets | coarser timing (OQ-0001-7) |
AFTER:
| **T2. 30 Hz tick, snapshots and input (chosen, TICK-30's baseline)** | low | lowest step cost | 146 kbps down; 9.18 of 16.67 ms | as T1 | about +25 ms from input to screen on average (+8.3 ms of input wait, +16.7 ms of display: trade-off 2; OQ-0001-15); fire intervals quantise harder (remainders carried) | none | the baseline TICK-30 sets | coarser timing (OQ-0001-7) |
END

### FIX PA-2 (Q9, E3b)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| E3b. E3 with projectiles every snapshot | medium | about 188 kbps | +42 kbps | none | none | none | no client flight | the fallback |
AFTER:
| E3b. E3 with projectiles every snapshot | medium | about 188 kbps | +42 kbps | none | none | none | no client flight | per projectile kind only (part 6) |
END

### FIX PA-3 (Q10, U1 and U2)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| **U1. Every tick, newest + repeats (chosen, at 30 Hz, repeating every unacknowledged command up to the cap)** | low | about 22.3 kbps at 30 Hz with 8 B commands | ≤ 48 kbps | the catch-up cap no larger than the redundancy cap | no extra input delay | CS1's command record | an input is lost only after several lost packets in a row | more bytes than U2 |
| U2. U1 with delta-coded repeats | low | about 36 kbps at 60 Hz | fewer bytes | as U1 | as U1 | none | cheaper | adopt once U1 is measured |
AFTER:
| U1. Every tick, newest + repeats sent whole (at 30 Hz, every unacknowledged command up to the cap) | low | about 22.3 kbps at 30 Hz with 8 B commands; with part 5's command 25.9–43.9 kbps at P2 and 43.9–53.5 kbps at the cap (bandwidth estimate) | ≤ 48 kbps only below the cap | the catch-up cap no larger than the redundancy cap | no extra input delay | CS1's command record | an input is lost only after several lost packets in a row | over the 48 kbps mark at the cap with part 5's command |
| **U2. U1 with delta-coded repeats (chosen)** | low | about 36 kbps at 60 Hz with 8 B commands; with part 5's command, measured (R2-XU-OQ4) | fewer bytes | as U1 | as U1 | none | keeps the larger command under the mark at the cap | a little more code; R2-BUD-REC1 takes it up once U1 is measured, adopted here at once (part 5) |
END

### FIX NA-1 (c), EA-2, PA-2, the optional fixcheck-a note (trade-off items 1, 2, 4, 5)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  1. *Fairness against the rewind cap.* At P2 (100 ms round trip) a 30 Hz client with a 100–150 ms delay needs a
     rewind of 200–250 ms, so the security review's 200 ms cap clips shots at the very latency VISION_V2 §4 names, and
     250 ms lengthens the victim's "hit behind cover" window, which no source measures. This is a feel trade-off, not
     an engineering one (OQ-0001-5).
  2. *Coarser input at 30 Hz.* About +8 ms of average input wait against 60 Hz, and the moment of a click inside a
     33 ms tick is not carried (OQ-0001-7).
  3. *Pre-1.0 transport crates.* Neither WebTransport crate claims production readiness; the thin interface, the
     version floors, the per-browser interop marks and the WebSocket fallback are the hedge.
  4. *Client flight for E3.* Correctness never rests on identical trajectories (landings are server events), but what a
     remote client draws between change events does; E3b is the measured fallback at +42 kbps.
  5. *Weight in the Worker.* The interpolation buffer and the aim ray live in the Worker, and in a local match the
     Worker also encodes and decodes every message for the loopback path; that cost is not measured yet. It is
     reported beside the client's sim share (ADR 0004's), not inside it, since that share is sim step time only
     (GLOSSARY.md:214–218).
AFTER:
  1. *Fairness against the rewind cap.* At P2 (100 ms round trip) a 30 Hz client with a 100–150 ms delay needs a
     rewind of 200–250 ms by R2's formula, and 233–383 ms once the server's input buffer (1–3 ticks) and the page's
     blend (0–1 tick) are counted (part 8), so the security review's 200 ms cap clips every honest shot at the very
     latency VISION_V2 §4 names and 250 ms nearly every one, while a longer cap lengthens the victim's "hit behind
     cover" window, which no source measures. Running commands on arrival (OQ-0001-6 (a)) would remove the buffer's
     term. This is a feel trade-off, not an engineering one (OQ-0001-5).
  2. *Coarser input at 30 Hz.* About +25 ms from input to screen against 60 Hz on average: +8.3 ms of input wait
     (half of each tick period: 16.7 − 8.3) and +16.7 ms of display, since the page draws the local player between the
     two newest blocks, one tick behind its newest prediction (33.3 against 16.7 ms; R1-TICK-REC5,
     r1-simulation-core.md line 2099; arithmetic). Whether the local player is drawn from its newest predicted tick
     instead is OQ-0001-15; the moment of a click inside a 33 ms tick is not carried (OQ-0001-7).
  3. *Pre-1.0 transport crates.* Neither WebTransport crate claims production readiness; the thin interface, the
     version floors, the per-browser interop marks and the WebSocket fallback are the hedge.
  4. *Client flight for E3.* Correctness never rests on identical trajectories (landings are server events), but what a
     remote client draws between change events does; a desync is a defect of the shared trajectory function
     (PM-0004-73), and E3b remains only per projectile kind, since E3b for every projectile breaks the budgets (part 6).
  5. *Weight in the Worker.* The interpolation buffer and the aim ray live in the Worker, and in a local match the
     Worker also encodes and decodes every message for the loopback path; that cost is not measured yet. It is
     reported beside the client's sim share (ADR 0004's), not inside it, since that share is sim step time only
     (GLOSSARY.md:214–218); ADR 0002's PM-0002-25 reports the Worker's time per tick in a local match.
END

### FIX PA-2 (Revisit when)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  share, E3 or E3b, the process model, the datagram cap); a browser moves to a new WebTransport draft; connection
AFTER:
  share, any per-kind E3b, the process model, the datagram cap); a browser moves to a new WebTransport draft; connection
END

### FIX TE-1 (its 0001 part: OQ-0001-2), coordinator item 4 (ADR 0004 S1's stand-ins)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **OQ-0001-2. How does the stress scene's sim load reach the 60 Hz margin run (TICK-60-SETUP)?** · *Why it matters:*
  the proof test runs before the foundation with a throwaway sim, and the margin must be measured in one run, since
  p99s do not add. · *Options* (test set-ups, not research options): (a) the sim core spike's sim (ADR 0004's spike)
  runs inside the proof-test server for that run; (b) a stand-in burns, tick by tick, the step times the sim core spike
  recorded; (c) the 60 Hz check waits until the real sim runs the stress scene (slice 6), and the tick stays at 30 Hz
  until then. **Recommended: (a)**, with (c) as the fallback if the sim core spike's sim cannot run the stress scene in
  time. (b) is weaker because a replayed series loses the link between the sim's load and the server-only load in the
  same fight, and the margin is a tail statistic (our inference). The 30 Hz baseline marks may use a stand-in: on the
  planning check they leave 7.49 ms. · *Decider:* the Advisor. · *What decides it:* the Advisor's reading of the
  options and the sim core spike's readiness. · *Research ids:* R2-BUD-REC6, R1-TICK-REC2.
AFTER:
- **OQ-0001-2. How does the stress scene's sim load reach the 60 Hz margin run (TICK-60-SETUP)?** · *Why it matters:*
  the proof test runs before the foundation with a throwaway sim, and the margin must be measured in one run, since
  p99s do not add; at 60 Hz the parts' ceilings already take 3.87 of 4.17 ms (ADR 0004 part 6), so a part the run
  leaves out can decide the tick rate, and the wave-1 sim has no ability and effect step, economy, objectives or
  moments yet (the test review, 2026-11-02). · *Options* (test set-ups, not research options): (a) the sim core
  spike's sim (ADR 0004's spike) runs inside the proof-test server for that run, and each part of the sim step share
  it lacks is stood in as ADR 0004's S1 stands it in, each stand-in named in the run's record: the effect step, which
  arrives with the content and ability spike (wave 3), as 384 rows in the effect pool (8 per soldier at N = 48) with
  its 1 ms ceiling burned per tick, and economy, objectives, moments and events as the remainder topped up per tick
  after the built projectiles and bot decisions, to 0.30 ms at 60 Hz (4.17 − 3.87, arithmetic on ADR 0004 part 6;
  2.80 ms at 30 Hz); (b) a stand-in burns, tick by tick, the step times the sim core spike recorded; (c) the 60 Hz
  check waits until the real sim runs the stress scene (slice 6), and the tick stays at 30 Hz until then; (d) the
  60 Hz check waits until the content and ability spike's effect step has joined the sim core spike's sim. **Recommended: (a)**
  with its burns: it keeps the single run, delays nothing and errs toward 30 Hz, since a pass still holds once the
  real parts meet their ceilings (the test review's reasoning); (c) is the fallback if the sim core spike's sim
  cannot run the stress scene in time. (b) is weaker because a replayed series loses the link between the sim's load
  and the server-only load in the same fight, and the margin is a tail statistic (our inference). The 30 Hz baseline
  marks may use a stand-in (with S1's stand-ins at 30 Hz: 384 effect rows and a 1 ms burn, the 2.80 ms remainder
  topped up): on the planning check they leave 7.49 ms. · *Decider:* the Advisor. · *What decides it:*
  the Advisor's reading of the options and the sim core spike's readiness. · *Research ids:* R2-BUD-REC6,
  R1-TICK-REC2.
END

### FIX NA-1 (d), NA Q1, SA-6, SA-8, GD-8 (the active-reload tap): OQ-0001-5, -6, -7
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **OQ-0001-5. The rewind cap: 200 or 250 ms, and the victim's side?** · *Why it matters:* at P2 a 30 Hz client needs
  200–250 ms of rewind, so 200 ms clips some shots at VISION_V2 §4's latency, while 250 ms lengthens the window in
  which a victim is hit just after reaching cover. · *Options:* (a) 200 ms (the default principles; the security review
  prefers it, with a tolerance of one snapshot interval); (b) 250 ms (the sync unit's proposal). · *Decider:* Mustafa
  and the Advisor. · *What decides it:* the proof test's numbers at P2 and P3 (the share of shots each cap clips, the
  "hit after cover" window), then a Feel Lab weighing. · *Research ids:* R2-XU-OQ1, R2-SYNC-OQ2, R2-SYNC-OQ15,
  R2-XU-OQ2.
- **OQ-0001-6. When does the server run each client's commands, and how does the client keep them just in time?** ·
  *Why it matters:* arrival order affects corrections, fairness and speed-hack exposure. · *Options:* (a) run commands
  as they arrive, several in one tick within the cap (Source); (b) buffer them and run exactly one per client per
  tick, repeating the last on a miss, late commands dropped and catch-up limited to the redundancy window; for the
  client, nudging its tick pace or re-aligning by skipping or repeating a tick (not researched). · *Decider:* the netcode proof test. ·
  *What decides it:* correction rates and fairness under P2 to P4. · *Research ids:* R2-SYNC-OQ3, R1-TICK-OQ8,
  R1-XT-OQ5 = R2-XT-OQ5 (the time-sync part).
- **OQ-0001-7. Do shots carry their moment inside a tick at 30 Hz?** · *Why it matters:* a 33 ms tick quantises when a
  click happens. · *Options:* (a) no: the command's tick and the view time's fraction only; (b) a sub-tick fire offset
  in the command. Not researched (no source read). · *Decider:* the netcode proof test, then Mustafa in the Feel Lab if
  it fails or feels late. · *What decides it:* R2-SYNC-REC5 mark 1's registration at 30 Hz. · *Research ids:*
  R1-TICK-OQ4.
AFTER:
- **OQ-0001-5. The rewind cap: which value, and the victim's side?** · *Why it matters:* at P2 a 30 Hz client needs
  200–250 ms of rewind by R2's formula and 233–383 ms once the input buffer and the page's blend are counted (part 8),
  so both candidate caps clip honest shots at VISION_V2 §4's latency, while a longer cap lengthens the window in which
  a victim is hit just after reaching cover; the cap is also the most rewind a client faking its latency can claim
  (part 8). · *Options:* (a) 200 ms (the default principles; the security review prefers it, with a tolerance of one
  snapshot interval); (b) 250 ms (the sync unit's proposal); (c) a cap set from the proof test's measured honest
  rewind (the p99 at P2), the one-interval tolerance unchanged (not researched: the research weighs 200 and 250 ms
  only); (d) run commands on arrival (OQ-0001-6 (a), Source's model), which removes the buffer's term from every
  rewind; (e) start the interpolation delay at 2 intervals instead of 3, at the cost of more extrapolation under loss
  (part 7). · *Recommendation:* none until the proof test reports; the netcode review leans to (c) with (d) measured
  beside it, but (c) is not researched, so choosing it is the deciders' call. · *Decider:* Mustafa and the Advisor. ·
  *What decides it:* the proof test's numbers at P2 and P3 (each shot's measured rewind, the share of shots each cap
  clips, the "hit after cover" window), then a Feel Lab weighing. · *Research ids:* R2-XU-OQ1, R2-SYNC-OQ2,
  R2-SYNC-OQ15, R2-XU-OQ2.
- **OQ-0001-6. When does the server run each client's commands, and how does the client keep them just in time?** ·
  *Why it matters:* arrival order affects corrections, fairness and speed-hack exposure, and the choice sets the
  honest rewind: a buffered command waits 1–3 ticks before it runs, which adds 33.3–100 ms to every rewind (part 8),
  while a command run on arrival adds none. · *Options:* (a) run commands as they arrive, several in one tick within
  the cap (Source), each command's tick strictly newer than the last one run, which with the lead bound stops a fast
  clock (part 5); (b) buffer them and run exactly one per client per tick, repeating the last on a miss, late commands
  dropped and catch-up limited to the redundancy window; for the client, nudging its tick pace or re-aligning by
  skipping or repeating a tick (not researched). · *Decider:* the netcode proof test. · *What decides it:* correction
  rates, fairness and the measured rewind under P2 to P4. · *Research ids:* R2-SYNC-OQ3, R1-TICK-OQ8, R1-XT-OQ5 =
  R2-XT-OQ5 (the time-sync part).
- **OQ-0001-7. Do shots and the active-reload tap carry their moment inside a tick at 30 Hz?** · *Why it matters:* a
  33 ms tick quantises when a click happens; for LOOP_V2 §7's Gears-style active reload ([DECIDED]) the page draws the
  window at the display's rate while the authority judges the tap by its tick, so near the window's edge the bar can
  show "in" and the verdict say "out". · *Options:* (a) no: the command's tick and the view time's fraction only; (b) a
  sub-tick offset in the command for fire and for the reload tap. Not researched (no source read). · *Decider:* for
  shots, the netcode proof test, then Mustafa in the Feel Lab if it fails or feels late; for the reload tap, Mustafa
  in the Feel Lab. · *What decides it:* R2-SYNC-REC5 mark 1's registration at 30 Hz; for the tap, the Feel Lab's
  timing check. · *Research ids:* R1-TICK-OQ4.
END

### FIX EA Q1, NA Q2, SA Q2, ROOM-CRASH = coordinator item 1 (new OQ-0001-15 to -18)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
  (r2-netcode.md, Open questions, lines 3260–3269).
AFTER:
  (r2-netcode.md, Open questions, lines 3260–3269).
- **OQ-0001-15. Is the local player drawn from the blend of the two newest blocks, one tick behind its prediction?**
  The page draws every entity between the two newest blocks (R1-TICK-REC5, r1-simulation-core.md line 2099), so the
  local player shows one tick behind its newest predicted state: 33.3 ms at 30 Hz against 16.7 ms at 60 Hz, about
  +25 ms from input to screen in all against 60 Hz (trade-off 2). · *Why it matters:* LOOP_V2 §7's "the shot feels
  instant" and the gun feel. · *Options:* (a) keep the blend for the local player too, and measure the absolute delay
  from input to screen; (b) draw the local player from the newest predicted tick, moved forward by the frame's
  fraction, while remote entities keep the blend (not researched; it risks small jitter); (c) decide after the proof
  test's numbers and Mustafa's feel check at P0. · *Recommendation:* (c), with (a)'s measurement in the proof test
  from the start (the engine review's). · *Decider:* Mustafa, by feel, with netcode-architect. · *What decides it:*
  the proof test's absolute input-to-screen p50 and p99 at P0 on ML and RD (reported), then his check. · *Research
  ids:* none carried (R1-TICK-REC5 is ADR 0004's; option (b) is not researched).
- **OQ-0001-16. How does each network profile split its latency and jitter over the two directions?** Part 13 gives
  the round trip only ("the injected latency is round-trip time": r2-netcode.md, lines 1987–1988) and proposes half
  the latency and a uniform ± half the jitter each way. · *Why it matters:* several marks pass or fail on the split:
  at P2 a snapshot is late if it arrives more than two intervals (66.7 ms) after sending, and with ±20 ms on the
  downlink (20 − 16.7) ÷ 40 = 8.3 % of snapshots are late against a 3 % mark, with ±10 ms none are, and with the
  whole round trip on the downlink every one is (the netcode review's arithmetic, 2026-11-02). · *Options:* (a) half
  each way, uniform (proposed: the round trip keeps the profile's mean and range); (b) the whole profile on each
  direction (stricter: P2 becomes a 200 ms round trip, P4's latency); (c) left to the simulator's builder and
  recorded. · *Recommendation:* (a). · *Decider:* the Advisor. · *What decides it:* the Advisor's reading, before the
  proof test starts. · *Research ids:* none carried (it refines part 13's answer to R2-XU-OQ12).
- **OQ-0001-17. Does a host's kick bar that player from the room for the rest of the match?** Today a kicked player
  who knows the room code can fetch a fresh token and rejoin (part 11). · *Why it matters:* a kick that does not hold
  leaves a host no way to keep a private room orderly but the lock. · *Options:* (a) yes: the room keeps the kicked
  player ids for its life (a guest who makes a new identity is held back by the per-address guest limit, keyed as in
  part 10); (b) no: a kick is only a disconnect, and the host uses the lock. · *Recommendation:* (a) (the security
  review's). · *Decider:* Mustafa (what a host may do, with R2-ROOM-OQ5). · *What decides it:* his call. · *Research
  ids:* none carried.
- **OQ-0001-18. When a room's simulation panics mid-match, does the match end (ROOM-CRASH)?** Part 11's proposal: the
  match ends, the crash is reported, clients get `RoomEnded` with a crash reason, and the process exits non-zero. ·
  *Why it matters:* it sets what players see when the server's sim fails, and what the supervisor and the match record
  handle. · *Options:* (a) the match ends (proposed): no VISION_V2 §6 item asks for resuming, and a tick that stopped
  halfway leaves state that cannot be trusted; (b) the room restores its last snapshot and continues (not
  researched; a generic feature no §6 item needs, which the trace rule keeps out). · *Recommendation:* (a) (the Rust
  review's). · *Decider:* the Advisor. · *What decides it:* the Advisor's ruling. · *Research ids:* none carried
  (R2-ROOM-REC2 mark 4: a room process that dies ends only its own match).
END

### FIX TE-3 (spike item 1)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
     input replay drives both transports to identical server state hashes on every tick (mark 2); over WebSocket, the
AFTER:
     input replay drives both transports to identical server state hashes on every tick (mark 2; a run counts only if
     every command ran at the tick stamped on it and the server's late-command counter reads 0, whatever OQ-0001-6
     picks for play: a late command makes the run invalid, not failed, and the input buffer is set deep enough for P0
     on both transports); over WebSocket, the
END

### FIX PA-2, NA-1 (e), EA-2 (spike item 5)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
   `maxDatagramSize` (R2-BUD-OQ3 = R2-TRN-OQ3, R2-XU-OQ5); E3 or E3b (0 desyncs of client-side flight over 1,000 throws,
   R2-BUD-REC2; only if projectiles are in the load); the process model (R2-ROOM-OQ1); the encoding cost per client
   (R2-BUD-OQ9); the controllers' behaviour under 2–5 % loss, server and browser side (R2-BUD-OQ10, R2-TRN-OQ8); each
   browser's negotiated draft and its behaviour with UDP blocked (R2-TRN-OQ1, R2-TRN-OQ2); the rewind estimate, by
   measuring how far behind the server's present the client's view is when its command arrives (R2-XU-OQ2); the real
   record and command sizes (R2-XU-OQ15, R2-XU-OQ4); linear against Hermite (R2-SYNC-OQ12); command timing
   (OQ-0001-6); whether shots need sub-tick stamps (OQ-0001-7). Reported, not judged: the egress per 48-player
   match-hour and the spike's own bill (R6-COST-REC1, ADR 0006).
AFTER:
   `maxDatagramSize` (R2-BUD-OQ3 = R2-TRN-OQ3, R2-XU-OQ5); whether any projectile kind needs E3b's per-snapshot
   updates (0 desyncs of client-side flight over 1,000 throws, R2-BUD-REC2; a desync is first a defect of the
   trajectory function, PM-0004-73; only if projectiles are in the load); the process model (R2-ROOM-OQ1); the
   encoding cost per client (R2-BUD-OQ9); the controllers' behaviour under 2–5 % loss, server and browser side
   (R2-BUD-OQ10, R2-TRN-OQ8); each browser's negotiated draft and its behaviour with UDP blocked (R2-TRN-OQ1,
   R2-TRN-OQ2); the honest rewind, by measuring how far behind the server's present the client's view is when its
   command runs, with each shot's RTT, delay, buffer wait and blend (R2-XU-OQ2; the cap itself is OQ-0001-5's); the
   real record and command sizes, whole and delta-coded (R2-XU-OQ15, R2-XU-OQ4); linear against Hermite
   (R2-SYNC-OQ12); command timing (OQ-0001-6); whether shots need sub-tick stamps (OQ-0001-7). Reported, not judged:
   the egress per 48-player match-hour and the spike's own bill (R6-COST-REC1, ADR 0006), and the absolute delay from
   input to screen at P0, p50 and p99 on ML and RD (OQ-0001-15).
END

### FIX PA-2 (migration step 1)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
   rate (30 Hz unless the 60 Hz margin passes), the WebTransport crate, E3 or E3b, the process model and the datagram
AFTER:
   rate (30 Hz unless the 60 Hz margin passes), the WebTransport crate, E3 and any per-kind E3b, the process model
   and the datagram
END

### FIX TE-3, TE-4 (test plan: determinism, counter wraps)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **Determinism by state hash:** recorded input replays give identical per-tick server state hashes on both transports
  (R2-TRN-REC3 mark 2) and on the loopback; the same seed and join-and-leave script give the same hash twice
  (R2-ROOM-REC6 mark 3); decoded snapshots rebuild the server's quantised state by hash (R2-SYNC-REC1 mark 1).
AFTER:
- **Determinism by state hash:** recorded input replays give identical per-tick server state hashes on both transports
  (R2-TRN-REC3 mark 2; a run counts only with 0 late commands, every command run at its stamped tick) and on the
  loopback; the same seed and join-and-leave script give the same hash twice (R2-ROOM-REC6 mark 3); decoded snapshots
  rebuild the server's quantised state by hash (R2-SYNC-REC1 mark 1).
- **Counter wraps:** a deterministic L1 test in the gate starts the server tick a few ticks below a multiple of 65,536
  and every 16-bit counter (sequence numbers, tick offsets) a few ticks below its wrap, so each run crosses every wrap
  within seconds while decoded snapshots still rebuild the server's state by hash; the 32-bit header tick cannot wrap
  inside a match (2^32 ticks at 30 Hz is about 4.5 years: arithmetic), and the soaks stay a second check (proposed:
  the 30 Hz soak's 54,000 ticks never reach 65,536 on their own).
END

### FIX SA-8, SA-9, SA-7, SA-13, GD-2, RR-3 (test plan: hostile input)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **Hostile input:** the forged-command and lag-switch suites (R2-SYNC-REC2 marks 3–4), the flood test on both
  transports (R2-TRN-REC1 mark 4, R2-TRN-REC3 mark 4), bad join tokens (R2-ROOM-REC3 mark 1), decoder fuzzing
  (R2-TRN-REC4 mark 4), forged upgrade picks and pickups refused and counted with no effect (part 12; proposed, the same
  harness as R2-SYNC-REC2 mark 3).
AFTER:
- **Hostile input:** the forged-command and lag-switch suites (R2-SYNC-REC2 marks 3–4), with a client whose clock
  runs fast (part 5's strictly newer ticks); the flood test on both transports (R2-TRN-REC1 mark 4, R2-TRN-REC3 mark
  4), with slow handshakes and with addresses cycled inside one IPv6 /64 (part 10); bad join tokens (R2-ROOM-REC3 mark
  1); decoder fuzzing (R2-TRN-REC4 mark 4); forged upgrade picks, purchases and pickups refused and counted with no
  effect (part 12; proposed, the same harness as R2-SYNC-REC2 mark 3); random sequences of decoded control messages
  driven into `room` (a pick on a closing offer, a host kicking itself, settings floods, join, leave and reconnect
  races) with 0 panics (proposed; the security review's); and a panic injected into the sim step, which ends the
  room's process with a non-zero exit, every client told or closed (part 11; proposed).
END

### FIX PA-2 (rollback)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **Client-side projectile flight desyncs:** E3b, about +42 kbps per client.
AFTER:
- **Client-side projectile flight desyncs:** a defect of the shared trajectory function, fixed there (PM-0004-73); for
  a kind that still desyncs, E3b for that kind only, its live count × 5.5 B per snapshot shown against the cap (E3b
  for every projectile breaks the budgets: part 6).
END

### FIX EA-5 + IE-1 (rollback: after acceptance)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **After acceptance:** the protocol carries its version in `protocols` and in `Join`, so a message change is a
  version bump; the frozen legacy game stays the playable build until v2 replaces it.
AFTER:
- **After acceptance:** the protocol carries its version in `protocols` and in `Join`, so a message change is a
  version bump; the lobby can send each client only to rooms of its own build (part 11), and whether two versions run
  side by side during a release, old rooms finishing while new ones start, is OQ-0006-14 (WT_DRAIN_SESSION can drain
  a session before a server shuts down: r2-netcode.md, R2-TRN-F37, lines 3439–3440); the frozen legacy game stays
  the playable build until v2 replaces it.
END

### FIX NA-3 (Appendix A: R1-ABL-REC4)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| R1-ABL-REC4 | adopted: the player's own weapon state is predicted, damage never; its ability half waits for ABL-PREDICT | part 4; OQ-0001-4 |
AFTER:
| R1-ABL-REC4 | adopted: the player's own weapon state is predicted, damage never; own cues are keyed by (entity, weapon, command sequence), so a replay never plays one twice, and the server never sends a client its own shots; its ability half waits for ABL-PREDICT | part 4; OQ-0001-4 |
END

### FIX PA-2 (Appendix A: R1-XT-OQ4)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| R1-XT-OQ4 | answered: E3; correctness never rests on cross-target identity, since landings are server events; E3b if the proof test counts a desync | part 6 |
AFTER:
| R1-XT-OQ4 | answered: E3; correctness never rests on cross-target identity, since landings are server events; a desync is first a defect of the trajectory function (PM-0004-73), and E3b is kept per projectile kind only | part 6 |
END

### FIX GD-1, SA-8, NA-9 (Appendix A: R2-SYNC-REC2)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| R2-SYNC-REC2 | adopted | part 5 |
AFTER:
| R2-SYNC-REC2 | changed: an aim point beyond the weapon's range is clamped to it, not refused, so it leaves mark 3's forged set (r2-netcode.md line 2062; a top-down cursor reaches about 28 m, 44 m with the ADS pan, R2-INT-F18, past the Beam Katana's 3.6 m and the Molotov's 0, LEGACY_TUNING.md:181); a command's tick must be strictly newer than the last one run, not only "not older" (line 2044; equal ticks allow a sustained speed hack); the snapshot also acknowledges the newest command received | part 5 |
END

### FIX NA-1, SA-5, SA-6, EA-1, PA-2 (Appendix A: R2-SYNC-REC5, R2-SYNC-REC6)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| R2-SYNC-REC5 | adopted; the cap value stays open | part 8; OQ-0001-5 |
| R2-SYNC-REC6 | adopted (E3, with E3b as the fallback) | parts 6, 7 |
AFTER:
| R2-SYNC-REC5 | changed: the server's estimate adds the command's buffer wait and half a tick for the page's blend to R2's RTT plus delay (Source's run-on-arrival model leaves both out); the RTT is client-inflatable, an accepted risk the cap bounds, with quinn's RTT and the ping RTT logged side by side; the per-command log goes to a per-match file, never the log stream; the cap value stays open | part 8; OQ-0001-5 |
| R2-SYNC-REC6 | adopted: E3, with E3b only per projectile kind (a desync is first a defect of the trajectory function, PM-0004-73); blasts on the timeline of the entity they hit | parts 6, 7 |
END

### FIX PA-3, PA-1 (Appendix A: R2-BUD-REC1, R2-BUD-REC2)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| R2-BUD-REC1 | changed: the snapshot half (30 Hz) stands; the tick half is replaced by TICK-30; inputs repeat every unacknowledged command up to the cap (R2-SYNC-REC2) instead of 3; marks restated at 30 Hz and against P2 and P3; its Shabzon mark (mean delivered fire rate within 1 % of 13.9 shots/s at the chosen tick, r2-netcode.md:2324–2325) is a sim mark, carried by ADR 0004's mark S4 (one time representation holds every rate, which meets R1-ABL-REC3's 1 % for the Shabzon) | parts 2, 5; budgets table; ADR 0004 |
| R2-BUD-REC2 | changed: its precision mark "decoded positions within 1 cm and angles within 0.01° of the server's values" (r2-netcode.md:2341) cannot pass for pitch, sent in 10 bits of 0.18° steps (line 1748); restated as positions within 1 cm, yaw within 0.01° (a 16-bit step is about 0.0055°, line 2263) and pitch within one 0.18° step (proposed: the field's own resolution); the hard cap proposed at 256 kbps | part 6; budgets table |
AFTER:
| R2-BUD-REC1 | changed: the snapshot half (30 Hz) stands; the tick half is replaced by TICK-30; inputs repeat every unacknowledged command up to the cap (R2-SYNC-REC2) instead of 3, delta-coded (its U2, adopted at once, since part 5's command is about twice its 8 B input); marks restated at 30 Hz and against P2 and P3; its Shabzon mark (mean delivered fire rate within 1 % of 13.9 shots/s at the chosen tick, r2-netcode.md:2324–2325) is a sim mark, carried by ADR 0004's mark S4 (one time representation holds every rate, which meets R1-ABL-REC3's 1 % for the Shabzon) | parts 2, 5; budgets table; ADR 0004 |
| R2-BUD-REC2 | changed: its precision mark "decoded positions within 1 cm and angles within 0.01° of the server's values" (r2-netcode.md:2341) cannot pass for pitch, sent in 10 bits of 0.18° steps (line 1748); restated as positions within 1 cm, yaw within 0.01° (a 16-bit step is about 0.0055°, line 2263) and pitch within one 0.18° step (proposed: the field's own resolution); the hard cap proposed at 256 kbps; its mean and p99 are judged in R2's estimate scene, where it set them (lines 2339–2340), and the stress scene is held by the cap | part 6; budgets table |
END

### FIX RR-3 (Appendix A: R2-ROOM-REC2)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| R2-ROOM-REC2 | adopted | part 11 |
AFTER:
| R2-ROOM-REC2 | adopted; a panic on the tick thread ends the match and exits the process non-zero (OQ-0001-18) | part 11 |
END

### FIX SA-3 (Appendix A: R2-ROOM-REC8)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| R2-ROOM-REC8 | adopted; where the secret finally lives is R2-ROOM-OQ11 (ADR 0006) | part 11 |
AFTER:
| R2-ROOM-REC8 | adopted; where the secret finally lives is R2-ROOM-OQ11 (ADR 0006); the SameSite=Strict cookie needs the client host and the lobby on one registrable domain (ADR 0006) | part 11 |
END

### FIX PA-2 (Appendix A: R2-BUD-OQ8)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| R2-BUD-OQ8 | answered: E3, E3b if the proof test counts a desync | part 6 |
AFTER:
| R2-BUD-OQ8 | answered: E3; E3b only for a projectile kind that still desyncs once the trajectory function holds (PM-0004-73) | part 6 |
END

### FIX NA-1 (e) (Appendix A: R2-XU-OQ2)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| R2-XU-OQ2 | spike: the netcode proof test measures how far behind the server's present the client's view is when its command arrives | part 8 |
AFTER:
| R2-XU-OQ2 | spike: the netcode proof test measures how far behind the server's present the client's view is when its command runs, with each shot's RTT, delay, buffer wait and blend | part 8 |
END

### FIX PA-3 (Appendix A: R2-XU-OQ4)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
| R2-XU-OQ4 | answered: every unacknowledged command up to a cap of 8 (data) within 48 kbps; the real size is measured by the proof test | part 5; bandwidth estimate |
AFTER:
| R2-XU-OQ4 | answered: every unacknowledged command up to a cap of 8 (data), the repeats delta-coded, within 48 kbps; the real size is measured by the proof test | part 5; bandwidth estimate |
END

### FIX EA-8 as routed from ADR 0002 (test plan: load and soak)
FILE: docs/adr/0001-multiplayer-netcode.md
BEFORE:
- **Load and soak:** bots as headless protocol clients, a full room at each mode's N, 30 minutes at P2 for every mode on
AFTER:
- **Load and soak:** bots as headless protocol clients on ADR 0002's `client-endpoint` crate, the endpoint the browser
  runs, a full room at each mode's N, 30 minutes at P2 for every mode on
END

### NOTE EA-1
FIX: part 7 now states the timing rule (spatial events written into the first block whose delayed time reaches their
tick; own, predicted, owner and HUD-only events at once), the client-tick bullet of "Components and data flow" follows,
and the `Snapshot` row says the event section is written at part 7's time. Routed: ADR 0002 part 5 (engine-architect)
must read that the block's event section holds the events due at that block, by part 7's rule; PM-0002-9
(test-engineer) checks order per timeline, and one new row checks that an event is drawn within one frame of its
entity's pose.
END

### NOTE GD-1
FIX: part 5 clamps an aim point beyond the weapon's reach (never refused; what a throw aimed past its reach does is
rule data, ADR 0005), and Appendix A's R2-SYNC-REC2 row says "changed". Routed: PM-0001-21 (test-engineer) drops
"beyond weapon range" from the forged set and keeps "off the map".
END

### NOTE NA-1 (and its Q1)
FIX: part 7 defines the client's server time; part 8's estimate adds the command's buffer wait and half a tick for the
blend, and restates the P2 arithmetic (233–383 ms); trade-off 1, OQ-0001-5 (options (c) to (e), retitled) and
OQ-0001-6 (the rewind in "why it matters") follow; spike item 5 and Appendix A's R2-XU-OQ2 row measure each shot's
RTT, delay, buffer wait and blend. Q1 is carried inside OQ-0001-5 (no separate question); option (c) is labelled not
researched, so the draft recommends no value until the proof test reports. Routed: PM-0001-20, -48 and -49 and
checklist step 10 (test-engineer) follow part 8's estimate: PM-0001-49 records per shot the RTT, the announced delay,
the buffer wait and the blend fraction; step 10 adds a run with the cap at the measured p99 at P2; PM-0001-48's
tolerance arithmetic uses the new estimate. docs/adr/README.md (engine-architect): OQ-0001-5's row takes the new
title "The rewind cap: which value, and the victim's side?".
END

### NOTE FA-L1 = A8
FIX: part 6's list now names every gameplay event of ADR 0004 part 19 (harvester deliveries and kills, structure kills
and stage changes added). Blasts: resent, not sent once. A blast that ends a projectile is that projectile's final
trajectory change, which the delta coding repeats until acknowledged (so it costs no event of its own); any other blast
is a gameplay event; beyond a projectile's relevance, only a cosmetic sound event inside part 9 (f)'s radius.
END

### NOTE EA-2 (and its Q1)
FIX: Q8's T2 row and trade-off 2 state the full arithmetic (+8.3 ms of input wait, +16.7 ms of display, about +25 ms
against 60 Hz); the not-researched option (draw the local player from its newest predicted tick) and Q1 are the new
OQ-0001-15 (decider Mustafa, by feel, with netcode-architect; recommended: decide after the proof test, measuring the
absolute delay from the start); spike item 5 reports the absolute input-to-screen p50 and p99 at P0 on ML and RD.
Routed: PM-0001-43 or PM-0002-13 (test-engineer) reports that absolute delay (reported, no pass number).
END

### NOTE EA-5 and IE-1
FIX (ADR 0001's half only): part 11's room directory records each room's protocol version and content hash and the
lobby routes joins by them; the failure-mode row no longer loops a refused client (one reload, only if the lobby names a
newer build; proposed); Rollback names two versions side by side as OQ-0006-14, with R2-TRN-F37's WT_DRAIN_SESSION;
the directory carries each room's build, as OQ-0006-14's option (b) asks ("a field ADR 0001's directory would add"),
harmless under its option (a). Nothing routed: infra-engineer's 8fa7173 holds the release question as OQ-0006-14.
END

### NOTE GD-2
FIX: part 12 adds `Buy{unitId}` (a unit or a vehicle) on the control stream, checked against the mode's content, the
team's tier (AB-5), the price with power-plant pricing (LOOP_V2 §3) and the team's own buy terminal (LOOP_V2 §2, §5),
as rule data; never predicted; the catalogue gains a `Buy` row; anti-cheat, the hostile-input test plan and part 4's
never-predicted list follow. Need: VISION_V2 §6.5. Routed: PM-0001-21 (test-engineer) gains a purchase failing each
condition; docs/adr/README.md (engine-architect) lists the `Buy` message with its trace if the register lists messages;
ADR 0005 (game-designer, AB-5 or the content model) holds the purchase rule's data (where and when a player may buy).
END

### NOTE GD-3
FIX: part 4 says what the own-state section carries (position, velocity, weapon state and the active effect rows or
attribute values that own movement and own weapon state read, picks and ability effects included); part 12 points to
it; the `Snapshot` row cites part 4's list. Routed: test-engineer (a PM-0001 row) or ADR 0005's content and ability
spike: 0 corrections at P2 from a scripted pick and a scripted effect that change move speed and fire interval.
END

### NOTE NA-3
FIX: part 4 keys own cues by (entity, weapon, command sequence) so a replay plays none twice (R1-ABL-REC4, lines
2427–2428), and part 6 never sends a client its own cosmetic shot events; Appendix A's R1-ABL-REC4 row follows. Routed:
test-engineer adds the row (at P2, replays forced as in PM-0001-44, every own shot's muzzle flash and sound plays
exactly once over 1,000 scripted shots; bites: cues written from the replay; the server echoing the owner's shots).
END

### NOTE PA-2
FIX: part 6 makes a desync a defect of the trajectory function (PM-0004-73), keeps E3b per projectile kind only and
shows why whole E3b cannot pass (188.3 kbps in R2's scene; 1,650 B per snapshot in the stress scene against the
1,067 B cap); Q6, Q9, trade-off 4, Revisit when, spike item 5, migration step 1, Rollback and Appendix A (R1-XT-OQ4,
R2-SYNC-REC6, R2-BUD-OQ8) follow. Routed: PM-0001-54's failure route and the "When a mark fails" sentence
(test-engineer) read: a desync is first fixed in the trajectory function; per-kind E3b only, its count × 5.5 B shown
against the cap; PM-0001-12 is not re-run with whole E3b.
END

### NOTE PA-3
FIX: the upstream bullet restates the arithmetic with part 5's command (about 15–20 B, the perf review's estimate): U1
at P2 25.9–43.9 kbps, at the cap 43.9–53.5 kbps against 48 kbps; part 5 adopts U2's delta-coded repeats at once (with
the newest-received acknowledgement of NA-9 narrowing the window); Q10's chosen row moves from U1 to U2; Appendix A's
R2-BUD-REC1 and R2-XU-OQ4 rows follow. The cap stays 8 (data). Note for the record: the perf review's upper figures
(46.8 and 57.4 kbps) imply 22 B commands; at its own 15–20 B the arithmetic gives 43.9 and 53.5 kbps (shown in the
bullet). PM-0001-13 already records the real size; it should record both forms (test-engineer).
END

### NOTE SA-3
FIX: part 11 states the SameSite=Strict consequence (the client host and the lobby on one registrable domain, MDN as
the security review read it), pointing to ADR 0006 part 6, item B4 and OQ-0006-15 (the domain); Appendix A's
R2-ROOM-REC8 row follows. R2-ROOM-REC8 mark 2 then runs on that setup in Brave (test-engineer).
END

### NOTE SA-5
FIX: part 8 sends the per-command offsets and aims (and the two RTTs once a second) as raw entries to a per-match side
file beside the match record, keyed by slot, never the `tracing` stream, with ADR 0006 part 9's figures (1,728,000
entries per 20-minute match at N = 48: about 518 MB as log lines, about 27.6 MB at 16 B a raw entry) and its 14 days.
ADR 0006 part 9 left the form (raw entries or per-player summaries) to ADR 0001: raw entries, proposed, until the
detection policy (B7) says what it needs. Nothing further routed (8fa7173 holds the store and retention).
END

### NOTE SA-6
FIX: part 8 and the anti-cheat bullet state the accepted risk (a client faking latency gets rewind up to the cap) and
log quinn's `rtt()` beside the ping RTT per client; OQ-0001-5 says the cap is also the most rewind a cheat can claim;
Q5's option B row follows. Routed: PM-0001-49 and -50 (test-engineer): the cheat client set includes one that delays
every pong by 150 ms; PM-0001-33's ±5 ms agreement is the honest baseline for the gap signal.
END

### NOTE SA-7
FIX: part 10 keys every per-address limit by IPv4 /32 and IPv6 /64 (the prefix length as data; RFC 4291 §2.5.1 as the
security review read it); part 11, the lobby-calls row and anti-cheat follow. Routed: PM-0001-25 (test-engineer)
includes one client cycling addresses inside one /64; ADR 0006 (infra-engineer) uses the same keying for the crash
endpoint's limits (SA-14).
END

### NOTE SA-8
FIX: part 5's check is "strictly newer than the last command run", so a fast clock runs no more commands than the
server's ticks plus the lead bound; OQ-0001-6 (a), anti-cheat and Appendix A's R2-SYNC-REC2 row follow. Routed:
PM-0001-21 (test-engineer) adds a client whose clock runs 1.5 times fast for 60 s and moves no further than an honest
client plus the lead bound.
END

### NOTE IE-7
FIX: part 10 points to OQ-0006-16 (the port plan: infra-engineer with netcode-architect, confirmed by the Advisor, as
8fa7173 wrote it) and, until it is answered, names its option (a) as the interim: one UDP and one TCP port per room
process from the VM's ranges, handed out by the lobby with the host (R2-ROOM-F18, r2 line 1562). ADR 0001 adds no port
question of its own. My view as co-decider: (a) for the netcode proof test; the TCP 443 question waits for
OQ-0001-13's fallback share, as OQ-0006-16 says. Routed: PM-0001-29 (test-engineer) adds one run from a network that
allows only TCP 443 (proposed).
END

### NOTE IE-11
FIX: the message catalogue gains a "Lobby ↔ supervisor" row (register, ask for a room, host and both ports or a
refusal, Closed, abnormal exit, draining; ADR 0006 part 6, not researched); part 11 points to it. Routed: ADR 0006 part
6 (infra-engineer) holds the interface paragraph with the same four exchanges; docs/adr/README.md's I-11 row
(engine-architect) reads "host and both ports".
END

### NOTE SA-9
FIX: part 10's `Join` deadline runs from the TCP accept or the QUIC handshake's first packet (TLS and the upgrade
inside it), with a cap on handshakes in progress; the hostile-input test plan adds slow handshakes. Routed: PM-0001-25
(test-engineer) adds a slow-loris client (one byte a second into the ClientHello and into the upgrade headers), closed by
the deadline.
END

### NOTE SA-12
FIX: part 11: the lobby validates display names (bounds as data: bytes and code points, no Cc or Cf, NFC; proposed, the
security review's) and the room re-checks the length before the roster.
END

### NOTE EA-10
FIX: the lobby-calls row lists crash reports and client field data, one report endpoint on the lobby (ADR 0006 part
10, as 8fa7173 put it).
END

### NOTE GD-8 (its 0001 items)
FIX, all four: "three" is no longer hard-coded (part 12: "three choices by default; the offer's size is data, ADR 0005
AB-8"; "a choice outside the offer"; the `PickUpgrade` row's "an index into the offer"); honest double clicks and
in-flight picks are counted apart from malformed ones (part 12 and the `PickUpgrade` row); E for a pickup against E for a
steal or a placement is rule data on OQ-0005-AB9's list (part 12); OQ-0001-7 now covers the active-reload tap, with the
Feel Lab deciding it (README's OQ-0001-7 row takes the new title: engine-architect). Routed: game-designer adds the
E-press precedence to OQ-0005-AB9's list.
END

### NOTE NA-9
FIX: each snapshot acknowledges the newest command received (ends repeats) beside the last one run (the replay's start);
the `Snapshot` header row and the upstream bullet follow (3 to 6 commands at P2, arithmetic shown).
END

### NOTE NA-12
FIX: part 2: 30 snapshots a second, every tick at 30 Hz, every second tick in the 60 Hz margin run. Routed: PM-0001-39
(test-engineer) names the snapshot rate.
END

### NOTE NA-13
FIX: a `Resync` message (client → room, control stream, rate-limited to one in flight, answered with `FullState`) in the
catalogue; part 6 names both resync paths; the two failure rows use it. Need: R1-TICK-REC5's online resync (VISION_V2
§4), already cited in the hidden-tab row.
END

### NOTE NA-14
FIX: part 6: a generation change at an index the client holds is a removal plus an addition, which also tells the
client to place the soldier (part 7's placing rule).
END

### NOTE NA-16
FIX: "parts and seated players" becomes "parts, and the pilot," in parts 6 and 9 (f).
END

### NOTE fixcheck-a optional note (PM-0002-25)
FIX: trade-off 5 cites ADR 0002's PM-0002-25 as the row that reports the Worker's time per tick in a local match.
END

### NOTE NA Q2
FIX: the new OQ-0001-16 (decider: the Advisor, before the proof test; recommended: half the latency and a uniform
± half the jitter each way), and part 13 states that split as proposed. Routed: PM-0001-3 and PM-0001-37
(test-engineer) follow the split and measure lateness from the expected arrival (NA-5).
END

### NOTE SA Q2
FIX: the new OQ-0001-17 (decider: Mustafa, with R2-ROOM-OQ5; recommended (a), a kick bars the player id for the
match); part 11's host-command sentence points to it. docs/adr/README.md (engine-architect) adds the row.
END

### NOTE GD-REJOIN
FIX (a pointer, no second question): part 11's reconnect sentence points to ADR 0004's OQ-0004-10 (as the coordinator
routed from rust-reviewer's batch, b815c85), with OQ-0005-AB10.
END

### NOTE RR-2 (its 0001 half)
FIX: part 11: room timers count ticks, `room` reads no clock and no OS randomness, wall-time limits live in the server's
I/O side; part 5: the client endpoint reads no clock, the host passes the time in. Routed: ADR 0002 parts 3, 5 and 7
(engine-architect): the lint covers `room`, `protocol` and `sim-wasm`; the host passes time and the receive stamps in the
input call; PM-0002-41's bite (`Instant::now()` in `room` fails) is test-engineer's.
END

### NOTE RR-3 and ROOM-CRASH
FIX: part 11: a tick-thread panic is fatal (caught at the loop's top, reported, `RoomEnded` with a crash reason, the
record marked crashed, a non-zero exit), and the tick and I/O threads share only bounded channels (R2-ROOM-F8 cited);
a failure-mode row, the connection-panic row and the hostile-input test plan follow; Appendix A's R2-ROOM-REC2 row
follows. ROOM-CRASH is the new OQ-0001-18 (decider: the Advisor; recommended (a), the match ends). Routed: ADR 0004
part 20 (rust-reviewer, now at b815c85) may name "OQ-0001-18" where it says "the Advisor's question in ADR 0001";
test-engineer adds the mark beside PM-0001-26 (a panic injected into the sim step ends the process within 1 s with a
non-zero exit, every client told or closed; bite: a tick thread whose panic leaves the process running).
END

### NOTE TE-1 (its 0001 part)
FIX: OQ-0001-2's "why it matters" names the missing parts; option (a) now carries ADR 0004 S1's stand-ins (384 effect
rows with a 1 ms burn; the remainder topped up to 0.30 ms at 60 Hz, 2.80 ms at 30 Hz); a new option (d) waits for the
content and ability spike's effect step; the recommendation stays (a), with the test review's reason. The test review's
Q1 is answered inside OQ-0001-2 (the Advisor's). Routed: SPIKE_PLAN §2 and §4 (engine-architect) read the same
stand-ins for the 60 Hz margin run; PM-0004-2 and PM-0001-2's stand-in clause follow (test-engineer).
END

### NOTE TE-3
FIX: spike item 1 and the test plan: the two-transport hash replay counts only with the late-command counter at 0,
every command run at its stamped tick, whatever OQ-0001-6 picks for play; a late command makes the run invalid, not
failed. Routed: PM-0001-30 (test-engineer) follows.
END

### NOTE TE-4
FIX: the test plan gains a deterministic L1 gate test that starts the server tick and every 16-bit counter just below
its wrap (the 32-bit header tick cannot wrap in a match: about 4.5 years at 30 Hz); the failure-mode row points to it
instead of the 60 Hz soak. Routed: test-engineer writes the counter-wrap row.
END

### NOTE also handled (named against ADR 0001's text in the reviews, not in the prompt's list)
- NA-4 (its 0001 half): parts 5 and 7 time clock and jitter samples by the datagram's arrival stamp; ADR 0002 part 5
  (engine-architect) passes the stamps in the input call; PM-0001-47's bite (receive times quantised to the tick) is
  test-engineer's.
- NA-10 (its 0001 half): part 4 states the real replay depth (5 to 7 ticks at P2, deeper at P3 and P4) and that the
  proof test records it per profile; PM-0001-44 (test-engineer) forces the P2 maximum.
- SA-2 (its 0001 half): part 11 signs tokens with a separate key per environment, so a production room refuses every
  staging or preview token; ADR 0006 part 4 (infra-engineer) holds the walled staging endpoint.
- SA-13 (its 0001 half): the hostile-input test plan drives random sequences of decoded control messages into `room`;
  PM-0001-34 or PM-0004-60 (test-engineer) carries the mark.
- PA-1 (its 0001 half, 0001:510, :539): the budgets row judges the 160 kbps mean and the 256 kbps p99 in R2's estimate
  scene, where R2-BUD-REC2 set them, and the stress scene by the cap, its mean reported; the downstream bullet shows the
  shot arithmetic; Appendix A's R2-BUD-REC2 row follows. This restores the research's own scene, so I see no need for a
  separate SS-BANDWIDTH open question; if the Advisor wants one, it is the perf review's. PM-0001-12 (test-engineer)
  follows (NA-2 and PA-1).
- GD-19 and GD-20: part 12 names GD-PICK-VIS (who else sees picks, Mustafa's); the uplinks state row names ADR 0004's
  OQ-0004-9 (GD-CARRIER). GD-PICK-VIS has no open-question number yet: docs/adr/README.md (engine-architect) lists it
  as DESIGN, or game-designer adds it beside OQ-0005-AB7, as the session routes it.
END

### NOTE routed from ADR 0002 (engine-architect's batch, not yet applied)
FIX, all six: part 7's event-release rule names both match kinds (EA-1); parts 5 and 7 use the arrival stamps, the
previous send time and the host's current time the input call passes as plain values, and part 5 says the client's
pong carries its hold time, which the server subtracts up to one tick period, while the server answers pings on its
I/O runtime as they arrive (NA-4; the `Ping` / `Pong` row follows); part 5, the components diagram and the client-tick
bullet read "the newest input, posted once per drawn frame", the Worker building the one command from the newest post
(NA-7); part 11 lists the timers `room` counts in ticks (the state timeouts, the 60 s grace window as 1,800 ticks)
and those `server` keeps on wall time (the `Join` deadline, the handshake cap, rate limits, the ping schedule) (RR-2);
part 10 leaves the staging server's certificate to OQ-0006-5 with ADR 0002 part 10's two shapes (IE-4); part 4, the
diagram and the load-and-soak line name the `client-endpoint` crate (EA-8). These hold whether or not ADR 0002's
batch lands first: each names ADR 0002 by part only.
END

### NOTE for the synthesis (engine-architect: docs/adr/README.md, SPIKE_PLAN, ENGINE_OVERVIEW)
- The open-question register: OQ-0001-5's title becomes "The rewind cap: which value, and the victim's side?";
  OQ-0001-7's becomes "Do shots and the active-reload tap carry their moment inside a tick at 30 Hz?" (decider: the
  netcode proof test, then Mustafa in the Feel Lab; the tap, Mustafa in the Feel Lab); new rows OQ-0001-15 (own
  movement drawn one tick behind: Mustafa, with netcode-architect), OQ-0001-16 (the profiles' direction split: the
  Advisor), OQ-0001-17 (does a kick bar the player: Mustafa), OQ-0001-18 (ROOM-CRASH: the Advisor); README:35's
  TICK-60-SETUP row names option (a)'s stand-ins; the I-11 row reads "host and both ports".
- ENGINE_OVERVIEW:37 and :94 read "the newest input, posted once per drawn frame" (as engine-architect's own ADR 0002
  note already routes); its diagram may name `client-endpoint` beside the client endpoint, as ADR 0001's now does.
- The trace list, if it lists messages: `Buy` (VISION_V2 §6.5), `Resync` (VISION_V2 §4 through R1-TICK-REC5's online
  resync), the lobby ↔ supervisor row (VISION_V2 §6.1).
- SPIKE_PLAN item 1 subsumes R1-ABL-REC4's own-weapon half (SPIKE_PLAN:123), which part 4 now states and test-engineer's
  new row tests; SPIKE_PLAN items 1 and 2 read OQ-0001-2 (a)'s stand-ins.
END
