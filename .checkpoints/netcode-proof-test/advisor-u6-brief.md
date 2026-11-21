# Advisor check: U6's brief rev 1 (2026-11-21)
General agent standing in for the advisor (main's `.claude/agents/advisor.md`, read via `git show origin/main:`).
Subject: `.checkpoints/netcode-proof-test/u6-brief.md` rev 1 (164 lines). Read-only except this file.

## Plan
1. Read the brief, U6's row (split copy), u6-facts.md; spot-check the facts' key claims (size counts, what exists,
   new packages, lane overlap, the 20 figures, the question numbers) against the code and plan.md.
2. Read the precedents: u4-build-plan.md (U4 split, :966-967), advisor-u4g-brief.md:188-193, u4g/u4h briefs §6,
   u5-brief.md (form, U5-SIZE :375-395), u4h-brief.md:350-367 (U4H-JUDGE).
3. Read Mustafa's TEST-FIRST (history.md:2234-2282), SIZE-CHECK (:2048-2051), WAIT-PLAN (a) (:7511-7516).
4. Judge items 1-4 of the task; write notes per item; verdict; DONE.

## Notes
- Read: the brief (164 lines), the row's split copy (46 lines), u6-facts.md (369), history.md:2040-2055 (SIZE-CHECK),
  :2234-2282 (TEST-FIRST), :7507-7516 (WAIT-PLAN (a)); advisor-u4g-brief.md:188-219; u4h-brief.md:253-368;
  u5-brief.md:301-407; advisor-u4f-f1-r1.md:100-133; u4-build-plan.md:960-967, :1074-1078; plan.md:459-482, :505-530,
  :569-612, :960-997, :1076-1166, :1240-1275; main's CHECKPOINTS.md §3 (DEP-HOLD).
- Facts spot-checked against the code (all hold): server/src/main.rs:1-6 stub; proto/src/msg.rs:98-113 JoinAccepted
  with no event base or press counters; room.rs:762-1028 the calls (`join` :814 returns `(u8, bool)`, `receive` :890
  takes no generation, `take_control` :784 bare bytes); input.rs:287 `if let Some(prev) = &self.accepted` (first command
  unchecked); delay.rs `poll` :1709, closing `wake_all` :1907 (row cites :1884: Q14's drift is real); figures.rs:866-966
  20 `"U6"` planned figures; figures.rs:190-200 `twin_hash_mismatch_count` refuses a non-`twin` role ("-30's form comes
  with U6"); link/src/lib.rs:64-88 `Link` with no `loss`, no send-buffer space, no `Send`; Cargo.lock has no
  instant-acme, hyper*, http-body-util, metrics*, webpki-roots; rcgen :1191 and tokio-tungstenite :1717 present;
  net/Cargo.toml: futures-util a dev-dependency, no `default` feature, no `bites` feature.
- Not in the facts or the brief, found here (they drive ADV-U6B-1):
  - **Rate adaptation is U6's and has no block.** plan.md:517-521 (§4.4) and :577-579 (§4.7): `loss` and send-buffer
    space "join at U6 with rate adaptation, their first consumer" (advisor-u4.md:202, Q6 U4-LOSS (b));
    u4-build-plan.md:966 lists "`loss`, the send-buffer space and rate adaptation (Q6)" for U6; :1076-1078 names its
    marks, -23 and -24 on L2 (R3). ADR 0001:450: "quinn's RTT and loss feed rate adaptation; under pressure the server
    drops low-priority entities first, then falls back to 20 Hz snapshots". The room has none of it today (no
    BufferFull, RTT or loss use in crates/room/src; `snapshot_hz` only at room.rs:860). T6 adds the Link items without
    their consumer.
  - **The room process's log series are mostly unwritten.** Of the 65 SERIES.md rows whose writer column is npt-server,
    51 have no string-literal writer anywhere in crates/ (my grep of each name's last segment; a heuristic, so a few may
    be built another way): among them `tick.v.region_query_ns`, `rank_pack_ns`, `pair_test_per_pair_ns` (SERIES.md:39,
    :44, :46), `event.reflood.time_ns` (:193), `second.s.down_bits` (:104), `tick.v.tick_allocs_count` (:55),
    `tick.v.server_only_ns`, `sim_ns`, `lagcomp_ns`, `overrun`, `second.v.quinn_rtt_ns` (:401), the panic and stall
    events (`event.tick_panic.*`, `event.conn_panic.*`, `event.stall_1s.*`, `event.stall_200ms.*`), `event.rotation.*`,
    `event.burst_return.*`, `event.no_join_close.*`, `event.slow_loris_close.*`, `header.params.cert_validity_ns`. The
    npt-room-written interest rows are unwritten too: `tick.s.nearest8_missing_count` (:64), `relevant_starved_count`
    (:68), `relevant_count` (:397), `tick.v.pair_count`, `tick.v.reflood` (:45), `header.params.area_count`. These are
    -8, -9 (marks.json:2742, :2845), -10, -12's inputs: "interest under load" is these writers, so Q4's "nothing new to
    build" does not hold. S4 (~400) and S5 (~350) are sized without them.
  - **E23's panic and stall injections** (plan.md:482: "panic and stall injections" as room-wide controls; -59,
    PM-0004-67) have no block: S7 parses `RoomControl`, nothing injects.
  - **-30's C part needs a runner and a judge change.** -30's C part (marks.json "C: loopback, 3 recordings x 2
    transports") replays recordings over both real transports (no block builds that replay over WtClient/WsClient), and
    its check is `twin_hash_mismatch_count` (marks.json -30 checks), which refuses -30's twins until C-U6-9 (U6j's J1).
  - **E14's network half** (plan.md:473: 1,000 never-joining sessions, oversize datagrams and stream openers, slow-loris
    on the game ports and on TCP 31000 and 80, the /64 cycler; -25 at R4, plan.md:821) is named by no unit row.
