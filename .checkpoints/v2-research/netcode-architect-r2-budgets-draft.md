# R2 unit budgets: tick rate and bandwidth budgets

Brief question (quoted): "Snapshots, client prediction, interpolation and lag compensation for 48+ players; **tick rate
and bandwidth budgets**; interest management for a top-down camera; ..." (`docs/V2_PHASE0_BRIEF.md` §4, R2 row; this
unit answers the bold part). Target it serves: "Matches feel fair up to about 100 ms of ping. Servers start in one EU
region." (`docs/VISION_V2.md` §4, `[DEFAULT]`). Scale: "two teams with 2–8 players, and three teams with 6–18 players"
to start; "The goal: 12v12v12 or 24v24" (VISION_V2 §3, `[DECIDED]`); LOOP_V2 §2: "12v12v12 to 16v16v16, possibly more,
in squads of 4. Bots fill empty slots."

Author: netcode-architect (Opus, xhigh). Date: 2026-10-27. Status: unit draft, not verified. All pass marks and
reference hardware here are **proposed**.

## Findings

Tiers: **official** (specifications, standards drafts) · **shipped game** (source code or docs of a shipped game or of
a shipping vendor's own library) · **other** (expert articles; Glenn Fiedler's articles are named by the brief but are
not a shipped game's own documentation). "Confirmed in the source" means the quoted sentence or code line was returned
verbatim by the fetch; "inferred" means the claim rests on the fetch tool's summary or on arithmetic from quoted facts.

### Transport limits (packet size, overhead, congestion)

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

### Encoding costs (quantization, delta compression, bit-packing)

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

### What shipped games and libraries publish about rates and sizes

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

### Buffers and rates for fairness

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

## Legacy facts

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

## Options

### A. Simulation tick and snapshot rate (with R1's tick unit)

| Option | Tick / snapshot / input send | Fairness at 100 ms RTT (F29, F32) | Downstream at 48 (table D) | Server CPU | Fit |
|---|---|---|---|---|---|
| T1 | 60 Hz / 30 Hz / 60 Hz | interp 100–150 ms; rewind 50 + 100..150 = 150–200 ms, within the ~200 ms cap of the netcode default principles (`.claude/agents/netcode-architect.md`:35-36) | 146 kbps | 2 × T2 for the step; snapshot encode at 30 Hz | Legacy `FIXED_DT`, Source's 66.7 Hz tick (F28); input timing error ±8 ms |
| T2 | 30 Hz / 30 Hz / 30 Hz | same interp and rewind as T1; +8 ms average input wait | 146 kbps | lowest step cost; the brief's own pass-mark example uses 30 Hz | Fire intervals quantize harder (WEAPONS row) |
| T3 | 60 Hz / 60 Hz / 60 Hz | interp 50–85 ms; rewind 100–135 ms | 286 kbps | encode 2 × T1 | Over Fiedler's 256 kbps example budget (F34) at 48 players |
| T4 | 20 Hz / 20 Hz / 30 Hz (Quake III defaults, F26) | interp 150–200 ms; rewind 200–250 ms: breaks the 200 ms cap at 100 ms RTT | 99 kbps | lowest | 1999 defaults sized for modems ("rate" 3000 B/s) |

Rewind = one-way latency (assumption: half of a symmetric 100 ms RTT) + interpolation delay, the Source formula (F29).
Interpolation delay = 3 snapshot intervals, plus up to one interval of jitter allowance (F32).

### B. Downstream encoding (each step applies on top of the previous)

| Option | What it is | 48 players at 30 Hz (table D) | Complexity / risk |
|---|---|---|---|
| E1 | Raw floats, full state every snapshot | about 536 kbps, 2 datagrams per snapshot | trivial; every snapshot fragmented in two |
| E2 | Quantized (centimetres, 16-bit angles), full state | about 251 kbps; a full snapshot is about 1021 bytes, close to the 1171-byte limit (over it with live projectiles or at 64 players) | low |
| E3 | E2 + delta against the last acknowledged snapshot, lists of changed entities, projectiles as spawn events + deterministic flight | about 146 kbps; about 609 bytes per snapshot | medium: per-client baseline ring (Quake III keeps 32, F25), application acks (F8) |
| E3b | E3, but projectiles re-sent in every snapshot | about 188 kbps | medium; needs no deterministic projectile flight on the client |
| E4 | E3 + arithmetic (range) coding | about 25 % below E3 by Fiedler's single data point (F23) | high: a context model to build and tune; defer until E3 is measured |

### C. Upstream (inputs)

| Option | What it is | Per client (IPv4) | Notes |
|---|---|---|---|
| U1 | Send every 60 Hz tick: newest input + 3 previous, 8 bytes each | 44.6 kbps | an input is lost only if 4 packets in a row are lost (independent loss at 5 %: 0.05^4 ≈ 6 × 10⁻⁶, arithmetic) |
| U2 | U1 with the 3 repeats delta-coded against the newest (about 2 bytes each, assumption) | 36.0 kbps | small extra code |
| U3 | Send at 30 Hz, two new ticks + two repeats (like Quake III's `cl_maxpackets` 30 with `cl_packetdup` 1, F26) | 22.3 kbps | adds up to one tick of input delay on half the inputs |

### D. Packet size

| Option | What it is | Trade-off |
|---|---|---|
| P1 | Every datagram at most min(runtime max datagram size, 1150 bytes of WebTransport payload); a snapshot that does not fit is cut into several datagrams by entity groups, each one decodable alone | no reassembly; a lost datagram loses only its entities (no 1 − p^n effect, F18); needs per-group baselines |
| P2 | Application-level fragmentation and reassembly of larger snapshots (Quake III style, F27) | simplest encoder; loss multiplies with fragment count (F18) |
| P3 | Grow to a probed path MTU (Quake III 1400, Valve 1300 UDP payload: F27, F30) | fewer splits; depends on the server stack's PMTU discovery and on the browser's maxDatagramSize (F10) |
| P4 | Join and full-resync snapshots over a reliable stream; datagrams carry only deltas | the large message never touches the datagram limit; TCP-like head-of-line wait only at join |

### E. Server CPU per tick (with R1's tick unit)

| Item | Budget idea (proposed) | Evidence |
|---|---|---|
| Whole tick at 60 Hz (16.7 ms) | ≤ 4 ms p99 for 48 players, about 25 % of the interval, leaving room for bursts and for several rooms per core | assumption; no source read gives a server tick budget |
| Whole tick at 30 Hz (33.3 ms) | ≤ 8 ms p99 | assumption |
| Simulation step | R1's tick unit | legacy JS 0.80–1.00 ms mean per step at 48 soldiers (CM4) |
| Snapshot encode, 48 clients | 10–50 µs per client per snapshot, so 0.5–2.4 ms per snapshot round | assumption: to be measured |
| Lag-compensation history | 1 s × 60 ticks × 48 soldiers × 64 bytes ≈ 184 KB per room; about 72 rewound shots per second at 48 players | Source keeps 1 s (F29); bytes per pose and shots per second are assumptions |

### Worked bandwidth estimate

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

## Recommendations

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

## Open questions

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

## Sources read

| URL | Title | Version or page date | Read |
|---|---|---|---|
| https://www.rfc-editor.org/rfc/rfc9000.html | RFC 9000, QUIC: A UDP-Based Multiplexed and Secure Transport | May 2021 | 2026-10-27 |
| https://www.rfc-editor.org/rfc/rfc9001.html | RFC 9001, Using TLS to Secure QUIC | May 2021 | 2026-10-27 |
| https://www.rfc-editor.org/rfc/rfc9002.html | RFC 9002, QUIC Loss Detection and Congestion Control | May 2021 | 2026-10-27 |
| https://www.rfc-editor.org/rfc/rfc9221.html | RFC 9221, An Unreliable Datagram Extension to QUIC | March 2022 | 2026-10-27 |
| https://www.rfc-editor.org/rfc/rfc9297.html | RFC 9297, HTTP Datagrams and the Capsule Protocol | August 2022 | 2026-10-27 |
| https://datatracker.ietf.org/doc/draft-ietf-webtrans-http3/ | WebTransport over HTTP/3 | draft-ietf-webtrans-http3-16, 2026-07-06 | 2026-10-27 |
| https://w3c.github.io/webtransport/ | WebTransport (W3C Editor's Draft) | 2026-10-27 | 2026-10-27 |
| https://www.rfc-editor.org/rfc/rfc6455.html | RFC 6455, The WebSocket Protocol | December 2011 | 2026-10-27 |
| https://www.rfc-editor.org/rfc/rfc8446.html | RFC 8446, TLS 1.3 | August 2018 | 2026-10-27 |
| https://gafferongames.com/post/snapshot_compression/ | Snapshot Compression (Glenn Fiedler) | 2015-01-04 | 2026-10-27 |
| https://gafferongames.com/post/snapshot_interpolation/ | Snapshot Interpolation (Glenn Fiedler) | 2014-11-30 | 2026-10-27 |
| https://gafferongames.com/post/state_synchronization/ | State Synchronization (Glenn Fiedler) | 2015-01-05 | 2026-10-27 |
| https://gafferongames.com/post/packet_fragmentation_and_reassembly/ | Packet Fragmentation and Reassembly (Glenn Fiedler) | 2016-09-06 | 2026-10-27 |
| https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/qcommon/qcommon.h | Quake III Arena source: qcommon.h | id Software, GPL v2+, master | 2026-10-27 |
| https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/qcommon/net_chan.c | Quake III Arena source: net_chan.c | master | 2026-10-27 |
| https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/qcommon/msg.c | Quake III Arena source: msg.c | master | 2026-10-27 |
| https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/server/sv_init.c | Quake III Arena source: sv_init.c | master | 2026-10-27 |
| https://raw.githubusercontent.com/id-Software/Quake-III-Arena/master/code/client/cl_main.c | Quake III Arena source: cl_main.c | master | 2026-10-27 |
| https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/player_lagcompensation.cpp | Source SDK 2013: player_lagcompensation.cpp | master | 2026-10-27 |
| https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/public/const.h | Source SDK 2013: const.h | master | 2026-10-27 |
| https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/server/gameinterface.cpp | Source SDK 2013: gameinterface.cpp | master | 2026-10-27 |
| https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/client/c_baseentity.cpp | Source SDK 2013: c_baseentity.cpp (read; `cl_interp` not in it) | master | 2026-10-27 |
| https://raw.githubusercontent.com/ValveSoftware/source-sdk-2013/master/src/game/shared/shareddefs.h | Source SDK 2013: shareddefs.h (read; no tick default in it) | master | 2026-10-27 |
| https://raw.githubusercontent.com/ValveSoftware/GameNetworkingSockets/master/include/steam/steamnetworkingtypes.h | GameNetworkingSockets: steamnetworkingtypes.h | master | 2026-10-27 |
| https://raw.githubusercontent.com/ValveSoftware/GameNetworkingSockets/master/src/steamnetworkingsockets/clientlib/csteamnetworkingsockets.cpp | GameNetworkingSockets: csteamnetworkingsockets.cpp | master | 2026-10-27 |
| https://raw.githubusercontent.com/ValveSoftware/GameNetworkingSockets/master/src/steamnetworkingsockets/steamnetworkingsockets_internal.h | GameNetworkingSockets: steamnetworkingsockets_internal.h | master | 2026-10-27 |

## Sources not read

| URL | Reason |
|---|---|
| https://developer.valvesoftware.com/wiki/Source_Multiplayer_Networking | HTTP 403: the site refused the fetcher (twice, also the raw form) |
| https://developer.valvesoftware.com/wiki/Latency_Compensating_Methods_in_Client/Server_In-game_Protocol_Design_and_Optimization | HTTP 403: the site refused the fetcher |
| https://gdcvault.com/play/1024001/-Overwatch-Gameplay-Architecture-and | redirected to gdconf.com (outside the allow list; vault maintenance page); the talk is video only |
| https://edgegap.com/blog/game-backend-deep-dive-overwatch-2016-netcode-architecture-rollback | outside the allow list (search lead only) |
| https://www.pcgamesn.com/overwatch/overwatch-netcode | outside the allow list (search lead only) |
| https://gamesbeat.com/overwatch-netcode-tick-rate-60hz/ | outside the allow list (search lead only) |

## Cross-unit notes

- **R1 tick unit:** REC1's 60 Hz preference and REC6's whole-tick budget depend on its step cost; weapon cooldowns
  need fractional-tick carry at either rate (`WEAPONS` row); swept projectile tests run per tick (GC15).
- **R1 determinism unit:** E3 sends projectiles as spawn events only if the client can fly them identically (OQ8).
- **Snapshot, prediction and lag-compensation unit:** uses REC4's numbers (100–150 ms interpolation, 200 ms rewind cap,
  1 s history); owns Hermite against linear (OQ7) and the baseline ring.
- **Interest-management unit:** the estimate is all-in-view; the interest cut scales only the per-entity part and the
  events (about 85 % of the 48-player snapshot: (352.5 + 141 + 25) / 609 B); camera numbers in CD5.
- **WebTransport stack and WebSocket fallback unit:** connection-ID and packet-number lengths (1157–1171 B usable),
  the server-side max datagram size API, congestion controller choice (OQ10), ACK frequency; fallback overhead per
  message (TCP/IP plus TLS 22 B plus WebSocket 2–8 B: F16, F17) and head-of-line blocking.
- **Rooms unit:** REC6's per-room CPU and memory give rooms per core.
- **R6:** about 7 Mbit/s egress and 3.2 GB per hour per 48-human room at E3 (all in view); IPv6 share (OQ11).
- **R5:** the network simulator runs (0–200 ms, jitter, 0–5 % loss) behind REC1–REC5's pass marks.
- **game-designer:** caps on vehicles and projectiles (OQ5); the VISION_V2 §3 team-size reading (OQ6).
