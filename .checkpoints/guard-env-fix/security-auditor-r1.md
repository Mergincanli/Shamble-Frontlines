Task: security-auditor review round 1 of guard-env-fix (diff origin/main...21615cb3 -- .claude/hooks), SEC-U7B-1.

Plan (blocks, in order):
B1. Read the diff (routine-guard.mjs, test file) and plan.md §8 conditions.
B2. Read surrounding code: bash() caller, plainForm/R-PROG, guard.mjs throw handling.
B3. Scratch probe: call unwrap()/decide() on GNU/BSD wrapper forms (no live commands).
B4. Judge getopt mirroring, mutants, messages/anchors, throw paths.
B4b. (after advisor) compare the head with main's unwrap() on the unread-value forms; the loop cap; the echoed word; --help.
B5. Write findings + verdict. DONE.

Notes (no exploit forms here: the exact forms are only in the answer to the caller):
- B1 done: diff read (routine-guard.mjs WRAP_OPTS + unwrap 295-340, caller in bash() 745-753; tests DAY_CASES rows, 11 DAY_MUTANTS, 3 routine rows); plan §8 conditions read.
- B2 done: one caller of unwrap() (bash():746; grep over repo); routines untouched (unwrapRole); KIT_AGENT deny for agents, dayAsk ??= for main session, scan goes on.
- B3 done: probe sa-r1-probe.mjs (unwrap only, forms given on its command line). The getopt-shaped forms tried either reach the real program with unreadable null where GNU getopt also does, or set unreadable.
- Finding SA-R1-1 (Low): one extra mutant on the `--` handling (routine-guard.mjs:318) survives the DAY_CASES; security-neutral (it only changes forms where git never runs); fix: one more ask row. The other nine extra mutants are killed.
- Out-of-scope additions: 2 (same class, other wrappers), plus 2 already under stated limits; detail only in the answer.
- Full suite rerun on head: routine-guard.test 1237/1237 (hooks unchanged since 21615cb).
- Finding SA-R1-2 (rating pending B4b): one of env's read-but-unfollowed options can hold the program in its value; unwrap() then takes a later word for the program, so a deny on what runs does not fire (an ask instead). Where: routine-guard.mjs:320. Fix: read the value's words in front of the rest, unreadable still set; a row and a mutant.
- B4 done: throw paths (none found), messages (anchors asserted once by the runner, suite green), routines (R-PROG first, rows present).
