# U6a build review, netcode seat (round 1 of 1, NPT-TRIM), 2026-11-22

Inputs: u6a-brief.md rev 1.1 (sha256 c109691a... checked), wip/u6a-build.patch (sha256 a9f8b6e0... checked; base
aa5c62b6), built tree /home/user/Shamble-Frontlines/.claude/worktrees/agent-a95ebf14368f277ff (read only).
(No Edit tool in this seat: the notes file is rewritten whole with Write after each block.)
W = /home/user/Shamble-Frontlines/.claude/worktrees/agent-a95ebf14368f277ff/spikes/netcode-proof

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
  ids from /proc/thread-self, read-back, "a failed pin or read is written as 0 and on stderr, never hidden", no tick
  vCPU = no pin and no event, "the I/O set excluding the tick vCPU's SMT siblings is checked against the SMT map at
  start (exit 2 if the settings break it)"); u6a-build.md read.
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
- FINDING A (Medium): Q-U6A-5's term "the I/O set excluding the tick vCPU's SMT siblings is checked against the SMT map
  at start (exit 2 if the settings break it)" is not built: parse_settings checks only tick not in io (main.rs:165-167);
  run() has the SMT map (main.rs:448) but never checks. A run script that puts the sibling in io_vcpus starts the room,
  which then logs sibling_free 0 and spends the run. Fix: in run(), after `let smt = ...`, refuse with EXIT_SETTINGS when
  any io vCPU is in the tick vCPU's sibling list (proc_read's sibling_free over `[io_vcpus]`), + one refusal case.
- FINDING B (Medium): a failed or partial /proc walk reads as alone 1. proc_read::tasks returns an empty list when
  read_dir("/proc") fails and skips any task it cannot read (proc_read.rs:165-197); not_alone over an empty or partial
  list is empty, so TickLoop::affinity writes alone 1 (tick_loop.rs:1277-1278). Same with /proc mounted hidepid=2 or a
  systemd unit's ProtectProc=invisible (other users' threads invisible) or the TASKS_MAX cut. Breaks Q-U6A-5's "a failed
  pin or read is written as 0 and on stderr, never hidden"; -1's alone would then pass on evidence that does not hold.
  Fix (about 6 lines): in affinity(), alone only when the walk saw the tick thread and pid 1 (tid 1) and stayed under
  TASKS_MAX; else 0 and one stderr line.
- proc_read.rs otherwise right: PF_KTHREAD 0x00200000; flags = 7th field after the last ')' (fixture with "(odd (name)
  x)"); steal from cpu<N> only, total = first 8 fields; taskset fixed path, env_clear, integer args, std null, exact
  read-back (proc_read.rs:222-255). Machine::read_local: no clock, no new dep, judge never calls it (grep), wasm32
  clippy passed in check3; same reads as lab/src/machine.rs.
- Link Loopback Arc<Mutex> (link/src/lib.rs): const asserts, poison taken as is, l1 untouched. Fine.
- FINDING C (High): npt-super's log header cannot sit in the run whose room it starts. super/src/main.rs:181-188 writes
  bite None, tick_hz 0, scene "" (features []); confirmed in the real reading .gate/u6a/starts-super.jsonl line 1
  ("bite":null,"tick_hz":0,"scene":""). The judge groups logs by header.run and makes a run INVALID when its logs
  disagree on set, role, bite, layer, profile, tick_hz, scene or date (judge/src/eval.rs:47-61, :542-547). The hold form
  exists so "U8's run plan starts every room through npt-super" (condition 3; -16's notes "every room start in every
  run"): in that run, room (30, "SS") vs super (0, "") -> the run INVALID. In its own run id instead, the -16 bite can
  never be judged: super's header names no bite (and a bite run with no bites-build log is INVALID, eval.rs:560-564).
  The bite test hides it: binary.rs:215-221 builds its "npt-super form" log from the ROOM's header (process changed
  only), so the double-load evidence does not hold for npt-super's real log. Fix (about 15 lines): npt-super takes
  `--scene`, `--tick-hz` and an optional `--bite` and writes them into its header (log(), :161-193); one npt-super test
  asserts log()'s header carries them; the bite test builds its super log through the same fields; U8 carry: super's
  log takes the run id of the room it starts.
- C-U6-17 (client/src/lib.rs:653-668): goal capped at next_tick_at(now + min(samples.rtt)/2) + 7 beside max_lead; 7 =
  REDUNDANCY_CAP 8 - 1 = CMD_CATCHUP - 1 (proto limits.rs:17, room input.rs:177). The carry's form. carry.rs: Delay link
  over Loopback (not l1), option B (room default), 10 ms path, 3rd exchange 600 ms slow both ways, buffer_full 0;
  P7b fails it on the up leg. Fine. (Note, not a finding: the target "never down" (:671) can sit above a cap that just
  shrank, e.g. lo_rtt falls; outside the carry's test, in not-verified.)
- C-U6-18: host_step (bots lib.rs:887-899) = step (endpoint.input(None, received) then the command) -> endpoint.step ->
  outgoing -> reliable; lab scenario.rs:1640 input(None), :1681 behaviour.step(&[]), :1688 endpoint.step, :1689
  outgoing, :1690 reliable. They match (condition 5 confirmed); a view host feeds the endpoint itself and passes &[].
- -2 parity: snapshot_clients equals lab's rule (logs.rs:629-632); everyone_in_view reads the room's relevant set (the
  mark's words, SERIES.md:273) where the lab read its mirrored frame (mirror.rs:324-346); the -2 test compares every
  count but sets `everyone: 0` on both sides (tests.rs:1071-1078), so flag parity with the lab is not tested (not
  verified; test seat's lane).
- Candidate (Low, carry U6e): the -2 counting runs on the tick thread after timers.end (tick_loop.rs:559-560), so its
  time is in no series (not work_ns, no phase); PM-0006-19's logging-cost ratio over work_ns cannot see it, and it runs
  with no log too (log None). For U6e's logging-cost work.
- Candidate (Low): a failed /proc/stat read or a total of 0 writes steal_permille 0 (tick_loop.rs:1019-1021), the best
  value -1 can read, so a failed read passes; rss and CPU likewise 0. Fix: leave the key out on None after the first
  second (a gap the judge sees).
- Candidate (Low, carry U6b): TickLoop::stop and Drop call runtime.block_on / shutdown_timeout (tick_loop.rs:1418-1427):
  called from inside an async context (U6b's listener runtime) they panic; and the runtime has no pub handle for U6b's
  listeners. U6b's brief names where TickLoop lives and who stops it.
- Evidence read: check3.log:331-342 all 11 PASS, test 866.8 s, 744/844, 0 failed, 0 skipped; starts2.log exit 0.
