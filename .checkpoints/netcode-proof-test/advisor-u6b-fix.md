# Advisor: U6b's fix-pass check (A3 of advisor-u6b-r1.md), 2026-11-23

Instructions: main's `.claude/agents/advisor.md` (`git show origin/main:.claude/agents/advisor.md`, 335 lines), read at
the start of this check. My ruling: advisor-u6b-r1.md sha256 dc6973da…35f5 (checked). Snapshot wip/u6b-fix.patch
e29c003c…ac64af1c (checked); the round's wip/u6b-build.patch 49f4488f…0c14c (checked); notes u6b-build.md 74e0d697…
(checked). T = the built tree .claude/worktrees/agent-a2661dc320c9e6d00/spikes/netcode-proof, read only.

## 1. Plan
1. Inputs: hashes (done above); the three reviews' Medium and Low rows, word for word; the brief rev 1.2's section 0.1
   condition 1, section 2 (U6e line), section 3 (bites, container readings), condition 5.
2. Item (1): for each Medium (SEC-U6B-1, -2; NR-U6B-1, -2; T-2..T-9) read the seat's fix words, then the tree at the
   notes' cited lines (base-vs-tree diff of the touched file where needed). Each built Low the same.
3. Item (2): R-PING against rev 1.2's paste-ready text: size bound, queue left out, counts in ws_accept + two
   SERIES rows, trait doc, the three tests, P17.
4. Item (3): grep T for WS_BUDGET_BITE / ws-budget / with_bite on WsServer / BudgetX2 outside cfg(test); ws_gate.rs vs
   base; main.rs bite list.
5. Item (4): every fx-plant-*.log (20) and every plant-*.diff in the worktree's .gate/u6b/: named test FAILED, the
   restore, the diff file's content matches the plant described.
6. Item (5): fx-check1.log and fx-check2.log: 11 steps, counts (754/850 base; 799/898 snapshot; 807/904 now), diff
   hash, test time.
7. Item (6): the size: the patch's own counts (git apply --numstat), the notes' running line, the 5,950 scope line and
   6,000 stop.
8. Item (7): fx-starts.log and fx-release.log: diff hash equal to fx-check2's, the 20 values, p50/max, binary hash.
9. Item (8): carries by number: the fix list section 3, my ruling's section 3 item 9, the builder's not-built Lows.
10. F-TICK-ROUND: the arithmetic and the tree's N test.
11. Findings, verdict, carries, Mustafa, DONE. A cargo test only if a claim needs it (through u6b-run.sh).

## 2. Items