- Rev 3.3's U6 row (b90fb7e, 2026-11-07) already holds the WsServer send path's first eight bullets; what came after
  are U3c's round carries (2026-11-08), the reader stamp, C-U6-1 to -19, -35's run and carry 60's weapon and health.

## Item 1: size, grouping, order, size lines
- **The arithmetic does not hold (ADV-U6B-2, Medium).** The facts' own block estimates (u6-facts.md:160-218) sum to:
  U6a (S1 750, S2 800, S3 600, S8 300) **2,450**, not 2,850; U6b (T1 750, T2 350, T3 400, T4 700, T5 400, T6 150)
  **2,750**, not 2,900; U6c (S4 400, S5 350, S6 350, S7 700, H3 300, H4 250) **2,350**, not 2,700; U6d (P1 750, P2 150,
  H1 650, H2 250, bots and npt-load 600) **2,400**, not 2,800: the brief's 2,800 is the facts' 2,200 (which held J1's
  400) plus 600, so J1 is counted in U6d and again in U6j. The four estimates sum to 11,250 while §6 and the U6-SIZE
  text say "9,500 to 10,000 plus about 600". §6's "1,400 to 1,900 for the judge figures" is J1 plus the figures (the
  facts say the figures are ~1,000-1,500, u6-facts.md:221-222), while J1 already sits in the 9,500-10,000. The facts'
  "natural splits" subtotals (:224-227) were the source of the drift; they do not match its blocks.
- **The block list is incomplete (ADV-U6B-1, High: the number sent to Mustafa would not hold, and Q4's lean would leave
  R2's interest marks with no writer).** Notes above: rate adaptation (U6's by the Q6 ruling; -23, -24), the interest
  writers, the ~50 other npt-server series, the panic and stall injections, -30's C runner; E14's floods unplaced. My
  rough additions: rate adaptation 300-500, the interest writers with a clock-free phase seam in the room 250-450, the
  other series writers and the injections 400-800, -30's C runner 150-300: about 1,100-2,050 more. Corrected whole
  without the judge: about **11,000 to 12,000** (rough; the recount decides). The judge's estimate may be light at its top:
  U4h's judge change priced its four figures at 260-550 (u4h-brief.md:76, :265), about 65-140 a figure, so 20 figures
  plus J1's 400 come to about 1,700-3,200 against the brief's 1,400-1,900; the recount shows its basis.
