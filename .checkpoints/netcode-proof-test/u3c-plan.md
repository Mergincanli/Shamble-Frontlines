# U3c plan, rev 2: DelayStream, fix 1's send gate, npt-tcpinfo, the mechanism tests and the kernel cross-check

Rev 1 was written 2026-11-07 about 19:40 UTC on head a8abd91 (code as at 7321cc0). **Rev 2 folds in the advisor's APPROVE
with conditions 1-16** (about 20:00 UTC; notes advisor-u3c-plan.md C1-C20 + verdict). The advisor ruled no re-review is
needed. Where the conditions and older text differ, **the conditions (numbered [Cn] below) win.**

Rulings:
- **R20 (a) is approved.** The unsafe exception is the advisor's to grant under ADR 0002:264-265 ("an exception needs a
  written invariant, a test and an `#[allow]` with its reason"). No escalation; npt-net itself stays free of unsafe.
- **The split is approved:** the gate and writer logic now, WsServer at U6, the functions shared and not copied.
- The DelayStream design, the limits, the 8,192 B request cap, tokio-rustls =0.26.6 and the kernel cross-check's role
  are approved with the conditions.

Sources:
- u3-build-plan.md's U3c line;
- advisor-u3q.md (M16, M17; S0-S14, especially S0, S9, S10, S12 and S13's cross-check);
- advisor-ws-docs-r2.md:383-386 and :410 (U3c (1)-(7), mechanism tests (a)-(e), plan text, R20);
- verify-ws-docs-r2.md:47 (W22, which mirrors fixes.rs:777);
- the catalogue, docs/design/netcode-proof-messages.md:61-78 (WebSocket frame and message limits 513 B; write buffer
  128 KB);
- plan §4.4 (size limits before any delay buffering; the Join deadline from TCP accept; send-buffer space to npt-room);
- plan §4.7 (`DelayStream` is npt-net's listed public item);
- Mustafa's U3-Q4 (b): above TLS, below tungstenite. His U3-Q5: tokio-rustls pinned in U3, audited at U5.

## 1. What U3c builds

### 1.1 tokio-rustls (M16) [C7]
- `tokio-rustls = { version = "=0.26.6", default-features = false, features = ["ring"] }` in the workspace; npt-net
  depends on it. The version was published 2026-09-28. Its manifest needs rustls ^0.23.27 with default features off and
  "std", so it resolves to the pinned 0.23.46 with no second rustls; `cargo tree -d` checks. rustls/ring is already on
  through quinn's default features, so the `ring` feature changes nothing in Cargo.lock. Cargo.lock gains the
  tokio-rustls entry only; npt-net's entry gains it too.
- **Why ring:** it is the one provider npt-net always has. M7 gives each WebTransport crate its own provider: wt-wtq
  uses aws-lc-rs (wt.rs:177-179) and wtransport uses ring (wt.rs:107-109).
- The provider is chosen explicitly through `ServerConfig::builder_with_provider(ring)`; there is no process default.
  TLS 1.3 only, ALPN "http/1.1". U5 audits it (§4.8).

### 1.2 DelayStream (S0-S12 over a real stream)
**Shape.**
- It wraps the server's TLS stream S: above TLS, below tokio-tungstenite.
- One `TcpModel` per connection: end 0 is RS, end 1 the client. `TcpConfig::default()`: Linux 6.8, the 64 KB budget.
- Profile: L2Config's. Its one-way delay is netted of `base_rtt_ns` through one helper shared with ImpairSocket
  (impair.rs:104's `rtt_ns.saturating_sub(base) / 2`, moved into a shared fn) [C5].
- Fates: one ChaCha8 stream per (L2Config.seed, connection index, direction) (S10). The accept path gives the
  connection index (first accepted = 0, recorded like ImpairSocket's slot ordinal).
- The real TCP and TLS handshakes run undelayed below the wrapper; recorded, as the docs PR's -29 note says.

**TLS records (S0) and the write side (S9)** [C1]:
- **Down direction.** Each poll_write takes **at most one record**: n = min(len, 16,384, free - 22), where free is
  model.free_budget(0) less the bytes released but not yet taken by S.
  - If free <= 22: Pending, woken when the budget frees. Never Ready(Ok(0)) for a non-empty buffer.
  - A **partial record never enters the model**. Each record counts as plaintext + 22 B, and its plaintext is
    released at the client end only once the whole record is delivered in order.
  - The pump writes each released down record into S with **one write call**, so real TLS records match the model's
    whenever rustls takes the record whole.
  - Test: free = 30,000 B with a 40,000 B write never exceeds the budget.
- poll_flush is Ready once the bytes are in the model, as a kernel socket's flush is.
- **Up direction** [C2]:
  - The client's plaintext, read from S, is cut into records at WebSocket frame boundaries, each + 22 B. The upgrade
    request is one record. Each record is written into the model at end 1 once its whole frame has been read; fixes.rs's
    INPUT = 60 + 6 + 22 uses the same rule.
  - It is released to tungstenite once delivered in order at end 0.
  - S is read only while free_budget(1), less the released-but-unread bytes, is **at least 549 B** (513 + 14 + 22). The
    up side therefore holds at most 64 KB in all, plus one frame being parsed and the request cap.

**The pump and its lifetime** [C3]:
- DelayStream::wrap spawns one tokio task per connection on the current runtime. The task owns S.
- It advances the model event by event (next_event, then advance(t)), so every release carries its model stamp.
  Released down records go into S at once; released up records go to tungstenite's reader.
- **Dropping DelayStream ends the pump and drops S at once**, for example when npt-server closes a slow-loris at its
  deadline.
- poll_shutdown waits at most **10 s** for delivery and for S to take the bytes, then drops S and counts it. An EOF
  from S ends the up direction after its last delivered byte.
- The clock's heap and waker table hold at most one live entry per live connection, removed when its pump ends.
  Test: open and close 1,000 connections; both tables end empty.

**The release clock (M14's pattern).** One process-wide dedicated thread:
- 1 ns timer slack, written and read back (l2::thread);
- a min-heap of (due, seq, connection) and a Condvar;
- at each model event's due time it wakes that connection's pump. No tokio timer is on the release path (1 ms
  resolution).
- Its origin is DelayStream's clock, `DelayStream::now_ns()`. **Every stamp DelayStream takes or reports is on this
  clock**: bursts, accepted_ns, the rows [C5].

**Bursts after wrap** [C4]:
- Per-session B1 on the fallback needs them: plan E23, and ADR 0001's check item 11 has Mustafa trigger a burst on
  WebSocket.
- **TcpModel** gets a pub(crate) method that adds a burst starting at or after the model's now; it refuses an earlier
  one. tcp_model.rs:2122 fixes bursts at creation today.
- **DelayStream** gets a pub method taking (start_ns, len_ns) on DelayStream's clock, logged for the replay test.
- Test: every transmission inside an added burst is lost, both ways.
- SetProfile on a DelayStream connection is carried to U6 [C14].

**Lateness (M1's pair)** [C10]:
- Down: measured at the return of S's poll_write for each released record. Up: at poll_read's return.
- Both are measured against the model's delivery stamp, with the same histograms and E[(L - L0 - 0.25 ms)+] as
  l2::stats.
- Reported per direction at P2: p50, p99, p99.9, max and E.
- **Trigger:** if p99.9 > 1 ms or max > 10 ms (the WebSocket bar's grid step), stop and bring it to the advisor before
  U3c closes.

**Limits before any delay buffering** (plan §4.4; catalogue 513 B) [C6]:
- The pump parses the upgrade request (cap 8,192 B; nginx's large_client_header_buffers default), then each frame
  header per RFC 6455 §5.2.
- These close the connection with a counted reason before any of the payload enters the model:
  - a frame, or a fragmented message's running total, above the limit;
  - an unmasked client frame (§5.1);
  - a control frame above 125 B, or a fragmented control frame (§5.5);
  - a 64-bit length with its top bit set;
  - a request over its cap.
- **One source:** the limit is L2Config's `ws_max_message_bytes` (513). tungstenite's WebSocketConfig takes the same
  field for max_frame_size and max_message_size. Also: max_write_buffer_size 128 KB (catalogue), write_buffer_size 0
  so each send writes through.
- A **seeded random-input test** of the request and frame parser: it never panics and never holds more than its
  bounds. No new dependency.
- Recorded:
  - the parser is a U5 fuzz target (-34);
  - the plain path (no wrapper) keeps tungstenite's own 64 KiB handshake cap (tungstenite machine.rs:169), while the
    wrapper's path caps the request at 8,192 B.

**Held bytes.**
- Down: at most 64 KB plus the acknowledged prefix of the front record. The budget covers released-but-untaken bytes,
  and a record's plaintext stays held until the record is delivered whole, so its acknowledged part no longer counts
  against the budget (NC-U3c-2, corrected 2026-11-08; tested at P2 with loss, G-NC2).
- Up: at most 64 KB in all, plus one frame in parsing (513 + 14 B) and the request cap [C2].
- A counter keeps the current and largest held bytes per direction. Nothing grows with client data beyond these.

**Join deadline (§4.4).** The accept path passes in the TCP accept stamp, `accepted_ns`, on DelayStream's clock,
never the end of a delay. npt-server reads it at U6.

**Per-return record** [C8]. At each burst's end, as the model passes it, the pump records:
- the stamp;
- outstanding(0) and srtt_ns(0);
- the bytes released but not yet taken by S;
- S12's per-burst times, per direction: from the link's return to the first in-order byte, and to the first whole
  message released.

This feeds mechanism test (c) and the SERIES burst_return fields DelayStream owns. Offset and tick phase are the
room's (U6).

**Surface: one list** [C5]:
- **pub:** `DelayStream::wrap(stream, cfg: &L2Config, conn: u32, accepted_ns: u64) -> io::Result<DelayStream<S>>`, the
  AsyncRead and AsyncWrite impls, `DelayStream::now_ns()`, the burst method, and the header, per-second and summary
  rows as std types (ImpairSocket's precedent, socket.rs:312-394).
- **pub(crate):** `srtt_ns()` (the model's end 0: what tcpi_rtt means, tcp.c:3842), `outstanding_bytes()`
  (write_seq - snd_una: what SIOCOUTQ means, tcp.c:630) and `free_bytes()` (the room in the 64 KB budget: the
  send-buffer space npt-room gets, §4.4). These are WsServer's inputs; npt-server reads them through Link.
- **L2Config** gains `ws_max_message_bytes` (513) and `ws_max_request_bytes` (8,192): fields, not new symbols. Its
  profile doc (config.rs:21-23, which says bursts are on ImpairSocket::now_ns) is updated: on the WebSocket path,
  bursts are on DelayStream::now_ns.
- wrap refuses a pass-through L2Config (R20: clean WebSocket runs go without the wrapper).

### 1.3 Fix 1's send gate and the per-tick writer (advisor-ws-docs-r2.md U3c (2), (3); W22) [C9]
- `pub(crate) fn snapshot_budget(srtt_ns: Option<u64>, tick_ns: u64) -> u64`:
  - N = (3 x srtt).div_ceil(2 x tick) + 1, at least 3, with srtt = 100 ms before the first sample;
  - fixes.rs's adaptive_limit becomes snapshot_budget(m.srtt_ns(0), TICK) x SNAP: one definition, test (e).
- `pub(crate) fn may_write(outstanding, owed, free, size, n) -> bool` holds only when all three of fixes.rs:777's
  conditions hold (W22):
  - owed == 0: nothing held above the kernel socket;
  - free >= size;
  - outstanding + size <= n x size.
  - **size is the whole tick's snapshot:** all its messages, written all or none, in the model's bytes (payload +
    WebSocket headers + 22 B per record), as fixes.rs's SNAP = 1,050.
- **The per-tick writer decision** lives in pub(crate) code that U6's WsServer calls, so test (b) tests the logic that
  ships: newest only, a skip counted, never written later. A cfg(test) driver runs it per tick over a real loopback
  WebSocket.

### 1.4 Mechanism tests (advisor-ws-docs-r2.md:385; in check.mjs's cargo test) [C9]
- **(a) The table at TICK = 33,333,333 ns:** no sample gives 6; 20 ms gives 3; 50 ms 4; 100 ms 6; 150 ms 8; 300 ms 15.
- **(b) Seeded sweeps** of the shipped writer logic (proptest is not in the workspace):
  - N never decreases as srtt grows and never falls below 3;
  - a write happens if and only if the three conditions hold;
  - after any write, outstanding <= N x size;
  - writes happen only on a tick;
  - a skipped snapshot is never written later;
  - every written snapshot is the newest at its tick.
- **(c) On DelayStream at P2 under B1**, through TLS and tokio-tungstenite on loopback:
  - at every return, outstanding <= N x size, with N the value in force at the writer's last write before the return.
    The gate guarantees only that N, and srtt can move after it (NC-U3c-1, corrected 2026-11-08). The per-return record
    gives outstanding; the writer's log gives N at its last write;
  - owed = 0 at each write.
  - The build states the counts (bursts, phases, connections), each real-time test's duration, and each band's level
    (99.9 % suggested).
  - **Judge on model state and identities, never on wall-clock thresholds.**
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
  - the one-record rule and its 30,000 / 40,000 test [C1];
  - the up bound [C2];
  - the request is one record;
  - an oversize frame, message, control frame or request, or an unmasked frame, each closes before buffering, counted.
    Bite: with the check off, the payload enters the model.
  - the held-bytes bounds hold under a reader that never reads;
  - wrap refuses a pass-through config;
  - an added burst loses every transmission inside it, both ways [C4];
  - drop and shutdown bounds, and 1,000 opens and closes leave the clock's tables empty [C3];
  - the parser's seeded random-input test [C6].
- **Replay:** the run's model counters and delivery stamps equal those of a pure TcpModel driven by the logged (stamp,
  record) writes and the logged added bursts. The real path moves no fate (S10).
- **Loopback TLS** (tokio-rustls, ring, TLS 1.3) + DelayStream + tokio-tungstenite, 30 Hz both ways at P2:
  - the 101 is delayed by the model;
  - up: each message's hand-over equals its model delivery stamp plus its lateness (an identity). Down: a bound, since
    after S takes the bytes they pass the kernel's real TCP and the client's TLS, which no in-process stamp sees
    (NC-U3c-3, corrected 2026-11-08);
  - the stall count is inside its band at P2 (S13 (f) on the real path; the level stated);
  - lateness is reported with condition 10's trigger.
- **TLS config:**
  - TLS 1.3 only; ALPN http/1.1; the ring provider, with no process default.
  - Both WebTransport features and the WebSocket server in one process (M7's one-process check, extended).

### 1.6 The kernel cross-check (S13's) [C11]
An evidence run here, outside check.mjs: it needs root and iptables. Its script and output are committed in the spike
tree (spikes/netcode-proof/results/u3c-kernel-check/), not only in .checkpoints/.

**Fixed in writing before the run** (in the script's header):
- **Pattern** (the scout's u3-tcp-stall-probe.py):
  - lo, a 2 % random drop on client-to-server packets of the probe's port only (iptables statistic);
  - 30 Hz, 200 B, TCP_NODELAY;
  - 3,000 messages x 3 runs.
- **Stall classification:** a stall is a message whose delay exceeds the base by more than one send gap. Its repair path
  is RACK if repaired within RTT + gap + 2 ms + the jitter allowance, else probe or timeout by the model's own counters
  on the same pattern. The build's script states the exact rule.
- **Band:** 99 % two-sided.
- **Congestion control:** TCP_CONGESTION is set to cubic on both probe sockets, since this container defaults to bbr
  (u3-scout-timing.md:388). If cubic is not available, bbr is recorded.
- **Recorded:** the kernel (6.18.44), HZ 250, tcp_recovery, tcp_early_retrans, tcp_sack, tcp_frto, tcp_rto_min_us and
  tcp_thin_linear_timeouts.

**The model** runs at the loopback's measured RTT, with loss on the data direction only (a cfg(test) knob in
tcp_model).

**Pass:**
- the RACK-path stall p50 within 2 ms of the kernel's;
- the timeout-path share within its band.

A miss goes to the advisor before U3c closes.

**Safety:**
- the iptables rule matches only the probe's port and is removed in a trap;
- `iptables -S` reads the same before and after;
- nothing else uses loopback during the run.
- **The main session runs it after the merge, one evidence run at a time**, never a build agent in parallel with other
  loopback work.

R1's S14 check on RS (veth + netem) stays the binding one: this container runs 6.18.44, not 6.8, and has no tc.

### 1.7 SERIES.md rows (M15)
- **Header:** the model's name and version ("linux-6.8-tcp v3"), TcpConfig, the S11 list, the clock's slack and policy,
  DelayStream's limits, the connection index rule, and whether the wrapper is on.
- **Per second, per direction:** segments, transmissions, lost by cause, repairs by path, max backoff, stalls (count,
  max, sum), held bytes max, old-duplicate discards, D-SACKs, undos, and the release lateness summary.
- **Per connection at its end:** a ws_summary.
- **Per return:** DelayStream's share of burst_return [C8].
- The -18 judged series (burst_heal, event.burst_return's room fields) stays with the spike-branch follow-ons after #45
  merges (advisor ORDER (3)).

### 1.8 npt-tcpinfo (R20 (a), approved) [C12]
**What it is.** A new workspace crate, used only by npt-net's WsServer on the plain path (P0, ML: no wrapper). Two
functions on a `BorrowedFd`:
- `srtt_us(fd) -> io::Result<Option<u32>>`: getsockopt(IPPROTO_TCP, TCP_INFO) into a zeroed `libc::tcp_info`.
  - If the returned length doesn't cover tcpi_rtt: Unsupported.
  - The field is read without more unsafe (offset_of!).
  - tcpi_rtt 0 maps to None (the 100 ms default).
- `unacked_and_unsent(fd) -> io::Result<u32>`: ioctl(SIOCOUTQ = TIOCOUTQ, 0x5411).

**(a) Lints.**
- ~~Keep `[lints] workspace = true`.~~ Superseded by the advisor's ruling of 2026-11-08 (C2b, b5e4043): the workspace
  forbids unsafe_code, so npt-tcpinfo carries its own `[lints]` table equal to the workspace's but for
  `unsafe_code = "deny"`, and check.mjs's exceptionLintProblems compares the two.
- `#[allow(unsafe_code, reason = "…")]` on the two functions only, not crate-wide.
- Each unsafe block has a SAFETY comment stating its invariant.
- Deny clippy's undocumented_unsafe_blocks and multiple_unsafe_ops_per_block.

**(b)** The getsockopt length check and the offset_of! read, as above.

**(c) Platforms.** Linux only, with a non-Linux stub returning Unsupported, so the workspace still builds anywhere.

**(d) Pins and checks.**
- libc is pinned at =0.2.191 in the workspace.
- check.mjs's deps step checks:
  - libc is npt-tcpinfo's only normal dependency;
  - only npt-net depends on npt-tcpinfo;
  - `allow(unsafe_code` appears only in npt-client-wasm and npt-tcpinfo (a grep).

**(e) Tests.**
- The non-reading peer (a small SO_RCVBUF): SIOCOUTQ grows and equals /proc/net/tcp's tx_queue (write_seq - snd_una
  too).
- EINVAL on a listening socket.
- The request constant is SIOCOUTQ (0x5411).
- tcpi_rtt > 0 after traffic.
- **The root run, which tells SIOCOUTQ from SIOCOUTQNSD** (the non-reading peer alone cannot: with a closed window
  nothing is in flight). The ACK direction is dropped for about 1 s while writing (iptables on the probe port, as in
  1.6). SIOCOUTQ must grow while tcpi_notsent_bytes stays near 0. The script is committed with 1.6's, and the main
  session runs it after the merge.

**(f)** R1's run plan (S14's veth + netem setup) reads srtt against netem's 100 ms. This container has no tc.

**(g)** Plan text first (1.9): §4.1, §4.2, §4.7, §4.8, and the Cargo.toml lint comment ("nothing else may" becomes
the two named exceptions with ADR 0002:264-265's rule).

### 1.9 Plan text (spike branch; committed before any code merges; the main session writes it) [C12g, C13]
- **§4.1:** npt-tcpinfo in the crate list.
- **§4.2:** npt-tcpinfo depends on libc only, and only npt-net uses it. The unsafe rule names its two exceptions.
- **§4.3, E15:** the WebSocket sentence becomes the Linux-faithful model (S0-S14, v3) plus fix 1, stated with fixes.rs:777's
  three conditions (W22).
- **§4.4:** WsServer applies the budget; npt-room sees BufferFull and the skipped count.
- **§4.7:** npt-tcpinfo's two functions; DelayStream's pub list as 1.2.
- **§4.8:** the audits gain tokio-rustls, npt-tcpinfo's unsafe and the parser's fuzz target.
- **§12.1:**
  - the U3 row: E15's constant becomes U3a's model, plus U3c's DelayStream with fix 1;
  - the U6 row: condition 14's carries.
- **§15:** R-25's row (the faithful model) and a new "WS-BAR's rule" row: the practice run, the 10 % tolerance, how
  the slowdown is measured and the numbers move, the investigation.
- **§0.5 (new):** Mustafa's 2026-11-07 words verbatim (13:45 and 14:30) and where each is met.
- **§16:** the docs PR's go, **pending until Mustafa gives it**.

### 1.10 Carried to U6 (in the checkpoint's carried list and §12.1's U6 row) [C14]
- **The gate:** WsServer calls snapshot_budget and may_write (no copy).
  - Its inputs: DelayStream's on L2; npt-tcpinfo's on the plain path.
  - owed = everything above the kernel socket: tungstenite's buffer plus rustls's unsent TLS.
  - The plain path's free value is stated with its reason.
- **Re-runs on WsServer:** (b)'s writer properties, (c), and (d)'s queueing-writer bite.
- **A plain-path check:** a client with a small receive buffer stops reading; after any write SIOCOUTQ <= N x size,
  and writes stop.
- **The run-time bite ws-budget-x2** in a npt-net `bites` feature (R11; npt-net has none today).
- **One tick's snapshot:** how WsServer sees a tick's whole snapshot, since Link::send_unreliable is per message
  (link/src/lib.rs:66; W18).
- **Other carries:**
  - SetProfile on a DelayStream connection;
  - where the per-tick syscalls run;
  - the header records whether the wrapper is on.

### 1.11 fixes.rs leftovers (r2) [C15]
The stale three_part() bar (fixes.rs:1530-1537) and the stats B5 bite-row pin go into this unit's fixes.rs edit, beside
adaptive_limit's change. They must not be dropped.

## 2. Order
1. **Plan text (1.9)**: the main session, in parallel with the builds, committed before any code merges.
2. **Build A** (one agent, isolated worktree): 1.1-1.5, 1.7, 1.11, and 1.6's script (not its run).
3. **Build B** (one agent, isolated worktree, in parallel): 1.8 with its check.mjs deps step, and 12e's root-run script
   (not its run).
4. **Each build ends with:**
   - check.mjs 9/9 in its worktree;
   - clippy -D warnings with and without features;
   - fmt;
   - Cargo.lock's diff stated;
   - notes inside the worktree (`.checkpoints/netcode-proof-test/u3c-build-a.md` or `-b.md`; the main session copies
     them).
5. **Merge, one at a time:** B, then A (or as they finish). check.mjs 9/9 and the gate after each, then a commit and a
   checkpoint.
6. **Evidence runs by the main session**, one at a time with nothing else on loopback:
   - the kernel cross-check (1.6);
   - 12e's root run;
   - DelayStream's lateness at P2 with condition 10's trigger.
   Their outputs are committed under spikes/netcode-proof/results/.
7. **One skeptic round on U3c's final diff:** rust-reviewer, netcode-architect and security-auditor (network input, a
   new dependency, the unsafe crate), plus test-engineer on whether it is proven. Then fixes, then the checkpoint.

## 3. Build route
- Two build agents in isolated worktrees (general-purpose, opus, xhigh), run as one workflow. The main session copies
  their notes, merges, runs check.mjs and the evidence runs.
- fixes.rs is edited only by build A during U3c.
- wt_tests.rs: build A may move the TLS test helpers into a shared cfg(test) module. The flaky-test fix waits until
  after A merges.

## 4. Risks
- **Release precision** through a tokio task wake (probably 10-50 us at p50): measured, with condition 10's trigger.
- **Record boundaries are approximations:** down at poll_write records, up at frame boundaries. Real TLS records differ
  invisibly on loopback; recorded in the header.
- **A stuck S** holds released bytes against the 64 KB budget, so nothing grows; its maximum is recorded.
- **A large write:** tungstenite's 128 KB write buffer holds a 65,541 B FullState while DelayStream takes it a record at a
  time. owed > 0 then makes the gate skip snapshots, as fixes.rs's harness does after a resume.
- **iptables on a shared container** [C16]: the rule matches only the probe's port, is removed in a trap, `iptables -S`
  is compared before and after, and the main session runs one evidence run at a time with nothing else on loopback.
- **Real-time tests cost time and can flake** [C16]: they are judged on model state and identities, never wall-clock
  thresholds, with stated counts, durations and band levels.
- **Different CPU** [C16]: WebSocket clean runs (no wrapper) and impaired runs pay different CPU. Recorded; the header
  says whether the wrapper is on.

## 5. Evidence
- check.mjs 9/9 with the new tests, and the bites failing for their own reasons.
- The kernel cross-check's numbers against its pre-written rule, and 12e's root run.
- DelayStream's lateness at P2 against condition 10's trigger.
- The replay equality.
- Cargo.lock's diff: tokio-rustls and npt-tcpinfo (with libc as its direct dependency) only.

## 6. For Mustafa
Nothing. The advisor's FYI, not a decision: the spike gains a second unsafe exception, a crate of about 40 lines with
two Linux calls, under ADR 0002's own exception clause.
