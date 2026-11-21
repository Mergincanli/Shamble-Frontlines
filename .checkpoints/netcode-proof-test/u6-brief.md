# U6's brief, rev 2 (2026-11-21; WAIT-PLAN (a) part 2, history.md:7511-7516): U6's build plan and its split
Rev 1 (164 lines) went to the advisor: APPROVE WITH CONDITIONS 1-9 (advisor-u6-brief.md, 319 lines; ADV-U6B-1 High:
blocks missing). Rev 2 meets them; sections 1, 6 and 8 go back for its short check (condition 9) before U6-SIZE and
U6-JUDGE reach him. The brief says what U6 builds, in which pieces, how each is tested and what is open; it points to
plan.md:1253 (U6's row), u6-facts.md (369 lines, every fact cited) and advisor-u6-brief.md, and repeats none of them.
Nothing here is built before U4 closes and U5 merges (the pieces share files with them: u6-facts.md:295-322).

## 0. Where things are
- **The row:** plan.md:1253 (the crates, WsServer's send path with fix 1, U3c's carries, the reader stamp, C-U6-1 to
  -19, -35's run); U6's engineering items E2, E5's load part, E6, E14's network half, E15's L2 half, E16, E18, E21-E23
  (plan.md:461-482); the public surface §4.7 (:600-608); rate adaptation §4.4 and §4.7 (:517-521, :577-579;
  advisor-u4.md:202; ADR 0001:450); the control plane and hosting §10.2-10.5 (:1066-1165); certificates §8.4-8.5
  (:960-996).
- **The register's U6 entries:** 110-111 (-35), 112-119 (the advisor's rulings on this brief, word for word), and 60,
  62 (d), 64, 75, 76, 88, 96, 97, 103, 104 (4) elsewhere (u6-facts.md:59-66).
- **What exists** (u6-facts.md:70-154): npt-net holds U3b-U3d's parts, held "until U6", with no WtServer, WsServer or
  clients; npt-server has U4's pump and schedule and a stub binary; npt-super, npt-host and npt-load are stubs; of the
  65 SERIES.md rows npt-server writes, 51 have no writer yet (advisor-u6-brief.md:34-44); the room has no rate
  adaptation (:27-33).
- **Packages:** new: instant-acme with hyper, hyper-util, hyper-rustls and http-body-util; metrics and
  metrics-exporter-prometheus. Already in the lock but new as direct dependencies or features: tokio-tungstenite and
  futures-util, rcgen, npt-alloc (register 104 (4)), npt-net's `bites` and `default` features (u6-facts.md:261-293;
  advisor-u6-brief.md:246-252).

## 1. What U6 builds: four pieces and a judge change (the split is his: U6-SIZE, section 8)
Blocks are u6-facts.md:160-218's plus the advisor's additions (ADV-U6B-1); each about 400 added lines at most, a
larger one split, each compiling with its tests passing before the next. Estimates are added lines, code and tests.

| Piece | Blocks (estimate) | Crates | Sum | Stop |
|---|---|---|---|---|
| U6a the room process core | S1 TickLoop and PhaseTimers on Loopback (750); S1b the interest writers through a clock-free phase seam in the room (250-450; entry 113); S2 the paired session and `JoinAccepted` change, C-U6-13, -14, -16, -19 (800); S3 C-U6-15's cheap wins with the bench (entry 115), C-U6-17, C-U6-18 (600); S8a the binary's settings, lifecycle, panic catch and non-zero exit (150-200) | server, room, proto, client, bots | 2,550-2,800 | past 6,000 |
| U6b the transports on loopback | T1 WtServer, both crates, C-U6-12 on both transports (750); T2 WtClient, WsClient (350); T3 WsServer's accept path (400); T4 WsServer's send path with fix 1 (700); T5 the L2 extras and the reader stamp (400); T6 `Link`'s loss, send-buffer space and `Send` (150); R rate adaptation, its first consumer (300-500); S8b the WebTransport certificate, its live swap and the transport wiring (100-150; entry 117) | net, link, l1, room, server | 3,150-3,400 | past 6,000 |
| U6c control plane, measurements, process control | S4 instrumentation and the header, carry 60, C-U6-4 (400); S5 metrics and logs, PM-0006-20's allocator twin (350); S9 the other npt-server series writers and E23's panic and stall injections (400-800); S6 AddrTable, the handshake cap, refusals per reason, deadlines (350); S7 ControlSocket, tokens, capabilities (700); H3 npt-super (300); H4 ops/units/, the `npt-ctl` and `npt-cert` wrappers, the sudo list (250; entry 116) | server, room, net, super, ops | 2,750-3,150 | past ~5,900 |
| U6d probe, host, load, hostile clients | P1 the -3 probe, C-U6-1 to -7 (750); P2 C-U6-8's stamp tests (150); H1 npt-host with `/npt/cert-hash.json` (650); H2 the instant-acme client behind `npt-cert` (250; entry 118); B1 npt-bots' npt-net glue and npt-load (600; entry 112); B2 E14's network floods (200-400; entry 112); B3 -30's container runner over both real transports (150-300) | server or lab, net, host, bots, lab | 2,750-3,100 | past ~5,850 |
| U6j the judge (only on U6-JUDGE (b)) | J1 C-U6-9 to -11 (400); J2 the 20 figures marked "U6" (1,300-2,800: at U4h's judge change's 65-140 lines a figure, u4h-brief.md:76, :265) | judge, marks/SERIES.md, marks/README.md | 1,700-3,200 | past ~4,900 |

