# The netcode proof test, re-planned under NPT-TRIM (draft for the advisor's one review, then Mustafa)
2026-11-22 00:2x Amsterdam. His decision: history.md:7561-7593. Facts: replan-marks.md (the 81-mark table, test-engineer)
and replan-units.md (the units, netcode-architect); both read only, their key claims checked in the code.

## The one screen
**The question, his words:** "does our server carry 48 players at 30 ticks a second within budget on the planned
machine, and do WebTransport and the WebSocket fallback hold up from real browsers over real networks."

**Kept units** (added lines: estimate, then likely at the 1.35-1.5 growth units have shown; one build lane for Rust)
| Unit | What it builds | Size | Needed by |
|---|---|---|---|
| U4f2 (finishing) | the forged-command test; FP-2 done; review round 2 now, then the check runs, merge, -21's 49 runs | built, about 2,643 | -21 (here) |
| U6a | server core: tick loop and timers, the server binary, instrumentation and run header | 1,675 (2,250-2,500) | R1 |
| U6b | transports: WebTransport on one crate, WebSocket with fix 1, the test clients, the certificate at start | 2,205 (2,950-3,300) | R1 |
| U6c | the network probe, the load generator and calibration, the page host, certificate issue | 1,950 (2,650-2,950) | R1 |
| U7 | plain browser client: connects on both transports, presses, snapshots, counters (Rust 360 on the lane, TypeScript 1,080 off it) | 1,450 (1,950-2,200) | R1's browser legs |
| U8 | run package without Safari: cloud-init, firewall, R1's run plan, operator files, wrappers, browser scripts (no Rust) | 1,360 (1,850-2,050) | Stop B, R1 |
| MOVED | the judge and marks.json report a moved mark or part as MOVED, never PASS (reviewer's read) | 360-720 | R1's report |
| U4k | U4's kept server rows: recovery after a loss burst and the ack fix, the loss-burst rows, counter wraps, the stress-scene hit rule | 540-1,280 (800-1,900) | R2, R3, R5 |
| U6d | room work: interest writers, the join message change (C-U6-13, -14, -16, -19), the room's cheap wins, rate adaptation | 1,800 (2,450-2,700) | R2, R3 |
| U6e | control socket, metrics and logs, series writers (lag compensation's two among them), several rooms per VM, -30's runner | 1,575-1,825 (2,150-2,750) | R2, R3, R5 |
| U6j | the judge's figures for R2, R3 and R5 | 1,650-3,050 | R2-R5 verdicts |
In all about 14,600-17,300 estimated, each unit under the 6,000 stop at 1.5x. Before R1: U6a, U6b, U6c, U7, U8.

**Runs** (plan.md §7.2's blocks, trimmed; prices read 2026-11-05: run server CCX23 EUR 0.1386 an hour, load machine
CAX21 EUR 0.0176; cap = planned hours plus one spare hour per VM, rounded up to EUR 0.10; each run plan comes to you)
| Run | Kept blocks | Server h | Load h | Planned | Cap |
|---|---|---|---|---|---|
| R1 | bring-up, certificate, headers, network probe, calibration, room start, four-browser smoke, ping, home path; plus the QUIC-blocked fallback and the dev certificate (moved in from R4); two self-tests still unsized | 4 | 4 | 0.62 | 0.80 |
| R2 | whole: server budgets at 48 (tick, snapshot, interest, bandwidth, memory, logging) | 11 | 11 | 1.72 | 1.90 |
| R3 | behaviour over real networks, without the rewind rows (-48, -49) and -54 | 10 | 10 | 1.56 | 1.80 |
| R5 | rooms per VM at N = 48, three times of day, and the 8-room soak; the N = 36 search goes | 17 | 56 | 3.34 | 3.60 |
| **All** | (was R1-R7: EUR 13.51 planned, 14.90 in caps) | **42** | **81** | **7.25** | **8.10** |
R5's load hours are rough (its extra load machines served the N = 36 search too); every run plan fixes its own. Your
part per run: create the VMs (plan §7.3) and one Cloudflare TXT record per certificate order (two in R1, about 2
minutes each); present in R1 for about 2 hours.

**R1's date: planned Thursday 3 December; Wednesday 2 December holds only with no slack.**
1. R1 needs three server pieces and the client's Rust half in series on one build lane, then Stop B: about 8,350-9,250
   likely lines in 7 days, about 1,300 merged a day, against about 1,175 a day merged last week.
2. Spend-limit cuts took 1.5 to 4 hours on most recent days (tonight 21:20 to 00:08 your time).
3. Closer: split R1, its server blocks first and its browser blocks when U7 lands (one more bring-up, about EUR 0.15),
   or more usage headroom this week.

## What changes, and why (each answer NPT-TRIM asked for)
- **The marks** (replan-marks.md §1, all 81): 48 kept and 24 moved as your list settles them; 9 open, which my leans
  settle as 4 kept and 5 moved: 52 kept, 29 moved. Kept per run: R1 9 (11 with the two R4 rows), R2 23, R3 18, R5 13,
  9 judged here. Eight kept marks have parts that move (twelve by the leans), so MOVED works per part too.
- **Cut from U6** (all to the security baseline before the first online playtest; replan-units.md §1): the flood and
  abuse clients, slow-loris deadlines and header caps, the address table and /64 cycler, Origin checks, socket-minted
  tokens and capabilities, per-session controls, per-slot systemd units, the second WebTransport crate, the
  certificate's live swap and revocation. Kept: key-only SSH, default-drop firewall, the Join deadline, size limits,
  panic-free decoding, log hygiene, a fixed connection cap, the roster check with per-run slot tokens.
- **One WebTransport crate: web-transport-quinn 0.13.x.** It takes our own network socket directly (the network models
  need that) and exposes the send-buffer check ADR 0001 asks for; wtransport skips a handshake check and has a panic
  path. No kept run compares crates (-30 compares the two transports). PM-0001-28 compares both crates in every browser:
  its carried-crate parts stay (a scripted page on the test laptop, after U7), the other crate's parts move to the real
  game's transport change-set (Slice 3), so the crate question (R2-TRN-OQ1) stays open until both run there (F2).
- **The certificate behind the firewall: DNS-01 by hand.** Nothing inbound opens (TCP 80 leaves the firewall list) and
  no API token exists anywhere: the run session starts an order on the server, you add one TXT record in Cloudflare and
  delete it after, the server checks Cloudflare's own name servers and finishes. Let's Encrypt publishes no validator
  addresses, so HTTP-01 would need port 80 open to everyone; it stays only as the fallback. The WebTransport
  certificate is the server's own, 13 days, pinned by hash in the browser.
- **Lag compensation's server cost is already carried:** the room keeps 2 s of hit history and rewinds every shot today
  (room.rs:32-33; rewind.rs:18-33). Kept budget marks -4, -5 and -7 need only its measuring: two series writers, about
  80-250 lines, inside U6e. U4h's estimate, tolerance and cap move.
- **U7 is a Wasm build of the client's existing connection code** (codec, acknowledgements, clock, command timing; no
  world step), not a TypeScript copy, so one reviewed implementation serves bots and browsers. The hash checks (C-U7-1,
  -2) go with the sim.
- **Promote later, each its own change-set on main when the first slice needs it:** the judge, the log and marks.json,
  the L1 network model and ImpairSocket to the dev tools; fix 1's gate, the TCP-info reader and the loop policy to the
  server after R2 and R3. Stay in spikes/: the lab, DelayStream and its TCP model (until R1's netem check), and the
  stand-in sim, protocol, room and client.
- **Order** (replan-units.md §7): U4f2's merge and -21's runs (Sun-Mon), then U6a, U6b, U6c, U7's Rust half on the
  build lane; U8 and U7's TypeScript beside it; Stop B (security and infra read the run package; that read is U8's one
  review round); R1. MOVED lands by R1's report: until it does, a moved part reads NOT-RUN, never PASS, and R1's logs
  are judged again when it lands. After R1: U4k, U6d, U6e, U6j, then R2, R3, R5. One review round per unit (netcode
  and test), one advisor check per brief.

## Decisions for you (with the re-plan; each has my lean)
- **RE-PLAN-OK:** this plan as a whole, with the table's leans (F1-F8, F10, F13: replan-marks.md §2): yes, or change.
- **MARK-NOTES** (marks.json text, yours): -7's notes say R2 measures at the room's 2,000 ms history span (-48's test
  cap moved: F9); -56's record check counts 4 items, not 10 (6 ride moved marks: F11). Lean: yes to both.
- **LAPTOP:** name the test laptop before R1, so R1's browser legs are judged; otherwise they are rough readings and a
  later short run judges them (F12). Lean: name it if you can.
- **PACK-P0:** may I pack the two closed P0 run folders and the c-u2 folders (about 8.6 GB down to about 2.4 GB; their
  verdicts kept) so the build lane stops hitting the disk floor? Lean: yes.
