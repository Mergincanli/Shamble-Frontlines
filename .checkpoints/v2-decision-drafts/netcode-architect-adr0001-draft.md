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

### Components and data flow
```
 Browser                                                     | Game server VM (ADR 0006: supervisor, firewall)
 page (TypeScript)        Worker (Rust sim crate as Wasm)    | room process (native Rust, one per match)
 ---------------------    -------------------------------    | ------------------------------------------------
 input, camera, draw  --> per-tick command + cursor ray      |  network I/O runtime (async, other threads):
 HUD, audio (ADR 0003)    client endpoint:                   |   WebTransport on quinn | WebSocket (tokio-
       ^                   - command builder + aim ray cast  |   tungstenite) -> decode, check sizes, bounded
       |                   - prediction + replay (part 4)    |   per-client queues; encode -> non-waiting send
       |                   - snapshot decode, ack            |  tick thread (30 Hz, ADR 0004's loop):
       |                   - interpolation buffer (part 7)   |   commands -> sim step (bots, weapons; hitscan via
       +---- per-tick  <-- - writes the output block         |   the lag-compensation query) -> hit history ->
             block         thin transport interface:         |   interest per client -> delta encode + events
             (ADR 0002)    [WebTransport | WebSocket |       |   -> hand datagrams to the I/O runtime; metrics
                            loopback]                        |  match record written at Ended (ADR 0006's shape)
                           local match only: the local       |
                           server (authority sim, room       | lobby (HTTPS; hosted by ADR 0006): guest id,
                           logic, bots) on the loopback side |  private-room create, code lookup, join token
```
- **The boundaries:** the page never handles a network packet and makes no Wasm calls; the Worker owns the connection
  (R1-BND-REC6, ADR 0002); the sim crate depends on no transport crate (R2-TRN-REC4 mark 3).
- **One server tick (30 Hz):** take the due ticks (ADR 0004); pick each client's command for this tick from its
  bounded queue (already decoded on the I/O side) and check it; run the sim step, in which bots produce commands
  through the same path and hitscan shots resolve through the lag-compensation query (counted in the server-only
  share); record the tick's hit volumes; for each client, query the region, run the enclosure test (re-flood on a
  portal change), rank and pack, delta-encode against its acknowledged snapshot and add its events; hand the datagrams
  to the I/O runtime without waiting; emit the counters (server tick time, per-phase time, bytes, overruns).
- **One client tick online (Worker):** decode the snapshots that arrived and acknowledge the newest; reset the local
  player to the server's state for its last processed command and replay the newer commands (skipped when they
  match); advance the interpolation clock; build this tick's command from the page's posted input, casting the aim ray
  against the state published for the page's view time; send the input datagram; write the block: the predicted local
  player, remote entities at the delayed time, the tick's events.
- **A local match (slices 1 and 2):** the same Worker build also runs the local server, the authority, with room logic
  and bots; the client endpoint reaches it through the loopback backend with the same encoded messages, and tests put
  the network simulator's profiles between them.

### State ownership
| State | Authority (online · local match) | Predicted on the client? | Sent to | Class |
|---|---|---|---|---|
| Soldier movement and position | room sim · Worker's local server | local player only (part 4) | everyone relevant | spatial; own state owner, full precision |
| Own weapon state (ammo, fire timers, reload, active reload) | room sim · local server | yes, local player | owner | owner |
| Weapon spread | room sim, authority-only stream · local server | never (cosmetic spread for tracers) | nobody | none |
| Hits, damage, deaths | room sim · local server | never | relevant clients; the shooter's hit confirmation | spatial; owner |
| Own ability activation, cost, cooldown | room sim · local server | not until ABL-PREDICT | owner; its effects to relevant clients | owner; spatial |
| Projectiles | room sim · local server | owner's cosmetic copy only | relevant clients, from the tick they become relevant | spatial |
| A piloted vehicle | room sim · local server | the pilot | relevant clients (parts with the parent) | spatial |
| Personal money | room sim · local server | never | owner | owner |
| Team economy and tech | room sim · local server | never | team, or public (R2-INT-OQ3, a design question) | team or public |
| Uplinks, countdown, steals, cache location | room sim · local server | never | everyone | public |
| Upgrade tracks, offers, picks | room sim (the player record, ADR 0004) · local server | never | owner | owner |
| Weapon drops and world pickups | room sim (sim entities, ADR 0004) · local server | not for now (OQ-0001-10) | relevant clients | spatial |
| Slot table, teams, squads, bots | room · local server | never | everyone (roster) | public |
| Room phase, host, settings | room (host commands checked) · local server | never | everyone in the room | public |
| Server tick and clock | room | client keeps an estimate | everyone (snapshot header) | public |
| Interpolation delay | client chooses, room clamps to the mode's range | n/a | room | n/a |
| View time and aim point | client stamps; room accepts within tolerance and rebuilds the ray | n/a | room | n/a |
| RTT used for lag compensation | room's own measurement | n/a | n/a | n/a |
| Camera and framing | page (ADR 0003); bounds held as mode data for interest | n/a | n/a | n/a |
| Animation, visual effects, sound, presentation randomness | page (ADR 0003) | n/a (cosmetic) | n/a | n/a |
| Guest identity and join tokens | lobby (signs); room verifies | n/a | the guest's own browser (cookie) | n/a |
| Content bundle and its hash | ADR 0005 builds; room names the hash at join | n/a | client host serves it (ADR 0006) | public |
| Match record | room writes it at Ended | n/a | storage (ADR 0006) | n/a |

### Message catalogue
Every message type has a visibility class; one with none fails the audit test (R2-INT-REC5). Sizes are the R2
estimate's where it has one; the rest are measured by the proof test (R2-XU-OQ15). The local server path carries the
same messages over the loopback backend.

| Message | Direction | Channel | Class | When | Contents | Server checks |
|---|---|---|---|---|---|---|
| `Input` | client → room | datagram | n/a | every tick (30 Hz) | newest snapshot decoded (ack, per entity group); every unacknowledged command up to the cap: tick, sequence, move, buttons, press counters, aim point, view time | part 5's list; size ≤ command size × cap; rate limit |
| `Snapshot` (one or more datagrams per tick) | room → client | datagram | per field | every tick (30 Hz) | header (32-bit server tick, baseline tick, last processed command, group index); own-state section (owner); spatial records; team and public sections; event section (gameplay events until acknowledged, cosmetic once) | n/a (the client drops what it cannot decode) |
| `Ping` / `Pong` | both ways | datagram | n/a | a few per second (proposed) | sender's time; on the answer, the receiver's tick | rate limit |
| `Join` | client → room | control stream, first message | n/a | once, within 2 s | join token, protocol version, content hash | token (signature first, audience, expiry, unique id), version, hash |
| `JoinAccepted` / `JoinRejected` | room → client | control stream | owner | once | slot, team, squad, mode id, mode hash, content hash, server tick, tick and snapshot rates / reason | n/a |
| `FullState` | room → client | unidirectional stream | per field | at join and on resync | the client's whole relevant world at one tick | n/a |
| `ClientSettings` | client → room | control stream | n/a | on change | announced interpolation delay | clamped to the mode's range, at most one interval per second |
| `RoomState` | room → client | control stream | public | on change | phase, countdown, roster (human or bot per slot) | n/a |
| `HostCommand` | client → room | control stream | n/a | private rooms, before start | start, kick, lock, settings (factions per team, allowed overrides) | sender is the host; values inside the mode's allowed overrides |
| `PickUpgrade` | client → room | control stream | n/a | when an offer is open | offer id, choice (one of three) | open offer to this player, choice in range, not already picked |
| `ContentReload` (development builds only) | room → client | control stream | public | on a valid content change | the changed set and the tick it applies from | only if ADR 0005 keeps development reload over the game connection (R1-DATA-REC5) |
| `RoomEnded` | room → client | control stream | public | at Ended | results | n/a |
| `Leave` / `Kick` | either way | control stream | owner | once | reason | n/a |
| Lobby calls (HTTPS) | browser → lobby | HTTPS | n/a | per visit or room | guest create (sets the cookie), private-room create, code lookup, token request | Origin check; rate limits per client, per address and global |
| Supervisor calls | supervisor ↔ room | local to the VM (ADR 0006) | n/a | per room | spawn with room id, mode id, overrides and the lobby's public key; health check; exit; record upload | ADR 0006 |

Upgrade offers, pick results, track changes, hit confirmations, kills, purchases and drops are not separate message
types: they are gameplay events or owner-section fields inside `Snapshot`, each with its class.

### Bandwidth estimate
- **Downstream per client (N = 48, everyone in view, E3, 30 Hz):** about 584 B per snapshot (609 B with events) and
  about 146.1 kbps; projectiles in every snapshot (E3b) about 188.3 kbps; the start modes' upper ends 44.2 kbps (4v4)
  and 70.3 kbps (6v6v6); 12v12v12 116.2 kbps; the N = 64 headroom run 183.7 kbps (r2-netcode.md, Options, Brief
  question 2, worked bandwidth estimate, lines 1764–1782). Interest management cuts the per-entity part and the events,
  not the overhead; on the Octopon-3 greybox the mark is a mean of at most 100 kbps (R2-BUD-REC2). This record adds the
  32-bit server tick (about +0.5 kbps, part 6) and action-start events for remote animation (not in the estimate; the
  proof test measures them, R2-XU-OQ15). IPv6 adds 20 B per datagram: +4.8 kbps at 30 Hz.
- **The per-client cap:** 256 kbps (proposed), about 1,067 B per snapshot interval at 30 Hz, counted like R2-BUD-REC2's
  mark (UDP payload plus 28 B per datagram); Fiedler's example budget is the same 256 kbps (r2-netcode.md, line 1789).
