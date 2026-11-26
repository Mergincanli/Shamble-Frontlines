# LIGHT-TAKEIN: the recipe (rev 0.3, as checked)
As checked: APPROVE WITH CONDITIONS L-1 to L-12 (advisor-light-takein.md), applied word for word; no new check; the
advisor's read of each use's pushed head stays (step 7). His word: history.md:8053 (LIGHT-TAKEIN (a), 2026-11-25 20:48
UTC), read there.
- LT-CLEAN (a), his, 2026-11-26 17:35 UTC (history.md:8108-8121, word for word there): a move that merges with no
  conflict is inside, by the clean path under step 2. Rev 0.2 was the main session's draft (05:5x UTC); rev 0.3 applies
  L-1 to L-12 (18:4x UTC); the checker's own text of each: advisor-light-takein.md section 2.

It covers only a move of main whose new merges touch only docs .md files (no CLAUDE.md, AGENTS.md, design doc or mark
row), with main's CI green and at most one conflict, STATUS.md's table. Anything outside: the full plan (MAIN-66's form
and its check). Every command in its own call; git and gh always start their own command, with no other program in front
of them.

## Entry checks (read only; any miss: nothing merged, the full plan)
- E0. `git fetch origin main` in its own call; SHA = `git rev-parse origin/main`, on the card; BASE = `git merge-base
  HEAD <SHA>`; the pull requests = `git log --first-parent --format='%h %s' <BASE>..<SHA>`. No other fetch before step 6
  (step 2 re-reads the local ref only). If the log prints nothing, nothing is due: stop.
- E1, docs .md only: `git diff --no-renames --name-only <BASE> <SHA> -- . ':(exclude,glob)docs/**/*.md'` prints nothing.
- E1b: `git diff --raw --no-renames <BASE> <SHA>` shows 100644 as every new mode, or 000000 for a deleted file: no link
  (120000), no submodule (160000), no mode change.
- E2, no kit-rule file: the same with `-- ':(glob)**/CLAUDE.md' ':(glob)**/CLAUDE.local.md' ':(glob)**/AGENTS.md'
  ':(glob)**/.claude/**'`.
- E3, no design or look doc (the widest reading: every one CLAUDE.md names, every design and look doc main's advisor.md
  names, docs/design/, docs/reference/): the same with `-- docs/design/ docs/reference/ docs/LOOP_V2.md docs/EDITOR_V2.md
  docs/VISION_V2.md docs/V2_PHASE0_BRIEF.md docs/LOOK_V2.md docs/GAME_DESIGN.md docs/DESIGN_NOTES.md
  docs/ART_DIRECTION.md docs/ANIMATION_PLAN.md docs/UNITS.md`.
- E4, no mark row (the widest reading: any `| PM-` row under docs/): `git diff --no-renames --name-only -G'^[|] PM-'
  <BASE> <SHA> -- docs/` prints nothing.
