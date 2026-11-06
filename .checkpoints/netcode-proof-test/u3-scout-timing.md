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

### F1a. tokio 1.54.1's timer: 1 ms ticks, deadlines rounded UP, then epoll's ms timeout rounded up again
- Tick = 1 ms from the runtime's start: `/root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.54.1/src/runtime/time/source.rs:22-33`
  (`instant_to_tick`: `dur.as_millis()`; `tick_to_duration`: `Duration::from_millis(t)`).
- A deadline is rounded up to the end of its millisecond: `source.rs:17-19` ("// Round up to the end of a ms",
  `self.instant_to_tick(t + Duration::from_nanos(999_999))`); applied to every Sleep at
  `/root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.54.1/src/time/sleep.rs:341-342` and `:408-409`.
- The driver parks for whole ms: `/root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.54.1/src/runtime/time/mod.rs:226-233` ("we effectively round up to
  1ms here"), `tick_to_duration(when.saturating_sub(now))`.
- With the I/O driver on (any runtime that does net I/O), the park is mio's epoll_wait, whose timeout is in ms and
  rounded up again: `/root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/mio-1.2.4/src/sys/unix/selector/epoll.rs:46-67` ("`Duration::as_millis` truncates, so round
  up", `to.checked_add(Duration::from_nanos(999_999))...as_millis()`, then `epoll_wait(..., timeout)`). mio 1.2.4 is
  the lockfile's pin (spikes/netcode-proof/Cargo.lock).
- tokio's own docs say so: `/root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.54.1/src/time/sleep.rs:14-15` ("`Sleep` operates at millisecond
  granularity and should not be used for tasks that require high-resolution timers").
- The alternative timer (`enable_alt_timer`) is behind `tokio_unstable` (`/root/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.54.1/src/runtime/builder.rs:444-450`):
  not usable under the spike's stable flags, and it does not change the ms tick (unconfirmed: not read in full).
- Consequence: expected lateness of sleep_until is 0 to about 2 ms (1 ms rounding of the deadline, up to 1 ms more
  from epoll's rounding of the remaining time), so it CANNOT meet 0.25 ms by construction. Measured below.
