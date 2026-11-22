# Advisor: U6b's build brief, rev 1 (the one check NPT-TRIM allows)

Instructions: main's `.claude/agents/advisor.md` (git show origin/main:.claude/agents/advisor.md, 335 lines), read 2026-11-22.
Rules in force: NPT-TRIM (history.md:7561-7593), his answers of 2026-11-22 (history.md:7595-7607; R1 Thursday 3 December),
TEST-FIRST (history.md:2234-2282).

## Plan
1. Read the brief (u6b-brief.md rev 1) and the drafter's notes (netcode-u6b-brief.md).
2. Read the carries: replan.md, replan-units.md §1 §2 §7, u6-brief.md rev 2.1, u6-facts.md, advisor-u6-brief.md,
   plan.md:1254, u6a-brief.md rev 1.1, advisor-u6a-brief.md (conditions 2, 3, 6), security-r1-deps.md (:249-253),
   u8-brief.md (DelayStream self-test, run plan), advisor-u8-brief.md if it exists.
3. Check the code facts the brief leans on (spikes/ crates, Cargo manifests, lock) by grep.
4. Rule: Q1-Q6; the seven contradictions; size and R1's date; Block 0's manifests and security's read; the time trap;
   anything else.
5. Verdict with paste-ready conditions; summary.

## Notes (one line per finding, as I go)
