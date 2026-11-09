# U3d plan rev 2 notes (netcode-architect), 2026-11-09
Branch claude/netcode-proof-test-plan-hcky5g, head dacdb77 (checkpoint-only over 768d00b's code). Plan stage: no code
edits, builds, test runs or network. Deliverable: u3d-plan.md rev 2 (Write), with a §0.1 finding table; notes here.
No earlier rev-2 notes file existed (checked at start).

## Plan
1. Read rev 1, its notes, both critiques in full. DONE.
2. Check each finding against the code at file:line (test F-A..F-O = 1-15, test's own numbering 1-12 = F-A..F-L per the
   checkpoint's summary .checkpoints/netcode-proof-test.md:560-561 "5 medium ... low/nits F6-F12"; rust F1-F10).
3. Decide per finding: meet (where) or defer (why, with evidence).
4. Build rules: Mustafa's shared-target rule (checkpoint :404-413) into §7.
5. Write rev 2; update u3d-plan-notes.md with a rev-2 block; DONE line here.

## Findings (one line each)
- M0 Mapping of the test-engineer's ids: 1 F-A, 2 F-B, 3 F-C, 4 F-D, 5 F-E (medium), 6 F-F, 7 F-G, 8 F-H, 9 F-I,
  10 F-J, 11 F-K, 12 F-L; F-M, F-N, F-O are extra nits on the review's :83-86 (folded as 12a-c).
- M1 test 1 (F-A) = rust F4 HOLDS: socket_tests.rs:186-211 polls only at first+110 ms (a late test thread only adds
  lateness); a task poller already Pending is woken at due by the release thread (socket.rs:711-713), so a blocker
  spawned after first_seen can lose the race. Fix taken: the test-engineer's (blocker first, signals, then blocks the
  only worker until first_seen + 110 ms): the lower bound by construction; rust F4's sleep-200 ms form is conditional.
- M2 rust F1 HOLDS: delay_tests.rs:3084 is #[tokio::test(flavor="multi_thread", worker_threads=4)], body under
  block_on on the test thread (tokio-1.54.1 runtime/scheduler/multi_thread/mod.rs:80-91 "The future will execute on
  the current thread"); quinn spawns its driver (quinn-0.11.12 endpoint.rs:152-160). Fix: rt.spawn on a built
  multi-thread runtime, enable_all; T1 and the run share one poller fn so T1 bites a block_on poller too.
- M3 rust F2 HOLDS: FateLog Vec::with_capacity at bind (stats.rs:470-480), push under the state lock from finish
  (impair.rs:686-697 -> log :338-343) and intake drops (impair.rs:500-506, :576-579); State.im private but visible to
  the child test module (socket.rs:63-64, :739-741); Impairer.fates pub(crate) (impair.rs:248); FateLog::push/drain
  pub(crate), drain = Vec::drain(..) keeps capacity (stats.rs:483-500); #[cfg(test)] entries() exists (stats.rs:502-505).
  Pre-touch is test-only: no production change.
- M4 rust F3 HOLDS: header_numbers -> ask_slack (socket.rs:285-307, :315) makes the release thread do /proc I/O under
  the state lock (socket.rs:667-677 -> thread.rs:41-56); run_summary locks and walks both histograms (socket.rs:400-414;
  stats.rs:94-99, :108-159); first_seen and drain lock (:262-264, :427-429). Fix: only try_send/poll_recv inside the
  window; stop time computed; every read after the stop.
- M5 rust F5 + test F-M HOLD: histogram takes every finish but Oversize (impair.rs:680-684); early fates carry due 0,
  out 0 (stats.rs:437-440 docs). E forms: kind in {Delivered, SendError, WouldBlock}; -3's delays: Delivered only,
  checked_sub; assert send_err + would_block = 0 in T2 and the run.
- M6 rust F6 HOLDS: the release thread wakes the poller (socket.rs:711-713) before its batch's try_send (:714-727,
  stamp :726), so a tokio wake's cost lands in DOWN lateness too. Text + the P1 thread-poller control (F-H) isolates it.
- M7 rust F8 / test F-L HOLD: poll_recv lock :501, now :503, finish Delivered :527; quinn drive_recv :497-514,
  poll_socket :772ff, loop :793, buffers :754-758 = min(1,472, 64 KiB) x max_receive_segments (ImpairSocket 1:
  socket.rs:549-551) = 1,472 B (quinn-proto-0.11.19 config/mod.rs:60); plan.md rows U2 :1221, U3 :1222, U6 :1225, U8
  :1227.
- M8 test F-E HOLDS: -3's checks carry no "window" key (marks.json:752-1021) so default `measured` (marks/README.md:100;
  phases :37); late_run never resets (impair.rs:246, :681-683; only late_second :738-741); set_default (socket.rs:251)
  would carry a socket across legs without a reset.
