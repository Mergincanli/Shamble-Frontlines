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
S6 three.js src/cameras/PerspectiveCamera.js (dev branch), https://raw.githubusercontent.com/mrdoob/three.js/dev/src/cameras/PerspectiveCamera.js
   (read 2026-10-27; branch dev, no revision number shown in the file excerpt). Tier: official (library source).
   Verbatim: fov property "The vertical field of view, from bottom to top of view, in degrees." getViewBounds():
   "Computes the 2D bounds of the camera's viewable rectangle at a given distance along the viewing direction."
   Sure: confirmed in the source (verbatim). -> legacy fov 15 is vertical; footprint arithmetic uses vertical fov.
   (threejs.org/docs/#api/en/cameras/PerspectiveCamera: hash-routed, fetch returned only the nav index: not usable.)

## RESUME 2026-10-27 21:24 UTC (coordinator message)
New rule: each finding a REC rests on carries the exact sentence(s) verbatim in notes and finding; claims seen only
as WebFetch paraphrase = "inferred", not "confirmed in the source". Window ends 23:10 UTC.
Verbatim status of S1-S5:
- S1: verbatim code lines recorded (SVF_BROADCAST "broadcast entities are always sent"; `if ( !CM_AreasConnected(
  clientarea, svEnt->areanum ) ) {` "blocked by a door"; `// this is a new entity, send it from the baseline`;
  `#define MAX_SNAPSHOT_ENTITIES 1024`; "if we are full, silently discard entities"; `if (~ent->r.singleClient &
  (1 << frame->ps.clientNum))` for SVF_CLIENTMASK; `if ( messageSize > 1500 ) {`; `rateMsec = ( messageSize +
  HEADER_RATE_BYTES ) * 1000 / rate;`) -> confirmed.
- S2: verbatim `if (cm.areas[area1].floodnum == cm.areas[area2].floodnum) {`; `cm.areaPortals[ area1 * cm.numAreas +
  area2 ]++;` / `--;`; then `CM_FloodAreaConnections ();` -> confirmed (constant-time compare; re-flood on change).
- S3: verbatim "MULTICAST_ALL	same as broadcast (origin can be NULL)"; "if (!CM_AreasConnected (area1, area2))";
  "// if the sound doesn't attenuate,send it to everyone"; "if (attenuation == ATTN_NONE)" "use_phs = false;";
  "if (channel & CHAN_RELIABLE)". PHS-mask test itself: paraphrase only -> inferred unless re-quoted.
- S4: verbatim "The first n objects in this sorted list are the objects you should send that frame."; "reset the
  priority accumulator to zero for objects that fit."; "leave the priority accumulator value alone for objects that
  didn't."; "I give them a higher priority of 100 while at rest objects have priority of 1."; "256kbit/sec";
  "a maximum of 64 state updates per-packet" -> confirmed.
- S5: verbatim "Cube n in snapshot 110 is the same as the baseline. One bit: Not changed!"; "around 15 kilobits
  per-second". Baseline-advance on ack: paraphrase -> inferred.
Next: Q2 PHS verbatim re-quote (optional), Source SDK 2013 ShouldTransmit (TF2), Chrome devtools Wasm memory
inspection, SourceTV delay (Valve wiki blocked -> search GitHub sdk for tv_delay), gdcvault Halo Reach / Overwatch.
S7 Source SDK 2013 (Valve), src/game/server/baseentity.cpp, https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/baseentity.cpp
   ("Copyright Valve Corporation, All rights reserved."; master; read 2026-10-27, chars 0-200000 of 273620). Tier:
   shipped game (Source engine SDK). Verbatim: CBaseEntity::ShouldTransmit: `int fFlags = DispatchUpdateTransmitState();`
   `if ( fFlags & FL_EDICT_PVSCHECK ) return FL_EDICT_PVSCHECK;` `else if ( fFlags & FL_EDICT_ALWAYS ) return
   FL_EDICT_ALWAYS;` `if ( pRecipientPlayer->GetTeam()->ShouldTransmitToPlayer( pRecipientPlayer, this ))`
   `// by default do a PVS check` `return FL_EDICT_PVSCHECK;`. UpdateTransmitState: EF_NODRAW without move child ->
   FL_EDICT_DONTSEND; `// Always send the world`; `// by default cull against PVS` -> FL_EDICT_PVSCHECK.
   SetTransmit: `// HLTV/Replay need to know if this entity is culled by PVS limits`; move-parents force children
   ("// Force our aiment and move parent to be sent."). The PVS test itself is in the (closed) engine.
   Sure: confirmed in the source (verbatim lines). -> Source's model: per entity, per recipient: always / PVS /
   never / custom, with a team hook to force sending to teammates; parent-child: sending a child forces its parent.
S8 Source SDK 2013, src/game/server/team.cpp, https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/team.cpp
   (master; read 2026-10-27). Tier: shipped game. Verbatim: "// Purpose: Teams are always transmitted to clients"
   `return SetTransmitState( FL_EDICT_ALWAYS );`; CTeam::ShouldTransmitToPlayer: "// Always transmit the observer
   target to players" `if ( pRecipient && pRecipient->IsObserver() && pRecipient->GetObserverTarget() == pEntity )`
   `return true;` else false. Sure: confirmed in the source (verbatim). -> In Source the team entity (team score
   etc.) goes to EVERY client: anything on it is public. Spectator: observer target always sent.
S9 Source SDK 2013 (TF2 game code), src/game/shared/tf/tf_player_shared.cpp, https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/shared/tf/tf_player_shared.cpp
   (master; read 2026-10-27, chars 0-100000 of 451076). Tier: shipped game (TF2). Verbatim send-table lines inside
   `BEGIN_SEND_TABLE_NOBASE( CTFPlayerShared, DT_TFPlayerShared )`: `SendPropInt( SENDINFO( m_nDisguiseTeam ), 3,
   SPROP_UNSIGNED ),` `SendPropInt( SENDINFO( m_nDisguiseClass ), 4, SPROP_UNSIGNED ),` `SendPropEHandle( SENDINFO(
   m_hDisguiseTarget ) ),` `SendPropInt( SENDINFO( m_iDisguiseHealth ), -1, SPROP_VARINT ),`. A separate
   "tfsharedlocaldata" table goes only to the owner via SendProxy_SendLocalDataTable (that it is owner-only: WebFetch
   paraphrase -> inferred). Sure: the disguise lines in the all-clients table: confirmed (verbatim); local-only proxy
   semantics: inferred. -> TF2 sends a disguised spy's disguise fields to every client (so enemy clients hold both
   the disguise and, by the base entity's team field, the truth: the true-team part NOT read -> unverified).
S10 Source SDK 2013, src/game/server/sendproxy.cpp, https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/sendproxy.cpp
   (master; read 2026-10-27). Tier: shipped game. Verbatim: "// Purpose: Proxy that only sends data to team members"
   `void* SendProxy_OnlyToTeam( ... CSendProxyRecipients *pRecipients, int objectID )` ...
   `pRecipients->ClearAllRecipients();` `for ( int i=0; i < pTeam->GetNumPlayers(); i++ )`
   `pRecipients->SetRecipient( pTeam->GetPlayer( i )->GetClientIndex() );`. Sure: confirmed (verbatim).
   -> Per-field team-only replication exists in a shipped engine: precedent for team-private economy fields.
S11 Chrome DevTools, "Inspect JavaScript ArrayBuffer with the Memory inspector", https://developer.chrome.com/docs/devtools/memory-inspector
   (page last updated 2024-09-06; read 2026-10-27). Tier: official (browser vendor docs). Verbatim: "Use the Memory
   inspector to inspect ArrayBuffer, TypedArray, and DataView memory in JavaScript"; "The Memory Inspector panel lets
   you inspect such objects in Wasm applications written in C++."; "Use the Memory inspector while you debug your web
   app for an efficient workflow." Sure: confirmed (verbatim). -> any player can read the client's JS/Wasm memory
   with the browser's built-in tools; data sent to the client is data the player can see.
- NOT READ: https://www.gdcvault.com/play/1014345/I-Shot-You-First-Networking (Aldridge, Bungie, GDC 2011, Halo:
  Reach networking): 302 redirect to https://gdconf.com/gdc-vault-maintenance/ (outside the allow list); the talk is
  a video in any case. Expect the same for Overwatch GDC 2017 (Tim Ford) -> list as "not read: video only / vault
  in maintenance".
