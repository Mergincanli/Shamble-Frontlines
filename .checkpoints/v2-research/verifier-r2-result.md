# verifier on R2: returned report (verbatim), workflow run wf_bce24fc0-5c6, 2026-10-28

VERDICT: PASS WITH NOTES. None of the 135 basis findings is wrong. Two are unsupported: SYNC-F34 in full, and one part of ROOM-F17. The ranked problems below are mostly conflicts the file already records as open questions (XU-OQs). I skipped the gate and symbol checks, as the task said.

Method: I fetched all sources with curl from allow-listed domains on 2026-10-28 and matched each quote by exact text search on the source itself. This departs from rule 3, which asks for WebFetch quotes; earlier R1 verifiers used the same method. WebFetch was not used. Two github.com repo pages returned 403. Notes, with every verdict and quote location: /home/user/Shamble-Frontlines/.checkpoints/v2-research/verifier-r2.md (ends with DONE). Downloaded copies: /tmp/claude-0/-home-user-Shamble-Frontlines/ed3ff506-9a45-5a14-adc3-75c262311890/scratchpad/r2v/dl/

Scope is the union of all "Rests on:" lists: SYNC 33, BUD 32, INT 20, TRN 32, ROOM 18. SYNC-F10/F27, BUD-F16/F17/F33 and TRN-F1/F2/F23/F27/F36/F37/F39 are in no list.

