# U4b build: the loop policy (npt-server's schedule.rs), the main session's record

Built 2026-11-09 about 22:05-22:15 UTC by the main session (u4-build-plan.md §5 U4b; Lanes: "U4b and U4j in the main
session"), in its own worktree `.gate/u4b-wt` (local branch u4b-build from 4b5812a, committed state: condition 10 (d)),
shared target `.gate/shared-target`, offline. Uncommitted there until its review and merge.

## What was built
- `crates/server/src/schedule.rs` (new): `Schedule`, `Due`, `Dropped`, `LoopCounters`, `due_ticks`; bites builds also
  `Schedule::with_bite` and `BITES` (PM-0004-67's three ids).
  - Tick k of a run falls due at base + ⌊k × 10⁹ / tick_hz⌋ ns (§4.3). The loop's clock zero is the loop's start, so
    the first run's base is 0 and `Schedule::new(tick_hz, catchup)` keeps the plan's two arguments.
  - A wake runs every owed tick up to the cap. Past the cap, the rest is dropped and counted, and the schedule
    rebases: the next run's tick 0 falls due when the tick after the last one owed would have. So after any wake the
    next tick is due after `now` (no tick left owed), and the loop is back on schedule at once.
  - Counters (ADR 0004 part 5: every overrun and dropped interval is a counter): `overruns` (wakes that found more
    than one tick due), `drops` (wakes that dropped time), `dropped_ticks`, `dropped_ns` (the dropped-time counter).
  - `next_due_ns()`: when the next tick falls due. The loop sets its timer from it, never from a count of its own
    (ADR 0004 part 5).
  - Worked in u128 with a closed form for the last due tick: ⌊((e + 1) × hz − 1) / 10⁹⌋ (⌊x⌋ ≤ e holds exactly when
    x < e + 1). A test checks it against a tick-by-tick reference over 120,000 wakes.
  - `Schedule::new` panics on a rate outside 1 to 1,000 (npt-sim's range for TICK_HZ) or a cap of 0. Both are
    programming errors: production callers pass npt-sim's constants.
- `crates/server/src/lib.rs`: `pub mod schedule;` and the root re-exports.
- `crates/server/clippy.toml` (new): the clock ban (§4.11, A14): `std::time::Instant`, `std::time::SystemTime`,
  `tokio::time::Instant` and their `now`. The two tokio entries carry `allow-invalid = true`, because npt-server has
  no direct tokio dependency yet, so clippy cannot resolve the path. **Known limit:** the tokio entries stay dormant
  until U6 names tokio as a direct dependency; the std bans are live now.
- `crates/server/tests/schedule.rs` (new): 10 tests, plus 6 more in bites builds (below).
- No manifest edit (U4s gave npt-server its `bites` feature), no check.mjs edit (§5 U4b), no other crate touched.

## Public items added (A19), npt-server
`schedule` (the module) · `Schedule` with `new`, `tick_hz`, `catchup`, `counters`, `next_due_ns` · `Due` (`to_run`,
`dropped`, `next`) · `Dropped` (`ticks`, `ns`) · `LoopCounters` (`overruns`, `drops`, `dropped_ticks`, `dropped_ns`) ·
`due_ticks` · bites builds only: `Schedule::with_bite`, `BITES` (also `schedule::BITES`).

Beyond §4.1's `Schedule` and `Due`:
- `Dropped` and `LoopCounters` are the field types those two need, as U4s's five field and config types were.
- `next_due_ns` is how the loop sets its timer.
- `tick_hz` and `catchup` are for carry 6's test.
- `counters` reads the counters.

None is for later.

## Tests (crates/server/tests/schedule.rs), on a fake clock at 30 Hz with catch-up 7 and at 60 Hz with 15
Each scenario is a check that returns what went wrong, so each bite is shown failing the check the clean schedule
passes.
1. `the_server_schedule_takes_its_rate_and_cap_from_npt_sim` (carry 6).
2. `on_schedule_one_tick_per_period`: 3 s of on-time wakes, one tick each, the schedule's own times, counters 0.
3. `a_late_wake_under_a_period_runs_one_tick`: half a period late, one tick, the next due time unmoved.
4. `a_200_ms_stall_runs_what_is_owed_and_is_back_on_schedule`: 6 (12 at 60 Hz), one overrun, no drop, then 1 s of
   one-tick wakes on the schedule's times.
5. `a_1_s_stall_runs_the_cap_drops_and_counts_the_rest_and_rebases`:
   - 30 Hz: runs 7, drops 23 ticks and 766,666,667 ns;
   - 60 Hz: runs 15, drops 45 ticks and 750,000,000 ns;
   - the counters hold the drop, the next tick is due when tick 31 (61) would have been, and the loop is on schedule
     after the rebase.
6. `never_more_than_the_cap_per_wake_and_the_counters_add_up`: 20,000 wakes at seeded random gaps of 0 to 3 s, at
   both rates, over 3 seeds. Every wake stays within the cap and leaves no tick owed. Every answer equals the
   tick-by-tick reference's, and the counters equal the wakes' own reports.
7. `an_early_wake_runs_nothing_and_the_answer_is_pure` (N4).
8-10. `a_zero_cap_is_refused`, `a_zero_rate_is_refused`, `a_rate_above_1000_is_refused` (should_panic, by message).

Bites builds add `bites::` with 6 tests:
- each bite fails its check;
- the ids are planted here and are in marks.json;
- another site's id or an unknown one gives `None`;
- each bite leaves the checks it does not target passing;
- a bitten instance and a clean one run side by side on two threads (T10).

## Bites (PM-0004-67's three; per instance, `Schedule::with_bite`; site `due_ticks`)
- **timer-period** ("counts ticks from its timer period instead of elapsed time"): one tick per wake, and the next
  tick falls one period after the wake. The rest of what is owed is lost uncounted, and lateness drifts every later
  tick.
- **uncapped-catchup**: every owed tick runs in one wake.
- **uncounted-drop**: the excess is dropped and the schedule rebases, but nothing is counted or reported.
- **In-place plants** (each clean test switched to its bite, one at a time; bites build; the file restored, sha256
  dfb7da2c… checked):
  - timer-period in test 4: `FAILED` "the wake at 200000000 ns ran 1 ticks, not 6";
  - uncounted-drop in test 5: `FAILED` "dropped Dropped { ticks: 0, ns: 0 }, not 23 ticks and 766666667 ns";
  - uncapped-catchup in test 6: `FAILED` "wake 0 ran 33 ticks, over the cap 7".

## Carries met (u4s-carries.md, U4b)
- **6 met:**
  - `Schedule::new` takes the rate and the cap;
  - the tests pass `npt_sim::loop_catchup_at(30)` and `loop_catchup_at(60)`, never a literal cap;
  - test 1 asserts that `Schedule::new(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP)` reads its cap back as
    `npt_sim::LOOP_CATCHUP`.
  - npt-server's and npt-lab's production callers do not exist yet (U4e, U4r, U6 build them), so the caller half of
    the rule binds them.
- **35 met:**
  - no tick count was needed outside npt-sim;
  - npt-server holds no const or static, and derives nothing from `TICK_HZ`;
  - the period comes from the rate passed in, tested at 30 and 60 Hz.

## Checks
- fmt clean (rustfmt applied: line wraps and import order only).
- clippy `-D warnings`, host, all targets: clean in both builds.
- Tests:
  - `cargo test -p npt-server`: 10 passed;
  - `--features bites`: 16 passed.
- check.mjs, all 10 steps, twice: running (results below when done).

## Reviews (plan §5 U4b): netcode-architect, rust-reviewer, test-engineer
They wait for free slots: the three wave-1 builders hold all 3 (CLAUDE.md, at most 3 agents).
