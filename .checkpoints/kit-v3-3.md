# Checkpoint: kit-v3-3 (A3 on Mustafa's list: the guard's follow-ups)

- **Branch:** `claude/sleepy-tesla-ofsxsr`, standing in for `cs/kit-v3-3`: this cloud session may push only there.
  Worktree: `/tmp/claude-0/-home-user-Shamble-Frontlines/263eddf3-7643-5640-b1d5-39be43d3567c/scratchpad/a3`, local
  branch `kit-v3-3`, from main 12d7961 (PR #37 merged). Pushes are fast-forwards. Checkpoint commits are made by hand
  and marked `[skip ci]`.
- **Mustafa's words**, word for word: `.checkpoints/kit-v3-3/mustafa-words.md`. His three items:
  1. ask before kit and gate edits in day sessions;
  2. the `sk-` key after `%20`, `%2C` or `%3A` (checkpoint.mjs's secret check);
  3. read-only GitHub Actions tools (such as `actions_get`) for pr-advisor.
  security-auditor reviews A3. His two additions (2026-11-04, after PR #37's merge):
  4. the worktree rule from the freeze, into the rules: never switch a session's main folder to another branch, a
     PR's head or an old commit; use a separate worktree (`git worktree add`). Why: switching loads that branch's
     `.claude/settings.json` and hooks into the running session;
  5. the deferred points and everything in the session's scratchpad queue file into the repo, in STATUS's
     follow-ups, alongside A3's plan. The queue files are now in `.checkpoints/kit-v3-3/queue/`: docs-pass-items.md
     (the queue file), a4-spec-2026-11-04.md (A4 in his words), mustafa-2026-11-03-list.md (his list: B.1 and B.2
     still queued) and a2-items.md (where A3's items came from).
- **For the plan to weigh** (STATUS "Open bugs and risks", A3's entry; the advisor's deferred F-1 from PR #37):
  - `git rev-list` refused for routines;
  - python allowed on main's tracked scripts but missing from the docs;
  - the advisor has no Write tool, so in a routine it returns its notes in its report;
  - an allow-list entry for `node .checkpoints/*`;
  - F-1: the trust rule misses a done block whose output was lost with a container.
  Out of scope unless the plan argues otherwise: the kit-v3-1 Windows test follow-up (its own change-set, advisor
  condition C6).
- **Current step:** 2, the build, part 1 (workflow wf_77fe99e1-57c: E10's doc quotes, plan revision 2 with the
  conditions and the E10 ruling, then build step 2, the STATUS follow-ups first).
  - E10 (`e10-docs.md`): the core claims confirmed verbatim; five wording contradictions stopped the build (C1).
  - The advisor's E10 ruling (`advisor-e10.md`): PROCEED with E10-1 to E10-9; `memory: local` kept; PC-SETUP (three
    information questions) replaces PC-MODE; WT-NIGHT unchanged, still Mustafa's.
  - Done: plan revision 2 (`plan-rev2.md`: rev 1 + C1-C12 + the E10 ruling, with a condition table in §0); build
    step 2, docs-keeper (`docs-keeper-step2.md`): STATUS's follow-ups and "Queued, in Mustafa's words" (A4, B.1,
    B.2; diffed identical), PR #37 marked merged: 93b9a07.
  - Done: build part 2 (wf_99ae356e-07e): steps 3-5 built, committed f22c4df (notes d3d73a6). Tests: routine-guard
    317/317, settings 124/124, checkpoint 25/25, 0 existing lines changed; gate --no-smoke PASS, symbols +0 -0.
    Step 4 could not build the Edit-tool ask and the strict merge rule: two old cases (:130, :110) asserted the old
    behaviour. The advisor ruled (`advisor-build-step4.md`): PROCEED with R1-R4.
  - An automated security review of f22c4df found four issues in step 4's new day code
    (`security-review-auto.md`): S-1 (high), a switch target after `--` reaches the guard's own git diff (the guard
    could write a file); S-2, an origin/main name ambiguity; S-3, no case folding on darwin; S-4, pushd/popd not
    tracked. Fixed in step 4b, each with a test.
  - Done: build part 3 (wf_3dad6479-df3): step 4b (R1-R4, the Edit-tool ask, S-1 to S-4), step 6 (the secret
    check; E3 0 new false positives), step 7 (the rule texts, no WT-NIGHT parts); python in the refusals (the main
    session): 2e82216. E11a (`e11a.md`): the guard's ask stops a kit-file Write in default, acceptEdits and auto.
    A second automated review of 2e82216 (`security-review-auto-2.md`): findings 5 (CDPATH set in the line) and 6
    (a switch target read as its branch), fixed in step 4c: bbcb96b. Docs-keeper's step 8 pass: 442fbf7. Full
    gate with smoke PASS (`gate-step8.md`); tests 555/555; CI green on 2e82216.
  - Done: the reviews on 442fbf7 (wf_f5f87ff2-e86, a3dfef2): security-auditor FAIL on SA-1 (a regression from S-4:
    a lost folder checked relative writes against the project folder); SA-2 (origin/main forgeable; on main
    already); SA-3, SA-4 minor; nits SA-5 to SA-10. The verifier PASS WITH NOTES (E1-E9 pass); F-b minor; nits F-c,
    N-1 to N-3. Skeptics: SA-1 to SA-4 and F-b upheld, F-a refuted. A third automated review (bbcb96b): findings 7
    and 8 (`security-review-auto-3.md`).
  - The advisor's fix-round ruling (`advisor-fix-round.md`): PROCEED. The cd tracking redesigned by construction
    (R-a to R-e: an unknown folder makes both checks ask); SA-2's routine half fixed in A3; F-b, SA-5, SA-6, SA-8
    fixed; limits L1, L4, L9 widened and L12-L15 added; follow-ups listed; PC-SETUP gains question (4).
  - Done: the fix round (wf_9120ac05-de5): the cd tracking by construction (R-a to R-e), SA-2's routine half (plus
    SA-2b, symbolic-ref), F-b, SA-5, SA-6, SA-8: 2acfbbc (notes ee32690); docs-keeper's pass: 22f2480. Tests
    716/716; every mutant killed; gate with smoke PASS.
  - Done: step 9, WT-NIGHT (a) (wf_d7a5441c-cfa): the night switch check, the Never exception clause, NIGHT_SHIFT.md
    step 4, CHECKPOINTS.md section 8's night clause: ed3ea8c (notes 3b3b788). Tests 768/768; 13 mutants killed; gate
    with smoke PASS. Reported only: F9-1 (the existing push and commit checks read plain diff output).
  - A fourth automated review (2acfbbc): findings 9 (SA-6 reads the index only), 10 (an assembled variable name),
    11 (git clone's option list) (`security-review-auto-4.md`).
  - Done: the delta review of 442fbf7..ed3ea8c (wf_8875d77e-6a9, 71d3544): security-auditor FAIL (SD-1, SD-2;
    SD-3 to SD-9 minor or nits), the verifier FAIL (V-2 = SD-1, V-3; V-1, V-4, V-5, D-1, N-a); skeptics upheld every
    finding. A fifth automated review (ed3ea8c): findings 12 (= SD-1) and 13a (= V-3); 13b refuted.
  - Done: the advisor's fix-pass-2 ruling (`advisor-fix-pass2.md`): PROCEED. Routine paths fixed by construction
    (SD-1 + A-3: a night switch, commit or push line holds that one command alone, pr-advisor exempt; V-3: exact
    show-ref lookups; SD-2: -z --no-renames --no-relative in protectedHit and the commit check, a named C6 edit);
    day fixes SD-3, SD-4, SD-5, SD-6, V-1; limits L9 and L11 widened, L16-L18 new; follow-up A-4; D-1 docs-keeper.
  - Done: fix pass 2 (wf_6ad69a99-cc0): code c8cbb1a, docs 01b158c (notes 66222f2, ae4a344; `builder-pass2-routine.md`,
    `builder-pass2-day.md`, `docs-keeper-pass2.md`). Routines: SD-1 + A-3 (a night switch, commit or push alone on its
    line), V-3 (exactCommit: show-ref --verify), SD-2 (DIFF_NAMES; a commit whose lists git can't give is refused, the
    builder's own stricter addition). Day: SD-3, SD-4, SD-5, SD-6, V-1. NIGHT_SHIFT.md step 4 fetch-first text. Tests
    864/864 (710, 127, 27); every new mutant killed (in-suite and real-git); no existing expected value moved (A3-own
    mutant retargets named); main's file 180/192 (3 pre-existing, 8 rev-parse scaffold, 1 SD-2 key scaffold: "refused:
    night: commit a staged rules file"); gate with smoke PASS (the main session, at the working tree). Docs: D-1 stale
    lines, L9 and L11 widened, L16-L18, follow-up A-4, the closed line (SD-4). CI on 01b158c started.
  - Done: pass 3 (wf_4c636903-c6d; `security-auditor-pass3.md`, `verifier-pass3.md`, `skeptic-pass3-*.md`, probes
    `sap3-*`, `vp3-*`, `skp3-*`). The verifier PASS WITH NOTES (864/864; main's file 180/192 as named; every ruled row
    in; no survivors; gate with smoke PASS; texts PASS); its notes: commit pathspecs (= P3-5), SD-5's false ask (= P3-7).
    security-auditor FAIL on routine paths, all on main 12d7961 too: P3-1 a `#` comment holding a quote, P3-2 a
    here-document delimiter with a quoted or escaped space, P3-3 `$'...'` (date carrier): each hides a whole command
    from parseShell (real bash ran a hidden night switch to cs/evil and CLAUDE.md writes, night and pr-advisor); P3-5
    a night commit with pathspecs (no `--`, `-- .`, globs, `:(top)`) skips the commit check (push check still holds).
    Minor: P3-4 a fetch backgrounded by an earlier call (L18's neighbour), P3-6 `.gitmodules` ignore = all hides a
    gitlink (`--ignore-submodules=none`), P3-7 SD-5 asks on `git clone <url> .gate/tools` (a regression on a control);
    nit P3-8 (`git stash && git switch main` asks: name it). Skeptics upheld all four majors (P3-1, P3-2 major not
    critical; P3-3 critical defensible; P3-5 major and wider). Night queue (issue #16): no unchecked item, so tonight's
    night shift does nothing; the PR Advisor reviews only ready PRs. CI green on 01b158c. No automated review of c8cbb1a
    has arrived.
  - Done: the advisor's pass-3 ruling (`advisor-pass3.md`; probes `adv3-probe*.out`; cut by the usage limit at its
    start, resumed after the 15:10 UTC reset): PROCEED with fix pass 3 inside A3, then a scoped pass 4. New finding
    P3-9 (major, on main too): `>&word` writes a file the guard never sees. Routines: R-PLAIN (the raw line in a plain
    form the guard reads exactly as bash does, or refused; no here-documents), proven by the corpus test C-PLAIN;
    dateCheck's word rule; the night commit allowlist (P3-5: `-m`, `--message`, `-a`, `-q`, `-s`, `--allow-empty`
    only); P3-4 (no git in the background); P3-6 (`--ignore-submodules=none` in DIFF_NAMES). Day: parseShell's five
    rules (comments, backslash-newline, ANSI-C, here-document delimiter as one word, `>&word`); P3-7 (clone targets
    narrowed); P3-8 named. Texts: an R-PLAIN rule line, L16 and L18 replaced, SD-3 and night-commit sentences,
    NIGHT_SHIFT.md step 7, follow-up FU-P3a.
  - Mustafa's scope stop (2026-11-05 ~16:10 UTC, word for word in `mustafa-words.md`): stop teaching the guard new
    shell syntax; routines: R-PLAIN; day: a line the guard can't fully read asks (D-ASK), not a parse; Day C's parser
    additions dropped; keep the clone fix (P3-7), the no-looser check and the full gate; one final review (pass 4) with
    his review budget (below Critical or High: a STATUS follow-up, not another pass); after A3, the kit is frozen until
    the netcode proof test is under way (A4 edits advisor.md, a kit file: it waits; B.1 and B.2 are docs).
  - Fix pass 3 so far (wf_8d8f9764-ff3, stopped at Day C's start, before any edit; its notes in `old/`): routine A done
    (R-PLAIN, C-PLAIN 212 + 2,000 lines, dateCheck, P3-4, NIGHT_SHIFT.md step 7; 967/967), routine B done (the night
    commit allowlist, P3-6; main's file 180/192, the same 12; gate --no-smoke PASS). Uncommitted in the worktree.
  - Done: fix pass 3 (wf_8d8f9764-ff3 resumed; notes `builder-pass3-routine-a.md`, `-routine-b.md`, `-day2.md`,
    `docs-keeper-pass3.md`): code 9589bfb, docs 60ae124 (notes ec68c2c). R-PLAIN (plus five narrowings the builder
    found), C-PLAIN (212 + 2,000 lines), dateCheck, P3-4, the night commit allowlist (P3-5), P3-6, D-ASK (the five
    forms ask; an agent refused; no new parsing), P3-7, NIGHT_SHIFT.md step 7. Tests 1084/1084 (930, 127, 27); every
    mutant killed; no routine line looser (1,260 decisions); main's file 180/192 (the same 12); gate with smoke PASS
    (the main session too). To name for the advisor (C6): four rows moved out of main's mutation table into
    PLAIN_BEHIND_MUTANTS (R-PLAIN refuses every row they guard). Older drift found by docs-keeper (not A3's): the
    night window differs between NIGHT_SHIFT.md:3 (07:10), :17 and REPO_MAP:75 (07:30) and CLAUDE.md (08:30).
  - Done: pass 4 (Agent tool; `security-auditor-pass4.md`, `verifier-pass4.md`, probes `sap4-*`, `verifier-pass4-*`):
    security-auditor FAIL: P4-S4 critical (a `.gate/` worktree lets a night commit and push of rules or CI files
    through: protectedRel maps only `.claude/worktrees/`), high P4-S3 (a command with no program skips the redirect
    checks), P4-S1 (a wrapper name as a path runs the file), P4-S2 (a quoted NAME= after a keyword passes R-PLAIN),
    P4-S5 (xvfb-run -e/-f write files); S1, S3, S4, S5 on main too. Medium P4-S6 (D-ASK misses here-document bodies);
    low P4-S7, P4-S8. The verifier PASS WITH NOTES (1084/1084, C-PLAIN plus its own 7,246-line fuzz, mutants, no
    routine line looser, main's file 180/192, gate with smoke PASS; lows P4-V2 to P4-V4; the C6 move described). The
    automated review of 9589bfb: finding 14 MEDIUM, day (`security-review-auto-6.md`). CI green on 60ae124. No routine
    text uses a wrapper as a command (the session's grep), so one rule may close S1, S2, S3, S5.
  - Done: the advisor's pass-4 ruling (`advisor-pass4.md`; probe `adv4-cd*.out`): PROCEED with fix pass 4 inside A3,
    checked by the verifier alone. New High A4-1 (a failed cd then `;`: the next redirect is checked in the wrong
    folder; on main too). R-PROG (a clause of R-PLAIN: every routine command starts with git, gh, npm, node, python,
    python3 or date by bare name, after only TZ=, LC_ALL=, LANG=, LANGUAGE=, NO_COLOR=; redirects after the program)
    closes P4-S1, -S2, -S3, -S5 and A4-1; the worktree path rule (kitName at the commit check and protectedHit;
    kitPath in the routines' writePath and writeTarget) closes P4-S4; S3's second layer moves the role redirect loops
    above `if (!argv.length) continue`. C6 accepted (plus a `git status > 'a*b'` row). Follow-ups FU-P4a, FU-P4b,
    FU-P4c (after the kit freeze). No night-window drift on main: this session's loaded CLAUDE.md is kit-v3-0's
    (3f07223), older than main's; read main's copies before relying on it.
  - Done: fix pass 4 (`builder-pass4.md`; cut by the usage limit at its last block, resumed after the 20:10 UTC reset):
    code 0c481e1 (CI green), docs 6f18311 (docs-keeper's final pass, `docs-keeper-final.md`, plus the session's three
    truth edits: the `.gate/` worktree and SA-3 follow-ups closed for routines, the limits intro names the pass-4
    ruling). 1188/1188; gate with smoke PASS (the main session too); no routine decision looser (1,348); every probe's
    harm refused; main's file 179/192 (#181: cat now gets R-PROG's reason, the Read/Grep hint lost). Beyond the ruling:
    two main mutation rows moved into KIT_BEHIND_MUTANTS (kitPath), for the advisor to accept.
  - Now: the verifier alone checks fix pass 4 (`verifier-fixpass4.md`), under his review budget; then step 10.
  - Mustafa, 2026-11-05 ~16:25 UTC (word for word in `queue/mustafa-2026-11-05-a4-b1.md`): A4-FREEZE (a), A4 waits
    until the netcode proof test is under way; a B.1 addition: his editor decisions (tool order, primitives and one
    scale, object sizes, the views, trace sketches, notes, snapshots, the travel-time ruler, stamps, the lean-editor
    principle) go into EDITOR_V2 with the plan view, marked as he marked them. A3's final docs pass copies both into
    STATUS's "Queued, in Mustafa's words". After A3: B.1, then B.2 (docs, each its own change-set, plan first).
  - Next: step 10 (the report, the checkpoint deleted, ready, the advisor's final-head review, needs-mustafa, his go).
- **Mustafa's answers (2026-11-05 ~10:30 UTC, word for word in `mustafa-words.md`):** WT-NIGHT (a), the exception
  only when the branch's rules, CI and gate files match main's: step 9 is unblocked. PC-SETUP: Auto mode; no mods
  (thinking about it); an Enterprise login (university); Windows itself (PowerShell), not WSL. So L7 (mods) does not
  apply on his PC by the docs (an Enterprise login), L9's WSL part does not apply, and E11 P4 runs in Auto.
- **Agents:** Opus 5.5 at high, at most 3 at a time.
- **On a usage limit** (Mustafa, 2026-11-05, word for word: "Just wait for the next reset, I can't ask to raise the
  org limit"): wait for the reset the error names, then resume from the notes and the workflow's journal.

## Next steps
1. Plan revision 1; the advisor's plan review (a general agent reading main's advisor.md).
2. If the advisor escalates: the decisions to Mustafa, then wait. If APPROVE: build in this worktree.
3. Gate and evidence; security-auditor (and the verifier) on the final diff; the report; delete the checkpoint;
   ready; the advisor's final-head review; `needs-mustafa`; his go.
