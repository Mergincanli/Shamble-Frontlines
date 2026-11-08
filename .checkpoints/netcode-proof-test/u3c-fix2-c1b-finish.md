# U3c last fix round, C1b finish (npt-net DelayStream): notes

Worktree /home/user/Shamble-Frontlines/.claude/worktrees/wf_10e244da-89a-1, at b5e40438d2fdcbe8e6c499f85a164e07a398fe73
(the spike branch claude/netcode-proof-test-plan-hcky5g, fast-forwarded from 626df67). Build dir: CARGO_TARGET_DIR
/home/user/Shamble-Frontlines/.gate/u3v3-harness-check-target only (shared: touch this tree's sources before a run that
must use it, and check each run's test count; G-METHOD). Toolchain cargo +1.99.1. Nothing committed. No iptables, tc or
long #[ignore]d runs. No new public symbol; no unsafe or libc in npt-net.

## Start
- `git merge --ff-only b5e4043`: fast-forwarded; `git log -1` = b5e40438d2fdcbe8e6c499f85a164e07a398fe73.
- `git apply --exclude='.checkpoints/*' .checkpoints/netcode-proof-test/wip/u3c-fix2-wt1.patch`: `--check` clean, applied
  clean. Touched: crates/net/src/l2/{delay.rs, delay_names.rs, delay_tests.rs, stats.rs}, marks/SERIES.md
  (995 insertions, 237 deletions).
- The first agent's plan (u3c-fix2-c1b.md) does carry Progress lines for items 1-7 (code and tests done, debug runs
  pass); none for items 8-10. Its scratch folder holds bites-run1.txt (10 bites, all BITE, on its own tree
  wf_ac5e4575-9da-1), probe2.txt (5 runs FIXED and R1-REVERTED) and the scripts bites.py, probe2.sh, series_split.py
  (all hard-coded to its worktree path).

