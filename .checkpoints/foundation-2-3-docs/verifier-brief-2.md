# The verifier's re-run brief for PR #52: the verifier's fix pass (after your FAIL on a1d0317)

You failed a1d0317 on F-V1 (High) and F-V3 (Low). The session fixed both in one commit, on the advisor agent's ruling
(plan revision 2e in `plan.md`; its ruling in `advisor-plan-rev1.md`, "PLAN REVISION 2e"; E1 is in its exact revised
text). Your earlier brief (`verifier-brief.md`) still sets your limits. One exception: its item 4's last bullet is
withdrawn. It told you to accept a claim of two unrecorded chat updates, and that claim is gone.

## The new head
- **Head:** the commit named in the session's message to you (the verifier's fix pass).
- **Base for this re-run:** a1d0317b5d6d6bef920e394c51c07ca869efb601.
- **The notes as used:** named in that commit's message.
- **The appendix's cut-off:** 2026-11-14 00:43:01 UTC.
- **Advisor-tool calls before the head's commit:** 12, all Fable 5.2.

## What to check (narrow)
1. **The diff from a1d0317 to the head, word by word.** Every changed sentence must hold against its record:
   - the transcript;
   - git;
   - the CI API;
   - the advisor agent's ruling text (for E1, the exact text it gave).
2. **The gate and the eight scripts**, as before (gate `--no-smoke` against 4d21c1a; words, report-appendix `--check`,
   decisions `--check --negative`, pointers `--check`, fixes `--check`, quotes, scope, and 2.2's decisions-entries
   `--check` from `.gate/fdn23-docs/s22/`).
3. **New: every sentence in the report about what was said in chat, checked against a transcript line.** Also check
   every main-session chat text from 22:40 UTC (2026-11-13) to the head's commit, and count them. Say whether any of
   them bears on how Mustafa was told about the advisor model and is missing from the D&D bullet (which names lines
   1887, 2122, 2869 and 2929).
4. **F-V3's new bullet:** the files it names as certain copies hold whole sentences of his words or longer, in some
   pushed version (`fp3-fragments-all.mjs` lists the runs per file). Check that it states no count it cannot back.
   Shorter shared runs are said to exist and are not listed, by design.
5. **The cut-off:** say whether any message of his came after 2026-11-14 00:43:01 UTC.
6. **Still pending by design (not findings):**
   - `{{NOTES-SHA}}`;
   - STATUS's and ROADMAP's #52 state;
   - the Reviews line "Its re-run on the verifier's fix pass and the advisor agent's review of the final head: to come.";
   - E7's runs for a1d0317's successor;
   - #40's second read;
   - the PR body;
   - any consultation after 2026-11-14 00:34:39 UTC;
   - the `.checkpoints/` deletion.

## Same rules as before
- Read-only; nothing posted on GitHub.
- His words stay out of your notes: pointers, lengths and booleans only.
- Notes: append a block "RE-RUN on <head>" to `.checkpoints/foundation-2-3-docs/verifier-1.md`. Write one line after
  each check, and end the block with DONE.
- Report in verifier.md's format: `VERDICT: PASS | FAIL | PASS WITH NOTES`, problems with file:line and their record.
