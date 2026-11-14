# U3d plan rev 1: rust-reviewer's adversarial review (notes)

Started 2026-11-09 on branch claude/netcode-proof-test-plan-hcky5g, HEAD f6041cb (95dac9c + checkpoint-only commits;
`git diff --stat 95dac9c HEAD` touches .checkpoints only). Read-only: no code edits, builds, test runs or network.

## Plan of this review
1. Read u3d-plan.md (rev 1), u3d-plan-notes.md, u3d-scout-req.md, u3d-scout-r1.md.
2. Check the code claims at file:line: socket.rs (poll_recv, release path, fate log, header_numbers/texts,
   release_thread_id), thread.rs (slack/policy reads), stats.rs (fate log sizes, E), impair.rs, config.rs,
   socket_tests.rs (M13 run, pace, poller), delay_tests.rs (runtime, loadavg), Cargo.toml (tokio features, quinn
   dev-dep?), quinn/quinn-udp sources in ~/.cargo registry (BATCH_SIZE, drive_recv).
3. Focus: placement and shape (test file only? new pub symbols?), no unsafe/libc in npt-net, measurement points not
   perturbing (no lock/alloc on release path; fate log first-touch under lock), slack/steal reads safe, identical
   behaviour on targets that build it (cfg gates, Linux-only /proc), no engine feature beyond need.
4. Findings with severity, where, claim, evidence, fix. Verdict.

## Findings (one line each, as found)
- R0 No earlier notes file of mine (checked at start). Inputs read: u3d-plan.md rev 1, u3d-plan-notes.md (N1-N25,
  DONE), u3d-scout-req.md (DONE), u3d-scout-r1.md (DONE). Code read: socket.rs (whole), socket_tests.rs :1-100,
  :170-240, :350-430, :480-639; thread.rs (whole); stats.rs :1-70, :100-165, :415-520; impair.rs :640-767, :338-343;
  config.rs (whole); l1 lib.rs :90-165; delay_tests.rs :160-195, :3080-3097; check.mjs :1115-1130; SERIES.md :386-473
  (parts); marks.json :740-760, :2340-2352; quinn-0.11.12 endpoint.rs :145-165, :365-400, :497-515, :740-870;
  quinn-udp-0.5.16 lib.rs :80-90, unix.rs :800-812; quinn-proto-0.11.19 config/mod.rs :60; tokio-1.54.1
  multi_thread/mod.rs :80-91, park.rs :360-380, multi_thread/worker.rs :1354-1377, :1424-1428, :1472-1476,
  inject.rs :56-60, idle.rs worker_to_notify, sync/notify.rs :1225-1247. /sys THP read: madvise.
- CONFIRMED (shape): no unsafe/libc possible in npt-net (workspace `unsafe_code = "forbid"`, Cargo.toml lints; npt-net
  [lints] workspace = true); no new dependency needed (tokio =1.54.1 has rt-multi-thread, sync, time, net, macros;
  std::future::poll_fn; `quinn::udp::BATCH_SIZE` is pub: quinn-udp lib.rs:85, re-exported quinn lib.rs:75; = 32 on
  both Linux targets, unix.rs:804-806 cfg(not(apple_slow))); no pub item; fate_log_capacity is a pub field (config.rs:60,
  default 0 :109); FateLog reserves at bind (stats.rs:476-480, Vec::with_capacity, no realloc: push checks cap first);
  Fate is 56 B in memory too (5 u64 + 2 u32 + 3 u8 -> 51, align 8). The exact `band` claim holds and is worse than
  stated: q.powi(n) = 0 makes every pmf 0, so it returns (0, n), a vacuous band that passes anything
  (delay_tests.rs:166-191).
- F1 MEDIUM (where the poller runs). §3.1 says "runs one task ... 4 workers, as delay_tests.rs:3084". That precedent is
  #[tokio::test], whose body runs under block_on ON THE TEST THREAD (tokio multi_thread/mod.rs:80-91 "The future will
  execute on the current thread"; parked by CachedParkThread, park.rs:366-380). A poller written as the test body or
  under rt.block_on measures a thread park/unpark hop, i.e. M13's (socket_tests.rs:55-60, :82), not quinn's (quinn
  spawns EndpointDriver: endpoint.rs:152-160). Fix: rt.spawn onto Builder::new_multi_thread().worker_threads(4)
  .enable_all() (npt-server's runtime has the I/O driver, so the parked worker sleeps where quinn's does); T1 and the
  self-test call the same poller fn, so T1's blocked single worker also bites a block_on poller.
