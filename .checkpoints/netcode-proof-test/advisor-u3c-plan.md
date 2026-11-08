# Advisor notes: U3c plan review (u3c-plan.md)

Started 2026-11-07. Branch claude/netcode-proof-test-plan-hcky5g, head 2ec1533; code as of 7321cc0.

## Plan
1. Read u3c-plan.md (done), then sources: advisor-ws-docs-r2.md:370-420, verify-ws-docs-r2.md:47,
   advisor-u3q.md (M14, M16, M17, S0-S14), u3-build-plan.md, plan.md §4.1-4.8, §12.1, catalogue :61-78.
2. Code: tcp_model.rs :2058-2205, fixes.rs :163-169 / :770-787, l2/socket.rs :646-737, l2/config.rs, link/src/lib.rs,
   Cargo.toml :56-58, Cargo.lock (libc, rustls, socket2).
3. Facts: crate sources in scratchpad (socket2/rustix/nix), tokio-rustls 0.26.6 deps.
4. Rule on 1-6.

## Findings (as I go)
- C1 facts re-checked: tokio-rustls 0.26.6 manifest (scratchpad u3c-crates): rustls ^0.23.27, default-features off,
  features ["std"]; tokio 1.0; defaults logging, tls12, aws_lc_rs; a `ring` feature = rustls/ring. Lock gains only
  tokio-rustls (rustls 0.23.46, tokio 1.54.1 already there). rustix 1.1.6 / nix 0.31.3: no tcp_info, no SIOCOUTQ
  (grep). socket2 0.6.5: only tcp_notsent_lowat. libc 0.2.191: tcp_info (gnu :248), TIOCOUTQ generic 0x5411 (x86-64
  and aarch64 alike). Cargo.toml:56-58 confirmed; client-wasm's exception is `unsafe_code = "allow"` in its manifest.
- C2 ring: quinn 0.11.12 default features include rustls-ring, so rustls/ring is on in npt-net in every feature set;
  aws-lc-rs only with wt-wtq (wt.rs:177-179). M7 = each crate's OWN provider (wtransport ring, wt-wtq aws-lc-rs), so
  the plan's "as M7 does" is loose: the reason for ring on WS is that it is the one provider always present.
