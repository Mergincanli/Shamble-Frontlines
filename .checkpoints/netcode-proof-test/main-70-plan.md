# MAIN-70: take main dae6d107 (#79 cs4b-2, "The world") into the lane: the plan (rev 0.1, DRAFT for the check)
Status: DRAFT, 2026-11-28 02:0x UTC, for the advisor agent's check. Nothing below runs before it, except read-only reads.
Form: main-68-plan.md rev 4's steps 1-7 as main-69-plan.md uses them, only what differs; MAIN-68's STATUS.md rule for
the one conflict (main-68-plan.md:89-99).

## Why
- His words (MAIN-52, history.md:2055-2057, word for word there): "Take main in when main changes code, CI or rules".
  #79 changes code (crates/, packages/, content/ at the repository root).
- Neither lighter form fits: the light recipe covers only docs .md moves (light-takein-recipe.md:9-10), and
  TAKEIN-CODE (b) is for a code move between a checked plan and its merge (history.md:8168). So the full form.
- PR #40's rules event (re-sent 01:56 UTC): "Merge conflict: merge the base into the head" (GitHub reports a conflict
  since 23:12 UTC on 11-27).
- Stop B's next commits (a new Y0, Y and X after the dist ruling: stopb-dist-diff.md) are cut on the lane's tip, so
  MAIN-70 comes first (MAIN-52's order; stopb-plan.md A4).

## Facts (read 01:56-02:0x UTC, read only, printing forms; HEAD then c333b75)
- `git fetch origin main`: "b8f02b01..dae6d107 main -> origin/main". `git log --oneline --first-parent
  b8f02b0..origin/main`: one line, "dae6d107 Merge pull request #79: cs4b-2, The world (content reaches the match, the
  static collision world, the schema writer)". Its message (a record of the other lane, not his word here): merged at the
  expected head b7145273 on his go given in the Builder V6 chat; the advisor agent approved that head; gate, smoke and
  toolchain green on it.
- `git merge-base HEAD origin/main`: b8f02b0149cc928964e54e194cd9c4d68f52bd94 (main as MAIN-69 took it in).
- `git diff --raw --no-renames b8f02b0 origin/main`: 107 paths, every new mode 100644 (A 44, M 63; no link, submodule,
  mode change or deletion); `--shortstat`: "107 files changed, 10839 insertions(+), 408 deletions(-)". By area:
  crates/devtools 28, packages/client 15, packages/types 13, packages/sim-worker 13, crates/sim 8, crates/content 8,
  packages/e2e 4, crates/sim-wasm 4, content/source 3, docs 7.
- No kit, CI or gate path: `git diff --stat b8f02b0 origin/main -- CLAUDE.md .claude .github tools AGENTS.md
  CLAUDE.local.md` prints nothing. No spikes/ path. No design or look doc (light-takein-recipe.md E3's list prints
  nothing). No mark row: `git diff --no-renames --name-only -G'^[|] PM-' b8f02b0 origin/main -- docs/` prints nothing.
  Docs: DECISIONS.md, REPO_MAP.md, ROADMAP.md, STATUS.md, STATUS_ARCHIVE.md, adr/README.md,
  reports/2026-11-27-cs4b2-world.md.
