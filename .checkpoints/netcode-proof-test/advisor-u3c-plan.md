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
