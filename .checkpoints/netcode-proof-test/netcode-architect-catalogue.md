Task: netcode-proof-test, netcode-architect: draft docs/design/netcode-proof-messages.md (the message catalogue,
plan §4.1's last entry; unit U1's class audit test reads it; U4's npt-proto writes against it). 2026-11-06.
Only this notes file and the doc are written. No code, no ADR or SPIKE_PLAN edits, no web, never commit.

## Plan (blocks, in order)
1. Read the plan's inputs: §4.1, §4.3 E3/E4/E10/E23, §4.4, §4.7, §6.1, §10.2, §10.3 (+ §11 for the overlay/form) -> notes
2. Read ADR 0001 (worktree .gate/npt-readings, PR #41's head): state table + Class column (~634), messages (~659-690),
   part 5, part 6, part 10, PM-0001-14/-21/-22/-25/-34/-36 -> notes
3. Draft the header (section 1) -> doc on disk
4. Table "Wire messages" (14 built rows + omitted rows) -> doc
5. Table "Field groups" (Snapshot, FullState) -> doc
6. Table "Session controls" -> doc
7. Table "Room-wide controls" -> doc
8. "What the class audit test asserts" (U1, U4) + questions for the main session -> doc
9. Self-check: column headers exact, row counts, every number traced, prose <= 120 chars -> notes
10. Hand back (summary, row counts, questions, DONE)

## Notes (one line per finding, with its source)
- No agent memory dir on disk; earlier decisions = my digest (netcode-architect-digest.md) and notes.
- Block 1 done (plan read): see block 1 findings below.
- Block 1 findings (plan): §4.1:371-373 the catalogue's line; §4.4:438-451 the two check homes (npt-room ticks /
  npt-server wall time; size limits at the socket before any delay buffering); §4.7:509-511 the 13 messages +
  SessionControl{SetProfile, TriggerBurst, BlockUdp, RecordStep}; Channel = Datagram | Control | FullState (§4.7:500);
  E7:421 a message on the wrong channel is dropped and counted as forged (-21) (client side too); §6.1:590-596
  INVALID only when a session outside the roster is admitted or a control is APPLIED without a capability; a rostered
  session sending a control without one also INVALID; pre-Join control closes the session, counted, never INVALID;
  every control/form record carries its source. §10.2:955-986 room-wide controls on /run/npt/ctl/NN, SO_PEERCRED uid
  npt; list: rewind cap, per-address cap raise (LG addresses only), roster+tokens, panic/stall injections, L2 per
  peer set (LG profile, -23 throttle, -24 bottleneck, -18 bursts, -29 UDP block); socket messages get same size
  limits + panic-free parsing; capability = 128 random bits minted via socket, constant-time compare; SetProfile and
  BlockUdp key on the holder's source address (all its connections, new ones, from QUIC first packet / TCP accept,
  until cleared or run end); new peer gets run default profile set on the socket; TriggerBurst on live session;
  overlay -> Worker -> control stream; bites: control/form from plain join refused+counted; pre-Join control closes.
  §10.2:985-986 "authority (none, capability or local socket)". §10.3:988-1003 token 128 bits minted on RS via
  socket, bound to room+slot, constant time, expires at run end; Join checks version + content hash; refuses joins
  beyond N; npt-host never mints; bots token by file, reusable for own slot; Join on a live slot -> Kick(replaced);
  browsers token + capability in URL fragment, bind at first use, rejoin after trap uses a resume secret kept in
  page memory; replaceState drops fragment; runner single-use public token, dedicated slot, job nonce (§9), 30-min
  expiry (§0.2:112); strays counted per run. §10.5:1045-1050: RS-to-LG key (npt-load-ctl forced command) "listed in
  the catalogue's control-plane table". §11:1119-1125: cap set over the socket before step 1, shown read-only (R-28);
  record form uses the capability. §11:1105 smoke link `?smoke=1#cap=…`, `&transport=ws`.
- Block 2 partial (ADR worktree HEAD 16763a7): state table 634-658, Class values used: "spatial; own state owner,
  full precision", owner, none (weapon spread, Sent to nobody, 638), "spatial; owner", "owner; spatial", spatial,
  "team or public", public, n/a. Catalogue 660-687 (Input 667 ... Lobby<->supervisor 684); "one with none fails the
  audit test (R2-INT-REC5)" 661; sizes R2's estimate else measured (R2-XU-OQ15) 661-662. Part 9 (e) 406-408 class
  vocab "public, team, owner or spatial". RNG state not in the state table: authority-only streams 212-215, 323-325.
