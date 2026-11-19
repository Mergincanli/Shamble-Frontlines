# U4f review panel, round 1: seat netcode (netcode-architect)

Tree: /home/user/Shamble-Frontlines/.claude/worktrees/agent-a0825a5a2491eb175 (HEAD 07289a4, U4f uncommitted). Read only.
No Edit tool in this seat: this file is rewritten whole with Write at each save.

## Plan
1. Read the scope: history.md:6896-6903 (SIZE-U4F (a)), advisor-u4f-stop.md (R2, R3, S1-S7, N1, :80-89 the U4f2 list).
2. Read u4f-brief.md 2.1-2.9 (clock, lead controller, command timing, checks, never-acker, cap, carry 80), 5.1;
   advisor-u4f-build1.md R2 (g), F1; advisor-u4f-brief.md's rulings :173-198.
3. Read ADR 0001:225-255 (commands, time) and plan §4.8 (u4-build-plan.md:338-341).
4. Read the diff in the worktree: client/src/clock.rs, client/src/lib.rs (controller), room/src/input.rs, room/src/room.rs
   (receive, tick, fill_input, route_events, encode_all, kick), events.rs, pump.rs's Pong, lab/src/scenario.rs (Server
   io/send, the run loop, the clock suite, FastClock), bots' fast clock and forgers, c-u4f-runs.mjs, c-u4f-r2.mjs,
   client/tests/trace.rs, room/tests/commands.rs.
5. Check the room's per-tick order and the lead bound counted from the tick the room runs next.
6. N1: hand-s22's 30 refused rewinds (w2-hand.log:32, :53): whose and why, from the logs and the code.
7. Runs only to prove a finding (none needed: every finding below is shown by the code and the existing logs).
8. Verdict and findings table at the end.

## Findings (one line each, as found)
- NC-U4F-1 (Medium): client/src/lib.rs:650-672 (`plan`): the target rises at most 2 a step from the old target whatever
  the goal (:655), so with every due command given `owed` never exceeds 4 and the skip branch (:668-672) is unreachable;
  after a forward jump of J ticks (a ClockSync reset, or a host stall of J ticks) the stamps lie about J ticks behind the
  room for about J steps, all late, `lead_now` negative. Against 2.2 rev 3.2 (TE31-1): "only a target beyond that (a
  ClockSync reset) leaves the stamps between unused, counted per client (`stamps_skipped`)", and 2.2's Bounds "at least
  1". f32 (ii) (client/tests/trace.rs:420-432) asserts only rise, bound and resets, so it passes. No U4f run reaches it.