- C3 ADR 0002:264-265 (Proposed; Mustafa signed it as the spikes' basis 2026-11-03): "unsafe_code is denied in every
  crate; an exception needs a written invariant, a test and an #[allow] with its reason". security-auditor.md:38
  "unsafe only with a written invariant". So an exception that follows that clause changes no rule: mine to grant.
  Mustafa's U3-Q2 (b) picked the /proc slack route "without unsafe code"; it stays true (release thread untouched).
- C4 tungstenite 0.30 handshake AttackCheck caps the request at 64 KiB / 512 reads (machine.rs:169-170). Plain path
  keeps that; DelayStream path gets 8,192 B. Recorded difference.
- C5 BUG in plan 1.2 write rule: "takes at most free - 22 B" is only right for one record; a chunk over 16,384 B is two
  records (+44 B) and can overrun the model's free budget by 22 B; model.write() would then take a partial record.
- C6 GAP: per-session controls on WS (E23: profile, B1; ADR check item 11 triggers B1 on the fallback) need bursts
  after wrap; TcpModel bursts are fixed at with_config (tcp_model.rs:2122), no setter; plan's surface has none.
- C7 GAP: up direction bound "64 KB in model + 64 KB released-unread" needs the same rule as down (room counts
  released-unread), else released-unread can reach 64 KB + the model's 64 KB.
- C8 GAP: pump lifetime: shutdown waits for delivery AND S taking it; a stuck S or a dropped DelayStream (slow-loris
  closed by npt-server's deadline) must end the pump in bounded time; heap/waker table bounded per live connection.
- C9 test gap: npt-tcpinfo's "grows and equals tx_queue with a non-reading peer" cannot tell SIOCOUTQ from
  SIOCOUTQNSD (zero window: everything unsent, nothing in flight). Needs a state with sent-unacked bytes (root run:
  drop the ACK direction) or a constant test.
- C10 r2 fixes.rs leftovers not placed: stale three_part() bar (fixes.rs:1530) and the stats B5 bite-row pin.
- C11 plan inconsistency: 1.2 says accessors pub(crate), "Public surface" says pub read-only accessors.
- C12 bites build: ws-budget-x2 is a run-time bite for R3 (R11); npt-net has no `bites` feature; carry to U6.
- C13 fixes.rs is cfg(test) (tcp_model.rs:3014-3015): snapshot_budget must live in non-test code. Plan implies it.
- C14 table (a) recomputed: 2xTICK = 66,666,666: none->6, 20->3, 50->4, 100->6, 150->8, 300->15. OK.
- C15 container: kernel 6.18.44, CONFIG_HZ=250, iptables present, tc absent (no netem here).
- C16 kernel cites re-read in scratchpad linux-src-v6.8/net_ipv4_tcp.c: SIOCOUTQ answ = write_seq - snd_una (:629-630,
  0 in SYN_SENT/SYN_RECV, EINVAL on LISTEN); SIOCOUTQNSD = write_seq - snd_nxt; tcpi_rtt = srtt_us >> 3 (:3842).
  tcp_ipv4.c:2899 /proc tx_queue = write_seq - snd_una. OK.
- C17 the scout's kernel probe ran with tcp_congestion_control = bbr (u3-scout-timing.md:388); the model is Cubic.
  The cross-check must set cubic per socket (TCP_CONGESTION) or record bbr.
- C18 Link::send_unreliable is per message (link/src/lib.rs:66); fix 1's size is the whole tick's snapshot. U4/U6 must
  settle how WsServer sees a tick (one WS message per tick, or a tick boundary). Carry.
- C19 L2Config's profile doc says bursts are on ImpairSocket::now_ns (config.rs:21-23); DelayStream's clock differs.
- C20 ImpairSocket nets base RTT inline (impair.rs:104: (rtt - base)/2); DelayStream should share it.

## Verdict
APPROVE with conditions (listed in the report). R20 (a) approved; the unsafe exception is mine under ADR 0002:264-265.
Split approved with U6 carries. No escalation.

## Side ruling: the flaky loopback P2 test (wt_tests.rs:203), 2026-11-07
- Re-read .gate/flaky-runs.log (5 ms spacing, 80 impaired route-runs): worst wtq up 178 with 7 real up losses, 75 up
  packets reordered, 17 congestion events, quinn lost 32, cwnd 2,904. .gate/flaky-runs-30hz.log (50 runs): min up 191;
  mean 22.9 up packets reordered per run (ACK-only and other packets between the 33 ms datagrams); in all 50 runs and
  both directions, missing datagrams <= ImpairSocket's real losses (awk over the logs).
- APPROVE the fix (TICK pacing for P2, 5 ms clean reference kept, >= 180 kept, app == quinn frame_rx) with: the
  sender-side twin (quinn frame_tx.datagram == app sends); the exact check missing <= real lost per direction; the
  numbers saved in the checkpoint (.gate/ is gitignored); a comment naming why TICK and that C6 owns the rest.
- (a) yes, and more: C6 at 30 Hz with snapshot-sized down datagrams, P1-P4 and B1, a FullState mid-run; record cwnd,
  lost_packets, congestion_events, quinn's datagram drops vs ImpairSocket's real losses and reorder counts; plus one
  test-only order-keeping jitter variant to size the effect.
- (b) independent per-packet jitter IS the written model (ADR 0001 part 13 :569-584, :1471-1472; OQ-0001-16; PM-0001-3).
  Part 13's premise "jitter alone cannot reorder 30 Hz inputs" holds for the inputs only. Changing the model changes an
  ADR's profile definition: Mustafa's, but only if C6 shows a WebTransport mark moves; then before R3 with numbers.
  Not an escalation today; pending item JITTER-ORDER in the checkpoint.

## Side rulings: U3c evidence (2026-11-08)
- (1) Kernel cross-check: HALF A GAP confirmed as the judged rule (read run.sh:8-46 in worktree agent-a7fafe27...;
  compare.py:12, :92, :116, :123: one rule, one code path for kernel and model). Reason: the scout's RACK cluster sits
  AT one gap (45 stalls, p50 33.5 ms against a 33.3 ms gap, u3-scout-timing.md:392-393), so "more than one gap" would
  cut that cluster in two; my C11 wording was wrong. Added: per run, judged episodes <= the client's tcpi_total_retrans
  (probe.py:111 reads it), else VOID (receiver noise); every run kept and committed; one rerun per VOID/INVALID, a
  second comes to me; a MISS is never rerun. -I INPUT 1 approved. The header notes the change and its date.
- (2) Tests: switch to tokio-tungstenite now, with futures-util as a DEV-dependency pinned =0.3.34 (Cargo.lock:389;
  default-features off, ["sink", "std"] as tokio-tungstenite 0.30.0 asks): no new package, ships nothing. The Bridge
  (delay_tests.rs:176-292) is a hand copy of tokio-tungstenite's AllowStd with different waker handling, and that is
  where DelayStream's Pending/wake/flush meets the library. DelayStream keeps separate reader/writer/shutter wakers
  (delay.rs:216-219); add one loopback test with read and write halves in separate tasks. U6: tokio-tungstenite +
  futures-util as a normal dependency (listed in §4.8, audited at U5), no hand adapter in shipped code.
- FYI items: lateness evidence quiet, >= 10,000 samples per direction, loadavg recorded; W7 endorsed, with fixes.rs's
  pinned tables unchanged after the swap.

## Side ruling: condition 10 fired in the quiet run; the other evidence (2026-11-08)
- Read at 989118e: results/u3c-lateness/lateness-run1-20261108T0041Z.txt (n 10,051 each way, 335 s, loadavg 0.94 ->
  0.00): down p99.9 1.908 ms max 4.200 ms E 3.5 us; up p99.9 2.111 ms max 2.839 ms E 6.5 us; stalls 204 (155-247) and
  200 (152-243), all RACK. Kernel check out-20261108T004723Z/verdict.txt: PASS (final), guard 60/61, 54/56, 58/61;
  RACK p50 33.344 vs 33.458 ms; share 6/172 in 1-15; one-gap count kernel 100 vs model 5,921 (the split the half-gap
  ruling avoided); iptables before == after (diff). siocoutq-root-check-20261108T005258Z.txt: R0-R4 pass, VERDICT PASS.
- RULING (a) ACCEPT. The tail is about 100x below the WebSocket marks' resolution (-18's 10 ms grid and 10 %
  tolerance of ~654 ms; -31's 250 ms p99) and doesn't feed the model's fates (replay test); E meets U3-Q1 (c)'s
  statistic; ImpairSocket shows the same tails in this container. Not (b) now: moving the pumps off tokio means
  re-homing S's TLS stream on another runtime, with no mark to gain.
- Carried to R1's run plan: DelayStream's self-test on RS under the WS part's real load (48 connections, P2, room
  running), >= 10,000 samples per direction; judged E under about 10 us (U3-Q1 (c)'s statistic) and p99 <= 1 ms per
  direction; p99.9 and max reported; each release split into clock-thread wake, pump start, and S's poll_write return
  (stamps added before R1). A miss comes to me before -18's and -31's WS parts count; (b) is the prepared fallback.

## Side ruling: condition 10 again, the post-restart A/B (2026-11-08)
- Checked at e317067: the six files' summary lines match the table (E down 11.6-22.1 us, up 25.9-38.2 us; p99 up
  1.37-1.88 ms; max up to 35.3 ms). Pre's delay.rs, delay_clock.rs and ws_frame.rs hash equal to 95597ef's, and
  `git diff 95597ef 989118e -- crates/net/src/` is empty, so run 1 and the A/B "pre" runs ran the same DelayStream:
  the change from E 3.5/6.5 us to 12-28 us on the same code is the environment. Pre and post interleave with no
  consistent order: F3 neither caused nor cured it.
- RULING: (a) stands, plus one condition. The WebSocket marks read medians and p99 (-18 on a 10 ms grid with a 10 %
  tolerance of ~654 ms; -31's 250 ms p99): a lateness p99 under 2 ms and isolated maxima up to 35 ms move none of
  them. The container is not evidence for a us-level bar; RS decides at R1 with the bar already set (E under about
  10 us and p99 <= 1 ms per direction, under 48 connections; p99.9 and max reported; a max over 10 ms there comes to
  me with the result).
- ADDED, before U3c's report: build the per-release split now (clock-thread wake, pump start, S's write return; the
  three parts must sum to the total), then one quiet diagnostic run here with ImpairSocket's M13 lateness run
  interleaved, both reported, not judged. Why now: it tells whether the tail sits in the tokio hop (then (b) gets
  built before R1) or in the thread wake that ImpairSocket shares (then the VM, and (b) would not help), without
  spending an R1 VM day on attribution. No plain quiet rerun: it adds nothing.

## Side rulings: U3c's last fix round (2026-11-08)
- Read u3c-fix-review-result.txt (R1 :~60, G-NC8 :85, G-PRE-WAKE :92, G-PRE-PANIC :99, C2R-1..10 :200-330).
- (1) C1b: no objection. Plus: R1's test is the critic's probe 1 (fails when reverted), probe 2's timings recorded back
  at pre-fix values; G-PRE-WAKE in the same change; G-PRE-PANIC tested with a panicking S; one critic pass on C1b's
  diff only (R1 slipped through a fix pass). Run 2 and the A/B "post" runs carried R1: the environment reading rests
  on the "pre" runs alone (E 11.6-28.2 us on unchanged code), which still holds. C10's diagnostic after C1b.
- (2) check.mjs scope (a)-(d) approved. Added to (b): try the critic's compiler backstop (workspace unsafe_code =
  "forbid"; the two exceptions keep their own [lints] tables, check.mjs checks them equal to the workspace's other
  lints); keep it only if check.mjs passes 9/9 on both builds (an external macro's allow(unsafe_code) would break it);
  if kept, C2R-1 and C2R-2's unsafe half leave (d). Each refusal gets a test and must not refuse this container's
  normal toolchain environment. Supersedes my 12a's "keep [lints] workspace = true" for npt-tcpinfo if forbid lands.
- (3) Split memory: (b) approved. Updates without perturbing the measure (relaxed per-bucket atomics, or a lock only
  for the increment, never across an await or under a Conn lock; no allocation on the release path); the per-release
  sum check kept as a counted violation; pooled per direction (what R1 reads); per-connection footprint back to about
  309,583 B, the shared part fixed per process, both recorded. (a) would not help: the 101 is a release. Carry to U6:
  accept cap x 0.31 MB recorded; whether -25's sessions ever get a DelayStream (P0 refuses the wrapper).

## Side ruling: the condition-10 diagnostic, UP-STAMP and U3c's close (2026-11-08)
(Returned by the advisor at ecdeeb0, read-only; appended by the main session.)
- **Checked at ecdeeb0** (code = f6cd1d7's: `git diff --stat f6cd1d7 ecdeeb0` touches only the result file, the
  checkpoint and the report). results/u3c-lateness/c10-diagnostic-20261108T2152Z.txt: every number in the main
  session's (a)-(e) matches (splits :25-47 and :83-105; ImpairSocket :62-63 and :120-121; totals :24, :36, :82, :94;
  run 1's E 3.5 / 6.5 us at lateness-run1-20261108T0041Z.txt:12-13). Woken + other = 10,051 per direction and run;
  the other subclasses add up; violations 0 in all four rows; each listed largest release sums to its L within
  0.001 ms. Trigger p99.9 > 1 ms or max > 10 ms (delay_tests.rs:196-197). The split's definitions (delay.rs:63-82)
  match the code (opening step :1711-1715; closing block and the reader's wake :1874-1884). Both release threads
  use the same mechanism (1 ns slack and a Condvar wait_timeout to the due time: delay_clock.rs:207, :261;
  socket.rs:651, :698). /proc/stat since boot: steal 0.47 % (supports the VM reading, not tied to any one spike).
  The gate --no-smoke passes on the working tree (base 626df67).
- **What the split shows (reported, not judged):**
  - No single part holds the tail: in the 20 largest releases listed, one part holds nearly all of L, work in 9,
    clock in 7, hop in 4.
  - The clock part is the VM's wake: the release thread's own OS wake (one entry; nothing of tokio or our code in
    it), with a tail (p99.9 1.6-2.3 ms, max 3.1-6.8 ms) as large as the whole; load average 0.01 at both runs' ends.
  - Hop against clock: the main session's (c) holds for the tail (the hop is no larger at p99.9, max and E in all
    four rows). At p50 and p99 the hop is slightly larger (0.065-0.070 against 0.051-0.054 ms; 0.24-0.32 against
    0.16-0.26 ms): the cost of unparking a parked worker, which (b) would swap for a thread wake of the same kind.
  - Up's p99, the bar's own statistic, sits in the work part (0.83 / 1.01 ms against clock 0.20 / 0.23 and hop
    0.27 / 0.32); up's total p99 here (1.44 / 1.77 ms) is over R1's p99 <= 1 ms bar; run 1 gave 0.509 ms on the same
    pre-fix code, so it is environment-sensitive.
  - ImpairSocket is a scale check, not like for like: its lateness covers its whole path (send, the kernel's
    loopback, recv) at 10,000 per second each way, its thread at 0.39-0.41 of a core. Its own up E (11.6 / 16.9 us)
    is now over U3-Q1 (c)'s "about 10 us" in this container too: U3d reports it, RS judges it.
  - E does not add across parts (each part's E is against its own p50 + 0.25 ms: stats.rs:58, :105): say "the
    largest part's E", never a share.
- **Correction: up's work is not shown to be the reader's turn.** The pump wakes the reader only at its pass's end
  (delay.rs:1882-1884), so up's work = the pump's whole pass after its opening step (its down writes into S, S's
  reads, the up loop) plus the reader's dispatch and poll. tokio puts a task woken from inside a worker into that
  worker's LIFO slot: no other worker is notified and none can steal it (tokio 1.54.1 runtime/scheduler/multi_thread/
  worker.rs:1386-1416, :478-481), so the reader runs on the pump's worker once the pump's poll returns. The split
  cannot tell the pass from the reader's turn; either way (b) would not remove it (the same pass on another thread,
  still waking the reader at its end).
