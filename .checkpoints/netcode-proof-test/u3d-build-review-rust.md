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
- CHECKED OK: L1 candidate withdrawn: T1 :1043's `- 20 * MS` cannot underflow (first_seen = the drain's t_in on the
  socket clock, impair.rs:517-518, socket.rs:262-264; pop_due releases only at due <= now, impair.rs:615-618; at is read
  after poll_recv's now). Same expression as the existing test :207.
- CHECKED OK: start_poller :676-712 defines the poller as an async block inside it and spawns it (condition 1); bites
  T1a/T1b replace `rt.spawn(poller)` inside the helper (u3d-bites.py:40-44); both failed on "inbound lateness".
  block_on is used only on JoinHandles (:983, :1035, :1038, :1427), never to poll the poller.
- CHECKED OK: window :1388-1429: only try_send (down pacer) and poll_recv (poller); test thread parked in join; prints
  :1454-1586 all after stop and before asserts :1589-1631. Pre-touch :1355-1375 before rt/poller/pacers; push checks
  cap (stats.rs:484-493), drain keeps capacity (:495-500).
- CHECKED OK: integers: u128 sums (:776, :869), div_ceil rank (:757-759), u128 isqrt band (:812-818), checked_sub on
  delay (:744-746), saturating late = histogram's own (impair.rs:681); rate assert in u128 (:1591).
- CHECKED OK: env :1326-1338 1..=600 else INVALID panic before bind. M13 sha256 29b21eab... both sides.
- CHECKED OK: no unsafe/libc/HashMap/pub/dep; Cargo.lock unchanged; no .gate inside the worktree (shared target held);
  npt-net not in check.mjs's wasm32 crates (check.mjs:24, :1106-1111): Linux-native only.
- FINDING low: slack assert :1627-1631 demands start slack == 0 under RT; plan §3.1 says "0 accepted"; thread.rs:9-11
  says a kernel without ed4fb6d7ef68 keeps 1 under FIFO, and a per-thread chrt after bind leaves start = 1.
- FINDING nit: recv_buf :686 `vec![0u8; ..]` (calloc) leaves its ~12 pages to be first touched by poll_recv's copy
  under the state lock (socket.rs:516); minor faults < 0.25 ms, same as quinn; optional touch.
- FINDING nit: FATE_BYTES by path :1469 vs import :4; env Err(_) covers NotUnicode -> 100 silently; touched not asserted.
- CHECKED OK: hang guards: T1 waker 2 s (:1005-1009), recv_timeout 2 s (:1031), blocker gives up at 2 s (:1018-1024),
  summary_when 5 s; drive's sink 200 ms idle, accounting 10 s (:967-972); self-test sink 500 ms idle. tokio runs its
  workers on the blocking pool and Runtime::drop waits for them, so a panic path waits at most for the blocker's 2 s.
- CHECKED: quinn-0.11.12 endpoint.rs:754-759 recv_buf is `vec![0; ..]` too: the poller's lazy first touch matches quinn.
- CHECKED: ed4fb6d7ef68 reports 0 and ignores non-zero slack for rt-policy tasks (search result summary; two commit
  mirrors failed to load, 404/429), so whether it covers SCHED_DEADLINE is NOT verified.
- No regressions: M13 byte-identical, Cargo.lock no diff, no production file changed, SERIES.md text only.
VERDICT: APPROVE WITH FINDINGS (1 low: the slack run condition under RT; 5 nits).
DONE 2026-11-09
