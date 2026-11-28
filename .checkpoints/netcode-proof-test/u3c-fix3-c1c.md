# U3c final small fix, C1c (npt-net DelayStream): notes

Worktree /home/user/Shamble-Frontlines/.claude/worktrees/wf_5c9f078a-da1-1, at c37dfe14f222d62f429785498e189262ca7a8f7b
(fast-forwarded from 626df67; `git log -1` = c37dfe1). Build dir CARGO_TARGET_DIR
/home/user/Shamble-Frontlines/.gate/u3v3-harness-check-target only; toolchain cargo +1.99.1. Nothing committed. No
iptables, tc, network or #[ignore]d runs. No new public symbol; no unsafe or libc in npt-net. Files written with
Write/Edit only. Scratch: .checkpoints/netcode-proof-test/u3c-fix3-c1c/.

## Plan
0. Touch every .rs under spikes/netcode-proof/crates (mtime only), df -h, build + record the baseline test count.
1. MEDIUM R1 no-spin: bound life().polls over a_full_budget's 1.5 s hold (< 2,000), measure polls fixed vs guard
   dropped, state the margin under load; revert `!s_owes_write` -> test fails.
2. LOW G-PRE-WAKE add_burst: a test through add_burst that fails when `if wake_pump` -> `if got[DOWN]`.
3. LOW G-PRE-PANIC other way: a test that a clean close and the shutdown-deadline end give reader and writer what they
   give today (not the panic marker's Other); `panicking()` made always true -> test fails.
4. INFO fix: poll_shutdown after a panicked pump returns Err(Other) (same kind as the reader); test; revert -> fails.
5. Doc: POOL's doc comment says a process measures one run (pool never reset; reset at run start is U6, plan 12.1).
6. fmt --check, clippy -D warnings (with/without --all-features), check.mjs 9/9 twice; test-count check each run.

## Progress
- Item 0 DONE: read the reviewer's C1b notes (verdict GAPS, 5 items) and the C1b finish notes. Touched all 62 .rs
  under crates (mtime only); df: 5.1 GB free. Baseline `cargo +1.99.1 test --locked -p npt-net --lib` (rebuilt
  npt_net-088534cbcaf56b90, 31.6 s): 151 passed, 0 failed, 11 ignored (162 listed, as C1b's runs).
- Code + tests written (not yet bitten):
  - item 4: delay.rs poll_shutdown: `down_done` -> Ok; else `pump_done` -> `failed.map_or(Ok(()), |k| Err(k.into()))`
    (a panicked pump's Other, an S error's own kind; a limit close or a cancelled pump: Ok as before); doc comment on
    it; module doc "never a clean EOF or a clean shutdown". Test: the panic test now also asserts flush and shutdown
    give Other (shutdown under a HANG timeout).
  - item 5: POOL's doc: never reset, a process measures one run; U6 carry (plan.md §12.1's own wording: one measured
    run per server process, or the pool reset at each run's start, stated in the run's header).
  - item 1: a_full_budget: `life().polls` before and after the 1.5 s hold; asserts < 2,000; prints the count.
  - item 2: the G-PRE-WAKE test's body moved into `an_application_step_that_reopens_the_up_room_wakes_the_pump(how,
    step)` (every assertion kept); the old test calls it with `outstanding_bytes()` (current), the new
    `an_acknowledgement_add_burst_steps_that_reopens_the_up_room_wakes_the_pump` with add_burst (1 ns burst 1,000 s
    ahead; asserts Ok while < 64 to come, refused after).
  - item 3: new `a_pump_that_ends_without_a_panic_never_gives_the_panic_error` + helper `ends_without_the_panic_error`:
    (a) clean close (request + client EOF read, 1,000 B down, shutdown Ok, client reads all + EOF), then the spawned
    pump's task aborted (is_cancelled); (b) the deadline end (512 B duplex, client never reads, 20,000 B, 300 ms).
    After each: read Ok(0), write BrokenPipe, flush Ok, shutdown Ok (today's behaviour, read from the code).
- First run (after `cargo fmt --all`): l2::delay* 34 passed, 0 failed, 1 ignored (164 listed = 162 + 2 new tests).
  a_full_budget polls over the hold, 3 runs: 130, 125, 120. Both G-PRE-WAKE tests pass at attempt 1 (room 176 ->
  5,116 B across 549 B).
- Hashes before the bites (after `cargo fmt --all`, `--check` clean): delay.rs 4bb0ebea...dc19f, delay_tests.rs
  adbd237a...a471. Each bite: Edit the fix out in place, run its test, Edit it back, hash checked.
- Item 1 DONE. BITE R1-SPIN (delay.rs:1880 `again = c.released[DOWN] > 0 && !s_owes_write` -> `c.released[DOWN] > 0`):
  a_full_budget FAILS at delay_tests.rs:792 "the pump spun while S owed it its wake": idle 194,705 / 174,534 / 187,831
  runs in the 1.5 s hold; under 8 CPU burners (node busy loops) on 4 cores 290,944 / 460,506 / 541,778. Restored
  (delay.rs hash 4bb0ebea... again). Fixed code: idle 130 / 125 / 120; under 8 burners (load avg 4.27) 122 / 125 /
  115 / 116 / 124; under 16 burners (load avg 6.25) 112 / 117 / 114. Margin: bound 2,000 is >= 15x the largest fixed
  count seen (130) and >= 87x below the smallest spin count (174,534); load did not raise the fixed count (the runs
  come from the model's events and S's wakes, not from CPU time).
- Item 2 DONE. BITE WAKE-ADDBURST-ONLY (delay.rs add_burst `if wake_pump` -> `if got[DOWN]`, the reviewer's exact
  revert), all l2::delay run: 33 passed, 1 FAILED: the new add_burst test at delay_tests.rs:2125 "the add_burst
  call's step that reopened the up room woke the pump once" left 0, right 1; the current() test still passes (its
  half untouched), as it should. Restored.
- Item 3 DONE. BITE PANIC-ALWAYS (Pump::drop `std::thread::panicking() && c.failed.is_none()` -> `c.failed.is_none()`),
  all l2::delay run: 33 passed, 1 FAILED: the new test at :2393 "a clean close, then the pump's task cancelled: the
  reader got Other (other error), not a clean EOF". Second run with the clean-close check skipped (a temporary line in
  the test, restored): FAILS at :2393 "the shutdown's deadline: the reader got Other". Both halves bite on their own.
  Restored (the Drop line and the test line).
- Item 4 DONE. BITE SHUTDOWN-OK (poll_shutdown back to `if c.down_done || c.pump_done { Ok }`), all l2::delay run: 33
  passed, 1 FAILED: the panic test at :2381 "a panicked pump's shutdown read as a clean one: ()" (the tokio-rt-worker
  panic at :2332 in the same output is PanicS's own planted panic). Restored.
- After every bite: delay.rs 4bb0ebea...dc19f and delay_tests.rs adbd237a...a471 (sha256) equal to before; fmt
  --check clean.
- Item 5 DONE (doc only; no test: no behaviour changed).
- Flakiness: the 5 changed tests (a_full_budget, both G-PRE-WAKE, both pump-end tests) 10/10 runs each, run together
  in one process each time; hold polls 116-141. Full npt-net lib (parallel): 153 passed, 0 failed, 11 ignored; 164
  listed (162 + 2 new); a_full_budget's hold 128 polls inside the full parallel suite.
- clippy `--workspace --all-targets -D warnings` clean on the measured build and with `--all-features`; `cargo fmt
  --all --check` clean.
- check.mjs run 1 (after a fresh mtime refresh of all 62 .rs; df 5.1 GB): all 9 steps PASS (99.6 s wall): fmt,
  clippy, clippy-wasm32, test (measured 260 passed, bites 274 passed, 0 failed, 0 skips; C1b's runs: 258 / 272, +2
  each = the 2 new tests), deps, refs (459 files), marks (81), self (35/35), redact (10/10, 148 items, 0 findings).
  `--list` after: 164 npt-net lib tests incl. both new ones (this tree's binary, G-METHOD).
- check.mjs run 2 (fresh mtime refresh; df 4.8 GB): all 9 steps PASS (81.6 s; 260 + 274 passed, 0 failed, 0 skips).
- Then two comment-only edits in delay_tests.rs (the G-PRE-WAKE helper's doc reflowed; the R1 bound's comment now
  carries C1c's measured ranges, 112-141 fixed and 170,000-540,000 bitten). delay_tests.rs now 17c63a39...8583
  (delay.rs unchanged 4bb0ebea...dc19f); fmt --check clean. So check.mjs is run twice more, on this final content.
- Item 6 DONE. Final content, check.mjs twice in a row, each after a fresh mtime refresh of all 62 .rs:
  run A all 9 PASS (88.2 s; test 260 + 274 passed, 0 failed, 0 skips; df 5.1 GB), run B all 9 PASS (87.9 s; same
  counts; df 4.8 GB). `--list` after each: 164. Then clippy `--workspace --all-targets -D warnings` clean with and
  without `--all-features`; `cargo +1.99.1 fmt --all --check` clean. Hashes: delay.rs 4bb0ebea...dc19f,
  delay_tests.rs 17c63a39...8583. Added lines: no `unsafe`, no `libc`, no `pub` (grep of the diff's + lines). Nothing
  committed; git status: the two l2 files modified, this notes file new.
- For the critic: item 4's change reaches past the panic case by design: a pump that ended on S's own error (write,
  flush or read) before the shutdown completed now gives that kind from poll_shutdown (it gave Ok), the same kind
  the reader and poll_flush already gave. Ends with no error (a limit's close, a cancelled or dropped pump) keep Ok;
  a completed or timed-out shutdown (`down_done`) keeps Ok. No existing test changed its result.
- Final diff: delay.rs +13/-2 (poll_shutdown, its doc, module doc, POOL doc); delay_tests.rs +144/-14 (R1 bound,
  G-PRE-WAKE helper + add_burst test, panic test's flush/shutdown asserts, new normal-end test + helper).

DONE
