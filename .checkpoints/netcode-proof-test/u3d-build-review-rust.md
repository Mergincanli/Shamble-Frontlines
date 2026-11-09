# U3d build review: rust-reviewer

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_2c76ec82-8fc-1 on dae4f0f396d655d194e4300e8d56f07b619be908 (uncommitted).
Read only: no edit, build or run in the worktree.

## Plan
1. Read the approved plan (u3d-plan.md), advisor-u3d.md (conditions), plan critics (test, rust).
2. git status / git diff in the worktree; read socket_tests.rs's appended block in full.
3. Check against the focus list: no unsafe / libc / new pub / dep / HashMap; measured window clean
   (no lock, alloc, log, print inside); pre-touch before; tokio runtime + spawn (no block_on poller);
   integer math (u128, isqrt, checked_sub); env bounds 1-600; hang guards on every wait; clippy/fmt idioms;
   identical behaviour on targets.
4. run.sh and SERIES.md text check.
5. Findings with severity, file:line, claim, evidence, fix. DONE line.

## Progress
(started)
- Read: plan rev 2 (558 lines), advisor-u3d.md, my plan review. Worktree diff: socket_tests.rs +995 (one import line,
  module doc line, block appended at :642-1632), SERIES.md 3 rows. Untracked: results/u3d-timing/, notes.
- Read socket_tests.rs :1-110, :480-1632 (M13 :508-640 + the new block).
- Candidate L1 (T1 :1043): `at.duration_since(first).as_nanos() as u64 - 20 * MS` unchecked; underflow panics in debug
  if at < first + 20 ms (possible in a bite run: poller at due). To check: first_seen vs t_in order; bites output.
- Candidate nit: FATE_BYTES used by path at :1469 while ALLOWANCE_NS/Fate imported at :4.
- Candidate nit: env Err(_) covers NotUnicode -> silently 100.
- To check: DEFAULT_MAX_DATAGRAM_BYTES == 1,472? poll_recv with 32 iovs; FateLog push/drain/entries; histogram's
  late = saturating?; Cargo.toml rust-version/lints; tokio features; l2 cfg gates (wasm32); run.sh.
