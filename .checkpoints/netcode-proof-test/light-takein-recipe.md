# LIGHT-TAKEIN: the recipe (DRAFT rev 0.1, 2026-11-26 05:4x UTC, the main session)
**Not usable until the advisor agent's recorded check** (the advisor tool's read is an extra, not that check). His word:
history.md:8053 (LIGHT-TAKEIN (a), 2026-11-25 20:48 UTC), read there. It covers only a move of main whose new merges
touch only docs .md files (no CLAUDE.md, AGENTS.md, design doc or mark row), with main's CI green and at most one
conflict, STATUS.md's table. Anything outside: the full plan (MAIN-66's form and its check). Every command in its own
call; git and gh always start their own command, with no other program in front of them.

## Entry checks (read only; any miss: nothing merged, the full plan)
- E0. `git fetch origin main` in its own call; SHA = `git rev-parse origin/main`, on the card; BASE = `git merge-base
  HEAD <SHA>`; the pull requests = `git log --first-parent --format='%h %s' <BASE>..<SHA>`. No fetch until step 2.
- E1, docs .md only: `git diff --no-renames --name-only <BASE> <SHA> -- . ':(exclude,glob)docs/**/*.md'` prints nothing.
- E2, no kit-rule file: the same with `-- ':(glob)**/CLAUDE.md' ':(glob)**/CLAUDE.local.md' ':(glob)**/AGENTS.md'`.
- E3, no design or look doc (the widest reading: every one CLAUDE.md names, docs/design/, docs/reference/): the same
  with `-- docs/design/ docs/reference/ docs/LOOP_V2.md docs/EDITOR_V2.md docs/VISION_V2.md docs/LOOK_V2.md
  docs/GAME_DESIGN.md docs/DESIGN_NOTES.md docs/ART_DIRECTION.md docs/ANIMATION_PLAN.md docs/UNITS.md`.
- E4, no mark row (the widest reading: any `| PM-` row under docs/): `git diff --no-renames --name-only -G'^[|] PM-'
  <BASE> <SHA> -- docs/` prints nothing.
- E5, main's CI green: on SHA, the push runs of gate (jobs gate and smoke) and toolchain all completed with success
  (GitHub tools). Still running: wait. Any other conclusion: outside.
- E6, at most one conflict, in STATUS.md's table: `git merge-tree --write-tree --name-only HEAD <SHA>` exits 0, or 1
  with exactly one CONFLICT line, "CONFLICT (content): Merge conflict in docs/STATUS.md". Its tree T goes on the card
  (the anchor); in `git show <T>:docs/STATUS.md` every line between the markers starts with `|` (table rows only).
