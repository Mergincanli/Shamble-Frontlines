# U3d build review (netcode-architect), 2026-11-09

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_2c76ec82-8fc-1 on dae4f0f (uncommitted). Read only: I
edited, built and ran nothing there. Plan: u3d-plan.md (b2e0980 + conditions 1-4); advisor-u3d.md.

## Plan
1. Read plan, advisor verdict (done).
2. Read the diff of socket_tests.rs (self-test, start_poller, helpers, T1-T7).
3. Check against: P1/seed/reorder; rate and length; poller shape (spawned, 32 x 1,472, loop until Pending, stop
   after Pending, defined inside start_poller); window rule; pre-touch; E forms + sample set; -3 cross-check formulas;
   integer bands; verdict lines; prints before asserts; run conditions.
4. Read run.sh (U3-Q9 facts, THP, steal per ^cpu line, build once, hash, interleaving, exit status).
5. Read SERIES.md diff (condition 2 wording).
6. Anything beyond the plan.
7. Findings, DONE.

## Progress
- Diff scope: socket_tests.rs two hunks only (top doc line + import; appended block after M13 at :641-1632); one
  removed line (the doc comment). No production file, Cargo.lock no diff. SERIES.md 3 hunks. results/u3d-timing/run.sh
  new, the only file in that folder.
- Poller (socket_tests.rs:676-712): async block built only inside start_poller, rt.spawn; one buffer of
  DEFAULT_MAX_DATAGRAM_BYTES (config.rs:81 = 1,472) x quinn::udp::BATCH_SIZE (32 on Linux; quinn-udp lib.rs:85);
  iovs via chunks_mut + array::from_fn as quinn-0.11.12 endpoint.rs:781-791; loop until Pending (:694-705), then
  stopped.poll (:706). Matches quinn's poll_socket shape (endpoint.rs:772-800, read in the registry). Condition 1 holds:
  self-test :1378, T1 :1003, drive (T2, T7) :921.
- P1: Profile::p1(), seed 11, reorder asserted (:1341-1342); longest = one_way + half_jitter (:1346); ppm from the
  profile; bands() pinned to marks.json literals in T6 (:1170-1182). Rate 10,000 each way via pace (:1393-1406); length
  1-600, default 100, INVALID before bind (:1326-1338).
- Pre-touch (:1355-1375): pushes up to cap under the lock, then drain_fate_log; FateLog::push cannot refuse before cap
  (stats.rs:482-491); Vec::drain keeps capacity (stats.rs:493-498). log_len 60,000 = 2n after the stop in the check
  record (no leftover).
- Window rule: inside the scope only try_send (down pacer) and poll_recv (poller); peer socket calls are not socket
  calls; stop = joins + 30 ms + 500 ms + notify_one (:1420-1427); run_summary, header_numbers, header_texts, loadavg, the
  fate log all after (:1431-1440). try_send always Ok (socket.rs:463-492), poll_recv never Err.
- E forms (:1442-1443, :789-797): over in_e_set = Delivered|SendError|WouldBlock = finish() kinds minus Oversize
  (impair.rs:653-697). L0 = 0 labelled the judged form; exact p50 and histogram p50 reported. excess rounds down, u128.
- Cross-check (:866-893): delay = out - t_in of Delivered only with checked_sub; count = every fate of the dir; loss =
  Lost x 1e6 within [lo x count, hi x count]; mean via exact sum; nearest rank ceil. T6 hand values re-checked.
- Verdict lines: would_one <= BAR_NS 10,000 (:800-807); per direction; agree lines vs p99.9 per form.
- Asserts after prints (:1588-1631): rate (integer), drops, accounted, loss_band over fates, send err/would-block,
  poller count, fate_log_full, slack start.
- run.sh: build once from the spike folder, JSON executable, sha256, --ignored --list check, runs the binary directly;
  header kernel, clocksource (+available), cpuidle driver/governor, CONFIG_HZ (/boot then /proc/config.gz, never
  CLK_TCK), THP; every ^cpu line before/after with steal = f9 / sum f2..f9; exit status kept; refuses CARGO_TARGET_DIR
  unless U3D_CHECK=1 (marks the record not evidence).
- SERIES.md :427/:452 condition 2's wording exact; no remaining "Mustafa's bar"/"about 10 µs" text.

## Findings
F1 (low) socket_tests.rs:1445-1449, :1627-1631. The slack run condition requires slack_start == 0 whenever the policy
read after the stop is SCHED_FIFO/RR; plan §3.1 says "1 at start under SCHED_OTHER (0 accepted under a real-time
policy)". slack_start is read by the release thread at its birth (socket.rs:648-651), the policy at header time
(socket.rs:351-361). M14's switch is applied from outside with chrt on release_thread_id (socket.rs:268; SERIES.md:469),
necessarily after the thread started, so a run with the recorded switch reads start 1 + FIFO and is INVALID by
construction; a process born RT on a kernel without ed4fb6d7ef68 reads 1 too (SERIES.md:470). SCHED_DEADLINE
(thread.rs:80) is not counted as real-time. Fix: under FIFO/RR/DEADLINE accept start in {0, 1}; under OTHER (or
unknown) require 1; print which case. No effect on the planned dry run (SCHED_OTHER).

F2 (nit) SERIES.md:396-397. "Its lateness fields cover the socket's whole life" follows "A unit test checks every
field it writes against this file", so "Its" reads as the unit test's. Fix: "The event's lateness fields ...".

F3 (nit) socket_tests.rs:1369-1374, :1467-1468. `touched` is printed, not checked. Moot today (push cannot refuse
before cap), but `touched == cap` after the prints would make "pre-touched" a run condition.

F4 (nit) socket_tests.rs:743-746 called at :1439. samples() panics on a Delivered fate with out < t_in before any
print, the one path where a failure loses the run's prints (T-9). An invariant on one monotonic clock; optional: count
them and assert after the prints.

F5 (nit) run.sh:52. U3D_CHECK's touch covers *.rs only; §7.1 item 2's rule is every file under crates. Check mode
only, never evidence.

F6 (nit, report wording) socket_tests.rs:1431, :1568-1572. CPU cores divide by a total that includes the ~530 ms stop
tail, as M13 does with its ~500 ms idle tail (:574-576); at 100 s vs 10 s the dilution differs (~0.5 % vs ~5 %), so
the self-test's cores read slightly higher than M13's for the same load. Say so in U3d's report when comparing.

Beyond the plan: `drive`, `runtime`, `fate_entries`, `Polled` helpers (serve T2/T7 as planned); run.sh's
CARGO_TARGET_DIR refusal, U3D_CHECK mode, ROUNDS, the dirty flag and the never-overwrite guard (enforce §7.1 item 6
and the record's honesty). Nothing traces to an unplanned feature.

Regressions: none. M13 untouched (only the top doc line changed and lines appended after :640); no production code.

VERDICT: APPROVE WITH FINDINGS.

DONE (2026-11-09).
