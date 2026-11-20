# Advisor ruling: U4f2's fix pass part A (2026-11-20), F-G, the early runs, and what part A's report changes

Read only (this file is my one write). Role: main's `.claude/agents/advisor.md`, as in advisor-u4f2-r1.md (left as it
is). Sources: the coordinator's message; history.md:7402-7410 (his words); the build record's "Block 9: the fix pass";
.gate/u4f/u4f2-fp*.log; the worktree (absolute paths only); wip/u4f2-build.patch (sha256 23c5f788…).

## Plan
1. Read his words (history.md:7402-7410), Block 9, the fp logs (T-K7's interval-3 form at tick 257, plant 9's B
   result, the size and times), and the code: bots `forge_step` and `fire_rest_at`/`forger(7, …)`, the twin path.
2. F-G: confirm the mechanism in the code; rule keep 3 (a limit) or require `every` above the rest.
3. FP-5's plant 9 under B: does the result hold, what F-E now means, and anything owed.
4. Early runs: whether R4 and R5 may run on part A's tree and count; what re-runs after FP-2.
5. Size against my 2,700 stop with FP-2 to come; the test time; the disk; the slips.
6. Tables and the short return.
Each block below as it finishes.
