# U4f's merge (M8): the plan edits, each as old text and new text for the Edit tool

Drafted 2026-11-19 for the main session, which applies them by Edit in the M8 commit (advisor-u4f-r1.md:339-355) after
reading them. Each old text was checked unique in its file (`grep -cF` = 1, my run) and has no trailing spaces; the
old texts of edits 3 to 6 stay unique after the others are applied (each new row begins with "U4f"). No edit touches
marks.json, the judge, an existing SERIES.md row or any word of Mustafa's; his decisions are cited by history.md line,
never retyped. Edits 1 to 7 are M8's (edit 8 records the rows left as they are); edits 9 and 10 are proposed only, not
in M8's list, for the main session to take or leave. Files: edits 1 to 6 and 10 u4-build-plan.md, edit 7 plan.md,
edit 9 u4s-carries.md.

**Which file "plan §4.7" and "§4.1's rows" are.** In u4f-brief.md the short name `plan` is N/u4-build-plan.md
(u4f-brief.md:206), and N/plan.md is always written `plan.md`. So 2.8 item 5's "plan §4.7" is u4-build-plan.md §4.7,
"Events (Q14)", with the cap's formula at :335 (the brief's own cite: u4f-brief.md:602; room/src/events.rs:22 names
"u4-build-plan.md §4.7"), and EA-U4F-2's "§4.1's rows" are u4-build-plan.md §4.1, "Public items: plan §4.7 plus these"
(the rows U4e's T6 kept: advisor-u4e-r2.md:66; u4e-report.md:89-91). plan.md's own §4.1 is the layout block, with no
per-crate rows, and its §4.7 lists symbols, with no cap text: if "plan.md §4.7" is read literally, edit 1 has no home
there. The C-U* carries do live in plan.md, in §12.1's unit rows (U5 at :1252, U6 at :1253, U7 at :1254).

**Numbers checked free (edit 7):** plan.md holds C-U6-1 to C-U6-16, C-U7-1 and C-U7-2, C-U8-1 to C-U8-6 and no C-U5
(`grep -o 'C-U[0-9]*-[0-9A-Z]*' plan.md`, my run). Across the repository's .md, .rs and .mjs files, C-U5-*, C-U6-17 to
-19 and C-U7-3 to -9 appear only in advisor-u4f-r1.md, advisor-u4f-pass.md, the card, u4f-build.md and three code docs
that already cite C-U6-17 by the ruling's number (room/src/input.rs:317, lab/src/scenario.rs:58, client/src/lib.rs:358).

---

## Edit 1: u4-build-plan.md §4.7, the event cap's text (u4f-brief.md 2.8 item 5: the measure, both bounds, the figures)
File: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4-build-plan.md

Why: 2.8 item 5 (u4f-brief.md:609-612); U4F-Q12 (b) has §4.7's text amended to state the measure and both bounds
(u4f-brief.md:1569-1573; ruled, advisor-u4f-brief.md:278-282); the stop ruling R2 (advisor-u4f-stop.md:107-137). The
formula's sentence stays as written (its history); the value and its derivation are added under it. Sources of the
figures: .gate/u4f/w2-cap/cap.txt (:2-43 the runs, :44 the value), u4f-build.md:414-449, room/src/events.rs:38-44.

Old:
```text
- The cap: the stress scene's measured rate of relevant gameplay events x (B1's 1 s + RTT + a margin), recorded. At
  about 350 hit events per second (net F121), a cap below about 400 would resync all 48 clients on every burst.
```

New:
```text
- The cap: the stress scene's measured rate of relevant gameplay events x (B1's 1 s + RTT + a margin), recorded. At
  about 350 hit events per second (net F121), a cap below about 400 would resync all 48 clients on every burst.
  - **Set at U4f: 1,600, no longer provisional** (u4f-brief.md 2.8; U4F-Q12 (b), advisor-u4f-brief.md:278-282; the
    advisor's stop ruling R2, advisor-u4f-stop.md:107-137): the room's default `EventQueue::CAP`
    (room/src/events.rs), with `event_queue_cap_provisional` false in every header. The formula above is not used: at
    EVQ-LOAD's rate it gives at most about 1,008 for any margin up to about 0.73 s, while 1,024 already overflowed at
    P3 without a burst (NA-3; P32's 27 forced resyncs).
  - **The measure:** the largest per-client high-water mark of queued gameplay events, per 30 s bin
    (`second.s.event_queue_max_count`) and per burst (`event.burst_heal.queue_max_count`), over nine short runs by
    ops/runs/c-u4f-cap.mjs: SS, 48 clients, the brawl under U4f's controller, option B, P2, P3 and B1 (two bursts),
    seeds 1 to 3, a 10 s warm-up and 60 s measured, the cap opened to 8,192 so that no overflow resync cuts the marks;
    every run exit 0 with 0 overflow resyncs (.gate/u4f/w2-cap/cap.txt; u4f-build.md:414-449).
  - **The lower bound, the honest need with a stated margin:** the largest mark, 1,045 (P3 seed 3, bin 0), x 1.5 =
    1,567.5, rounded up to a multiple of 64: 1,600.
  - **The upper bound** (carry 50; NA-12): a never-acker's events expire once older than 255 ticks
    (room/src/events.rs:19-20), so a cap above the lowest client's mean queued rate x 255 / 30 is never reached and
    -22's headroom check would fail a correct build: 509.2 a second (P3 seed 2) x 8.5 = 4,328. 1,600 lies below it.
  - **The figures** (per-client high-water marks, bin 0 / bin 1, seeds 1 to 3): P2 161 / 156, 166 / 152, 157 / 188;
    P3 689 / 1,001, 683 / 449, 1,045 / 935; B1 747 / 651, 669 / 695, 739 / 633, its bursts 615 to 727. Room-wide
    reliable events 503.4 to 572.5 a second (bin means); the clients' mean queued rates 509.2 to 554.7 a second
    (cap.txt:2-44).
  - **Not measured:** P0, where each acknowledgement returns within a step, so an honest queue empties at each step
    (advisor-u4f-stop.md:115-117), and 10-minute windows. -20's judged set (six 10-minute P3 runs) is the check: an
    honest client with any overflow resync there comes to the advisor before -22's set, the need is measured again
    with the cap opened, the value re-set by the same rule under carry 50's bound and -20's set re-run at it; with
    none, 1,600 stands, and -20's set report states the 10-minute high-water mark and the margin it leaves, a margin
    under 1.5 named for U4g (advisor-u4f-stop.md:126-134). The judged -20, -21 and -22 runs all run at this value.
  - The cap is a design constant -22's headroom check reads, not a bar (marks.json:5227; u4f-brief.md 5.6 B4).
```

---

## Edit 2: u4-build-plan.md §5, U4f's entry, with U4f2's new entry after it
File: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4-build-plan.md

Why: the stop ruling's plan text (advisor-u4f-stop.md:98-99): U4f's entry moves -47, -20 and -22 and builds -21's room
side and three suites; the new entry depends on U4f and moves -21's eight parts with their twins and 13 bites; his
SIZE-U4F (a) (history.md:6896-6903). U4f2's size line follows SIZE-CHECK (history.md:2048-2051; the reading
:2067-2072) as the stop ruling asks (advisor-u4f-stop.md:86-88): the entry states its size. The line "past at more than
3,000" is twice the stated size, derived here; the advisor's check of U4f2's brief confirms or corrects it.

