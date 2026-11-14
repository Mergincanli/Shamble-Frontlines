TASK: advisor confirmation of the final plan SHA 94c5c5a5aa4022a65476009001ceebbf7dd15f82 (diff 00984ae..94c5c5a: revision 2.1 folds L-1..L-6; revision 2.2 adds the old plan's Appendix B rules), day session, read only.

PLAN (blocks):
D1. Diff stat; confirm only plan.md and kit-texts.py (and checkpoint files) changed; read the diff.
D2. Check L-1..L-6 folded.
D3. Check revision 2.2 against the old plan's Appendix B (26e0d82 plan.md:636-655) and PR #40's facts; PM-0002-42 unchanged?
D4. K-4 How line; Part 4 row 108; kit-texts.md regenerated?
D5. Verdict; DONE.

NOTES:
- D1 part 1 done (diff lines 1-150). Scope: only .checkpoints/ changed vs main; working tree equals 94c5c5a. Folded so far: L-1 (D1, -52 later of slice 5 and PM-0003-36), L-2 (CS-E replacement jobs), L-3 (archive only needed paths, never .claude/ or CLAUDE.md; no prompt expected), L-6 (each v2 job; legacy jobs stay as moved). Revision 2.2 so far: Appendix B rule block (Part 3), CS-C1/CS-D/CS-C/CS-E evidence lines, PM-0002-42 spike variant "after PR #40 merges, when Appendix B lifts", Part 4 row 108, #40 head 30b7b83. To check: old plan Appendix B text (when it lifts), pages.md:3-4, CLAUDE.md:46, K-4 How line, L-4, L-5.
- D1-D4 done: L-1..L-6 folded; revision 2.2 checked against 26e0d82 :636-655; #40 check.mjs at 30b7b83 not local (not checked); main has no non-Markdown spikes/ text outside docs/.checkpoints/spikes; Appendix K identical (30 items, 19 quotes); Part 4 108 = 50/49/5/2/2. Lows R3-1 (spike-path wording narrower than old Appendix B and reading.md:113-114) and R3-2 (spike variant trigger; legacy plant meets the or-clause). Verdict APPROVE 94c5c5a.
- D5: final answer, handed back unchanged:
**APPROVE 94c5c5a5aa4022a65476009001ceebbf7dd15f82.** There are two Lows; neither blocks. Fold them in without a new review, or carry them into CS-C's plan.

**What I checked:**
- Only `.checkpoints/` differs from main, and the working tree equals 94c5c5a.
- Appendix K is identical to `kit-texts.md` (30 items, 19 quotes of his).
- Part 4 recounts to 108 rows: 50 first part, 49 later, 5 dropped, 2 not ours, 2 chat.
- L-1 to L-6 are all folded in as asked.
- The Appendix B items on `.cargo`, `"type"` and the rule globs match 26e0d82 :636-655.
- Row 108's citations hold (`pages.md:3-4`, CLAUDE.md:46).
- K-4's new Opus line restates CLAUDE.md's existing rule and loosens nothing.

**PM-0002-42 is unchanged.** Its text is untouched, and the planted import from `legacy/` meets its clause "an import from `legacy/` or `spikes/`".

**Lows**
- **R3-1.** plan.md:232-234 and :777-778 say #40's check fails on "the text of the spike folder's path". The old Appendix B (26e0d82 :637) and your own reading.md:113-114 say any text `spikes/`. Fix: use `spikes/`, the stricter wording. No non-Markdown file outside `docs/`, `.checkpoints/` and `spikes/` contains it today, so the change costs nothing.
- **R3-2.** plan.md:194 and :350 have the spike-folder variant join "after PR #40 merges, when Appendix B lifts". But once #40 merges, its spike sits in main's tree (ADR 0002 part 1 keeps `spikes/`), so Appendix B need not lift at that point. Fix: call the variant an extra that joins when Appendix B lifts, not a clause that holds PM-0002-42's met-at (slice 5).

**Not checked:** #40's `check.mjs` at 30b7b83, which is not in this clone.
DONE

- Advisor tool consulted before sending: PM-0002-42 confirmed on what holds either way (text untouched, no plant dropped, variant moved); read the kit-texts.py hunk (K-5 gone, How line, K-4 (a) [to verify]); plan diff is 235 lines, all read; read #40 check.mjs at 4bd5ac3 by git show (referenceProblems :1789-1801 fails on any "spikes/" text; :1786 says its rule holds until the foundation G5 and G8 exist); added R3-3 (old Appendix B :638 no exclude line).
- D5 (redone): final answer v2 (254 words), handed back character for character; supersedes the block above:
**APPROVE 94c5c5a5aa4022a65476009001ceebbf7dd15f82.** There are three Lows, and none blocks. You can fold them in without a further review, or carry them into CS-C's plan.

**What I checked:**
- Only `.checkpoints/` differs from main.
- Appendix K is identical to `kit-texts.md`.
- Part 4 recounts to 108 rows: 50 first part, 49 later, 5 dropped, 2 not ours, 2 chat.
- L-1 to L-6 are all folded in.
- Row 108's citations hold.
- K-4's new Opus line loosens nothing.

**PM-0002-42 is unchanged.** Its text is untouched, no plant is dropped, and only the spike-folder variant moves to a later trigger.

**Lows**
- **R3-1.** plan.md:232-234 and :777-778 say #40's check fails on "the text of the spike folder's path". In fact it fails on any `spikes/` text (#40's `check.mjs` :1799, read at 4bd5ac3). The old Appendix B (26e0d82 :637) and reading.md:113-114 agree. Fix: write "the text `spikes/`".
- **R3-2.** plan.md:194 and :350 let the variant join "after PR #40 merges, when Appendix B lifts". That is not a real event: after the merge, the spike folder and its check sit in main's tree (ADR 0002 :141), and #40's own check (:1786) holds its rule "until the foundation's G5 and G8 exist". Fix: name the event, and make :194's met-at "the later of slice 5's change-set and that".
- **R3-3.** CS-C's `exclude` (:352) departs from the old plan's "no `exclude` line" (26e0d82 :638). Since it names no `spikes/` text, the rule's purpose holds: say so.

**Not checked:** #40's head 30b7b83, which is not in this clone.
DONE