- **RULING C10-B: APPROVE: (b) is not built before R1.** The hop does not own the tail; the release thread's own OS
  wake carries one of the same size, which (b) cannot help. **Fallback order if R1's DelayStream self-test misses:**
  (1) the split, with the new stamp, comes to me first; (2) if up's miss sits in the pass part, the first remedy to
  measure is inside the pump: wake the reader right after the step that releases its record, and yield (a task woken
  during its own poll is rescheduled as a yield: tokio task/harness.rs:155-161; scheduler/multi_thread/handle.rs:
  122-123), so the reader runs first; (3) (b) stays the fallback for a miss in the hop.
- **RULING UP-STAMP: APPROVE, changed:**
  1. Stamp R at the reader's wake (the closing `wake_all`, delay.rs:1884), not at the release (already P or T). Up's
     woken work then splits into pass = R − P and reader = (3) − R; in the other class the pass is from T.
  2. A reader that returns before R (the record is readable once the opening step's lock drops, so a reader awake
     for its own reasons can take it first) gets its own counted class with no reader part, so the sum stays exact;
     an identity test and a misattribution bite, as b5c9009 did.
  3. Built in U6, not U8: R1 measures WsServer's reader (its own task, split halves), so U6's loopback self-test of
     WsServer reports the four parts before the run package is built.
  4. The bar stays on L as defined: L ends where the application has the bytes; a kernel socket's delivery also waits
     for its reader's turn, so the bar errs on the strict side.
  5. Record steal at R1: /proc/stat's steal read before and after each DelayStream and ImpairSocket self-test run (two
     reads, nothing in the measured path).
  - Plan text: plan.md:1225 (U6 row) gains items 1-3; plan.md:1227 (U8 row): "each release split into the clock wake,
    the pump's start and S's write, stamps added in U6 or U8" becomes "the split built in U3c (b5c9009) plus U6's
    reader stamp; steal recorded; a max over 10 ms comes to the advisor with the result" (the max rule is from my
    "condition 10 again" ruling, not yet in the row).