Old:
```text
### U4f, clock and commands (depends on U4r; size M; its own brief)
- **Builds:** `ClockSync` (§4.8); the client's tick-lead controller (net F104); `InputBuffer` option A; every other
  room-side check with its -21 counter; the server-side checks' -21 counters and judged runs on U4e's pump (U4e builds
  and unit-tests the checks: U4E-Q11); the cheats' `Behaviour`s (the 11 forged kinds per §4.10, the clamp points, the
  1.5x clock, the lag switch, the never-acker).
- **Marks moved** (build, merge, then the judged runs): -47 (1,000 connects at P2; bites newest-raw-sample and
  tick-quantised receive times); -20 (P3, both options; bite redundancy-1); -21's 8 parts with their twins (§4.10) and
  13 bites, each bite run in the suite and option §4.14 or marks.json's notes name; -22 (one never-acker among 48 for
  10 min, no precondition (README §4 states the exception: U4j); its runs still log PM-0001-2's series (shots,
  projectiles, vehicles, teams, portals), and U4f's report states the load each run carried (the advisor's condition
  4); bite unbounded over a full S30).
- **Reviewers:** netcode-architect, security-auditor (hostile input), test-engineer, rust-reviewer.
```

New:
```text
### U4f, clock and commands (depends on U4r; size M; its own brief)
- **Builds:** `ClockSync` (§4.8); the client's tick-lead controller (net F104); `InputBuffer` option A; every other
  room-side check with its -21 counter; the server-side checks' -21 counters on U4e's pump (U4e builds and unit-tests
  the checks: U4E-Q11); -21's room and sim side whole, and three of -21's four suites (clamp, lag switch, fast clock)
  with the cheats' `Behaviour`s they run (the clamp points, the 1.5x clock, the lag switch); the never-acker; and the
  forgers of kinds 1, 2, 4, 5 and 6, built ahead for U4f2's forged suite (Mustafa's SIZE-U4F (a), history.md:6896-6903;
  the advisor's stop ruling, advisor-u4f-stop.md:72-99).
- **Marks moved** (build, merge, then the judged runs): -47 (1,000 connects at P2; bites newest-raw-sample and
  tick-quantised receive times); -20 (P3, both options; bite redundancy-1); -22 (one never-acker among 48 for 10 min, no
  precondition (README §4 states the exception: U4j); its runs still log PM-0001-2's series (shots, projectiles,
  vehicles, teams, portals), and U4f's report states the load each run carried (the advisor's condition 4); bite
  unbounded over a full S30). The sets run -47, -20, then -22, -20's 10-minute P3 runs read for honest overflow
  resyncs before -22's set starts (advisor-u4f-stop.md:126-134). -21's eight parts move to U4f2: U4f's c-u4f-r2.mjs
  rows for -21 are an early read of the built suites, not -21's record (advisor-u4f-stop.md:90-94).
- **Size:** M (about 3,000 added lines: SIZE-CHECK, history.md:2048-2051); 6,213 at its size stop, closed by
  SIZE-U4F (a) (history.md:6896-6903); 6,425 after its round-1 fix pass (u4f-report.md).
- **Reviewers:** netcode-architect, security-auditor (hostile input), test-engineer, rust-reviewer; round 1 also
  seated engine-architect (u4f-review-r1-full.md).

### U4f2, -21's forged-command suite and -21's runs (depends on U4f; size: about 1,500 added lines; its own short brief)
- **Why a unit of its own:** Mustafa's SIZE-U4F (a) (history.md:6896-6903), on the advisor's stop ruling R1
  (advisor-u4f-stop.md:72-99): U4f closed as built, and the rest of the forged-command test with all of -21's runs comes
  here, sized and reviewed apart.
- **Builds** (advisor-u4f-stop.md:80-89, which stands): npt-bots' forgers of kinds 3, 7, 8, 9, 10 and 11 with f15's
  six cases (kinds 1, 2, 4, 5 and 6 were built ahead in U4f); npt-lab's `--suite forged` (11 forger slots, 37 honest
  open-loop clients) and `--forge-every` with its header param and SERIES.md row; the attribution by u4f-brief.md 2.4
  item 6 and `event.forged_kind.*`; the twins and the twin's skip in the lab (kind 10's and kind 3's rows); f16 (24),
  f22 (2), f23 and f33 (13); 5.3's p1, p7, p14 and p17; -21's set in c-u4f-runs.mjs; f28's clamp and lag-switch B
  halves as its first block (the stop ruling R3 (ii)); its own end checks (u4f-brief.md section 6, items 1 to 5). Its
  brief is short (TEST-FIRST rule 3): what it builds and tests, pointing to U4f's brief sections, to
  advisor-u4f-build1.md's 22 lines and to the stop ruling; it goes to the advisor before its build.
- **Carries, by number:** C-U4F2-1 to -4 and C-U4F2-SEC-1 (advisor-u4f-r1.md:65-87); ADV-U4FP-3, beside C-U4F2-2
  (advisor-u4f-pass.md:95-98); and U4f's not-met -21 carries, 46's lab twins and 86's forged rows
  (advisor-u4f-stop.md:97).
- **Marks moved** (build, merge, then the judged runs): -21's 8 parts with their twins (§4.10) and 13 bites, each bite
  run in the suite and option §4.14 or marks.json's notes name. c-u4f-r2.mjs runs whole again from U4f2's merge commit
  (all 21 rows: 5.3's record), then -21's set: 24 measured, 12 twins and 13 bites in two judge calls
  (advisor-u4f-stop.md:90-94). -21 is judged only after U4f2 merges.
- **Size:** about 1,500 added lines (builder 3's estimate, about 1,400 to 1,550: u4f-build.md:863-866), so past its
  plan at more than 3,000 (twice the plan, by SIZE-CHECK: history.md:2048-2051); the card writes it, with that source,
  before its first snapshot (advisor-u4f-stop.md:86-88). Its report states its own counts and the sum with U4f's
  against u4f-brief.md 2.10's minimums (advisor-u4f-stop.md:95-96).
- **Reviewers:** netcode-architect, security-auditor (hostile input), test-engineer, rust-reviewer (U4f's:
  advisor-u4f-stop.md:88-89).
```

