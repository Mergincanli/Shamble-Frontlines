# U6a's brief, rev 1 (2026-11-22): the server core
**The unit:** the first piece of U6 as Mustafa's NPT-TRIM cut it (history.md:7561-7593; his answers of 2026-11-22,
history.md:7595-7607: RE-PLAN-OK yes, R1 on Thursday 3 December). Its row: replan-units.md:58 (S1, S8a, C-U6-17 and
-18, T6, S4: 1,675 estimated, 2,250-2,500 likely), first needed by R1 for -1, -2, -16 and PM-0006-11. Build: Mon 23 Nov
on the one cargo lane; round, fixes and merge Tue 24 (replan-units.md §7). Short by TEST-FIRST: what is built, how it
is tested, what is open; it points to the rest. Reference, never retyped: u6-brief.md rev 2.1 (the uncut brief),
u6-facts.md (file:line facts), advisor-u6-brief.md (its conditions), U6's row plan.md:1254 (it was :1253 when
replan-units.md and u6-brief.md cited it), u4f-brief.md sections 1 and 4 (the start, stops and build rules), and
security-r1-deps.md (the R1 path's dependency read, DONE: SEC-R1D-1 is U6a's, section 5).
**One advisor check** (NPT-TRIM), no re-check unless it finds a Critical or High. Section 9's seven questions are the
only asks; every other choice goes by its lean, one line each (sections 1 and 5).

## 0. Where things are
- **`<base>`:** the lane head the main session fills at launch, after Block 0 (U4f2 merged; U4's report in), with its
  spikes/ tree, its check.mjs counts (U4f2's head: 709 measured and 806 bites, u4f2-report.md:136, plus FP-10's) and
  this brief's sha256. Today (u6-facts.md §2, re-read at 69b518a): npt-server has `Decoder`, `Schedule`, `due_ticks`
  and a stub binary (server/src/main.rs:1-6), no tokio, no TickLoop; `Link` has no `Send` bound and both of its
  in-process backends are `Rc<RefCell<..>>` (link/src/lib.rs:133; l1/src/lib.rs:1747), so `Send` cannot sit on
  `Link` itself; npt-super is a stub; the room's relevant set is crate-private (room/src/room.rs:248); only npt-lab
  writes -2's series (lab/src/logs.rs:653-715, mirror.rs) and the machine header (lab/src/machine.rs, role `C`).
- **The marks U6a serves at R1** (marks.json): -1 (`event.affinity.alone` and `.sibling_free` from process `room`,
  start and end; `second.v.steal_permille` from RS; `machine_named`), -2 (the room's scene counts and
  `tick.s.everyone_in_view`, process `room`), -16 (`event.room_start.spawn_to_open_ns`, writer npt-super:
  SERIES.md:200; "timed by the supervisor's stand-in": ADR 0001:1604), PM-0006-11 (`machine_named`,
  `event.cpu_read.changed`, `sibling_free`). Bites: PM-0001-16/double-load, PM-0001-1/shared-vcpu,
  PM-0006-11/no-smt-map.

## 1. What U6a builds (blocks in order; each compiles with its tests passing before the next)
- **Block 0, U6's lock change and U6a's manifests** (lean: the main session's, on the lane before `<base>` is filled,
  because `cargo update` needs the network and builders run offline; the builder's tree then holds no manifest change,
  and any further one stops it). Two lock diffs, each checked on its own (section 5): (a) SEC-R1D-1's bump; (b) U6a's
  direct-dependency moves. Then security-auditor's answer on exactly that content, dependency-check.md, the save.
- **Block 1, T6 `Send`** (crates/link; ~70): `Loopback` becomes `Send` (shared state behind `Arc<Mutex<..>>`); a
  `Send` link is `Box<dyn Link + Send>`, which the I/O runtime holds; L1 stays single-threaded and not `Send`;
  compile-time asserts. `loss` and send-buffer space: Q-U6A-4 (lean: with U6d's rate adaptation).
- **Block 2, S1 TickLoop and PhaseTimers** (crates/server, proto; ~850). New `server/src/tick_loop.rs` and
  `phase.rs`: pub `TickLoop`, pub `PhaseTimers` (plan.md:601's list). The tick thread alone owns `Room<World>` and
  runs `due_ticks` on the monotonic clock (at most `LOOP_CATCHUP` ticks a wake, dropped time counted); a tokio
  multi-thread I/O runtime owns each session's link and `Decoder` and decodes there (plan.md §4.4); bounded channels
  both ways, a full one counted and never blocking the tick; per session per tick the room gets the decoded messages
  with arrival stamps, `Decoder::rtt_ns` (register 96), sent bytes and the largest datagram; `take_control`'s bytes
  go out on the channel npt-proto reads from the frame (new pub `npt_proto::channel_of`, C-U6-16 (c): Q-U6A-3); a
  kicked, left or replaced session is closed, the replaced one sent `Kick(replaced)` first (C-U6-14's first clause:
  Q-U6A-3); a tick-thread panic is caught (`panic = "unwind"`, plan.md E2), `RoomEnded(crash)` goes to every session,
  a crash record and a `crash` footer are written and the caller gets a non-zero end. PhaseTimers reads the clock at
  tick and phase edges into a fixed-size record per tick; U6a writes the loop's own five series from it (`tick.v`
  `work_ns`, `start_jitter_ns`, `dropped_ns`, `overrun`, `wake_ticks_count`); the room-phase series are U6d's and
  U6e's. **The clock:** one item reads it (lean: `server/src/clock.rs`'s `now_ns`, with the reviewed per-item allow
  server/clippy.toml asks for); everything else takes `now_ns`.
- **Block 3, S8a the binary** (crates/server main.rs; ~275): settings from one JSON file (`deny_unknown_fields`, a
  size cap, every value range-checked; tick rate 30 only, C-U6-16 (b) being moved), its fields the header's (run,
  set, role, marks, date, commit, layer, profile, seed, scene, clients, `cmd_timing`, the machine's role, name, plan,
  location and address), the tick and I/O vCPUs and the log path; per-slot tokens minted at start (16 bytes each from
  /dev/urandom; roster.rs:1-3: "minting is U6's"; the re-plan keeps the 0600 file, replan-units.md:24-25, and no piece
  named it) into a 0600 file for the run script, never logged; the room built, TickLoop started, one `OPEN` line on
  stdout; the end by a duration or stdin closing (no tokio `signal` feature); exit 0, 2 for bad settings, non-zero
  after a crash (U6-Q17 term (2)). The bite PM-0001-16/double-load reads the settings file twice, each read behind
  1 s, so spawn to Open passes 2 s (this brief's reading of the bite's words).
