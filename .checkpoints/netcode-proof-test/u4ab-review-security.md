# security-auditor: U4a / U4b round 1 review (netcode proof test)

Started 2026-11-10. Read-only on both worktrees. Focus: U4a decoders as attack surface. U4b: not reviewed.

## Plan
1. Read plan §5 U4a + §4.1/4.3/4.4-4.6/4.10/4.11/4.14, advisor rulings, carries 1-5 and 34, catalogue doc, netcode rules.
2. Read U4a builder notes (u4a-build.md).
3. `git status` / `git diff` in the U4a worktree.
4. Per decoder (bits, control, input, snapshot, wrap, msg, quant, hash, limits, error):
   size check before alloc/parse; length prefixes; integer overflow in sizes/counts; panic paths
   (index, slice, unwrap, arithmetic overflow, shifts >= width, as-casts); trailing bytes; unknown fields/
   reserved bits; per-type and per-channel limits.
5. Tests: wire.rs, bites.rs, catalogue.rs, common/mod.rs: do they prove a2 prefixes + random buffers, a3, a6,
   catalogue items 2, 3, 6.
6. check.mjs / check.test.mjs: what it checks (panic lint? unsafe?), clippy.toml.
7. Weigh skeptic S1 (layout shared by own-state and remote groups).
8. Findings with file:line, scenario, cheapest proof, before-merge flag.

## Progress
- 1 done: plan §4.1/4.3-4.6/4.10/4.11/4.14/§5 U4a read; u4a-build.md read; catalogue rows :61-101, :136-137 read.
- 2-4 done (read): bits.rs, error.rs, limits.rs, msg.rs, input.rs, quant.rs, snapshot.rs, control.rs, wrap.rs (part),
  hash.rs, lib.rs, clippy.toml; seam.rs:85-195 and grid.rs:40-100 for layout/Field bounds.
  Totality walk: every index in msg.rs decode (378-379, 392) is behind `bytes.get(frame)` at 369; row()/content_limit()
  only after the 1..=15 check (371); BitReader get/array/take all checked (bits.rs:69-128); layout <= FIELDS=12 <= 16
  by const asserts (seam.rs:184-195, snapshot.rs:73), Field bits 1..=32 (grid.rs:59-69 asserts min<max), so no shift
  overflow on decode; groups-1 in Snapshot::put guarded by group>=groups (snapshot.rs:460). overflow-checks=true in
  release for npt-proto (Cargo.toml:111-112) - any arithmetic overflow would be a panic; none found on decode paths.
  Allocation: Input with_capacity(<=8) after the count cap (input.rs:200-204); note <=480 after len check (control.rs
  228-241); peers with_capacity(<=16) (control.rs:389-392); Snapshot/FullState Vec::new + push bounded by the size
  check (each item >= 21 bits). No allocation before size checks.
- 5 done: wire.rs (a1, a2 prefix + random + flips, a3, a6, a8), catalogue.rs diff (items 1, 2, 3 size half, 6).
- 6 done: check.mjs diff (adds checks, weakens none; STEPS called with no args at :1644), check.test.mjs no side effects.
## Findings (one line each, as found)
