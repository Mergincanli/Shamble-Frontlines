# rust-reviewer: U3c fix pass, part C1 (npt-net), re-review

Scope: `git diff 95597ef..81203da -- spikes/` (C1's files: delay.rs, delay_clock.rs, delay_names.rs, delay_tests.rs,
stats.rs, ws_frame.rs, wt_tests.rs, Cargo.toml, SERIES.md). Build dir CARGO_TARGET_DIR=.gate/u3c-review-target.
No source edited; scratch copies only under .gate/ or the session scratchpad.

## Plan
1. Read the diff of delay.rs (Pump rewrite: F3/F2/F4/F5/F10; add_burst F1/S7/NC-U3c-8; gate_inputs F-L; NC-U3c-4).
2. Read ws_frame.rs (S2) against httparse 1.10.1's line rules (registry source).
3. Read stats.rs (F7 Running).
4. Read delay_tests.rs changes (each finding's test).
5. Build, clippy, fmt, run the l2 tests.
6. Bites (revert in a scratch copy) for the high-value ones: F1, F3/F4 pump, S2, F7, F-K.
7. Race analysis of the pump's lock change (lost wakeups, double commits, partial take, ordering vs current()/add_burst).

## Findings log (one line each as found)
