# U4a + U4b review, round 1 (netcode-architect)

Started 2026-11-10. Read only: no edit, build or run in either worktree.

## Plan
1. Read the approved plan u4-build-plan.md §5 U4a/U4b and §4.1, §4.3, §4.4-§4.6, §4.10, §4.11, §4.14.
2. Read advisor-u4.md (U4 verdict, Rulings, Ruling: U4s, Ruling: F1 and F2) and u4s-carries.md (1-5, 34, 6, 35).
3. Read docs/design/netcode-proof-messages.md and .claude/rules/netcode.md.
4. Read the builders' notes: u4a-build.md, u4b-build.md.
5. U4a: git diff/status in its worktree; read crates/proto src (bits, control, error, hash, input, limits, msg,
   quant, snapshot, wrap, lib), tests, check.mjs. Check each message's fields, channels, directions, limits; decode
   order and refusals; Wrap compares; snapshot header and group split; EntityRecord mapping; -57/-17/-21/-58 bites.
6. S1: read the skeptic note at 6652be4, seam.rs:114-137; decide cheapest correct fix and when it must land.
7. U4b: git diff in .gate/u4b-wt; read schedule.rs, lib.rs, tests/schedule.rs; ADR 0004 :207-231; marks.json
   PM-0004-67's "what"; check due-tick rule, cap, drop and rebase, counters, three bites.
8. Findings: one line each below as found. Then verdicts.

## Progress
