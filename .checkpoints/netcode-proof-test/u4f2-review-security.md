# U4f2 review round 1: the security seat (security-auditor), 2026-11-20

Scope: hostile input by u4f2-brief.md section 7 (the forged kinds, the attribution, `--forge-every`'s parser), as the
panel brief's security bullet (u4f2-panel-brief.md:50-54). Read only: the patch
`.checkpoints/netcode-proof-test/wip/u4f2-build.patch` and the builder's worktree
`/home/user/Shamble-Frontlines/.claude/worktrees/agent-aa62d31b6fa5b9807` by absolute path. No runs.

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

