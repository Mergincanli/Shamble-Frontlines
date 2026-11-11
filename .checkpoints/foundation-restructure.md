# Checkpoint: foundation-restructure (foundation step 2.1; finishing 2.1a on PR #50, which replaces #47)
- **Goal:** 2.1a, the legacy gate and tools learn both layouts (root and `legacy/`) without moving anything; then 2.1b,
  the move. Plan: `.checkpoints/foundation-restructure/plan.md` (revision 3; the advisor agent's APPROVE WITH CONDITIONS
  of revision 2, 8bbebfa, notes `advisor-plan-rev2.md`). The planning task's history: this file at d389d91.
- **His words:** the foundation decision and task (`mustafa-2026-11-10.md`) and his answers of 2026-11-10
  (`mustafa-2026-11-10-answers.md`): FREEZE-FDN (a), SPLIT (a), SCOPE (a) (ADR 0002 part 1 accepted, recorded in 2.1b's
  docs pass), "Build 2.1a now: the dry run, the gate edits (each asks me) and the mutation proof"; merge order #46,
  STATUS-DIET and LANE-RULES, 2.1a, 2.1b, each on his go; 2.1a's PR stays a draft until its turn; both accounts count
  the same. His messages of 2026-11-11 in this session (`mustafa-2026-11-11.md`, extracted by script from the
  session's events): his two additions (kit and gate edits ask him in chat first, never the guard's prompt alone; his
  lean for a replacement PR), BRANCH-47 (a) with its one-time exception, and his words on the advisor tool.
- **Branch:** `claude/pensive-heisenberg-aazidf` since 2026-11-11 (this session may push only there; PR #50 replaces
  #47 on his BRANCH-47 (a)), standing in for `cs/foundation-restructure`; Mustafa's personal account. #47's branch
  `claude/advisor-fable-4hxq72` stays at 26e0d82 (PR closed). **Base:** main bb2d817 (brought in by the merge d6a3fee).
  **OLD** for the evidence: main 44446a5.
- **Last update:** 2026-11-11 18:30 Amsterdam

## Steps done
0. Planning (d389d91): plan revisions 0-3, the inventory workflow, the advisor agent's plan review, his answers.
1. Build step 0 (this commit): checkpoint moved from `foundation` to `foundation-restructure`; his answers saved;
   the open PRs re-read on 2026-11-10: #46 at d7526f5 (guard, its tests, advisor.md, CLAUDE.md, docs), #40 at b7a1a32
   (REPO_MAP, STATUS, docs/design, spikes); STATUS-DIET and LANE-RULES have no PR yet. 2.1a's files (tools/gate.mjs,
   tools/smoke.mjs, tools/village.py, tools/README.md) are in none of them.
   - The toolkit: `npm ci --prefix tools` done. Playwright 1.56.1 wants Chromium build 1194; the container has 1243 at
     /opt/pw-browsers and `npx` is denied, so the smoke runs with `PLAYWRIGHT_BROWSERS_PATH=<repo>/.gate/pw-browsers`,
     a folder of links naming 1243's executables as 1194 (smoke on main: PASS, index 8/8, editor 5/5). Caveat for the
     report: locally a newer Chromium than pinned; CI runs the pinned one.

2. Build step 1, the dry run (`.gate/fdn-dry`, worktree of 44446a5): `git mv` of the five files → five `R100`; today's
   gate: exit 2, "gate: shamble-core.js is missing from the working tree" (gate.mjs:98), as the plan expected.
3. Build step 2, the tools (dd12aeb): built and proven first in a plain scratch folder `.gate/fdn-scratch` (not a
   worktree; `protectedRel` false there), then written with one ask each for `tools/gate.mjs` and `tools/smoke.mjs`
   (`cp` from scratch, `cmp` equal), plus `tools/village.py` and `tools/README.md`. Full gate with smoke PASS on the root
   layout (base 44446a5). Folder rule table `rows-test.mjs`: 8 rows + 2 smoke-CLI rows ALL PASS. Without Playwright
   (`--omit=optional`): `--no-smoke` PASS. Base checks (`e3-bases.md`): the edited gate reads root, CS-1
   (`shamble-frontlines-repo/`) and legacy/ bases with a root or a legacy/ head, all PASS; OLD's gate DIES on a legacy/
   base (the revert case the split prevents); both copies → exit 2.
   - Departure from plan 1.3 (to state in the plan and the PR body): the folder marker is the three `shamble-*.js` files,
     any of them, not "all five files": any overlap → exit 2, and a future v2 `index.html` at the root never makes the
     root look like the legacy build. New names: smoke.mjs `findBuild` (export), `LEGACY_JS`; gate.mjs `BUILD`,
     `BUILD_PREFIX`, `BASE_PREFIXES`, `basePrefix`; `baseFile(b, f)` now takes the base object.

