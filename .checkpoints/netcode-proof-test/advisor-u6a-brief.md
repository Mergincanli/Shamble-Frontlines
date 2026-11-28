# Advisor: U6a's build brief, rev 1 (one check, NPT-TRIM)
Seat: the advisor agent, working from `git show origin/main:.claude/agents/advisor.md` (main's reviewed copy), in a day
session, 2026-11-22. Branch claude/netcode-proof-test-plan-hcky5g at 4a7bd30. One check, no re-check unless a Critical
or High (NPT-TRIM, history.md:7591).

## Plan
1. Read the rules in force: NPT-TRIM (history.md:7561-7593), his answers (:7595-7607), TEST-FIRST (:2234-2282), the card.
2. Read the brief (u6a-brief.md rev 1) and the drafter's notes (netcode-u6a-brief.md).
3. Read the re-plan (replan.md; replan-units.md §1 and §7), U6's row (plan.md:1254), u6-brief.md rev 2.1 and
   advisor-u6-brief.md, security-r1-deps.md.
4. Verify the brief's code facts against the tree (link, l1, room, server, super, lab, SERIES.md, check.mjs).
5. Rule Q1-Q7; the six places; size and R1's date; Block 0; anything else.
6. Verdict with paste-ready conditions; summary.

## Notes (one line per finding, as found)
- Steps 1-3 DONE: read history.md:7561-7607, :2234-2282, :2046-2052; the card; u6a-brief.md (279 lines);
  netcode-u6a-brief.md; replan.md; replan-units.md whole; plan.md:1254 (whole row), :415-445, :459-482, :505-530,
  :569-620, :819; u6-brief.md rev 2.1; advisor-u6-brief.md; security-r1-deps.md whole; marks.json -1, -2, -16,
  PM-0006-11; replan-marks.md:18-33, :82, :106-108.
- Facts that hold (step 4): server/src/main.rs:1-6 stub; server/Cargo.toml has no tokio, sha2, serde (it reaches tokio
  through npt-net); link/src/lib.rs:133-135 `Rc<RefCell<Pipe>>`; l1/src/lib.rs:1747-1750 `End` on `Rc<RefCell<Net>>`;
  room.rs:248 `relevant: Vec<usize>` private; roster.rs:1-3 "minting is U6's"; lab/src/logs.rs:653, :715 write -2's
  series; SERIES.md:200 -16's writer npt-super; check.mjs:98-100 npt-super's rule; server/clippy.toml's clock ban asks
  for reviewed per-item allows; super has no clippy.toml (no clock ban); every -1/-2/PM-0006-11 series the brief names
  has a SERIES.md row today (:22-97, :129-130, :151, :352, :416, :444), so "new rows only" adds none.
- F-A (Medium): deny.toml has `[graph] all-features = true` and `ignore = []` (deny.toml:5-10); security-u5-deps.md:241-243
  warned pin-utils RUSTSEC-2026-0441 (via loom's cfg(loom) edge) "will report it"; no cargo-deny run on this lock is
  recorded in the papers; dependency-check.md:9-10 cleared cargo-deny 0.22.0 and cargo-audit 0.23.0 "for Block 8 only"
  (U5's, never run). So u6a-brief.md:137-138's "both clean" cannot hold as written.
- F-B (Medium): plan.md:1216-1219's smoke opens "the same link" then "&transport=ws" in each browser: on one slot that is
  a replacement, a client restarted mid-run, inside C-U6-19's gate (plan.md:1254) while C-U6-19 waits for U6d; the
  brief states the line for U6a's tests only (u6a-brief.md:175-178).
- F-C (Medium): -16 is "process spawn to Open" (ADR 0001:1604), Open the state that takes joins (ADR 0001:484); U6a's
  OPEN line comes before any listener (u6a-brief.md:55, :223): U6b must move it. -16's notes say "every room start in
  every run" (marks.json -16), so npt-super needs a form that starts a run's room and keeps it, not only time-starts.
- F-D (Medium): ADR 0001:1588: "one vCPU that no other thread of any process uses". The brief excludes per-CPU kernel
  threads (:65-66): a necessary reading; unbound kernel threads then count, and U8's slice setting (:108-109) moves no
  kernel thread, so -1 reads 0 at R1 unless U8 also moves them (workqueue cpumask, the movable kthreads).
- F-E (Low): Cargo.lock lists each workspace member's dependencies, so Block 0 (b) changes the lock's npt-server and
  npt-super entries (u6a-brief.md:199 is ambiguous); a bare `cargo update` would move everything; the offline builder
  needs url 2.5.9 fetched.
- F-F (Low): manifest traps for an offline builder: workspace tokio has no `io-std` (Cargo.toml:58); npt-log has no
  `bites` feature (log/Cargo.toml); npt-server has no npt-l1 dev-dependency (server/Cargo.toml); a test-local delaying
  Link exists as the pattern (server/tests/common/mod.rs:82 `Gate`).
- F-G (Low): u6a-brief.md:117 "`OPEN` within 2 s" is a container timing bar against :112.
- F-H (Low): the writer thread first appears in Block 5 (:69-70), after Blocks 2 and 4 write series (:46-47, :62-66);
  marks/README.md §1: "The writer never runs on the tick thread".
- F-I (Low): `StealReader` pub before its reader (U6c; U6-Q12 left the probe's home to U6c), against Q-U6A-4's own
  principle.
- F-J (Low): SERIES.md:151 `cpu_read.changed` compares with "the run's first read": the brief names no source.
- F-K (Low): C-U6-17 changes npt-client's `plan()` (client/src/lib.rs:621-665), the code -20's and -21's L1 sets were
  judged on; nothing in the brief says a moved pin stops.
- F-L (Low): u6-facts.md puts C-U6-16 (c) in S1 at :182-186, not :191-193 (u6a-brief.md:255); u4f2-report.md's counts
  are at :135 (the brief cites :136).
- Size arithmetic holds: 70+850+275+520+250+180+100+100 = 2,345; re-plan 750+175+200+150+400 = 1,675.
- Lead bound: `Room::LEAD_BOUND_MS = 400` (room.rs:539), settable only by `with_lead_bound_ms` (:616); npt-client does
  not depend on npt-room, so a compiled bound needs one constant both reach (U7's brief).
- Lab's host order: input, the behaviour's step, the endpoint's step, `outgoing` (lab/src/scenario.rs:1640-1689).
- Steps 4 DONE. Next: rulings.
- security-u6a-deps.md DONE (read whole, 139 lines): CLEAR WITH CONDITIONS SEC-U6AD-1 to -6 (two Medium: -1 exact
  workspace entries, no tokio io-std/process/signal; -3 the token file by create_new + mode 0600; -4 no socket); its
  binding check :119-137 (lock (a) first, then the manifest edits with an OFFLINE refresh, lock (b) exactly 5 added
  lines). It answers F-E and most of F-F; its item 5 still expects deny "clean", so F-A stands.

## 1. Q-U6A-1 to -7 (each inside the plan, undoable, no pass mark, no money, nothing on main: mine, TEST-FIRST rule 2)
- **Q-U6A-1, -16's timer: Ruled (a), with two terms.** -16 is kept at R1 (replan-marks.md:33, :106) and its writer is
  npt-super (SERIES.md:200; ADR 0001:1604), so leaving it to U6e's H3 (replan-units.md:62) drops a kept R1 mark, which
  would be his; building the slice keeps his approved plan. Terms: (1) condition 3 (b)'s hold form beside
  `time-starts`; (2) U6e's H3 builds the density harness on it and shrinks by this slice. npt-super keeps check.mjs:98-100,
  spawns by argument vector (SEC-U6AD-6), no systemd (cut: replan-units.md:28-30).
- **Q-U6A-2, -2's series: Ruled (a).** -2 is kept in every kept run, R1's headers block first (plan.md:819;
  replan-marks.md:19, :106), every -2 series reads `process` `room` (marks.json -2), and only npt-lab writes them
  today (lab/src/logs.rs:653-715). `ClientSlot::relevant` as a read-only getter, npt-room staying float- and clock-free.
- **Q-U6A-3, C-U6-16 (c) and C-U6-14's first clause in U6a: Ruled yes.** Both say what TickLoop does
  (plan.md:1254: "TickLoop sends ...", "TickLoop, when `Room::join` reports a replacement, ..."), and u6-facts.md:182-186
  already put (c) in S1; building TickLoop without them builds it wrong once and again in U6d. The rest of -14 (a
  generation, or the test that no closed session's message reaches the room) and -16 (a) stay U6d's.
- **Q-U6A-4, T6's `loss` and send-buffer space: Ruled (a), U6d.** plan.md:519-521 and :577-579 and the Q6 ruling
  (advisor-u4.md:202-203): they "join at U6 with rate adaptation, their first consumer"; U6d is still U6, so the plan
  holds and replan-units.md:58's "T6 ... loss, space" is the outlier. U6a builds the `Send` S1 needs; l1 stays out of
  U6a's lane.
- **Q-U6A-5, pinning: Ruled (a), with terms.** E2 (plan.md:462) and ADR 0001:1588 need the tick thread alone on its
  vCPU and the transport runtime off its sibling; unsafe is forbidden (Cargo.toml's lints) and (b) is a new crate with
  its own read. (c) splits one start across the room and a script and needs a handshake before the start read. Terms:
  `/usr/bin/taskset` by `std::process::Command` with `env_clear()`, integer arguments built by the room, no shell;
  each thread's id from `/proc/thread-self`; the main thread pins itself to the settings' I/O set before it spawns any
  thread (the writer, the runtime and its blocking pool inherit it), the tick thread pins itself to its vCPU; every pin
  read back from `/proc/self/task/<tid>/status` (`Cpus_allowed_list`) and that read is what `alone` and `sibling_free`
  report; a failed pin or read is written as 0 and on stderr, never hidden; settings with no tick vCPU pin nothing and
  write no affinity event (R5's unpinned rooms are U6e's, PM-0006-13); the I/O set excluding the tick vCPU's SMT
  siblings is checked against the SMT map at start (exit 2 if the settings break it); the spawns count inside -16's
  time (a real start pays them); U8's brief confirms util-linux's taskset on the image.
- **Q-U6A-6, C-U7-3's cap source: Ruled, no wire change; nothing joins U6a.** The earlier lean for a `JoinAccepted`
  field rested on "the bound is room data" (plan.md:1255); under NPT-TRIM every kept run runs at 30 Hz (C-U6-16 (b)
  moved: replan-units.md:45-46) with LEAD-BOUND frozen at 400 ms (room.rs:539), and U6a's settings carry no lead-bound
  field, so the bound is one constant for every room this test starts. U7's brief puts it where both sides reach it
  (npt-client does not depend on npt-room) with a test that the client's cap equals the room's bound; if the bound
  turns out to depend on `cmd_timing`, the page's pinned config names the option and the header records it. U7's
  brief check confirms the form.
- **Q-U6A-7, the engine-architect seat: Ruled (a).** NPT-TRIM: "netcode and test by default" (history.md:7590), which
  replaces my ADV-U6B-8 seat for this unit. The seven pub items are small and named (u6a-brief.md:89-91), check.mjs's
  deps step holds the layering, and the netcode seat reads them against plan §4.7 with the `Send` change and the panic
  catch's unwind path. One term: `Machine::read_local` takes no clock and no dependency, builds for wasm32 (npt-log is a
  leaf: plan.md:423-425), and the judge never calls it.

## 2. The six places (each inside the plan; none changes a pass mark, a run day or money; none is his)
1. **-16's writer in U6e after R1** (replan-units.md:58 vs :62): Q-U6A-1 (a). The slice moves to U6a; H3 keeps the
   density harness. Not his: it keeps the R1 mark list he approved (it would be his only if -16 left R1).
2. **-2's server series in no piece:** Q-U6A-2 (a), U6a's room log. Not his.
3. **C-U6-16 (c) is TickLoop's while :61 puts it in U6d:** Q-U6A-3, U6a, with -14's first clause. Not his.
4. **T6 versus rate adaptation** (plan.md:577-579): Q-U6A-4, `loss` and space with rate adaptation in U6d; `Send` in
   U6a. Not his.
5. **C-U6-19 in U6d changes register 119's gate; SA-F1-1 open until then:** the move is the approved re-plan's
   (replan-units.md:48-50, :61; RE-PLAN-OK yes, history.md:7600), so the gate's "before U6a's merge" reads "before U6d's
   merge". What the brief misses: the row's gate is about runs (plan.md:1254: "before any run with a throwing or
   rejoining hostile client or a client restarted mid-run"), and R1 runs before U6d merges, with a smoke that opens the
   same link twice per browser (plan.md:1216-1219). Condition 2. Not his: no mark or run day moves if R1's legs each
   take their own slot. SA-F1-1 staying open through R1 is the approved re-plan's consequence; PR #40 is needs-mustafa
   anyway.
6. **The per-slot 0600 token file built by no piece** (replan-units.md:21-23; the brief cites :24-25): U6a's S8a, as the brief has it (the room
   owns the roster; roster.rs:1-3), on SEC-U6AD-2 and -3's terms. Not his.

## 3. Size, and R1's date
- **The arithmetic holds:** 70 + 850 + 275 + 520 + 250 + 180 + 100 + 100 = 2,345 against the re-plan's 750 + 175 + 200
  + 150 + 400 = 1,675 (replan-units.md:58).
- **Each addition is needed before R1:** the room log (-2, R1's headers block: plan.md:819), -16's timer (-16 first
  counted at R1: marks.json -16), the token file (no client joins without it: replan-units.md:21-23), the pinning and
  the shared machine reader (-1 and PM-0006-11 at R1; `machine_named` reads every log's header, npt-super's included:
  SERIES.md:280), `channel_of` and -14's first clause (TickLoop built right once). Only T6's `loss` and space leave
  (Q-U6A-4).
- **Each belongs in U6a:** all hang on TickLoop or the binary U6a builds; placing them in U6b or U6c moves the same
  lines to the same lane before the same run and saves none.
- **The stop stays past 6,000:** replan.md:25 ("each unit under the 6,000 stop at 1.5x", approved), U6-SIZE's
  first-piece rule (u6-brief.md:111-112). Likely 3,150-3,800 is under it. The card writes 2,345 beside the re-plan's
  1,675 (SIZE-CHECK, history.md:2048-2051).
- **The lane before R1:** U6a 3,150-3,800 + U6b 2,950-3,300 + U6c 2,650-2,950 + U7's Rust about 490-540 = about
  9,250-10,600 likely lines (the re-plan's 8,350-9,250: replan-units.md:207-209). With R1 on Thursday 3 December the
  lane gets one day more than the re-plan's Wednesday table (replan-units.md:194-204): 8 days instead of 7, so about
  1,150-1,325 merged a day, against about 1,175 a
  day merged 15-21 November (replan-units.md:208-209). That is where Wednesday 2 December stood in the re-plan ("holds
  only with no slack": replan.md:41).
- **R1's date: it still holds, with no slack left.** The spare day Thursday gave is about used by U6a's extra
  lines. One lost day (a spend-limit cut of most of a day, a second round on any lane unit, a Block 0 surprise) and it
  no longer holds; then he gets three lines at once with the re-plan's closers (split R1: its server blocks first; usage
  headroom: replan.md:45-46). Not a decision today: one line in the day's message.
- **An optional lever (the main session's lean, only if U6a's build runs long):** the four per-second reads no R1 mark
  reads (`rss_bytes` -15, `proc_cpu_permille` -27/PM-0006-13, `io_cpu_permille` -11, `out_queue_count` R2's queue
  figures: SERIES.md:81, :88, :93, :416) can go to U6e's S5, about 100-150 lines off the lane before R1. Undoable.
- **The brief's day line** (u6a-brief.md:4-5: build Mon 23, round and merge Tue 24) does not fit 3,150-3,800 likely
  lines at about 2,000 built a day (replan-units.md:208-209): build Mon 23-Tue 24, round and merge Wed 25 (condition 8).

## 4. Block 0
- **(a) SEC-R1D-1's url 2.5.9 bump: Ruled, U6a's Block 0,** as security-r1-deps.md:196-201 and :280 place it (U6's
  lock change; U6a is first on the lane). Its binding check: exactly url and form_urlencoded (security-r1-deps.md:249-253;
  security-u6a-deps.md:122-123), on its own first.
- **(b) the four direct dependencies of npt-server and sha2 for npt-super: Ruled, with security's conditions.**
  security-u6a-deps.md: CLEAR WITH CONDITIONS SEC-U6AD-1 to -6; its binding check :124-136 (an offline refresh, exactly
  5 added lock lines, the tree checks). The brief's "no package, version or checksum" holds; the lock's two members'
  dependency arrays do change (security-u6a-deps.md:43-46).
- **The lean (the main session runs Block 0 on the lane before `<base>`): Ruled, agreed.** Builders run offline
  (`CARGO_NET_OFFLINE=true`, u6a-brief.md:214), `cargo update -p url` and the advisory database need the network, the
  main session edits manifests anyway, and DEP-HOLD holds the save until the answers and dependency-check.md exist.
- **What Block 0 misses (ADV-U6A-1, Medium):** "both clean" (u6a-brief.md:137-138; security-u6a-deps.md:134) cannot
  be expected. deny.toml reports every advisory not ignored with a reason and date, over all features (deny.toml:5-10),
  and pin-utils 0.1.1 RUSTSEC-2026-0441 (unmaintained, through loom's cfg(loom) edge) is in the lock:
  security-u5-deps.md:241-243 warned that this config "will report it", and the papers record no cargo-deny run on
  this lock yet, so this is its first. And the tools' clearance reads "for Block 8 only" (dependency-check.md:9-10),
  U5's Block 8, which never ran. Fix: condition 1. Plus: after the save, `cargo fetch --locked` so the offline builder
  finds url 2.5.9's and form_urlencoded 1.2.3's sources; if `cargo update -p url --precise 2.5.9` will not move
  form_urlencoded, `cargo update -p form_urlencoded --precise 1.2.3` first, and the diff check decides.

## 5. Anything else (findings on the one scale)
- **ADV-U6A-2, Medium: C-U6-19's gate covers R1.** The row's gate is about runs (plan.md:1254); R1 runs before U6d's
  merge; the smoke opens "the same link" and then "&transport=ws" in each browser (plan.md:1216-1219), which on one slot
  token is a replacement, a client restarted mid-run. The brief states the line for U6a's tests only
  (u6a-brief.md:175-178). Condition 2.
- **ADV-U6A-3, Medium: what OPEN means for -16.** -16 times "process spawn to Open" (ADR 0001:1604), Open being the
  state that takes joins (ADR 0001:484); U6a's OPEN comes with no listener bound (u6a-brief.md:55, :223). If U6b does
  not move it, R1's -16 PASS rests on a start without its listeners and certificate: evidence that would not hold. And
  marks.json -16's notes say "every room start in every run", which `time-starts` alone (u6a-brief.md:75-79) cannot
  give a run's measured room. Condition 3.
- **ADV-U6A-4, Medium: `alone` and kernel threads.** ADR 0001:1588: "one vCPU that no other thread of any process
  uses". Excluding per-CPU kernel threads (u6a-brief.md:65-66) is a necessary reading (they cannot move); an unbound
  kernel thread's mask holds every vCPU on a stock kernel, so it counts, and U8's slice setting (u6a-brief.md:108-109)
  moves no kernel thread: -1 would read 0 at R1 and V-S would make the runs that list it INVALID (marks/README.md
  :269-270). Condition 4.
- **ADV-U6A-5, Low: offline-builder traps** beyond SEC-U6AD-1's (stdin on a std thread): npt-log has no `bites`
  feature (crates/log/Cargo.toml), so PM-0006-11/no-smt-map is planted in npt-server; npt-server has no npt-l1
  dev-dependency, so C-U6-17's 600 ms-pong test uses a test-local delaying `Link` over Loopback
  (server/tests/common/mod.rs:82's `Gate` is the pattern); npt-super's fake child comes without a manifest change
  (security-u6a-deps.md:91-92); no new dev-dependency anywhere. Each would otherwise stop the build. Condition 5.
- **ADV-U6A-6, Low:** "`OPEN` within 2 s" (u6a-brief.md:117) is a container timing bar against :112. Condition 5.
- **ADV-U6A-7, Low:** the writer thread first appears in Block 5 (:69-70) while Blocks 2 and 4 already write series
  (:46-47, :62-66); marks/README.md §1: the writer never runs on the tick thread. Condition 5.
- **ADV-U6A-8, Low:** `StealReader` pub before its reader (U6c; U6-Q12 left the probe's home to U6c's brief), against
  Q-U6A-4's own principle (u6a-brief.md:66-67, :259). Condition 5.
- **ADV-U6A-9, Low:** `cpu_read.changed` compares with "the run's first read" (SERIES.md:151); the brief names no
  source. Condition 5.
- **ADV-U6A-10, Low:** C-U6-17 changes `plan()` (client/src/lib.rs:621-665), the code -20's and -21's L1 sets were
  judged on; and `host_step`'s order should equal the one lab's L1 sets ran (lab/src/scenario.rs:1640-1689: input, the
  behaviour's step, the endpoint's step, `outgoing`). Condition 5.
- **ADV-U6A-11, Low: U6b's carries from U6a:** OPEN's move (condition 3); `Kick(replaced)` must reach the old client
  before the close on both transports (an immediate QUIC close can drop unsent stream data); U6b's security seat reads
  U6a's token minting and file with the `Join` path, since U6a's round has no security seat (u6a-brief.md:233-236).
  Condition 6.
- **ADV-U6A-12, Low: cites and readings.** u6-facts.md's S1 entry is :182-186 (u6a-brief.md:255 cites :191-193);
  u4f2-report.md's counts are at :135 (:16 cites :136); the token line is replan-units.md:21-23 (:54 cites :24-25). The
  -16 bite's reading (u6a-brief.md:57-58: both loads behind 1 s, so the start passes 2 s) is the only one under which
  the bite fails `max < 2 s`; accepted, named in the report's readings line. Condition 7.
- **ADV-U6A-13, Low:** the day line (u6a-brief.md:4-5). Condition 8.
- **Right as written:** the blocks' order and lane; the bans (no socket, LEAD-BOUND frozen, E22); the plants P1-P11 and
  the three bites, each failing by name; +33 measured and +36 bites (2+10+5+8+4+2+1+1 = 33); check.mjs's 11 steps with
  the 1.5x time stop; one builder, the ff-only start; agents never commit; carries quoted word for word (C-U6-17, -18,
  -4, carry 60, register 96, SEC-R1D-1, C-U6-16 (c), C-U6-14's first clause: each matches plan.md:1254,
  u4s-carries.md:286-290, :628-632, security-r1-deps.md:196-201); one round, netcode and test at xhigh.

## Verdict: APPROVE WITH CONDITIONS
No Critical or High, so no re-check (NPT-TRIM, history.md:7591). Q-U6A-1 to -7 and the six places are ruled above; none
is his. Rev 1.1 applies these by Edit before launch; the main session records the rulings under "Choices taken without
him" and names them, one line each, in the day's message.

Conditions (paste-ready, each an Edit in u6a-brief.md rev 1.1):
1. **Block 0 (ADV-U6A-1).** Section 5's binding check is security-u6a-deps.md:119-137 with security-r1-deps.md:249-253
   for (a); SEC-U6AD-1 to -6 go into section 6's bans by number. Before the deny and audit run, the main session asks
   security-auditor, in one line: (i) how pin-utils 0.1.1 RUSTSEC-2026-0441 (unmaintained, loom's cfg(loom) edge) is
   handled, since deny.toml's `ignore = []` with `[graph] all-features = true` reports it (security-u5-deps.md:241-243)
   (my lean: an ignore entry with its reason and date, as deny.toml:9 provides, as its own one-line deny.toml change
   read by security-auditor); (ii) its yes to cargo-deny 0.22.0 and cargo-audit 0.23.0 here, cleared "for Block 8 only"
   (dependency-check.md:9-10). "Clean" then means no advisory but the one it names; anything else cargo-deny reports
   (its first recorded run on this lock) goes back to security-auditor before the save. After the save, `cargo fetch
   --locked` on the lane so the offline builder finds the bumped sources; if `cargo update -p url --precise 2.5.9`
   will not move form_urlencoded, `cargo update -p form_urlencoded --precise 1.2.3` first, and the diff check decides.
2. **C-U6-19's gate (ADV-U6A-2).** Section 4's gate line reads: "Until U6d merges, U6a's, U6b's, U6c's, U7's and U8's
   briefs and R1's run plan each state in one line that their tests or runs hold no throwing or rejoining hostile
   client and no client restarted mid-run, or bring C-U6-19 first. In R1 each browser leg (each browser on each
   transport, the QUIC-blocked fallback, -36's three) joins its own slot once, or the room restarts between legs; the
   token file holds a slot for every leg. SA-F1-1 stays open until U6d merges."
3. **OPEN and npt-super (ADV-U6A-3).** Block 3: "U6a's OPEN means the room built and TickLoop running; U6b's brief
   moves it after both listeners accept and the WebTransport certificate is made, so spawn to Open covers a real start
   (ADR 0001:484, :1604); U6a's container readings are a lower bound, never -16's evidence." Block 6 adds a hold form:
   npt-super starts a run's room, times it to OPEN, writes the event, keeps the room until its own end (stdin held
   open, closed at npt-super's end) and records its exit; `time-starts` is N of those with no hold; so U8's run plan
   starts every room through npt-super (marks.json -16's notes: "every room start in every run").
4. **`alone` (ADV-U6A-4).** Block 4 states: "A task counts against `alone` when its allowed list holds the tick vCPU,
   except the tick thread and kernel threads bound to that vCPU only (PF_KTHREAD in /proc/<pid>/task/<tid>/stat's
   flags, allowed list exactly the tick vCPU); an unbound kernel thread counts." Section 3's affinity fixtures add a
   per-CPU kernel thread on the tick vCPU (excluded), an unbound one (counts) and a user thread (counts). Section 2
   carries to U8's brief: R1's run prep moves every movable kernel thread and the unbound workqueue mask off the tick
   vCPU besides the slices, and R1's bring-up reads `alone` once before the first measured run. The reading "per-CPU
   kernel threads bound to the tick vCPU do not count" of ADR 0001:1588 joins the readings-PR list on the card.
5. **Build traps and small fixes (ADV-U6A-5 to -10).** Section 6: PM-0006-11/no-smt-map planted in npt-server (npt-log
   has no `bites` feature); C-U6-17's test on a test-local delaying `Link` over Loopback (server/tests/common/mod.rs:82's
   pattern), not npt-l1; npt-super's fake child without a manifest change; no new dev-dependency. Section 3: the binary
   test asserts OPEN within the 5 s real-clock cap and reports the time (no 2 s bar here). Block 2 builds the writer
   thread with the loop's series; Block 5 adds -2's fields to its record. `StealReader` stays pub(crate); U6c's brief
   makes it pub if its probe lands outside npt-server. `cpu_read.changed` compares with a settings field the run
   script fills from the run's first start (empty at the first start: `changed` 0). C-U6-17: every existing npt-client,
   npt-server and npt-lab test and every pinned hash or recording passes unchanged; a moved pin stops the build and
   comes to the advisor. `host_step`'s module text names the order lab's L1 sets ran (scenario.rs:1640-1689), and the
   netcode seat confirms they match.
6. **U6b's carries (ADV-U6A-11).** Section 2 lists for U6b's brief: OPEN's move (condition 3); `Kick(replaced)`
   reaching the old client before the close on both transports; its security seat reads U6a's token minting and file
   with the `Join` path.
7. **Cites (ADV-U6A-12).** u6-facts.md:182-186; u4f2-report.md:135; replan-units.md:21-23; the -16 bite's reading in
   the report's readings line.
8. **The day line and size (ADV-U6A-13).** Section 0: "Build Mon 23-Tue 24; round, fixes and merge Wed 25." Section 8
   adds: "R1 on Thursday 3 December still holds on these numbers, with no slack: the lane before R1 is about
   9,250-10,600 likely lines in 8 days; one lost day and it slips (three lines to him at once). Lever, if the build runs
   long: the four per-second reads no R1 mark reads go to U6e's S5."

## What I could not check
- No build or run (an advisor brief check): the size figures are judgements, checked for sums only; whether cargo-deny
  0.22 reports anything beyond pin-utils on this lock (wildcard path dependencies under `wildcards = "deny"`, for one)
  I could not run, hence condition 1's "anything else goes back".
- Whether `cargo update -p url --precise 2.5.9` moves form_urlencoded by itself (cargo's behaviour; condition 1 covers
  both outcomes).
- Kernel-thread masks on a Hetzner CCX23 image: reasoned from stock Linux, not read on the VM (condition 4's bring-up
  read settles it).

DONE
