# U4f's review round 1: the five seats' results (workflow wf_a011d9f2-abf), 2026-11-19

Written by script, word for word, from the workflow's journal (each seat's StructuredOutput result). Tree reviewed: builder 3's worktree, equal to wip/u4f-build.patch 74ae26aa... (runner hashes diff 54cb6faa..., untracked e8dc3663...). The brief: u4f-panel-brief.md. Each seat's notes: u4f-review-<seat>.md. Nothing below is the main session's.

## netcode (netcode-architect): APPROVE WITH FINDINGS

**Notes file:** /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-review-netcode.md

### NC-U4F-1 (Medium)

- **Where:** worktree spikes/netcode-proof/crates/client/src/lib.rs:650-672 (ClientEndpoint::plan); the target arm at :655; the skip branch at :668-672; test client/tests/trace.rs:420-432
- **What:** The controller raises its target by at most 2 a step from the old target, whatever the goal. When the page gives every due command (the bots under controlled() and the trace test both do), `owed` never goes above 4. So the skip branch (owed > REDUNDANCY_CAP, stamps_skipped) can never run. Concrete case: the server's time jumps J ticks ahead of the client's estimate. A ClockSync reset does this (f32 (ii) jumps 12 s, J = 360), and so does a host stall of J ticks (no input or step called, e.g. a throttled Worker). After that jump the stamps sit about J ticks behind the room's next tick for about J more steps. Every one of those commands is late: under option B it is stamped for a tick already run; under option A it is more than W = 8 behind. lead_now goes negative (about -350 in f32 (ii)), below 2.2's Bounds of 'at least 1'. This goes against 2.2 rev 3.2 (TE31-1, U4F-Q7 (iii) (c)): 'only a target beyond that (a ClockSync reset) leaves the stamps between unused, counted per client (stamps_skipped)'. f32 (ii) asserts only rise, bound and resets, so it passes. No U4f run reaches this: L1 has no resets or stalls, and B1's 1 s burst keeps samples inside the 2 s window. The client is never counted as forged here, only late.
- **Proof:** By reading plan(). Let owed_k = target_k - last_k. After a step where all due commands are given, either last = target, or last = last_prev + 2 and target - last <= 2. Then target rises by at most 2, so owed <= 4 < 8. Cheapest proof for the fix pass: add two asserts to f32 (ii) after its 12 s jump. One: stamps_skipped() > 0. Two: 2 s plus 10 steps after the jump, lead_now() >= Some(1), and every stamp arrives no more than 8 behind the model room's next tick. Both fail as built. Add a stall case as well (no input or step for 1 s, then on-time within a few steps); it fails the same way.
- **Fix:** In plan(), put one arm before the 2-a-step arm: `Some(t) if dist(goal, t) > Limits::REDUNDANCY_CAP as i64 => goal`. The existing owed > 8 branch then skips the stamps in between and counts them. Add the asserts above and a stall case to f32 (ii). f5, f32 (i) and f36 keep asserting stamps_skipped 0 on honest traces: L1's bounded jitter never moves the goal more than 8 past the target.

### NC-U4F-2 (Low)

- **Where:** worktree spikes/netcode-proof/ops/runs/c-u4f-runs.mjs:49 (SETS[22].args); lab/src/scenario.rs:2274 forces the controller for suite clock only
- **What:** -22's judged set and its PM-0001-22/unbounded bite run pass no `--controller 1`. All 48 clients, the never-acker included, therefore run U4r's fixed lead: hand-s22's header has lead_ticks_count 2, and its summary line says 'lead 2'. U4F-Q7 (i) keeps with_lead for U4r's suites and -30's pre-check only. -20, -47 and every -21 row run U4f's controller, and the cap of 1,600 (B3) was measured under it. No -22 check reads the lead, so the verdict should not move, but the set does not run the honest client U4f built.
- **Proof:** .gate/u4f/w2-hand/s22/room.jsonl:1 (params.lead_ticks_count 2, no controller_max_lead_count); w2-hand.log:31-32; c-u4f-runs.mjs:49.
- **Fix:** Add `--controller 1` to SETS[22].args. This changes no run count, mark, judge file or SERIES.md row. Alternatively, name the choice and the reason for it in U4f's report.

### NC-U4F-3 (Low)

- **Where:** worktree spikes/netcode-proof/crates/lab/src/scenario.rs:47-49 (CONTROLLER_MAX_LEAD's doc); client/src/lib.rs:354-357 (with_clock's doc); room/src/input.rs:321-323
- **What:** The docs say 'at most 9 so option B's buffer holds every command'. That holds only when the up delay is at least about 2 ticks whenever the cap binds. The cap counts 9 from the clock's estimate of the current tick at the send, and next_tick_at (clock.rs:213-221) can be one above the true next tick. one_way_hi_ns takes the 2 s window's maximum RTT (clock.rs:225-229), so a single slow pong pushes the goal to the cap while the real one-way delay stays under a tick. In that case honest stamps reach 9-10 past the room's last tick. Option B's 8 then refuse the excess as kind 10, which is a forged kind, on an honest client. This cannot happen on L1, whose jitter is bounded and uniform (l1/src/lib.rs:150-202), so no U4f run meets it. It can happen on L2 or a real network.
- **Proof:** By the arithmetic: stamp <= next_est + 8 <= true_next_at_send + 9, and Full needs stamp - room_next_at_arrival >= 8. That allows at most 1 tick between send and arrival while the cap binds. A trace with one 600 ms pong and a 10 ms path, under a model option-B buffer of 8, shows it.
- **Fix:** Reword both docs to state the true condition. Carry to L2: cap the stamp at the earliest arrival's next tick + 7 (next_tick_at(now + lo_rtt / 2) + 7), which holds under both options with no read of cmd_timing.

### NC-U4F-4 (Low)

