# U3C-HELD-1 plan drafter: notes

Role: the plan drafter (read only; writes only this file and u3c-held-1-plan-draft.md). Ran nothing.

## Plan of my work
1. Read the card's finding (netcode-proof-test.md:3870-3885), merge.log:51-52, c62h-bound-build.md:426-455,
   the advisor's merge-stop ruling.
2. Read both design notes in full (netcode-architect-design.md, test-engineer-evidence.md).
3. Re-check the key cited code: delay.rs (check_held_bound, held_now, down_free, model_write, harvest, pump's
   loops and closing block, the two growth sites), delay_tests.rs (G-NC2, the stalled-client test), SERIES.md
   rows, u3c-plan.md:122-125.
4. Read check.mjs's test step and FX-10, main-run.sh's usage and floor, plan.md §12.4, brief 8.1, R4,
   history.md:2187 (the one-command-per-call rule).
5. Note every disagreement between the two notes.
6. Write the plan draft (under about 220 lines).

## Findings (one line each, with sources)