---

## Edits 3 to 6: u4-build-plan.md §4.1, one U4f row per crate U4f changes
File (all four): /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4-build-plan.md

Why: EA-U4F-2's fix (u4f-review-r1-full.md:259-264): the merge commit updates §4.1's npt-room, npt-client, npt-bots,
npt-sim and npt-proto rows; FP20 (advisor-u4f-r1.md:276-277). The table already keeps several rows per crate, so U4f's
items go in a new row per crate, placed after that crate's last row; no existing row's words change. Each old text is
the opening cells of the next crate's row, and the new text puts U4f's row in front of it. The items and their
consumers: u4f-report.md, "Public items". The C-U7-4 trim and C-U4F2-2's built-ahead items are named in the rows, so no
later unit reads them as approved surfaces without a consumer.

### Edit 3: npt-sim (after the last npt-sim row)
Old:
```text
| npt-proto | `RoomControl` (unfeatured; N5) |
```

New:
```text
| npt-sim | U4f, bites builds only: `World::CHECK_BITES` (PM-0001-21's two sim-side check bites, no-cooldown and refuse-not-clamp, each planted through `World::with_bite`) | a list apart from `World::BITES`, which npt-hash-wasm's replayer indexes and npt-sim's tests pin (NA-9, U4F-Q1 (a); the advisor's U4f build-1 ruling, R2 (d)); the sites' five inert lines outside the `bites` gate accepted as built (EA-U4F-3; the advisor's U4f round-1 ruling) |
| npt-proto | `RoomControl` (unfeatured; N5) |
```

