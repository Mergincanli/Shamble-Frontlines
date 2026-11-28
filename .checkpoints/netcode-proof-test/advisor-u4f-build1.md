# The advisor's ruling on U4f's partial build (builder 1), 2026-11-18

Seat: a day session's advisor (general agent on main's `.claude/agents/advisor.md`, read with `git show origin/main:`),
effort as set by the session. Read only: no build, no run of the code, no edit of a source or the brief, no commit.
Inputs checked: the brief u4f-brief.md sha256 135325a0bb524104… (matches); the patch wip/u4f-build.patch sha256
92fc99aa9f01aebf… (matches); u4f-lead-bound.md sha256 57c588ec… (equals history.md:6806's freeze record); the build
notes u4f-build.md whole; .gate/u4f/end-check-1.log :352, :359 and end-check-2.log's tail (both "check: all 11 steps
pass", exit 0; tests 605 measured, 701 bites); .gate/u4f/test-times.log; .gate/u4f/server-all-features.log (exit 0,
12:14 UTC). Under TEST-FIRST (history.md:2254-2279): every ruling here is inside the plan, undoable, changes no pass
mark, costs no money and touches nothing on main (rule 2, :2261), so it goes by this ruling; the main session names the
choices in the next day's message.

## Verdict, most important first
**APPROVE the continuation (R3, R4), with conditions.** The built part is sound and proven by two clean end checks;
three Medium gaps would stall the second builder or the runs after the merge, and each gets a fix line below.
- **R2 (g), the frozen lead bound: APPROVE. The room's compare matches the frozen text and 2.4's anchor; no change of
  value or meaning; nothing goes to Mustafa.** The row's words get a Low fix (quote the frozen sentence, name the
  anchor's source) before the merge, and the reading is named in one line of the next day's message.
- **F1 (Medium, new): the fast clock's rule as ruled (U4F-Q11 (v) (b)) never draws a kind-1 refusal under that
  anchor**, so f19 would fail and stop builder 2. The cheat's past-bound stamp moves to meet the room; the room stays.
- **F2 (Medium, from R2 (d)): `World::CHECK_BITES` is not routed by the lab**, so `--bite PM-0001-21/no-cooldown` and
  `refuse-not-clamp` are usage errors today; two of 5.1's bite runs (and 5.3's p8) could not run.
- **F3 (Medium, from R1): f28 never runs the contract checker** its row names, so "every written name has its row" is
  unproven for controller-only names until 5.3 (a stall after the merge).
- **R4: APPROVE the main session's lean**: build up to his 6,000-line stop and stop there; the three lines go to him at
  the stop, not as a forecast now. Inside the plan there is no cheaper option that cuts work without cutting evidence
  or a bar; the three lines' option is a plan change only he can make (split U4f), named below.
- **R3: APPROVE the main session's order with refinements**: fixes first, then -20's rest, the event cap's measurement
  (-20, -21 and -22 all run at its value), -47, -22, the run scripts and one counted check, then -21 smallest suite first.
- **R1: APPROVE the split** (one test per option for f6 and f28), with F3's fix folded in.
- **R2 (a) APPROVE, (b) APPROVE, (c) REVISE, (d) APPROVE with F2's fix, (e) APPROVE, (f) APPROVE with three items named.**
- **Launch condition (disk):** 5.1 GiB free now (`df -h /home/user`, my read), about 1 GiB above the runner's 4 GiB floor;
  the shared target holds 6,921 MiB (`du -sm`, my read); end check 1 alone took about 3.8 GB (notes :201, :213: 8,970
  to 5,179 MiB). The main session decides by P27 (between waves, no lane running: now) whether to part-clean before
  builder 2 starts; my lean: yes, the six part-cleans, D2's readings before and after on the card.

## R2 (g): the lead bound's anchor (the frozen value)
- **The code:** kind 1 refuses when `dist(c.tick, k.next_tick) > lead_bound` (patch :1306-1310), with `next_tick` the
  room's next tick to run (patch :2053-2059; room.rs:826 sets it to t + 1 after tick t; `last_tick()` is next - 1,
  room.rs:572-575). f10 pins it: join tick j (the next tick), j + 12 runs, j + 13 is kind 1 (patch :4120-4133).
