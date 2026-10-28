# R2: Netcode

- **Topic:** R2, netcode (`docs/V2_PHASE0_BRIEF.md` §4, row R2).
- **Author agent:** netcode-architect. Units (wave 1 of `.checkpoints/v2-research/plan.md`, one netcode-architect agent
  each): `sync`, `budgets`, `interest`, `transport`, `rooms`. Assembly: netcode-architect (wave 1, step 2 "Assemble":
  nothing new is claimed; where units disagree, both findings stay and the conflict is an open question).
- **Date:** 2026-10-27 (every source below was read that day).
- **Status:** draft: verification pending.
- **The brief's questions for this topic (quoted, `docs/V2_PHASE0_BRIEF.md` §4, row R2):** "Snapshots, client
  prediction, interpolation and lag compensation for 48+ players; tick rate and bandwidth budgets; interest management
  for a top-down camera; the WebTransport server stack in Rust plus the WebSocket fallback; rooms and private matches;
  learn from shipped games (Valve's Source networking, Overwatch's GDC talks, Glenn Fiedler's articles, Quake 3)"

| # | Brief question | Unit | Ids | Draft and notes (`.checkpoints/v2-research/`) |
|---|---|---|---|---|
| 1 | Snapshots, client prediction, interpolation and lag compensation for 48+ players | `sync` | R2-SYNC-F1–F35, REC1–REC9, OQ1–OQ14 | `netcode-architect-r2-sync-draft.md`, `netcode-architect-r2-sync.md` |
| 2 | Tick rate and bandwidth budgets | `budgets` | R2-BUD-F1–F35, REC1–REC6, OQ1–OQ11 | `netcode-architect-r2-budgets-draft.md`, `netcode-architect-r2-budgets.md` |
| 3 | Interest management for a top-down camera | `interest` | R2-INT-F1–F20, REC1–REC7, OQ1–OQ8 | `netcode-architect-r2-interest-draft.md`, `netcode-architect-r2-interest.md` |
| 4 | The WebTransport server stack in Rust plus the WebSocket fallback | `transport` | R2-TRN-F1–F39, REC1–REC6, OQ1–OQ9 | `netcode-architect-r2-transport-draft.md`, `netcode-architect-r2-transport.md` |
| 5 | Rooms and private matches | `rooms` | R2-ROOM-F1–F18, REC1–REC8, OQ1–OQ10 | `netcode-architect-r2-rooms-draft.md`, `netcode-architect-r2-rooms.md` |
| 6 | Learn from shipped games (Valve's Source networking, Overwatch's GDC talks, Glenn Fiedler's articles, Quake 3) | all five | an index under Findings | – |
| – | Conflicts and gaps between the units (added at assembly) | – | R2-XU-OQ1–OQ18 | – |

Totals: 147 findings, 36 recommendations, 70 open questions (52 from the units, 18 added at assembly), 113 sources
read (131 entries across the units; 18 were a source another unit had also read), 26 sources not read.

**How to read this file.**
- The findings, options, recommendations and open questions are copied from the unit drafts unchanged, ids included.
  The assembly adds only this header, the TL;DR, the index for brief question 6, the cross-unit open questions
  (`R2-XU-OQ1`–`R2-XU-OQ18`: conflicts and gaps between units), the pointers, the list of recommendations that hinge on
  a source not read, and the merged source lists. A remark by the assembler inside copied text is marked
  "[Assembler's note: …]".
- Inside a unit's section, a bare id (F12, REC3, OQ2) is that unit's id: in the sync unit's text, F12 means R2-SYNC-F12.
- Option labels are local to each unit. Two of them clash: the sync unit's network profiles P0–P4 and B1 (defined just
  before its recommendations) are not the budgets unit's packet-size options P1–P4, and the transport unit's option
  groups Q1–Q6 are not the brief questions 1–6 above.
- The units name each other by subject. "The tick-rate unit", "the bandwidth unit", "the tick and bandwidth unit" and
  "the tick-rate and bandwidth unit" are budgets. "The snapshots unit", "the snapshot-model unit", "the prediction
  unit" and "the snapshot, prediction and lag-compensation unit" are sync. "The transport unit" and "the stack unit"
  are transport. "R1's tick unit", "R1's determinism unit" and "R1's data unit" are units of R1 (simulation core).
- Each unit says near the top of its section what its certainty marks mean (confirmed in the source · inferred ·
  unverified); the sync and budgets units also define their source tiers there (official · shipped game · other). The
  units do not tier Glenn Fiedler's articles alike (R2-XU-OQ16).
- "The netcode default principles" are the starting design in `.claude/agents/netcode-architect.md`, which an ADR may
  change; they are not VISION_V2 targets.
- Legacy ids (CS1, GC5, CD5, …) are rows of `docs/LEGACY_INVENTORY.md`; `T`, `WEAPONS` and the like are tables of
  `docs/LEGACY_TUNING.md`. The inventory's marks are working recommendations (the Advisor's R7-MARKS (a)). Rows marked
  "look brief pending" stay pending: the camera framing and the client cutaway enter the interest unit only as
  parameters, and nothing here decides the look.
- Every pass mark and every reference machine is proposed. No spike was run and no engine code was written; nothing
  here is decided. Notes ids (S-Q1, S-GF3, …) point into the unit's notes file; the interest unit's S1–S14 are source
  ids, shown in the Units column of the source tables.

## TL;DR

- Found: 147 findings from 113 sources (specifications, the Quake III and Source SDK 2013 code, Fiedler's articles).
  Overwatch's talks (video only), Valve's wiki (HTTP 403) and off-list pages went unread; 7 recommendations touch them.
- Recommended (marks proposed): 30 Hz delta snapshots as datagrams, a 60 Hz tick if R1's CPU mark holds, own
  prediction, a capped rewind, camera-based relevance, quinn plus WebSocket, one process per room, signed join tokens.
- Open: 18 conflicts and gaps between the units (R2-XU-OQ1–OQ18), such as the rewind cap (200 or 250 ms).

## Findings

Grouped by the brief's questions. Each unit's findings are copied from its draft unchanged, followed by the legacy facts it used.

### Brief question 1: snapshots, client prediction, interpolation and lag compensation for 48+ players (sync unit)

Unit **sync**. Draft: `.checkpoints/v2-research/netcode-architect-r2-sync-draft.md`; notes with every source summary and its exact quotes: `.checkpoints/v2-research/netcode-architect-r2-sync.md`. Author: netcode-architect (Opus 5.5, effort xhigh), 2026-10-27.

**How to read the certainty marks.** "Confirmed in the source" means the source's own words (quoted in the finding)
say it. "Inferred" means my reading, my arithmetic, or a point I could only see in the fetch tool's paraphrase.
**Source tiers in this unit.** No official document or specification describes game-state synchronisation; the one
official source here is RFC 9221 (QUIC datagrams). The shipped-game tier is the released source code of two shipped
games: Quake III Arena (id Software's GPL release) and the Source SDK 2013 (Valve), read on github.com and
raw.githubusercontent.com. I read the code because the write-ups of Quake 3's model are off the allow list and Valve's
wiki pages refused the fetch (HTTP 403). Glenn Fiedler's articles are tier "other" (practitioner articles the brief
names). The Overwatch GDC talks are video only and the GDC Vault redirected to an off-list page: not read, and no
recommendation rests on them.

#### State-sync model

**R2-SYNC-F1.** In peer-to-peer deterministic lockstep, every player gets the latency of the most-lagged player.
· Source: https://gafferongames.com/post/what_every_programmer_needs_to_know_about_game_networking/ (the part on
peer-to-peer lockstep; page dated 2010-02-24) · Tier: other · Read 2026-10-27 · Confirmed in the source: "This means that each player
in the game has latency equal to the most lagged player."

**R2-SYNC-F2.** Joining a lockstep game in progress is rare, because it needs a complete deterministic starting state.
· Source: same page, same part · Tier: other · Read 2026-10-27 · Confirmed in the source: "this is not common due to
the difficulty of capturing and transmitting a completely deterministic starting point".

**R2-SYNC-F3.** Lockstep needs results that match to the bit, and floating-point behaviour differs between compilers,
operating systems and instruction sets.
· Source: https://gafferongames.com/post/deterministic_lockstep/ (page dated 2014-11-29) · Tier: other · Read 2026-10-27
· Confirmed in the source: "Exact down to the bit-level." and "Differences in floating point behavior between compilers,
OS's and even instruction sets".

**R2-SYNC-F4.** A lockstep simulation cannot step a frame until it has every input for that frame, and over TCP each
lost packet stalls it.
· Source: same page · Tier: other · Read 2026-10-27 · Confirmed in the source: "the simulation on the right can only
simulate frame n when it has the input for that frame." and "each time a packet is lost, TCP has to wait RTT*2 while it
is resent."

**R2-SYNC-F5.** Snapshot interpolation runs no simulation on the receiver, so it needs no determinism. Fiedler names it
as the technique for simulations that are not deterministic or for higher player counts.
· Source: https://gafferongames.com/post/snapshot_interpolation/ (page dated 2014-11-30, series "Networked Physics") ·
Tier: other · Read 2026-10-27 · Confirmed in the source: "So if your simulation is not deterministic or you want higher
player counts then you need a different technique." and "snapshot interpolation doesn't run any simulation on the right
side at all!"

**R2-SYNC-F6.** State synchronisation (both sides simulate, the sender sends input and state) is approximate, and its
cost is time spent hunting divergence and pops. Fiedler's fix for the two sides drifting apart is to quantise the
state on both sides.
· Source: https://gafferongames.com/post/state_synchronization/ (page dated 2015-01-05) · Tier: other · Read 2026-10-27 ·
Confirmed in the source: "we don't just send input, we send both input and state."; "State synchronization is an
approximate and lossy synchronization strategy."; "In practice, this means you'll spend a lot of time tracking down
sources of extrapolation divergence and pops."; "The solution is to quantize the state on both sides."

**R2-SYNC-F7.** Delta compression against an acknowledged baseline: the receiver acks the most recent snapshot it got;
the sender moves its baseline forward only on a newer ack, so the baseline is about one round trip old; before any ack
it encodes against the known initial state; an unchanged object costs one bit.
· Source: https://gafferongames.com/post/snapshot_compression/ (page dated 2015-01-04) · Tier: other · Read 2026-10-27 ·
Confirmed in the source: "the most recent snapshot I have received is snapshot n"; "The sender takes this most recent
ack and if it is more recent than the previous ack updates the baseline snapshot to this value."; "a baseline that is
roughly RTT (round trip time) in the past."; "this snapshot is encoded relative to the initial state of the
simulation."; "Cube n in snapshot 110 is the same as the baseline. One bit: Not changed!"

**R2-SYNC-F8.** Quake III shipped this model. The server keeps a ring of the last 32 frames it sent each client and
deltas the new snapshot from the frame that client last reported receiving. It sends a full (non-delta) snapshot when
the client has no valid frame, when the reported frame is 29 or more sequences old, or when that frame's entities have
been overwritten in the server's entity buffer. Unchanged entities cost no bytes. A new entity is sent from its
per-entity baseline, and a removed entity as a delta to nothing. A snapshot holds at most 1024 entities.
· Source: https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/server/sv_snapshot.c
(SV_WriteSnapshotToClient, SV_EmitPacketEntities) and .../code/qcommon/qcommon.h (PACKET_BACKUP); id Software GPL source
release, master (one commit, no version shown) · Tier: shipped game · Read 2026-10-27 · Confirmed in the source:
`#define PACKET_BACKUP 32` with "number of old messages that must be kept on client and server for delta compression and
ping estimation" (the source spells it "comrpession"); `} else if ( client->netchan.outgoingSequence - client->deltaMessage >= (PACKET_BACKUP - 3) ) {`
"// client hasn't gotten a good message through in a long time" ("Delta request from out of date packet."); "// we have
a valid snapshot to delta from" `oldframe = &client->frames[ client->deltaMessage & PACKET_MASK ];`; "// the snapshot's
entities may still have rolled off the buffer, though"; "// because the force parm is qfalse, this will not result // in
any bytes being emited if the entity has not changed at all"; "// this is a new entity, send it from the baseline"; "//
the old entity isn't present in the new message" `MSG_WriteDeltaEntity (msg, oldent, NULL, qtrue );`;
`#define MAX_SNAPSHOT_ENTITIES 1024`.

**R2-SYNC-F9.** In Quake III, every client packet carries the sequence number of the last server message it received
(the delta baseline), and the client asks for a non-delta snapshot when it no longer holds the snapshot it would delta
from.
· Source: https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/client/cl_input.c (CL_WritePacket) ·
Tier: shipped game · Read 2026-10-27 · Confirmed in the source: "// write the last message we received, which can // be
used for delta compression, and is also used // to tell if we dropped a gamestate" `MSG_WriteLong( &buf, clc.serverMessageSequence );`
and `if ( cl_nodelta->integer || !cl.snap.valid || clc.demowaiting || clc.serverMessageSequence != cl.snap.messageNum ) { MSG_WriteByte (&buf, clc_moveNoDelta);`.

**R2-SYNC-F10.** Quake III's shipped defaults: 20 snapshots per second requested ("snaps"), a rate of 3000 bytes per
second, at most 30 client packets per second, and each packet repeating the commands of 1 earlier packet.
· Source: https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/client/cl_main.c (CL_Init cvar
registration) · Tier: shipped game · Read 2026-10-27 · Confirmed in the source: `Cvar_Get ("snaps", "20", CVAR_USERINFO | CVAR_ARCHIVE );`,
`Cvar_Get ("rate", "3000", CVAR_USERINFO | CVAR_ARCHIVE );`, `cl_maxpackets = Cvar_Get ("cl_maxpackets", "30", CVAR_ARCHIVE );`,
`cl_packetdup = Cvar_Get ("cl_packetdup", "1", CVAR_ARCHIVE );`. That "rate" is in bytes per second is inferred from
SV_RateMsec (S-Q1 notes), not quoted.

**R2-SYNC-F11.** QUIC datagrams, which WebTransport datagrams use, are never retransmitted. They can arrive reordered,
can be dropped by the receiver, and cannot be fragmented, so each must fit in one QUIC packet.
· Source: https://www.rfc-editor.org/rfc/rfc9221.html (RFC 9221, March 2022, Standards Track; §5.2, §5.3, §5) · Tier:
official · Read 2026-10-27 · Confirmed in the source: "Although DATAGRAM frames are not retransmitted upon loss
detection, they are ack-eliciting"; "Due to reordering, this can include a DATAGRAM frame that was thought to be lost";
"they MAY be dropped by the receiver if the receiver cannot process them"; "DATAGRAM frames cannot be fragmented;
therefore, application protocols need to handle cases". The link from WebTransport datagrams to RFC 9221 is the
transport unit's to confirm (inferred here).

#### Client prediction and reconciliation

**R2-SYNC-F12.** Client-side prediction keeps a circular buffer of the local player's past states and inputs. When a
correction arrives, the client rewinds to it and replays the newer inputs. What the player's own input did not cause
(being hit by a rocket) cannot be predicted.
· Source: https://gafferongames.com/post/what_every_programmer_needs_to_know_about_game_networking/ (the part on
client-side prediction) · Tier: other · Read 2026-10-27 · Confirmed in the source: "keep a circular buffer of past
character state and input for the local player on the client"; "In effect the client invisibly "rewinds and replays"
the last n frames of local player character movement"; "Only in the rare case, such as a player getting hit by a
rocket". Also: "As Tim Sweeney writes in The Unreal Networking Architecture: "The Server Is The Man"."

**R2-SYNC-F13.** On every frame, Quake III re-runs all unacknowledged commands on top of the newest player state from
the server, using the same movement code (Pmove) as the server. Its own comment says that means "quite a few pmoves"
per frame, and suggests skipping the replay when the snapshot matches the prediction.
· Source: https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/cgame/cg_predict.c
(CG_PredictPlayerState and its header comment) · Tier: shipped game · Read 2026-10-27 · Confirmed in the source: "For
normal gameplay, it will be the result of predicted usercmd_t on top of the most recent playerState_t received from the
server."; "Each new snapshot will usually have one or more new usercmd over the last, but we simulate all
unacknowledged commands each time, not just the new ones."; "This means that on an internet connection, quite a few
pmoves may be issued each frame."; "OPTIMIZE: don't re-simulate unless the newly arrived snapshot playerState_t differs
from the predicted one."; `Pmove (&cg_pmove);`.

**R2-SYNC-F14.** Quake III stops predicting when its command buffer no longer reaches back to the newest snapshot, and
holds the last good prediction.
· Source: same file · Tier: shipped game · Read 2026-10-27 · Confirmed in the source:
`if ( oldestCmd.serverTime > cg.snap->ps.commandTime && oldestCmd.serverTime < cg.time ) {` …
`CG_Printf ("exceeded PACKET_BACKUP on commands\n");` … `return;`.

**R2-SYNC-F15.** Quake III smooths prediction errors. An error over 0.1 units is added to an error vector that fades out
linearly over `cg_errordecay` milliseconds, 100 by default, "to ease the jerk".
· Source: cg_predict.c (as above) and https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/cgame/cg_main.c
(cvar table) · Tier: shipped game · Read 2026-10-27 · Confirmed in the source: "We detect prediction errors and allow
them to be decayed off over several frames to ease the jerk."; `if ( len > 0.1 ) {`;
`f = ( cg_errorDecay.value - t ) / cg_errorDecay.value;`; `VectorScale( cg.predictedError, f, cg.predictedError );`;
`{ &cg_errorDecay, "cg_errordecay", "100", 0 },`. That the drawn view adds the decaying error is inferred: the code that
applies it to the view is in another file I did not read.

**R2-SYNC-F16.** Fiedler's 2004 rule of thumb for corrections: snap when the error is over 2.0 m, otherwise move 10 % of
the way each update; snap derived values such as velocity instead of smoothing them. The server ignores input older than
its current time.
· Source: https://gafferongames.com/post/networked_physics_2004/ (page dated 2004-09-04) · Tier: other · Read
2026-10-27 · Confirmed in the source: `if ( distance > 2.0f )` `current.position = state.position;`; "move 10% of the
distance between the current position and the correct position"; "directly snapping derivative quantities such as
velocity"; "ignores any input received with a time value less than the current time"; "the client rewinds to the
corrected physics state and time and replays the stored moves".

**R2-SYNC-F17.** Quake III also predicts item pickups (as a "predictable event", hiding the item at once) and jump
pads, with item prediction on by default. Other entities are not simulated on the client: the predicted player collides
against the solid entities of the current snapshot.
· Source: cg_predict.c (CG_TouchItem, CG_TouchTriggerPrediction, CG_BuildSolidList) and cg_main.c · Tier: shipped game
· Read 2026-10-27 · Confirmed in the source: `{ &cg_predictItems, "cg_predictItems", "1", CVAR_ARCHIVE },`;
"BG_AddPredictableEventToPlayerstate( EV_ITEM_PICKUP, cent->currentState.modelindex , &cg.predictedPlayerState);";
"// remove it from the frame so it won't be drawn" "cent->currentState.eFlags |= EF_NODRAW;"; "// never pick an item up
twice in a prediction"; "BG_TouchJumpPad( &cg.predictedPlayerState, ent );"; "When a new cg.snap has been set, this
function builds a sublist of the entities that are actually solid, to make for more efficient collision detection"
"cg_solidEntities[cg_numSolidEntities] = cent;".

**R2-SYNC-F18.** Input redundancy. Each Quake III packet carries every command made since the packet `cl_packetdup`+1
packets back (cl_packetdup is 1 by default and clamped to 0..5), at most 32 commands, each delta-coded against the one
before. Fiedler's lockstep sends every unacknowledged input until it is acknowledged.
· Sources: cl_input.c (CL_WritePacket), qcommon.h, cl_main.c; https://gafferongames.com/post/deterministic_lockstep/ ·
Tier: shipped game and other · Read 2026-10-27 · Confirmed in the source:
`oldPacketNum = (clc.netchan.outgoingSequence - 1 - cl_packetdup->integer) & PACKET_MASK;`
`count = cl.cmdNumber - cl.outPackets[ oldPacketNum ].p_cmdNumber;`; `MSG_WriteDeltaUsercmdKey (&buf, key, oldcmd, cmd);`
`oldcmd = cmd;`; `#define MAX_PACKET_USERCMDS 32`; Fiedler: "redundantly include all inputs in each UDP packet until we
know for sure the other side has received them."

**R2-SYNC-F19.** A Quake III command carries the client's server time ("so the amount of movement can be determined
without allowing cheating") and its view angles as 16-bit values (65,536 steps per turn), stored in int fields.
· Source: cl_input.c (CL_FinishMove) · Tier: shipped game · Read 2026-10-27 · Confirmed in the source: "// send the
current server time so the amount of movement // can be determined without allowing cheating"
`cmd->serverTime = cl.serverTime;`; `cmd->angles[i] = ANGLE2SHORT(cl.viewangles[i]);`; and from
https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/q_shared.h
`#define ANGLE2SHORT(x) ((int)((x)*65536/360) & 65535)` and the struct `usercmd_t` { `int serverTime;`
`int angles[3];` `int buttons;` `byte weapon;` `signed char forwardmove, rightmove, upmove;` }.

**R2-SYNC-F20.** A Source command carries the tick the client made it on, and a seed "for shared random functions". The
server side has a second seed that only the server fills.
· Source: https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/shared/usercmd.h (class
CUserCmd; Source SDK 2013, master, no version shown) · Tier: shipped game · Read 2026-10-27 · Confirmed in the source:
"// the tick the client created this command" `int tick_count;`; `int random_seed;` "// For shared random functions";
`int server_random_seed;` "// Only the server populates this seed"; "// Client only, tracks whether we've predicted
this command at least once" `bool hasbeenpredicted;`. That the server seed exists so clients cannot steer spread is
inferred; the source does not say why.

#### Entity interpolation and clock

**R2-SYNC-F21.** Fiedler's rule for the interpolation buffer: enough delay to lose two packets in a row. At 2–5 % loss,
about 3 send intervals works best: 300 ms at 10 packets per second (350 ms with jitter), 150 ms at 30, 85 ms at 60.
· Source: https://gafferongames.com/post/snapshot_interpolation/ · Tier: other · Read 2026-10-27 · Confirmed in the
source: "My rule of thumb is that the interpolation buffer should have enough delay so that I can lose two packets in a
row"; "Experimentally I've found that the amount of delay that works best at 2-5% packet loss is 3X the packet send
rate."; "At 10 packets per-second this is 300ms."; "If we send 30 snapshots per-second we can get the same amount of
packet loss protection with a delay of 150ms."; "60 packets per-second needs only 85ms." Reading the 150 ms and 85 ms as
3 intervals plus a jitter margin is inferred.

**R2-SYNC-F22.** Extrapolation tracks non-linear motion badly. Hermite interpolation (which uses velocity) removed the
artefacts that linear interpolation showed at 10 packets per second.
· Source: same page · Tier: other · Read 2026-10-27 · Confirmed in the source: "extrapolation doesn't work very well for
rigid bodies because their motion is non-linear and unpredictable."; "I can no longer see any artifacts."

**R2-SYNC-F23.** Quake III steers its clock to approach, but not pass, the newest snapshot, and draws players between
the previous and the newest snapshot (so under one snapshot interval of delay; inferred from the clock rule and F25). It resets the clock when the error
exceeds 500 ms, halves an error over 100 ms, and otherwise drifts 1–2 ms per frame. It extrapolates when the newest
snapshot is late, and a user setting nudges the delay by up to ±30 ms.
· Source: https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/client/cl_cgame.c (CL_AdjustTimeDelta,
CL_SetCGameTime) and .../code/cgame/cg_snapshot.c (CG_ProcessSnapshots) · Tier: shipped game · Read 2026-10-27 ·
Confirmed in the source: "Our ideal time would be to have the adjusted time approach, but not pass, the very latest
snapshot."; `#define RESET_TIME 500`; "// fast adjust, cut the difference in half"; "// slow drift adjust, only move 1
or 2 msec"; `if ( cls.realtime + cl.serverTimeDelta >= cl.snap.serverTime - 5 ) { cl.extrapolatedSnapshot = qtrue;`;
"// cl_timeNudge is a user adjustable cvar that allows more // or less latency to be added in the interest of better
// smoothness or better responsiveness."; "// if we still don't have a nextframe, we will just have to // extrapolate".

**R2-SYNC-F24.** Quake III does not interpolate an entity across a teleport or on its first snapshot; it places it
directly. Dropped snapshots are counted on the lag meter.
· Source: cg_snapshot.c (CG_SetNextSnap, CG_ReadNextSnapshot) · Tier: shipped game · Read 2026-10-27 · Confirmed in the
source: "// if this frame is a teleport, or the entity wasn't in the // previous frame, don't interpolate";
`cent->interpolate = qfalse;`; "// record as a dropped packet" `CG_AddLagometerSnapshotInfo( NULL );`.

**R2-SYNC-F25.** Quake III interpolates players linearly between two snapshots, but evaluates other entities, such as
missiles, from their trajectories at the client's current time.
· Source: https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/cgame/cg_ents.c
(CG_CalcEntityLerpPositions, CG_InterpolateEntityPosition, CG_AddPacketEntities) · Tier: shipped game · Read 2026-10-27
· Confirmed in the source: "// make sure the clients use TR_INTERPOLATE"; `cent->lerpOrigin[0] = current[0] + f * ( next[0] - current[0] );`;
`cg.frameInterpolation = (float)( cg.time - cg.snap->serverTime ) / delta;`; "// just use the current frame and evaluate
as best we can" `BG_EvaluateTrajectory( &cent->currentState.pos, cg.time, cent->lerpOrigin );`.

**R2-SYNC-F26.** Source extrapolates by default when its interpolation history runs out.
· Source: https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/client/c_baseentity.cpp ·
Tier: shipped game · Read 2026-10-27 · Confirmed in the source: cl_extrapolate, default "1", help text "Enable/disable
extrapolation if interpolation history runs out." Source's default interpolation delay (cl_interp) is not in the SDK
files I read, and the Valve wiki pages that give it refused the fetch, so that number is unread.

#### Lag compensation

**R2-SYNC-F27.** Shipped Quake III has no lag compensation. Its hitscan traces run against the players' current server
positions, so shooters must lead their targets.
· Source: https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/g_weapon.c (Bullet_Fire,
weapon_railgun_fire) · Tier: shipped game · Read 2026-10-27 · The traces are confirmed in the source
(`trap_Trace (&tr, muzzle, NULL, NULL, end, passent, MASK_SHOT);`). The absence of any rewind is inferred: the fetch
tool searched the file and found none, but I did not read every line.

**R2-SYNC-F28.** Source rewinds the other players to the shooter's view: the command's tick minus the client's
interpolation delay. It trusts the client's tick only while it stays within 200 ms of the server's own estimate (round
trip plus interpolation), and otherwise uses that estimate. The total is clamped to `sv_maxunlag`, 1.0 s by default.
· Sources: https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/player_lagcompensation.cpp
(StartLagCompensation) and .../src/public/inetchannelinfo.h · Tier: shipped game · Read 2026-10-27 · Confirmed in the
source: `ConVar sv_maxunlag( "sv_maxunlag", "1.0", FCVAR_DEVELOPMENTONLY, "Maximum lag compensation in seconds", true, 0.0f, true, 1.0f );`;
`correct+= nci->GetLatency( FLOW_OUTGOING );`; `correct += TICKS_TO_TIME( lerpTicks );`;
`correct = clamp( correct, 0.0f, sv_maxunlag.GetFloat() );`; `int targettick = cmd->tick_count - lerpTicks;`;
`if ( fabs( deltaTime ) > 0.2f )` "// difference between cmd time and latency is too big > 200ms, use time correction
based on latency" `targettick = gpGlobals->tickcount - TIME_TO_TICKS( correct );`; and from inetchannelinfo.h
`virtual float GetLatency( int flow ) const = 0; // current latency (RTT), more accurate but jittering`.

