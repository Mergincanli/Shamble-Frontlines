# Advisor review: docs-after-kit-v3-1, plan revision 1 (in progress)

Reviewer: general agent running main's `.claude/agents/advisor.md` (origin/main 868e165), plan-review section.

## Review plan
1. Read main's CLAUDE.md (protocol, reporting rule, Models and effort) and the plan.
2. Check A1/A2 against CLAUDE.md line ~130, advisor.md PR review heading and step 0, NIGHT_SHIFT.md line 8, the guard.
3. Check B against STATUS, ROADMAP, REPO_MAP.
4. Check C against ADR 0002 part 5 (~line 359) and OQs, ADR 0006 OQs, adr/README counts, SPIKE_PLAN §3, ENGINE_OVERVIEW 68.
5. Check D1 (kit-v3-1 report line 169), D2 (LEGACY_INVENTORY KT14).
6. Judge the five questions; write the verdict.

## Notes as I go
- CLAUDE.md:130 read; settings.json:3 effortLevel high; NIGHT_SHIFT.md:8 has (plus `date`).
- Guard: routine-guard.mjs:446 date; :459 node AND python on main's scripts; :458 PR Advisor runs no program;
  :594 GITHUB_READ = /^(get_|list_|search_)|_read$/ -> every actions_* tool (get and list) falls to :647 deny;
  :213-219 no rev-list; :453 echo/ls denied; :292/:376 PR Advisor writes only .checkpoints/.gate (Write tool).
- advisor.md:141-144 "Your notes ... write them with Bash" -> impossible in a routine (no echo); advisor agent has no Write.
- ROUTINE-EFFORT is an existing DECIDED code (STATUS:334, kit-v3-0 report:165, settings.test.mjs:57). Collision.
- ADR 0002: part 5 :359-365 "flips everywhere at once"; ALSO :1486-1489 failure-mode bullet "becomes production's
  default" and OQ-0002-4 option (b) :1023 -> both contradict PORTAL-EMBED unless pointed. ADR 0004 :212-213 defers to 0002.
- "portal" already = enclosure portal (ADR 0001/0004, GLOSSARY:66). New text must say "web portal".
- No frame-ancestors / X-Frame-Options anywhere in drafts: no contradiction.
- README counts: 0002 8/7 -> 9/8; 0006 22/17 -> 23/18; total 116/96 -> 118/98. Plan right.
- V2_PHASE0_BRIEF:103,108 "effort high", "Keep its triggers": superseded, Mustafa's brief, don't edit.
- "PR event starts from PR head": no source in the repo; plan cites none.
- infra-engineer frontmatter effort high -> needs xhigh for ADR 0006 text. Plan states no effort.
- "Guard test B1" enabled daily 09:41 Amsterdam -> next run 2026-11-05 09:41.
- #34 merge commit date 13:34:59 UTC.

- Coordinator update folded in (Mustafa 2026-11-04): B2 confirmed; un-pause claimed but API enabled false 12:58:29Z; kit-v3-2 item for read-only Actions tools.

---

# Verdict (word for word)

**Verdict: APPROVE, with the 16 conditions below.** The plan does what it says it will do, its scope is right, and the
counts and facts I checked are correct. Its wording is the weak spot: one decision code is already in use, two claims
say more than the evidence shows, and three places in ADR 0002 still contradict the PORTAL-EMBED rule after C2. Each fix
is exact and small, so a second plan round is not needed; I check them on the final diff. Mustafa's update of
2026-11-04 is folded in: conditions 4, 5, 15 and 16, "Decisions needed" and "For Mustafa".

Reviewed as a general agent following main's `.claude/agents/advisor.md` (868e165), plan-review section, against the
files in the worktree (main 868e165 + 381471d). Git, Read, Grep and Glob only; no gate run (no code yet).

## What I checked
- The counts in C4 are right: ADR 0002 goes from 8 numbered / 7 open to 9 / 8, ADR 0006 from 22 / 17 to 23 / 18, and
  the total from 116 / 96 to 118 / 98 (README lines 17, 21, 23; OQ headings at 0002:983–1068 and 0006:822–1137).
- The `date` change matches `.claude/NIGHT_SHIFT.md:8` and the guard (`routine-guard.mjs:446`). The guard refuses
  `actions_get` and `git rev-list` (`:647`, `:498`), as B2 showed. The `sk-` gap in B5 is real: `checkpoint.mjs:30`
  allows `%3D` before `sk-` and nothing else.