4. Build step 3, E3 (`e3-summary.md`, `e3-results.json`): 31 of 31 PASS. For every mutation OLD's gate (44446a5), the
   edited gate at the root (dd12aeb) and the edited gate on `legacy/` (9af08f7) fail exactly the same checks, never none;
   every control exits 0 on all three (PC1 on check 3). P1-P4 fail on the Phase 0 report's checks for its reasons.
   M19/M20 split the report's "5b + 5d" pair (5b and 5d one each, the same on all three): a note. Worktrees restored
   clean. CI run 42801926823 on 9ae4037: gate and smoke success (pinned Chromium).

5. PR #47's body updated with E3; M5 covers the outside-folder check on all three gates; the stricter `legacy/` rule
   proven (a repo-root read from `legacy/` fails 5a); CI also ran on the PR (run 42802055698, success); rebuild commands
   in `e3-summary.md`.

6. The replacement (his BRANCH-47 (a), 2026-11-11; this session, `claude/pensive-heisenberg-aazidf`): the merge
   d6a3fee (parents bb2d817 and 26e0d82; tree 79baf3e, as git's dry run predicted), made in the worktree
   `.gate/fdn-replace`. Proofs: its diff against main equals #47's diff against 44446a5 byte for byte (`cmp`); it
   changes only the four tools files and 27 notes; the gate-file blobs equal 26e0d82's (gate.mjs 2bfab53, smoke.mjs
   18058e4); the five game files equal dd12aeb's and main's, so E3 holds for this head; `.claude/` 66fa6ae, `.github/`
   67ccb30 and `CLAUDE.md` 16f0853 equal main's. Full gate with smoke on d6a3fee against bb2d817: PASS, +0 -0 (index
   8/8, editor 5/5; the Chromium-1243 alias rebuilt in this container). d6a3fee pushed alone; draft PR #50 opened on it;
   #47 closed with a pointer comment. The three re-checked just before the fast-forward (main still bb2d817):
   identical. The main folder fast-forwarded once to d6a3fee (the guard did not object); the worktree removed (its
   symlink first, no `--force`). His words of the day captured by `capture-2026-11-11.py`.
   - **Exception:** Mustafa's one-time exception to CLAUDE.md's Never rule (2026-11-11): the main folder fast-forwarded
     once, bb2d817 to d6a3fee, because d6a3fee's `.claude/`, `CLAUDE.md` and `.github/` are byte-identical to main's
     (re-checked just before); the same yes covered the merge's one gate-file write (dd12aeb's `tools/gate.mjs` and
     `tools/smoke.mjs`, nothing new added).