### Edit 4: npt-proto (after the last npt-proto row)
Old:
```text
| npt-client | a dependency on npt-link, for `Channel`;
```

New:
```text
| npt-proto | U4f: `Pong::server_ns` (the answering end's time at the `Pong`'s send, 8 B; the catalogue's Pong row, docs/design/netcode-proof-messages.md:92) | ClockSync's exact server time (U4F-Q6 (b); u4f-brief.md 2.1): the room's `Pong` sets it, a client's sets 0 and the room never reads it |
| npt-client | a dependency on npt-link, for `Channel`;
```

### Edit 5: npt-client (after the npt-client row)
Old:
```text
| npt-room | `Room<S: SimCore>`;
```

New:
```text
| npt-client | U4f: `ClockSample` (its fields `offset_ns`, `rtt_ns`, `at_ns`, `tick`, `server_ns`); the public methods of plan.md §4.7's `ClockSync`: `offset_ns`, `samples` and `resets`, and `new`, `period_ns`, `ping_every_ns`, `unknown_pongs`, `next_tick_at` and `one_way_hi_ns`, these six trimmed by C-U7-4; `ClientEndpoint`'s methods `with_clock`, `clock`, `commands_due`, `stamps_skipped`, `lead_now`, `not_due`, `unencodable_commands`, `unknown_pongs` (public since U4f), the unfeatured `with_quant`, and, built ahead for U4f2, `skip_command` and `twin_left_out`; bites builds only, `BITES` (6 ids) | u4f-brief.md 2.1 (ClockSync), 2.2 and U4F-Q7 (the controller, its due and skip counts), 2.9 (c), 2.12 (the twin's skip); the -47 read (`clock`); the lab's `client_counts` (`lead_now`, `unknown_pongs`, `stamps_skipped`, `unencodable_commands`); f36 (ii) (`not_due`); P33 (`with_quant`: npt-server's tests pass npt-proto's planted `Quant`; the advisor's U4f build-1 ruling, R2 (e)); C-U4F2-2 (the built-ahead pair); C-U7-4 (ClockSync's surface trimmed when it becomes the Worker's); the advisor's U4f round-1 ruling (EA-U4F-1, EA-U4F-2; `join_tick` removed by its FP12) |
| npt-room | `Room<S: SimCore>`;
```