- N1 settled (line of its own below).
- NC-U4F-2 (Low): ops/runs/c-u4f-runs.mjs:49 (SETS[22].args): -22's judged set and its bite run carry no `--controller
  1` (scenario.rs:2274 forces it for `clock` only); hand-s22's header has `lead_ticks_count` 2 and its line "lead 2".
  So -22's 48 clients run U4r's fixed lead, outside U4F-Q7 (i)'s scope (U4r's suites and -30's pre-check), while -20,
  -47 and every -21 row run the controller and the cap (B3) was measured under it. No -22 check reads the lead.
- NC-U4F-3 (Low): lab/src/scenario.rs:47-49 and client/src/lib.rs:354-357 (`with_clock`'s doc): "at most 9 so option
  B's buffer holds every command" holds only while the up delay is at least about 2 ticks whenever the cap binds. The cap
  counts from the clock's current tick at the send (`next_tick_at` is up to 1 above the true next tick:
  clock.rs:213-221), so with the upper one-way estimate high (one slow pong keeps the 2 s window's max RTT up:
  clock.rs:225-229) and the real delay under a tick, stamps reach 9-10 past the room's last tick and option B's 8 refuse
  the excess as kind 10 (input.rs:321-323), a forged kind, on an honest client. Not reachable on L1 (bounded uniform
  jitter, l1/src/lib.rs:150-202).
- NC-U4F-4 (Low, a carry): client/src/lib.rs:729-737 and clock.rs:131: each `Ping`'s send time is the step's `now`; ADR
  0001:242-245 has the host pass "the time it sent the previous step's datagrams". Equal on L1 (the lab sends at the
  step's instant: scenario.rs:1593); in the browser's Worker a delay d between the step and the send biases every
  offset sample by d/2 and every RTT by d.
- NC-U4F-5 (Low, report): the controller's buffer target is padded beyond "the smallest input buffer that holds" (ADR
  0001:252-253): one tick period of margin (lib.rs:634-636), `next_tick_at`'s upper bound (up to one more period of
  phase), and `one_way_hi_ns` = max RTT less half the min RTT (the one-way maximum plus half the spread). Option B's
  measured depth in -20's runs is then an upper bound on B's smallest buffer, which OQ-0001-6 sets against A's "none".
- S1-S7 (advisor-u4f-stop.md:198-210): no disagreement. S1 in particular: the fast clock is on a fixed lead (scenario.rs
  :1192), no -21 check reads a ping series, and a controller-driven 1.5x clock would see its offset move 0.5 s a second
  and reset at every sample, so its sending no `Ping` is the sound choice.

## Checked and right (no finding)
- ClockSync against ADR 0001:239-245 and plan §4.8: NTP's offset (t0 + t1 + hold) / 2 - s and RTT t1 - t0 - hold
  (clock.rs:155-158) are right for the room's `Pong` (server_ns = the pump's `now` at the send, hold = now - the ping's
  arrival: pump.rs:458-468); drift 2 ms a step, halve over 100 ms, reset over 500 ms (clock.rs:188-200); samples timed by
  arrival (lib.rs:542-548), never by the step; no clock read; integer ns; outstanding 8, samples 64, window 2 s; nothing
  exposed for interpolation. The lab's -47 read instant and true offset (scenario.rs:1530-1546) match 2.1.
- `next_tick_at` (clock.rs:215-221): with the Pong naming the next tick to run at its send and that tick due at or after
  the send, the formula gives the true next tick or one above, never below (exact estimate); the wrap of a negative tick
  count through `as u32` is right.
- The lead bound counted from the tick the room runs next: kind 1 at input.rs:265 with `next_tick` = the room's next
  tick at the receive (room.rs:923; io runs before the due tick at the same instant: scenario.rs:1323-1349); the join
  anchor and `JoinAccepted.tick` are both the next tick (room.rs:829, :857); the controller's cap counts 9 from its
  estimate of the current tick, which lies at most at the true next tick, so an honest stamp is at most 9 past the
  room's next tick at the send and at arrival (margin 3 under the 12); the fast clock's every 10th stamp newest + 3 +
  bound (bots lib.rs:216-223, F1) and the kind-1 forger newest + 2 + bound + k (bots lib.rs:177) both exceed it at P0.
- Both options: A runs on arrival up to 8 a tick (input.rs:316-323, drain :384-391), past window W = 8 by signed
  distance from the next tick (:310), arrival order then the total key (room.rs:1093, :1109; canonical :444-446); B
  keeps U4e's buffer with kind 2 ahead of `late` (NA-13) and its repeat on a miss (input.rs:365-371). `cmd_on_time` and
  `input_missing` by stamp, computed before take or drain (room.rs:1072). Every tick compare is a signed distance.
- The room's per-tick order (room.rs:1027-1051): begin_tick, the refusal records swapped (receives before this tick),
  fill_input (kicked skipped, on_time, take or drain, canonical), the step, history, flood, reports, interest,
  route_events (expire, queue, overflow: one forced resync, a second inside it kicks), encode_all (an unacknowledged
  forced full state past FULL_STATE_ACK_TICKS kicks; a full state, else no snapshot while one is in flight, else a
  snapshot), next tick. The lab's instant: accepts, io (pump, Join, receive), ticks, each slot's send (the Kick sent,
  then the kicked session dropped: scenario.rs:711-745), then the clients' steps.
- The never-acker, the cap and carry 76's Kick path: hand-s22 shows the designed path at the new cap (overflow resync at
  t 89 with headroom 0 and `ack_starved` dropped 0; kick at t 150 = 89 + 61 > FULL_STATE_ACK_TICKS 60 with dropped 1;
  its counts written once at the kick: lab.jsonl `session_end` "kicked"); -22's checks read window "all"
  (marks.json:5170-5194), so the warm-up's overflow counts; 1,600 sits under the never-acker's own P0 rate x 8.5 s
  (about 546 a second here, about 4,600) as under carry 50's 4,328. U4f changed carry 76's path only by the
  `session_end` record (scenario.rs:729-743 against HEAD's :592-599).
- No -21 judged path before U4f2: c-u4f-runs.mjs's SETS hold 47, 22 and 20 only; c-u4f-r2.mjs's -21 rows write under
  .gate/ and fail by length, as the stop ruling reads them.

## N1 (its own line)
N1: hand-s22's 30 refused rewinds, and hand-s22b's 1,018, are all slot 0's, the never-acker's (s22/lab.jsonl
client_counts slot 0: rewinds_refused_count 30, commands_count 2, steps_without_command_count 177; s22b slot 0: 1,018,
1, 1,077; every other slot 0). Why: the never-acker never acknowledges its join's full state, so the room sends it no
snapshot (room.rs:1319-1321: no snapshot while a full state is in flight) until the overflow's forced one at t 89; its
endpoint's newest tick stays put, so under -22's fixed lead (NC-U4F-2) the bot gives a command only after each full
state; option B runs the last command again every tick (fire held) with that full state's view tick, which the 2 s hit
history (HISTORY_MS 2,000, 60 ticks) drops 60 ticks later: refused from about t 62 to 89 and t 149-150 (the kick at
150), 30; in s22b (no overflow, no kick) from about t 62 to the end, 1,018 of 1,080 ticks. Expected and harmless: no -22
check reads rewinds. The -22 set report should say so in one line, so no reader takes it for a room defect.

## Verdict
APPROVE WITH FINDINGS. One Medium (NC-U4F-1), for this round's fix pass (a small change in `plan` and one test), and
four Lows; no Critical or High. Nothing found needs a marks.json, judge-file, SERIES.md-row or lead-bound change.

| Id | Severity | Where | What | Cheapest proof | Fix |
|---|---|---|---|---|---|
| NC-U4F-1 | Medium | client/src/lib.rs:650-672 | the target ramps 2 a step from the old target, so the reset skip is dead code and a forward jump of J ticks leaves stamps J behind for about J steps, all late, `lead_now` < 1 | f32 (ii) with two added asserts after the 12 s jump: `stamps_skipped() > 0` and, 2 s plus 10 steps after it, `lead_now() >= Some(1)` and every stamp arriving no more than 8 behind the room's next tick: both fail as built (lead about -350, skipped 0); a stall case (no `input`/`step` for 1 s) shows the same | in `plan`, before the 2-a-step arm, `Some(t) if dist(goal, t) > REDUNDANCY_CAP => goal` (the existing owed > 8 branch then skips and counts); f32 (ii) gains the asserts above and a stall case; f5, f32 (i) and f36 still assert `stamps_skipped` 0 on honest traces |
| NC-U4F-2 | Low | ops/runs/c-u4f-runs.mjs:49 | -22's set and bite run on U4r's fixed lead, not the controller | hand-s22's header `lead_ticks_count` 2 (w2-hand/s22/room.jsonl:1) | add `--controller 1` to SETS[22].args (no run count, mark or judge change), or name the choice and why in U4f's report |
| NC-U4F-3 | Low | lab/src/scenario.rs:47-49; client/src/lib.rs:354-357 | "at most 9 so option B's buffer holds every command" fails when the cap binds with a sub-tick real delay (an RTT spike): kind 10 on an honest client | a trace with one 600 ms pong and a 10 ms path under a model option-B buffer of 8 | reword the doc to its true condition and carry for L2: cap the stamp at `next_tick_at(now + lo / 2) + 7` (the earliest arrival's next tick + 7) |
| NC-U4F-4 | Low | client/src/lib.rs:729-737; clock.rs:131 | the ping's send time is the step's `now`, not the host's send time of ADR 0001:242-245 | read; equal on L1 | a carry to the Worker host's unit: the endpoint takes the previous step's send time and ClockSync stamps the ping with it |
| NC-U4F-5 | Low | client/src/lib.rs:634-646; clock.rs:213-229 | the buffer target carries a margin period, a phase upper bound and the RTT spread: B's measured depth is an upper bound on B's smallest buffer | read | -20's set report and the OQ-0001-6 reading name it; no code change in U4f |

## Ran (read only; no cargo, no check.mjs, nothing in the worktree changed)
- git log / status / diff --stat / diff HEAD of the worktree's files named above; git show HEAD: of input.rs and of
  scenario.rs's `send`.
- Reads (sed, grep, Read) of the brief, the rulings, ADR 0001, the plan, the build notes and the sources.
- Greps of .gate/u4f/w2-hand.log, w3-hand.log and the run folders .gate/u4f/w2-hand/s22 and s22b (lab.jsonl
  client_counts per slot, overflow_resync, session_end, ack_starved; room.jsonl's header).

## Not checked
- Nothing was built or run by this seat; NC-U4F-1's and NC-U4F-3's numbers are read from the code, not measured.
- U4f2's list (the forgers of kinds 3, 7-11, `--suite forged`, the attribution, the twins, f16, f22, f23, f33, p1, p7,
  p14, p17, -21's set, f28's B halves); the forgers of kinds 1, 2, 4, 5, 6 only for their kind-1 anchor.
- The security seat's hostile-value review of 2.4's kinds beyond the order and the anchors; the rust seat's overflow and
  allocation review; the test seat's counts, bites and SERIES.md rows; the arch seat's public items.
- carry 80 (b)-(e) beyond reading (e)'s site; the logs.rs writers; the contract checker; the -47 suite's rejoin logging.
- Whether the sim's debug order assert (world.rs:257-261) can meet a slot's command ticks 2^31 or more apart under the
  no-lead-bound bite (the builder notes the equal-tick case; release bite runs carry no debug asserts).
