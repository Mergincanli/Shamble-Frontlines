# U3c fix agent C1: DelayStream, ws_gate, their tests, SERIES.md

Worktree /home/user/Shamble-Frontlines/.claude/worktrees/wf_3ea724fc-a63-1, base 95597ef705856fa7935dcb869542628810549976
(the spike branch claude/netcode-proof-test-plan-hcky5g). Build dir: .gate/u3v3-harness-check-target (shared with C2).
Not touched (agent C2's): check.mjs, check.test.mjs, crates/tcpinfo, the root script. Nothing committed.
Scratch outputs: .checkpoints/netcode-proof-test/u3c-fix-c1/.

## Plan (one block per finding; Progress line after each)
1. F1 + S7: DelayStream::add_burst refuses (InvalidInput) when start_ns.checked_add(len_ns) is None on DelayStream's
   clock; due() saturating on both terms; record_return saturating. Test with len u64::MAX (refused; no burst in the
   model, burst_ends unchanged) and the largest accepted length (due() is the burst end, never in the past). Doc line
   for U6: a client-triggered burst's start and length are range-checked by npt-server before add_burst (carry).
   NC-U3c-8 rides here: a start at or before the model's now is refused (documented).
2. F-L / NC-U3c-5 / F11: pub(crate) gate_inputs() -> ws_gate::Inputs: one lock, one step, four readings. Test (c) uses
   it.
3. F3 + F2 + F4 (+F5, F9 clamp, F10): Pump::poll holds the Conn lock only to stage and commit, never across S's
   poll_write, poll_flush, poll_read or poll_shutdown. Down: the front record staged into the pump's scratch once
   (kept across Pending and partial writes: F4), committed after. Up: S read into a pump-owned buffer, copied into the
   scanner's buffer under the lock, then the model is stepped to a fresh now before the records enter it (the
   model's now never goes back: another task may have stepped it meanwhile; NC-U3c-3's intake). current() and
   poll_shutdown wake only after the lock is released (a guard that wakes on drop). F5: a zero-capacity read with data
   released is Ready at once. F10: move_up compacts the scanner's buffer once per call. Bite for F3: a probe S whose
   poll_* try the Conn lock (manual pump polling, nothing else running): never held.
4. S2: the request scanner ends the head where httparse 1.10.1 (tungstenite 0.30's parser) does: leading empty lines
   skipped, then the first empty line (LF, or CR LF) after a non-empty line. Tests: bare LF, mixed, leading blank
   lines, a differential against tungstenite's own TryParse on seeded inputs, and the reproduced input (a 4,096 B
   head ending in "\n\n" then a 604 B frame) through DelayStream: closed Frame, counted, nothing entered.
5. F-K + NC-U3c-1 + NC-U3c-10 + F-B (test (c)): run until both returns have their first down byte and message (model
   state; a HANG guard in ticks), N recorded at each write (the formula's, from the gate's srtt), each return judged
   against N at the last write before it (model stamps from the log), vacuity guard outstanding > (N(srtt at the
   return) − 1) x SNAP for the faithful runs, and Return.at_ns == start + b + SEC.
6. F-B (burst test): logged start == requested clock start − origin. F-A: wrap's profile-burst conversion (ended,
   straddling, future; burst_ends; losses inside the clipped window; refusal at 65). F-G: client EOF ends the up
   direction after its last byte. F-M: slack 1 read back (header). F-R2: the 549 B floor (probe S). F-H: lower bounds
   in the held-bytes test. F-F: pin the limit close's discard of unreleased earlier records. F-C, F-O, F-I.
