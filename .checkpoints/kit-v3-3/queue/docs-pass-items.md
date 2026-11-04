# Docs pass: items collected so far (2026-11-04)

The docs pass is a rules change-set (it edits CLAUDE.md), plan first, and needs Mustafa's go. It opens only after part B2
has run, because #34 must be the only ready PR.

- **CLAUDE.md:** the routine shell list names `date`, matching the guard (Mustafa).
- **The trigger rule:** the PR Advisor runs only on a schedule or API trigger, never on PR events (Mustafa,
  PR-HOME-REPLACE (a)). Its trigger is `CRON_TZ=Europe/Amsterdam 45 7,13,19 * * *`, with no setup script. Its prompt is
  "every open pull request… stay on the default branch". Its connectors are removed.
- **STATUS and ROADMAP:**
  - #33 merged (e4d8b22); #30 merged (e2b5799, 2026-11-04 10:21 UTC).
  - The advisor's nit: "a draft while A1 runs".
  - The optional KT14 nit in LEGACY_INVENTORY.
- **Routines:**
  - Both role environments exist.
  - Part A passed.
  - B1 passed (Mustafa, 2026-11-04 13:20, second try; the first had the typo `pr-adviso`, an unknown role, refused as
    designed). Rows checked from the transcript, session_0173vVo6XGr8xapun6WUNTpM:
    - starts on main;
    - the 7 reads and notes ran;
    - the 7 refusals had exactly the expected messages;
    - the PR Advisor can read CI's gate job log.
  - The old PR Advisor session from the PR #26 event (stuck since 2026-10-25) stopped at step 0 with the role unset,
    changing nothing. Mustafa archived it.
  - B2 passed (Mustafa's "Run now", 2026-11-04 13:26:48–13:35:54 UTC, session_0173vVo7uY6BfxLBnYjEBJLV, 57 turns,
    $3.36, effort max). Rows checked from the transcript:
    - starts on main: the first `git log` showed HEAD e2b5799 (main's tip then) on the harness's own branch
      `claude/quirky-bohr-k7scly`, clean tree; it then ran `git switch --detach origin/main` (same commit). That
      branch exists only in the routine's container, not on GitHub;
    - the role: every guard refusal names `routine (pr-advisor)`, and it went past step 0;
    - it read #34 only through git (the branch as a remote-tracking ref) and the GitHub tools, and ran no programs;
    - CI: the 4 checks and the gate job logs; the guard refused `actions_get` (run and job details) four times;
    - acted on its own: review comment 6632017016 naming the full head a432692…, label `ready-to-merge`, re-checked the
      head and the checks at 13:34:46, merged at 13:35:00 as 868e165 with `expectedHeadSha` a432692…;
    - no permission prompt: the only control requests are the mode (auto), initialize (no pending permission
      requests) and context-usage ones; no permission request or response event; all 7 of the run's
      `permission_denials` are guard refusals (`echo`, `ls`, 4 × `actions_get`, `git rev-list`).
  - Possible guard follow-ups seen in B2 (not decided): `actions_get` is not on the PR Advisor's GitHub list, so it
    cannot read CI run start times; `git rev-list` is not on the routines' git list. A GitHub tool error, not the
    guard: reading labels by number fails for a PR ("Could not resolve to an Issue with the number of 34"); labelling
    still worked.
  - B2's own nit: the corrected sentence in the kit-v3-1 report gives only one CI run's times (09:30:25–41), though
    it cites both runs (the other: 09:30:23–38).
- **PROBE-FILE (c), known noise:**
  - The Stop hook is Claude Code on the web's own: the harness's launcher settings register
    `~/.claude/stop-hook-git-check.sh`, written by the environment manager at session start. The repository defines no
    Stop hook.
  - It asks a session to commit and push when the checkout has uncommitted changes, untracked files that are not
    ignored, or unpushed or unsigned commits.
  - The PR Advisor's notes under `.checkpoints/` set it off at the end of every run. The guard refuses the push, and the
    hook fires once per stop (`stop_hook_active`).
  - `.gate/` is already gitignored.
- **ROUTINE-EFFORT, observed:** routines run at effort max, not high.
  - The harness starts Claude Code with `--effort max`, and a command-line flag beats the project's `effortLevel`.
  - Evidence: B2's session record (`effort_level: max`) and this container's launch log (`--effort max` on every
    start).
- **The PR #26 follow-ups:**
  - **Kit and gate write limits:** done for routines by #30 (protectedRel). There is no ask-first for day sessions.
  - **The URL-escaped secret gap:** open. On main, the generic `sk-` pattern in `checkpoint.mjs` misses `%20`, `%2C`
    and `%3A` before `sk-`; only `%3D` is handled. Anthropic `sk-ant-` keys are caught. GitHub push protection is on
    (Mustafa).
  - Whichever change-set Mustafa picks will close them.
- **Mustafa:** the "Guard test B1" routine is scheduled daily (09:41); disable or delete it.
- **PORTAL-EMBED (Mustafa, 2026-11-04; an open question, keep the door open, don't design it):** the game may also
  launch on web portals such as CrazyGames and Poki, inside their iframe, sharing our servers and player pool. Add it
  as an open question to the ADR index and register (`docs/adr/README.md`, my reading of "the README": it holds the
  open-question register and counts) and to ADR 0002 and ADR 0006. The rule it sets: the client must keep working
  inside a third-party iframe, so the boundary spike may pick a design that needs cross-origin isolation (the
  SharedArrayBuffer ring, B3: ENGINE_OVERVIEW line 68, SPIKE_PLAN line 253) only if a non-isolated fallback stays
  available. Also touch: the register's open-question total (README line 23), SPIKE_PLAN's boundary spike pass rule,
  and the glossary's isolation-flag entry (GLOSSARY lines 637–655) if their wording would contradict it. Check
  whether ADR 0006's hosting and header rules (COOP/COEP only with the isolation flag) need a note, without deciding.

## For the NEXT docs pass (after PR #35 merges), from the advisor's final-head APPROVE of 76c35bf (2026-11-04)
1. Minor: STATUS calls PR #35 a "draft" (STATUS:3, :19, :83): it is ready, then "merged" once it lands.
   Taken by B3 (PR #36).
2. Minor: ROADMAP's "Next / open" queue (ROADMAP:70) leaves out A4 (the review policy); STATUS has it.
   Taken by B3 (PR #36).
3. Nit: "about 14:02" (STATUS:44, :400): the first API read after the un-pause was 14:03:10 UTC.
   Taken by B3 (PR #36).
4. Nit: the PR body was an older copy of the report: refreshed without a push (done 2026-11-04).
5. Nit: "B3" means three things (Mustafa's list item; the SharedArrayBuffer ring; ADR 0006 part 13's CSP row);
   OQ-0006-23 uses two in one entry, and the report's "Proposed next step: B3" has no qualifier.
   STATUS and ROADMAP part: taken by B3 (PR #36). Still open: README row 86 and ADR 0006's OQ-0006-23 (both inside
   his quoted words) and the merged #35 report, for the drafts' next revision.
6. Nit: one uneven wrap at STATUS:230 (137 characters).
   Taken by B3 (PR #36).
7. Note: CLAUDE.md:141 "seen in TEST-RUN B1 and B2": B1 was the separate "Guard test B1" routine on the PR Advisor
   environment, not the PR Advisor routine; still true of how a start began on main. No change needed.
Also for it: the night shift's first guarded night (2026-11-05 from 00:10) and the PR Advisor's first scheduled run
(18:45 UTC) are the next proof of the routine rules; record what they show.
- Routine proof, scheduled PR Advisor start: its first scheduled run (2026-11-04 18:45:37 UTC, session
  cse_0173vVo8Ju2Jb2eN5ZDzJdtK) FAILED with ROUTINE_RUN_FAILURE_REASON_USAGE_LIMIT_REACHED (finished 18:45:44), the
  same org usage limit that cut B3's fix round (reset 18:50 UTC). Next scheduled run: 2026-11-05 06:45 UTC (07:45
  Amsterdam). Routines share the account's usage limits with day sessions. (Read with get_trigger, 2026-11-04 ~18:55 UTC.)

## For the next docs pass, from the advisor's final-head APPROVE of PR #36 (a32e4c0, 2026-11-04 ~19:20 UTC)
1. Low: the B3 report's "Changes to his words" (report:56-57) leaves out plan rev 1 §3's approved deviations: a
   capital first letter (LOOP_V2:147 the war line; :169 the bot line), labels before older quoted lines ("Name:",
   "Levelling:"), and the splits/repeats (item 11's first sentence alone in §5; item 4's bot half again in §10). Add
   one line (they are recorded at c5d86cb, plan-rev1-folded.md:88-91).
   Not taken by A2 (PR #37): a report edit, left for a later pass; noted in STATUS (Follow-ups).
2. Nit: the B3 report body is 83 lines (skill: ~60); "fixed in the last commit" (report:37) means c5d86cb.
   Not taken by A2 (PR #37): a report edit, with item 1; noted in STATUS (Follow-ups).
3. Nit: STATUS (:3, :21, :87) and ROADMAP (:91) call PR #36 a "draft": it is ready, then merged.
   Taken by A2 (PR #37).
4. Note: #36 merges with a merge commit (not squash), so the branch's step commits stay reachable from main.
   Taken by A2 (PR #37): recorded in STATUS (In flight, B3's row) and ROADMAP (B3's entry).
5. Environment note: local smoke needs Playwright Chromium build 1194 (Playwright 1.56.1); this container has 1243 in
   /opt/pw-browsers. The advisor installed 1194 into the scratchpad (PLAYWRIGHT_BROWSERS_PATH) to run the full gate.
   A candidate for the environment's setup, or tools/README's "Check a new machine".

## For a later pass, from A2 (PR #37, final head 342ff49, 2026-11-04 ~22:25 UTC)
1. E9 left untested: the "first line names another task" case (notes moved into old/) and worktree agents
   (STATUS Open bugs and risks, kit-v3-2 items 2-4). A small dry run in a later kit pass could close them.
2. Workflows: this container's workflow runner allows CPUs - 2 agents at once (4 CPUs here, so 2); E9 text E's waves
   of 3 and 2 never ran 3 together. CHECKPOINTS.md §7's cap-in-workflows line could say a script must still wave its
   calls, since a bigger machine's runner allows up to 16. Candidate for A4 or a later kit pass.
3. The toy-box tone outside A2's files (README.md:4, GAME_DESIGN.md:14, :17, :28, :50-51, ROADMAP.md:10) is recorded
   in STATUS (KIT-ALIGN); a later docs pass rewords it against LOOK_V2.
4. REPO_MAP's CHECKPOINTS.md row was fixed in A2's last commit (restart-trust rule; "checked while agents run,
   restarted if it has stopped").
5. From the advisor's final-head APPROVE of PR #37 (342ff49, 2026-11-04 ~22:30 UTC; comment 6643739289), deferred:
   - F-1 (for the next kit change-set, A3 or later): CHECKPOINTS.md §4's trust rule does not cover a lost container
     where the notes survive but an uncommitted output outside .checkpoints/ does not. Add "if a later block finds a
     done block's output missing, note it and redo that block"; record it in STATUS under kit-v3-2's risks.
   - R-1 (nit): the kit-v3-2 report is 69 lines (skill: ~60). R-2 (nit): its Risks omit "nothing enforces the cap or
     the scratch rule" and the notes-folder script prompt (STATUS items 1 and 5 carry both). A later report edit.