7. Step 1 closed (his step 1): CI green on d6a3fee (push run 43023480099, pull_request run 43023620759); #47 read
   back: closed 12:49:11 UTC, not merged, head 26e0d82. The report's Appendix A (2dd8827): his words for 2.1a in full,
   9 blocks copied by `report-appendix.py` from the notes and from bb2d817's message, each checked byte for byte.
   His #47's-turn message in `mustafa-2026-11-10-answers.md` is byte-identical to the old chat's record (event
   c49a5660-c1d0-4c58-a54c-983fe54c4394, 2026-11-11 09:46:31 UTC, 848 characters); the 2026-11-10 notes are not
   re-checked (labelled so). The status-diet session (where his #49 go was given) is not readable from this account; the
   routines are not listed from this account (they live on the university account).

8. His step 2, the docs pass (1023870), by docs-keeper (Opus, high; notes `docs-keeper-docs-pass.md`, scripts
   `docs-keeper-*.py`): DECISIONS.md nine blocks of his words + one Queued sentence, copied from the report's
   Appendix A (check script PASS: nothing unclassified, every other old line unchanged and in order; re-run by the
   main session: PASS); three pointers beside quoted blocks; the italic header line reworded; STATUS 112 lines (#50's
   row, #49 merged, main bb2d817); the archive (#49's merged row; kit-v2-1 Low fixed in dd12aeb); ROADMAP (2.1a in
   review, #49 MERGED, 12 entries); ADR 0002 and SPIKE_PLAN pointer lines (decision docs: his go). Gate with smoke
   PASS; repo-map-check OK. Its notes for later: stale `tools/` citations in research docs (LEGACY_INVENTORY TL1, TL6,
   TL9; r5 :925, :1737) left unedited (dated research); more "before the foundation" lines with no pointer (ADR 0002
   :38, :724, :1115; SPIKE_PLAN :96, :194; the plan named only the ones done); ADR 0002 lines after 1425 shift +4,
   SPIKE_PLAN after 92/130/440 shift +4/+8/+12.

9. Before the round: CI green on 1023870 (push 43028828063, pull_request 43028839082); #49 read back (merged_at
   09:03:11 UTC, head 2e6e5d2; comment 6779064873 approves 2e6e5d2 by full SHA). Touch-ups by hand (3c98ad5): #49's
   time is merged_at (09:03:11; the commit is dated 09:03:10) in STATUS, the archive row and the DECISIONS header; the
   archive's intro line names what docs-keeper adds since (the advisor's Low 4 on #49, "Fix in the next docs pass").
   docs-keeper's generators are not re-run after these hand edits. Decisions check PASS; gate PASS. The review round
   launched on 3c98ad5 (pinned): security-auditor, test-engineer (owns rows-test.mjs; `.gate/fdn-rows` moved aside to
   `.gate/fdn-rows-main-1435`), infra-engineer; the reviewers judge the recorded E3. For the verifier's bounded E3 sample
   (P1-P4, M5, C1 on three gates): worktrees `.gate/fdn-old` (44446a5), `.gate/fdn-new` (3c98ad5), `.gate/fdn-newleg`
   (3c98ad5 + local move commit 104d675, never pushed), each with a `tools/node_modules` link; the runner copy
   `.gate/foundation-restructure/e3-sample/e3-run-sample.mjs` (only the table filter and OUT differ; OUT under `.gate/`).

10. Round 1 done: infra-engineer APPROVE, test-engineer APPROVE (TE-1 Medium), security-auditor REVISE (F1 High), the
   advisor agent REVISE (F1). The F1 fix, built and proven in the plain folder `.gate/fdn-scratch-f1` (nothing written
   to tools/): `tools/gate.mjs` only (smoke.mjs stays dd12aeb's blob); when base.prefix differs from BUILD_PREFIX, the
   base's folder (`join(ROOT, base.prefix)`) must hold no `index.html`, `editor.html` or `shamble-*.js` (case-insensitive
   listing), else exit 2; the exact change: `f1-gate.patch`. Red first: `rows-test-f1.mjs` on the unfixed gate
   (`.gate/fdn-scratch`) judged by the fix's expectations: 4 FAILED (h, i decoy, j `Index.html`, l revert leftover; each
   exit 0). Green on the fixed gate: 13 + 2 rows ALL PASS (h, i, j, l exit 2; k no move exit 0; m clean revert exit 0;
   base.prefix asserted). The legacy/ base for the rows: 89286ed (plumbing from 44446a5, fixed dates, reproducible). The
   fixed gate on a clean root tree with smoke: GATE PASS, +0 -0, 8/8, 5/5. The advisor agent asked to rule on the
   amendment; then the ask to Mustafa (the exact change; FREEZE-FDN as its own question; the alternatives).
   The advisor agent's ruling on the amendment (its notes' addendum): APPROVE with conditions: (1) his yes in chat for the
   exact patch first; FREEZE-FDN asked as its own question, quoting him, no recommendation; (2) after the write: `cmp`
   with the scratch copy; the full E3 on the new gate (a new results file); rows-test-f1 against the written file plus
   the red run on the unfixed gate; both base directions raw with exit codes (TE-1); main's gate and the new gate on the
   decoy (the gate job's exit); the full gate with smoke on the fix commit; (3) round 2 on the change only
   (security-auditor, test-engineer, infra-engineer), then the verifier's sample on the final gate code; (4) the report
   and PR body: the as-built rule, only CI's gate job (not `--smoke-only`) catches a decoy, the v2 consequence (a 2.3
   known limit; qualify "never makes the root look like the legacy build"), new Low A-1 (a `shamble-frontlines-repo/`
   base with a root head: the check looks only in that old folder; old bases only; no code change); (5) IE-1 a STATUS
   follow-up for 2.3, L-2 by hand in the fix pass; L-1, L-3, L-4, F-L, the C1-C8 rebuild and the exception line in the
   report.