- **2.4's anchor is the next tick.** 2.4 item 4 (brief :429-437) uses one `t` for kind 1 and for W ("`stamp.wrapping_sub(t)
  as i32` against the bound or `-(W as i32)`"), and W's `t` is "the tick it would run at" (2.3, :401), the next tick;
  the same item names room.rs:439, where `next_tick` starts. 2.6's kind-1 forger stamps "newest + 2 + the bound + k, k
  from 1" because newest + 2 is "the highest tick the room can be at on its arrival at P0" (:519): the next tick at
  arrival. The join anchor is "the room's next tick at the join: room.rs:681" (:402). The room names this tick to every
  client as its tick: `JoinAccepted.tick` (room.rs:681) and the `Pong`'s tick (the next tick to run, :371).
- **The frozen text points to 2.4.** Its fence ends "is refused and counted as kind 1 (2.4), never run"
  (u4f-lead-bound.md:22), and his yes reads "as the fenced text of your entry has it" (history.md:6797). The value is
  unchanged: 400 ms, 12 ticks at 30 Hz and 24 at 60 Hz by `ticks_for_ms_at` (patch :1969, :1992; f31). The entry's own
  arithmetic holds exactly under this anchor: a client that never idles runs at most 12 more commands an episode than
  the server's ticks (stamps at most next + 12 when accepted; u4f-lead-bound.md:9-11); under a last-tick anchor it
  would be 11. So "next" restates the frozen meaning; it does not change it. Nothing goes to him.
- **The row's words (Low, before the merge):** the new row (patch :2722) paraphrases the frozen sentence. Since the fence
  itself makes this row the value's record ("recorded in every run's header ... a new SERIES.md row", u4f-lead-bound.md
  :21), it quotes the sentence and names the anchor's source: instruction 8. The row is new (in U4f's own subsection),
  so this edits no existing row.
- **Named in the next day's message** (rule 2), one line: "The lead bound counts from the tick the room runs next, as
  the brief's 2.4 has it (the tick its join and pong messages name); the value stays 400 ms."

## F1 (Medium): the fast clock's past-bound stamp never reaches kind 1
- U4F-Q11 (v) (b) (brief :1557-1561): "each stamp one tick newer than its previous, held at its newest tick plus the lead
  bound, every 10th command one tick past it", so its highest stamp is newest + bound + 1. A snapshot of tick N is coded
  after tick N runs, so on any later arrival the room's next tick is at least N + 1, and newest + bound + 1 - next <=
  bound: never kind 1. f19 asks "lead-bound and not-newer refusals each above 0 per episode (SA-6)" (:702), and 2.6's
  row reports them per episode (:536). As ruled, builder 2 meets a failing f19 (a stop) or a fast clock whose bound is
  never shown acting.
- **Fix (this ruling; it amends my (v) (b)):** every 10th command is stamped the larger of the previous stamp and newest
  + 3 + the bound: at P0 the room's next tick on arrival is at most newest + 2 (2.6's kind-1 row), so it lies past the
  bound; the other stamps keep (v) (b)'s rule (the larger of the previous stamp and min(previous + 1, newest + the
  bound)), so stamps never decrease and every `Input` encodes (TE3-4). f19 runs at P0, the suite's profile (U4F-Q11 (ii)
  (a)). The not-older-than bite still shows (its held equal stamps run under `>=`); the clean excess stays at most 0.

## F2 (Medium): the lab's bite routing misses `World::CHECK_BITES`
- `Bites::of` sets `world: Sim::BITES.contains(&id)` (lab/src/scenario.rs:231; `Sim = World`, :57) and refuses an id no
  instance plants (:254-266). `World::CHECK_BITES` is a list apart (patch :2651-2655), which no lab line reads (the
  patch names it only in lab/tests/bite_homes.rs, :3833). Fix: instruction 10.

## F3 (Medium): f28 does not run the contract checker
- f28's row: "every series of 2.7 written, with its SERIES.md row ... (the contract checker's own check, run on those
  short scenarios ...; contract.rs:6-9)" (:711). The built f28 reads series values only (patch :932-959); no line of
  the patch calls `contract::check` (Grep: no "contract" in the patch). The r2 test (contract.rs:466-472) runs a
  fixed-lead P0 run, so the names written only under the controller (`controller_max_lead_count`, the press series
  under A) are unchecked until 5.3. Fix: instruction 6.

## R1: the time-rule stop (f6 23.86 s, f28 23.78 s; the line 22.96 s)
- **APPROVE the split**: f6 and f28 each become one test per option (test-times.log :45, :67 against :23's 11.48 s; f40,
  one run, 12.03 s at :89). Each run is the P2_SHORT-form run the line was set for (2.10, :667-680), so each split test
  sits near 1 times the guard test; the total stays under the test step's own line (698.9 s against 1,207 s: notes
  :194, :204). The counts rise by 2 in each build, stated with their `--list` lines. f28's later suites follow the same
  rule: one test per suite and option, never several runs in one test.
- f6, f28 and f40 make the same short run with the same settings up to five times (patch :894-906, :913, :937, :965): a
  run two tests share may be made once per test process and read by both (each test, run alone, still makes it once).

## R2: the builder's readings
- **(a) APPROVE** refuse-not-clamp's refusal as `RefusalKind::Cooldown`: `RefusalKind` is a seam type (sim/src/seam.rs:509)
  and a new variant a seam change (2.13, :784-786); the plant is bites-only (patch :2664-2668, :2688-2706) and the
  measured build is unchanged (wasm 27,000 ticks, 0 mismatching: notes :206). It holds only if the clamp suite's clean
  run has no sim refusal of any kind on the clamp cheat's commands, and `event.clamp_kind.counted_count` counts, per
  beyond-reach (slot, seq), any sim refusal record whatever its kind (SERIES.md:144: "from the sim's refusal counters
  only"): instruction 12. The report names the Cooldown kind under that bite.
- **(b) APPROVE** the late/past-window check after kinds 4-6 and before 10. 2.4 item 3's words are "with option B's late
  rule and option A's past window (U4F-Q18) after kind 2" (:426-428): the place is after kind 2. Honest commands pass
  kinds 4-6, so no honest count moves; a forged item that is also late is now counted under its own kind, as item 5
  ("the first check that refuses") and item 6's attribution want (:438-455), instead of `late` and so "unrejected".
  Condition: f11 proves it (instruction 9).
- **(c) REVISE** `brawl-presses` as a new value of SERIES.md:348's row. That row lists its values ("`brawl`, or an
  open-loop script `presses`, `clamp` or `steady`"); a judged run writing another one leaves an existing row wrong
  about what runs write, and making it right edits that row, which his 11:31 yes sends to him first (history.md:6801).
  No check reads the param (the judge and the run scripts have no reader: Grep), so the cheap route asks him nothing:
  the header writes `brawl` and the press rule gets its own new param and row (instruction 7).
- **(d) APPROVE** `World::CHECK_BITES` (bites-only): adding to `World::BITES` would break its pins outside the lane
  (sim/tests/world.rs:2294-2309; npt-hash-wasm's indexes 1..=4, hash-wasm/src/lib.rs:10, :84). With F2's fix.
- **(e) APPROVE** the moved test (crates/lab/tests/ is in the lane: brief :876) and `ClientEndpoint::with_quant` (patch
  :214-220): it plants exactly what the old `with_bite(id)` planted for a proto id (`self.quant = Quant::with_bite(..)`,
  patch :236; loopback.rs's new lines, patch :2482-2483), and no test-only fix exists without a manifest change (a stop).
  Recorded as a deviation from section 6 item 4's "by test edits only" (:1179-1180), for the engine-architect. The
  P33 evidence (`cargo test --locked -p npt-server --all-features`, exit 0) is from 12:14 UTC, before blocks 3-6: it is
  re-run at the end (instruction 14).
- **(f) APPROVE** the public items past 2.13 (notes :283-297), each with a consumer, for the engine-architect's review,
  with three items the notes did not mark: `join_tick()` (2.13 says "the endpoint keeps its `JoinAccepted` tick, no new
  public item", :792), `lead_now()` (not in 2.13's list; its consumer is the settled-lead series) and `unknown_pongs()`
  made public with no lab writer (2.1's Bounds row: "public only if the lab logs it", :374). Instruction 11.

## R4: the size, and the stop
- **Now:** 3,753 added lines (notes :232-235), 1.25 times M's 3,000, under the 6,000 stop (P18's copy :1905-1908; his
  restatement history.md:6803). Built components ran about twice the brief's estimates (client about 1,421 against 700,
  room about 1,821 against 950), about 60 % tests. By R3's order, B1-B6 likely add about 2,000 lines, so the stop
  likely lands at or soon after -21's start.
- **APPROVE the lean:** his rule is a stop at the event ("past 6,000 added lines the build stops and I get three lines"),
  not a forecast; a forecast now would be a second decision message today against rule 2 (:2260). P18 asks the size on
  the card at every snapshot beside its plan: the main session writes 3,753 beside 3,000 and the 6,000 line now, with
  the projection, on the card (no message).
- **At the stop** (instruction 5): no new block; the end checks run on the tree as it stands (verification adds no line),
  so the main session holds a proven tree whatever he answers; the three lines go in the next day's message, its first
  line saying it is stopped on him (rule 2). The panel waits for his answer, so its one round lands on the head that
  merges.
- **The cheaper options:** inside the plan, none cuts work without cutting evidence (CLAUDE.md: "shrinking evidence" is
  not allowed) or a bar (marks.json's parts need both options and every count). Line economy inside the plan is the
  builder's (instruction 15), not an option for him. So the three lines offer:
  (a) **split U4f at the stop** (a plan change, his): close U4f with what is built (the clock, the controller, option A,
  every room check, -20, -47 and -22 with their scripts) and give -21's cheats, suites, twins and scripts a unit of
  their own next, pointing to the same brief sections and rulings, with its own size line; no pass mark, bar, run or
  rule changes; -20, -47 and -22 can be judged meanwhile; the cost is one more panel review and merge;
  (b) **go on**: U4f continues to about the projected total, one review and merge.
  My provisional lean: (a), since by this order the stop lands at a clean boundary; confirmed at the stop with the real
  numbers. The template, filled then: 1. What grew: U4f at N lines against M's 3,000 (the stop 6,000); client and room
  about twice the brief's estimates, about 60 % tests. 2. Why: every check, option and bite has its own unit test, as
  the brief's minimum counts ask; built: (list); not built: (list, estimate). 3. The cheaper option: (a), or (b) to
  about T lines; the lean.

## R3: the order (blocks for builder 2)
- **APPROVE the lean** (what 5.3's pre-runs and the -47, -22 and -20 sets need first, -21 last), refined: B1 fixes
  these rulings ask, so the built part is mergeable; B2 -20's rest; B3 the event cap's measurement, because 2.8 item 5
  (:609-612) has the judged -20, -21 and -22 runs all run at its value; B4 -47; B5 -22 (with 2.5's per-session counters
  at mid-run ends, which -22's kicked never-acker and -21's forgers both need); B6 the run scripts for what is built and
  one counted check, so a test-step problem shows before -21's many lab tests; B7-B9 -21, smallest suite first (clamp;
  lag switch with fast clock, which share A1's movement series; forged last), each with its script rows, so a stop
  leaves whole suites; B10 the end checks. A -21 mark is judgeable only with all its parts, so within -21 the order
  serves only the size stop's clarity.

## Instruction lines for builder 2's prompt (copy as they stand)
```text
1. Who and where: one builder (general-purpose, opus, effort xhigh) in builder 1's worktree,
   /home/user/Shamble-Frontlines/.claude/worktrees/agent-a856ba7ccb7d032c3, branch u4f-build, under
   /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4f-run.sh and W1-TARGET's lock, by u4f-brief.md rev 3.3
   (sha256 135325a0bb524104…, filled at launch) and the advisor's ruling advisor-u4f-build1.md. Commit nothing, push nothing.
