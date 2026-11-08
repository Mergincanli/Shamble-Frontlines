# U3c skeptic round: test-engineer notes

Scope: git diff 2c68bf9..989118e -- spikes/ ; plan u3c-plan.md rev 2 [C1]-[C16]; plan.md rev 3.3 U3c text.
Focus: proof per item/condition (bites, mutations), model-state vs wall clock judgement, real-time flake risk,
wt_tests FT1-FT4 fix, check.mjs deps checks + self-tests, evidence scripts' pass rules, commit-message claims.
Build dir: CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/u3c-review-target (no network changes).

## Plan
1. Read u3c-plan rev 2, advisor notes, build/critic notes.
2. Diff stat, read each file in diff.
3. Map conditions to tests; check bites.
4. Run tests (cargo test) where cheap; check.mjs self-tests.
5. Findings.

## Notes
