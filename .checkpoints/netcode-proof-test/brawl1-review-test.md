# test-engineer: BRAWL-1 round 1 review (PR #40, spike branch claude/netcode-proof-test-plan-hcky5g)

Snapshot under review: wip/brawl1.patch sha256 ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7 (verified by sha256sum).
Packet: brawl1-packet.md sha256 16eab462c4e0fcbcd8360aad7a951207275927c7f0f6813b794df54c12f4b1a4 (verified).
Read copy: .claude/worktrees/brawl1 (352a245 + uncommitted change). Logs of my runs: .gate/u4r/brawl1-review-test/.

## Plan
1. Read packet rulings (conditions 1-18, N2-1, TE-R2-1/2, RR4-L1/L2, EA-U4R-1/2, RR3-L2).
2. Read the diff per file: bots/src/lib.rs (decision fn, leash, cfg(test) module), client lib + endpoint test,
   scenario.rs (fp3, N2-1 4th check, parse caps, constructor/wake), logs.rs (figure, no-log flag, unit test),
   precheck.rs, hash.rs (MAX_ENTRY_TICKS, r10_fp3), manifests, SERIES.md.
3. Check each test against conditions 2-5, 12, 13: can it fail? negative cases? exact texts?
4. Read the build logs: .gate/u4r/brawl1/ p1-*.log (proofs a-e, cond 6), p2-*.log, repin*.log, final-*.log.
5. Plants (cond 6 / Q5): read logs of the two N2-1 plants; if missing, note it.
6. Run what is cheap through u4r-run.sh if a claim needs it.
7. Findings with file:line, then StructuredOutput.

## Findings (one line each, as found)
