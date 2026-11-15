# BRAWL-1 review, round 1, netcode-architect seat

Snapshot: wip/brawl1.patch sha256 ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7 (verified 2026-11-15)
Packet: brawl1-packet.md sha256 16eab462c4e0fcbcd8360aad7a951207275927c7f0f6813b794df54c12f4b1a4 (verified)
Read copy: .claude/worktrees/brawl1 (352a245 + patch uncommitted)

## Plan
1. Read the packet: approved plan, conditions 1-18, rulings copied.
2. Read the patch whole.
3. Focus items:
   a. client read item: newest complete Hash-filed tick by endpoint wrap rule; dead bit.
   b. leash under a lead's delay and under B1 bursts (conditions 1-3; pattern (iii)).
   c. room log in-building figure (condition 10).
   d. Part 1 refactors (wake step, run_leg SimClock, shared constructor): behaviour change?
4. Run tests via u4r-run.sh if useful; logs to .gate/u4r/brawl1-review-netcode/.
5. Findings, one line each as found.

## Progress

## Findings (one line each)
