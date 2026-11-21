# U5 allocation bounds per fuzz target (test-engineer, 2026-11-21)
Task: for each of U5's 17 fuzz targets (PM-0001-34), the largest single heap allocation any input within the type's
wire limit can make its decode path ask for (option (a) of U5-ALLOC; advisor-u5-brief.md :39-45, :63-77, :244-261).
Read at head a94c865 (branch claude/netcode-proof-test-plan-hcky5g). Read only, except this file and .gate/u5-probe.

## Plan
1. Read limits.rs:12-51, messages doc §2 (the limits). 2. Read msg.rs `decode` (:303) and every per-type decode it
reaches (input.rs, snapshot.rs, control.rs, wrap.rs, bits.rs, quant.rs, error.rs). 3. Read seam types (sim seam.rs:
Command, EntityRecord, EntityDelta, ...). 4. Read client lib.rs receive path for Snapshot/FullState and the scenes.
5. Read ws_frame.rs Scanner. 6. If disk > 6 GiB: scratch worktree, size_of probe under the lane lock; else arithmetic.
7. One block per target below, each with sites, reach, element size, bytes at growth, size-first check.
8. Summary table.
Disk at start: 7,705,645,056 B free (above 6 GiB = 6,442,450,944): probe allowed.

## Notes (one line per finding, as I go)

## Blocks (one per target; a restart continues after the last complete block)
