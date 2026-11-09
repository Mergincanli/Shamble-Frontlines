# U4 build plan (rev 1) review: test-engineer

Task: adversarial review of .checkpoints/netcode-proof-test/u4-build-plan.md (rev 1), against scouts u4-scout-arch.md,
u4-scout-marks.md, u4-scout-netcode.md, plan.md and the code. Focus: requirements and carries met and proven, marks
with tests that bite, judge figures, fixtures and evidence honest, nothing judged here that only a run can judge,
split testable unit by unit. No code edits, no builds, no runs. Memory: no agent-memory dir found (.claude/ has none).
Rule (checkpoint :294-295): notes by the Write tool only, no shell writes, no cd. (The first append went in by a shell
heredoc before I read that rule; this rewrite carries it unchanged.)

## Plan
1. Read the build plan in full.
2. Read plan.md U4 section, R/C carries that name U4; the marks for U4.
3. Read the 3 scouts; check each claim the plan leans on.
4. Check code file:line for the plan's claims (crate layout, existing harness, judge).
5. Findings list; verdict.

## Progress
- step 1 done: read u4-build-plan.md rev 1 in full (646 lines).
- scout-marks read (F1-F32).

## Findings (a line each as found)
