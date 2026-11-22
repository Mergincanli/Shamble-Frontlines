# U6a build review, netcode seat (round 1 of 1, NPT-TRIM), 2026-11-22

Inputs: u6a-brief.md rev 1.1 (sha256 c109691a... checked), wip/u6a-build.patch (sha256 a9f8b6e0... checked; base
aa5c62b6), built tree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a95ebf14368f277ff (read only).
(No Edit tool in this seat: the notes file is rewritten whole with Write after each block.)
W = /home/user/Shamble-Frontlines/.claude/worktrees/agent-a95ebf14368f277ff/spikes/netcode-proof (every file:line
below is under W). No cargo run made: every claim below is from reading the tree and the evidence logs.

## Plan
1. Read the brief's 0.1, 1-4, 7; advisor-u6a-brief.md (Q-U6A rulings); u6a-build.md (F1-F9, O1, O2).
2. Read the patch file list; then each new file in the worktree: tick_loop.rs, phase.rs, clock.rs, main.rs (settings,
   tokens), S4 readers (steal, rss, schedstat, affinity, taskset), room log writer, npt-super, client plan() cap,
   bots host_step, proto channel_of, room ClientSlot::relevant, log Machine::read_local, link Loopback Send.
3. Check scope items one by one; a notes line per finding, saved after each.
4. Read the evidence logs (check3, plants, starts) only to confirm claims I lean on.
5. Verdict, findings table, views on F1, F3, F4, F7, F9, O1, O2, not verified, DONE.

