# U3c plan: DelayStream, fix 1's send gate, the mechanism tests and the kernel cross-check (for the advisor)

Branch claude/netcode-proof-test-plan-hcky5g, head a8abd91 (code as at 7321cc0). Written 2026-11-07 about 19:40 UTC.

Sources:
- u3-build-plan.md, U3c line.
- advisor-u3q.md: M16, M17; S0-S14, especially S0, S9, S10, S12 and S13's cross-check.
- advisor-ws-docs-r2.md:383-386 and :410: U3c items (1)-(7), mechanism tests (a)-(e), the plan text, R20.
- verify-ws-docs-r2.md:47: W22, which mirrors fixes.rs:777.
- The catalogue, docs/design/netcode-proof-messages.md:61-78: WebSocket frame and message limits 513 B; the write buffer 128 KB.
- Plan §4.4: size limits before any delay buffering; the Join deadline from TCP accept; send-buffer space to npt-room.
- Plan §4.7: `DelayStream` is npt-net's listed public item.
- Mustafa's U3-Q4 (b): above TLS, below tungstenite. U3-Q5: tokio-rustls pinned in U3, audited at U5.

## 1. What U3c builds (npt-net; everything pub(crate) except DelayStream)

### 1.1 tokio-rustls (M16)
- `tokio-rustls = { version = "=0.26.6", default-features = false }` in the workspace; npt-net depends on it.
  - Published 2026-09-28. It depends on rustls ^0.23.27 with default features off, so it resolves to the pinned 0.23.46 and there is no second rustls; `cargo tree -d` will check.
  - Its default features (aws_lc_rs, tls12, logging) stay off. The provider is chosen explicitly: ring through `ServerConfig::builder_with_provider` and `rustls::crypto::ring::default_provider()`, as M7 does for the WebTransport crates. TLS 1.3 only, ALPN "http/1.1".
- Cargo.lock gains the tokio-rustls entry only; npt-net's entry gains it too.
- U5 audits it (§4.8).

