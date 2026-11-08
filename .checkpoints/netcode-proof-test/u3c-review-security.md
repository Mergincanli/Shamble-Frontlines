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