**R2-SYNC-F29.** Source keeps up to `sv_maxunlag` (1 s) of history per player. Each record holds position, angles, the
bounding box, the simulation time, and also animation layers and pose parameters, because Source's hitboxes follow the
animation.
· Source: player_lagcompensation.cpp (LagRecord, FrameUpdatePostEntityThink) · Tier: shipped game · Read 2026-10-27 ·
Confirmed in the source: fields `Vector m_vecOrigin;`, `QAngle m_vecAngles;`, `float m_flSimulationTime;`,
`LayerRecord m_layerRecords[MAX_LAYER_RECORDS];`, `float m_flPoseParameters[MAXSTUDIOPOSEPARAM];`;
`int flDeadtime = gpGlobals->curtime - sv_maxunlag.GetFloat();` … `track->Remove( tailIndex );`. The reason ("hitboxes
follow animation") is inferred from the stored fields.

**R2-SYNC-F30.** Source stops rewinding a player at a record where they were dead, or where they jumped more than 64
units between two records (a teleport).
· Source: player_lagcompensation.cpp (BacktrackPlayer) · Tier: shipped game · Read 2026-10-27 · Confirmed in the source:
`if ( !(record->m_fFlags & LC_ALIVE) )` "// player most be alive, lost track"; `if ( delta.Length2DSqr() > m_flTeleportDistanceSqr )`
"// lost track, too much difference"; `sv_lagcompensation_teleport_dist` default "64".

**R2-SYNC-F31.** Source applies no lag compensation when the shooter is a bot, in single player, or to the shooter's own
position.
· Source: player_lagcompensation.cpp (StartLagCompensation) · Tier: shipped game · Read 2026-10-27 · Confirmed in the
source: `|| player->IsBot()` "// not for bots"; "// no lag compensation in single player"; `if ( player == pPlayer )`
"// Don't lag compensate yourself you loser...".

**R2-SYNC-F32.** Source rewinds only targets that are worth it: enemies (teammates only with friendly fire on), entities
already sent to the shooter and acknowledged, players within 1.5 × their maximum speed × `sv_maxunlag`, and players
inside a 45° cone in front of the shooter.
· Source: https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/player.cpp
(CBasePlayer::WantsLagCompensationOnEntity; the tool read the first 100,000 of 265,981 characters) · Tier: shipped game
· Read 2026-10-27 · Confirmed in the source: "// Team members shouldn't be adjusted unless friendly fire is on."; "// If
this entity hasn't been transmitted to us and acked, then don't bother lag compensating it."; "// get max distance
player could have moved within max lag compensation time, " `maxDistance = 1.5 * pPlayer->MaxSpeed() * sv_maxunlag.GetFloat();`;
"// If their origin is not within a 45 degree cone in front of us, no need to lag compensate."
`float flCosAngle = 0.707107f; // 45 degree angle`. The tool would not reproduce the whole function, so how the
distance and cone tests combine is inferred.

**R2-SYNC-F33.** Source caps how many client command ticks it will run "in packet loss conditions", as a guard against
speed hacks.
· Source: player.cpp (sv_maxusrcmdprocessticks, CBasePlayer::PhysicsSimulate) · Tier: shipped game · Read 2026-10-27 ·
Confirmed in the source: help text "Maximum number of client-issued usrcmd ticks that can be replayed in packet loss
conditions, 0 to allow no restrictions"; "// still make sure to avoid speedhax"; "// Run a copy of the user's last
command". The default value is not in the part I read.

**R2-SYNC-F34.** Without lag compensation, the server registers a hit only if the shooter aimed where the target would
be, about one round trip plus the interpolation delay later. With it (F28), the server judges the shot against where the
victim was in the shooter's view, so a victim can be hit just after reaching cover on their own screen. That window
grows with the shooter's latency, up to the cap.
· Source: inferred from F27 and F28 (the Valve wiki pages that explain this with a picture returned HTTP 403) · Tier:
shipped game (reasoning from their code) · Read 2026-10-27 · Inferred.

#### Projectiles

**R2-SYNC-F35.** Quake III sends a projectile as a trajectory: a type (linear, gravity, …), a start time, a start
position and a velocity. Server and client both compute positions from it with the same shared function. A bounce
replaces the trajectory (new base, time and reflected velocity). Each frame the server sweeps the projectile from its
previous position to its new one, ignoring its owner, and moves it 50 ms forward on its first frame.
· Sources: https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/g_missile.c (fire_grenade,
G_BounceMissile, G_RunMissile) and .../code/game/bg_misc.c (BG_EvaluateTrajectory) · Tier: shipped game · Read
2026-10-27 · Confirmed in the source: `#define MISSILE_PRESTEP_TIME 50`;
`bolt->s.pos.trTime = level.time - MISSILE_PRESTEP_TIME; // move a bit on the very first frame`;
`bolt->s.pos.trType = TR_GRAVITY;`; `SnapVector( bolt->s.pos.trDelta ); // save net bandwidth`;
`VectorMA( velocity, -2*dot, trace->plane.normal, ent->s.pos.trDelta );`; `ent->s.pos.trTime = level.time;`;
`BG_EvaluateTrajectory( &ent->s.pos, level.time, origin );`; "// ignore interactions with the missile owner"
`trap_Trace( &tr, ent->r.currentOrigin, ent->r.mins, ent->r.maxs, origin, passent, ent->clipmask );`; in bg_misc.c
TR_GRAVITY `result[2] -= 0.5 * DEFAULT_GRAVITY * deltaTime * deltaTime;`. The client calls the same function (F25).
The record, from https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/q_shared.h: `trajectory_t`
{ `trType_t trType;` `int trTime;` `int trDuration; // if non 0, trTime + trDuration = stop time` `vec3_t trBase;`
`vec3_t trDelta; // velocity, etc` }.

#### Legacy facts used (sync unit)

| Row / table | Fact | What it tells v2 |
|---|---|---|
| CS1 | `FIXED_DT` = 1/60, a `BTN` bitmask and `makeInputCommand(seq,tick)` exist, but nothing reads them; the loop runs on variable frame time (game:3053). | The command shape (sequence, tick, buttons, move, aim) is the right start (F18, F19, F20), but it must be wired to a fixed tick from day one. |
| CS2 | Every soldier's hit shape is three volumes (head sphere r 0.40 m at 2.12 m; torso capsule r 0.52 m; legs capsule r 0.42 m); crouch never applies. | Lag-compensation history needs only position, facing and stance per tick, not animation (unlike Source, F29). The torso is about 1.04 m wide. |
| CS7 | `resolveShot` is a pure read-only query; its comment (core:1491-1492) says it exists so the server can run it against a rewound snapshot. No rewind is built; static hits all carry id −1. | Rewind fits as a query parameter ("resolve at tick t") rather than moving entities back and restoring them. Hit events need stable ids for every target. |
| CS6, CS9 | Shots and sight loop over every static with no broad phase; units never block sight. | At 48+ the rewound shot query needs a spatial index (R4 and the interest-management unit). |
| CS8, GW16 | Today the scene is the truth and the SIM mirrors it each frame. | v2 inverts it: the Rust sim's per-tick state is what snapshots and history are taken from. |
| GC3 | The cursor ray resolves against the SIM to a world aim point at any height (platform tops at 1.20 / 2.50 / 4.00 m). Grenades, the bow and the Juggernaut cannon still aim at the y = 0 ground point. Aiming at height is LOOP_V2 §9 [OPEN]. | The client already picks a 3D aim point with the sim's own ray. A command can carry that point (REC8). |
| GC5 | Shots start from a fixed chest point (`playerFireOrigin`), not the animated muzzle. Spread comes from the shared `Math.random`. Cooldowns reset instead of carrying their remainder: 10.0 / 12.0 / 13.1 shots per second at 30 / 60 / 144 Hz, against 13.9 in the table. | The server can rebuild the exact shot ray from sim state alone. Spread needs a seeded sim RNG the client can share for predicted tracers (F20). Fire timing must carry remainders on the fixed tick (tick-rate unit). |
| GC7 | The Riot Shield's frontal block reads the victim's facing. | Rewound history must hold facing and the shield and stance flags, or a rewound shot judges the block wrong. |
| GC13, GC14, GC15 | Grenade and bolt flight integrate frame dt; the preview and the projectile use different code and origins (animated muzzle). The bolt rays its swept segment each step so it never tunnels. Grenade fuse 1.5 s, blast 6.5 m. | One trajectory function in the Rust core, called by server, client display and preview (F35). Keep the swept test. Origins come from sim mount points. |
| GC17 | The Juggernaut's cannon fires from an animated node, and `performance.now` reaches its aim. It is not in the SIM, so nothing can hit it. It seats 1 (VEHICLE_TYPES). Pilot speed 5.5 m/s; cannon 130 damage, range 130 m. | Vehicles need sim-owned mount points, their own multi-part hit volumes in snapshots and history, and prediction for the pilot. |
| GN6, GN7 | Bots fire through `resolveShot` in the same hit world as the player, with spread from `Math.random`. A second view without ramps exists only because ramps are boxes. | One hit world for every shooter (STATUS.md:68, K1). Bot shots use present state and are not lag-compensated (F31). |
| GP2 | Reload, ability, throw, bow release, E, weapon switch and ADS run at event time. | These become press and release edges in the command, and must survive redundant resending without being lost or doubled (F18). |
| GP13 | The variable-dt sim runs inside rAF and freezes the world for pause, shop, tutorial and hit-stop. | None of that survives multiplayer: the client renders an interpolated, predicted view of a fixed-tick sim. Hit-stop becomes client-only presentation. |
| GC2 | 12 fixed soldier slots (slot = sim id). At 48 soldiers, combat costs 0.80–1.00 ms per update step (JS). | Slot id = entity id carries over. Humans take over bot slots (STATUS.md:68), so the entity id stays stable across that hand-over. |
| STATUS.md:68 | 11 wall-clock reads and shared `Math.random` draws make same-seed matches differ. | Prediction replays need a deterministic sim (R1); every random draw that affects state comes from seeded streams. |
| `T` (LEGACY_TUNING) | `moveSpeed` 6.3 m/s, `sprintMult` 1.4286 (about 9.0 m/s); unit `speedMult` up to 1.5 (Warrior). | In 200 ms a sprinting soldier moves about 1.8 m, more than the torso's width: without lag compensation, a 100 ms player would have to lead by nearly two bodies (F34; inferred). |
| `WEAPONS` | Fire intervals: Shabzon 0.072 s (13.9 shots/s), AMK 0.2 s, Lancer burst 0.06 s; Bagpipe 8 pellets per 0.85 s. Ranges: AMK 300 m, bow 200, Lancer 95, Shabzon 55, Bagpipe 34. | Upper bound at 48 players all on full auto: about 670 rewound rays per second (inferred). |
| `T_CHARGE`, `T` | Bow bolt up to 94 m/s, gravity 14; grenade up to 26 m/s, gravity 20 (the grenade's flight code hard-codes 20 instead of reading the table, GC13); `fragSpeed` 64 m/s. | A bolt at 94 m/s moves about 1.6 m per 1/60 s tick (inferred), so swept tests are needed. Ballistic flight is a closed-form trajectory, cheap to replicate (F35). |

### Brief question 2: tick rate and bandwidth budgets (budgets unit)

Unit **budgets**. Draft: `.checkpoints/v2-research/netcode-architect-r2-budgets-draft.md`; notes with every source summary and its exact quotes: `.checkpoints/v2-research/netcode-architect-r2-budgets.md`. Author: netcode-architect (Opus, xhigh), 2026-10-27. All pass marks and reference hardware here are **proposed**.

Brief question (quoted): "Snapshots, client prediction, interpolation and lag compensation for 48+ players; **tick rate
and bandwidth budgets**; interest management for a top-down camera; ..." (`docs/V2_PHASE0_BRIEF.md` §4, R2 row; this
unit answers the bold part). Target it serves: "Matches feel fair up to about 100 ms of ping. Servers start in one EU
region." (`docs/VISION_V2.md` §4, `[DEFAULT]`). Scale: "two teams with 2–8 players, and three teams with 6–18 players"
to start; "The goal: 12v12v12 or 24v24" (VISION_V2 §3, `[DECIDED]`); LOOP_V2 §2: "12v12v12 to 16v16v16, possibly more,
in squads of 4. Bots fill empty slots."

Tiers: **official** (specifications, standards drafts) · **shipped game** (source code or docs of a shipped game or of
a shipping vendor's own library) · **other** (expert articles; Glenn Fiedler's articles are named by the brief but are
not a shipped game's own documentation). "Confirmed in the source" means the quoted sentence or code line was returned
verbatim by the fetch; "inferred" means the claim rests on the fetch tool's summary or on arithmetic from quoted facts.

#### Transport limits (packet size, overhead, congestion)

- **R2-BUD-F1.** QUIC refuses paths that cannot carry 1200-byte UDP payloads, so 1200 bytes is the only datagram size
  a WebTransport-over-HTTP/3 sender may assume without path MTU discovery. Quote: "QUIC MUST NOT be used if the network
  path cannot support a maximum datagram size of at least 1200 bytes." · RFC 9000 §14 (Datagram Size),
  https://www.rfc-editor.org/rfc/rfc9000.html · official · read 2026-10-27 · confirmed in the source.
- **R2-BUD-F2.** QUIC never relies on IP fragmentation. Quotes: "UDP datagrams MUST NOT be fragmented at the IP layer."
  and (IPv4) "the Don't Fragment (DF) bit MUST be set if possible, to prevent fragmentation on the path." · RFC 9000
  §14 · official · read 2026-10-27 · confirmed in the source.
- **R2-BUD-F3.** The 1200-byte floor leaves headroom under the IPv6 minimum MTU: "QUIC assumes a minimum IP packet size
  of at least 1280 bytes." "Assuming the minimum IP header size of 40 bytes for IPv6 and 20 bytes for IPv4 and a UDP
  header size of 8 bytes," "this results in a maximum datagram size of 1232 bytes for IPv6 and 1252 bytes for IPv4."
  · RFC 9000 §14 · official · read 2026-10-27 · confirmed in the source.
- **R2-BUD-F4.** A QUIC 1-RTT (short header) packet carries 1 byte of flags, a destination connection ID of 0 to 20
  bytes and a packet number of 1 to 4 bytes before the protected payload. Quote (Figure 19): "Header Form (1) = 0,
  Fixed Bit (1) = 1, Spin Bit (1), Reserved Bits (2), Key Phase (1)," "Packet Number Length (2), Destination Connection
  ID (0..160), Packet Number (8..32), Packet Payload (8..)" · RFC 9000 §17.3.1 · official · read 2026-10-27 · confirmed
  in the source.
- **R2-BUD-F5.** QUIC's packet protection adds 16 bytes per packet. Quote: "These cipher suites have a 16-byte
  authentication tag and produce an output 16 bytes larger than their input." · RFC 9001 §5.3 (AEAD Usage),
  https://www.rfc-editor.org/rfc/rfc9001.html · official · read 2026-10-27 · confirmed in the source.
- **R2-BUD-F6.** A QUIC DATAGRAM frame must fit in one QUIC packet. Quote: "DATAGRAM frames cannot be fragmented." The
  frame type is 0x30 or 0x31, and the Length field is present only when the low (LEN) bit is set, so the last frame in
  a packet can omit it (the LEN detail is from the fetch summary: inferred). · RFC 9221 §5 and §4,
  https://www.rfc-editor.org/rfc/rfc9221.html · official · read 2026-10-27 · confirmed in the source (fragmentation);
  inferred (LEN bit).
- **R2-BUD-F7.** Datagrams are congestion-controlled and may be dropped, not queued, when the controller says no.
  Quote: the sender "MUST either delay sending the frame until the controller allows it or drop the frame without
  sending it." · RFC 9221 §5.4 · official · read 2026-10-27 · confirmed in the source.
- **R2-BUD-F8.** Lost datagrams are never resent, and transport acknowledgements do not prove the application used the
  data, so a snapshot protocol needs its own acknowledgements. Quote: "Although DATAGRAM frames are not retransmitted
  upon loss detection, they are ack-eliciting." The "acknowledgement does not guarantee that the application on the
  receiver successfully processed the data" sentence came through the fetch summary. · RFC 9221 §5.2 · official · read
  2026-10-27 · confirmed in the source (no retransmission); inferred (application acks needed).
- **R2-BUD-F9.** A WebTransport datagram costs one variable-length integer (the Quarter Stream ID, 1 byte when the
  value is below 64) on top of the QUIC DATAGRAM frame. Quotes: RFC 9297 §2.1, "When used with HTTP/3, the Datagram
  Data field of QUIC DATAGRAM frames uses the following format:" "HTTP/3 Datagram { Quarter Stream ID (i), HTTP
  Datagram Payload (..), }"; draft-ietf-webtrans-http3-16 §4.5, "The WebTransport datagram payload is sent unmodified
  in the "HTTP Datagram Payload" field of an HTTP Datagram"; RFC 9000 §16 Table 4: a 1-byte varint has 6 usable bits
  (0–63). · https://www.rfc-editor.org/rfc/rfc9297.html, https://datatracker.ietf.org/doc/draft-ietf-webtrans-http3/
  (version -16, dated 2026-07-06, in WG Last Call: a draft, not an RFC) · official · read 2026-10-27 · confirmed in
  the source.
- **R2-BUD-F10.** In the browser API the usable datagram size is a runtime value tied to the path MTU estimate, not a
  constant. Quotes: maxDatagramSize is "The maximum size data that may be passed to a WebTransportDatagramsWritable."
  (§5.3); for HTTP/3 "the value is related to the estimate of the path MTU, which is reduced by some
  implementation-defined amount" (§5.1); it starts as "an implementation-defined integer". · W3C WebTransport,
  https://w3c.github.io/webtransport/ (Editor's Draft dated 2026-10-27, "Don't cite this document other than as work in
  progress") · official (editor's draft) · read 2026-10-27 · confirmed in the source.
- **R2-BUD-F11.** Writing a datagram larger than maxDatagramSize does not raise an error: the write algorithm returns
  "a promise resolved with undefined" and the datagram is not queued. · W3C WebTransport §4.3 (writeDatagrams, step 4)
  · official (editor's draft) · read 2026-10-27 · inferred (the quoted fragment is verbatim; "not queued, no error" is
  the fetch summary's reading of the step).
- **R2-BUD-F12.** The browser exposes the numbers an adaptive budget and a clock sync need. Quotes (§6.13): smoothedRtt
  "The smoothed round-trip time (RTT) currently observed on the connection."; rttVariation "The mean variation in
  round-trip time samples currently observed on the connection."; estimatedSendRate "The estimated rate at which queued
  data will be sent by the user agent, in bits per second." and "If the user agent does not currently have an
  estimate, the member MUST be the null value." · W3C WebTransport §6.13 · official (editor's draft) · read 2026-10-27 ·
  confirmed in the source.
- **R2-BUD-F13.** Outgoing datagrams can be given a maximum age, and the browser counts the ones that expired or were
  lost. Quotes (§6.14): expiredOutgoing counts datagrams "being older than outgoingMaxAge before they were able to be
  sent"; lostOutgoing is "The number of sent datagrams that were declared lost." outgoingMaxAge is a duration in
  milliseconds or null (§5.3, fetch summary). · W3C WebTransport §5.3, §6.14 · official (editor's draft) · read
  2026-10-27 · confirmed in the source (stats); inferred (outgoingMaxAge units).
- **R2-BUD-F14.** The "low-latency" congestion-control option exists in the API but is a feature at risk that no
  browser implements yet. · W3C WebTransport §6.9 · official (editor's draft) · read 2026-10-27 · inferred (fetch
  summary; no verbatim quote obtained).
- **R2-BUD-F15.** QUIC starts with a congestion window of about ten full datagrams and may shrink to two. Quotes:
  "Endpoints SHOULD use an initial congestion window of ten times the maximum datagram size" "limiting the window to
  the larger of 14,720 bytes or twice the maximum datagram size"; minimum window: "The RECOMMENDED value is 2 *
  max_datagram_size." Pacing: "A sender SHOULD pace sending of all in-flight packets based on input from the
  congestion controller." · RFC 9002 §7.2, §7.7, https://www.rfc-editor.org/rfc/rfc9002.html · official · read
  2026-10-27 · confirmed in the source.
- **R2-BUD-F16.** WebSocket framing adds 2 bytes per server-to-client message up to 125 bytes, 4 bytes up to 65 535
  bytes, plus a 4-byte mask on every client-to-server frame. Quotes: "Payload length: 7 bits, 7+16 bits, or 7+64 bits"
  "If 126, the following 2 bytes interpreted as a 16-bit unsigned integer are the payload length." "Masking-key: 0 or
  4 bytes"; §5.1 "a client MUST mask all frames that it sends to the server". · RFC 6455 §5.1, §5.2,
  https://www.rfc-editor.org/rfc/rfc6455.html · official · read 2026-10-27 · confirmed in the source (fields); inferred
  (the 2-byte base header, read from the frame layout).
- **R2-BUD-F17.** Over the WebSocket fallback, each TLS 1.3 record adds a 5-byte header, 1 byte of inner content type
  and the AEAD tag. Quote: the length is "the sum of the lengths of" "the content and the padding, plus one for the
  inner content type, plus any expansion added by the AEAD algorithm." · RFC 8446 §5.2,
  https://www.rfc-editor.org/rfc/rfc8446.html · official · read 2026-10-27 · confirmed in the source (length rule);
  inferred (5-byte header, from the struct: 1 + 2 + 2 bytes).
- **R2-BUD-F18.** Fragmenting a game packet multiplies its loss: any lost fragment loses the whole packet, so
  P(loss) = 1 − p^n (with 1 % loss: 2 fragments about 2 %, 10 fragments about 9.5 %). The same article gives 1200 bytes
  as "a conservative lower bound on the IPv4 Internet today" and "somewhere around 1472" as the typical size before
  packets are dropped or fragmented. Quote: "if any fragment of that packet is lost then the entire packet is lost." ·
  Fiedler, "Packet Fragmentation and Reassembly", https://gafferongames.com/post/packet_fragmentation_and_reassembly/
  (page date 2016-09-06) · other · read 2026-10-27 · confirmed in the source.

#### Encoding costs (quantization, delta compression, bit-packing)

- **R2-BUD-F19.** Bounded, quantized positions cost far less than floats: "With experimentation I found that 512 values
  per-meter (roughly 2mm precision) provides enough precision." "That's 18 bits for x, 18 bits for y and 14 bits for z
  giving a total of 50 bits per-position (originally 96)." · Fiedler, "Snapshot Compression",
  https://gafferongames.com/post/snapshot_compression/ (page date 2015-01-04) · other · read 2026-10-27 · confirmed in
  the source.
- **R2-BUD-F20.** Orientation in "smallest three" form: "This gives a result of 2 + 9 + 9 + 9 = 29 bits
  per-orientation (down from 128 bits)." Linear velocity can be left out when snapshots are interpolated at 60 Hz:
  "This means we can avoid sending linear velocity entirely." · Fiedler, "Snapshot Compression" · other · read
  2026-10-27 · confirmed in the source.
- **R2-BUD-F21.** Delta compression against the last snapshot the client acknowledged makes unchanged objects nearly
  free: "Each cube that isn't sent costs 1 bit (not changed)." The baseline moves to the newest ack: "The sender takes
  this most recent ack and if it is more recent than the previous ack" ... "updates the baseline snapshot to this
  value." A flat changed-bit for 901 objects still costs "54kbps of bandwidth" at 60 packets per second, which is why
  the article switches to lists of changed indices (about 5.5 bits each, relative-encoded), and relative positions
  (about 26.1 bits against 50 absolute; these averages are from the fetch summary). · Fiedler, "Snapshot Compression" ·
  other · read 2026-10-27 · confirmed in the source (1 bit, baseline, 54 kbps); inferred (averages).
- **R2-BUD-F22.** Header overhead dominates a quiet snapshot stream: stationary steady state was "around 15 kilobits
  per-second", "composed entirely of our own packet header (uint16 sequence, uint16 base, bool initial)" "plus IP and
  UDP headers (28 bytes)." Uncompressed, the 901-object scene needed "17.37 megabits per-second!" at 60 Hz. ·
  Fiedler, "Snapshot Compression" · other · read 2026-10-27 · confirmed in the source.
- **R2-BUD-F23.** Context-aware arithmetic coding beat the best hand bit-packing by 25 % on the same data: "This
  implementation by Fabian Giesen beat my best bit-packed result by 25%." · Fiedler, "Snapshot Compression" · other ·
  read 2026-10-27 · confirmed in the source.
- **R2-BUD-F24.** Quake III delta-encodes each entity against the client's acknowledged snapshot with one "changed" bit
  per field up to the last changed field: "MSG_WriteByte( msg, lc ); // # of changes", "MSG_WriteBits( msg, 1, 1 ); //
  changed". Positions are floats sent as a 13-bit integer when integral ("#define FLOAT_INT_BITS 13"), else as 32 bits;
  "{ NETF(weapon), 8 }", "{ NETF(eFlags), 19 }", "{ NETF(legsAnim), 8 }", "{ NETF(torsoAnim), 8 }". ·
  https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/qcommon/msg.c (Quake III Arena source, GPL
  v2+, id Software 1999–2005) · shipped game · read 2026-10-27 · confirmed in the source.
- **R2-BUD-F25.** Quake III keeps 32 old messages per client for delta compression: "#define PACKET_BACKUP 32 // number
  of old messages that must be kept on client and // server for delta comrpession and ping estimation"; the server
  sizes its snapshot-entity ring as "svs.numSnapshotEntities = sv_maxclients->integer * PACKET_BACKUP * 64;" (room for
  64 entities per stored snapshot per client: inferred from the formula). · qcommon.h and server/sv_init.c, same repo ·
  shipped game · read 2026-10-27 · confirmed in the source.

#### What shipped games and libraries publish about rates and sizes

- **R2-BUD-F26.** Quake III's defaults: server frame rate "sv_fps = Cvar_Get ("sv_fps", "20", CVAR_TEMP );"; client
  "Cvar_Get ("snaps", "20", ...)" (snapshots per second requested), "cl_maxpackets = Cvar_Get ("cl_maxpackets", "30",
  CVAR_ARCHIVE );" (client packets per second), "cl_packetdup = Cvar_Get ("cl_packetdup", "1", CVAR_ARCHIVE );"
  (repeat old commands), and "Cvar_Get ("rate", "3000", CVAR_USERINFO | CVAR_ARCHIVE );" (bytes per second); up to
  "#define MAX_PACKET_USERCMDS 32 // max number of usercmd_t in a packet". · server/sv_init.c, client/cl_main.c,
  qcommon/qcommon.h, same repo · shipped game (1999 defaults) · read 2026-10-27 · confirmed in the source (lines); the
  meaning of each cvar beyond its name is inferred.
- **R2-BUD-F27.** Quake III caps a network packet at 1400 bytes and fragments larger messages itself: "#define
  MAX_PACKETLEN 1400 // max size of a network packet", "#define FRAGMENT_SIZE (MAX_PACKETLEN - 100)", "#define
  MAX_MSGLEN 16384 // max length of a message, which may // be fragmented into multiple packets". · qcommon/net_chan.c,
  qcommon/qcommon.h · shipped game · read 2026-10-27 · confirmed in the source.
- **R2-BUD-F28.** Source engine games tick at 66.7 Hz by default, and TF2, Counter-Strike: Source and Day of Defeat:
  Source ignore the tickrate override: "#define DEFAULT_TICK_INTERVAL (0.015) // 15 msec is the default"; "// [Forrest]
  For Counter-Strike, set default tick rate of 66 and removed -tickrate command line parameter."; "#if !defined( TF_DLL
  ) && !defined( CSTRIKE_DLL ) && !defined( DOD_DLL )" around the "-tickrate" override. ·
  https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/public/const.h and
  .../src/game/server/gameinterface.cpp (Source SDK 2013, master) · shipped game · read 2026-10-27 · confirmed in the
  source.
- **R2-BUD-F29.** Source rewinds a shot by the client's outgoing latency plus its interpolation delay, clamped to
  sv_maxunlag (default and maximum 1 s), and keeps that much history: "ConVar sv_maxunlag( "sv_maxunlag", "1.0",
  FCVAR_DEVELOPMENTONLY, "Maximum lag compensation in seconds"" ... "true, 0.0f, true, 1.0f );"; "correct+=
  nci->GetLatency( FLOW_OUTGOING );" "correct += TICKS_TO_TIME( lerpTicks );" "correct = clamp( correct, 0.0f,
  sv_maxunlag.GetFloat() );"; purge: "int flDeadtime = gpGlobals->curtime - sv_maxunlag.GetFloat();". It also checks
  "if ( fabs( deltaTime ) > 0.2f )" (a 200 ms tolerance between the client's claimed and the server's computed target
  time; that reading is inferred, the context was not quoted). ·
  https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/player_lagcompensation.cpp ·
  shipped game · read 2026-10-27 · confirmed in the source (lines); inferred (meaning of the 0.2 check).
- **R2-BUD-F30.** Valve's own transport library limits UDP payloads to 1300 bytes by default: "/// Max size of UDP
  payload. Includes API payload and" "/// any headers, but does not include IP/UDP headers" "const int
  k_cbSteamNetworkingSocketsMaxUDPMsgLen = 1300;" and "int32, MTU_PacketSize, 1300, ..." ·
  https://raw.githubusercontent.com/ValveSoftware/GameNetworkingSockets/master/src/steamnetworkingsockets/steamnetworkingsockets_internal.h
  and .../clientlib/csteamnetworkingsockets.cpp (GameNetworkingSockets, master) · shipped game (Valve's shipping
  library) · read 2026-10-27 · confirmed in the source.
- **R2-BUD-F31.** The same library's default per-connection send-rate clamp is 256 KiB/s (about 2.1 Mbit/s), set by
  hand, with bandwidth estimation still a stated goal: "Minimum/maximum send rate clamp, in bytes/sec." "The default
  value is 256K." "Eventually we hope to have the library estimate the bandwidth of the channel" ·
  "DEFINE_CONNECTON_DEFAULT_CONFIGVAL( int32, SendRateMin, 256*1024, 1024, 0x10000000 );" · include/steam/steamnetworkingtypes.h
  and csteamnetworkingsockets.cpp · shipped game (library) · read 2026-10-27 · confirmed in the source.

#### Buffers and rates for fairness

- **R2-BUD-F32.** An interpolation buffer of three send intervals rides out two lost snapshots in a row at 2–5 % loss,
  plus a jitter allowance; at 30 snapshots per second that is 150 ms, at 60 it is 85 ms. Quotes: "My rule of thumb is
  that the interpolation buffer should have enough delay so that I can lose two packets in a row"; "Experimentally I've
  found that the amount of delay that works best at 2-5% packet loss is 3X the packet send rate." "At 10 packets
  per-second this is 300ms."; "the interpolation videos above were recorded with a delay of 350ms."; "If we send 30
  snapshots per-second we can get the same amount of packet loss protection with a delay of 150ms."; "60 packets
  per-second needs only 85ms." · Fiedler, "Snapshot Interpolation", https://gafferongames.com/post/snapshot_interpolation/
  (page date 2014-11-30) · other · read 2026-10-27 · confirmed in the source ("3X the packet send rate" means three
  send intervals: inferred from the 10 pps → 300 ms example).
- **R2-BUD-F33.** Hermite interpolation needs velocity in the snapshot, which raises bandwidth "slightly"; linear
  interpolation at 10 pps showed visible artifacts. · Fiedler, "Snapshot Interpolation" · other · read 2026-10-27 ·
  inferred (fetch summary).
- **R2-BUD-F34.** A fixed bandwidth budget per client is enforced by a priority accumulator: "To get started each frame
  walk over all objects in your simulation and calculate their current priority." "This is an array of float values,
  one value per-object, that is remembered from frame to frame." "The first n objects in this sorted list are the
  objects you should send that frame." "The desired bandwidth can even be adjusted on the fly." The example budget:
  "typically you have some maximum bandwidth you want to support like 256kbit/sec". Objects that were sent reset to
  zero, skipped ones keep their total (fetch summary). · Fiedler, "State Synchronization",
  https://gafferongames.com/post/state_synchronization/ (page date 2015-01-05) · other · read 2026-10-27 · confirmed in
  the source (quoted lines); inferred (reset rule).
- **R2-BUD-F35.** Inputs are repeated in every packet so that one lost packet rarely loses an input, and a
  state-synchronization jitter buffer of "say 4-5 frames @ 60HZ" is enough there. · Fiedler, "State Synchronization" ·
  other · read 2026-10-27 · confirmed in the source (jitter quote); inferred (redundant inputs, fetch summary).

#### Legacy facts used (budgets unit)

| Source | Fact | What it tells v2 |
|---|---|---|
| Inventory CS1; LEGACY_TUNING `FIXED_DT`, `BTN` | `FIXED_DT` = 1/60 is declared but unread; the live loop is variable-step (game:3053). `BTN` has 7 bits (FIRE 1, ADS 2, RELOAD 4, MELEE 8, GRENADE 16, SHIELD 32, SPRINT 64); `makeInputCommand(seq, tick)` carries seq, tick, move, aim, buttons. | The input record starts at 7 button bits plus move and aim; 60 Hz is a stated intent, not a measured legacy behaviour. |
| Inventory CS8 | The hit world is players (id, position, alive, stance, team) plus kinded static boxes with an alive flag and optional Y rotation; stance is always 0. | A starting field list for the replicated soldier record; static boxes are map data, not per-snapshot traffic. |
| Inventory CD21, CD4; LEGACY_TUNING `T` | Ground 372 × 252 m; arena half-size 186 × 126 m; every legacy map shares it. LOOP_V2 §9: Octopon-3 size `[OPEN]`, "Start near Cabana's footprint". | 16 bits per horizontal axis at 1 cm covers ±327.68 m, nearly twice the legacy half-extent. |
| Inventory GP3; `shamble-game.js:2774-2775` | Player speed = 6.3 m/s × sprintMult (at most 1.62) × 1.3 in Banzai × 0.72 with the shield: at most about 13.3 m/s. Jump 11 m/s, gravity 24 (CD4). | Bounds for relative position deltas and for velocity quantization (±20 m/s leaves margin). Bots' `speedMult` path was not checked. |
| Inventory CD5 | Camera height 40, boom 56.6, fov 15°. | Input to the interest unit (a narrow top-down view), not used here. |
| Inventory CM4, GC2 | DM_Tri, 48 soldiers in combat: 0.80–1.00 ms mean per update step, p95 0.97–1.54 ms, 3 reps (legacy JavaScript, headless, `tools/perfbots.js`); 12 fixed soldier slots today. | A ballpark for one simulation step at 48 soldiers; not a server measurement and not Rust. |
| Inventory GE6, CD14, GC17 | One harvester per team; one vehicle type (Juggernaut: 5.5 m/s, 3 shells 80 ms apart). VISION_V2 §6.4: vehicles multi-part, seats `[OPEN]`. | Vehicle counts in the estimate are assumptions; turret yaw is a second angle per vehicle. |
| Inventory GC14, GC15, GC11 | Frags (1.5 s fuse, 3 carried), Molotovs (1.4 s), Torque Bow bolts (0.55 s after sticking), all swept per step. | Projectiles are short-lived; their count, not their size, drives the cost. |
| LEGACY_TUNING `WEAPONS` | Fastest fire interval: Shabzon 0.072 s (13.9 shots/s); Lancer bursts 3 × 0.06 s; Bagpipe 8 pellets. Juggernaut ripple 80 ms (GC17). | 0.072 s is 2.16 ticks at 30 Hz and 4.32 ticks at 60 Hz: without carrying the fractional remainder, the Shabzon would fire at 10/s (30 Hz, −28 %) or 12/s (60 Hz, −14 %). |

### Brief question 3: interest management for a top-down camera (interest unit)

Unit **interest**. Draft: `.checkpoints/v2-research/netcode-architect-r2-interest-draft.md`; notes with every source summary and its exact quotes: `.checkpoints/v2-research/netcode-architect-r2-interest.md`. Author: netcode-architect, 2026-10-27. The camera framing is "look brief pending": it enters here only as parameters. Every pass mark and reference machine below is proposed.

How sure: "confirmed in the source" means the quoted words were returned verbatim from the source; "inferred" means
a WebFetch paraphrase, or arithmetic and reasoning on confirmed facts (the inputs are named); "unverified" means not
checked. Sources S1–S14 are listed in section 6. [Assembler's note: the interest unit's section 6 is merged into Sources at the end of this file; the "Units" column gives each S id.]

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

#### Legacy facts used (interest unit)

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

### Brief question 4: the WebTransport server stack in Rust plus the WebSocket fallback (transport unit)

Unit **transport**. Draft: `.checkpoints/v2-research/netcode-architect-r2-transport-draft.md`; notes with every source summary and its exact quotes: `.checkpoints/v2-research/netcode-architect-r2-transport.md`. Author: netcode-architect, 2026-10-27.

Brief question (quoted): "the WebTransport server stack in Rust plus the WebSocket fallback" (docs/V2_PHASE0_BRIEF.md §4, R2 row,
one of its six questions). Unit scope from the wave-1 prompt: the specification status (W3C API, IETF protocol over HTTP/3 and its
drafts), browser support in Chrome, Edge, Firefox and Safari, datagrams versus streams and the datagram size limit, certificates,
the Rust crates, the WebSocket fallback and head-of-line blocking, one transport interface over both, congestion control, and
what a spike must prove.

Every source was read on 2026-10-27.
"Confirmed in the source" is used only where the exact words were quoted back from the page (the quotes are in the finding);
anything seen only in a fetch summary is marked "inferred".

#### Specification status

**R2-TRN-F1.** The W3C WebTransport API is a Candidate Recommendation Snapshot dated 30 July 2026. It "will remain a Candidate
Recommendation at least until 30 October 2026". It is not yet a W3C Recommendation.
Source: https://www.w3.org/TR/webtransport/, status line ("W3C Candidate Recommendation Snapshot, 30 July 2026") and "Status of
this document". Tier: official. Read 2026-10-27. How sure: confirmed in the source.

**R2-TRN-F2.** The wire protocol, WebTransport over HTTP/3, is still an Internet-Draft, not an RFC. The page reads:
"draft-ietf-webtrans-http3-16", last updated "2026-07-06", intended status "Proposed Standard", WG state "In WG Last Call", IESG
state "I-D Exists", "Expires 7 January 2027". The history page lists 17 revisions (-00, 2021-02-17, to -16), and the WG Last Call
began on 2026-03-08.
Source: https://datatracker.ietf.org/doc/draft-ietf-webtrans-http3/ (document fields) and /history/. Tier: official. Read 2026-10-27.
How sure: confirmed in the source for the -16 fields; inferred (fetch summary) for the revision list and the WGLC date.

**R2-TRN-F3.** WebTransport over HTTP/2, the TCP variant, is also a draft: draft-ietf-webtrans-http2-15 (2026-07-06), in WG Last
Call, intended Proposed Standard.
Source: https://datatracker.ietf.org/doc/draft-ietf-webtrans-http2/. Tier: official. Read 2026-10-27. How sure: inferred (fetch
summary; only the abstract sentence was quoted).