### 1.2 DelayStream (S0-S12 over a real stream)
**Shape.**
- It wraps the server's TLS stream S: above TLS, below tokio-tungstenite.
- One `TcpModel` per connection: end 0 is RS, end 1 the client. It uses `TcpConfig::default()`: Linux 6.8 and the 64 KB budget.
- The profile is L2Config's, its one-way delay net of `base_rtt_ns` as ImpairSocket's is (PM-0001-3's reading).
- Fates come from one ChaCha8 stream per (L2Config.seed, connection index, direction) (S10). The connection index is given by the accept path (first accepted = 0, recorded like ImpairSocket's slot ordinal).
- The real TCP and TLS handshakes run undelayed, below the wrapper. This is recorded, as the docs PR's -29 note already says.

**TLS records (S0).**
- Each poll_write chunk becomes TLS records of at most 16,384 B of plaintext. Each record counts as plaintext + 22 B in the model, as rustls cuts records per write call.
- A down record's plaintext is released at the client end only once its whole record is delivered in order: a TLS record cannot be decrypted earlier.
- Up direction: the client's plaintext, read from S, is cut into records at WebSocket frame boundaries, each + 22 B. The upgrade request is one record. Each record is written into the model at end 1 once the whole frame has been read; fixes.rs's INPUT = 60 + 6 + 22 uses the same rule. It is released to tungstenite once delivered in order at end 0.

**The write side (S9).**
- poll_write takes at most the free budget less 22 B, where the free budget = model.free_budget(0) less the bytes released but not yet taken by S. The bytes held therefore never exceed 64 KB.
- Pending beyond that, woken when the budget frees; never Ready(Ok(0)) for a non-empty buffer.
- poll_flush is Ready once the bytes are in the model, as a kernel socket's flush is.
- poll_shutdown completes after everything written has been delivered at the client end and S has taken it; then S shuts down. An EOF from S ends the up direction after its last delivered byte.

**The pump.**
- DelayStream::wrap spawns one tokio task per connection on the current runtime. It owns S.
- It advances the model event by event (next_event, then advance(t)), so every release carries its model stamp.
- It writes released down records into S at once.
- It reads S only while the model's end 1 has room for a whole frame plus 22 B. The client's real socket then pushes back, as a full send buffer would.
- It hands released up records to tungstenite's reader.

**The release clock (M14's pattern).**
- One process-wide dedicated thread, 1 ns timer slack written and read back (l2::thread), a min-heap of (due, seq, connection) and a Condvar.
- At each model event's due time it wakes that connection's pump. No tokio timer sits on the release path (tokio's resolution is 1 ms).
- Its origin is DelayStream's clock (`DelayStream::now_ns()`, an associated fn), on which the profile's bursts are stamped.

**Lateness (M1's pair).**
- Down: measured at the return of S's poll_write for each released record. Up: at poll_read's return. Both against the model's delivery stamp.
- The same histograms and E[(L - L0 - 0.25 ms)+] as l2::stats. Reported only: U3-Q1's bar is ImpairSocket's. The advisor rules if DelayStream's E is far above it.

**Limits before any delay buffering** (plan §4.4; catalogue 513 B).
- The pump parses the upgrade request (cap 8,192 B, proposed: nginx's large_client_header_buffers default), then each frame header per RFC 6455 §5.2.
- These close the connection with a counted reason before any of the payload enters the model:
  - a frame, or a fragmented message's running total, above 513 B;
  - an unmasked client frame (§5.1);
  - a control frame above 125 B, or a fragmented control frame (§5.5);
  - a 64-bit length with its top bit set;
  - a request over its cap.
- tungstenite gets the same limits as well: WebSocketConfig max_frame_size and max_message_size 513, max_write_buffer_size 128 KB per the catalogue, write_buffer_size 0 so each send writes through.

**Held bytes.**
- Per connection: down at most 64 KB (the budget covers released-but-untaken bytes too); up at most 64 KB in the model, 64 KB released but unread, one frame in parsing (513 + 14 B) and the request cap.
- The pump stops reading S when the up side is full. A counter keeps the current and the largest held bytes per direction.
- Nothing grows with client data beyond these.

**Join deadline (§4.4).** The accept path passes in the TCP accept stamp (`accepted_ns`), never the end of a delay. npt-server reads it at U6.

**Per-return record.** At each burst's end (profile.bursts_ns), as the model passes it, the pump records:
- the stamp;
- outstanding(0) and srtt_ns(0);
- the bytes released but not yet taken by S.

This feeds mechanism test (c) and the SERIES burst_return fields that DelayStream owns. Offset and tick phase are the room's, at U6.

**Accessors (pub(crate)), for the send gate and npt-server:**
- srtt_ns() is the model's end 0: what tcpi_rtt means (tcp.c:3842).
- outstanding_bytes() is write_seq - snd_una: what SIOCOUTQ means (tcp.c:630).
- free_bytes() is the room in the 64 KB budget: the send-buffer space npt-room gets (§4.4).
- counters (S12), the header and per-second rows (M15), and the run summary.

**Public surface:** DelayStream only (§4.7).
- `DelayStream::wrap(stream, cfg: &L2Config, conn: u32, accepted_ns: u64) -> io::Result<DelayStream<S>>`, the AsyncRead and AsyncWrite impls, `now_ns()`, and read-only accessors returning std types.
- L2Config gains two fields, `ws_max_message_bytes` (513) and `ws_max_request_bytes` (8,192): fields, not new symbols.
- A pass-through L2Config is refused by wrap (R20: clean WebSocket runs go without the wrapper).

### 1.3 Fix 1's send gate (advisor-ws-docs-r2.md U3c (2), (3); W22)
- `pub(crate) fn snapshot_budget(srtt_ns: Option<u64>, tick_ns: u64) -> u64`: N = (3 x srtt).div_ceil(2 x tick) + 1, at least 3, with srtt = 100 ms before the first sample.
  - fixes.rs's adaptive_limit becomes snapshot_budget(m.srtt_ns(0), TICK) x SNAP: one definition, test (e).
- `pub(crate) fn may_write(outstanding, owed, free, size, n) -> bool` is true only when all three of fixes.rs:777's conditions hold (W22):
  - owed == 0: nothing held above the stream, so tungstenite's write buffer is empty;
  - free >= size: the stream's 64 KB room takes the whole snapshot;
  - outstanding + size <= n x size.
  - Sizes are in the model's bytes (payload + WebSocket header + 22 B per record), as fixes.rs's SNAP = 1,050 is.
- **Where it runs.** In npt-link's WebSocket backend inside WsServer, per plan §4.7: "newest-only replace sits inside its backend". WsServer itself is U6's (plan §12.1).
  - U3c builds the two functions plus a cfg(test) writer that applies them each tick over a real loopback WebSocket.
  - U6's WsServer Link calls the same functions from send_unreliable (Sent, or BufferFull with the skip counted, never written later) and re-runs mechanism test (c) on WsServer.
  - **Proposed split; the advisor rules.** The alternative is building the WsServer Link's send path now. It needs U6's I/O-task and channel design, so it would be half of U6.

### 1.4 Mechanism tests (advisor-ws-docs-r2.md:385; in check.mjs's cargo test)
- **(a) The table at TICK = 33,333,333 ns:** no sample gives 6; 20 ms gives 3; 50 ms 4; 100 ms 6; 150 ms 8; 300 ms 15.
- **(b) Seeded sweeps** (no new dependency: proptest is not in the workspace):
  - N never decreases as srtt grows and never falls below 3;
  - a write happens if and only if the three conditions hold;
  - after any write, outstanding <= N x size;
  - writes happen only on a tick;
  - a skipped snapshot is never written later;
  - every written snapshot is the newest at its tick.
- **(c) On DelayStream at P2 under B1**, through TLS and tokio-tungstenite on loopback:
  - at every return, outstanding <= N(srtt then) x size, from the per-return record;
  - owed = 0 at each write.
- **(d) Bites, each of which must fail**, as test-only variants in the style of tcp_model's Variant:
  - budget_scale 2 fails (a) and (c);
  - "outstanding = unsent only" (the SIOCOUTQNSD mistake) fails (c);
  - a queueing writer fails (b) and -31's "at most one snapshot above the socket".
- **(e) One function:** fixes.rs's adaptive_limit calls snapshot_budget, plus a grid test that both give the same N.

### 1.5 DelayStream's own tests
- **Unit** (S as an in-memory duplex):
  - order and integrity both ways (hashes);
  - every release no earlier than its model stamp;
  - Pending at a full budget and woken on free; never Ok(0);
  - free drops by len + 22 per record, and records split at 16,384 B;
  - the request is one record;
  - an oversize frame, oversize message, unmasked frame, oversize control frame or oversize request each closes before buffering, counted. Bite: with the check off, the payload enters the model.
  - the held-bytes bounds hold under a reader that never reads;
  - wrap refuses a pass-through config.
- **Replay:** the run's model counters and delivery stamps equal those of a pure TcpModel driven by the logged (stamp, record) writes. The real path moves no fate (S10).
- **Loopback TLS** (tokio-rustls, ring, TLS 1.3) + DelayStream + tokio-tungstenite, 30 Hz both ways at P2:
  - the 101 is delayed by the model;
  - each message's delay equals its model delivery stamp minus its write stamp, plus the lateness;
  - the stall count is inside its binomial band at P2: S13 (f), on the real path;
  - lateness is reported (p50/p99/p99.9/max, E).
- **TLS config test:**
  - TLS 1.3 only; ALPN http/1.1; the ring provider, with no process default.
  - Both WebTransport features and the WebSocket server in one process (M7's one-process check, extended).

### 1.6 The kernel cross-check (S13's; an evidence run, not in check.mjs because it needs root and iptables)
- The pattern of the scout's u3-tcp-stall-probe.py (it already ran here):
  - lo, a 2 % random drop on client-to-server packets only (iptables statistic);
  - 30 Hz, 200 B, TCP_NODELAY;
  - 3,000 messages x 3 runs.
- It is compared with the model at the loopback's measured RTT, with loss on the data direction only (a cfg(test) knob in tcp_model).
- Pass:
  - the RACK-path stall p50 within 2 ms of the kernel's;
  - the timeout-path share within its binomial band.
- This container runs 6.18.44 with HZ 250 (the scout's 204 ms RTO), not 6.8; that is recorded. R1's S14 check on RS is the binding one.

### 1.7 SERIES.md rows (M15)
- **Header:** the model's name and version ("linux-6.8-tcp v3"); TcpConfig; the S11 list; the clock's slack and policy; DelayStream's limits; the connection index rule.
- **Per second, per direction:** segments; transmissions; lost by cause; repairs by path; max backoff; stalls (count, max, sum); held bytes max; old-duplicate discards; D-SACKs; undos; release lateness summary.
- **Per connection at its end:** a ws_summary.
- The -18 per-return series (burst_heal, event.burst_return) stays with the spike-branch follow-ons after #45 merges (advisor ORDER (3)). DelayStream already records its share of the fields (1.2).

### 1.8 R20, for the advisor's ruling before it is built
**The problem.** Fix 1 on WebSocket connections without DelayStream (P0, ML's real path) needs the kernel's numbers: the smoothed RTT (TCP_INFO's tcpi_rtt) and the unsent + unacknowledged bytes (SIOCOUTQ).

**Facts (grepped):**
- socket2 0.6.5 (in the lock) exposes neither; only tcp_notsent_lowat.
- The latest rustix (1.1.6) and nix (0.31.3) expose neither safely. rustix has typed TCP options but no tcp_info; any ioctl is unsafe in both.
- The workspace denies unsafe_code, and npt-client-wasm is the only exception ("nothing else may": Cargo.toml:56-58).
- libc 0.2.191 is already in the lock (through tokio, mio and quinn-udp) and has `tcp_info` (linux gnu) and `TIOCOUTQ` (= SIOCOUTQ).
- M14 bars unsafe and libc in npt-net.

**(a) Recommended: a new workspace crate `npt-tcpinfo`**, Linux only, two functions on a `BorrowedFd`:
- `srtt_us(fd) -> io::Result<u32>`: getsockopt(IPPROTO_TCP, TCP_INFO), returning tcpi_rtt.
- `unacked_and_unsent(fd) -> io::Result<u32>`: ioctl(SIOCOUTQ).
- About 40 lines: two unsafe blocks with SAFETY comments, `clippy::undocumented_unsafe_blocks` denied, and the crate's own `#![allow(unsafe_code)]` as the workspace's second exception.
- Tests on a loopback pair with a small SO_RCVBUF and a peer that doesn't read:
  - SIOCOUTQ grows and equals /proc/net/tcp's tx_queue (also write_seq - snd_una);
  - tcpi_rtt > 0 after traffic.
- npt-net depends on it, and no third-party code is added.
- Plan text first (M6's rule): §4.1's crate list; §4.2 (it depends on libc only, and only npt-net uses it); §4.7 (its two functions); and the workspace lint comment.
- U5's audit covers it. WsServer (U6) calls it per tick per connection: 2 syscalls x 48 x 30 Hz = 2,880 a second.

**(b) Netlink sock_diag:** INET_DIAG with INET_DIAG_INFO, where idiag_wqueue = write_seq - snd_una (tcp_diag.c) and tcp_info has the RTT.
- No unsafe of ours.
- But at least four new third-party crates to pin and audit (netlink-packet-sock-diag, -core, -utils, netlink-sys, plus their dependencies), and a netlink request per connection per tick or one dump per tick.

**(c) Refused by R20:** a pass-through DelayStream.

**If (a): when and who.**
- When: built in U3c if approved now; it is small and testable here.
- Whether the unsafe exception needs Mustafa: the advisor decides. It is a workspace rule from U1, not an ADR constraint.

### 1.9 Plan text (advisor-ws-docs-r2.md:386; spike branch, this unit's first commit, before code)
- §4.3 E15's WebSocket sentence becomes the Linux-faithful model (S0-S14, v3) plus fix 1's readings.
- §4.4: WsServer applies the budget, and npt-room sees BufferFull and the skipped count.
- §4.7: npt-tcpinfo, if (a).
- §12.1's U3 row.
- §15: R-25's row (the faithful model) and a new "WS-BAR's rule" row.
- §0.5: Mustafa's 2026-11-07 words verbatim (13:45 and 14:30) and where each is met.
- §16: the docs PR's go.

## 2. Order
1. The advisor rules on this plan, R20 included.
2. Plan text (1.9).
3. tokio-rustls pin; the gate functions and mechanism tests (a), (b), (e).
4. DelayStream (clock, pump, records, limits, counters, stats) with its unit and replay tests.
5. Loopback TLS + tungstenite tests; mechanism test (c) and the bites (d).
6. npt-tcpinfo, if (a).
7. SERIES rows.
8. The kernel cross-check run.
9. check.mjs 9/9, the gate, a commit and a checkpoint after each of steps 3-8.
10. Then one skeptic round on U3c's diff: rust-reviewer, netcode-architect and security-auditor (network input, a new dependency, the unsafe crate). This is the review step's set, run once on the final diff, as for U3b.

## 3. Build route
- A build agent in an isolated worktree (general-purpose, opus, xhigh), as for U3b. Its notes stay inside the worktree, and the main session copies them over, merges and runs check.mjs.
- fixes.rs's adaptive_limit change (3 lines) is applied at merge time by the main session, so nothing else edits fixes.rs meanwhile.
- The run-structured bound waits for the scene's message sizes (U4) and is not in U3c.

## 4. Risks
- **Release precision.** Through a tokio task wake it is probably 10-50 us at p50. It is measured and reported, against ImpairSocket's in the same container.
- **Record boundaries are approximations:** down at poll_write chunks, up at frame boundaries. Real TLS records differ, invisibly on loopback. This is recorded in the header.
- **A stuck S** (the real socket full) holds released bytes. They count against the 64 KB budget, so nothing grows. Its maximum is recorded; on loopback it is expected to be 0.
- **A large write.** tungstenite's 128 KB write buffer holds a 65,541 B FullState while DelayStream takes it in budget-sized pieces. owed > 0 then makes the gate skip snapshots, as fixes.rs's harness does after a resume.

## 5. Evidence
- check.mjs 9/9 with the new tests; the bites failing for their own reasons.
- The kernel cross-check's numbers, against its pass rule.
- DelayStream's lateness at P2.
- The replay equality.
- Cargo.lock's diff: tokio-rustls only, plus npt-tcpinfo if (a).

## 6. For Mustafa
Nothing, unless the advisor escalates the unsafe exception (R20 (a)).
