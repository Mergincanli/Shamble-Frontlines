Task: verify research topic R2 (netcode), docs/research/r2-netcode.md, wave 1 step 3 (Verify) of .checkpoints/v2-research/plan.md

## Progress log
- started 2026-10-28; no prior notes
Scope (union of "Rests on:" lists, 135 findings): SYNC F1-F9,F11-F26,F28-F35 (33; F10,F27 not in any list) · BUD F1-F15,F18-F32,F34,F35 (32; F16,F17,F33 not listed) · INT F1-F20 (20) · TRN F3-F22,F24-F26,F28-F35,F38 (32; F1,F2,F23,F27,F36,F37,F39 not listed) · ROOM F1-F18 (18). Plus sourced numbers in RECs/pass marks.
Method (same as R1 verifiers): raw files and HTML fetched with curl from allow-listed domains into scratchpad r2v/dl/ (untrusted data), text-extracted, quotes checked by exact string match (whitespace/quote normalized). WebFetch only if curl fails, asking for verbatim quotes.
- Downloads: 109/111 sources fetched (github.com repo pages for wtransport and hyperium/h3 returned 403; raw README of h3 fetched instead). Page dates of all 9 Fiedler articles match the file.
- SYNC-F1 confirmed: gaffer what_every_programmer, section "Peer-to-Peer Lockstep": "This means that each player in the game has latency equal to the most lagged player."
- SYNC-F2 confirmed: same section: "this is not common due to the difficulty of capturing and transmitting a completely deterministic starting point".
- SYNC-F3 confirmed: deterministic_lockstep: "Exact down to the bit-level." and "Differences in floating point behavior between compilers, OS's and even instruction sets".
- SYNC-F4 confirmed: deterministic_lockstep: "the simulation on the right can only simulate frame n when it has the input for that frame." and "each time a packet is lost, TCP has to wait RTT*2 while it is resent (actually it can be much worse, but I'm being generous…)" (finding cuts before the parenthesis).
- SYNC-F5 confirmed: snapshot_interpolation, intro: both quotes verbatim.
- SYNC-F6 confirmed: state_synchronization: all four quotes verbatim.
- SYNC-F7 confirmed: snapshot_compression, delta section: all five quotes verbatim ("this snapshot is encoded relative to the initial state of the simulation" sits inside quotation marks in the source; same para offers the alternative "send down the initial state using a non-delta encoded path, eg. as one large data block").
- SYNC-F12 confirmed: what_every_programmer, "Client-Side Prediction": all quotes verbatim; full sentence: "Only in the rare case, such as a player getting hit by a rocket, or bumping into an enemy, will the client's location need to be corrected."
- SYNC-F16 confirmed: networked_physics_2004: all six quotes verbatim.
- SYNC-F21 confirmed: snapshot_interpolation: all quotes verbatim (para "My rule of thumb..." and "How can we reduce..."). Source does not say 150 ms = 3 intervals + jitter (the finding marks that inferred).
- SYNC-F22 confirmed: "extrapolation doesn't work very well for rigid bodies because their motion is non-linear and unpredictable"; "hermite interpolation for position @ 10pps ... I can no longer see any artifacts"; linear artifacts: "Look closely though and you can see some artifacts on the right side."
Next: Quake III files (SYNC-F8,F9,F13-F15,F17-F19,F23-F25,F35).
- SYNC-F8 confirmed: q3 qcommon.h:117-118 `#define PACKET_BACKUP 32 // number of old messages that must be kept on client and // server for delta comrpession and ping estimation` (comment spans 2 lines); sv_snapshot.c:136-158 SV_WriteSnapshotToClient (all quoted lines; 29 = PACKET_BACKUP-3), :89-106 SV_EmitPacketEntities, :228 MAX_SNAPSHOT_ENTITIES 1024. Extra: with oldframe NULL, SV_EmitPacketEntities sets from_num_entities = 0 (:62-63), so every entity takes the "new entity, send it from the baseline" path: SYNC-REC1's "inferred" step is confirmed by the code.
- SYNC-F9 confirmed: cl_input.c:713-716 and :748-750 CL_WritePacket.
- SYNC-F13 confirmed: cg_predict.c:398-406 header comment, :584 `Pmove (&cg_pmove);`.
- SYNC-F14 confirmed: cg_predict.c:469-474; comment above (:466-468) "if we don't have the commands right after the snapshot, we can't accurately predict a current position, so just freeze at the last good position we had".
- SYNC-F15 confirmed: cg_predict.c:410, :550, :559, :566; cg_main.c:252 `{ &cg_errorDecay, "cg_errordecay", "100", 0 },`.
- SYNC-F17 confirmed: cg_predict.c:41, :72, :271, :303-306, :375; cg_main.c:266 cg_predictItems "1".
- SYNC-F18 confirmed: cl_input.c:731-737 (clamp 0..5), :738 MAX_PACKET_USERCMDS cap, :769-770; qcommon.h:121; cl_main.c:2305; Fiedler deterministic_lockstep: "...we redundantly include all inputs in each UDP packet until we know for sure the other side has received them?" (question mark in source).
- SYNC-F19 confirmed: cl_input.c:512-517; q_shared.h:1082 ANGLE2SHORT, :1248-1254 usercmd_t.
- SYNC-F23 confirmed: cl_cgame.c:810, :820, :844-858, :962-971 (timeNudge clamped to ±30), :985-986; cg_snapshot.c:366.
- SYNC-F24 confirmed: cg_snapshot.c:217, :68 (x3), :296-297.
- SYNC-F25 confirmed: cg_ents.c:718, :741, :761-762, :1004 (cg_smoothClients default "0", cg_main.c:305, so players are TR_INTERPOLATE).
- SYNC-F35 confirmed: g_missile.c:25, :544 (x5), :585, :547, G_BounceMissile :42 (reflect), :53-55 (new trBase and trTime), :70, :466-470; bg_misc.c:1229; q_shared.h:1271-1275 trajectory_t.
- BUD-F24 confirmed: q3 msg.c:915, :928, :844 (with comment :842-843 "the float will be sent with FLOAT_INT_BITS, otherwise all 32 bits will be sent"), field table :800-807.
- BUD-F25 confirmed: qcommon.h:117-118; sv_init.c:227 numSnapshotEntities formula.
- BUD-F26 confirmed: sv_init.c:596 sv_fps "20"; cl_main.c:2304-2305, :2348-2349; qcommon.h:121.
- BUD-F27 confirmed: net_chan.c:50, :52; qcommon.h:169.
Next: Source SDK files (SYNC-F20,F26,F28-F33; BUD-F28,F29; INT-F8-F13; ROOM-F15).
- SYNC-F20 confirmed: source-sdk usercmd.h:139-140, :159, :161, :167-168.
- SYNC-F26 confirmed: c_baseentity.cpp:80 `static ConVar cl_extrapolate( "cl_extrapolate", "1", FCVAR_CHEAT, "Enable/disable extrapolation if interpolation history runs out." );`
- SYNC-F28 confirmed (with note): player_lagcompensation.cpp:34 sv_maxunlag, StartLagCompensation :389, :393-396, :399, :402, :405 `float deltaTime = correct - TICKS_TO_TIME(gpGlobals->tickcount - targettick);`, :407-411; inetchannelinfo.h:56 GetLatency "// current latency (RTT), more accurate but jittering". Notes: (a) the RTT reading rests on that header comment, but the call passes a flow (FLOW_OUTGOING), so whether the term is RTT or one-way is not settled by the code (BUD-F29 reads it as outgoing; XU-OQ2); (b) the clamp applies to the server's estimate `correct`; when the client tick is trusted the rewind can exceed the clamp by up to 0.2 s ("The total is clamped" is slightly loose).
- SYNC-F29 confirmed: LagRecord :114-126 (also m_vecMinsPreScaled/MaxsPreScaled :116 = bounding box), purge :258, :290.
- SYNC-F30 confirmed: BacktrackPlayer :473-482; :28 sv_lagcompensation_teleport_dist "64".
- SYNC-F31 confirmed: :363-369 (single player, bot, also observers excluded) and :425-426.
- SYNC-F32 confirmed (with note): player.cpp WantsLagCompensationOnEntity :749-782, all quotes verbatim. Combination now read: distance test returns true first (:767-768), else cone test (:778-780): targets are near OR in the 45° cone (after team and acked filters).
- SYNC-F33 confirmed: player.cpp:103 help text verbatim, default "24" (the finding said the default was unread); :3392-3413 PhysicsSimulate.
- BUD-F28 confirmed: const.h:31; gameinterface.cpp:829-851 (comment :835, #if :838, -tickrate override :843-848).
- BUD-F29 confirmed: same lines as SYNC-F28; the 0.2 s check's meaning is confirmed by the comment at :409 ("difference between cmd time and latency is too big > 200ms, use time correction based on latency").
- INT-F8 confirmed: baseentity.cpp ShouldTransmit :4049-4096 (quoted one-liners are the multi-line if/else with braces removed), UpdateTransmitState :4024-4025.
- INT-F9 confirmed: baseentity.cpp:4118, :4133.
- INT-F10 confirmed: team.cpp:102-106, :114-115.
- INT-F11 confirmed: sendproxy.cpp:79-96 SendProxy_OnlyToTeam.
- INT-F12 confirmed: tf_player_shared.cpp: DT_TFPlayerShared table begins :535, disguise props :577-582, and :590 `SendPropDataTable( "tfsharedlocaldata", 0, &REFERENCE_SEND_TABLE( DT_TFPlayerSharedLocal ), SendProxy_SendLocalDataTable )` (local table :511): the owner-only split is visible in code (proxy name), reach-all-clients still an inference.
- INT-F13 confirmed: hltvdirector.cpp:24, :215-216.
- ROOM-F15 confirmed: tf_bot_manager.cpp:27-29 ConVar help strings, :432. Note: tf_bot_quota default "0" and tf_bot_quota_mode default "normal" (:26-27): fill is an option, not TF2's default.
- BUD-F30 confirmed: GNS steamnetworkingsockets_internal.h:263-266; csteamnetworkingsockets.cpp:87 MTU_PacketSize 1300.
- BUD-F31 confirmed: steamnetworkingtypes.h:1235-1240 (all three quotes, comment spans lines; also "these two options should always be set to the same value, to manually configure a specific send rate"); csteamnetworkingsockets.cpp:84-85.
Next: Fiedler BUD-F18-F23, F32, F34, F35, INT-F14, F15, TRN-F20, F22, ROOM-F9.
- BUD-F18 confirmed: gaffer packet_fragmentation: "if any fragment of that packet is lost then the entire packet is lost"; formula "1 - probability_of_fragment_being_delivered ^ num_fragments"; "Two fragments: 1 - (99/100) ^ 2 = 2%", "Ten fragments: 1 - (99/100) ^ 10 = 9.5%"; "my first guess for a conservative lower bound on the IPv4 Internet today would be 1200 bytes"; "the typical number before packets start to get dropped or fragmented is somewhere around 1472".
- TRN-F20 confirmed: same article: "has the option of a) fragmenting that packet, or b) dropping the packet"; "suddenly all packets start getting dropped!"; "calculate how many 1024 byte fragments it needs to be split into"; "100 fragments: 1 - (99/100) ^ 100 = 63.4%" (verbatim, so the 63.4 % the finding marks inferred is confirmed).
- BUD-F19 confirmed: snapshot_compression: 512 values per-meter and 18/18/14 = 50 bits quotes verbatim.
- BUD-F20 confirmed: "2 + 9 + 9 + 9 = 29 bits per-orientation (down from 128 bits)"; "Linear interpolation is good enough at 60HZ. This means we can avoid sending linear velocity entirely."
- BUD-F21 confirmed: "Each cube that isn't sent costs 1 bit (not changed)... At 60 packets per-second this adds up to 54kbps of bandwidth"; baseline quotes as SYNC-F7; averages verbatim: "an average of 5.5 bits per-relative index", "on average 26.1 bits for changed positions values" (finding marks these inferred; they are verbatim).
- BUD-F22 confirmed: "around 15 kilobits per-second", header and "IP and UDP headers (28 bytes)" quotes; "17.37 megabits per-second!".
- BUD-F23 confirmed: "This implementation by Fabian Giesen beat my best bit-packed result by 25%."
- BUD-F32 confirmed: same quotes as SYNC-F21 (snapshot_interpolation). "150 ms = 3 intervals + jitter" stays the unit's inference (Fiedler only says 150 ms gives "the same amount of packet loss protection").
- BUD-F34 confirmed: state_synchronization, sections "Packet Structure"/"Priority Accumulator": all quotes verbatim; reset rule verbatim: "reset the priority accumulator to zero for objects that fit but leave the priority accumulator value alone for objects that didn't" (finding marks it inferred).
- BUD-F35 confirmed: "You just need to delay packets just enough (say 4-5 frames @ 60HZ)"; redundancy verbatim: "Like deterministic lockstep we send multiple redundant inputs so in the case of packet loss it's very unlikely that an input gets dropped." (finding marks it inferred).
- INT-F14 confirmed: state_synchronization: all quotes incl. player priority "1000000", "a maximum of 64 state updates per-packet", "256kbit/sec".
- INT-F15 confirmed: snapshot_compression "Cube n in snapshot 110 is the same as the baseline. One bit: Not changed!" and "around 15 kilobits per-second".
- TRN-F22 confirmed: udp_vs_tcp: all five quotes verbatim.
- ROOM-F9 confirmed: why_cant_i_send_udp: all five quotes verbatim; also "Connect tokens are short lived and rely on a shared private key between the web backend and the dedicated server instances"; "A private portion, encrypted and signed by the shared private key"; "a list of dedicated server addresses that the connect token is valid for"; "The REST call returns a connect token to that client encoded as base64 over HTTPS."
Next: RFCs (BUD-F1-F9, F15-F17; SYNC-F11; TRN-F16, F17, F24; ROOM-F11).
- SYNC-F11 confirmed: RFC 9221 §5.2 "Although DATAGRAM frames are not retransmitted upon loss detection, they are ack-eliciting" and "Due to reordering, this can include a DATAGRAM frame that was thought to be lost"; §5.3 "they MAY be dropped by the receiver if the receiver cannot process them"; §5 "DATAGRAM frames cannot be fragmented; therefore, application protocols need to handle cases".
- TRN-F16 confirmed (RFC part): §2 "QUIC datagrams are subject to QUIC congestion control."; §5.4 delay-or-drop; §5.2; §5; §5.3 "DATAGRAM frames do not provide any explicit flow control signaling". Draft part below.
- BUD-F6 confirmed: RFC 9221 §5 no fragmentation; §4 "0x30 and 0x31 ... LEN bit (0x01) ... if this bit is set to 0, the Length field is absent and the Datagram Data field extends to the end of the packet" (verbatim; finding marks LEN detail inferred).
- BUD-F7 confirmed: RFC 9221 §5.4 verbatim.
- BUD-F8 confirmed: §5.2 verbatim, and §5.2 also verbatim: "acknowledgement of a DATAGRAM frame only indicates that the transport-layer handling on the receiver processed the frame and does not guarantee that the application on the receiver successfully processed the data. Thus, this signal cannot replace application-layer signals that indicate successful processing." (finding marks it inferred; it is verbatim).
- BUD-F1, F2, F3 confirmed: RFC 9000 §14 (Datagram Size): all quotes verbatim.
- BUD-F4 confirmed: RFC 9000 §17.3.1 1-RTT Packet figure, fields verbatim.
- BUD-F5 confirmed: RFC 9001 §5.3 AEAD Usage verbatim.
- BUD-F9 confirmed: RFC 9297 §2.1 format quotes verbatim; RFC 9000 §16 Table 4 row "00 1 6 0-63". Draft -16 §4.5 part below.
- BUD-F15 confirmed: RFC 9002 §7.2 initial and minimum window quotes; §7.7 pacing quote.
- TRN-F24 confirmed: RFC 9002 §7 NewReno and CUBIC quotes, §7.6 kMinimumWindow, §7.2, §7.7 all verbatim.
- TRN-F17 confirmed (RFC part): RFC 9000 §14 verbatim. quinn part below.
- TRN-F39 (not in any Rests on): RFC 9000 §1 and §5.1 quotes found verbatim.
- BUD-F16 confirmed: RFC 6455 §5.2 "Payload length: 7 bits, 7+16 bits, or 7+64 bits", "If 126, the following 2 bytes ...", "Masking-key: 0 or 4 bytes"; §5.1 "a client MUST mask all frames that it sends to the server".
- BUD-F17 confirmed: RFC 8446 §5.2 length rule verbatim.
- ROOM-F11 confirmed: RFC 7519 §4.1.3 aud, §4.1.4 exp (+ leeway sentence), §4.1.7 jti: all verbatim.
Next: datatracker drafts (BUD-F9 draft part, TRN-F3-F6, F16 draft part, F26, ROOM-F13), W3C ED/TR (BUD-F10-F14, TRN-F10, F12, F13, F18, F19).
- TRN-F3 confirmed: datatracker draft-ietf-webtrans-http2 page: "draft-ietf-webtrans-http2-15", "Last updated 2026-07-06", "In WG Last Call", "Proposed Standard" (finding marks it inferred; the fields are on the page).
- TRN-F4 confirmed: draft -16 §7.1 "Negotiating the Draft Version": both quotes; §9.2 value 0x2c7cf000 (SETTINGS_WT_ENABLED); draft -07 App. A.1 "Changes between draft versions 02 and 07": quote verbatim, WEBTRANSPORT_MAX_SESSIONS = 0xc671706a; draft -02 ENABLE_WEBTRANSPORT = 0x2b603742.
- TRN-F16 draft part confirmed: -16 §4.5 Datagrams "the payload field directly follows the Quarter Stream ID field, which is at the start of the QUIC DATAGRAM frame payload"; §4.1 "datagrams, which are not retransmitted".
- BUD-F9 draft part confirmed: -16 §4.5 "The WebTransport datagram payload is sent unmodified in the "HTTP Datagram Payload" field". Doc page: "draft-ietf-webtrans-http3-16", "Last updated 2026-07-06", "In WG Last Call".
- TRN-F26 confirmed: -16 §8 Security Considerations "all of the WebTransport sessions share both congestion control and flow control context".
- ROOM-F13 confirmed: -16 §3.2 Creating a New Session (two quotes), §5.2 Limiting the Number of Simultaneous Sessions ("Servers SHOULD limit the rate of incoming WebTransport sessions on HTTP/3 connections.").
- (not in Rests on) TRN-F23 §3.4, TRN-F36 §3.2, TRN-F37 §6 quotes also found verbatim.
- BUD-F10 confirmed: W3C ED (w3c.github.io, "Editor's Draft, 27 October 2026", "Don't cite this document other than as work in progress"): §5.3 maxDatagramSize "The maximum size data that may be passed to a WebTransportDatagramsWritable."; §5.1 path MTU sentence; [[OutgoingMaxDatagramSize]] "an implementation-defined integer".
- BUD-F11 confirmed: ED §4.3 Procedures, writeDatagrams: "If duplex.[[OutgoingMaxDatagramSize]] is less than data's [[ByteLength]], return a promise resolved with undefined." This step comes before "Enqueue chunk", so no error and not queued is read directly from the algorithm (finding marks it inferred). TR §4.3 has the same step (with "datagrams" for "duplex").
- BUD-F12 confirmed: ED §6.13 WebTransportConnectionStats: all four quotes verbatim.
- BUD-F13 confirmed: ED §6.14 both quotes; §5.1 "expiration duration for outgoing datagrams (in milliseconds), or null" (units verbatim; finding marks inferred).
- BUD-F14 confirmed: ED §6.9 Configuration: "This is a hint to the user agent. This configuration option is considered a feature at risk due to the lack of implementation in browsers of a congestion control algorithm, at the time of writing, that optimizes for low latency." (finding says no verbatim quote obtained; it is verbatim).
- TRN-F10 W3C part confirmed: TR §6.9 same sentence.
- TRN-F12 W3C part confirmed, location error: the [[Reliability]] "supports-unreliable"/"reliable-only" steps are in TR §6.2 Constructor, not §6.5; requireUnreliable sentence §6.9 verbatim.
- TRN-F13 W3C part confirmed: TR §6.9 custom certificate requirements: all four quotes verbatim; SHA-256 only: "If hash.value is not null and hash.algorithm is an ASCII case-insensitive match with "sha-256"" (verbatim; finding marks inferred).
- TRN-F18 confirmed: TR §5.1 (three quotes) and §4.3 writeDatagrams step.
- TRN-F19 W3C part confirmed: TR §5.1 "beyond which datagrams are dropped from the head of the queue"; §4.3 "If more than duration milliseconds have passed since timestamp, then".
Next: BCD JSON (TRN-F7-F11, F19 rows, F34, F38), MDN pages, WebKit post.