- NOT READ: https://technology.riotgames.com/node/108 "Demolishing Wallhacks with VALORANT's Fog of War" (Riot,
  Paul Chamberlain): outside the allow list. (Search snippet only = lead, not a source; nothing rests on it.)
  Also https://www.riotgames.com/en/news/demolishing-wallhacks-valorants-fog-war: outside the allow list.
S12 Quake III Arena source, code/game/g_utils.c, https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/g_utils.c
   (GPL, Id Software 1999-2005; master; read 2026-10-27). Tier: shipped game. Verbatim G_TempEntity header: "Spawns
   an event entity that will be auto-removed" / "The origin will be snapped to save net bandwidth"; body:
   `e->s.eType = ET_EVENTS + event;` `SnapVector( snapped );		// save network bandwidth` `// find cluster for
   PVS` `trap_LinkEntity( e );`. G_Sound: `te = G_TempEntity( ent->r.currentOrigin, EV_GENERAL_SOUND );`.
   G_AddEvent: "// clients need to add the event in playerState_t instead of entityState_t". Sure: confirmed
   (verbatim). -> Q3 events (sounds, impacts) are short-lived entities that pass the same per-client PVS/area test as
   everything else; origin snapped (quantized) to save bandwidth.
S13 Quake III Arena source, code/game/g_combat.c, https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/g_combat.c
   (GPL; master; read 2026-10-27). Tier: shipped game. Verbatim, player_die: `// broadcast the death event to
   everyone` `ent = G_TempEntity( self->r.currentOrigin, EV_OBITUARY );` `ent->r.svFlags = SVF_BROADCAST;	// send
   to everyone`. ScorePlum: `// only send this temp entity to a single client` `plum->r.svFlags |=
   SVF_SINGLECLIENT;`. Sure: confirmed (verbatim). -> kill feed = broadcast event; personal score popups =
   single-client event. Note: the obituary carries the victim's origin -> broadcast reveals death positions.
   (that the obituary's position is used by clients: not read -> don't claim.)
