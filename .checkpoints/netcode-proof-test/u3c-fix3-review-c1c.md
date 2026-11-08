# rust-reviewer: U3c fix3 C1c review (DelayStream)

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_5c9f078a-da1-1, uncommitted on c37dfe1.
Answers my C1b verdict items 1-5 (u3c-fix2-review-c1b.md).

## Plan
1. df; git status / git diff in the worktree; read the fix agent's notes.
2. Reproduce `node spikes/netcode-proof/check.mjs` in the worktree first (shared target; touch sources first; record test counts).
3. Review each item's code + test: does it fix, does the test bite.
4. Regression read: pump wake/spin, shutdown + panic paths, lock order, timing bounds (poll bound margin).
5. Scratch copy to .gate/c1c-review-scratch; cheap reverts (items 1-4); measure poll counts under load where cheap.
6. Touch worktree sources, final check in worktree (test counts); verdict.

## Progress
- (2026-11-08) df: 4.5 GB free on /. Notes created.
- Diff read: delay.rs +13/-2 (poll_shutdown, its doc, module doc, POOL doc), delay_tests.rs +144/-14. Hashes at start: delay.rs 4bb0ebea..dc19f, delay_tests.rs 17c63a39..8583 (= fix agent's final).
- check.mjs run 1 (mine, worktree, after touching all 62 .rs, shared target): all 9 PASS, 85.2 s wall; test 260 measured + 274 bites passed, 0 failed, 0 skips. `--list` after: 164 npt-net lib tests incl. both new ones (binary npt_net-088534cbcaf56b90). Output: .gate/check-c1c-review-1.txt.
- Code read (no regression found so far): poll_shutdown (delay.rs:1610-1635) checks down_done before pump_done, so a shutdown S completed (or the deadline gave up on) stays Ok even if the pump later fails; pump_done -> failed's kind or Ok. Lock: the early returns hold only the conn lock, no S poll, no wake cloned; io::Error::from(ErrorKind) is the Simple repr (no allocation; not the release path anyway). Wakeups: failed is set under the lock then the pump returns Ready; Pump::drop (delay.rs:1660-1675) sets pump_done and takes the shutter under the lock, so a shutter registered before is woken and one polled after sees pump_done: no lost wakeup. No caller of DelayStream's shutdown outside tests (grep crates/); tokio-tungstenite 0.30 calls poll_shutdown only in MaybeTlsStream's pass-through (stream.rs:81-90).
- Item 5 verified: POOL doc (delay.rs:183-186) matches plan.md:1225 (§12.1 U6 row: "one measured run per server process, or the pool reset at each run's start, stated in the run's header").
- Scratch copy .gate/c1c-review-scratch/netcode-proof (cp -r, diff -r identical to the worktree's spike). Script .checkpoints/netcode-proof-test/u3c-fix3-review-c1c/rbites.py (edit, run --exact --test-threads 1, restore, sha256 checked); log rbites-run.txt there.
- CONTROL (scratch, unmodified): the 5 changed tests pass (164 lib tests listed: 5 run + 159 filtered); hold polls 115.
- BITE R1-SPIN (`again = released[DOWN] > 0`, guard dropped, delay.rs:1880): a_full_budget FAILS at delay_tests.rs:792, 158,276 pump runs in the 1.5 s hold. Item 1 pinned.
- BITE WAKE-ADDBURST-ONLY (add_burst `if wake_pump` -> `if got[DOWN]`, delay.rs:1137): the add_burst test FAILS at :2124 (left 0, right 1); the current() test passes. Item 2 pinned.
- BITE PANIC-ALWAYS (Pump::drop `c.failed.is_none()` alone, delay.rs:1664): the normal-end test FAILS at :2392 (clean close: reader got Other); the panic test passes. With the clean-close check skipped (test edit in scratch), it FAILS at :2392 in the deadline half (reader got Other): each half bites alone. Item 3 pinned.
- BITE SHUTDOWN-OK (poll_shutdown back to `down_done || pump_done -> Ok`): the panic test FAILS at :2380 ("a panicked pump's shutdown read as a clean one: ()"). Item 4 fixed and pinned.