- **The grouping is sound in shape.** Four pieces by layer (room core; transports; control plane and instrumentation;
  probe, host and load) each touch a coherent crate set, each ends compiling with tests, and each has a natural review
  panel. Homes for the additions: rate adaptation in U6b beside T6 (the Q6 principle: `loss` joins with its consumer);
  the interest writers in U6a with PhaseTimers (S1; Q4's ruling); the other series writers and the injections in U6c
  (S4, S7); -30's C runner in U6b (its first run); E14's floods in U6d (Q2's ruling). My rough piece sizes after that
  and S8's split: U6a about 2,500-2,800, U6b about 3,000-3,500, U6c about 2,800-3,200, U6d about 2,600-2,800 (the
  paste-ready text below uses them until the recount replaces them). If U6c's recount passes about 3,500, move H4 (the
  unit file and wrappers) to U6d beside H2: `npt-cert` wraps H2's client, and stop B reads both together. A sub-unit
  that still passes about 3,500 after that takes a fifth piece; four stays right while each is near 3,000.
- **The order is right, and for one more reason than the brief gives.** U6a first: WsServer's per-tick hand-off (W18,
  F11) and the probe's poller (row:24) need TickLoop's runtime; and C-U6-17 (row:42: "before an honest client runs on a
  link other than L1") sits in U6a's S3, so it is in before U6b's loopback tests and -30's C run put honest clients on
  L2. U6b before U6c: S6's AddrTable, handshake cap and deadlines sit in front of both accept paths. U6c before U6d: the
  probe writes `second.v.steal_permille` "as npt-server does" (C-U6-4, S4), and H2 sits behind H4's `npt-cert`. One seam
  to name in the sub-unit briefs: F7's accept-cap sizing (T3, row:14) and S6's handshake cap are one cap, built once.
- **S8 splits.** The WebTransport certificate's only consumers are WtServer (U6b) and npt-host (U6d); the binary on
  Loopback has no endpoint. U6a keeps the binary's settings, lifecycle, panic catch and non-zero exit (E2); the
  certificate, its rotation swap and the transport wiring go to U6b (Q17 below).
- **The size lines follow U4g's and U4h's form** (first piece keeps M's 6,000; the others about twice their estimate:
  u4h-brief.md:269-271). Accepted. Two fixes: the brief calls it "U4g2's and U4h2's form" (it is U4g's and U4h's split
  form); U6j's line is twice the middle, as U4H-JUDGE's was (400-900, stop past 1,300), so about 3,300 at 1,400-1,900,
  not 3,800 (twice the top). At the units' growth so far (U4f 1.5 whole, U4f2 1.35: u4h-brief.md:266-268) every piece
  stays under its stop at the corrected estimates.

## Item 2: U6-SIZE and U6-JUDGE (his)
- **Both are his.** U6-SIZE: a split is a plan change (advisor-u4g-brief.md:191-192; U4G-Q1 and U4H-SIZE went to him),
  and (a) needs a new stop, his number (SIZE-CHECK). U6-JUDGE: TEST-FIRST rule 2, "What still waits for me: any change
  to a pass mark, to the judge or to marks.json" (history.md:2262); U4H-JUDGE is the precedent. Neither is a stop: U6
  starts after U4 closes and U5 merges. Neither can't-wait: no ESCALATE.
- **My lean: (b) on both.**
  - U6-SIZE (b): at about 11,000-12,000 lines whole, (a) is a certain mid-build stop unless he sets a new number near
    23,000, and one review panel over four programs, the control plane and sudo wrappers among them, reads worse than
    four panels of about 3,000. The pieces' build order is the same under (a) and (b), so nothing waits for his answer
    until U6a is built.
  - U6-JUDGE (b): the judge stays one coherent change read line by line by test-engineer and rust-reviewer; its inputs
    are already defined (all 20 figures have SERIES.md rows: each name's row found once, e.g. :262, :264, :278, :279,
    :288, :294), so it builds on made-up logs as U4H-JUDGE does; and J1's C-U6-9 must merge before -30's container
    check after U6b, which (b) makes easy and (a) ties to U6b's panel.
- **U6-SIZE's text, what is wrong (ADV-U6B-5, Medium):** its numbers inherit ADV-U6B-1 and -2; "The row grew after the
  plan was approved: nineteen follow-ups ... and the WebSocket send-path work were added" overstates: the send path's
  core is in rev 3.3 (b90fb7e), what came later is its follow-ups, and the main cause is that M was set before the work
  was counted (npt-server, the transports' servers and clients, the host and the control plane unbuilt); it lacks the
  growth line (U4f 1.5, U4f2 1.35) and the "nothing waits" line both precedents carry (u4h-brief.md:340, :346-347); it
  lacks (a)'s review cost and the "cheaper option" line; "the test network" is jargon. The pieces' stops: twice the
  estimate as the brief has it, but never above his 6,000 (a piece near 3,000 is itself an M; at the recount U6b or U6c
  may pass 3,000, and twice that would exceed M's own stop).
- **U6-JUDGE's text, what is wrong (ADV-U6B-3, Medium):** "Their formulas go in the series file": they are already
  there; "Only the network runs (after U8 and your stop B) wait for it" is false: -30's container check after U6b uses
  `twin_hash_mismatch_count`, which refuses -30's twins until C-U6-9 (figures.rs:190-200; marks.json -30's C part); (b)
  "shares no file with U6's sub-units": SERIES.md is shared (U6d adds C-U6-5's names, U6b the reader stamp's;
  u6-facts.md:322); C-U6-11's span start is a rule in marks/README.md (:79-80, "a client admitted later is -19's join,
  built with U6"), a judge-contract text his yes must name; C-U6-9 changes an existing SERIES.md row (:297); (a)'s cost
  and (c)'s loss (12 marks not-run, plus -30's container check and -19) are not stated; "the timing of a killed room"
  is wrong (kill_p99_ratio is the other rooms' p99 after the kill, figures.rs:912-916); U6j's numbers follow the
  recount (ADV-U6B-1).
- **Paste-ready texts** (rev 2 replaces each number with its recount where it differs; my figures are rough):
```text
U6-SIZE: how to build unit U6 (the game server program, the two network transports, the control plane, the page host
and its certificates, and the load generator).
The plan calls U6 medium (M): about 3,000 added lines, your stop past 6,000. That size was set before the work was
counted. Counted block by block now, from the plan's U6 row and the code as it stands, U6 is about 11,000 to 12,000
lines: the server program, the WebTransport and WebSocket servers and clients, the control plane and the page host are
still unbuilt, and about thirty follow-ups from later reviews were added to the row after you approved the plan.
Units so far came to about 1.35 to 1.5 times their estimates. Built whole, U6 would pass your 6,000 stop about halfway
through.
(a) Build U6 whole: one review and one merge, but your stop would have to move to about 23,000 (twice the estimate's
    middle), a new number from you; and one review of 11,000 to 12,000 lines across four programs, the control plane
    and its admin commands among them, is harder to do well than four reviews of about 3,000.
(b) Split it into four parts, built one after another, each with its own short brief, review and merge, each stopping
    past twice its estimate and never above your 6,000:
    U6a the room server's core: the tick loop, the session and join changes, the room's speed fixes and its
    measurements of who sees whom (about 2,500 to 2,800 lines; stop past 6,000);
    U6b the two transports on this machine's simulated network, and the server sending less when a link is congested
    (about 3,000 to 3,500);
    U6c the control plane, the server's measurements and logs, and its test controls (about 2,800 to 3,200);
    U6d the timing probe, the page host and its certificates, process control, the load generator and the hostile
    test clients (about 2,600 to 2,800).
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
(b) Yes, as its own small change (the judge only, about 1,700 to 3,200 lines; stop past twice its middle), reviewed and
    merged early, beside U6's first parts. It shares only the series file with them, each change on its own rows.
(c) No: those 20 figures stay not-run, so 12 marks cannot be judged; -30's container check and -19 cannot be judged
    either, and the three changes come back to you.
-30's container check, right after U6's transport part, waits for the first of the three changes; the 20 figures serve
only the network runs after U8 and your stop B.
My lean, and the advisor's: (b).
```

## Item 3: the advisor's questions (Q2, Q4, Q13, Q15-Q18, Q21) and the deferrals
Each is inside the plan, undoable, changes no pass mark, judge or marks.json, costs nothing and touches nothing on
main (TEST-FIRST rule 2), so each goes by this ruling, recorded with its reason and named in the day's message.
- **U6-Q2 (npt-bots' npt-net glue and npt-load): Ruled, U6d,** as leaned: u4-build-plan.md:966-967 gives them to U6
  "with the clock bans' reviewed allows (§4.11)", and the allows come with them; after C-U6-18 (U6a's S3: one entry
  point for a Behaviour's host order "before npt-bots and npt-load host Behaviours", row:43). E14's network floods
  (plan.md:473; -25 at R4), which no unit row names, go with them in U6d (they are npt-bots clients on npt-net's
  clients); its `ws://` and wrong-Origin pages go with U7's web harness. Each is counted in the recount.
- **U6-Q4 (interest under load): Ruled differently from the lean.** "Nothing new beyond U6c's per-second counters"
  does not hold: the series -8, -9, -10 and -12 read have no writers (Notes). Interest under load is: npt-server's
  `tick.v.region_query_ns`, `tick.v.rank_pack_ns`, `tick.v.pair_test_per_pair_ns` and `event.reflood.time_ns`, timed
  by PhaseTimers through a phase seam the room exposes without reading a clock (§4.4: npt-room counts ticks, no clock);
  npt-room's `tick.s.nearest8_missing_count`, `tick.s.relevant_starved_count`, `tick.s.relevant_count`,
  `tick.v.pair_count`, `tick.v.reflood` and `header.params.area_count`. Home: U6a beside PhaseTimers (S1), since it
  opens room.rs anyway; the seam's form chosen in U6a's brief with the netcode seat; each writer tested on Loopback
  with a fixed scene. -8, -9, -10 and -12 are judged at R2 (plan.md:724-725) on npt-room's interest code as it stands.
- **U6-Q13 (-35's swap form): Ruled as leaned, with four terms.** (1) npt-net's own `default` feature names the one
  WebTransport crate; no dependent sets `default-features = false` on npt-net or names a `wt-*` feature, held by a test
  or check.mjs line; (2) with both features on (all-features builds and tests), the rule for which crate WtServer uses
  is stated and tested; (3) U6b's end runs U5's tool uncounted on a scratch swap (never committed) and shows only
  crates/net changes; (4) the one counted run follows U6d's merge, not U6b's (ADV-U6B-4). A swap that needs a line
  outside crates/net comes to the advisor first (register 111), to him if it changes what counts.
- **U6-Q15 (C-U6-15): Ruled as leaned, with one term.** The cheap wins are U6a's code, each with the rust-reviewer's
  U4e bench (row:39's method) before and after in the container, reported and never judged; the judged measurement is
  R2's pre-run step (U8). Term: over the bench's ticks the room's state hash is unchanged, and a win that changes the
  encoded bytes (the relevant set's order among them) names every pinned file it moves (det-u4, the recordings) with
  its reason, to the advisor before U6a merges.
- **U6-Q16 (the unit file and wrappers): Ruled as leaned, with two terms.** ops/units/, and §4.1 gains that one line;
  `User=npt-room-NN` (plan.md:1078 allows it or DynamicUser; fixed users match the slot directories' 0700 ownership,
  :1081). Terms: the files either fit U8's 32 KiB cloud-init check or are installed from the clone (U8's choice,
  stated in U8's brief); infra-engineer and security-auditor read them in the sub-unit's round (the sudo list against
  GTFOBins) and again at stop B. If the recount needs it, H4 moves to U6d beside H2.
- **U6-Q17 (the WebTransport dev certificate): Ruled, the room server makes it, with four terms.** (1) The key never
  leaves the slot's 0700 directory; only the hash goes to a path npt-host's user can read; (2) PM-0006-26's next
  certificate and its swap on a live endpoint (new handshakes on the new one, the 48 live sessions kept) are designed
  with WtServer, so the certificate part of S8 moves to U6b with the binary's transport wiring, and U6a keeps the
  binary's settings, lifecycle, panic catch and non-zero exit; (3) npt-host's `/npt/cert-hash.json` (plan.md:994-995)
  is U6d's; (4) PM-0006-27's 15-day certificate is made the same way on a setting recorded in the header.
- **U6-Q18 (instant-acme): Ruled, no 90-day fallback, with three terms.** The plan's default stands (`shortlived`,
  failing closed: plan.md:966-968); if R1's staging read (plan.md:970-971) finds no `shortlived`, the fallback comes
  back to security-auditor and the advisor then (C11, :969). Terms: the root store for hyper-rustls is chosen at U6d's
  Block 0 by security-auditor's read; U6d confirms instant-acme 0.8.5's revocation call by name (plan.md:967); its
  tests run against an in-process fake directory (no network to Let's Encrypt here), the first real order being R1's
  staging dry run.
- **U6-Q21 (C-U6-19's line): Ruled, widened.** Under U6-SIZE (b), C-U6-19 closes at U6a's merge, so its gate's "every
  unit brief before U6's merge" (advisor-u4f-f1-r1.md:120-122) reads "before U6a's merge": U4g2's brief, U4h2's under
  U4H-SIZE (b) (its thrower bots throw, so the line matters), the U4h judge change's under U4H-JUDGE (b), any change
  F-B-COUNT (b) makes, and U6j's if it is briefed before U6a merges, each state in one line that its runs hold no
  throwing or rejoining hostile client and no client restarted mid-run, or bring C-U6-19 first.
- **U6-Q14 (the drift):** confirmed (delay.rs:1907 in `poll` :1709); U6b's brief cites the new line.
- **The deferrals (Q5-Q12, Q19, Q20) to each sub-unit's brief: confirmed,** chosen there with the advisor, with three
  notes. Q8 (C-U6-19's form) takes the netcode and security seats' read as the carry says (row:45). Q10 (C-U6-11's span
  start) is a marks/README.md rule: chosen in U6j's brief with test-engineer's read, inside his U6-JUDGE yes because the
  corrected text names it; a form that changes what -19 counts beyond "from its first snapshot, as the warm-up rule
  words it" comes to him. Q11: if -3's sends and echoes cannot give 10,000 a direction, the advisor before R1's run plan
  (C-U6-6). Q12: §4.7 lists npt-lab's `probe` (plan.md:612) and no probe item in npt-server (:600-601); a home in
  npt-server adds a public item, so engine-architect reads it.

## Item 4: missing or wrong
- **ADV-U6B-4, Medium: -35's counted run is placed after U6b.** Register 110 says "The one counted run follows U6's
  merge, "once, after the swap" (marks.json:7321)", and -35 judges that "switching WebTransport crates changes no code
  outside the transport module": U6c (handshake refusals per reason, metrics) and U6d (the probe on quinn's socket)
  can still add crate-bound code outside crates/net. Under the split, "U6's merge" is U6d's. Fix §2 and §1: the counted
  run after U6d's merge; uncounted tool runs as checks at U6b's end and each later piece's end (Q13 term 3). Under
  U5-SIZE (b), U5b (the swap checker) is built before that run; the brief says so in one line.
- **-30's container part** waits for U6b and for U6j's J1 (C-U6-9): §2's "after U6b" gains "and U6j's J1"; §1's "U6j
  ... merged before the network runs that need its figures" gains "and before -30's container run".
- **ADV-U6B-6, Medium: C-U6-19's gate** (Q21's ruling): §2 says it closes in U6a's S2 (right), so the gate's "before
  U6's merge" reads "before U6a's merge", and §5's line covers U4g2, U4h2 under U4H-SIZE (b), the U4h judge change, any
  F-B-COUNT (b) change and U6j if briefed before U6a merges, not U4g2's alone. SA-F1-1 stays open until U6a merges.
- **ADV-U6B-7, Low: DEP-HOLD per sub-unit** is wider than "a sub-unit that adds packages": main's CHECKPOINTS.md §3
  holds any change to a `Cargo.toml` or `Cargo.lock` (a new direct dependency already in the lock, a feature, a version).
  Every piece likely has a Block 0: U6a (npt-server's runtime and its own manifest), U6b (tokio-tungstenite and
  futures-util as normal dependencies, npt-net's `bites` and `default` features, rcgen for the certificate with
  aws-lc-rs kept off), U6c (metrics, metrics-exporter-prometheus, npt-alloc as a normal dependency: register 104 (4)),
  U6d (instant-acme's stack, npt-bots on npt-net). Wording: "a sub-unit that changes any manifest or lock starts with
  Block 0".
- **ADV-U6B-8, Low: reviewers.** State each at xhigh (netcode work: CLAUDE.md), at most three at a time. U6c gains
  netcode-architect (S6's wall-time limits and deadlines, the tick-edge timers, the injections are server and protocol
  work) and perf-auditor (instrumentation and the allocator twin on the tick path). engine-architect joins any piece
  that adds a public item or changes a public trait, not only a dependency edge: U6a (a framed message's channel read,
  C-U6-16 (c); a session generation or end-session call, C-U6-14; the room's phase seam, Q4), U6b (`Link` gains
  `loss`, send-buffer space and `Send`), U6d if the probe lands in npt-server (Q12).
- **ADV-U6B-9, Low: S8's split and seams** (Q17): the certificate and transport wiring to U6b; F7's accept-cap sizing
  and S6's handshake cap built once (item 1); C-U6-12's control-message limit applies on both transports, not WtServer
  alone; carry 60's header fields named in one block (S4 builds the header; T5 lists them too).
- **ADV-U6B-10, Low: the register and the plan.** §9's "the register gains nothing until a sub-unit's brief adds
  carries" changes: as with U4h (entries 94-101) and U5, the rulings above go into the register's U6 section now
  (next free numbers, each quoting this file), and the C-U6-19 line into U4g2's and U4h2's sections when they are
  written. The plan's U6 row gains one line once his U6-SIZE answer is in (as SIZE-U4F's U4f2 line and U5-Q2's lines):
  under (b), the four pieces with their size lines and -35's counted run after U6d; under U6-JUDGE (b), U6j's line;
  §4.1 gains ops/units/ (Q16). The card writes each piece's size line with its source before its first snapshot
  (SIZE-CHECK).
- **Stop B, the lane wait, the gates otherwise: right as written.** Stop B before any VM (plan.md:1267-1270), the
  sudo list against GTFOBins there; nothing built before U4 closes and U5 merges (the pieces share manifests, lock,
  check.mjs, crates/net/Cargo.toml, room.rs and crates/bots with U4g2, U4h, U5: u6-facts.md:295-322); U6j "beside"
  runs under the lane lock and the disk lines, as U5-EARLY did, and after the U4h judge change takes register 64.
  C-U6-17 in U6a before any honest client off L1: right.
- **The brief's form** follows TEST-FIRST rule 3 and u5-brief.md's shape: it points to the row, the facts and the
  plan and repeats little. Good.
- **Two nits (Low):** "U4g2's and U4h2's form" is U4g's and U4h's split form; U6j's stop is twice the middle (about
  3,300 at 1,400-1,900; the recount's middle if it moves).

## Verdict
**APPROVE WITH CONDITIONS.** The split into four pieces built one after another, their order (U6a, U6b, U6c, U6d),
U6j as a judge change of its own, and the size-line form are sound; both decisions are his and both leans, (b), stand.
The numbers and the two texts do not hold yet, and the block list misses plan items U6 owns, so rev 2 comes back for a
short check of the changed sections before anything reaches him. Nothing is built now (U6 waits for U4 and U5).

Findings: ADV-U6B-1 High (blocks missing: rate adaptation, the room process's log series, the injections, -30's
runner, E14's floods; Q4's lean does not hold); -2 Medium (the sums: U6d counts J1 twice, the pieces sum to 11,250
against the text's 10,100-10,600); -3 Medium (U6-JUDGE's text); -4 Medium (-35 after U6b); -5 Medium (U6-SIZE's text);
-6 Medium (C-U6-19's gate under the split); -7 to -10 Low (DEP-HOLD wording, reviewers, S8 and seams, register and
plan lines).

Conditions (each met by an Edit in u6-brief.md rev 2):
1. Recount and complete the blocks (ADV-U6B-1, -2): add rate adaptation (U6b), the interest writers (U6a, Q4), E23's
   panic and stall injections and the other npt-server series writers (U6c; each piece's brief names the SERIES.md rows
   it writes, and U6's close report shows every npt-server row written or named for a later unit), -30's container
   runner (U6b) and E14's floods (U6d); fix the sums (U6d without J1; §6's judge figure); show U6j's basis against U4h's
   per-figure cost; a piece past about 3,500 moves a block (H4 to U6d first) or the brief adds a fifth piece.
2. U6-SIZE's text becomes this file's (item 2), with the recount's numbers; each piece stops past twice its estimate,
   never above his 6,000.
3. U6-JUDGE's text becomes this file's (item 2), with the recount's numbers.
4. -35's counted run after U6d's merge, uncounted tool checks before it; -30's container part after U6b and U6j's J1;
   one line on U5b's place under U5-SIZE (b).
5. C-U6-19 closes at U6a's merge, and §5's line covers every brief before it (U6-Q21's ruling).
6. DEP-HOLD: "a sub-unit that changes any manifest or lock starts with Block 0".
7. Reviewers at xhigh; U6c gains netcode-architect and perf-auditor; engine-architect on every piece that adds a public
   item or changes a public trait.
8. The rulings U6-Q2, Q4, Q13, Q15 to Q18 and Q21 folded with their terms (S8's split among them); §9 says they go
   into the register's U6 section now, and that the plan's U6 row (and §4.1) gain their lines once his U6-SIZE and
   U6-JUDGE answers are in.
9. Rev 2's sections 1, 6 and 8 come back to the advisor for a short check (numbers and the two texts) before U6-SIZE
   and U6-JUDGE go into a day's message. Not a stop: no build waits on it.

**Leans:** U6-SIZE (b); U6-JUDGE (b). Both his; neither can't-wait, so no ESCALATE. If the day's message is already
long, they can wait a day: nothing needs them until U6a is built.

**What I could not check:** the facts' block estimates are judgements, not counts (I checked their sums, not their
size); my series grep matches each name's last segment as a string literal, so a writer built another way would show
as missing (I expect few); I read no reviewer seat's view and ran nothing (no build, by the rules of this check).

DONE

## Condition 9: the short check of rev 2's sections 1, 6 and 8 (2026-11-21)
Read: u6-brief.md rev 2 (196 lines), whole, and its cites into this file (:27-33, :34-44, :64-71, :77-78, :81-86,
:128-174, :226-232, :246-252: each points at the passage it names).

- **Section 1, the sums: all hold.** U6a 750 + 250-450 + 800 + 600 + 150-200 = 2,550-2,800; U6b 750 + 350 + 400 + 700
  + 400 + 150 + 300-500 + 100-150 = 3,150-3,400; U6c 400 + 350 + 400-800 + 350 + 700 + 300 + 250 = 2,750-3,150; U6d
  750 + 150 + 650 + 250 + 600 + 200-400 + 150-300 = 2,750-3,100; whole 11,200-12,450; U6j 400 + 20 x 65-140 =
  1,700-3,200. U6d no longer holds J1. Every piece is under about 3,500.
- **Stops: all hold.** U6a keeps 6,000; U6b twice 3,275 = 6,550, capped at 6,000; U6c twice 2,950 = 5,900; U6d twice
  2,925 = 5,850; U6j twice 2,450 = 4,900. At 1.5 times each top stays under its stop (U6a 4,200; U6b 5,100; U6c 4,725;
  U6d 4,650; U6j 4,800 against 4,900, the closest), so section 6's sentence holds.
- **Section 6: holds.** The basis named (judgements, not counts; the additions; the judge at U4h's per-figure cost), M
  set before counting, about thirty follow-ups after rev 3.3 (b90fb7e), the growth line, "about halfway" (6,000 of a
  middle of 11,825 is 51 %), U4g's and U4h's split form, the split his.
- **Section 8: both texts hold.** U6-SIZE: whole "about 11,200 to 12,500" (12,450 rounded); (a)'s "about 24,000" is
  twice 11,825 = 23,650, rounded to the thousand: acceptable; the four parts' figures match the table; process control
  under U6c, the replay under U6d. U6-JUDGE: 1,700-3,200, stop about 4,900, (a)'s 400-800 a part (1,700-3,200 over
  four), 12 marks under (c), and the last line now "after U6's last part", which matches section 2.
- **Departure 1, -30's container runner in U6d: Ruled, accepted.** With it U6b would be 3,300-3,700, past the line I
  set; the runner hosts clients on WtClient and WsClient, which is B1's kind of glue, and its crate (npt-lab) is
  already U6d's. The cost is only timing: -30's container run moves from after U6b to after U6d and J1, still a
  container run long before U8 (register 88 asks only that -30 stays NOT-RUN until U6). U6b's own loopback tests still
  cover each transport. No register entry changes (my runner home was a note in item 1, not a ruling of item 3).
- **Departure 2, npt-super in U6c: Ruled, accepted; the error was mine.** My piece sizes (:77-78) kept H3 in U6c, and my
  text's "process control" under U6d contradicted them. H4 stays in U6c too (U6c is under 3,500, so the move I allowed
  is not needed); the order still holds, since H2 in U6d sits behind H4's `npt-cert`.
- **One Low, an optional Edit with no new check:** the U6-SIZE text gives only U6a's stop. As both precedents give each
  part's stop (u4h-brief.md:343-345), add them after the estimates: U6b "(about 3,150 to 3,400; stop past 6,000)", U6c
  "(about 2,750 to 3,150; stop past about 5,900)", U6d "(about 2,750 to 3,100; stop past about 5,850)".

**Verdict on rev 2's sections 1, 6 and 8: APPROVE.** Conditions 1-9 are met for the parts I checked; both departures
accepted; the Low above is optional. U6-SIZE and U6-JUDGE may go into a day's message as written (with or without that
Edit). Leans unchanged: (b) and (b), both his, neither a stop.

DONE