- **Where:** worktree spikes/netcode-proof/crates/client/src/lib.rs:729-737 (the controller's Ping, sent_ns = self.now_ns); client/src/clock.rs:131 (outstanding records now_ns)
- **What:** ClockSync uses the step's `now` as each Ping's send time t0. ADR 0001:242-245 has the host pass 'the time it sent the previous step's datagrams' with each input call, and the endpoint's input() takes no such value. On L1 the two times are equal, because the lab sends at the step's instant (scenario.rs:1593). In the browser's Worker, a delay d between the step and the actual send would bias every offset sample by d/2 and every RTT by d.
- **Proof:** By reading. There is no run to make; on L1 the delay is 0.
- **Fix:** No change in U4f. Carry it to the unit that builds the Worker host: the endpoint takes the previous step's send time, and ClockSync stamps each ping with it.

### NC-U4F-5 (Low)

- **Where:** worktree spikes/netcode-proof/crates/client/src/lib.rs:634-646 (margin and goal); client/src/clock.rs:213-229
- **What:** The controller's buffer target is padded beyond 'the smallest input buffer that holds' (ADR 0001:252-253). It adds one tick period of margin (for a lost first copy), next_tick_at's upper bound (up to one more period, for a phase the client does not infer), and one_way_hi_ns, which is the maximum RTT less half the minimum RTT (the one-way maximum plus half the spread). Option B's measured depth in -20's runs is then an upper bound on B's smallest buffer. OQ-0001-6's rule sets that depth against option A's 'none'.
- **Proof:** By reading. f5's settled leads (P0 3, P1 3, P2 4, P3 5, P4 6: u4f-build.md:121-123) sit near the one-way delay plus about 2 ticks.
- **Fix:** No code change in U4f. -20's set report and the OQ-0001-6 reading say that B's depth is the controller's padded target, with its three terms, so B is not judged on a padded depth.

### Its own lines

- N1: hand-s22's 30 refused rewinds, and hand-s22b's 1,018, are all slot 0's, the never-acker's (s22/lab.jsonl client_counts slot 0: rewinds_refused_count 30, commands_count 2, steps_without_command_count 177; s22b slot 0: 1,018, 1 and 1,077; every other slot has 0). Why: the never-acker never acknowledges its join's full state, so the room sends it no snapshot (room.rs:1319-1321: no snapshot while a full state is in flight) until the overflow forces one at t 89. Its newest tick stays put, so under -22's fixed lead the bot gives a command only after each full state. Option B then runs the last command again every tick (fire held) with that full state's view tick, and the 2 s hit history (60 ticks) drops that tick 60 ticks later. Refused: about t 62-89 and t 149-150 (kick at 150), 30 in all; in s22b (no overflow, no kick) from about t 62 to the end, 1,018 of 1,080 ticks. Expected and harmless: no -22 check reads rewinds. The -22 set report should say so in one line.
- S1-S7 (advisor-u4f-stop.md:198-210): no disagreement. On S1: the fast clock runs on a fixed lead (scenario.rs:1192) and no -21 check reads a ping series. A controller-driven 1.5x clock would see its offset move 0.5 s every second and reset at every sample, so sending no Ping is the sound choice.
- The lead bound is counted from the tick the room runs next (R2 (g)), checked at every site. Kind 1 is at input.rs:265, with next_tick taken at the receive (room.rs:923); io runs before the due tick at the same instant (scenario.rs:1323-1349). The join anchor and JoinAccepted.tick are both the next tick (room.rs:829, :857). The controller's cap lets an honest stamp go at most 9 past the room's next tick, a margin of 3 under the 12. The fast clock's every 10th stamp (newest + 3 + bound) and the kind-1 forger (newest + 2 + bound + k) both exceed the bound at P0.
- The room's per-tick order is right: begin_tick, refusal swap, fill_input (on_time by stamp before take or drain, kicked skipped, canonical total key), step, history, flood, reports, interest, route_events (one forced resync, then a kick), encode_all (an unacked forced full state kicks after 60 ticks), next tick. The lab's instant runs io, then ticks, then each slot's send (Kick, then the kicked session dropped), then the client steps. hand-s22 shows the never-acker's path at cap 1,600: resync at t 89 with headroom 0, kick at t 150, counts written once at the kick.

### What it ran

- git -C-free reads in the worktree: git log -1, git status --short, git diff HEAD --stat, git diff HEAD of client/src/lib.rs, room/src/input.rs, room/src/room.rs, room/src/events.rs, server/src/pump.rs, proto/src/msg.rs, sim/src/world.rs, docs/design/netcode-proof-messages.md
- git show HEAD: of room/src/input.rs and of lab/src/scenario.rs's Server::send (before/after of carry 76's path)
- sed/grep/Read of the sources: client/src/clock.rs whole, client/tests/trace.rs :1-480, room/tests/commands.rs :1-175 and :820-1000, lab/src/scenario.rs (Server, run loop, clock suite, FastClock, f19/f28 fast tests, header params), bots/src/lib.rs (forged, fast_stamp, outgoing, step), l1/src/lib.rs profiles, sim/src/ticks.rs, ops/runs/c-u4f-runs.mjs whole, c-u4f-r2.mjs :1-102
- sed/grep of u4f-brief.md (2.1-2.9, 5.1, Q3, Q6-Q8, f-rows), advisor-u4f-stop.md, advisor-u4f-build1.md :1-160, advisor-u4f-brief.md :173-198 and :369-419, history.md :6780-6910, u4f-build.md (blocks 3, B5, hand runs), ADR 0001:225-255, u4-build-plan.md §4.8, marks.json :5150-5230, u4s-carries.md carry 80
- grep of .gate/u4f/w2-hand.log and w3-hand.log; grep of .gate/u4f/w2-hand/s22 and s22b lab.jsonl (client_counts per slot, overflow_resync, session_end, ack_starved) and room.jsonl's header
- No cargo, no check.mjs, no runner call: every finding is shown by the code and the existing logs

### What it did not check

I built and ran nothing. NC-U4F-1's and NC-U4F-3's numbers come from reading the code, not from a run. I did not review U4f2's list: the forgers of kinds 3 and 7-11, --suite forged, the attribution, the twins, f16, f22, f23, f33, p1, p7, p14, p17, -21's set and f28's B halves. I read the forgers of kinds 1, 2, 4, 5 and 6 only for their kind-1 anchor. Other seats cover: the hostile-value review of 2.4's kinds beyond the order and the anchors (security), overflow and allocation (rust), the 2.10 counts, bites, SERIES.md rows and the marks.json/judge read (test), and the public items (arch). Not read beyond the parts named: carry 80 (b)-(e) past (e)'s site, the logs.rs writers, the contract checker, and the -47 suite's rejoin logging. Also not checked: whether the sim's debug order assert (world.rs:257-261) can meet one slot's command ticks 2^31 or more apart under the no-lead-bound bite. The builder already handles the equal-tick case, and the release bite runs carry no debug asserts.

## security (security-auditor): APPROVE WITH FINDINGS

**Notes file:** /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-review-security.md

### SEC-U4F-1 (Low)

- **Where:** spikes/netcode-proof/crates/bots/src/lib.rs:167-192 (`forged`), :307-316 (`forger`, `every.max(1)` at :311), :461-476 (`outgoing`)
- **What:** The kind 2 and kind 5 forgers forge against `prev`, the endpoint's previous command, not against the room's newest accepted command. `forger` allows `every` = 1. Example for kind 2: seq n is forged and refused; seq n+1 is then forged to tick_n, which a correct room accepts because tick_n is newer than tick_{n-1}. For kind 5, the fire counter set to prev + covered + 1 equals the room's own covered count when the steady script holds fire without pressing it (no edge on n), so it also passes. A correct room accepts these commands, and U4f2's attribution would count them as unrejected: a false FAIL. The same happens after a lost previous command on a lossy profile. Exploit sketch: none against the server; this is a trap in the test harness for U4f2's forged suite.
- **Proof:** Cheapest proof is a room-level test in f16's shape: run `forger(2, 1, 12)` and `forger(5, 1, 12)` on the steady script at P0 into a `Room<World>` and count forged seqs that have no refusal record of their kind. Today that count is above 0 (about half). f15 cannot show it: it uses every 3 and checks only the forged field against the endpoint's previous command (bots/src/lib.rs:1044, :1093-1106). This is a plant for the fix pass; I did not run it.
- **Fix:** `forger` returns None for every < 2, and its doc states the precondition: the previous command must have been accepted, which holds for every >= 2 on a lossless profile (P0). U4f2's brief carries the same rule to `--forge-every`'s parser (refuse < 2) and to f16. A few lines; it rides the fix pass or moves to U4f2 by number.

