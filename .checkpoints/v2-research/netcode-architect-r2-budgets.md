Task: R2 unit "budgets" (tick rate and bandwidth budgets), v2 phase 0 research wave 1, netcode-architect. Started 2026-10-27.

## Progress
- (start) notes created; next: read plan.md, brief, VISION_V2, README, inventory rows.
- (done) read plan.md, plan-amendments.md, brief §0 §4 §5, VISION_V2 §3 §4 §6, research README, LOOP_V2 §1-3 §9.
  VISION §3: 2 teams 2-8 players, 3 teams 6-18 players (totals, ambiguous: per team?), goal 12v12v12 (36) or 24v24 (48). LOOP_V2 §2: 12v12v12 to 16v16v16 (48), squads of 4, bots fill.
  VISION §4: fair up to ~100 ms ping, one EU region. §6: 1 modes as data, 2 true 3D (floors, aim at height), 4 vehicles first-class (multi-part, seats OPEN), 5 economy server-side, 6 telemetry, 8 moments.
## Legacy facts (inventory rows, tuning tables)
- CS1: FIXED_DT 1/60 unused; BTN 7 bits (FIRE1 ADS2 RELOAD4 MELEE8 GRENADE16 SHIELD32 SPRINT64); makeInputCommand(seq,tick,move,aim,buttons). Loop variable dt clamp 0.05 (game:3053).
- CS8: SIM record schema players(id,pos,alive,stance,team) + statics (kind,id,alive,box,rotY). Stance always 0.
- CD4/GP3: walk 6.3 m/s, accel 14, gravity 24, jumpV 11, per-unit speedMult up to 1.5 (warrior) and sprintMult up to 1.62; arena half 186 x 126 (ground 372 x 252 m, CD21). Max ground speed ~6.3*1.5*1.62=15.3 m/s (banzai 1.3x on top -> ~19.9?) [check: banzai speed game:2774].
- CD5: camera height 40, boom 56.6, fov 15 deg (narrow view; interest unit).
- CM4: DM_Tri bases 92-106 m from centre; perfbots 48 soldiers combat 0.80-1.00 ms per update step (JS legacy, mean), p95 0.97-1.54, 3 reps.
- GC2: 12 fixed slots today. GE6: 1 harvester per team (900 HP). CD14/GC17: 1 vehicle type Juggernaut (pilot 5.5 m/s, 3 shells 80 ms apart, 130 m). 
- GC14: frags fuse 1.5 s, max 3 carried; molotov fuse 1.4 s. GC15: Torque Bow bolt fuse 0.55 s, swept ray per step.
- WEAPONS (LEGACY_TUNING): fastest fireInterval shabzon 0.072 s (~13.9/s), lancer burst 3 x 0.06; bagpipe 8 pellets; amk range 300 m.
- LOOP_V2 §9 map size OPEN, "start near Cabana's footprint" (372 x 252).
Next: sources. Order: Valve wiki (Source MP networking, lag comp paper, networking entities), Gaffer (snapshot compression, snapshot interpolation, state sync, packet fragmentation, serialization), RFC 9000/9001/9221/9297/8899/8200, webtrans-http3 draft, W3C WebTransport, MDN, RFC 6455, Quake 3 source (github), GameNetworkingSockets (github), GDC vault Overwatch page.
## Sources
- S1 developer.valvesoftware.com/wiki/Source_Multiplayer_Networking: NOT READ, HTTP 403 twice (also ?action=raw). Try once more later / list as not read (blocked 403).
- S2 https://gafferongames.com/post/snapshot_compression/ (Glenn Fiedler, "Snapshot Compression", page date 2015-01-04), read 2026-10-27, tier other-expert (Fiedler; brief names him), confirmed in source:
  901 cubes, 60 snapshots/s, uncompressed 17.37 Mbps (321 bits/cube: quat 128, vel 96, pos 96, bool 1); 32-byte header est; target 256 kbps.
  smallest-three quat: 2 + 3x9 = 29 bits. velocity bounded 32 m/s 11 bits/comp then DROPPED ("linear interpolation good enough at 60 Hz").
  position 512 values/m (~2 mm): x,y 18 bits over [-256,255], z 14 bits over [0,32] = 50 bits. state 80 bits.
  delta vs last acked baseline: unchanged = 1 bit; changed-bit for 901 cubes = 54 kbps at 60 pps; indices 10-bit when <=90 changed; relative index avg 5.5 bits;
  relative position avg 26.1 bits (abs 50); relative orientation avg 23.3 bits. Steady state stationary ~15 kbps = own header (uint16 seq, uint16 base, bool) + IP+UDP 28 bytes.
  No final kbps number stated. Arithmetic coding (Giesen) beat bit-packing by 25%. No MTU statement.
