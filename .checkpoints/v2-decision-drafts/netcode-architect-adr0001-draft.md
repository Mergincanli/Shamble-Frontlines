# ADR-0001: Multiplayer and netcode

**Status:** Proposed
**Date:** 2026-10-31
**Author:** netcode-architect (the "Pass marks" table: test-engineer, plan step 4) · **Reviewed by:** (filled in at review) · **Signed off by Mustafa:** no

This record decides how v2 plays online. One match server decides everything that counts; players send only their
inputs, one command per tick; the server sends each player a compressed picture of what that player may see, 30 times
a second. A player's own movement and own gun react at once on their screen and are corrected smoothly when the server
disagrees; everyone else is drawn about 100 ms in the past; a shot is judged against what the shooter saw, within a
cap. The server ticks at 30 Hz, and 60 Hz only if the netcode proof test shows the 60 Hz margin. The record also sets
the budgets that test is judged against, the message catalogue, the transport (WebTransport with a WebSocket fallback
behind one interface, plus a local backend so the game runs as a client plus a local server from slice 1), rooms,
private matches and bots on the server. The reason: VISION_V2 asks for matches of up to 48 players that feel fair to
about 100 ms of ping, and the R2 research found one path that shipped games use.

## Context

### Requirements
- **Players:** the largest mode is 24v24, so [N](../research/GLOSSARY.md#player-count-n) = 48 (MODES-3 (a); brief §1),
  with a headroom run at N = 64 ([the design point and headroom](../research/GLOSSARY.md#the-design-point-and-headroom)).
  The start modes are 1v1 to 4v4 and 2v2v2 to 6v6v6; the goal is 12v12v12 or 24v24 (VISION_V2 §3). Bots fill empty
  slots (VISION_V2 §3 [DEFAULT], LOOP_V2 §2).
- **Tick:** 30 Hz is the baseline; 60 Hz only if the netcode proof test shows at least 25 % headroom (TICK-30, brief §1),
  read as [the 60 Hz margin](../research/GLOSSARY.md#the-60-hz-margin) (TICK-60-HEADROOM, the advisor's reading of
  2026-10-31, confirmed by Mustafa and the Advisor at the stop).
- **Latency and region:** matches feel fair up to about 100 ms of ping; servers start in one EU region (VISION_V2 §4
  [DEFAULT]). Desktop Chrome, Edge, Firefox and Safari (VISION_V2 §4 [DECIDED]).
- **VISION_V2 §6 items served:** 6.1 (rooms and private matches, modes as data), 6.2 (aiming at height; what is hidden
  follows floors and interiors), 6.3 (abilities as input edges; the prediction scope), 6.4 (vehicles: the pilot's
  prediction, parts in snapshots and in the hit history), 6.5 (economy and objectives server-side), 6.6 (the room
  writes the match record), 6.8 (tick-stamped events for moments). §6.4's seats are [OPEN], §6.8's kill-cams, replays
  and highlights and §6.10's accounts are [LATER]: this record builds none of them and says what keeps them possible.
- **LOOP_V2:** §7 split feedback ("the shot feels instant; hit feedback arrives when the hit is confirmed"); §4 the
  uplink countdown and steals, and a cache location everyone is told; §3 personal money; §9 aiming at height and gentle
  aim help ([OPEN], technical); §8 a match of about 20 minutes; §10 bots across three teams.
- **LOOK_V2:** §3 interiors and upper floors; §4 ruins you can enter and roofs that fade when the player is inside
  [DEFAULT]. What a player is sent follows the same enclosures; how the cutaway looks is ADR 0003's.
- **Settled inputs:** the stack (WebTransport with a WebSocket fallback, our own netcode, server authority: brief §1);
  MODES-3 (a); TICK-30; SLICE-ORDER and its three conditions (STATUS, "Decisions pending"); SEC-SPREAD (a); ABL-PREDICT
  (waits for a Feel Lab test); BASELINE-WHEN (b); UPGRADE-SCOPE (Mustafa, 2026-10-31); SPIKE-SCOPE (a); PREVIEW-WALL
  (a); [the foundation](../research/GLOSSARY.md#the-foundation) and
  [the netcode proof test](../research/GLOSSARY.md#the-netcode-proof-test) (GLOSSARY §8; brief §2).

### The findings the decision turns on
1. Delta snapshots against each client's last acknowledged snapshot tolerate loss and reordering without stalls, cost
   nothing for unchanged entities, support joining mid-match, and need no bit-exact agreement between browsers (Quake
   III and Fiedler: r2-netcode.md, Recommendations, Brief question 1, R2-SYNC-REC1, lines 1996–2024). The netcode must
   not depend on identical results between Wasm and native; lockstep is rejected (r1-simulation-core.md,
   Recommendations, unit determinism, R1-DET-REC1, lines 2129–2145).
2. At N = 48, everyone in view, delta encoding (E3) at 30 Hz costs about 146.1 kbps per client; 60 Hz snapshots cost
   about 286.1 kbps, over Fiedler's 256 kbps example budget (r2-netcode.md, Options, Brief question 2, worked bandwidth
   estimate, lines 1770–1790; R2-BUD-REC1, lines 2302–2325).
3. The parts of [the server tick budget](../research/GLOSSARY.md#the-server-tick-budget) at N = 48 add up, as a planning
   check of ceilings, to 9.18 of 16.67 ms at 30 Hz (10.18 on a tick that re-floods after a portal change) and 7.28 of
   8.33 ms at 60 Hz, where a re-flood tick that also encodes a snapshot reaches 8.28 ms and its server-only share 4.41
   of 4.17 ms (GLOSSARY.md, §1 Tick budgets, lines 53–80). This is why TICK-30 makes 30 Hz the baseline.
4. In a browser every received byte is readable, so withholding data is the only defence against wallhacks that holds
   (r2-netcode.md, Recommendations, Brief question 3, R2-INT-REC2, lines 2445–2465).
5. QUIC guarantees only 1,200-byte UDP payloads and forbids IP fragmentation, and a datagram cannot be fragmented, so a
   48-player snapshot must split into datagrams that decode alone (r2-netcode.md, R2-BUD-REC3, lines 2345–2360).
6. Neither Rust WebTransport crate claims production readiness, and some networks do not carry QUIC, so the
   WebTransport layer sits behind a thin interface with a WebSocket fallback (r2-netcode.md, Recommendations, Brief
   question 4, R2-TRN-REC1, lines 2565–2615; R2-TRN-REC3, lines 2639–2671).
7. A spread seed the client knows lets a cheat cancel the spread through the aim point it sends (the security review's
   finding H2: r2-netcode.md, R2-SYNC-REC3, lines 2068–2105).

### What this record takes from and gives to the other drafts
- **Takes:** from ADR 0004, the fixed-tick mechanism, the sim step share and its parts, determinism and the state hash,
  the player record that holds the upgrade tracks, weapon drops and pickups as sim entities, the random stream kind for
  upgrade offers, bot behaviour, the ray and sight queries, and the Worker restart after a trap; from ADR 0002, the
  Worker, the per-tick output block's layout, the page-to-Worker posting, the generated types, the network simulator
  and the load harness; from ADR 0005, the content bundle and its hash, the mode file format, unit data, the ability
  data model and the upgrade tracks as data; from ADR 0006, the reference server, the VMs and their supervisor, the
  firewall, the lobby's hosting and store, the certificates, the staging game server and the monitoring; from ADR 0003,
  the client reference machines and everything drawn or played on the page.
- **Gives:** the tick rate rule and the 60 Hz margin; the server tick budget's total and its server-only share; the
  message protocol between the Worker and the server, and on the local server path; the command's fields, including
  the aim point; the snapshot fields the output block is filled from online; the visibility classes; the network
  profiles; rooms, bots and the lobby protocol; the room writing the match record when the match ends; the netcode
  proof test and its budgets.

### What the legacy game proves (docs/LEGACY_INVENTORY.md)
- CS1 (line 169, mark: drop): the input command (sequence, tick, buttons, move, aim) and a 1/60 s fixed step were
  declared but never wired; the v2 command and tick are this record's.
- CS7 (line 175, rewrite in Rust): `resolveShot` is a pure query that never changes the world, written so a server can
  run it against a rewound snapshot.
- GC5 (line 244, rewrite in Rust): shots leave from a fixed chest point, not the animated muzzle, so nothing animated
  decides a hit; spread drew from the shared `Math.random`.
- GC2 (line 241, rewrite in Rust): slot id equals sim id, so a human can take over a bot's slot without a remap.
- GP2 (line 312, port to TypeScript): actions run at key-event time outside the step; in v2 they become press edges in
  the command.
- GC13 (line 252, rewrite in Rust): the throw preview and the real launch start from different points; one shared
  trajectory function fixes it.
- GN8 (line 273, port to TypeScript): bot-shot sound is culled beyond 90 m and visuals beyond 50 m, two radii.
- GC1 (line 240, rewrite in Rust) and STATUS's ADR-001 inputs (STATUS.md line 199): the server must repeat the team
  allow-list, and humans take over bot slots.
- GP13 (line 323, redesign): a variable step inside `requestAnimationFrame` that freezes the world for pause, shop and
  hit-stop cannot survive multiplayer.

## Decision
v2's multiplayer is server-authoritative: an authoritative room process per match, running the same Rust simulation
as the browser's Worker, receives only sequenced input commands and sends each client delta-compressed, quantised,
interest-filtered snapshots at 30 Hz over WebTransport datagrams, with a WebSocket fallback behind one thin transport
interface. The client predicts only its own player and corrects smoothly; it draws everyone else about 100 ms in the
past; hitscan shots are lag-compensated within a cap. From slice 1 the same messages run between the client and a local
server in the browser. The parts:

1. **Authority.** The match server is [the authority](../research/GLOSSARY.md#the-authority) online; in a local match it
   is the client's own Worker running the same build. Clients send only input commands and room control messages: the
   server never takes client state, a hit the client reports or an RTT the client reports. Every outcome (movement
   results, shots, damage, deaths, economy, objectives, upgrade offers and picks, drops and pickups) comes from the
   authority's sim. *Why:* one source of truth, and every client message can be checked. *Research:* R2-SYNC-REC1,
   R2-SYNC-REC3, R1-DET-REC1 (lockstep rejected), R2-SYNC option 3.5 C rejected (client-side hit detection is an
   authority leak: r2-netcode.md, Options, Brief question 1, lines 1647–1653). *Need:* VISION_V2 §6.1, §6.5 ("all
   server-side").
2. **The tick rate rule (TICK-30).** The server ticks at 30 Hz ([the tick period](../research/GLOSSARY.md#tick-period)
   33.3 ms). It moves to 60 Hz only if the netcode proof test shows all of this, in one run on
   [one vCPU](../research/GLOSSARY.md#the-measuring-unit-one-vcpu) of
   [the reference server](../research/GLOSSARY.md#the-reference-server), in
   [the stress scene](../research/GLOSSARY.md#the-stress-scene) at N = 48 with a snapshot-receiving client on every
   soldier, over R2-BUD-REC6's 30-minute soak, re-flood ticks included: a p99
   [server tick time](../research/GLOSSARY.md#server-tick-time) of at most 6.25 ms against the server tick budget at
   60 Hz, 8.33 ms (a 60 Hz margin of at least 25 %: 1 − 6.25 ÷ 8.33 = 25 %), and every 60 Hz mark the research sets, as
   written (R2-BUD-REC6's two shares, 4.2 ms each; R1-TICK-REC2 P1–P3 at 60 Hz). Otherwise the tick stays at 30 Hz.
   Snapshots go out on every tick (30 Hz); inputs are sampled and sent every tick. 60 or 64 Hz is asked only if the
   margin passes (R1-TICK-OQ1). *Why:* at N = 48 the 60 Hz marks leave almost no margin (finding 3), while every legacy
   weapon still fits one shot per tick at 30 Hz and the replay work per correction halves (r1-simulation-core.md,
   Recommendations, unit tick, R1-TICK-REC2, lines 2029–2064). *Research:* R1-TICK-REC2 (changed), R2-BUD-REC1
   (changed), R1-TICK-OQ1. *Need:* VISION_V2 §6.2 and §6.4 (movement, jumps and big vehicles step with the tick) and
   §4 (fair up to about 100 ms of ping). This part replaces four research lines (the research files do not change in
   this PR):
   - R1-TICK-REC2's rule (r1-simulation-core.md:2029, its heading, and the decision rule at lines 2061–2062, "60 Hz if
     P1–P3 pass at 60 Hz; otherwise 30 Hz if they pass at 30 Hz … otherwise back to the drafts") now reads: "30 Hz is
     the baseline; 60 Hz only with a 60 Hz margin of at least 25 % and every 60 Hz mark as written; P1–P3 stay marks:
     at 30 Hz for the baseline, at 60 Hz for the 60 Hz margin run."
   - R2-BUD-REC1's tick half (r2-netcode.md:2302, "simulation tick 60 Hz if R1's tick spike meets its CPU mark, else
     30 Hz") is replaced the same way; its snapshot half (30 Hz) stands.
   - "… and the rule does not change" (r2-netcode.md:2415–2416, R2-BUD-REC6) now reads: "The mark is checked at
     30 Hz, the baseline, and at 60 Hz in the 60 Hz margin run; a failure at 30 Hz goes back to the drafts, a failure
     at 60 Hz keeps the tick at 30 Hz."
   - "… which does not change" (GLOSSARY.md:85, the Tick period entry) is replaced by GLOSSARY §8's closing note: "30 Hz
     is the baseline, 60 Hz needs a 60 Hz margin of at least 25 %."
3. **The server tick budget at 30 Hz.** p99 server tick time at most 16.7 ms (50 % of the tick period), max under one
   tick period (33.3 ms), zero overruns, split into
   [the sim step share and the server-only share](../research/GLOSSARY.md#the-sim-step-share-and-the-server-only-share),
   at most 8.3 ms p99 each, measured separately. The sim step share and its parts are ADR 0004's; this record adds that
   share into the total and never restates its parts. Inside the server-only share (ceilings, a planning check, not a
   bound: [time statistics](../research/GLOSSARY.md#time-statistics)): snapshot encoding at most 50 µs per client per
   snapshot (48 × 50 µs = 2.40 ms); a rewound shot at most 20 µs at 700 shots per second (700 ÷ 30 × 20 µs = 0.47 ms per
   tick); the region query at most 0.2 ms and ranking and packing at most 0.5 ms (0.70 ms); the enclosure pair test at
   most 0.1 µs per enemy pair (768 pairs at 16v16v16: 0.08 ms); on a tick with a portal change a re-flood of at most
   1 ms; command handling and the tick's non-waiting send calls share what is left (each send call at most 1 ms). The
   parts sum to 3.64 of 8.33 ms (4.64 on a re-flood tick), and the whole server tick to 9.18 of 16.67 ms (10.18)
   (GLOSSARY.md, §1, lines 53–69). The p99 is always measured directly, in one run. The
   [transport runtime](../research/GLOSSARY.md#transport-runtime-time) runs beside the tick on other threads, at most
   1 vCPU per process with 48 sessions; it counts in rooms per VM, not in server tick time. R2-BUD-REC6's reason
   (r2-netcode.md:2402–2403) is restated: "the topics' own part marks fit inside the server tick budget's 50 % of the
   tick period at 30 Hz, re-flood ticks included, and at 60 Hz on every tick but one that both encodes a snapshot and
   re-floods after a portal change (at N = 54, MODES-3 (b)); at N = 48 that tick fits by 0.05 ms (8.28 of 8.33 ms)
   while its server-only share is 0.24 ms over its own 25 % (4.41 of 4.17 ms) (GLOSSARY.md:62–75)." *Why:* one
   accounting for the whole server tick, sized at the baseline rate. *Research:* R2-BUD-REC6 (changed), R2-SYNC-REC9,
   R2-XU-OQ8 (settled by the glossary), R2-TRN-REC1 mark 2. *Need:* VISION_V2 §6.1 (rooms at 48 players).
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
   until ABL-PREDICT is answered (interim: an ability press waits for the server's confirmation; ADR 0005's ability
   system carries both answers). SEC-SPREAD (a) applies: weapon spread is known only to the server (in a local match,
   the Worker's authority sim), drawn from an
   [authority-only stream](../research/GLOSSARY.md#shared-stream-and-authority-only-stream) (ADR 0004); the client draws
   its tracers from a similar-looking cosmetic spread, and the hit event corrects them. *Why:* no input lag for what the
   player controls, no authority given to the client, and no spread a cheat can cancel. *Research:* R2-SYNC-REC3,
   R1-ABL-REC4, R1-TICK-REC2 P3, R1-ECS-OQ3, R1-DET-OQ6, R1-XU-OQ2, R2-XT-OQ7, R2-SYNC-OQ11, SEC-SPREAD, ABL-PREDICT
   (open). *Need:* VISION_V2 §4; LOOP_V2 §7 (split feedback); §6.4 (the vehicle the player drives); §6.3 (abilities
   wait for ABL-PREDICT).
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
   tick-stamped gameplay events (hits, kills, deaths, purchases, captures, tier changes, pick results, drops) ride in the
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
   32-bit server tick, other ticks are 16-bit offsets from it: a 16-bit tick wraps after 36.4 minutes at 30 Hz (65,536
   ÷ 30 s) and 18.2 minutes at 60 Hz, inside a 30-minute soak (proposed; +2 B on the estimate's 8 B header, about
   +0.5 kbps at 30 Hz). *Not the block:* the wire format is per client, delta-coded and bit-packed, so the Worker
   decodes it into its own state and interpolation buffer and then writes the block in ADR 0002's layout. *Divergence:*
   no state hash is exchanged with clients (a client holds only its interest set, and a compared
   [state hash](../research/GLOSSARY.md#state-hash) must leave authority-only streams out); the Worker counts
   prediction mismatches and correction sizes from part 4's replay check, which the proof test reads; whether they are
   reported from players' machines is client field data (ADR 0003 measures, ADR 0006 stores). *Why:* loss-tolerant
   deltas near half a datagram, bounded bursts, no fragment loss. *Research:* R2-SYNC-REC1, R2-BUD-REC2, R2-BUD-REC3,
   R2-SYNC-REC6, R2-SYNC-REC7, R2-SYNC-OQ4, R2-BUD-OQ8, R1-XT-OQ4 = R2-XT-OQ3, R1-XT-OQ9 = R2-XT-OQ8 = R3-XT-OQ2,
   R1-XT-OQ11 = R2-XT-OQ11, R1-XT-OQ6 = R2-XT-OQ6, R1-BND-OQ8, R1-DET-OQ9, R2-XU-OQ5, R2-XU-OQ6, R2-XU-OQ15. *Need:*
   VISION_V2 §6.1, §6.4, §6.5, §6.8 (events stamped with ticks).
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
9. **Interest management and visibility classes.** (a) A relevance region per client: a yaw-proof circle derived from
   the camera's worst-case view held as data per mode (height, boom, field of view, the largest allowed aspect, the
   largest client-side camera offset, the maximum height of interest) plus a motion margin, with enter and exit
   hysteresis; candidates come from a uniform grid. (b) Enclosure hiding: each building interior and each of its floors
   is an area, each door, window, stairwell and breakable wall a portal; connectivity is flood-numbered, compared in
   constant time and re-flooded when a portal opens or closes; an enemy in an area not connected to the viewer's is not
   sent, unless the visibility rule lets the camera see in. The area ids come from map data shared with navigation
   (ADR 0004; ADR 0005's map document). (c) One shared visibility pass serves the bots, and hides by line of sight only
   if the game rule says the character's sight limits what the player sees (R2-INT-OQ1, a design question for Mustafa).
   (d) A per-client priority accumulator packs entities under the per-client cap (part 6): the player's own state and
   vehicle always; the nearest 8 enemies in every snapshot; every relevant entity at least every 4 snapshots. (e) A
   visibility class on every field, event and message type, held as data: public, team, owner or spatial; which
   economy state is team-private is R2-INT-OQ3 (a design question). (f) Events have a radius per event type (data;
   sound larger than visuals) and pass the area test; shots at or hitting the viewer are always sent; an event whose
   shooter is not relevant to the viewer carries no shooter id, and its origin is clipped to where the shot enters the
   viewer's region; big units are relevant by their bounds plus a per-type visual radius; parts and seated players
   travel with their parent. A shooter can only hit what it was sent (part 8). The client's cutaway follows the same
   visibility rule (R2-XT-OQ15: how it looks is ADR 0003's). Spectators get no interest profile unless Mustafa wants
   them (R2-XU-OQ17); one would be one more profile, as R2-INT-REC7 designs it. *Why:* withholding data is the only
   anti-wallhack measure that holds in a browser; interest bounds the average, the accumulator the peak. *Research:*
   R2-INT-REC1 to R2-INT-REC6, R2-INT-REC7 (changed: not built), R1-ABL-OQ8, R2-INT-OQ5, R2-INT-OQ6, R2-INT-OQ7.
   *Need:* VISION_V2 §6.2 (floors, interiors, line of sight across heights), §6.5 (the economy's secrets), §6.1 (48
   players in one match), §6.4 (big units and their parts).
10. **Transport.** The server runs on quinn 0.11.x; the WebTransport layer is picked by the netcode proof test between
    wtransport 0.7.x and web-transport-quinn 0.13.x; h3-webtransport and a hand-written handshake are ruled out. The
    fallback is binary WebSocket through tokio-tungstenite 0.30.x: the room process serves no HTTP pages, so R2's
    alternative, axum's `ws`, brings nothing. One transport per client, never both at once; `requireUnreliable: true`
    where supported; WebTransport is tried first with a timeout (proposed 1.5 s), then WebSocket; no WebTransport over
    HTTP/2. Over WebSocket the server sends only the newest snapshot and drops any unsent older one. Inputs, snapshots
    and pings travel as datagrams (binary messages on the fallback); one bidirectional control stream per session
    carries room control; the first full state goes on its own unidirectional stream. A thin transport interface on
    both sides (send unreliable, send reliable on a named channel, receive, link state and RTT), outside the sim crate,
    has three backends: WebTransport, WebSocket, and a local loopback backend that carries the same encoded bytes
    between the client side and the local server in the browser (SLICE-ORDER condition 1). The network simulator's
    first layer plugs in behind the same interface (ADR 0002). Every decoder returns an error and never panics, and
    checks sizes before it allocates; the sim thread never parses raw network bytes. The protocol version rides in the
    `protocols` option where supported and in `Join` everywhere. Congestion: quinn's NewReno or Cubic, picked by
    measurement (BBR is out while quinn calls it experimental); the non-waiting `send_datagram` with a check of
    `datagram_send_buffer_space`; a short `outgoingMaxAge` on client input datagrams (proposed 100 ms); quinn's RTT and
    loss feed rate adaptation; under pressure the server drops low-priority entities first, then falls back to 20 Hz
    snapshots (two of every three ticks), never above the cap. Hostile-input limits, all data and all set explicitly:
    at most 2 client-opened streams each way, stream windows of about 64 KB, a connection window of about 128 KB, a
    datagram receive buffer of about 16 KB; an input datagram larger than the command size times the redundancy cap is
    dropped; per-connection rate limits; `Join` within 2 s (until then one stream, datagrams dropped); a modest cap on
    connections per IP address (above 1); new sessions rate-limited, with quinn's retry or refusal under load and
    `H3_EXCESSIVE_LOAD` for abusers; an Origin allow-list answered with 403 on both transports; `wss` only; explicit
    WebSocket frame, message and write-buffer limits. Version floors: quinn-proto ≥ 0.11.18, rustls ≥ 0.23.46, ring ≥
    0.17.12, tokio ≥ 1.54.1, wtransport's `dangerous-configuration` feature off, kept current by
    [the advisories check](../research/GLOSSARY.md#the-advisories-check) (its policy is
    [the security baseline](../research/GLOSSARY.md#the-security-baseline)'s). Certificates are ADR 0006's: in
    production [a certificate per hostname](../research/GLOSSARY.md#game-server-certificate) (R6-HOST-REC4;
    carry-forward 4); development and preview servers use self-signed, pinned certificates valid at most 14 days, whose
    hash the page fetches from its own HTTPS origin (R2-TRN-REC5). *Why:* datagrams suit newest-wins data; both crates
    sit on quinn and the interface keeps a swap cheap; WebSocket reaches networks without QUIC; the loopback backend
    runs the network's own messages from slice 1. *Research:* R2-TRN-REC1, R2-TRN-REC2, R2-TRN-REC3, R2-TRN-REC4
    (changed: a third backend), R2-TRN-REC6, R2-BUD-REC5, R2-TRN-OQ1, OQ2, OQ3, OQ5, OQ6, OQ7, OQ8, OQ9, R2-XU-OQ10,
    R2-XU-OQ11. *Need:* VISION_V2 §6.1; §4 (four browsers); the loopback backend: SLICE-ORDER condition 1 and GLOSSARY
    §8's foundation only; congestion handling (R2-TRN-REC6, R2-BUD-REC5): VISION_V2 §4 only.
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
    looted weapons and how fast tracks level are Mustafa's (ADR 0005 lists them). *Why:* the tracks belong to the
    player, so they are owner data the server holds, and offers that are never predicted need no client copy of the
    random stream that draws them (its kind is ADR 0004's). *Research:* no research id concerns upgrades; the parts used
    are R2-INT-REC5's classes, R2-SYNC-REC2's validation and R2-SYNC-F17 (Quake III's predicted pickups, the
    alternative OQ-0001-10 weighs). *Need:* UPGRADE-SCOPE.
13. **The network profiles.** One named set for every network mark, latency as the round trip, every run also
    reordering: P0 = 0 ms, no jitter, 0 % loss; P1 = 50 ms, ±10 ms, 1 %; P2 = 100 ms, ±20 ms, 2 %; P3 = 150 ms,
    ±30 ms, 5 %; P4 = 200 ms, ±40 ms, 5 %; B1 = P2 plus one burst of 1 s at 100 % loss (r2-netcode.md, Recommendations,
    Brief question 1, lines 1987–1992). Every R2 mark is restated against one: the budgets unit's "100 ms RTT, ±10 ms
    jitter, 2 % loss" becomes P2; the interest unit's "0, 50, 100 and 200 ms latency with ±20 ms jitter and 0–5 % loss"
    becomes P0, P1, P2 and P4; the rooms unit's "0-200 ms simulated latency" with 0–5 % loss becomes P0 to P4. The
    netcode proof test's "100 ms of simulated latency" is read as P2, VISION_V2 §4's "about 100 ms of ping"; the Advisor
    confirms the reading (OQ-0001-3). The simulator that runs the profiles is ADR 0002's. *Research:* R2-XU-OQ12,
    R2-XT-OQ14 = R5-XT-OQ10. *Need:* VISION_V2 §4 only.
14. **The netcode proof test and its budgets.** The netcode proof test is judged against the budgets below
    ("Budgets for the netcode proof test"). It runs before the foundation (SLICE-ORDER condition 3), throwaway, in
    `spikes/` (SPIKE-SCOPE (a)), once Mustafa and the Advisor approve the drafts. It takes in the research spikes of
    R2-BUD-REC6 (the server tick budget in the stress scene with a snapshot-receiving client on every soldier),
    R2-SYNC-REC9 (48 clients for 30 minutes), R2-TRN-REC1 and R2-TRN-REC3 (both transports: interop, load, hostile
    input), R2-ROOM-REC2 (a room holds its tick; memory per room) and R1-TICK-REC2's P1 and P3, with R2-SYNC-REC3's and
    R2-SYNC-REC5's marks for the one human client. *Need:* GLOSSARY §8 (the netcode proof test); VISION_V2 §6.1.