**R2-TRN-F4.** Draft versions are not wire-compatible. Each version is negotiated through an HTTP/3 SETTINGS codepoint, and the
highest version both sides support wins. Draft -07, Appendix A.1: "The following changes make the draft-02 and draft-07
versions of this protocol incompatible". Draft -16, §7.1 (to be removed before publication): "Each draft version defines a
distinct codepoint for SETTINGS_WT_ENABLED" and "The highest version supported by both endpoints is selected." The -16 codepoint is
0x2c7cf000 (§9.2). Draft -02 used SETTINGS_ENABLE_WEBTRANSPORT = 0x2b603742, and draft -07 used
SETTINGS_WEBTRANSPORT_MAX_SESSIONS = 0xc671706a.
Sources: https://datatracker.ietf.org/doc/html/draft-ietf-webtrans-http3-07 (App. A.1, §8.2);
https://datatracker.ietf.org/doc/html/draft-ietf-webtrans-http3 (-16, §7.1, §9.2);
https://datatracker.ietf.org/doc/html/draft-ietf-webtrans-http3-02 (§8.2). Tier: official. Read 2026-10-27. How sure: confirmed in
the source (quoted fragments and codepoints).

**R2-TRN-F5.** QUICHE, the QUIC and HTTP/3 library Chromium uses, defines only the draft-02 and draft-07 WebTransport settings
in its HTTP constants header: "SETTINGS_WEBTRANS_DRAFT00 = 0x2b603742," and "SETTINGS_WEBTRANS_MAX_SESSIONS_DRAFT07 = 0xc671706a,".
That file has no -16 SETTINGS_WT_ENABLED.
Source: https://raw.githubusercontent.com/google/quiche/main/quiche/quic/core/http/http_constants.h (main branch). Tier: official
(source code). Read 2026-10-27. How sure: confirmed in the source for the constants. The step to "Chrome speaks draft-02/-07 today"
is inferred: this file was the only one read, and the page itself does not say that Chrome uses QUICHE.

**R2-TRN-F6.** Both Rust WebTransport stacks advertise the same draft-02 and draft-07 settings, and neither defines the -16
codepoint:
- wtransport-proto (master) defines `SETTINGS_ENABLE_WEBTRANSPORT: VarInt = VarInt::from_u32(0x2b60_3742)` and
  `SETTINGS_WEBTRANSPORT_MAX_SESSIONS: VarInt = VarInt::from_u32(0xc671_706a)`.
- web-transport-proto 0.6.3 has `enable_webtransport()`, which sends all of them, with the comments
  "ENABLE_DATAGRAM_DEPRECATED = 0xFFD277, // still used by Chrome", "// Removed in draft 06" (on 0x2b603742 and 0x2b603743) and
  "// TODO remove when 07 is in the wild".

Sources: https://raw.githubusercontent.com/BiagioFesta/wtransport/master/wtransport-proto/src/settings.rs;
https://docs.rs/web-transport-proto/latest/src/web_transport_proto/settings.rs.html. Tier: official (source code); the
"still used by Chrome" comment is the maintainer's claim (other). Read 2026-10-27. How sure: confirmed in the source.

#### Browser support (VISION_V2 §4 targets)

**R2-TRN-F7.** All four target browsers ship WebTransport today, with versions and release dates from MDN's compatibility data:

| Browser | WebTransport since | Released | serverCertificateHashes since | Current today |
|---|---|---|---|---|
| Chrome | 97 | 2022-01-04 | 100 (2022-03-29) | 156 (2026-10-20) |
| Edge | "mirror" of Chrome | (as Chrome) | (as Chrome) | (as Chrome) |
| Firefox | 114 | 2023-06-06 | 125 (2024-04-16) | 158 (2026-10-13); ESR 153 (2026-07-21) |
| Safari (macOS; iOS "mirror") | 26.4 | 2026-03-24 | 26.4 | 27 (2026-09-14) |

MDN: "Baseline 2026" "Newly available" "Since March 2026, this feature works across the latest devices and browser versions." It
adds: "Some parts of this feature may have varying levels of support."
Sources: https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/WebTransport.json (`"chrome": … "version_added": "97"`,
`"firefox": … "114"`, `"safari": … "26.4"`, `"edge": "mirror"`, `"safari_ios": "mirror"`, serverCertificateHashes "100" / "125" /
"26.4"); browsers/chrome.json, firefox.json, safari.json in the same repository (the release_date values quoted above);
https://developer.mozilla.org/en-US/docs/Web/API/WebTransport_API (last modified 2026-10-21). Tier: official. Read 2026-10-27. How
sure: confirmed in the source. Edge's Edge-specific version is not given: "mirror" means MDN copies Chrome's data.

**R2-TRN-F8.** Sending datagrams uses a different API per browser.
- Chrome and Edge have only `datagrams.writable`: `"chrome": { "version_added": "97" }`. MDN now marks it
  `"standard_track": false, "deprecated": true`, and Safari has `"version_added": false`.
- Safari has only the standard `datagrams.createWritable()`: `"safari": { "version_added": "26.4" }`, and Chrome has
  `"version_added": false`.
- Firefox has both: `writable` since 114 and `createWritable` since 155 (2026-09-01).

Source: https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/WebTransportDatagramDuplexStream.json. Tier: official.
Read 2026-10-27. How sure: confirmed in the source (quoted JSON lines).

**R2-TRN-F9.** Connection statistics (`getStats()`) are missing on two of the four targets.
- Chrome: `"version_added": false`, `"impl_url": "https://crbug.com/41492543"`.
- Firefox: `"partial_implementation": true`, `"notes": "Method is defined but throws a not-implemented error."`
- Safari: "26.4".

So the browser cannot read its RTT or estimated send rate from the API on Chrome, Edge or Firefox today.
Source: https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/WebTransport.json. Tier: official. Read 2026-10-27. How
sure: confirmed in the source for the data; the consequence is inferred.

**R2-TRN-F10.** The browser congestion-control hint is weak.
- The `congestionControl` option is `"version_added": false` in Chrome, and 114 / 26.4 in Firefox / Safari.
- MDN: "This is a hint to the user agent."
- W3C §6.9: "low-latency" is "considered a feature at risk due to the lack of implementation in browsers".

Sources: BCD WebTransport.json (as above); https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/WebTransport (last modified
2026-10-12); https://www.w3.org/TR/webtransport/ §6.9. Tier: official. Read 2026-10-27. How sure: confirmed in the source.

**R2-TRN-F11.** Chrome and Edge lack several newer options.
- `requireUnreliable` is `"version_added": false` in Chrome (Firefox 114, Safari 26.4).
- The fetch summary of the same data also showed these: `allowPooling`, stream `sendOrder` and `createSendGroup` false in Chrome;
  `protocols` in Chrome 143 (2025-12-02), Firefox 155 and Safari 26.4; `reliability` and `supportsReliableOnly` in Chrome 157,
  which is still beta (release 2026-11-03); Firefox's `createUnidirectionalStream` partial ("Returns a `WritableStream` instead of
  a `WebTransportSendStream`").
- Firefox ESR 153 (2026-07-21) is older than Firefox 155, so it lacks the 155 additions (`protocols`, `createWritable`,
  `createSendGroup`).