- E1-E4 also run on each first-parent merge M that E0's log lists, with `<M>^1 <M>` in place of `<BASE> <SHA>`, each
  command in its own call; any name printed is a miss (his words are per merge: "main's new merges touch only docs .md
  files").
- E5, main's CI green: on SHA, the push runs of gate (jobs gate and smoke) and toolchain all completed with success
  (GitHub tools). Still running: wait. Any other conclusion: outside.
- E6, at most one conflict, in STATUS.md's table: `git merge-tree --write-tree --name-only HEAD <SHA>` exits 0, or 1
  with exactly one CONFLICT line (until his answer to LT-CLEAN, exit 0 is a miss: the full plan), "CONFLICT (content):
  Merge conflict in docs/STATUS.md"; in `git show <its tree>:docs/STATUS.md` every line between the markers starts with
  `|` (table rows only) and `git grep -c -E '^<<<<<<<' <its tree> -- docs/STATUS.md` prints a count of 1 (one block;
  with exit 0 there is none). The lines between the markers are found with `git grep -n -E '^(<<<<<<<|=======|>>>>>>>)'
  <its tree> -- docs/STATUS.md`. This tree is not the anchor: merge-tree writes the revisions as spelled into
  STATUS.md's marker lines, so the tree's id depends on the spelling (05:5x: b518460 with 07d673b gives a2f8c1ac;
  MAIN-67's full H0 with origin/main gives its recorded 151318fe; the two differ only in those two lines). The anchor T
  is taken at step 1.
- E7, this branch's STATUS.md differs from main's only by its #40 row: `git diff --numstat <BASE> HEAD --
  docs/STATUS.md` prints "1 1" and its two lines are #40 rows; `git grep -n -F '| #40 |' <SHA> -- docs/STATUS.md`
  finds one row; and `git diff --name-only -G'^[|] #40 [|]' <BASE> <SHA> -- docs/STATUS.md` prints nothing (main's own
  #40 row is unchanged since BASE, so step 3's swap drops nothing of main's).
- E8, the lane: `git status --short` has no letter in its first column; no other unit's no-save window is open and no
  counted run of mine runs (the card's Running work); PR #40's rules event is in context (subscribe again if not). A
  miss here means wait, and start again at E0 when the lane is free. Only a miss in E1-E7 sends the move to the full
  plan.
- Tested on history 05:4x UTC (read only): MAIN-66's move 28254a7..c1da1bd and MAIN-67's c1da1bd..07d673b: E1 names
  code and CI paths (outside). #67 alone 311c048..c1da1bd: E1-E3 empty, E4 names docs/adr/0006 (PM-0006-2's
  revision): outside under the plain reading. #66 alone 28254a7..09faefa: E1-E4 empty (inside; E5-E8 not tried on
  history). E2 names CLAUDE.md on c9bd268, E3 names docs/VISION_V2.md on c934fc0. E7 today: "1 1" on BASE 07d673b.

## Steps (MAIN-66's 1-7 in the forms the guard passed there; every number derived, none pinned)
1. The resume line on the card by Edit (replacing), word for word: "LIGHT-TAKEIN (light-takein-recipe.md as checked,
   advisor-light-takein.md), running steps 1-8 for main <SHA>. If `git rev-parse -q --verify MERGE_HEAD` prints <SHA>,
   the merge is in progress: no save, no `git merge --abort`, no reset. If `git diff --name-only --diff-filter=U` names
   docs/STATUS.md, resume at step 3; otherwise resume at step 3's bars, then step 4. If HEAD^2 is <SHA> and HEAD is not
   on the remote yet, resume at step 5. The card may show unstaged: leave it; step 6's card save takes it. Anything
   else: the advisor rules."; the saver stopped by TaskStop with its proof; H0 = HEAD, named in the chat; the anchor T =
   the first line of `git merge-tree --write-tree --name-only <H0, full> origin/main` (this spelling only), on the card,
   with E6's shape checks again on it; the base light run `bash .checkpoints/netcode-proof-test/main-run.sh
   .gate/light-<SHA7>/light-base.log node check.mjs --only refs,marks,self,redact` gives N0, the marks line, S0 and R0
   with its items. The no-save window opens: no hand save, commit or push until step 6; a Stop hook gets one line.
2. `git rev-parse origin/main` prints SHA (else back to E0); the command named in the chat; `git merge origin/main`.
   Expected: exit 1 with E6's one CONFLICT; exit 0 only once he has answered LT-CLEAN (a), then as the clean path below.

   **The clean path (only after his LT-CLEAN (a)):** If step 2 exits 0, git has made the merge commit. Skip step 3's
   Edit and `git add`, and skip step 4's `git commit --no-edit`; it would exit 1, since nothing is left to commit. Run
   each of these in its own call, every diff naming both commits in this order:
   - `git rev-parse HEAD^1` prints H0, and `git rev-parse HEAD^2` prints SHA;
   - `git diff --numstat <SHA> HEAD -- docs/STATUS.md` prints "1 1", and its lines are the #40 rows;
   - `git diff --name-only H0 HEAD` equals `git diff --name-only <BASE> <SHA>`;
   - `git diff --shortstat H0 HEAD -- . ':(exclude)docs/STATUS.md' ':(exclude).checkpoints/'` equals `git diff
     --shortstat <BASE> <SHA> -- . ':(exclude)docs/STATUS.md'`;
   - L-8's numstat, as `git diff --numstat H0 HEAD -- . ':(exclude)docs/STATUS.md' ':(exclude).checkpoints/'`;
   - `git diff --name-only H0 HEAD -- spikes/` prints nothing;
   - the anchor `git diff --quiet <T> HEAD -- . ':(exclude).checkpoints/' ':(exclude)docs/STATUS.md'` exits 0.

   Never put HEAD in the place of `--cached`: that reverses the diff. If git opens an editor or waits, stop; the
   advisor rules.
3. STATUS.md, by Edit: the block becomes main's side of it, main's #40 row replaced by H0's #40 row word for word;
   nothing else changes; no marker left; `git add docs/STATUS.md`. Bars: `git diff --cached --numstat <SHA> --
   docs/STATUS.md` prints "1 1" and its lines are the #40 rows; `git diff --name-only --diff-filter=U` prints nothing;
   `git diff --cached --name-only H0` equals `git diff --name-only <BASE> <SHA>`; `git diff --cached --shortstat H0 -- .
   ':(exclude)docs/STATUS.md'` equals the same over <BASE> <SHA>; `git diff --cached --numstat H0 -- .
   ':(exclude)docs/STATUS.md'` equals `git diff --numstat <BASE> <SHA> -- . ':(exclude)docs/STATUS.md'`, line for line;
   `git diff --cached --name-only H0 -- .checkpoints/ spikes/` prints nothing; the anchor `git diff --cached --quiet <T>
   -- . ':(exclude).checkpoints/' ':(exclude)docs/STATUS.md'` exits 0.
4. `git commit --no-edit` (parents H0 and SHA, git's own message; the card save carries the trailers); the anchor again
   with HEAD.
5. a. `node tools/gate.mjs --no-smoke`: GATE PASS. b. `node .claude/scripts/repo-map-check.mjs`: "repo map OK", its
   numbers recorded. c. `node --test .claude/scripts/checkpoint.test.mjs`: 0 fail. d. the light run again: every count
   as step 1's except HEAD and refs, refs = N0 + A - D (`git diff --no-renames --name-status <BASE> <SHA>`). e.
   `git diff --quiet H0 HEAD -- .claude/ .github/ tools/` exits 0 (so MAIN-66's 5e, the guard tests, has nothing to read).
   f. `git diff --quiet <SHA> HEAD -- . ':(exclude)spikes/' ':(exclude).checkpoints/' ':(exclude)docs/'` exits 0, and
   `git diff --name-only <SHA> HEAD -- docs/` prints the same names as `git diff --name-only <BASE> H0 -- docs/`
   (MAIN-66's C1 (c): PR #40 stays risk tier 1).
6. Main's CI on SHA read again; the card at 200 lines or fewer; the card save pushes; CI on the pushed head green (gate,
   smoke, toolchain; push and pull request); the saver restarts. A move of main after E0 changes nothing for this use:
   no fetch before step 6. Read `git ls-remote origin refs/heads/main` at step 6. If it names a newer tip, step 8's line
   ends "; main has moved on to <new7>: its own E0 follows step 7". Pull-request runs test PR #40's head merged with
   main's tip of that moment, so a red pull-request run is read against main's new tip first, and the advisor rules.
7. The advisor's read of the pushed head by SHA (his word: it stays per move), in advisor-main-66.md Q1 (d)'s form with
   this use's SHAs (H0, the merge, the pushed card save), its toolchain baseline main's toolchain job on SHA; in a usage
   window's first 30 minutes or riding an agent already running. Once step 6's CI is green, the lane may go on, on that
   file's Q1 (a)-(f) terms; a step-7 finding above Low stops new lane steps until the advisor rules.
8. His one line, in the chat at step 6 and in the day's message: "LIGHT-TAKEIN: main <BASE7>..<SHA7> taken in (#a, #b:
   docs only, CI green), merge <m7>, pushed <p7>; the advisor's read follows."

## Stops
- Before step 2: any miss means nothing merged; the full plan.
- From step 2 on: any exit or output not as written stops there (no `git merge --abort`, no reset, no other form); the
  merge stays; the advisor rules (main-61-plan.md:106-128's stop rules and risks, as at MAIN-64 to MAIN-67).
- A command refused or asking: M66-2's rule (prove nothing changed; tell him the command, (a) a prompt or (b) wait).

## Questions for the checker (answered: advisor-light-takein.md section 3; Q3 by his LT-CLEAN (a))
- Q1, "design doc": E3 is the widest list I can name. Are docs/SPIKE_PLAN.md (item 1 is this test), V2_PHASE0_BRIEF.md,
  ENGINE_OVERVIEW.md or ADR text outside mark rows design docs? My lean: no (his words leave out four kinds only); the
  marks step and the head read are the second wall.
- Q2, "mark row": E4 reads any `| PM-` row under docs/. The narrower reading: only rows marks.json hashes. My lean: keep
  the widest (it put #67 outside).
- Q3: is a clean merge (no conflict) inside? My lean: yes, a subset of "the one conflict"; step 3's bars still hold.
- Q4: the line to him in the chat at step 6 and in the day's message, or only the day's message? My lean: both.
