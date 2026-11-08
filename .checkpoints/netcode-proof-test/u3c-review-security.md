# U3c skeptic round: security-auditor notes

Scope: `git diff 2c68bf9..989118e -- spikes/` (U3c), plan.md rev 3.3 U3c text, u3c-plan.md rev 2 [C1]-[C16].
Focus: network input + supply chain (frame parser, request cap, buffers keyed by peer data, TLS config,
new deps, npt-tcpinfo unsafe exception and check.mjs, root scripts' iptables handling).
Rules: no network changes on this machine; builds only in .gate/u3c-review-target; no source edits.

## Plan
1. Read u3c-plan.md, advisor-u3c-plan.md, plan.md U3c text, build/critic notes (skim).
2. Diff stat; read ws_gate / ws_config / DelayStream / fixes.rs / npt-tcpinfo / check.mjs / root scripts.
3. Check deps (Cargo.toml, Cargo.lock), cargo deny/audit config if present.
4. Run the tests in the review target dir.
5. Findings, one notes line each; DONE.

## Progress
- (start 2026-11-08) Read u3c-plan rev 2, advisor-u3c-plan (incl. side rulings), rust and netcode reviewers' notes (no
  duplication: their F1 add_burst overflow, F7 565 KB/conn, F11/F-inputs, F-held are theirs; I cite, not repeat).
- ws_frame.rs read in full: header buf index <= 13, 64-bit top-bit check :226, unmasked :234, control >125/!FIN :237,
  frame :240, message running total saturating :248, request cap :159 (after the CRLFCRLF test, so exactly-cap passes).
  No alloc. OK against C6's list.
- ws_config.rs: TLS13 only :32, ALPN http/1.1 :37, builder_with_provider(ring) :31, no install_default anywhere (grep);
  tungstenite max_frame/max_message from one arg :54-55, max_write_buffer 128 KB, write_buffer 0, accept_unmasked false
  (default; one_source asserts it). read_buffer_size NOT set: tungstenite default 128 KiB BytesMut per connection
  (tungstenite protocol/mod.rs:98, frame/mod.rs:126).
- tls_test.rs is #[cfg(test)] (l2.rs: `#[cfg(test)] pub(crate) mod tls_test`): fixed-seed Ed25519 key never reachable
  from a non-test build. OK.
- tcpinfo linux.rs: sound (rust-reviewer verified cites); SO_DOMAIN/TYPE/PROTOCOL gate before ioctl. OK.
- FINDING S1 (low): check.mjs's unsafe_code scan (check.mjs RELAX_UNSAFE regex, unsafeProblems) misses relaxings rustc
  accepts. Probed: node import of unsafeProblems on crafted crates/room/src/evil.rs: `#[allow(dead_code, /* x */
  unsafe_code)]` MISSED, macro-built `#[allow($l)]` + `lax!(unsafe_code)` MISSED; .cargo/config.toml (rustflags) not
  read at all. Scratch crate (scratchpad/lintprobe, target .gate/u3c-review-target/lintprobe, rustc 1.99.1): each of
  comment-in-list, macro-built, RUSTFLAGS=-Aunsafe_code, --cap-lints=warn COMPILES an unsafe block under [lints]
  unsafe_code="deny"; `cargo rustc -- -F unsafe_code` rejects all three tested. Also not caught: an allow of
  clippy::undocumented_unsafe_blocks / multiple_unsafe_ops_per_block inside npt-tcpinfo (plan 1.8 (a) denies them in
  lib.rs only). Fix: compiler-based check per non-exception crate (cargo clippy/rustc with -F unsafe_code), plus a
  masked-token scan for `unsafe` outside the two crates, plus refuse rustflags in .cargo/config.toml/env; extend the
  tcpinfo scan to the two clippy lints. The Cargo.toml comment's "fails on any other crate that relaxes it" overclaims.
