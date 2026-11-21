# U6's brief, rev 1 (2026-11-21; WAIT-PLAN (a) part 2, history.md:7511-7516): U6's build plan and its split
For the advisor's check (TEST-FIRST rule 3: a short brief before the build). It says what U6 builds, in which pieces,
how each is tested and what is open; it points to plan.md:1253 (U6's row, 46 lines once split on `<br>`) and to
u6-facts.md (369 lines, every fact with its cite) and repeats neither. Nothing here is built before U4 closes and U5
merges (the plan's unit order; the lanes share files: u6-facts.md:295-322).

## 0. Where things are
- **The row:** plan.md:1253: the crates (npt-server, npt-net, npt-super, npt-host), WsServer's send path with fix 1
  (U3c's carries, the reader stamp), U3d's C-U6-1 to -8, U4j's C-U6-9 to -11, U4e's C-U6-12 to -16, U4f's C-U6-17
  and -18, U4f-F1's C-U6-19, and -35's counted run. The engineering items U6 owns: E2, E5's load part, E6, E15's L2
  half, E16, E18, E21 to E23 (plan.md:461-482); the public surface §4.7 (:600-608); the control plane and hosting
  §10.2-10.5 (:1066-1165); certificates §8.4-8.5 (:960-996).
- **The register's other U6 entries:** 60, 62 (d), 64, 75, 76, 88, 96, 97, 103, 104 (4), 110, 111
  (u6-facts.md:59-66).
- **What exists** (u6-facts.md:70-154): npt-net holds U3b-U3d's parts, held `allow(dead_code)` "until U6", but no
  WtServer, WsServer or clients; npt-server has U4's pump and schedule and a stub binary; no TickLoop, PhaseTimers,
  ControlSocket, AddrTable, runtime or metrics; npt-super, npt-host and npt-load are stubs; the room's calls U6 drives
  are at room.rs:762-1028; ops/ has no units or wrappers.
- **New packages** (u6-facts.md:261-274): instant-acme with its HTTP stack (hyper, hyper-util, hyper-rustls,
  http-body-util), metrics and metrics-exporter-prometheus; the counting allocator as a normal dependency (register
  104 (4)). Already in the lock: tokio-tungstenite, futures-util, wtransport, web-transport-quinn, rcgen, tracing
  (:276-293).

## 1. What U6 builds: four sub-units and a judge change (the split is his: U6-SIZE, section 8)
The blocks are u6-facts.md:160-218's, each about 400 added lines at most (a larger one split), each compiling with its
tests passing before the next. Estimates are code plus tests.

| Sub-unit | Blocks (u6-facts.md) | Crates | Estimate | Stop |
|---|---|---|---|---|
| U6a room process core | S1 TickLoop and PhaseTimers on Loopback; S2 the paired session and `JoinAccepted` change (C-U6-13, -14, -16, -19); S3 C-U6-15's cheap wins, C-U6-17, C-U6-18; S8 the binary and the WebTransport dev certificate | server, proto, room, client, bots | ~2,850 | past 6,000 |
| U6b transports on L2 loopback | T1 WtServer (both crates; C-U6-12); T2 WtClient, WsClient; T3 WsServer's accept path; T4 WsServer's send path with fix 1; T5 the L2 extras and the reader stamp; T6 Link's `loss`, send-buffer space and `Send` | net, link, l1, server | ~2,900 | past ~5,800 |
| U6c control plane and process control | S4 instrumentation and the header (carry 60, C-U6-4); S5 metrics and logs (PM-0006-20's allocator twin); S6 AddrTable, the handshake cap, refusals per reason, deadlines; S7 ControlSocket, tokens, capabilities; H3 npt-super; H4 the unit file, the `npt-ctl` and `npt-cert` wrappers, the sudo list | server, room, super, net, ops | ~2,700 | past ~5,400 |
| U6d probe, host and load | P1 the -3 probe (C-U6-1 to -7); P2 C-U6-8's stamp tests; H1 npt-host; H2 the instant-acme client behind `npt-cert`; npt-bots' npt-net glue and npt-load (u4-build-plan.md:966-967) | server or lab, net, host, bots | ~2,800 | past ~5,600 |
| U6j the judge (only on U6-JUDGE (b)) | J1 C-U6-9 to -11 (after register 64's follow-ups, u4s-carries.md:455-457); the 20 figures marked "U6" (judge/src/figures.rs:866-966) | judge, marks/SERIES.md | ~1,400-1,900 | past ~3,800 |

- **Order:** U6a, U6b, U6c, U6d, one after another, each merged before the next starts: U6b's WsServer and U6d's probe
  run on U6a's runtime (row:24; u6-facts.md:228-229); U6c and U6d's host feed stop B (plan.md:1267-1270). U6j shares
  no file with them and may go beside them, merged before the network runs that need its figures.
- **Each sub-unit:** its own short brief at launch (blocks, size line, tests, the carries it meets), its `<base>` and
  "there now" re-read, one isolated builder in the shared target under the lane lock (u5-run.sh's form, a names-only
  copy per sub-unit), its review round (two at most), its merge. A sub-unit that adds packages starts with block 0:
  every manifest and lock by path, then the DEP-HOLD stop and security-auditor's read before anything is fetched or
  built (U5's Block 0 rule).
- **Why four, not three:** at these estimates three pieces cannot each stay near 3,000 (u6-facts.md:223-227).

## 2. Marks and runs
- **Container runs after a merge:** -30's C part on both real transports (marks.json:6575; register 88: NOT-RUN until
  U6), after U6b; the reader stamp's loopback self-test reports its four parts before U8 builds the run package
  (row:22; plan.md:1255), after U6b; -35's one counted run after the swap (row:46; entries 110-111), after U6b. The
  bites U6 builds (ws-budget-x2 in npt-net's `bites` feature, PM-0006-20/planted-format, the host's 404 bite, the
  control-plane bites: u6-facts.md:234-240) run in their sub-unit's tests.
- **No VM run in U6.** Every R1-R7 run follows U8 and his stop B (plan.md:815-834, :1267-1270); u6-facts.md:241-251
  lists the judged runs that need U6 merged first.
- **Gates U6 closes:** C-U6-19 (before -19's runs, PM-0004-61's online half, any throwing or rejoining hostile client
  or a client restarted mid-run: row:45), with C-U6-13 and -16 in the same change and review (U6a's S2); C-U6-17
  (any honest client off L1); C-U6-11 (-19's join hash).

## 3. How it is tested
- **Netcode (deterministic, injected latency, jitter and loss):** every carry's test as the row states it, on L2 over
  loopback (ImpairSocket under quinn; DelayStream under tungstenite) or on Loopback with an injected clock (U4e's test
  host, crates/server/tests/common/mod.rs). U6a: TickLoop's thread, bounded channels, the panic catch (RoomEnded(crash),
  non-zero exit), S2's plants (the base not sent, the first-command check skipped, a stale session's message reaching
  the room), each failing by name. U6b: U3c's writer properties, test (c) and the queueing-writer bite re-run on
  WsServer; the plain-path SIOCOUTQ check; the reader stamp's identity test and misattribution bite (row:22).
- **Security:** the control socket's peer check and its size-limited, panic-free parser; token and capability
  minting, binding and replacement; the 100,000-address table; slow-loris deadlines; the host's allow-list and 404
  bite; the certificate profile failing closed (plan.md:964-969). New parsers on network or socket input get their
  fuzz entries in U5's crate, each a second dependency read if its manifest changes.
- **Performance:** C-U6-15's cheap wins each with the rust-reviewer's bench before and after (the judged bench stays
  R2's); TickLoop's and WsServer's per-tick paths allocate nothing in steady state (a counting-allocator test).
- **End checks at every block end:** check.mjs's steps on both builds, clippy and fmt, the wasm32 check, the gate;
  the disk lines (no build call under 5.5 GB, a whole-workspace run only at 8.8 GB free, the 4 GiB floor).

## 4. The carries, by number
All of the row's carries (C-U6-1 to -19, U3c's WsServer bullets, the reader stamp, -35) and the register entries of
section 0 are met by number in their sub-unit's report; U6's close report folds the four (and U6j's).

## 5. The rules that stand
- Plan §4.1-4.2's layout and dependency rules, §4.7's public surface, §4.8's exact pins and audits;
  `.claude/rules/netcode.md` for crates/server and crates/net; no client string verbatim in logs (E22).
- DEP-HOLD per sub-unit (section 1); the shared target and lane lock; never cd into a builder's copy.
- **His stop B** before any VM: the control plane, the page host and the sudo wrappers reviewed (plan.md:1267-1270).
- **C-U6-19's gate line:** U4g2's brief, when written, states in one line that its runs have no throwing or rejoining
  hostile client and no client restarted mid-run (as U4g, U4h and U5 do: u6-facts.md:366-367).

## 6. Size
SIZE-CHECK (history.md:2048-2051): M about 3,000 added lines, the stop past 6,000. U6's blocks come to about 9,500 to
10,000, plus about 600 for the bots' glue and npt-load and about 1,400 to 1,900 for the judge figures
(u6-facts.md:220-222). The row grew after rev 3.3: the nineteen C-U6 carries and U3c's send-path bullets came from
later reviews, and the facts show npt-server, the host and the clients mostly unbuilt. Built whole, U6 would pass the
6,000 stop about two thirds of the way through. The split (section 1) is a plan change, so it is his (as U4G-Q1 and
U4H-SIZE were: advisor-u4g-brief.md:188-193): U6-SIZE in section 8. Each sub-unit's stop is about twice its estimate,
the first keeping M's 6,000 (U4g2's and U4h2's form).

## 7. Reviewers and rounds
U6a: netcode-architect, security-auditor (the session and `JoinAccepted` change: SA-F1-1 stays open until it closes),
rust-reviewer, test-engineer, perf-auditor (TickLoop). U6b: netcode-architect, security-auditor, rust-reviewer,
test-engineer, perf-auditor (the send path). U6c: security-auditor, infra-engineer (units, wrappers, sudo list),
rust-reviewer, test-engineer. U6d: security-auditor (host, ACME), infra-engineer, netcode-architect (the probe),
test-engineer, rust-reviewer. U6j: test-engineer and rust-reviewer read every judge line. engine-architect on each
sub-unit that adds a crate dependency edge. Two rounds at most each (the review budget).

## 8. Questions
**His (two; neither a stop now: U6 starts after U4 and U5; for a day's message after this brief is checked):**
```text
U6-SIZE: how to build unit U6 (the game server, the two network transports, the control plane and the page host).
The plan calls it medium (M): about 3,000 added lines, your stop past 6,000. Counted block by block from the plan's
U6 row and the code as it stands, it is about 9,500 to 10,000 lines, plus about 600 for the bots' network side and the
load generator. The row grew after the plan was approved: nineteen follow-ups from later reviews and the WebSocket
send-path work were added to it. Built whole, it would pass your 6,000 stop about two thirds of the way through.
(a) Build U6 whole: one review and one merge, but your stop would have to move to about 20,000 (twice the plan), a new
    number from you.
(b) Split it into four sub-units, built one after another, each with its own short brief, review and merge: U6a the
    room server's core (the tick loop, the session and join changes, the room's speed fixes, the server program:
    about 2,850 lines; stop past 6,000); U6b the two transports on the test network (about 2,900; stop past about
    5,800); U6c the control plane, measurements, logs and process control (about 2,700; stop past about 5,400); U6d
    the probe, the page host, its certificates and the load generator (about 2,800; stop past about 5,600).
No pass mark, judge rule or marks.json text changes under either, and no run is dropped.
My lean: (b).
```
```text
U6-JUDGE: the judge gains the computations U6's network runs need. marks.json already names 20 figures the judge
cannot compute yet (the cost of keeping clients in sync, bits per client down and up, memory and queue growth, the
timing of a killed room, how well the two round-trip times agree, the control stream's delay under bulk load, the
time to close after a crash, egress bytes against the formula, the cost of logging, the allocation twin), and three
judge changes from earlier reviews wait for U6 (the twin check's second form for -30, the allocation twin's role,
-19's join coverage). Their formulas go in the series file; each gets pass, fail and gap test files, and the
test-engineer and the rust-reviewer read every judge line. This changes the judge's code, not marks.json, a pass mark
or a number.
(a) Yes, inside U6's sub-units, each with the runs it serves.
(b) Yes, as its own small change (the judge only, about 1,400 to 1,900 lines; stop past about 3,800), reviewed and
    merged before the network runs that need them. It shares no file with U6's sub-units.
(c) No: those figures stay not-run, and the three changes come back to you.
Only the network runs (after U8 and your stop B) wait for it.
My lean: (b).
```
**The advisor's (TEST-FIRST rule 2), with my leans; u6-facts.md:326-367's numbers:**
- Q2, npt-bots' npt-net glue and npt-load: in U6d (u4-build-plan.md:966-967 gives them to U6; the row's omission is
  an oversight; the load runs need them before R2).
- Q4, interest under load: nothing new to build beyond U6c's per-second counters; -8, -9, -10 and -12 are judged on
  R2 on npt-room's interest code (plan.md:464, :724-725).
- Q15, C-U6-15: the cheap wins are U6a's code with the bench before and after; the judged bench is a pre-run step of
  R2's plan (U8).
- Q16, the unit file and wrappers: in a new ops/units/ (§4.1 gains one line), `User=npt-room-NN` as plan.md:1078
  words it, checked at stop B.
- Q17, the WebTransport dev certificate: made by the room server at start (it holds the endpoint; the supervisor
  stays a process-control stand-in).
- Q18, instant-acme: no 90-day fallback now (the shortlived profile, failing closed); the root store for
  hyper-rustls goes to security-auditor's read at U6d's block 0.
- Q13, -35's swap: U6b makes the transport choice a feature npt-net turns on by default for itself, so a swap
  changes only crates/net; if that proves impossible, the advisor before the run (row:46).
- Left to each sub-unit's brief, chosen there with the advisor: C-U6-3's drain cadence (Q5), C-U6-16 (a)'s limit form
  and (c)'s channel (Q6, Q7), C-U6-19's form (Q8), C-U6-14's generation or test (Q9), C-U6-11's span start (Q10),
  C-U6-6's rate (Q11), the probe's home (Q12), the POOL (Q19), carry 60's field (Q20).
- Q14, drift: the reader stamp's site moved from delay.rs:1884 to :1907 (the same closing `wake_all` in `poll`,
  :1709); U6b's brief cites the new line.
- Q21: U4g2's brief carries C-U6-19's line (section 5).

## 9. For the main session
- Now: this brief to the advisor; after its check, U6-SIZE and U6-JUDGE in the next day's message; the register
  gains nothing until a sub-unit's brief adds carries.
- At each sub-unit's launch: its brief (blocks from section 1), the `<base>` fill, the runner copy, block 0's DEP-HOLD
  read where packages change, the size line on the card.
