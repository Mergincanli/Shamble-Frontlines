# U4e round 2, test-engineer seat (on what changed: FX-1 to FX-13)

Seat: test-engineer (role file read from origin/main). Copy: .claude/worktrees/wf_0fa6595a-3e2-1 (uncommitted on
97bb9d3). Log prefix: te (.gate/u4e/r2-te-*.log). Judged against R1-R8 and T1 (u4e-r2-packet.md :1761-2160).

## Plan
1. Start hashes: the 23 changed/new files under crates/ hashed, saved as .gate/u4e/r2-hashes-start.sha256, checked
   equal to fx-hashes-end.sha256 (`sha256sum -c` from the copy's root).
2. Read the builder's record, "Fix pass, round 1": each FX's test name and plant; read each FX's diff in the packet and
   in the copy; judge each test against R1-R6's form (what it must assert).
3. For each FX (FX-1 (a) and (b), FX-2 to FX-13's tests), plant by Edit (old/new text recorded below), run through the
   runner with `-x` and `--no-fail-fast`, read the test runner's summary line, restore by Edit, check the file's sha256
   against step 1's. My own form where I doubt the builder's.
4. Round-1 test findings in the fix pass (TE-2 to TE-6, TE-7 and TE-8 riders): each has a test a plant makes fail.
5. Settle by run what other round-2 seats' findings a run can settle (u4e-review-r2-{netcode,security,arch,rust}.md).
6. One counted check.mjs (`-x`), every step's line, the counts as deltas on 458 / 524 / 52 and on 473 / 541 / 52.
7. End: `sha256sum -c` of r2-hashes-start.sha256 from the copy's root; verdict.

## Step 1: start hashes
- 23 files hashed (sha256sum from the copy's root), saved as .gate/u4e/r2-hashes-start.sha256 with Write.
  `cmp` with fx-hashes-end.sha256: identical; `sha256sum -c fx-hashes-end.sha256`: 23 OK, exit 0. git status: 6 modified,
  17 new under crates/ (plus the record u4e-build.md, outside crates/).

## Findings and plant log