### SEC-U4F-2 (Low)

- **Where:** spikes/netcode-proof/crates/lab/src/scenario.rs:1899; spikes/netcode-proof/crates/room/src/room.rs:599-605 (`with_lead_bound_ms`, public), :625-628 (`lead_bound()`, documented as 'for the run's header')
- **What:** `header.params.lead_bound_ms` is written from the constant `Room::LEAD_BOUND_MS`, not from the room's own `lead_bound()`. No lab path calls `with_lead_bound_ms` today, and the fast clock's excess uses the room's real bound (scenario.rs:1186), so no run is wrong now. But if a later lab path set another bound, every header would still record the frozen 400 ms while the room ran the other value. The LEAD-BOUND record would then be false, and a -21 run could pass under a wider bound than its header states.
- **Proof:** Grep, done: scenario.rs:1899 inserts `Room::<Sim>::LEAD_BOUND_MS`, and the only caller of `with_lead_bound_ms` is room/tests/commands.rs:814.
- **Fix:** Write `server.room.lead_bound().0` into the header, or assert at the header write that the room's value equals `Room::LEAD_BOUND_MS`. One line; rides the fix pass.

### Its own lines

- S1 (the fast client sends no Ping) against my SA3-7: CONFIRM. SA3-7 asked only that its ping overflow be reported, never failed. No room check reads a client's Ping: kind 1 counts from the room's own next tick (input.rs:265). The fixed-lead endpoint still answers the room's own Pings (client/src/lib.rs:712-725, :788-792), so the room keeps its RTT samples for that client, and the ping bucket keeps its pump tests. The suite simply sends no ping load; no check is weaker.
- Nothing built lets -21 be judged before U4f2. c-u4f-runs.mjs has no -21 set (SETS holds 47, 22 and 20; its usage check at :62 refuses any other key). c-u4f-r2.mjs's -21 rows are 20 s and 70 s pre-runs written only under .gate/, never results/, and each -21 count part fails there by length (1,000 points, 100 bursts, 10 episodes; no forged rows). None of the 20 new SERIES.md rows claims a -21 judgement.
- S1-S7 and N1 as ruled (advisor-u4f-stop.md:198-210): I disagree with none.
- Hostile input as built: the room-side checks of kinds 1-6, 8 and 10 run in 2.4's order on session-scoped values. The slot is overwritten from the session (room.rs:992). Every structure a client sizes is bounded: kept window 16, stamps 64, buffer 8, refusal records 4,096 a tick, event queue 1,600. Every new tick compare is a signed distance, and no new arithmetic on a wire value can panic in a debug or test build.
- Never-acker and carry 76's Kick path: a client that acks 0 groups fills its queue to 1,600 and gets one forced resync. A second overflow inside that resync, or the forced full state still unacknowledged after FULL_STATE_ACK_TICKS, kicks it (room.rs:1255-1257, :1289-1295). The kicked slot's counts survive (scenario.rs:729-741). The cap of 1,600 sits under carry 50's 4,328: I agree with R2.
- Recorded design property for the OQ-0001-6 choice (C-OQ6-SEC-1; ruled by U4F-Q18 (a), not a finding): the sim advances movement, the weapon clock and the reload once per command run (world.rs:1409-1417). So under option A a burst after an idle compresses up to W + bound = 20 ticks of all three into about 3 ticks (f38's bound). Under option B it is one command a tick.
- Carry for U4f2 (C-U4F2-SEC-1): `Room::refusals()` and `refusals_dropped()` have no lab reader yet. When the attribution reads the refusal records, it must also log refusals_dropped, so a refused forged item whose record was dropped can never read as unrejected. At 48 clients nothing can drop (at most 3,072 records a tick, under the 4,096 limit).
- Note N-SEC-1: a forged field inside a rejoin's overlap window (at most the bound's 12 ticks) is counted as `rejoin_overlap`, not under its own kind (input.rs:277-279 runs before kinds 4-6). The command is dropped either way and no judged forger rejoins.
- Memory not updated: this brief limits my writes to the notes file and runner logs. The proposed memory lines (threat model, the option A property, the two U4f2 carries) are in the notes file under 'Own lines'.

### What it ran

- sha256sum of /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/wip/u4f-build.patch (74ae26aa..., matches)
- git log -1, git status --short and git apply --numstat on the patch, run in the worktree folder (29 files)
- git diff HEAD of room.rs, input.rs, sim/world.rs, server/pump.rs, proto/msg.rs, room/events.rs, room/lib.rs, marks/SERIES.md and docs/design/netcode-proof-messages.md (read-only, in the worktree)
- Recomputed the runner's tree hashes with u4f-run.sh:67's own commands: diff 54cb6faa..., untracked e8dc3663..., equal to review-pre.log and both end checks
- Greps of the sources, marks.json and SERIES.md named in the notes (read-only)
- No cargo, no runner call, no plant: both findings are proved by reading and grep; no runner log written

### What it did not check

I built, ran and timed nothing. The end checks, the hand runs and the cap figures are the builders' logs, which I took from the stop ruling and the notes. Files I read only in part: lab/src/scenario.rs (:440-760, :984-1047, :1186, :1232, :1890-1905 of 4,215 lines), client/src/lib.rs (:700-800), client/src/clock.rs (:135-230), bots/src/lib.rs (:60-620, :1018-1162) and room/tests/commands.rs (:100-280 plus its test list). Not read: lab/src/logs.rs, the client tests (trace.rs, carry80.rs, bites.rs), proto/tests/pong.rs, room/tests/bites.rs, room.rs's unit tests, the server tests, c-u4f-cap.mjs, and how the judge reads the -21 series. Brief sections 2.10-2.13, 4, 5.3, 5.6 and 6, advisor-u4f-brief.md L1-L6, advisor-u4f-fold-check.md, and advisor-u4f-build1.md beyond its findings index are other seats' ground. Not checked: the client's ClockSync against a hostile server (outside this threat model; the rust seat's arithmetic), and the view-time tolerance and rewind cap (U4h's, known). Transport, origin checks, tokens, CSP, supply chain and CI are untouched by the patch (no manifest, lockfile, workflow or hosting file among its 29 files).

## test (test-engineer): APPROVE WITH FINDINGS

**Notes file:** /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-review-test.md

### TE-U4F-1 (Medium)

- **Where:** spikes/netcode-proof/crates/client/tests/trace.rs:385, :391 (doc :374-375)
- **What:** f5 checks a different number from the one the brief bars, and it loosens B1's bar to 3 % with no ruling. The test computes `late = 1 - on_time/ticks`, which counts ticks whose command never arrived as late. The brief's f5 (u4f-brief.md:688) says at most 1 % of commands may arrive after their stamped tick, on every profile, B1 included. The code uses `budget = if p.burst.is_some() { 0.03 } else { 0.01 }`, while the test's own doc still says 1 %. Neither advisor-u4f-build1.md nor advisor-u4f-stop.md rules on it; builder 1 only noted it (u4f-build.md:122-123). On the final tree, B1 is not on time on 0.01879 of its ticks, and 23 of about 1,490 ticks are missing. So only about 5 ticks (0.34 %) arrived late: the brief's 1 % bar, measured as written, would pass. The 3 % is not needed, and the evidence for B1 as built does not match the brief's claim.
- **Proof:** Run on the final tree, shared: .gate/u4f/review-test/f5-b1.log:16, 'f5 B1: settled lead 4 (U4R-Q14's 5), not on time 0.01879, missing 23, most ahead 3'. Builder 1's log shows the same line (.gate/u4f/b3-client.log:80).
- **Fix:** Count commands that arrived after their stamp apart from ticks whose command never arrived. Hold the late share at <= 1 % on all six profiles and print the missing count beside it. Or take a B1 bar to the advisor with its arithmetic. Make the test's doc match the code.

