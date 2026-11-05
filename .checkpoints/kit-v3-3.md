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
  - Now: the fix round (wf_9120ac05-de5): builder-fix1 (the cd redesign), builder-fix2 (SA-2 and the small
    fixes, the full gate), docs-keeper (the limits and follow-ups).
  - Next: commit; the automated review of the fix commit; security-auditor and the verifier on the fix diff only;
    step 9 (WT-NIGHT) after Mustafa's answer; step 10 (the report, ready, the advisor's final review).
- **Agents:** Opus 5.5 at high, at most 3 at a time.
- **On a usage limit** (Mustafa, 2026-11-05, word for word: "Just wait for the next reset, I can't ask to raise the
  org limit"): wait for the reset the error names, then resume from the notes and the workflow's journal.

## Next steps
1. Plan revision 1; the advisor's plan review (a general agent reading main's advisor.md).
2. If the advisor escalates: the decisions to Mustafa, then wait. If APPROVE: build in this worktree.
3. Gate and evidence; security-auditor (and the verifier) on the final diff; the report; delete the checkpoint;
   ready; the advisor's final-head review; `needs-mustafa`; his go.
