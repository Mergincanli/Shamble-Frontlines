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
- TRN-F7 confirmed: BCD api/WebTransport.json: WebTransport chrome 97, edge "mirror", firefox 114, safari 26.4, safari_ios "mirror"; serverCertificateHashes 100/125/26.4. browsers/*.json: chrome 97 2022-01-04, 100 2022-03-29, 156 2026-10-20 "current"; firefox 114 2023-06-06, 125 2024-04-16, 158 2026-10-13 "current", 153 2026-07-21 "esr"; safari 26.4 2026-03-24, 27 2026-09-14 "current". MDN WebTransport_API (last modified 2026-10-21): "Baseline 2026", "Newly available", "Since March 2026, this feature works across the latest devices and browser versions.", "Some parts of this feature may have varying levels of support." Extra: edge.json shows Edge 156 current, released 2026-10-22.
- TRN-F8 confirmed: BCD DatagramDuplexStream.json: writable chrome 97, firefox 114, safari false, standard_track false, deprecated true; createWritable chrome false, firefox 155, safari 26.4; firefox 155 = 2026-09-01.
- TRN-F9 confirmed: getStats chrome false with impl_url https://crbug.com/41492543; firefox 114 partial_implementation, notes "Method is defined but throws a not-implemented error."; safari 26.4.
- TRN-F10 confirmed: congestionControl option chrome false, firefox 114, safari 26.4; MDN ctor (modified 2026-10-12) "This is a hint to the user agent."; W3C §6.9 (above).
- TRN-F11 confirmed (BCD data): requireUnreliable chrome false / firefox 114 / safari 26.4; allowPooling, sendOrder (both stream kinds), createSendGroup chrome false; protocols 143/155/26.4 (chrome 143 = 2025-12-02); reliability and supportsReliableOnly chrome 157 (2026-11-03, status "beta"); firefox createUnidirectionalStream partial, notes "Returns a `WritableStream` instead of a `WebTransportSendStream`."; ESR 153 < 155. All rows the finding marked inferred are in the data.
- TRN-F12 confirmed: WebKit blog 17862 ("WebKit Features for Safari 26.4", 2026-03-24): "When the underlying network environment doesn't support QUIC, WebTransport can run over HTTP/2 and TCP as a fallback with the same API."; W3C (location §6.2, see above); MDN ctor requireUnreliable: "If true, the connection cannot be established over HTTP/2 if an HTTP/3 connection is not possible." "By default, the value is false."
- TRN-F13 confirmed: W3C (above); MDN ctor "This option is only supported for transports using dedicated connections (allowPooling is false)."
- TRN-F14 confirmed: MDN ctor: all three quotes verbatim (serverCertificateHashes).
- TRN-F19 BCD rows confirmed: outgoingMaxAge and incomingMaxAge 97/114/26.4; incoming/outgoingMaxBufferedDatagrams chrome 151, firefox false, safari "preview".
- TRN-F21 confirmed: WebKit blog: "they're built on TCP, which means head-of-line blocking can cause delays when packets are lost" and "It runs over HTTP/3 and QUIC, providing multiple streams that won't block each other."
- TRN-F34 confirmed: BCD WebSocket.json: chrome 5, edge 12, firefox 11 plus {prefix "Moz", version_added 7, version_removed 11}, safari 5; worker_support chrome 5, firefox 37, safari 5; binaryType chrome 15, firefox 11, safari 6 (finding marks binaryType inferred; it is in the data).
- TRN-F35 confirmed: MDN WebTransport_API: three quotes verbatim.
- TRN-F38 confirmed: MDN ctor protocols sentence verbatim; BCD protocols 143/155/26.4 (finding marks versions inferred; they are in the data).
- (not in Rests on) TRN-F37 browser rows: draining chrome false, firefox 155, safari 26.4.
- ROOM-F12 confirmed: MDN WebSocket() (modified 2026-06-22) "new WebSocket(url)" / "new WebSocket(url, protocols)"; MDN WebTransport() options exactly allowPooling, congestionControl, protocols, requireUnreliable, serverCertificateHashes; "The scheme must be HTTPS, and the port number needs to be explicitly specified." "No auth header" stays the unit's inference.
Next: QUICHE, wtransport, web-transport-proto, quinn docs, crates.io, moq README, h3, axum (TRN-F5, F6, F15, F17, F25, F28-F33).
- TRN-F5 confirmed: quiche http_constants.h:43-45 "// draft-ietf-webtrans-http3", `SETTINGS_WEBTRANS_DRAFT00 = 0x2b603742,`, `SETTINGS_WEBTRANS_MAX_SESSIONS_DRAFT07 = 0xc671706a,`; no 0x2c7cf000 / WT_ENABLED in file.
- TRN-F6 confirmed: wtransport-proto settings.rs:247-248 (no 2c7c codepoint); docs.rs web-transport-proto 0.6.3 settings.rs: line 77 "ENABLE_DATAGRAM_DEPRECATED = 0xFFD277, // still used by Chrome", line 80 "// Removed in draft 06" (above WEBTRANSPORT_ENABLE_DEPRECATED = 0x2b603742), "// TODO remove when 07 is in the wild", enable_webtransport().
- TRN-F15 confirmed: docs.rs wtransport 0.7.2 Identity: "This function uses the ECDSA P-256 algorithm.", "The total length of the validity period MUST NOT exceed two weeks.", `.validity_days(14)`; Cargo.toml:57 `default = ["self-signed", "ring"]`, :35 rcgen 0.14.5 optional, :65 self-signed = ["dep:rcgen"].
- TRN-F17 quinn part confirmed: docs.rs quinn 0.11.12 TransportConfig initial_mtu "Must be at least 1200, which is the default, and known to be safe for typical internet applications"; min_mtu "Must be at least 1200, which is the default"; mtu_discovery_config "Enabled by default."
- TRN-F25 confirmed: quinn::congestion: Cubic "The RFC8312 congestion controller, as widely used for TCP", NewReno "A simple, standard congestion controller", Bbr "Experimental! Use at your own risk."; Controller / ControllerFactory traits; TransportConfig example NewRenoConfig; Connection: send_datagram, "They may be lost or delivered out of order", send_datagram_wait "will wait for buffer space during congestion conditions", rtt "Current best estimate of this connection's latency (round-trip-time)", congestion_state "for debugging purposes", datagram_send_buffer_space exists.
- TRN-F28 confirmed: crates.io quinn newest 0.11.12, updated_at 2026-09-14T10:08:47.662258Z, created_at 2018-10-02T20:18:56.824108Z; downloads 381,661,773 on 2026-10-28 (379,896,223 on 10-27 is consistent).
- TRN-F29 confirmed: crates.io wtransport 0.7.2, 2026-08-11T22:08:08.540291Z, 2023-05-25T19:45:52.658071Z, 1,088,337 today; Cargo.toml rust-version "1.88", quinn 0.11.6 runtime-tokio, rustls 0.23.23, features ring/aws-lc-rs/fips; README:31-32 both quotes verbatim; docs.rs Connection: send_datagram, receive_datagram, max_datagram_size, rtt, quic_connection "Returns a reference to the inner QUIC connection." Stars/issues not checked (github.com page 403 for curl).
- TRN-F30 confirmed: crates.io web-transport-quinn 0.13.5, 2026-10-24T00:53:07Z, created 2024-04-06; releases 0.13.0 (2026-09-27) to 0.13.5 (6 releases), 0.12.0 2026-08-07 (finding marks the list inferred; registry versions list confirms). moq README "Crates" list: all four quotes verbatim (markdown links stripped). README also lists a generic `web-transport` crate "delegating to web-transport-quinn or web-transport-wasm depending on the platform" (relevant to TRN-REC4/OQ6, not cited).
- TRN-F31 confirmed: hyperium/h3 README:16 "The `h3` crate is still very experimental."; crates.io h3-webtransport 0.1.2, updated 2025-05-06T21:21:35Z, 3 releases (0.1.0 2023-11-06, 0.1.1, 0.1.2).
- TRN-F32 confirmed: datatracker QMux page: "draft-ietf-quic-qmux-02", 2026-07-06, "WG Document", "Proposed Standard"; abstract both fragments verbatim (state fields marked inferred are on the page).
- TRN-F33 confirmed: crates.io tokio-tungstenite 0.30.0, 2026-07-11T12:58:44Z, created 2017-03-17; axum 0.8.9, 2026-04-14T07:55:20Z; docs.rs axum 0.8.9 extract::ws "Handle WebSocket connections." and "Available on crate feature ws only." (dependency ^0.29.0 not re-checked).
Next: three.js, Chrome DevTools, AWS GameLift, PlayFab, Agones, OWASP, tokio, Rust std, Cargo, netcode STANDARD (INT-F16, F17; ROOM-F1-F8, F10, F14, F16-F18).
- INT-F16 confirmed: developer.chrome.com memory-inspector ("Last updated 2024-09-06 UTC"): all three quotes verbatim.
- INT-F17 confirmed: three.js PerspectiveCamera.js (dev):49-50 "The vertical field of view, from bottom to top of view, / in degrees." (spans two comment lines); :227 getViewBounds doc verbatim.
- ROOM-F1 confirmed: GameLift gamelift-sdk-interactions: ProcessReady quote :71; placement steps :130-155 ("searches those fleets for an active server process that's not hosting a game session", status ACTIVATING, "Changes the server process's number of game sessions to 1.", ActivateGameSession sentence).
- ROOM-F2 confirmed: same page RESERVED, AcceptPlayerSession sentences; API_PlayerSession Status: TIMEDOUT (60 seconds) and COMPLETED quotes verbatim.
- ROOM-F3 confirmed: sdk-interactions ProcessEnding quote and "The server process must respond healthy or not healthy within one minute."; fleets-multiprocess "...unhealthy for three consecutive reports."; API_RuntimeConfiguration GameSessionActivationTimeoutSeconds "If the game session does not become active before the timeout, it is ended and the game session status is changed to TERMINATED." and "Valid Range: Minimum value of 1. Maximum value of 600."
- ROOM-F4 confirmed: fleets-multiprocess intro and "Test different configurations..." quotes; RuntimeConfiguration "An Amazon GameLift Servers instance is limited to 50 processes running concurrently."
- ROOM-F5 confirmed: PlayFab lifecycle (ms.date 2022-01-26, updated 2025-05-01): all four quotes verbatim.
- ROOM-F6 confirmed: agones high-density-gameservers.md: three quotes verbatim; front matter 2021-08-31; Counters shown as `{{< beta title="Counters And Lists" gate="CountsAndLists" >}}`; "rooms" Counter present.
- ROOM-F7 confirmed: docs.rs tokio 1.54.1 task module and spawn_blocking: all quotes verbatim; the thread::spawn link is verbatim too: "For workloads that run indefinitely or for extended periods (for example, background workers or persistent processing loops), prefer a dedicated thread created with thread::spawn." (finding marks it inferred).
- ROOM-F8 confirmed: doc.rust-lang.org std::thread (1.99.1): three quotes verbatim; Cargo profiles "Terminate the process upon panic."
- ROOM-F10 confirmed: netcode STANDARD.md ("# netcode 1.02"): four quotes verbatim; also "If the dedicated server public address is not in the list of server addresses in the private connect token, ignore the packet." (the "not addressed to it" part).
- ROOM-F14 confirmed: OWASP Session Management Cheat Sheet: four quotes verbatim.
- ROOM-F16 confirmed: PlayFab login basics (ms.date 2025-02-20, updated 2025-10-31): five quotes verbatim.
- ROOM-F17 confirmed with note: create-a-lobby (updated 2026-03-24) and ownership-changes (updated 2025-09-10): all five quotes verbatim. Overreach: the source says the owner "has special permissions to update data on the lobby", not that the owner alone updates it (finding "owner alone"; ROOM-REC5 "sole write rights"). "configured during creation" is said of the four settings; "can't be changed" is said only of ownerMigrationPolicy ("This policy is configured when the lobby is created and can't be changed.").
- ROOM-F18 confirmed: sdk-interactions introduction: "When joining a game session, the game client connects directly to a game session using the game session's unique IP address and port number."
Next: inferred findings (SYNC-F34, INT-F18, F19, F20) arithmetic check; numbers in RECs and pass marks; ranked other findings.
- SYNC-F34 unsupported (the unit's own inference; no source passage read: the Valve wiki pages that explain it returned 403). The reasoning is consistent with F28's code, but no source states the "hit behind cover" window.
- INT-F18 confirmed (arithmetic re-done): pitch atan(40/56.6)=35.25°; near/far edge depression 42.75°/27.75° -> ground 43.27 m / 76.03 m from camera foot -> 13.33 m before and 19.43 m beyond the look point (depth 32.76 m); half-widths 0.2341 x axial depth 58.43 / 85.18 -> 27.36 m / 39.88 m wide; area 1,101 m²; far corner 27.8 m from the look point. Legacy inputs CD5 (height 40, boom 56.6, fov 15) match LEGACY_INVENTORY.md:86. Source part (vertical FOV) = INT-F17 confirmed.
- INT-F19 confirmed (arithmetic): 372x252 = 93,744 m² (CD21, LEGACY_INVENTORY.md:102); r 28/40/50 -> 2.63/5.36/8.38 %; 47 x 5.36 % = 2.5; 47 x 12-25 B = 564-1,175 B; x 20/s x 8 = 90-188 kbit/s. Record sizes are illustrative (marked).
- INT-F20 confirmed (arithmetic): pairs 576/768/432; 768 x 2.7-5.7 µs = 2.07-4.38 ms, x3 = 6.2-13.1 ms; x20/s = 4.1-8.8 % of a core. hasLOS 2.7-5.7 µs matches LEGACY_INVENTORY.md:174/:177.
- Legacy numbers in recs checked against docs: T moveSpeed 6.3, sprintMult 1.4286 (LEGACY_TUNING.md:117, :137) -> 9.0 m/s, 1.8 m in 200 ms; WEAPONS shabzon fireInterval 0.072 (:175) -> 13.9/s, 2.16 / 4.32 ticks; 48 x 13.9 = 667 shots/s; CM4 0.80-1.00 ms (INVENTORY :152); GN8 50 m / 90 m (:273). Default principles (.claude/agents/netcode-architect.md:34-36): "about 100 ms behind (2–3 snapshots)", "~1 s of hitbox history", "capped (~200 ms)" match the file's quotes. LOOP_V2 "everyone is told where" (:41) and "about 20 min" (:100) match; VISION pillar quote "I sat in a building and kept harassing them" (VISION_V2.md:21) matches.
- Extra source facts relevant to recs: (1) W3C ED §6.13: `required WebTransportDatagramStats datagrams;` is a member of WebTransportConnectionStats, so lostOutgoing/expiredOutgoing, like smoothedRtt/rttVariation/estimatedSendRate, come only from transport.getStats(), which BCD shows missing on Chrome/Edge and throwing on Firefox (TRN-F9). (2) quinn send_datagram: "Previously queued datagrams which are still unsent may be discarded to make space for this datagram, in order of oldest to newest."; send_datagram_wait "effectively prioritizes old datagrams over new datagrams" (supports TRN-REC6's non-waiting send).
- VISION_V2 §6 trace checked (VISION_V2.md:61-80): BUD-REC3, BUD-REC4, BUD-REC5 and TRN-REC6 name no §6 item (as XU-OQ18 says). INT-REC7 (spectators) cites §6.1 and §6.8, neither of which names spectators (§6.8's kill-cams and replays are [LATER]). INT-REC4 cites §6.9, which is a renderer budget ("the renderer handles a few very large units"), not a network need.
Next: write final table and ranked findings.

- INT-F1 confirmed: q3 sv_snapshot.c SV_AddEntitiesVisibleFromPoint :309, :327-383 (all quoted lines verbatim; "broadcast entities are always sent" :358, "blocked by a door" :370).
- INT-F2 confirmed: sv_snapshot.c :228, :269 "if we are full, silently discard entities", SV_RateMsec :532-544, :588.
- INT-F3 confirmed: sv_snapshot.c :89-99.
- INT-F4 confirmed: q3 cm_test.c :429 floodnum compare, CM_AdjustAreaPortalState :381-405 (++ / -- then CM_FloodAreaConnections).
- INT-F5 confirmed: q2 server/sv_send.c :156-158 multicast comments, :230, SV_StartSound :358-363 (use_phs = false).
- INT-F6 confirmed: q3 g_utils.c G_TempEntity :481-502, G_Sound :607.
- INT-F7 confirmed: q3 g_combat.c player_die :499-504, ScorePlum :37-38.

## Final table (claim id · claim, short · source · verdict · location)
SYNC-F1 · lockstep: latency of most-lagged player · gaffer what_every_programmer · confirmed · "Peer-to-Peer Lockstep"
SYNC-F2 · late join rare (deterministic start) · same · confirmed · same section
SYNC-F3 · bit-exact; float differs by compiler/OS/ISA · gaffer deterministic_lockstep · confirmed · two quotes verbatim
SYNC-F4 · lockstep waits for input; TCP RTT*2 stall · same · confirmed · para "Above you can see ... TCP" (quote truncated before "(actually...")
SYNC-F5 · snapshot interp needs no determinism · gaffer snapshot_interpolation · confirmed · intro
SYNC-F6 · state sync approximate; quantize both sides · gaffer state_synchronization · confirmed · intro + "Quantize Both Sides"
SYNC-F7 · delta vs acked baseline; 1 bit unchanged · gaffer snapshot_compression · confirmed · delta section
SYNC-F8 · Q3 ring 32, full snapshot rules, 1024 cap · q3 sv_snapshot.c, qcommon.h · confirmed · sv_snapshot.c:89-158,:228; qcommon.h:117-118
SYNC-F9 · Q3 client sends last seq; asks no-delta · q3 cl_input.c · confirmed · :713-716, :748-750
SYNC-F11 · QUIC datagrams not resent, reorder, drop, no fragment · RFC 9221 · confirmed · §5, §5.2, §5.3
SYNC-F12 · client prediction ring, rewind/replay · gaffer what_every_programmer · confirmed · "Client-Side Prediction"
SYNC-F13 · Q3 replays all unacked cmds; OPTIMIZE note · q3 cg_predict.c · confirmed · :398-406, :584
SYNC-F14 · Q3 stops predicting past cmd buffer · cg_predict.c · confirmed · :466-474 ("freeze at the last good position")
SYNC-F15 · Q3 error decay, 0.1 units, 100 ms · cg_predict.c, cg_main.c · confirmed · :410,:550-566; cg_main.c:252
SYNC-F16 · Fiedler 2004: snap >2 m, 10 %, ignore old input · gaffer networked_physics_2004 · confirmed · six quotes verbatim
SYNC-F17 · Q3 predicts item pickups, jump pads; solid list · cg_predict.c, cg_main.c · confirmed · :41,:72,:271,:303-306,:375; cg_main.c:266
SYNC-F18 · Q3 packetdup redundancy, cap 32; Fiedler redundant inputs · cl_input.c, qcommon.h, cl_main.c, gaffer lockstep · confirmed · cl_input.c:731-770; qcommon.h:121
SYNC-F19 · Q3 cmd serverTime, 16-bit angles · cl_input.c, q_shared.h · confirmed · cl_input.c:512-517; q_shared.h:1082,:1248-1254
SYNC-F20 · Source cmd tick_count, random_seed · source usercmd.h · confirmed · :139-168
SYNC-F21 · 3x send interval; 300/350/150/85 ms · gaffer snapshot_interpolation · confirmed · paras "My rule of thumb", "How can we reduce"
SYNC-F22 · extrapolation poor; Hermite removed artifacts · same · confirmed · extrapolation para; Hermite @10pps para
SYNC-F23 · Q3 clock rules 500/100/1-2 ms, extrapolate, ±30 nudge · cl_cgame.c, cg_snapshot.c · confirmed · cl_cgame.c:810-986; cg_snapshot.c:366
SYNC-F24 · no interp across teleport/first snap · cg_snapshot.c · confirmed · :217, :68, :296-297
SYNC-F25 · players lerped, missiles by trajectory · cg_ents.c · confirmed · :718, :741-762, :1004
SYNC-F26 · Source cl_extrapolate default 1 · c_baseentity.cpp · confirmed · :80
SYNC-F28 · Source rewind = tick - lerp; 200 ms trust; clamp 1 s · player_lagcompensation.cpp, inetchannelinfo.h · confirmed (notes: RTT reading rests on header comment while the call passes FLOW_OUTGOING; clamp is on the estimate, client-trusted path can exceed it by up to 0.2 s) · :34, :389-411; inetchannelinfo.h:56
SYNC-F29 · Source 1 s history with anim layers · player_lagcompensation.cpp · confirmed · LagRecord :114-126; purge :258, :290
SYNC-F30 · stop at dead record or >64-unit jump · same · confirmed · :473-482; :28
SYNC-F31 · no lag comp for bots, SP, self · same · confirmed · :363-369, :425-426
SYNC-F32 · rewind only enemies, acked, near, in 45° cone · source player.cpp · confirmed (code: near OR in cone) · :749-782
SYNC-F33 · cap on replayed cmd ticks (speed hack) · player.cpp · confirmed (default "24", unread by unit) · :103, :3392-3413
SYNC-F34 · "hit behind cover" window grows with latency · none (unit's inference; Valve wiki 403) · unsupported (no source passage) · –
SYNC-F35 · Q3 projectile trajectories, bounce, sweep, 50 ms prestep · g_missile.c, bg_misc.c, q_shared.h · confirmed · g_missile.c:25-70,:466-585; bg_misc.c:1229; q_shared.h:1271-1275
BUD-F1 · QUIC needs 1200-byte paths · RFC 9000 · confirmed · §14
BUD-F2 · no IP fragmentation, DF bit · RFC 9000 · confirmed · §14
BUD-F3 · 1280 IP minimum; 1232/1252 · RFC 9000 · confirmed · §14
BUD-F4 · 1-RTT header fields · RFC 9000 · confirmed · §17.3.1 figure
BUD-F5 · 16-byte AEAD tag · RFC 9001 · confirmed · §5.3
BUD-F6 · DATAGRAM no fragment; 0x30/0x31 LEN bit · RFC 9221 · confirmed (LEN rule verbatim) · §5, §4
BUD-F7 · datagrams delayed or dropped by CC · RFC 9221 · confirmed · §5.4
BUD-F8 · not resent; transport ack != app processed · RFC 9221 · confirmed (app-ack sentence verbatim) · §5.2
BUD-F9 · WT datagram = QSID varint + payload · RFC 9297, draft -16, RFC 9000 · confirmed · RFC 9297 §2.1; -16 §4.5; RFC 9000 §16 Table 4
BUD-F10 · maxDatagramSize runtime, tied to PMTU · W3C ED 2026-10-27 · confirmed · §5.3, §5.1
BUD-F11 · oversize write resolves undefined, not queued · W3C ED · confirmed (algorithm order shows it) · §4.3 writeDatagrams
BUD-F12 · smoothedRtt, rttVariation, estimatedSendRate · W3C ED · confirmed · §6.13
BUD-F13 · outgoingMaxAge; expired/lost counters · W3C ED · confirmed (ms units verbatim §5.1) · §6.14, §5.1
BUD-F14 · low-latency option at risk, unimplemented · W3C ED · confirmed (verbatim, unit had no quote) · §6.9
BUD-F15 · initial window 10x, min 2x, pacing · RFC 9002 · confirmed · §7.2, §7.7
BUD-F18 · fragment loss 1-p^n; 1200, 1472 · gaffer packet_fragmentation · confirmed · MTU paras; "Two/Ten fragments" list
BUD-F19 · 512/m, 50 bits per position · gaffer snapshot_compression · confirmed · position quantization para
BUD-F20 · 29-bit smallest three; no velocity at 60 Hz · same · confirmed · orientation para; "Linear interpolation is good enough at 60HZ"
BUD-F21 · 1 bit unchanged; 54 kbps; 5.5 / 26.1 bits · same · confirmed (averages verbatim) · "Incremental Improvements" onward
BUD-F22 · 15 kbps header floor; 17.37 Mbit/s raw · same · confirmed · verbatim
BUD-F23 · arithmetic coding beat bit-packing by 25 % · same · confirmed · verbatim
BUD-F24 · Q3 per-field changed bits, 13-bit floats · q3 msg.c · confirmed · :800-807, :842-844, :915, :928
BUD-F25 · PACKET_BACKUP 32; snapshot-entity ring · qcommon.h, sv_init.c · confirmed · qcommon.h:117; sv_init.c:227
BUD-F26 · Q3 defaults sv_fps 20, snaps 20, rate 3000, maxpackets 30, packetdup 1 · sv_init.c, cl_main.c, qcommon.h · confirmed · sv_init.c:596; cl_main.c:2304-2349
BUD-F27 · Q3 MAX_PACKETLEN 1400, own fragmenting · net_chan.c, qcommon.h · confirmed · net_chan.c:50-52; qcommon.h:169
BUD-F28 · Source 15 ms tick; TF2/CS:S/DoD ignore -tickrate · const.h, gameinterface.cpp · confirmed · const.h:31; gameinterface.cpp:829-851
BUD-F29 · rewind = outgoing latency + lerp, clamp 1 s; 0.2 check · player_lagcompensation.cpp · confirmed (0.2 meaning verbatim in comment :409) · :34, :389-411, :258
BUD-F30 · GNS UDP payload 1300 · GNS internal.h, csteamnetworkingsockets.cpp · confirmed · internal.h:263-266; .cpp:87
BUD-F31 · GNS send rate 256 KiB/s set by hand · steamnetworkingtypes.h, .cpp · confirmed · types.h:1235-1240; .cpp:84-85
BUD-F32 · 3 intervals; 150 ms at 30, 85 at 60 · gaffer snapshot_interpolation · confirmed (150 = 3 intervals + jitter is the unit's reading) · as SYNC-F21
BUD-F34 · priority accumulator; 256 kbit/s example · gaffer state_synchronization · confirmed (reset rule verbatim) · "Packet Structure", "Priority Accumulator"
BUD-F35 · redundant inputs; jitter buffer 4-5 frames · same · confirmed (redundancy verbatim) · "Jitter Buffer"; packet structure para
INT-F1 · Q3 per-client entity tests (flags, areas, PVS) · q3 sv_snapshot.c · confirmed · SV_AddEntitiesVisibleFromPoint
INT-F2 · Q3 1024 cap, silent discard, rate pacing · sv_snapshot.c · confirmed · :228, :269, :532-544, :588
INT-F3 · entering entity from baseline; unchanged free · sv_snapshot.c · confirmed · :89-99
INT-F4 · Q3 O(1) area connectivity, reflood on door · q3 cm_test.c · confirmed · CM_AreasConnected, CM_AdjustAreaPortalState
INT-F5 · Q2 multicast ALL/PHS/PVS; no-attenuation to all · q2 sv_send.c · confirmed · SV_Multicast, SV_StartSound
INT-F6 · Q3 events as temp entities, snapped origin · q3 g_utils.c · confirmed · G_TempEntity, G_Sound
INT-F7 · Q3 obituary broadcast, score plum single client · q3 g_combat.c · confirmed · player_die, ScorePlum
INT-F8 · Source ALWAYS/PVSCHECK/DONTSEND, team hook · source baseentity.cpp · confirmed (quotes are brace-less forms of multi-line code) · :4024-4025, :4049-4096
INT-F9 · child forces parent to be sent · baseentity.cpp · confirmed · :4118, :4133
INT-F10 · team entity always sent; observer target sent · team.cpp · confirmed · :102-115
INT-F11 · per-field team-only proxy · sendproxy.cpp · confirmed · :79-96
INT-F12 · TF2 disguise fields in shared table · tf_player_shared.cpp · confirmed · :535, :577-582, :590
INT-F13 · SourceTV tv_delay 30 s · hltvdirector.cpp · confirmed · :24, :215-216
INT-F14 · accumulator mechanics, player priority, 64 per packet · gaffer state_synchronization · confirmed · "Priority Accumulator"
INT-F15 · unchanged = 1 bit; 15 kbps stationary · gaffer snapshot_compression · confirmed · verbatim
INT-F16 · DevTools memory inspector reads JS/Wasm memory · developer.chrome.com memory-inspector (2024-09-06) · confirmed · three quotes verbatim
INT-F17 · fov vertical degrees; getViewBounds · three.js PerspectiveCamera.js (dev) · confirmed · :49-50, :227
INT-F18 · footprint 27.4/39.9 x 32.75 m, r ≈ 28 m · arithmetic on CD5 + F17 · confirmed (recomputed) · –
INT-F19 · 2.6/5.4/8.4 % of map; 90-188 kbit/s illustration · arithmetic on CD21 · confirmed (recomputed) · –
INT-F20 · 576/768/432 pairs; 2.1-4.4 ms; 4-9 % core · arithmetic on CS6/CS9 · confirmed (recomputed) · –
TRN-F3 · WT over HTTP/2 draft -15, WGLC · datatracker draft-ietf-webtrans-http2 · confirmed (fields on page) · document fields
TRN-F4 · drafts not wire-compatible; codepoints · drafts -16, -07, -02 · confirmed · -16 §7.1, §9.2; -07 App. A.1, §8.2; -02 §8.2
TRN-F5 · QUICHE has only draft-02/-07 settings · quiche http_constants.h · confirmed · :43-45
TRN-F6 · wtransport and web-transport-proto same settings · settings.rs (both) · confirmed · wtransport-proto :247-248; web-transport-proto 0.6.3 lines 77-80
TRN-F7 · all four browsers ship WT; versions, dates · BCD WebTransport.json + browsers/*.json; MDN API page · confirmed · JSON entries; MDN Baseline box
TRN-F8 · datagram writer differs per browser · BCD DatagramDuplexStream.json · confirmed · writable / createWritable entries
TRN-F9 · getStats missing Chrome/Edge, throws Firefox · BCD WebTransport.json · confirmed · getStats entry
TRN-F10 · congestionControl hint weak · BCD, MDN ctor, W3C TR · confirmed · BCD option entry; MDN options; TR §6.9
TRN-F11 · Chrome lacks requireUnreliable etc. · BCD WebTransport.json + browsers · confirmed (all rows in data) · option and method entries
TRN-F12 · Safari HTTP/2 fallback; reliability; requireUnreliable · WebKit 17862, W3C TR, MDN ctor · confirmed; location wrong: [[Reliability]] steps are TR §6.2 Constructor, not §6.5 · WebKit WebTransport para; TR §6.2, §6.9
TRN-F13 · pinning rules X.509v3, ≤2 weeks, P-256, no RSA, SHA-256 · W3C TR, MDN ctor · confirmed (SHA-256-only verbatim) · TR §6.9 custom certificate requirements
TRN-F14 · pinning for non-public/ephemeral hosts · MDN ctor · confirmed · serverCertificateHashes option text
TRN-F15 · wtransport self_signed P-256, 14 days · docs.rs wtransport 0.7.2 Identity; Cargo.toml · confirmed · Identity::self_signed; Cargo.toml:57
TRN-F16 · datagrams unreliable, CC'd, unfragmented; WT prefix · RFC 9221; draft -16 · confirmed · RFC §2, §5-§5.4; -16 §4.1, §4.5
TRN-F17 · 1200-byte floor; quinn initial_mtu 1200 · RFC 9000; docs.rs quinn TransportConfig · confirmed · §14; initial_mtu, min_mtu, mtu_discovery_config
TRN-F18 · max datagram size varies; oversize vanishes · W3C TR · confirmed · §5.1, §4.3
TRN-F19 · queues drop; maxAge support per browser · W3C TR; BCD · confirmed (BCD rows in data) · §5.1, §4.3; DatagramDuplexStream entries
TRN-F20 · Fiedler oversize/fragments; 1200, 1024, 63.4 % · gaffer packet_fragmentation · confirmed (63.4 % verbatim) · MTU paras; "Sending Packet Fragments"
TRN-F21 · WebSocket HOL; WT streams don't block · WebKit 17862 · confirmed · WebTransport paras
TRN-F22 · Fiedler UDP only, don't mix TCP/UDP · gaffer udp_vs_tcp · confirmed · body and "Conclusion"
TRN-F24 · NewReno, CUBIC, min window 2 · RFC 9002 · confirmed · §7, §7.2, §7.6, §7.7
TRN-F25 · quinn controllers, send_datagram, rtt · docs.rs quinn 0.11.12 · confirmed · congestion module; Connection; TransportConfig
TRN-F26 · sessions share CC and flow control · draft -16 · confirmed · §8
TRN-F28 · quinn 0.11.12 mature · crates.io API · confirmed · newest_version, updated_at, created_at
TRN-F29 · wtransport 0.7.2; not production-ready · crates.io, Cargo.toml, README, docs.rs · confirmed (stars/issues not checked: github.com 403) · README:31-32; Cargo.toml:13-65
TRN-F30 · web-transport-quinn 0.13.5 family, fast releases · crates.io, moq README · confirmed (release list in registry) · README "Crates"
TRN-F31 · h3 very experimental; h3-webtransport idle · h3 README; crates.io · confirmed · README:16; versions list
TRN-F32 · QMux -02 WG Document, abstract · datatracker · confirmed (fields on page) · document page, abstract
TRN-F33 · tokio-tungstenite, axum ws mature · crates.io; docs.rs axum 0.8.9 · confirmed (^0.29.0 dependency not re-checked) · extract::ws page
TRN-F34 · WebSocket in all four, in workers · BCD WebSocket.json · confirmed (binaryType in data) · WebSocket, worker_support, binaryType
TRN-F35 · WT in workers, HTTPS, explicit port · MDN WebTransport_API · confirmed · notes and constructor para
TRN-F38 · protocols option carries app version · MDN ctor; BCD · confirmed (versions in data) · protocols option
ROOM-F1 · GameLift ready/activate, one session per process · AWS gamelift-sdk-interactions · confirmed · "Initializing a game server process", "Creating a game session"
ROOM-F2 · reserve slot, validate, 60 s timeout · same + API_PlayerSession · confirmed · "Adding a player"; Status
ROOM-F3 · ProcessEnding, health 1 min, 3 strikes, 1-600 s activation · sdk-interactions, fleets-multiprocess, API_RuntimeConfiguration · confirmed · as cited; Valid Range
ROOM-F4 · density by processes per instance; 50 cap; test · fleets-multiprocess, API_RuntimeConfiguration · confirmed · intro; "Choose the number..."; description
ROOM-F5 · PlayFab StandingBy->Active, exit at end · learn.microsoft.com lifecycle · confirmed · "States of a game server", "Terminating..."
ROOM-F6 · Agones multi-session cheaper, harder; rooms Counter Beta · agones high-density-gameservers.md · confirmed · body; beta shortcode
ROOM-F7 · tokio cooperative; spawn_blocking finite; thread::spawn · docs.rs tokio 1.54.1 · confirmed (thread::spawn link verbatim) · task module; spawn_blocking
ROOM-F8 · panic ends thread unless abort; 2 MiB stack · doc.rust-lang.org std::thread; Cargo profiles · confirmed · "Panics", "Stack size"; profiles "panic"
ROOM-F9 · Fiedler connect tokens · gaffer why_cant_i_send_udp · confirmed · connect token section
ROOM-F10 · netcode 1.02 single-use tokens · mas-bandwidth netcode STANDARD.md · confirmed · server-side checks list
ROOM-F11 · JWT aud, exp leeway, jti · RFC 7519 · confirmed · §4.1.3, §4.1.4, §4.1.7
ROOM-F12 · WS ctor url+protocols; WT options list · MDN WebSocket(), WebTransport() · confirmed ("no auth header" stays inferred) · Syntax; options
ROOM-F13 · extended CONNECT, :authority/:path, rate-limit · draft -16 · confirmed · §3.2, §5.2
ROOM-F14 · ≥64 bits entropy; URL IDs leak · OWASP Session Management · confirmed · "Session ID Entropy"; URL and logging passages
ROOM-F15 · TF2 bot quota fill/match, auto vacate · tf_bot_manager.cpp · confirmed (note: default mode "normal", quota 0) · :26-29, :432
ROOM-F16 · anonymous login first, recoverable later · PlayFab login basics · confirmed · five quotes verbatim
ROOM-F17 · lobby settings at creation; owner rights; migration · PlayFab create-a-lobby, ownership-changes · confirmed except "owner alone" (source: "special permissions to update data") = unsupported part · create page settings list; ownership-changes Automatic row
ROOM-F18 · client connects to session IP and port · AWS gamelift-sdk-interactions · confirmed · introduction
Counts: 135 findings checked · confirmed 133 (one with a location error: TRN-F12) · wrong 0 · unsupported 2 (SYNC-F34 whole; ROOM-F17 the "owner alone" part).
DONE
