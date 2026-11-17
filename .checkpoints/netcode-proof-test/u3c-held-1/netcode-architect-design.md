# U3C-HELD-1: netcode-architect's design notes (cause and fix)

Agent: netcode-architect. READ ONLY (no builds, tests, cargo). Writes only this file (Write tool).
Branch: claude/netcode-proof-test-plan-hcky5g (PR #40).

## Plan
1. Read the card (netcode-proof-test.md:3870-3885), the kept evidence (merge.log:51-52, c62h-bound-build.md:426-455,
   advisor-c62h-bound-merge-stop-full.json C3-C4).
2. Read delay.rs: the failing test and every function it calls; grep held, down_free, released_model, 64 KB bound.
3. Explain the transient (order of writes/releases, which counter moves first, when the test samples, what
   held_over_bound [1, 0] means); the bites build's feature-gated code and why only once.
4. If the code does not support the release-transient story, say so and give the cause it does support.
5. Fix design: the check, where evaluated (every write), symbols; diff as text.
6. Planted bug: exact plant, place, expected failure line.
7. NC-U3c-2 bound sentence: every place, file:line, which become wrong.
8. Size estimate with basis.

## Findings (one line per finding, appended as found)
