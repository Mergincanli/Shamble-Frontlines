# The advisor's ruling on U4f's review round 1, 2026-11-19

Seat: a day session's advisor (general agent on main's `.claude/agents/advisor.md`, read whole with `git show
origin/main:`), effort as the session set it. Read only: this file is my one write; no build, no run of the code, no
commit. The round: u4f-review-r1-full.md (sha256 6747175eda4922e9…, checked), on builder 3's worktree (HEAD 07289a4),
equal to wip/u4f-build.patch (sha256 74ae26aa…, checked; `git apply --check` on the lane's head 7a59d7a exits 0, and
07289a4 equals that head in spikes/netcode-proof and docs/design). Under TEST-FIRST rule 2 (history.md:2259-2262) every
ruling here is inside plan rev 3.3, can be undone, changes no pass mark, costs no money and touches nothing on main, so
it goes by this ruling; the main session names the three choices below in the day's message.

## Verdict
**APPROVE WITH CONDITIONS: one fix pass (FP1-FP19 below), then the merge. No round 2 panel by default. Nothing goes to
Mustafa.**
- **The four Mediums are real** (each checked in the code, below) and are fixed in the pass, each with a test that fails
  on the reviewed tree first where the finding says it fails. RR-U4F-1 goes first: NC-U4F-1's fix makes RR-U4F-1's path
  reachable on an honest client (my reading: a reset moves the stamp past 255 ticks above the older unacknowledged
  commands).
- **TE-U4F-1:** restore the brief's measure (late apart from missing). The 3 % is not a bar in his sense; with the
  brief's measure B1 passes 1 % as built (0.34 %), so no bar changes at all.
- **NC-U4F-2:** add `--controller 1` to -22's set. Mine to rule: it restores U4F-Q7 (i) as ruled.
- **EA-U4F-8:** fix in the pass (two label lines). U4f2's brief takes five carries by number (C-U4F2-1 to -4 and
  C-U4F2-SEC-1).
- **Both security findings are fixed in U4f's pass** (SEC-U4F-1's guard, SEC-U4F-2's header); U4f2 carries the parser
  half of SEC-U4F-1. No security finding stays open on U4f.
- **Size:** the pass adds about 250-320 lines to 6,213. It needs nothing from him (his SIZE-U4F (a) covers the one
  review round, and a round is a review plus one fix pass). My guard: a stop at 500 added lines in the pass.
- **Correction to my stop ruling** (the test seat is right; I counted the hunk myself): advisor-u4f-stop.md:220-221's
  "20 new rows; ... 19 rows marked none" reads "20 new lines (a heading, two blank lines, the table's two head lines
  and 15 rows); 14 rows marked none, the reorder row naming the unchanged rule".

## His words this rests on (read in history.md, not retyped)
TEST-FIRST (:2234-2282): rule 2's four tests, and what stays his: "any change to a pass mark, to the judge or to
marks.json; money; anything on main; the stops and run days". LEAD-BOUND (:6791-6806): no marks.json text, judge file
or existing SERIES.md row changes. SIZE-U4F (a) (:6896-6903): U4f closes with what is built; U4f2 is next with its own
size line and review. His 13:35 rule (:2125, :2127): a new bar, or a change to one, comes to him first; a reviewer's
read of every marks.json or judge change. The one-round choice of 2026-11-17 (:6818-6819, TEST-FIRST rule 3).

## Question 2: TE-U4F-1, f5's B1 budget
- **Checked:** trace.rs:385 computes `1 - on_time/ticks`, which counts a tick with no command arrived (missing) as late;
  :391 sets 3 % for B1, 1 % for the rest; the doc (:374-375) says 1 %. The brief's f5 (u4f-brief.md:688) bars "at most
  1 % of commands arriving after their stamped tick", on every profile. B1 on the final tree: not on time 0.01879,
  missing 23 (.gate/u4f/review-test/f5-b1.log:16; b3-client.log:80): about 28 of 1,490 ticks not on time, 23 of them
  missing, so about 5 late (0.34 %).
- **Ruling: restore the brief's measure** (FP3). Late (arrived after its tick) is counted apart from missing (never
  arrived); late is held at 1 % on all six profiles; missing is printed beside it. The combined share stays asserted at
  1 % on the five profiles with no burst, as built, so no evidence is dropped. B1's missing ticks are the profile's 1 s
  of total loss; no bar reads them (B1 has no -20 part; -20's two shares are P3's, read by f36 and the judged runs).