## Notes (one line per finding, as found)
- Step 1 DONE: brief 0.1, 1-4, 7 read; advisor-u6a-brief.md Q-U6A-1..7 terms read (Q5's terms, :82-92: env_clear,
  ids from /proc/thread-self, read-back, "a failed pin or read is written as 0 and on stderr, never hidden" (:89), no
  tick vCPU = no pin and no event, "the I/O set excluding the tick vCPU's SMT siblings is checked against the SMT map at
  start (exit 2 if the settings break it)" (:90-91)); u6a-build.md read.
- tick_loop.rs read whole (1,447). Clock: one item (clock.rs:21) with its allow; no other Instant/SystemTime in the
  diff (pump.rs:238's SystemTime is base's wallclock bite). No with_lead_bound_ms anywhere in the diff (LEAD-BOUND 400 ms
  holds: room.rs:547). Room has no time input at all (room.rs:899 pump_report, :912 receive): F1 confirmed at base;
  register 96's reader was "U4h's tolerance" (u4s-carries.md:628-632), and U4h left the spike.
- Closing order read: join() offers Kick then close() (store Release) at tick_loop.rs:661-667; poll() loads the flag
  (Acquire) BEFORE draining (:844-859), so the Kick is always drained before the link drops. Holds on Loopback.
- Candidate (Low, carry): the one inbound queue (INBOUND_CAP 4,096, tick_loop.rs:51) is shared by every session, and
  the decoder passes every Joined stream message other than Ping/Pong with no rate limit (pump.rs:476). One session's
  Resync flood fills it; another session's Join is then dropped and that session closed (tick_loop.rs:875-882).
  No hostile client in U6a (C-U6-19's gate line); for U6b/U6d: a per-session share per tick.
- tick_loop/tests.rs read whole: schedule, catch-up (7, dropped on the first tick only, sums = counters), flood both
  ways, decode thread names, RTT 42 -> 77 ms, 262,148 B frame -> FullState, replacement + Leave, panic at tick 8 (8 tick
  records, tick_panic, crash footer), PhaseTimers exact, pinned run, -2 counts vs dump, everyone_in_view, stuck writer,
  every counter, smoke. Each scope item has its test. channel_of (proto msg.rs) checked by hand: a control frame and a
  full-state frame can never both match (u16 = n-2 vs u32 = n-4), so any length is safe.
- F3 checked: Room::tick calls sim.step exactly once (room.rs:1061); Tallied delegates every SimCore call; fine.
- F4 checked: Scene::stress() carries StandIns::s1() (scene.rs:188) and the binary uses it, but nothing burns; the
  header writes stand_ins [] and burn/top-up 0 (main.rs:380-382): truthful "as run". marks.json -2's notes: the
  stand-ins are "named in header.params.stand_ins; ... the judge reports it" (no stat): no R1 check fails on it.
  The burn has no owner after the re-plan (scene.rs:7-8 "the server's tick loop ... burns them (U6)").
- main.rs read whole. SEC-U6AD-2 holds (take(CAP+1) :114-119; deny_unknown_fields, no flatten/untagged/Value :59-102;
  clients range-checked before mint :145, :431). SEC-U6AD-3 holds (create_new + mode 0o600 :266-270, no chmod; existing
  path exit 2 :436-441; urandom read_exact :227; no Debug on Settings/Checked/LoopSettings). SEC-U6AD-1 holds (both
  manifests exact). SEC-U6AD-5 holds (:280-296). No socket, no serde_json on link bytes.
- FINDING A (Medium) -> NR-U6A-2. FINDING B (Medium) -> NR-U6A-3. FINDING C (High) -> NR-U6A-1 (table below).
- proc_read.rs otherwise right: PF_KTHREAD 0x00200000; flags = 7th field after the last ')' (fixture with "(odd (name)
  x)"); steal from cpu<N> only, total = first 8 fields; taskset fixed path, env_clear, integer args, std null, exact
  read-back (proc_read.rs:222-255). Machine::read_local: no clock, no new dep, judge never calls it (grep), wasm32
  clippy passed in check3; same reads as lab/src/machine.rs.
- Link Loopback Arc<Mutex> (link/src/lib.rs): const asserts, poison taken as is, l1 untouched. Fine.
- C-U6-17 (client/src/lib.rs:653-668): goal capped at next_tick_at(now + min(samples.rtt)/2) + 7 beside max_lead; 7 =
  REDUNDANCY_CAP 8 - 1 = CMD_CATCHUP - 1 (proto limits.rs:17, room input.rs:177). The carry's form. carry.rs: Delay link
  over Loopback (not l1), option B (room default), 10 ms path, 3rd exchange 600 ms slow both ways, buffer_full 0;
  P7b fails it on the up leg. Fine.
- C-U6-18: host_step (bots lib.rs:887-899) = step (endpoint.input(None, received) then the command) -> endpoint.step ->
  outgoing -> reliable; lab scenario.rs:1640 input(None), :1681 behaviour.step(&[]), :1688 endpoint.step, :1689
  outgoing, :1690 reliable. They match (condition 5 confirmed); a view host feeds the endpoint itself and passes &[].
- -2 parity: snapshot_clients equals lab's rule (logs.rs:629-632); everyone_in_view reads the room's relevant set (the
  mark's words, SERIES.md:273) where the lab read its mirrored frame (mirror.rs:324-346); the -2 test compares every
  count but sets `everyone: 0` on both sides (tests.rs:1071-1078), so flag parity with the lab is not tested.
- Evidence read: check3.log:331-342 all 11 PASS (test 866.8 s; 744 and 844; 0 failed, 0 skipped); plant-p6, -p8, -p11
  each FAIL their named test; starts2.log exit 0; starts-super.jsonl line 1 holds "bite":null,"tick_hz":0,"scene":"".

## Verdict: REVISE
One High (NR-U6A-1): npt-super's own log cannot be judged in the run whose room it starts, and the -16 bite's evidence
was shown on a hand-made log, not npt-super's. Two Medium (NR-U6A-2, -3), both terms of the advisor's Q-U6A-5 ruling
left unbuilt. Everything else in my scope holds (below). NPT-TRIM: the High opens round 2, on what changes only.
The three fixes are about 40-55 lines, so U6a stays near 5,510 of SIZE-CHECK's 6,000.

## Findings (worktree paths under W)
| id | sev | where | the failure scenario | smallest fix |
|---|---|---|---|---|
| NR-U6A-1 | High | crates/super/src/main.rs:181-188 (header: `bite: None`, `tick_hz: 0`, `scene: String::new()`); crates/server/tests/binary.rs:215-221 | The judge groups logs by `header.run` and makes a run INVALID when its logs disagree on set, role, bite, layer, profile, tick_hz, scene or date (crates/judge/src/eval.rs:47-61, :542-547). Condition 3's hold form exists so U8 starts every room through npt-super and -16 counts "every room start in every run". npt-super's log in that run says tick_hz 0 and scene "" while the room's says 30 and "SS", so the run is INVALID. That is every R1 run, if U8 places it so. In its own run id instead, the -16 bite can never be judged: npt-super names no bite, and a run that names one with no bites-build log is INVALID (eval.rs:560-564). The double-load bite test builds its "npt-super form" log from the ROOM's header with only `process` changed, so its evidence does not hold for npt-super's real log (starts-super.jsonl line 1 shows the real header). | npt-super takes `--scene`, `--tick-hz` and an optional `--bite`, and `log()` (:161-193) writes them into its header. One npt-super test asserts `log()`'s header carries them. The bite test builds its super log from those same three values on npt-super's header form, not from the room's. U8 carry: npt-super's log takes the run id of the room it starts, and npt-super ends by its stdin closing, never by a kill (a log with no footer makes the run INVALID, eval.rs:579-584). A judge-side exemption for process `super` would change the judge: that is Mustafa's call, named here and not proposed. |
| NR-U6A-2 | Medium | crates/server/src/main.rs:165-167 (the only vCPU-set check), :448 (the SMT map is at hand, never checked) | Q-U6A-5's term "the I/O set excluding the tick vCPU's SMT siblings is checked against the SMT map at start (exit 2 if the settings break it)" (advisor-u6a-brief.md:90-91) is not built. A run script that puts the tick vCPU's sibling in `io_vcpus` starts the room. The room then logs `sibling_free` 0, which fails -1 and PM-0006-11, and the run is spent. | In `run()`, after `let smt = machine.smt.clone();`, when `tick_vcpu` is set and any `io_vcpus` entry shares a list of `smt` with it (the test proc_read's `sibling_free` makes), return `fail(EXIT_SETTINGS, ..)`. The binary cannot reach that crate-private function, so either a 6-line copy in main.rs or `TickLoop::start` returning an error kind the binary maps to 2. Add one refusal case on a fixed map. |
| NR-U6A-3 | Medium | crates/server/src/proc_read.rs:165-197 (`tasks`: an empty list when `read_dir("/proc")` fails, unreadable tasks skipped, cut at TASKS_MAX); crates/server/src/tick_loop.rs:1277-1278, :1285 | `not_alone` over an empty or partial walk is empty, so `affinity` writes `alone` 1. The same happens when /proc is mounted `hidepid=2`, or a systemd unit sets `ProtectProc=invisible`: other users' threads are hidden, and the room reads "alone" while root's threads can run on the tick vCPU. An I/O thread whose `allowed` read fails is dropped from `sibling_free` the same way. This breaks Q-U6A-5's "a failed pin or read is written as 0 and on stderr, never hidden" (advisor-u6a-brief.md:89), and -1 would then pass on evidence that does not hold. | In `affinity()`, write `alone` 1 only when the walk holds the tick thread and tid 1 (pid 1's main thread, hidden under hidepid) and `tasks.len() < TASKS_MAX`; else write 0 and one `eprintln!`. Likewise `sibling_free` 0 when any I/O tid's `allowed` is `None`. About 8 lines. A test on a fake /proc root is optional (`tasks(proc)` already takes the root). |
| NR-U6A-4 | Low | crates/server/src/tick_loop.rs:1019-1021 (`steal.map_or(0, ..)`), :1023 (rss 0) | A failed /proc/stat read, a missing `cpu<N>` line or a zero total writes `steal_permille` 0. That is the best value -1's steal check can read, so a failed read passes. RSS and the CPU shares also write 0 on failure. | Leave `steal_permille` out of the record when the read is `None` after the first second, so the judge sees a gap. The same for `rss_bytes`. |
| NR-U6A-5 | Low (carry U6d) | crates/server/src/tick_loop.rs:51, :868-883; pump.rs:476 | One inbound queue serves every session, and every Joined stream message but Ping and Pong passes the decoder with no rate limit. One session's Resync flood (the flood test sends 10,000) fills the queue. Another session's `Join` is then dropped and that session closed (:877-882), and other sessions' reports are dropped. U6a has no hostile client (C-U6-19's gate line). | U6d, with its rate limits (.claude/rules/netcode.md: "per-connection rate and size limits; disconnect abusers"): a per-session cap on offers per room tick in `SessionIo::poll`, the excess counted and the session closed. |
| NR-U6A-6 | Low (carry U6e) | crates/server/src/tick_loop.rs:559-560 | The -2 counting runs on the tick thread after `timers.end`, so its time is in no series: not `work_ns`, no phase. PM-0006-19's logging-cost ratio over `work_ns` cannot see it, and the counting also runs when the room has no log. | U6e (logging cost): count inside a timed phase, and skip it when `rows` is `None`. No change in U6a. |
| NR-U6A-7 | Low (carry U6b) | crates/server/src/tick_loop.rs:1418-1427, :1438-1444 | `stop()` and `Drop` call `runtime.block_on` and `shutdown_timeout`. Called inside an async context (U6b's listeners on their own tokio runtime) they panic. The runtime also has no pub handle, so U6b's listeners cannot spawn on it. | U6b's brief states where TickLoop lives, which runtime its listeners use (a pub `Handle` or their own), and that `stop` runs on a plain thread. |
| NR-U6A-8 | Low | crates/server/src/tick_loop.rs:662-666 (Kick(replaced)), :751-755 (RoomEnded) | When the old session's queue is full, the `offer` of `Kick(replaced)` or `RoomEnded` fails silently: the client gets only a close, and nothing counts it. | Count the failed offer in `out_full` (2 lines). U6b's carry (condition 6, the Kick reaching the old client before the close) builds on it. |

## In my scope and holding (no finding)
- TickLoop schedule: `due_ticks` on the loop clock, at most LOOP_CATCHUP (7) a wake, dropped time on the wake's first tick
  only, each series summing to the loop's counter (tests.rs:276-325; P5). Overrun from the schedule's own counter.
- Bounded channels: tick-ward `sync_channel(4,096)` with `try_send`, session-ward tokio `channel(2,048)` with
  `try_send`, writer `sync_channel(1,024)` with `try_send`. Each full one is counted, and the tick never waits on one.
  The one blocking send is `Row::End` after the loop has stopped (:766), so a footer is not lost. P1 fails the flood test.
- Decoding off the tick thread: the tick thread gets decoded `Message`s only, and the link is polled on `npt-io` only
  (tests.rs:462-503).
- Register 96: each report carries `Decoder::rtt_ns`, which the tick thread copies into each slot before every tick
  (:527-535; tests.rs:538-567; P3). The room has no reader (F1).
- C-U6-16 (c): `channel_of` is unambiguous at any length (proved by hand above), the fallback is Control, and P2 fails
  the test.
- Kick, leave and replace: the replaced session gets Kick then its close flag and is unrouted at once. Its later
  Leave never reaches the room (P6). A Leave frees the slot and closes the session, and a kicked one closes after its
  last messages (code at :728-731, untested: O1).
- Panic path: `catch_unwind` around the loop; `panic = "unwind"` (the workspace sets no abort). RoomEnded(crash) goes
  to each session, then `tick_panic` with `server_ns` and `t`, then the crash footer. `stop()` gives `Ended::Crash`,
  the binary exits 3. A double fault still ends Crash (`join` Err), with a `stopped` footer. P4 fails the test.
- Binary: settings refused and taken as SEC-U6AD-2 says. Token file 0600 by `create_new`, an existing path exits 2,
  48 tokens distinct, `nptj_` + 22 base64url, absent from log, stdout and stderr (binary.rs:145-156). OPEN within 5 s
  (20 starts: p50 634 ms, max 979 ms, debug build, a reading). Exit 0, 2, 1 and 3 mapped (main.rs:399-537). stdin is
  read on a std thread spawned after the pin, so it inherits the I/O set and never counts against `alone`. No socket.
- S4: steal from the tick vCPU's line only (P9). RSS from VmRSS. Per-thread schedstat. All reads run on the writer or
  the caller thread, never the tick thread. Kernel threads follow condition 4's rule exactly (fixtures for per-CPU,
  unbound and user; P8). taskset runs at /usr/bin/taskset with env_clear, integer arguments, no shell and exact
  read-back. The header's machine comes from `read_local`, `clock_res_ns` is measured, `build` holds the binary's
  SHA-256 by a fixed buffer, and carry 60's seven params come from `Scene::weapon()`/`health()` for four scenes (P11).
  The no-smt-map bite clears only the header's map; `sibling_free` keeps the real one.
- Room log -2: fixed `TickRow`s go to the writer thread. The counts are compared tick by tick with an independent
  count (BTreeMaps, the sim's dump events), and every name is on every record. A stuck writer drops rows (counted)
  and the tick runs on.
- npt-super timer and hold form: spawn to OPEN on its own clock (npt-super has no clock ban). Command with separate
  arguments, stdout lines bounded at 256 B, stderr inherited and never logged. A start past the deadline is killed,
  waited for, its elapsed time written and its exit non-zero. The hold form keeps the room until npt-super's stdin
  closes. A killed npt-super still closes the room's stdin pipe, so the room ends cleanly.
- C-U6-17 cap and test, C-U6-18 order: above.
- Bans: .claude/rules/netcode.md (no client state accepted; inputs decoded and checked by the base decoder; queues
  bounded; no wire change, so no catalogue edit). Clock: one item, plus the test's allowed real-clock reads. No
  HashMap/HashSet. No socket. LEAD-BOUND 400 ms untouched. No client string or token in any log, event or panic text.
- Carries by number: C-U6-17 met; C-U6-18 met; C-U6-4 npt-server's half met (StealReader stays pub(crate)); carry 60
  met for npt-server's header; register 96 met for TickLoop's half (F1); C-U6-16 (c) met; C-U6-14's first clause met
  (the kicked branch untested, O1); C-U6-19's gate line stated (tests.rs:3-5); SEC-R1D-1 is Block 0's (not in this diff).
  Conditions 3 (OPEN text, hold form), 4 (alone), 5 (traps, writer from Block 2, pub(crate), cpu_read source, pins
  unchanged, host_step order) met, except NR-U6A-1 against condition 3's intent. SEC-U6AD-1 to -6 hold.
- New pub items (in place of an engine-architect seat): `TickLoop` (start, connect, event, ticks, finished,
  clock_resolution_ns, stop), `LoopSettings`, `PhaseTimers`, `channel_of`, `ClientSlot::relevant`,
  `Machine::read_local` and `Behaviour::host_step`. Each has one consumer named, and none widens a trait.
  `LoopSettings` is beyond the brief's list but is `TickLoop::start`'s argument, so it is needed. Its fields are pub,
  `tokens` among them, with no Debug (SEC-U6AD-3). Its bites-only `bite` field means another crate's struct literal
  must cfg it too, which U6b's should know. `read_local` keeps Q-U6A-7's term (no clock, no dependency, builds for
  wasm32, the judge never calls it). `StealReader` stays pub(crate).

## Views on the builder's findings (ruling-ready)
- **F1 (the room's RTT and arrival stamp): fine as built, not a defect for R1; a carry.** The room is clock-free by
  design and takes no time input (room.rs:899, :912). Register 96's reader was "U4h's tolerance" (u4s-carries.md:
  628-632), which left with U4h. TickLoop's half is built, tested and planted (Core::rtt_ns, P3). The arrival stamp's
  consumer is U6b's reader stamp (u6b-brief.md:136, :251). No R1 mark reads either. Register 96 stays open as "the
  room's half waits for its reader": the advisor homes it with whichever unit adds a room input for RTT.
- **F3 (crate-private SimCore wrapper): fine as built.** It delegates every call, Room::tick steps once (room.rs:1061),
  and its counts equal the sim's dump tick by tick.
- **F4 (no CPU stand-in burn): not a defect for R1 (the header truthfully says none; -2 only reports the list);
  an unowned piece.** ADR 0004's S1 burn (scene.rs:7-8: "the server's tick loop reads these and burns them (U6)") has
  no unit after the re-plan. It must land before the first run that times the tick under S1 (the -4 to -27 runs that
  -2's notes list). For the advisor to home, likely U6e with the other writers. If that moves a kept run, it is
  Mustafa's.
- **F7 (Accept after JoinAccepted): fine as built and tested.** `accept_due` (tick_loop.rs:718-723), and the decode
  test asserts JoinAccepted arrives first (tests.rs:477-481).
- **F9 (the slow-down leg hides the cap's need): fine as built.** The test runs both legs, and P7b fails it on the up
  leg. Record the reading in the report.
- **O1 (kicked-session close untested): not a defect for R1** (R1 runs no never-acker, so the room kicks no one).
  Carry the test to U6d with C-U6-14's rest (its closed-session test takes a kicked session too). Low.
- **O2 (protocol version and content hash as binary constants, main.rs:32-35): fine for U6a** (no client joins until
  U6b). Carry to U6b and U7: one home both the binary and the clients or page reach, the `mode_hash` too, with a test
  that they are equal (Q-U6A-6's LEAD-BOUND pattern). main.rs's constants are binary-private today, so U6b's clients
  cannot import them.

## What I did not verify
- No cargo or judge run: NR-U6A-1 is read from eval.rs:47-61, :542-547, :560-564 and starts-super.jsonl. No judge was
  run over a room log plus a super log.
- The C-U6-17 cap when the cap shrinks faster than time (lo_rtt falls, or the offset estimate drops after a slow sample
  leaves). The target "never down" (client/src/lib.rs:671) can then sit above the new cap. That is outside the carry's
  named test.
- everyone_in_view parity with npt-lab's mirrored-frame flag (untested, above). Exit 3 end to end from the binary (the
  panic path is tested in the library only). The kicked branch (O1). A session-task panic on the I/O runtime (U6e's
  injection: the slot would stay held until a rejoin).
- Anything on RS: kernel-thread masks, hidepid, taskset on the image (U8). Real transports, 48 sessions through
  TickLoop (the smoke has 2), the 1 ms poll's CPU cost at 48 sessions.
- The test seat's lane (assertion strength, the plants' full set) beyond the spot-checks above.

DONE