- S3 https://gafferongames.com/post/snapshot_interpolation/ (Fiedler, "Snapshot Interpolation", page date 2014-11-30), read 2026-10-27, other-expert, confirmed:
  uncompressed cube 225 bits; 60 pps = 11.6 Mbit/s; 10 pps ~2 Mbit/s. Rule: buffer lets you "lose two packets in a row"; "delay that works best at 2-5% packet loss is 3X the packet send rate" (i.e. 3 send intervals) + jitter 1-2 frames @60 -> 10 pps used 350 ms; "30 snapshots per-second ... same packet loss protection with a delay of 150ms"; "60 packets per-second needs only 85ms". Hermite needs linear velocity (bandwidth up slightly); slerp ok w/o angular velocity. Extrapolation poor. Snapshots over UDP, no retransmit.
- S4 https://gafferongames.com/post/state_synchronization/ (Fiedler, "State Synchronization", page date 2015-01-05), read 2026-10-27, other-expert, confirmed:
  "typically you have some maximum bandwidth you want to support like 256kbit/sec", adjustable on the fly; per-packet byte budget = bandwidth target minus header; priority accumulator (add priority each frame, sort, fill packet until budget, reset sent ones to 0; skipped rise); MaxInputsPerPacket 32, MaxStateUpdatesPerPacket 64 of 901 cubes; redundant inputs in every packet; jitter buffer "say 4-5 frames @ 60HZ" for state sync; sim 60 Hz both sides. Precision 4096 pos values/m, 15 bits/quat comp (state sync needs more precision than snapshot interp).
- S5 https://gafferongames.com/post/packet_fragmentation_and_reassembly/ (Fiedler, page date 2016-09-06), read 2026-10-27, other-expert, confirmed:
  real-world IPv4 MTU 1500 (2016); IPv4 standard minimum 576; "conservative lower bound on the IPv4 Internet today" 1200 bytes; typical drop/fragment threshold ~1472; any lost fragment loses the packet: P(loss)=1-p^n; at 1% loss: 2 fragments ~2%, 10 ~9.5%. His own fragments 1024 bytes, max 256 per packet.
- S6 https://www.rfc-editor.org/rfc/rfc9000.html (RFC 9000 QUIC, May 2021), read 2026-10-27, official spec, confirmed:
  §14 "QUIC MUST NOT be used if the network path cannot support a maximum datagram size of at least 1200 bytes." "UDP datagrams MUST NOT be fragmented at the IP layer." IPv4 DF bit MUST be set if possible. "QUIC assumes a minimum IP packet size of at least 1280 bytes" -> max datagram 1232 B IPv6, 1252 B IPv4 (min headers 40/20 + UDP 8).
  §14.2 PMTU = whole IP packet incl IP+UDP headers. §14.3 BASE_PLPMTU consistent with smallest allowed max datagram size (RFC 8899 §5.1). §18.2 max_udp_payload_size default 65527, values below 1200 invalid.
  §17.3.1 1-RTT header: 1 byte flags (form, fixed, spin, reserved 2, key phase, PN length 2) + DCID 0..160 bits + PN 8..32 bits + payload. §16 varint 1/2/4/8 bytes: 6/14/30/62 usable bits.
- S7 https://www.rfc-editor.org/rfc/rfc9221.html (RFC 9221 QUIC DATAGRAM extension, March 2022), read 2026-10-27, official, confirmed:
  §3 max_datagram_frame_size (incl type+length+payload), RECOMMENDED 65535 = any frame that fits a packet; §4 type 0x30/0x31, Length present only if LEN bit; §5 "DATAGRAM frames cannot be fragmented"; max_udp_payload_size and PMTU limit size further; app protocol handles too-large. §5.4 congestion controlled: sender "MUST either delay sending the frame until the controller allows it or drop the frame". §5.2 not retransmitted, ack-eliciting; sender MAY tell app of suspected loss (not exact); ack != processed -> app needs its own acks.