Source: BCD WebTransport.json and browsers/*.json. Tier: official. Read 2026-10-27. How sure: confirmed in the source for
requireUnreliable and the release dates; inferred (fetch summary) for the other rows.

**R2-TRN-F12.** Safari can run a WebTransport session over HTTP/2 and TCP, and the API says which kind a session got.
- WebKit's Safari 26.4 post: "WebTransport can run over HTTP/2 and TCP as a fallback with the same API". Such a session is
  reliable only. W3C §6.5: "If the connection is an HTTP/3 connection, set transport.[[Reliability]] to "supports-unreliable"",
  and "If the connection is an HTTP/2 connection" it becomes "reliable-only".
- `requireUnreliable` forbids the fallback. A session "cannot be established over an HTTP/2 connection if an HTTP/3 connection
  is not possible" (W3C §6.9; MDN: "By default, the value is `false`.").

Sources: https://webkit.org/blog/17862/ ("WebKit Features for Safari 26.4", 2026-03-24); https://www.w3.org/TR/webtransport/ §6.5,
§6.9; MDN WebTransport() constructor page. Tier: official. Read 2026-10-27. How sure: confirmed in the source.

#### Certificates

**R2-TRN-F13.** Certificates pinned by hash (`serverCertificateHashes`) are tightly constrained. W3C §6.9:
- "the certificate MUST be an X.509v3 certificate";
- "the total length of the validity period MUST NOT exceed two weeks";
- "it MUST include ECDSA with the secp256r1 (NIST P-256) named group";
- "It MUST NOT contain RSA keys".

Only SHA-256 is checked. MDN: "This option is only supported for transports using dedicated connections (`allowPooling` is
`false`)."
Sources: https://www.w3.org/TR/webtransport/ §6.9; MDN WebTransport() constructor page. Tier: official. Read 2026-10-27. How sure:
confirmed in the source. Two details come from fetch summaries and are inferred: the SHA-256-only rule (§6.9/§14.4), and "no
interstitial" for certificate errors (§14.1).

**R2-TRN-F14.** Hash pinning is meant for hosts without a public certificate.
- MDN: "This feature allows developers to connect to WebTransport servers that would normally find it challenging to obtain a
  publicly trusted certificate, such as hosts that are not publicly routable, or ephemeral hosts like virtual machines."
- The hashes come from somewhere trusted: "The web application might typically fetch the hashes from a trusted intermediary."
- Without hashes, the normal web certificate check applies: "If empty, the user agent uses the same PKI certificate verification
  procedures it would use for a normal fetch operation."

Source: https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/WebTransport. Tier: official. Read 2026-10-27. How sure:
confirmed in the source.

**R2-TRN-F15.** wtransport can create a certificate that meets these rules. `Identity::self_signed`: "This function uses the `ECDSA
P-256` algorithm." and "The total length of the validity period MUST NOT exceed two weeks." Its builder example uses
`.validity_days(14)`. This is the default `self-signed` feature (rcgen 0.14.5).
Sources: https://docs.rs/wtransport/latest/wtransport/struct.Identity.html (0.7.2);
https://raw.githubusercontent.com/BiagioFesta/wtransport/master/wtransport/Cargo.toml (`default = ["self-signed", "ring"]`). Tier:
official (docs, source). Read 2026-10-27. How sure: confirmed in the source.

#### Datagrams and their size

**R2-TRN-F16.** QUIC datagrams are unreliable, congestion-controlled and cannot be fragmented. RFC 9221 (March 2022):
- §2: "QUIC datagrams are subject to QUIC congestion control."
- §5.4: "The sender MUST either delay sending the frame until the controller allows it" "or drop the frame without sending it".
- §5.2: "Although DATAGRAM frames are not retransmitted upon loss detection, they are ack-eliciting".
- §5: "DATAGRAM frames cannot be fragmented; therefore, application protocols need to handle cases" where data does not fit.
- §5.3: "DATAGRAM frames do not provide any explicit flow control signaling".

WebTransport over HTTP/3 carries its datagrams this way, and each one starts with a session prefix. Draft -16, §4.5: "the payload
field directly follows the Quarter Stream ID field, which is at the start of the QUIC DATAGRAM frame payload". §4.1: they "are not
retransmitted".
Sources: https://www.rfc-editor.org/rfc/rfc9221.html; https://datatracker.ietf.org/doc/html/draft-ietf-webtrans-http3 (-16). Tier:
official. Read 2026-10-27. How sure: confirmed in the source.

**R2-TRN-F17.** The size floor is 1,200 bytes of UDP payload.
- RFC 9000 §14: "QUIC MUST NOT be used if the network path cannot support a maximum datagram size of at least 1200 bytes."
- quinn's `initial_mtu`: "Must be at least 1200, which is the default, and known to be safe for typical internet applications"
  (`min_mtu` also defaults to 1200; MTU discovery is "Enabled by default.").
- The usable application payload is smaller once the QUIC packet header, the frame header and the Quarter Stream ID are
  subtracted. No source read gives that exact number.

Sources: https://www.rfc-editor.org/rfc/rfc9000.html §14; https://docs.rs/quinn/latest/quinn/struct.TransportConfig.html (0.11.12).
Tier: official. Read 2026-10-27. How sure: confirmed in the source for the 1,200-byte rules; the smaller payload is inferred.

**R2-TRN-F18.** The browser's datagram size limit is not fixed, and a datagram over the limit vanishes without an error.
- W3C §5.1: "The maximum datagram size depends on the protocol that is in use." (for HTTP/3 it follows the path MTU estimate).
  "The user agent MAY update" it while connecting or connected.
- Over HTTP/2 "there is no equivalent limit."
- §4.3: when the limit "is less than data's [[ByteLength]]", writing returns "a promise resolved with undefined", and nothing is
  sent.

Source: https://www.w3.org/TR/webtransport/ §4.3, §5.1. Tier: official. Read 2026-10-27. How sure: confirmed in the source (quoted
fragments).

**R2-TRN-F19.** Browser datagram queues drop rather than block.
- Incoming: the queue has a limit "beyond which datagrams are dropped from the head of the queue" (§5.1).
- Outgoing: datagrams older than `outgoingMaxAge` are removed ("If more than duration milliseconds have passed since timestamp,
  then:", §4.3).
- `outgoingMaxAge` and `incomingMaxAge` exist in Chrome 97, Firefox 114 and Safari 26.4. The newer `*MaxBufferedDatagrams` limits
  exist only in Chrome 151 (Safari: preview; Firefox: none).

Sources: https://www.w3.org/TR/webtransport/; BCD WebTransportDatagramDuplexStream.json. Tier: official. Read 2026-10-27. How sure:
confirmed in the source for the W3C rules; the per-browser rows are inferred (fetch summary).

**R2-TRN-F20.** Fiedler on oversized and fragmented packets. Games have seen oversized packets where "suddenly all packets start
getting dropped!"; a router "has the option of a) fragmenting that packet, or b) dropping the packet." He gives a conservative
bound of "1200 bytes" and splits larger packets into "1024 byte fragments". Losing any fragment loses the whole packet, so loss
compounds with the fragment count (his example, per the fetch summary: 1 % loss over 100 fragments is about a 63.4 % chance of
losing the packet).
Source: https://gafferongames.com/post/packet_fragmentation_and_reassembly/ ("Packet Fragmentation and Reassembly", Glenn
Fiedler, 2016-09-06). Tier: shipped-game practitioner. Read 2026-10-27. How sure: confirmed in the source for the quoted fragments;
the 63.4 % figure is inferred.

#### Streams, ordering and head-of-line blocking

**R2-TRN-F21.** WebSocket runs over TCP, so one lost packet stalls everything behind it. WebTransport streams do not block each
other. WebKit, about WebSockets: "head-of-line blocking can cause delays when packets are lost". About WebTransport: "It runs
over HTTP/3 and QUIC, providing multiple streams that won't block each other."
Source: https://webkit.org/blog/17862/. Tier: official (vendor). Read 2026-10-27. How sure: confirmed in the source.

**R2-TRN-F22.** Fiedler, from shipped-game practice, on TCP for real-time games:
- "whenever a packet is dropped it has to stop and wait for that data to be resent";
- "On the client game objects stop receiving updates so they appear to be standing still";
- "My recommendation is not only that you use UDP, but that you only use UDP for your game protocol";
- "TCP tends to induce packet loss in UDP packets", hence "Don't mix TCP and UDP!".

The article dates from 2008, before QUIC. The mixing warning is about running TCP and UDP flows side by side.
Source: https://gafferongames.com/post/udp_vs_tcp/ ("UDP vs. TCP", Glenn Fiedler, 2008-10-01). Tier: shipped-game practitioner. Read
2026-10-27. How sure: confirmed in the source.

**R2-TRN-F23.** Priorities within a session are up to the application.
- Draft -16 §3.4: "WebTransport does not provide any priority signaling mechanism for streams and datagrams" within a session.
  RFC 9218 scheduling is "RECOMMENDED".
- The W3C API adds `sendOrder` and `sendGroup` on streams, where higher send order "MUST starve" lower within a group. Per the
  BCD summary, Chrome lacks `sendOrder` (Firefox 119, Safari 26.4).

Sources: https://datatracker.ietf.org/doc/html/draft-ietf-webtrans-http3 §3.4; https://www.w3.org/TR/webtransport/; BCD. Tier:
official. Read 2026-10-27. How sure: confirmed in the source for the quoted fragments; the browser rows are inferred.

#### Congestion control

**R2-TRN-F24.** QUIC's standard congestion control can shrink the send window to two packets. RFC 9002 (May 2021), §7:
- "This document specifies a sender-side congestion controller for QUIC similar to TCP NewReno."
- "A sender can unilaterally choose a different algorithm to use, such as CUBIC".
- §7.7: "A sender SHOULD pace sending of all in-flight packets based on input from the congestion controller."
- §7.6: after persistent congestion, "the sender's congestion window MUST be reduced to the minimum congestion window
  (kMinimumWindow)", where §7.2 says "The RECOMMENDED value is 2 * max_datagram_size".
- §7.2: the initial window is "ten times the maximum datagram size (max_datagram_size)," "while limiting the window to the larger
  of 14,720 bytes or twice the maximum datagram size."

Source: https://www.rfc-editor.org/rfc/rfc9002.html. Tier: official. Read 2026-10-27. How sure: confirmed in the source.

**R2-TRN-F25.** The server picks its own congestion controller in quinn 0.11.12.
- The `congestion` module offers Cubic ("The RFC8312 congestion controller, as widely used for TCP"), NewReno ("A simple,
  standard congestion controller") and Bbr ("Experimental! Use at your own risk."), behind pluggable `Controller` and
  `ControllerFactory` traits. The default is not stated there, and the `TransportConfig` example uses `NewRenoConfig`.
- `Connection::send_datagram`: "Transmit `data` as an unreliable, unordered application datagram"; "They may be lost or delivered
  out of order".
- `send_datagram_wait` "will wait for buffer space during congestion conditions".
- `rtt()`: "Current best estimate of this connection's latency (round-trip-time)".
- `congestion_state` exists "for debugging purposes".

Sources: https://docs.rs/quinn/latest/quinn/congestion/index.html; https://docs.rs/quinn/latest/quinn/struct.Connection.html;
https://docs.rs/quinn/latest/quinn/struct.TransportConfig.html (all 0.11.12). Tier: official. Read 2026-10-27. How sure: confirmed in
the source.

**R2-TRN-F26.** All WebTransport sessions on one connection share its congestion control. Draft -16 §8: "all of the WebTransport
sessions share both congestion control and flow control context". Datagrams and streams on one connection therefore compete for
the same window.
Source: https://datatracker.ietf.org/doc/html/draft-ietf-webtrans-http3 (-16) §8. Tier: official. Read 2026-10-27. How sure:
confirmed in the source for the quote; the competition is inferred.

**R2-TRN-F27.** The W3C stats dictionary defines `smoothedRtt`, `rttVariation`, `minRtt`, `estimatedSendRate` and
`atSendCapacity`, plus datagram counters (`droppedIncoming`, `expiredIncoming`, `expiredOutgoing`, `lostOutgoing`).
`bytesAcknowledged` is at risk.
Source: https://www.w3.org/TR/webtransport/ (stats section). Tier: official. Read 2026-10-27. How sure: inferred (fetch summary).
Usable only on Safari today (F9).

#### Rust crates (versions on 2026-10-27)

**R2-TRN-F28.** quinn, the QUIC base both WebTransport crates build on, is mature and active: `"newest_version":"0.11.12"`,
`"updated_at":"2026-09-14T10:08:47.662258Z"`, `"created_at":"2018-10-02T20:18:56.824108Z"`, `"downloads":379896223`.
Sources: https://crates.io/api/v1/crates/quinn; docs.rs quinn 0.11.12. Tier: official (registry). Read 2026-10-27. How sure:
confirmed in the source.

**R2-TRN-F29.** wtransport 0.7.2: `"newest_version":"0.7.2"`, `"updated_at":"2026-08-11T22:08:08.540291Z"`,
`"created_at":"2023-05-25T19:45:52.658071Z"`, `"downloads":1083504`.
- Cargo.toml: `rust-version = "1.88"`, quinn 0.11.6 with runtime-tokio, rustls 0.23.23, rcgen 0.14.5 optional, and features
  ring (default), aws-lc-rs, fips.
- Its README: "The *WTransport* library, while functional, is not considered completely production-ready." "It should be used
  with caution and may undergo changes as the WebTransport specification evolves."
- Its API offers `send_datagram`, `receive_datagram`, `max_datagram_size`, `rtt`, open and accept for both stream kinds, and
  `quic_connection()` ("Returns a reference to the inner QUIC connection.", feature `quinn`), so quinn's stats and congestion
  state stay reachable.
- The GitHub page shows 716 stars and 29 open issues (fetch summary).

Sources: https://crates.io/api/v1/crates/wtransport; https://raw.githubusercontent.com/BiagioFesta/wtransport/master/wtransport/Cargo.toml;
https://raw.githubusercontent.com/BiagioFesta/wtransport/master/README.md; https://docs.rs/wtransport/latest/wtransport/struct.Connection.html
(0.7.2); https://github.com/BiagioFesta/wtransport. Tier: official (registry, source, docs); README and repository page: other
(the maintainer's own words). Read 2026-10-27. How sure: confirmed in the source, except the star and issue counts (inferred).

**R2-TRN-F30.** web-transport-quinn 0.13.5 belongs to a family of crates.
- Registry: `"newest_version":"0.13.5"`, `"updated_at":"2026-10-24T00:53:07.167572Z"`,
  `"created_at":"2024-04-06T05:33:00.667490Z"`, `"downloads":2033705`.
- It moves fast. The registry summary lists six releases from 0.13.0 (2026-09-27) to 0.13.5, and 0.12.0 on 2026-08-07. Whether
  these minor bumps broke the API was not checked (no changelog read).
- README:
  - "web-transport-quinn mirrors the Quinn API, abstracting away the HTTP/3 setup."
  - "web-transport-wasm wraps the browser API".
  - "web-transport-trait defines an async trait, currently implemented by web-transport-quinn and qmux."
  - "qmux implements QMux (draft-ietf-quic-qmux) over TCP/TLS/WebSocket, with backwards compatibility" "for the legacy
    WebTransport-over-WebSocket wire format."
- The README makes no stability statement.

Sources: https://crates.io/api/v1/crates/web-transport-quinn; https://raw.githubusercontent.com/moq-dev/web-transport/main/README.md;
https://github.com/moq-dev/web-transport. Tier: official (registry); README: other. Read 2026-10-27. How sure: confirmed in the source
for the newest version and the README quotes; inferred for the release list.

**R2-TRN-F31.** The h3 route is experimental and has gone quiet.
- h3's README: "The `h3` crate is still very experimental."
- h3-webtransport: `"newest_version":"0.1.2"`, `"updated_at":"2025-05-06T21:21:35.491571Z"`, three releases since 2023-11-06.

Sources: https://github.com/hyperium/h3; https://crates.io/api/v1/crates/h3-webtransport. Tier: other (README); official
(registry). Read 2026-10-27. How sure: confirmed in the source.

**R2-TRN-F32.** QMux is an early IETF draft that offers QUIC's operations over a TCP-like stream. draft-ietf-quic-qmux-02
(2026-07-06) is a "WG Document" of the quic WG, intended Proposed Standard. Abstract: "QMux version 1 provides, over
bi-directional streams such as TLS," "the same set of stream and datagram operations that applications rely upon in QUIC version
1."
Source: https://datatracker.ietf.org/doc/draft-ietf-quic-qmux/. Tier: official. Read 2026-10-27. How sure: confirmed in the source for
the abstract; the state fields are inferred.

**R2-TRN-F33.** The WebSocket crates are mature and widely used.
- tokio-tungstenite: `"newest_version":"0.30.0"`, `"updated_at":"2026-07-11T12:58:44.438987Z"`,
  `"created_at":"2017-03-17T17:27:49.033103Z"`, `"downloads":330093365`.
- axum: `"newest_version":"0.8.9"`, `"updated_at":"2026-04-14T07:55:20.258398Z"`, `"downloads":548097362`. It has a WebSocket
  extractor: "Handle WebSocket connections.", "Available on crate feature `ws` only.", and its dependency list includes
  tokio-tungstenite ^0.29.0.

Sources: https://crates.io/api/v1/crates/tokio-tungstenite; https://crates.io/api/v1/crates/axum;
https://docs.rs/axum/latest/axum/extract/ws/index.html (0.8.9). Tier: official. Read 2026-10-27. How sure: confirmed in the source;
"axum's ws runs on tokio-tungstenite" is inferred from the dependency list.

#### Browser platform and protocol details

**R2-TRN-F34.** WebSocket has been in all four browsers for over a decade and works in workers. Top-level `"version_added"`:
chrome "5", edge "12", firefox "11" (with the `"Moz"` prefix from 7 to 11), safari "5". `worker_support`: chrome "5", firefox
"37", safari "5". The fetch summary adds binaryType since Chrome 15, Firefox 11 and Safari 6.
Source: https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/WebSocket.json. Tier: official. Read 2026-10-27. How sure:
confirmed in the source (quoted JSON); binaryType inferred.

**R2-TRN-F35.** WebTransport runs in Web Workers and needs HTTPS. MDN: "Note: This feature is available in Web Workers."; "This
feature is available only in secure contexts (HTTPS)"; "Note that the scheme needs to be HTTPS, and the port number needs to be
explicitly specified."
Source: https://developer.mozilla.org/en-US/docs/Web/API/WebTransport_API. Tier: official. Read 2026-10-27. How sure: confirmed in the
source.

**R2-TRN-F36.** The server must check where a connection comes from and may cut off abuse.
- Draft -16 §3.2: "the WebTransport server MUST verify the Origin header to ensure that the specified origin is allowed", and
  should answer 403 otherwise.
- §8: "Implementations SHOULD track the use of WebTransport features", and an endpoint "MAY treat activity that is suspicious as
  a connection error of type H3_EXCESSIVE_LOAD".

Source: https://datatracker.ietf.org/doc/html/draft-ietf-webtrans-http3 (-16). Tier: official. Read 2026-10-27. How sure: confirmed in
the source (quoted fragments).

**R2-TRN-F37.** Sessions can be drained before they close. Draft -16 §6 defines WT_CLOSE_SESSION (0x2843) and WT_DRAIN_SESSION
(0x78ae): "either endpoint can send a WT_DRAIN_SESSION (0x78ae) capsule". The W3C `draining` promise exists in Firefox 155 and
Safari 26.4, not in Chrome.
Sources: draft -16 §6; BCD WebTransport.json. Tier: official. Read 2026-10-27. How sure: confirmed in the source for the draft
quote; inferred for the browser rows.

**R2-TRN-F38.** The `protocols` option can carry the application protocol version. MDN: "The server may also reject the connection
if it doesn't support any of the protocols, in which case `ready` will reject with an error." Browser support: Chrome 143,
Firefox 155, Safari 26.4.
Sources: MDN WebTransport() constructor page; BCD. Tier: official. Read 2026-10-27. How sure: confirmed in the source for MDN; inferred
for the versions.

**R2-TRN-F39.** QUIC connections can survive a change of client address. RFC 9000 §1: QUIC "allows connections to continue after
changes in network topology or address mappings"; §5.1: "The primary function of a connection ID is to ensure that changes in
addressing at lower protocol layers" do not misroute packets.
Source: https://www.rfc-editor.org/rfc/rfc9000.html. Tier: official. Read 2026-10-27. How sure: confirmed in the source for the
fragments.

#### Legacy facts used (transport unit)

- **CS1** (fixed tick and input-command groundwork, `shamble-core.js:1479-1481`; marked **drop**). The legacy build declares an
  input command (seq, tick, move, aim, buttons bitmask) but never sends or reads it. There is no networking code at all. For v2,
  the transport carries no legacy format. Its smallest and most frequent message is the per-tick input command, which belongs in
  an unreliable datagram (it is small and the next one supersedes it). The format itself belongs to the snapshots and
  prediction unit.
- **GP2** (input mapping and listeners; `shamble-game.js:3252-3294`). Many actions fire at event time (reload, ability, grenade
  release, bow release, E, weapon switch). The inventory's v2 note turns them into per-tick input bits and events. For the
  transport this means one-shot actions must survive the loss of a datagram. Either they ride as bits repeated in every
  redundant input copy, or they go on a reliable stream. Which one is a protocol choice for the prediction unit.
- **CS11** (headless SIM fence). The v2 simulation is a Rust crate with no browser dependency, so the transport stays outside
  the simulation. It decodes messages into simulation inputs and encodes snapshots out, and the simulation never touches a
  socket.
- **STATUS.md:70, ADR-001 inputs.** Three of them come in over the transport's reliable messages: humans take over bot slots,
  the server repeats the team allow-list (`teamOf`), and map data needs a full schema check on the server (SEC-MDL-2). So
  reliable messages from clients need server-side validation. No transport measurement exists in the legacy build.

### Brief question 5: rooms and private matches (rooms unit)

Unit **rooms**. Draft: `.checkpoints/v2-research/netcode-architect-r2-rooms-draft.md`; notes with every source summary and its exact quotes: `.checkpoints/v2-research/netcode-architect-r2-rooms.md`. Author: netcode-architect, 2026-10-27.

Brief question (quoted): "rooms and private matches" (V2_PHASE0_BRIEF §4, row R2). Unit scope as assigned: the room and
match lifecycle (create, fill, start, end, tear down); one process per room versus many rooms per process at 48+
players; joining with a short-lived signed token from a lobby, and reconnecting; private matches with room codes,
including VISION_V2 §3's organized play; bots filling empty slots on the server; modes, team count, team size and
factions as data (VISION_V2 §6.1); guest play first (VISION_V2 §5); what a spike must prove.

Every source read 2026-10-27. "Confirmed in
the source" means the claim rests on sentences quoted verbatim below; "inferred" means a conclusion drawn from them,
or a claim seen only as a WebFetch paraphrase.

**Room and match lifecycle (hosting services)**

- **R2-ROOM-F1.** In Amazon GameLift Servers, a server process reports ready, the service picks a process that is not
  hosting a game session, sets the session to ACTIVATING, and the process calls `ActivateGameSession()` only when it can
  accept players; one process hosts one game session at a time. Verbatim: "`ProcessReady()` to communicate readiness to
  host a game session."; "searches those fleets for an active server process that's not hosting a game session";
  "Changes the server process's number of game sessions to `1`."; "When the server process is ready to accept player
  connections, it calls the server SDK operation `ActivateGameSession()` and waits for player connections." ·
  https://docs.aws.amazon.com/gameliftservers/latest/developerguide/gamelift-sdk-interactions.html, "Initializing a
  game server process" and "Creating a game session" · official · read 2026-10-27 · confirmed in the source (the
  one-session-per-process reading is inferred from the two quoted sentences).
- **R2-ROOM-F2.** GameLift reserves a player slot before the player connects and the game server validates the player
  on connect; an unvalidated reservation times out after 60 s and reopens the slot; a dropped player's session becomes
  COMPLETED. Verbatim: "Creates a new `PlayerSession` object and sets the status to `RESERVED`."; "the game server
  process calls the service API operation `AcceptPlayerSession()` to validate the player session ID. The server process
  either accepts or rejects the connection."; "TIMEDOUT -- A player session request was received, but the player did
  not connect and/or was not validated within the timeout limit (60 seconds)."; "COMPLETED -- The player connection has
  been dropped." · S1 ("Adding a player to a game", "Removing a player") and
  https://docs.aws.amazon.com/gameliftservers/latest/apireference/API_PlayerSession.html, "Status" (API version
  gamelift-2015-10-01) · official · read 2026-10-27 · confirmed in the source.
- **R2-ROOM-F3.** GameLift ends a session by the process ending: the server ends the session and calls
  `ProcessEnding()`; the service then marks session and process TERMINATED and may start a new process. Health checks
  must be answered within one minute, and three consecutive unhealthy reports shut a process down. A session that does
  not become active within the activation timeout (1 to 600 s) is terminated. Verbatim: "The game server process ends
  the game session and initiates process shut down by calling the server SDK operation `ProcessEnding()`."; "The server
  process must respond healthy or not healthy within one minute."; "Amazon GameLift Servers automatically shuts down
  server processes that are reported unhealthy for three consecutive reports."; "If the game session does not become
  active before the timeout, it is ended and the game session status is changed to `TERMINATED`." · S1 "Shutting down
  the game session"; https://docs.aws.amazon.com/gameliftservers/latest/developerguide/fleets-multiprocess.html
  "Optimize a fleet for multiple processes"; https://docs.aws.amazon.com/gameliftservers/latest/apireference/API_RuntimeConfiguration.html
  "GameSessionActivationTimeoutSeconds" · official · read 2026-10-27 · confirmed in the source.
- **R2-ROOM-F4.** GameLift packs density by running several processes per instance, not several sessions per process;
  the right count is found by testing. Verbatim: "You can set up a managed EC2 fleet's runtime configuration to run
  multiple game server processes per instance. This uses your hosting resources more efficiently."; "An Amazon GameLift
  Servers instance is limited to 50 processes running concurrently."; "Test different configurations for your game to
  find the right number of processes for your preferred instance type." · S4 (intro, "Choose the number of processes
  per instance"); S3 (description) · official · read 2026-10-27 · confirmed in the source.
- **R2-ROOM-F5.** PlayFab Multiplayer Servers uses the same model: a server is allocated once (StandingBy to Active),
  is never returned to the pool, is protected from scale-down while Active, and the process should exit when the
  session ends. Verbatim: "A Game server can never transition back to StandingBy state when it is in the Active state.
  Once the game state is set to Active, the only way to get a new game server in StandingBy state is if your game server
  process exits."; "The fact that the game server is active means that it will not be taken down during potential Build
  scale downs"; "When the game session ends, you should gracefully terminate the game server process so you can get a
  new game server." · https://learn.microsoft.com/en-us/gaming/playfab/multiplayer/servers/multiplayer-game-server-lifecycle,
  "States of a game server", "Terminating a game server process" (ms.date 2022-01-26, updated 2025-05-01) · official ·
  read 2026-10-27 · confirmed in the source.
- **R2-ROOM-F6.** Agones (the googleforgames/agones project's docs) documents many sessions per game server as a
  cheaper but harder pattern; its Beta "rooms" Counter lets allocation prefer an already-allocated server with spare
  room capacity. Verbatim (as quoted by WebFetch): "it can be a more economical use of resources to run multiple
  concurrent game sessions"; "this requires more management on behalf of the integrated game server process and
  external systems"; "since it works around the common Kubernetes and/or Agones container lifecycle." ·
  https://raw.githubusercontent.com/googleforgames/agones/main/site/content/en/docs/Integration%20Patterns/high-density-gameservers.md
  (main branch; front-matter date 2021-08-31; Counters marked Beta, gate CountsAndLists) · official (project docs) ·
  read 2026-10-27 · confirmed in the source for the quoted sentences; the Counter mechanics are inferred (paraphrase).

**Many rooms in one Rust process**

- **R2-ROOM-F7.** Tokio tasks are cooperative, and blocking work stalls every task on that worker thread;
  `spawn_blocking` is for work that ends, and work that runs indefinitely belongs on its own thread. Verbatim: "a task
  is allowed to run until it yields"; a blocking operation "would block the entire thread, preventing other tasks from
  running"; `spawn_blocking` is "intended for non-async operations that eventually finish on their own"; for "workloads
  that run indefinitely or for extended periods" use `thread::spawn`; "tasks spawned using spawn_blocking cannot be
  aborted because they are not async." · https://docs.rs/tokio/latest/tokio/task/index.html and
  https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html (tokio 1.54.1) · official · read 2026-10-27 ·
  confirmed in the source (quoted fragments); that the page names `thread::spawn` for long-running work is from
  WebFetch's paraphrase around the quote, so that link is inferred.
- **R2-ROOM-F8.** In Rust an uncaught panic ends only its own thread and is reported to `join`, unless the binary is
  built with `panic = "abort"`, which terminates the whole process; the default thread stack is 2 MiB. Verbatim: "If
  the panic is not caught the thread will exit, but the panic may optionally be detected from a different thread with
  `join`."; panics "can nonetheless be caught (unless compiling with `panic=abort`) with `catch_unwind`"; Cargo
  `"abort"`: "Terminate the process upon panic."; "The default stack size is platform-dependent and subject to change.
  Currently, it is 2 MiB on all Tier-1 platforms." · https://doc.rust-lang.org/std/thread/index.html "Panics", "Stack
  size" (page shows Rust 1.99.1, 2026-10-13); https://doc.rust-lang.org/cargo/reference/profiles.html "panic" ·
  official · read 2026-10-27 · confirmed in the source.

**Joining with a token, and reconnecting**

- **R2-ROOM-F9.** Glenn Fiedler's browser-game design: a web backend authenticates the player and returns a short-lived
  connect token over HTTPS; it is signed and encrypted with a key shared by the backend and the game servers, lists the
  server addresses it is valid for, and a server rejects a token it has already seen and a second connection with the
  same client id. Verbatim: "the web backend performs authentication and when a client wants to play, it makes a REST
  call to obtain a connect token"; "Connect tokens are short lived"; "checks if the connect token has already been used
  by searching a short history of connect token HMACs"; "This prevents one connect token from being used to connect
  multiple clients."; "only one client by unique client id may be connected at any time." ·
  https://gafferongames.com/post/why_cant_i_send_udp_packets_from_a_browser/ (2017-02-26) · shipped game (engineer's
  articles, named in the brief) · read 2026-10-27 · confirmed in the source.
- **R2-ROOM-F10.** The netcode 1.02 standard makes a connect token single-use and tells a rejected client to fetch a new
  one; a server ignores a token that is expired, not addressed to it, or for a client id already connected. Verbatim:
  "A connect token is therefore usable for exactly one connection."; "The correct client response is to request a new
  connect token from the web backend."; "If the connect token expire timestamp is <= the current timestamp, ignore the
  packet."; "If a client with the client id contained in the private connect token data is already connected, ignore
  the packet." · https://raw.githubusercontent.com/mas-bandwidth/netcode/main/STANDARD.md (netcode 1.02, main branch) ·
  shipped game (engineer's protocol spec) · read 2026-10-27 · confirmed in the source.
- **R2-ROOM-F11.** RFC 7519 defines the claims such a token needs: an audience the receiver must match, an expiry with
  small clock-skew leeway, and a unique id. Verbatim: "Each principal intended to process the JWT MUST identify itself
  with a value in the audience claim." ... "then the JWT MUST be rejected."; "identifies the expiration time on or after
  which the JWT MUST NOT be accepted for processing."; "Implementers MAY provide for some small leeway, usually no more
  than a few minutes, to account for clock skew."; "The identifier value MUST be assigned in a manner that ensures that
  there is a negligible probability that the same value will be accidentally assigned to a different data object" ·
  https://www.rfc-editor.org/rfc/rfc7519.html §4.1.3, §4.1.4, §4.1.7 (May 2015) · official · read 2026-10-27 · confirmed
  in the source.
- **R2-ROOM-F12.** Neither browser transport lets the page set an HTTP auth header: the WebSocket constructor takes only
  a URL and sub-protocols, and the WebTransport constructor's options are `allowPooling`, `congestionControl`,
  `protocols`, `requireUnreliable` and `serverCertificateHashes`; a WebTransport URL must be https with an explicit port.
  Verbatim: "`new WebSocket(url)` `new WebSocket(url, protocols)`"; "The scheme must be HTTPS, and the port number needs
  to be explicitly specified." · https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket (modified
  2026-06-22) and https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/WebTransport (modified 2026-10-12) ·
  official · read 2026-10-27 · confirmed in the source for the signatures and option list; "no auth header" is inferred
  (neither page says it in words).
- **R2-ROOM-F13.** A WebTransport session is an extended CONNECT whose `:authority` and `:path` name the server
  resource, so one endpoint can tell sessions apart by path; servers should rate-limit new sessions. Verbatim: "In order
  to create a new WebTransport session, a WebTransport client sends an HTTP extended CONNECT request."; "Both the
  :authority and the :path value MUST be set; these fields identify the desired WebTransport server resource.";
  "Servers SHOULD limit the rate of incoming WebTransport sessions on HTTP/3 connections." ·
  https://datatracker.ietf.org/doc/draft-ietf-webtrans-http3/ (draft-ietf-webtrans-http3-16, 2026-07-06, work in
  progress) §3.2, §5.2 · official (IETF draft) · read 2026-10-27 · confirmed in the source; routing rooms by path is
  inferred.
- **R2-ROOM-F14.** A guessable secret needs at least 64 bits of entropy, and identifiers in URLs leak through logs,
  history and the Referer header. Verbatim: "Session identifiers must have at least 64 bits of entropy to prevent
  brute-force session guessing attacks."; "Each "bit" of entropy doubles the number of possible outcomes."; IDs in the
  URL "might disclose the session ID (in web links and logs, web browser history and bookmarks, the Referer header or
  search engines)."; "Sensitive data like the session ID should not be included in the logs" ·
  https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html "Session ID Entropy" and its passages on IDs in URLs and on logging (no
  page date) · official (OWASP) · read 2026-10-27 · confirmed in the source.

**Bots, guests and private rooms**

- **R2-ROOM-F15.** Team Fortress 2's server fills a quota with bots: in 'fill' mode bots make up the gap to N players,
  bots can wait for the first human, and bots leave to make room for humans. Verbatim (ConVar help strings): "If
  'fill', the server will adjust bots to keep N players in the game, where N is bot_quota."; "If 'match', the server
  will maintain a 1:N ratio of humans to bots, where N is bot_quota."; "If nonzero, bots wait until a player joins
  before entering the game."; "If nonzero, bots will automatically leave to make room for human players."; code:
  `desiredBotCount = MAX( 0, desiredBotCount - nNonTFBotsOnGameTeams );` ·
  https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/tf/bot/tf_bot_manager.cpp ·
  shipped game (source code) · read 2026-10-27 · confirmed in the source.
- **R2-ROOM-F16.** PlayFab recommends an anonymous login for new players (no input), then offering a recoverable
  login; an anonymous account is lost with the device, and a simple custom id lets attackers take accounts. Verbatim:
  "They require zero input from the player, so there's no friction to the first time user experience"; "Your game
  should use an anonymous login for creating a new account and linking new devices to an existing account."; "once the
  anonymous login is complete, you should provide the option to add recoverable login credentials"; "If the player
  loses or breaks their device, the account is lost"; "you must generate your custom IDs with care. If they're too
  simple, you risk hackers stealing the accounts of others." ·
  https://learn.microsoft.com/en-us/xbox/playfab/identity/player-identity/login/login-basics-best-practices (ms.date
  2025-02-20, updated 2025-10-31) · official · read 2026-10-27 · confirmed in the source.
- **R2-ROOM-F17.** PlayFab Lobby fixes a lobby's capacity, access policy, owner and owner-migration policy at creation;
  the owner alone updates lobby data; under the Automatic policy ownership passes to another connected member; a
  client-owned lobby is deleted when its last member leaves. Verbatim: "Several important settings for a lobby are
  configured during creation: maxMemberCount, accessPolicy, owner, and ownerMigrationPolicy."; "accessPolicy defines
  who may discover the lobby's connection string"; "owner defines the lobby's owner which has special permissions to
  update data on the lobby."; "The Lobby service automatically assigns another connected member as owner when the
  current owner leaves or disconnects."; "For client-owned lobby, the lobby is always deleted when the last member
  leaves." · https://learn.microsoft.com/en-us/gaming/playfab/features/multiplayer/lobby/create-a-lobby (updated
  2026-03-24) and https://learn.microsoft.com/en-us/gaming/playfab/features/multiplayer/lobby/ownership-changes
  (updated 2025-09-10) · official · read 2026-10-27 · confirmed in the source.
- **R2-ROOM-F18.** In the process-per-session model each session is reached at its own address and port. Verbatim:
  "When joining a game session, the game client connects directly to a game session using the game session's unique IP
  address and port number." · S1 (introduction) · official · read 2026-10-27 · confirmed in the source.

#### Legacy facts used (rooms unit)

| Row / table | Fact | What it tells v2 rooms |
|---|---|---|
| CD10, `TEAMS` table | Three teams as data (allies, soviets, empire); an implicit "order matters" rule makes soviets the first enemy. | Teams already data; a mode must name its teams and factions explicitly, never by table order. |
| CD11, `TUNE` table | `teams.size` 4, `ff` false; bot sight budget 480 checks/s, sized for 12 soldiers (STATUS at f2ccc13 :68). | Team size and friendly fire are mode data; the shared bot budget must be re-sized per room for 48+. |
| GC2 | 12 fixed slots (0 = you, 1-3 allies, 4-7 soviets, 8-11 empire); slot id = SIM id; unused teams parked when they have no HQ; perfbots at 48 soldiers: combat 0.80-1.00 ms per update step (JS, headless). STATUS :68: "humans take over bot slots". | Keep a slot table with slot id = sim id, sized from the mode; park unused teams by the mode, not "has an HQ"; the 48-soldier JS number is the only CPU figure we have (order of magnitude only). |
| GN2, GN3, GC9 | Bot records mix sim and view state; unit picked with `Math.random`; per-slot deterministic spawn chain; respawn 2.0 s / 4.0 s (`T` table). | Bots become sim-owned slot occupants on the server with seeded randomness; spawn rules come from LOOP_V2 §2. |
| GW1 | `PLAYER_FACTION` / `PLAYER_TEAM` globals fix one local player to the Allies. | Team and squad become per-connection state assigned by the room. |
| GW2 / GE1, `RULES_DEFAULT` | Rules are per-map data merged over defaults; the merge accepts any key, including `__proto__` (SEC-MDL-2). | Mode and rules data must be schema-validated by the server at load. |
| GW11 | The boot map comes from the URL (`?map=tri`). | The room (server) chooses the map from the mode, never the client. |
| GE12 | `resetRound` mutates many globals in place; `gameOver` freezes the world. | A room should start each match from a fresh sim instance, not an in-place reset. |
| GP13, PG8 | Pause, shop and tutorial stop `update()`; "Restart round" is a player button. | A room never pauses for one player; restart and end belong to the room (or the private-room host). |
| CS1 | A 1/60 s fixed step and an input-command record (seq, tick, move, aim, buttons) exist but nothing reads them. | Bots and humans can share one input-command path into the sim. |
| GC1 | One team choke point; the server must repeat the team allow-list. | The room validates every team id in joins and host commands. |
| CD2, GE15 | Career lives in localStorage (KVStore), loaded with no validation. | Client storage keeps only the guest credential and settings; progression is server-side (VISION §5, §6.10 LATER). |

### Brief question 6: learn from shipped games (all units)

The brief names four sources. This index points to the findings above that come from each; it adds no claim.

| Named source | What the units read | Findings |
|---|---|---|
| Valve's Source networking | The Source SDK 2013 code (TF2 included) and the code of Valve's GameNetworkingSockets library. The Valve Developer Community wiki pages returned HTTP 403 (Sources not read). | R2-SYNC-F20, F26, F28–F33 (F34 is the sync unit's own inference from F27 and F28); R2-BUD-F28–F31; R2-INT-F8–F13; R2-ROOM-F15 |
| Overwatch's GDC talks | Not read: video only, and GDC Vault redirected to a page outside the allow list. | None. R2-SYNC-OQ3, R2-SYNC-OQ10 and R2-BUD-OQ1 name what they could settle. |
| Glenn Fiedler's articles | Ten articles on gafferongames.com. The units tier them differently (R2-XU-OQ16). | R2-SYNC-F1–F7, F12, F16, F18 (with Quake III), F21, F22; R2-BUD-F18–F23, F32–F35; R2-INT-F14, F15; R2-TRN-F20, F22; R2-ROOM-F9 |
| Quake 3 | The Quake III Arena GPL source release (id Software); the Quake II source for one finding. | R2-SYNC-F8–F10, F13–F15, F17–F19, F23–F25, F27, F35; R2-BUD-F24–F27; R2-INT-F1–F4, F6, F7 (R2-INT-F5 is Quake II) |
| Also tiered "shipped game" by a unit | The netcode 1.02 standard (mas-bandwidth/netcode, `STANDARD.md`). | R2-ROOM-F10 |

## Options

Per brief question, as each unit set them out (option labels are local to the unit).

### Brief question 1: snapshots, client prediction, interpolation and lag compensation for 48+ players (sync unit)

#### 3.1 State-sync model

| Option | How it works | Fit and costs | Risk |
|---|---|---|---|
| A. Deterministic lockstep (inputs only) | Every peer runs the whole sim from everyone's inputs. | Lowest bandwidth. But latency is the worst player's (F1), it stalls on a missing input (F4), late join is hard (F2), and it needs bit-exact results on every client (F3). §6.1 rooms and late joining suffer; 48 browsers must all agree. | High: one divergent client breaks the match; every client holds the whole state, so a wallhack sees everything (inferred). |
| B. Predict-everything with rollback (inputs, the server re-simulates) | Clients run the full sim ahead and roll back on corrections. | I read no source on this in this run (the Overwatch talk, a likely example, is video only). The client cost grows with the entity count at 48+. | Unassessed here. |
| C. State synchronisation (input + state, both sides simulate) | Fiedler's priority accumulator plus extrapolation. | Low bandwidth, but "approximate and lossy" with divergence and pops (F6). Built for physics objects, not for authority over many players. | Medium. |
| D. Full snapshots, no delta | The server sends the whole visible state every snapshot. | Simplest; robust to loss. Bandwidth scales with entities × rate (bandwidth unit). | Bandwidth at 48+. |
| E. Delta snapshots against each client's acknowledged snapshot | Quake III's shipped model (F8, F9) and Fiedler's (F7). Interest management picks the entities. | Tolerates loss and reordering (needed by F11). Late join and rejoin start from the per-entity baselines (fits §6.1 rooms). No client determinism is needed for correctness (F5). The server encodes per client. | Low; well proven. Spikes when a client's baseline expires (a full or baseline-relative snapshot). |

#### 3.2 Client prediction scope

| Option | What the client predicts | Fit |
|---|---|---|
| A. None | Nothing: every input waits a round trip. | Fails "fair up to about 100 ms" (§4) and LOOP_V2 §7 "the shot feels instant". |
| B. Own movement only | Quake III and QuakeWorld style (F12, F13). | Proven; cheap; corrections are rare when the code matches. |
| C. Own movement, own weapon state, cosmetic shot feedback, own vehicle | Like B, plus ammo, cooldowns, reload and active reload, plus muzzle flash and tracer from the shared spread seed (F20). The server confirms hits (LOOP_V2 §7 split feedback). Plus predictable pickups and touches as in Quake III (F17). | Serves LOOP_V2 §7 directly. The weapon state already lives in the sim (GC4). |
| D. Abilities and projectiles too | Molotov throw, Riot Shield raise, Banzai, bow bolt, all predicted and rolled back if wrong. | Better feel for abilities. Needs cosmetic rollback of effects; I read no source on it in this run. |

#### 3.3 Correction smoothing

| Option | Behaviour | Source |
|---|---|---|
| A. Snap | Show the corrected state at once. | (baseline) |
| B. Linear error decay | Add the error to a visual offset that fades to zero over a fixed time (Quake III: 100 ms). | F15 |
| C. Exponential pull | Move 10 % of the way per update; snap above 2 m; snap velocity. | F16 |

#### 3.4 Interpolation delay and clock

| Option | Delay | Source |
|---|---|---|
| A. Under one snapshot interval, then extrapolate (Quake III) | The clock approaches the newest snapshot. The delay is small, but every late or lost snapshot means extrapolating. | F23 |
| B. A fixed number of snapshot intervals | For example 2–3 intervals (the netcode default principles' starting point, about 100 ms). | F21 (the rule); Source's own 100 ms default not read (F26) |
| C. Adaptive | Start at 3 intervals; widen with measured jitter and loss; narrow when the link is clean. The clock drifts slowly as in Quake III. | F21, F23 |

#### 3.5 Lag compensation for hitscan

| Option | Behaviour | Fit |
|---|---|---|
| A. None (Quake III) | Shooters lead by RTT + interpolation (F27, F34). | At 100 ms and 9 m/s that is about 1.8 m of lead: unfair for §4. |
| B. Server rewind to the shooter's view time, capped (Source) | History of hit volumes; the shot resolves against the targets as the shooter saw them (F28–F32). | Fair for the shooter up to the cap; the victim's "hit behind cover" window grows with it (F34). |
| C. Client-side hit detection, server trusts it | The client says what it hit. | An authority leak (trivially cheatable); rejected by the default principles ("never client state"). |

#### 3.6 Projectiles (frags, Molotovs, bolts, later Javelin rockets)

| Option | Behaviour | Fit |
|---|---|---|
| A. Server only, shown when the server's spawn arrives | Quake III, with a fixed 50 ms prestep (F35). | Simple; the owner sees their throw one round trip late. |
| B. Server-simulated; the owner shows a cosmetic copy at once and swaps it for the server's | The owner sees it instantly; others see the server's. | Serves LOOP_V2 §7. Needs a match rule (owner + command sequence) and a hand-over without a pop. |
| C. Client-authoritative projectiles | The client says where it landed. | Authority leak: rejected. |

Within A and B, how far the server moves the new projectile forward on spawn: none, a fixed prestep (Quake III: 50 ms),
or the shooter's latency up to the cap. That is open (OQ6).

#### 3.7 Aim encoding (aiming at height, §6.2)

| Option | Bits per command (proposed precision) | Fit |
|---|---|---|
| A. Direction: yaw + pitch, 16 bits each (as Quake III's angles, F19) | 32 | Compact. An error in the predicted fire origin shifts the whole ray, so it misses the clicked point by that error. |
| B. World aim point, quantised (positions to 1 cm as in the default principles) | About 60–75, depending on map bounds (the bandwidth unit sizes it) | Keeps the point the player clicked at any height (GC3), whatever the origin error. The server rebuilds the ray from its own fire origin to that point. Easy to check (range, LOS). |

### Brief question 2: tick rate and bandwidth budgets (budgets unit)

#### A. Simulation tick and snapshot rate (with R1's tick unit)

| Option | Tick / snapshot / input send | Fairness at 100 ms RTT (F29, F32) | Downstream at 48 (table D) | Server CPU | Fit |
|---|---|---|---|---|---|
| T1 | 60 Hz / 30 Hz / 60 Hz | interp 100–150 ms; rewind 50 + 100..150 = 150–200 ms, within the ~200 ms cap of the netcode default principles (`.claude/agents/netcode-architect.md`:35-36) | 146 kbps | 2 × T2 for the step; snapshot encode at 30 Hz | Legacy `FIXED_DT`, Source's 66.7 Hz tick (F28); input timing error ±8 ms |
| T2 | 30 Hz / 30 Hz / 30 Hz | same interp and rewind as T1; +8 ms average input wait | 146 kbps | lowest step cost; the brief's own pass-mark example uses 30 Hz | Fire intervals quantize harder (WEAPONS row) |
| T3 | 60 Hz / 60 Hz / 60 Hz | interp 50–85 ms; rewind 100–135 ms | 286 kbps | encode 2 × T1 | Over Fiedler's 256 kbps example budget (F34) at 48 players |
| T4 | 20 Hz / 20 Hz / 30 Hz (Quake III defaults, F26) | interp 150–200 ms; rewind 200–250 ms: breaks the 200 ms cap at 100 ms RTT | 99 kbps | lowest | 1999 defaults sized for modems ("rate" 3000 B/s) |

Rewind = one-way latency (assumption: half of a symmetric 100 ms RTT) + interpolation delay, the Source formula (F29).
Interpolation delay = 3 snapshot intervals, plus up to one interval of jitter allowance (F32).

[Assembler's note: "table D" in tables A and B points to the results table of the worked bandwidth estimate below, not to option table D (packet size).]

#### B. Downstream encoding (each step applies on top of the previous)

| Option | What it is | 48 players at 30 Hz (table D) | Complexity / risk |
|---|---|---|---|
| E1 | Raw floats, full state every snapshot | about 536 kbps, 2 datagrams per snapshot | trivial; every snapshot fragmented in two |
| E2 | Quantized (centimetres, 16-bit angles), full state | about 251 kbps; a full snapshot is about 1021 bytes, close to the 1171-byte limit (over it with live projectiles or at 64 players) | low |
| E3 | E2 + delta against the last acknowledged snapshot, lists of changed entities, projectiles as spawn events + deterministic flight | about 146 kbps; about 609 bytes per snapshot | medium: per-client baseline ring (Quake III keeps 32, F25), application acks (F8) |
| E3b | E3, but projectiles re-sent in every snapshot | about 188 kbps | medium; needs no deterministic projectile flight on the client |
| E4 | E3 + arithmetic (range) coding | about 25 % below E3 by Fiedler's single data point (F23) | high: a context model to build and tune; defer until E3 is measured |

#### C. Upstream (inputs)

| Option | What it is | Per client (IPv4) | Notes |
|---|---|---|---|
| U1 | Send every 60 Hz tick: newest input + 3 previous, 8 bytes each | 44.6 kbps | an input is lost only if 4 packets in a row are lost (independent loss at 5 %: 0.05^4 ≈ 6 × 10⁻⁶, arithmetic) |
| U2 | U1 with the 3 repeats delta-coded against the newest (about 2 bytes each, assumption) | 36.0 kbps | small extra code |
| U3 | Send at 30 Hz, two new ticks + two repeats (like Quake III's `cl_maxpackets` 30 with `cl_packetdup` 1, F26) | 22.3 kbps | adds up to one tick of input delay on half the inputs |

#### D. Packet size

| Option | What it is | Trade-off |
|---|---|---|
| P1 | Every datagram at most min(runtime max datagram size, 1150 bytes of WebTransport payload); a snapshot that does not fit is cut into several datagrams by entity groups, each one decodable alone | no reassembly; a lost datagram loses only its entities (no 1 − p^n effect, F18); needs per-group baselines |
| P2 | Application-level fragmentation and reassembly of larger snapshots (Quake III style, F27) | simplest encoder; loss multiplies with fragment count (F18) |
| P3 | Grow to a probed path MTU (Quake III 1400, Valve 1300 UDP payload: F27, F30) | fewer splits; depends on the server stack's PMTU discovery and on the browser's maxDatagramSize (F10) |
| P4 | Join and full-resync snapshots over a reliable stream; datagrams carry only deltas | the large message never touches the datagram limit; TCP-like head-of-line wait only at join |

#### E. Server CPU per tick (with R1's tick unit)

| Item | Budget idea (proposed) | Evidence |
|---|---|---|
| Whole tick at 60 Hz (16.7 ms) | ≤ 4 ms p99 for 48 players, about 25 % of the interval, leaving room for bursts and for several rooms per core | assumption; no source read gives a server tick budget |
| Whole tick at 30 Hz (33.3 ms) | ≤ 8 ms p99 | assumption |
| Simulation step | R1's tick unit | legacy JS 0.80–1.00 ms mean per step at 48 soldiers (CM4) |
| Snapshot encode, 48 clients | 10–50 µs per client per snapshot, so 0.5–2.4 ms per snapshot round | assumption: to be measured |
| Lag-compensation history | 1 s × 60 ticks × 48 soldiers × 64 bytes ≈ 184 KB per room; about 72 rewound shots per second at 48 players | Source keeps 1 s (F29); bytes per pose and shots per second are assumptions |

#### Worked bandwidth estimate

**Inputs.** Each row is sourced or marked "assumption".

| Input | Value | Source |
|---|---|---|
| IPv4 + UDP header | 28 B (IPv6: 48 B) | F3 |
| QUIC short header | 1 B flags + 8 B connection ID + 2 B packet number = 11 B | F4 (field ranges); 8 B and 2 B are assumptions (server stack chooses) |
| QUIC AEAD tag | 16 B | F5 |
| DATAGRAM frame type, no Length (last frame) | 1 B | F6 (inferred LEN rule) |
| WebTransport Quarter Stream ID | 1 B | F9 |
| Transport overhead per datagram | 57 B IPv4, 77 B IPv6 | sum of the above |
| Game snapshot header (server tick, baseline tick, last processed input tick, flags) | 8 B | assumption (Fiedler's own header was about 5 B, F22) |
| Largest payload in a 1200-byte UDP payload | 1171 B (8 B connection ID, 2 B packet number); 1157 B worst case (20 B, 4 B) | F1, F4, F5, F6, F9: arithmetic |
| Horizontal position | 16 bits per axis at 1 cm (±327.68 m) | netcode default principle, positions to the centimetre (`.claude/agents/netcode-architect.md`:30); range from CD21 |
| Height | 12 bits at 1 cm (0–40.95 m) | assumption: VISION_V2 §6.2 floors; LOOP_V2 §9 size open |
| Aim yaw | 16 bits | netcode default principle, angles to 16 bits (`.claude/agents/netcode-architect.md`:30) |
| Aim pitch (aiming at height) | 10 bits (0.18° steps) | assumption (VISION_V2 §6.2) |
| Health, flags, weapon | 8 + 8 + 4 bits (vehicle health 11 bits) | assumption; max soldier health 150 (UNIT_TYPES), Juggernaut 1500 (CD14) |
| Full soldier record | 90 bits (11.25 B) | sum |
| Delta soldier record, moving | about 60 bits (7.5 B): 7-bit index, 6 group bits, 2 × (1 + 9) bits relative x/z (±2.56 m covers 13.3 m/s × 133 ms baseline age), yaw 16, pitch half the time, health and flags rarely | assumption built on F21 (changed lists, relative positions) and GP3 speeds; baseline age = 100 ms RTT + one interval |
| Delta vehicle record, moving | about 75 bits (9.4 B): adds hull yaw and turret yaw (multi-part, VISION_V2 §6.4) | assumption |
| Full vehicle record | about 150 bits (18.75 B) including 4 seats × 6-bit occupant | assumption (seats `[OPEN]`, VISION_V2 §6.4) |
| Projectile spawn | 88 bits (11 B): type, owner, position, velocity 3 × 10 bits, tick offset | assumption; flight then simulated on the client from the spawn (needs R1's determinism) |
| Projectile per-snapshot update (E3b) | 5.5 B | assumption |
| Local player's own state (ammo, money, cooldowns, own velocity for reconciliation) | 16 B per snapshot | assumption |
| Structures and objectives | 3 changing per snapshot × 18 bits ≈ 7 B | assumption |
| Team economy state | 2 B per snapshot on average | assumption |
| Shot events (cosmetic, sent once) | 1.5 per player per second (30 % firing × 5 shots/s) × 4 B | assumption (Shabzon max 13.9/s, WEAPONS) |
| Reliable gameplay events (kills, captures, buys), resent until acked | 5 B per player per second (at 48: 10 events/s × 8 B × 3 sends) | assumption |
| Projectiles alive | 32 at 48 players, spawns 0.44 per player per second | assumption (fuses 0.55–1.5 s, GC14, GC15) |
| Vehicles | one per squad of 4 (LOOP_V2 §2) + one harvester per team (GE6) | assumption |

**Formula (E3, 30 Hz).** Bytes per snapshot = 65 + 16 + (P − 1) × 7.5 + V × 9.4 + 7 + 2; per second add events
15.8 × P bytes (6 shots + 5 reliable + 4.8 projectile spawns). P = players and bots, V = vehicles; every entity is
assumed in view (no interest management).

**Results, downstream per client.**

| Case | P | V | Bytes per snapshot | Rate | Per client |
|---|---|---|---|---|---|
| Two teams, 4v4 (VISION_V2 §3 start, upper end) | 8 | 4 | 180 | 30 Hz | 44.2 kbps |
| Three teams, 6v6v6 (start, upper end) | 18 | 7 | 283 | 30 Hz | 70.3 kbps |
| 12v12v12 (goal) | 36 | 12 | 465 | 30 Hz | 116.2 kbps |
| 24v24 or 16v16v16 (goal; design point 48) | 48 | 15 | 584 (609 with events) | 30 Hz | **146.1 kbps** |
| 32v32 (48+ headroom) | 64 | 18 | 732 | 30 Hz | 183.7 kbps |
| 48, E3 at 20 Hz | 48 | 15 | 584 | 20 Hz | 99.4 kbps |
| 48, E3 at 60 Hz | 48 | 15 | 584 | 60 Hz | 286.1 kbps |
| 48, E3b (projectiles in every snapshot) | 48 | 15 | 760 | 30 Hz | 188.3 kbps |
| 48, E2 (quantized, no delta) | 48 | 15 | 1021 (1373 with 32 projectiles: over 1171) | 30 Hz | about 251 kbps |
| 48, E1 (raw floats) | 48 | 15 | about 2209 in 2 datagrams | 30 Hz | about 536 kbps |
| 48, illustrative interest cut to 16 soldiers and 5 vehicles in view | 16 visible | 5 | 250 | 30 Hz | about 62 kbps (the interest unit gives the real cut) |

IPv6 adds 20 B per datagram: +4.8 kbps at 30 Hz. The upstream per client is in table C (22–45 kbps; IPv6 +4.8 to
+9.6 kbps; QUIC acks riding on input packets, about 8 B each, add about 3.8 kbps at 60 Hz: assumption). A 48-human room
at E3 sends about 48 × 146 kbps ≈ 7.0 Mbit/s and receives about 2.1 Mbit/s (U1); about 3.2 GB egress per match-hour.
Bots cost no network traffic.

Comparison points: Fiedler's example budget 256 kbit/s (F34); Valve's library default clamp 256 KiB/s ≈ 2.1 Mbit/s
(F31); Quake III's 1999 default 3000 B/s = 24 kbit/s (F26).

### Brief question 3: interest management for a top-down camera (interest unit)

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

### Brief question 4: the WebTransport server stack in Rust plus the WebSocket fallback (transport unit)

#### Q1. The WebTransport server stack in Rust

| Option | What it is | Complexity and effort | Maturity and maintenance | Draft and interop | Risk | Fit with VISION_V2 |
|---|---|---|---|---|---|---|
| A. wtransport 0.7.x on quinn | Its own session, stream and datagram API; self-signed certificate helper; access to the inner quinn connection | Low: one crate, tokio | Since 2023-05; 0.7.2 (2026-08-11); about 1.08 M downloads; README "not considered completely production-ready" (F29) | Draft-02 and -07 settings (F6) | Pre-1.0 API; one project; lags when browsers move to a new draft | Serves the rooms and authoritative server of §6.1 |
| B. web-transport-quinn 0.13.x on quinn | Mirrors the quinn API; sibling crates: a browser wrapper, an async trait, qmux over WebSocket | Low to medium | Since 2024-04; 0.13.5 (2026-10-24); about 2.03 M downloads; six releases in four weeks; no stability statement (F30) | Draft-02 and -07 settings (F6) | Frequent pre-1.0 releases (API stability not checked); its own trait and qmux are young | Same; the trait and qmux could cover the fallback too (Q4) |
| C. h3 + h3-quinn + h3-webtransport | hyperium's HTTP/3 with a WebTransport extension | Medium: lower level | h3 "still very experimental"; h3-webtransport 0.1.2, idle since 2025-05 (F31) | Not checked | Highest: experimental and idle | Same; nothing extra |
| D. Our own minimal WebTransport handshake on quinn | Write the HTTP/3 SETTINGS and CONNECT exchange ourselves (as web-transport-proto does) | High: protocol work and upkeep on every draft change | Ours | Whatever we write | High effort for no game-specific gain | Weak: an engine feature the game does not need |

All four run on quinn (0.11.12, mature, F28), whose congestion control the server can choose (F25).

#### Q2. The fallback when WebTransport fails

| Option | What it is | Pros | Cons |
|---|---|---|---|
| A. Binary WebSocket (tokio-tungstenite, directly or through axum's `ws`) | A second, TCP-based transport into the same server, carrying the same messages | Mature crates (F33); in all four browsers since long ago, also in workers (F34); simple | Head-of-line blocking: one loss stalls every later snapshot (F21, F22); no unreliable channel |
| B. The browser's own WebTransport over HTTP/2 (Safari 26.4) | The same WebTransport API over TCP (F12) | No second client code path | Needs server support for draft-ietf-webtrans-http2 (still a draft, F3). None of the Rust crates read claims it (absence: unverified). Only Safari is documented as doing it. Still reliable only, with the same head-of-line blocking |
| C. qmux over WebSocket (web-transport's `qmux`) | QUIC-style streams and datagrams carried over a WebSocket (F30, F32) | One stream and datagram API over both transports | QMux is draft -02, a WG document (F32); young crate; "datagrams" over TCP are still head-of-line blocked |
| D. No fallback | WebTransport only | Simplest | Players on networks where "the underlying network environment doesn't support QUIC" (WebKit, F12) cannot play; the brief names the fallback (§0) |

#### Q3. Mapping the game's messages to datagrams and streams

| Message | Option A: datagram | Option B: stream | Notes |
|---|---|---|---|
| Input commands (client to server, every tick) | Yes: small, newest wins; repeat the last few in each datagram to survive loss (F16: no retransmission) | No: a lost packet would delay every later input (head-of-line blocking within a stream, F21) | Legacy CS1 and GP2 |
| Snapshots (server to client, 20–30 Hz) | Yes, if each fits under the size limit (F17, F18) and is decodable on its own; deltas are built against the last acknowledged snapshot, so a lost one costs nothing extra | Only over the WebSocket fallback | Splitting one snapshot into fragments multiplies loss (F20). Split by entity group into independent datagrams instead |
| Reliable messages (join, match setup, team and slot changes, purchases, chat, end of match) | No | Yes: one bidirectional control stream per session, validated by the server (STATUS.md:70) | Use priorities where supported (F23) |
| Bulk data (map definition, tuning tables at join) | No | A separate unidirectional stream, so it never blocks the control stream (F21) | Chrome lacks `sendOrder` (F11, F23) |

#### Q4. One transport interface over both

- **A. Our own thin interface**, written twice. The client side is TypeScript (one implementation on WebTransport, one on
  WebSocket); the server side is a Rust trait or enum. It needs four operations: send unreliable, send reliable on a named
  channel, receive, and a link-state or RTT probe. The client must feature-detect the datagram writer (F8). Smallest, fully
  ours, game-specific.
- **B. web-transport-trait plus qmux.** These cover the server side (F30) and give real stream semantics over WebSocket. They
  also tie us to a fast-moving 0.x family and an early draft (F32).
- **C. A WebTransport polyfill over WebSocket in the browser.** The only lead (webtransport-ponyfill-websocket, named on the old
  Chrome page) was not read. Not assessed.

#### Q5. Certificates

- **Development: self-signed certificates pinned by hash** (`serverCertificateHashes`): ECDSA P-256, at most 14 days, SHA-256
  hash (F13, F15). Supported in Chrome 100, Firefox 125 and Safari 26.4 (F7). Pooling is not allowed (F13).
- **Production option A: a publicly trusted certificate** for each game server hostname. The browser does its normal check
  (F14). Issuance and renewal belong to R6.
- **Production option B: hash pinning in production.** MDN names ephemeral virtual machines as the use case. The hashes come
  from a trusted intermediary, here our HTTPS matchmaker (F14). The certificate must be re-issued every 14 days or sooner, and
  any cached hash invalidated (F13).

#### Q6. Congestion control for a real-time stream

- Datagrams obey the connection's congestion controller and are delayed or dropped when the window is full (F16). The browser
  hint is a no-op in Chrome, and "low-latency" is at risk (F10). The window can fall to 2 × max_datagram_size after persistent
  congestion (F24). What a real-time stream can control is on our side:
  - **A. Server controller choice** in quinn: NewReno, Cubic, or experimental BBR (F25).
  - **B. Drop stale data instead of queueing.** Never wait for send space for a snapshot: use `send_datagram`, not
    `send_datagram_wait` (F25). Use a short `outgoingMaxAge` for client inputs (F19).
  - **C. Adapt the rate.** Lower snapshot rate or size when quinn's RTT or loss rises. The policy belongs to the tick and
    bandwidth unit.

### Brief question 5: rooms and private matches (rooms unit)

#### 3.1 Where a room runs (process model)

| Option | Isolation | CPU and memory at 48+ | Ports, routing, deploys | Complexity and risk | Fit |
|---|---|---|---|---|---|
| A. One OS process per room (GameLift and PlayFab default: F1, F4, F5) | Strongest: a crash, leak or runaway loop ends one room. | One sim thread per room plus a small I/O runtime; per-process baseline memory unknown (unverified; spike measures). Density comes from more processes per host (F4). | One port per room (F18); the lobby hands out host:port; WebTransport URLs carry an explicit port anyway (F12). Teardown = process exit, so nothing accumulates across matches. Drain = stop starting new rooms. | Lowest: lifecycle equals process lifetime; needs a small supervisor (R6). | §6.1 rooms; §4 fairness (no noisy neighbour inside a process). |
| B. Many rooms per process, one dedicated OS thread per room sim, shared async network I/O | Partial: a panic stays in its thread only with `panic = "unwind"` (F8); an abort, memory corruption in unsafe code, out-of-memory or a stuck lock still takes all rooms. | Shared baseline; each room thread has a 2 MiB default stack (F8); one runtime for all sockets. | One port and one certificate for all rooms; rooms routed by WebTransport `:path` (F13). A deploy must drain every room in the process. | Higher: room registry, per-room accounting, crash containment, "more management" (F6). | §6.1; cheaper for many tiny rooms. |
| C. Many rooms as tasks on one shared async runtime | Weakest. | A room's tick loop is long-running CPU work; on a tokio worker it stalls other tasks (F7). | As B. | Tick jitter across rooms; ruled out by F7. | Poor. |

#### 3.2 How a player is admitted

| Option | How | Trade-offs |
|---|---|---|
| A. Signed join token checked offline by the room | Lobby signs {player id, guest flag, room id (audience), team or slot if pre-assigned, expiry, unique id}; room checks signature, audience, expiry, unused id (F9, F10, F11). | No lobby call on the join path; keys must be shared or published; replay blocked by the used-id history (F9, F10). |
| B. netcode.io-style encrypted token (F9, F10) | As A, but the private part is AEAD-encrypted with a key shared by lobby and servers. | Hides the contents and can carry per-connection keys; every server holds a key that can mint tokens. |
| C. Opaque ticket validated by a call back to the lobby (GameLift `AcceptPlayerSession`, F2) | Room asks the lobby "is ticket X valid for room R?" | Simplest crypto; adds a round trip and makes the lobby a live dependency of every join. |

Token carriage: neither browser API sets an auth header (F12). The token can go in the URL (leaks into logs and
history, F14), in the sub-protocol list, or in the first message on a reliable stream; the last keeps it out of URLs
and access logs (inferred from F12 and F14).

#### 3.3 Reconnecting

- **A. Hold the slot for a grace window; rejoin with a fresh token.** The room keeps the player's slot, team and squad;
  the client fetches a new token for the same room and player id (F10's "request a new connect token"); the room maps
  the player id back to the slot; a newer connection for the same player id replaces the older one (F9's one client per
  client id, used here as replace rather than ignore). One verification path for joins and rejoins.
- **B. Room-issued resume secret.** The room gives each client a resume secret at join; the client presents it to the
  room directly. Saves a lobby round trip; adds a second credential type the room must issue and protect.
- **C. No hold.** A dropped player rejoins as a new player, if a slot is free. Simplest; loses the slot to a bot or to
  another player, and splits the player's match record.
What the player's soldier does during the grace window (stands, is bot-driven, is removed) is a game rule: Mustafa's.

#### 3.4 Private matches and room codes

| Option | Trade-offs |
|---|---|
| A. Short room code that only locates a room; joining still needs a lobby token; code lookups rate-limited | Easy to read out over voice; a 6-symbol code from 32 symbols is 32^6 = 1,073,741,824 codes (about 30 bits), far below the 64 bits a secret needs (F14), so it must not be the secret. With 1,000 live private rooms a random guess hits some room with probability about 9.3e-7; at a proposed 10 lookups per minute per client that is about 0.013 hits per client per day (arithmetic, inferred). A host lock and kick bound the harm. |
| B. Long invite link that is itself the credential (at least 64 bits, F14) | Unguessable; but a link is a URL and leaks through logs, history and Referer (F14); poor for voice. |
| C. Both: code to find, link to share | Most convenient; two paths to build and test. |
Room settings: PlayFab fixes capacity, access and owner policy at creation and lets only the owner change room data
(F17). For organized play (VISION §3 [DECIDED]) the creator's settings must at least include the faction of each team;
which other settings a host may change, and when they freeze, is design.

#### 3.5 Bots filling slots

- **A. Per-team fill quota on the server:** bots per team = team size − humans on that team, recomputed on every join
  and leave; a human takes a bot's slot (TF2 'fill' and auto-vacate, F15; legacy STATUS :68).
- **B. Ratio ('match', F15):** bots scale with humans. Does not keep teams full, so it does not meet VISION §3's "bots
  fill empty slots".
- **C. Fixed or no bots:** only for private rooms whose host turns bots off, if Mustafa allows it.
When a human replaces a bot (at once, or when the bot next dies) is a game rule.

#### 3.6 Modes as data

- **A. Mode files validated at load; a room is a mode id plus the overrides the mode allows.** Team count (2 or 3),
  team size, factions per team, squad size, rules table, map list, bot policy, match length and the allowed private
  overrides are data; the slot table is built from the mode (GC2). Format and hot reload belong to R1's data unit.
- **B. Modes in Rust code.** Simple at first; every new mode or team size is a code change, against VISION §3 [DEFAULT]
  and §6.1.
- **C. Per-map rules only (legacy GW2).** Mixes map and mode; cannot express "same map, 2 or 3 teams".

#### 3.7 Guest identity

- **A. Anonymous guest account from the lobby:** zero input; the lobby issues a random guest id and a secret of at
  least 64 bits (F14, F16) that the client keeps in local storage; a login can later be linked to it (F16).
- **B. No identity (a random name per session):** no reconnect across a page reload, no later linking.
- **C. Login required:** against VISION §5 [DECIDED] "guest play first".

### Brief question 6: learn from shipped games

No options of its own: the shipped games' practice enters the options above through the findings they cite.

## Recommendations

One block per brief question, copied from the units. Each recommendation keeps its reason, its "Rests on:" list, the VISION_V2 §6 item(s) it serves and its pass marks, all proposed. R2-XU-OQ18 lists the four that name no §6 item.

### Brief question 1: snapshots, client prediction, interpolation and lag compensation for 48+ players (sync unit)

"The netcode default principles" below means the starting design in `.claude/agents/netcode-architect.md` (lines 26–46:
for example ~100 ms interpolation, ~1 s of hit history, a rewind cap of ~200 ms), which an ADR may change. They are
not VISION_V2 targets.

Network profiles for every pass mark below (proposed; the injected latency is round-trip time, matching Source's
"current latency (RTT)", F28):
P0 = 0 ms, no jitter, 0 % loss · P1 = 50 ms, ±10 ms, 1 % · P2 = 100 ms, ±20 ms, 2 % · P3 = 150 ms, ±30 ms, 5 % ·
P4 = 200 ms, ±40 ms, 5 % · B1 = P2 plus one burst of 1 s at 100 % loss.
VISION_V2 §4's fairness target covers up to about 100 ms (P2). P3 and P4 test that play degrades gracefully. Every spike
also covers reordering (F11).
The tick and snapshot rates are not chosen here (tick-rate unit); the marks are written per tick and per snapshot
interval.

**R2-SYNC-REC1. State sync: delta snapshots against each client's last acknowledged snapshot (option 3.1 E).**
- How it works. The server keeps one ring of recent per-tick world states, shared by all clients. For each client it
  keeps a small ring of the snapshots it sent: each is a tick plus the list of entity ids interest management chose.
  Every client packet carries the newest snapshot it received. The server deltas each new snapshot for that client from
  that acknowledged one: unchanged entities cost no bytes; new entities go from per-entity-type defaults; removals are
  explicit.
- When there is no usable baseline (never acknowledged, acknowledged too long ago, or dropped from the ring), the server
  encodes against the per-entity defaults instead of the whole raw state, as Fiedler does (F7). Quake III does the same
  through its "new entity from its baseline" path (F8; that a full snapshot takes that path is inferred from the code
  quoted). The ring length is data: proposed 32 snapshots, as Quake III.
- Snapshots go as unreliable datagrams, each sized to fit one QUIC packet (F11). Reliable events go on streams
  (transport unit).
- Reason: of the options with sources read, it is the one that tolerates loss and reordering without stalls while
  unchanged entities cost nothing (F7, F8), needs no bit-exact agreement between 48 browsers for correctness, supports
  joining and rejoining mid-match (§6.1 rooms and private matches; humans taking over bot slots), and keeps the server
  the only authority. A shipped game (Quake III) and Fiedler both document it.
- Rests on: F1, F2, F3, F4, F5, F6, F7, F8, F9, F11.
- Serves: VISION_V2 §6.1 (modes as data, rooms and private matches), §6.2 (positions in 3D), §6.4, §6.5 (economy and
  objectives server-side), and §4 (fairness).
- Spike pass marks (proposed):
  1. 48 clients (bots plus scripted headless clients), 10 minutes each under P0, P2, P3 and B1: every snapshot a client
     decodes rebuilds exactly the server's quantised state for that tick and that client's entity set (100 % of
     decoded snapshots, checked by hash).
  2. After the B1 burst, every client is decoding deltas again within 500 ms of the link coming back.
  3. No datagram exceeds the transport unit's payload size.
  4. The server's snapshot work for 48 clients, measured per snapshot on the proposed reference server, is reported and
     stays within the share of the tick budget the tick-rate unit sets.

**R2-SYNC-REC2. Input commands: one per tick, sent with redundancy, carrying the view time.**
- Each command holds:
  - its tick and sequence;
  - the move vector and the button bitfield;
  - button edges as small press counters, so a resent command can neither lose nor double a press (GP2);
  - the aim (REC8);
  - the client's view time for remote entities: the interpolation tick plus a fraction, which lag compensation uses
    (REC5).
- Every datagram repeats all commands the server has not yet acknowledged, up to a cap (proposed 8, as data). Quake III
  repeats one earlier packet's worth by default and allows up to 32 commands (F18).
- The server drops commands older than the last one it ran (F16), and caps how many commands per client it runs per
  tick (the speed-hack guard, F33).
- Reason: redundancy costs a few bytes and removes the need to retransmit (F4, F18). A view time stamped by the client
  but checked by the server is the Source model (F20, F28).
- Rests on: F4, F16, F18, F19, F20, F28, F33.
- Serves: §4 (fairness), §6.2 (aiming at height), §6.3 (abilities as input edges).
- Spike pass marks (proposed):
  1. Under P3 (5 % random loss), at least 99.9 % of commands reach the server before their tick runs.
  2. 10,000 scripted presses under P3 with reordering: 0 lost and 0 doubled.
  3. Forged commands (a tick in the future, an aim beyond weapon range, more commands than the cap) are rejected and
     counted, with 0 effect on the sim.

**R2-SYNC-REC3. Client prediction: own movement, own weapon state, own vehicle and cosmetic shot feedback (option 3.2
C); replay from the authoritative state; linear error decay (option 3.3 B), plus a snap threshold.**
- Prediction. The client keeps a ring of its unacknowledged commands. On each snapshot it resets the local player (and
  the vehicle it pilots) to the server's state for the last command the server ran, then replays the newer commands
  through the same Rust movement and weapon code the server runs, compiled to Wasm. That is Quake III's method (F12,
  F13).
- Skipping the replay. When the server's state matches the stored prediction for that tick, the client skips the replay
  (Quake III's own OPTIMIZE note, F13).
- No baseline. If the command ring no longer reaches back to the snapshot, the client holds its prediction and counts a
  miss (F14).
- Smoothing. The client draws an error offset that fades over a data-set time (proposed 100 ms, Quake III's default,
  F15). It snaps above a data-set distance (proposed 2 m, F16), and clears the offset on respawn or teleport (F24).
- What the client does not predict:
  - damage, deaths and hits on others (F12: what your own input did not cause);
  - the uplink countdown and steals (LOOP_V2 §4): the client shows the server's tick-stamped state.
- The local player's own state goes in its snapshot without lossy quantisation, or the server quantises its own state
  the same way each tick. Otherwise every replay starts from a slightly different state (the quantise-both-sides lesson,
  F6; applied here, inferred).
- Reason: this removes input lag for everything the player controls without giving the client authority over
  anything.
- Identical results between Wasm and native are not needed for sync to be correct (F5). They decide how often
  prediction is corrected, which is why R1's identical-results goal matters to the feel (inferred).
- Rests on: F5, F6, F12, F13, F14, F15, F16, F17, F20, F24.
- Serves: §4 (fairness), LOOP_V2 §7 (split feedback: the shot feels instant, the hit is confirmed), §6.4 (vehicles).
- Spike pass marks (proposed):
  1. Under P0–P4 on a scripted course with no other players: 0 corrections larger than 1 mm whenever every command
     arrived in time; every other correction is counted and traced to a late or lost command.
  2. Under P3 in a 48-player brawl: corrections larger than 5 cm in fewer than 1 % of snapshots, and the visible
     correction never moves the player more than 5 cm in one frame below the snap threshold.
  3. Input-to-screen latency for own movement and muzzle flash is the same at P0 and P4, within one tick.

**R2-SYNC-REC4. Interpolation: remote entities drawn on an adaptive delay (option 3.4 C), starting at 3 snapshot
intervals; short extrapolation, then hold.**
- The delay. The client draws remote entities between the two snapshots that bracket "server time minus the delay".
  The delay starts at 3 snapshot intervals (Fiedler's "lose two packets in a row" rule, F21), widens by the measured
  jitter, and narrows slowly on a clean link. Proposed bounds: 2 to 5 intervals, as data.
- The clock. The client's estimate of server time drifts by at most 1–2 ms per frame, halves errors over 100 ms, and
  resets over 500 ms (Quake III's rules, F23).
- Late snapshots. When the next snapshot is late, the client extrapolates for at most one snapshot interval, then holds
  (F22: extrapolation is poor; F26: Source extrapolates as a fallback too).
- No smearing. Entities that respawn, teleport or first appear are placed, not interpolated (F24).
- Linear interpolation first. Hermite only if the spike shows artefacts on fast movers at the chosen snapshot rate
  (F22; OQ12).
- Projectiles and big units follow REC6 and REC7.
- Reason: Quake III's sub-interval delay (F23) gives up smoothness at every lost snapshot. Across the 0–5 % loss the
  netcode test plan covers, Fiedler's three-interval rule is the measured choice (he measured it at 2–5 %). At an
  assumed 30 Hz snapshot rate, 3 intervals is 100 ms (the default principles' ~100 ms), and Fiedler reports 150 ms
  with a jitter margin (F21).
- Rests on: F21, F22, F23, F24, F26.
- Serves: §4 (fairness), §6.4 (big vehicles drawn smoothly).
- Spike pass marks (proposed):
  1. Under P2, extrapolation in fewer than 1 % of rendered frames; under P3, fewer than 3 %.
  2. Under B1, no extrapolation beyond the one-interval cap.
  3. The adaptive delay settles within 5 s of a profile change.
  4. 0 visible slides across respawns and teleports in 1,000 scripted respawns.

**R2-SYNC-REC5. Lag-compensated hitscan: rewind to the shooter's view time, as a pure query on a per-tick history of
sim-owned hit volumes, capped (option 3.5 B).**
- History. Every tick the server records, in flat arrays, each hittable mover's hit state: soldiers (position, facing,
  stance, alive, Riot Shield up), vehicles and big units (position, hull and turret transforms, alive per part), and
  harvesters.
  - Animation is cosmetic, so unlike Source no animation layers are stored (F29 against CS2).
  - Its length is the cap plus a margin. The default principles' ~1 s, also Source's default (F28), costs about
    0.1 MB at 60 Hz for 60 movers (inferred), but nothing in §6 uses more than the cap.
- The shot. The server builds the ray from its own authoritative fire origin at the command's tick (the
  `playerFireOrigin` rule, GC5) toward the command's aim (REC8). It resolves the ray against the movers' recorded state
  at the view time, and against the static world at present. Movers are not moved back and restored: `resolveShot`
  already is a pure query built for this (CS7).
- View time. The command's view time is accepted if it lies within 200 ms of the server's estimate (RTT plus that
  client's interpolation delay); otherwise the estimate is used (F28).
- The cap. The rewind is capped at a data value, proposed 250 ms. Reading "about 100 ms of ping" as RTT, a 100 ms player
  with a 100–150 ms delay needs 200–250 ms (F28's formula; inferred). The default principles' ~200 ms covers only the
  low end. Beyond the cap, shooters lead by the excess.
- Filters (Source's, F32):
  - targets the shooter could hit (team filter, friendly fire from mode data);
  - targets sent to and acknowledged by the shooter (interest management);
  - targets within reach of the ray (our form of Source's distance and cone tests).
  - A target that died or teleported between the view time and now stops at that record (F30).
- Bots. Shots by bots use present state, with no rewind (F31).
- Feedback. Hit confirmation goes back as a reliable event with the tick (LOOP_V2 §7).
- Reason: without it, a 100 ms player must lead a sprinting soldier by about 1.8 m (F34, legacy `T`). With it, the
  shooter's view is the truth up to the cap, and the victim's "hit behind cover" window is bounded by the cap (F34).
- Rests on: F28, F29, F30, F31, F32, F34 (F34 is my reasoning from F28; F27 only illustrates the option rejected).
- Serves: §4 ("fair up to about 100 ms of ping"), LOOP_V2 §7, §6.4 (multi-part vehicle hits).
- Spike pass marks (proposed):
  1. A scripted shooter fires at a strafing, sprinting target (9 m/s, changing direction at random) under P0–P3. Shots
     whose ray hits the target's volume in the shooter's rendered view register on the server in at least 99 % of
     cases up to P2 and 95 % at P3, and false hits (a miss in the view counted as a hit) stay below 0.5 %.
  2. The measured "hit after cover" window never exceeds the cap.
  3. The history for 48 soldiers plus 12 vehicles and harvesters stays at or under 1 MB.
  4. A rewound shot against 48 candidates takes at most 20 µs at the 99th percentile on the proposed reference server,
     at 700 shots per second.

**R2-SYNC-REC6. Projectiles: server-simulated, replicated as trajectories, evaluated by one shared Rust function; the
owner shows an immediate cosmetic copy (option 3.6 B).**
- Replication. Each projectile is an entity sent as a trajectory record: type, start tick, start position, velocity and
  fuse. It is re-sent only when the trajectory changes (bounce, stick, explode), each change stamped with its tick
  (F35).
- One function. The server, the client display and the throw preview all call the same trajectory function in the
  Rust core (Wasm on the client). That fixes GC13's mismatches.
- Swept tests. The server sweeps each step's segment against movers and statics at present, with no rewind (Quake III
  does the same, F35), keeping the legacy swept test (GC15).
- What other clients see. Remote clients evaluate remote projectiles on the interpolation timeline (server time minus
  the delay), so a frag lands where the interpolated target is drawn. Quake III instead evaluates missiles at its own
  near-present time (F25), which works only because its delay is under one interval (inferred).
- The owner. The owner sees a cosmetic projectile from its predicted state at once. It is matched to the server's by
  (owner, command sequence) and blended over the error-decay time.
- Blasts. Blasts are server events with a tick, played on the timeline of the entity they hit.
- Spawn fast-forward is open (OQ6).
- Reason: trajectories cost bytes only on change, cannot drift between server and client, and keep the server the
  authority for every landing point.
- Rests on: F12, F15, F25, F35.
- Serves: §6.3 (abilities such as the Molotov), §6.4 (vehicle weapons), LOOP_V2 §5 (Javelin rockets) and §7.
- Spike pass marks (proposed):
  1. 10,000 random throws: the trajectory function gives bit-identical points on Wasm and native.
  2. Under P2, the owner's cosmetic copy is within 0.5 m of the server's at hand-over, and the hand-over never moves it
     more than 5 cm in one frame.
  3. Remote clients see the explosion within 10 cm of the drawn impact point on interpolated targets.

**R2-SYNC-REC7. Vehicles and big multi-part units: the pilot predicts the vehicle through its own command stream; parts
are sim data in snapshots and history.**
- The pilot. The piloting player predicts the vehicle exactly as REC3 predicts the soldier. The pilot's command drives
  the vehicle, and the pilot's snapshot carries the vehicle's state at full precision, like the local player's own
  state (REC3).
- Everyone else interpolates it (REC4).
- Parts. Each part (hull, turret, legs as needed) is a hit volume with its own transform and health in sim data. Part
  transforms are compact state (yaw, pitch); animation stays cosmetic (GC16, GC17).
- Weapons. Vehicle weapons fire from sim mount points, never from an animated node (GC17). Hitscan weapons get REC5;
  projectile weapons REC6.
- Seats for several players are §6.4 [OPEN], so no passenger model is proposed (OQ7).
- Reason: it follows from the shipped prediction model (F12, F13) applied to whatever the player controls. Multi-part
  history is the same table as REC5.
- Rests on: F12, F13, F15, F28, F29.
- Serves: VISION_V2 §6.4 (vehicles as first-class, big and multi-part).
- Spike pass marks (proposed):
  1. A piloted Juggernaut under P2 and P3 meets REC3's correction marks.
  2. Hits on each part meet REC5's registration marks.
  3. A soldier next to a moving vehicle gets corrections larger than 5 cm in fewer than 2 % of snapshots under P2
     (OQ5 sets the collision rule).

**R2-SYNC-REC8. Aiming at height: the command carries a quantised world aim point (option 3.7 B), picked on the client
against its rendered world.**
- On the client. The cursor ray is cast into the world as the client draws it: interpolated remote entities and the
  static world, using the same Rust ray query (GC3 already does this through the SIM). The client quantises the hit
  point to the command's precision. Any "gentle aim help onto bodies" (LOOP_V2 §9 [OPEN]) shapes this point on the
  client.
- On the server. The server rebuilds the ray from its own fire origin at the command's tick toward that point, extends
  it to the weapon's range, and resolves it with REC5. Line of sight across floors is the server's own height-aware
  query (R4).
- Throws. Throws and charged shots use the same point for heading and pitch, replacing today's y = 0 ground heading
  (GC3, GC13).
- Reason:
  - a top-down cursor names a point, not a direction;
  - sending the point keeps the player's intent exact across small prediction errors in the fire origin;
  - it lets the server check range and plausibility.
  - Precision does not decide it. A 16-bit angle step (F19: 65,536 steps per turn) is about 0.0055°, about 3 cm at the
    AMK's 300 m (inferred arithmetic), so a direction is precise enough. The difference is what an error in the
    predicted fire origin does. With a direction, the server's ray runs parallel to the client's and misses the
    clicked point by the origin error. With a point, the ray passes through the clicked point and the origin error only
    changes the angle. The point is chosen for intent at height and for validation, not for precision.
- Rests on: F19, F28 (the view time that makes the picked point consistent with the rewound targets).
- Serves: VISION_V2 §6.2 (aiming at height; line of sight across heights).
- Spike pass marks (proposed):
  1. Scripted shots at static targets on the ground, at 2.5 m and at 4 m (legacy platform heights, GC3), under P0–P3:
     the server's hit point lies within 5 cm of the client-picked point in at least 99.9 % of shots.
  2. The aim costs at most 80 bits per command before delta coding.

**R2-SYNC-REC9. What changes at 48+: budget the per-client and per-shot costs, and make every cost scale with what each
client sees, not with the match size.**
- Snapshot encoding is per client (REC1). Its cost is clients × visible entities, so interest management and priority
  (other units) set the real numbers.
- Lag compensation scales with shots per second (at most about 700 at 48, from the WEAPONS table; inferred) times
  candidates, so the reach filter and the spatial index matter more than the history size.
- Prediction cost per client does not grow with the player count: only the local player and its vehicle are replayed
  (F13, F17).
- Bots fill slots, act on present state, and are never rewound for (F31). A human taking over a bot slot keeps the
  entity id (GC2).
- Reason: 48 players multiply the server's per-client work and the shots per second, while the client's work stays
  flat. The spikes must measure the server side at the full count.
- Rests on: F8, F13, F17, F31, F32.
- Serves: VISION_V2 §3 (12v12v12 or 24v24; design for 48+), §6.1.
- Spike pass marks (proposed):
  1. One match server with 48 clients (any mix of bots and headless clients) under P2 for 30 minutes. Snapshot
     encoding, lag compensation and command handling together use at most 25 % of the tick budget at the 99th
     percentile on the proposed reference server.
  2. The same match at 24 and 36 players shows the server's sync cost growing no faster than linearly in players ×
     visible entities.

### Brief question 2: tick rate and bandwidth budgets (budgets unit)

- **R2-BUD-REC1. Rates: snapshots at 30 Hz; simulation tick 60 Hz if R1's tick spike meets its CPU mark, else 30 Hz;
  inputs sampled every tick and sent every tick with the 3 previous inputs repeated (U1, U2 once measured).**
  Reason: of 20, 30 and 60 Hz, 30 Hz is the only snapshot rate that keeps both the 48-player downstream under the 256 kbps example budget
  (146 kbps against 286 at 60 Hz) and the lag-compensation rewind inside the ~200 ms cap at 100 ms of ping (150–200 ms
  against 200–250 at 20 Hz). A 60 Hz tick halves input-timing and fire-interval quantization (Shabzon 4.32 ticks against
  2.16), matches legacy `FIXED_DT` and Source's 66.7 Hz, and does not change the snapshot budget. Weapon cooldowns carry
  the fractional tick remainder at either rate. Upstream at 60 Hz with 4 copies is under 50 kbps.
  Rests on: R2-BUD-F26, F28, F29, F31, F32, F34, F35; legacy CS1 and `WEAPONS`; the worked estimate.
  Serves VISION_V2 §6.1 (modes of any size), §6.2 (aiming at height in the input), §4 (fair to about 100 ms).
  Spike must prove (proposed): at 48 players (simulated clients plus bots), 30 Hz snapshots, 100 ms RTT, ±10 ms jitter,
  2 % loss: rewind ≤ 200 ms for 100 % of shots; server-side missing-input rate ≤ 0.1 % of player-ticks at 5 % random
  loss; upstream ≤ 48 kbps per client (IPv4, U1); mean delivered fire rate of the Shabzon within 1 % of 13.9 shots/s
  at the chosen tick.
- **R2-BUD-REC2. Downstream encoding E3 with a budget: quantize (1 cm positions in 16/16/12 bits, 16-bit yaw, 10-bit
  pitch), delta-encode against the last snapshot each client acknowledged (application acks, a per-client ring of about
  32 snapshots ≈ 1 s at 30 Hz), send changed-entity lists, send projectiles as spawn events with client-side
  deterministic flight plus correction events, send cosmetic shot events once and gameplay events until acked; enforce
  a hard per-client cap with a priority accumulator.** Reason: E3 brings the all-in-view 48-player case from about 536
  kbps (raw) to about 146 kbps and keeps every delta snapshot near 600 B, half a datagram; the cap makes worst-case
  bursts degrade by priority instead of by loss. E4 waits for measured E3 numbers.
  Rests on: R2-BUD-F8, F19, F20, F21, F22, F23, F24, F25, F34; the worked estimate (assumptions marked).
  Serves VISION_V2 §3 (12v12v12 or 24v24), §6.1, §6.4 (vehicles with turrets and seats), §6.5 (economy state
  server-side), §6.8 (events stamped with ticks for moments).
  Spike must prove (proposed): at 48 players all in view, 30 Hz, 10-minute bot match: mean ≤ 160 kbps and p99 of
  1-second windows ≤ 256 kbps per client, counted at the UDP payload plus 28 B per datagram; with the interest unit's
  filter on the Octopon-3 greybox: mean ≤ 100 kbps; decoded positions within 1 cm and angles within 0.01° of the
  server's values; 0 desyncs of deterministic projectile flight against the server over 1000 throws (if E3, else E3b).
- **R2-BUD-REC3. Packet size: no snapshot datagram above min(the runtime maximum datagram size, 1150 bytes of
  WebTransport payload); no IP or application-level fragmentation of snapshots; a snapshot that does not fit is split
  by entity groups into datagrams that decode alone (P1); the join and full-resync snapshot goes over a reliable stream
  (P4) unless the spike shows P1 is enough.** Reason: QUIC guarantees only 1200-byte UDP payloads and forbids IP
  fragmentation; DATAGRAM frames cannot be fragmented; the browser does not send oversize writes (F11, inferred);
  fragment loss multiplies. At 48 players a full snapshot is about 1021 B without projectiles and about 1373 B with 32
  live projectiles, the latter over the 1171 B usable payload; at 64 players even the projectile-free full snapshot is about
  1257 B (65 + 16 + 63 × 11.25 + 18 × 18.75 + 130, arithmetic). So the split path is needed from the first large
  match, not later.
  Rests on: R2-BUD-F1, F2, F3, F4, F5, F6, F9, F10, F11, F18, F27, F30.
  Serves VISION_V2 §3 (48+ players), §4 (browsers first).
  Spike must prove (proposed): 0 datagrams larger than the reported maximum (server and browser side) in a 30-minute
  48-player soak; a client joining mid-match has a complete world within 300 ms at 100 ms RTT and 2 % loss; the
  maxDatagramSize each target browser reports is recorded (Chrome, Edge, Firefox, Safari where it runs WebTransport).
- **R2-BUD-REC4. Buffers for "fair to about 100 ms": interpolation delay 100 ms by default (3 snapshot intervals at 30
  Hz), adapting up to 150 ms from measured jitter and loss; lag-compensation rewind = one-way latency + the client's
  interpolation delay, capped at 200 ms; 1 s of hitbox history; a server input buffer of 1–3 ticks, adapted per
  client; clock offset from RTT samples (the browser's smoothedRtt and rttVariation plus our own timestamps).**
  Reason: Fiedler's three-interval rule at 30 Hz gives 100 ms, 150 ms with a jitter allowance; Source's rewind formula
  at 100 ms RTT then gives 150–200 ms, inside the cap; Source keeps 1 s of history.
  Rests on: R2-BUD-F12, F29, F32, F35.
  Serves VISION_V2 §4 (fair to about 100 ms; one EU region).
  Spike must prove (proposed): at 100 ms RTT, ±10 ms jitter, 2 % loss: ≤ 0.5 % of rendered frames need extrapolation
  with a 100 ms delay; at 5 % loss the adaptive delay stays ≤ 150 ms; clock offset error ≤ 5 ms after 2 s of samples
  (measured against the simulator's true clock).
- **R2-BUD-REC5. Adapt the send budget to the congestion controller: read estimatedSendRate, lostOutgoing and
  expiredOutgoing (browser side) and the server stack's equivalents, plus our own acks; set the outgoing maximum age to
  about one snapshot interval so stale snapshots expire instead of queueing; under pressure drop low-priority entities
  first, then fall back to 20 Hz snapshots, never above the cap.** Reason: datagrams are congestion-controlled and may
  be dropped by the sender; the window can fall to two datagrams; the "low-latency" option is not implemented by any
  browser (F14, inferred); Valve's library still sets its rate by hand.
  Rests on: R2-BUD-F7, F12, F13, F14, F15, F31, F34.
  Serves VISION_V2 §4 (fair to about 100 ms).
  Spike must prove (proposed): with the link throttled to 128 kbps for 10 s, the client keeps receiving snapshots
  (≤ 2 missing per second), the local player's own state is never deprioritized, and the full rate returns within 2 s
  after the throttle ends.
- **R2-BUD-REC6. A server CPU budget per tick for the whole tick, not just the simulation step: at 48 players, p99 ≤ 25
  % of the tick interval (≤ 4 ms at 60 Hz, ≤ 8 ms at 30 Hz) including the simulation (R1), lag-compensated hit tests
  and snapshot encoding for every client, measured separately.** Reason: no source read gives a tick budget; 25 %
  leaves room for bursts and for more than one room per core; per-client encode cost is the unknown that grows with
  player count.
  Rests on: legacy CM4 (0.80–1.00 ms per step in JS at 48 soldiers); R2-BUD-F25, F29 (history and ring sizes); the
  estimate's assumptions.
  Serves VISION_V2 §3 (48+), §6.1 (rooms).
  Spike must prove (proposed): on the reference server (proposed: one vCPU of the EU instance type R6 recommends), 48
  players, 30-minute soak: p99 whole tick ≤ 4 ms at 60 Hz (or ≤ 8 ms at 30 Hz), 0 missed ticks, snapshot encode ≤ 50 µs
  per client per snapshot, memory per room reported.

### Brief question 3: interest management for a top-down camera (interest unit)

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

### Brief question 4: the WebTransport server stack in Rust plus the WebSocket fallback (transport unit)

**R2-TRN-REC1. Build the server on quinn 0.11.x. Pick the WebTransport layer by spike between wtransport 0.7.x and
web-transport-quinn 0.13.x, behind our own thin session interface. Rule out h3-webtransport and a hand-written handshake.**
Reason:
- Both candidates sit on the same mature QUIC stack (F28) and speak the same draft-02 and -07 settings as QUICHE (F5, F6).
- Neither claims production readiness, and each has a different risk: wtransport says it is not production-ready (F29);
  web-transport-quinn moves fast in 0.x (F30). A thin interface keeps a later swap cheap.
- h3 is "still very experimental" and h3-webtransport is idle (F31).
- Option D is generic protocol work no VISION_V2 §6 item asks for.

Rests on: F4, F5, F6, F25, F28, F29, F30, F31.
Serves: VISION_V2 §6.1 (rooms and private matches need a client-to-server link); §6.5 (economy server-side), §6.6 (telemetry
recorded server-side) and §6.8 (moments recognized server-side), which all assume an authoritative server reached over this
transport; and the §4 targets (four browsers, fair to about 100 ms).
What a spike must prove (proposed pass marks):
1. Interop: Chrome 156, Edge 156, Firefox 158 and Safari 27 (macOS) each open a WebTransport session to the Rust server with
   each candidate crate, and exchange datagrams plus one bidirectional stream. At least 99 of 100 connection attempts succeed per
   browser on a clean local network (proposed). Record the negotiated draft per browser; this answers OQ1.
2. Load: one server process holds 48 WebTransport sessions. Each sends 60 input datagrams/s (≤ 100 bytes) and receives 30
   snapshot datagrams/s (≤ 1,000 bytes; placeholder numbers until the tick and bandwidth unit sets them). Proposed limits: at
   most 1 CPU core for the transport layer, and p99 server send-call time ≤ 1 ms, on reference server hardware R6 names
   (proposed).
3. Swap cost: the thin interface hides the crate. Switching crates in the spike changes no code outside the transport module
   (checked by diff).

**R2-TRN-REC2. Send inputs and snapshots as datagrams, and reliable traffic on streams. Cap each snapshot datagram at a proposed
1,000 bytes of payload and make every datagram decodable on its own (no fragmenting of one snapshot). Put reliable control on
one bidirectional stream and bulk data on its own unidirectional stream.**
Reason:
- Datagrams are never retransmitted and never block one another, which suits newest-wins data (F16, F21).
- The guaranteed floor is 1,200 bytes of UDP payload minus headers (F17). An oversized datagram vanishes silently (F18). Fiedler
  shows fragment loss compounds (F20).
- Separate streams keep a big map download from delaying a slot change (F21).
- Input redundancy covers loss of one-shot actions (GP2).

Rests on: F16, F17, F18, F19, F20, F21, F22.
Serves: §6.1 (the per-mode match flow over the control stream); the §4 fairness target (no head-of-line stalls on the
WebTransport path); the 48+ design point of VISION_V2 §3.
What a spike must prove (proposed pass marks):
1. `maxDatagramSize` is recorded at session start and every 10 s on each of the four browsers. Pass: ≥ 1,000 bytes on all four,
   on loopback and on a real EU path; otherwise the cap drops to the smallest value measured.
2. Under a network simulator at 100 ms RTT, ±20 ms jitter and 2 % loss, the share of snapshots lost or late (arriving more than
   two snapshot intervals after sending) is at most the injected loss + 1 percentage point. Proposed.
3. With 5 % loss, no one-shot action is lost: 0 dropped out of 10,000 scripted actions, whether through input redundancy or
   the reliable stream.
4. A 1 MB bulk transfer on its own stream during play adds ≤ 5 ms to the p99 delay of control-stream messages. Proposed.

**R2-TRN-REC3. Fall back to binary WebSocket (tokio-tungstenite 0.30.x, or axum 0.8.x `ws` if the server already serves HTTP
through axum). Choose one transport per client, never both at once. Ask for `requireUnreliable: true` where the browser supports
it. Do not serve WebTransport over HTTP/2.**
Reason:
- Some networks don't support QUIC (F12). WebSocket is everywhere, including workers (F34), and its crates are mature (F33).
- Safari's own HTTP/2 fallback would need a server-side draft implementation that none of the crates read offers (F3, F12),
  and it is reliable-only anyway.
- `requireUnreliable` makes a WebTransport session fail cleanly, so our WebSocket path takes over (F12; Chrome ignores the
  option, F11).
- Running TCP and UDP side by side hurts the UDP flow (F22).
- Over WebSocket, the server must send only the newest snapshot. It drops any unsent older one, so stalls do not stack up
  (F21, F22).

Rests on: F3, F11, F12, F21, F22, F33, F34.
Serves: the §4 browser target (all four browsers, on all networks); §6.1.
What a spike must prove (proposed pass marks):
1. With UDP blocked to the server, each of the four browsers reaches a playing WebSocket session ≤ 3 s after the page starts
   connecting. Proposed: WebTransport attempt timeout 1.5 s, then WebSocket.
2. The same recorded input replay drives both transports to identical server state hashes on every tick (the per-tick state
   hash of the default principles).
3. At 100 ms RTT and 1 % loss over WebSocket: report the head-of-line stall distribution. Proposed pass: p99 extra snapshot delay
   ≤ 250 ms, and the server never holds more than one unsent snapshot per client.

**R2-TRN-REC4. Use our own thin transport interface on both sides, with two backends each, and keep it outside the simulation.**
- Client (TypeScript): WebTransport and WebSocket backends. Feature-detect the datagram writer (`createWritable()` on Safari and
  Firefox 155+, `writable` on Chrome and Edge). Measure RTT with our own ping messages: `getStats()` is missing on Chrome and Edge
  and throws on Firefox.
- Server (Rust): one session type per backend, feeding the same per-client inbound and outbound queues.
- Carry our protocol version in the `protocols` option where supported (Chrome 143+, Firefox 155+, Safari 26.4) and in the
  first control message everywhere else.

Reason:
- The browser API differs today (F8, F9, F11, F38). The simulation must not depend on any I/O (CS11).
- web-transport-trait and qmux are young (F30, F32). Four operations do not justify a dependency. The question stays open
  (OQ6).

Rests on: F8, F9, F11, F30, F32, F35, F38.
Serves: §6.1; R1's simulation in a Web Worker (WebTransport and WebSocket both work in workers, F34, F35).
What a spike must prove (proposed pass marks):
1. One client build runs unchanged on all four browsers over both backends. The feature-detection path is covered by a test per
   browser in the end-to-end suite (R5).
2. App-level RTT agrees with quinn's server-side `rtt()` within ±5 ms at 100 ms injected RTT. Proposed.
3. The simulation crate has no dependency on the transport crates (checked with `cargo tree`).

**R2-TRN-REC5. Certificates.**
- Development and previews: self-signed ECDSA P-256 certificates valid ≤ 14 days, made by the server at start (wtransport's
  `Identity::self_signed`, or rcgen with web-transport-quinn), pinned through `serverCertificateHashes`. The hash is fetched from
  the page's own HTTPS origin.
- Production: a publicly trusted certificate per game server hostname. Hash pinning in production stays an option, only if R6
  picks short-lived ephemeral servers.

Reason:
- The pinning rules (X.509v3, ≤ 2 weeks, ECDSA P-256, no RSA, no pooling) are strict, and every target supports them (F7, F13).
- MDN frames pinning as the tool for non-public and ephemeral hosts, with hashes from a trusted intermediary (F14).
- wtransport already generates compliant certificates (F15).

Rests on: F7, F13, F14, F15.
Serves: §6.1 (private matches on real servers); the §4 browsers.
What a spike must prove (proposed pass marks):
1. All four browsers connect with a pinned self-signed certificate, and all four with a publicly trusted one: 100 % of 20
   attempts each.
2. Certificate rotation with 48 live sessions: 0 sessions dropped, and new connections succeed with the new hash within one
   reload.
3. A certificate valid for 15 days is rejected by all four browsers (a negative test of the rule).

**R2-TRN-REC6. Treat congestion control as ours to manage on the server.**
- Start with quinn's NewReno or Cubic. Pick between them by measurement; BBR is ruled out while quinn labels it experimental.
- Never block the simulation tick on a send, and never queue stale snapshots: always the non-waiting `send_datagram`, with a
  check of `datagram_send_buffer_space`.
- Give client input datagrams a short `outgoingMaxAge`, proposed 100 ms.
- Feed quinn's RTT and loss figures into the tick and bandwidth unit's rate adaptation.

Reason:
- Datagrams are congestion-controlled and are dropped or delayed under pressure (F16).
- The browser hint does nothing in Chrome, and "low-latency" is at risk (F10).
- The window can collapse to two packets (F24). All streams and datagrams share it (F26).
- quinn offers a pluggable controller and the non-blocking send (F25).

Rests on: F10, F16, F19, F24, F25, F26.
Serves: the §4 fairness target (about 100 ms) at the §3 scale (48+ players).
What a spike must prove (proposed pass marks):
1. With a bottleneck set below the snapshot stream's rate (proposed: 50 % of it), the newest snapshot the client holds is never
   older than 3 snapshot intervals. Server memory per session stays flat (no queue growth over a 10-minute soak).
2. Simulation tick duration at p99 is unchanged (within 5 %) between a clean network and the bottleneck run.
3. NewReno and Cubic are compared under the REC2 netem profile. Pick the one with the lower p99 snapshot age; record the numbers.

### Brief question 5: rooms and private matches (rooms unit)

- **R2-ROOM-REC1. An explicit room lifecycle, one fresh simulation per match.** States: Created (mode and map loaded,
  slot table built) → Open (accepting joins; bots fill) → Countdown → Running → Ended (final scores, results and
  telemetry written) → Closed (connections closed, room gone). Each state has a timeout: activation (F3's 1 to 600 s
  pattern), an empty-room timeout, the mode's match length (LOOP_V2 §8: about 20 min). The room answers a supervisor
  health check (F3). A match never pauses or restarts for one player (GP13, PG8), and the next match starts from a new
  sim instance (GE12).
  Reason: the shipped hosting services all model a session as a bounded life with activation, health and an end (F1,
  F3, F5); legacy shows in-place resets and world freezes do not survive multiplayer.
  Rests on: F1, F3, F5. Serves VISION §6.1 (rooms), §6.6 (results and telemetry recorded at Ended).
  Spike must prove (proposed): (1) 100 create → fill with bots → run 60 s → end → close cycles in one host with zero
  leaked rooms, threads, sockets or ports, and memory back within 1 MB of baseline after the cycles; (2) a room stuck in
  Created past its activation timeout is closed and reported; (3) results and telemetry for a match are written before
  Closed in 100 % of normal ends.
- **R2-ROOM-REC2. One room per OS process to start; inside it, the sim on its own thread and network I/O on an async
  runtime.** The supervisor (R6) starts a process per room; the lobby hands out host and port. Option B (many rooms per
  process) is the fallback only if the spike shows per-process memory or start-up cost is too high for the host budget,
  and then needs `panic = "unwind"` with per-room containment.
  Reason: it is the default model of GameLift and PlayFab (F1, F4, F5), it gives the strongest isolation for a fair
  tick (VISION §4), teardown is process exit, and density comes from processes per host (F4); many rooms per process
  is "more economical" but needs "more management" (F6), and rooms must not share an async worker (F7).
  Rests on: F1, F4, F5, F6, F7, F8, F18. Serves VISION §6.1, §4 (fair up to about 100 ms of ping).
  Spike must prove (proposed; reference host proposed: one 4-vCPU EU cloud VM, R6 to name it): (1) one 48-player room
  (bots) holds its tick: p99 tick time at most 50 % of the tick period chosen by the tick-rate unit, over 20 minutes;
  (2) 8 such rooms as 8 processes on the reference host: each room's p99 within 10 % of the single-room figure;
  (3) per-room process memory measured (RSS at start and after 20 minutes) and reported, with a proposed ceiling of
  64 MB for a 48-player room; (4) killing one room's process (SIGKILL) leaves the other rooms' tick p99 and connections
  unchanged; (5) room start-up (process spawn to Open) under 2 s.
- **R2-ROOM-REC3. Join with a short-lived signed token from the lobby, checked by the room offline (option 3.2 A).**
  Claims: player id, guest flag, display name, room id as audience, team/slot if pre-assigned, expiry (proposed 30 s
  after issue, leeway at most 5 s), unique id. The room rejects a token whose audience is not itself, that has expired,
  whose unique id it has already seen within the expiry window, or with a bad signature. The token travels in the first
  message of a reliable stream, never in the URL. The room, not the client, assigns team and squad within the mode's
  rules (GW1, GC1).
  Reason: an offline check keeps the lobby off the hot join path; single use and the audience stop replay and
  cross-room use (F9, F10, F11); browsers cannot send an auth header (F12) and URLs leak (F14).
  Rests on: F9, F10, F11, F12, F14. Serves VISION §6.1. Signature scheme and key handling: open (OQ2).
  Spike must prove (proposed): (1) 1,000 generated bad tokens (expired, wrong room, replayed, altered byte, wrong key,
  missing claim) are all rejected, and 1,000 good ones all accepted; (2) a join completes (transport ready to first
  snapshot) within 1 RTT + 100 ms at 0-200 ms simulated latency with 5 % loss; (3) no token appears in server access
  logs.
- **R2-ROOM-REC4. Reconnect by holding the slot for a grace window and rejoining with a fresh token (option 3.3 A).**
  A dropped player keeps slot, team and squad for a grace window (proposed 60 s, matching F2's reservation timeout as a
  starting value); the client fetches a new token for the same room; the newest connection for a player id wins and the
  older one is closed; after the window the slot follows the bot-fill rule.
  Reason: the netcode standard makes tokens single-use and sends a rejected client back to the backend (F10); a
  reservation with a timeout is how GameLift holds slots (F2); one connection per player id prevents duplicate
  soldiers (F9). Slot id = sim id (GC2) makes the rebind cheap.
  Rests on: F2, F9, F10. Serves VISION §6.1. What the soldier does meanwhile: OQ3.
  Spike must prove (proposed): (1) in the network simulator, 200 forced drops of 1-55 s at 0-200 ms latency, 0-5 %
  loss: 100 % rejoin the same slot and team, with no duplicate soldier; (2) a second tab with the same player id takes
  over within 1 s and the first is closed; (3) a drop longer than the window hands the slot to the fill rule.
- **R2-ROOM-REC5. Private rooms: a short code that only locates the room, a host with server-checked rights, settings
  fixed at start (option 3.4 A).** Codes come from a cryptographically secure random source, are unique among live
  rooms, die with the room, and lookups are rate-limited per client and per address (F13's rate-limit advice applied to
  lookups, inferred); joining still needs a token (REC3). The creator is the host; the room checks every host command
  (start, kick, lock, team and faction settings) against the host's player id. For organized play the settings carry a
  faction per team (VISION §3 [DECIDED]); which settings exist beyond that, and host migration, are design (OQ5, OQ6).
  Reason: a human-readable code is far below a secret's entropy (F14), so it must only locate; PlayFab fixes room
  policy at creation and gives the owner sole write rights (F17).
  Rests on: F14, F17 (F13 for rate limiting). Serves VISION §6.1 (private matches), §3 (organized play).
  Spike must prove (proposed): (1) 10,000 codes generated with no collision among live rooms; (2) lookups above the
  limit (proposed 10 per minute per client) are refused; (3) every host command sent by a non-host is rejected; (4) a
  private room created with factions {Soviets, Allies} on a two-team mode starts with exactly those factions.
- **R2-ROOM-REC6. Bots fill per team on the server and drive the same input path as humans (option 3.5 A).** The slot
  table records for each slot whether it is human, bot or empty; bots per team = team size − humans on that team; a
  joining human takes a bot's slot; bots produce the same input commands a client sends, so the sim cannot tell them
  apart (inferred from the brief's one-simulation principle and legacy CS1, the unused `makeInputCommand` record). The bots' shared sight
  budget scales with the room size (CD11).
  Reason: a shipped game keeps servers full this way (F15); legacy already plans humans taking bot slots (GC2, STATUS
  :68).
  Rests on: F15. Serves VISION §3 ("Bots fill empty slots"), §6.1.
  Spike must prove (proposed): (1) 3 teams × 16 slots, 30 humans (simulated clients) joining and leaving in random
  order for 10 minutes: after every change, within 1 s, each team has exactly its size in humans plus bots and no team
  exceeds it; (2) a human never waits for a slot while a bot holds one on the team they are assigned to; (3) the same
  seed and the same join/leave script give the same per-tick state hash on two runs.
- **R2-ROOM-REC7. Modes as validated data; a room is a mode id plus allowed overrides (option 3.6 A).** The mode
  defines team count, team size, faction per team (or "chosen by host"), squad size, rules table, maps, bot policy and
  match length; the server rejects at load any file with unknown keys, out-of-range values (team count other than 2 or
  3, size 0) or prototype keys (SEC-MDL-2); the slot table and team index are built from the mode (GC2, CD10).
  Reason: VISION §3 [DEFAULT] and §6.1 want all of it as data; legacy shows teams as data work (CD10, GC2) and that
  unchecked merges are a hole (GW2).
  Rests on: legacy CD10, GC2, GW2 (no external finding needed); format per R1. Serves VISION §6.1.
  Spike must prove (proposed): (1) mode files for 2×2, 2×8, 3×6, 3×18, 12v12v12 and 24v24 load and each runs 60 s
  with bots, with no code change between them; (2) 20 malformed mode files are all rejected at load with a named error.
- **R2-ROOM-REC8. Guest first: an anonymous guest account from the lobby (option 3.7 A).** On first visit the lobby
  issues a guest id and a random secret of at least 64 bits; the client stores them (the KVStore successor) and uses
  them to get join tokens; a later login links to the same id. The room only ever sees the join token.
  Reason: zero-input anonymous accounts are the recommended start, with a recoverable login added later (F16); the
  secret must be unguessable (F14, F16).
  Rests on: F14, F16. Serves VISION §6.1 (a player identity for rooms and reconnects) and prepares §6.10 [LATER]
  without building it; VISION §5 [DECIDED].
  Spike must prove (proposed): (1) a fresh browser profile reaches a room with no input beyond "Play"; (2) a page reload
  keeps the same guest id and rejoins the held slot (with REC4); (3) the secret is 128 random bits from the OS source
  (proposed; at least 64 required).

Room control messages (proposed, reliable stream; per-player cost is a few hundred bytes per event, negligible
against snapshots, inferred): `Join{token}` → `JoinAccepted{slot, team, squad, modeId, modeHash, serverTick}` or
`JoinRejected{reason}`; `RoomState{phase, countdown, roster}` on change; `HostCommand{start | kick | lock | settings}`
(private rooms); `RoomEnded{results}`; `Leave`. The lobby speaks HTTPS: guest create, room create (private), code
lookup, token request.

### Brief question 6: learn from shipped games

No recommendation of its own. The lessons are the basis of the recommendations above: their "Rests on:" lists name the findings, and the index under Findings maps those findings to the four sources the brief names.

## Open questions

First each unit's own, then the assembly's cross-unit questions, the pointers, and the recommendations that hinge on a source not read.

### Brief question 1: snapshots, client prediction, interpolation and lag compensation for 48+ players (sync unit)

- **R2-SYNC-OQ1. Snapshot and tick rates.** They set the numbers in REC4 (the delay in milliseconds), REC5 (the cap) and
  REC1 (the ring length in seconds). What decides it: the tick-rate and bandwidth unit, then this unit's marks
  recomputed.
- **R2-SYNC-OQ2. The rewind cap: 200 ms (the netcode default principles) or 250 ms (proposed here)?** A higher cap is fairer to the
  shooter at 100 ms and lengthens the victim's "hit behind cover" window (F34). What decides it: the REC5 spike's numbers
  at P2 and P3, then Mustafa and the Advisor weigh the feel (the Feel Lab, LOOP_V2 §7).
- **R2-SYNC-OQ3. When the server runs commands.** It can run each client's commands as they arrive, possibly several in
  one tick, within a cap (Source's approach, F33). Or it can buffer them and run exactly one per client per tick,
  repeating the last command on a miss. Arrival order affects determinism records and speed-hack exposure. What
  decides it: a spike comparing correction rates and fairness under P2–P4. The Overwatch netcode talk would inform it
  but was not readable (video only).
- **R2-SYNC-OQ4. Snapshots larger than one datagram.** At 48+ a snapshot may not fit one QUIC packet, which cannot be
  fragmented (F11). One option is the application's own fragmentation, all or nothing (losing one part loses the
  snapshot). The other is independent parts, each a delta of some entities against the same baseline and acknowledged
  separately. What decides it: the bandwidth unit's snapshot size at 48+ and the transport unit's datagram payload.
- **R2-SYNC-OQ5. Collision between predicted players and remote players or vehicles.** Remote movers are drawn in the
  past, so colliding with them mispredicts (Quake III collides the predicted player with the snapshot's solid entities,
  F17). Soldiers could not collide with each other, collide softly, or collide
  fully. What decides it: a game-design call (does body-blocking matter in LOOP_V2's fights?) and the REC7 spike's
  correction rate.
- **R2-SYNC-OQ6. Projectile spawn fast-forward.** The options are none, a fixed prestep (Quake III's 50 ms, F35), or the
  shooter's latency up to the cap. Fast-forward makes throws land where aimed at high ping, but lets high-ping
  projectiles appear mid-flight for others. What decides it: a Feel Lab test under P2 and P3.
- **R2-SYNC-OQ7. Several seats in a vehicle (VISION_V2 §6.4 [OPEN]).** Who predicts the vehicle, and how a gunner's aim
  relates to a hull the gunner does not drive. What decides it: Mustafa's seat decision, then a design pass.
- **R2-SYNC-OQ8. What else gets rewound.** Destructible statics (the alive flag) and large moving vehicles as cover (R4's
  moving obstacles) could be judged at present (simpler, favours the victim) or rewound. What decides it: the REC5
  spike with a destructible wall and a moving Juggernaut as cover.
- **R2-SYNC-OQ9. Melee and short-range abilities.** Should the Banzai blade (one-hit kill, 3.6 m, 120°), the bayonet and
  the Riot Shield block be judged at the attacker's view time? At 9 m/s a target moves about 1.8 m in 200 ms, half the
  blade's reach (inferred). What decides it: a game-design call plus the REC5 spike extended to melee.
- **R2-SYNC-OQ10. Predicting abilities (option 3.2 D).** No source on predicting abilities and rolling them back was
  read in this run; the Overwatch talks are video only. What decides it: a Feel Lab comparison of predicted against
  server-confirmed Molotov and Shield, or a readable source.
- **R2-SYNC-OQ11. The shared spread seed.** A seed the client knows makes predicted tracers match the server's spread
  (F20), but lets a cheat cancel the spread. Source adds a server-only seed (F20), whose purpose is inferred. What
  decides it: security-auditor's protocol review, then Mustafa (feel against cheat resistance).
- **R2-SYNC-OQ12. Hermite interpolation.** It needs velocity in snapshots (more bytes) and is only worth it if linear
  interpolation shows artefacts at the chosen snapshot rate (F22). What decides it: the REC4 spike at that rate, with
  sprinting soldiers and the Juggernaut.
- **R2-SYNC-OQ13. Bots see the present, humans see the past.** Bots act on present state (F31) while humans see entities
  about 100 ms late. What decides it: the game-designer's bot reaction tuning (GN4 scans every 0.25 s), checked in the
  48-player spike.
- **R2-SYNC-OQ14. Valve's wiki pages (unread, HTTP 403).** The Source Multiplayer Networking and Lag Compensation pages
  would confirm Source's default interpolation delay and its explanation of shooter against victim. No recommendation
  rests on them; a verifier who can open them should check F26 and F34.

### Brief question 2: tick rate and bandwidth budgets (budgets unit)

- **R2-BUD-OQ1.** What tick and update rates do recent large-team shooters publish (Overwatch's 63 Hz, Battlefield,
  Fortnite, Apex Legends, Valorant)? None was read: the Overwatch talk is video on GDC Vault (redirected to a
  maintenance page outside the allow list), the rest sit on vendor blogs and press outside the allow list. Decides:
  reading them in a later run, if their domains are added; until then the spike's own numbers decide.
- **R2-BUD-OQ2.** Source's default snapshot rate and interpolation delay (`cl_updaterate`, `cl_interp`) are engine-side
  and not in the SDK; the Valve Developer Community pages refused the fetcher (HTTP 403). Decides: reading those pages
  in another run; REC4 does not rest on them.
- **R2-BUD-OQ3.** What maxDatagramSize does each browser report, and how much below 1200 is it? Decides: the spike's
  per-browser measurement (REC3); the stack unit owns the server side.
- **R2-BUD-OQ4.** Octopon-3's size and height range (LOOP_V2 §9 `[OPEN]`) set the position bits; beyond ±327 m or 41 m
  of height, add a bit or use 2 cm. Decides: the map change-set's greybox.
- **R2-BUD-OQ5.** Caps on vehicles and live projectiles per team are not in VISION_V2 or LOOP_V2; the estimate assumes
  one vehicle per squad and 32 projectiles. Decides: game-designer's caps; the spike then tests the cap as the worst
  case.
- **R2-BUD-OQ6.** VISION_V2 §3 says "two teams with 2–8 players, and three teams with 6–18 players": totals or per
  team? The estimate reads them as totals; per team they would mean up to 16 and 54 players. 54 is above the 48 design
  point but inside the estimate's 64-player row. Decides: Mustafa.
- **R2-BUD-OQ7.** Hermite or linear interpolation (F33): Hermite adds a velocity. Sent absolute (3 × 10 bits per
  moving entity, 62 moving entities at 48 players) it costs about +56 kbps at 30 Hz (arithmetic), which would push the
  48-player case to about 202 kbps; delta-coded it costs less. Decides: the snapshot-model unit and the spike's visual
  check.
- **R2-BUD-OQ8.** Can the client simulate projectile flight deterministically from a spawn event (E3), or must
  projectiles ride in every snapshot (E3b, +42 kbps at 48)? Decides: R1's determinism unit and the spike's desync count.
- **R2-BUD-OQ9.** The real cost of encoding 48 per-client delta snapshots and of encryption per datagram is unknown.
  Decides: REC6's spike.
- **R2-BUD-OQ10.** How the server's QUIC congestion controller (and the browser's) behaves for a 30 Hz stream of
  600-byte datagrams under 2–5 % random loss (does it throttle below 150 kbps?). Decides: the spike under the network
  simulator; the stack unit names the controller.
- **R2-BUD-OQ11.** Share of players on IPv6 (+20 B per datagram) on the chosen EU host. Decides: R6 and the spike's
  telemetry.

### Brief question 3: interest management for a top-down camera (interest unit)

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

### Brief question 4: the WebTransport server stack in Rust plus the WebSocket fallback (transport unit)

**R2-TRN-OQ1. Which WebTransport draft does each browser negotiate today, and when do they move to -16?** Only Chromium's QUICHE
constants were read (F5). Nothing read says what Firefox 158 or Safari 27 send. Both Rust crates speak the -02/-07 settings (F6).
If a browser drops them before the crates add -16's `SETTINGS_WT_ENABLED`, connections fail. Decided by: the REC1 interop
spike (log the negotiated settings), plus a recurring test against Chrome Beta, Firefox Nightly and Safari Technology Preview
(R5's CI).

**R2-TRN-OQ2. Does Chrome or Firefox fall back to WebTransport over HTTP/2 the way Safari does, and does any Rust crate serve it?**
The sources read document only Safari's fallback (F12). Chrome lacks `requireUnreliable` (F11), so its behaviour without UDP is
unknown. Decided by: the REC3 spike with UDP blocked, and the crates' own documentation (none of the pages read mentions HTTP/2).

**R2-TRN-OQ3. What `maxDatagramSize` do the four browsers report on real paths?** The spec lets it vary and change (F18). Decided
by: the REC2 measurement.

**R2-TRN-OQ4. Production certificates: public certificates per hostname or short-lived pinned ones?** This depends on how R6
hosts game servers (fixed hosts or ephemeral machines) and on the HTTPS matchmaker as the trusted intermediary (F14). Decided by:
R6's hosting choice, then the REC5 spike.

**R2-TRN-OQ5. wtransport or web-transport-quinn?** The maintenance risk differs: "not considered completely production-ready"
(F29) versus fast 0.x churn (F30). Contributor counts and issue response times were not read. Decided by: the REC1 spike pass
marks, plus a short maintenance check by rust-reviewer (releases in the last 6 months, open security issues on rustsec.org).

**R2-TRN-OQ6. Is qmux worth adopting as the fallback framing later?** It would give real streams over WebSocket and one API over
both (F30, F32), but it is draft -02. Decided by: whether REC4's thin interface needs more than one ordered channel over WebSocket
in the spike. If it does, re-assess qmux.

**R2-TRN-OQ7. How many players will need the WebSocket fallback?** No source read gives a share of networks that block UDP or
QUIC. Decided by: connection telemetry once a playtest runs (R6 monitoring). This sizes how good the fallback must feel.

**R2-TRN-OQ8. Does the browser's own congestion controller limit client inputs in practice?** Inputs are small, but they are
congestion-controlled too (F16), and Chrome ignores the hint (F10). Decided by: the REC6 netem spike, measured on the client
side.

**R2-TRN-OQ9. Valve's Source networking page could not be read (HTTP 403).** It was not needed for any transport
recommendation, and the snapshots and prediction unit covers it.

### Brief question 5: rooms and private matches (rooms unit)

- **R2-ROOM-OQ1. Process per room or many per process, finally?** Decided by REC2's spike numbers (per-process memory,
  start-up, tick p99 with neighbours) against R6's host sizes and costs.
- **R2-ROOM-OQ2. Token signature scheme and keys.** Asymmetric signature (rooms hold only a public key) or a shared key
  with AEAD as in netcode.io (F9, F10)? Key rotation and storage? Not researched here (R6's security baseline is
  deferred by the Advisor; security-auditor reviews R2's protocol). Decided by security-auditor's review.
- **R2-ROOM-OQ3. The soldier of a dropped player during the grace window.** Stands still, bot-driven, or removed? A game
  rule: Mustafa (game-designer can model it).
- **R2-ROOM-OQ4. Joining a match in progress.** May humans join a Running public or private match? Does the bot leave
  at once or at its next death? Team auto-balance? Game rules: Mustafa.
- **R2-ROOM-OQ5. What a private-room host may set.** Faction per team is decided (VISION §3). Team size, bots on or off,
  mode rules, map, how clan members are put on one team (VISION §7 lists clan and guild features as open): Mustafa.
- **R2-ROOM-OQ6. Host leaves.** Pass host automatically to another connected human (PlayFab "Automatic", F17), or leave
  the room hostless? Design, with the engine cost small either way.
- **R2-ROOM-OQ7. Do private matches count?** Service points (LOOP_V2 §8) and balance telemetry (§6.6): a tag on the
  match record is cheap; whether they count is Mustafa's call.
- **R2-ROOM-OQ8. Can WebTransport survive an address change without a reconnect?** QUIC has connection migration;
  browser support for it in WebTransport was not researched here (transport unit). It changes how often REC4 runs, not
  its design.
- **R2-ROOM-OQ9. Where the lobby runs and stores guest ids and the room directory.** Same binary as the supervisor or a
  separate service; which store: R6.
- **R2-ROOM-OQ10. Code length and alphabet.** 6 symbols from 32 is a proposal; whether a password is also offered for
  private rooms is design plus security-auditor.

### Cross-unit open questions (assembly)

Where two units disagree, both findings and recommendations stand as written above; these questions name the conflict
and resolve nothing. Each says what the drafts leave to decide it.

- **R2-XU-OQ1. The rewind cap: 250 ms (sync) or 200 ms (budgets)?** R2-SYNC-REC5 proposes a cap of 250 ms and
  R2-SYNC-OQ2 asks "200 ms (the netcode default principles) or 250 ms (proposed here)?". R2-BUD-REC4 caps the rewind at
  200 ms, R2-BUD-REC1's pass mark asks for "rewind ≤ 200 ms for 100 % of shots", and budgets option table A, row T4 (20
  Hz snapshots), reads "rewind 200–250 ms: breaks the 200 ms cap at 100 ms RTT". Decided by: R2-SYNC-OQ2's route (the
  REC5 spike at P2 and P3, then Mustafa and the Advisor in the Feel Lab), after R2-XU-OQ2.
- **R2-XU-OQ2. Is the latency term in Source's rewind formula a round trip or one way?** The sync unit reads it as the
  round trip: R2-SYNC-F28 quotes `correct+= nci->GetLatency( FLOW_OUTGOING );` and, from `inetchannelinfo.h`,
  `virtual float GetLatency( int flow ) const = 0; // current latency (RTT), more accurate but jittering`. Its
  profiles therefore inject round-trip time, and R2-SYNC-REC5 gets 200–250 ms at 100 ms. The budgets unit reads the
  same line as "outgoing latency" (R2-BUD-F29). Its option table A uses "one-way latency (assumption: half of a
  symmetric 100 ms RTT)" and gets 150–200 ms; `inetchannelinfo.h` is not among the budgets unit's sources.
  R2-INT-REC1's motion margin also uses one-way latency. Decided by: the verifier's reading of
  `player_lagcompensation.cpp` and `inetchannelinfo.h` at the source; the rewind numbers in both units follow.
- **R2-XU-OQ3. Interpolation delay: bounds and extrapolation marks.** R2-SYNC-REC4 starts at 3 snapshot intervals and
  adapts within "2 to 5 intervals, as data". Its mark is extrapolation in fewer than 1 % of rendered frames under P2
  and fewer than 3 % under P3. R2-BUD-REC4 uses "100 ms by default (3 snapshot intervals at 30 Hz), adapting up to
  150 ms". Its mark is "≤ 0.5 % of rendered frames need extrapolation with a 100 ms delay" at 100 ms RTT, ±10 ms and
  2 % loss, and "at 5 % loss the adaptive delay stays ≤ 150 ms". Decided by: one bound and one mark in the synthesis,
  then the REC4 spikes. The duplicate question on Hermite against linear interpolation (R2-SYNC-OQ12, R2-BUD-OQ7)
  rides with it.
- **R2-XU-OQ4. Input redundancy and the size of a command.** R2-SYNC-REC2 repeats "all commands the server has not
  yet acknowledged, up to a cap (proposed 8, as data)". R2-BUD-REC1 and option U1 send the "newest input + 3
  previous, 8 bytes each", and R2-BUD-REC1's mark is "upstream ≤ 48 kbps per client (IPv4, U1)". The sync unit's
  command carries tick and sequence, move vector, buttons, press counters, the aim and the view time (R2-SYNC-REC2),
  and its aim point alone may take up to 80 bits, which is 10 bytes (R2-SYNC-REC8, mark 2). The transport unit's load
  test uses input datagrams of at most 100 bytes as placeholders (R2-TRN-REC1, mark 2). Decided by: the command
  layout (sync), then the budgets unit's upstream rows recomputed with it.
- **R2-XU-OQ5. The snapshot datagram cap: 1,000 bytes (transport) or 1,150 bytes (budgets)?** R2-TRN-REC2 caps
  "each snapshot datagram at a proposed 1,000 bytes of payload", and its mark 1 passes at "≥ 1,000 bytes on all
  four" browsers. R2-BUD-REC3 allows up to "min(the runtime maximum datagram size, 1150 bytes of WebTransport
  payload)", from its worked limit of 1,157–1,171 usable bytes. R2-SYNC-REC1's mark 3 defers to "the transport unit's
  payload size". The budgets unit's projectile-free full snapshot at 48 players, about 1,021 bytes (R2-BUD-REC3), lies
  between the two caps. Both units propose splitting a large snapshot into entity-group datagrams that decode alone
  (R2-BUD-REC3 option P1; R2-TRN-REC2), which is one of the two options R2-SYNC-OQ4 leaves open. Decided by: the
  per-browser `maxDatagramSize` measurements (R2-BUD-OQ3 = R2-TRN-OQ3), then one cap.
- **R2-XU-OQ6. How a joining or resyncing client gets its first full state, and how fast.** R2-SYNC-REC1 encodes a
  client with no usable baseline "against the per-entity defaults", in datagrams. R2-BUD-REC3 sends "the join and
  full-resync snapshot … over a reliable stream (P4) unless the spike shows P1 is enough". The rooms unit's
  cross-unit note says a join or rejoin "needs a full baseline snapshot and reset acks". The time marks differ:
  R2-BUD-REC3 asks for "a complete world within 300 ms at 100 ms RTT and 2 % loss"; R2-ROOM-REC3 asks for a join
  "(transport ready to first snapshot) within 1 RTT + 100 ms at 0-200 ms simulated latency with 5 % loss";
  R2-SYNC-REC1 asks that clients decode deltas again "within 500 ms of the link coming back" after the B1 burst.
  Decided by: one join path and one join mark in the synthesis, then the spikes.
- **R2-XU-OQ7. Hit-history length and the rewound-shot load.** R2-SYNC-REC5 keeps "the cap plus a margin" (about
  0.1 MB for 60 movers if it kept 1 s at 60 Hz, inferred) with a mark of at most 1 MB, and its mark 4 sizes the rewind
  query for 700 shots per second (the upper bound "at 48 players all on full auto", R2-SYNC-REC9 and its `WEAPONS`
  legacy row). R2-BUD-REC4 keeps "1 s of hitbox history", and budgets table E estimates about 184 KB per room and
  "about 72 rewound shots per second at 48 players" (assumptions). Decided by: the REC5 spike (sync) and the CPU
  accounting of R2-XU-OQ8.
- **R2-XU-OQ8. One CPU accounting for the server tick.** The units give shares of different wholes:
  - R2-BUD-REC6: the whole tick at p99 ≤ 25 % of the tick interval (≤ 4 ms at 60 Hz, ≤ 8 ms at 30 Hz), simulation,
    hit tests and encoding included; encoding ≤ 50 µs per client per snapshot.
  - R2-ROOM-REC2: one 48-player room's tick at p99 ≤ 50 % of the tick period.
  - R2-SYNC-REC9: snapshot encoding, lag compensation and command handling together ≤ 25 % of "the tick budget" at
    p99. R2-SYNC-REC1 mark 4 defers to "the share of the tick budget the tick-rate unit sets", and R2-SYNC-REC5 mark 4
    sets ≤ 20 µs per rewound shot at p99.
  - R2-INT-REC3: the visibility pass ≤ 10 % of "the server tick budget". R2-INT-REC1: the region query ≤ 0.2 ms per
    tick. R2-INT-REC4: ranking and packing ≤ 0.5 ms per tick. R2-INT-REC2: ≤ 0.1 µs per enemy pair, and a re-flood
    ≤ 1 ms.
  - R2-TRN-REC1: the transport layer on at most 1 CPU core, with p99 send-call time ≤ 1 ms.

  The term "tick budget" has no single definition, and the shares are not summed against one total anywhere. Decided by:
  the synthesis, which sets one tick budget and divides it; then the spikes.
- **R2-XU-OQ9. The reference server.** R2-BUD-REC6 proposes "one vCPU of the EU instance type R6 recommends", and
  R2-ROOM-REC2 proposes "one 4-vCPU EU cloud VM, R6 to name it" (8 rooms on it). R2-TRN-REC1 asks for "reference
  server hardware R6 names". The sync unit's marks (R2-SYNC-REC1, REC5, REC9) and the interest unit's marks (REC1,
  REC3, REC4) name "the proposed reference server" without defining it. Decided by: R6's instance choice; the synthesis
  names one reference machine.
- **R2-XU-OQ10. Can the browser read its own connection statistics?** R2-BUD-F12 and R2-BUD-F13 (from the W3C
  Editor's Draft at `w3c.github.io`) describe `smoothedRtt`, `rttVariation`, `estimatedSendRate`, `expiredOutgoing`
  and `lostOutgoing`. R2-BUD-REC4 takes the clock offset from "the browser's smoothedRtt and rttVariation plus our own
  timestamps". R2-BUD-REC5 reads "estimatedSendRate, lostOutgoing and expiredOutgoing (browser side)". R2-TRN-F9 (MDN
  browser-compat-data) says `getStats()` is `"version_added": false` in Chrome, and in Firefox "Method is defined but
  throws a not-implemented error."; only Safari 26.4 has it. R2-TRN-F27 says the dictionary is "Usable only on Safari
  today", and R2-TRN-REC4 measures RTT with our own ping messages. Decided by: the verifier's check of R2-TRN-F9 at its
  source; the budgets recommendations then keep or drop the browser-side counters.
- **R2-XU-OQ11. Where the outgoing maximum age applies, and its value.** R2-BUD-REC5 would "set the outgoing maximum
  age to about one snapshot interval so stale snapshots expire instead of queueing". The age it describes,
  `outgoingMaxAge`, is a setting of the browser's API (R2-BUD-F13, R2-TRN-F19). R2-TRN-REC6 gives "client input
  datagrams a short `outgoingMaxAge`, proposed 100 ms". It keeps stale snapshots out of the server's queue through
  quinn's non-waiting `send_datagram` with a check of `datagram_send_buffer_space`. Decided by: the budgets and
  transport recommendations naming one guard per direction; R2-TRN-REC6 mark 1 measures snapshot age.
- **R2-XU-OQ12. The network profiles behind the pass marks.** The sync unit defines P0–P4 and B1, with P2 = 100 ms
  round trip, ±20 ms jitter and 2 % loss. The budgets marks use 100 ms RTT, ±10 ms and 2 % loss (R2-BUD-REC1, REC4).
  R2-TRN-REC2 uses 100 ms RTT, ±20 ms and 2 % loss. R2-INT-REC1 uses "0, 50, 100 and 200 ms latency with ±20 ms
  jitter and 0–5 % loss". The rooms marks use "0-200 ms simulated latency" with 0–5 % loss. The interest and rooms
  marks do not say whether "latency" means round trip or one way. Decided by: one set of named profiles for all R2
  spikes (R5's network simulator runs them).
- **R2-XU-OQ13. Team sizes: totals or per team?** The budgets estimate reads VISION_V2 §3's "two teams with 2–8
  players, and three teams with 6–18 players" as totals (rows 4v4 and 6v6v6) and raises it as R2-BUD-OQ6. The rooms
  unit's marks read them per team: mode files "2×2, 2×8, 3×6, 3×18" (R2-ROOM-REC7) and "3 teams × 16 slots"
  (R2-ROOM-REC6). Decided by: Mustafa (R2-BUD-OQ6).
- **R2-XU-OQ14. The legacy top speed of a soldier.** The units derive three figures from the legacy tables:
  - sync (`T` row): "`moveSpeed` 6.3 m/s, `sprintMult` 1.4286 (about 9.0 m/s); unit `speedMult` up to 1.5
    (Warrior)". It feeds the 1.8 m lead in R2-SYNC-F34 and REC5, and the 9 m/s target in R2-SYNC-REC5's mark 1.
  - interest (CD4, `UNIT_TYPES` row): "sprint ×1.43 (T), up to ×1.62 per class (about 10.2 m/s)". It feeds
    R2-INT-REC1's motion margin.
  - budgets (GP3 row, `shamble-game.js:2774-2775`): "at most about 13.3 m/s", with sprint, Banzai and shield factors.
    It feeds the delta-record range and the velocity bounds, and that row adds "Bots' `speedMult` path was not
    checked".

  Decided by: the verifier, against `docs/LEGACY_TUNING.md` and the cited game lines; then one top speed per unit
  type.
- **R2-XU-OQ15. Snapshot size per client: the interest illustration against the budgets estimate.** R2-INT-F19
  illustrates "47 soldiers × 12–25 bytes per changed record … 90–188 kbit/s per client at 20 snapshots/s". It marks the
  sizes as illustrative and leaves record sizes to the snapshots unit. The budgets estimate uses "about 60 bits
  (7.5 B)" per moving soldier delta at 30 Hz: 584 B per snapshot (609 B with events) and 146.1 kbps at 48 players, all
  in view. Decided by: the measured record sizes of the R2-BUD-REC2 spike.
- **R2-XU-OQ16. The tier of Glenn Fiedler's articles.** The sync, budgets and interest units tier them "other". The
  budgets unit says they "are named by the brief but are not a shipped game's own documentation". The transport unit
  tiers two of them "shipped-game practitioner" (R2-TRN-F20, F22). R2-TRN-F20's article ("Packet Fragmentation and
  Reassembly") is the one R2-BUD-F18 tiers "other". The rooms unit tiers one "shipped game (engineer's articles,
  named in the brief)" (R2-ROOM-F9), and tiers the netcode 1.02 standard "shipped game" too (R2-ROOM-F10). Decided
  by: the verifier, by the README rule ("Prefer official docs and specifications, then talks and papers from shipped
  games, then everything else, marked as such."); the source tables below show each unit's tier.
- **R2-XU-OQ17. Spectators and casters fall between two units.** R2-INT-REC7 says the spectator delay "is room data,
  set by the rooms unit". The interest unit's cross-unit note also asks the rooms unit "whether a private match may
  turn hiding off for casters". The rooms draft sets no spectator delay and has no spectator question: its cross-unit
  note says a spectator or caster "(not in the brief) would need its own interest rule; no research done". Decided
  by: Mustafa (whether rooms have spectators or casters at all), then a rooms follow-up.
- **R2-XU-OQ18. Four recommendations name no VISION_V2 §6 item.** R2-BUD-REC3 serves "§3 (48+ players), §4 (browsers
  first)". R2-BUD-REC4 and R2-BUD-REC5 serve "§4 (fair to about 100 ms…)". R2-TRN-REC6 serves "the §4 fairness target
  (about 100 ms) at the §3 scale (48+ players)". `CLAUDE.md` asks every engine feature to trace to a §6 need. Decided
  by: the fixer (a §6 item the draft's own reasoning supports) or the Advisor (whether a §3 or §4 target is trace
  enough).

### Pointers between the R2 units

These come from the units' cross-unit notes and duplicate questions; they claim nothing new.
- **Same question in two units:** R2-SYNC-OQ12 = R2-BUD-OQ7 (Hermite or linear interpolation). R2-BUD-OQ3 =
  R2-TRN-OQ3 (each browser's `maxDatagramSize`). R2-SYNC-OQ14, R2-BUD-OQ2 and R2-TRN-OQ9 (the Valve wiki pages, HTTP
  403). R2-BUD-OQ10 and R2-TRN-OQ8 (how the congestion controllers treat the game's streams; R2-TRN-REC6 mark 3
  compares NewReno and Cubic).
- **Questions another unit answers with a proposal:** R2-SYNC-OQ1 (snapshot and tick rates) has R2-BUD-REC1, and the
  sync marks are to be recomputed once the rates are chosen. R2-SYNC-OQ4 (snapshots larger than one datagram) has
  R2-BUD-REC3 and R2-TRN-REC2, which disagree on the cap (R2-XU-OQ5). R2-BUD-OQ8 (projectiles as spawn events) meets
  R2-SYNC-REC6, whose mark 1 asks for a bit-identical trajectory function on Wasm and native. The interest unit
  leaves to the sync unit how an entity entering a client's set is coded ("static baseline as in Q3, F3, or full
  state"), how its removal is signalled, the record sizes and the entity-id space ("Q3 caps at 1,024, F2"); the sync
  unit starts late join and rejoin from per-entity defaults (R2-SYNC-REC1).
- **Links the units asked for:** R2-SYNC-F11 says "The link from WebTransport datagrams to RFC 9221 is the transport
  unit's to confirm (inferred here)", and R2-TRN-F16 quotes it (draft -16 §4.5). R2-ROOM-OQ8 (an address change without
  a reconnect) meets R2-TRN-F39 (QUIC connection IDs); neither unit researched browser support for it.
- **Lag compensation and interest:** R2-SYNC-REC5 rewinds only targets "sent to and acknowledged by the shooter", and
  the interest unit's note adds "a shooter cannot hit what it was never sent". So the interest set also bounds who can
  be hit.
- **One-shot actions:** the transport unit leaves "input redundancy or the reliable stream" to the prediction unit.
  R2-SYNC-REC2 proposes press counters inside redundant commands, and R2-TRN-REC2 mark 3 tests both paths.
- **The WebSocket fallback and sync:** the sync unit leaves its effect on snapshot delay to the transport unit
  (R2-TRN-REC3 mark 3), and R2-SYNC-REC1 is meant to work over either transport.
- **Rooms and transport:** the join flow, slot takeover and team allow-list checks run on the reliable control
  stream; the protocol version can ride in `protocols` (R2-TRN-F38); WT_DRAIN_SESSION can drain a session before a
  server shuts down (R2-TRN-F37). Several rooms per process would route by `:path` (R2-ROOM-F13), and one process per
  room gives one port per room (R2-ROOM-F18). The rooms unit notes per-process certificates if
  `serverCertificateHashes` is used (certificates: R2-TRN-REC5).
- **Rooms and budgets:** R2-BUD-REC6's per-room CPU and memory set rooms per core (R2-ROOM-REC2), and
  R2-ROOM-REC2's tick mark uses the budgets unit's tick period.
- **Interest and budgets:** R2-INT-REC4 needs the per-client byte budget and the snapshot rate from the budgets unit
  (R2-BUD-REC1, REC2). The budgets estimate is all in view: the interest cut scales only "the per-entity part and the
  events" ("about 85 % of the 48-player snapshot").

- **Sync and budgets:** the sync unit asks that GC5's cooldown rounding ("fire timers reset instead of carrying the
  remainder") be fixed in any tick choice, and that the full-snapshot cost after a long loss (R2-SYNC-REC1) and the aim
  point's bit cost (R2-SYNC-REC8) be counted in the budget; Fiedler's quantisation figures are in its notes (S-GF3).
  The budgets unit's note leaves the baseline ring to the sync unit.
- **Sync and interest:** the snapshot's entity cap and priority order feed R2-SYNC-REC1 and REC9.
- **Sync and transport:** snapshots go as datagrams, each fitting one QUIC packet (R2-SYNC-F11, OQ4); hit
  confirmations go as reliable events (R2-SYNC-REC5). The transport unit's note adds that quinn's server-side `rtt()`
  is available (R2-TRN-F25), and that deltas against the last acknowledged snapshot fit lossy datagrams.
- **Sync and rooms:** late join and rejoin start from per-entity defaults (R2-SYNC-REC1), and a human taking over a bot
  slot keeps the entity id (GC2, STATUS.md:68). The rooms unit's note adds that `JoinAccepted` carries the server tick
  for clock sync.
- **Budgets and transport:** the budgets unit leaves to the transport unit the connection-ID and packet-number lengths
  (1157–1171 B usable), the server-side maximum datagram size API, the congestion controller (R2-BUD-OQ10), ACK
  frequency, the fallback's overhead per message (R2-BUD-F16, F17) and head-of-line blocking. The transport unit's note
  says the congestion window "can collapse to 2 × max_datagram_size" (R2-TRN-F24) and bounds the snapshot budget, and
  that rate adaptation can read quinn's RTT and loss figures (R2-TRN-REC6).
- **Interest and transport:** public and team events (R2-INT-REC5, REC6) ride the reliable stream and spatial state
  rides datagrams; per-client filtering runs before encoding, "so the transport never holds data a client must not
  get". Splitting a snapshot by entity group (R2-TRN-REC2) "pairs with per-client relevance sets".
- **Rooms and the token:** the rooms unit puts the join token in the first reliable message, for both transports.

### Pointers to other topics

From the units' cross-unit notes, with no new claims:
- **R1 (simulation core):**
  - Identical results on Wasm and native set the prediction correction rate (R2-SYNC-REC3), not sync correctness
    (R2-SYNC-F5).
  - R2-SYNC-REC6 needs a bit-identical trajectory function, and spawn-only projectiles need R1's determinism
    (R2-BUD-OQ8).
  - Seeded random streams per purpose for the shared spread seed (R2-SYNC-F20) and a seeded RNG per room.
  - R2-BUD-REC1's 60 Hz depends on R1's tick-unit step cost. Weapon cooldowns carry the fractional tick at either rate,
    and swept projectile tests run per tick (GC15).
  - The visibility pass and relevance sets run on the fixed tick, in a deterministic order, checked by the state hash.
  - Where the transport lives, in the simulation worker or its own worker, is R1's choice (R2-TRN-F34, F35).
  - The simulation must be instantiable per room with no globals (GW1, GE12). The mode and rules file format, schema
    and hot reload belong to R1's data unit.
- **R3 (rendering):** the client cutaway (GW17) must follow the same visibility rule as R2-INT-REC2; the look brief
  decides how it looks (look brief pending).
- **R4 (physics and navigation):**
  - The rewound shot query needs a spatial index (CS6) and height-aware line of sight (CS9, §6.2).
  - Moving obstacles such as big vehicles as cover (R2-SYNC-OQ8).
  - Enclosure areas and portals as map data, floors, and destructible walls that open portals.
  - The cost of a ray query with a spatial index (R2-INT-OQ5).
- **R5 (toolchain and quality):**
  - The network simulator runs (0–200 ms, jitter, 0–5 % loss) behind R2-BUD-REC1–REC5, R2-ROOM-REC3, REC4 and REC6,
    and the shared profiles of R2-XU-OQ12.
  - An end-to-end test per browser for the transport paths, plus a recurring run against beta channels (R2-TRN-OQ1).
- **R6 (infrastructure):**
  - About 7 Mbit/s of egress and 3.2 GB per match-hour per 48-human room at E3, all in view; the IPv6 share
    (R2-BUD-OQ11).
  - UDP on the EU hosts and load balancers, and connection IDs for routing (R2-TRN-F39).
  - Publicly trusted certificates and their renewal; the HTTPS matchmaker that hands out certificate hashes
    (R2-TRN-OQ4).
  - Connection telemetry for the fallback share (R2-TRN-OQ7).
  - The room supervisor, fleet scaling, regions, draining on deploys and VM retirement, per-room crash reports, the
    lobby's store and signing-key storage (R2-ROOM-OQ9).
  - Telemetry of relevance churn and budget overruns; the reference server (R2-XU-OQ9).
  - R6's security baseline is deferred by the Advisor (2026-10-27).
- **security-auditor (R2 protocol review):**
  - The shared spread seed (R2-SYNC-OQ11).
  - Command validation (R2-SYNC-REC2: view-time window, command cap, aim range), and the rewind cap as an exploit
    surface (a client that fakes high latency).
  - The Origin check on WebTransport (R2-TRN-F36). The transport unit did not source a WebSocket Origin check: it did
    not read RFC 6455, which the budgets unit read for framing only (R2-BUD-F16).
  - Per-session rate limits and H3_EXCESSIVE_LOAD (R2-TRN-F36), and server-side validation of every reliable client
    message (STATUS.md:70).
  - The token signature scheme and keys (R2-ROOM-OQ2), room-code brute force, and host-command authority.
  - The reader-client and byte-audit tests as anti-cheat evidence (R2-INT-REC2, REC5).
- **rust-reviewer:** the crate choice (R2-TRN-REC1) and its maintenance check (R2-TRN-OQ5: releases in the last
  6 months, open security issues on rustsec.org).
- **game-designer and Mustafa:** R2-SYNC-OQ2, OQ5, OQ6, OQ9, OQ13; R2-BUD-OQ5, OQ6; R2-INT-OQ1, OQ3, OQ4, OQ6, OQ8;
  R2-ROOM-OQ3–OQ7; R2-XU-OQ13, OQ17.
- **The look brief (Mustafa and the Advisor):** R2-INT-OQ1 and OQ2 (what a player may see; the camera framing and
  aspect) and the cutaway (GW17) stay look brief pending.
- **Not in the brief, not researched:** matchmaking beyond rooms (public queues, skill), per the rooms unit.

### Recommendations that hinge on a source not read

For the Advisor's search for another copy (SRC-ACCESS (a), 2026-10-27). This list says only which source was not read
and why; it claims nothing about what the sources say.

| Recommendation | What hinges on it | Unread source | Why not read |
|---|---|---|---|
| R2-SYNC-REC1 | Option 3.1 B (predict everything, with rollback) was left unassessed. | Timothy Ford, "Overwatch Gameplay Architecture and Netcode", GDC 2017: https://gdcvault.com/play/1024001/-Overwatch-Gameplay-Architecture-and (also the www. form). Write-ups of the talk on edgegap.com (https://edgegap.com/blog/game-backend-deep-dive-overwatch-2016-netcode-architecture-rollback), sohu.com, juejin.cn and gamedev.net. | The talk is video only, and the vault redirected to https://gdconf.com/gdc-vault-maintenance/, outside the allow list. The write-ups are outside the allow list. |
| R2-SYNC-REC2 | When the server runs each client's commands (R2-SYNC-OQ3). | The same Overwatch talk (gdcvault 1024001). | Video only (as above). |
| R2-SYNC-REC3 | The prediction scope stops short of abilities (option 3.2 D, R2-SYNC-OQ10). | The Overwatch GDC 2017 talks on the same vault, including Dan Reed's on networking scripted weapons and abilities (the draft gives no URL for it). | Video only (as above). |
| R2-SYNC-REC4 | It cites R2-SYNC-F26, whose part on Source's default interpolation delay (`cl_interp`) is unread; REC4 uses only F26's confirmed part (Source extrapolates as a fallback). R2-SYNC-OQ14 names F26 for checking against the wiki pages and says no recommendation rests on the pages themselves. | https://developer.valvesoftware.com/wiki/Source_Multiplayer_Networking; https://developer.valvesoftware.com/wiki/Interpolation. | HTTP 403 (the domain is on the allow list; the site refused the fetch). |
| R2-SYNC-REC5 | It rests on R2-SYNC-F34, the unit's own inference from F27 and F28. The unit names these pages as the check for F34 and F26 (R2-SYNC-OQ14), and says no recommendation rests on the pages themselves. | https://developer.valvesoftware.com/wiki/Lag_compensation and .../Lag_Compensation; https://developer.valvesoftware.com/wiki/Source_Multiplayer_Networking; Yahn Bernier's paper, https://developer.valvesoftware.com/wiki/Latency_Compensating_Methods_in_Client/Server_In-game_Protocol_Design_and_Optimization. | HTTP 403 (the domain is on the allow list; the site refused the fetch). The sync unit did not attempt Bernier's paper after four 403s; the budgets unit got HTTP 403. |
| R2-BUD-REC1 | The rates are chosen without reading what recent large-team shooters publish about theirs (R2-BUD-OQ1). | gdcvault 1024001 (as above); https://edgegap.com/blog/game-backend-deep-dive-overwatch-2016-netcode-architecture-rollback; https://www.pcgamesn.com/overwatch/overwatch-netcode; https://gamesbeat.com/overwatch-netcode-tick-rate-60hz/. | Video only and a redirect outside the allow list (the talk); outside the allow list, search leads only (the rest). |
| R2-INT-REC3 | How conservative line-of-sight hiding must be (R2-INT-OQ5). | https://technology.riotgames.com/node/108 and https://www.riotgames.com/en/news/demolishing-wallhacks-valorants-fog-war; Unreal's relevancy and priority docs (https://dev.epicgames.com/documentation/unreal-engine/actor-priority-in-unreal-engine and the ReplicationGraph API pages). | Outside the allow list. |

## Verifier spot-check

Pending: the verifier's table goes here.

## Sources

### Sources read

Every source the five units read, merged and de-duplicated: 113 rows from 131 unit entries (some rows cover several pages of one source, as the unit listed them). All were read on 2026-10-27. The "Units" column gives each unit's entry number (interest: its S id) and the tier that unit gave the source.

#### Specifications and standards

| # | Source | Title | Version or page date | Read | Units (tier) |
|---|---|---|---|---|---|
| 1 | https://www.rfc-editor.org/rfc/rfc9221.html | RFC 9221: An Unreliable Datagram Extension to QUIC | March 2022, Standards Track | 2026-10-27 | sync 1 (official); budgets (official); transport 8 (official) |
| 2 | https://www.rfc-editor.org/rfc/rfc9000.html | RFC 9000, QUIC: A UDP-Based Multiplexed and Secure Transport | May 2021 | 2026-10-27 | budgets (official); transport 9 (official) |
| 3 | https://www.rfc-editor.org/rfc/rfc9001.html | RFC 9001, Using TLS to Secure QUIC | May 2021 | 2026-10-27 | budgets (official) |
| 4 | https://www.rfc-editor.org/rfc/rfc9002.html | RFC 9002, QUIC Loss Detection and Congestion Control | May 2021 | 2026-10-27 | budgets (official); transport 10 (official) |
| 5 | https://www.rfc-editor.org/rfc/rfc9297.html | RFC 9297, HTTP Datagrams and the Capsule Protocol | August 2022 | 2026-10-27 | budgets (official) |
| 6 | https://datatracker.ietf.org/doc/draft-ietf-webtrans-http3/ | WebTransport over HTTP/3 (document page; the transport unit also read /history/) | draft-ietf-webtrans-http3-16, 2026-07-06 (Internet-Draft) | 2026-10-27 | budgets (official); transport 2 (official); rooms 17 (official) |
| 7 | https://w3c.github.io/webtransport/ | WebTransport (W3C Editor's Draft) | 2026-10-27 | 2026-10-27 | budgets (official) |
| 8 | https://www.rfc-editor.org/rfc/rfc6455.html | RFC 6455, The WebSocket Protocol | December 2011 | 2026-10-27 | budgets (official) |
| 9 | https://www.rfc-editor.org/rfc/rfc8446.html | RFC 8446, TLS 1.3 | August 2018 | 2026-10-27 | budgets (official) |
| 10 | https://www.w3.org/TR/webtransport/ | WebTransport (W3C) | Candidate Recommendation Snapshot, 30 July 2026 (four fetches: status, §4.3, §5.1, §6.5, §6.9, §14, references) | 2026-10-27 | transport 1 (official) |
| 11 | https://datatracker.ietf.org/doc/html/draft-ietf-webtrans-http3 | WebTransport over HTTP/3 (text of -16) | draft -16, 2026-07-06 | 2026-10-27 | transport 3 (official) |
| 12 | https://datatracker.ietf.org/doc/html/draft-ietf-webtrans-http3-02 | WebTransport over HTTP/3, draft -02 | 2021-10-25 | 2026-10-27 | transport 4 (official) |
| 13 | https://datatracker.ietf.org/doc/html/draft-ietf-webtrans-http3-07 | WebTransport over HTTP/3, draft -07 | 2023-06-13 | 2026-10-27 | transport 5 (official) |
| 14 | https://datatracker.ietf.org/doc/draft-ietf-webtrans-http2/ | WebTransport over HTTP/2 | draft-ietf-webtrans-http2-15, 2026-07-06 | 2026-10-27 | transport 6 (official) |
| 15 | https://datatracker.ietf.org/doc/draft-ietf-quic-qmux/ | QMux | draft-ietf-quic-qmux-02, 2026-07-06 | 2026-10-27 | transport 7 (official) |
| 16 | https://www.rfc-editor.org/rfc/rfc7519.html | RFC 7519, JSON Web Token (JWT) | May 2015 | 2026-10-27 | rooms 13 (official) |

#### Browser and web documentation, data and code

| # | Source | Title | Version or page date | Read | Units (tier) |
|---|---|---|---|---|---|
| 17 | https://raw.githubusercontent.com/mrdoob/three.js/dev/src/cameras/PerspectiveCamera.js | three.js, `src/cameras/PerspectiveCamera.js` | dev branch; no revision shown in the file | 2026-10-27 | interest S6 (official) |
| 18 | https://developer.chrome.com/docs/devtools/memory-inspector | Chrome DevTools, "Inspect JavaScript ArrayBuffer with the Memory inspector" | page updated 2024-09-06 | 2026-10-27 | interest S11 (official) |
| 19 | https://developer.mozilla.org/en-US/docs/Web/API/WebTransport_API | WebTransport API (MDN) | last modified 2026-10-21 | 2026-10-27 | transport 11 (official) |
| 20 | https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/WebTransport | WebTransport() constructor (MDN) | last modified 2026-10-12 | 2026-10-27 | transport 12 (official); rooms 16 (official) |
| 21 | https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/WebTransport.json | MDN browser-compat-data: api/WebTransport.json | main | 2026-10-27 | transport 13 (official) |
| 22 | https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/WebTransportDatagramDuplexStream.json | MDN browser-compat-data: api/WebTransportDatagramDuplexStream.json | main | 2026-10-27 | transport 14 (official) |
| 23 | https://raw.githubusercontent.com/mdn/browser-compat-data/main/api/WebSocket.json | MDN browser-compat-data: api/WebSocket.json | main | 2026-10-27 | transport 15 (official) |
| 24 | https://raw.githubusercontent.com/mdn/browser-compat-data/main/browsers/chrome.json | MDN browser-compat-data: browsers/chrome.json, firefox.json and safari.json (release dates) | main | 2026-10-27 | transport 16 (official) |
| 25 | https://webkit.org/blog/17862/ | WebKit Features for Safari 26.4 | 2026-03-24 | 2026-10-27 | transport 17 (official) |
| 26 | https://developer.chrome.com/docs/capabilities/web-apis/webtransport | Using WebTransport (Chrome for Developers) | page dated 2020-06-08 (old; read but not cited for any finding) | 2026-10-27 | transport 18 (official) |
| 27 | https://raw.githubusercontent.com/google/quiche/main/quiche/quic/core/http/http_constants.h | QUICHE: quiche/quic/core/http/http_constants.h | main | 2026-10-27 | transport 19 (official) |
| 28 | https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket | WebSocket() constructor (MDN) | modified 2026-06-22 | 2026-10-27 | rooms 15 (official) |

#### Rust crates and documentation

| # | Source | Title | Version or page date | Read | Units (tier) |
|---|---|---|---|---|---|
| 29 | https://crates.io/api/v1/crates/wtransport | crates.io API: wtransport | 0.7.2 | 2026-10-27 | transport 20 (official) |
| 30 | https://github.com/BiagioFesta/wtransport | wtransport repository page (GitHub) | repository page | 2026-10-27 | transport 21 (other) |
| 31 | https://raw.githubusercontent.com/BiagioFesta/wtransport/master/README.md | wtransport README | master | 2026-10-27 | transport 22 (other) |
| 32 | https://raw.githubusercontent.com/BiagioFesta/wtransport/master/wtransport/Cargo.toml | wtransport Cargo.toml | 0.7.2 | 2026-10-27 | transport 23 (official) |
| 33 | https://raw.githubusercontent.com/BiagioFesta/wtransport/master/wtransport-proto/src/settings.rs | wtransport-proto: src/settings.rs | master | 2026-10-27 | transport 24 (official) |
| 34 | https://docs.rs/wtransport/latest/wtransport/ | docs.rs: wtransport (crate root) | 0.7.2, page 2026-10-15 | 2026-10-27 | transport 25 (official) |
| 35 | https://docs.rs/wtransport/latest/wtransport/struct.Identity.html | docs.rs: wtransport::Identity | 0.7.2 | 2026-10-27 | transport 26 (official) |
| 36 | https://docs.rs/wtransport/latest/wtransport/struct.Connection.html | docs.rs: wtransport::Connection | 0.7.2 | 2026-10-27 | transport 27 (official) |
| 37 | https://crates.io/api/v1/crates/quinn | crates.io API: quinn | 0.11.12 | 2026-10-27 | transport 28 (official) |
| 38 | https://docs.rs/quinn/latest/quinn/congestion/index.html | docs.rs: quinn::congestion | 0.11.12 | 2026-10-27 | transport 29 (official) |
| 39 | https://docs.rs/quinn/latest/quinn/struct.Connection.html | docs.rs: quinn::Connection | 0.11.12 | 2026-10-27 | transport 30 (official) |
| 40 | https://docs.rs/quinn/latest/quinn/struct.TransportConfig.html | docs.rs: quinn::TransportConfig | 0.11.12 | 2026-10-27 | transport 31 (official) |
| 41 | https://crates.io/api/v1/crates/web-transport-quinn | crates.io API: web-transport-quinn | 0.13.5 | 2026-10-27 | transport 32 (official) |
| 42 | https://github.com/moq-dev/web-transport | web-transport repository page (moq-dev, GitHub) | repository page | 2026-10-27 | transport 33 (other) |
| 43 | https://raw.githubusercontent.com/moq-dev/web-transport/main/README.md | web-transport README (moq-dev) | main | 2026-10-27 | transport 34 (other) |
| 44 | https://docs.rs/web-transport-proto/latest/web_transport_proto/ | docs.rs: web-transport-proto (crate root, struct.Setting.html and src/web_transport_proto/settings.rs.html) | 0.6.3 | 2026-10-27 | transport 35 (official) |
| 45 | https://crates.io/api/v1/crates/h3-webtransport | crates.io API: h3-webtransport | 0.1.2 | 2026-10-27 | transport 36 (official) |
| 46 | https://github.com/hyperium/h3 | h3 repository page (hyperium, GitHub) | repository page | 2026-10-27 | transport 37 (other) |
| 47 | https://crates.io/api/v1/crates/tokio-tungstenite | crates.io API: tokio-tungstenite | 0.30.0 | 2026-10-27 | transport 38 (official) |
| 48 | https://crates.io/api/v1/crates/axum | crates.io API: axum | 0.8.9 | 2026-10-27 | transport 39 (official) |
| 49 | https://docs.rs/axum/latest/axum/extract/ws/index.html | docs.rs: axum::extract::ws | 0.8.9 | 2026-10-27 | transport 40 (official) |
| 50 | https://docs.rs/tokio/latest/tokio/task/index.html | docs.rs: Module tokio::task | tokio 1.54.1 | 2026-10-27 | rooms 7 (official) |
| 51 | https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html | docs.rs: tokio::task::spawn_blocking | tokio 1.54.1 | 2026-10-27 | rooms 8 (official) |
| 52 | https://doc.rust-lang.org/std/thread/index.html | Module std::thread (Rust standard library) | Rust 1.99.1, 2026-10-13 | 2026-10-27 | rooms 9 (official) |
| 53 | https://doc.rust-lang.org/cargo/reference/profiles.html | Profiles, The Cargo Book | no version on the section | 2026-10-27 | rooms 10 (official) |

#### Hosting, identity and security guidance

| # | Source | Title | Version or page date | Read | Units (tier) |
|---|---|---|---|---|---|
| 54 | https://docs.aws.amazon.com/gameliftservers/latest/developerguide/gamelift-sdk-interactions.html | Game client/server interactions with Amazon GameLift Servers | no page date | 2026-10-27 | rooms 1 (official) |
| 55 | https://docs.aws.amazon.com/gameliftservers/latest/apireference/API_PlayerSession.html | PlayerSession (Amazon GameLift Servers API reference) | API gamelift-2015-10-01 | 2026-10-27 | rooms 2 (official) |
| 56 | https://docs.aws.amazon.com/gameliftservers/latest/apireference/API_RuntimeConfiguration.html | RuntimeConfiguration (Amazon GameLift Servers API reference) | API gamelift-2015-10-01 | 2026-10-27 | rooms 3 (official) |
| 57 | https://docs.aws.amazon.com/gameliftservers/latest/developerguide/fleets-multiprocess.html | Optimize game server runtime configuration on managed Amazon GameLift Servers | no page date | 2026-10-27 | rooms 4 (official) |
| 58 | https://learn.microsoft.com/en-us/gaming/playfab/multiplayer/servers/multiplayer-game-server-lifecycle | Lifecycle of a multiplayer server - PlayFab | 2022-01-26, updated 2025-05-01 | 2026-10-27 | rooms 5 (official) |
| 59 | https://raw.githubusercontent.com/googleforgames/agones/main/site/content/en/docs/Integration%20Patterns/high-density-gameservers.md | Agones: High Density GameServers | main; front matter 2021-08-31 | 2026-10-27 | rooms 6 (official) |
| 60 | https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html | OWASP Session Management Cheat Sheet | no page date | 2026-10-27 | rooms 18 (official) |
| 61 | https://learn.microsoft.com/en-us/xbox/playfab/identity/player-identity/login/login-basics-best-practices | Login basics and best Practices - PlayFab | 2025-02-20, updated 2025-10-31 | 2026-10-27 | rooms 19 (official) |
| 62 | https://learn.microsoft.com/en-us/gaming/playfab/features/multiplayer/lobby/ownership-changes | Lobby ownership changes - PlayFab | 2023-02-28, updated 2025-09-10 | 2026-10-27 | rooms 20 (official) |
| 63 | https://learn.microsoft.com/en-us/gaming/playfab/features/multiplayer/lobby/create-a-lobby | Create a lobby - PlayFab | 2021-10-25, updated 2026-03-24 | 2026-10-27 | rooms 21 (official) |

#### Shipped-game source code

| # | Source | Title | Version or page date | Read | Units (tier) |
|---|---|---|---|---|---|
| 64 | https://github.com/id-Software/Quake-III-Arena | Quake III Arena GPL Source Release (+ README.txt) | one commit on master; no version or date shown; GPL-2.0 | 2026-10-27 | sync 2 (shipped game) |
| 65 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/server/sv_snapshot.c | Quake III server snapshots | GPL source release, master (no version shown) | 2026-10-27 | sync 3 (shipped game); interest S1 (shipped game) |
| 66 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/qcommon/qcommon.h | Quake III common header | GPL source release, master (no version shown) | 2026-10-27 | sync 4 (shipped game); budgets (shipped game) |
| 67 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/client/cl_input.c | Quake III client input and command packets | GPL source release, master (no version shown) | 2026-10-27 | sync 5 (shipped game) |
| 68 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/client/cl_main.c | Quake III client main (cvar defaults) | GPL source release, master (no version shown) | 2026-10-27 | sync 6 (shipped game); budgets (shipped game) |
| 69 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/client/cl_cgame.c | Quake III client time and snapshot clock | GPL source release, master (no version shown) | 2026-10-27 | sync 7 (shipped game) |
| 70 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/cgame/cg_predict.c | Quake III client prediction | GPL source release, master (no version shown) | 2026-10-27 | sync 8 (shipped game) |
| 71 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/cgame/cg_main.c | Quake III cgame cvar defaults | GPL source release, master (no version shown) | 2026-10-27 | sync 9 (shipped game) |
| 72 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/cgame/cg_snapshot.c | Quake III snapshot transitions | GPL source release, master (no version shown) | 2026-10-27 | sync 10 (shipped game) |
| 73 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/cgame/cg_ents.c | Quake III entity interpolation | GPL source release, master (no version shown) | 2026-10-27 | sync 11 (shipped game) |
| 74 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/g_weapon.c | Quake III weapon fire | GPL source release, master (no version shown) | 2026-10-27 | sync 12 (shipped game) |
| 75 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/g_missile.c | Quake III missiles | GPL source release, master (no version shown) | 2026-10-27 | sync 13 (shipped game) |
| 76 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/bg_misc.c | Quake III shared trajectory code | GPL source release, master (no version shown) | 2026-10-27 | sync 14 (shipped game) |
| 77 | https://github.com/ValveSoftware/source-sdk-2013 | Source SDK 2013 repository (layout) | master; no version or commit date shown | 2026-10-27 | sync 15 (shipped game) |
| 78 | https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/player_lagcompensation.cpp | Source lag compensation | master (no version shown) | 2026-10-27 | sync 16 (shipped game); budgets (shipped game) |
| 79 | https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/player.cpp | Source player (WantsLagCompensationOnEntity, command limits) | master (no version shown); first 100,000 of 265,981 characters read | 2026-10-27 | sync 17 (shipped game) |
| 80 | https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/public/inetchannelinfo.h | Source net channel info (GetLatency) | master (no version shown) | 2026-10-27 | sync 18 (shipped game) |
| 81 | https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/client/c_baseentity.cpp | Source client entity (interpolation, extrapolation cvars) | master (no version shown); read in two parts (0–100,000 and 100,000–186,120) | 2026-10-27 | sync 19 (shipped game); budgets (shipped game) |
| 82 | https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/shared/usercmd.h | Source user command | master (no version shown) | 2026-10-27 | sync 20 (shipped game) |
| 83 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/q_shared.h | Quake III shared header (ANGLE2SHORT, usercmd_t, trajectory_t) | GPL source release, master (no version shown) | 2026-10-27 | sync 28 (shipped game) |
| 84 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/qcommon/net_chan.c | Quake III Arena source: net_chan.c | master | 2026-10-27 | budgets (shipped game) |
| 85 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/qcommon/msg.c | Quake III Arena source: msg.c | master | 2026-10-27 | budgets (shipped game) |
| 86 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/server/sv_init.c | Quake III Arena source: sv_init.c | master | 2026-10-27 | budgets (shipped game) |
| 87 | https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/public/const.h | Source SDK 2013: const.h | master | 2026-10-27 | budgets (shipped game) |
| 88 | https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/gameinterface.cpp | Source SDK 2013: gameinterface.cpp | master | 2026-10-27 | budgets (shipped game) |
| 89 | https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/shared/shareddefs.h | Source SDK 2013: shareddefs.h (read; no tick default in it) | master | 2026-10-27 | budgets (shipped game) |
| 90 | https://raw.githubusercontent.com/ValveSoftware/GameNetworkingSockets/master/include/steam/steamnetworkingtypes.h | GameNetworkingSockets: steamnetworkingtypes.h | master | 2026-10-27 | budgets (shipped game) |
| 91 | https://raw.githubusercontent.com/ValveSoftware/GameNetworkingSockets/master/src/steamnetworkingsockets/clientlib/csteamnetworkingsockets.cpp | GameNetworkingSockets: csteamnetworkingsockets.cpp | master | 2026-10-27 | budgets (shipped game) |
| 92 | https://raw.githubusercontent.com/ValveSoftware/GameNetworkingSockets/master/src/steamnetworkingsockets/steamnetworkingsockets_internal.h | GameNetworkingSockets: steamnetworkingsockets_internal.h | master | 2026-10-27 | budgets (shipped game) |
| 93 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/qcommon/cm_test.c | Quake III Arena source, `code/qcommon/cm_test.c` | id Software GPL release; master | 2026-10-27 | interest S2 (shipped game) |
| 94 | https://raw.githubusercontent.com/id-Software/Quake-2/master/server/sv_send.c | Quake II source, `server/sv_send.c` | GPL, header "Copyright (C) 1997-2001 Id Software"; master | 2026-10-27 | interest S3 (shipped game) |
| 95 | https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/baseentity.cpp | Source SDK 2013, `src/game/server/baseentity.cpp` | "Copyright Valve Corporation"; master; characters 0–200,000 of 273,620 read | 2026-10-27 | interest S7 (shipped game) |
| 96 | https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/team.cpp | Source SDK 2013, `src/game/server/team.cpp` | master | 2026-10-27 | interest S8 (shipped game) |
| 97 | https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/shared/tf/tf_player_shared.cpp | Source SDK 2013 (TF2 code), `src/game/shared/tf/tf_player_shared.cpp` | master; characters 0–100,000 of 451,076 read | 2026-10-27 | interest S9 (shipped game) |
| 98 | https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/sendproxy.cpp | Source SDK 2013, `src/game/server/sendproxy.cpp` | master | 2026-10-27 | interest S10 (shipped game) |
| 99 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/g_utils.c | Quake III Arena source, `code/game/g_utils.c` | GPL release; master | 2026-10-27 | interest S12 (shipped game) |
| 100 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/g_combat.c | Quake III Arena source, `code/game/g_combat.c` | GPL release; master | 2026-10-27 | interest S13 (shipped game) |
| 101 | https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/hltvdirector.cpp | Source SDK 2013, `src/game/server/hltvdirector.cpp` | master | 2026-10-27 | interest S14 (shipped game) |
| 102 | https://raw.githubusercontent.com/mas-bandwidth/netcode/main/STANDARD.md | netcode 1.02 standard (mas-bandwidth/netcode, STANDARD.md) | main | 2026-10-27 | rooms 12 (shipped game) |
| 103 | https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/tf/bot/tf_bot_manager.cpp | Source SDK 2013 (Team Fortress 2): tf_bot_manager.cpp | master | 2026-10-27 | rooms 14 (shipped game) |

#### Practitioner articles (Glenn Fiedler)

| # | Source | Title | Version or page date | Read | Units (tier) |
|---|---|---|---|---|---|
| 104 | https://gafferongames.com/post/what_every_programmer_needs_to_know_about_game_networking/ | What Every Programmer Needs To Know About Game Networking (Glenn Fiedler) | 2010-02-24 | 2026-10-27 | sync 21 (other) |
| 105 | https://gafferongames.com/post/introduction_to_networked_physics/ | Introduction to Networked Physics (Fiedler) | 2014-11-28 (read; no claim cited) | 2026-10-27 | sync 22 (other) |
| 106 | https://gafferongames.com/post/deterministic_lockstep/ | Deterministic Lockstep (Fiedler) | 2014-11-29 | 2026-10-27 | sync 23 (other) |
| 107 | https://gafferongames.com/post/snapshot_interpolation/ | Snapshot Interpolation (Fiedler) | 2014-11-30 | 2026-10-27 | sync 24 (other); budgets (other) |
| 108 | https://gafferongames.com/post/snapshot_compression/ | Snapshot Compression (Fiedler) | 2015-01-04 | 2026-10-27 | sync 25 (other); budgets (other); interest S5 (other) |
| 109 | https://gafferongames.com/post/state_synchronization/ | State Synchronization (Fiedler) | 2015-01-05 | 2026-10-27 | sync 26 (other); budgets (other); interest S4 (other) |
| 110 | https://gafferongames.com/post/networked_physics_2004/ | Networked Physics (2004) (Fiedler) | 2004-09-04 | 2026-10-27 | sync 27 (other) |
| 111 | https://gafferongames.com/post/packet_fragmentation_and_reassembly/ | Packet Fragmentation and Reassembly (Glenn Fiedler) | 2016-09-06 | 2026-10-27 | budgets (other); transport 42 (shipped-game practitioner) |
| 112 | https://gafferongames.com/post/udp_vs_tcp/ | UDP vs. TCP (Glenn Fiedler) | 2008-10-01 | 2026-10-27 | transport 41 (shipped-game practitioner) |
| 113 | https://gafferongames.com/post/why_cant_i_send_udp_packets_from_a_browser/ | Why can't I send UDP packets from a browser? (Glenn Fiedler) | 2017-02-26 | 2026-10-27 | rooms 11 (shipped game) |

Rows by tier, as the units gave them: official 58 · shipped game 41 · other 12 · split: other / shipped-game practitioner 1 · shipped-game practitioner 1.

Legacy inputs (repository, read 2026-10-27, as the units list them): rows of `docs/LEGACY_INVENTORY.md` and tables of `docs/LEGACY_TUNING.md` named in each unit's "Legacy facts used" table; the interest unit also cites `shamble-game.js:2191-2200` and `:2819` (the camera boom), and the budgets unit `shamble-game.js:2774-2775`.

### Sources not read

Merged from the five units' lists (26 rows). The reasons are the units' own.

| # | Source | Reason | Units |
|---|---|---|---|
| 1 | https://developer.valvesoftware.com/wiki/Source_Multiplayer_Networking (and the `action=raw` form) | HTTP 403: the domain is on the allow list, but the site refused the fetch | sync, budgets, interest, transport |
| 2 | https://developer.valvesoftware.com/wiki/Lag_compensation and https://developer.valvesoftware.com/wiki/Lag_Compensation | HTTP 403 (as row 1) | sync |
| 3 | https://developer.valvesoftware.com/wiki/Interpolation | HTTP 403 (as row 1) | sync |
| 4 | https://developer.valvesoftware.com/wiki/Latency_Compensating_Methods_in_Client/Server_In-game_Protocol_Design_and_Optimization (Yahn Bernier, "Latency Compensating Methods in Client/Server In-game Protocol Design and Optimization") | HTTP 403 (budgets); not attempted after four 403s from the same site (sync) | budgets, sync |
| 5 | https://developer.valvesoftware.com/wiki/Networking_Entities | HTTP 403 (as row 1) | interest |
| 6 | https://developer.valvesoftware.com/wiki/TFBot and https://developer.valvesoftware.com/wiki/Tf_bot_reevaluate_class_in_spawnroom | HTTP 403; the TF2 source code was read instead | rooms |
| 7 | https://gdcvault.com/play/1024001/-Overwatch-Gameplay-Architecture-and (Timothy Ford, GDC 2017; also the www. form) | Video only; the vault redirected to https://gdconf.com/gdc-vault-maintenance/, which is outside the allow list and was not followed (the interest unit did not attempt it) | sync, budgets, interest |
| 8 | Other Overwatch GDC 2017 talks on the same vault (Dan Reed on networking scripted weapons and abilities; Philip Orwig on replay technology) | Video only (same vault) | sync |
| 9 | https://www.gdcvault.com/play/1014345/I-Shot-You-First-Networking (Aldridge, Bungie, GDC 2011, Halo: Reach) | Redirects to https://gdconf.com/gdc-vault-maintenance/, outside the allow list; the talk is video only | interest |
| 10 | https://edgegap.com/blog/game-backend-deep-dive-overwatch-2016-netcode-architecture-rollback (and other edgegap.com write-ups of the Overwatch talk) | Outside the allow list (search lead only) | budgets, sync |
| 11 | sohu.com, juejin.cn and gamedev.net write-ups of the Overwatch talk (seen in search results; no URL kept) | Outside the allow list | sync |
| 12 | https://www.pcgamesn.com/overwatch/overwatch-netcode | Outside the allow list (search lead only) | budgets |
| 13 | https://gamesbeat.com/overwatch-netcode-tick-rate-60hz/ | Outside the allow list (search lead only) | budgets |
| 14 | fabiensanglard.net write-ups of the Quake 3 network model | Outside the allow list; the released source was read instead | sync |
| 15 | https://technology.riotgames.com/node/108 ("Demolishing Wallhacks with VALORANT's Fog of War") | Outside the allow list | interest |
| 16 | https://www.riotgames.com/en/news/demolishing-wallhacks-valorants-fog-war | Outside the allow list | interest |
| 17 | https://dev.epicgames.com/documentation/unreal-engine/actor-priority-in-unreal-engine | Outside the allow list | interest |
| 18 | https://dev.epicgames.com/documentation/unreal-engine/API/Plugins/ReplicationGraph/UBasicReplicationGraph | Outside the allow list | interest |
| 19 | https://docs.unrealengine.com/4.27/en-US/API/Plugins/ReplicationGraph/UBasicReplicationGraph/index.html | Outside the allow list | interest |
| 20 | https://cedric-neukirchen.net/docs/multiplayer-compendium/actor-relevancy-and-priority | Outside the allow list (third-party) | interest |
| 21 | https://threejs.org/docs/#api/en/cameras/PerspectiveCamera | Fetched, but the hash-routed page returned only the navigation index; the source file (interest S6) was read instead | interest |
| 22 | The press pages in the search results about Safari 26.4 (9to5mac.com, digitaltoday.co.kr, iphonesoft.fr and others) | Outside the allow list; the official WebKit post was read instead | transport |
| 23 | webtransport-ponyfill-websocket (named on the old Chrome page; no URL followed) | Not read; no recommendation rests on it | transport |
| 24 | https://1-58-0.agones.dev/site/docs/integration-patterns/high-density-gameservers/ (and other agones.dev versions) | Outside the allow list; the same page was read from GitHub | rooms |
| 25 | https://docs.amazonaws.cn/en_us/gamelift/latest/developerguide/integration-server-sdk-unreal-ref-actions.html and other docs.amazonaws.cn pages | Outside the allow list; docs.aws.amazon.com was read instead | rooms |
| 26 | https://learn.microsoft.com/en-us/xbox/playfab/multiplayer/servers/billing-for-thunderhead | Not fetched: fleet cost is R6's (search lead only) | rooms |