- The merge SHAs and times are right: #33 e4d8b22 (21:57 UTC on 11-03), #30 e2b5799 (10:21:55 UTC), #34 868e165 (the
  merge commit is dated 13:34:59 UTC; use GitHub's `merged_at` for the exact second).
- No draft blocks framing: nothing anywhere in the drafts says `frame-ancestors` or `X-Frame-Options`.
- CLAUDE.md's night window (07:30) matches NIGHT_SHIFT.md, so D2's figures are right.
- Mustafa's confirmation of B2 matches the facts the plan cites, so B1–B4 cover his "record parts A and B and #34's
  merge".

## Answers to the five questions
1. **Complete and correct?** Mostly. What is missing is in conditions 1, 6, 7, 9, 12 and 15 below.
2. **PORTAL-EMBED.** The OQ wording (C1, C3) rightly stays a question. C2 does not resolve its clash with "then the
   default flips everywhere at once": with C2 alone, the bullet says the isolated design takes over everywhere and also
   that a non-isolated path must stay. Two more passages in ADR 0002 say the same as "flips everywhere" and are not in
   the plan (condition 6). C2 also widens Mustafa's example (B3) to C2 and W3. That is a fair reading of his general
   words ("a design that needs cross-origin isolation"), but it is a reading, so it must be marked as one and its
   consequence for W3 recorded (condition 7). Keep C5 and C6: they carry his rule to the places a spike planner reads,
   and they decide nothing new. Each should cite OQ-0002-9 rather than restate the rule.
3. **A1 and A2.** Both claims go past the evidence.
   - "A command-line flag beats the project's effortLevel": B2's record shows `effort_level: max` although
     `.claude/settings.json:3` says high. The `--effort max` flag, though, was seen in a day session's launch log, not in
     the routine's.
   - "A pull-request event would start it from the PR's own head commit": nothing in the repository supports this, and
     the plan cites no source. B2 shows only that a Run-now start began on main (e2b5799).
   Conditions 2 and 3 fix both.
4. **Reviewers and evidence.** The reviewer set fits. What is missing: each agent's effort (condition 11) and a local
   gate run (condition 12).
5. **What the docs must record now:** see conditions 1, 4, 5, 9, 10 and 15.

## Conditions
1. **Rename the new effort decision.** ROUTINE-EFFORT is already a decided code: the Advisor's (b) of 2026-10-25
   (STATUS line 334; `docs/reports/2026-10-24-kit-v3-0.md` line 165; `settings.test.mjs:57` is named after it). Give the
   open question a new code, for example EFFORT-SOURCE, in the plan, A1, B6, the report and the questions to Mustafa.
   Next to STATUS's ROUTINE-EFFORT (b) entry, add one line: its premise, that a routine's main session runs at medium,
   no longer holds, since B2 ran at max. The (b) rule stays.
2. **A1, the effort line: say only what was seen, and name the sessions.** "This session" means nothing to a later
   reader. Wording along these lines: "Routines run at effort max: TEST-RUN B2's session record
   (session_0173vVo7uY6BfxLBnYjEBJLV) reads `effort_level: max`, although `.claude/settings.json` sets
   `effortLevel: "high"`; a cloud day session's launch log (name that session's id) shows the cloud harness starting
   Claude Code with `--effort max`, the likely cause." A general rule ("a command-line flag beats project settings")
   may stay only if it cites the Claude Code settings documentation's precedence list, and the verifier checks that
   source. Add, if the launch log shows it, that cloud day sessions start the same way.
3. **A1, the pull-request sentence: cite it or cut it back.** Keep the rule (Mustafa, 2026-11-04). For the reason,
   either cite a source the verifier can check, or write what is known. Sources: the Claude Code documentation on
   routine GitHub triggers, or the start commit in the session record of the old PR #26 event run (a cheap read with
   the session tools). What is known: "so every run starts on main's rules; a pull-request start could load the PR
   head's settings and hooks before step 0 switches to main." Also use one wording for how it starts across A1, A2, B2
   and B4. It has only a schedule trigger set up; it may also be started by hand or through the API ("Run now", as B2
   was); it is never started by pull-request events. Today A2 says "schedule or an API trigger" while B2 says "schedule
   trigger only".
