# R2 unit interest: interest management for a top-down camera

Brief question (quoted): "interest management for a top-down camera" (`docs/V2_PHASE0_BRIEF.md` §4, row R2: "Snapshots,
client prediction, interpolation and lag compensation for 48+ players; tick rate and bandwidth budgets; interest
management for a top-down camera; the WebTransport server stack in Rust plus the WebSocket fallback; rooms and private
matches; learn from shipped games (Valve's Source networking, Overwatch's GDC talks, Glenn Fiedler's articles, Quake 3)").

Author: netcode-architect, 2026-10-27. Status: unit draft (not yet verified). The camera framing is "look brief
pending": it enters here only as parameters. Every pass mark and reference machine below is proposed.

## 1. Findings

How sure: "confirmed in the source" means the quoted words were returned verbatim from the source; "inferred" means
a WebFetch paraphrase, or arithmetic and reasoning on confirmed facts (the inputs are named); "unverified" means not
checked. Sources S1–S14 are listed in section 6.

**What shipped engines send to each client**

- **R2-INT-F1.** Quake III builds each client's snapshot entity list separately, testing each entity in turn: it
  skips entities flagged never-send, sends single-client and client-mask entities only to their recipients, always
  sends broadcast entities, skips entities whose area is not connected to the client's area ("blocked by a door"),
  and skips entities outside the client cluster's potentially visible set (PVS).
  Verbatim (`SV_AddEntitiesVisibleFromPoint`): `if ( ent->r.svFlags & SVF_NOCLIENT ) {` · `if ( ent->r.svFlags &
  SVF_SINGLECLIENT ) {` / `if ( ent->r.singleClient != frame->ps.clientNum ) {` · `if (~ent->r.singleClient & (1 <<
  frame->ps.clientNum))` (SVF_CLIENTMASK) · `if ( ent->r.svFlags & SVF_BROADCAST ) {` with the comment "broadcast
  entities are always sent" · `if ( !CM_AreasConnected( clientarea, svEnt->areanum ) ) {` with "blocked by a door" ·
  `clientpvs = CM_ClusterPVS (clientcluster);` and `if ( bitvector[l >> 3] & (1 << (l&7) ) ) {`.
  Source: S1, https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/server/sv_snapshot.c (`code/server/sv_snapshot.c`, `SV_AddEntitiesVisibleFromPoint`). Tier: shipped game. Read 2026-10-27.
  How sure: confirmed in the source.
- **R2-INT-F2.** Quake III caps a snapshot at 1,024 entities and silently drops any beyond that; it has no
  per-entity priority. Instead it paces each client's whole snapshot stream by that client's byte rate.
  Verbatim: `#define	MAX_SNAPSHOT_ENTITIES	1024`; in `SV_AddEntToSnapshot` the comment "if we are full, silently
  discard entities"; in `SV_RateMsec`: `if ( messageSize > 1500 ) {` and `rateMsec = ( messageSize +
  HEADER_RATE_BYTES ) * 1000 / rate;`; then `client->nextSnapshotTime = svs.time + rateMsec;`.
  Source: S1, https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/server/sv_snapshot.c (`SV_AddEntToSnapshot`, `SV_RateMsec`, `SV_SendMessageToClient`). Tier: shipped game. Read 2026-10-27.
  How sure: the cap, the discard and the rate pacing are confirmed in the source. "No per-entity priority" is
  inferred: nothing in the fetched file ranks entities (absence, not a quoted statement).
- **R2-INT-F3.** In Quake III, an entity that enters a client's snapshot, having been absent from the frame it is
  delta-coded against, is coded against its static baseline rather than its last state. An unchanged entity costs
  no bytes.
  Verbatim (`SV_EmitPacketEntities`): `// this is a new entity, send it from the baseline` followed by
  `MSG_WriteDeltaEntity (msg, &sv.svEntities[newnum].baseline, newent, qtrue );`; an unchanged entity "will not
  result in any bytes being emited if the entity has not changed at all".
  Source: S1, https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/server/sv_snapshot.c. Tier: shipped game. Read 2026-10-27. How sure: confirmed in the source.
- **R2-INT-F4.** Quake III answers "are these two areas connected?" in constant time. It compares flood numbers that
  are precomputed by a flood fill through open portals. A door changes a reference-counted portal and triggers a
  fresh flood of the whole map.
  Verbatim (`cm_test.c`): `if (cm.areas[area1].floodnum == cm.areas[area2].floodnum) {` · in
  `CM_AdjustAreaPortalState`: `cm.areaPortals[ area1 * cm.numAreas + area2 ]++;` (open) and `--;` (close), then
  `CM_FloodAreaConnections ();`.
  Source: S2, https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/qcommon/cm_test.c. Tier: shipped game. Read 2026-10-27. How sure: confirmed in the source.
- **R2-INT-F5.** Quake II routes a positioned message to every client (MULTICAST_ALL), to the clients that could
  hear it (MULTICAST_PHS), or to the clients that could see it (MULTICAST_PVS), and skips clients behind closed
  areas. A sound that does not fade with distance goes to everyone, and reliable sounds take the reliable stream.
  Verbatim (`sv_send.c`): "MULTICAST_ALL	same as broadcast (origin can be NULL)" · `if (!CM_AreasConnected (area1,
  area2))` · in `SV_StartSound`: "// if the sound doesn't attenuate,send it to everyone", `if (attenuation ==
  ATTN_NONE)`, `use_phs = false;` · `if (channel & CHAN_RELIABLE)`.
  Source: S3, https://raw.githubusercontent.com/id-Software/Quake-2/master/server/sv_send.c (`SV_Multicast`, `SV_StartSound`). Tier: shipped game. Read 2026-10-27. How sure: the broadcast, area,
  no-attenuation and reliable lines are confirmed in the source. That PHS means "the origin cluster's potentially
  hearable set, tested per client cluster" is inferred (WebFetch paraphrase only).
- **R2-INT-F6.** In Quake III, sound and other events are short-lived entities. They are linked into the PVS like any
  entity, so the same per-client test filters them, and their origin is snapped to whole units to save bandwidth.
  Verbatim (`g_utils.c`, `G_TempEntity`): "Spawns an event entity that will be auto-removed" · "The origin will be
  snapped to save net bandwidth" · `SnapVector( snapped );		// save network bandwidth` · `// find cluster for PVS` ·
  `trap_LinkEntity( e );`; `G_Sound`: `te = G_TempEntity( ent->r.currentOrigin, EV_GENERAL_SOUND );`.
  Source: S12, https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/g_utils.c. Tier: shipped game. Read 2026-10-27. How sure: confirmed in the source.
- **R2-INT-F7.** Quake III broadcasts the death event (the kill feed) to every client, but sends a score pop-up to
  one client only.
  Verbatim (`g_combat.c`): `// broadcast the death event to everyone` · `ent = G_TempEntity( self->r.currentOrigin,
  EV_OBITUARY );` · `ent->r.svFlags = SVF_BROADCAST;	// send to everyone` · `// only send this temp entity to a
  single client` · `plum->r.svFlags |= SVF_SINGLECLIENT;`.
  Source: S13, https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/g_combat.c (`player_die`, `ScorePlum`). Tier: shipped game. Read 2026-10-27. How sure: confirmed in the source.
  The death event carries the victim's origin (`self->r.currentOrigin`), so in Q3 a broadcast death also reveals
  where it happened (inferred from that line).
- **R2-INT-F8.** In the Source engine every entity, for each recipient, is sent always, culled by the PVS (the
  default) or never sent. A team hook can force an entity to a team's players.
  Verbatim (`baseentity.cpp`, `CBaseEntity::ShouldTransmit`): `if ( fFlags & FL_EDICT_PVSCHECK ) return
  FL_EDICT_PVSCHECK;` · `else if ( fFlags & FL_EDICT_ALWAYS ) return FL_EDICT_ALWAYS;` · `if (
  pRecipientPlayer->GetTeam()->ShouldTransmitToPlayer( pRecipientPlayer, this ))` · `// by default do a PVS check`;
  `UpdateTransmitState`: `// by default cull against PVS` → `return SetTransmitState( FL_EDICT_PVSCHECK );`.
  Source: S7, https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/baseentity.cpp. Tier: shipped game. Read 2026-10-27. How sure: confirmed in the source. The PVS test itself is in the
  closed engine and was not read.
- **R2-INT-F9.** In Source, sending a child entity forces its parent to be sent as well, so attached parts arrive
  together.
  Verbatim (`CBaseEntity::SetTransmit`): "// Force our aiment and move parent to be sent." · "// in HLTV/Replay mode
  always transmit entitys with move-parents".
  Source: S7, https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/baseentity.cpp. Tier: shipped game. Read 2026-10-27. How sure: confirmed in the source.
- **R2-INT-F10.** In Source the team entity, which carries team-wide state, goes to every client. A spectator always
  receives the player it is watching.
  Verbatim (`team.cpp`): "// Purpose: Teams are always transmitted to clients" · `return SetTransmitState(
  FL_EDICT_ALWAYS );` · "// Always transmit the observer target to players" · `if ( pRecipient &&
  pRecipient->IsObserver() && pRecipient->GetObserverTarget() == pEntity )`.
  Source: S8, https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/team.cpp. Tier: shipped game. Read 2026-10-27. How sure: confirmed in the source.
- **R2-INT-F11.** Source can limit a single network field to one team's players.
  Verbatim (`sendproxy.cpp`): "// Purpose: Proxy that only sends data to team members" ·
  `pRecipients->ClearAllRecipients();` · `for ( int i=0; i < pTeam->GetNumPlayers(); i++ )` ·
  `pRecipients->SetRecipient( pTeam->GetPlayer( i )->GetClientIndex() );`.
  Source: S10, https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/sendproxy.cpp (`SendProxy_OnlyToTeam`). Tier: shipped game. Read 2026-10-27. How sure: confirmed in the source.
- **R2-INT-F12.** TF2 puts a disguised spy's disguise team, class, target and health in its shared player table,
  not in the owner-only local table, so they appear to reach every client.
  Verbatim (`tf_player_shared.cpp`, inside `BEGIN_SEND_TABLE_NOBASE( CTFPlayerShared, DT_TFPlayerShared )`):
  `SendPropInt( SENDINFO( m_nDisguiseTeam ), 3, SPROP_UNSIGNED ),` · `SendPropInt( SENDINFO( m_nDisguiseClass ), 4,
  SPROP_UNSIGNED ),` · `SendPropEHandle( SENDINFO( m_hDisguiseTarget ) ),` · `SendPropInt( SENDINFO(
  m_iDisguiseHealth ), -1, SPROP_VARINT ),`.
  Source: S9, https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/shared/tf/tf_player_shared.cpp. Tier: shipped game. Read 2026-10-27. How sure: the lines and their table are confirmed in the source.
  That this table reaches all clients, and that a separate "tfsharedlocaldata" table reaches only the owner, are
  inferred (WebFetch paraphrase). Whether the spy's true team is also sent was not read: unverified.
- **R2-INT-F13.** Source's spectator broadcast (SourceTV) runs 30 s behind the live game by default.
  Verbatim (`hltvdirector.cpp`): `static ConVar tv_delay( "tv_delay", "30", 0, "SourceTV broadcast delay in seconds",
  true, 0, true, HLTV_MAX_DELAY );` · `// broadcast time is current time - delay time` · `newBroadcastTick -=
  TIME_TO_TICKS( m_fDelay );`.
  Source: S14, https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/hltvdirector.cpp. Tier: shipped game. Read 2026-10-27. How sure: confirmed in the source. The file does not give the
  delay's purpose as anti-cheat ("ghosting"); that reading is inferred.
- **R2-INT-F14.** Fiedler's priority accumulator works like this. Each object gets a priority every frame, which is
  added to an accumulator the object keeps. Objects are sent in accumulator order until the packet's byte budget is
  full. Objects that were sent reset to zero; objects that were not keep their total, so they move up the queue.
  The player's own object gets a priority high enough to be in every packet.
  Verbatim: "The first n objects in this sorted list are the objects you should send that frame." · "reset the
  priority accumulator to zero for objects that fit." · "leave the priority accumulator value alone for objects
  that didn't." · "I give them a higher priority of 100 while at rest objects have priority of 1." · budget example
  "256kbit/sec" · "a maximum of 64 state updates per-packet".
  Source: S4, https://gafferongames.com/post/state_synchronization/ ("State Synchronization", section "Priority accumulator"). Tier: other (practitioner article). Read
  2026-10-27. How sure: confirmed in the source. The player's priority "1000000" is also quoted there.
- **R2-INT-F15.** With delta compression against an acknowledged baseline, an object that has not changed costs one
  bit.
  Verbatim: "Cube n in snapshot 110 is the same as the baseline. One bit: Not changed!"; a stationary scene costs
  "around 15 kilobits per-second".
  Source: S5, https://gafferongames.com/post/snapshot_compression/ ("Snapshot Compression"). Tier: other. Read 2026-10-27. How sure: confirmed in the source. That the
  baseline advances to the newest acked snapshot (about one round trip old) is inferred (paraphrase).

**The browser and the camera**

- **R2-INT-F16.** Chrome's built-in developer tools let a player inspect the raw memory of a page's JavaScript and
  WebAssembly while it runs.
  Verbatim: "Use the Memory inspector to inspect ArrayBuffer, TypedArray, and DataView memory in JavaScript" ·
  "The Memory Inspector panel lets you inspect such objects in Wasm applications written in C++." · "Use the Memory
  inspector while you debug your web app for an efficient workflow."
  Source: S11, https://developer.chrome.com/docs/devtools/memory-inspector. Tier: official. Read 2026-10-27 (page updated 2024-09-06). How sure: confirmed in the source.
  Consequence (inferred): every byte the server sends to a browser client can be read by that player with tools
  already in the browser, so client-side hiding protects nothing. The robust anti-wallhack measure is never to send
  what the player must not see.
- **R2-INT-F17.** three.js's `PerspectiveCamera.fov` is the vertical field of view in degrees. The camera can report
  the 2D bounds of its view rectangle at a given distance.
  Verbatim: "The vertical field of view, from bottom to top of view, in degrees." · `getViewBounds()`: "Computes the
  2D bounds of the camera's viewable rectangle at a given distance along the viewing direction."
  Source: S6, https://raw.githubusercontent.com/mrdoob/three.js/dev/src/cameras/PerspectiveCamera.js (three.js `src/cameras/PerspectiveCamera.js`, dev branch). Tier: official. Read 2026-10-27. How sure:
  confirmed in the source.

**Arithmetic on legacy numbers (inferred)**

- **R2-INT-F18.** The legacy default camera (style 3: height 40 m, horizontal boom 56.6 m, vertical FOV 15°) at
  16:9 sees a ground trapezoid about 27.4 m wide at the near edge and 39.9 m at the far edge, and 32.75 m deep
  (13.3 m toward the camera and 19.4 m beyond the look point). That is about 1,100 m². Its farthest corner is about
  28 m from the look point, so a circle of radius about 28 m covers the view at any camera yaw. Style 5's ADS pan
  (foxPanMax 16 m) would push this to about 44 m.
  Inputs: CD5, GP4, CAM_STYLES (legacy), F17 (vertical FOV). Method: the four frustum-corner rays meet the ground
  plane; pitch atan(40/56.6) = 35.25°, half-FOV 7.5°, horizontal half-tangent 16/9 × tan 7.5° = 0.234. How sure:
  inferred (arithmetic; not run in code, as the no-spike rule requires). The framing is look brief pending: these
  numbers are an example, not a target.
- **R2-INT-F19.** On the legacy arena (372 m × 252 m = 93,744 m², CD21), a relevance circle of radius 28 m covers
  2.6 % of the map, 40 m covers 5.4 % and 50 m covers 8.4 %. With 48 players spread evenly, a 40 m circle holds
  about 2.5 of the 47 others. In the worst case, everyone fighting over the centre field (LOOP_V2 §3, §9), it holds
  all 47. Interest management therefore cuts the average per-client snapshot by roughly an order of magnitude, and
  the worst case by nothing. Only per-entity priority and rate control under a per-client byte budget bound the
  peak. Illustration: 47 soldiers × 12–25 bytes per changed record (record sizes belong to the snapshots unit) is
  564–1,175 bytes per snapshot, or 90–188 kbit/s per client at 20 snapshots/s, before vehicles, projectiles and
  events.
  Inputs: CD21, F18, F2, F14. How sure: inferred (the sizes are illustrative, not measured).
- **R2-INT-F20.** The cost of hiding enemies by per-pair line of sight is bounded by the enemy pairs, because
  teammates are always relevant: 24v24 gives 24 × 24 = 576 unordered pairs, 16v16v16 gives 3 × 16 × 16 = 768, and
  12v12v12 gives 432. Legacy `hasLOS` measured 2.7–5.7 µs per call in JavaScript (linear over 220 static boxes, no
  broad phase). At that cost a 768-pair pass with one sample point takes 2.1–4.4 ms, and 6–13 ms with three points.
  At 20 passes/s, one point is 4–9 % of one core. A Rust ray cast with a spatial index has not been measured.
  Inputs: CS6, CS9 (legacy measurements, reports/2026-10-15-team-prep-fixes.md:56; the hardware is not recorded),
  VISION_V2 §3. How sure: inferred.

## 2. Legacy facts used

| Row or table | Fact | What it tells v2 |
|---|---|---|
| CD5, GP4, `CAM_STYLES` (LEGACY_TUNING) | Default style 3: camHeight 40, camBack 56.6 (horizontal boom, shamble-game.js:2819), fov 15, yaw 45° plus middle-mouse yaw, dead zone 0.60 × 0.52, look-ahead; ADS 'pitch'; style 5 'pan' with foxPanMax 16 m. Styles 1–5 range from 15.5 to 52 m high and 12 to 52° FOV. *Look brief pending.* | The view area depends strongly on the framing, so the relevance region must be data computed from the camera parameters (F18), not a constant. Camera yaw and pan move the view, so the region must cover rotation and the largest client-side camera offset. |
| CD21 | Arena 372 × 252 m (arenaX 186, arenaZ 126) | Map area for the bandwidth fraction (F19). Octopon-3's size is open (LOOP_V2 §9). |
| CD4, `UNIT_TYPES` | Walk 6.3 m/s; sprint ×1.43 (T), up to ×1.62 per class (about 10.2 m/s) | Sets the relevance margin: an infantry unit moves about 2.6 m in 250 ms. |
| `JUG`, `JUG_CANNON`, `VEHICLE_TYPES` | Juggernaut: maxSpeed 6, targetH 5.6 m, cannon range 130 m, 1 seat | Big units and their shells reach well past a ~28 m view, so relevance must use entity bounds, and projectiles must be entities in their own right (REC6). |
| CD6, GN4 | Bot sight 44 m; one sight budget of 480 LOS checks per second, carried over; demand about 1,100/s at 48 soldiers (LOS-SIZE open); eyes at +1.45 m | The server already needs pairwise visibility for bots. One shared visibility pass can serve bots and interest (REC3). |
| CS6, CS9 | `rayStatics` is linear over every static (220 in the test); `hasLOS` 2.7–5.7 µs, statics only, soldiers never block, 0.5 m end margin | Cost baseline for F20. The inventory already asks for a spatial index in the Rust sim. |
| CS8, GW16 | The legacy hit world is players plus kinded static boxes with an alive flag | The world the interest test reads. In v2 the Rust sim owns it. |
| GW15 | Blasts use scene raycasts, bullets use the SIM statics (two worlds) | v2 needs one LOS query, which the interest test then shares. |
| GW17 | Client-only cutaway: a roof fades while the local player is under it; anything between camera and player fades to 24 %. *Look brief pending.* | Other buildings' roofs stay opaque, so enemies inside them are hidden on screen. If the server still sends them, a memory reader sees them (F16). The server's hiding rule and the client's cutaway must follow one shared rule. |
| GW20 | The enterable barracks: five walls with a door gap, a roof; LOOP_V2 §2: enemies cannot enter a barracks interior | The first enclosure (an area with a door portal). Spawning players inside it must not be visible to enemies through walls. |
| GN8 | Bot muzzle flash, tracer and impact culled beyond 50 m from the camera's look point; sound beyond 90 m; shots at you or hitting you are never culled | Legacy already gives sound a larger radius than visuals, and always delivers shots aimed at the viewer (REC5). |
| GP6 | Positional enemy shots with inverse-distance gain; no voice cap, no occlusion | The client needs sound events from outside the screen. |
| GP11 | The scoreboard shows each side's alive count and tier, including the hostile side | Legacy treats the enemy tech tier as public. Whether v2 does is a design question (OQ3). |
| (none) | No inventory row names a minimap or radar | A v2 minimap is new: its data needs are open (OQ4). |

## 3. Options

**Q-A. The relevance region (what is "near enough to send")**

| Option | Complexity | CPU per tick | Bandwidth | Risk | Fit with VISION_V2 §6 |
|---|---|---|---|---|---|
| A1. No region: send everything | None | Lowest | Highest; the worst case equals the average | Every enemy position reaches every client (F16); serialization grows with N² | Fails §6.2's hiding needs |
| A2. Fixed circle around the player | Low | Grid query, trivial | Good | Wrong if the framing, aspect or pan changes; pop-in at the edge | Weak: the framing is look brief pending |
| A3. Region from the camera parameters as data: a yaw-proof circle around the player of radius = view-footprint circumradius + the largest camera offset the client rig may use (dead zone, look-ahead, ADS pan) + a motion margin, with enter/exit hysteresis | Medium | Grid query plus one distance test per candidate | Good (F19) | Needs the client rig to stay inside the server's bound (OQ2) | Good: §6.1 (modes as data: the camera per mode) |
| A4. A3 shifted toward the aim direction (the server has the aim from input commands) | Medium+ | Same | Slightly better than A3 | The aim the server sees is about one RTT old; a quick flick shows an edge before the data arrives | As A3; an optimisation over it |

**Q-B. Hiding what the player must not see (anti-wallhack)**

| Option | Complexity | CPU | Strength against memory readers | Visual risk | Fit |
|---|---|---|---|---|---|
| B1. Region only | Lowest | None extra | Weak: enemies inside buildings, on hidden floors and behind walls in view are all sent | None | Fails §6.2 / pillar 6 ("I sat in a building…") |
| B2. Enclosures as areas with portals (interiors, each floor, barracks; openings as portals; destroyed walls open portals), Q3-style flood connectivity (F4) | Medium (needs area and portal data from the map; R4) | O(1) per pair, plus a re-flood when a structure changes | Strong for interiors and floors; does nothing for open-ground walls | Low if portals are conservative | Direct fit with §6.2 (floors, interiors) |
| B3. Per-pair line of sight from the viewer's character (fog of war), conservative: several sample points and a look-ahead | High | Highest (F20); amortizable | Strongest | Pop-in at corners under latency if not conservative | Only if the game rule says the character's sight limits the view (OQ1) |
| B4. B2 everywhere, plus B3 only through portals (doors, windows) and for wall types flagged "sight-blocking" | High | Between B2 and B3 | Strong | As B3, but at fewer places | Fits §6.2. Its weight depends on OQ1. |

**Q-C. Rate control inside the region**

| Option | Complexity | Peak bound | Fairness and feel | Precedent |
|---|---|---|---|---|
| C1. Every relevant entity every snapshot; when a client's byte rate is exceeded, its whole snapshot stream slows down | Low | Rate only: the whole picture goes stale together | A crowded fight lowers everyone's update rate at once | Quake III (F2) |
| C2. Per-client priority accumulator with a byte budget per snapshot | Medium | Hard bound per snapshot | Close, threatening entities stay fresh; far ones degrade gracefully and are never starved | Fiedler (F14) |
| C3. Fixed tiers (near every snapshot, far every k-th) | Low–medium | Approximate | Simple, but it ignores threat and can exceed the budget in a crowd | (no read source) |

**Q-D. Edge cases**

| Case | Options | Notes |
|---|---|---|
| Spectators (rooms, private matches) | D1 follow a player: the spectator gets that player's interest (F10); D2 free camera on a delayed whole-map stream (F13); D3 live whole-map stream | D3 is ghosting by design: a spectator in voice contact with a team sees everything live (inferred) |
| Big units and vehicles | D4 relevance by entity bounds plus a per-type visual radius (data); D5 always relevant | D5 is simple but leaks every vehicle's position. D4 also covers a Juggernaut body overlapping the view. |
| Multi-part vehicles, passengers | Send parts and seated players with their parent (F9) | §6.4: big, multi-part, seats [OPEN] |
| Sound, muzzle flash, impacts | D6 events filtered by their own radius per event type (data), area-connected (F5, F6); D7 send all | Legacy: sound radius 90 m vs visuals 50 m (GN8) |
| Global events (kill feed, uplink captured, tech tier reached) | Broadcast on the reliable stream (F5, F7) | A kill-feed event with an exact position reveals that position (F7) |
| Teammates' pings, minimap | Team-only reliable messages; teammates always relevant at a low rate for the minimap (F11) | What the minimap shows is open (OQ4) |
| Team-private economy and objectives (§6.5) | Per-field visibility classes: public / team / owner / spatial (F1, F8, F11) | Source sends its team entity to everyone (F10), so team-private data must never sit on a public record |
| The Spy's disguise (LOOP_V2 §5, LATER) | Send enemies only the disguise as the identity they see; keep the true team in team- and owner-only fields | F12: TF2 sends the disguise fields to all clients (the true team was not read). v2 must not leak the truth. |

## 4. Recommendations

- **R2-INT-REC1. Relevance region from camera parameters held as data (option A3), with hysteresis.**
  Per mode, the server holds the camera's worst-case view: height, boom, vertical FOV, the largest allowed aspect,
  the largest client-side camera offset (dead zone, look-ahead, ADS pan) and the maximum height of interest. From
  these it derives a yaw-proof radius: the footprint circumradius (F18 method), plus the offset bound, plus a motion
  margin (top speed × (interpolation delay + one-way latency + one snapshot interval)). An entity is a candidate when
  its bounds intersect that circle around the player's server position. It stays relevant until it leaves a slightly
  larger exit radius, because each re-entry costs a baseline-coded record (F3). Candidates come from a uniform grid
  over the map. Keep A4 (shift toward the aim) as a later optimisation, only if the spike shows it is needed.
  - Reason: the framing is look brief pending and legacy styles vary the view several times over (CD5, CAM_STYLES),
    so the region must follow the data. The server cannot know the client's exact camera, so the region must bound
    it. Re-entries cost bytes (F3, F15).
  - Rests on: F1, F3, F15, F17, F18, F19.
  - Serves: §6.1 (modes as data: team sizes up to 24v24 on one server; the camera per mode), §6.2 (what is hidden
    follows the 3D world).
  - Spike must prove (proposed): (a) no pop-in. Over a recorded 10-minute bot match replayed under 0, 50, 100 and
    200 ms latency with ±20 ms jitter and 0–5 % loss, 0 rendered frames contain an entity whose bounds intersect the
    client's actual view while the client lacks its state. (b) Churn: at most 2 enter/exit events per client per
    second, averaged over the match (proposed). (c) The region query for 48 clients takes at most 0.2 ms per tick on
    the reference server (proposed). (d) Bytes per client per second, with the region against without it, reported
    for the even-spread case and the centre-fight case (F19 predicts about 10 times fewer in the first and no saving
    in the second).

- **R2-INT-REC2. Enclosure hiding as the main anti-wallhack measure (option B2): interiors and floors as areas,
  openings as portals.**
  Map data marks each building interior, and each floor of it, as an area, and each door, window, stairwell and
  breakable wall as a portal. The server keeps connectivity as Q3 does: flood numbers, compared in constant time,
  re-flooded when a portal opens or closes (a door, or a wall destroyed) (F4). An enemy inside an area is relevant to
  a viewer only if the viewer's area is connected to it, or if the agreed visibility rule lets the camera see into
  it (a roof destroyed or cut away: OQ1). The client's cutaway (GW17) follows the same rule, so what is not sent is
  also what the screen would not show.
  - Reason: in a browser every received byte is readable (F16), so withholding data is the only defence that holds.
    Q3's area test is the shipped precedent and costs O(1) per pair (F1, F4). Pillar 6's "I sat in a building and
    kept harassing them" needs occupants hidden from those outside.
  - Rests on: F1, F4, F16 (plus legacy GW17, GW20).
  - Serves: §6.2 (floors and building interiors, line of sight across heights).
  - Spike must prove (proposed): (a) a "reader" test client that logs every enemy state it receives logs 0 enemies
    inside an area not connected to the viewer, over a 10-minute bot match on a greybox with at least two enterable,
    two-floor buildings and the barracks (LOOP_V2 §2). (b) The pair test costs at most 0.1 µs per enemy pair, and a
    re-flood after a portal change at most 1 ms for the map's area count (proposed). (c) 0 frames where an enemy
    walking through an open door is visible to the client before its state arrives (the same harness as REC1 (a)).

- **R2-INT-REC3. One shared visibility pass for bots and interest, used for line-of-sight hiding only if the game
  rule asks for it (B3/B4 depend on OQ1).**
  If the rule says the character's sight limits what the player sees, the server runs one visibility pass per
  snapshot interval. It covers enemy pairs only, inside the region and in connected areas, rays the static world
  through the spatial index the sim already needs (CS6), is conservative (several sample points, a look-ahead of
  one RTT), and is amortized across ticks like legacy's sight budget (GN4). Bots read the same results for target
  acquisition. If the rule says no, the pass still serves the bots, and interest uses REC1 and REC2 only.
  - Reason: bots already need pairwise sight (GN4), and the enemy-pair count bounds the cost (F20). One shared pass
    avoids paying twice.
  - Rests on: F20 (inferred cost), F16 (plus legacy CS6, CS9, GN4).
  - Serves: §6.2 (line of sight across heights); LOOP_V2 §10 (bots).
  - Spike must prove (proposed): at 24v24 and 16v16v16, all players in one fight on the greybox, the full pass with
    three sample points takes at most 10 % of the server tick budget on the reference server (the tick rate comes
    from the tick unit), with 0 pop-in frames as REC1 (a), and Wasm and native results identical per tick (R1's
    state-hash test).

- **R2-INT-REC4. A per-client priority accumulator under a byte budget per snapshot (option C2), layered on
  relevance.**
  Each relevant entity gets a priority each snapshot: a per-type base from data, times a factor that falls with
  distance from the view centre, times a threat factor (aiming at or shooting the viewer). The player's own state
  and vehicle are always included. Entities are packed in accumulator order up to the client's byte budget; sent
  ones reset, unsent ones carry their total (F14). A floor guarantees that every relevant entity is sent at least
  every k snapshots.
  - Reason: interest bounds the average, not the peak (F19). Q3's whole-stream pacing degrades everything at once
    (F2). The accumulator keeps a crowded fight inside the budget without starving anyone (F14).
  - Rests on: F2, F14, F19.
  - Serves: §6.1 (48+ players in one match); §6.9 (big units and their effects must not crowd out the soldiers
    near you).
  - Spike must prove (proposed): in the worst case (48 players plus vehicles and projectiles in the centre field),
    0 snapshots exceed the per-client budget (the number comes from the tick and bandwidth unit). The nearest 8
    enemies are in every snapshot; every relevant entity is sent at least every 4 snapshots. Ranking and packing for
    48 clients take at most 0.5 ms per tick on the reference server.

- **R2-INT-REC5. A visibility class on every field and event type, held as data: public, team, owner, spatial.**
  - Public (on the reliable stream): match clock, points, uplink sites with owner and countdown, the cache location
    (LOOP_V2 §4: "everyone is told where"), and the kill feed without positions, unless the design wants them (F7).
  - Team: teammates' positions for the minimap at a low rate, pings, the team's share of a harvester rebuild,
    team-funded actions, and the team's tech progress if it is private (OQ3).
  - Owner: personal money, ammunition, cooldowns, score pop-ups (F7: `SVF_SINGLECLIENT`).
  - Spatial: everything filtered by REC1 to REC3.
  - The Spy (LATER): an enemy client receives the disguise as the identity it sees; the true team stays in team and
    owner fields (F12 shows the risk).
  - Reason: Q3 and Source both filter per entity and per field (F1, F8, F11). Source's public team entity (F10)
    shows the trap: team-private economy data on a public record. §6.5 puts the economy server-side, and keeping
    the economy's secrets is part of that.
  - Rests on: F1, F7, F8, F10, F11, F12, F16.
  - Serves: §6.5 (economy and objectives); §6.1 (team count and size as data); §6.4 (marking: a marked enemy
    becomes team-relevant to the marking team, through the same class).
  - Spike must prove (proposed): a byte-level audit during a 10-minute three-team bot match. Each received message
    is decoded against the schema, and team A's client holds 0 fields of class team or owner belonging to teams B
    or C. Every message type in the catalogue has a class (the audit test fails on an unclassified one).

- **R2-INT-REC6. Events and long-range entities: their own radius per type, area-aware; parts travel with their
  parent.**
  Sound and muzzle-flash events are short-lived records (F6). Each event type carries its own radius as data (sound
  larger than visuals, as legacy has 90 m against 50 m: GN8). Events are filtered by the area test (F5), their
  origin is quantized (F6), and shots at or hitting the viewer are always sent. Events with no fall-off, such as an
  announcer line, are broadcast reliably (F5). Big units are relevant by their bounds plus a per-type visual radius
  (Juggernaut targetH 5.6 m). Long-range shells (cannon range 130 m) are their own entities, relevant where they
  fly. Vehicle parts and seated players are sent with their parent (F9).
  - Reason: precedent in Q2 and Q3 (F5, F6, F7) and in Source (F9); legacy already separates sound and visual radii
    (GN8).
  - Rests on: F5, F6, F7, F9 (plus legacy GN8, JUG).
  - Serves: §6.4 (vehicles: big, multi-part, seats [OPEN]); §6.9 (spectacle); LOOP_V2 §7 (feel kit: muzzle flash
    and sound as data).
  - Spike must prove (proposed): 0 events delivered beyond their type's radius or into an unconnected area. Every
    shot aimed at the viewer is delivered (0 misses over the soak). At the centre-fight peak, event bytes stay inside
    the REC4 budget. A Juggernaut whose body overlaps the view is never missing (the REC1 (a) harness).

- **R2-INT-REC7. Spectators as their own interest profile: follow-a-player live, or free camera on a delayed
  stream.**
  A spectator who follows a player receives that player's interest (F10: the observer target is always sent). A
  free-camera spectator in a room or private match receives a whole-map stream delayed by a per-room value (Source's
  default is 30 s: F13). The delay is room data, set by the rooms unit.
  - Reason: a live whole-map view in a match where the spectator can talk to a team defeats REC2 (inferred).
    Source's shipped spectator stream is delayed (F13).
  - Rests on: F10, F13, F16.
  - Serves: §6.1 (rooms and private matches); §6.8 (moments; kill-cams and replays are [LATER], so nothing more
    here).
  - Spike must prove (proposed): a free-camera spectator's stream is never newer than the room's delay (0 frames
    over the soak). A follow-mode spectator receives exactly the followed player's spatial set (diffed per snapshot
    against that player's set, 0 differences beyond the spectator's own records).