7. NC-U3c-4: Return gains ws_owed_bytes (c.owed at the return); SERIES row; the difference from r2:379 (3) recorded.
   NC-U3c-2, NC-U3c-3, NC-U3c-9, NC-U3c-11, F6 (docs; F6 also moves the /proc read out of the clock's state lock).
8. F7: the footprint. One Lateness per direction (the connection's), the per-second window kept as a difference
   against a sparse mark (only the buckets touched this second), one shared summary routine; else state it. Either
   way the footprint goes into the header rows and SERIES.md, with U6's accept cap noted.
9. F-P: the >= 180 band level in wt_tests.
10. check.mjs 9/9 twice; clippy -D warnings with and without features; cargo fmt --check.

## Notes for the main session (carry list)
**Plan text to correct (u3c-plan.md; the main session's words):**
- 1.4 (c) (NC-U3c-1): "at every return, outstanding <= N(srtt then) x size" becomes "at every return, outstanding <=
  N x size, N the one in force at the writer's last write before the return". Why: the gate guarantees outstanding <=
  N(srtt at a write) x size right after each write, and outstanding never grows between writes, but srtt (so N) can
  fall after the budget filled (a SACK-only or partial acknowledgement). R13's per-return reading on RS needs the same:
  record N (or srtt) at each write, judge each return against the last write's.
- 1.2 "Held bytes" (NC-U3c-2): down "at most 64 KB" becomes "at most 64 KB plus the acknowledged prefix of the front
  record not yet released (a loss inside it), so below 64 KB + 16,406 B"; up "at most 64 KB plus one frame and the
  request cap" gains "plus the acknowledged prefix of the front unreleased record (below 549 B, or 8,214 B for the
  request)".
- 1.5 loopback (NC-U3c-3): the identity is proven for the release half, up only; down is a bound. The intake (send to
  model write) is printed (p50 about 0.3 ms in debug), not judged; it goes into R1's self-test split beside the clock
  wake, the pump start and S's poll_write return.
- 1.2 per-return record (NC-U3c-4): it gains `owed` (ws_owed_bytes). r2:379 (3)'s outstanding_bytes = the record's
  outstanding_bytes + ws_owed_bytes + what WsServer holds itself; SERIES.md says so.

**U6 carries (for the checkpoint's carried list and plan §12.1's U6 row):**
- S7 / F1: npt-server range-checks the per-session B1 control a client triggers before calling add_burst: a stated
  maximum burst length and a start within the session. add_burst itself now refuses start + length beyond u64::MAX
  and a start at or before the model's now.
- F11: WsServer reads `gate_inputs()` once per tick on the delayed path (one lock, one step).
- F6: `header_numbers` blocks (the clock thread answers): call it through spawn_blocking.
- F7 / S3: the accept cap. `ws_conn_fixed_bytes` = 309,583 B per connection at the defaults (was about 565 KB), plus
  the held bytes (below 81,942 B each way), plus a 135,040 B dense mark per direction once a second's lateness touches
  more than 256 buckets, plus tungstenite's own buffers (its 128 KiB read buffer, S3).

**Behaviour changes a reviewer should know:**
- Up records now enter the model at a fresh now taken after S's read (they entered at the pump run's start before),
  so the intake no longer includes the down writes done earlier in the run (NC-U3c-3). The replay test still holds.
- The request scanner ends the head where httparse does (S2): bare LF, mixed line ends, leading empty lines.
- add_burst refuses a start at the model's now (NC-U3c-8); tests always asked for now + 1 ms, unchanged.
- Reads of S go through a 527 B pump buffer (one largest frame), so the 8,192 B request takes up to 16 reads.

## Progress
- Baseline (95597ef, debug, alone, 3 runs) short loopback lateness: down p50 0.460/0.460/0.422 ms, p99 0.887/0.936/2.391
  ms, p99.9 = max 1.716/7.400/2.866 ms; up p50 0.590/0.570/0.557 ms, p99 2.063/2.016/2.818 ms, max 4.783/4.598/4.657 ms
  (u3c-fix-c1/baseline-loopback-{1,2,3}.txt).
- Code done (first pass; the l2::delay, stats, ws_frame and ws_gate tests, 47, pass): F1+S7+NC-U3c-8, gate_inputs, the
  Pump rewrite (F3, F4, F2, F5, F9's clamp, F10), S2's scanner, F7's Running, NC-U3c-4's ws_owed_bytes, F6 (doc, and
  the clock's /proc read out of its lock), the SERIES rows, the HANDSHAKE text.
- Progress: F1 + S7 fixed. add_burst refuses start + len > u64::MAX (and start <= the model's now, NC-U3c-8); due() and
  record_return saturate. Test a_burst_past_the_clocks_range_is_refused_and_never_wraps: u64::MAX and u64::MAX − start
  + 1 refused with the state unchanged; u64::MAX − start accepted, due() == u64::MAX, the idle pump runs <= 1 time; a
  start at the model's now refused, +1 ns accepted. U6 carry in add_burst's doc (S7: npt-server range-checks a
  client-triggered burst's start and length).
- Progress: F-L / NC-U3c-5 / F11 fixed: pub(crate) gate_inputs() (one lock, one step); test (c) uses it.
- Progress: S2 fixed. The scanner follows httparse's line rules. Tests the_head_ends_where_tungstenite_ends_it and
  the_head_matches_tungstenite_on_seeded_inputs (20,000 inputs: 14,997 end at the same byte, 4,746 refused by
  tungstenite at the scanner's end, 257 not ended; 0 mismatches); each_limit gains the reproduced 4,096 B "\n\n" head
  with a 604 B frame (closed Frame, 4,118 B entered), a mixed head and a leading-blank-lines head.
- Progress: F-K + NC-U3c-1 + NC-U3c-10 + F-B (in (c)) fixed. Test (c) runs to model state (both returns' first byte and
  message down; hang guard 900 ticks), pairs N at each write with its logged record, judges against N at the last
  write before the return, guards against vacuity (outstanding > (N(srtt at the return) − 1) x SNAP) and checks
  at_ns == start + b + 1 s. Run 1: A 218 ticks, B 183, x2 200, unsent 184; faithful 7,350 vs 7,350 B (N 7 and 7) at
  all four returns; x2 14,700 and 16,800 B; unsent 15,750 and 15,750 B.
- Progress: F3 + F2 + F4 fixed (the pump never holds the lock across S; `Stepped` wakes after unlocking;
  poll_shutdown wakes after unlocking; the staged record is kept across Pending), with F5, F10 and F9's clamp. Test
  the_pump_never_calls_s_under_the_lock_and_wakes_come_after_it (a hand-polled pump: S never under the lock, staged
  bytes not copied again, the split counted, current()'s and poll_shutdown's wakes after the lock, F-R2's floor).
- After (debug, alone, 3 runs) short loopback lateness: down p50 0.431/0.444/0.388 ms, p99 0.725/2.904/1.244 ms, max
  0.885/4.175/3.867 ms; up p50 0.553/0.413/0.440 ms, p99 1.244/5.383/1.941 ms, max 2.555/9.140/2.805 ms
  (u3c-fix-c1/after-loopback-debug.txt). At n = 301 per direction no change is shown either way (agent C2's builds share
  the machine). The up intake (model write − send) is printed now: p50 0.29-0.30 ms (NC-U3c-3).
- Bites (u3c-fix-c1/bites.py, each mutation applied, its test run, the file restored; output bites-run1.txt): 15 of 15
  bite: F1 range refusal, NC-U3c-8, F2 (current's wakes before the unlock; poll_shutdown's under the lock), F3 (S's
  write under the lock), F4 (re-copy each attempt), F-R2 (no floor), F5, S2 (no bare-LF end; leading lines not
  skipped), F-B (no origin, burst test alone), F-A (ends on the clock's axis), F-G (EOF while records wait), F7 (mark
  never set; window stays dense).
- Progress: F7 fixed. stats::Running (one histogram per direction, the window as a difference against a sparse mark,
  dense fallback beyond 256 buckets; one summarize() shared with Lateness, whose own tests still pass); test
  a_running_window_equals_a_fresh_histogram_of_its_samples (exact equality over sparse, dense and empty windows). Header
  row ws_conn_fixed_bytes = 309,583 B (its_rows asserts it), SERIES.md row and module doc state it with U6's cap.
- Progress: NC-U3c-4 fixed: Return.owed, ws_owed_bytes row, the difference from r2:379 (3) recorded in SERIES.md.
- Progress: NC-U3c-2, NC-U3c-3 (doc + intake print + fresh now), NC-U3c-9, NC-U3c-11 (HANDSHAKE text, SERIES row), F6
  (doc; the clock's /proc read outside its state lock; its_rows calls header_numbers through spawn_blocking) fixed.
- Progress: F-B (burst test: logged start == asked − origin), F-A (wrap_moves_the_profiles_bursts_to_the_models_axis),
  F-G (an_eof_from_s_ends_the_up_direction_after_its_last_byte), F-M (its_rows: slack 1 now and at start, set 1), F-H
  (held-bytes lower bounds: down >= 64 KB − 16,384 B, up >= 62,237 B; run: 65,360 and 62,632), F-F (limit_case's
  identity handed == released; a 10 s one-way case pins the discard: entered 170 B, released 0, handed 0), F-C (ring
  named directly), F-O (doc and name: ..._no_byte_seen_before_its_release), F-I (one-record test at a 20 s round
  trip), F-P (wt_tests: P(X >= 21) = 9.3e-10 for Binomial(200, 0.02)), F5 (zero-capacity read test) fixed.
- clippy -D warnings with and without --all-features: clean; cargo fmt --all --check: clean.