### Edit 6a: npt-room (after the npt-room row)
`Room::cmd_timing` has no consumer at the merge (u4f-report.md, "What was not verified"); it is listed because it is
public, its reason 2.3's setting.

Old:
```text
| npt-server | `Decoder::pump`
```

New:
```text
| npt-room | U4f: `CmdTiming` and `Refusal` with `kind()` (to be renamed `CommandRefusal`, its records in the sim's struct form, by C-U4F2-3); `Room`'s `LEAD_BOUND_MS` (400 ms, frozen), `with_cmd_timing`, `set_cmd_timing`, `cmd_timing`, `with_lead_bound_ms`, `lead_bound`, `refusals`, and, built ahead for U4f2, `refusals_dropped`; `ClientCounters`' fields `lead_refused`, `not_newer`, `seq_refused`, `move_refused`, `aim_refused`, `seq_outside_window`, `rejoin_overlap`; `ClientSlot`'s getters `on_time`, `presses`, `ran_early`, `commands_run`; bites builds only, `BITES` (16 ids) | u4f-brief.md 2.3 (option A beside B), 2.4 (the checks' order, kinds and counters), U4F-Q8 (i) (b) (the lead bound as room data, in ms; Mustafa's LEAD-BOUND (a), history.md:6791-6806), U4F-Q9 (the refusal records); `set_cmd_timing` for the lab, which holds the room inside its server; the per-tick values -20's series and U4F-Q3 (c) read (`on_time`, `presses`, `ran_early`, `commands_run`); C-U4F2-2 and C-U4F2-SEC-1 (`refusals_dropped`); the advisor's U4f round-1 ruling (EA-U4F-1, EA-U4F-2; `InputBuffer::KEPT_WINDOW` made crate-private by its FP12) |
| npt-server | `Decoder::pump`
```

### Edit 6b: npt-bots (after the npt-bots row)
Old:
```text
| npt-lab | subcommands `scenario`
```

