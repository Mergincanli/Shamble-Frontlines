Task: R2 unit interest (interest management for a top-down camera), v2 phase 0 research wave 1, netcode-architect. Started 2026-10-27.

## Progress log
- Read: plan.md, plan-amendments.md, brief §0/§4/§5, VISION_V2 whole, LOOP_V2 whole, research README.
- VISION §6 items for my unit: §6.1 modes as data (2/3 teams any size; rooms, private matches -> spectators),
  §6.2 3D sim (floors, interiors, ledges, LOS across heights), §6.4 vehicles big multi-part (marking!), §6.5 economy
  and objectives server-side, §6.8 moments (kill-cam LATER), §6.9 spectacle (few very large units).
  §3 scale 2-8 / 6-18 start, goal 12v12v12 or 24v24; §4 fair to ~100 ms; one EU region.
- LOOP_V2 knowledge facts: cache spawn "everyone is told where" (§4.1, public); uplink steal by holding E (public
  state); team tech = team total, tiers (privacy not stated -> OQ); personal $ (private; harvester rebuild share shown
  per player §3); Spy disguise as enemy + hack refinery (§5, LATER, warning to defenders open); Hammer leech (§6).

## Legacy facts (done)
- CD5/GP4/CAM_STYLES: default cam style 3: camHeight 40, camBack 56.6 (horizontal boom: game:2819
  camDesired = anchor - fwd*camBack, y+camHeight), fov 15 (three.js vertical fov), yaw 45 deg + middle-mouse yaw,
  ADS 'pitch' (style 5 'pan', foxPanMax 16 m), look-ahead. Look brief pending -> parameter.
  My arithmetic (inferred): D=69.3 m, pitch 35.25 deg; ground footprint at 16:9 = trapezoid near width 27.6 m,
  far width 40.2 m, depth 32.75 m (13.3 m toward cam, 19.4 m beyond look point), area ~1,110 m2; farthest corner
  27.95 m from look point -> circle r~28 m covers any yaw. Other styles: 1 (24/16, fov 52), 2 (15.5/10.5, fov 52),
  4 (41.5/65.5 fov 12), 5 (52/42 fov 18, pan ADS).
- CD21: ground 372 x 252 m (arenaX 186, arenaZ 126) = 93,744 m2. Circle r=28 -> 2,463 m2 = 2.6 %; r=50 -> 8.4 %.
- CD4: walk 6.3 m/s, sprintMult 1.4286 (T) / per class up to 1.62 (UNIT_TYPES) -> ~9-10.2 m/s. JUG speed 3.2,
  maxSpeed 6, pilot moveSpeed 5.5; JUG_CANNON range 130 m (>> screen radius), JUG targetH 5.6 m (big unit).
- CD6/GN4: bot sight 44 m (enemyDetect), LOS budget 480/s shared, retarget 0.25 s, recheck 0.1 s; demand ~1,100/s
  at 48 (LOS-SIZE open). Eyes at +1.45 m.
- CS6/CS9: rayStatics linear over all statics (220 in test), no broad phase; hasLOS 2.7-5.7 us/call in JS (B7);
  statics only (soldiers never block); 0.5 m end margin. Inventory says Rust sim needs spatial index.
- CS8/GW16: legacy SIM world = players + kinded static boxes w/ alive flag.
- GW15: blasts use scene raycasts vs bullets SIM statics (two LOS worlds) -> one LOS query in v2.
- GW17: occlusion cutaway: roof fades while the local player is under it; structures between cam and player fade
  to 24 %; client-only cosmetic. -> other buildings' roofs stay opaque: enemies inside are hidden on screen but would
  be in the snapshot under camera-rect-only interest (wallhack leak).
- GW20: enterable barracks (5 walls, door gap, roof); LOOP_V2 §2: enemies can't enter barracks interior.
- GN8: bot muzzle/tracer/impact culled beyond 50 m from camera look point (botCullR), sound beyond 90 m (botSfxR);
  shots at/hitting you never culled. Sound radius ~1.8x visual radius in legacy.
- GP6: positional enemy shots, inverse-distance gain; no occlusion; no voice cap.
- GP10/GP11: HUD/scoreboard; scoreboard shows kills/deaths, alive count and tier per side (hostile tier shown!
  GP11 "each side's alive count and tier") -> legacy shows enemy tier publicly.
- No minimap row in the inventory (grep "minimap|radar" no hit) -> v2 minimap is new.
Next: sources. Plan: Valve wiki (Source MP Networking, Networking Entities, PVS, SourceTV), Q3 sv_snapshot.c +
cm_test (areas), Q2 SV_Multicast (PHS), Gaffer State Synchronization (priority accumulator), Snapshot Compression,
three.js PerspectiveCamera doc, Chrome devtools Wasm debugging (client memory readable), TF2 SDK spy disguise,
gdcvault Halo Reach / Overwatch (probably video only).
- NOT READ: https://developer.valvesoftware.com/wiki/Source_Multiplayer_Networking and /wiki/Networking_Entities:
  HTTP 403 Forbidden from the server (twice, 2026-10-27); not retried. Use Source SDK 2013 code on GitHub instead.