- Part 5 220-275: cap 8 (221); command fields 225-228; checks 231-239; trailing bytes/unknown fields dropped whole
  (238); ping hold time <= 1 tick (247-250). Part 6 276-330: ring 32 (279); E3 (279-281); cap 256 kbps = ~1,067 B
  (290-291); datagram cap rule (291-293); groups decode alone (293-294); FullState reliable (294-295); event queue
  bounded, overflow -> resync or disconnect (301-302); cosmetic once never own (303); 32-bit header tick (318-321).
  Part 10 419-474: limits 442-453 (2 client streams each way, ~64 KB stream window, ~128 KB connection window, ~16 KB
  datagram rx buffer, input > command size x cap dropped, Join within 2 s; until then one stream, datagrams dropped
  (446); WS frame/message/write-buffer limits explicit (452-453)).
- Block 2 done. PM rows (worktree lines): -14 1550 (cap rule; bite: a group never split), -21 1559, -22 1560, -25 1563
  (Join 2 s from accept/first packet; slow-loris; /64 cycler; bites: library default limits, deadline from upgrade end,
  whole-IPv6 keying), -34 1574 ("every message type of the catalogue fuzzed"; control sequences into room = PM-0004-60),
  -36 1576 (maxDatagramSize at start + every 10 s; bite read once). Part 11 475-533: join token claims 494-497 (draft),
  test tokens allowed 1641 ("proposed"); names Cc/Cf + NFC rule 497-499; RoomEnded(crash) 488-490; Kick on host
  command 512-514. Part 13 568-586: B1 570; explicit 1 % reorder step per flow and direction 581-585 (R-5 in PR #41).
  Checklist 1656-1709: cap set over the local socket before step 1, shown read-only with the command-timing option in
  force (1667-1669, R-28); steps 2/3/12 SetProfile, 11 B1 from overlay (own commands lost too: 1699-1700), 13 WS
  forced (UDP blocked). EDITORIAL in PR #41: "The profile changes from the overlay." appears twice (1665 and 1667).
  Migration step 2 (1722-1726): the foundation's protocol crate is fresh; slice 2 (1731) slot table; slice 3 (1734-1738)
  lobby, tokens, byte-level audit R2-INT-REC5. Part 9 (e) classes proven at slice 3 (1635-1637), not here.
  Skeleton check: spikes/netcode-proof/crates/{proto,link}/src/lib.rs are 4-5 line stubs (U1 part 1), so no code to
  match yet. .claude/rules/netcode.md:17 = "Every protocol change updates the message catalogue in docs/design/".
- PM-0006-23 (ADR 0006:1378) 1 s stall inside one tick; PM-0004-67 (ADR 0004:1193) 200 ms and 1 s stalls -> InjectStall.
- Plan §9:917-923 runner token: dedicated slot; unused 30 min, bound <= 60 min; binds at first use to a job nonce; no
  capability -> Join needs a bind field (browser resume secret / runner nonce / bot's own value): proposed.
- Design calls (mine, proposed, in the doc): Join.bind 16 B; capability carried in every SessionControl, bound to the
  slot it was minted for; control-stream framing u16 length + u8 type; full-state stream u32 length; WS messages carry
  a 1 B channel tag; a post-Join undecodable control message closes with Kick(malformed); FullState carries no event
  section and a forced resync clears the event queue; Leave c->r, Kick r->c (draft row: "either way").
- Sizes (proposed, arithmetic in the doc): Input 166 B (6 + 8 x 20, draft 706-710 estimates); Snapshot 1,000 B send
  until -36 then min(smallest, 1,150 B), receive 1,150 B; FullState 65,536 B (SS whole world ~6.5 KB); Resync 8 B;
  Ping/Pong 32 B; Join 128 B (69 B); JoinAccepted 256 B (80 B); JoinRejected 16 B; ClientSettings 8 B; RoomEnded 64 B;
  Leave/Kick 16 B; SessionControl 512 B (504 B with a 480 B note); socket messages 1,024 B.
- Gaps -> questions: (Q-STATUS) no room->client message carries the overlay's server values (profile in force,
  server RTT, cap, option, clipped shots) or a "saved" echo; (Q-SWITCH) OQ-0001-6 option + NewReno/Cubic on neither
  control list; (Q-OPEN) how npt-super learns Open (-16) without npt-proto; (Q-NONE) state table "none" vs
  authority-only; (Q-VOCAB) omitted lobby/supervisor rows use the draft's direction/channel words; (Q-FUZZ) socket
  messages fuzzed under -34?; (Q-CAP) "a per-run capability" read as one per browser slot; (Q-RESYNC) queue cleared.
- Next: block 3 (write the doc header).
