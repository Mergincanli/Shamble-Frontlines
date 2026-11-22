# The netcode proof test, re-planned under NPT-TRIM (for Mustafa)
2026-11-22 Amsterdam. His decision: history.md:7561-7593. Facts: replan-marks.md (the 81-mark table, test-engineer)
and replan-units.md (the units, netcode-architect); both read only, their key claims checked in the code. The advisor's
one review: APPROVE WITH CONDITIONS (advisor-replan.md; ADV-RP-1 High, -2 to -9 Medium, -10 to -16 Low), its
paste-ready P1-P6 applied word for word and its Lows folded in, so no re-check.

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
| R8 (later, rev 3.3's own short run; not in the sums) | only if ML is not named by R1: ML's judged legs of -1, -14, -28, -29, -36, PM-0006-16, PM-0006-25; no Safari | 3 | 3 | 0.47 | 0.70 |
R5's load hours are rough (its extra load machines served the N = 36 search too); every run plan fixes its own. Your
part per run: create the VMs (plan §7.3), type your home address into the firewall rule, and one Cloudflare TXT record
per certificate order (two in R1, about 2 minutes each); present in R1 for about 2 hours.

**R1's date: planned Thursday 3 December; Wednesday 2 December holds only with no slack.**
1. R1 needs three server pieces and the client's Rust half in series on one build lane, then Stop B: about 8,350-9,250
   likely lines in 7 days, about 1,300 merged a day, against about 1,175 a day merged last week.
2. Spend-limit cuts took 1.5 to 4 hours on most recent days (tonight 21:20 to 00:08 your time).
3. Closer: split R1, its server blocks first and its browser blocks when U7 lands (one more bring-up, about EUR 0.15),
   or more usage headroom this week.

## What changes, and why (each answer NPT-TRIM asked for)
- **The marks** (replan-marks.md §1, all 81): 48 kept and 24 moved as your list settles them; 9 open, which my leans
  settle as 4 kept and 5 moved: 52 kept, 29 moved. Kept per run: R1 9 (11 with the two R4 rows), R2 23, R3 18, R5 13,
  9 judged here. Eight kept marks have parts that move (twelve by the leans), so MOVED works per part too. Destinations
  by number: Slice 1's change-sets 6 (Look), 7 (Lag), 8 (Shoot) and 9 (Throw), Slice 3, slice 6; -34 per message type
  as the real protocol adds it (change-set 3's codec first), -35 at Slice 3 (its swap needs the second transport); -55
  at the first slice that adds vehicles (not cut yet); R4's rows at the security baseline before the first online
  playtest; Safari later (BROWSER-SET (a)); RD's parts when RD is named.
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
  addresses, so HTTP-01 would need port 80 open to everyone; it is not built: a failed order stops the run and comes
  back to you. R2, R3 and R5 have no browser, so their run plans may skip the public order (the load machines pin the
  room's own certificate); a split R1 would make a fifth order for the same name in 7 days (plan.md:979). The WebTransport
  certificate is the server's own, 13 days, pinned by hash in the browser.
- **Lag compensation's server cost is already carried:** the room keeps 2 s of hit history and rewinds every shot today
  (room.rs:32-33; rewind.rs:18-33). Kept budget marks -4, -5 and -7 need only its measuring: two series writers, about
  80-250 lines, inside U6e. R5's density marks (-27, PM-0006-13, -14) pay the rewind in every room too. U4h's estimate,
  tolerance, cap and per-shot checks move, and are not in these numbers.
- **U7 is a Wasm build of the client's existing connection code** (codec, acknowledgements, clock, command timing; no
  world step), not a TypeScript copy, so one reviewed implementation serves bots and browsers. The hash checks (C-U7-1,
  -2) go with the sim.
- **Promote later, each its own change-set on main when the first slice needs it:** the judge, the log and marks.json,
  the L1 network model and ImpairSocket to the dev tools; fix 1's gate, the TCP-info reader and the loop policy to the
  server after R2 and R3. Stay in spikes/: the lab, DelayStream and its TCP model (until R1's netem check), and the
  stand-in sim, protocol, room and client. npt-l1 meets change-set 3's own L1: it is offered to that plan rather than
  built twice; the lab stays because it is bound to the stand-in sim.
- **Hand-off, so nothing is dropped:** the moved table (each mark or part, its marks.json bar unchanged, its
  destination) reaches the game lane before change-set 3's plan, its first destination: a paste block for your game
  chat (For Mustafa, item 2), then this test's final report on main. That lane changes, on your go, its build plan's
  OQ-0001-6 line (Slice 3 now picks: :389-391 and Part 6) and takes the moved marks into its change-sets.
- **Order** (replan-units.md §7): U4f2's merge and -21's runs (Sun-Mon), then U4's report (Mon); U6a, U6b, U6c and
  U7's Rust half on the build lane; U8 and U7's TypeScript beside it; Stop B's read (security and infra on the run
  package, U6c's host and the wrappers: U8's one round) from U6c's merge, beside U7's Rust half; R1. After R1: MOVED
  first on the lane, then R1's report (a moved part reads NOT-RUN until then, never PASS); then U4k (a new unit), U6d,
  U6e, U6j; R2, R3, R5. One round per unit, netcode and test, plus security on U6b, U6c and U6e (transports, page host
  and certificate client, metrics exporter: they face the internet in every run); one advisor check per brief.

## Decisions for you (each with my lean)
- **RE-PLAN-OK:** this plan as a whole. It reads your words in five places; say if one is wrong. (1) Two R4 rows that
  are not hostile input stay, in R1 (+0.45 h): the fallback when QUIC is blocked, and the certificate pinned by hash
  in every browser; your question names the fallback. (2) The choice between the two ways of timing commands
  (OQ-0001-6) leaves this test for Slice 3, since its inputs moved; R2 and R5 run the draft's buffered way, R3 tests
  both. (3) U7 runs the bots' own connection code as WebAssembly (codec, acknowledgements, clock); it steps no world,
  so "no sim in Wasm" holds, and there is one implementation, not a second in TypeScript. (4) The certificate comes
  from a DNS record you add and delete, so nothing opens to the internet. (5) -58 (counter wraps) stays, its two
  prediction checks moved; that brings back U4G-Q2, your yes to the wording of -58's two lost-tick checks, before
  -58's set, after R1. The table's other leans: replan-marks.md §2. Lean: yes.
- **MARK-NOTES** (marks.json text, yours). (1) -7: R2 measures lag compensation's time and memory at the room's 2,000 ms
  history, which covers any cap the real game sets (-48's test cap, which -7 was sized from, moved). (2) -56's record
  counts 4 items, not 10: items 5-10 move with their marks; item 1 keeps only the carried crate's results on -11 and
  -28 (the other crate, -25, -35 and the pick move to Slice 3); item 2 keeps each browser's draft and its fallback
  (with RE-PLAN-OK (1)), Safari later; item 4 keeps NewReno against Cubic, while the browsers' rates at P2 to P4 move
  with the human run. Lean: yes to both.
- **LAPTOP:** browser legs are judged only on ML, the typical student laptop with integrated graphics that you name for
  the whole game (OQ-0003-1); your MSI gives rough readings only. Seven kept marks wait on it (-1, -14, -28, -29, -36,
  PM-0006-16, PM-0006-25). (a) Name ML and bring it to R1: judged there. (b) Not yet: R1 reads rough, and R8, a short
  later run (about 3 h, cap EUR 0.70), judges them once ML is named. Lean: (a) only if such a laptop is at hand; else (b).
- **PACK-P0:** may I pack the two closed P0 run folders and the c-u2 folders (about 8.6 GB down to about 2.4 GB, with a
  checksum list and a read-back; their verdicts kept) so the build lane stops hitting the disk floor? Lean: yes.

**For Mustafa:** 1. Answer the four decisions in one message. 2. After your yes, paste the moved-mark block I give you
into your game chat, before its change-set 3 plan. 3. Say whether Wed 2 or Thu 3 December suits you for about 2 hours
at home (R1). 4. Before R1: Chrome and Firefox on the laptop; Stop B's steps (about Tue 1 December).
