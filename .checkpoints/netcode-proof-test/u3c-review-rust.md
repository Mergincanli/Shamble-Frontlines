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
- (resumed 2026-11-08 after the disk-full stop; F1-F6 re-checked against the code at 989118e: still hold)
- F1 confirmed: tcp_model.rs:2190-2201 add_burst accepts any len_ns>0; delay.rs:708 saturates to u64::MAX on the model axis; delay.rs:434 `x + self.origin` then overflows (u64::MAX + origin). add_burst(now, u64::MAX) ("forever") is enough. wrap's own profile path cannot overflow (end - origin + origin = end). Rank medium (spin in release: clock thread + pump at 100 %, the burst end never passes since step_to filters x <= t).
- F7 (low) per-connection fixed memory: Conn holds 4 Lateness (late, late_run x2), each 2 x Box<[u64; 8,440]> (stats.rs:11-15, :62-68) = 135,040 B; 540 KB per DelayStream, allocated at wrap before any WebSocket byte, plus parse (8,219 B) and scratch (16 KB). 512 peers = ~290 MB; 1,000-connection test allocates it 1,000 times. Bounded per connection, but the count of connections is the client's: U6's accept cap must state it.
- tcpinfo re-check: 3 unsafe blocks, not 2 (srtt_us getsockopt :148; int_option getsockopt :192 inside a closure covered by the fn's allow; ioctl :235). Each one op + SAFETY. BorrowedFd lifetime makes the SO_* -> ioctl sequence TOCTOU-free (fd can't be closed/reused while borrowed). Sound.
- ws_frame re-check: Header buf index <= need-1 <= 13 (need<=14); Payload take bounded by input; no alloc; server scanner never errors (check() false); OK.
- delay_clock: schedule/table in sync (remove old (d,s,key) on replace; pop -> table.remove); no lost wake (wait_until set under lock; notify when due < wait_until; thread re-checks before waiting); Duration::from_nanos(due-now) safe (all due<=now popped first). Lock order conn -> clock state -> (clock thread) info; nothing takes conn under clock lock. OK.
- Pump lifetime: dropped flag + pump_waker wake -> Ready -> Pump::drop releases key; reschedule refuses after dropped/pump_done; no entry can leak after Pump::drop (every reschedule caller checks pump_done under the same lock). OK.
- F1 REPRODUCED (scratch copy .gate/u3c-review-f1, a test appended there only; target .gate/u3c-review-target): DelayStream over a duplex at quiet P2, add_burst(now+1 ms, len). Control len 10 s: Ok, 0 pump polls in the next 200 ms. len u64::MAX, debug: PANIC "attempt to add with overflow" at delay.rs:434:55 (due), the burst already pushed, so every later current()/poll then panics in its caller. len u64::MAX with [profile.dev.package.npt-net] overflow-checks=false (= release, Cargo.toml:91-95 covers sim/proto only): Ok, then 3,870 pump polls in 200 ms on an idle connection, pump never ends (busy spin of clock thread + pump). Rank MEDIUM. Fix: refuse start_ns.checked_add(len_ns) == None in DelayStream::add_burst (clock axis), and saturating_add in due(); a test with u64::MAX.
