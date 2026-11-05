Task: advisor review of A3 (kit-v3-3) plan revision 1 (plan-rev1.md), per main's advisor.md.
Plan (blocks):
1. Read main's advisor.md (role, checks, verdict format).
2. Read plan-rev1.md.
3. Read critic.md, reviser.md, readers' notes (skim for gaps).
4. Verify key claims against worktree files (guard, checkpoint.mjs secret patterns, settings, routines denies).
5. Write verdict + conditions + decisions.
Notes:
- block 1 done: advisor.md read (main 12d7961). A3 touches .claude/ = escalation cat 3 -> PR needs-mustafa; plan may proceed on his explicit asks. Next: block 2.
- block 2 done: plan-rev1.md read (595 lines; 12 decisions; §4 edits; §5 ~90 tests; E1-E11).
- finding P1: F1-FIT (a) fixes F-1 in CHECKPOINTS.md; item 5 asks only to RECORD the deferred points -> exceeds; take (b).
- finding P2: READ-SCOPE (a) scopes all routine reads (get_job_logs, gh -R) -> hardening not asked; take (b) + follow-up.
- finding P3: SECRET-DECODE B (pctDecode re-test, env on decoded, @ URL) exceeds item 2; take A (one ESC fragment).
- finding P4: AF-COVER (b) script-text check is heuristic + noisy; exotic writers, ~/.claude exceed "seatbelt"; trim.
- finding P5: WT-NIGHT is an exception to his absolute words -> Mustafa's; others are technical -> advisor decides.
- Next: block 3 (critic, reviser, queue file for "three deferred points"), block 4 (code checks).
- block 3 done: critic (23 findings), reviser (all upheld), queue/docs-pass-items.md:123-127 = F-1, R-1, R-2 deferred
  in the PR #37 final-head APPROVE (~22:30 UTC), just before his addition (~22:45) -> plan's reading CONFIRMED.
- block 4 done (code checks at worktree, main 12d7961):
  routine-guard.mjs:268-272 Bash and mcp dispatch before `if (!role) return null` -> day Edit ask hooks at :272 as planned.
  :233-236 gitArgsCheck refuses abbreviations of known/danger long options -> '--output' in GIT_LONG_DANGER also stops --outp.
  :184-197 log/diff/show have no '--output' entry today -> routine write hole real (settings.json:175-177 deny only day full spelling).
  :594 GITHUB_READ does not match actions_* -> refused today; :603 returns null for reads before the :604 repo check.
  actions_get enum (ToolSearch schema): get_workflow, get_workflow_run, get_workflow_job, download_workflow_run_artifact,
  get_workflow_run_usage, get_workflow_run_logs_url; actions_list: list_workflows, list_workflow_runs, list_workflow_jobs,
  list_workflow_run_artifacts -> plan's ACTIONS_READ = read-only subset, matches.
  checkpoint.mjs:30 API key boundary `(?:^|[^A-Za-z0-9-]|%3[Dd])` -> %20/%2C/%3A miss confirmed; :109-110 patterns on raw blob.
  guard.mjs:23-34 print() deny shapes only; :39 `if (d) print(d.deny, d.interrupt)`.
  settings.test.mjs:56 ask list must be [] -> AF-MECH (b) keeps it.
  settings.json:270 bootstrap imports ${CLAUDE_PROJECT_DIR}/.claude/hooks/guard.mjs each call -> hook code read per call.
  docs-keeper.md:24-25 CLAUDE.md outside its scope; no agent file edits kit files -> AF-AGENT (b) breaks nothing.
  STATUS.md:277 "Follow-ups" is a bullet under Decisions pending (:136) -> QUEUE-HOME placement condition.
- finding P6: item 4 words name switching (branch, PR head, old commit); patch-applying commands (cherry-pick, am, apply,
  revert, read-tree -u, stash) are not switching -> stated limits.
- finding P7: guard.mjs should fail closed if an ask ever arrives with a role set (belt and braces for A26).
- finding P8: protectedHit refactor + gh() vcwd touch routine paths -> existing tests must pass unedited.
- Next: block 5, write verdict.

