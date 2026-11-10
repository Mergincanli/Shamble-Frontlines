Task: fold check Lens A (conditions 1 and 5, Mustafa quotations) on plan revision 2 (b6c20be) of foundation-toolchain.
Plan: B1 identity check b6c20be vs working copy; B2 read advisor rev1 conditions; B3 read rev2 plan K2/FDN-NOCODE parts; B4 sources (d389d91 SCOPE, 828a3e0 files, STATUS 680-710, session file); B5 quote checks; B6 verdicts.
B1 done: b6c20be plan == working copy (diff empty). Next B2.
B2 done: advisor conditions 1-6 and F-1/F-2 read (advisor-plan-rev1.md:53,57); session words file read. Next B3.
B3 done: rev2 plan read (In one screen 17-45, 1.2 58-73, 1.3 75-85, Part 5 280-309, Part 8 362-391, Part 9 393-421, Part 11 442-445, Decisions 447-464). K2a/K2b used consistently in 36-37, 81, 287-288, 305-307, 376, 396, 444; 404 says "Before K2 is written" (quoting condition 5). Next B4 sources.
- note: Decisions entry says "Needed before step 5 only" - "step 5" not explained inside entry (self-containment, plan:462).
- note: entry's "SCOPE option (a)" code not explained in plain words (plan:456).
B4a done: CLAUDE.md on 2d22b69 == working copy; :20-22 (No engine code...stop.), :45 heading, :52 'Phase 0 writes no v2 code.', advisor.md:107 and :323 all match plan quotes (plan:289-290, 78, 80-81). Next B4b d389d91.
B4b done: d389d91 plan.md:531-534 SCOPE (a) clause '... and 2.2's kit edit scopes the constraints and adds your 2026-11-10 decision as the exception to the "no v2 code" lines' matches plan:457-458 (wrap only); FREEZE-FDN (a) at d389d91:512-514 'for 2.2, its CI job, the constraint-scope lines and the rules' globs'. Next B4c 828a3e0.
B4c done: 828a3e0 words: decision line 7 matches plan:59-60; answers SCOPE line 13 matches plan:61-63; follow-up item 3 matches plan:18-19; his SCOPE reply does not repeat the 2.2 kit-edit clause (C1b ok). His chat FREEZE-FDN answer (answers:9) has no parenthesis list; plan quotes STATUS text instead (check B4d).
B4d done: STATUS 2d22b69:686-689 FREEZE-FDN text matches plan:64-67 and Decisions plan:454-456 (C1f part 2 ok). LANE-RULES STATUS:709-710 "it is an extra. The recorded plan review by the advisor agent stays required." matches plan:72 quote.
- finding (Low): plan:71 unquoted "Fable 5.2 may advise through the advisor tool" placed under STATUS:700-710 in a "word for word" section; STATUS:704 reads "may be the session's advisor through Claude Code's advisor tool"; the plan's wording is from his decision (828a3e0 mustafa-2026-11-10.md:7). Mis-sourced paraphrase.
Next B4e: 828a3e0 restructure plan Part 5 FREEZE-FDN option text; SPIKE_PLAN fast checks on save; grep other quotes.
B4e done: FREEZE-FDN option (a) identical at 828a3e0 restructure plan:515-524 and d389d91:509-518; 'for 2.2, its CI job, the constraint-scope lines and the rules' globs' matches plan:453-454 (C1f part 1 ok). SPIKE_PLAN 2d22b69:438-439 'fast checks on save' matches plan:295. Next B5 grep remaining quotes.
B5 done: no further Mustafa quotes in plan:140-360 (Part 6 'his words on main do not name' is not a quote). Next B6 diff rev1 vs rev2 for lens areas.
B6 done: rev1 (f535752) K2 = CLAUDE.md:45 + :51-53 covered; FDN-NOCODE two lines; rev2 splits correctly; drafts unchanged in substance (rev1 plan:276-279 vs rev2 305-309).
- finding (Low): rev2 Decisions plan:451-452 says "those three lines still say "no engine code"" and In one screen plan:37 "the three "no engine code" lines"; CLAUDE.md:52 says "Phase 0 writes no v2 code." (no "engine code"); rev1's "both lines still say "No engine code"" was true; the split made it false for one line. Also lowercase vs "No engine code" (CLAUDE.md:21, advisor.md:107).
- finding (Low): plan:404 Part 9 still says "Before K2 is written" (unsplit residual; restates condition 5).
- finding (Low, C1e): entry plan:462 "Needed before step 5 only" - step 5 not explained in entry; plan:456 "SCOPE option (a)" code not explained in plain words.
- finding (Low): Decisions needed (plan:447) is not followed by a "For Mustafa" section; Part 11 (442) precedes it, appendices follow (same in rev1, not a regression; plan, not a report).
- finding (Low): step 5 (plan:374-377) gives no branch if the ADR 0002 Status check fails (stop/ask unstated); "only then" implies wait.
- note: plan:5-6 advisor's sentence "Condition 1 must go into the plan before FDN-NOCODE is sent to Mustafa." not in advisor-plan-rev1.md (unverifiable from notes; advisor words, not Mustafa's).
B7 verdicts: C1a holds (Low residual 404); C1b holds; C1c holds; C1d holds; C1e fails (Low); C1f holds; C5 holds; Q holds for byte-for-byte quotes, Low mis-sourced paraphrase plan:71.
DONE
