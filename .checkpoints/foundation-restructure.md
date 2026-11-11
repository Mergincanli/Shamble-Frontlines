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
- **Last update:** 2026-11-11 14:20 Amsterdam

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

## Current step

His step 2, the docs pass: docs-keeper (Opus, effort high) briefed with the facts below; the saver running.

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
- (none yet)

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

## Running work
- Saver: background task bjteqwe4t, started 14:02 Amsterdam, stops by 15:52 (110 min); last check 14:35: ok (watching).
- docs-keeper: the docs pass, running in the background (notes `docs-keeper-docs-pass.md`).
- The worktree `.gate/fdn-replace`: removed. E3's worktrees: gone with the old
  container (step 3 note).