- DEP-HOLD: main changes Cargo.toml, Cargo.lock and crates/{content,devtools,sim-wasm,sim}/Cargo.toml (the root
  workspace, not the spike's). Cargo.lock's diff adds 33 registry packages (`^+source = "registry`; approx, arrayvec,
  autocfg, bitflags, bytemuck, byteorder, downcast-rs, either, ...). This branch changes none of the root manifests
  (`git diff --name-only b8f02b0 HEAD -- Cargo.toml Cargo.lock crates packages package.json package-lock.json` prints
  nothing), so after the merge each equals main's byte for byte: DEP-HOLD's own exception ("not held when its content
  in the push equals its content on main", .claude/CHECKPOINTS.md §3; checkpoint.mjs:23-24, :135), MAIN-66's and
  MAIN-68's precedent. No dependency-check.md line for main's files. Unlike MAIN-68, registry packages enter (they were
  reviewed in #79's own lane); the spike's own manifests (spikes/netcode-proof/) are untouched.
- `git merge-tree --write-tree --name-only HEAD origin/main`: tree 54c70bb0b80664f978824b725e0b1884518e9e25, one
  CONFLICT line, "CONFLICT (content): Merge conflict in docs/STATUS.md"; auto-merging DECISIONS.md, REPO_MAP.md,
  STATUS.md and STATUS_ARCHIVE.md.
- STATUS.md's block (markers :33, :38, :41 by `git grep -n` on that tree): ours (:34-:37) this branch's #40 row, then
  "none yet | MAIN-WALL ...", "#75 (draft) | change-set 4b-1 ...", "#78 (draft) | CI-TIME ..."; theirs (:39-:40) main's
  #40 row and "none yet | change-set 4b-2, The world ...". Main dropped the three rows and added 4b-2's.
  - `git diff --numstat b8f02b0 HEAD -- docs/STATUS.md`: "1 1", the #40 row (this branch's only STATUS.md change).
  - `git diff --name-only -G'^[|] #40 [|]' b8f02b0 origin/main -- docs/STATUS.md` prints nothing: main's own #40 row is
    unchanged since b8f02b0, so the swap drops nothing of main's.
  - Main's STATUS.md: 120 lines (`git grep -c '' origin/main -- docs/STATUS.md`); `git diff --numstat b8f02b0
    origin/main -- docs/STATUS.md`: 15/15.
- Main's CI on dae6d107, its push runs, attempt 1, both success: gate 46081134384 (23:12:08-23:12:51) and toolchain
  46081134408 (23:12:08-23:22:01).
- The refs step (check.mjs:2695-2700) counts `git ls-files`: +44 added paths, so step 5d's refs = N0 + 44. The marks,
  self and redact steps read none of the 107 paths (no spikes/ path, no mark row).

## Steps (main-68-plan.md rev 4's steps 1-7; only what differs; each command its own call, printing forms)
1. As MAIN-69's step 1, with dae6d107fdbe942567104bcf1c9a2cbc6b3a1e0f for b8f02b0...: the resume line by Edit (C5 (a)'s
   form); usage read (figure and go on the resume line); the saver stopped by TaskStop, `pgrep -c -f checkpoint.mjs`
   printing 0; H0 named; `git fetch origin` alone, origin/main must be dae6d107 with its CI as in Facts (a further move:
   P-2 as widened by TAKEIN-CODE (b); else stop for a new check); the anchor T alone, `git merge-tree --write-tree
   --name-only H0 origin/main`, bar: exit 1, exactly the one CONFLICT line of Facts (anything else stops before step 2);
   T into this file by Edit; `mkdir -p /home/user/Shamble-Frontlines/.gate/main-70`; the base light run (`bash
   .checkpoints/netcode-proof-test/main-run.sh /home/user/Shamble-Frontlines/.gate/main-70/light-base.log node
   check.mjs --only refs,marks,self,redact`, run_in_background), giving N0, M0 and the self count.
2. Named in the chat first: `git merge origin/main`, after `git rev-parse origin/main` alone prints dae6d107...
   Expected: exit 1, "CONFLICT (content): Merge conflict in docs/STATUS.md", "Automatic merge failed"; `git status
   --short` shows `UU docs/STATUS.md` and main's other 106 paths staged (A 44, M 62). Refused before git runs: as
   MAIN-68's step 2.
3. STATUS.md by MAIN-68's rule (main-68-plan.md:89-94): the block becomes main's side (:39-:40), with main's #40 row
   replaced by H0's #40 row byte for byte (by Edit, reading the file first); no marker left.
4. The bars, after `git add docs/STATUS.md`, each alone (main-68-plan.md:95-99): `git grep -c -E
   '^(<<<<<<<|=======|>>>>>>>)' -- docs/STATUS.md` prints nothing; `wc -l docs/STATUS.md` = 120; `git diff --cached
   --numstat origin/main -- docs/STATUS.md` prints "1 1" and `-U0` shows main's #40 row out and H0's in; `git diff
   --cached --name-status origin/main` lists only docs/STATUS.md... (no: the branch's own files differ from main too;
   the bar is `git diff --cached --name-only H0` = main's 107 paths exactly, STATUS.md among them). Then `git commit
   --no-edit` (git's merge message).
5. a-c as MAIN-69's: GATE PASS (`node tools/gate.mjs --no-smoke`); "repo map OK" with its two numbers (#79 changes
   REPO_MAP.md); the checkpoint tests pass, 0 fail (no .claude/scripts/ file changes). The content checks, each alone:
   `git rev-parse HEAD^1 HEAD^2` prints H0 then dae6d107...; `git diff --name-only H0 HEAD` prints exactly main's 107
   paths; `git diff --stat H0 HEAD -- . ':(exclude)docs/STATUS.md'` equals `git diff --stat b8f02b0 origin/main --
   . ':(exclude)docs/STATUS.md'`'s last line; `git diff --name-only T HEAD` prints only docs/STATUS.md (the anchor holds
   the markers); d: the light run into .gate/main-70/light.log, every count equal to the base run's except HEAD, and
   refs = N0 + 44.
6. As MAIN-69's: main's CI on dae6d107 read again; `wc -l` on the card (200 or fewer); the card save pushes the merge
   (DEP-HOLD passes by the equal-to-main exception: before the save, alone, `git diff --name-only origin/main HEAD --
   Cargo.toml Cargo.lock crates packages package.json package-lock.json` prints nothing); CI on the pushed head, push
   and pull request: gate (gate, smoke) and toolchain green (the toolchain job now builds #79's root crates); the saver
   restarts.
7. The advisor's check of the pushed head by SHA (MAIN-65 step 7's form).
- One line to him (MAIN-52's record): "main dae6d107 taken in (#79 cs4b-2: the world, 107 files, code and docs; the
  STATUS.md conflict resolved by MAIN-68's rule), plan first (MAIN-70)."

## Order
- MAIN-70 runs while the dist ruling waits (stopb-dist-diff.md): no build or counted run of mine runs during steps 1-6
  (the path-A rebuild of the diagnosis finishes first). Worktrees .gate/stopb-a and .gate/stopb-b stay (they are at Y and
  unaffected); no worktree removal between steps 1 and 6.

## Questions for the checker
- Q1: step 4's whole-tree bar: is `git diff --cached --name-only H0` = main's 107 paths the right one (main-68's form)?
- Q2: DEP-HOLD with 33 registry packages entering: is the equal-to-main exception enough, or is a security read owed
  for the lane (the spike builds none of them; CI's toolchain job does)?
- Q3: the guard tests (MAIN-68's e): skipped, as #79 changes no .claude/ file?

## Change type, tier, size
Process (a merge of main's reviewed content), risk tier 1 (no kit, CI or gate file), size: main's 107 paths; the lane's
own lines: STATUS.md's #40 row kept (0 net), the card.