- **Totals:** U6a-U6d about 11,200 to 12,450 added lines; U6j apart. U6d no longer counts J1 (ADV-U6B-2).
- **Two moves from the advisor's homes** (for its short check): -30's container runner goes to U6d, which keeps U6b
  under about 3,500 (with it, 3,300-3,700); npt-super (process control) stays in U6c, where the advisor's piece sizes
  (advisor-u6-brief.md:77-78) put it, so U6-SIZE's text names it under U6c.
- **Order:** U6a, U6b, U6c, U6d, each merged before the next starts (advisor-u6-brief.md:81-86): WsServer's per-tick
  hand-off and the probe's poller need TickLoop's runtime; C-U6-17 is in before any honest client runs on L2; S6's
  limits sit in front of both accept paths; the probe writes steal "as npt-server does" (C-U6-4); H2 sits behind H4's
  `npt-cert`. U6j goes beside them, after the U4h judge change takes register 64, under the lane lock and the disk
  lines, merged before -30's container run and the network runs.
- **Seams named once:** F7's accept-cap sizing (T3) and S6's handshake cap are one cap, built once; C-U6-12's
  control-message limit holds on both transports; carry 60's header fields are built in S4, and T5 lists them.
- **Each piece:** its own short brief at launch (blocks, size line, tests, the carries it meets, the SERIES.md rows it
  writes), its `<base>` and "there now" re-read, one isolated builder in the shared target under the lane lock (a
  names-only runner copy), its review round (two at most), its merge. **A piece that changes any manifest or lock
  starts with Block 0:** every manifest and lock by path, then the DEP-HOLD stop and security-auditor's read before
  anything is fetched or built (ADV-U6B-7; each piece likely has one).

## 2. Marks and runs
- **Container runs after a merge:** the reader stamp's loopback self-test reports its four parts after U6b, before U8
  builds the run package (row:22; plan.md:1255); -30's C part on both real transports (marks.json:6575; register 88)
  after U6d (its runner) and U6j's J1 (C-U6-9: figures.rs:190-200 refuses -30's twins until then); -35's one counted
  run after U6d's merge ("once, after the swap": U6c and U6d can still add code outside crates/net), with U5's tool run
  uncounted on a scratch swap at U6b's end and each later piece's end (entry 114). Under U5-SIZE (b), U5b (the swap
  checker) is built before that counted run. The bites U6 builds (u6-facts.md:234-240) run in their piece's tests.
- **No VM run in U6.** Every R1-R7 run follows U8 and his stop B (plan.md:815-834, :1267-1270); u6-facts.md:241-251
  lists the judged runs that need U6 first.
- **Gates U6 closes:** C-U6-19 at U6a's merge (S2, with C-U6-13 and -16 in the same change and review; SA-F1-1 stays
  open until then); C-U6-17 in U6a; C-U6-11 in U6j (its span start a marks/README.md rule).

## 3. How it is tested
- **Netcode (deterministic, injected latency, jitter and loss):** every carry's test as the row states it, on L2 over
  loopback (ImpairSocket under quinn; DelayStream under tungstenite) or on Loopback with an injected clock (U4e's test
  host). U6a: TickLoop's thread, bounded channels, the panic catch; S2's plants (the base not sent, the first-command
  check skipped, a stale session's message reaching the room), each failing by name; the interest writers on a fixed
  scene. U6b: U3c's writer properties, test (c) and the queueing-writer bite on WsServer; the plain-path SIOCOUTQ
  check; the reader stamp's identity test and misattribution bite; rate adaptation on L2 profiles (loss and RTT in,
  low-priority entities dropped first, then 20 Hz: ADR 0001:450). U6c: each injection and each series writer.
- **Security:** the control socket's peer check and size-limited, panic-free parser; token and capability minting,
  binding and replacement; the 100,000-address table; slow-loris deadlines; E14's floods against both accept paths;
  the host's allow-list and 404 bite; the certificate profile failing closed, tested against an in-process fake ACME
  directory (entry 118). New parsers on network or socket input get fuzz entries in U5's crate, each a second
  dependency read if its manifest changes.
