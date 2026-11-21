# U3c last fix round, agent C1b: npt-net's DelayStream (R1, G-PRE-WAKE, G-PRE-PANIC, G-F10, G-NC4, G-FH, G-NC8, G-NC2, the split's memory)

Worktree /home/user/Shamble-Frontlines/.claude/worktrees/wf_ac5e4575-9da-1, at b2e879b (the spike branch
claude/netcode-proof-test-plan-hcky5g, fast-forwarded). Build dir: CARGO_TARGET_DIR
/home/user/Shamble-Frontlines/.gate/u3v3-harness-check-target only (shared with C2b: touch this tree's sources before any
run that must use it, and check each run's test count; G-METHOD). Not touched (C2b's): check.mjs, check.test.mjs, the
workspace Cargo.toml's lint tables, crates/tcpinfo. Nothing committed. No iptables, tc or long #[ignore]d runs.

Inputs: u3c-fix-review-result.txt (R1, G-F10, G-NC4, G-NC2, G-FH, G-NC8, G-PRE-WAKE, G-PRE-PANIC), u3c-fix-review-c1.md
(probes 1-4), advisor-u3c-plan.md "Side rulings: U3c's last fix round" (binding: R1's test = probe 1, fails reverted;
probe 2's timings recorded; G-PRE-WAKE in the same change; G-PRE-PANIC tested with a panicking S; split memory (b)).

## Plan (one block per item; a Progress line after each)
1. **R1 + G-PRE-WAKE (one change, delay.rs).**
   - Pump::poll: the down loop remembers whether it ended on S's Pending (S then owes the wake). The closing block,
     under the lock, owes the pump a wake of its own (`cx.waker().wake_by_ref()` after the lock is released) when a
     released down record is left unwritten and S owes none: any record released after the down loop (the up loop's
     steps, R1) is written at once by the next pass. Kept: the up loop's `[UP]` for the reader.
   - current() and add_burst: one helper (`Conn::app_step`) steps the model and owes the pump a wake for a released
     down record (as today) or when the step lifted the up room to what the pump waits for, from below it: the front
     pending record's length + 22 B when the scanner holds one, else the read floor (an acknowledgement at the client's
     end reopened the room; G-PRE-WAKE).
   - Tests (hand-polled pump, current_thread, judged on model state and wake stamps, never wall-clock thresholds):
     (a) probe 1: S's write sleeps 30 ms, fixed(400), A then B 20 ms apart, the pump polled just after A's delivery:
     after the run, any released down record left unwritten must have a wake of the pump's waker stamped at or after
     its release step; vacuity: the record was released by the pump's up-loop step (By::Mid), retried up to 5 times
     on a stalled thread; the next pass writes it. (b) G-PRE-WAKE: the up room filled by a flood read by one pump pass,
     the released records read by tungstenite's side, then the test's own current() steps the acknowledgements: on the
     call whose step lifts the room across what the pump waits for, that call woke the pump (test-thread wakes only).
   - Probe 2 (spawned pump, slow S): timings recorded (temporary test, removed after).
2. **G-PRE-PANIC.** Pump::drop sets `failed = Other` when `std::thread::panicking()` (tokio drops a panicked task's
   future inside its panic guard). Test: a spawned pump over an S that panics in poll_write; tungstenite's side reads
   the request, then an error (kind Other), never Ok(0).
3. **G-F10.** Test: probe 4's shape (fixed(20); the request + 6,000 frames of 10 B; nobody reads until the up room is
   below the floor (model state); then read to EOF; every byte in order). Vacuity: a cfg(test) count of move_up's
   compactions that shifted pending bounds is > 0.
4. **G-NC4.** one_record_per_write's Pending write (owed 10,044 B), then a burst ending soon: the return records owed
   10,044. Test (c): owed == 0 at each faithful return.
5. **G-FH.** max[DOWN] >= BUDGET − 22 x 10 − 1,024 (its derivation: <= 22 B left free, at most 9 records' 22 B, S's
   duplex at most 1 KB).
6. **G-NC8.** The model stepped 10 s ahead of the clock, not 10 ms.
7. **G-NC2.** A cfg(test) check in held_now: down (and up) plaintext held <= 64 KB + the acknowledged prefix of the
   front unreleased record, every sample; its largest excess over 64 KB kept. Test: P2 with its loss, a writer that
   keeps the budget full with 16 KB records, a client draining S; runs until held > 64 KB was seen (model state; hang
   guard); asserts no sample over the bound, an excess > 0 and < one record, and first losses > 0.
8. **The split's memory (the advisor's (b)).** stats::Pooled: a histogram of relaxed per-bucket atomics (count, sum,
   max), no lock, no allocation on record; its summary copies the buckets into a Lateness and goes through the same
   `summarize`. A process-wide pool (OnceLock, allocated at the first wrap, never on the release path): per direction
   five parts, three kinds, and the per-release sum check's violations. Conn keeps its two total `Running`s (and a
   cfg(test) mirror of its violations). Rows: the split leaves the per-second window and the summary; a run-level
   reading (`event.ws_split.<name>`, pub(crate); U6 carry: npt-server's writer) with the violations rows. Header: 
   ws_conn_fixed_bytes back to 309,583 B; new ws_split_fixed_bytes (per process). SERIES.md rows moved and reworded.
   Identity test: per release from the log, the per-connection violation count equal to the log's broken releases
   (0 with no bite, > 0 with each bite), the pool's counts grown by at least this connection's. Stats test: a pooled
   histogram fed from 4 threads equals a Lateness fed the same samples. loopback() prints the pooled parts.
9. **Bites** (each fix reverted in place, its test run, restored): R1, G-PRE-WAKE, G-PRE-PANIC, G-F10 (shift dropped),
   G-NC4 (owed 0), G-FH (Pending at 20 KB free), G-NC8 (by reading: a 15 ms stall), G-NC2 (bound tightened), split
   (sum check removed; pool not recorded).
10. fmt, clippy -D warnings (both builds), check.mjs 9/9 twice.

## Progress
- Item 1 done (code + tests): Pump::poll's closing block self-wakes when a released down record is left unwritten and S
  owes no write wake (`s_owes_write`); `Conn::up_need` + `Conn::app_step` used by current() and add_burst (wake on a
  released down record or an up room lifted across what the pump waits for). Tests
  `a_down_record_released_in_a_pump_pass_is_written_or_owed_a_wake` (attempt 1: B released by the up loop, self-wake
  73 us later, inside the pass) and `an_acknowledgement_an_application_task_steps_that_reopens_the_up_room_wakes_the_pump`
  (room 176 -> 5,116 B across 549 B, one wake) pass. Debug build of the lib tests: 8 s. Bites and probe 2 come later.
- Items 2-7 done (code + tests), all pass (debug): G-PRE-PANIC (Pump::drop sets failed = Other while panicking; test
  `a_pump_that_panics_gives_the_reader_an_error_never_a_clean_eof`); G-F10 (`Life.shifted`; test
  `tiny_frames_that_fill_the_up_room_drain_whole_and_in_order`: 96,148 of 96,148 B, 17 compactions with records
  pending); G-NC4 (one_record_per_write: return owed 10,044 B; (c) owed 0 at faithful returns); G-FH (down_least =
  BUDGET − 220 − 1,024 = 64,292 B; seen 65,360); G-NC8 (10 s ahead); G-NC2 (`Conn::check_held_bound` in held_now,
  `Life.held_over_bound` / `held_beyond_budget`; test `under_loss_the_held_bytes_stay_within_64_kb_and_the_front_records_acknowledged_prefix`:
  100 writes, excess 14,326 B, 0 samples above the bound, 28 first losses).
