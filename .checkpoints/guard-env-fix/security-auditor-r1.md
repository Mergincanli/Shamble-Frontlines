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
- B4b done (scratch compare of main's and the head's unwrap(), outside the repository):
  - SA-R1-2 rated Medium: for the separate-value spellings main reached the program in the value and denied; the head asks (a deny lifted to an approvable ask in the main session; agents still refused). For the attached spellings main passed silently and the head asks (an improvement). Verdict REVISE on this alone; fix as noted above.
  - Finding SA-R1-3 (Low, pre-existing, same on main): unwrap()'s iteration cap (routine-guard.mjs:308) ends with a wrapper left as the program and unreadable null; arguably under stated limit L16; fix: set unreadable when the cap is hit with words left, plus a row.
  - Finding SA-R1-4 (Low): the ask/deny reason echoes the raw unknown word (any length, newlines included: measured 330 chars) into the text Mustafa decides on (routine-guard.mjs:321, :326, :749-751); fix: echo only a capped, printable-only form (e.g. first 32 chars, control chars replaced).
  - --help of env/nice/stdbuf not run: the head's guard itself refuses those forms for an agent (unknown option), so the option lists come from the plan's coreutils 9.4 citation and recall.
- B5 done: verdict REVISE (SA-R1-2 Medium; Lows SA-R1-1, -3, -4). Scratch copies under .gate/guard-env-fix/ (gitignored) left in place.
DONE
