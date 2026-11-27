# Checkpoint: review-tracks
- **Goal:** the review tracks and who merges (his paste of 2026-11-25 18:25 Amsterdam, recorded whole in
  docs/DECISIONS.md by #73; working copy readable at 68b91211:.checkpoints/guard-wrappers/queued-review-tracks.md),
  plus SOURCE-GO (a) (his 2026-11-26 15:05 item 4: "a change under content/source/ needs my go"). Order: his
  SPEED-ORDER (b), so this comes before the guard change-set. Risk tier 2. Plan first; no rule file edited before his
  go on the plan. Plan answers his items 3a-g and 4a-d; shape per item 5 (old beside new, his narrowed words listed,
  security-auditor and the advisor agent on the plan, sized bottom-up, stop line twice the size, a smaller version
  beside it above 150 added lines).
- **Branch:** claude/confident-gates-tbnq38 (stands in for cs/review-tracks) · **Base:** main 2671595f (+ #77 open,
  tier 0, two docs files, for the PR Advisor's 07:45 run: T10's second half)
- **Last update:** 2026-11-27 morning Amsterdam

## Steps done
1. MAIN-WALL closed (#76 merged 2671595f; #77 ready for the PR Advisor) · no commit here

2. #77's head fixed after a stray checkpoint commit (4e8a87e8, tree equal to the approved 2136822c); the advisor agent
   re-approved; nothing is pushed to the lane branch until #77 merges (this checkpoint lives in .gate/ until then)
3. Input B (test-engineer): the full-track reviewer rule (security-first slot, then by changed lines; a script
   pick.mjs), checked on stops 3a and 3b: 13 Highs found, 0 missed; his effort words of 12 November quoted
   (DECISIONS:3160) · .gate/review-tracks/inputs-B.md

4. Input A: timings re-measured (his vs mine close; #68's split points differ; #69's cells sum above the branch), 17
   distinct needs-mustafa reasons, merge mechanics (guard: gate+smoke+expected head; routine allow-list; schedule
   07:45/13:45/19:45); three findings: decision-docs rule makes every change-set needs-mustafa, code PRs wait for his go,
   four newer rules missing from the kit's triggers · .gate/review-tracks/inputs-A.md
5. Plan drafted (Plan agent, no write tool: saved by script from its hand-back) · .gate/review-tracks/plan.md, 488
   lines: track.mjs (light/full, go floors, eyes paths, reviewer picks, words-record hash check), merge ways (1)/(2),
   "His go keeps" section, the 19-row table, 4a-d answers, his narrowed words, size ~290 (smaller ~150), ten decisions
6. security-auditor on the plan: REVISE: SA-RT-1 Critical (a forged merge-check comment plus a promptless merge: fix
   GUARD-MARK, the guard refuses the marker in any MCP write and refuses subagent merges), SA-RT-2 High (the after-check
   cannot catch a forgery; only his Advisor's own list can), SA-RT-3 High (the words hash proves wholeness, not that he
   typed it: his Advisor's check lists each entry), SA-RT-4 High (no blanket allow line: MERGE-PROMPT (c), the guard
   returns allow only for a qualifying way-(1) merge), Mediums SA-RT-5..11 (main's copy of track.mjs decides; wider
   full floors; CI-deciding files full; light security triggers; open-finding rule; eyes paths for feels; WAKE-END (a)),
   Low SA-RT-12; size ~415 vs stop 580 · .gate/review-tracks/security-auditor-plan.md
7. The advisor agent on the plan: REVISE: H1 (moved signals unchecked by the routine), H2 (forgery overclaim), H3 (words
   origin), H4 (which approval); M1-M10, L1-L5; no misquotes (14 OLD quotes, 13 DECISIONS lines checked); decisions for
   him cut to four (PLAN-GO, MERGE-MARK, MERGE-PROMPT, WAKE-END) · .gate/review-tracks/advisor-plan.md
8. Plan revised with both reviews folded in (670 lines; size ~510, stop ~1,020; smaller ~270, stop ~540; four decisions:
   PLAN-GO (a), MERGE-MARK (a), MERGE-PROMPT (c), WAKE-END narrowed (b) / default (a)) · revise-notes.md
9. security-auditor re-review: APPROVE (SA-RT-1..12 closed; departures accepted; WAKE-END narrowed (b) acceptable; four
   new Lows for the build: SA-RT2-1 table drift, SA-RT2-2 after-check exclusion, SA-RT2-3 refusal scope, SA-RT2-4
   legacy/ in the light security triggers)
10. The advisor agent's re-review: APPROVE with C1 (applied: PASS only when nothing in "His go keeps" applies, else
    FOR HIS GO) and C2 (met by security-auditor's re-review), C3/C4 at build; three decisions for him
11. Plan sent to him for PLAN-GO, MERGE-PROMPT, WAKE-END (plan: .gate/review-tracks/plan.md, 679 lines)

12. Mustafa (2026-11-26, in this chat): "PLAN-GO: a / MERGE-PROMPT: A / WAKE-END: a": the full version; MERGE-PROMPT (a)
    = the allow-list line for every merge (the option both reviewers advised against: SA-RT-4 High reopens); WAKE-END (a).
    Asked him once to confirm MERGE-PROMPT (a) against (c), since the options were listed (c) first.

13. He asked what a guard load failure means; explained (with (a) a broken guard means an unchecked merge; GitHub's
    ruleset still requires a PR and green gate and smoke). He proposed keeping the four checks without the file check;
    offered as (d). Mustafa (2026-11-26, in this chat): "d": MERGE-PROMPT (d): the guard skips the prompt when the
    approved version, gate+smoke+toolchain green, not a subagent and a real unedited PASS comment hold; no file-list
    check; his Advisor's PASS covers his go where his rules need it (his Brave check or yes first); a broken guard ends
    in the prompt.

14. Plan updated for his answers (705 lines; size ~490, stop ~980; about 10 asks at build: one Edit per file, two for
    the guard and its tests; build in a plain copy, not a worktree). The updating agent broke the interim rule twice
    (read-only git after cd/sed in one command): reported to him. Both reviewers take a quick look at (d) (running).

15. security-auditor's quick look at (d): APPROVE, ready to build; SA-RT3-1 Medium for the build (the guard's gh()
    refuses plain gh comment/review/edit commands carrying the PASS marker, including from a file); SA-RT2-1's other
    half (guard file equals main's) a build Low. The advisor agent's quick look: running.

16. The advisor agent's quick look at (d): records his decision correctly, ready to build after #77 merges; Q1 met
    (security-auditor's quick look APPROVE), Q2 applied in the plan (advisor.md:205-208 OLD/NEW), Q3 done: his four
    messages and the offered option text saved by script in .gate/review-tracks/ (to copy into .checkpoints/review-tracks/
    once the branch may move); Q4 (deny-list) folded into SA-RT3-1's guard refusal for the build.

17. #77 merged by the PR Advisor at 2026-11-26 18:52:18 UTC (its 19:45 run; merge commit 47bb79e9): T10's PR Advisor
    half, so both halves of T10 are done and his TOKEN-RELEASE (c) condition is met. Lane branch fast-forwarded to main
    b9aad019 (#75 merged since); notes moved from .gate/review-tracks/ into .checkpoints/review-tracks/.

18. Build copy .gate/review-tracks/build/ (plain copy of .claude/ and CLAUDE.md.new): track.mjs (160 lines) and
    track.test.mjs written; 61/61 pass in the copy

19. Guard built in the copy: GUARD-MARK (mcp() for every GitHub write, comment edits, gh() plain forms and body files),
    subagent merges refused, toolchain in the merge check, the MERGE-PROMPT (d) allow (passAllows), guard.mjs's allow
    output; tests: fixture (toolchain rides every list unless named, opted out or counted) and the new cases plus a
    mutation run of the 14 new rules and guard.mjs's day-only allow: routine-guard 1575/1575, track 61/61, checkpoint
    33/33, kept-logs 9/9; settings 126/127 in the copy, the same failure in an untouched copy (the copy's location)

20. Rule text built in the copy: CLAUDE.md.new (+39/-13), advisor.md (+76/-34: category 5, the classes, "His go keeps",
    plan-review line, evidence rule, step 2/4/5/6 changes, after-check, how the check counts, Q2, kit tests),
    security-auditor.md (+2/-1). Size against main: 585 added (guard 37, its tests 151, guard.mjs 4, advisor 76, sa 2,
    CLAUDE 39, track.mjs 160, its tests 116); stop line about 980. Suites in the copy: 1575, 61, 33, 9.

## Current step
Pre-apply review of the copy (security-auditor, test-engineer, the advisor agent), so fixes cost no extra asks; then
the applies (about 10 asks), cmp, suites, gate, one commit. Then: fast-forward the lane branch to main, move the notes from
.gate/review-tracks/ into .checkpoints/review-tracks/ (plan, reviews, inputs, saved messages; no exploit forms), push the
checkpoint, and build in a plain copy (not a worktree), one Edit per kit file asked of him (about 10).
Inputs in parallel: (A) his timings of #65, #68, #69 measured from commit and event times, and every needs-mustafa
reason in the rule files with file:line; (B) CORE-REVIEWERS: the Highs of stops 3a (#65) and 3b (#68), who found each,
and a rule for the at-most-two extra reviewers. Notes: .checkpoints/review-tracks/.

## Next steps
1. Plan (.checkpoints/review-tracks/plan.md): the track script (files touched -> light or full; in doubt full), LIGHT-DEF,
   the five full-track exceptions, CORE-REVIEWERS, who merges, EYES-KEEP, the needs-mustafa table, SOURCE-GO, answers 4a-d,
   old beside new, size. Also listed as candidates: the CLAUDE.md accounts lines (his three accounts; needs his words).
2. security-auditor and the advisor agent on the plan; then his go.

## Decisions taken
- SPEED-ORDER (b): review tracks before the guard change-set · Mustafa · 2026-11-26
- SOURCE-GO (a) rides with the review tracks · Mustafa · 2026-11-26 15:05

## Running work
- none