New:
```text
| npt-bots (library) | U4f: `Behaviour`'s `controlled`, `with_press_every`, `never_fires`, `never_acks`, `with_points`, `lag_switch` and `take_bursts`, `fast_clock` and `fast_left_out`, `outgoing` (the cheats' hook on the endpoint's datagrams), and, built ahead for U4f2, `forger` and `take_forged` | u4f-brief.md 2.2 (the controller runs' command rule), 2.6 (the cheats and the never-acker), U4F-Q4 (b) (the no-fire clock script), U4F-Q11 (i) (a) as the advisor's U4f build-1 ruling F4 set it (`with_press_every`); `outgoing` and `take_bursts` beyond 2.13's words (the host order: C-U6-18); `fast_left_out` written by the lab as `event.fast_clock.left_out_count` (its FP12); C-U4F2-2 (`forger`, `take_forged`); the advisor's U4f round-1 ruling (EA-U4F-1, EA-U4F-2) |
| npt-lab | subcommands `scenario`
```

---

## Edit 7: plan.md §12.1, the five later carries in the U5, U6 and U7 rows
File: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/plan.md

Why: advisor-u4f-r1.md:137-154 ("to be written in plan.md's rows by the main session (the next free numbers
there)"); the numbers checked free (above). Each carry's text is the ruling's, word for word, in quotation marks; for
C-U7-4 the ruling's four sub-bullets are joined inline, each already ending in its own semicolon. The short labels
before each quotation follow the rows' own form (C-U6-12's in the U6 row, C-U7-1's in the U7 row).

### Edit 7a: the U5 row (C-U5-1)
Old:
```text
| U5 | S | Fuzzing (-34); the dependency and swap checks (-35); the audits (§4.8) |
```

New:
```text
| U5 | S | Fuzzing (-34); the dependency and swap checks (-35); the audits (§4.8). **C-U5-1** (the advisor's U4f round-1 ruling, 2026-11-19, advisor-u4f-r1.md:138-140; RR-U4F-4): "an Input fuzz target through `Room::receive` under options A and B (seeded, in the debug test build). It asserts no panic, exactly one counter per offered command, and the bounds: held at most 8, refusals at most 4,096 a tick, kept window at most 16." |
```

### Edit 7b: the U6 row's end (C-U6-17, C-U6-18)
Old:
```text
with the first change to npt-server's pre-`Join` path (TE2-L2) |
```

New:
```text
with the first change to npt-server's pre-`Join` path (TE2-L2)<br>**U4f's carries (the advisor's U4f round-1 ruling, 2026-11-19, advisor-u4f-r1.md:141-145):**<br>• **C-U6-17 the controller's cap off L1 (NC-U4F-3):** "before an honest client runs on a link other than L1, the controller's cap holds under both options. The form: the stamp capped at the earliest arrival's next tick + 7 (`next_tick_at(now + lo_rtt / 2) + 7`). Its test: one 600 ms pong and a 10 ms path, under option B's buffer of 8, with no kind-10 refusal on the honest client."<br>• **C-U6-18 a Behaviour's host order (EA-U4F-9):** "a Behaviour's host order (input, the endpoint's step, `outgoing`) gets one entry point, or module text that states the order as the contract, before npt-bots and npt-load host Behaviours." |
```

### Edit 7c: the U7 row's end (C-U7-3, C-U7-4)
"Mustafa's go" in the old text is the row's own wording (plan text, not his words); it stays unchanged.

Old:
```text
a CI home is .github/ work, risk tier 2, Mustafa's go) |
```