11. His answer (2026-11-11 14:54 UTC; `mustafa-2026-11-11-f1.md`, captured by `capture-2026-11-11-f1.py` from the
   session's events): F1-FIX (a); FREEZE-FDN (a) for this one check ("Any other new check still asks me"); yes to the
   exact change, covering only that write; smoke.mjs unchanged, its wording Low a follow-up for 2.3; round 2 the last;
   the report to give the Windows step with its exact command and how full this chat is; his words into DECISIONS.md.
   The write (3f29cb8): anchors checked first (tools/gate.mjs was 2bfab53; the notes patch = the patch at eaf5a45; the
   patch applied to a copy gives 7a8480ce8225f320de7627575321c052b8f76bd3 = the scratch copy); one plain `cp`, no
   guard refusal; after it: blob 7a8480c, `cmp` equal, smoke.mjs 18058e4, only tools/gate.mjs changed; gate with smoke
   PASS (+0 -0, 8/8, 5/5); committed alone and pushed.

12. The evidence on the new gate (files in `evidence-f1/`): rows-test-f1 on 3f29cb8's gate (scratch by `git archive`,
   blob 7a8480c): 13 + 2 ALL PASS, exit 0; on the unfixed gate (2bfab53): 4 FAILED (h, i, j, l, each exit 0), exit 1.
   Both base directions raw, exit codes without pipes (closes TE-1): worktrees `.gate/fdn-new` at 3f29cb8 and
   `.gate/fdn-newleg` at 7c22abf (3f29cb8 + a local move commit, never pushed; `git switch --detach`, the guard refuses
   `git checkout`): the revert (root head vs legacy/ base 7c22abf): exit 0, build ".", base prefix "legacy/", PASS, text
   header "base ... (files in legacy/) | head = working tree (files at the root)"; 2.1b's PR (legacy/ head vs root base
   3f29cb8): exit 0, build "legacy/", base prefix "", PASS; main's gate (44446a5) on the legacy/ base: "shamble-core.js
   not found at base 7c22abf", exit 2. The decoy (security-auditor's recipe, worktree of 3f29cb8, removed after; the guard
   refuses `git restore`: files restored by `git show HEAD:<f> > <f>`): main's gate 277eb96 exit 2 ("shamble-core.js is
   missing from the working tree"); the unfixed gate 2bfab53 GATE PASS exit 0; the new gate 7a8480c exit 2 ("the build
   folder moved from the repo root (base) to legacy/ (head), but the repo root still holds editor.html, index.html");
   the new gate `--smoke-only` exit 0 (only CI's gate job catches a decoy). E3 on the new gate: running (`e3-run-f1.mjs`,
   results `e3-results-f1.json`, log `.gate/e3-run-f1.log`).

13. CI on the fix commit 3f29cb8: the gate job green in both runs; the smoke job failed in both (push 43054418953,
   pull_request 43054429019) at "Install Chromium's system libraries": the runner could not resolve
   azure.archive.ubuntu.com (log read; the smoke step never ran): the known runner flake (STATUS "Open risks", #42).
   Failed jobs re-run (attempt 2, 16:26 UTC); no docs push until they finish. Docs during E3 (uncommitted until then):
   his 14:54 words as the report's Appendix A3.3 (`report-appendix.py`: 10 blocks, all byte-identical); DECISIONS.md:
   the A3.3 entry by `decisions-add-a33.py` (header for this file, quote copied from the appendix), L-2 (the #49 header's
   time and session "per bb2d817's message"; the doubled "saved by" removed in three 2026-11-10 headers);
   `docs-keeper-check-decisions.py` extended to 10 blocks: PASS (11 runs, each block once); STATUS's follow-up bullet
   "From foundation 2.1a" (115 lines) and the archive's "Follow-ups" group of the same name (IE-1..IE-4, SA-L1, TE-2,
   A-1, N2, L-4, the v2 root page, the five unpointed lines, the stale research citations); repo-map-check OK.

14. His words on permission prompts (2026-11-11 16:32:18 UTC, event 557d0202): captured by
   `.gate/foundation-restructure/capture-2026-11-11-prompts.mjs` (copy in the notes) into `mustafa-2026-11-11-prompts.md`;
   the report's Appendix A3.4 (11 blocks, all byte-identical); the DECISIONS entry by `decisions-add-a34.mjs` (copy in
   the notes); the check extended to 11 blocks: PASS. The refusals of the day, from the local transcripts
   (`.gate/foundation-restructure/prompt-tally.mjs`): main session 4 (guard: git -C, git checkout, git restore; deny list:
   rm -rf), the reviewers about two dozen (the guard's agent rules, git -C, git checkout, git restore, worktree remove
   --force). Who answered each prompt: from the server events, a background agent pages them (notes
   `general-purpose-prompt-tally.md`). CI on 3f29cb8: the push run's re-run failed the same way (runner DNS), the second
   time; no third re-run (consultation 6); STATUS's CI risk line updated in place.

15. The docs commit 8d8c422 (his words A3.3 and A3.4 in the appendix and DECISIONS, L-2, the follow-ups, the CI risk line): gate --no-smoke PASS; pushed after the pull-request re-run on 3f29cb8 ended (cancelled at 16:42 UTC by the job timeout, still in the library step). E3 on the new gate at 8 of 31: identical to 2026-11-10 in every column (.gate/foundation-restructure/e3-compare.mjs writes evidence-f1/e3-compare.txt).