## 5. Open questions

- **R2-INT-OQ1. What may a player see: what the overhead camera could show, or what the character can see?**
  Under the first rule, roofs and hidden floors hide, and walls in open ground do not. Under the second (fog of
  war), every wall hides. This decides whether REC3's ray pass hides anything or only serves bots, and how the cutaway
  works (GW17). It is a game rule tied to the look. Decided by Mustafa with the game-designer, after the look brief
  sets the framing.
- **R2-INT-OQ2. The camera framing and the aspect ratio.** Height, FOV, ADS pan or zoom, look-ahead and the largest
  allowed aspect set REC1's radius, and they are look brief pending. A wider monitor sees more ground, so a fair
  match needs either a fixed world width per mode (letterboxing beyond it) or a region sized for the widest allowed
  aspect. Decided by the look brief and Mustafa.
- **R2-INT-OQ3. Which economy and objective state is team-private?** LOOP_V2 makes the cache location public (§4)
  and personal money personal (§3). It does not say whether enemies see a team's tech total or tier (legacy's
  scoreboard shows it: GP11), an uplink's countdown, harvester health or a rebuild in progress. REC5's classes hold
  whatever is decided. Decided by Mustafa with the game-designer (LOOP_V2 owner).
- **R2-INT-OQ4. What the minimap shows.** Teammates only; enemies spotted by any teammate (shared vision); enemies
  marked by an ability (§6.4 "marking"); objectives. Each choice changes which enemy states become team-relevant,
  and so what a memory reader learns. Decided by game design. The technique (team class plus marks) fits every
  answer.