New:
```text
a CI home is .github/ work, risk tier 2, Mustafa's go). **C-U7-3** (the advisor's U4f round-1 ruling, 2026-11-19, advisor-u4f-r1.md:146-148; NC-U4F-4, and EA-U4F-5's cap source): "the endpoint takes the previous step's send time, and ClockSync stamps each `Ping` with it (NC-U4F-4). The controller's cap gets a source the browser client can reach. My lean: a `JoinAccepted` field, since the bound is room data (U4F-Q8 (i) (b)); a wire change goes through the catalogue first." **C-U7-4** (the same ruling, advisor-u4f-r1.md:149-154; RR-U4F-2, RR-U4F-5 (3) and (4), EA-U4F-1): "ClockSync's hardening when it becomes the Worker's: the offset clamped, not mapped to i64::MAX (RR-U4F-2), with a unit case at server_ns = u64::MAX; the next ping time in closed form (RR-U4F-5 (3)); `period_ns.max(1)` or a clamped tick rate (RR-U4F-5 (4)); its public surface trimmed: `ClockSync::new`, `period_ns`, `unknown_pongs`, `next_tick_at` and `one_way_hi_ns` each made crate-private, removed or given a consumer, and `ping_every_ns` removed (EA-U4F-1)." |
```

---

## Edit 8: the §4.1 rows left unchanged (no edit; for the record)
- npt-server (u4-build-plan.md:195): no new public item; the room's `Pong` sets the new field (server/src/pump.rs).
- npt-lab (:198): binary settings only, no library item (u4f-brief.md 2.13); its new settings and suites are in USAGE
  (lab/src/scenario.rs:1985-1993) and its new header params in SERIES.md:372-391.

---

## Edit 9 (proposed, not in M8's list): u4s-carries.md, a U4f2 section by number
File: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4s-carries.md

Why: the stop ruling moves U4f's not-met -21 carries to U4f2 by number (advisor-u4f-stop.md:97), and briefs take their
carries from this file (the card's standing rule). This points to them and to the ruling's U4f2 carries, without
retyping either; if the main session prefers U4f2's brief alone to hold them, skip it.

Old:
```text
## U4g (prediction, interpolation and events)
```

New:
```text
## U4f2 (-21's forged-command suite and -21's runs; after U4f)
- From U4f's section above, by number (advisor-u4f-stop.md:97; u4f-report.md, "Carries"): 46's lab half (-21's twins,
  u4f-brief.md 2.12, f16), and 86 for the forged suite's option-A rows (5.3's p1, p7, p14, p17) and -21's 5.3 record
  from U4f2's merge commit. The rest of 46, 80 and 86 is met by U4f.
- The advisor's U4f carries for U4f2, word for word at advisor-u4f-r1.md:69-85: C-U4F2-1 (SEC-U4F-1's parser half),
  C-U4F2-SEC-1, C-U4F2-2 (the built-ahead items), C-U4F2-3 (`CommandRefusal`), C-U4F2-4 (FP8's line reworded); and
  ADV-U4FP-3, beside C-U4F2-2 (advisor-u4f-pass.md:95-98).

## U4g (prediction, interpolation and events)
```

---

## Edit 10 (proposed, not in M8's list): u4-build-plan.md §5's Lanes line, U4f2 in the order
File: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4-build-plan.md

Why: U4f2 opens npt-room (C-U4F2-3) and client/src/lib.rs (C-U4F2-2, ADV-U4FP-3), the files this line keeps to one
lane at a time; the card's next steps already run U4f, then U4f2, then U4g. U4g's own entry ("depends on U4f") is left
as it is: no ruling moves it.

Old:
```text
- **Then** U4r (after U4c, U4d2, U4e, U4j) and its runs; then U4f, U4g and U4h in order (they share the room's and the
  client's files: never two at once).
```

New:
```text
- **Then** U4r (after U4c, U4d2, U4e, U4j) and its runs; then U4f, U4f2, U4g and U4h in order (they share the room's
  and the client's files: never two at once).
```

---

## Not drafted here (seen while drafting)
- u4-build-plan.md §0's "twelve sub-units" list (:28-35) does not name U4f2. Left: M8 does not ask for it.
- A1's reading of ADR 0001:1611's "moves" is not yet in u4-build-plan.md §10's readings list (:971-984); the card holds
  it under "Open findings", for the readings PR. Left: not M8's.
- The card's "380,276 B" (ADV-U4FP-1) is the card's next save, not this commit's plan text.