- M9 test F-D HOLDS, and the cheap producer exists: the probe can only get per-datagram drain stamps from the fate log
  (ImpairSocket's pub API: socket.rs:176-427, drain_fate_log :427 the only per-datagram source), and -3's L2 series
  need them (SERIES.md:401-406). So the probe computing both E forms over -3's measured records from the same entries
  (C-U6-5) answers F-D and F-E at once, with no stats.rs field and no reset method (rust F7).
- M10 test F-C HOLDS: rev 1 §4 has no test on delay = out - t_in. PM-0001-3's P1 bands are all rule-derived from P1
  (d 25 ms, a 5 ms; marks.json:763-1021: loss 0.0095-0.0105, mean 23.75-26.25 ms, p001 19.75-20.25, p999 29.75-30.25,
  p90 28.75-29.25) and -3 has an RTT mean check (:1010-1021) the self-test cannot compute (no echo): say so.
  npt-net has no serde_json (crates/net/Cargo.toml dev-deps: futures-util only): bands derived from Profile::p1() by
  the rule, pinned to marks.json's numbers as cited literals; a marks.json text read deferred (report-only lines).
- M11 judge's nearest rank is integer (eval.rs:261-265; fixture :1595-1597); stats.rs's quantile is integer div_ceil
  (stats.rs:116-120): the helper uses (num, den) integers, never f64.
- M12 build rules: spike .cargo/config.toml sets build.target-dir ../../.gate/netcode-proof-test/target (relative to
  the copy), check.mjs spawns cargo with the inherited env (check.mjs:1070-1071, no --target-dir), so
  CARGO_TARGET_DIR in the copy's env reaches every cargo step (env beats config: cargo's documented precedence; not
  re-read here, no network: medium-high).
- M13 rust F9 HOLDS: tokio-1.54.1 sync/notify.rs:1224-1247 locks the waiter list when a Notified is polled in Waiting;
  quinn's driver locks and reads now before poll_recv (endpoint.rs:369-376). Order named: poll_recv first, stop after.
- M14 impair.rs:119-123: delay = d - a + U[0, 2a], no offset subtracted -> E with L0 = 0 is the exact predictor; and
  E(L0=0) >= E(L0=p50) always (the integrand is non-increasing in the threshold). Mustafa's Q1 (c) in the session
  record: "expected excess lateness beyond 0.25 ms" (checkpoint :493-494), no L0; M1 adds L0 (advisor-u3q.md:235)
  and says "report". With C-U6-5 both forms cost the same at R1 -> U3D-L0 recommendation moves to (b).
- M15 check.mjs base counts at U3c's close: 260 measured, 274 bites (checkpoint :523-524); U3d adds 6 non-ignored
  tests (T1-T6) -> 266 / 280 expected, plus one ignored (the self-test) and M13 unchanged; npt-net has no feature
  that touches l2::socket (crates/net/Cargo.toml [features]: wt-wtransport, wt-wtq only).
- M16 Decisions for rev 2: tests T1 (fixed order), T2 (live identity + lost > 0 + no send errors), T3, T4 (integer
  rank), T5 (exact E on fixed vectors: 999 x 100 us + 1 x 10.35 ms -> 10,000 / 10,100 ns), T6 (cross-check on fixed
  fates: D 20.00-29.99 ms step 10 us, top two L 0.3 ms -> p999 30.28 ms FAIL vs 29.98 by due - t_in; 10 lost -> share
  10/1010); M13 untouched (copy, no helper); pre-touch; window rule; control round P1 thread poller; all ^cpu lines;
  THP; binary run directly after --no-run with its SHA; carries C-U6-1..7, C-U8-1..4; shared-target build rules.
- M17 Line pins re-read: socket.rs lock :501, now :503, Delivered :527, slack ask :667-677, wake :713, try_send :724, stamp :726; socket_tests.rs inbound test fn :185 (#[test] :184), pace :485, M13 fn :512; impair.rs late_run :246, record :682, take_second :737. Writing rev 2 now.
- M18 Rev 2 written (513 lines): §0.1 table (25 rows), §7.1 build rules, T5/T6 added, C-U6-5..7, C-U8-4, U3D-WINDOW.
  Citations re-checked after writing: SERIES.md pacer :405-406 (fixed from :402-406); advisor-u3c-plan.md:158 (hop
  p50); checkpoint Q9 :501; T5's last bite reworded. u3d-plan-notes.md gained a "Rev 2" block (R2-1..R2-5).
DONE 2026-11-09
