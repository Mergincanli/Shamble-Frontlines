# Advisor: U4 wave 1 rulings (PR #40, branch claude/netcode-proof-test-plan-hcky5g, draft)

Seat: day session, advisor agent run as a general agent reading main's `.claude/agents/advisor.md` (this session
started before main's advisor.md last changed). Read-only except this file. Effort: high (netcode work: the caller
asked for rulings, not an ADR).

## Plan
1. Read the sources: resume card (netcode-proof-test.md top), history.md "Decisions taken" of 2026-11-10, advisor-u4.md
   (verdict, conditions, Rulings, U4s, F1/F2), u4-build-plan.md rev 2 (check sha256), plan.md (the sections the rulings
   touch: §4.1, §4.2, §4.4, §4.14, the U8 row, line 1436), u4s-carries.md, the message catalogue.
2. Read wave 1's records and reviews: u4a-build.md, u4b-build.md, u4c-build.md (context only), u4d-build.md,
   u4ab-review-{netcode,security,rust,test}.md, u4d-review-{netcode,rust,arch,test}.md, skeptic S1 (git show), the U4e
   brief draft rev 2 (git show).
3. Spot-check the code where a ruling turns on it (U4d's worktree: the vehicle parts, the press counters, the stress
   scene, D-8/D-9, load_movers; U4a's worktree: SOLDIER, GROUPS, the snapshot header, the public items).
4. Rule, one block per code, writing each ruling below as soon as it is made:
   W1-U4D (High, each Medium, the press-counter wording, the two tuning choices, carry 13), W1-S1, W1-N2, W1-PUB,
   W1-U4E (Q1-Q6, Q8, Q10, Q11), W1-PLAN, W1-TARGET.
5. Return the report: each code with its ruling in one or two lines, the text changes, Decisions needed, For Mustafa.

## Sources read (filled in as I go)

## Rulings (one block per code, appended as made)
