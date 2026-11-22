# Advisor: U8's build brief, rev 1 (one check under NPT-TRIM)

Working from main's `.claude/agents/advisor.md` (git show origin/main, 335 lines, read 2026-11-22).
Brief: `.checkpoints/netcode-proof-test/u8-brief.md` (368 lines). Drafter's notes: `infra-u8-brief.md`.

## Plan
1. Read the rules in force: history.md:7561-7607 (NPT-TRIM, his answers of 2026-11-22), :2234-2282 (TEST-FIRST).
2. Read replan.md, replan-units.md §3, §5, §7; plan.md:1256, §7.2-§7.3, §12.2-§12.3.
3. Read security-r1-deps.md (SEC-R1D-2, -3, -5, -6, npm rules), advisor-u6a-brief.md cond 2-4, u6a-brief.md rev 1.1.
4. Check facts: tests.rs:1407, advisor-u3d.md:80-83, u3c-kernel-check rig, prices, arithmetic (hours, cap, lines, days).
5. Rule Q-U8-1 to -7; the U6c test change; size; R1's run plan and dates; stale plan.md lines; Stop B; other findings.
6. Verdict and paste-ready conditions.

## Notes (one line per finding)
