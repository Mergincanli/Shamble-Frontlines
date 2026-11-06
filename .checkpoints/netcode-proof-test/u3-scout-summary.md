# U3 scouting: the synthesis (workflow wf_d6bdeed6-3c0, 4 of 4 agents, DONE 2026-11-06 ~17:30 UTC)
Sources: `u3-scout-wt.md` (netcode-architect), `u3-verify-wt.md` (verifier), `u3-scout-timing.md` (rust-reviewer),
`u3-verify-timing.md` (verifier); probes `u3-scout-wt-probe.rs`, `u3-timer-probe.rs`, `u3-verify-*-probe.rs`,
`u3-tcp-stall-probe.py` (throwaway; builds under `.gate/netcode-proof-test/u3-*`). The workflow's journal (outside the
repository): `/root/.claude/projects/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/subagents/workflows/wf_d6bdeed6-3c0/journal.jsonl`.
Verdicts: WebTransport scout 16 claims, 14 confirmed, 2 refuted; timing scout 15 claims, 12 confirmed, 2 unclear,
1 refuted. Refuted or unclear claims are listed below; use only confirmed claims without re-checking.

## The answer to R5-TEST-OQ2: YES for both crates (R-22 does not trigger)
- Neither crate's own builder takes a custom socket (both bind a real one), but both accept connections from a quinn
  endpoint we build: `quinn::Endpoint::new_with_abstract_socket(EndpointConfig, Some(ServerConfig),
  Arc<ImpairSocket>, Arc::new(quinn::TokioRuntime))` (quinn-0.11.12 endpoint.rs:133).
- wtransport 0.7.2: its `quinn` feature (no new dependencies; Cargo.lock unchanged) opens
  `IncomingSessionFuture::with_quic_incoming(quinn::Incoming)`; then `req.accept()`. It reaches
  `datagram_send_buffer_space` only through `quic_connection()` (quinn feature) and panics (`unreachable!`) on
  `SendDatagramError::Disabled`; it never validates the client's SETTINGS (endpoint.rs:628 TODO).
- web-transport-quinn 0.13.6: built in: `Request::accept(quinn::Connection)` then `req.ok()`; `Session` derefs to
  `quinn::Connection` (session.rs:639-640). `Server::new(endpoint)` also works but drops failed handshakes silently.
- Both link one quinn 0.11.12 / quinn-proto 0.11.19 / quinn-udp 0.5.16, so one ImpairSocket type fits both.
- A throwaway probe in this container: a delaying ImpairSocket under each crate accepted a real WebTransport session,
  echoed datagrams, saw the real peer address, and quinn's RTT carried the injected 2 x 20 ms (echo p50 40.65-41.15 ms).
  Its scope: a fixed delay in one direction; its pacer reached 6.5 kpps, not 10 kpps (refuted claim C15).

## ImpairSocket's shape (confirmed parts)
- It must behave like a network: never WouldBlock; one datagram per Transmit (`max_transmit_segments` = 1 and
  `max_receive_segments` = 1, so GSO/GRO are off; L2 then costs more CPU than the plain path, so only P0 through
  ImpairSocket is like-for-like and the clean references must run through the same clock mode); `poll_recv` returns
  only due datagrams and stores the waker, never `Ok(0)` or an error for an empty queue; it copies quinn's buffers
  (Transmit borrows: a buffer pool at 20,000/s); `may_fragment` passes through.
- The release clock is a dedicated OS thread (tokio timers have 1 ms granularity: lateness p50 1.19-1.26 ms).
- Fates: one ChaCha8 stream per (peer, direction) with fixed words per datagram, as L1 (not one RNG per direction:
  interleaving would move fates). The reorder step = L1's as-built R-5 step (held until 1 ns after its successor),
  so PM-0001-3 reads one count on both layers (C5). Jitter: independent per datagram (L1 already answers this).
- quinn-udp's `UdpSocketState::send` turns non-WouldBlock errors into Ok (unix.rs:195-219): use `try_send` and log
  each error kind, or the fate log counts kernel send failures as delivered. RecvMeta has no kernel receive stamp.
- PM-0001-23/-24 need a shaper: a FIFO at rate R with a drop-tail byte limit (payload + 28 B). R-17's source
  (review2-findings.md:50, NC2-F3) says -24's deep queue is intended: a server that ignores the bottleneck fails on age.