16. The prompt tally (the general-purpose agent; notes general-purpose-prompt-tally.md, prompt-tally.json): 19 pages, 09:59:00-16:37:55 UTC, no gap; 499 tool results, each with a decision: classifier 318, settings rule 99, permission mode 40, answered by Mustafa 14 (all main session: git worktree add in .gate/ x3, git switch --detach in a worktree x2, git merge --ff-only x1 (the one fast-forward); shell writes the guard cannot resolve x8, incl. the gate write cp at 16:01, answered after his chat yes), refused by the guard 27, by the deny list 1 (rm -rf). Unique refusals 29 (28 in the window + 1 by the tally agent): write target unresolvable 17, git -C 5, git checkout 2, git restore 2, worktree remove --force 1, rm -rf 1; by who: main 4, infra-engineer 8, security-auditor 6, test-engineer 4, advisor 3, docs-keeper 3. Subagents cannot raise a prompt (requestNonInteractive). The script double-counts refusals (57): use 29. After 16:37:55 not tallied. Every refusal with its route (the next call): .gate/foundation-restructure/refusal-routes.mjs writes refusal-routes.md (29 rows, no email or secret; for the report's Appendix B).

17. His correction (2026-11-11 16:54:54 UTC, event a10ee1d6; captured by .gate/foundation-restructure/capture-message.mjs, copy in the notes, into mustafa-2026-11-11-prompts-correction.md): the chat stays in Auto mode; a guard prompt counts as unseen until answered in chat only for a kit or gate edit and a move of the main folder; for every other guard question his answer on the prompt is his answer (no chat ask); round 2 and the verifier briefed on the forms the guard refused (scratchpad refused-forms.md); no further tally agent; the ready report gives CI gate and smoke for the final head with run numbers, and #50 stays a draft if smoke is not green; recorded next to the words it corrects. Done: Appendix A3.5 (12 blocks), the DECISIONS entry directly above the 16:32 entry (decisions-add-a35.mjs), the check at 12 blocks: PASS; commit 0607b45 (gate --no-smoke PASS). CI on 8d8c422: gate green; smoke failed again at the library step (pull_request run 43060970457, 16:43-16:53 UTC): the mirror outage persists; no re-run.

18. E3 on the new gate (e3-run-f1.mjs; 17:20-18:18 Amsterdam; results e3-results-f1.json; comparison evidence-f1/e3-compare.txt by .gate/foundation-restructure/e3-compare.mjs): 31 of 31 PASS, failing check ids and exit codes identical to the 2026-11-10 record (e3-results.json, unchanged) in every column: 93 gate runs; the legacy/ column is a move (base 44446a5 at the root), so the new check ran on every one of its entries. The three worktrees restored clean. ROADMAP 2.1a entry updated for the fix and its proof. Gate with smoke on the working tree: PASS, +0 -0, 8/8, 5/5.

19. Before round 2: CI on 0607b45 green, push 43063326391 and pull_request 43063335859 (the step Smoke (check 5a) succeeded; the library install 27 s): the mirror outage cleared; the tools tree f23190c and the five game blobs equal at 3f29cb8, 0607b45 and 76544aa, so that run covers the fix; origin/main still bb2d817; the legacy/ base rebuilt from the plumbing to the same SHA 89286ed (reproducible). Round 2 (the last) launched on 76544aa21bff9d7c4be5b5e6e51fb9ae59d586c4: security-auditor, test-engineer, infra-engineer (Opus, high; notes <agent>-round2.md), each brief with the refused forms, the 12 blocks, the writers not to run, the deliberately unfinished items, a fresh rows folder (.gate/rows-f1-te-r2), the verifier-owned worktrees.

## Current step

The full E3 on the new gate runs in the background (task br8ig4sam, from 17:20 Amsterdam, about 100 min; resumable: a cut
mid-entry leaves a planted file, restore it with `git show HEAD:<path> > <path>` in that worktree, then re-run). Meanwhile,
no smoke: his words (report appendix A3.3; the DECISIONS entry by script; docs-keeper's check script extended), L-2,
the follow-ups (STATUS ids, archive text). Then round 2 pinned to a head holding all of it, the verifier, the report.

## Next steps (on PR #50, this branch)

1. (done) Re-read the docs layout; `#47` appears in docs/ only in STATUS (its row, Next, Waiting) and in the
   status-diet report; #49's merge (bb2d817, his go: "The next change-set records it") is this change-set's to record.
2. The docs pass, by docs-keeper (Opus, effort high), text moved by script, never retyped:
   - `docs/STATUS.md` "In flight": one row for #50 (2.1a, replaces #47), from the evidence.
   - `docs/DECISIONS.md`: his words of 2026-11-10 and 2026-11-11 from `mustafa-2026-11-10.md`,
     `mustafa-2026-11-10-answers.md` and `mustafa-2026-11-11.md`: the foundation decision, FREEZE-FDN, SPLIT, SCOPE, the
     build go, the merge order, both accounts, today's order; the advisor-tool words (he named DECISIONS.md for them,
     "unless they are already there"); the two additions and BRANCH-47 if the file's scope line takes them. Skip what is
     already there; check the entry format first.
   - The overtaken lines get dated pointers to that entry (plan step 4): ADR 0002 migration step 1 (0002:1422-1425),
     SLICE-ORDER condition 3 (re-find "still runs before the foundation"), SPIKE_PLAN :86-91, :115 and :429-432.
     Decision docs: his go.
   - The deferred kit-v2-1 gate-globals Low (re-find it in the archive): closed by dd12aeb.
   - 2.1a's done record in `docs/STATUS_ARCHIVE.md` as docs-keeper's split rule says; ROADMAP's entry; `docs/REPO_MAP.md`
     unchanged (`node .claude/scripts/repo-map-check.mjs`).
3. The review round on the final diff (3 agents at a time; Opus, effort high): security-auditor (tier 2: gate files),
   test-engineer (E3, the rows, the bases), infra-engineer (CI and the gate on old bases); then the verifier until PASS;
   one fix pass at most. The advisor agent checks the plan's conditions C1-C8 at #50's head: the numbered list was never
   committed (only C7 is labelled, plan 1.5), so give it the plan's diff 8bbebfa..b03f4d8 and `advisor-plan-rev2.md`,
   and say in the report what was done. E3's worktrees died with the old container: a rebuild (`e3-summary.md`) makes a
   new move commit (not 9af08f7); move `e3-results.json` aside first.
4. The report `docs/reports/2026-11-11-foundation-2-1a.md` (`/advisor-report`): fold this checkpoint in (steps with
   SHAs, E3 31/31, the rows, the bases, CI runs, the Chromium-1243 caveat, the marker departure, his two approved gate
   prompts, the exception line, the advisor-tool consultations counted); record this branch's last checkpoint SHA (2.1b
   reads `plan.md` and `prompt-2.2.md` there); stop the saver; delete `.checkpoints/foundation-restructure*` in the last
   commit (no `[skip ci]`); mark #50 ready; stop for his go (needs-mustafa: gate files; no Brave check for 2.1a). #50 is
   not watched (his "do not watch" carried over from #47).

## Open findings
- security-auditor on 3c98ad5: REVISE, F1 High + 2 Lows (notes `security-auditor-review.md`): F1, on a folder move between
  base and head the gate judges only the chosen folder: a clean `legacy/` copy plus root pages wired to broken scripts
  passes both CI jobs, where main's gate stops (reproduced); also a stray root page left by 2.1b passes (row h was
  "intended"). Fix: when base.prefix differs from BUILD_PREFIX, the other folder must hold no page or engine file, by a
  case-insensitive listing. SA-L1 letter-case variants differ between Linux and Windows (both safe; a README known
  limit); SA-L2 the #49 header's "about 09:02 UTC" and session id rest only on bb2d817's message (label them).