- **Block 4, S4 instrumentation and the header** (crates/server, log; ~520): the header through npt-log with
  `machine` from a new `npt_log::Machine::read_local` (the /proc and /sys reads lab/src/machine.rs makes, the caller
  passing role, name, plan, location, address and `clock_res_ns`; npt-lab's copy untouched), `clock_res_ns` measured
  as the smallest non-zero step of the monotonic clock, `build` with the binary's SHA-256, and carry 60's params;
  per second `second.v.steal_permille` (the tick vCPU's `/proc/stat` steal), `rss_bytes`, `proc_cpu_permille`,
  `io_cpu_permille` (the runtime threads' schedstat) and `second.s.out_queue_count`; events `affinity` (start, end:
  `alone` when no other thread of any process may run on the tick vCPU, per-CPU kernel threads excluded;
  `sibling_free` from the SMT map) and `cpu_read.changed`. Pinning: Q-U6A-5. C-U6-4: the steal reader is pub
  (`npt_server::StealReader`) so U6c's probe writes the series "as npt-server does"; the carry closes there. Bite
  PM-0006-11/no-smt-map: the header written without `smt`.
- **Block 5, the room log for -2** (crates/server, room; ~250; Q-U6A-2): a writer thread fed fixed-size per-tick
  records (marks/README.md §1: never on the tick thread) writes `tick.v` `soldiers_count`, `smallest_team_count`,
  `snapshot_clients_count`, `big_vehicles_count`, `very_large_units_count`, `projectiles_count`,
  `effects_per_soldier_max_count`, `second.v` `shots_count`, `portal_changes_count`, `reliable_events_count`,
  `projectile_spawns_count`, `stand_in_changes_count`, `stand_in_spawns_count`, `header.params.stand_ins`, and
  `tick.s.everyone_in_view` from a new pub `ClientSlot::relevant` getter over the set the room encodes.
- **Block 6, -16's start timer** (crates/super; ~180; Q-U6A-1): `npt-super time-starts`: spawns the room binary N
  times one at a time, times spawn to its `OPEN` line, writes `event.room_start.spawn_to_open_ns` in its own log
  (process `super`, machine from `Machine::read_local`), closes the room's stdin and records its exit; a 10 s Open
  deadline (a start past it is killed and its elapsed time written). npt-super keeps reaching no sim, room or proto
  (check.mjs:98-100). The 8-room density harness stays U6e's H3.
- **Block 7, C-U6-17** (crates/client lib.rs `plan()`, :621-665; ~100): the stamp capped at
  `next_tick_at(now + lo_rtt / 2) + 7` beside `max_lead`; its test in npt-server's tests (section 3).
- **Block 8, C-U6-18** (crates/bots lib.rs; ~100): one entry point, `Behaviour::host_step` (input, the endpoint's
  step, `outgoing`, in that order), and the module text naming it as the contract; npt-lab keeps its own call order
  this unit; U6c's npt-load hosts through it.
- **Choices by lean** (no ask): Block 0 on the lane by the main session; JSON settings; one clock item; Loopback
  `Send` by `Arc<Mutex>`; the loop's five series only; the machine reader in npt-log; the steal reader pub; tokens
  from /dev/urandom; C-U6-18 as an entry point; no existing SERIES.md row edited (register 98's writer cell stays for
  the next SERIES.md edit).
- **New pub items** (plan §4.7's whole-surface rule; the netcode seat reads them): npt-server `TickLoop`,
  `PhaseTimers` (listed), `StealReader`; npt-proto `channel_of`; npt-room `ClientSlot::relevant`; npt-log
  `Machine::read_local`; npt-bots `Behaviour::host_step`. Each named in the report with its reason.

## 2. What U6a does not build
- **U6b:** WtServer on web-transport-quinn, WsServer with fix 1, WtClient and WsClient, the connection cap (N + 8),
  the Join deadline over TLS, `bites`, SetProfile, the reader stamp, the binary's transport wiring and settings
  (`cc`, transport), the WebTransport certificate at start (S8b). Its pins (security-r1-deps.md:222-224):
  futures-util `=0.3.34` as a normal dependency of npt-net (CLEAR); rcgen `=0.14.10` direct with
  `default-features = false, features = ["ring"]` (S8b).
- **U6c:** the -3 probe and C-U6-1 to -8 (C-U6-4's writer side), npt-bots' npt-net glue, npt-load and calibration,
  the page host, DNS-01 issue. Its pins (H2; security-r1-deps.md:225-230): instant-acme `=0.8.5` with
  `default-features = false, features = ["ring", "hyper-rustls"]` and its 16 crates at the exact versions listed
  there; SEC-R1D-4, each ACME call under a tokio timeout.
- **U6d:** the interest writers (S1b), C-U6-13, the rest of C-U6-14 (a generation or the test that no closed
  session's message reaches the room), C-U6-16 (a), C-U6-19 and the `JoinAccepted` change, C-U6-15's cheap wins,
  rate adaptation (with `loss` and send-buffer space by Q-U6A-4's lean).
- **U6e:** ControlSocket, metrics and logs, the allocator twin (so no counting-allocator test here), the other series
  writers (lag compensation's two among them), the panic and stall injections, npt-super's density harness, -30's
  runner. **U6j:** the judge's figures and C-U6-9 to -11. **U7, U8:** the browser client; the run package (it sets
  the system slices' allowed CPUs so the tick vCPU is free; U6a only reads).

## 3. How it is tested
- **Unit and integration tests** (injected clock wherever a time is asserted; real-clock tests at most 5 s each, no
  timing bar in the container): T6 Loopback across threads, Send asserts (2); TickLoop on the schedule, the catch-up
  bound and dropped time, bounded channels under a flood, decoding off the tick thread, register 96's RTT hand-off,
  `channel_of` on a full state above 262,144 B, kicked/left/replaced closing, the panic path (`RoomEnded(crash)`,
  crash record, footer, non-zero end), PhaseTimers' five values exact, one real-clock smoke (2 clients, 2 s) (10);
  settings accepted and refused, the token file (0600, N distinct, absent from every log), `OPEN` within 2 s and
  exit 0 from `CARGO_BIN_EXE_npt-server`, exit codes (5); the steal, RSS, schedstat and affinity parsers on fixed
  /proc texts, kernel threads excluded, `sibling_free` on fixed SMT maps, `cpu_read.changed`, the header's machine
  and carry 60's params equal to `Scene::weapon()` and `health()` (8); the room log's -2 series on a fixed SS scene
  equal to the sim's own counts, `everyone_in_view` 1 then 0 with one soldier moved past the radius, the writer
  never blocking the tick, every counter on every record (4); npt-super against a fake child (`OPEN` after 200 ms)
  and a silent one (deadline) (2); C-U6-17's carry test (one 600 ms pong, a 10 ms path, option B's buffer of 8, no
  kind-10 refusal) (1); C-U6-18's order (1). **At least +33 measured.**
- **Bites** (bites build, each failing its mark's check by name on the log it writes): PM-0001-16/double-load
  (spawn to Open at least 2 s), PM-0006-11/no-smt-map (`machine_named` 0), PM-0001-1/shared-vcpu (a second thread
  pinned to the tick vCPU: `alone` 0; the run's arrangement is U8's, the read is tested here). **+3 bites tests.**
- **Plants** (scratch edits, one `-x -k` runner call each, restored and sha256-checked; each must fail the named
  test): P1 an unbounded channel (the flood test); P2 lab's `stream_of` copied for the control channel (the 262,144 B
  test); P3 the RTT not handed over (register 96's test); P4 the panic catch removed (the panic test); P5 every owed
  tick run (the catch-up test); P6 a replaced session routed before its kick (the replacement test); P7 C-U6-17's
  cap removed (the carry test); P8 kernel threads counted in `alone` (the affinity test); P9 steal over all vCPUs
  (the steal test); P10 `outgoing` before the step (C-U6-18's test); P11 carry 60's params dropped (the header test).
- **check.mjs's 11 steps** (fmt, clippy, clippy-wasm32, doc, test, deps, wasm, refs, marks, self, redact) PASS on
  both builds; counts as deltas on `<base>`'s: before = `<base>`'s measured and bites; after = at least +33 and +36.
  The test step's time reported against `<base>`'s; past 1.5 times it stops (u4f-brief.md section 1 item 5, TE31-10).
- **Block 0's evidence:** each lock diff as section 5 binds it; `cargo deny check advisories bans sources` and
  `cargo audit` on the bumped lock, both clean, with the advisory database's date.
- **Container readings, reported, never judged:** 20 starts' spawn to Open (p50, max), the smoke's ticks and
  overruns, the affinity events with the tick thread pinned (4 vCPUs here, taskset present).
- **The report gives:** the counts and provenance lines; each plant's failing test and restore hash; each bite's
  failing check; the SERIES.md rows written (condition 1: the five loop series, the five per-second reads,
  `out_queue_count`, the three events, the 15 -2 names, carry 60's seven params, `spawn_to_open_ns`), every other
  npt-server row named for U6b-U6e; the new pub items; added lines per block against section 8; what is not
  verified (no RS, no load, no transport; the tick vCPU "alone" only as far as this container's processes go).

## 4. The carries, word for word, and the advisor's conditions
- **C-U6-17** (plan.md:1254): "**C-U6-17 the controller's cap off L1 (NC-U4F-3):** "before an honest client runs on
  a link other than L1, the controller's cap holds under both options. The form: the stamp capped at the earliest
  arrival's next tick + 7 (`next_tick_at(now + lo_rtt / 2) + 7`). Its test: one 600 ms pong and a 10 ms path, under
  option B's buffer of 8, with no kind-10 refusal on the honest client.""
- **C-U6-18** (plan.md:1254): "**C-U6-18 a Behaviour's host order (EA-U4F-9):** "a Behaviour's host order (input,
  the endpoint's step, `outgoing`) gets one entry point, or module text that states the order as the contract,
  before npt-bots and npt-load host Behaviours.""
- **C-U6-4** (plan.md:1254): "**C-U6-4:** the probe process writes `second.v.steal_permille` as npt-server does;"
  (npt-server's half here; closed by U6c's probe).
- **Carry 60** (u4s-carries.md:286-290): "60. **TUNE-1's header (advisor-u4-wave1.md:252-253; the advisor's W1-U4D
  ruling; homed by the advisor's U4d close ruling):** "Each run's header records the scene's weapon (interval,
  damage, reach, magazine, reload, spread) and its soldiers' health, read from `Scene::weapon()` and
  `Scene::health()`. U4r's runner writes them into npt-lab's L1 headers, and the contract test shows them in every
  header. U6's npt-server header does the same (plan.md's U6 row).""
- **Register 96** (u4s-carries.md:628-632), the hand-off S1 makes: "U4h's callers (the lab, scenario.rs:704;
  npt-server's tests, common/mod.rs:290, loopback.rs:213) pass each session's `Decoder::rtt_ns`; U6's TickLoop makes
  the same hand-off."
- **SEC-R1D-1** (security-r1-deps.md:196-201, Medium): "Fix: U6's lock change runs `cargo update -p url --precise
  2.5.9` (brings form_urlencoded 1.2.3); no Origin check uses `Url::origin()` on client input before that." U6a is
  first on the cargo lane, so its Block 0 makes it (section 5).
- **If Q-U6A-3 holds:** C-U6-16 (c) (plan.md:1254): "(c) TickLoop sends `Room::take_control`'s bytes on a channel
  that npt-proto or the room gives it (a public read of a framed message's channel, or each message's `Limits::TYPES`
  row beside its bytes), never by a copy of the test lab's `stream_of`, which finds the type byte at the offset of
  the channel it is trying to learn and holds only while a full state's length stays below 262,144 B (SEC-U4E-R2-4,
  A-R2-1; the card's A9 follow-up, at the next npt-proto change, may take it first);" and C-U6-14's first clause:
  "TickLoop, when `Room::join` reports a replacement, sends the old connection `Kick(replaced)` and closes it before
  routing anything else, and closes a kicked or left session;".
- **C-U6-19's gate line** (register 119, U6-Q21): the re-plan moves C-U6-19 to U6d (replan-units.md:48-50, :61), so
  "before U6a's merge" now reads "before U6d's merge", and U6a's brief states it: **U6a's tests hold no throwing or
  rejoining hostile client and no client restarted mid-run; the one replacement test checks routing only, no press
  or shot count.** SA-F1-1 stays open until U6d merges.
- **advisor-u6-brief.md, on U6a's blocks:** condition 1 (the rows U6a writes, section 3); condition 5 as re-placed
  above; condition 6 and ADV-U6B-7 (Block 0: "a sub-unit that changes any manifest or lock starts with Block 0";
  "U6a (npt-server's runtime and its own manifest)"); condition 7 and ADV-U6B-8 (every reviewer at xhigh; the
  engine-architect seat: Q-U6A-7); condition 8 with U6-Q17 term (2) ("U6a keeps the binary's settings, lifecycle,
  panic catch and non-zero exit"; the certificate is U6b's) and U6-Q21; ADV-U6B-9 (S8's split; carry 60's header
  fields built in one block, S4 here, which U6b's T5 lists). The item-1 note "C-U6-17 in U6a before any honest
  client off L1" holds: Block 7 merges before U6b's loopback clients. U6-Q4 and U6-Q15 moved with S1b and C-U6-15 to
  U6d (replan-units.md:61).

## 5. Dependencies (named up front; DEP-HOLD)
- **No new crate.** Two lock changes, each its own diff, made in Block 0:
  - **(a) SEC-R1D-1:** `cargo update -p url --precise 2.5.9`. The lock diff is exactly url 2.5.8 to 2.5.9 and
    form_urlencoded 1.2.2 to 1.2.3, and nothing else (the binding check, security-r1-deps.md:249-253; anything else
    goes back to security-auditor before the save). Then `cargo deny check advisories bans sources` and `cargo audit`
    on that lock, the advisory database's date recorded. The tools as dependency-check.md:9-10 says: cargo-deny 0.22.0
    and cargo-audit 0.23.0, installed one at a time with `--locked`, build folders under .gate/, the advisory database
    never inside the repository or a worktree.
  - **(b) U6a's direct-dependency moves:** npt-server gains `tokio` (workspace `=1.54.1`, its features as set; already
    compiled through npt-net) for the I/O runtime, `sha2` (`=0.11.1`) for the binary's hash, and `serde` and
    `serde_json` (`=1.0.229`, `=1.0.154`) for the settings file; npt-super gains `sha2`. All are in Cargo.lock today;
    the diff is only npt-server's and npt-super's dependency lists (no package, version or checksum). This is not in
    security-r1-deps.md's candidates, so **it needs its own short read** of exactly those lines.
- **DEP-HOLD** (main's .claude/CHECKPOINTS.md §3, Mustafa 2026-11-18): a save that changes a Cargo.toml or Cargo.lock
  against the remote branch is held until security-auditor's answer and dependency-check.md exist for exactly that
  content (one line per file, naming the answers' notes files: security-r1-deps.md for (a), the short read for (b)).
- **Until the bump lands, no code calls `Url::origin()` on client input** (SEC-R1D-1). U6a writes no URL handling.
- **Any other crate** (Q-U6A-5's option (b): core_affinity or nix) is new and needs its own read before use.

## 6. The rules that stand
- **Start:** one builder (general-purpose, opus, xhigh), `isolation: worktree`, which starts at main. First step,
  in its worktree: `git merge --ff-only <base>` (if it cannot fast-forward, `git switch -c u6a-build <base>`), then
  `git log -1 --format=%H` shows `<base>`; both in its notes first (the lesson of u4f-brief.md:292-295). `<base>` is
  left for the main session to fill, after Block 0's save.
- **Notes:** `.checkpoints/netcode-proof-test/u6a-build.md` in the worktree: the task, the blocks, each finished
  block at once, a notes line per finding, `git diff --stat <base>`'s added lines after each block.
- **The shared target** `/home/user/Shamble-Frontlines/.gate/shared-target` and `CARGO_NET_OFFLINE=true`, every
  cargo call through a U6a runner (the main session's names-only copy of u4f-run.sh, logs under .gate/u6a/), the
  lane lock and the disk floors of u4f-brief.md section 4 rules 1-9; commands as section 4 gives them.
- **Files:** U6a's lane is crates/server, link, proto (`channel_of` and its tests), room (the getter and its tests),
  log (`read_local`), super, client (C-U6-17), bots (C-U6-18), and marks/SERIES.md new rows only. Not U6a's: every
  Cargo.toml and Cargo.lock (Block 0's, done before launch; a further change stops the build), crates/net, l1 (unless
  Q-U6A-4 keeps T6 whole), lab, judge, marks.json, marks/README.md, any existing SERIES.md row, check.mjs. A part that
  needs one stops, is reported, and the rest is built.
- **Bans:** `.claude/rules/netcode.md` (crates/server); the clock and hash-map bans (server/clippy.toml; npt-room
  float-free); bites per instance; no network socket (the binary binds nothing until U6b; tests on Loopback or an
  injected clock); LEAD-BOUND frozen at 400 ms; no client string or token in any log (E22).
- **SIZE-CHECK** (history.md:2048-2051): past 6,000 added lines the build stops, the end checks run on the tree as
  it stands, and the main session gives Mustafa three lines (what grew, why, the cheaper option).
- **Agents never commit;** the main session snapshots, merges and commits. The builder's brief carries, word for
  word: "Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo
  fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with
  Write."

## 7. Review
One round (NPT-TRIM), netcode-architect and test-engineer, each at xhigh; a second only for an open Critical or High.
No U6a block faces the internet in a run (the re-plan's security seat sits on U6b, U6c and U6e: replan.md:93), so
security-auditor does not join the round; its part is Block 0's reads; the token file and the settings parser are
read by the netcode seat.

## 8. Size
- **The re-plan's figure:** 1,675 estimated, 2,250-2,500 likely (replan-units.md:58).
- **This brief's:** about 2,350 (2,150-2,550): T6's `Send` 70 (loss and space, about 80, to U6d by Q-U6A-4's lean);
  S1 850 (with `channel_of` and C-U6-14's clause, +100); S8a 275 (tokens and the settings file, +100); S4 520
  (pinning and the shared machine reader, +120); the room log for -2 250 and -16's timer 180 (both new); C-U6-17 100;
  C-U6-18 100. Block 0 adds about 10 manifest lines. **Likely 3,150-3,800** at the units' 1.35-1.5. The stop stays
  past 6,000 (SIZE-CHECK).
- The 700 lines over the re-plan are R1's needs no piece held (-2's room log, -16's timer, the tokens, the pinning),
  so they land before R1 in any placement: the lane gains about 950-1,050 likely lines before R1.

## 9. Open questions (each with my lean; the advisor's check answers them)
- **Q-U6A-1, -16's timer** (SERIES.md:200 and ADR 0001:1604 give it to npt-super, which is U6e's, after R1): (a)
  U6a builds npt-super's start timer (Block 6); (b) -16 is not counted at R1. **Lean (a):** -16 is kept at R1
  (replan-marks.md:33), and (b) would drop a kept R1 part, which is his.
- **Q-U6A-2, -2's series on RS:** (a) the room log in U6a with `ClientSlot::relevant` (Block 5); (b) with U6d's
  interest writers, -2's RS parts a gap until then. **Lean (a):** -2 is kept in every run, R1 first.
- **Q-U6A-3, C-U6-16 (c) and C-U6-14's first clause in U6a** (the re-plan puts both carries in U6d, but both describe
  what TickLoop does when it is built; u6-facts.md:191-193 put (c) in S1). **Lean: yes;** the rest of -14 and -16
  stays U6d's.
- **Q-U6A-4, T6's `loss` and send-buffer space:** plan.md:577-579 and the Q6 ruling (advisor-u4.md:202) join them
  "with rate adaptation, its first consumer", which the re-plan puts in U6d. (a) U6d; (b) U6a, as replan-units.md:58
  places T6 whole. **Lean (a):** no trait item without its reader; U6a builds the `Send` S1 needs.
- **Q-U6A-5, pinning the tick thread** (E2; npt-server forbids unsafe; no plan line names a way): (a) the room runs
  `/usr/bin/taskset` (fixed path, integer arguments, no shell) on its own thread ids and reads the result back from
  /proc; (b) a new crate (core_affinity or nix), with its own DEP-HOLD read; (c) U8's run script pins from outside and
  the room only reads. **Lean (a):** no unsafe, no new crate, and the read-back is what -1 judges.
- **Q-U6A-6, C-U7-3's cap source:** replan-units.md:221 says the `JoinAccepted` field "joins U6a" if kept, but U7's
  brief check comes after U6a merges (:199-200). **Lean:** rule now for no wire change (the room's bound compiled
  into the same commit's Wasm: replan-units.md §4), so nothing joins U6a.
- **Q-U6A-7, the engine-architect seat** (ADV-U6B-8: on every piece that adds a public item or changes a public
  trait; U6a adds seven items and changes Loopback): (a) netcode and test only, as NPT-TRIM's default and the
  re-plan say; (b) add engine-architect. **Lean (a).**

## 10. For the main session
- **Today:** this brief to the advisor (one check); security-auditor's short read of section 5 (b)'s lines.
- **Before launch (Mon 23, on the lane):** Block 0 (a) and (b), each diff checked, cargo-deny and cargo-audit with
  the database date, dependency-check.md, the save; then fill `<base>` and its counts; the runner copy; the size line
  (section 8) on the card before the first snapshot; the advisor's answers to section 9 applied, rev 1.1 (no
  re-check unless it finds a Critical or High).
- **Close:** the builder returns its notes and return list (u4f-brief.md section 7's shape); snapshot; check.mjs in a
  merge worktree; the round; fixes; merge; U6a's report (counts, plants, bites, rows, size, carries by number,
  Block 0's lock diffs and audit results).