- The control plane keeps an `Arc<ImpairSocket>` (E23); with every impairment off it is the P0 pass-through.
- One quinn `ServerConfig` for both crates: through wtransport's builder would make the wt-wtq path depend on
  wtransport (against one crate per feature); a hand-built rustls config needs ALPN `h3`, TLS 1.3, and one crypto
  provider (a dual-provider panic when both features compile). `with_custom_tls_and_transport` sets no ALPN.
- A small `datagram_send_buffer_size` below k x (payload + size_of Datagram) makes every send TooLarge.

## Timing (the advisor's 0.25 ms condition)
- No mechanism met 0.25 ms at p99 in this shared container; a plain thread sleep misses at p99 (319 us-3.0 ms); a
  200 us spin tail meets it at the median (23 us). Run-to-run variance is large (p99.9 2.1-16 ms between identical
  runs), so one-run comparisons decide nothing (unclear claim C4: SCHED_FIFO's edge within noise).
- Timer slack: `/proc/thread-self/timerslack_ns` does not exist; `/proc/<tid>/timerslack_ns` does (refuted C11);
  slack 1 ns or SCHED_FIFO met p99 0.25 ms in some runs here.
- The verifier's reading of -3's real need: the expected excess lateness, E[(L - 0.25 ms)+] about 10 us or less at
  P1 (the tightest band), measured at -3's real rate (10,000/s per direction = 20,000 releases/s), with the inbound
  lateness measured where quinn polls, not in the release thread.

## Linux's TCP repair vs E15's rule (the WebSocket DelayStream)
- With steady traffic, Linux repairs one lost segment by RACK in about RTT + 33 ms; a tail loss by TLP (2 x SRTT);
  otherwise the RTO, SRTT + max(RTTVAR, 200 ms) (about 300 ms at P2), doubling. E15's max(200 ms, 2 x RTT) sits
  between RACK and the RTO. This container's 204 ms RTO implies HZ=250.
- Consequence: a Linux-faithful model makes PM-0001-18's B1 recovery about 1.1 s, above its 500 ms limit.
- tokio-tungstenite 0.30 accepts any `AsyncRead + AsyncWrite + Unpin`, so a DelayStream can sit in the accept path.
  `wss` needs tokio-rustls, which is neither pinned nor in the registry (a new dependency: plan §4.8's review).
  Placement options: below TLS (TcpStream -> DelayStream -> TLS -> tungstenite) or above TLS and below tungstenite
  (frames visible, so the message size limit applies literally before the delay buffer). DelayStream must expose its
  free budget as the send-buffer space npt-room reads (§4.4).

## Questions for the advisor (U3), before U3's build plan
1. The 0.25 ms condition's statistic and direction: p99 or p99.9 (offset recorded), or the expected excess at P1;
   measured at 20,000 releases/s; inbound where quinn polls; on RS at R1.
2. Release-thread privileges: SCHED_FIFO through the systemd unit (CAP_SYS_NICE / LimitRTPRIO), timer slack via
   `/proc/self/timerslack_ns` without unsafe code (the workspace forbids `unsafe_code` outside npt-client-wasm), or a
   spin lead (about 0.07 core per 1,000 releases/s).
3. E15's repair constant: keep max(200 ms, 2 x RTT) documented as between RACK and the RTO, or a lazy RACK/TLP/RTO
   model in DelayStream (and L1's C1), given -18's 500 ms limit against a faithful 1.1 s.
4. "The message size limit applies before buffering": the reading with a wrapper below TLS, or place the wrapper
   above TLS.
5. tokio-rustls: pin which version, fetched in U3 or U6.
6. QUIC migration on L2 runs: `migration(false)`, or re-key the per-peer profile.
7. npt-net's public surface: `ImpairSocket::bind(addr, cfg)` adds an L2 config type (and profile/control types) beyond
   plan §4.7's list.
8. The Join deadline with a delayed Incoming (it must not run "from the end of a delay": §4.4) and size limits before
   any delay buffering: where in npt-server.
9. RS facts to read at R1 before judging timing: kernel (6.8 GA or HWE), CONFIG_HZ, cpuidle driver, clocksource.
DONE