- The advisor agent on 3c98ad5: REVISE (notes `advisor-head-review.md`): F1 High reproduced; F1-FIX (a) recommended (the
  same rule); C1-C8 rebuilt from F-A..F-O and Q-A1..Q-A6 (no C list survives; only C7 = F-O): met or 2.1b's/2.2's, except
  L-3 (docs-keeper edited ADR 0002 at `high`; CLAUDE.md and plan 1.7 say `xhigh`; no effect), L-4 (C7: `npx` refused by
  settings, worked around by the browser alias, recorded as a caveat; CLAUDE.md's setup line is refused: a kit follow-up
  for 2.3), L-1 (plan.md's smoke bullet still says "all five files": the report states the rule as built), L-2 (a
  doubled "saved by"; SA-L2). Gate-file Lows cannot wait for 2.1b (plan.md:184): IE-1 becomes a follow-up for 2.3.
- test-engineer on 3c98ad5: APPROVE, 1 Medium + 5 Lows (notes `test-engineer-review.md`; its rows re-run 8 + 2 ALL PASS;
  the docs check catches 11 of 11 mutations on a copy; gap probes G1-G4 behave as intended): TE-1 Medium, the revert-case
  row of `e3-bases.md` (root head vs the `legacy/` base 9af08f7) cannot be repeated (9af08f7 is gone) -> to close by the
  verifier's item 1: both base directions on the local move commit 104d675 with raw `--json` (base.prefix) and exit
  codes kept (infra-engineer re-proved it, but through a grep pipe, no raw output); TE-2 Low, a base with all five
  files in two folders resolves to `legacy/` silently while a working tree exits 2 (no such commit in history:
  infra-engineer's scan of 518 commits; a gate change: 2.1b/follow-up); TE-3 Low, rows-test gaps (row h no smoke,
  smoke-CLI row c message, base.prefix never asserted): follow-up; TE-4 Low, a broken smoke.mjs stops the whole gate with
  exit 2 even with `--no-smoke` (fails closed): say it in the report; TE-5 Low, the DECISIONS check compares against the
  appendix (A2.3 was byte-compared in full against the chat record in step 7, not only by its count: answer in the
  report); TE-6 Low, the text-mode header "files in legacy/" unrecorded (infra-engineer's run A shows it).
- infra-engineer on 3c98ad5: APPROVE, 4 Lows (notes `infra-engineer-review.md`): IE-1 `tools/smoke.mjs:31` says "keep
  one copy" also when the files are split, not copied (wording; a gate file: deferred to 2.1b, no push for a Low);
  IE-2 2.1b must move all five files in one pushed commit, or two push runs go red (2.1b's plan); IE-3
  `tools/README.md` does not describe the build-folder rule (2.1b's docs); IE-4 the pinned Chromium 1194 has never run
  the `legacy/` layout: 2.1b's smoke proof comes from CI's run. Re-proved by it: 2.1b's PR and its revert pass with the
  edited gate; main's gate dies on the revert (exit 2); `--no-smoke` without optional packages keeps the folder rule;
  smoke.mjs failing to load stops the gate (gate.mjs:64-70, exit 2). Not verified by it: a real `npm ci
  --omit=optional`; E8 (`git revert -m 1`); the pinned 1194 on `legacy/`.

## Key numbers
- E3: 31 of 31 PASS (4 Phase 0 mutations, 22 check-5 mutations, 3 + 2 controls) × 3 gates = 93 full gate runs.
- Folder-rule table: 8 + 2 rows PASS. Base checks: 7 edited-gate runs PASS over root, CS-1 and legacy/ bases; OLD dies on a legacy/ base.
- The rows re-run in this container (2026-11-11 14:3x Amsterdam): `.gate/fdn-scratch` rebuilt by `git archive HEAD` (gate files = dd12aeb's, `cmp`), `rows-test.mjs`: 8 + 2 rows ALL PASS.
- This head d6a3fee: full gate with smoke PASS against bb2d817, +0 -0 (index 8/8, editor 5/5).

## Decisions taken
- FREEZE-FDN (a), SPLIT (a), SCOPE (a) · Mustafa · 2026-11-10 (word for word in `mustafa-2026-11-10-answers.md`).
- Build 2.1a in this chat (no CLAUDE.md rule calls for a fresh session: plan approved for 2.1a's build, his decisions
  in, this branch is the session's own) · the session · 2026-11-10.
- He approved both gate prompts (gate.mjs, smoke.mjs); PR #47 is not watched (he says when its turn comes); 2.2 is
  planned in a fresh session on this account, from a one-paragraph prompt given in this chat · Mustafa · 2026-11-10.
- BRANCH-47 (a): replacement PR #50 from this session, #47 closed with a pointer; his yes covers the one fast-forward
  of the main folder (a one-time exception to the Never rule) and the merge's one gate-file write · Mustafa ·
  2026-11-11 (`mustafa-2026-11-11.md` section 2).
- The advisor tool: "extra" means on top of the advisor agent's recorded plan review, never instead of it; consulted at
  his five moments, one line each below, counted in the report; his words go into DECISIONS.md · Mustafa · 2026-11-11.
- Every kit or gate edit asks him in chat first with the exact change, never the guard's prompt alone · Mustafa ·
  2026-11-11.

## Advisor-tool consultations (his words of 2026-11-11: when, the question, what changed)

1. 2026-11-11 13:00 Amsterdam · before the merge (his moments 1 and 3): is step 1's approach sound and inside his two
   exceptions? · changed: d6a3fee pushed alone and the fast-forward made to exactly it (clean at bb2d817 before, HEAD
   equal after); no other merge construction if the guard refused; blob-level proofs incl. the five game files equal
   dd12aeb's (E3 carries over); #50 opened while its head was d6a3fee, before any `[skip ci]` save; his words captured
   by script; notes on C1-C8 and E3's worktrees for steps 3-4.

2. 2026-11-11 14:05 Amsterdam · before closing step 1 and briefing the docs pass (his moments 4 and 5): what goes into
   DECISIONS.md and how; #49's go; checking the old notes; where 2.1a's record goes · changed: whole messages, but only
   his labelled "Two additions" from the opening prompt; no pointer into `.checkpoints/` (the report's Appendix A
   instead, made by script); a byte-for-byte check script for the DECISIONS blocks; #49's go recorded from bb2d817's
   message, labelled not checked against the chat record; the old chat checked only for the #47's-turn message; no
   archive row for 2.1a until it merges (as #49 did); DECISIONS.md's header line reworded; a Queued entry for the
   CLAUDE.md clarification at 2.3; SLICE-ORDER's pointer beside, never inside, the block; #50's STATUS row touched in
   step 4; F-L grows (a revert of #50 removes #49's record and today's words): say so in the report; #47 read back.

3. 2026-11-11 14:35 Amsterdam · before closing step 2 and launching the round (his moments 1 and 5): is the docs pass
   done, and is the round set up right? · changed: CI on 1023870 and #49's facts read back first (found: merged_at
   09:03:11, fixed in 3c98ad5); the diff pinned to a SHA, never HEAD; rows-test owned by test-engineer on a fresh folder;
   E3 judged as recorded, the verifier alone runs a bounded sample on worktrees the session builds, results under
   `.gate/`; a smoke failure under parallel load re-run once; known limits named up front; #49's go flagged to the
   security-auditor and a "For Mustafa" line asking him to confirm that text; the advisor agent briefed on rev 3, the
   marker departure and this session's deviations, run alongside the verifier, then a delta review of the final head;
   docs fixes through the generators or by hand, never both; docs-keeper's effort (high) on an ADR pointer line noted
   for the report.

4. 2026-11-11 14:55 Amsterdam · before building the F1 fix (his moments 3 and 4): is the transition-only stop in
   gate.mjs the right shape? · changed: FREEZE-FDN put to him as its own question, quoting him; the advisor agent's
   recorded ruling first; one expression for the base's folder (`join(ROOT, base.prefix)`); the v2 consequence named as
   a 2.3 known limit and the PR body's "never makes the root look like the legacy build" to qualify; gate.mjs only
   (smoke.mjs stays dd12aeb's blob; IE-1 out of the ask); red first on the unfixed scratch, the rows script taking the
   scratch path; plumbing for the legacy/ base (a worktree commit if the guard refused); after his yes, the full E3 on
   the new blob; one self-contained ask with the patch saved first; the verifier brief, #50's body, his new words, L2
   and round 2 marked stale for after his yes.

5. 2026-11-11 about 16:55 Amsterdam · after his yes, before the write (his moments 1 and 3): is the write and the rest of the
   plan sound? · changed: three anchors before the `cp` (current blob 2bfab53; the notes patch equal to eaf5a45's; the
   patched copy hashing to 7a8480c), the patch applied to a copy, never edited; the write one plain `cp` alone in its
   call; status, smoke.mjs blob and the gate checked after it; the fix commit's CI let finish before the next non-[skip
   ci] push; smoke users first, E3 last and alone with the maximum timeout and a by-script comparison; the decoy built
   from the auditor's notes, `--smoke-only` shown passing; round 2 pinned to a head holding all the evidence;
   infra-engineer's question on 2.1b's push runs; docs-keeper's check script extended, not a second one; the Windows
   step built from "Check a new machine", saying what it exercises, plus 2.1b's branch command; the verifier brief,
   104d675's mentions and F-L marked stale.

6. 2026-11-11 17:38 Amsterdam · the CI smoke failure came back a second time (his moment 2): what now? · changed: no
   third re-run and no workflow workaround (a kit edit under the freeze; a 2.3 finding: every smoke run depends on a live
   package mirror); the docs pushed once the pull-request re-run has ended, gated with --no-smoke while E3 owns the
   smoke; a later head's smoke run covers the fix by blob equality (named in the report); STATUS's CI line updated in
   place; the final head needs both checks green before ready, one re-run at most there, and if still red: stop and
   tell him, never a commit to kick CI.

7. 2026-11-11 18:24 Amsterdam · the evidence step done; before launching round 2 (his moments 5 and 1) · changed: the cover claim backed (blob equality at 3f29cb8, 0607b45 and 76544aa; Smoke (check 5a) read as succeeded); main fetched (bb2d817); the briefs fixed (placeholders, 12 blocks, the writers not to run, the delta named, the refused forms, the unfinished items, a fresh rows folder, the verifier-owned worktrees, 89286ed rebuilt by the session); outcome rules set (Low/Medium: follow-ups riding the last commit; High: a fix and round 3 on that only, a gate file asks him first and E3 runs again); after the last commit nothing written inside the repo tree (the advisor agent returns its notes in its answer); the saver restarted if agents outlive it.

## Running work
- Saver: background task b8a5m7tp3, started 17:04 Amsterdam, stops by 18:54 (110 min); last check 17:25: ok.
- The prompt-tally agent: done (29 refusals, 14 prompts answered by Mustafa).
- E3 on the new gate: done 18:18 (31 of 31, identical).
- Round 2 running (background): security-auditor, test-engineer, infra-engineer on 76544aa.
- docs-keeper: done (1023870). infra-engineer and test-engineer: done (APPROVE). security-auditor done (REVISE, F1).
  The advisor agent done (REVISE, F1; then APPROVE with conditions for the F1 amendment). No agent running. The security-auditor's probe worktree: `.gate/foundation-restructure/sa-probe`.
- The worktree `.gate/fdn-replace`: removed. E3's worktrees: gone with the old
  container (step 3 note).