### TE-U4F-2 (Medium)

- **Where:** spikes/netcode-proof/crates/room/src/input.rs:179, :229-231; room/tests/commands.rs:464-484 (f12 (iv))
- **What:** U4F-Q9 (a)'s flood test was never built. Rev 3.3 of 2.10 makes it firm: '+1 by U4F-Q9 (a) (the kept window never holds more than K under a flood of new sequences, a negative case)' (u4f-brief.md:655-656). The advisor's ruling says the same (advisor-u4f-brief.md:265). No test reads the kept window's length. It is on no builder's not-built list (u4f-build.md:256, :539-541, :849-857) and not on U4f2's list (advisor-u4f-stop.md:80-89), so it went missing silently. f12 (iv) sends 20 sequences and checks that sequence 2 (18 back) is outside the window, which passes for any K up to 17: K = 16's edge is not pinned either. Because of this, U4f's negative cases come to 44 against its share of 45. The bound does hold by construction (pop_front at input.rs:229-231).
- **Proof:** Grep: KEPT_WINDOW appears only at input.rs:179 and :229. No test checks `kept.len()`. f12 (iv)'s case is 18 back.
- **Fix:** Add an in-crate test in input.rs that offers 10,000 new sequences (across the u16 wrap) and asserts `kept.len() <= KEPT_WINDOW` after each one. Pin f12 (iv)'s edge: a changed copy 16 back is kind 3 (inside), and 17 back is seq_outside_window.

### TE-U4F-3 (Low)

- **Where:** spikes/netcode-proof/crates/room/tests/commands.rs:946-960; room/src/input.rs:272
- **What:** The not-older-than bite's unit proof cannot tell its plant from no-newer-tick's. Both proofs run the same `equal_tick` case. If the plant at input.rs:272 (`d >= 0`) were replaced by `true` (kind 2 removed whole), both proofs would still pass, so the plant described as '`>=` in place of `>`' is not pinned. Separately, no test shows that this bite reaches the fast-clock suite's tally. Hand run p9 does show it: not_newer 0 and excess -7/-6, against clean p3's 29-30 and -36/-36 (.gate/u4f/w3-hand/p9/lab.jsonl, p3/lab.jsonl).
- **Proof:** Describe the plant: change input.rs:272 to `let newer = true;` under the bite. pm_0001_21_not_older_than_lets_an_equal_tick_command_run still passes.
- **Fix:** In pm_0001_21_not_older_than, also send an older stamp (j+1 after j+2) and assert the bitten room still refuses it as kind 2. Optionally add a bites-build f19 that asserts not_newer_count 0 and an excess above the clean run's at 2 s.

### TE-U4F-4 (Low)

- **Where:** spikes/netcode-proof/crates/client/tests/carry80.rs:64-82 (f24)
- **What:** f24 does not pin U4F-Q16's bound (ACK_RING_TICKS = 32). Its cases are newest+40 (dropped) and newest+20 (taken), which pass for any bound from 20 to 39.
- **Proof:** Read: the only offsets are +40 and +20.
- **Fix:** Add cases at +32 (taken, sentinel filed) and +33 (dropped, newest unchanged, refused +1).

### TE-U4F-5 (Low)

- **Where:** spikes/netcode-proof/crates/client/tests/trace.rs:365-371 (f4)
- **What:** f4 is counted as a negative case, but it feeds no hostile input. It only asserts that an honest P0 endpoint sends 98..=101 pings in 10 s, a total rather than 'never exceeds' in any one second. The brief's other half, '(a faster rate is limited and counted by the pump)', is covered by U4e's existing e9 test (server/tests/pump.rs:193-212), not by anything new. f31 (clock.rs:337-347) pins the gap at every 3rd step, which does cover the per-second rate.
- **Proof:** Read of f4's body and of e9.
- **Fix:** In the report's negative count, name e9 as f4's negative half, or make f4 assert the count in each 1 s window.

### TE-U4F-6 (Low)

- **Where:** spikes/netcode-proof/crates/lab/src/scenario.rs:1899; room/src/room.rs:601
- **What:** The run header's lead_bound_ms records the constant `Room::LEAD_BOUND_MS`, not the bound the room was built with. `Room::with_lead_bound_ms` exists, so a run built with another bound would still record 400. No lab path sets another bound today.
- **Proof:** Read: p.insert("lead_bound_ms", Room::<Sim>::LEAD_BOUND_MS).
- **Fix:** Record `room.lead_bound().0` from the live room.

### TE-U4F-7 (Low)

- **Where:** spikes/netcode-proof/crates/lab/src/scenario.rs:4186-4197
- **What:** f2_the_sims_check_bites_are_routed_to_the_world_alone loops over `Sim::CHECK_BITES`, so it still passes if an id is dropped from that list. Nothing else pins CHECK_BITES' two ids (sim/tests pins World::BITES only). A dropped id would first show as a usage error in a judged bite run.
- **Proof:** Describe the plant: delete one id from World::CHECK_BITES (world.rs:951). f2's test still passes.
- **Fix:** Loop over the two literal ids (PM-0001-21/no-cooldown, PM-0001-21/refuse-not-clamp).

### TE-U4F-8 (Low)

- **Where:** spikes/netcode-proof/crates/client/tests/trace.rs:365-474, :476-555
- **What:** Several seeded traces do not name their seed in their doc, against 2.10's generator rule (TE3-11, u4f-brief.md:661-664): f4 (seed 4), f5 (5), f32 (ii) (32), f32 (iii) (33), f36 (i) (36), and the three bites and f30 (47, 48, 20, 30).
- **Proof:** Read of the doc comments against the code.
- **Fix:** Add one doc phrase to each naming its seed.

### Its own lines

