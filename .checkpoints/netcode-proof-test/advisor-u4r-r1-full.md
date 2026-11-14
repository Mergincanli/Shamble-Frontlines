# Advisor: the ruling on U4r review round 1 (workflow wf_549d69bf-7f3), 2026-11-14

Written by script, word for word, from the workflow's journal (advisor-u4r-r1's StructuredOutput result). Its notes: advisor-u4r-r1.md; the findings: u4r-review-r1-full.md. Nothing below is the main session's.

## The fix pass, in order

1. THE COPY AND ITS RULES (the main session sends this with the items below). A fresh builder copy (copy 6), in u4r-copy5-prompt.md's form with its resume part brought up to date. It starts from the reviewed snapshot: wip/u4r.patch, sha256 572c4f69..., applied on 2fbec77. Notes: u4r-build.md, 1,586 lines, plus a new section 'Fix pass, round 1'. Brief section 4's rules hold throughout. Every change is made with the Edit tool. Every run goes through the runner with -x, df first, 4 GiB before any release or wasm32 build. Each plant is restored by Edit, and the file's sha256 is checked equal to its pre-plant value. Counts are deltas from 526/603/76, and each new test is named with its file. A check.mjs test-step failure is recorded whole under TE2-5 and never retried. The Lows below ride this push, which the Mediums need anyway (A4: a Low never triggers a push). At the end the main session cuts a new snapshot by script, and round 2 reviews its diff against 572c4f69.
2. FP-0 (the main session, before copy 6 starts; INF-U4R-4; this ruling approves the runner edit). In .checkpoints/netcode-proof-test/u4r-run.sh (:63, :68) and main-run.sh (:38, :43), replace the df -BG read with `avail=$(df -B1 --output=avail /home/user | tail -1 | tr -dc 0-9)`. Stop on `[ "$avail" -lt $((4*1024*1024*1024)) ]`, and log `free $((avail/1048576)) MiB` where the line said `free ${avail} GB`. Use the Edit tool only. Keep each change as a .diff beside its runner, in INF2-4's form. Run `bash -n` on each runner; it must print nothing. Copy 6's first runner log must show the MiB line. No other runner line changes.
3. FP-1 (NC-U4R-1 = SEC-U4R-1, with EA-U4R-7's owner half). In scenario.rs Server::io, the end-of-session branch (:488-497): end a session when `x.decoder.is_closed()` or `x.link.state() == LinkState::Closed`, after the pump has drained what had arrived. Take its id and slot and set `self.sessions[i] = None`. If the slot is Some and `self.owner.get(&slot) == Some(&id)`, call `self.room.leave(slot)` and `self.owner.remove(&slot)`. Then push `(slot, "closed")`. The module text (:21-25) adds that a closed decoder or link ends the session and frees the room's slot, as Room::leave's doc ('a closed link') has it. TEST: one new test beside r8 and r9 (Loopback, lab_room), with two cases. (a) A joined session sends one control-stream frame that `Quant::default().decode(Direction::ClientToRoom, Channel::Control, ..)` refuses with an error other than WrongChannel or WrongDirection; assert that refusal first. Then call server.io and assert: `server.room.client(slot)` is None, `server.owner` holds no entry for the slot, and `server.ended` holds (slot, "closed"). Then a new endpoint joins with that slot's token and bind: assert it is admitted (owner holds the new session id) and that `ended` gains no (slot, "replaced"). (b) A joined session whose client end is dropped, then server.io: the same three asserts. PLANT: delete the `self.room.leave(slot)` call; both cases must fail by name. Restore by Edit; sha256 checked.
4. FP-2 (T-1; with SEC-U4R-4 = EA-U4R-6). In r8 (scenario.rs:1943-1979): (a) replace the room_counts compare with the whole counters struct. Before the five Inputs: `let before = server.room.client(0).map(|c| c.counters().clone());`. After them: `assert_eq!(server.room.client(0).map(|c| c.counters().clone()), before, "the first's Inputs reach no room call")`. Nothing ticks inside the loop. Remove room_counts if nothing else uses it. (b) Right after the second join, drain c1 and assert that exactly one Control-channel message is there and that it decodes (Direction::RoomToClient, Channel::Control, Quant::default()) to Message::Kick(Kick::Replaced). The module text (:21-23) adds: 'the Kick(replaced) is sent once; on L1 the old link is dropped in the same instant, so a lost first transmission is not resent (best effort)'. TWO PLANTS, each restored by Edit with its sha256 checked. (i) In drop_slot, keep `self.ended.push((slot, why))` but skip `*s = None`: r8 must fail at 'the first's Inputs reach no room call'. (ii) Delete the Kick(replaced) send (:409-414): r8 must fail at the Kick assert.
5. FP-3 (RR3-M1 = SEC-U4R-2; with SEC-U4R-3, RR3-L1 and RR3-L3). (a) In hash.rs manifest(), before anything is generated, add these refusals, each naming the entry and the value. A runner entry whose profile is not one of P0, P1, P2, P3, P4 or B1. A runner entry unless `warmup_s.checked_add(measured_s).and_then(|s| s.checked_mul(u64::from(npt_sim::TICK_HZ))) == Some(u64::from(ticks))`. A runner entry whose span_ms is not scenario's ROOM_HISTORY_MS (2,000). Any entry whose bytes is above 67,108,864 (64 MiB: four times today's largest entry, 16,950,444 B); a larger recording comes to the advisor. (b) In scenario.rs record(), build argv as a Vec<String> of separate items ("--run", "regenerate", ..., "--profile", profile, "--seed", seed, ...). Never use format! followed by split_whitespace. Its doc says the argv holds only these flags, so ruling 1 point 9 (a)'s refusal of --lead and --every-step holds on this path by construction. (c) In scenario.rs parse(): use checked_mul for the three `* hz` products (warm-up, measured, after-max) and checked_add for warm-up + measured and for + after-max. When bursts > 0, compute the last burst's tick with checked operations before burst_starts is called: warmup_ticks + (burst_first_s + (bursts - 1) x burst_every_s) x hz. Each overflow is a usage error naming its option. TESTS: hash.rs unit cases beside carry 82's: profile "P2 --record x" refused, naming the entry; ticks 8,999 with 10 + 290 refused; span_ms 1,000 refused; bytes 67,108,865 refused; the three committed manifests (recordings.json, det-recordings.json, precheck-recordings.json) still read. One parse case: --measured-s 1000000000000000000 refused as a usage error naming --measured-s. PLANT: remove the profile check; the "P2 --record x" case must fail by name. Restore by Edit; sha256 checked.
6. FP-4 (NC-U4R-3, with T-6's verdict half). In precheck.rs verdict (:486-504), add two VOID causes. First, a leg's refused_logged above 0, beside rewinds_refused: '<leg> refused Rewinds logged N'. Second, a non-empty `ended` on a leg: '<leg> sessions ended on the leg [..]: their counters cannot be read'. A client whose session ends on a leg can then never read as 0 late. Update verdict's doc. TEST: one unit test of verdict, with LegOutcome::default() doctored once per cause (late, rewinds_refused, refused_logged alone, off_stamp, rtt_void, ended); each gives VOID and names its cause. Also HashCompare::Empty gives VOID, Equal gives PASS and Mismatch gives FAIL. PLANT: remove the `ended` cause; its case must fail by name. Restore by Edit; sha256 checked.
7. FP-5 (the run scripts: INF-U4R-2 and INF-U4R-3, with INF-U4R-8, INF-U4R-9, INF-U4R-11, NC-U4R-6, SEC-U4R-7 and T-7), in ops/runs/c-u4r-runs.mjs and c-u4r-r2.mjs. (a) ORDER, in both scripts: the usage check and the git checks; refuse an untracked file under spikes/netcode-proof/crates (`git status --porcelain --untracked-files=all -- crates`); then (d)'s refusal (c-u4r-runs.mjs only); then (h)'s name checks; then (b)'s up-front projection. Next, each release build, with floor() (at least 4 GiB free: section 4 rule 6) before each one. Then (b)'s projection again. Only then create any set folder (logs, results, out). Binaries go to a folder that no exists-check reads; in c-u4r-r2.mjs, move bin out of `out`, for example to GATE/bin/r2-<commit12>. A stop before the first run must leave nothing that blocks a re-run at that commit. (b) PROJECTION: a constant table citing u4r-build.md:739-748. Per 10-minute run listing PM-0001-57: P0 509.1 MB, P2 219.8 MB, P3 125.8 MB, B1 190.6 MB. Any other run: 56 MB. Scale each by the run's measured seconds over 600. For the precheck set, use the recording's manifest bytes plus the pairs' output, as measured in your own earlier pre-check runs (cite the log). For c-u4r-r2.mjs, use your own 13-suite figure (about 145 MB) plus the new suite at the p0 suite's size. Refuse to start unless free minus the projected total is at least 4 GiB, and print both figures. (c) PER RUN: the floor before each run becomes 'free minus that run's projected bytes is at least 4 GiB'. (d) c-u4r-runs.mjs: the header says it runs in the main folder with CARGO_TARGET_DIR unset (plan §7.1 item 6), not through main-run.sh, and that the main session's notes say whether it held the lane lock (flock -x on .gate/shared-target/.lane.lock) for the set. It refuses a set CARGO_TARGET_DIR: a shared-target run comes to the advisor first (brief section 12 item 8). Its cargo builds get --offline. runs.txt's head gains 'target: the main folder's own cache' and the uptime and load average at start and at end (os.uptime(), os.loadavg()). c-u4r-r2.mjs also gets --offline; it keeps running under the runner (no CARGO_TARGET_DIR refusal). (e) Right after the set folder is created, copy marks/marks.json (plus marks/SERIES.md for c-u4r-r2.mjs) into it and write their sha256s in the runs.txt or r2.txt head. Read the bite list from that copy, and pass the copies to the judge and to contract. Before the judge (c-u4r-runs.mjs) and before writing r2.txt (c-u4r-r2.mjs), re-check `git rev-parse HEAD` and `git status --porcelain --untracked-files=no` against their start values, and stop if either changed. (f) c-u4r-r2.mjs step(): run each command through check.mjs's exported run() with a 600 s limit, so it gets its own process group, killed whole. A stopped command's exit line reads 'stopped at <s> s'. A stop is a result, never a retry (condition 5). (g) In c-u4r-runs.mjs one(), parse /overflow resyncs (\d+)/. When the figure is above 0 on a run listing PM-0001-17 or PM-0001-18, append ' (overflow resyncs: to the advisor before it counts, carry 58)'. For the late-command pair, print 'as expected' only when the whole result line equals 'result: VOID (L1 late_count 1)'. (h) Before any use, refuse the precheck entry's name unless it matches /^[A-Za-z0-9-]+$/, and each bite id's tail unless it matches /^[A-Za-z0-9-]+$/, each by name. (i) T-7: c-u4r-r2.mjs gains a whole-as-lost suite: the bites build, the p0 suite's SS P0 settings, --role bite --bite PM-0001-17/whole-as-lost, --marks PM-0001-17. Its expect text: 'carry 62 (h)'s bite: measured incomplete equal to the client count (48), as in the builder's .gate/u4r/whole-as-lost/ dry check; -17's verdict as it comes until marks.json holds ruling 4's plain check'. EVIDENCE: each script runs main() only when it is the entry module and exports its pure helpers (the projection, the name check, the summary-line flag, the late-command line check). A scratch node:test file in the notes folder, never committed, asserts each helper: a P0 set at today's free space is refused; '../x' is refused; a summary line with 'overflow resyncs 3' on a -17 run is flagged; 'result: VOID (L1 RTT rule: ...)' is not 'as expected'. Its output goes in the report. Run `node --check` on both scripts, and run the new suite's three npt-lab and npt-judge lines by hand on the copy, as section 5 item 5 asks, with their output in the notes.
8. FP-6 (small code Lows riding files the Mediums open). 1. NC-U4R-2: scenario::parse refuses, as a usage error, a --bite whose only instance is late_command ('--bite PM-0001-30/late-command is the -30 pre-check's (npt-lab precheck)'); one parse case. 2. EA-U4R-3 = RR3-L8: in Bites::of, set `schedule: Schedule::with_bite(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP, id).is_some()`. In schedule(), world() and net(), replace the quiet fallback with `.expect("Bites::of checked that this instance plants it")`, as room() and client() already do. Add one test assertion: Bites::of on the pump's wallclock id gives schedule false and pump true. 3. RR3-L4: put cfg(feature = "bites") on logs.rs's whole_as_lost and bitten_at and on the as_lost branch, and on precheck.rs's hold and released, the hold and release branches and late_bite's computation, so the measured binary holds no bite code (plan.md:630-631); cfg_attr allow(dead_code) where a field turns dead in the measured build, as decoder_bite has. Clippy and tests stay green in both builds, with counts unchanged. 4. RR3-L7: in replay.rs, before the loop, apply `u8::try_from(h.clients)` and refuse 0 and anything above 255 as a Failed error, before any .bytes file is read; one unit case with a header of 256 clients. 5. T-4: in the clean r1, assert that fate(a) is Some, that each process's tick record count is above 0 in both legs, and that the client half compared at least one record (count per process, not the sum). 6. T-5: contract.rs short_run() returns (Scratch, Vec<Log>) and scenario.rs r4_run returns (Scratch, Summary); each test holds the guard until its last assert. 7. T-3: in check.test.mjs's real-process test, make the child `/bin/sh -c 'echo child $$; sleep 30 & echo grandchild $!; wait'`, at the same 200 ms limit with the same asserts. 8. SEC-U4R-5: in check.mjs, refuse in every crate any metavariable (`$`) anywhere inside an attribute's text, from `#[` or `#![` to its matching `]` in the mask (no attribute holds one today). Correct the comment at :694-695 and the problem text to match. Add one self-test case with `#[cfg_attr(all(), $a)]`. 9. SEC-U4R-6: at check.mjs:1495, the profile refusal names the line and the matched text: `${path}:${line} has a profile key or table (${JSON.stringify(text)})`. Update the self-test's expected line. 10. INF-U4R-5: in det-u4.mjs, import STEPS from check.mjs and use `const steps = { deps: STEPS.deps, "det-u4": ... }`, so it exits 0 only when both pass. 11. INF-U4R-10, points (1) and (3): run() passes killSignal: "SIGKILL" with detached: true, and its doc's 'SIGTERM' changes to match. wasmStep's default clock becomes monotonic (`() => performance.now()`). Its printed lines stay unchanged, and the fake-clock tests stay as they are. 12. EA-U4R-9: rename server/tests/schedule.rs's pin test to `npt_server_s_four_bites_are_listed_under_their_marks_json_ids`, with a doc line: three are the schedule's and one the pump's (U4R-Q6 (a)).
9. FP-7 (docs only, no behaviour). 1. NC-U4R-7: bots/src/lib.rs:49-52 document TURN_EVERY and RETARGET_EVERY as pure command counts, with no time or speed claim. The report names the need under carry 39: a rate-derived constant from npt-sim, by carry 18's route. 2. EA-U4R-1: precheck.rs:3 reads 'on its own simulated clock (integer ns)'. lib.rs:3-5 says SimClock and the runner live in the npt-lab binary, and that the runner is scenario::run with Server. The report's public-items line reads: 'L1Scenario is built as scenario::run plus Server (scenario.rs:340-545, :702-1158); the pre-check's run_leg is a second loop with 7.2's pacing, sharing Server for step 2'. 3. EA-U4R-4: scenario.rs:59-60 cites npt-sim's private CHEST_CM (sim/src/world.rs:79). The report lists PLACEMENT_Z_BOUND_CM beside INTEREST_RADIUS_CM and ROOM_HISTORY_MS as copies of private npt-sim constants, this one unpinned. 4. EA-U4R-5: bots/src/lib.rs:172-175 adds: 'a host that computes a view must feed the endpoint itself and pass &[]'. 5. EA-U4R-8: scenario.rs:10-11 reads 'each reliable message on the one stream channel that decodes it (`stream_of`; the paragraph below)'. The report states the route and why: no npt-proto pub item reads a framed message's type byte. 6. RR3-L2 (b): at the ADMIT_EVENTS_BOUND check, scenario.rs says carry 75's bound is checked on logged runs only, and not on a no-log run (--logs 0, or `npt-lab hash`'s regeneration).
10. FP-8 (the end checks, after every edit above; T-2 = INF-U4R-1, and section 5). 1. Section 5's end checks: check.mjs twice, all 11 steps passing in each run, with the counts as deltas and every new test named. 2. Then, through the runner with -x, df first and the 4 GiB floor, run each of these once: `node ops/runs/det-u4.mjs` (now with deps), and `npt-lab hash --manifest crates/lab/precheck-recordings.json`, as regen-precheck.log ran it. Both PASS lines go in the report, each log labelled with the patch's sha256 at the moment it ran. A SHA-256 mismatch: stop. No re-pin and no retry; bring the log whole to the advisor, with the first differing tick. 3. The report labels the old det-u4.log (01:08-01:11 UTC, before ruling 3's mirror.rs and scenario.rs changes) and regen-precheck.log (2026-11-13 20:27, copy 1) with the tree each ran on. From now on, each builder log is labelled by the patch sha256 at its run (or by the time of the last source edit before it). 4. Both regenerations join section 5's end checks for any later copy.
11. FP-9 (report words, no code; the report must be on disk before round 2). 1. RR3-L5: /tmp/npt-lab-tests holds 124 folders: the 93 that ruling 2 counted, plus 31 from the builder's c3-reg runs at 00:29-00:34 UTC, made before the helper changed. None was deleted (Mustafa's LEFTOVER-DELETE). 2. INF-U4R-7: the wasm step's figures, each labelled: warm, with uptime; after a restart, page cache cold and dependencies warm. Say that M9's cold figure (every release artifact built from nothing) was not measured for U4r's tree, bounded by U4d2's 61.6 s plus U4r's added builds. Give det-u4.mjs's time with its non-cargo sum against 240 s. Use the fix pass's own end-check figures, beside round 1's 39.6 s, 41.9 s and 200.5 s (non-cargo 190.9 s). 3. EA-U4R-2: name the three sites that name npt_sim::World (logs.rs:97, :455, :578; precheck.rs:277) and the two room constructions as a deviation from U4-SEAM (u4-build-plan.md:162-163), with its follow-up. 4. List every follow-up this ruling names, for STATUS, each with its home: NC-U4R-4 (U4g's brief); NC-U4R-5 with EA-U4R-7's logs half (the first logged replacement: U4f or U6); EA-U4R-1's shared loop and EA-U4R-2's constructor (the next npt-lab lane, before the pre-check's re-run after U4h); EA-U4R-4 (the next lane that opens crates/sim); EA-U4R-5 (the U6 brief's bots'-view follow-up); NC-U4R-7's derivation (carry 18's route); RR3-L2 (a) (the next npt-lab lane); RR3-L6 (measured inside ruling 1 point 3 (4)'s judge timing); T-6's contract half (the next unit that edits contract.rs); INF-U4R-6 (U4h's brief); INF-U4R-10 (2) (check.mjs's next change); SEC-U4R-4's U6 half (U6's TickLoop carry). 5. T-7's correction of ruling 4's further finding, in the advisor's words: the main session's npt-judge --only PM-0001-17 check reads the record run's whole-as-lost folder, and the builder's folder is a labelled dry check only.

## Round 2

Round 2 is the second and last round, unless a Critical or High is open after it. Five seats at xhigh, at most three at a time, in two batches that count as one round. Batch 1: netcode-architect, security-auditor, test-engineer. Batch 2: rust-reviewer, infra-engineer.

Subject, only what changed:
- the diff from snapshot 572c4f69 to the fix pass's new snapshot, cut by script as round 1's packet was;
- the plant logs;
- the two check.mjs end checks;
- the det-u4.mjs and precheck-regeneration logs (FP-8);
- FP-0's two runner diffs;
- the builder's report, which must be on disk before round 2 starts (INF-U4R-7, RR3-L5 and EA-U4R-1/-2/-4/-8 are report words).

Each seat confirms its own round-1 findings fixed as ruled, or deferred as ruled, with file:line, and reviews the changed lines. Nothing unchanged is reviewed again.

Who reads what:
- netcode-architect: FP-1's session ending, FP-4's verdict causes, FP-5 (g) and NC-U4R-2's refusal.
- security-auditor: FP-1 (the boundary, both halves), FP-3 (the manifest and argv, the bounds), FP-5 (h), and SEC-U4R-5/-6 in check.mjs.
- test-engineer: every new test and plant in FP-1 to FP-4, T-3, T-4, T-5, FP-5's helper test and its whole-as-lost suite, FP-6's added cases, and FP-8's regeneration evidence and labels.
- rust-reviewer: FP-3, RR3-L4's cfg gating in both builds, RR3-L7, and EA-U4R-3/RR3-L8 with the new expects.
- infra-engineer: FP-0, FP-5 (a) to (f), INF-U4R-5, INF-U4R-10 (1) and (3), and FP-8's times against their budgets.

engine-architect sits out: no public item, crate dependency or seam change is ruled. rust-reviewer reads its EA-U4R-3, and the main session reads its doc and rename items (EA-U4R-1, -4, -5, -8, -9) itself. engine-architect rejoins only if the new snapshot adds a pub item, a dependency or a change to the seam.

The budget: a round-2 finding below High becomes a STATUS follow-up named in the report (a Low never triggers a push). A Critical or High opens round 3 for that finding alone, and one still open after round 3 stops the change-set for Mustafa. Not rounds: the gate after every step, and my review of the pushed head, which always runs.

## For Mustafa

Round 1 found no Critical or High finding. All 10 Mediums are confirmed: two real code defects (a closed connection leaves a ghost player slot in the room; recordings manifest text can inject extra settings), a weak test, an unproven recording re-run, and four disk-safety problems in the run scripts and runners. All are fixed in one pass. Of the 40 Lows, 26 ride the same pass, 9 are fixed in part, and 5 become follow-ups. Nothing in this ruling changes a word or decision of yours, a pass bar or a mark. The only words corrected are mine (ruling 4, about which folder the whole-as-lost check reads). After the fix pass no security finding stays open in U4r. The one exception is a design line carried to U6.

Decisions needed:
- SIZE-U4R. U4r has grown well past its plan, and your SIZE-CHECK rule (2026-11-10: 'stop and tell me in three lines: what grew, why, and the cheaper option') was not applied to it. What grew: about 7,600 lines (1,286 inserted into 13 files, 6,290 in 13 new files) for a unit sized M. U4c and U4j, also sized M, came to about 3,000. Why: the runner (scenario.rs, 2,304 lines with its tests), the -30 pre-check (977), the log writer (786), the contract checker (632), the mirror (490), the replay (429), the run scripts and check.mjs's carries 83-85, plus four build rulings' fixes. The fix pass adds more, mostly tests. Options: (a) the fix pass as ruled: the 10 Mediums plus the small Lows in the same files; (b) cheaper: the Mediums, the one Low tied to a Medium (NC-U4R-3) and the security Lows (SEC-U4R-3 to -7) only, with every other Low a STATUS follow-up. Why it matters: your rule asks for a stop before more growth. My recommendation is (a). The riding Lows sit in files the Mediums already open. Several of them guard the hours-long record runs, which cannot be re-run at the same commit. Deferring them means a later unit re-opens and re-reviews the same files.

For Mustafa:
1. Answer SIZE-U4R with (a) or (b). The main session sends you the three lines and starts the fix pass on your answer. Nothing else is needed now: no merge, no Brave check, no other decision. PR #40 stays a draft.

A heads-up, not a decision: 4.68 GiB is free. The P0 judged set needs about 3.2 GB on top of the 4 GB floor, so the main session will first free space by the approved part-cleans (ruling 1 point 3). If that is not enough, it comes to you under that ruling's point 5 (your LEFTOVER-DELETE route).

## Each finding

### NC-U4R-1 (Medium; fix now)

- **Builder:** FP-1
- **Why:** Confirmed in the read copy. scenario.rs:488-497 drops the session and pushes (slot, 'closed') but never calls room.leave or owner.remove. pump.rs:414-421 closes a joined session on any bad stream message (close_malformed, :364-367). Room::leave (room.rs:689-694) is the only call that frees the slot, and its doc names 'a closed link'. No honest U4r run reaches this path. But U4f's hostile clients and the pre-check's Server run on this glue, and carry 76 calls it a security boundary. So it is a real defect with a small reach today. It is the same defect as SEC-U4R-1, fixed once.

### NC-U4R-2 (Low; fix now)

- **Builder:** FP-6 item 1
- **Why:** Confirmed. Bites::of counts late_command in `any` (scenario.rs:226-236), and only precheck.rs:297 reads it. So `scenario --bite PM-0001-30/late-command` runs clean under a bite id. Brief 2.1 makes an id that no instance of the run plants a usage error. No script passes it today. It rides the parse edit FP-3 makes anyway.

### NC-U4R-3 (Low; fix now)

- **Builder:** FP-4
- **Why:** Confirmed. precheck.rs:455-462 reads late and rewinds_refused once, at the leg's end, with unwrap_or_default, so a slot that is gone reads 0. verdict (:486-504) ignores both refused_logged and `ended`. FP-1 makes a closed session leave the room, which makes the gone-slot case reachable on that path too, so it is fixed in the same pass, with T-6's verdict tests. VOID on an ended session is my reading of 7.2: when a client's counters cannot be read, the setup condition is unshown, and that is VOID, never PASS or FAIL. No current result changes: the existing test asserts that `ended` is empty (:846).

### NC-U4R-4 (Low; follow-up)

- **Where it goes:** U4g's brief, written by the main session: either the lab writes one `tick` counter_wrap record when the room's tick crosses a multiple of 65,536, or the advisor re-reads -58's count against the names actually reported. A STATUS follow-up until then.
- **Why:** Confirmed. The room and the client push only five names (input.rs:86, :104; room.rs:765; client lib.rs:661, :673). -58's check counts at least 6, the server tick's low 16 bits among them. All of -58 is judged after U4g (U4R-Q5), so this is not a U4r defect. Carry 77 (a), one record per name, is met.

### NC-U4R-5 (Low; follow-up)

- **Where it goes:** A STATUS follow-up for the first unit whose runs log a replacement (U4f's hostile clients or U6's reconnect): key `ended` and `gone` by session, or reset the slot in Logs::admit when a new session takes it.
- **Why:** Confirmed: drop_slot pushes (slot, 'replaced'), and run() calls logs.gone for every entry, whatever its reason. No U4r run replaces a session, and today the failure is closed: a later filing counts as a conflict and the log ends crash, or the coverage rule finds a gap, so the judge reads INVALID, never a wrong PASS. Keying by session needs a design for a later join's ticks (carry 62 (d)), and that design belongs to the next unit. This is the same finding as EA-U4R-7's logs half.

### NC-U4R-6 (Low; fix now)

- **Builder:** FP-5 (g)
- **Why:** Confirmed. c-u4r-runs.mjs:181-184 records each summary line and flags nothing. Carry 58 sends any overflow resync on a judged -17 or -18 run to the advisor before the run counts, and ruling 4 point 1 (main-session item 4) expects about 48 on the tick16 run. :164 compares only the result's first word, while precheck's own test asserts the whole line 'result: VOID (L1 late_count 1)' (:932). It rides the script edit FP-5 makes anyway.

### NC-U4R-7 (Low; fix now)

- **Builder:** FP-7 item 1
- **Where it goes:** Deriving the constants from the rate: carry 18's route, named under carry 39 in the report.
- **Why:** Confirmed. bots lib.rs:50 and :52 are command counts, but their docs claim a time ('about 1 s') and a speed. That is the local rate-derived form brief 2.6 (carry 39) bars. Nothing judged reads the cadence, and the bots stay deterministic at any rate. The doc reword rides FP-7. The report names the need under carry 39, so the main session can route an npt-sim item by carry 18 if a time-based cadence is ever wanted.

### T-1 (Medium; fix now)

- **Builder:** FP-2 (a) and plant (i)
- **Why:** Confirmed by reading; not run. room_counts (scenario.rs:1903-1908) leaves out ack_refused and ack_refused_commands. A misrouted Input from the replaced endpoint acknowledges a tick that the fresh AckRing never sent, so receive_input returns at room.rs:783-788 after raising only those two counters. The 'Inputs reach no room call' assert therefore cannot fail for the misroute it names. r8 as a whole still catches that glue plant later, at the Leave assert, so carry 76's evidence holds overall: Medium, not High. The fix is sound: ClientCounters derives PartialEq and Eq (room.rs:63-64), pump_report changes no counter (:699-708), and nothing ticks inside the loop.

### T-2 (Medium; fix now)

- **Builder:** FP-8
- **Why:** Confirmed. det-u4.log ran 01:08:04-01:11:41 UTC. Ruling 3's 8 (A) then changed mirror.rs keep() and scenario.rs's call to it at 01:44-01:49 (u4r-build.md:1215-1222). That is on the bots' view path, which feeds the recorded inputs. SS-P0-s4 was last regenerated in regen-precheck.log (2026-11-13 20:27, copy 1), and c5-end1/c5-end2 regenerate only recordings.json's three synthetic recordings. So no run at this head shows the runner recordings byte for byte. Both paths fail closed at the merge, so Medium. The fix pass also edits record(), so both regenerations must run on its final tree anyway. Same as INF-U4R-1.

### T-3 (Low; fix now)

- **Builder:** FP-6 item 7
- **Why:** Confirmed (check.test.mjs:2901-2923). A node child must start and spawn its grandchild inside 200 ms, and the builder's logs show 203.7-204.0 ms in total. TE2-5 forbids retrying a self-step failure, so one slow start would stop a merge check. A /bin/sh child keeps the same limit and the same asserts.

### T-4 (Low; fix now)

- **Builder:** FP-6 item 5
- **Why:** Confirmed. fate_difference (scenario.rs:1642-1651) gives None when both fates are None. `compared` (:1653-1677) is summed over room and client and checked once against 200 (:1694), which the room's ticks alone clear. It rides the scenario.rs test edits.

### T-5 (Low; fix now)

- **Builder:** FP-6 item 6
- **Why:** Confirmed. short_run (contract.rs:452-460) and r4_run (scenario.rs:1812-1822) return only their results, so the Scratch guard is dropped before the caller's asserts run. A failing r2 or r4 therefore keeps no logs, which goes against my ruling 2 point 32 ('kept on failure').

### T-6 (Low; fix now)

- **Builder:** FP-4 (the verdict cases)
- **Where it goes:** Contract half: a STATUS follow-up for the next unit that edits crates/lab/src/contract.rs. It adds doctored cases for carry 60's missing header param, a missing class_audit event, a duplicate admit, a log npt-log refuses (exit 1) and a usage error (exit 2).
- **Why:** Confirmed. The pre-check verdict's VOID causes for rewinds_refused, commands off their stamp, the RTT rule and Empty are never exercised, and the contract checker has no negative case for five paths. The verdict half is fixed now with NC-U4R-3 (FP-4), because LegOutcome derives Default (precheck.rs:121), which makes the cases cheap. The contract half adds five doctored cases to a file that no Medium opens, so it waits.

### T-7 (Low; fix now)

- **Builder:** FP-5 (i)
- **Why:** Confirmed. This corrects my ruling 4's further finding. That ruling had the main session judge the builder's .gate/u4r/whole-as-lost/ before -17's judged runs, while ruling 2 point 33 says the builder's logs name 2fbec77 for uncommitted code. No script re-makes that run: c-u4r-r2.mjs's SUITES has no such suite. My words now read: 'runs npt-judge --only PM-0001-17 on the record run's whole-as-lost folder (c-u4r-r2.mjs's suite, run from the merge commit) and on its tick16 folder; the builder's .gate/u4r/whole-as-lost/ is a labelled dry check only'. These words are the advisor's, not Mustafa's.

### SEC-U4R-1 (Medium; fix now)

- **Builder:** FP-1
- **Why:** The same defect as NC-U4R-1, confirmed. The exploit sketch holds: input.rs:127-141 repeats last_run with no bound. Its link-closed half is accepted too: Room::leave's contract names 'a closed link'. Loopback and L1 report Closed only when an end is dropped, and no judged run does that mid-run (run() keeps every client until the stop), so no judged result moves. It is a security finding, so it is fixed in the PR, not deferred.

### SEC-U4R-2 (Medium; fix now)

- **Builder:** FP-3
- **Why:** The same defect as RR3-M1, confirmed. hash.rs:275-280 takes `profile` as any string, and record() (scenario.rs:588-594) formats it into a line and then splits on whitespace. Manifest text can therefore add --record or --bytes, whose files and folders are created before any SHA check, or change the regeneration's settings. That walks around carry 82's own name check. Committed, reviewed text bounds the reach, so Medium. It is a security finding, fixed in the PR.

### SEC-U4R-3 (Low; fix now)

- **Builder:** FP-3 (a) and (c)
- **Why:** Confirmed. Nothing ties warmup_s and measured_s to `ticks` before generation (hash.rs:283-284, :312), and parse's `* hz` is unchecked (scenario.rs:1322-1324), with no release overflow-checks for npt-lab. All three runner entries satisfy (10 + 290) x 30 = 9,000 ticks, so the new relation refuses nothing current. It rides FP-3, in the same function. Same class as RR3-L1 and RR3-L3.

### SEC-U4R-4 (Low; fix now)

- **Builder:** FP-2 (b), plant (ii) and the module-text line
- **Where it goes:** U6 half: U6's TickLoop carry (SEC-U4E-5's U6 half). The old connection's stream is flushed and finished before it is closed. A replaced session's Kick gets one transmission once its link is dropped on L1.
- **Why:** Confirmed. drop_slot sends the Kick and drops the link at once (:409-416). L1's close cancels resends (l1/src/lib.rs:1685-1696). r8 never polls c1. On Loopback the Kick still arrives after the server end drops, because Drop clears only the dropping side, so the new assert is sound there. Carry 76's two tests do not name the Kick's arrival, hence Low. It rides r8's FP-2 edit. Same as EA-U4R-6.

### SEC-U4R-5 (Low; fix now)

- **Builder:** FP-6 item 8
- **Why:** Confirmed in check.mjs:694-698. The rule matches only an attribute whose text starts with `$` (/#\s*!?\s*\[\s*\$/). A `#[cfg_attr(all(), $a)]` carrying `path = ...` passes the scan, as the seat's probe log and its rustc build of that form show. So the class SEC-U4D2-R2-3 (b) raised stays open under my narrow carry 85 (c) wording, and the comment at :694-695 claims more than the rule does. No attribute in the spike holds a `$` today, so the wider rule refuses nothing current. It is a security finding, fixed in the PR.

### SEC-U4R-6 (Low; fix now)

- **Builder:** FP-6 item 9
- **Why:** Confirmed. The refusal at check.mjs:1495 names only the file. Its twins at :1471 and :1488 name their value, and carry 85 asks each refusal to name its source and value. It rides the check.mjs edit FP-6 makes anyway.

### SEC-U4R-7 (Low; fix now)

- **Builder:** FP-5 (h)
- **Why:** Confirmed. c-u4r-runs.mjs:144-148 builds the --record file and the --out folder from precheck-recordings.json's `name` before hash::manifest's name check runs. :192 builds a run folder from the bite id's tail. Both come from committed text, so Low. It rides the script edit FP-5 makes anyway.

### EA-U4R-1 (Low; fix now)

- **Builder:** FP-7 item 2
- **Where it goes:** A STATUS follow-up for the next npt-lab lane, before the -30 pre-check's re-run after U4h (carry 40): one shared loop for RQ8's steps 1-3.
- **Why:** Confirmed. There is no L1Scenario item. precheck.rs:350 runs a plain u64 `now`, while its doc (:3) says SimClock, and lib.rs:3-5 places items where they are not. The doc corrections and the report's public-items line are made now. Sharing one loop would refactor the pre-check's run order, and its evidence would then have to be made again, so that part is kept out of this fix pass.

### EA-U4R-2 (Low; follow-up)

- **Where it goes:** A STATUS follow-up for the same next npt-lab lane as EA-U4R-1, before the pre-check's re-run after U4h: `pub(crate) type Sim = npt_sim::World` and one server constructor used by run and run_leg.
- **Why:** Confirmed. npt_sim::World is named in logs.rs:97, :455 and :578 and in precheck.rs:277, and the room is built in two places, against U4-SEAM (u4-build-plan.md:162-163). Refactoring both construction paths inside a fix pass risks behaviour, and no current result differs. The report names the deviation and its three sites now (FP-9).

### EA-U4R-3 (Low; fix now)

- **Builder:** FP-6 item 2
- **Why:** Confirmed. scenario.rs:214 sets `schedule` from npt_server::BITES, which now also holds the pump's wallclock id, while Schedule::with_bite returns None for that id. schedule(), world() and net() fall back quietly, while room() and client() expect. Same as RR3-L8. It rides the Bites::of edit FP-6 makes anyway.

### EA-U4R-4 (Low; fix now)

- **Builder:** FP-7 item 3
- **Where it goes:** A STATUS follow-up for the next unit whose lane opens crates/sim: make the clamp bound (or the fire-origin height) a pub npt-sim item, read by npt-lab.
- **Why:** Confirmed. PLACEMENT_Z_BOUND_CM (scenario.rs:59-61) copies npt-sim's private CHEST_CM (world.rs:79), with no cite, and s1 pins it only against the literal 3,955. The cite and the report line are added now. A pub npt-sim item needs a lane that opens crates/sim.

### EA-U4R-5 (Low; fix now)

- **Builder:** FP-7 item 4
- **Where it goes:** The U6 brief's bots'-view follow-up (ruling 1 point 7): one route for feeding the endpoint.
- **Why:** Confirmed. The only real host feeds the endpoint itself and passes &[] (scenario.rs:962-968), while the tests use the `received` route (behaviour.rs:92-165). A doc line now; keeping a single route changes npt-bots' public Behaviour, which waits.

### EA-U4R-6 (Low; fix now)

- **Builder:** FP-2 (b) and plant (ii)
- **Where it goes:** Its U6 line: as SEC-U4R-4.
- **Why:** The same as SEC-U4R-4, confirmed: deleting the Kick send at :409-414 keeps today's r8 green.

### EA-U4R-7 (Low; fix now)

- **Builder:** FP-1 (owner half)
- **Where it goes:** The logs half: NC-U4R-5's follow-up.
- **Why:** Confirmed. The owner half (the closed path at :488-495 never removes `owner`) is fixed by FP-1. The logs half (`ended` keyed by slot, so logs.gone runs on a replacement) is NC-U4R-5 and waits for the same reasons.

### EA-U4R-8 (Low; fix now)

- **Builder:** FP-7 item 5
- **Why:** Confirmed. The code never reads Limits::TYPES: the only match is the comment at :11. stream_of tries both stream channels (:133-144), so the module doc contradicts itself. A doc reword, plus the route stated in the report.

### EA-U4R-9 (Low; fix now)

- **Builder:** FP-6 item 12
- **Why:** Confirmed. server/tests/schedule.rs:348's name says three bites, and :350 asserts four. A rename with a doc line; the test count is unchanged.

### RR3-M1 (Medium; fix now)

- **Builder:** FP-3
- **Why:** Confirmed, and it resolves the seat's REQUEST CHANGES. Beyond SEC-U4R-2's half, parse refuses --lead and --every-step only when record.is_some() (:1377), but record() sets record_mem after parse, so ruling 1 point 9 (a)'s refusal does not hold on the regeneration path. Once argv is built from fixed flags with the profile checked by name, no manifest value can carry a flag. The gap then closes by construction, so no new refusal is needed beyond FP-3.

### RR3-L1 (Low; fix now)

- **Builder:** FP-3 (a) and (c)
- **Why:** Confirmed. For a runner entry, carry 82's ticks <= bytes/12 limits nothing (hash.rs:283-284, :312), and `bytes` itself is unbounded, so `ticks` can still reach Vec::with_capacity near u32::MAX: RR-L5's limit is incomplete. The cap at 64 MiB is four times today's largest entry (16,950,444 B; recordings.json's largest is 16,700,849 B). It rides FP-3, in the same function.

### RR3-L2 (Low; fix now)

- **Builder:** FP-7 item 6 (part (b))
- **Where it goes:** Part (a): a STATUS follow-up for the next npt-lab lane. On a no-log run `filed` keeps only what `started` needs.
- **Why:** Confirmed by reading. (a) On a no-log run, `filed` grows `res` for the whole run, about 7 MB for a 9,000-tick regeneration: bounded, so it waits. (b) events_stamped is counted only when logs are written (scenario.rs:928-932), so carry 75's bound goes unchecked on --logs 0 and on record() runs. A doc line now says so.

### RR3-L3 (Low; fix now)

- **Builder:** FP-3 (c)
- **Why:** Confirmed. parse's three `* hz` products, warm-up + measured, + after_max and burst_starts' sums use plain arithmetic on command-line text. Cargo.toml turns release overflow-checks on only for npt-sim and npt-proto. It rides FP-3's parse edit. One checked computation of the last burst's tick before burst_starts bounds burst_starts' own sums.

### RR3-L4 (Low; fix now)

- **Builder:** FP-6 item 3
- **Why:** Confirmed. logs.rs:316 (the as_lost branch) and precheck.rs:193 and :317 (the hold) have no cfg. plan.md:630-631 says the measured builds 'never contain' bites, and code that is unreachable is still contained. npt-server gates its own site (pump.rs:451-452). The change is mechanical, and the counts are unchanged.

### RR3-L5 (Low; fix now)

- **Builder:** FP-9 item 1
- **Why:** Confirmed: I counted 124 folders in /tmp/npt-lab-tests myself. That is the 93 from ruling 2 plus 31 from the builder's c3-reg runs at 00:29-00:34 UTC, made before the helper changed. Report words only; nothing is deleted (LEFTOVER-DELETE).

### RR3-L6 (Low; follow-up)

- **Where it goes:** The main session reports read_log's share of npt-judge's time in ruling 1 point 3 (4)'s measurement on one P0 run's logs. If it matters, the next npt-log change makes it one pass or uses borrowed keys. A STATUS follow-up.
- **Why:** An estimate, not a measurement: the 30-50 % was not run. read_log does parse each line twice (log lib.rs:578-579), but the cost can be measured at no extra run, inside the judge timing that ruling 1 point 3 (4) already requires.

### RR3-L7 (Low; fix now)

- **Builder:** FP-6 item 4
- **Why:** Confirmed. replay.rs:265 sizes vec![None; n] from the u32 header, and the u8 check (:272) runs per slot inside the loop, after slot 0's ticks have already allocated. Three lines and one case.

### RR3-L8 (Low; fix now)

- **Builder:** FP-6 item 2
- **Why:** The same as EA-U4R-3, confirmed: schedule.rs:252-257 lists four ids, while with_bite searches only BITE_IDS' three.

### INF-U4R-1 (Medium; fix now)

- **Builder:** FP-8
- **Why:** The same as T-2, confirmed by times and by code path: the bots' view comes from the mirror (scenario.rs:963-968), and their commands feed the recorded tick_input. The builder logs all carry one label for different code states. A re-pin without a ruling is not allowed: a SHA-256 mismatch comes to me whole.

### INF-U4R-2 (Medium; fix now)

- **Builder:** FP-5 (a)
- **Why:** Confirmed. c-u4r-runs.mjs creates the bin folder (:127), runs three release builds (:128-130) and creates the logs folder (:131), all before the first floor() (:146 or :180). c-u4r-r2.mjs creates out/bin (:111) before its builds and its first floor() (:125). A floor stop therefore leaves a folder that the exists-check (:123 and :109) turns into a refusal of every re-run at that commit. Sessions may not delete it (LEFTOVER-DELETE), so the only way on is a new commit. Rule 6 also asks for 4 GB before every release build.

### INF-U4R-3 (Medium; fix now)

- **Builder:** FP-5 (a) to (c)
- **Why:** Confirmed. floor() checks only that 4 GiB is free now. Ruling 1 point 3 (3) stops a set when the next run's projected logs would take free space below 4 GB, and the main session cannot apply that inside one script call. At today's 4.68 GiB, the P0 set (about 3.2 GB at u4r-build.md:739-748's rates) would stop after run 2, with free space under the floor and the commit blocked.

### INF-U4R-4 (Medium; fix now)

- **Builder:** FP-0 (the main session, not the builder)
- **Why:** Confirmed. df -B1 reads 5,021,802,496 B (4.68 GiB), while df -BG reads '5G'. So the runners' 4 GB floor acts at about 3.01 GiB. The floor is the advisor's W1-TARGET rule (advisor-u4-wave1.md:577), not Mustafa's words, so this is not High. No run so far went below 4 GiB (the logs' '5 GB' were 4.7-4.9 GiB). The runners are outside the patch, so the main session edits them, and this ruling approves that runner edit.

### INF-U4R-5 (Low; fix now)

- **Builder:** FP-6 item 10
- **Why:** Confirmed. det-u4.mjs:30 runs wasmStep without the deps step, so run on its own it could PASS on builds that check.mjs would refuse. Adding deps costs about 4 s (c5-end1 deps 3.8 s) and does not touch the 240 s non-cargo sum.

### INF-U4R-6 (Low; follow-up)

- **Where it goes:** U4h's brief (the unit whose recording joins det-recordings.json): det-u4.mjs prints its own non-cargo sum and the line 'over 240 s: to the advisor (ruling 1 point 1)', without changing wasmStep's default print. A STATUS follow-up.
- **Why:** Confirmed: the sum is added by hand today (190.9 s). But the proposed change to wasmStep's summary would break ruling 1 point 1 (4): 'With the defaults it runs and prints exactly what it does now'. The sum only matters once U4h's recording (carry 87) grows it. The report states today's 190.9 s of 240 s.

### INF-U4R-7 (Low; fix now)

- **Builder:** FP-9 item 2
- **Why:** Confirmed. c5-end2's 41.9 s was measured on a cold machine with warm dependencies (u4r-build.md:1525), and M9's 'cold' means every release artifact built from nothing. Report words only, updated with the fix pass's own end-check figures.

### INF-U4R-8 (Low; fix now)

- **Builder:** FP-5 (d)
- **Why:** Confirmed. c-u4r-runs.mjs's header says nothing about how it is run, and run under main-run.sh it would build in the shared target, which brief section 12 item 8 sends to the advisor first. runs.txt records no target and no load, and cargo gets no --offline. It rides FP-5.

### INF-U4R-9 (Low; fix now)

- **Builder:** FP-5 (e)
- **Why:** Confirmed. The commit is checked once (:115; r2 :104), but marks.json is read live at the end of a set that runs for hours (:176, :196; r2 :130-131). A marks.json commit made during a set (carry 62 (h)'s check, ruling 4) would judge it against a file other than the commit runs.txt names. It rides FP-5.

### INF-U4R-10 (Low; fix now)

- **Builder:** FP-6 item 11 (points 1 and 3)
- **Where it goes:** Point (2): a STATUS follow-up for check.mjs's next change: a group kill after a timed call that did not exit 0, with pid reuse weighed.
- **Why:** Confirmed (check.mjs:1894-1906, :2321). (1) spawnSync's default SIGTERM can hang a call past the deadline if the child handles that signal. (3) Date.now is the wall clock. Both are one-line, safe changes; wasmStep uses now() only for differences. (2), a group kill after every timed call, carries a pid-reuse risk on an empty group and goes beyond carry 84 (b)'s 'stopped by the deadline', so it waits.

### INF-U4R-11 (Low; fix now)

- **Builder:** FP-5 (f)
- **Why:** Confirmed. c-u4r-r2.mjs:93-99 spawns its 39 commands with no timeout under the exclusive lane lock, and u4r-run.sh:53's flock has no -w. A hang would hold every lane until someone kills it by hand: INF-4's case, which carry 84 fixed for the wasm step only. It rides FP-5.
