# U6a round 2, test seat (NPT-TRIM: what the fix pass changed only), 2026-11-23

Inputs: u6a-fix-pass.md (FP-1 to FP-12), u6a-build.md (FP-n lines), wip/u6a-build.patch (expected sha256
022ddb84...98e1; 19 files; 5,843 added), .gate/u6a/fp*.log, check4.log, fp-restored.log. Tree read only:
W = /home/user/Shamble-Frontlines/.claude/worktrees/agent-a95ebf14368f277ff/spikes/netcode-proof
Round 1: u6a-review-test.md (TR-U6A-1 to -12).

## Plan
1. Check the patch hash; read u6a-fix-pass.md and the FP lines in u6a-build.md.
2. (1) TR-U6A-1: super's new header test and the reworked double-load bite vs -16's check and the judge's rules.
3. (2) Each new or changed test's assertions (FP-2, -3, -4, -6, -7, -8, -9, -10, -11, -12).
4. (3) Plants: P7/P7b on the final tree and option A; P8, P9 (+FP-4), P12-P14; fp-restored.log's hashes.
5. (4) check4's counts against section 3's bars.
6. (5) Anything the fixes broke in round 1's scope.
7. Verdict, table, not verified, DONE.

## Notes