## Applied diff vs its plan, item by item
1. R1 + G-PRE-WAKE: PRESENT. Pump::poll keeps `s_owes_write` (set only on the down loop's S Pending); the closing block
   computes `again = released[DOWN] > 0 && !s_owes_write` under the lock and calls `cx.waker().wake_by_ref()` after
   `wake_all`, outside it. The down loop breaks only on `released[DOWN] == 0` or S's Pending (or returns), so a released
   record left at the end was released after the loop (up loop's By::Mid step, or another task's step) or S owes the
   wake: no spin, no lost write. `Conn::up_need` (front pending record's end + 22 B, else the read floor) and
   `Conn::app_step` (wake on got[DOWN] or room crossing up_need from below) replace `step_to(t, By::App)` in add_burst and
   current(). Tests present: `a_down_record_released_in_a_pump_pass_is_written_or_owed_a_wake` (probe 1, hand-polled,
   SlowS 30 ms, fixed(400), records 20 ms apart, StampWake; vacuity: B's kind MID, 5 retries) and
   `an_acknowledgement_an_application_task_steps_that_reopens_the_up_room_wakes_the_pump` (hand-polled, ProbeWake counts
   test-thread wakes, 3 retries). Probe 2 timings: in its scratch probe2.txt (its tree), the probe test itself removed.
2. G-PRE-PANIC: PRESENT. Pump::drop sets failed = Other while `std::thread::panicking()` and failed is None. Test
   `a_pump_that_panics_gives_the_reader_an_error_never_a_clean_eof` (PanicS panics at its first write; read gives
   Other, never Ok(0); a later write refused with Other).
3. G-F10: PRESENT. `Life.shifted` (cfg(test)) counted in move_up when bounds stay pending after the shift. Test
   `tiny_frames_that_fill_the_up_room_drain_whole_and_in_order` (fixed(20), request + 6,000 frames of 10 B, client shut,
   wait for up_room < read_floor, then read to EOF; got == sent; shifted > 0).
4. G-NC4: PRESENT. one_record_per_write now takes a return during the Pending write (owed 10,044, outstanding BUDGET,
   at_ns start + 1 ms); test (c) asserts owed == 0 at each faithful return.
5. G-FH: PRESENT. `down_least = BUDGET - OVERHEAD * 10 - 1_024` (64,292 B), comment carries the 65,316 B derivation.
6. G-NC8: PRESENT. The model stepped `10 * SEC` ahead of the clock; doc updated.
7. G-NC2: PRESENT. `Conn::check_held_bound` (cfg(test)) in held_now: per direction held <= budget + acknowledged prefix
   of the front unreleased record; counts `held_over_bound`, keeps `held_beyond_budget`. Test
   `under_loss_the_held_bytes_stay_within_64_kb_and_the_front_records_acknowledged_prefix` (P2, 16 KB writes, a draining
   client; until >= 100 writes, excess > 0 and first losses > 0; asserts 0 samples over, excess < 16,406 B).
8. Split memory (b): PRESENT. stats::Pooled (relaxed per-bucket count/sum atomics + fetch_max, Box<[AtomicU64]>
   allocated at new(), summary copies into a Lateness and uses the same summarize); `static POOL: OnceLock<Pool>` created
   in build (before any release); per direction 5 parts, 3 kinds, violations. Conn keeps `late: [Running; 2]` only (and
   a cfg(test) `split_violations`). late() records into the pool with no lock of its own (atomics under the connection
   lock: no nested lock, no await, no allocation) and counts a violation when the parts do not sum to L. Rows: the split
   left `names::window` (per second and ws_summary); new `names::split_rows` + `SPLIT_VIOLATIONS` + `SPLIT_ROWS`;
   `DelayStream::split_rows()` pub(crate) (no new public symbol). Header: `ws_split_fixed_bytes` added (HEADER_NUMBERS 22
   -> 23); its_rows asserts 309,583 and 1,350,400. SERIES.md: the 70 split rows moved from `second.v.` to a new
   `event.ws_split` table, 2 violations rows added (72 rows there, 0 left under second.v.), both header rows updated,
   prose updated. Tests: stats `a_pooled_histogram_fed_from_threads_equals_one_fed_the_same_samples`; identity test now
   asserts per-connection violations == the log's broken releases (0 with no bite, > 0 under each bite) and `pool_grew`.
   A grep finds no stale reference (1,731,983 or `second.v.ws_late_*` split names) outside SERIES's own new rows.
9. Bites: run by the first agent on its own tree (bites-run1.txt: R1, G-PRE-WAKE, G-PRE-PANIC, G-F10, G-NC4, G-FH,
   G-NC2, SPLIT-CHECK, SPLIT-POOL, POOLED-ATOMIC all BITE; G-NC8 by reading only). To redo here, on this tree.
10. fmt, clippy (both builds), check.mjs 9/9 twice: NOT DONE. To do here.

## Progress
- Build on this tree (sources' mtimes refreshed first, G-METHOD; the guard refuses a touch of the Cargo.toml files, so
  .rs only): npt-net lib 151 passed, 0 failed, 11 ignored; 162 listed (the first agent's runs also show 162).
- Item 9 (bites) DONE on this tree: script u3c-fix2-c1b-finish/bites.py, output u3c-fix2-c1b-finish/bites-run.txt.
  Each bite reverts its fix in place, runs only its test(s) with `--exact`, restores; sha256 of every l2/*.rs equal
  before and after. Results: R1 BITES (:1995, "no wake owed (R1)", past the MID vacuity check); G-PRE-WAKE BITES
  (:2095, woke 0 not 1); G-PRE-PANIC BITES (:2317, "a panicked pump read as a clean EOF"); G-F10 BITES (pump panics on
  a slice index, reader hang/EOF at :2192); G-NC4 (owed forced to 0) BITES (:729, 0 vs 10,044); G-NC4-NONZERO (owed +1)
  BITES in test (c) (:3256, "owed at the return"); G-FH (Pending at 20 KB free) BITES (:791, down_least); G-NC8: the
  control (a 15 ms test-thread stall before "+1 accepted", 10 s margin kept) HOLDS, the same stall with the old 10 ms
  margin BITES (:1259); G-NC2 (bound tightened to the budget) BITES (:2249, [366, 0] samples over); SPLIT-CHECK (the
  per-release check removed) BITES (:2900, violations [0, 0] vs the log's [59, 60]); SPLIT-POOL (the hop part not
  recorded in the pool) BITES (:2932, pool_grew); POOLED-ATOMIC (sum's fetch_add made a racy load/store) BITES
  (stats.rs:678, excess differs); SPLIT-SIZE (pool heap counted as 12 histograms) BITES (:2551).
- Item 1's probe 2 DONE on this tree (temporary test u3c-fix2-c1b-finish/probe2.rs appended, run 5 times per state by
  probe2.sh, then removed; l2/*.rs sha256 equal to before; output u3c-fix2-c1b-finish/probe2.txt). Spawned pump over
  SlowS, fixed(200) (100 ms one way), A then B of 1,000 B; "B late" = B's lateness (S's write return - B's model
  delivery stamp). FIXED: S 30 ms / gap 20 ms 40,482-40,674 us; S 2 ms / gap 1 ms 3,413-3,627 us; S 0 215-305 us.
  R1 REVERTED: 130,397-132,273 us; 102,382-102,478 us; S 0 217-880 us. Decomposed: B's due fell 1.15-1.2 ms after A's
  (20.2 ms in the 30 ms case), inside A's write; with the fix B's write began 0.2-0.4 ms after A's write returned in
  every run (that is the sleep's own overshoot), against 89-99 ms with R1 reverted. The critic's "about 2.6 ms / 39.9
  ms" (and the first agent's 2.2-2.5 / 39.2-40.5 ms) differ from 3.4-3.6 / 40.5-40.7 ms only by where B's due falls
  inside A's write (the first agent's B came due 2.2 ms after A, at A's return; the critic's spacing is not recorded;
  here 1.2 ms after, so B also waits A's last 1.2-1.4 ms): B = rest of A's write + B's own write, no hold. Back at
  pre-fix behaviour.
- fmt: `cargo fmt --all --check` found unformatted lines in the new tests (delay_tests.rs: pool_grew's array, long
  println!/assert! lines in the R1 and G-PRE-WAKE tests); `cargo fmt --all` applied (only npt-net l2 files and
  SERIES.md are modified in the tree, as before); `--check` then clean. clippy `--workspace --all-targets -D warnings`
  clean on the measured build and with `--all-features`.
- Item 1 (R1 + G-PRE-WAKE) DONE: code checked against every down-loop exit (above), both tests pass on this tree,
  both bite, probe 2 recorded.
- Item 2 (G-PRE-PANIC) DONE: test passes; bites (a panicked pump read as a clean EOF without the fix).
- Item 3 (G-F10) DONE: test passes (shifted > 0, every byte in order); bites (dropping the shift panics the pump).
- Item 4 (G-NC4) DONE: Return.owed read in one_record_per_write (10,044 at a Pending write) and in (c) (0 at every
  faithful return); forcing it to 0 bites the first, adding 1 bites (c).
- Item 5 (G-FH) DONE: down_least 64,292 B (the 65,316 B derivation less S's 1 KB); Pending at 20 KB free bites.
- Item 6 (G-NC8) DONE: 10 s margin; a 15 ms stall passes with it and fails with the old 10 ms.
- Item 7 (G-NC2) DONE: bound checked on every held-bytes sample under P2's loss; tightening it to the budget bites
  (366 samples over).
- Item 8 (the split's memory (b)) DONE: process pool of relaxed atomics, per-connection totals kept, violations
  counted per release; identity test + 2 bites + pool growth; stats Pooled test; header rows ws_conn_fixed_bytes
  309,583 B and ws_split_fixed_bytes 1,350,400 B (asserted in its_rows) and in SERIES.md; SPLIT-CHECK, SPLIT-POOL,
  POOLED-ATOMIC and SPLIT-SIZE bite.
- Item 10 DONE. check.mjs (CARGO_TARGET_DIR the shared dir), twice in a row, all 9 steps PASS both times
  (u3c-fix2-c1b-finish/check-run1.txt, check-run2.txt): fmt, clippy, clippy-wasm32, test (measured build 258 passed,
  bites build 272 passed, 0 failed, 0 skips; 109.0 s then 74.7 s), deps, refs (441 files), marks (81), self (35/35),
  redact (10/10, 148 items, 0 findings). Run 2 after a fresh mtime refresh of every crate's .rs. G-METHOD: after each
  run, `cargo test -p npt-net --lib -- --list` (no rebuild, 0.05 s) lists 162 tests including the 6 new ones and no
  probe: the binary check.mjs ran is this tree's.
- Constraints: the diff adds no `pub` item (only pub(crate)), no `unsafe`, no `libc` (grep of added lines). Nothing
  committed. No iptables, tc or #[ignore]d run. Final diff saved: u3c-fix2-c1b-finish/final.patch (on b5e4043; 5 files,
  1,010 insertions, 237 deletions).
- Open for the main session: the critic pass on C1b's diff only (advisor's ruling (1)); C10's diagnostic after C1b;
  G-PLAN (u3c-plan.md 1.2 / 1.4 (c) / 1.5 text, R13's reading) is the main session's text work, not in this diff.

DONE
