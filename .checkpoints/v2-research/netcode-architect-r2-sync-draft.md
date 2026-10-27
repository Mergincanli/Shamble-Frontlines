# R2 unit sync: snapshots, prediction, interpolation and lag compensation for 48+

Brief question (quoted): "Snapshots, client prediction, interpolation and lag compensation for 48+ players; […] learn
from shipped games (Valve's Source networking, Overwatch's GDC talks, Glenn Fiedler's articles, Quake 3)"
(`docs/V2_PHASE0_BRIEF.md` §4, row R2; the row's other questions belong to the other R2 units).

Author: netcode-architect (Opus 5.5, effort xhigh), 2026-10-27. Status: unit draft for the R2 assembler; not verified.
Notes with every source summary and its exact quotes: `.checkpoints/v2-research/netcode-architect-r2-sync.md`.

**How to read the certainty marks.** "Confirmed in the source" means the source's own words (quoted in the finding)
say it. "Inferred" means my reading, my arithmetic, or a point I could only see in the fetch tool's paraphrase.
**Source tiers in this unit.** No official document or specification describes game-state synchronisation; the one
official source here is RFC 9221 (QUIC datagrams). The shipped-game tier is the released source code of two shipped
games: Quake III Arena (id Software's GPL release) and the Source SDK 2013 (Valve), read on github.com and
raw.githubusercontent.com. I read the code because the write-ups of Quake 3's model are off the allow list and Valve's
wiki pages refused the fetch (HTTP 403). Glenn Fiedler's articles are tier "other" (practitioner articles the brief
names). The Overwatch GDC talks are video only and the GDC Vault redirected to an off-list page: not read, and no
recommendation rests on them.

---

## 1. Findings

### State-sync model

**R2-SYNC-F1.** In peer-to-peer deterministic lockstep, every player gets the latency of the most-lagged player.
· Source: https://gafferongames.com/post/what_every_programmer_needs_to_know_about_game_networking/ (section "Peer-to-Peer
Lockstep"; page dated 2010-02-24) · Tier: other · Read 2026-10-27 · Confirmed in the source: "This means that each player
in the game has latency equal to the most lagged player."

**R2-SYNC-F2.** Joining a lockstep game in progress is rare, because it needs a complete deterministic starting state.
· Source: same page, same section · Tier: other · Read 2026-10-27 · Confirmed in the source: "this is not common due to
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
cost is time spent hunting divergence and pops. It works only if both sides quantise the state the same way.
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

### Client prediction and reconciliation

**R2-SYNC-F12.** Client-side prediction keeps a circular buffer of the local player's past states and inputs. When a
correction arrives, the client rewinds to it and replays the newer inputs. What the player's own input did not cause
(being hit by a rocket) cannot be predicted.
· Source: https://gafferongames.com/post/what_every_programmer_needs_to_know_about_game_networking/ (section
"Client-Side Prediction") · Tier: other · Read 2026-10-27 · Confirmed in the source: "keep a circular buffer of past
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

**R2-SYNC-F17.** Quake III also predicts item pickups and jump pads, with item prediction on by default. Other entities
are not simulated on the client: they come from snapshots.
· Source: cg_predict.c (CG_TouchItem, CG_TouchTriggerPrediction) and cg_main.c · Tier: shipped game · Read 2026-10-27 ·
Default confirmed in the source (`{ &cg_predictItems, "cg_predictItems", "1", CVAR_ARCHIVE },`). The rest is inferred:
I saw it only in the fetch tool's paraphrase.

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

### Entity interpolation and clock

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
the previous and the newest snapshot (so under one snapshot interval of delay). It resets the clock when the error
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

### Lag compensation

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

### Projectiles

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

---

## 2. Legacy facts used

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

---

## 3. Options

### 3.1 State-sync model

| Option | How it works | Fit and costs | Risk |
|---|---|---|---|
| A. Deterministic lockstep (inputs only) | Every peer runs the whole sim from everyone's inputs. | Lowest bandwidth. But latency is the worst player's (F1), it stalls on a missing input (F4), late join is hard (F2), and it needs bit-exact results on every client (F3). §6.1 rooms and late joining suffer; 48 browsers must all agree. | High: one divergent client breaks the match; a wallhack sees everything. |
| B. Predict-everything with rollback (inputs, the server re-simulates) | Clients run the full sim ahead and roll back on corrections. | I read no source on this in this run (the Overwatch talk, a likely example, is video only). The client cost grows with the entity count at 48+. | Unassessed here. |
| C. State synchronisation (input + state, both sides simulate) | Fiedler's priority accumulator plus extrapolation. | Low bandwidth, but "approximate and lossy" with divergence and pops (F6). Built for physics objects, not for authority over many players. | Medium. |
| D. Full snapshots, no delta | The server sends the whole visible state every snapshot. | Simplest; robust to loss. Bandwidth scales with entities × rate (bandwidth unit). | Bandwidth at 48+. |
| E. Delta snapshots against each client's acknowledged snapshot | Quake III's shipped model (F8, F9) and Fiedler's (F7). Interest management picks the entities. | Tolerates loss and reordering (needed by F11). Late join and rejoin start from the per-entity baselines (fits §6.1 rooms). No client determinism is needed for correctness (F5). The server encodes per client. | Low; well proven. Spikes when a client's baseline expires (a full or baseline-relative snapshot). |

### 3.2 Client prediction scope

| Option | What the client predicts | Fit |
|---|---|---|
| A. None | Nothing: every input waits a round trip. | Fails "fair up to about 100 ms" (§4) and LOOP_V2 §7 "the shot feels instant". |
| B. Own movement only | Quake III and QuakeWorld style (F12, F13). | Proven; cheap; corrections are rare when the code matches. |
| C. Own movement, own weapon state, cosmetic shot feedback, own vehicle | Like B, plus ammo, cooldowns, reload and active reload, plus muzzle flash and tracer from the shared spread seed (F20). The server confirms hits (LOOP_V2 §7 split feedback). Plus predictable pickups and touches as in Quake III (F17). | Serves LOOP_V2 §7 directly. The weapon state already lives in the sim (GC4). |
| D. Abilities and projectiles too | Molotov throw, Riot Shield raise, Banzai, bow bolt, all predicted and rolled back if wrong. | Better feel for abilities. Needs cosmetic rollback of effects; I read no source on it in this run. |

### 3.3 Correction smoothing

| Option | Behaviour | Source |
|---|---|---|
| A. Snap | Show the corrected state at once. | (baseline) |
| B. Linear error decay | Add the error to a visual offset that fades to zero over a fixed time (Quake III: 100 ms). | F15 |
| C. Exponential pull | Move 10 % of the way per update; snap above 2 m; snap velocity. | F16 |

### 3.4 Interpolation delay and clock

| Option | Delay | Source |
|---|---|---|
| A. Under one snapshot interval, then extrapolate (Quake III) | The clock approaches the newest snapshot. The delay is small, but every late or lost snapshot means extrapolating. | F23 |
| B. A fixed number of snapshot intervals | For example 2–3 intervals (the netcode default principles' starting point, about 100 ms). | F21 (the rule); Source's own 100 ms default not read (F26) |
| C. Adaptive | Start at 3 intervals; widen with measured jitter and loss; narrow when the link is clean. The clock drifts slowly as in Quake III. | F21, F23 |

### 3.5 Lag compensation for hitscan

| Option | Behaviour | Fit |
|---|---|---|
| A. None (Quake III) | Shooters lead by RTT + interpolation (F27, F34). | At 100 ms and 9 m/s that is about 1.8 m of lead: unfair for §4. |
| B. Server rewind to the shooter's view time, capped (Source) | History of hit volumes; the shot resolves against the targets as the shooter saw them (F28–F32). | Fair for the shooter up to the cap; the victim's "hit behind cover" window grows with it (F34). |
| C. Client-side hit detection, server trusts it | The client says what it hit. | An authority leak (trivially cheatable); rejected by the default principles ("never client state"). |

### 3.6 Projectiles (frags, Molotovs, bolts, later Javelin rockets)

| Option | Behaviour | Fit |
|---|---|---|
| A. Server only, shown when the server's spawn arrives | Quake III, with a fixed 50 ms prestep (F35). | Simple; the owner sees their throw one round trip late. |
| B. Server-simulated; the owner shows a cosmetic copy at once and swaps it for the server's | The owner sees it instantly; others see the server's. | Serves LOOP_V2 §7. Needs a match rule (owner + command sequence) and a hand-over without a pop. |
| C. Client-authoritative projectiles | The client says where it landed. | Authority leak: rejected. |

Within A and B, how far the server moves the new projectile forward on spawn: none, a fixed prestep (Quake III: 50 ms),
or the shooter's latency up to the cap. That is open (OQ6).

### 3.7 Aim encoding (aiming at height, §6.2)

| Option | Bits per command (proposed precision) | Fit |
|---|---|---|
| A. Direction: yaw + pitch, 16 bits each (as Quake III's angles, F19) | 32 | Compact. A small error in the predicted fire origin turns into a miss at long range (AMK 300 m). |
| B. World aim point, quantised (positions to 1 cm as in the default principles) | About 60–75, depending on map bounds (the bandwidth unit sizes it) | Keeps the point the player clicked at any height (GC3), whatever the origin error. The server rebuilds the ray from its own fire origin to that point. Easy to check (range, LOS). |

---

## 4. Recommendations

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
  encodes against the per-entity defaults instead of the whole raw state, as Quake III and Fiedler do. The ring length
  is data: proposed 32 snapshots, as Quake III.
- Snapshots go as unreliable datagrams, each sized to fit one QUIC packet (F11). Reliable events go on streams
  (transport unit).
- Reason: it is the only option that tolerates loss and reordering without stalls, needs no bit-exact agreement between
  48 browsers for correctness, supports joining and rejoining mid-match (§6.1 rooms and private matches; humans taking
  over bot slots), and keeps the server the only authority. A shipped game (Quake III) and Fiedler both document it.
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
  1. Under P3 with 5 % random loss, at least 99.9 % of commands reach the server before their tick runs.
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
  1. Under P0–P4 on a scripted course with no other players: 0 corrections larger than 1 mm.
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
  - Its length is the cap plus a margin. The default principles' ~1 s, also Source's default (F28), costs about 0.1 MB at 60 Hz for
    60 movers (inferred), but nothing in §6 uses more than the cap.
- The shot. The server builds the ray from its own authoritative fire origin at the command's tick (the
  `playerFireOrigin` rule, GC5) toward the command's aim (REC8). It resolves the ray against the movers' recorded state
  at the view time, and against the static world at present. Movers are not moved back and restored: `resolveShot`
  already is a pure query built for this (CS7).
- View time. The command's view time is accepted if it lies within 200 ms of the server's estimate (RTT plus that
  client's interpolation delay); otherwise the estimate is used (F28).
- The cap. The rewind is capped at a data value, proposed 250 ms. Reading "about 100 ms of ping" as RTT, a 100 ms player
  with a 100–150 ms delay needs 200–250 ms (F28's formula; inferred). The default principles' ~200 ms covers only the low end.
  Beyond the cap, shooters lead by the excess.
- Filters (Source's, F32):
  - targets the shooter could hit (team filter, friendly fire from mode data);
  - targets sent to and acknowledged by the shooter (interest management);
  - targets within reach of the ray.
  - A target that died or teleported between the view time and now stops at that record (F30).
- Bots. Shots by bots use present state, with no rewind (F31).
- Feedback. Hit confirmation goes back as a reliable event with the tick (LOOP_V2 §7).
- Reason: without it, a 100 ms player must lead a sprinting soldier by about 1.8 m (F34, legacy `T`). With it, the
  shooter's view is the truth up to the cap, and the victim's "hit behind cover" window is bounded by the cap (F34).
- Rests on: F27, F28, F29, F30, F31, F32, F34.
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
  the vehicle, and the snapshot carries the vehicle's state as the pilot's predicted state.
- Everyone else interpolates it (REC4).
- Parts. Each part (hull, turret, legs as needed) is a hit volume with its own transform and health in sim data. Part
  transforms are compact state (yaw, pitch); animation stays cosmetic (GC16, GC17).
- Weapons. Vehicle weapons fire from sim mount points, never from an animated node (GC17). Hitscan weapons get REC5;
  projectile weapons REC6.
- Seats for several players are §6.4 [OPEN], so no passenger model is proposed (OQ7).
- Reason: it follows from the shipped prediction model (F12, F13) applied to whatever the player controls. Multi-part
  history is the same table as REC5.
- Rests on: F12, F13, F15, F28, F29.
- Serves: VISION_V2 §6.4 (vehicles as first-class, big and multi-part), §6.9 (spectacle).
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

---

## 5. Open questions

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
  past, so colliding with them mispredicts. Soldiers could not collide with each other, collide softly, or collide
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

---

## 6. Sources read (all read 2026-10-27)

| # | URL | Title | Version or page date |
|---|---|---|---|
| 1 | https://www.rfc-editor.org/rfc/rfc9221.html | RFC 9221: An Unreliable Datagram Extension to QUIC | March 2022, Standards Track |
| 2 | https://github.com/id-Software/Quake-III-Arena (+ README.txt) | Quake III Arena GPL Source Release | one commit on master; no version or date shown; GPL-2.0 |
| 3 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/server/sv_snapshot.c | Quake III server snapshots | as 2 |
| 4 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/qcommon/qcommon.h | Quake III common header | as 2 |
| 5 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/client/cl_input.c | Quake III client input and command packets | as 2 |
| 6 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/client/cl_main.c | Quake III client main (cvar defaults) | as 2 |
| 7 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/client/cl_cgame.c | Quake III client time and snapshot clock | as 2 |
| 8 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/cgame/cg_predict.c | Quake III client prediction | as 2 |
| 9 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/cgame/cg_main.c | Quake III cgame cvar defaults | as 2 |
| 10 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/cgame/cg_snapshot.c | Quake III snapshot transitions | as 2 |
| 11 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/cgame/cg_ents.c | Quake III entity interpolation | as 2 |
| 12 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/g_weapon.c | Quake III weapon fire | as 2 |
| 13 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/g_missile.c | Quake III missiles | as 2 |
| 14 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/bg_misc.c | Quake III shared trajectory code | as 2 |
| 15 | https://github.com/ValveSoftware/source-sdk-2013 | Source SDK 2013 repository (layout) | master; no version or commit date shown |
| 16 | https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/player_lagcompensation.cpp | Source lag compensation | as 15 |
| 17 | https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/player.cpp | Source player (WantsLagCompensationOnEntity, command limits) | as 15; first 100,000 of 265,981 characters read |
| 18 | https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/public/inetchannelinfo.h | Source net channel info (GetLatency) | as 15 |
| 19 | https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/client/c_baseentity.cpp | Source client entity (interpolation, extrapolation cvars) | as 15; read in two parts (0–100,000 and 100,000–186,120) |
| 20 | https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/shared/usercmd.h | Source user command | as 15 |
| 21 | https://gafferongames.com/post/what_every_programmer_needs_to_know_about_game_networking/ | What Every Programmer Needs To Know About Game Networking (Glenn Fiedler) | 2010-02-24 |
| 22 | https://gafferongames.com/post/introduction_to_networked_physics/ | Introduction to Networked Physics (Fiedler) | 2014-11-28 (read; no claim cited) |
| 23 | https://gafferongames.com/post/deterministic_lockstep/ | Deterministic Lockstep (Fiedler) | 2014-11-29 |
| 24 | https://gafferongames.com/post/snapshot_interpolation/ | Snapshot Interpolation (Fiedler) | 2014-11-30 |
| 25 | https://gafferongames.com/post/snapshot_compression/ | Snapshot Compression (Fiedler) | 2015-01-04 |
| 26 | https://gafferongames.com/post/state_synchronization/ | State Synchronization (Fiedler) | 2015-01-05 |
| 27 | https://gafferongames.com/post/networked_physics_2004/ | Networked Physics (2004) (Fiedler) | 2004-09-04 |
| 28 | https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/game/q_shared.h | Quake III shared header (ANGLE2SHORT, usercmd_t, trajectory_t) | as 2 |

## 7. Sources not read

| URL | Reason |
|---|---|
| https://developer.valvesoftware.com/wiki/Source_Multiplayer_Networking (and the `action=raw` form) | On the allow list, but the site returned HTTP 403 to the fetch tool |
| https://developer.valvesoftware.com/wiki/Lag_compensation and https://developer.valvesoftware.com/wiki/Lag_Compensation | HTTP 403 (as above) |
| https://developer.valvesoftware.com/wiki/Interpolation | HTTP 403 (as above) |
| Yahn Bernier, "Latency Compensating Methods in Client/Server In-game Protocol Design and Optimization" (hosted on the Valve wiki) | Not attempted after four 403s from the same site; same reason |
| https://gdcvault.com/play/1024001/-Overwatch-Gameplay-Architecture-and (Timothy Ford, GDC 2017), also the www. form | Not read: video only; the vault redirected to https://gdconf.com/gdc-vault-maintenance/, which is outside the allow list and was not followed |
| Other Overwatch GDC 2017 talks on the same vault (Dan Reed on networking scripted weapons and abilities; Philip Orwig on replay technology) | Not read: video only (same vault) |
| edgegap.com, sohu.com, juejin.cn and gamedev.net write-ups of the Overwatch talk (seen in search results) | Outside the allow list |
| fabiensanglard.net write-ups of the Quake 3 network model | Outside the allow list; the released source was read instead |

## 8. Cross-unit notes

- **Tick rate and bandwidth.**
  - REC4's delay and REC5's cap are in snapshot intervals and need the chosen rates (OQ1).
  - Quake III's defaults (F10) are a historical floor only.
  - GC5's cooldown rounding (fire timers reset instead of carrying the remainder) must be fixed in any tick choice.
  - The full-snapshot or baseline-relative cost after a long loss (REC1) and the aim point's bit cost (REC8) belong in
    the budget.
  - Fiedler's quantisation figures (smallest-three 29 bits; 512 values per metre) are in the notes (S-GF3).
- **Interest management.**
  - Source compensates only entities already sent to and acknowledged by the shooter (F32). The chosen interest set
    therefore also limits who can be hit through lag compensation.
  - The snapshot's entity cap and priority order feed REC1 and REC9.
- **WebTransport stack.**
  - Snapshots go as datagrams, each fitting one QUIC packet (F11; OQ4).
  - Reliable hit-confirm and trajectory-change events go on streams.
  - The WebSocket fallback has no unreliable channel, so stale snapshots queue behind lost ones. REC1's "deltas against
    the acknowledged snapshot" still works, but the delay grows. That unit should measure it.
- **Rooms and private matches.** Late join and rejoin start from per-entity defaults (REC1). A human taking over a bot
  slot keeps the entity id (GC2, STATUS.md:68).
- **R1 (simulation core).**
  - Identical results between Wasm and native set the prediction correction rate (REC3), not sync correctness (F5).
  - REC6 needs the trajectory function to be bit-identical on both.
  - Seeded random streams per purpose (spread, cosmetic) are needed for F20's shared seed.
- **R4 (physics and navigation).**
  - The rewound shot query needs a spatial index (CS6) and height-aware line of sight (CS9, §6.2).
  - Moving obstacles (big vehicles) are OQ8.
- **Security-auditor (R2 protocol review).** Look at OQ11 (the shared spread seed), REC2's command validation (view-time
  window, command cap, aim range) and the rewind cap as an exploit surface (a client that fakes high latency).
- **Game-designer.** OQ5 (body-blocking), OQ9 (melee rewind), OQ13 (bot reaction against human view delay), and the
  feel of OQ2 and OQ6 in the Feel Lab.