- **Performance:** C-U6-15's cheap wins each with the bench before and after, the state hash unchanged, a moved pinned
  file named with its reason (entry 115); the per-tick paths allocate nothing in steady state (a counting-allocator
  test).
- **Series:** each piece's brief names the SERIES.md rows it writes; U6's close report shows every npt-server row
  written or named for a later unit (condition 1).
- **End checks at every block end:** check.mjs's steps on both builds, clippy and fmt, the wasm32 check, the gate; the
  disk lines (no build call under 5.5 GB, a whole-workspace run only at 8.8 GB free, the 4 GiB floor).

## 4. The carries, by number
All of the row's carries, register entries 110-119 and the other entries of section 0 are met by number in their
piece's report; U6's close report folds the four pieces' and U6j's.

## 5. The rules that stand
- Plan §4.1-4.2's layout and dependency rules, §4.7's public surface, §4.8's exact pins and audits;
  `.claude/rules/netcode.md` for crates/server and crates/net; no client string verbatim in logs (E22).
- DEP-HOLD per piece (section 1); the shared target and lane lock; never cd into a builder's copy.
- **His stop B** before any VM: the control plane, the page host and the sudo wrappers reviewed, the sudo list against
  GTFOBins (plan.md:1267-1270; entry 116).
- **C-U6-19's gate line (entry 119):** until U6a merges, each of these briefs states in one line that its runs hold no
  throwing or rejoining hostile client and no client restarted mid-run, or brings C-U6-19 first: U4g2's, U4h2's under
  U4H-SIZE (b), the U4h judge change's under U4H-JUDGE (b), any change F-B-COUNT (b) makes, and U6j's if briefed before
  U6a merges.

## 6. Size
SIZE-CHECK (history.md:2048-2051): M about 3,000 added lines, the stop past 6,000. U6's pieces come to about 11,200 to
12,450 (section 1), U6j about 1,700 to 3,200 more. The basis: u6-facts.md's block estimates (judgements scaled on
comparable files, not counts) plus the advisor's additions for the missing blocks (advisor-u6-brief.md:64-71) and the
judge at U4h's per-figure cost. M was set before the work was counted: npt-server, the transports' servers and clients,
the host and the control plane are unbuilt, and about thirty follow-ups were added to the row after rev 3.3. Units so
far came to about 1.35 to 1.5 times their estimates (u4h-brief.md:266-268). Built whole, U6 would pass the 6,000 stop
about halfway through. The split is a plan change, so it is his (advisor-u4g-brief.md:191-192): U6-SIZE. The size
lines follow U4g's and U4h's split form: the first piece keeps M's 6,000; each other piece stops past twice its
estimate's middle, never above 6,000; U6j past twice its middle (as U4H-JUDGE's was). At 1.5 times, every piece stays
under its stop.

