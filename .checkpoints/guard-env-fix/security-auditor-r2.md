Task: security-auditor review round 2 of guard-env-fix (diff 21615cb3..11887d0b -- .claude/hooks): SA-R1-1/-2/-4 fixes.

Plan (blocks, in order):
C1. Read the round-2 diff.
C2. Probe unwrap() on the -S edge forms (empty, blanks only, last word, nested, value holding -- / -C / NAME=), no live command.
C3. Extra mutants on the new lines; a size/time probe of the re-insertion.
C4. Write verdict. DONE.

Notes (no exploit forms here):
- C1 done: split re-insertion at routine-guard.mjs:324-328 (v from next word or the rest of the word), shownWord at :308, used at :329 and :335; rows test.mjs:1193-1197; one new mutant (:1402).
- C2 done: 23 edge forms probed (scratchpad, outside the repository): empty value, blanks only, -S last word, attached and long-with-= forms, nested -S, values holding --, -C, NAME=, a lone -, -i/-u, another wrapper: no throw; each reaches the program GNU env -S would run, unreadable always stays set. Escapes, quotes and ${} inside the value are not split by the guard: they fall to the ask (never a silent pass).
- C3 done: 9 extra mutants (sa-r2-extra-mutants.mjs). Reason mutants (cut, newline) are caught by the SA-R1-4 rows (assert.match), not by the kind-only mutant runner; tab replacement has no row (nit).
- Finding SA-R2-1 (Low): three mutants on the re-insertion survive (short attached value offset, empty words kept, split on one space): each turns a deny into an ask, never a pass. Fix: DENY rows for the short attached spelling, an empty or leading-blank value, and a tab/double-blank value (test.mjs after :1195).
- Finding SA-R2-2 (Low, pre-existing class): unwrap()'s option loop copies the array per word (quadratic); a very long line can make the guard run past the hook timeout (none set in settings.json; Claude Code's default applies, and a timed-out hook does not block). Measured 50,000 words: about 4 s plain (as on main), about 22 s when every re-inserted word is another split option. Fix (follow-up): walk an index instead of slicing, or set unreadable above a word cap.
- Nit: shownWord leaves Unicode line/paragraph separators and bidi controls as they are (U+2028/2029, U+202A-202E, U+2066-2069); optional.
- Full suite on head da7266e (hooks equal 11887d0): 1244/1244.
- C4 done: verdict APPROVE (SA-R1-2 closed; SA-R1-1, SA-R1-4 closed; SA-R1-3 deferred as stated; SA-R2-1, SA-R2-2 Low follow-ups). Scratch copies in .gate/guard-env-fix/ left (gitignored).
DONE
