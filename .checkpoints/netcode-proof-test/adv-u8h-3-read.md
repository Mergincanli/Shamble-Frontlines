# ADV-U8H-3's read: the delay clock thread's vCPU (the main session, 2026-11-27 21:5x UTC; read only)
Question (advisor-u8-head.md:175): DelayStream's pump is a tokio task, but its release clock is "one process-wide
dedicated thread" (crates/net/src/l2/delay_clock.rs:1); that thread's vCPU decides whether ds-selftest may join the run
list when bring-up reads `alone` 0 (R1.md:243-245; r1.sh:38 TIMING_BLOCKS).

## What the code says
- The thread is spawned once per process by the first `Clock::start` (delay_clock.rs:92-99, `std::thread::Builder`
  named "npt-delay-clock"), so it inherits the spawning thread's CPU mask (Linux threads inherit the affinity mask).
- Two callers reach the first `start`:
  - the run header: main.rs:737 `DelayStream::header_numbers_from(&c.l2)` -> delay.rs:1333-1336 (`clock.start`, then
    `release`), on the room's main thread;
  - a WebSocket accept: ws_server.rs:476 `DelayStream::wrap(...)` -> delay.rs:1181 (`clock.start`), on the room's
    npt-io runtime ("the transports' listeners and sessions run on this runtime", tick_loop.rs:1326-1330).
- Masks: the room starts under `/usr/bin/taskset -c "$hk"` (npt-ctl; wrappers.test.sh:127), hk being the housekeeping
  set without the tick vCPU (npt-ctl:71 `npt_housekeeping "$online" "$tick" "$sib"`); main.rs:841 pins the main thread
  to `io_vcpus` (`TickLoop::pin_to_io`); start_with pins the starting thread to `io_vcpus` before the runtime, the
  writer and the tick thread are made (tick_loop.rs:1317-1323); main.rs:244 refuses `io_vcpus` holding `tick_vcpu`,
  and main.rs:655 refuses an SMT sibling of it. Only the tick thread (and the shared-vCPU bite's second thread) is
  pinned to the tick vCPU afterwards (tick_loop.rs:1362, :1390, :1418).
- So the clock thread runs on the I/O vCPUs (or the housekeeping set), never on the tick vCPU.

## What it does not settle
- Whether ds-selftest's judged numbers depend on anything on the tick vCPU (the tick thread's send stamps feed the
  streams it releases): a netcode reading, not this read's.
- Moving ds-selftest out of TIMING_BLOCKS is a U8 line (r1.sh:38, R1.md:243-245 and any test that pins the list); U8
  stands at 6,282 + the two notes = 6,284, his stop.
- Until a ruling, the skip stands (advisor-u8-head.md:175: "it is the conservative side").