- F2 MEDIUM (instrument on the measured path). Fate-log pages are first touched under the state lock (finish ->
  log -> push: impair.rs:686-697, :338-343; release thread socket.rs:729-735; poll_recv :527). Plan risk 2 estimates
  "a few µs" per 4 KB fault, unmeasured; this container is a Firecracker guest (kernel 6.18.44-fc-v310), where a
  guest first touch can also fault on the host. THP is madvise here (/sys/kernel/mm/transparent_hugepage/enabled),
  so ~27,400 4 KB faults. Zero-production-cost removal: before the pacers start, fill the log to capacity through
  the crate-visible `s.shared.lock().im.fates.push(..)` (FateLog::push pub(crate), Impairer.fates pub(crate)
  impair.rs:248; socket_tests is socket's child, so the private `shared`/`State.im` are visible) and empty it with
  `drain_fate_log(|_| {})` (Vec::drain keeps the capacity). Carry the same to C-U6-3; record THP in the header facts.
  (Wording fix for the final: say "a VM guest (steal 529 ticks, N15)"; the "fc" in the kernel name only suggests
  Firecracker. Lost/drop fates are logged at intake too (impair.rs:501, :506, :576), so the reader thread and the
  down pacer also touch log pages under the lock.)
- F3 LOW (observer calls inside the window). header_numbers -> ask_slack makes the release thread do /proc file I/O
  while it holds the state lock (socket.rs:667-677 -> thread.rs:39-55); run_summary walks both run histograms under
  the lock, 4 passes over up to 8,440 buckets each (socket.rs:400-414; stats.rs:91-99, :108-159); first_seen and
  drain_fate_log lock too (:262-263, :427-429). M13 calls them only after the joins (socket_tests.rs:575, :611). The
  plan lists the prints and the "slack 1 at start" condition without saying when they are read. Fix: between the first
  send and the poller's stop the only ImpairSocket calls are try_send and poll_recv; the stop time computed from
  start + n/rate + max one-way (30 ms at P1) + 500 ms, never by polling run_summary (summary_when, :43-53); every
  header/summary/fate read after the stop (l2_timer_slack_start_ns is the start value whenever read).
- F4 LOW (T1's race). T1 spawns the blocker only after the test thread sees first_seen, inside the 20 ms before the
  datagram is due; the existing thread-poller test needs no such race (it simply does not poll: socket_tests.rs:
  186-211). Under check.mjs's parallel test step (cargo test --workspace, debug, both builds: check.mjs:1123-1128) a
  late spawn lets the poller poll on time -> lateness ~0 -> a spurious fail. Fix: spawn the blocker first (it signals
  over a std channel, then sleeps 200 ms), then send; lateness >= 50 ms holds if the drain comes within 130 ms of the
  signal; keep the upper bound at - first - 20 ms.
- F5 LOW (sample set for the fate-log forms). The histogram takes every finished fate but Oversize (impair.rs:680-684:
  Delivered, SendError, WouldBlock outbound); T2's "delivered fates = l2_late_*_count" holds only with no send error.
  Fix: the E forms select exactly impair.rs:680's set (kind != Oversize among finished fates; out_ns != 0); -3's delays
  take Delivered only; subtract with checked_sub after filtering (a lost entry has out_ns 0: stats.rs:440-441; 0 - t_in
  panics in the debug test build rather than failing with a message).
- F6 LOW (the hop is not inbound-only). The release thread wakes the poller before sending its batch (socket.rs:711-727).
  A tokio task's wake from a foreign thread takes the inject mutex and may lock idle's synced and unpark a worker
  (tokio worker.rs:1354-1377, :1424-1428, :1472-1476; inject.rs:56-60; idle.rs worker_to_notify), unlike Unpark's
  thread::unpark (socket_tests.rs:55-60). So the self-test's DOWN lateness also differs from M13/c10's: faithful to
  quinn, not like-for-like. Fix: say so in §0, §5.1 and risk 3; compare down only within the self-test's rounds.
- F7 LOW (no new public symbol). C-U6-2 offers "the run window reset at each leg's start": a new pub method on
  ImpairSocket, which plan §4.7 must list first (plan.md:580-584: "Everything else in npt-net stays pub(crate)").
  Fix: carry "one ImpairSocket per leg" (no new item); a reset only through a §4.7 entry.
- F8 NIT (cites). Inbound stamp: socket.rs:503 (now) and :527 (finish Delivered); :501 is the lock, :525 dst_ip. quinn's
  drive_recv is endpoint.rs:497-512; :772-857 is RecvState::poll_socket (iovs :781-792, loop :793). quinn's buffers are
  min(max_udp_payload_size, 64 KiB) x max_receive_segments each (endpoint.rs:754-759) = 1,472 B by default
  (quinn-proto-0.11.19 config/mod.rs:60), not 1,500 B: no effect at 40 B, but "as quinn" should say 1,472.
- F9 NIT (stop branch). If the stop is a select!, the receive branch goes first (biased) or the stop is an AtomicBool
  read after Pending: a tokio Notify polled in Waiting state locks its waiter list each poll (sync/notify.rs:1225-1247),
  before poll_recv's stamp. quinn locks its endpoint and reads the clock first (endpoint.rs:370-375): tens of ns either
  way, but the plan should name the order.
- F10 NIT (bounds and integers). NPT_L2_SELFTEST_SECS sizes a 1.12 MB/s log at bind; a typo (10000) reserves 11 GB and
  aborts on allocation failure: cap it (e.g. <= 600 s) with an INVALID message. Loss band in integers (u64::isqrt);
  sums in u128 as summarize does (stats.rs:131-152); per-direction arrays, never a HashMap (iter_over_hash_type deny,
  Cargo.toml [workspace.lints.clippy]).
- (cite fixes: M13's run_summary is socket_tests.rs:576, header_numbers :611; Fate.out_ns stats.rs:439-440.)
- VERDICT: APPROVE WITH FINDINGS. Shape right (test-only in socket_tests.rs, no unsafe/libc, no pub item, no
  dependency, Cargo.lock unchanged, no engine feature); rev 2 should fold F1-F3 (plan text, no design change), F4-F7
  as text, F8-F10 at the builder's discretion.
DONE 2026-11-09