- **Is the 3 % a bar in his sense? No.** His bars are the pass marks: marks.json's checks and thresholds, the judge, and
  a value that sets how loose a judged check is (5.6 B1-B9; B3, the lead bound's allowance, is the one of that kind).
  f5's 1 % is a unit test's limit in U4f's brief, taken from carry 74 (u4s-carries.md:345-355), and carry 74's 1 % is an
  advisor rule from U4e's round-1 ruling ("comes to the advisor before its marks count"), not a pass mark. No marks.json
  check reads f5. So a change to f5's limit would be mine under rule 2, not his. It is not taken: the brief's measure
  passes as built, so the 3 % buys nothing and is dropped.

## Question 3: NC-U4F-2, -22's set without the controller
- **Checked:** c-u4f-runs.mjs:49's SETS[22].args has no `--controller 1`; scenario.rs:2274 forces it for `--suite
  clock` only; with_lead's own doc (client/src/lib.rs:345-347) says the fixed lead is kept "for U4r's suites and the -30
  pre-check (U4F-Q7 (i)); U4f's controller is the honest client's". So -22's set departs from a ruling.
- **Ruling: mine, inside the plan.** Adding it restores U4F-Q7 (i) as ruled. It changes no pass mark, check, judge
  file, marks.json text, SERIES.md row or run count. It makes -22 run at the controller under which B3 measured the cap
  of 1,600, costs nothing, touches nothing on main, and can be undone. One more thing the seat did not name: the
  never-ack short run that f21, f14 (iii) and f28's -22 half read (scenario.rs:3784-3791) also runs without the
  controller, so it changes with the set, and -22's two command lines are run by hand again (FP6).

## Question 4: EA-U4F-8, and U4f2's carries
- **EA-U4F-8: fix in the pass** (FP8), though it is Low. It is the only path by which a -21 verdict exists before
  U4f2, and its files look like 5.3 records. Two label lines fix it, and the pass happens anyway. The seat's cite
  should read advisor-u4f-stop.md:90-94 (the early read), not :83-86 (U4f2's list).
- **Into U4f2's brief, by number** (beside advisor-u4f-stop.md:80-89's list, which stands):
  - **C-U4F2-1 (SEC-U4F-1's rest):** `--forge-every`'s parser refuses a value below 2 (a usage error). f16, or a
    room-level case in its shape, shows that `forger(2, ·)` and `forger(5, ·)` at the judged interval, on the steady
    script at P0 into a `Room<World>`, leave no forged sequence without a refusal record of its own kind.
  - **C-U4F2-SEC-1 (the security seat's, as written):** the attribution logs `Room::refusals_dropped()` beside the
    records it reads, so a refused forged item whose record was dropped never reads as unrejected (0 expected at 48
    clients: at most 3,072 records a tick against 4,096).
  - **C-U4F2-2 (EA-U4F-1's built-ahead items):** `ClientEndpoint::skip_command` and `twin_left_out`,
    `Behaviour::forger` and `take_forged`, and `Room::refusals_dropped` are U4f2's inputs, built in U4f with no
    consumer there. `skip_command` gets its own unit test (its index and stamp used, nothing sent), for U4f2's test
    seat. `twin_left_out`'s doc, and the row U4f2 gives it (5.6 B2), name both of its causes: the twin's skip, and
    the codec's pair rule after FP1.
  - **C-U4F2-3 (EA-U4F-7):** before the attribution reads the room's records, npt-room's `Refusal` enum is renamed
    for what it is (`CommandRefusal`), and its records take the sim's struct form `{ slot, seq, kind }`. U4f2's lane
    opens npt-room for this alone. Why there: its only consumer is U4f2's attribution, and f33 proves the form.
  - **C-U4F2-4 (EA-U4F-8's follow-on):** when U4f2 adds the forged rows to c-u4f-r2.mjs, it rewords FP8's line, so
    the run from U4f2's merge commit reads as -21's 5.3 record.
  - Not into U4f2: N-SEC-1 (a forged field inside a rejoin's overlap is counted `rejoin_overlap`). It is dropped either
    way, and no judged forger rejoins: noted here, no action.

## Question 5: Mustafa
**None.** No finding or fix changes a pass mark, the judge or marks.json (`git diff --stat 5b29296b --
marks/marks.json crates/judge marks/README.md` in the reviewed tree: empty, my run). No finding or fix costs money or
touches main. No run day is involved. The size is covered by SIZE-U4F (a) (below). The pass's SERIES.md edits are
cells and a title in U4f's own new subsection (FP7, FP9, FP12), not an existing row. The lead-bound row's meaning cell
is untouched (advisor-u4f-build1.md line 8). **Name in the day's message (rule 2):**
1. -22's judged set runs U4f's controller, as every other U4f suite (NC-U4F-2).
2. f5 is measured as the brief writes it, with late counted apart from missing; the unruled 3 % is dropped
   (TE-U4F-1). No bar changes.
3. No second review round on the fix pass (the 2026-11-17 choice); my check of the pass instead.

## Per-finding table (severity on CLAUDE.md's scale; each seat's label kept)
| Id | Severity | Ruling | Why |
|---|---|---|---|
| NC-U4F-1 | Medium | **fix: FP2** | checked: the target rises at most 2 a step from the old target (lib.rs:652-657), so with every due command given `owed` stays at most 2-4 and :668's skip never runs; after a reset or stall the stamps run late for about J steps, against 2.2 rev 3.2's rule |
| NC-U4F-2 | Low | **fix: FP6** (mine, rule 2) | restores U4F-Q7 (i) as ruled; no mark, check, row or count changes |
| NC-U4F-3 | Low | the three docs ride (FP10); the stamp cap **carry C-U6-17** | no U4f run meets it (L1's jitter is bounded); the first non-L1 honest client is U6's, then U8's L2 parts |
| NC-U4F-4 | Low | **carry C-U7-3** | on L1 the send delay is 0; the Worker host is U7's |
| NC-U4F-5 | Low | -20's set report and the OQ-0001-6 reading (the main session's, after the merge) | no code; the reading names B's depth as the controller's padded target with its three terms |
| SEC-U4F-1 | Low | **fix: FP5**; its parser half **C-U4F2-1** | a three-line guard in a file the pass opens (FP5 opens it); the parser and the room-level proof are its consumer's |
| SEC-U4F-2 (= TE-U4F-6 = EA-U4F-5's header half) | Low | **fix: FP7** | checked: scenario.rs:1899 writes the constant, not the room's bound; one line in an open file |
| TE-U4F-1 | Medium | **fix: FP3** | the test bars another number than the brief's, under an unruled 3 % (Question 2) |
| TE-U4F-2 | Medium | **fix: FP4** | checked: no test reads the kept window's length (KEPT_WINDOW only at input.rs:179, :229); a ruled negative case missing; 44 of U4f's 45 |
| TE-U4F-3 | Low | rides FP15 | commands.rs opens for FP4; the plant shows the proof tells the two bites apart; the optional f19 bites test is not taken (p9 shows it) |
| TE-U4F-4 | Low | rides FP16 | carry80.rs opens for FP1; two cases pin U4F-Q16's 32 (lib.rs:817: `> 32` is dropped) |
| TE-U4F-5 | Low | rides FP18 (a doc phrase); the report names e9 | the brief's f4 row names the pump's half; e9 (U4e's) is it, and f31 pins the per-step gap; no new code |
| TE-U4F-6 | Low | = SEC-U4F-2: FP7 | one finding |
| TE-U4F-7 | Low | rides FP17 | scenario.rs opens; the literal ids pin both |
| TE-U4F-8 | Low | rides FP18 | trace.rs opens; 2.10's generator rule (TE3-11) asks the seed in the doc |
| RR-U4F-1 | Medium | **fix: FP1** | checked: drop_unencodable (lib.rs:751-774) keeps consecutive indices only; the codec also refuses a tick step outside 0..=255 (proto input.rs:158-159); the comment at :697-698 is wrong |
| RR-U4F-2 | Low | **carry C-U7-4** | clock.rs is not opened; only a broken or hostile server reaches it, which is U7's boundary |
| RR-U4F-3 | Low | rides FP1 | the same lines as FP1's fix |
| RR-U4F-4 | Low | **carry C-U5-1** | U5 fuzzes every message type (u4-build-plan.md §6); TE-U4F-2's test pins the kept window now |
| RR-U4F-5 | Low | (1), (2), (5) ride FP19; (3), (4) **carry C-U7-4** | (1) scenario.rs and (2), (5) bots open; (3), (4) are clock.rs's |
| EA-U4F-1 | Low | the open files' items: FP12; clock.rs's six **C-U7-4**; the built-ahead items **C-U4F2-2** | A5's rule (advisor-u4e-r1.md:158-167): no public item without a consumer, without its reason |
| EA-U4F-2 | Low | the close (FP20): the report's list from the merged tree; §4.1's rows in the merge commit | report and plan text, the main session's |
| EA-U4F-3 | Low | **accepted as built**; the report names the lines (FP20) | the measured build's behaviour is unchanged (refuse_far always false; wasm 27,000 ticks, 0 mismatching, in A2 and B); gating needs an npt-sim edit for no behaviour change |
| EA-U4F-4 | Low | rides FP13 | bots opens for FP5; the room's extent read once, so the clamp suite cannot drift into an aim-bounds suite |
| EA-U4F-5 | Low | header half FP7; const assert rides FP11; the cap's source **carry C-U7-3** | scenario.rs opens; `ticks_for_ms_at` is const (sim/src/ticks.rs:25) |
| EA-U4F-6 | Low | rides FP14 (the drift test); the npt-sim route not taken | scenario.rs opens; the test catches drift where the lab uses the copy; no header param needed for a report |
| EA-U4F-7 | Low | **C-U4F2-3** | its consumer is U4f2's attribution; nothing reads the records today |
| EA-U4F-8 | Low | **fix: FP8**; C-U4F2-4 | the one path by which -21 is judged before U4f2 (Question 4) |
| EA-U4F-9 | Low | **carry C-U6-18** | the lab is the only host today; U6's npt-bots and npt-load host the same Behaviours |
| N1 (netcode's line) | note | -22's set report, one line: the refused rewinds are slot 0's, the never-acker's, and why | no -22 check reads rewinds |
| C-OQ6-SEC-1 | note | the OQ-0001-6 reading names it (with NC-U4F-5) | ruled by U4F-Q18 (a); a design property, not a finding |
| N-SEC-1 | note | no action | above |
| The stop ruling's count | Low | corrected above | counted myself |

The new carries for later units, to be written in plan.md's rows by the main session (the next free numbers there):
- **C-U5-1:** an Input fuzz target through `Room::receive` under options A and B (seeded, in the debug test build). It
  asserts no panic, exactly one counter per offered command, and the bounds: held at most 8, refusals at most 4,096
  a tick, kept window at most 16.
- **C-U6-17:** before an honest client runs on a link other than L1, the controller's cap holds under both options.
  The form: the stamp capped at the earliest arrival's next tick + 7 (`next_tick_at(now + lo_rtt / 2) + 7`). Its
  test: one 600 ms pong and a 10 ms path, under option B's buffer of 8, with no kind-10 refusal on the honest client.
- **C-U6-18:** a Behaviour's host order (input, the endpoint's step, `outgoing`) gets one entry point, or module text
  that states the order as the contract, before npt-bots and npt-load host Behaviours.
- **C-U7-3:** the endpoint takes the previous step's send time, and ClockSync stamps each `Ping` with it (NC-U4F-4).
  The controller's cap gets a source the browser client can reach. My lean: a `JoinAccepted` field, since the bound is
  room data (U4F-Q8 (i) (b)); a wire change goes through the catalogue first.
- **C-U7-4:** ClockSync's hardening when it becomes the Worker's:
  - the offset clamped, not mapped to i64::MAX (RR-U4F-2), with a unit case at server_ns = u64::MAX;
  - the next ping time in closed form (RR-U4F-5 (3));
  - `period_ns.max(1)` or a clamped tick rate (RR-U4F-5 (4));
  - its public surface trimmed: `ClockSync::new`, `period_ns`, `unknown_pongs`, `next_tick_at` and `one_way_hi_ns`
    each made crate-private, removed or given a consumer, and `ping_every_ns` removed (EA-U4F-1).

## The fix pass
**Who and where:** one builder (general-purpose, opus, effort xhigh).
- **Its worktree:** a fresh worktree on the lane's head, with wip/u4f-build.patch (74ae26aa…) applied after `git apply
  --check`; its runner hashes before any edit must equal review-pre.log's (diff 54cb6faa…, untracked e8dc3663…).
- **Its runner and notes:** u4f-run.sh; logs under .gate/u4f/w4-*; notes appended to u4f-build.md under "## Fix pass
  (round 1)", each item written when done, with SIZE-CHECK after each.
- **The standing rules** of advisor-u4f-build1.md's lines 2, 5, 15 and 22 (:162-246) apply. In short: never edit
  marks.json, an existing SERIES.md row, marks/README.md or crates/judge; disk before each build; one command per call;
  files written with the Write and Edit tools only.
- **Snapshot** at each check-in with worktree_patch.py.
- **Launch condition (disk):** 5.0 GiB free now (`df -h /home/user`, my read), and the shared target holds 6,730 MiB
  (`du -sm`, my read): about 1 GiB above the runner's floor, against two end checks. My lean by P27 (no lane running
  now): part-clean before the builder starts, as before builders 2 and 3.

**The list, in order** (each item's acceptance check named; "fails first" means one runner call on the reviewed code
before the fix, its log kept):
1. **FP1, RR-U4F-1, with RR-U4F-3 in the same lines:**
   - The fix: after the per-command retain, drop_unencodable keeps the longest newest prefix of `unacked` that encodes
     whole (trial encodes, as bots/src/lib.rs:478-489), and counts the rest in `left_out`. `step()` encodes the
     `Input` by value and rebuilds it only on the error path (no `input.clone()`). The comment at lib.rs:697-698
     states the codec's two rules. `twin_left_out`'s doc names both causes.
   - Acceptance: a new test beside f25 (carry80.rs): a fixed-lead endpoint with one command unacknowledged, its
     newest tick moved by +300 (a snapshot), one more command, then a step. The `Input` decodes with the newest
     command, and the older one is counted left out. It fails first (0 commands). f25 still passes.
2. **FP2, NC-U4F-1:**
   - The fix: in `plan()`, an arm `Some(t) if dist(goal, t) > Limits::REDUNDANCY_CAP as i64 => goal` before the
     2-a-step arm (lib.rs:652-657); the existing skip at :668 then counts the stamps between. The doc of `plan()`,
     and that of `with_clock`, say a goal past the cap (a clock reset or a host stall) moves the target at once.
   - Acceptance in f32 (ii), after its 12 s jump:
     - `stamps_skipped() > 0`;
     - from 2 s plus 10 steps after the jump, `lead_now() >= Some(1)`;
     - every stamp first arriving from then on arrives before its tick runs (P0 there: no loss).
   - A stall case (P0, no input or step for 1 s): skipped stamps above 0, and from the third step after the stall,
     every stamp arrives before its tick.
   - Both cases fail first. f5, f32 (i) and f36 keep `stamps_skipped` 0, unchanged.
3. **FP3, TE-U4F-1:**
   - f5 counts late (arrived after its tick) apart from missing.
   - Acceptance: f5 asserts late at most 1 % on all six profiles, and keeps the combined share at most 1 % on the
     five without a burst. Its line prints late and missing per profile. The 0.03 literal is gone, and the doc says
     what the code checks.
4. **FP4, TE-U4F-2:**
   - An in-crate test in input.rs's `mod tests` (:403) offers 10,000 new sequences across the u16 wrap. It asserts
     `kept.len() <= KEPT_WINDOW` after each, and equality from the 16th on (+1 test in both builds; a negative case,
     45 of U4f's 45).
   - Its plant: input.rs:229-231's `pop_front` removed. The test fails by name in one `-x -k` call; the file is
     restored and its sha256 checked (P26).
   - f12 (iv) pins the edge by sequence. With 1..=20 offered, a changed copy of seq 5 (the oldest kept) is kind 3, and
     one of seq 4 is `seq_outside_window`, never forged, under both options (+0). The seat's "16 and 17 back" counts
     the newest as one back.
5. **FP5, SEC-U4F-1:**
   - The fix: `forger` returns None for `every < 2` (in place of `every.max(1)`, bots/src/lib.rs:311). Its doc states
     the precondition: the previous command was accepted, which every interval of 2 or more gives on a lossless
     profile.
   - Acceptance: a bots case in f15's module, where `forger(k, 1, 12)` is None and `forger(k, 2, 12)` is Some for
     kinds 1, 2, 4, 5 and 6.
6. **FP6, NC-U4F-2:**
   - The fix: `--controller 1` in SETS[22].args (c-u4f-runs.mjs:49) and in its header sentence (:11-13), and in the
     never-ack short run's settings (scenario.rs:3790-3791).
   - Acceptance:
     - f21, f14 (iii) and f28's -22 half pass under it, in both builds' end checks;
     - -22's measured line and its PM-0001-22/unbounded bite line are run by hand on a short setting (section 6
       item 5; `-x -k`, as w2-hand.log's s22 and s22b). Each header has `controller_max_lead_count` 9 and no
       `lead_ticks_count`;
     - s22 shows headroom 0, one overflow resync, then the kick; s22b shows neither.
7. **FP7, SEC-U4F-2 (= TE-U4F-6, EA-U4F-5's header half):**
   - The fix: the header's `lead_bound_ms` is read from the live room's `lead_bound().0` (scenario.rs:1899). The new
     row's "Written by" cell reads "npt-lab (from the room's `lead_bound()`)". Its meaning cell is unchanged.
   - Acceptance: f6 under both options still reads 400, and no `LEAD_BOUND_MS` is left at the header write (grep).
8. **FP8, EA-U4F-8:** one line in c-u4f-r2.mjs's header comment, and one in r2.txt's head (the `lines` array,
   :141-147): "-21's rows (p2-p4, p8, p9, p13, p15, p16, p18-p20) from a commit before U4f2's merge are an early read
   of the built suites, not -21's 5.3 record (advisor-u4f-stop.md:90-94)". Acceptance: `node --check` exits 0, and
   grep finds both lines.
9. **FP9, the stop ruling's riders (advisor-u4f-stop.md:204-206):**
   - S4: USAGE gains `[--lag-hold-ms <ms>] [--lag-every-ms <ms>] (with --suite lag-switch)`.
   - S5: the subsection title names the `event.reorder.*` row as the one that a check (the existing `reorder` rule)
     reads.
   - S6: c-u4f-runs.mjs:7's order reads -47, -20, -22, then -21's (R2).
   - Acceptance: grep, and the end checks' refs and marks steps.
10. **FP10, NC-U4F-3's docs** (scenario.rs:47-49, client/src/lib.rs:354-357, room/src/input.rs:321-323): each states
    the true condition. The cap of 9 keeps every honest command inside option B's 8 only while the up delay is at
    least about 2 ticks whenever the cap binds, which L1's bounded jitter always gives; other links are C-U6-17's.
    Docs only.
11. **FP11, EA-U4F-5's tie:** `CONTROLLER_MAX_LEAD` moves below the `use` lines, with a const assert. The assert ties it
    to at most `CMD_CATCHUP + 1` and below `ticks_for_ms_at(TICK_HZ, Room::<Sim>::LEAD_BOUND_MS)`. Acceptance: both
    builds compile, and clippy passes (end checks).
12. **FP12, EA-U4F-1 in the open files** (each change shown compiling with clippy's warnings as errors, both builds):
    - `ClientEndpoint::join_tick()` is removed: `plan()` reads the field, and 2.13 says "no new public item".
    - `InputBuffer::KEPT_WINDOW` becomes pub(crate).
    - `ClientEndpoint::not_due()` stays public, with a consumer: f36 (ii)'s controller run (bots/src/lib.rs:841)
      asserts it 0, as 2.2's bot rule says.
    - `Behaviour::fast_left_out()` stays public, with a consumer: the lab writes it per episode as
      `event.fast_clock.left_out_count`, in U4f's own `event.fast_clock.*` row, and f19 asserts it 0 at P0. f28's
      fast-clock contract check then covers the name.
13. **FP13, EA-U4F-4:** f18 and f15's kind-6 case assert against the extent the room reads (the greybox's first
    area, as room.rs:544 reads it), not the literal 12,800, and FAR_CM's doc names f18. Acceptance: no 12_800 is
    left in those two asserts (grep), and both pass.
14. **FP14, EA-U4F-6:** a lab unit test (scenario.rs's tests) walks one soldier at full length through a `World` for
    1 s. It asserts the distance equals `crate::logs::MOVE_CM_S` (600 cm), so a change to the sim's walk fails it by
    name (+1).
15. **FP15, TE-U4F-3:**
    - pm_0001_21_not_older_than (commands.rs:958) also sends an older stamp (j + 1 after j + 2). It asserts the
      bitten room still refuses it as kind 2.
    - Its plant: input.rs:272 read as `let newer = true;` under the bite. The proof fails by name in one `-x -k` call;
      the file is restored and its sha256 checked.
16. **FP16, TE-U4F-4:** f24 gains newest + 32 (taken, the sentinel filed) and newest + 33 (dropped, the newest
    unchanged, `refused()` + 1).
17. **FP17, TE-U4F-7:** scenario.rs:4186's test names the two literal ids (PM-0001-21/no-cooldown,
    PM-0001-21/refuse-not-clamp), in place of looping over `Sim::CHECK_BITES`.
18. **FP18, TE-U4F-8 and TE-U4F-5's phrase:**
    - The docs name their seeds: f4, f5, f32 (ii), f32 (iii), f36 (i), the three bites and f30 (4, 5, 32, 33, 36, 47,
      48, 20, 30).
    - f4's doc names e9 (server/tests/pump.rs:193-212) as its negative half.
    - Acceptance: grep.
19. **FP19, RR-U4F-5 (1), (2), (5):**
    - FastClock::tick's counter differences (scenario.rs:1030-1031) use saturating_sub, or i64 differences.
    - So do the lag switch's `now_ns - first` and `now_ns - at` (bots/src/lib.rs:407, :421).
    - toward_centre's doc (:250-256) states that `lane` passes offsets.
    - No new test.

**The main session's, at the close** (FP20; not the builder's):
- the report's public-item list, re-derived from the merged tree, each item with its reason or "built ahead for U4f2"
  (EA-U4F-2), and §4.1's rows in the merge commit (U4e's T6);
- EA-U4F-3's inert lines, named, with "bites builds only" corrected in the record;
- TE-U4F-5's e9, in the negative count;
- this ruling's correction of the stop ruling's count;
- S7's two cites, re-pointed at the next card save.

**After the merge, in the set reports:**
- NC-U4F-5 and C-OQ6-SEC-1, in -20's set report and the OQ-0001-6 reading;
- N1, in -22's.

**Its end checks** (u4f-brief.md section 6, both builds; `-x` and `df` before each), on one tree:
1. check.mjs twice, all 11 steps passing both times:
   - Each run's last line.
   - The test step's two counts, as deltas on 5b29296b's, against U4f's share (advisor-u4f-stop.md:95-97): +73
     measured, +88 bites, 45 negative cases (2.10's minimums less U4f2's 46 tests and 22 negatives: the test seat's
     arithmetic). Each new test has its `--list` line; the negatives are named by row.
   - The self count and the provenance lines.
   - The test step's wall time against 1,207 s.
   - Each new test, once alone, with its time (2.10's time rule).
   - A failing C62H-BOUND short P2 test stops for Mustafa (section 1 item 5); any other failure stops for me.
2. Item 4: `cargo test --locked -p npt-server --all-features`, on the final tree.
3. Item 5: FP6's two hand lines, and `node --check` of both run scripts.
4. Item 6, the gate, is the merge step's.

**Size against his line:**
- U4f stands at 6,213 added lines, past his 6,000 stop. He got the three lines and answered (a): "Close U4f with what
  is built. The three finished cheat tests stay in it and are reviewed with it". The (a) he answered says "one review
  round, then the merge" (advisor-u4f-stop.md:36-37), and CLAUDE.md's round is one review plus one fix pass. The pass
  builds no new part: it fixes the reviewed ones. So its lines need nothing from him.
- My estimate is about 250-320 added (FP1, FP2 and FP4 about 40 each; the riders about 100).
- **My guard:** SIZE-CHECK after each item. Past 500 added lines in the pass (about 6,700 in all), the builder stops at
  that item's end and comes to me: new scope goes to U4f2, or to him if it changes the plan.
- The final count goes in the day's message and U4f's report.

**Round 2: not by default.**
- Why: the 2026-11-17 choice (history.md:6818-6819, "one round; a second only where round 1 leaves a High or a design
  change"). Round 1 left no Critical or High. No fix changes the design: FP2 builds 2.2 rev 3.2's own rule; FP6 restores
  U4F-Q7 (i); FP1 fixes a bug.
- In its place, my check of the pass (not a round: CLAUDE.md never cuts it):
  - the pass's diff whole, against 74ae26aa;
  - each acceptance check's log, and each fails-first and plant log;
  - the end checks' last lines, counts and times;
  - the SERIES.md cells FP7, FP9 and FP12 touch;
  - the final snapshot's sha256 and runner hashes.
- If a fix departs from its seat's named fix or from its acceptance check, round 2 runs on that item alone, with that
  seat: netcode for NC-*, test for TE-*, rust for RR-*, security for SEC-*, arch for EA-*. A third round only for an
  open Critical or High.

**The merge's conditions**, on U4r's M1-M9 form (advisor-u4r-r2-full.md:8-16), with U4f's terms:
1. **M1:** wip/u4f-build.patch holds the pass's final tree, and its sha256 is the one my check names.
2. **M2:** spikes/netcode-proof and docs/design on the lane's head still equal 07289a4's. If main has meanwhile changed
   code, CI or rules, take main in first (MAIN-52, plan first) and re-check.
3. **M3 and M4:** `git apply --check`, then `git apply`. The merged files are cmp-equal to the reviewed ones, and the
   runner's tree hashes match.
4. **M5:** one counted check.mjs run on the merged tree, all 11 steps passing, with the pass's counts.
5. **M6:** a failing test is never retried unnamed: it stops the merge and comes to me.
6. **M7:** `node tools/gate.mjs --no-smoke` (section 6 item 6) passes, and CI's gate and toolchain jobs are green on
   the pushed head.
7. **The D10 read:** `git diff --stat 5b29296b..<merge> -- spikes/netcode-proof/marks/marks.json
   spikes/netcode-proof/crates/judge` is empty. The report names it in its own line ("no marks.json or judge change;
   nothing for his 13:35 read").
8. **DEP-HOLD:** no manifest or lockfile among the files.
9. **M8, one commit** carrying:
   - the files as applied;
   - `git rm` of wip/u4f-build.patch (WIP-CLEAN);
   - plan §4.7's cap text (2.8 item 5);
   - the plan's U4f and U4f2 entries (advisor-u4f-stop.md:98-99);
   - §4.1's rows (FP20);
   - u4f-build.md's merge section;
   - u4f-report.md, on u4r-report's model. The report has:
     - A1's movement reading, in a line of its own, naming who reviewed it (the advisor and the test-engineer);
     - the D10 line;
     - SERIES.md's 15 new rows, and the pass's cell edits;
     - the deviations: S1's no pings, `with_quant`, `CHECK_BITES` and EA-U4F-3's lines;
     - what is built ahead for U4f2;
     - f28's B halves, moved;
     - U4f's share counts;
     - SIZE-CHECK against SIZE-U4F (a);
     - every round-1 finding's route (this table), and the carries by number.
10. **M9:** after the push, the saver restarted, CI green and the card updated. Then 5.3's 21 pre-runs (-21's rows an
    early read, FP8), the part-clean at A6's place, and the sets -47, -20, -22 in R2's order and on its condition.
    The merge into the lane needs no go of his (his (a); as U4r's, history.md:4659-4662). PR #40 keeps its own go.

## What I read, and what I did not
**Read:**
- main's advisor.md, whole;
- u4f-review-r1-full.md, whole, and u4f-panel-brief.md, whole;
- u4f-brief.md: 2.2 and 2.3 (:380-409), 2.10 (:645-729), 2.13 (:782-800), 5.1 (:959-983), 5.3 (:997-1052), 5.6
  (:1127-1161), 6 (:1163-1186) and U4F-Q7 (:1465-1486);
- history.md: :2115-2135, :2234-2282, :4654-4662, :5489-5500, :6780-6919;
- my rulings advisor-u4f-stop.md, advisor-u4f-build1.md and advisor-u4f-fold-check.md, whole; advisor-u4f-brief.md
  :1-60 and :160-300; advisor-u4e-r1.md :150-172; advisor-u4r-r2-full.md :1-16;
- u4s-carries.md:345-355 (carry 74), u4-build-plan.md §6 (:891-906), and plan.md's C-U* numbers (grep);
- the card (netcode-proof-test.md), whole.

**In builder 3's worktree, read only:**
- client/src/lib.rs :340-365, :486-580 and :600-860;
- client/tests/trace.rs, whole; carry80.rs :60-110;
- proto/src/input.rs :130-180;
- bots/src/lib.rs :70-82, :180-190, :330-345, :470-495, :836-880, :925-942, :975-1020 and :1098-1108;
- room/src/input.rs :170-300; room/tests/commands.rs :250-262, :314-324 and :455-490; room/src/room.rs :596-630;
- lab/src/scenario.rs :40-52, :1005-1040, :1886-1902, :1965-1975, :3780-3795, and greps of the controller,
  never-ack and lag options;
- c-u4f-runs.mjs :1-80 and c-u4f-r2.mjs :1-66 and :136-150;
- SERIES.md's diff, whole;
- f5's lines in review-test/f5-b1.log and b3-client.log.

**Ran (read only):**
- sha256sum of the round file and of the patch;
- `git rev-parse` and `git status` in the worktree;
- `git apply --check` of the patch in the main folder (exit 0), and `git diff --quiet 07289a4 HEAD -- spikes/netcode-proof
  docs/design` (exit 0);
- `git diff --stat 5b29296b -- marks.json crates/judge marks/README.md` in the worktree (empty);
- `df -h`, `du -sm` of the shared target, and greps.

**Not read or not checked:**
- The five seats' own notes files (the round file carries each seat's return word for word).
- u4f-build.md, the end-check logs beyond the lines named, clock.rs, logs.rs, sim/src/world.rs and the rest of
  scenario.rs and room.rs.
- I built, ran and timed nothing.
- What I checked by reading the code: NC-U4F-1's mechanism (lib.rs:640-674), RR-U4F-1's (lib.rs:751-774 against the
  codec's rule), TE-U4F-2's grep, SEC-U4F-2's line, NC-U4F-2's settings, EA-U4F-8's header, S4's USAGE and the SERIES.md
  count. The interplay of FP1 and FP2 is my own reading, not a run.
- Taken from the seats without re-deriving them: NC-U4F-1's -350 and its J-step span, NC-U4F-3's arithmetic,
  SEC-U4F-1's false-FAIL share ("about half"), and the rust and arch seats' other greps.

## Decisions needed
None.

## For Mustafa
Nothing. The day's message names the three choices above (rule 2).