TABLE (claim id · claim · source · verdict · location)
SYNC-F1 · lockstep: latency of most-lagged player · gaffer what_every_programmer · confirmed · "Peer-to-Peer Lockstep"
SYNC-F2 · joining a lockstep game late is rare · same · confirmed · same section
SYNC-F3 · bit-exact needed; floats differ by compiler/OS/ISA · gaffer deterministic_lockstep · confirmed · verbatim
SYNC-F4 · lockstep waits for input; TCP RTT*2 stall · same · confirmed · TCP para (quote cut before "(actually…")
SYNC-F5 · snapshot interpolation needs no determinism · gaffer snapshot_interpolation · confirmed · intro
SYNC-F6 · state sync is approximate; quantise both sides · gaffer state_synchronization · confirmed · intro, "Quantize Both Sides"
SYNC-F7 · delta vs acked baseline; unchanged costs 1 bit · gaffer snapshot_compression · confirmed · delta section
SYNC-F8 · Q3 ring of 32, full-snapshot rules, 1024 cap · q3 sv_snapshot.c, qcommon.h · confirmed · sv_snapshot.c:89-158, :228; qcommon.h:117-118
SYNC-F9 · Q3 client sends last seq; asks for no-delta · cl_input.c · confirmed · :713-716, :748-750
SYNC-F11 · QUIC datagrams: no resend, reorder, drop, no fragmenting · RFC 9221 · confirmed · §5, §5.2, §5.3
SYNC-F12 · prediction ring, rewind and replay · gaffer what_every_programmer · confirmed · "Client-Side Prediction"
SYNC-F13 · Q3 replays all unacked cmds; OPTIMIZE note · cg_predict.c · confirmed · :398-406, :584
SYNC-F14 · Q3 stops predicting past its cmd buffer · cg_predict.c · confirmed · :466-474
SYNC-F15 · Q3 error decay, 0.1 units, 100 ms · cg_predict.c, cg_main.c · confirmed · :410, :550-566; cg_main.c:252
SYNC-F16 · snap at >2 m, else 10 %; ignore old input · gaffer networked_physics_2004 · confirmed · verbatim
SYNC-F17 · Q3 predicts item pickups and jump pads · cg_predict.c, cg_main.c · confirmed · :41-375; cg_main.c:266
SYNC-F18 · input redundancy, packetdup, 32-cmd cap · cl_input.c, qcommon.h, cl_main.c, gaffer lockstep · confirmed · cl_input.c:731-770
SYNC-F19 · cmd carries serverTime and 16-bit angles · cl_input.c, q_shared.h · confirmed · :512-517; q_shared.h:1082, :1248-1254
SYNC-F20 · Source cmd tick_count and seeds · usercmd.h · confirmed · :139-168
SYNC-F21 · 3x send interval; 300/350/150/85 ms · gaffer snapshot_interpolation · confirmed · "My rule of thumb…", "How can we reduce…"
SYNC-F22 · extrapolation poor; Hermite removed artifacts · same · confirmed · extrapolation para; Hermite @10pps para
SYNC-F23 · Q3 clock rules 500/100/1-2 ms, ±30 nudge · cl_cgame.c, cg_snapshot.c · confirmed · cl_cgame.c:810-986; cg_snapshot.c:366
SYNC-F24 · no interpolation across teleport or first snapshot · cg_snapshot.c · confirmed · :68, :217, :296-297
SYNC-F25 · players lerped; missiles from trajectories · cg_ents.c · confirmed · :718, :741-762, :1004
SYNC-F26 · Source cl_extrapolate defaults to 1 · c_baseentity.cpp · confirmed · :80
SYNC-F28 · rewind = tick − lerp; 200 ms trust; 1 s clamp · player_lagcompensation.cpp, inetchannelinfo.h · confirmed, with notes (see problem 3) · :34, :389-411; .h:56
SYNC-F29 · 1 s history incl. animation layers · player_lagcompensation.cpp · confirmed · :114-126, :258, :290
SYNC-F30 · stop at dead record or >64-unit jump · same · confirmed · :28, :473-482
SYNC-F31 · no lag compensation for bots, single player, self · same · confirmed · :363-369, :425-426
SYNC-F32 · rewind only filtered targets (team, acked, distance, cone) · player.cpp · confirmed (code: near OR in cone) · :749-782
SYNC-F33 · cap on replayed cmd ticks · player.cpp · confirmed (default "24") · :103, :3392-3413
SYNC-F34 · "hit behind cover" window grows with latency · none (unit's inference) · unsupported · –
SYNC-F35 · Q3 projectile trajectories, bounce, sweep, 50 ms prestep · g_missile.c, bg_misc.c, q_shared.h · confirmed · g_missile.c:25-585; bg_misc.c:1229
BUD-F1 · QUIC needs 1200-byte paths · RFC 9000 · confirmed · §14
BUD-F2 · no IP fragmentation; DF bit · RFC 9000 · confirmed · §14
BUD-F3 · 1280 minimum; 1232/1252 · RFC 9000 · confirmed · §14
BUD-F4 · 1-RTT header fields · RFC 9000 · confirmed · §17.3.1
BUD-F5 · 16-byte AEAD tag · RFC 9001 · confirmed · §5.3
BUD-F6 · no fragmenting; 0x30/0x31 LEN bit · RFC 9221 · confirmed · §5, §4
BUD-F7 · congestion control delays or drops datagrams · RFC 9221 · confirmed · §5.4
BUD-F8 · no resend; transport ack ≠ app processed · RFC 9221 · confirmed · §5.2
BUD-F9 · WT datagram = Quarter Stream ID + payload · RFC 9297, draft -16, RFC 9000 · confirmed · §2.1; -16 §4.5; Table 4
BUD-F10 · maxDatagramSize is a runtime value · W3C Editor's Draft 2026-10-27 · confirmed · §5.3, §5.1
BUD-F11 · oversize write resolves undefined, not queued · W3C ED · confirmed · §4.3 writeDatagrams
BUD-F12 · smoothedRtt, rttVariation, estimatedSendRate · W3C ED · confirmed · §6.13
BUD-F13 · outgoingMaxAge; expired and lost counters · W3C ED · confirmed · §6.14, §5.1
BUD-F14 · "low-latency" option at risk · W3C ED · confirmed · §6.9
BUD-F15 · initial window 10x, minimum 2x, pacing · RFC 9002 · confirmed · §7.2, §7.7
BUD-F18 · fragment loss 1−p^n; 1200, 1472 · gaffer packet_fragmentation · confirmed · MTU paras, fragment list
BUD-F19 · 512 per metre, 50 bits per position · gaffer snapshot_compression · confirmed · verbatim
BUD-F20 · 29-bit orientation; no velocity at 60 Hz · same · confirmed · verbatim
BUD-F21 · 1 bit, 54 kbps, 5.5 / 26.1 bits · same · confirmed · "Incremental Improvements" on
BUD-F22 · 15 kbps header floor; 17.37 Mbit/s raw · same · confirmed · verbatim
BUD-F23 · arithmetic coding 25 % better · same · confirmed · verbatim
BUD-F24 · Q3 per-field change bits, 13-bit floats · msg.c · confirmed · :800-807, :842-844, :915, :928
BUD-F25 · PACKET_BACKUP 32; entity ring · qcommon.h, sv_init.c · confirmed · :117; sv_init.c:227
BUD-F26 · Q3 default rates · sv_init.c, cl_main.c, qcommon.h · confirmed · sv_init.c:596; cl_main.c:2304-2349
BUD-F27 · MAX_PACKETLEN 1400; own fragmenting · net_chan.c, qcommon.h · confirmed · :50-52; :169
BUD-F28 · 15 ms tick; TF2/CS:S/DoD ignore -tickrate · const.h, gameinterface.cpp · confirmed · const.h:31; :829-851
BUD-F29 · rewind = outgoing latency + lerp, 1 s clamp, 0.2 s check · player_lagcompensation.cpp · confirmed · :34, :389-411
BUD-F30 · GameNetworkingSockets UDP payload 1300 · GNS internal.h, .cpp · confirmed · :263-266; .cpp:87
BUD-F31 · send rate 256 KiB/s, set by hand · steamnetworkingtypes.h, .cpp · confirmed · :1235-1240; .cpp:84-85
BUD-F32 · 3 intervals; 150 ms at 30 Hz, 85 at 60 · gaffer snapshot_interpolation · confirmed · as SYNC-F21
BUD-F34 · priority accumulator; 256 kbit/s example · gaffer state_synchronization · confirmed · "Priority Accumulator"
BUD-F35 · redundant inputs; 4-5 frame jitter buffer · same · confirmed · "Jitter Buffer"
INT-F1 · Q3 per-client entity tests · sv_snapshot.c · confirmed · :309-383
INT-F2 · 1024 cap, silent discard, rate pacing · sv_snapshot.c · confirmed · :228, :269, :532-588
INT-F3 · entering entity coded from baseline · sv_snapshot.c · confirmed · :89-99
INT-F4 · O(1) area test; reflood on door change · cm_test.c · confirmed · :381-429
INT-F5 · Q2 multicast ALL/PHS/PVS · q2 sv_send.c · confirmed · :156-363
INT-F6 · events as temp entities, snapped origin · g_utils.c · confirmed · :481-502, :607
INT-F7 · obituary broadcast; score plum to one client · g_combat.c · confirmed · :37-38, :499-504
INT-F8 · Source transmit states and team hook · baseentity.cpp · confirmed (quotes are multi-line code without braces) · :4024-4096
INT-F9 · child forces parent to be sent · baseentity.cpp · confirmed · :4118, :4133
INT-F10 · team entity always sent; observer target sent · team.cpp · confirmed · :102-115
INT-F11 · team-only field proxy · sendproxy.cpp · confirmed · :79-96
INT-F12 · TF2 disguise fields in shared table · tf_player_shared.cpp · confirmed · :535-590
INT-F13 · tv_delay 30 s · hltvdirector.cpp · confirmed · :24, :215-216
INT-F14 · accumulator mechanics · gaffer state_synchronization · confirmed · "Priority Accumulator"
INT-F15 · unchanged = 1 bit; 15 kbps · gaffer snapshot_compression · confirmed · verbatim
INT-F16 · DevTools memory inspector · developer.chrome.com (2024-09-06) · confirmed · verbatim
INT-F17 · fov is vertical; getViewBounds · three.js PerspectiveCamera.js · confirmed · :49-50, :227
INT-F18 · camera footprint, r ≈ 28 m · arithmetic · confirmed (recomputed) · –
INT-F19 · map fractions; 90-188 kbit/s · arithmetic · confirmed (recomputed) · –
INT-F20 · enemy pairs; 2.1-4.4 ms · arithmetic · confirmed (recomputed) · –
TRN-F3 · WT over HTTP/2 draft -15, in Last Call · datatracker · confirmed · document fields
TRN-F4 · draft versions not wire-compatible · drafts -16/-07/-02 · confirmed · -16 §7.1, §9.2; -07 App. A.1
TRN-F5 · QUICHE has only -02/-07 settings · quiche http_constants.h · confirmed · :43-45
TRN-F6 · both Rust stacks use -02/-07 settings · settings.rs ×2 · confirmed · :247-248; 0.6.3 lines 77-80
TRN-F7 · four browsers ship WT; versions and dates · BCD, browsers/*.json, MDN · confirmed · JSON entries
TRN-F8 · datagram writer differs per browser · BCD DatagramDuplexStream · confirmed · writable / createWritable
TRN-F9 · getStats missing on Chrome/Edge, throws on Firefox · BCD WebTransport.json · confirmed · getStats entry
TRN-F10 · congestionControl hint weak · BCD, MDN, W3C TR · confirmed · TR §6.9
TRN-F11 · Chrome lacks newer options · BCD · confirmed · option entries
TRN-F12 · Safari HTTP/2 fallback; requireUnreliable · WebKit 17862, TR, MDN · confirmed; location wrong (TR §6.2, not §6.5) · TR §6.2, §6.9
TRN-F13 · certificate pinning rules · TR, MDN · confirmed · TR §6.9
TRN-F14 · pinning for non-public hosts · MDN WebTransport() · confirmed · serverCertificateHashes
TRN-F15 · wtransport self-signed P-256, 14 days · docs.rs 0.7.2, Cargo.toml · confirmed · Identity; :57
TRN-F16 · datagrams unreliable, congestion-controlled, unfragmented · RFC 9221, draft -16 · confirmed · §2-§5.4; -16 §4.1, §4.5
TRN-F17 · 1200-byte floor; quinn initial_mtu · RFC 9000, docs.rs quinn · confirmed · §14; TransportConfig
TRN-F18 · size limit varies; oversize vanishes · W3C TR · confirmed · §5.1, §4.3
TRN-F19 · queues drop; maxAge support · TR, BCD · confirmed · §5.1, §4.3
TRN-F20 · Fiedler on fragments, 63.4 % · gaffer packet_fragmentation · confirmed · verbatim
TRN-F21 · WebSocket head-of-line blocking; WT streams · WebKit 17862 · confirmed · WebTransport paras
TRN-F22 · UDP only; don't mix TCP/UDP · gaffer udp_vs_tcp · confirmed · body, "Conclusion"
TRN-F24 · NewReno, CUBIC, minimum window · RFC 9002 · confirmed · §7, §7.2, §7.6, §7.7
TRN-F25 · quinn controllers and datagram API · docs.rs quinn 0.11.12 · confirmed · congestion, Connection
TRN-F26 · sessions share congestion control · draft -16 · confirmed · §8
TRN-F28 · quinn 0.11.12 registry data · crates.io · confirmed · API fields
TRN-F29 · wtransport 0.7.2; "not … production-ready" · crates.io, Cargo, README, docs.rs · confirmed (stars not checked) · README:31-32
TRN-F30 · web-transport-quinn family, fast releases · crates.io, moq README · confirmed · "Crates" list
TRN-F31 · h3 "still very experimental" · h3 README, crates.io · confirmed · README:16
TRN-F32 · QMux -02 · datatracker · confirmed · page fields, abstract
TRN-F33 · tokio-tungstenite and axum ws · crates.io, docs.rs · confirmed · extract::ws
TRN-F34 · WebSocket everywhere, in workers · BCD WebSocket.json · confirmed · entries
TRN-F35 · WT in workers, HTTPS, explicit port · MDN WebTransport_API · confirmed · notes
TRN-F38 · protocols option · MDN, BCD · confirmed · protocols option
ROOM-F1 · GameLift process/session model · AWS sdk-interactions · confirmed · "Creating a game session"
ROOM-F2 · slot reservation, 60 s timeout · same, API_PlayerSession · confirmed · Status
ROOM-F3 · ProcessEnding, health, activation 1-600 s · AWS ×3 · confirmed · as cited
ROOM-F4 · several processes per instance; 50 cap · fleets-multiprocess, RuntimeConfiguration · confirmed · intro
ROOM-F5 · PlayFab server lifecycle · learn.microsoft.com · confirmed · "States of a game server"
ROOM-F6 · Agones high density · agones md · confirmed · body, beta shortcode
ROOM-F7 · tokio cooperative tasks; thread::spawn · docs.rs tokio 1.54.1 · confirmed · task, spawn_blocking
ROOM-F8 · panic and stack size · Rust std::thread, Cargo profiles · confirmed · "Panics", "Stack size"
ROOM-F9 · Fiedler connect tokens · gaffer why_cant_i_send_udp · confirmed · token section
ROOM-F10 · netcode 1.02 single-use tokens · netcode STANDARD.md · confirmed · server checks
ROOM-F11 · JWT aud/exp/jti · RFC 7519 · confirmed · §4.1.3, §4.1.4, §4.1.7
ROOM-F12 · constructor signatures and options · MDN ×2 · confirmed ("no auth header" stays inferred) · Syntax, options
ROOM-F13 · extended CONNECT; rate limiting · draft -16 · confirmed · §3.2, §5.2
ROOM-F14 · 64-bit entropy; URL leaks · OWASP session cheat sheet · confirmed · "Session ID Entropy"
ROOM-F15 · TF2 bot quota · tf_bot_manager.cpp · confirmed (fill is not the default) · :26-29, :432
ROOM-F16 · anonymous login first · PlayFab login basics · confirmed · verbatim
ROOM-F17 · lobby settings and owner · PlayFab lobby ×2 · confirmed except "owner alone", which is unsupported (source says "special permissions to update data") · create page; ownership-changes
ROOM-F18 · connect to session IP and port · AWS sdk-interactions · confirmed · introduction

COUNTS: 135 checked · confirmed 133 (TRN-F12 has a wrong location) · wrong 0 · unsupported 2 (SYNC-F34 in full; ROOM-F17 one part)

RECOMMENDATIONS WITH WRONG OR UNSUPPORTED BASIS CLAIMS
- R2-SYNC-REC5: SYNC-F34 (unsupported).
- R2-ROOM-REC5: ROOM-F17, the "owner alone" part. The REC says "sole write rights".
- All other 34 recommendations: none.

OTHER FINDINGS, RANKED
1. BUD-REC4 and BUD-REC5 depend on browser stats that 3 of 4 target browsers lack. They use smoothedRtt, rttVariation, estimatedSendRate, lostOutgoing and expiredOutgoing (BUD-F12, F13). All of these come only from transport.getStats(); W3C ED §6.13 has `required WebTransportDatagramStats datagrams;` inside WebTransportConnectionStats. TRN-F9 (confirmed) shows getStats missing on Chrome and Edge and throwing on Firefox. Neither REC lists TRN-F9, so as written they work only on Safari 26.4+. Only outgoingMaxAge works on all three engines. XU-OQ10 records the conflict.
2. BUD-REC1 and BUD-REC4 treat "one-way latency + interpolation" as Source's formula, but the code doesn't say one-way.
   - The code calls GetLatency(FLOW_OUTGOING), and the only definition read says "current latency (RTT)" (inetchannelinfo.h:56). One-way is the table A assumption.
   - Read as RTT, 30 Hz gives a 200–250 ms rewind and breaks the ~200 ms cap that BUD-REC1 uses to reject 20 Hz.
   - The cap itself comes from the default principles (netcode-architect.md:35-36), not from a source. XU-OQ1 and OQ2 record this.
3. SYNC-REC5 has two problems.
   - It rests on SYNC-F34, an inference with no source, and its pass mark 2 depends on it.
   - In Source the 1 s clamp applies only to the server's own estimate. A trusted client tick can land up to 0.2 s past it (player_lagcompensation.cpp:399-411). SYNC-REC5 keeps the 200 ms trust rule and adds a 250 ms cap, but does not say the cap applies to the final view time. Without that, the effective rewind can reach about 450 ms.
4. Weak links to VISION_V2 §6.
   - BUD-REC3, BUD-REC4, BUD-REC5 and TRN-REC6 name no §6 item (XU-OQ18 says so).
   - INT-REC7 adds a spectator feature; its cited §6.1 and §6.8 do not name spectators, and §6.8's replays are [LATER]. XU-OQ18 does not list it.
   - INT-REC4 cites §6.9, which is a renderer budget, not a network need.
5. TRN-REC3 claims that "none of the crates read" serve WebTransport over HTTP/2. No finding in its Rests on list states this.
6. ROOM-REC5's "sole write rights" overstates F17. Also, only ownerMigrationPolicy is said to be unchangeable after creation.
7. ROOM-REC6 says "a shipped game keeps servers full this way". TF2's fill mode is opt-in: by default tf_bot_quota is "0" and the mode is "normal" (:26-27). It also counts total players, not players per team.
8. BUD-REC4's "server input buffer of 1–3 ticks" has no source and is not marked proposed in the recommendation text. F35's "4-5 frames @ 60HZ" is a receive-side buffer for state packets, not a server input buffer.
9. TRN-REC4's Serves line uses TRN-F34 (WebSocket in workers), but its Rests on list omits it.
10. Many parts the units marked "inferred" are verbatim in the source and can be upgraded:
    - SYNC: F33 (default 24); SYNC-REC1's full-snapshot path (sv_snapshot.c:62-63).
    - BUD: F6 (LEN bit), F8, F11, F13 (units), F14, F21 (averages), F34 (reset rule), F35 (redundancy).
    - TRN: F3, F11, F13 (SHA-256), F19, F20 (63.4 %), F30, F32, F34, F38.
    - ROOM: F7.
11. Pass marks: every pass-mark block is labelled "(proposed)"; none has an unmarked, unsourced number. Every sourced number I checked in the recommendations matches its source. Legacy numbers match the legacy docs, and the worked estimate's 146 kbps and 1171 B arithmetic is right.

NOT VERIFIED
- The star and issue counts in TRN-F29 (github.com returned 403); no recommendation uses them.
- axum's tokio-tungstenite ^0.29.0 dependency in TRN-F33.
- The Valve wiki and Overwatch talks: unreadable (403, video only), as the file already says.