- FINDING S2 (medium, REPRODUCED): request-head parser differential. ws_frame.rs:152 ends the head only at "\r\n\r\n";
  httparse 1.10.1 (tungstenite's parser) also ends it at a bare "\n\n" / "\r\n\n" (httparse lib.rs:1105-1116).
  tungstenite reads the handshake in 4,096 B chunks (tungstenite lib.rs:36, buffer.rs:57) and DelayStream hands
  min(record rest, buf.remaining()) (delay.rs:1036), so a head of exactly 4,096 B ending "\n\n" completes tungstenite's
  handshake with an empty tail (server.rs:256-258's JunkAfterRequest does not fire); the rest of the scanner's "head"
  record (up to the 8,192 cap) is read by tungstenite as frames, and the scanner's frame view stays misaligned after.
  Scratch run (scratchpad/headdiff, public DelayStream API, target .gate/u3c-review-target/headdiff): 604 B masked frame
  after the head. CONTROL CRLF: DelayStream closes Frame=1, up written 4,096 B (head only). BARE LF: handshake OK, 101
  sent, DelayStream closes all 0, up written 4,708 B (the over-limit frame went through the model), tungstenite then
  MessageTooLong{604,513}. Impact bounded (64 KB budget; tungstenite's own 513 B limits are the backstop), but C6's
  "closes before any payload enters the model, counted" is false for a hostile client and R4's close counts would
  misattribute. Fix: end the head at LF (CR)? LF as httparse does (or httparse's own Complete(size)); add bare-LF,
  mixed and leading-blank-line cases to the C6 test; make the U5 fuzz target a differential against tungstenite's
  handshake (head length equal).
- Deps: lock diff adds only tokio-rustls 0.26.6 (deps rustls, tokio) and npt-tcpinfo (libc; socket2 dev). tokio-rustls
  manifest (registry): default = logging, tls12, aws_lc_rs; ring = rustls/ring; early-data off (no 0-RTT); rustls
  ^0.23.27 default-features off + std. cargo-deny / cargo-audit NOT installed here (`no such command`), so not run;
  deny.toml (advisories all errors, crates.io only) stays U5's. Manual RustSec read (rustsec.org, 2026-11-08):
  tokio-rustls's only advisory RUSTSEC-2020-0019 (patched >=0.13.1); rustls's Sep-Oct 2026 advisories
  RUSTSEC-2026-0285 (patched >=0.23.45), -0346, -0347, -0348 (brotli only), -0349 all patched >=0.23.46: the pinned
  0.23.46 is the first fixed release. Note for U5: 0.23.46 is the floor, never downgrade. libc: rustsec package page
  404 (no advisories listed). OK.
- tcpinfo dev-dep socket2 pinned in the crate manifest (=0.6.5, features all), not via the workspace: nit, consistent
  version with the lock.
- Ran (CARGO_TARGET_DIR=.gate/u3c-review-target): npt-net --lib filtered to ws_frame, ws_config, ws_gate, each_limit,
  a_full_budget, the_request_is_one_record, the_websocket_tls, wrap_refuses, dropping_it, a_shutdown_s_never,
  a_thousand_opens: 23 passed. npt-tcpinfo: 8 passed. `node check.mjs --only deps,self`: PASS (78 crate files), 21/21.
- FINDING S3 (low): per-connection, client-triggered memory before U6's caps. ws_config.rs:52-58 leaves tungstenite's
  read_buffer_size at 128 KiB (tungstenite protocol/mod.rs:50-57, :98; FrameCodec::new frame/mod.rs:126 allocates it
  at once and in_buf_max_read lets one read pull 128 KiB of client bytes); tungstenite's own doc recommends ~4 KiB for
  many connections. With DelayStream's ~565 KB per wrap (rust-reviewer F7) that is ~700 KB per TLS-completed
  connection, plus an unmeasured model cost per 6 B empty frame (28 B model record each). All bounded per connection;
  the count is the client's. Fix: read_buffer_size ~4 KiB in ws_config + one_source; U6: accept caps (per address,
  total) before TLS, the message-rate limit closes the connection (drops DelayStream), E14 run with the wrapper on.
- FINDING S4 (nit): root scripts sound (port-scoped rules, trap/finally removal, iptables -S before==after, refusal if
  a rule exists; committed evidence before==after, 3 lines). Nits: (a) siocoutq-root-check.py:283-294 finally can be
  cut by a second SIGINT/TERM/HUP (on_signal raises inside remove_rule) leaving the 100 % DROP rule in, -S compare
  skipped: ignore the signals at the start of finally; (b) no `iptables -w` anywhere (xtables lock contention makes
  -I/-D fail; -C failing under the lock ends remove_rule's loop early; -S diff is the backstop, exit 3) and a
  CalledProcessError from -I/-D is not in the except tuple (exit 1 = "fail" by the script's own codes);
  (c) run.sh:77 reads the generic env var PORT (often set for web servers); the bind check on 127.0.0.1 mitigates.
- FINDING S5 (nit, U6 carries + check gaps): (a) "one source" is a convention across two call sites (delay.rs:581/633
  read cfg.ws_max_message_bytes; ws_config.rs:52 takes a bare usize; only the test pair() at delay_tests.rs:341 ties
  them): let U6's WsServer build both from one &L2Config (ws_config(&L2Config)). (b) The TLS accept runs before and
  below DelayStream (delay.rs:10-11 HANDSHAKE note) and tokio-rustls's accept has no timeout: U6's 2 s Join deadline
  from accepted_ns must wrap the TLS accept too. (c) check.mjs PLACED_ALLOW accepts `reason = ""` (node probe: empty
  reason on srtt_us -> 0 problems), against ADR 0002:264-265's "an #[allow] with its reason"; crateFiles reads only
  crates/ (a member elsewhere would go unscanned). (d) snapshot_budget (ws_gate.rs) has no upper bound and srtt is
  client-influenced on the plain path (delayed ACKs); memory stays bounded by the socket's free room, so U6's
  plain-path `free` must be the socket's real bound (stated, e.g. SO_SNDBUF), not N.
- FINDING S6 (nit): plan.md:627 (§4.8, U5's audit list) says "npt-tcpinfo's two unsafe blocks"; linux.rs has three
  (`unsafe {` at :51 getsockopt TCP_INFO, :95 getsockopt SOL_SOCKET in int_option, :138 ioctl SIOCOUTQ). U5's audit
  list should name all three (the SOL_SOCKET gate is what makes the ioctl sound). Also plan.md:432 "two functions
  (getsockopt TCP_INFO and ioctl SIOCOUTQ)": the second function also makes the SO_* getsockopts.
- Plan.md:432-433 "greps that allow(unsafe_code appears nowhere else": see S1, the grep is bypassable.
- Not re-ruled (per task): C10 lateness trigger. Not re-reported: rust F1 (add_burst overflow spin, medium; agree it
  is reachable only by the host API, U6 must not let a client choose len_ns), F7, F11; netcode F-held, F-inputs.
- FINDING S7 (low, carry; security angle on rust F1): DelayStream::add_burst (delay.rs:691-735) will be driven by the
  per-session B1 control (plan E23; ADR 0001 check item 11), which a client triggers (plan.md:500 "the capability
  check of per-session controls"). F1's overflow (delay.rs:708 saturating push, :434 `x + self.origin`) spins the
  process-wide release clock thread (delay_clock.rs, one thread for every connection), so on release builds it would
  be a cross-session DoS, not a self-DoS. Fix: F1's checked_add in add_burst/due(), and at U6 range-check the
  control's start and length (e.g. len <= a stated maximum, start within the session) before calling add_burst.

## Coverage (transport layer: client action -> where the server checks it -> gap)
| Client action | Checked at | Gap |
|---|---|---|
| Oversize upgrade request | ws_frame.rs:159 (8,192 B, DelayStream path); tungstenite 64 KiB AttackCheck (plain path) | S2: bare-LF head ends earlier in tungstenite |
| Frame / message over 513 B | ws_frame.rs:240/:252 before the model; tungstenite max_frame/max_message (ws_config.rs:54-55) | S2 (misaligned scanner after a bare-LF head); tungstenite still closes |
| Unmasked frame | ws_frame.rs:234; tungstenite accept_unmasked_frames=false | none |
| Bad control frame (>125 B, fragmented) | ws_frame.rs:237; tungstenite | none |
| 64-bit length, top bit set | ws_frame.rs:226; tungstenite size check | none |
| Fragmented message running total | ws_frame.rs:247-253 (saturating) | none |
| Never reads (down) | 64 KB model budget, Pending, tungstenite 128 KB write bound, rustls buffer limit | U6 deadline/close |
| Never sends / slow request | nothing in DelayStream (by design) | U6: Join deadline must cover the TLS accept (S5 b) |
| Flood of tiny frames | up room >= 549 B read floor; 64 KB budget | CPU per 6 B frame unmeasured; U6 rate limit must close (S3) |
| Many connections | none in U3c | U6 caps; ~700 KB per connection (S3) |
| TLS downgrade / ALPN | TLS 1.3 only, ALPN http/1.1, ring (tests: TLS 1.2 refused with alert 70, h3 refused) | none |
| Per-session B1 control | add_burst refuses past/zero/65th burst | S7 + rust F1 overflow; U6 range check |

## Verdict
APPROVE_WITH_FINDINGS. No high. S2 (medium, reproduced) breaks C6 for a hostile client: fix in U3c's fix round, not
carried. S1 (low) check.mjs's unsafe scan is bypassable (rustc-proven). S3, S7 (low) carries with numbers. S4-S6 nits.
Scratch left for the main session's cleanup (not deleted; Mustafa's rule): .gate/u3c-review-target/headdiff (445 MB),
.gate/u3c-review-target/lintprobe (248 KB); sources in the session scratchpad (headdiff/, lintprobe/).
DONE
