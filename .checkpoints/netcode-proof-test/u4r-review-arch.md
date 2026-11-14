# U4r review, round 1: engine-architect seat (batch 2)

Snapshot under review: .checkpoints/netcode-proof-test/wip/u4r.patch, sha256 572c4f69435da2ad6f62d54d4bcbc47b04d6329b5493e0faf78fcecfdc4a09c2 (checked by sha256sum at start: equal).
Packet u4r-packet.md sha256 cbd82485... (equal); u4r-packet-tests.md sha256 d12555c8... (equal).
Read copy: .gate/u4r-snap1 (2fbec77 + patch). 26 entries in git status (13 M, 13 ??).
No engine-architect memory on disk (.claude/agent-memory-local/ holds only security-auditor).

## Plan (seat list: advisor-u4r-panel-full.md section 4)
1. Public items: every pub item the diff adds, by crate; compare to brief 2.7 allowed list + rulings (with_bite, contract/precheck subcommands). Report's list if on disk.
2. check.mjs changes counted as d2-r2 counted them (EA2-1, ruling 1 point 1): wasmStep's two options + defaults; carries 83 (b)-(d), 84, 85; new exports; det-u4.mjs imports wasmStep.
3. The seam (2.6): only merged pub items of npt-room/client/server/sim; no source change beyond lane; nothing depends on npt-lab; no other crate's bite id spelled in npt-lab non-test sources.
4. U4R-Q6 (a) server items: with_bite shape, BITES at four + pin in tests/schedule.rs, loopback.rs one-home test.
5. My earlier findings: carry 82 (EA-8), 83 (a)(d) (EA-3, EA-6, EA2-3), 75 (A3), 76 A6 half, 59 (A2-L3), 3.3 carries 6 and 20 (arch F6, arch N2).
6. Report's statements of missing read-only items as follow-ups (r4 spatial half, U6 bots' view, HISTORY_MS at U4h); ruling 4 point 2's bite-id choice.

## Progress
