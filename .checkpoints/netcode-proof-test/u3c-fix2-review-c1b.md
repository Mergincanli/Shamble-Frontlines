# rust-reviewer: U3c fix2 C1b review (DelayStream)

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_10e244da-89a-1, uncommitted on b5e4043.
Files: crates/net/src/l2/{delay.rs, delay_names.rs, delay_tests.rs, stats.rs}, marks/SERIES.md.

## Plan
1. Read git diff of delay.rs (pump paths: R1 wake, app_step wake, Drop panic, move_up compaction), stats.rs (Pooled), delay_names.rs.
2. Check lock order, lost wakeups, pending writes, model-time monotonicity, Pooled concurrency, panic path.
3. Per brief item: test exists, does it bite (scratch-copy revert where cheap).
4. Run check.mjs with the shared target dir.
5. Verdict.

## Progress
- (started 2026-11-08) notes created; reading diff.
- check.mjs run 1 (mine, shared target dir): all 9 steps PASS, 76 s wall; test step 258 measured + 272 bites passed, 0 failed. `--list` after: 162 npt-net lib tests incl. the 6 new ones (this tree's binary).
- Read the full diff of delay.rs / stats.rs / delay_names.rs / delay_tests.rs; the fix agent's notes and bites.py / bites-run.txt (13 bites + 1 control, all as expected, on this tree, in place with restore).
- Verified tokio 1.54.1 drops a panicked task's future inside its catch_unwind guard (~/.cargo/registry/src/*/tokio-1.54.1/src/runtime/task/harness.rs:523-540), so Pump::drop sees thread::panicking() (G-PRE-PANIC premise holds).
- G-METHOD hazard confirmed: a cp -a scratch copy of the spike is "Fresh" against the shared target (same artifact npt_net-088534cbcaf56b90): cargo's path-source hash is workspace-relative. So a bitten scratch build would be reused by the worktree. Mitigation: rbites.py --final restores + touches the scratch sources and rebuilds from identical content before my second check.mjs run.
- Started my own bites (scratch copy /home/user/Shamble-Frontlines/.gate/c1b-review-scratch/netcode-proof): rbites.py -> rbites-run.txt.
- My bites (scratch copy, rbites-run.txt), each reverted then restored: R1-OFF BITES (delay_tests.rs R1 test "no wake owed (R1)"); WAKE-CURRENT-ONLY-OFF BITES (woke 0 vs 1); PANIC-OFF BITES ("a panicked pump read as a clean EOF"); F10-OFF BITES (pump slice-index panic, reader error); FH-20K BITES; NC4-ZERO BITES (owed 0 vs 10,044); NC2-TIGHT BITES ([366, 0] samples over).
- FINDING (low, gap): WAKE-ADDBURST-ONLY-OFF: reverting only add_burst's G-PRE-WAKE wake (delay.rs:1132 `if wake_pump` -> `if got[DOWN]`) leaves all 30 l2::delay tests passing. The G-PRE-WAKE test drives current() only (delay_tests.rs `ds.outstanding_bytes()`); add_burst's half has no test that fails on revert.
- INFO bites: PANIC-ALWAYS (failed = Other on every pump end, panicking or not, delay.rs:1653) -> all 30 l2::delay tests pass: the non-panicking side of G-PRE-PANIC's condition is unpinned (a normal end, e.g. the shutdown deadline at delay.rs:1850-1853, would turn into an error with no test noticing). R1-SPIN (`again = released[DOWN] > 0`, dropping `!s_owes_write`, delay.rs:1869) -> all 30 pass: the "cannot spin" guard is unpinned. Measuring the spin next (polls under a non-reading client).
- SPIN measured (spin-probe2-run.txt; scratch, a_full_budget test with a temporary print of pump polls over its 1.5 s hold, client not reading, S Pending, released down records untaken): FIXED 118 / 122 / 121 polls; R1-SPIN (no `!s_owes_write`) 177,315 / 168,794 / 182,725 polls, and the test still PASSES. => FINDING (medium, gap): the guard that keeps R1's self-wake from spinning a worker while S is Pending is correct (delay.rs:1708/1768/1869) but no test pins it; a client that stops reading would pin a tokio worker at ~115k polls/s if a refactor drops it. Fix: assert a bound on life().polls over that hold (120 vs 170k: e.g. < 2,000 is far from both).
- Probe 2 reproduced (scratch, the fix agent's probe2.rs appended, 3 runs): FIXED S 30 ms / gap 20 ms: B 40.34-40.50 ms after its release; S 2 ms / gap 1 ms: 3.32-3.53 ms; S 0: 0.17-0.24 ms. Decomposed (S 2 ms): B due 1.09-1.14 ms after A, A's write returned 1.16-1.22 ms after B's due, B's own write 2.2-2.3 ms => no hold (B's write starts at A's return). Matches the fix agent's 3.4-3.6 / 40.5-40.7 and explains the critic's 2.6 / 39.9 by B's due position.
- Probe 2 with R1 off (scratch, 3 runs): S 30/20: 130.36-130.54 ms; S 2/1: 102.30-102.42 ms (B's write returns ~100 ms after A's). Fix restores the pre-fix shape.
- Flakiness: R1, G-PRE-WAKE and G-PRE-PANIC tests 10/10 each on the unbitten scratch tree.
- Restored: scratch l2/*.rs hashes equal to the worktree's after every script; `diff -r` scratch vs worktree spike: identical; `rbites.py --final` touched + rebuilt npt_net-088534cbcaf56b90 at 20:30:40 from identical content (newer than every scratch file). Could not delete the scratch copy (rm denied); it is identical to the worktree, so harmless, but it shares unit hashes (G-METHOD): delete it before anyone edits it.
- Worktree: fmt --check clean; clippy --workspace --all-targets -D warnings clean, with and without --all-features (cached from identical content); check.mjs run 2: all 9 steps PASS (74.3 s; 258 + 272 passed, 0 failed). Worktree l2/*.rs hashes unchanged by me; git status as at start. Added lines: no `unsafe`, no `libc`, no new `pub` item.

## Code review (no regression found)
- R1 (delay.rs:1708, :1768, :1869, :1873-1875): the down loop exits only on released == 0, S Pending (s_owes_write), or Ready; so `again` is true only for a record released after the loop (up loop's By::Mid step, or an app task's step, which also wakes via app_step). Self-wake is after the lock is released (F2); tokio turns a self-wake during poll into a yield (back of queue). No lost write; no spin while the guard stands.
- G-PRE-WAKE (delay.rs:484-502, :1112, :1132, :1164, :1167): up_need = the front scanned record's len + 22 (move_up keeps the buffer starting at it) else read_floor; need depends only on bounds/read_floor, which step_to never changes; crossing test per step; wake fired after drop(c) / via Stepped's field order. Covers both stop reasons of the read loop (move_up room, read floor). Poll_read's own freed-record wake stays.
- G-PRE-PANIC (delay.rs:1649-1664): tokio-1.54.1 harness.rs:523-540 drops the future in the unwinding Guard, so thread::panicking() is true; the pump's own MutexGuard drops first while unwinding, so Pump::drop's lock cannot self-deadlock; poisoning tolerated (Shared::lock). poll_read checks failed before pump_done (delay.rs:1528-1533), so Err(Other).
- Pool (delay.rs:182-212, :559-576; stats.rs Pooled): relaxed fetch_add / fetch_max only; no lock of its own; OnceLock initialised in build (:1052) before any release so late() never blocks on init; no allocation on the release path (summary allocates, off path). Under the conn lock only atomics (allowed by ruling (3)). pool_grew's subtraction cannot underflow (per-location read-read coherence on monotone counters).
- Model time: every stepper reads the clock under the conn lock (pump :1700-1703, :1801; app_step callers :1111, :1163), TcpModel::advance keeps max (tcp_model.rs:2284-2291): monotone, unchanged by this diff.
- No new panic reachable from input: indices are dir/kind constants; bucket_of clamps; test-only subtractions (check_held_bound) are cfg(test).

## Verdict: GAPS (code APPROVE; REVISE for two small tests)
1. MEDIUM gap: R1's no-spin guard unpinned (numbers above). Add a bound on life().polls during a_full_budget's 1.5 s hold.
2. LOW gap: add_burst's G-PRE-WAKE wake (delay.rs:1132) unpinned; reverting it alone passes all 30 l2::delay tests.
3. LOW: G-PRE-PANIC's `thread::panicking()` condition unpinned the other way (PANIC-ALWAYS passes all tests): a normal end (shutdown deadline, delay.rs:1850-1853) turned into an error would go unnoticed.
4. INFO: poll_shutdown returns Ok(()) after a panicked pump (delay.rs:1605-1606; pre-existing), the writer-side twin of the clean-EOF lie.
5. INFO / U6 carry: POOL is process-wide and never reset (delay.rs:188): a process serving more than one run mixes runs in ws_split; state "one run per process" or reset at run start.
DONE