- U4f's share of 2.10's minimums: measured tests +78 against its share of 73 (119 less U4f2's 46: f15 x6, f16 24, f22 2, f23 1, f33 13). Bites build +95 against 88 (134 less 46). Negative cases 44 against its share of 45 (67 less U4f2's 22: f15 x6, f22 2, f23 1, f33 13). The one short is U4F-Q9 (a)'s flood test (TE-U4F-2); f4's negative half is U4e's e9 (TE-U4F-5).
- U4f's 44 negative cases by row: f3 2, f4 1, f7 1, f8 3, f10 8, f11 2, f12 5, f13 4, f14 3, f15 5, f19 1, f20 1, f21 1, f24 1, f25 1, f26 1, f27 1, f35 1, f39 1, and U4F-Q6 (b)'s Pong field 1 (server/tests/pump.rs:336).
- SERIES.md's 20 new lines, read in full: one new subsection (a heading, two blank lines, the table's two head lines, 15 rows), no existing row touched (-0). 14 rows' Marks cells read 'none (recorded)' or 'none (reported)'. The event.reorder.* row names the existing `reorder` rule, unchanged (S5, ruled). The header.params.lead_bound_ms row quotes u4f-lead-bound.md:22's fenced sentence byte for byte (file sha256 57c588ec..., equal to the freeze). No new name repeats an old row's, and the old -21 rows (:33-34, :144-145, :155-156) keep their words. Every name the new lab tests' runs write has a row: contract (b) is asserted in f28 x5, f18, f20 and f14 (iii). One nit outside the build: advisor-u4f-stop.md's '20 new rows; 19 marked none' counts lines as rows; it is 15 rows, 14 marked none.
- His 13:35 rule has nothing to read: the patch changes no marks.json and no judge file (numstat of its 29 files; spikes/netcode-proof at HEAD equals 5b29296b).
- Bites (NA-11): marks.json's 17 ids for -20, -21, -22 and -47 each appear as a literal in exactly one crate's src, spelled as marks.json spells them. The 14 new unit proofs each check both the clean and the bitten side. LAB_BITES is unchanged. The client and room bite pins are updated.
- Plants: f9, f26 x2, f27 and D14 each fail by name in the runner's logs, and each restored file's sha256 is equal. All ran on earlier trees, not the final one (no finding).
- Nothing built lets -21 be judged before U4f2: c-u4f-runs.mjs's SETS holds 47, 22 and 20 only (:47-51) and its usage refuses any other key (:67); c-u4f-r2.mjs writes only under .gate/.
- S1-S7 and N1: no disagreement (N1 is the netcode seat's).

### What it ran

- bash /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-run.sh /home/user/Shamble-Frontlines/.claude/worktrees/agent-a0825a5a2491eb175 /home/user/Shamble-Frontlines/.gate/u4f/review-test/f5-b1.log cargo test --locked -p npt-client --test trace f5_ -- --nocapture (shared, 14:11:59-14:12:33 UTC, exit 0; diff 54cb6faa..., untracked e8dc3663...)
- Read only: recomputed the tree hashes with u4f-run.sh:67's own commands (equal to end checks A2 and B); sha256sum and `git apply --numstat` of wip/u4f-build.patch; `git diff --quiet 5b29296b HEAD -- spikes/netcode-proof` (exit 0)
- python3 -I on a scratchpad script that lists the test functions the worktree adds or removes against HEAD (96 added, 1 moved)
- greps of the sources, marks.json, SERIES.md, check.mjs, the run scripts and the .gate/u4f logs (end checks, plants, hand runs, time logs)

### What it did not check

Whether the room's check order and logic match ADR 0001 and 2.4 beyond what the tests assert (the netcode and security seats); Rust safety, overflow and allocation (rust seat); the public items (arch seat). I re-ran no plant and no bite: the plants' failures are read from the builders' logs on earlier trees. I ran no full test step, no check.mjs, no lab scenario and no judge call. The f1, f36 and event-cap figures are the builders' and were not re-run. The fast clock's 60 s bite projection (+180) stays arithmetic until 5.3's p9. I read brief sections 5.3, 5.6 and 6 only where my findings needed them; I did not read advisor-u4f-brief.md's L1-L6 or the fold check in full, nor c-u4f-cap.mjs.

## rust (rust-reviewer): APPROVE WITH FINDINGS

**Notes file:** /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-review-rust.md

### RR-U4F-1 (Medium)

- **Where:** spikes/netcode-proof/crates/client/src/lib.rs:751-774 (drop_unencodable; the comment's claim at :697-698; the fallback at :702-705)
- **What:** carry 80 (c)'s fix enforces only one of the Input codec's two pair rules. drop_unencodable drops commands that fail to encode alone, then keeps the newest run of consecutive indices (:770). It never checks the other rule: an older command's tick must be 0 to 255 below its newer neighbour (proto/src/input.rs:158-159, u8::try_from(n.tick.wrapping_sub(c.tick)) gives OutOfRange). So when the stamp jumps more than 255 ticks between two unacknowledged neighbours, the second encode fails too and the step sends an Input with only acknowledgements. The stale commands stay in `unacked`, so up to 7 more steps fail the same way and carry no command (about 0.27 s at 30 Hz). Then one Input of 8 commands arrives, most of them late. The comment says 'the newest consecutive run of the rest is kept (the codec's rule)', which is not what the code does. How it is reached: a fixed-lead endpoint (lab-only: U4r's suites and the fast clock) whose newest tick jumps more than 255 after an outage with lost commands, or the controller after 128 or more steps with no page command while old commands are still unacknowledged. No judged profile reaches it (B1's bursts last 1 s: l1/src/lib.rs:197-200). f25 (client/tests/carry80.rs:82-99) tests only the out-of-range aim case.
- **Proof:** Code read: lib.rs:766-773 checks only `a.wrapping_sub(1) == *b` on indices; proto input.rs:158-159 refuses a tick step outside 0..=255 when encoding; npt-bots' own rewrite gets this right by trial-encoding and popping the oldest (bots/src/lib.rs:478-489). Cheapest test, for the fix pass: a fixed-lead endpoint, one command left unacknowledged, the newest tick moved by +300 (a whole snapshot or full state), one more command, then step. Today the Input decodes with 0 commands; after the fix it should hold the newest command, with the dropped older one counted as left out.
- **Fix:** After the per-command retain, keep the longest newest prefix of `unacked` that encodes whole (at most 8 trial encodes, on a rare path), the way bots/src/lib.rs:478-489 does. Or truncate at the first pair whose u8::try_from(newer.tick.wrapping_sub(older.tick)) fails. Count the rest in `left_out`, correct the comment at :697-698, and add the test above beside f25.

### RR-U4F-2 (Low)

- **Where:** spikes/netcode-proof/crates/client/src/clock.rs:163
- **What:** `i64::try_from(offset).unwrap_or(i64::MAX)` flips the sign of an offset below i64::MIN. Example: a Pong with server_ns = u64::MAX and echo = arrival = 0 gives an offset of about -1.8e19, which is stored as +9.2e18. Only a Pong that matches an outstanding (id, echo) pair gets this far, so only a broken or hostile server can cause it. No panic.
- **Proof:** Code read: `offset` is an i128 at :157; try_from fails on both sides and both map to i64::MAX.
- **Fix:** `offset.clamp(i128::from(i64::MIN), i128::from(i64::MAX)) as i64`, plus a unit case with server_ns = u64::MAX.

### RR-U4F-3 (Low)

- **Where:** spikes/netcode-proof/crates/client/src/lib.rs:696 and :702
- **What:** `encode(&Message::Input(input.clone()), ..)` clones the commands Vec on every step, including the normal path, only so the rare error path can reuse `input`. That error path then rebuilds `input.commands` from `unacked` anyway (:701). The result is one more heap allocation per client step, on what will be the Wasm client's hot path.
- **Proof:** Code read: the clone is at :696, before the error is known; :701 overwrites `input.commands`.
- **Fix:** Encode `Message::Input(input)` by value. On error, build a fresh Input { ack_tick, ack_groups, commands } from `unacked` after drop_unencodable.

### RR-U4F-4 (Low)

- **Where:** spikes/netcode-proof/crates/room/tests (new case; the code under test is room/src/input.rs:210-391 and room/src/room.rs:920-1110)
- **What:** No seeded hostile-input property test drives Room::receive with random Inputs: arbitrary ticks around 0 and 2^31, sequences across the u16 wrap, i8::MIN moves, wrapping press counters, aims at the grid's limits, random ack fields, under both options. My walk of every arithmetic site found no panic or overflow. The edge cases are pinned one at a time (commands.rs f8 :143, f10 kind 4 :218, f13 :512-572; input.rs f38 :569; room.rs f8's 10,000 random triples). Nothing keeps the no-panic claim true as the checks change.
- **Proof:** Grep of room/tests and server/tests: no random-Input test through Room::receive. The only seeded random tests are f12's honest trace (commands.rs:423) and server/tests/schedule.rs's wakes.
- **Fix:** Add a splitmix64-seeded test (as commands.rs:411-418) of about 10,000 hostile datagrams over a few hundred ticks, under options A and B, in the debug test build (npt-room's overflow checks are on there). Assert no panic, exactly one counter per offered command, and the bounds: held at most 8, refusals at most 4,096, kept window at most 16.

### RR-U4F-5 (Low)

- **Where:** lab/src/scenario.rs:1030-1031; bots/src/lib.rs:407, :421, :250-254 with :354; client/src/clock.rs:65, :122-124, :219
- **What:** Hardening nits, none reachable today. (1) FastClock::tick's `refused[i] - self.base[i]` is a u64 subtraction: it underflows (panic in a debug or test build, wrap in a release lab run) if slot 0's counters reset mid-episode. (2) The lag switch's `now_ns - first` and `now_ns - at` assume host time never goes back. (3) ClockSync::ping's `while next <= now_ns` loop takes O(gap / 100 ms) steps and never ends at now_ns == u64::MAX. (4) next_tick_at divides by period_ns, which is 0 for tick_hz above 10^9 (ClockSync::new is pub). (5) toward_centre's doc says 'not for other inputs', but `lane` now passes offsets (x - x0); its i64 arithmetic makes that harmless.
- **Proof:** Code read at the lines given. The lab parser bounds its own options (scenario.rs:2122-2144), and the seeded host offsets are under 1,000 s (:1252), so none of these is reached by any run today.
- **Fix:** Use saturating_sub or i64 differences in the lab and bots; compute the next ping time in closed form; use `period_ns.max(1)` (or clamp tick_hz); reword toward_centre's precondition.

### Its own lines

- Identical results on Wasm and native: the wasm step passed in end checks A2 (w3-end-check-a2.log:198) and B (w3-end-check-b.log:185). Native leg 1 was compared with native leg 2 and with Wasm leg 1, 27,000 ticks each, 0 mismatching, h2 equal. clippy-wasm32 passed on the measured and bites builds, and the deps step found no fma, simd128 or relaxed-simd. U4f's sim change (world.rs:595-612, :947-951, :1318-1322, :1429-1457) is bites-only: in the measured build `refuse_far` is the constant false and `ready` is unchanged, so the fire rule is the same as before. The new npt-client and npt-room code is integer-only, and no result depends on usize width (all len() casts widen), so it gives the same results on both targets by Rust's integer rules. The wasm step runs only npt-sim, so for the client and room this is by reading, not by a run.
- unsafe: none added. A grep of the patch's added lines finds none, and the deps step confirms the workspace still forbids unsafe_code with no new exception. The patch also adds no float outside tests, no HashMap or HashSet, and no clock read.
- Panics or overflow on hostile values in npt-room: none found. Every tick compare goes through `dist` (input.rs:34-36, an i64 from wrapping_sub as i32). Kind 4 widens i8 to i32 before squaring (at most 32,768). Kind 5 does a u8 wrapping_sub into i64. kept_stamps[slot] and run_presses[slot] are the same length as `clients` (slots.max(1)), and `slot` has already passed clients.get_mut. Counters are u64 += 1, and the per-tick press sums are u32 and reset every tick.
- Allocation in the room's per-tick path: nothing new once warmed up. `drained` is taken and put back (room.rs:1066, :1108); ran_early, wraps and rewinds_refused are cleared with their capacity kept; incoming and refusals are swapped (room.rs:1032-1033). kept is at most 16, stamps at most 64, buffered at most 8 (32 in bites builds), refusals at most 4,096. The one heap case is canonical's stable sort (room.rs:444-445), and only above about 102 commands in one tick: each element is 40 B and the standard library's stable sort keeps a 4 KiB stack scratch. That happens only on option A burst ticks, and the room's existing stable sorts (room.rs:1130, :1200, :1387-1388) behave the same way.
- Dependencies: the patch has no Cargo.toml or Cargo.lock among its 29 files, so cargo audit and cargo deny results are unchanged by U4f.
- Reviewed tree: the runner's git status call (.gate/u4f/review-rust/tree.log, 14:17 UTC) gives diff 54cb6faa and untracked e8dc3663. Both equal review-pre.log and both end checks, and the patch's sha256 is 74ae26aa29c229d6.
- S1-S7 and N1 (advisor-u4f-stop.md:198-210): no disagreement from this seat.
- -21 before U4f2: I found nothing in the Rust that would let -21 be judged early. I did not read the ops/runs scripts (not Rust).

### What it ran

- bash u4f-run.sh <worktree> /home/user/Shamble-Frontlines/.gate/u4f/review-rust/tree.log git status --short (shared lock, no cargo; exit 0; its header logs the tree hashes: diff 54cb6faa..., untracked e8dc3663..., the same as review-pre.log and end checks A2 and B)
- Read-only checks outside the runner: sha256sum of wip/u4f-build.patch; git apply --numstat on it (29 files, no manifest or lockfile); git diff HEAD of the reviewed files into the scratchpad for reading; greps of the patch's added lines for unsafe, floats, HashMap, clocks and unwraps; greps of the end-check logs A2 and B for their step lines; df (5.0 GiB free)

### What it did not check

What I did not run: any cargo build, test, clippy, bench or Wasm build. The fmt, clippy, doc, test and wasm figures come from end checks A2 and B. No benchmark exists for Room::tick or the client step before and after U4f (the spike has no cargo bench or criterion), so the speed lines come from reading. The test step's 692.1 s and 685.6 s are the only timing. Wasm size: the npt-client wasm32 build is linted and documented, but the wasm step makes no release build of it, so I have no byte count. ClockSync's i128 division will pull compiler-builtins' 128-bit division routines into a wasm32 client (not measured, likely small). I read the test files only for their hostile-value cases, not for counts or assertion strength (the test seat's job). I did not read the ops/runs/*.mjs scripts or SERIES.md (not Rust). I checked the lab's logic (episodes, tallies, -47's read instant) only for overflow and panics, not for correctness (the netcode and test seats' job). N1 (hand-s22's refused rewinds) belongs to the netcode seat.

## arch (engine-architect): APPROVE WITH FINDINGS

**Notes file:** /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-review-arch.md

### EA-U4F-1 (Low)

- **Where:** client/src/lib.rs:490 (join_tick), :516 (not_due); client/src/clock.rs:63 (new), :85 (period_ns), :90 (ping_every_ns), :105 (unknown_pongs), :215 (next_tick_at), :225 (one_way_hi_ns); room/src/input.rs:179 (KEPT_WINDOW); room/src/room.rs:637 (refusals_dropped); bots/src/lib.rs:340 (fast_left_out). Paths under spikes/netcode-proof/crates/ in the worktree.
- **What:** Public items that nothing outside their own crate uses: no other crate, no test, no plan line, no ruling. A5's rule (advisor-u4e-r1.md:158-167) says these become pub(crate). `join_tick` also contradicts 2.13 rev 3.2 ("the endpoint keeps its JoinAccepted tick, no new public item"). `ping_every_ns` is used nowhere, not even inside the crate. Two of them are counts no log writes, so their "0 expected" claims are never shown: `fast_left_out` (the fast clock's copies dropped from an Input that would not encode, which changes what the room sees of the cheat) and `refusals_dropped` (refusal records past 4,096 a tick, which U4f2's attribution will read). Built ahead for U4f2, with no consumer and no test in U4f: `ClientEndpoint::skip_command` (:565) and `twin_left_out` (:527), and `Behaviour::forger` (:307) and `take_forged` (:319).
- **Proof:** Grep each name over spikes/netcode-proof/crates: `.not_due(`, `.join_tick(`, `ping_every_ns`, `next_tick_at`, `one_way_hi_ns`, `period_ns`, `KEPT_WINDOW`, `refusals_dropped`, `fast_left_out`, `skip_command`, `twin_left_out`. None has a hit outside the crate that defines it. u4f-build.md:858 says skip_command is "unused by the lab".
- **Fix:** Make the first list pub(crate) and remove `ping_every_ns`. Either log `fast_left_out` in the fast-clock episode record (a new SERIES.md row) or make it pub(crate). In the report, name skip_command, twin_left_out, forger and take_forged as built ahead for U4f2 (the stop ruling already does this for the forgers), and hand skip_command's missing test to U4f2's test seat.

### EA-U4F-2 (Low)

- **Where:** /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-build.md:283-295 (builder 1), :569-581 (builder 2); builder 3's block :692-866 has no list
- **What:** The public-item list A19 requires is incomplete and partly stale. Builder 3's four npt-bots items (fast_clock, fast_left_out, forger, take_forged) are in no list. Builder 1's list still names `Behaviour::brawl_presses` (:293), which builder 2 replaced with `with_press_every` (:571-572). ClockSync's nine pub methods and ClockSample's five pub fields appear only as "ClockSync, ClockSample". World::CHECK_BITES (bites builds only) and the unfeatured ClientEndpoint::with_quant need their reasons stated; I accept both. §4.1's rows have not been brought up to date (U4e's T6 step at the merge).
- **Proof:** Compare the diff's added `pub` lines (git diff HEAD -U0, grep `^\+.*\bpub\b`) with the two lists.
- **Fix:** Re-derive the report's list from the merged tree, each item with its reason or "built ahead for U4f2". The merge commit updates §4.1's npt-room, npt-client, npt-bots, npt-sim and npt-proto rows.

### EA-U4F-3 (Low)

- **Where:** sim/src/world.rs:1429-1438 and :1452-1457; u4f-build.md:42 and :239 ("bites builds only")
- **What:** The sites of npt-sim's two -21 check bites leave control flow in the authority shot phase that is not behind `#[cfg(feature = "bites")]`: `let mut ready` and `let mut refuse_far = false` (each with an unused_mut allow), the refuse_far refusal block, `let ready = ready && !refuse_far;` and `&& !refuse_far`. 2.13 allows "their sites, bites builds only", and every earlier sim bite site sits behind that gate (`bitten()` itself is bites-only, :988-989; StdSin :1008; Speed :1358). So the World::step source of a normal build now carries a bite's control flow. Behaviour is unchanged (refuse_far is always false outside bites builds, and the wasm step matched 27,000 ticks both ways in both end checks), so this is not a seam change. But the build record's "bites builds only" is not accurate.
- **Proof:** In `git diff HEAD -- crates/sim/src/world.rs` those lines carry no cfg. Grep `bitten(`: every other site is under the gate.
- **Fix:** Keep the refusal decision and its effect inside the gate, for example a bites-only helper that returns (ready, refused_far) plus a `#[cfg(not(feature = "bites"))]` version that returns (ready, false). Or state the lines in the report and let the advisor accept them.

### EA-U4F-4 (Low)

- **Where:** sim/src/greybox.rs:62 (HALF_MAP, private); room/src/room.rs:544 (kind 6's extent from `areas.rects.first()`); bots/src/lib.rs:76-80 (FAR_CM 11,000), :185 (kind 6's 20,000), :934 and :1104 (tests assert against the literal 12_800); room/tests/commands.rs:258, :320
- **What:** The map's extent is defined in three places. npt-sim keeps it private, the room reads it from the greybox, and npt-bots hard-codes it, with its tests checking against the hard-coded copy. If the greybox's extent changes, f18 still passes while the room refuses the clamp script's points as kind 6 (the clamp suite quietly becomes an aim-bounds suite), and the kind-6 forger may land inside the map. NA-8 was exactly this drift.
- **Proof:** Read the sites. f18 (bots/src/lib.rs:925-940) never reads the greybox.
- **Fix:** In lane: f18 and f15's kind-6 case assert against the extent the room reads (World::new(..).greybox().areas().rects[0]; the bots test module already names World). FAR_CM's doc names that test.

### EA-U4F-5 (Low)

- **Where:** lab/src/scenario.rs:47-49 (CONTROLLER_MAX_LEAD = 9, placed between two `use` lines), :1899 (the header's lead_bound_ms); room/src/room.rs:601-605 (with_lead_bound_ms, public)
- **What:** The lab's controller cap of 9 depends, by its own doc, on npt-room's public lead bound (12 ticks less a margin) and option B's buffer (InputBuffer::CMD_CATCHUP). Only a comment ties it to them: no const assert, no test. The header writes `lead_bound_ms` from the constant Room::LEAD_BOUND_MS, not from the bound the room actually ran (`server.room.lead_bound().0`). The two are equal today only because no lab path calls with_lead_bound_ms. A route for U7: the client crate has no way to learn the bound (it does not depend on npt-room and no message carries it), so a real client's cap would be configuration only.
- **Proof:** Read the three sites. with_lead_bound_ms has one caller, a test (room/tests/commands.rs:814).
- **Fix:** Add a const assert or lab test pinning CONTROLLER_MAX_LEAD against CMD_CATCHUP and ticks_for_ms_at(TICK_HZ, LEAD_BOUND_MS). Write the header from the room's lead_bound(), or have the lab check the two are equal. Move the const out of the `use` block. Add a U7 note that the cap needs a source the client can reach (a JoinAccepted field or an npt-sim constant; the advisor's call).

### EA-U4F-6 (Low)

- **Where:** lab/src/logs.rs:194-196 (MOVE_CM_S = 600, "a copy of its private Scene::move_speed"), :701 (tick.v.lagswitch_excess_move_um); lab/src/scenario.rs:1023 (event.fast_clock.excess_move_um); sim/src/scene.rs:139, :184, :244 (pub(crate) move_speed)
- **What:** The two -21 movement checks (lag switch and fast clock) read an excess that the lab computes from a hard-coded copy of npt-sim's private walk speed, and no header records that speed. TUNE-1 made Scene::weapon and Scene::health public for exactly this reason. If the sim's walk got slower, a cheat's extra movement would pass unseen; if it got faster, a correct build would fail. Both scenes use 600 today, so it holds now.
- **Proof:** Read the three sites. No header param names a walk speed (lab/src/scenario.rs:1880-1940).
- **Fix:** In lane: a lab test that walks one soldier at full length through a World and checks MOVE_CM_S per second, which catches drift. The clean fix, a public Scene::move_speed() plus a header param, is an npt-sim item, so it is the advisor's call (carry 26's route).

### EA-U4F-7 (Low)

- **Where:** room/src/input.rs:52 (pub enum Refusal), room/src/room.rs:632 (refusals() -> &[(u8, u16, Refusal)]); sim/src/seam.rs:498-505 (pub struct Refusal { slot, seq, kind: RefusalKind }); plan N/u4-build-plan.md:246-247
- **What:** One concept now has two public types with the same name and opposite meanings across the seam. npt-sim's `Refusal` is the record { slot, seq, kind } in plan §4.2's form. npt-room's new `Refusal` is the kind enum, and its records are tuples, yet their doc cites ":246-247's form". U4f2's attribution (2.4 item 6) must read both lists: kind 7 from the sim, kinds 1-6, 8 and 10 from the room. It would have to alias one import and convert one shape into the other. Nothing goes wrong today: the room's records have no consumer outside room/tests/commands.rs.
- **Proof:** Read the two definitions. Grep `Refusal` in lab/src: only npt_sim::RefusalKind today.
- **Fix:** Cheapest before U4f2 uses it: rename the room's enum for what it is (for example CommandRefusal) and give its record the sim's struct form { slot, seq, kind }.

### EA-U4F-8 (Low)

- **Where:** ops/runs/c-u4f-r2.mjs:2-3 and :22-23 (header), :44 (M21), :55-66 (eleven -21 rows judged with --only PM-0001-2,PM-0001-21), :141-147 (r2.txt's head)
- **What:** The stop ruling (advisor-u4f-stop.md:83-86) says these -21 rows, run from U4f's merge commit, are "an early read of the built suites and not -21's record"; -21's 5.3 record is the whole run from U4f2's merge commit. But the script's header and r2.txt's head say only "U4f's 5.3 pre-runs from commit X", and each -21 row's <key>-verdict.json looks like a 5.3 record. Nothing in the output tells a later reader otherwise. c-u4f-runs.mjs holds no -21 set, so this is the only path by which -21 gets judged before U4f2.
- **Proof:** Read the header and head lines. Grep "early read" in c-u4f-r2.mjs and c-u4f-runs.mjs: no hit.
- **Fix:** Add one line to the header comment and one to r2.txt's head: "-21's rows from a commit before U4f2's merge are an early read, not -21's 5.3 record (advisor-u4f-stop.md:83-86)". No row, run count, mark or judge rule changes.

### EA-U4F-9 (Low)

- **Where:** bots/src/lib.rs:399-494 (Behaviour::outgoing), :559-588 (Behaviour::step); lab/src/scenario.rs:1570-1578 (the lab calls both, in order)
- **What:** Each cheat (the never-acker, clamp twin, lag switch, fast clock, forger) is split across two calls the host must make in order: `step`, then `outgoing` on the endpoint's datagrams. A host that skips `outgoing` runs every cheat as an honest client, with no error and no count. It holds today because the lab is the only host (npt-bots' two binaries are 6-line stubs), but U6's npt-bots and npt-load will host the same Behaviours. Routed to U6.
- **Proof:** Read the two methods. `outgoing` is the only place any cheat changes a datagram.
- **Fix:** Route to U6: one entry point that runs the endpoint's input, the endpoint's step and the cheat's outgoing in order and returns the datagrams, or module text that states the order as the contract.

### Its own lines

- Public items and the seam rule: the seam is unchanged (Command, Rewind, TickInput and SimCore's methods; no seam.rs change). Every new pub item is either in 2.13, named by a builder, or listed in EA-U4F-1 and EA-U4F-2. World::CHECK_BITES (bites only) and ClientEndpoint::with_quant (identity outside bites builds) go beyond 2.13; I accept both with their reasons.
- Crate edges and dependency rules: no Cargo.toml or Cargo.lock change, no new or upward edge. bots -> proto, client -> proto/sim, room -> proto/sim and lab -> everything are all existing edges. The moved bite-homes test sits in npt-lab, the top crate, under cfg(feature = "bites").
- Frozen files: the patch changes no marks.json, judge file, marks/README.md or existing SERIES.md row (one SERIES.md hunk, @@ -371,0 +372,20 @@: 20 lines added, 0 removed). No built part needs a frozen file changed to pass.
- -21 before U4f2: c-u4f-runs.mjs holds no -21 set (SETS 47, 22, 20 only), and the forged rows p1, p7, p14 and p17 are not in c-u4f-r2.mjs. The only way -21 gets judged before U4f2 is c-u4f-r2.mjs's early-read rows, which carry no label saying so (EA-U4F-8).
- S1-S7 and N1 (advisor-u4f-stop.md:198-210): I disagree with none.

### What it ran

- No runner call and no cargo command: every finding is proven by a read or a grep.
- In the worktree: git rev-parse HEAD (07289a4), git status --short, git diff HEAD --stat, and git diff HEAD over client, room, sim, proto, server, bots, lab, marks/SERIES.md and docs/design/netcode-proof-messages.md.
- Greps of every new pub item over spikes/netcode-proof/crates and ops/runs to find who uses it across crate edges.
- df -h /home/user at the end: 5.0G free.

### What it did not check

These belong to the other seats: the clock and controller arithmetic against ADR 0001 and plan §4.8, the room's per-tick order, R2 (g)'s lead-bound anchor, the cap and carry 76's Kick path, and N1 (netcode); the hostile-input kinds, the clamp points and S1 (security); test counts, the bites against marks.json and the read of SERIES.md's 20 new lines (test); panics, overflow, allocation and Wasm identity (rust). I ran no test, bite, clippy or build, and did not prove that the pub(crate) changes in EA-U4F-1 compile. I read the lab's 1,301 added scenario.rs lines only for how they use the crates' public items, not line by line. Hand-off to the rust seat: the client's encode now clones the Input each step (client/src/lib.rs:696, :702), one extra allocation per step per client.