4. **A1, the shell list must be exact or not claim to match the guard.** The guard also allows `python`/`python3` on
   main's tracked scripts (`routine-guard.mjs:446`, `:459`), and it lets the PR Advisor run no program at all (`:458`).
   Either say "(the PR Advisor runs none)" and drop "this matches the guard" from the evidence, or list python too. The
   python gap goes on kit-v3-2's list next to the Actions item (or on KIT-FOLLOWUPS' list, as Mustafa decides); this
   pass changes no guard.
5. **A2, three changes.**
   - The known limits name every `actions_*` tool, not only `actions_get`. The guard's read pattern
     (`routine-guard.mjs:594`, `^(get_|list_|search_)|_read$`) matches neither `actions_get` nor `actions_list`. Write it
     as "B2 met `actions_get` four times; the code refuses `actions_list` the same way". Per Mustafa's update, the
     sentence says the limit holds until kit-v3-2 decides whether the PR Advisor may use read-only Actions tools, with
     security-auditor's review. The plan's "Not in this pass" names kit-v3-2 for this limit, not KIT-FOLLOWUPS, and
     kit-v3-2's item covers every `actions_*` read, not only `actions_get`.
   - Fix "Your notes" (`advisor.md:141–144`, "write them with Bash"). In a routine, the shell has no `echo` or other
     file tool (B2: `echo` and `ls` refused; `:453`). The routine's session writes notes with the Write tool, which the
     guard allows under `.checkpoints/` and `.gate/` (`:292`). The advisor agent has no Write tool (frontmatter: Read,
     Grep, Glob, Bash), so as a subagent in a routine it returns its notes in its report. Record this as a known limit;
     giving the agent Write is a kit change (kit-v3-2 or KIT-FOLLOWUPS).
   - The heading states only what the routine's stored prompt says. Quote the prompt in B4. If the prompt sets no order,
     drop "oldest first". Write "every open PR that step 1 does not skip", not "every open PR".
6. **C2, make ADR 0002 say one thing in every place.** Besides the new sentence in part 5, change three passages:
   - part 5 (line 364), "then the default flips everywhere at once": qualify it, for example "everywhere the client
     runs isolated; inside a third-party iframe the non-isolated path stays available (PORTAL-EMBED, OQ-0002-9)";
   - the failure-mode bullet (lines 1486–1489): "the measured alternatives take over behind the isolation flag, which
     then becomes production's default";
   - OQ-0002-4 option (b) (line 1023).
   Each gets the same short qualifier and a pointer to OQ-0002-9. Say nothing about how the fallback is built (no
   build variants, no runtime detection): that would be designing it. engine-architect checks that the four passages
   agree.
7. **C1, mark the widening and record its one consequence; no options.** In OQ-0002-9, "Why it matters" says the rule
   is read as covering every design that needs isolation, B3, C2 and W3 (the advisor's reading, 2026-11-04, of
   "a design that needs cross-origin isolation"). It also says: for W3, the only non-isolated fallback is W2, the timer
   pacing whose failed jitter mark is what would bring in W3 (ADR 0004 part 5, PM-0004-68). If that happens, the
   question returns to Mustafa with the spike's numbers. "Options: none weighed" stays.
8. **Write "web portal", never a bare "portal".** "Portal" already means a door or window between enclosure areas in
   ADR 0001, ADR 0004 and GLOSSARY (line 66). Every new sentence says "web portal" (his words) or names the sites. For
   example, C3's "for the portal side" becomes "for the web-portal side".
9. **C3, add two related entries, with no claims.** OQ-0006-23's related list also names OQ-0006-3 (where guest ids
   are stored) and OQ-0006-15 (the domains). Add no statement about iframe storage or cookies: none is researched.
10. **B4, the "Guard test B1" routine has a deadline.** It is enabled and runs next on 2026-11-05 at 09:41 Amsterdam.
    STATUS and the report's "For Mustafa" ask him to disable or delete it before then. STATUS also notes, in one
    clause, that V2_PHASE0_BRIEF §3 step 2 ("effort high", "Keep its triggers") is outdated by the 2026-11-04 facts and
    ruling. Mustafa's brief itself is not edited.
