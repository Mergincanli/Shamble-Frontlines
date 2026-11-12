# U4d-F round 2: test-engineer (on what changed: FX-1 and FX-2)

Copy: /home/user/Shamble-Frontlines/.claude/worktrees/wf_9ce88fa3-452-1 (branch u4df-build, HEAD dad9566, change
uncommitted). Round 1's files: /home/user/Shamble-Frontlines/.gate/u4df-r1. Packet: u4df-r2-packet.md (read whole).
Plan judged against: advisor-u4df.md R2 and T3. Role: main's .claude/agents/test-engineer.md (read, via git show
origin/main). Runner: u4df-run.sh, log prefix te2- under .gate/u4df/. Rules: plants by Edit in place, restored by Edit,
sha256 checked against r2-start.sha256; TE2-5 for any failure I did not plant (re-run that crate with --no-fail-fast at
once, name and panic here). Notes kept by Write/Edit only.

## Plan
1. sha256 of every file under the copy's spikes/netcode-proof/crates/sim (paths from the copy's root) -> Write to
   .gate/u4df/r2-start.sha256.
2. world.rs, tests/world.rs, tests/replay.rs: sha256 = round 1's [1] and = .gate/u4df-r1's; replay.rs and scene.rs:
   diff against .gate/u4df-r1's = FX-1 and FX-2 only (read the diff against R2 and T3, word by word for FX-2; rewrap
   <= 120).
3. FX-1's test by name in both builds (measured and --all-features), --nocapture, its counts line; its run time.
4. Plants Q1-Q5 (R2's forms; the builder's record for reference), each by Edit in replay.rs, the whole measured npt-sim
   suite --no-fail-fast, -x; each must fail FX-1's test by name; restore by Edit; sha256 -c after each. Plus at least
   one wrong-reckoning plant of my own design that R2 does not list (FX-1's reach).
5. The three-seed test once by name: its 21 "ES seed" lines = end-three-seeds.log's.
6. Settle by run any netcode-architect round-2 finding a run can settle (u4df-r2-netcode.md).
7. check.mjs once at the end, -x, counted: every step's line; deltas on 367 / 414 / 52 (expected 368 / 415 / 52).
8. sha256sum -c r2-start.sha256 from the copy's root at the end: all OK.

## Progress