Time 21:3x UTC. Next: Unreal docs search (not read list), then check hysteresis/relevancy source? then write draft.
- NOT READ (outside the allow list): Unreal Engine docs https://dev.epicgames.com/documentation/unreal-engine/actor-priority-in-unreal-engine,
  https://dev.epicgames.com/documentation/unreal-engine/API/Plugins/ReplicationGraph/UBasicReplicationGraph,
  https://docs.unrealengine.com/4.27/en-US/API/Plugins/ReplicationGraph/UBasicReplicationGraph/index.html;
  https://cedric-neukirchen.net/docs/multiplayer-compendium/actor-relevancy-and-priority (also third-party).
  Search snippets are leads only: nothing rests on them (NetCullDistanceSquared, NetPriority ratio, grid node).
S14 Source SDK 2013, src/game/server/hltvdirector.cpp, https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/hltvdirector.cpp
   (master; read 2026-10-27). Tier: shipped game. Verbatim: `static ConVar tv_delay( "tv_delay", "30", 0, "SourceTV
   broadcast delay in seconds", true, 0, true, HLTV_MAX_DELAY );` `// broadcast time is current time - delay time`
   `newBroadcastTick -= TIME_TO_TICKS( m_fDelay );` `// instant broadcast, no delay`. Director uses the delay to pick
   shots from events already happened. Sure: confirmed (verbatim). Purpose "against ghosting": NOT stated -> inferred.

## Analysis notes (for the draft)
- Pairs needing a visibility test (enemy pairs only; teammates always relevant): 24v24: 24*24=576 unordered;
  16v16v16: 3*16*16=768; 12v12v12: 3*144=432; 48 all-vs-all 1,128 (upper bound).
- Legacy JS hasLOS 2.7-5.7 us (220 statics, linear) -> 768 rays = 2.1-4.4 ms per full pass, single sample point;
  x samples (e.g. 3) -> 6-13 ms. At 20 Hz passes: 4-9 % (1 pt) of one core. Rust+BVH unmeasured -> spike.
- Bandwidth: uniform spread, relevance circle r=40 m (28 footprint + 12 margin) = 5,027 m2 / 93,744 = 5.4 % -> ~2.5 of
  47 others; worst case (all at centre) = all 47. Interest cuts the average ~an order of magnitude, not the peak.
  Peak is bounded only by priority/rate accumulator + per-client byte budget. Illustrative record 12-25 B (snapshot
  unit owns sizes): 60 entities x 12 B = 720 B, x 25 B = 1,500 B per snapshot.
- Margin: m >= v_max*(t_interp + RTT/2 + t_snapshot) ~ 10.2 m/s * (0.1+0.1+0.05) = 2.6 m for infantry at 200 ms RTT;
  camera pan (ADS pan 16 m in style 5) and yaw rotation matter more -> circle around the look point + aim offset.
- Camera model question: (1) camera vision (what the overhead camera could show: roofs hide interiors, hidden floors)
  vs (2) character LOS fog. Design/look choice -> OQ for Mustafa (game-designer). Tech supports both; enclosure
  (area/portal) test needed in both; rays only in (2) or for windows/doors.
Next: write the draft (time ~21:42 UTC).
- 21:45 UTC: writing draft now (all sources S1-S14 summarized above; analysis notes done).
