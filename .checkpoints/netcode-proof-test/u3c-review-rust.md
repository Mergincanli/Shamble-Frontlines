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