- **Upstream per client:** with the estimate's 8 B commands, newest plus 3 repeats, sending at 30 Hz costs about 22.3 kbps
  (option U1's 44.6 kbps at 60 sends per second, halved; r2-netcode.md, Options, Brief question 2, table C, lines
  1704–1710). The mark is at most 48 kbps (IPv4: R2-BUD-REC1), which at 30 sends per second leaves 200 B per datagram
  (48,000 ÷ 8 ÷ 30), 143 B after the 57 B of IPv4 transport overhead (r2-netcode.md, line 1742), for the header and
  the repeated commands with their aim points (at most 80 bits each) and view times. The real command size is
  measured (R2-XU-OQ4).
- **One room of 48 humans:** about 48 × 146 kbps ≈ 7.0 Mbit/s out and about 3.2 GB per match-hour (r2-netcode.md,
  line 1786); about 48 × 22.3 kbps ≈ 1.07 Mbit/s in (arithmetic). Bots cost no network traffic. R6 prices 3.16 GB per
  room-hour inside the 20 TB each VM includes (r6-infrastructure.md, R6-COST-REC1, lines 703–719; ADR 0006).
- **WebSocket fallback:** the same payload plus TCP, TLS and WebSocket framing; not estimated here, measured in the
  proof test.

### Budgets for the netcode proof test
The netcode proof test's "48 simulated clients run at a 30 Hz tick, with tick time, bandwidth per client and memory
all within the budgets that ADR 0001 sets" (GLOSSARY §8) is judged against these. Every server mark is measured on one
vCPU of the reference server (R6-HOST-REC2's CCX23; ADR 0006), in the stress scene at N = 48 with a snapshot-receiving
client on every soldier, over a 30-minute soak, re-flood ticks included, unless the row says otherwise.

| Budget | Value | Basis |
|---|---|---|
| Server tick time, 30 Hz (the baseline) | p99 ≤ 16.7 ms; max < 33.3 ms; 0 overruns (0 missed ticks) | R2-BUD-REC6 (r2-netcode.md:2398–2416) restated from GLOSSARY §1's 30 Hz column (GLOSSARY.md:53–69, 167–188); R1-TICK-REC2 P1 |
| Its two shares, 30 Hz | sim step time p99 ≤ 8.3 ms (ADR 0004's parts); server-only work p99 ≤ 8.3 ms, measured separately | R2-BUD-REC6; R2-SYNC-REC9 mark 1; GLOSSARY.md:189–213 |
| Parts of the server-only share | snapshot encode ≤ 50 µs per client per snapshot p99; rewound shot ≤ 20 µs p99 at 700 shots/s; region query ≤ 0.2 ms p99; ranking and packing ≤ 0.5 ms p99; pair test ≤ 0.1 µs per pair p99; re-flood ≤ 1 ms p99 (reported beside the whole, never summed into it) | R2-BUD-REC6; R2-SYNC-REC5 mark 4; R2-INT-REC1 (c); R2-INT-REC4; R2-INT-REC2 (b) |
| The 60 Hz margin run (decides 60 Hz) | p99 server tick time ≤ 6.25 ms against 8.33 ms; each share ≤ 4.2 ms p99; R1-TICK-REC2 P1–P3 at 60 Hz; one run, same setup | TICK-60-HEADROOM (the advisor's reading, confirmed at the stop); GLOSSARY §8, the 60 Hz margin |
| Prediction replay | 200 ms of game time (6 ticks at 30 Hz) ≤ 1 ms p99, minimum laptop | R1-TICK-REC2 P3 (r1-simulation-core.md:2058–2060) |
| Transport runtime | ≤ 1 vCPU per process with 48 sessions; p99 send call ≤ 1 ms | R2-TRN-REC1 mark 2 (r2-netcode.md:2565–2615), load restated at 30 Hz: 30 input datagrams per second per session |
| Downstream per client | mean ≤ 160 kbps and p99 of 1-second windows ≤ 256 kbps, N = 48, everyone in view, 30 Hz, counted at UDP payload + 28 B per datagram; hard cap 256 kbps (proposed); 0 datagrams above the cap rule | R2-BUD-REC2 (r2-netcode.md:2326–2344); R2-BUD-REC3 |
| Upstream per client | ≤ 48 kbps (IPv4) under P2 | R2-BUD-REC1 (r2-netcode.md:2302–2325), restated against P2 |
| Memory per room | resident memory reported at start and after 20 minutes, ≤ 64 MB at N = 48 (proposed); memory per session flat over the soak (no queue growth); hit history ≤ 1 MB | R2-ROOM-REC2 mark 3 (r2-netcode.md:2762–2782); R2-TRN-REC6 mark 1; R2-SYNC-REC5 mark 3 |
| Room start-up | process spawn to Open < 2 s | R2-ROOM-REC2 mark 5 |

### Failure modes
| Failure | What happens | Bound or mark |
|---|---|---|
| Loss, jitter, reordering | deltas against the last acknowledged snapshot; commands repeated until acknowledged; extrapolate one interval, then hold | P0–P4; R2-SYNC-REC1, REC2, REC4 marks |
| A burst of total loss (B1) | the client holds; on return the server deltas from an older baseline or the defaults | deltas again within 500 ms (R2-SYNC-REC1 mark 2) |
| No QUIC on the player's network | WebTransport times out, WebSocket takes over | playing ≤ 3 s after the page starts connecting (R2-TRN-REC3 mark 1) |
| A browser moves to a WebTransport draft the crate lacks | connections fail on that browser; WebSocket keeps it playable | the recurring test against beta browsers (R2-TRN-OQ1; ADR 0002) |
| Congestion (the window can fall to 2 datagrams) | low-priority entities first, then 20 Hz snapshots; the local player's own state never dropped | R2-BUD-REC5 mark; R2-TRN-REC6 marks |
| A client never acknowledges events | its event queue stops at the cap; resync or disconnect | R2-BUD-REC2's mark |
| Server tick overrun | catch-up bounded, dropped time recorded (ADR 0004's loop) | 0 overruns in the soak; counters (ADR 0006) |
| A panic in one connection's handler | that connection closes; the room's tick and other connections go on | R2-ROOM-REC2 mark 6 |
| A room process dies | that match ends; other rooms on the VM are unaffected | R2-ROOM-REC2 mark 4 |
| A trap in the client's Wasm sim | the Worker restarts (ADR 0004) and resyncs through `FullState` | the join mark below |
| A hidden tab or a sleeping laptop | online, the client does not simulate the gap; it resyncs | back in step within 0.5 s (R1-TICK-REC5, ADR 0004) |
| A player drops | slot held for the grace window; fresh token; newest connection wins | 100 % rejoin the same slot (R2-ROOM-REC4) |
| The lobby is down | no new joins; running rooms go on (tokens are checked offline) | none |
| Version or content mismatch | `JoinRejected` with the reason; the client reloads the bundle | R1-DATA-REC6's mark (ADR 0005) |
| A tick counter wraps | wire ticks are 32-bit in the header, 16-bit offsets elsewhere | the 60 Hz margin run's 30-minute soak crosses 65,536 ticks |
| Certificate expiry | ADR 0006's alert and renewal | ADR 0006 |
| No Hetzner capacity in the EU | the reference server cannot be ordered | R6-HOST-OQ1 (ADR 0006) |

### Anti-cheat implications
- Server authority: the client's sim online is a predicting copy, never the authority; no hit, state or RTT a client
  reports is trusted.
- Command checks and the lag-switch cap (part 5); forged commands are rejected and counted, with no effect beyond
  their valid part (R2-SYNC-REC2 mark 3).
- Lag compensation: the view-time tolerance of one snapshot interval, the final cap and the server's own RTT stop a
  client from buying extra rewind; view-time offsets and aims are logged for later detection (part 8).
- Spread lives in an authority-only stream (SEC-SPREAD (a)), so a cheat cannot cancel it through the aim point.
- Aim help, if adopted, is applied by the server; aim help shaped on a client cannot be told from an aimbot.
- Interest management withholds what a player may not see: enclosures, the region, the stripped shooter id and the
  re-based projectile records; a "reader" test client and a byte-level audit prove it (R2-INT-REC2, REC5).
- Upgrades and pickups: a pick the server did not offer, or a pickup it did not place, is refused (part 12).
- Hostile input: the transport limits, Origin checks, `wss` only, decoders that never panic, fuzzing (part 10).
- Admission: asymmetric join tokens checked before the replay cache, single use; room codes only locate; host
  commands checked against the host's id; the guest secret out of script's reach (part 11).
- Not decided here: cheat detection, the content security policy and key storage wait on the security baseline,
  listed open in ADR 0006 (BASELINE-WHEN (b)).

## Options considered
One table per question this record answers, with the template's dimensions as columns. Options and their facts are
the research's (r2-netcode.md, Options, lines 1606–1975, unless a row says otherwise); "chosen" marks the proposed
option. An option that contradicts a settled input appears as one line and is not weighed.

### Q1. The state-sync model (sync options 3.1, lines 1612–1620)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| A. Deterministic lockstep | high | lowest bandwidth | latency is the worst player's; stalls on a missing input; 48 browsers must agree bit for bit | every client holds the whole state: a wallhack sees everything | one slow player lags everyone | legacy is not deterministic (STATUS: 11 wall-clock reads) | least bandwidth | late join hard; one divergent client breaks the match; R1-DET-REC1 rejects it |
| B. Predict everything, roll back | high | unassessed | client cost grows with entities at 48+ | unassessed | unassessed | none | (no source read: the Overwatch talk is video only) | unassessed, so not proposable |
| C. State synchronisation | medium | low bandwidth | built for physics objects, not authority over many players | medium | divergence and pops | none | low bandwidth | "approximate and lossy" |
| D. Full snapshots, no delta | low | about 251 kbps at N = 48 (E2) | bandwidth grows with entities × rate | server authority | robust to loss | none | simplest | over budget at 48+ |
| **E. Delta against each client's acknowledged snapshot (chosen)** | medium | about 146 kbps at N = 48 (E3) | per-client encoding, bounded by interest | server is the only authority; no client determinism needed | tolerates loss and reordering | CS7's pure shot query fits a rewound server | shipped (Quake III), documented (Fiedler); late join from defaults | a spike when a baseline expires; per-client rings |

### Q2. What the client predicts (sync options 3.2, lines 1622–1629)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| A. Nothing | lowest | none | none | nothing to exploit | every input waits a round trip | none | simplest | fails VISION_V2 §4 and LOOP_V2 §7 "the shot feels instant" |
| B. Own movement only | low | small | flat per client | none | weapons lag a round trip | legacy has no prediction | Quake III, QuakeWorld | shots and reloads still lag |
| **C. Own movement, weapon state, vehicle, cosmetic shot feedback (chosen, with a cosmetic spread)** | medium | replay ≤ 1 ms per 200 ms | flat per client: only the local player and its vehicle replay | with a shared seed a cheat cancels spread, so the spread stays authority-only (SEC-SPREAD (a)) | serves LOOP_V2 §7's split feedback | weapon state already lives in the sim (GC4) | instant feel for what the player controls | tracers corrected by the hit event |
| D. Abilities and projectiles too | high | cosmetic rollback of effects | larger replays | more client state | better ability feel | none | best feel for abilities | no source read on ability rollback; ABL-PREDICT decides |

### Q3. Correction smoothing (sync options 3.3, lines 1631–1637)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| A. Snap | lowest | none | none | none | visible jumps | none | exact at once | pops on every correction |
| **B. Linear error decay over a set time, plus a snap threshold (chosen)** | low | a visual offset per frame | none | none | smooth; Quake III's 100 ms | none | shipped default | lags the truth for 100 ms |
| C. Exponential pull, snap above 2 m | low | none | none | none | smooth, long tail | none | simple | a slow tail on large errors |

### Q4. The interpolation delay and where its buffer lives (sync options 3.4, lines 1639–1645; R2-XT-OQ2, lines 3333–3339)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| A. Under one interval, then extrapolate (Quake III) | low | none | none | none | every late or lost snapshot extrapolates | none | smallest delay | poor smoothness at 2–5 % loss |
| B. A fixed number of intervals | low | none | none | none | steady | none | simple | no answer to changing jitter |
| **C. Adaptive, starting at 3 intervals (chosen)** | medium | a jitter estimate | none | the server clamps the announced delay | smooth at 2–5 % loss (Fiedler) | none | measured rule | 100–167 ms of delay |
| Buffer on the page (TypeScript) | medium | a snapshot buffer per frame on the page | page CPU | none | as C | GP13's loop is replaced anyway | close to drawing | the page would need the view time, the aim ray and sim maths the boundary forbids |
| **Buffer in the Worker (chosen)** | medium | delayed poses written into each block | page stays light | none | as C | none | view time, aim ray and command built in one place | the block carries delayed poses (ADR 0002's layout) |

### Q5. Lag compensation for hitscan (sync options 3.5, lines 1647–1653)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| A. None (Quake III) | lowest | none | none | none | a 100 ms shooter leads by about 1.8 m | none | simple | unfair for VISION_V2 §4 |
| **B. Server rewind to the view time, capped (chosen)** | medium | ≤ 20 µs per shot; ≤ 1 MB of history | grows with shots per second | the tolerance, cap and own RTT bound the abuse | the shooter's view is the truth up to the cap | `resolveShot` is a pure query built for this (CS7) | Source's shipped model | the victim's side unmeasured (R2-SYNC-OQ15) |
| C. Client-side hit detection | low | none | none | an authority leak, trivially cheatable | instant | none | instant | rejected |

### Q6. Projectiles (sync options 3.6, lines 1655–1664; encodings E3 and E3b, lines 1694–1700)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| A. Server only, shown when the spawn arrives | low | none | none | server authority | the owner sees the throw a round trip late | GC14, GC15's sim logic | simple | breaks LOOP_V2 §7's instant feel |
| **B. Server-simulated; the owner's cosmetic copy at once (chosen)** | medium | a hand-over blend | none | server decides every landing point | instant for the owner | GC13's shared trajectory idea | serves LOOP_V2 §7 | a match rule (owner, command sequence) and a hand-over |
| C. Client-authoritative | low | none | none | authority leak | instant | none | none | rejected |
| **E3: trajectories, flight on the client (chosen)** | medium | about 146 kbps at N = 48 | bytes only on change | landings are server events | a divergence shows only until the next change event | one trajectory function | cheapest | rests on bit-identical flight for display |
| E3b: projectiles in every snapshot | low | about 188 kbps | +42 kbps | same | always matches | none | no client flight needed | more bytes: the fallback if the proof test counts a desync |

### Q7. The aim in the command, and who casts the cursor ray (sync options 3.7, lines 1666–1671; R2-XT-OQ1, lines 3326–3332)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| A. Direction: yaw and pitch, 16 bits each | low | 32 bits | none | the server cannot check a clicked point | an origin error shifts the whole ray | legacy aims at the ground plane | compact | misses the clicked point by the origin error |
| **B. A quantised world aim point (chosen)** | medium | at most 80 bits | none | the server checks range and plausibility | keeps the clicked point at any height | GC3 already rays into the SIM | intent exact at height | more bits |
| The page casts the ray | medium | page-side ray code | none | none | as B | GC3's ray | near the camera | sim maths in TypeScript or Wasm calls from the page, both ruled out by R1-BND-REC1 |
| **The Worker casts it against what it published (chosen)** | medium | one ray per tick | none | none | as B | none | the same query and world as the server (R4-LOS-REC3) | the page posts its cursor ray and view time (ADR 0002) |

### Q8. The tick and snapshot rates (budgets option table A, lines 1675–1692)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| T1. 60 Hz tick, 30 Hz snapshots, 60 Hz input | medium | 2 × T2's step cost | 146 kbps down; the 60 Hz marks leave almost no margin at N = 48 | as T2 | finer input and fire timing (±8 ms) | legacy `FIXED_DT` is 1/60 | legacy parity | only with the 60 Hz margin (TICK-30) |
| **T2. 30 Hz tick, snapshots and input (chosen, TICK-30's baseline)** | low | lowest step cost | 146 kbps down; 9.18 of 16.67 ms | as T1 | +8 ms average input wait; fire intervals quantise harder (remainders carried) | none | the baseline TICK-30 sets | coarser timing (OQ-0001-7) |
| T3. 60 Hz tick and snapshots | medium | 2 × the encoding | 286 kbps, over the 256 kbps example | as T1 | shorter delay | none | interpolation 50–85 ms | over the bandwidth budget |
| T4. 20 Hz tick, snapshots 20 Hz | settled by TICK-30 (a tick below the 30 Hz baseline) | | | | | | | |

### Q9. Downstream encoding (budgets option table B, lines 1694–1700)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| E1. Raw floats, full state | trivial | about 536 kbps | two datagrams per snapshot | none | none | none | trivial | far over budget |
| E2. Quantised, full state | low | about 251 kbps | 1,373 B with projectiles: over the datagram | none | none | none | simple | over budget |
| **E3. E2 + delta + spawn-event projectiles (chosen)** | medium | about 146 kbps | about 609 B per snapshot | none | none | none | half a datagram | per-client rings, acks |
| E3b. E3 with projectiles every snapshot | medium | about 188 kbps | +42 kbps | none | none | none | no client flight | the fallback |
| E4. E3 + range coding | high | about 25 % below E3 (one data point) | CPU per client | none | none | none | fewer bytes | deferred until E3 is measured |

### Q10. Upstream (budgets option table C, lines 1704–1710)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **U1. Every tick, newest + repeats (chosen, at 30 Hz, repeating every unacknowledged command up to the cap)** | low | about 22.3 kbps at 30 Hz with 8 B commands | ≤ 48 kbps | the catch-up cap equals the redundancy cap | no extra input delay | CS1's command record | an input is lost only after several lost packets in a row | more bytes than U2 |
| U2. U1 with delta-coded repeats | low | about 36 kbps at 60 Hz | fewer bytes | as U1 | as U1 | none | cheaper | adopt once U1 is measured |
| U3. Send at 30 Hz with two new ticks | low | about 22.3 kbps | fewer sends | as U1 | adds up to one tick of delay at a 60 Hz tick | none | cheap | applies only to a 60 Hz tick |

### Q11. Packet size (budgets option table D, lines 1712–1719)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **P1. Datagrams under the cap; split by entity groups that decode alone (chosen for deltas)** | medium | per-group baselines | a lost datagram loses only its entities | none | none | none | no reassembly | per-group acks |
| P2. Application fragmentation | low | none | loss multiplies with fragments | none | stalls under loss | none | simplest encoder | fragile at 2–5 % loss |
| P3. Grow to a probed path MTU | medium | none | fewer splits | none | none | none | fewer datagrams | depends on each browser's `maxDatagramSize` |
| **P4. Join and resync over a reliable stream (chosen)** | low | a stream per transfer | none | none | a one-off wait at join | none | the large message never meets the limit | head-of-line wait at join only |

### Q12. Server CPU per tick (budgets option table E, lines 1721–1729)
Settled by the glossary's resolution 1 (GLOSSARY.md:167–188: the server tick budget is 50 % of the tick period with two
25 % shares; 25 % for the whole server tick was weighed and not taken, the advisor, 2026-10-30).

### Q13. The relevance region (interest Q-A, lines 1796–1803)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| A1. No region | none | highest bandwidth | serialisation grows with N² | every enemy position reaches every client | none | legacy sends nothing (single player) | none | fails §6.2's hiding |
| A2. A fixed circle | low | trivial | good | some | pop-in if the framing changes | none | simple | wrong once the framing or aspect changes |
| **A3. A circle from camera parameters as data, with hysteresis (chosen)** | medium | grid query + one test per candidate | good (about 10 times fewer bytes when spread out, none in a centre fight) | the circle also covers ground behind the camera (an accepted risk) | no pop-in if the client stays in the bound | CD5's framing values become the data | follows the data per mode | needs the client rig inside the server's bound |
| A4. A3 shifted toward the aim | medium+ | same | slightly better | smaller leak behind the camera | a quick flick may show an edge first | none | fewer bytes | a later optimisation only if the spike needs it |

### Q14. Hiding what a player must not see (interest Q-B, lines 1805–1812)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| B1. Region only | lowest | none | none | weak: occupants of buildings and hidden floors are sent | none | none | free | fails pillar 6 ("I sat in a building…") |
| **B2. Enclosures as areas with portals (chosen)** | medium | O(1) per pair plus a re-flood | ≤ 0.1 µs per pair, ≤ 1 ms per re-flood | strong for interiors and floors | low risk if portals are conservative | GW17's cutaway shows the same structures | Q3's shipped area test | needs area and portal data from the map |
| B3. Per-pair line of sight (fog of war) | high | highest | amortisable | strongest | pop-in at corners under latency | GN4's sight budget | strongest | only if R2-INT-OQ1's rule says the character's sight limits the view |
| B4. B2 plus B3 through portals and sight-blocking walls | high | between B2 and B3 | between | strong | as B3 at fewer places | none | targeted | its weight depends on R2-INT-OQ1 |

### Q15. Rate control inside the region (interest Q-C, lines 1814–1820)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| C1. Everything relevant, slow the whole stream when over | low | none | rate only | none | a crowded fight stales everyone at once | none | Quake III | no peak bound per snapshot |
| **C2. Per-client priority accumulator under a byte budget (chosen)** | medium | ≤ 0.5 ms per tick for 48 clients | a hard bound per snapshot | none | close threats stay fresh; far ones degrade, never starve | none | Fiedler | tuning per type |
| C3. Fixed tiers | low–medium | none | approximate | none | ignores threat | none | simple | no read source; can exceed the budget |

### Q16. The WebTransport server stack (transport Q1, lines 1837–1846)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **A. wtransport 0.7.x on quinn (a candidate)** | low | one crate | the spike's load mark | inherits quinn's advisories; `dangerous-configuration` off | none | new | one crate, certificate helper | "not considered completely production-ready" |
| **B. web-transport-quinn 0.13.x on quinn (a candidate)** | low–medium | crate family | the spike's load mark | inherits quinn's advisories | none | new | mirrors quinn's API | frequent 0.x releases |
| C. h3 + h3-webtransport | medium | none | unknown | not checked | none | new | hyperium's | "still very experimental", idle since 2025-05: ruled out |
| D. Our own handshake on quinn | high | upkeep on every draft change | ours | ours | none | new | full control | generic protocol work no §6 item asks for: ruled out |
The netcode proof test picks between A and B (R2-TRN-OQ5).

### Q17. The fallback when WebTransport fails (transport Q2, lines 1848–1855)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **A. Binary WebSocket (chosen)** | low | mature crates | head-of-line blocking; no unreliable channel | Origin check, `wss`, explicit limits | stalls under loss (the newest-snapshot rule bounds them) | none | everywhere, also in workers | worse under loss |
| C. qmux over WebSocket | medium | a young crate, draft -02 | still head-of-line blocked | as A | as A | none | one stream and datagram API on both | early draft; revisit only if needed (R2-TRN-OQ6) |
B (WebTransport over HTTP/2) and D (no fallback): settled by the stack (WebTransport with a WebSocket fallback, brief §1).

### Q18. One transport interface over both, and the local path (transport Q4, lines 1866–1875; SLICE-ORDER condition 1)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **A. Our own thin interface, both sides, with a loopback backend for the local server (chosen)** | low | four operations, three backends | none | bounded queues; decoders outside the sim thread | none | CS11's fence becomes a crate boundary | smallest, game-specific; the simulator's L1 plugs in behind it (R5-TEST-REC4) | written twice (Rust, TypeScript) |
| B. web-transport-trait plus qmux | medium | a fast-moving 0.x family | none | young code | none | none | real streams over WebSocket | ties us to an early draft |
| C. A WebTransport polyfill over WebSocket | unknown | unknown | unknown | unknown | unknown | none | none | not assessed (the lead was not read) |
A local server reached by direct calls instead of messages: settled by SLICE-ORDER condition 1 ("no offline-only shortcuts").

### Q19. Congestion control for a real-time stream (transport Q6, lines 1887–1896)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **A. Choose the server controller in quinn (chosen: NewReno or Cubic by measurement)** | low | none | measured | none | snapshot age under a bottleneck | none | pluggable | BBR experimental |
| **B. Drop stale data, never queue (chosen)** | low | none | no queue growth | none | fresh snapshots | none | the tick never blocks on a send | lost snapshots |
| **C. Adapt the rate (chosen)** | medium | none | degrades by priority, then rate | none | fewer updates under pressure | none | stays playable | tuning |
The three combine; they are not alternatives.

### Q20. Where a room runs (rooms 3.1, lines 1900–1906)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **A. One OS process per room (chosen)** | lowest | per-process memory unknown (measured) | density from processes per host | a crash or leak ends one room | a fair tick (no noisy neighbour inside) | legacy resets in place (GE12) | GameLift's and PlayFab's default | one port per room |
| B. Many rooms per process, a thread each | higher | a shared baseline | one port and certificate | an abort or out-of-memory takes all rooms | as A while healthy | none | cheaper for tiny rooms | the fallback only if A is too costly |
| C. Rooms as tasks on one async runtime | lower | cheapest | a long tick stalls other tasks | as B | tick jitter across rooms | none | none | ruled out |

### Q21. How a player is admitted (rooms 3.2, lines 1908–1919)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **A. A signed token checked offline by the room (chosen, asymmetric)** | medium | none | no lobby call on the join path | a compromised room cannot mint tokens (asymmetric); replay blocked by the used-id cache | fast joins | none | lobby off the hot path | keys to publish and rotate |
| B. netcode.io-style encrypted token | medium | none | same | every server holds a key that can mint tokens | same | none | hides contents | shared key risk |
| C. A ticket checked by a call to the lobby | low | none | a round trip per join | the lobby a live dependency | slower joins | none | simplest crypto | lobby on every join |

### Q22. Reconnecting (rooms 3.3, lines 1921–1931)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **A. Hold the slot, rejoin with a fresh token (chosen)** | low | none | none | one verification path | the player returns to their slot | slot id = sim id (GC2) | one join path | a held slot during the window |
| B. A room-issued resume secret | medium | none | saves a lobby call | a second credential to protect | as A | none | faster | more to secure |
| C. No hold | lowest | none | none | none | the slot goes to a bot or another player | none | simplest | splits the match record |

### Q23. Private matches and room codes (rooms 3.4, lines 1933–1943)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **A. A short code that only locates; joining needs a token (chosen)** | low | none | none | rate limits; the code is not the secret (about 30 bits) | easy to read out over voice | none | simple | brute-force bounded only by limits (OQ-0001-11) |
| B. A long invite link as the credential | low | none | none | unguessable, but URLs leak | poor over voice | none | strong | leaks through logs and history |
| C. Both | medium | two paths | none | as A and B | most convenient | none | flexible | two paths to build and test |

### Q24. Bots filling slots (rooms 3.5, lines 1945–1953)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **A. A per-team fill quota on the server (chosen)** | low | bots' CPU in the sim step share | teams always full | bots use the human input path | full matches at any human count | GC2's slots, STATUS's "humans take over bot slots" | meets "bots fill empty slots" | when a human replaces a bot is a game rule |
| B. A ratio of bots to humans | low | none | teams not kept full | none | uneven | none | none | fails VISION_V2 §3 |
| C. Fixed or no bots | lowest | none | none | none | empty slots | none | for a host who turns bots off | only if Mustafa allows it (R2-ROOM-OQ5) |

### Q25. Modes as data (rooms 3.6, lines 1955–1962)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **A. Validated mode files; a room is a mode id plus allowed overrides (chosen)** | low | none | none | unknown and prototype keys refused (SEC-MDL-2) | none | CD10's teams as data | VISION_V2 §3 and §6.1 | format work (ADR 0005) |
| B. Modes in Rust code | low | none | none | none | none | none | simple at first | every mode a code change |
| C. Per-map rules only | low | none | none | GW2's unchecked merge | none | legacy GW2 | none | cannot say "same map, 2 or 3 teams" |

### Q26. Guest identity (rooms 3.7, lines 1964–1971)
| Option | Complexity | Cost | Scalability and performance | Security and anti-cheat | Look and feel | Effort from the current code | Pros | Cons |
|---|---|---|---|---|---|---|---|---|
| **A. An anonymous guest account from the lobby (chosen, secret in an HttpOnly cookie)** | low | a lobby store (ADR 0006) | none | a secret of at least 64 bits no script can read | zero input before play | KVStore keeps settings today | reconnect and later linking | needs the lobby |
| B. No identity | lowest | none | none | none | none | none | simplest | no reconnect across a reload |
C (login required): contradicts VISION_V2 §5 [DECIDED] "guest play first".

## Trade-off analysis
- **Why this design wins.** Server authority with delta snapshots, local-only prediction, interpolation and a capped
  rewind is the model the research could verify in shipped code (Quake III, Source) and in Fiedler's measurements; it
  needs no bit-exact agreement between 48 browsers, joins mid-match from defaults, and makes every server cost scale
  with what each client sees rather than with the match (R2-SYNC-REC1, R2-SYNC-REC9). The alternatives either fail the
  fairness target (no prediction, no lag compensation), leak authority (client-side hits or projectiles), break the
  bandwidth budget (60 Hz snapshots, full snapshots), or rest on sources nobody could read (predict-everything with
  rollback).
- **Why 30 Hz.** At N = 48 the 60 Hz marks leave 0.05 ms on the worst tick and overrun the server-only share there
  (finding 3); 30 Hz leaves 7.49 ms of the 16.67 ms budget on the planning check (16.67 − 9.18; 6.49 on a re-flood
  tick), keeps one shot per tick for every legacy weapon, and halves the replay per correction. The 60 Hz path stays
  open through the 60 Hz margin rule.
- **The strongest arguments against.**
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
     Worker also encodes and decodes every message for the loopback path; that cost is not measured yet and counts in
     the client's sim share, ADR 0004's (the proof test reports it).
  6. *One process per room* may cost more memory than shared processes; option B is the fallback the proof test can
     trigger.

## Consequences
- **Easier:** one simulation path for local and online play, so every slice exercises the network's own messages;
  anti-cheat by construction (authority, withholding, checks); a WebTransport crate swap confined to one module; bots
  that fill slots through the human input path; costs that scale with what each client sees; tests that replay
  recorded inputs to the same state hash on both transports.
- **Harder:** every gameplay feature must be an input, an authoritative result and replicated state or events, each
  field with a visibility class; the local player's code must be replayable; the Worker carries the interpolation
  buffer, the aim ray and, in local matches, the loopback codec; testing needs the network simulator, soaks and real
  browsers; players on the WebSocket fallback see head-of-line stalls under loss.
- **Revisit when:** the netcode proof test reports (the tick rate, the WebTransport crate, the rewind cap's clipped
  share, E3 or E3b, the process model, the datagram cap); a browser moves to a new WebTransport draft; connection
  telemetry shows the fallback share (OQ-0001-13); Mustafa decides seats (§6.4 [OPEN]), spectators (R2-XU-OQ17) or the
  line-of-sight rule (R2-INT-OQ1); ABL-PREDICT is answered; the real sim's step times at slice 6 differ from the proof
  test's; the security baseline is researched (BASELINE-WHEN (b)).
- **Hard constraints in CLAUDE.md that change** (for v2 code only, under SPIKE-SCOPE (a); nothing in CLAUDE.md or
  `.claude/` changes in this PR):
  - "the game runs by double-clicking index.html (file://)": WebTransport works only in secure contexts (HTTPS)
    (r2-netcode.md, Findings, Brief question 4, R2-TRN-F35, lines 1267–1271), so the online client is served over
    HTTPS; how the v2 build is opened locally is ADR 0002's.
  - "Persistence only through `KVStore`": the guest secret lives in an HttpOnly cookie on the lobby's origin that no
    script can read (R2-ROOM-REC8); the guest id may stay in the KVStore successor.
  - "Animation is cosmetic: shots come from `playerFireOrigin()`" is kept and becomes a server rule (part 8).
  - The classic-script, three.js r128 and "no external assets" constraints are ADR 0002's, ADR 0003's and LOOK_V2 §8's
    (ADR 0005), not this record's.
- **Legacy inventory rows** (the inventory itself changes only once Mustafa accepts the drafts):

| Row | Mark now | Proposed mark | Why |
|---|---|---|---|
| GC3 (cursor aim and ADS sight line, line 242) | port to TypeScript; look brief pending; the height-aim rule pending LOOP_V2 §9 [OPEN] | split: the camera-to-cursor ray stays port to TypeScript; the aim ray query against the world is rewrite in Rust (the sim crate in the Worker); the sight line's appearance is ADR 0003's | part 5: the Worker casts the aim ray against the state it published (R2-SYNC-REC8, R2-XT-OQ1); aiming at height is served through VISION_V2 §6.2, aim help stays LOOP_V2 §9 [OPEN] |
| GN8 (bot-shot effects and sound cull, line 273) | port to TypeScript; look brief pending | split: the per-event-type radii (sound 90 m, visuals 50 m) become the server's event interest radii as data, rewrite in Rust; the client's cosmetic cull stays port to TypeScript; how the effects look is ADR 0003's | part 9 (f): events filtered by a radius per type on the server (R2-INT-REC6) |

## Open questions
- **OQ-0001-1. Is TICK-60-HEADROOM's reading confirmed?** The 60 Hz margin is measured against the server tick budget
  at 60 Hz (8.33 ms), so 60 Hz needs a p99 server tick time of at most 6.25 ms in part 2's setup, plus every 60 Hz mark
  the research sets. · *Why it matters:* it decides whether and how 60 Hz can ever be chosen. · *Options:* (a) confirm
  the advisor's reading as ruled on 2026-10-31; (b) another reading of TICK-30's "25 % headroom". · *Decider:* Mustafa
  and the Advisor, "confirm at the stop". · *What decides it:* their confirmation. · *Research ids:* R1-TICK-REC2,
  R2-BUD-REC6, R1-TICK-OQ1.
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
- **OQ-0001-3. Is "100 ms of simulated latency" P2?** The proof test's human client runs at P2: 100 ms round trip,
  ±20 ms jitter, 2 % loss (part 13). · *Why it matters:* it sets what "look correct" is judged at. · *Options:* (a) P2,
  the round trip, as VISION_V2 §4's "ping"; (b) 100 ms each way, a 200 ms round trip (P4's latency). · *Decider:* the
  Advisor, who wrote the definition. · *What decides it:* the Advisor's reading. · *Research ids:* R2-XU-OQ12,
  R2-XU-OQ2.
- **OQ-0001-4. Does the client predict the player's own abilities (ABL-PREDICT)?** · *Why it matters:* it sets how an
  ability press feels at 100 ms and how much the Worker replays. · *Options:* (yes) R1-ABL-REC4's prediction key (the
  command's sequence number), a rejected press rolled back, cues keyed so they play once; (no) a press waits for the
  server's confirmation (the interim). · *Decider:* Mustafa. · *What decides it:* a Feel Lab test that compares both
  ways on the Molotov and the Riot Shield, once abilities are playable. · *Research ids:* ABL-PREDICT, R2-SYNC-OQ10,
  R1-ABL-OQ4, R1-ABL-REC4.
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
  client, nudging its tick pace or re-aligning by whole ticks (not researched). · *Decider:* the netcode proof test. ·
  *What decides it:* correction rates and fairness under P2 to P4. · *Research ids:* R2-SYNC-OQ3, R1-TICK-OQ8,
  R1-XT-OQ5 = R2-XT-OQ5 (the time-sync part).
- **OQ-0001-7. Do shots carry their moment inside a tick at 30 Hz?** · *Why it matters:* a 33 ms tick quantises when a
  click happens. · *Options:* (a) no: the command's tick and the view time's fraction only; (b) a sub-tick fire offset
  in the command. Not researched (no source read). · *Decider:* the netcode proof test, then Mustafa in the Feel Lab if
  it fails or feels late. · *What decides it:* R2-SYNC-REC5 mark 1's registration at 30 Hz. · *Research ids:*
  R1-TICK-OQ4.
- **OQ-0001-8. How far does a new projectile move forward on spawn?** · *Why it matters:* throws landing where aimed at
  high ping, against projectiles appearing mid-flight for others. · *Options:* none; a fixed prestep (Quake III's
  50 ms); the shooter's latency up to the cap. · *Decider:* Mustafa. · *What decides it:* a Feel Lab test under P2 and
  P3 once projectiles are playable (slice 4). · *Research ids:* R2-SYNC-OQ6.
- **OQ-0001-9. What else does lag compensation cover?** Destructible statics and moving vehicles as cover (judged at
  present or rewound), and melee and short-range abilities (the Banzai blade, the bayonet, the Riot Shield block) at
  the attacker's view time. · *Why it matters:* at 9 m/s a target moves about 1.8 m in 200 ms, half the blade's reach.
  · *Options:* per item, present state or rewound. · *Decider:* the netcode proof test for cover; for melee, a design
  call by Mustafa with the game-designer. · *What decides it:* the rewind test extended with a destructible wall, a
  moving Juggernaut as cover, and melee. · *Research ids:* R2-SYNC-OQ8, R2-SYNC-OQ9.
- **OQ-0001-10. Are weapon pickups predicted?** · *Why it matters:* a pickup that waits for the server shows one round
  trip late. · *Options:* (a) not predicted (the interim); (b) Quake III's predicted pickups, the item hidden at once
  (R2-SYNC-F17), rolled back if the server refuses. · *Decider:* Mustafa. · *What decides it:* a Feel Lab test once
  drops exist, alongside ABL-PREDICT's. · *Research ids:* none owned (UPGRADE-SCOPE; R2-SYNC-F17).
- **OQ-0001-11. Room codes: length, alphabet and access.** · *Why it matters:* guests are free, so in practice the code
  is the access secret. · *Options:* 6 symbols from 32 (the proposal); 8 or more; tokens for a private room only on the
  host's approval; the host lock on by default for organized play. · *Decider:* Mustafa (what a host may set,
  R2-ROOM-OQ5) with the security review's options. · *What decides it:* Mustafa's call. · *Research ids:*
  R2-ROOM-OQ10.
- **OQ-0001-12. Do far sound events reveal too much?** · *Why it matters:* an exact position on a gunshot 80 m away is
  a radar for a cheat. · *Options:* a coarser origin at distance; direction only; an accepted leak. · *Decider:* Mustafa
  with the game-designer. · *What decides it:* their call, checked by the reader-client test. · *Research ids:*
  R2-INT-OQ6.
- **OQ-0001-13. What do real players' networks show?** How many need the WebSocket fallback, the IPv6 share, and how
  often an address change forces a reconnect. · *Why it matters:* how good the fallback must feel, the IPv6 overhead,
  how often reconnect runs (not its design). · *Options:* none (measurements). · *Decider:* connection telemetry from
  the first online playtest (ADR 0006's monitoring). · *What decides it:* the measured shares. · *Research ids:*
  R2-TRN-OQ7, R2-BUD-OQ11, R2-ROOM-OQ8.

## What a spike must prove
The spike is the netcode proof test (GLOSSARY §8; brief §2), in the Advisor's words:

> - **The netcode proof test.** A throwaway spike that proves the online path:
>   - the Rust simulation runs natively as a server, and as Wasm in the client;
>   - they connect over WebTransport, with the WebSocket fallback;
>   - 48 simulated clients run at a 30 Hz tick, with tick time, bandwidth per client and memory all within the budgets that
>     ADR 0001 sets;
>   - one human client at 100 ms of simulated latency sees prediction, reconciliation and lag-compensated hits that look
>     correct.

It runs before the foundation (SLICE-ORDER condition 3), throwaway, in `spikes/` (SPIKE-SCOPE (a)), only after
Mustafa and the Advisor approve the drafts. Its sim is a throwaway one with movement, hitscan and bots as protocol
clients, native and Wasm. It takes in the research spikes named in part 14. What it must prove, by decision part, with
candidate marks from the research (all proposed there; restated at 30 Hz and against part 13's profiles):

1. **The online path runs end to end on four browsers (parts 1 and 10).**
   - Each WebTransport candidate crate: Chrome, Edge, Firefox and Safari each succeed in at least 99 of 100 connection
     attempts on a clean local network, the negotiated draft recorded (R2-TRN-REC1 mark 1); switching crates changes no
     code outside the transport module (mark 3); the lockfile meets the version floors (mark 5).
   - With UDP blocked, each browser reaches a playing WebSocket session within 3 s (R2-TRN-REC3 mark 1); one recorded
     input replay drives both transports to identical server state hashes on every tick (mark 2); over WebSocket, the
     p99 extra snapshot delay is reported and at most 250 ms at P2 (restated from "100 ms RTT and 1 % loss"), and the
     server never holds more than one unsent snapshot per client (mark 3).
   - One client build runs unchanged on the four browsers over both backends; app-level RTT agrees with quinn's `rtt()`
     within ±5 ms at P2; the sim crate depends on no transport crate (`cargo tree`); 10 million random and mutated
     inputs per message type give 0 panics and 0 allocations above the size limits (R2-TRN-REC4 marks 1–4).
   - `maxDatagramSize` recorded at session start and every 10 s on the four browsers, at least 1,000 B on loopback and on
     a real EU path, otherwise the cap drops to the smallest value measured; lost-or-late snapshots at most the injected
     loss plus 1 percentage point at P2; 0 of 10,000 one-shot actions lost at P3; a 1 MB bulk transfer adds at most 5 ms
     to the control stream's p99 (R2-TRN-REC2 marks 1–4).
   - Development certificates (ADR 0006's): all four browsers connect with a pinned self-signed certificate in 20 of 20
     attempts, and reject one valid for 15 days (R2-TRN-REC5 marks 1 and 3).
2. **48 simulated clients stay within the budgets (parts 2, 3, 6, 11, 14).** The "Budgets for the netcode proof test"
   table above, plus: every decoded snapshot rebuilds the server's quantised state for that client by hash under P0,
   P2, P3 and B1, deltas again within 500 ms after B1, no datagram above the payload cap (R2-SYNC-REC1 marks 1–4); the
   server's sync cost growing no faster than linearly in players × visible entities at 24, 36 and 48 (R2-SYNC-REC9
   mark 2); a client joining mid-match has a complete world within 300 ms at P2 (R2-BUD-REC3, the one join mark,
   R2-XU-OQ6); at least 99.9 % of commands reach the server before their tick at P3, 0 lost and 0 doubled of 10,000
   presses, every forged command rejected and counted, a lag-switch client held to the catch-up cap (R2-SYNC-REC2 marks
   1–4); missing inputs at most 0.1 % of player-ticks at P3 (R2-BUD-REC1); a never-acknowledging client capped and
   resynced or dropped (R2-BUD-REC2); with the link throttled to 128 kbps for 10 s, at most 2 missing snapshots per
   second, the own state never deprioritised, the full rate back within 2 s (R2-BUD-REC5); with a bottleneck at 50 % of
   the snapshot rate, the newest snapshot never older than 3 intervals and memory per session flat, server tick p99
   within 5 % of a clean run, NewReno against Cubic recorded (R2-TRN-REC6 marks 1–3); a flood (1,000 sessions that never
   join, 10 clients sending oversize datagrams and opening streams) leaves server tick p99 within 10 % of a clean run
   and memory under the ceiling, and a wrong origin gets 403 on all four browsers (R2-TRN-REC1 mark 4); a panic in one
   connection's handler closes only that connection (R2-ROOM-REC2 mark 6). For rooms per VM: 8 rooms as 8 processes on
   one reference server, each room's p99 within 10 % of one room, and a killed room leaves the others unchanged
   (R2-ROOM-REC2 marks 2 and 4, on the CCX23 and the CX33: R6-HOST-REC2, ADR 0006).
3. **The 60 Hz margin (part 2):** in one run with part 2's setup, p99 server tick time at most 6.25 ms at a 60 Hz tick,
   both shares within 4.2 ms, and R1-TICK-REC2 P1–P3 at 60 Hz; how the sim load gets there is OQ-0001-2. A failure
   keeps 30 Hz.
4. **The human client at P2 sees prediction, reconciliation and lag-compensated hits that look correct (parts 4, 7,
   8).** Mustafa judges "look correct" in Brave (test-engineer writes his checklist); the measurable marks beside it:
   - corrections: 0 larger than 1 mm whenever every command arrived in time, P0 to P4; in a brawl of 48 at P3,
     corrections larger than 5 cm in fewer than 1 % of snapshots and never more than 5 cm in one frame below the snap
     threshold; input-to-screen latency for own movement and muzzle flash the same at P0 and P4 within one tick
     (R2-SYNC-REC3 marks 1–3); the 6-tick replay within 1 ms p99 (R1-TICK-REC2 P3);
   - interpolation: extrapolation in fewer than 1 % of rendered frames at P2 and 3 % at P3, never beyond one interval
     under B1, the adaptive delay settled within 5 s of a profile change, 0 visible slides in 1,000 scripted respawns
     (R2-SYNC-REC4 marks 1–4, the one mark R2-XU-OQ3 asks for); clock offset error at most 5 ms after 2 s of samples,
     against the simulator's true clock (R2-BUD-REC4);
   - hits: shots whose ray hits the target in the shooter's view register on the server in at least 99 % of cases up to
     P2 and 95 % at P3, false hits below 0.5 %, against a strafing, sprinting target at 9 m/s (R2-SYNC-REC5 mark 1); no
     shot rewound beyond the cap, and the share each candidate cap clips recorded (mark 6, OQ-0001-5); a cheat client at
     the edge of the tolerance gains no more rewind than the tolerance (mark 5); history at most 1 MB and a rewound shot
     at most 20 µs p99 (marks 3 and 4); server hit points within 5 cm of the client-picked point in at least 99.9 % of
     shots at the ground, 2.5 m and 4 m, P0 to P3, and the aim at most 80 bits per command (R2-SYNC-REC8 marks 1–2);
   - feedback: no damage number or kill on another soldier shown before the server's confirmation (R1-ABL-REC4).
5. **What the proof test decides:** the WebTransport crate (R2-TRN-OQ5); the datagram cap from the measured
   `maxDatagramSize` (R2-BUD-OQ3 = R2-TRN-OQ3, R2-XU-OQ5); E3 or E3b (0 desyncs of client-side flight over 1,000 throws,
   R2-BUD-REC2; only if projectiles are in the load); the process model (R2-ROOM-OQ1); the encoding cost per client
   (R2-BUD-OQ9); the controllers' behaviour under 2–5 % loss, server and browser side (R2-BUD-OQ10, R2-TRN-OQ8); each
   browser's negotiated draft and its behaviour with UDP blocked (R2-TRN-OQ1, R2-TRN-OQ2); the rewind estimate, by
   measuring how far behind the server's present the client's view is when its command arrives (R2-XU-OQ2); the real
   record and command sizes (R2-XU-OQ15, R2-XU-OQ4); linear against Hermite (R2-SYNC-OQ12); command timing
   (OQ-0001-6); whether shots need sub-tick stamps (OQ-0001-7). Reported, not judged: the egress per 48-player
   match-hour and the spike's own bill (R6-COST-REC1, ADR 0006).

### Pass marks
Written by test-engineer (plan step 4).

## Migration plan
In the slice order (SLICE-ORDER). The legacy game stays playable, frozen, until v2 replaces it (it moves to `legacy/`
in the foundation).
1. **The netcode proof test** (before the foundation): run after the drafts are approved, beside the other phase 0
   spikes `docs/SPIKE_PLAN.md` orders. *Done when:* the pass marks are met and recorded in this record, with the tick
   rate (30 Hz unless the 60 Hz margin passes), the WebTransport crate, E3 or E3b, the process model and the datagram
   cap named; Mustafa's Brave checklist passed.
2. **The foundation** (ADR 0002 carries it): the protocol crate (message types, the codec, the thin transport interface
   with its loopback backend) goes into the client + local server skeleton, its types generated for TypeScript. *Done
   when:* an empty scene runs through the full stack in the browser with CI green and a preview link (GLOSSARY §8), and
   the Worker's client side and local server exchange `Join`, `JoinAccepted`, `Input` and `Snapshot` over the loopback
   backend, with the network simulator's L1 behind it in CI.
3. **Slice 1, move & shoot:** movement and shooting are driven by input commands from day one and replayable
   (SLICE-ORDER condition 2); the local server is the authority; prediction and reconciliation of own movement and
   weapon state run against it through the loopback. *Done when:* a recorded input replay gives the same per-tick state
   hash on two runs, and R2-SYNC-REC2 marks 1–2 and R2-SYNC-REC3 marks 1–3 pass on the loopback under L1's profiles.
4. **Slice 2, bots and three teams, played locally:** bots on the local server through the human input path, the slot
   table and teams built from mode data (the format is ADR 0005's). *Done when:* R2-ROOM-REC6 marks 1–3 and R2-ROOM-REC7
   marks 1–2 pass on the local server.
5. **Slice 3, online:** the room process on the staging game server (ADR 0006), the lobby (guests, tokens, private
   rooms), both transports, interest management, lag compensation and reconnect; the first multiplayer preview behind
   the access wall (PREVIEW-WALL (a)); the security baseline researched before the first online playtest
   (BASELINE-WHEN (b)). *Done when:* R2-ROOM-REC1, REC3, REC4, REC5 and REC8's marks, the reader client (R2-INT-REC2)
   and the byte-level audit (R2-INT-REC5) pass, and Mustafa's first online playtest is run.
6. **Slice 4, the loop:** economy and objectives with their visibility classes (team-private state as Mustafa decides,
   R2-INT-OQ3), the uplink countdown never predicted, the match record written at Ended, upgrade offers, picks, drops
   and pickups (part 12), and the Feel Lab tests (OQ-0001-4, -5, -8, -10). *Done when:* the refusal tests of part 12
   and R2-ROOM-REC1 mark 3 pass, and Mustafa has answered the Feel Lab questions.
7. **Slice 5, Octopon-3 and Editor v2:** enclosure areas and portals from the map document (ADR 0004, ADR 0005); the
   editor's Test plays the map with bots through the local server (EDITOR_V2 §3). *Done when:* the downstream mean is at
   most 100 kbps with interest on the Octopon-3 greybox (R2-BUD-REC2) and R2-INT-REC1 (a) and R2-INT-REC2 (c) show 0
   pop-in frames there.
8. **Slice 6, scale to 48:** the budgets again with the real sim in the stress scene, 8 rooms per VM (with ADR 0006),
   the soaks on the test-mode list, and the 60 Hz margin run here if OQ-0001-2 chose (c). *Done when:* the budgets
   table passes with the real sim and R5-TEST-REC5's soak passes for every mode on
   [the test-mode list](../research/GLOSSARY.md#the-test-mode-list).

## Test plan
- **Network simulation:** L1, our link simulator behind the thin transport interface (seeded, on simulated time, in the
  gate), and L2, an impairment wrapper under quinn's socket for real browsers, with a delaying TCP proxy for WebSocket
  (R5-TEST-REC4, ADR 0002), running part 13's profiles P0 to P4 and B1, each with reordering.
- **Determinism by state hash:** recorded input replays give identical per-tick server state hashes on both transports
  (R2-TRN-REC3 mark 2) and on the loopback; the same seed and join-and-leave script give the same hash twice
  (R2-ROOM-REC6 mark 3); decoded snapshots rebuild the server's quantised state by hash (R2-SYNC-REC1 mark 1).
- **Load and soak:** bots as headless protocol clients, 48 per room, 30 minutes at P2 for every mode on the test-mode
  list, with server tick time, bytes per client, memory slope, queue lengths, desyncs, panics and hangs measured, and
  one 3-hour soak before each playtest (R5-TEST-REC5, ADR 0002).
- **Hostile input:** the forged-command and lag-switch suites (R2-SYNC-REC2 marks 3–4), the flood test on both
  transports (R2-TRN-REC1 mark 4, R2-TRN-REC3 mark 4), bad join tokens (R2-ROOM-REC3 mark 1), decoder fuzzing
  (R2-TRN-REC4 mark 4), forged upgrade picks and pickups refused and counted with no effect (part 12; proposed, the same
  harness as R2-SYNC-REC2 mark 3).
- **Anti-wallhack:** the reader client that logs everything it receives (R2-INT-REC2 (a), R2-SYNC-REC6 mark 4) and the
  byte-level audit of every message against its class (R2-INT-REC5).
- **Browsers:** one client build on Chrome, Edge, Firefox and Safari over both backends (R2-TRN-REC4 mark 1), and a
  recurring run against their beta channels (R2-TRN-OQ1; ADR 0002's CI).
- **Playtests:** Mustafa's Brave checklist for the human client at P2 (test-engineer), then online playtests from
  slice 3.

## Rollback
- **The 30 Hz baseline fails at N = 48:** back to the drafts with the measured numbers (part 2's restated rule).
- **The 60 Hz margin fails:** nothing to undo; the tick stays at 30 Hz.
- **One WebTransport crate fails:** the other one, behind the thin interface (a swap touches only the transport module,
  R2-TRN-REC1 mark 3). **Both fail:** back to the drafts; the transport is a settled input, so Mustafa and the Advisor
  decide.
- **Client-side projectile flight desyncs:** E3b, about +42 kbps per client.
- **One process per room is too heavy:** many rooms per process (option B), with `panic = "unwind"` and per-room
  containment (R2-ROOM-REC2).
- **Corrections, interpolation or hit registration miss their marks:** the delays, the cap, the tolerance and the
  smoothing are data, so a first retry needs no code change; otherwise back to the drafts with the numbers.
- **After acceptance:** the protocol carries its version in `protocols` and in `Join`, so a message change is a
  version bump; the frozen legacy game stays the playable build until v2 replaces it.

## Appendix A: research dispositions
One row per id the decision map gives this record (127).

| id | disposition | where or why |
|---|---|---|
| R1-TICK-REC2 | changed: TICK-30 replaces its rule (r1-simulation-core.md:2029, 2061–2062); 30 Hz is the baseline, 60 Hz only with the 60 Hz margin; P1–P3 stay marks at 30 Hz and, for the margin run, at 60 Hz; P1 and P2 are ADR 0004's, P3 this record's | parts 2, 4; budgets table |
| R1-ABL-REC4 | adopted: the player's own weapon state is predicted, damage never; its ability half waits for ABL-PREDICT | part 4; OQ-0001-4 |
| R1-ECS-OQ3 | answered: only the local soldier and the vehicle it drives are replayed, no whole-world replay | part 4 |
| R1-TICK-OQ1 | spike: only if the 60 Hz margin passes; then ADR 0004's determinism suite and ADR 0002's Worker smoothness test decide | part 2 |
| R1-TICK-OQ3 | answered: TICK-30 sets the rate; the shipped rates stay unread and no decision here rests on them | part 2 |
| R1-TICK-OQ4 | open: OQ-0001-7 | part 5 |
| R1-TICK-OQ8 | open: OQ-0001-6 | part 5 |
| R1-DET-OQ6 | answered: own movement, own weapon state, own vehicle and cosmetic shot feedback; abilities per ABL-PREDICT | part 4 |
| R1-DET-OQ9 | answered: no state hash is exchanged with clients; the Worker counts prediction mismatches and correction sizes; the test hash function is ADR 0004's | part 6 (Divergence) |
| R1-BND-OQ8 | answered: no; the wire format is per client and delta-coded, and the Worker writes ADR 0002's block from its own state | part 6 (Not the block) |
| R1-ABL-OQ4 | open: OQ-0001-4 (rides with ABL-PREDICT; the replay from the authoritative state is the model for everything predicted) | parts 4, OQ-0001-4 |
| R1-ABL-OQ8 | answered: tags carry a visibility class; marking makes an enemy team-relevant; the Spy's disguise ([LATER]) would be public with its truth in team and owner fields | part 9 (e) |
| R1-XU-OQ2 | answered: local replay only, 200 ms of game time within 1 ms p99 | part 4 |
| ABL-PREDICT | open: OQ-0001-4 (interim: ability presses wait for the server) | part 4 |
| SEC-SPREAD | answered: (a) applied: spread in an authority-only stream, cosmetic tracers corrected by the hit | part 4 |
| MODES-3 | answered: (a) applied: N = 48 in every mark | parts 2, 3; budgets table |
| R1-XT-OQ1 | answered: the Worker casts the cursor ray against the state it published and the command carries a world aim point; how the page's ray crosses is ADR 0002's | part 5 |
| R1-XT-OQ3 | answered: the interpolation buffer lives in the Worker | part 7 |
| R1-XT-OQ4 | answered: E3; correctness never rests on cross-target identity, since landings are server events; E3b if the proof test counts a desync | part 6 |
| R1-XT-OQ5 | answered: command stamping, the input buffer and the clock from own pings; tick nudging and sub-tick stamps stay open | part 5; OQ-0001-6, OQ-0001-7 |
| R1-XT-OQ6 | answered: `Join` and `JoinAccepted` carry the content hash, the snapshot is not the block, divergence is counted not hashed, a development reload message only if ADR 0005 keeps it | part 6; message catalogue |
| R1-XT-OQ7 | answered: no whole-world replay; ADR 0004 may drop R1-ECS-REC1 mark 3 | part 4 |
| R1-XT-OQ9 | answered: remote soldiers carry stance flags, weapon id and aim, and action starts as cosmetic events | part 6 |
| R1-XT-OQ11 | answered: records name kind and index, the generation on entry; the header carries a 32-bit tick, other ticks are 16-bit offsets | part 6 (Wire widths) |
| R2-SYNC-REC1 | adopted | part 6 |
| R2-SYNC-REC2 | adopted | part 5 |
| R2-SYNC-REC3 | adopted, with SEC-SPREAD (a); abilities per ABL-PREDICT | part 4 |
| R2-SYNC-REC4 | changed: the buffer lives in the Worker (R2-XT-OQ2), and its 2–5 interval bound and marks are the one bound and mark R2-XU-OQ3 asks for | part 7 |
| R2-SYNC-REC5 | adopted; the cap value stays open | part 8; OQ-0001-5 |
| R2-SYNC-REC6 | adopted (E3, with E3b as the fallback) | parts 6, 7 |
| R2-SYNC-REC7 | adopted | parts 4, 6 |
| R2-SYNC-REC8 | adopted; the Worker casts the ray | part 5 |
| R2-SYNC-REC9 | adopted | parts 3, 14 |
| R2-BUD-REC1 | changed: the snapshot half (30 Hz) stands; the tick half is replaced by TICK-30; inputs repeat every unacknowledged command up to the cap (R2-SYNC-REC2) instead of 3; marks restated at 30 Hz and against P2 and P3 | parts 2, 5; budgets table |
| R2-BUD-REC2 | adopted; the hard cap proposed at 256 kbps | part 6; budgets table |
| R2-BUD-REC3 | adopted, with one cap rule (R2-XU-OQ5) | part 6 |
| R2-BUD-REC4 | changed: the interpolation bound follows R2-SYNC-REC4 (R2-XU-OQ3); the rewind cap is OQ-0001-5 and the history keeps the cap plus a margin, not 1 s (R2-XU-OQ7); the input buffer and the clock are adopted; need: VISION_V2 §4 only | parts 5, 7, 8 |
| R2-BUD-REC5 | adopted; need: VISION_V2 §4 only | part 10 |
| R2-BUD-REC6 | changed: restated at 30 Hz, its reason corrected (r2-netcode.md:2402–2403, carry-forward 2), its "the rule does not change" replaced by TICK-30 | parts 2, 3; budgets table |
| R2-INT-REC1 | adopted | part 9 (a) |
| R2-INT-REC2 | adopted | part 9 (b) |
| R2-INT-REC3 | adopted: the pass serves the bots and hides by sight only if R2-INT-OQ1's rule asks; its 5 % is part of ADR 0004's sim step share | part 9 (c) |
| R2-INT-REC4 | adopted | part 9 (d) |
| R2-INT-REC5 | adopted | part 9 (e); message catalogue |
| R2-INT-REC6 | adopted | part 9 (f) |
| R2-INT-REC7 | changed: not built; a spectator profile is added only if Mustafa wants spectators (R2-XU-OQ17); no VISION_V2 §6 item names them | part 9 |
| R2-TRN-REC1 | adopted; its load mark restated at 30 Hz (30 input datagrams per second per session) | part 10; budgets table |
| R2-TRN-REC2 | adopted, with one cap rule | parts 6, 10 |
| R2-TRN-REC3 | adopted; tokio-tungstenite picked, since the room serves no HTTP | part 10 |
| R2-TRN-REC4 | changed: a third, loopback backend for the local server (SLICE-ORDER condition 1) | part 10 |
| R2-TRN-REC6 | adopted; need: VISION_V2 §4 only | part 10 |
| R2-ROOM-REC1 | adopted; the fresh sim per match also resets the player records (UPGRADE-SCOPE) | part 11 |
| R2-ROOM-REC2 | adopted | part 11 |
| R2-ROOM-REC3 | adopted, with R2-ROOM-OQ2's signature scheme | part 11 |
| R2-ROOM-REC4 | adopted | part 11 |
| R2-ROOM-REC5 | adopted; code length and access stay open | part 11; OQ-0001-11 |
| R2-ROOM-REC6 | adopted | part 11 |
| R2-ROOM-REC7 | adopted; the file format and validation are ADR 0005's | part 11 |
| R2-ROOM-REC8 | adopted; where the secret finally lives is R2-ROOM-OQ11 (ADR 0006) | part 11 |
| R2-SYNC-OQ1 | answered: TICK-30 and part 2 (30 Hz tick and snapshots); the marks are restated at 30 Hz | parts 2, 6–8 |
| R2-SYNC-OQ2 | open: OQ-0001-5 | part 8 |
| R2-SYNC-OQ3 | open: OQ-0001-6 | part 5 |
| R2-SYNC-OQ4 | answered: independent entity-group datagrams, deltas against one baseline, acknowledged per group | part 6 |
| R2-SYNC-OQ5 | spike: the netcode proof test measures corrections next to movers (R2-SYNC-REC7 mark 3); the collision rule itself is a design question (R2-XT-OQ13) | What a spike must prove |
| R2-SYNC-OQ6 | open: OQ-0001-8 | part 6 |
| R2-SYNC-OQ8 | open: OQ-0001-9 | part 8 |
| R2-SYNC-OQ9 | open: OQ-0001-9 | part 8 |
| R2-SYNC-OQ10 | open: OQ-0001-4 | part 4 |
| R2-SYNC-OQ11 | answered: SEC-SPREAD (a) | part 4 |
| R2-SYNC-OQ12 | spike: the netcode proof test (linear first; Hermite only on artefacts) | part 7 |
| R2-SYNC-OQ14 | answered: no recommendation rests on the unread Valve pages; the victim's side is OQ-0001-5 | part 8 |
| R2-SYNC-OQ15 | open: OQ-0001-5 | part 8 |
| R2-BUD-OQ1 | answered: TICK-30 sets the rate; the unread shipped rates decide nothing here | part 2 |
| R2-BUD-OQ2 | answered: no recommendation rests on the unread Source defaults | part 7 |
| R2-BUD-OQ3 | spike: the netcode proof test records each browser's `maxDatagramSize` | part 6 |
| R2-BUD-OQ4 | answered: the protocol holds ±327.68 m and 0–40.95 m at 1 cm; a larger map adds a bit or uses 2 cm once LOOP_V2 §9's map size [OPEN] is set | part 6 |
| R2-BUD-OQ6 | answered: MODES-3 (a) | Context |
| R2-BUD-OQ7 | spike: the netcode proof test (the same question as R2-SYNC-OQ12) | part 7 |
| R2-BUD-OQ8 | answered: E3, E3b if the proof test counts a desync | part 6 |
| R2-BUD-OQ9 | spike: the netcode proof test measures encoding per client | budgets table |
| R2-BUD-OQ10 | spike: the netcode proof test (R2-TRN-REC6's marks) | What a spike must prove |
| R2-BUD-OQ11 | open: OQ-0001-13 | Open questions |
| R2-INT-OQ5 | spike: only if R2-INT-OQ1's rule asks for sight hiding; the pass's settings and cost are measured in ADR 0004's sim core spike (its 5 % is ADR 0004's number) | part 9 (c) |
| R2-INT-OQ6 | open: OQ-0001-12 | part 9 (f) |
| R2-INT-OQ7 | answered: the hysteresis and margin values are data, set from R2-INT-REC1 marks (a) and (b) when interest management is built in slice 3 | part 9 (a); Migration plan, step 5 |
| R2-TRN-OQ1 | spike: the netcode proof test logs each browser's negotiated draft; then ADR 0002's recurring beta-channel run | What a spike must prove |
| R2-TRN-OQ2 | spike: the netcode proof test with UDP blocked | What a spike must prove |
| R2-TRN-OQ3 | spike: the netcode proof test (R2-TRN-REC2 mark 1) | part 6 |
| R2-TRN-OQ5 | spike: the netcode proof test picks the crate | part 10 |
| R2-TRN-OQ6 | answered: qmux is not adopted; it is re-assessed only if the thin interface needs more than one ordered channel over WebSocket | part 10; Options Q17 |
| R2-TRN-OQ7 | open: OQ-0001-13 | Open questions |
| R2-TRN-OQ8 | spike: the netcode proof test, measured on the client side | What a spike must prove |
| R2-TRN-OQ9 | answered: no transport decision needs the unread page | Sources |
| R2-ROOM-OQ1 | spike: the netcode proof test (R2-ROOM-REC2's marks) | part 11 |
| R2-ROOM-OQ2 | answered: the security review's scheme (asymmetric signature, one pinned algorithm, token type, key id, signature before the cache, a bounded cache), accepted with this record; key storage waits on the security baseline (ADR 0006) | part 11 |
| R2-ROOM-OQ8 | open: OQ-0001-13 | Open questions |
| R2-ROOM-OQ10 | open: OQ-0001-11 | part 11 |
| R2-XU-OQ1 | open: OQ-0001-5 | part 8 |
| R2-XU-OQ2 | spike: the netcode proof test measures how far behind the server's present the client's view is when its command arrives | part 8 |
| R2-XU-OQ3 | answered: 2 to 5 intervals, starting at 3, with R2-SYNC-REC4's marks | part 7 |
| R2-XU-OQ4 | answered: every unacknowledged command up to a cap of 8 (data) within 48 kbps; the real size is measured by the proof test | part 5; bandwidth estimate |
| R2-XU-OQ5 | answered: min(the smallest measured `maxDatagramSize`, 1,150 B), 1,000 B in the load until measured | part 6 |
| R2-XU-OQ6 | answered: the first full state over a reliable stream, deltas after its acknowledgement; one join mark, a complete world within 300 ms at P2 | part 6; What a spike must prove |
| R2-XU-OQ7 | answered: the history keeps the cap plus a margin, at most 1 MB; the load is sized at 700 shots per second | part 8 |
| R2-XU-OQ8 | answered: settled by the glossary (one server tick budget, two 25 % shares), restated at 30 Hz | part 3 |
| R2-XU-OQ10 | answered: own pings and acknowledgements; browser counters only where `getStats()` exists | parts 5, 10 |
| R2-XU-OQ11 | answered: one guard per direction: `outgoingMaxAge` on client inputs, the non-waiting `send_datagram` on the server | part 10 |
| R2-XU-OQ12 | answered: one named set, P0–P4 and B1 | part 13 |
| R2-XU-OQ13 | answered: MODES-3 (a) | Context |
| R2-XU-OQ15 | spike: the netcode proof test measures the record sizes | bandwidth estimate |
| R2-XU-OQ16 | answered: Fiedler's articles are cited as practitioner articles, behind specifications and shipped-game code (the README's order); the one rule here resting on them, three intervals of interpolation, is measured by the proof test | part 7 |
| R2-XU-OQ18 | answered: the drafts' trace reading (a VISION_V2 §3 or §4 target counts like LOOP_V2, written "need: VISION_V2 §4 only" and listed in docs/adr/README.md): R2-BUD-REC4, R2-BUD-REC5, R2-TRN-REC6; R2-BUD-REC3 traces to §6.1; R2-INT-REC7 is not built | parts 7, 10 |
| R2-XT-OQ1 | answered (= R1-XT-OQ1) | part 5 |
| R2-XT-OQ2 | answered (= R1-XT-OQ3) | part 7 |
| R2-XT-OQ3 | answered (= R1-XT-OQ4) | part 6 |
| R2-XT-OQ5 | answered (= R1-XT-OQ5); its open parts are OQ-0001-6 and OQ-0001-7 | part 5 |
| R2-XT-OQ6 | answered (= R1-XT-OQ6) | part 6; message catalogue |
| R2-XT-OQ7 | answered (= R1-XT-OQ7) | part 4 |
| R2-XT-OQ8 | answered (= R1-XT-OQ9) | part 6 |
| R2-XT-OQ11 | answered (= R1-XT-OQ11) | part 6 |
| R2-XT-OQ14 | answered: every R2 mark restated against a named profile | part 13 |
| R3-XT-OQ1 | answered (= R1-XT-OQ3) | part 7 |
| R3-XT-OQ2 | answered (= R1-XT-OQ9) | part 6 |
| R4-XT-OQ1 | answered (= R1-XT-OQ1) | part 5 |
| R4-XT-OQ3 | answered (= R1-XT-OQ7) | part 4 |
| R5-XT-OQ2 | answered (= R1-XT-OQ3) | part 7 |
| R5-XT-OQ10 | answered (= R2-XT-OQ14) | part 13 |
