# U3c skeptic round: rust-reviewer notes

Scope: git diff 2c68bf9..989118e -- spikes/ ; plan.md rev 3.3 U3c text; u3c-plan.md rev 2 [C1]-[C16].
Focus: npt-tcpinfo unsafe blocks + SAFETY; DelayStream state machine (wakers, Pending, poll_shutdown bound,
pump lifetime, locks across awaits, clock thread heap + waker table); panics on peer input; overflow;
allocation per datagram/record; cfg(test) knobs never in shipped code.
Build dir: CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/u3c-review-target

## Plan
1. Read u3c-plan.md conditions, advisor notes (skim).
2. tcpinfo crate: linux.rs, lib.rs, stub.rs, tests.rs, Cargo.toml.
3. delay.rs, delay_clock.rs, delay_names.rs, ws_frame.rs, tls_test.rs.
4. ws_gate.rs, ws_config.rs, tcp_kernel.rs, tcp_model.rs + fixes.rs.
5. Cargo.toml features / cfg(test) reach; build + clippy + tests.
6. wt_tests diff.

## Findings (a line per finding as found)
- tcpinfo/src/linux.rs read: both unsafe blocks one op each, SAFETY comments present; libc gnu tcp_info tcpi_rtt @68 matches uapi (libc gnu/mod.rs:248-276); TIOCOUTQ generic 0x5411 (arch/generic/mod.rs:35). No finding yet.
- tcpinfo unsafe #2 (ioctl) guarded by SO_DOMAIN/SO_TYPE/SO_PROTOCOL reads with len==4 check (linux.rs:153-197); one op per block; sound. getsockopt TCP_INFO into [u8; sizeof tcp_info], rtt read via get() after len check (linux.rs:217-231): sound, no panic.
- ws_frame Scanner: header buf index bounded (need<=14), record len >=1 per boundary (no zero-length up record -> no false EOF), request cap closes at len>=max_head; no alloc. OK.
- F1 (low/medium) add_burst overflow: delay.rs:708 burst_ends.push(s.saturating_add(len_ns)) then due() delay.rs:434 `x + self.origin` overflows when start_ns+len_ns > u64::MAX (model.add_burst accepts any len>0, tcp_model.rs add_burst). npt-net release has no overflow-checks (Cargo.toml only sim/proto) -> wraps to a past due -> clock fires at once, pump re-arms same past due -> spin. Debug: panic in pump/add_burst under lock.
- F2 (low) current() wakes pump + reader/writer while holding the Conn lock (delay.rs:750-755), contrary to wakers() doc 'call after the lock is released' (delay.rs:507); woken task on another worker blocks on the mutex.
- F3 (low) Pump::poll holds the std Mutex<Conn> across S's poll_write/poll_flush/poll_read/poll_shutdown (delay.rs:1200-1335): rustls encrypt + syscalls under the lock; tungstenite's task blocks a runtime worker meanwhile and adds to measured lateness. Not an await, no deadlock (lock order conn->clock only; clock thread wakes outside its lock).
- F4 (nit) Pump re-copies the front record into scratch on every attempt (delay.rs:1228-1233) incl. after Pending.
- F5 (nit) poll_read with buf.remaining()==0 and released data returns Pending (delay.rs:1033, 1078-1082); tungstenite never does it.
- F6 (low) header_numbers -> clock.slack_now() blocks the caller up to 1 s on a Condvar (delay_clock.rs:154-175, delay.rs:890); pub fn, U6 calls it from async code.
