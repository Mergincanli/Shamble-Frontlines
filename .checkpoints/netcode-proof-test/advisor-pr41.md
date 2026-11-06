Task: advisor PR review of #41 (claude/npt-readings, R-PR of the netcode proof test), head 2ccc184b55f62430ece8f47afcfe8c058816f863, base af8393e9d40f3bd382e77f86e6df2612785c706c. Day session, verdict only.

## Plan (blocks)
B1. Read advisor-verdict.md (R-PR bullets, exact words, Mustafa answers as recorded).
B2. Read the diff (stat + word diff) of the 5 files; file list check (no .claude/CLAUDE.md/.github/gate files).
B3. Check each R-PR bullet at its anchor, exact words (scripted compare).
B4. Check nothing else changed / no mark moved; pointers on overtaken lines accurate, add no rule.
B5. Mustafa's answers quoted exactly; rule on the "applies it" sentence.
B6. CI check runs on head (GitHub tools); PR state; no .checkpoints in PR.
B7. Verdict written.

## Notes
- full head SHA 2ccc184b55f62430ece8f47afcfe8c058816f863; merge-base = af8393e (main at PR open). 5 files, +96 -34.
- B1 DONE: verdict has 15 R-PR bullets (R-1/18 ports 0006; R-5 part 13; R-6/9 PM-0001-3; R-7 PM-0001-28; R-10 OQ-0001-9 cover; R-12 bites rows 0001/0006/0004; R-13 SPIKE_PLAN:53-55; R-14 SPIKE_PLAN item 1; R-15 PM-0004-67; R-16 PM-0001-59; R-17 PM-0001-23/-24; R-25 0001:1444-1445; R-26 0006:1314-1316; R-28 checklist; FIXED-IP SPIKE_PLAN:64-66,173 with <date>). Watch: R-12 "digest F proposals, each written into its own row" (is it just placed text?); FIXED-IP <date> should become 2026-11-06. Next B2.