- E7, this branch's STATUS.md differs from main's only by its #40 row: `git diff --numstat <BASE> HEAD --
  docs/STATUS.md` prints "1 1" and its two lines are #40 rows; `git grep -n -F '| #40 |' <SHA> -- docs/STATUS.md`
  finds one row.
- E8, the lane: `git status --short` has no letter in its first column; no other unit's no-save window is open and no
  counted run of mine runs (the card's Running work); PR #40's rules event is in context (subscribe again if not).
- Tested on history 05:4x UTC (read only): MAIN-66's move 28254a7..c1da1bd and MAIN-67's c1da1bd..07d673b: E1 names
  code and CI paths (outside). #67 alone 311c048..c1da1bd: E1-E3 empty, E4 names docs/adr/0006 (PM-0006-2's
  revision): outside under the plain reading. #66 alone 28254a7..09faefa: E1-E4 empty (inside; E5-E8 not tried on
  history). E2 names CLAUDE.md on c9bd268, E3 names docs/VISION_V2.md on c934fc0. E7 today: "1 1" on BASE 07d673b.

## Steps (MAIN-66's 1-7 in the forms the guard passed there; every number derived, none pinned)
1. The resume line on the card by Edit (replacing); the saver stopped by TaskStop with its proof; H0 = HEAD, named in
   the chat; the base light run `bash .checkpoints/netcode-proof-test/main-run.sh .gate/light-<SHA7>/light-base.log node
   check.mjs --only refs,marks,self,redact` gives N0, the marks line, S0 and R0 with its items. The no-save window opens:
   no hand save, commit or push until step 6; a Stop hook gets one line.
2. `git rev-parse origin/main` prints SHA (else back to E0); the command named in the chat; `git merge origin/main`.
   Expected: exit 0 (git commits; the bars below use HEAD for `--cached`), or exit 1 with E6's one CONFLICT.
3. STATUS.md, by Edit: the block becomes main's side of it, main's #40 row replaced by H0's #40 row word for word;
   nothing else changes; no marker left; `git add docs/STATUS.md`. Bars: `git diff --cached --numstat <SHA> --
   docs/STATUS.md` prints "1 1" (or "0 0") and its lines are the #40 rows; `git diff --name-only --diff-filter=U` prints
   nothing; `git diff --cached --name-only H0` equals `git diff --name-only <BASE> <SHA>`; `git diff --cached --shortstat
   H0 -- . ':(exclude)docs/STATUS.md'` equals the same over <BASE> <SHA>; `git diff --cached --name-only H0 --
   .checkpoints/ spikes/` prints nothing; the anchor `git diff --cached --quiet <T> -- . ':(exclude).checkpoints/'
   ':(exclude)docs/STATUS.md'` exits 0.
4. `git commit --no-edit` (parents H0 and SHA, git's own message; the card save carries the trailers); the anchor again
   with HEAD.
5. a. `node tools/gate.mjs --no-smoke`: GATE PASS. b. `node .claude/scripts/repo-map-check.mjs`: "repo map OK", its
   numbers recorded. c. `node --test .claude/scripts/checkpoint.test.mjs`: 0 fail. d. the light run again: every count
   as step 1's except HEAD and refs, refs = N0 + A - D (`git diff --no-renames --name-status <BASE> <SHA>`). e.
   `git diff --quiet H0 HEAD -- .claude/ .github/ tools/` exits 0 (so MAIN-66's 5e, the guard tests, has nothing to read).
6. Main's CI on SHA read again; the card at 200 lines or fewer; the card save pushes; CI on the pushed head green (gate,
   smoke, toolchain; push and pull request); the saver restarts.
7. The advisor's read of the pushed head by SHA (his word: it stays per move), in a usage window's first 30 minutes or
   riding an agent already running.
8. His one line, in the chat at step 6 and in the day's message: "LIGHT-TAKEIN: main <BASE7>..<SHA7> taken in (#a, #b:
   docs only, CI green), merge <m7>, pushed <p7>; the advisor's read follows."

## Stops
- Before step 2: any miss means nothing merged; the full plan.
- From step 2 on: any exit or output not as written stops there (no `git merge --abort`, no reset, no other form); the
  merge stays; the advisor rules (main-61-plan.md:106-128's stop rules and risks, as at MAIN-64 to MAIN-67).
- A command refused or asking: M66-2's rule (prove nothing changed; tell him the command, (a) a prompt or (b) wait).

## Questions for the checker
- Q1, "design doc": E3 is the widest list I can name. Are docs/SPIKE_PLAN.md (item 1 is this test), V2_PHASE0_BRIEF.md,
  ENGINE_OVERVIEW.md or ADR text outside mark rows design docs? My lean: no (his words leave out four kinds only); the
  marks step and the head read are the second wall.
- Q2, "mark row": E4 reads any `| PM-` row under docs/. The narrower reading: only rows marks.json hashes. My lean: keep
  the widest (it put #67 outside).
- Q3: is a clean merge (no conflict) inside? My lean: yes, a subset of "the one conflict"; step 3's bars still hold.
- Q4: the line to him in the chat at step 6 and in the day's message, or only the day's message? My lean: both.