- **R2-INT-OQ5. How conservative LOS hiding must be, and what it costs in Rust.** The shipped write-up on server-side
  fog of war for a competitive shooter (Riot's VALORANT article) is outside the allow list and was not read. The
  Unreal relevancy and priority docs were not read either. The sample points, the look-ahead, and the cost of a
  BVH ray on the Octopon greybox decide it. They come from the REC3 spike and from R4's line-of-sight work.
- **R2-INT-OQ6. Do far sound events reveal too much?** An exact position on a gunshot 80 m away gives a cheat a
  radar, which a player only roughly hears. Options: a coarser origin at distance, direction only, or an accepted
  leak. No read source settles it (a search lead only, not read). Decided by game design plus a reader-client test.
- **R2-INT-OQ7. Hysteresis and margin values.** No read source gives values for exit radius, linger time or motion
  margin. The REC1 spike measures churn against bytes and pop-in, and sets them.
- **R2-INT-OQ8. The Spy's rules (LOOP_V2 §5 [OPEN]: what breaks the disguise, how defenders are warned)** decide
  which fields are disguise and which truth. Settled when the Spy is designed (LATER).

## 6. Sources

**Sources read**

| Id | Source | Version / date | Read |
|---|---|---|---|
| S1 | Quake III Arena source, `code/server/sv_snapshot.c`, https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/server/sv_snapshot.c | id Software GPL release, header "Copyright (C) 1999-2005 Id Software"; master branch | 2026-10-27 |
| S2 | Quake III Arena source, `code/qcommon/cm_test.c`, https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/qcommon/cm_test.c | same release; master | 2026-10-27 |
| S3 | Quake II source, `server/sv_send.c`, https://raw.githubusercontent.com/id-Software/Quake-2/master/server/sv_send.c | GPL, header "Copyright (C) 1997-2001 Id Software"; master | 2026-10-27 |
| S4 | Glenn Fiedler, "State Synchronization", https://gafferongames.com/post/state_synchronization/ | 2015-01-05 | 2026-10-27 |
| S5 | Glenn Fiedler, "Snapshot Compression", https://gafferongames.com/post/snapshot_compression/ | 2015-01-04 | 2026-10-27 |
| S6 | three.js, `src/cameras/PerspectiveCamera.js`, https://raw.githubusercontent.com/mrdoob/three.js/dev/src/cameras/PerspectiveCamera.js | dev branch; no revision shown in the file | 2026-10-27 |
| S7 | Source SDK 2013, `src/game/server/baseentity.cpp`, https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/baseentity.cpp | "Copyright Valve Corporation"; master; characters 0–200,000 of 273,620 read | 2026-10-27 |
| S8 | Source SDK 2013, `src/game/server/team.cpp`, https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/team.cpp | master | 2026-10-27 |
| S9 | Source SDK 2013 (TF2 code), `src/game/shared/tf/tf_player_shared.cpp`, https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/shared/tf/tf_player_shared.cpp | master; characters 0–100,000 of 451,076 read | 2026-10-27 |
| S10 | Source SDK 2013, `src/game/server/sendproxy.cpp`, https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/sendproxy.cpp | master | 2026-10-27 |
| S11 | Chrome DevTools, "Inspect JavaScript ArrayBuffer with the Memory inspector", https://developer.chrome.com/docs/devtools/memory-inspector | page updated 2024-09-06 | 2026-10-27 |
| S12 | Quake III Arena source, `code/game/g_utils.c`, https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/g_utils.c | GPL release; master | 2026-10-27 |
| S13 | Quake III Arena source, `code/game/g_combat.c`, https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/g_combat.c | GPL release; master | 2026-10-27 |
| S14 | Source SDK 2013, `src/game/server/hltvdirector.cpp`, https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/hltvdirector.cpp | master | 2026-10-27 |

Legacy inputs (repository, read 2026-10-27): `docs/LEGACY_INVENTORY.md` rows CD4, CD5, CD6, CD21, CS6, CS8, CS9, GW15,
GW16, GW17, GW20, GN4, GN8, GP4, GP6, GP11; `docs/LEGACY_TUNING.md` tables `T`, `TUNE`, `CAM_STYLES`, `JUG`,
`JUG_CANNON`, `VEHICLE_TYPES`, `UNIT_TYPES`; `shamble-game.js:2191-2200`, `:2819` (the camera boom).

**Sources not read**

| URL | Reason |
|---|---|
| https://developer.valvesoftware.com/wiki/Source_Multiplayer_Networking | HTTP 403 Forbidden from the server (2026-10-27), not retried |
| https://developer.valvesoftware.com/wiki/Networking_Entities | HTTP 403 Forbidden from the server (2026-10-27), not retried |
| https://www.gdcvault.com/play/1014345/I-Shot-You-First-Networking (Aldridge, Bungie, GDC 2011, Halo: Reach) | Redirects to https://gdconf.com/gdc-vault-maintenance/, outside the allow list; the talk is video only |
| Overwatch, GDC 2017 networking talk (Tim Ford), GDC Vault | Video only; GDC Vault was redirecting to an off-list maintenance page; not attempted |
| https://technology.riotgames.com/node/108 ("Demolishing Wallhacks with VALORANT's Fog of War") | Outside the allow list |
| https://www.riotgames.com/en/news/demolishing-wallhacks-valorants-fog-war | Outside the allow list |
| https://dev.epicgames.com/documentation/unreal-engine/actor-priority-in-unreal-engine | Outside the allow list |
| https://dev.epicgames.com/documentation/unreal-engine/API/Plugins/ReplicationGraph/UBasicReplicationGraph | Outside the allow list |
| https://docs.unrealengine.com/4.27/en-US/API/Plugins/ReplicationGraph/UBasicReplicationGraph/index.html | Outside the allow list |
| https://cedric-neukirchen.net/docs/multiplayer-compendium/actor-relevancy-and-priority | Outside the allow list (third-party) |
| https://threejs.org/docs/#api/en/cameras/PerspectiveCamera | Fetched, but the hash-routed page returned only the navigation index; S6 (the source file) was read instead |

## 7. Cross-unit notes

- **Snapshots, prediction, interpolation and lag compensation:** an entity entering a client's set has no baseline
  in that client's acked frame; how it is coded (static baseline as in Q3, F3, or full state) and how removal is
  signalled are that unit's. Record sizes (F19 uses 12–25 bytes as an illustration only) and the entity-id space
  (Q3 caps at 1,024, F2) are too. Lag compensation needs history for entities the shooter had; a shooter cannot
  hit what it was never sent.
- **Tick rate and bandwidth budgets:** REC4 needs the per-client byte budget and the snapshot rate. REC3's pass
  mark is a share of the tick budget. Whether one snapshot must fit in one datagram is that unit's (or the
  transport unit's) question.
- **WebTransport server stack and WebSocket fallback:** public and team events (REC5, REC6) ride the reliable
  stream; spatial state rides datagrams. Per-client filtering runs before encoding, so the transport never holds
  data a client must not get.
- **Rooms and private matches:** spectator slots, the spectator delay as room data (REC7), and whether a private
  match may turn hiding off for casters.
- **R1 (simulation core):** the visibility pass and relevance sets run on the fixed tick, iterate in a deterministic
  order (Q3 sorts by entity number, S1) and are checked by the state hash on Wasm and native.
- **R4 (physics and navigation):** enclosure areas and portals as map data, floors, the spatial index for rays and
  its cost (OQ5), and destructible walls that open portals.
- **R3 (rendering):** the client cutaway (GW17) must follow the same visibility rule as REC2; the look brief
  decides how it looks.
- **R6 (infrastructure) and the security-auditor:** the reader-client and byte-audit tests (REC2, REC5) are
  anti-cheat evidence; telemetry of relevance churn and budget overruns.
- **game-designer and Mustafa:** OQ1, OQ3, OQ4, OQ6, OQ8.
