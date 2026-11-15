# BRAWL-1 round 1: rust-reviewer seat (PR #40, claude/netcode-proof-test-plan-hcky5g)

Snapshot under review: wip/brawl1.patch sha256 ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7 (verified
by sha256sum, 1482 lines). Packet: brawl1-packet.md sha256 16eab462c4e0fcbcd8360aad7a951207275927c7f0f6813b794df54c12f4b1a4
(2414 lines). Read copy: .claude/worktrees/brawl1 at 352a245 + the change uncommitted (10 files modified).

## Plan
1. Read the packet: approved plan, conditions 1-18, the rulings it copies; list what the seat must check.
2. Read the patch whole; then the touched files in the read copy (bots/src/lib.rs, client/src/lib.rs, client/tests/endpoint.rs,
   lab/src/{hash,logs,precheck,scenario}.rs, the two recordings json, marks/SERIES.md).
3. Focus checks: integer steering (overflow, 127^2 bound, no floats); parse() 64/32-bit bounds and order; manifest
   entry-ticks cap; no-log flag in logs.rs; crate-private surface and #[cfg(test)] placement (cond. 4); determinism (no
   time/randomness outside seeded paths).
4. Runs (runner only, logs under .gate/u4r/brawl1-review-rust/): clippy -D warnings, fmt --check, the tests touched;
   probes only as findings (no edits to the read copy).
5. Findings: a line each below as found. Then verdict.

## Progress
- step 0 done: sha256s verified; notes started.
- step 1 done: packet read whole (2414 lines): plan s1-7, s11-12; conditions 1-18; sets ruling Q1 + BRAWL-1; r2 rulings
  (TE-R2-1/2, N2-1, RR4-L1/L2); r1 (EA-U4R-1/2/5, RR3-L2); build ruling 1 points 1 and 7; carries 18/39/74/75/82/87.
- step 2 in progress: code read (bots lib.rs whole, client own_soldier + ring, proto quant ranges, wrap.rs, lab
  parse/burst_starts/manifest/logs filed+finish+room_tick, Server::build, wake, Bites identity, SimClock).

## Findings (a line each, as found)