2. Restart first (brief section 1 item 3; P14): read .checkpoints/netcode-proof-test/u4f-build.md whole, then
   `git diff --stat 5b29296b03f4e116c0afac8b89d9bc15cc1adaf7` and `git status --short` in the worktree; they must show
   builder 1's 24 files (18 changed with 1,606 insertions, 6 new with 2,147 lines: 3,753 added). A difference stops you
   before any edit. Append your notes to u4f-build.md under "## Builder 2", its first line this task; each block written
   at once when done, with SIZE-CHECK's count after it.
3. Read in the brief only what your blocks need: sections 1, 2.5 to 2.8, 2.10 (the rows you build), 2.11, 2.12, 4, 5.3,
   5.4, 5.6, 6 and 7, section 10's Ruled lines, and section 12 item 3's copies those cite.
4. Blocks, in this order, each compiling with its tests passing before the next, each at most about 400 added lines:
   B1 the fixes of lines 6 to 14; B2 -20's rest (line 16); B3 the event cap's measurement (line 17); B4 -47 (line 18);
   B5 -22 (line 19); B6 the run scripts for B2-B5 and one counted check (line 20); B7 -21's clamp; B8 -21's lag switch
   and fast clock; B9 -21's forged suite (line 21 for B7-B9); B10 the end checks of section 6, twice.
