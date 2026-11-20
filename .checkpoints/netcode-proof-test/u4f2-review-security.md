# U4f2 review round 1: the security seat (security-auditor), 2026-11-20

Scope: hostile input by u4f2-brief.md section 7 (the forged kinds, the attribution, `--forge-every`'s parser), as the
panel brief's security bullet (u4f2-panel-brief.md:50-54). Read only: the patch
`.checkpoints/netcode-proof-test/wip/u4f2-build.patch` and the builder's worktree
`/home/user/Shamble-Frontlines/.claude/worktrees/agent-aa62d31b6fa5b9807` by absolute path. No runs. (This agent has
no Edit tool: the file is rewritten whole with Write at each block.)

## Plan
1. Read u4f2-build.md (the builder's record: blocks, plants, F-A..F-E, departures). Check the patch's sha256.
2. Threat model: each forged kind (3, 7, 8, 9, 10, 11) as a hostile client's command. For each, find the room's own
   refusal check (crates/room input.rs / room.rs) and the forger (crates/bots forged code), and confirm the refusal
   depends only on room/sim state, never on a client-supplied claim.
3. The rebuilt kind 7: fire edges inside the cooldown, reload edges inside the reload. Where does the room/sim refuse
   them, and which sim state decides? Weigh F-B (dead forger, respawn reset) from the hostile side: can a forged edge
   while dead or just after respawn change state (fire, reload, cooldown skip)?
4. The attribution (C-U4F2-3, SEC-1): `CommandRefusal`, `RefusalRecord { slot, seq, kind }`, the records' bounds,
   `refusals_dropped_count`, what the lab reads, whether a hostile client can flood records (unbounded growth) or hide
   its refusals behind honest ones (attribution matching by (slot, seq, kind)).
5. `--forge-every`'s parser: below 2 a usage error; 0, 1, negative, overflow (u32/u64), odd values, non-numeric, leading
   +, whitespace; the interval's use in arithmetic (modulo, multiplication overflow).
6. Any new path where a forged command changes state (room or sim): every new branch in room.rs/input.rs.
7. Panics reachable from input in the new code (indexing, unwrap, arithmetic overflow) in room and lab paths.
8. Findings, coverage table, verdict.

## Notes (one line per finding as found)
- Stamp: the patch hashes to e9a0ae3e3f48af18**e**a7a947fdf9aba849061fa32fbd91876a948c46f3046406b (64 hex); the panel
  brief (u4f2-panel-brief.md:11) prints 63, one "e" dropped after "18". The patch read is the one named. (A wording
  slip in the panel brief, not the diff; no finding.)
- Room (patch :2134-2410): a rename plus the struct form only; `receive_input` (room.rs:932-1035) unchanged in logic.
  Records capped at REFUSALS_HELD 4,096 per tick (room.rs:37, :944-950), the rest counted in a u64: bounded.
- forged.rs: tallies keyed by the 16-bit seq (BTreeMap/BTreeSet), so per-forger memory is at most 65,536 entries
  whatever the run length (the module text's "bounded by its length" holds, and better). Dropped room records can only
  raise `unrejected_count` (no record = unrejected for room kinds), never hide an item: SEC-1's direction is safe.
- Parser (scenario.rs:2240-2247): `parse::<u64>()`, `k >= 2`; "0", "1", "x", "-1", "" refused; "+12" and "012" taken
  (Rust's unsigned parse takes a leading '+'); no upper bound.
- bots lib.rs:944 `f.every * u64::from(f.burst_refused())`: unchecked multiply of the operator's interval (x 8 under
  A, x 4 under B). Cargo.toml:116-120 turns overflow checks on in release for npt-sim and npt-proto only: npt-bots
  wraps in release, panics in debug. -> SA-U4F2-1 (Low, below).