## Verdict (block 5)
**APPROVE, with conditions C1-C12.** Build may start after E10 (C1); only the night-shift switch (WT-NIGHT) waits for
Mustafa's answer (C2). The PR is needs-mustafa at merge (escalation category 3: `.claude/`, CLAUDE.md, security rules):
his reconfirmation of the merge, no Brave check (nothing a player sees).

Why: all five of his items are met (1 day ask; 2 sk- after %XX; 3 Actions reads for pr-advisor; 4 the worktree rule;
5 the deferred points and queue into STATUS), the routines' denies are untouched or stronger, the reading of "the three
deferred points" is right (F-1, R-1, R-2: queue/docs-pass-items.md:123-127, deferred ~22:30 UTC, his addition ~22:45).
But revision 1 grew past his words in five places; the conditions cut those back to follow-ups, and I take the
technical decisions myself so Mustafa gets only the one that is truly his.

### Decisions I take (inside his words; he can overrule any at merge)
- AF-MECH: (b), a PreToolUse "ask" from the day guard only when SHAMBLE_ROLE is unset; ask list stays [] (settings.test.mjs:56).
- AF-AGENT: (b), refuse a subagent's kit edit, IF E10 confirms `agent_id` is in subagent hook input; else (a) the ask.
- AF-COVER: a trimmed (b): file tools; >, >>, &>, tee, sed -i (all forms), perl -i/-pi, cp, mv, rm, ln, touch,
  truncate; plus git rm / git mv on kit paths, directory targets (`.`, ancestors of .claude/.github/tools, a folder
  holding CLAUDE.md), and a GIT_DIR=/GIT_WORK_TREE= prefix (asks: it would blind the folder check). NOT built, stated
  limits in STATUS: the script-text check (A32: low precision, it asks on any harness that only mentions `.claude/`,
  and low recall, a path built at run time passes), install, dd, git config --file, format-patch -o, archive -o,
  ~/.claude/settings*.json (A31, A34). Why: his item is "ask before kit and gate edits", a seatbelt in a watched session
  (plan §2's own words); the wall stays the PR review and his go.
- WT-DAY: (ii), trimmed to what switches the main folder (his words: "another branch, a PR's head or an old commit"):
  git switch / --detach with a protected difference from origin/main (no name exempt), git switch -c with another
  start point, gh pr checkout, git merge / git pull other than the WT-UPDATE forms, git bisect. cherry-pick, am, apply,
  revert, read-tree -u, stash apply|pop (T21): stated limits.
- WT-UPDATE: (a).
- SECRET-DECODE: (A) only, the widened ESC fragment in the API key, AWS and Slack patterns (%XX incl. %25-chains and
  %uXXXX, backslash escapes); `=XX` only if E3 shows 0 new hits. NOT built: (B) pctDecode re-test, env values on decoded
  text, `@` in URL credentials: follow-ups. Why: his item is the sk- key after %20/%2C/%3A; one shared fragment
  meets it for the same bug class in three patterns; (B) is a new kind of check.
- F1-FIT: (b), record F-1 in STATUS's follow-ups; no CHECKPOINTS.md trust-rule edit, no E8. Why: his words "Record them
  in the repo, in STATUS's follow-ups"; the PR #37 advisor deferred it to "A3 or later".
- READ-SCOPE: (b), the owner/repo check on the new Actions reads only; get_job_logs on another repo, gh -R/--repo,
  search_*: one follow-up line for security-auditor.
- NODE-ALLOW: (a), no entry; recorded as decided.
- QUEUE-HOME: (a), with C9's placement.
- The `--output` entries for rev-list, diff, log, show: kept (a write hole in the guard, found while adding rev-list;
  bugs before features).

### Conditions
C1. E10 (the doc quotes) runs before step 3: hook ask vs deny order; ask in default, acceptEdits, auto and cloud
    sessions; `agent_id` in subagent hook input; what `memory: local` grants; what a running session reloads. A page
    that contradicts AF-MECH or WT-DAY stops the build and comes back to me.
C2. WT-NIGHT is built last (switchTargetCheck, NIGHT_SHIFT.md:26, the Never bullet's exception clause, T15-T16c, T20),
    only after Mustafa answers. Until then, everything else is built; if he has not answered when the rest is done,
    wait; never ship the Never bullet with an exception he has not accepted.
C3. Cut from the build: dayScriptCheck (A32), A31, A34, T21's main-folder patch commands, pctDecode and its cases, the
    env-on-decoded and `@` cases, the READ-SCOPE (a) cases (get_job_logs x/y, gh -R), the CHECKPOINTS.md F-1 edits
    (:77-78, :110-111, :119) and E8. Each becomes one line in STATUS's follow-ups or its stated-limits list.
C4. CLAUDE.md's new step-3 sentence names only what the guard checks: drop "scripts that name them"; say file tools and
    shell writes, and that the PR review is the wall.
C5. guard.mjs prints an ask only for PreToolUse with no role; if `d.ask` ever arrives with SHAMBLE_ROLE set, it prints a
    deny (fail closed). A test for it (a fake decide is fine), plus A26 as planned.
C6. Routine paths: every existing routine-guard and settings test passes unedited (additions only; any edit to an
    existing case named and justified in the report); the protectedHit refactor and gh()'s new vcwd keep the routine
    push/save and gh checks byte-for-byte in behaviour (E1 on main and head).
C7. ACTIONS_READ exactly as §4.1 (the listed methods; download_workflow_run_artifact, get_workflow_run_logs_url and
    unknown or missing methods refused; night refused; owner/repo must be REPO), matching the tool schemas' enums.
C8. The secret tests keep test strings synthetic and split in the source; E3's false-positive scan over `git ls-files`
    is in the report with its count; the refusal still names only file and kind.
C9. STATUS (his words: "in STATUS's follow-ups"): F-1, R-1, R-2 and every open docs-pass-items point go in the
    Follow-ups bullet (STATUS.md:277); A4 and B.1/B.2 word for word in a "Queued, in Mustafa's words" block directly
    under it, with date and source; A3's own words quoted whole in the committed report. The E9 table (every queue
    point -> its STATUS line or its closed proof) lives in the committed report, so nothing in the queue files is lost
    when the notes folder is deleted. This is built first (plan step 2), as planned.
C10. advisor.md:150: keep `memory: local`; the line states what E10 finds (if memory grants Write/Edit, say the advisor
    writes only its memory and notes, and that the guard confines pr-advisor writes to .checkpoints/, .gate/ and agent
    memory, routine-guard.mjs:292-294).
C11. The report lists, under changes to his words, each trim above and each stated limit, so his merge reconfirmation
    covers the coverage level; security-auditor is asked to weigh the stated limits explicitly.
C12. Plan §8 step 1 changes to: advisor APPROVE -> E10 -> build (WT-NIGHT waits, C2); the decisions to Mustafa are only
    WT-NIGHT and PC-MODE. Agents: Opus 5.5 at high, at most 3 at a time, checkpoint saved after every step.

### Decisions for Mustafa
- WT-NIGHT: your worktree rule says "never switch a session's main folder"; the night shift's handoff today switches
  its folder to the item's branch (NIGHT_SHIFT.md:26). Why it matters: without an answer the night either breaks your
  rule or loses its handoff. Options: (a) an exception: the night may switch only when the branch's rules, CI and gate
  files equal main's, otherwise it refuses and stops the item (after each kit merge, older branches stop until a day
  session updates them); (b) no exception: the night works in a separate worktree, which needs its own tested
  change-set first, and until then A3 makes the night stop the item instead of switching. Recommendation: (a).
- PC-MODE (information only): which permission mode your Windows PC sessions use (default, acceptEdits or auto), so the
  after-merge check runs in it.
- block 5 done: verdict written. DONE
DONE