11. **Model and effort per agent**, written into the plan (advisor.md's plan checklist, CLAUDE.md "Models and effort"):
    all Opus 5.5 (`model: opus`); engine-architect xhigh (its frontmatter); infra-engineer passed `effort: "xhigh"`,
    since it reviews ADR text; docs-keeper, security-auditor and the verifier at high (their frontmatter).
12. **Run the gate locally as well as in CI.** Run `npm ci` in `tools/` in this worktree, then
    `node tools/gate.mjs --no-smoke`, which must PASS with symbols +0 −0. Smoke comes from CI (the GATE-SETUP (a)
    precedent). My final-head review needs the gate anyway.
13. **D1:** say which run each time span belongs to (push run 41807980703, pull-request run 41807986424). Extend the
    existing correction note: "_Corrected 2026-11-04 (PR #34; PR #35: the second run's times)_".
14. **D2 is in, not optional.** It was carried twice (2026-10-28, and the #30 review's note 7). Date the note and cite
    NIGHT_SHIFT.md lines 2–3 and 16.
15. **B2 and B4 record what the routines API shows, not what is reported.** Mustafa says he un-paused the PR Advisor.
    The API, read at about 13:57 UTC (get_trigger, list_triggers), still shows trig_01YPyWAsQmsaQoydhMGG3CkJ as
    `enabled: false`, `updated_at` 2026-11-04T12:58:29Z. Re-read the API right before the final docs commit and quote
    that reading with its time. If it still disagrees, STATUS says: "Mustafa reports un-pausing it (2026-11-04); the
    routines API reads `enabled: false` at <time>: being checked". B2's Phase item 4 says the same. Nothing may say
    the routine is on until the API shows it.
16. **Don't use "A2" for two things.** The plan's A2 is the advisor.md edit; Mustafa's update uses "A2" for the
    kit-v3-2 item. In every doc this pass writes, call his item "kit-v3-2 (read-only Actions tools for the PR
    Advisor)", never "A2". Quote his words once in the report as its source: "the guard refused actions_get, a
    read-only GitHub Actions lookup the PR Advisor needs for CI details; consider allowing read-only Actions tools for
    pr-advisor, with security-auditor's review".

## Not conditions (notes)
- CLAUDE.md:81 ("the PR Advisor routine reviews it") is still true under a schedule: the review comes at the next run.
  No edit needed.
- REPO_MAP lines 76 and 110 ("high effort") still describe the file correctly. docs-keeper may add "(cloud sessions:
  max; CLAUDE.md)"; it is not required.
- The checkpoint's steps carry no SHAs yet; add 381471d to step 1.

## Decisions needed
- **KIT-FOLLOWUPS** (already with Mustafa) · Which change-set closes the PR #26 follow-ups: the URL-escaped `sk-` gap,
  and asking before kit and gate edits in day sessions? B2 found three more small kit items: `git rev-list` is refused,
  the docs and the guard disagree on python, and the advisor agent has no Write tool in a routine. The Actions read
  limit already goes to kit-v3-2 (Mustafa, 2026-11-04). · Why it matters: these are guard and kit changes, which this
  docs pass may only record, not make. · Options: (a) put them all in kit-v3-2, next to the Actions item, as one kit
  change-set after this pass; (b) a separate kit change-set for the PR #26 follow-ups. · Recommendation: (a). They are
  small, they touch the same files (`routine-guard.mjs`, `checkpoint.mjs`, `advisor.md`), and security-auditor reviews
  them once.
- **EFFORT-SOURCE** (today's ROUTINE-EFFORT, renamed by condition 1; already with Mustafa) · Should a day session look
  up whether any setting controls the effort a cloud routine runs at (today max, from the harness)? · Why it matters:
  CLAUDE.md should name the real control, not a setting that loses to the harness. · Options: (a) a short docs lookup
  in a day session, recorded in CLAUDE.md; (b) leave it at "routines run at max, as seen". · Recommendation: (a). It is
  cheap, and max costs nothing in quality.

## For Mustafa
1. Disable or delete the "Guard test B1" routine (trig_01XmLByUmTibCu5XXQR7iN26) before 2026-11-05 09:41 Amsterdam,
   its next run.
2. Check the PR Advisor routine's switch on its Routines page. You un-paused it, but at about 13:57 UTC the routines
   API still read `enabled: false` (last change 12:58:29 UTC). Expected: it shows on, with runs at 07:45, 13:45 and
   19:45 Amsterdam time. Once it is on, it will class PR #35 as needs-mustafa, since #35 changes `.claude/` and
   `CLAUDE.md`.
3. Answer KIT-FOLLOWUPS and EFFORT-SOURCE (above) when you like. This pass records them as open and does not wait on
   them.
4. Later: after the advisor approves PR #35's final head, say "merge #35" in this session's chat. No Brave check: no
   player sees, hears or feels this change.
