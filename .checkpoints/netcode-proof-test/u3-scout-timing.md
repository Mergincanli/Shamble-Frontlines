# U3 scout: timers, TCP retransmission, the WebSocket DelayStream (rust-reviewer)

Branch claude/netcode-proof-test-plan-hcky5g, head ad527f3. Research only: no source, manifest or doc edits.
Started 2026-11-06 (fresh; no earlier notes file existed when resumed after the usage limit).

## Plan
1. Q1a: tokio 1.54.1 time driver resolution and rounding (file:line).
2. Q1b: probe crate (src in .checkpoints/netcode-proof-test/u3-timer-probe.rs, Cargo.toml + build under
   .gate/netcode-proof-test/timer-probe/, offline, workspace's pinned tokio). Measure lateness for 0.1/0.5/1/2/10/50 ms,
   >=10,000 waits each: (a) tokio sleep_until multi-thread, (b) std::thread::sleep dedicated thread, (c) sleep to
   due-100us then spin. p50/p99/p99.9/max. Also timerslack_ns and clocksource.
3. Q2: Linux TCP RTO_MIN, sysctl/route rto_min, RACK-TLP, TLP timeout, one lost segment mid-stream at 30 Hz, P2.
4. Q3: tokio-tungstenite 0.30.0 accept_* bounds; WebSocketConfig limits; per-connection delayed-bytes cap.
5. Q4: DelayStream sketch.
6. Final summary, DONE.

## Findings