- **RULING U3C-CLOSE: APPROVE: U3c closes when all hold:**
  1. C2d: its critic returns complete with no regression; each of N1-N5 fixed with a test that fails on revert or, if
     a deliberate-only way around (as N3 may be), named in the known limits. A regression or a new finding at low or
     above comes to me.
  2. Re-run on the closing head: after C2d merges, check.mjs (9/9) and the gate (--no-smoke, with its reason); the
     nine step lines with their counts and the head SHA in the report itself (today it cites "the commit messages").
  3. Report fixes (u3c-report.md): (a) "What happened": the A/B showed the rise after run 1 came with no code change,
     not that the remaining tail is not ours: say no single part holds the tail, the release thread's own OS wake
     carries one of the same size, and up's p99 sits in up's work part, to be split at U6. (b) The diagnostic row:
     the parts' p99s; "the largest part's E (the parts' E do not add)" for "most of up's E"; the split cannot yet
     tell the pass from the reader's turn; ImpairSocket as a scale check (10,000 per second, its whole path); the
     three verdicts for "TBD". (c) Known limits: "p99.9 up to about 3.5 ms" -> about 3.7 ms (lateness-ab-4: up p99.9
     3.719 ms); add plainly that since the restart up's p99 here is 1.37-1.88 ms, over R1's p99 <= 1 ms bar
     (checkpoint :639 says so; the report does not). (d) Carried forward: U6 the reader stamp; U8/R1 the split built
     and reported at R1 with the reader stamp and steal recorded, R1 following the fallback order above.
  4. Plan text: the U6 and U8 rows as above, in the same commit that folds the report.
  5. The checkpoint pushed with the close (§12.1's unit rule); the scratch copies the critics flagged go in Mustafa's
     cleanup ask (no agent deletes them; e.g. .gate/c1c-review-scratch shares its unit hash in the shared target:
     u3c-fix3-review-c1c.md:41).
  - Nothing else wrong found: the kernel cross-check passed on its only run (one out- folder, committed); the
    check.mjs counts agree (C2c's 258 / 272 plus C1c's two tests make 260 / 274); u3c-plan.md 1.8 (a) is marked
    superseded (:275) and the tcpinfo manifest says why (crates/tcpinfo/Cargo.toml:9-12); plan §4.2 and §4.8 match
    the forbid-plus-two-tables layout and the four unsafe blocks (spikes/netcode-proof/Cargo.toml:87); the U6 row
    carries the split pool's one run per process and the shutdown error after an unclean close; the request-cap
    difference is recorded at config.rs:66-68.
  - Not checked by me: check.mjs 9/9 (not re-run while C2d builds in a shared target); "no agent or build ran" during
    the diagnostic (the load-average lines are consistent with it); C2d's result (not in yet).
- **Verdicts:** C10-B APPROVE ((b) not built before R1; up's reason corrected). UP-STAMP APPROVE with changes (stamp at
  the reader's wake, built in U6 with an early-reader class, an identity test and a bite; bar on L; steal at R1; U6
  and U8 rows updated). U3C-CLOSE APPROVE on five conditions (above). Decisions needed: none (all three are mine).
  For Mustafa: nothing from this ruling.