## 7. Reviewers and rounds
Every reviewer at xhigh (netcode work), at most three at a time, two rounds at most per piece.
- U6a: netcode-architect, security-auditor, rust-reviewer, test-engineer, perf-auditor (TickLoop), engine-architect (a
  framed message's channel read, C-U6-16 (c); a session generation or end-session call, C-U6-14; the phase seam).
- U6b: netcode-architect, security-auditor, rust-reviewer, test-engineer, perf-auditor (the send path),
  engine-architect (`Link` gains `loss`, send-buffer space and `Send`).
- U6c: security-auditor, infra-engineer (units, wrappers, sudo list), netcode-architect (wall-time limits, deadlines,
  the injections), perf-auditor (instrumentation and the allocator twin on the tick path), rust-reviewer,
  test-engineer.
- U6d: security-auditor (host, ACME, floods), infra-engineer, netcode-architect (the probe), test-engineer,
  rust-reviewer; engine-architect if the probe lands in npt-server (a new public item).
- U6j: test-engineer and rust-reviewer read every judge line.

## 8. Questions
**His (two; neither a stop: U6 starts after U4 and U5; for a day's message after the advisor's short check; the
advisor's texts, advisor-u6-brief.md:128-174, with the recount's numbers and the two moves of section 1):**
```text
U6-SIZE: how to build unit U6 (the game server program, the two network transports, the control plane, the page host
and its certificates, and the load generator).
The plan calls U6 medium (M): about 3,000 added lines, your stop past 6,000. That size was set before the work was
counted. Counted block by block now, from the plan's U6 row and the code as it stands, U6 is about 11,200 to 12,500
lines: the server program, the WebTransport and WebSocket servers and clients, the control plane and the page host are
still unbuilt, and about thirty follow-ups from later reviews were added to the row after you approved the plan.
Units so far came to about 1.35 to 1.5 times their estimates. Built whole, U6 would pass your 6,000 stop about halfway
through.
(a) Build U6 whole: one review and one merge, but your stop would have to move to about 24,000 (twice the estimate's
    middle), a new number from you; and one review of 11,200 to 12,500 lines across four programs, the control plane
    and its admin commands among them, is harder to do well than four reviews of about 3,000.
(b) Split it into four parts, built one after another, each with its own short brief, review and merge, each stopping
    past twice its estimate and never above your 6,000:
    U6a the room server's core: the tick loop, the session and join changes, the room's speed fixes and its
    measurements of who sees whom (about 2,550 to 2,800 lines; stop past 6,000);
    U6b the two transports on this machine's simulated network, and the server sending less when a link is congested
    (about 3,150 to 3,400);
    U6c the control plane, the server's measurements and logs, its test controls and process control (about 2,750 to
    3,150);
    U6d the timing probe, the page host and its certificates, the load generator, the hostile test clients and the
    replay of recorded matches over both transports (about 2,750 to 3,100).
No pass mark, judge rule or marks.json text changes under either, and no run is dropped.
The cheaper option is (b): three more reviews and merges, and no mid-build stop waiting on you. U6 starts only after
U4 and U5 close, and its first part is built first either way, so nothing waits for this answer until that part is
done.
My lean, and the advisor's: (b).
```
```text
U6-JUDGE: the judge gains the computations U6's measurements need. marks.json already names 20 figures the judge
cannot compute yet (the cost of keeping clients in sync, bits per client down and up, memory and queue growth, the
other rooms' timing after one room is killed, how well the two round-trip times agree, the control stream's delay under
a bulk transfer, the time to close after a crash, egress bytes against the formula, the cost of logging, the
allocation twin); their formulas are already written in the series file (marks/SERIES.md). Three judge changes from
earlier reviews also wait for U6: the twin check's second form for -30 (each run against its other-transport twin),
the allocation twin's role for PM-0006-20, and how -19's join check covers a client admitted mid-run. With them, one
row of the series file and one rule of the results contract (marks/README.md) gain those cases. Each computation gets
pass, fail and gap test files on made-up logs, and the test-engineer and the rust-reviewer read every judge line. This
changes the judge's code and those two texts, not marks.json, a pass mark or a number.
(a) Yes, inside U6's four parts, each with the measurements it serves (about 400 to 800 lines more in each part).
(b) Yes, as its own small change (the judge only, about 1,700 to 3,200 lines; stop past about 4,900), reviewed and
    merged early, beside U6's first parts. It shares only the series file with them, each change on its own rows.
(c) No: those 20 figures stay not-run, so 12 marks cannot be judged; -30's container check and -19 cannot be judged
    either, and the three changes come back to you.
-30's container check, after U6's last part, waits for the first of the three changes; the 20 figures serve only the
network runs after U8 and your stop B.
My lean, and the advisor's: (b).
```
**The advisor's rulings, filed as register entries 112-119 (word for word there):** U6-Q2 npt-bots' glue, npt-load
and E14's floods in U6d (the `ws://` and wrong-Origin pages with U7); U6-Q4 interest under load is the series writers
of S1b, not nothing new; U6-Q13 the swap form, four terms, the counted run after U6d; U6-Q15 the cheap wins with the
bench, the state hash unchanged; U6-Q16 ops/units/, `User=npt-room-NN`, read at the round and at stop B; U6-Q17 the
room server makes the certificate, its part of S8 in U6b; U6-Q18 no 90-day fallback, the root store at U6d's Block 0,
a fake directory in tests; U6-Q21 C-U6-19's line widened (section 5).
**Deferred to each piece's brief, chosen there with the advisor** (confirmed, advisor-u6-brief.md:226-232): Q5, Q6,
Q7, Q8 (with the netcode and security seats' read), Q9, Q10 (a marks/README.md rule, chosen in U6j's brief with
test-engineer's read, inside his U6-JUDGE yes), Q11, Q12 (engine-architect if the probe lands in npt-server), Q19,
Q20. Q14's drift: U6b cites delay.rs:1907.

## 9. For the main session
- Now: rev 2's sections 1, 6 and 8 to the advisor for its short check; the rulings are in the register (112-119).
- After the check: U6-SIZE and U6-JUDGE in a day's message (it can wait a day: nothing needs them until U6a is built).
- After his answers: the plan's U6 row gains its line (the pieces with their size lines, -35 after U6d; U6j under
  U6-JUDGE (b)) and §4.1 gains ops/units/ (entry 116).
- At each piece's launch: its brief, the `<base>` fill, the runner copy, Block 0's DEP-HOLD read, its size line on the
  card before its first snapshot.