5. Size (P18; his size stop): after every block, SIZE-CHECK by the notes' counting rule (u4f-build.md :49-50) in your
   notes. At the first block end past 6,000 added lines: build nothing more; run section 6's end checks on the tree as it
   stands; write in your notes the count, each component against SIZE-CHECK's table (brief :927-939), what is built and
   what is not by mark and suite, and your estimate for the rest; then return.
6. f6 and f28: one test per option each (f6_*_a and f6_*_b; f28's -20 half the same), every later f28 suite one test per
   suite and option; each f28 test runs `contract::check` on its run's logs against marks/SERIES.md and asserts no name
   failure (every written name has its row); its other parts by their own rules at that scale. A short run two tests make
   with the same settings may be made once per test process and read by both. Each new test timed alone as before.
7. The -20 behaviour: `--behaviour brawl` stays the brawl; U4F-Q11 (i) (a)'s press rule is a new setting
   `--brawl-press-every <n>` (2 in -20's runs), its header param `header.params.brawl_press_every_count` (absent when off)
   with a new row in U4f's subsection; `brawl-presses` leaves the settings and the header. Never write a value
   SERIES.md:348's row does not list.
8. The `header.params.lead_bound_ms` row's meaning cell reads: the room's lead bound in ms, frozen at Mustafa's LEAD-BOUND
   yes of 2026-11-18 (400 ms: 12 ticks at 30 Hz, 24 at 60 Hz, by `ticks_for_ms_at`); the frozen text (u4f-lead-bound.md's
   fence) reads "A command stamped more than the bound ahead of the room's tick is refused and counted as kind 1 (2.4),
   never run."; the room's tick is 2.4's, the tick the room runs next (the one its JoinAccepted and Pong name). No other
   word of it, and never the value.
9. f11: add, under both options, a command that is late and breaks kind 4 (and one that is late and breaks kind 6):
   each counted under its forged kind, never `late` (+0 tests).
10. npt-lab's `Bites::of` (lab/src/scenario.rs:227-235) routes `World::CHECK_BITES`' two ids to the world as it does
   `World::BITES`', planted through `World::with_bite`; a bites-build lab test shows each id accepted and routed to the
   world only.
11. Public items: name in your notes, against 2.13, every public item you add, and builder 1's `join_tick()` and
   `lead_now()` as beyond 2.13's list; the lab writes the endpoint's `unknown_pongs()` as `unknown_pongs_count` in
   `event.client_counts` (added to U4f's new client_counts row), else it goes back to crate-private; `Room::set_cmd_timing`
   refuses, by a debug assertion, to change the option once any slot has joined.
12. The clamp cheat fires by holding fire (no press edges), so the clean sim refuses none of its commands; the lab's
   `event.clamp_kind.counted_count` counts, per beyond-reach command (slot, seq), any sim refusal record whatever its
   kind (refuse-not-clamp records Cooldown); f18 asserts the clean clamp run's sim refusals on the cheat's slot are 0.
13. The fast clock (U4F-Q11 (v) (b) as amended by advisor-u4f-build1.md F1): every 10th command is stamped the larger of
   the previous stamp and newest + 3 + the bound; every other stamp is the larger of the previous stamp and
   min(previous + 1, newest + the bound); f19 runs at P0 and asserts kind-1 and kind-2 refusals each above 0 in every
   episode and excess commands at most 0. No debug-build test runs no-newer-tick or not-older-than with two equal stamps
   of one slot in one tick (world.rs:2084's debug assertion; u4f-build.md :74-76).
14. Also: the summary line prints the controller's cap, not `lead_for(profile)`, on a controller run (u4f-build.md
   :307); section 6 item 4's `cargo test --locked -p npt-server --all-features` re-run on the final tree; the moved test
   stated -1 (npt-server) and +1 (npt-lab) with its `--list` lines.
15. Lean code, never less evidence: shared test helpers, per-case tests through one helper (each case still its own
   test for the time rule), one doc line per test naming its row and claim; f33 (and f14 and f35 where their claim
   allows) on the lab's own functions with hand-made records, not on scenario runs (2.10's smallest-scale rule). The
   deferred Lows that ride (RR-B1-7 (1), RR-CB-2's docs, TE-B1-R2-1) ride only where the file is open, and are left
   (9.2) if the count nears 6,000.
16. B2: `second.s.buffer_depth_count` (2.7's -20 row; U4F-Q3 (c): 0 under option A, `buffer_depth_ticks_count` not
   written under A), U4F-Q3 (c)'s ran-early series (a new row), f36 (ii); the plants of f26, f27 and D14, each in one
   `-x -k` runner call, its diff in your notes, restored and its sha256 checked (P26).
17. B3 (2.8 items 3-5; U4F-Q12 (b)): the queue's high-water series per client (a new row); ops/runs/c-u4f-cap.mjs on
   c-u4r-r2.mjs's model; its 9 runs (SS, 48 clients, the brawl with U4f's controller, P2, P3 and B1 with at least 2
   bursts, seeds 1 to 3, under the runner, `-x`); in your notes the figures, the stated margin and the value (the largest
   per-client high-water mark times the margin, checked below the never-acker's rate x 255 / 30 s); the value into the
   room's default with `event_queue_cap_provisional` false. Plan §4.7's text is the main session's.
18. B4: the `clock` suite (`--connects`, `params.connects_count` with a new row), U4F-Q4 (b)'s never-firing script,
   `event.connect_clock.offset_err_ns` at 2.1's read instant, f29.
19. B5: the never-acker and the `never-ack` suite, -22's series in 2.7's -22 row that no writer has yet, f21; 2.5's
   per-session counters at every mid-run session end (before the `Leave` reaches the room, before `Room::leave`, before
   a `Decoder` drops; a kicked slot's at its kick), f14, f35; f39's kicked-slot rejoin case through the never-acker's
   kick (+0).
20. B6: ops/runs/c-u4f-r2.mjs with 5.3's p5, p6, p10, p11, p12 and p21, and ops/runs/c-u4f-runs.mjs with the -47, -22
   and -20 sets in 5.4's call forms (ADV-CBH-2's hardenings; the stop note), on c-u4r-r2.mjs's and c-u4r-runs.mjs's
   models; each npt-lab command line they issue run by hand on a short setting (section 6 item 5); then one counted
   check.mjs run (`-x`, `df` first), its test step's time in your notes against 1,207 s.
21. B7-B9 (-21, 2.6, 2.7, 2.12): each suite whole with its tests (clamp: f18; lag switch: f20; fast clock: f19, f37 and
   U4F-Q11 (iii) (c)'s test; forged: the 11 forgers, `--forge-every`, the twins and the twin's skip in the lab,
   `event.forged_kind.*`, f15, f16, f22, f23, f33), and its rows in c-u4f-r2.mjs (5.3's p1-p4, p7-p9, p13-p20) and
   c-u4f-runs.mjs (-21's two calls).
22. Every stop of section 1 item 5 stands, the time rule and the test step's 1,207 s line among them; never edit
   marks.json, an existing SERIES.md row, marks/README.md or crates/judge (stop and report); disk before each build by
   section 4 rule 6 (never clean or pack); write files with the Write and Edit tools only (P24); one command per call in
   the simple forms (P17).
```

## Findings, each with its severity (CLAUDE.md's scale)
| Id | Severity | What | Fix |
|---|---|---|---|
| F1 | Medium | the fast clock's past-bound stamp never reaches kind 1 (U4F-Q11 (v) (b) against 2.4's anchor) | line 13 |
| F2 | Medium | `World::CHECK_BITES` unrouted in `Bites::of`: two bite runs impossible | line 10 |
| F3 | Medium | f28 runs no contract check | line 6 |
| F4 | Medium | `brawl-presses` widens SERIES.md:348's closed list without his yes | line 7 |
| F5 | Low | the lead-bound row paraphrases the frozen sentence | line 8 |
| F6 | Low | `join_tick`, `lead_now` unmarked beyond 2.13; `unknown_pongs` public with no writer | line 11 |
| F7 | Low | `set_cmd_timing` can change the option after joins | line 11 |
| F8 | Low | P33's evidence predates blocks 3-6 | line 14 |
| F9 | Low | f39's kicked-slot case unproven (notes :77-78) | line 19 |
| F10 | Low | the summary line's "lead N" on controller runs (notes :307) | line 14 |
| F11 | Low | the same short run made up to five times (f6, f28, f40) | line 6 |

## What I checked, and what I did not
- **Read:** the inputs above; the brief's sections 1, 2.1-2.13, 4, 5.1-5.6, 6, U4F-Q8, Q11, Q12 and section 12 item 3's
  copies; history.md:2234-2282 and :6791-6806; u4f-lead-bound.md whole and its sha256; my earlier rulings
  (advisor-u4f-brief.md, advisor-u4f-fold-check.md, advisor-u4f-lead-bound.md); the patch's input.rs, room.rs, world.rs,
  SERIES.md, scenario.rs, loopback.rs, bite_homes.rs and client lib.rs hunks and commands.rs's f7-f13; the base's
  room.rs:395-458, :566-575, :672-721, lab scenario.rs:205-275, contract.rs:1-40 and :444-472, SERIES.md:142-159 and
  :340-351, marks.json:4981-5013, ADR 0001:224-257, sim seam.rs:509-520, l1 lib.rs:60-73 and :176-197.
- **Ran (read-only):** sha256sum of the brief, the patch and u4f-lead-bound.md; `df -h /home/user`; `du -sm` of the
  shared target; `nproc` (4); greps of the logs named above.
- **Not checked:** I built, ran and timed nothing; the end checks' and the time log's figures are the builder's runs,
  read from their logs. F1's arithmetic is by reading (the snapshot of tick N follows tick N's run, room.rs:826), not a
  run. The disk growth of builder 2's work is an estimate from builder 1's two end checks. The projection to about
  5,750 lines after B6 is mine, from the brief's component estimates at builder 1's observed ratio.

## Decisions needed
None now. Every ruling here goes by TEST-FIRST rule 2; the main session names in the next day's message: the anchor
reading (R2 (g)), the fast clock's stamp (F1), the `--brawl-press-every` route (R2 (c)) and the split tests (R1). The
size stop's three lines go to Mustafa only when the build passes 6,000 lines (R4).

## For Mustafa
Nothing now.
