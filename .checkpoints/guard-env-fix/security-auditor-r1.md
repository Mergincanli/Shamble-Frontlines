Task: security-auditor review round 1 of guard-env-fix (diff origin/main...21615cb3 -- .claude/hooks), SEC-U7B-1.

Plan (blocks, in order):
B1. Read the diff (routine-guard.mjs, test file) and plan.md §8 conditions.
B2. Read surrounding code: bash() caller, plainForm/R-PROG, guard.mjs throw handling.
B3. Scratch probe: call unwrap()/decide() on GNU/BSD wrapper forms (no live commands).
B4. Judge getopt mirroring, mutants, messages/anchors, throw paths.
B5. Write findings + verdict. DONE.

Notes:
