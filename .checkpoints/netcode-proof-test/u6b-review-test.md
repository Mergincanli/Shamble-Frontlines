# U6b review, the test seat (test-engineer, xhigh), 2026-11-23

Inputs checked by sha256 at start: brief 32baaebd... OK; advisor check 3c0f8121... OK; snapshot patch 49f4488f... OK;
builder notes be6807f4... OK. W = /home/user/Shamble-Frontlines/.claude/worktrees/agent-a2661dc320c9e6d00/spikes/netcode-proof.
No test-engineer memory folder exists (.claude/agent-memory-local/test-engineer absent).

## 1. Plan
1. Read the advisor's check (conditions bearing on tests: 1, 3, 5, 7, 8) and the builder's notes (Plan, items 1-6,
   plant table, counts).
2. Section 3's named tests, block by block: list each test the brief names, find it in W (grep the patch's test names),
   read its body, judge whether it can fail for the claim it is named for (negative cases, injected clock, no timing bar).
3. Plants: for each of P1-P11, P14-P16, read .gate/u6b/plant-p*.log: the planted edit, the named test failing, the
   failure's reason matching the plant, the restore and its sha256 against the tree.
4. The bite PM-0001-18/ws-budget-x2: find `with_bite`, `Bite::BudgetX2`, the two bites tests; check per-instance arming
   by id, the other instance at the formula, the armed instance failing test (c)'s bound; check it is compiled out of
   the measured build (SEC-U6BD-2).
5. Counts: section 3's line as rev 1.1 condition 5 restates it (+45 measured, +47 bites) against check5.log and base
   (754, 850); the wt module tests joined by Block 0.
6. Test step time 891.5 s vs base 954.8 s (1.5x stop).
7. End-to-end tests over WtClient and WsClient at P2 (tokens, snapshots, no kind-10).
8. Timing: the two 20-iteration kick tests under the 5 s cap (condition 8); F-P14's note; the proc_read wait; flake risk.
9. F28's test (npt-super log flushed per record).
10. Container readings: what their record must show.
11. Run a test only where a claim needs it, through u6b-run.sh, logs .gate/u6b/rev-test-<n>.log.

## 2. Notes (one line per finding, saved as I go)