## Sources read
S1 Quake III Arena source, code/server/sv_snapshot.c, https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/server/sv_snapshot.c
   (id Software, GPL release; header "Copyright (C) 1999-2005 Id Software"; repo master; read 2026-10-27). Tier: shipped game.
   SV_AddEntitiesVisibleFromPoint, per entity, in order: skip !linked; SVF_NOCLIENT skip; SVF_SINGLECLIENT (only
   r.singleClient); SVF_NOTSINGLECLIENT (all but one); SVF_CLIENTMASK (bitmask of up to 32 clients, errors if
   clientNum >= 32); no double add via portals (snapshotCounter); SVF_BROADCAST always sent; area check
   CM_AreasConnected(clientarea, areanum / areanum2) "blocked by a door"; PVS cluster bit test vs CM_ClusterPVS(
   clientcluster); SVF_PORTAL recursion from origin2 (with generic1 distance cull). areabits written per frame.
   SV_EmitPacketEntities: new entity in snapshot -> "send it from the baseline" (MSG_WriteDeltaEntity vs
   sv.svEntities[n].baseline); removed -> delta to NULL; unchanged entity emits no bytes. qsort by entity number;
   MAX_SNAPSHOT_ENTITIES 1024, "if we are full, silently discard entities". Rate: SV_RateMsec caps message 1500,
   HEADER_RATE_BYTES 48, rateMsec = (size+48)*1000/rate; next snapshot delayed by rate (rateDelayed) but never sooner
   than snapshotMsec. Sure: confirmed in the source (via WebFetch summary with verbatim quotes).
   No priority/accumulator: Q3 sends every relevant entity each snapshot and throttles the whole snapshot by rate.
S2 Quake III Arena source, code/qcommon/cm_test.c, https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/qcommon/cm_test.c
   (same release, master; read 2026-10-27). Tier: shipped game. Area connectivity precomputed by flood fill
   (CM_FloodAreaConnections -> CM_FloodArea_r following open portals, con[i] > 0); CM_AreasConnected is a
   constant-time compare of floodnum. Doors: CM_AdjustAreaPortalState ref-counts cm.areaPortals[a1*numAreas+a2]
   (++ open, -- close, error on negative) then re-floods all areas. CM_WriteAreaBits ORs bits of areas in the same
   flood (several viewpoints combine). CM_ClusterPVS = pointer into precomputed vis data (all visible if map not vised).
   Sure: confirmed in the source (summary with short verbatim excerpts).
   -> For v2: building interiors/floors as areas, doors/openings as portals: O(1) per pair relevance test, recompute
   only when a door/wall state changes (destructible buildings!).
S3 Quake II source, server/sv_send.c, https://raw.githubusercontent.com/id-Software/Quake-2/master/server/sv_send.c
   ("Copyright (C) 1997-2001 Id Software", GPL v2+; master; read 2026-10-27). Tier: shipped game.
   SV_Multicast: MULTICAST_ALL (everyone, origin may be NULL), MULTICAST_PHS (clients whose cluster is in the
   origin cluster's potentially hearable set), MULTICAST_PVS (potentially visible set); for PHS/PVS clients in
   unconnected areas (CM_AreasConnected false, i.e. behind a closed door) are skipped; _R variants go on the reliable
   stream. SV_StartSound: positioned sounds sent via PHS multicast; ATTN_NONE (no attenuation: global chatter,
   voiceovers) -> "send it to everyone"; CHAN_RELIABLE -> reliable variant. Sure: confirmed in the source (summary
   with short verbatim excerpts).
   -> Precedent: sound events have their OWN (larger) relevance set than visual entities; global announcements go to
   all on the reliable channel.
S4 Glenn Fiedler, "State Synchronization", Gaffer On Games, 2015-01-05, https://gafferongames.com/post/state_synchronization/
   (read 2026-10-27). Tier: other (respected practitioner article; author shipped netcode). Section "Priority
   accumulator": each frame compute per-object priority (player 1,000,000 so always sent; interacting 100; at rest
   1), add to a per-object float accumulator carried frame to frame, sort descending, walk the list filling the
   packet up to a byte budget (example target 256 kbit/s, adjustable on the fly; skip an update that doesn't fit and
   try the next), max 64 state updates per packet, 901 cubes; reset the accumulator of sent objects to 0, leave the
   others (so they rise to the front). Without accumulation, low-priority objects would never be sent. Sure:
   confirmed in the source.
S5 Glenn Fiedler, "Snapshot Compression", Gaffer On Games, 2015-01-04, https://gafferongames.com/post/snapshot_compression/
   (read 2026-10-27). Tier: other (practitioner). Delta vs last acked baseline (baseline ~1 RTT old); unchanged
   object = 1 bit; before first ack, encode vs known initial state; changed-index list when <= 90 changed else per-cube
   bits; 1-bit not-changed for 901 cubes at 60 Hz costs ~54 kbit/s; stationary steady state ~15 kbit/s, mostly
   headers. Mostly the snapshots unit's; for me: an entity's absence from the baseline means its first appearance
   costs a full (or initial-state-relative) record -> churn at the relevance edge costs bytes. Sure: confirmed.
