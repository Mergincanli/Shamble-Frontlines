# Advisor: LIGHT-TAKEIN recipe check (light-takein-recipe.md, DRAFT rev 0.2)
2026-11-26, 14:5x-15:2x UTC window. Advisor agent (general-purpose, opus, effort xhigh), day session, read only.
I acted by main's advisor.md (read whole in this run).
- Recipe: light-takein-recipe.md, sha256 133f3ee5df82ff62b64add0322cb38869e15979a7b4469c0c95e28f091038a84.
- His word: LIGHT-TAKEIN (a), history.md:8053 (2026-11-25 20:48 UTC), word for word:
  > LIGHT-TAKEIN: (a). Write the recipe once and have the advisor check it. It is used only inside the limits you
  > named: main's new merges touch only docs .md files (no CLAUDE.md, AGENTS.md, design doc or mark row), main's CI is
  > green, and the one conflict is STATUS.md's table. The advisor's read of the pushed result stays per move, and I get
  > one line per use naming the pull requests taken in. Anything outside falls back to the full plan.
- The limits he points to ("the limits you named") are answer-1914.md:16-17, the same three. Behind them is the lean
  in advisor-main-66.md:181-210.

## 1. Verdict: APPROVE WITH CONDITIONS L-1 to L-12
The recipe keeps each of his limits, and every number in it is derived, not pinned:
- E1-E4 are the docs-only limit;
- E5 is main's CI;
- E6 is the one conflict;
- step 7 is the advisor's read per move;
- step 8 is his one line;
- "Stops" is the fall-back to the full plan.

Twelve fixes, each worded below to apply word for word:
- **One High (L-1).** E3's list of design docs leaves out docs/V2_PHASE0_BRIEF.md, which main's advisor.md counts
  among "the design and look docs". As written, the recipe would take one in by the light path, which is looser than
  his words as main's rules read them.
- **Three Medium:**
  - L-2: his limit is per merge, and E1-E4 read only the whole move;
  - L-3: E6 lets two conflict blocks through, and step 3's rule is written for one;
  - L-4: the clean-merge path stops at step 4 by its own rule, and "HEAD in place of `--cached`" reverses its diffs.
    It is worded out, and it waits for his answer (LT-CLEAN).
- **The rest are Low.**
- **One question for him (LT-CLEAN):** is a merge with no conflict inside his limit? Until he answers, the recipe
  treats it as outside (L-4).

**The recipe stays unusable until all twelve are applied word for word, under an "As checked" header (L-10).** Then it
needs no new check. Step 7's read of each use's pushed head stays, as he said.

His word does not cover main's move to 4a62873 (#73, two guard files): that move takes the full plan
(advisor-main-68.md).

## 2. Conditions (paste-ready; each names where it goes)
- **L-1 (High): E3 gains V2_PHASE0_BRIEF.md.**
  - Edit: in E3, put ` docs/V2_PHASE0_BRIEF.md` after `docs/VISION_V2.md`.
  - Edit: the parenthesis becomes "(the widest reading: every one CLAUDE.md names, every design and look doc main's
    advisor.md names, docs/design/, docs/reference/)".
  - Why: main's advisor.md, Risk tiers, risk tier 0 bullet: "the design and look docs (`VISION_V2`, `LOOP_V2`,
    `LOOK_V2`, `EDITOR_V2`, `V2_PHASE0_BRIEF`, `DESIGN_NOTES`, `ART_DIRECTION`, `GAME_DESIGN`, `UNITS`)". E3 has all of
    them but V2_PHASE0_BRIEF.
- **L-2 (Medium): E1-E4 per merge.** Add after E4: "E1-E4 also run on each first-parent merge M that E0's log lists,
  with `<M>^1 <M>` in place of `<BASE> <SHA>`, each command in its own call; any name printed is a miss (his words are
  per merge: "main's new merges touch only docs .md files")."
  - Why: a code change that a later merge undid nets to nothing in `<BASE> <SHA>`, yet that merge touched code.
  - The take-in's content would still be docs only, so the reach is small. Only the reading of his words is looser.
- **L-3 (Medium): one block only.** Add to E6, before "This tree is not the anchor": "and `git grep -c -E '^<<<<<<<'
  <its tree> -- docs/STATUS.md` prints a count of 1 (one block; with exit 0 there is none). The lines between the
  markers are found with `git grep -n -E '^(<<<<<<<|=======|>>>>>>>)' <its tree> -- docs/STATUS.md`."
  - Why: git prints one CONFLICT line per file, not per block. Step 3 says "the block", and his words say "the one
    conflict".
- **L-4 (Medium): the clean path, held until his answer (LT-CLEAN, Decisions needed).**
  - Edit, in E6: after "exits 0, or 1 with exactly one CONFLICT line", add: "(until his answer to LT-CLEAN, exit 0 is
    a miss: the full plan)".
  - Edit, in step 2: replace "Expected: exit 0 (git commits; the bars below use HEAD for `--cached`), or exit 1 with
    E6's one CONFLICT." with "Expected: exit 1 with E6's one CONFLICT; exit 0 only once he has answered LT-CLEAN (a),
    then as the clean path below."
  - Add after step 2, as "The clean path (only after his LT-CLEAN (a))": "If step 2 exits 0, git has made the merge
    commit. Skip step 3's Edit and `git add`, and skip step 4's `git commit --no-edit`; it would exit 1, since nothing
    is left to commit. Run each of these in its own call, every diff naming both commits in this order:
    - `git rev-parse HEAD^1` prints H0, and `git rev-parse HEAD^2` prints SHA;
    - `git diff --numstat <SHA> HEAD -- docs/STATUS.md` prints "1 1", and its lines are the #40 rows;
    - `git diff --name-only H0 HEAD` equals `git diff --name-only <BASE> <SHA>`;
    - `git diff --shortstat H0 HEAD -- . ':(exclude)docs/STATUS.md' ':(exclude).checkpoints/'` equals `git diff
      --shortstat <BASE> <SHA> -- . ':(exclude)docs/STATUS.md'`;
    - L-8's numstat, as `git diff --numstat H0 HEAD -- . ':(exclude)docs/STATUS.md' ':(exclude).checkpoints/'`;
    - `git diff --name-only H0 HEAD -- spikes/` prints nothing;
    - the anchor `git diff --quiet <T> HEAD -- . ':(exclude).checkpoints/' ':(exclude)docs/STATUS.md'` exits 0.

    Never put HEAD in the place of `--cached`: that reverses the diff. If git opens an editor or waits, stop; the
    advisor rules."
  - Edit, in step 3's first bar: delete ` (or "0 0")`. With E7 and L-6, the bar is always "1 1". With no difference,
    numstat prints nothing; it never prints "0 0".
  - Why the order matters: on MAIN-67's merge, `git diff 07cc9b7 --shortstat b518460 -- .
    ':(exclude)docs/STATUS.md' ':(exclude).checkpoints/'` (HEAD in place of `--cached`) printed "7 files changed, 17
    insertions(+), 543 deletions(-)". That is main's 543 / 17, swapped, so the bar would fail.
- **L-5 (Low): modes.** Add after E1: "E1b: `git diff --raw --no-renames <BASE> <SHA>` shows 100644 as every new
  mode, or 000000 for a deleted file: no link (120000), no submodule (160000), no mode change." (advisor.md asks for the
  raw read; E1 reads names only.)
- **L-6 (Low): main's own #40 row.** Add to E7: "and `git diff --name-only -G'^[|] #40 [|]' <BASE> <SHA> --
  docs/STATUS.md` prints nothing (main's own #40 row is unchanged since BASE, so step 3's swap drops nothing of
  main's)."
- **L-7 (Low): `.claude/` at any depth.** In E2, add ` ':(glob)**/.claude/**'` to the pathspec list.
  - Why: E1's `docs/**/*.md` also passes a Markdown file under a `.claude/` folder inside docs/. Git's glob does not
    treat leading dots specially; that is my reading of git's pathspec rules, not tested here.
  - A `.claude/` folder at any depth is risk tier 2 (advisor.md).
- **L-8 (Low): the auto-merged docs, file by file.** Add to step 3's bars: "`git diff --cached --numstat H0 -- .
  ':(exclude)docs/STATUS.md'` equals `git diff --numstat <BASE> <SHA> -- . ':(exclude)docs/STATUS.md'`, line for line".
  This is MAIN-66's both-ways numstat (advisor-main-66.md, notes :328-330). The shortstat bar stays.
- **L-9 (Low): the whole tree after the merge.** Add to step 5: "f. `git diff --quiet <SHA> HEAD -- .
  ':(exclude)spikes/' ':(exclude).checkpoints/' ':(exclude)docs/'` exits 0, and `git diff --name-only <SHA> HEAD --
  docs/` prints the same names as `git diff --name-only <BASE> H0 -- docs/` (MAIN-66's C1 (c): PR #40 stays risk tier
  1)."
- **L-10 (Low): the header and the resume line.**
  - Replace the recipe's first two lines with: "# LIGHT-TAKEIN: the recipe (rev 0.3, as checked)" and "As checked:
    APPROVE WITH CONDITIONS L-1 to L-12 (advisor-light-takein.md), applied word for word; no new check; the advisor's
    read of each use's pushed head stays (step 7). His word: history.md:8053 (LIGHT-TAKEIN (a), 2026-11-25 20:48 UTC),
    read there."
  - Step 1's resume line, word for word: "LIGHT-TAKEIN (light-takein-recipe.md as checked, advisor-light-takein.md),
    running steps 1-8 for main <SHA>. If `git rev-parse -q --verify MERGE_HEAD` prints <SHA>, the merge is in progress:
    no save, no `git merge --abort`, no reset. If `git diff --name-only --diff-filter=U` names docs/STATUS.md, resume at
    step 3; otherwise resume at step 3's bars, then step 4. If HEAD^2 is <SHA> and HEAD is not on the remote yet,
    resume at step 5. The card may show unstaged: leave it; step 6's card save takes it. Anything else: the advisor
    rules."
- **L-11 (Low): steps 6 and 7.**
  - Add to step 6: "A move of main after E0 changes nothing for this use: no fetch before step 6. Read `git ls-remote
    origin refs/heads/main` at step 6. If it names a newer tip, step 8's line ends "; main has moved on to <new7>: its
    own E0 follows step 7". Pull-request runs test PR #40's head merged with main's tip of that moment, so a red
    pull-request run is read against main's new tip first, and the advisor rules."
  - Step 7 reads: "The advisor's read of the pushed head by SHA (his word: it stays per move), in advisor-main-66.md Q1
    (d)'s form with this use's SHAs (H0, the merge, the pushed card save), its toolchain baseline main's toolchain job
    on SHA; in a usage window's first 30 minutes or riding an agent already running. Once step 6's CI is green, the lane
    may go on, on that file's Q1 (a)-(f) terms; a step-7 finding above Low stops new lane steps until the advisor
    rules."
- **L-12 (Low): E0 and E8.**
  - In E0, "No fetch until step 2." becomes "No other fetch before step 6 (step 2 re-reads the local ref only). If the
    log prints nothing, nothing is due: stop."
  - Add to E8: "A miss here means wait, and start again at E0 when the lane is free. Only a miss in E1-E7 sends the
    move to the full plan."

## 3. The recipe's questions
- **Q1. Which other docs count as design docs?**
  - **docs/V2_PHASE0_BRIEF.md is one: L-1.**
  - **docs/SPIKE_PLAN.md, docs/ENGINE_OVERVIEW.md and ADR text outside mark rows are not; they stay inside.**
    - Neither his words nor main's lists call them design docs. CLAUDE.md calls ENGINE_OVERVIEW a decision draft.
    - For the ADRs, his words name only "mark row". That leaves their other text inside, while E4 keeps every `| PM-`
      row out.
  - The second wall is step 7. It reads each hunk in docs/SPIKE_PLAN.md and docs/adr/ by name. A hunk inside SPIKE_PLAN
    item 1 (this test's entry) goes to the advisor's ruling before the lane's next step.
- **Q2. Which reading of "mark row"? Keep the widest, as written** (any `| PM-` row under docs/).
  - It is stricter than the rows marks.json hashes (check.mjs:1869-1874 and :1880-1906 hash PM-0001-n, PM-0004-67 and
    PM-0006-11 to -27).
  - Stricter costs only a full plan, never a wrong take-in.
  - It put #67 outside: I re-ran E4 on 311c048..c1da1bd, and it names docs/adr/0006-infrastructure-operations.md.
- **Q3. Is a clean merge inside? His words do not settle it, so it goes to him (LT-CLEAN).**
  - My lean is yes: "the one conflict is STATUS.md's table" reads as an upper bound, a clean merge has no conflict
    outside that table, and it needs less hand work (no Edit at all).
  - His words also read as "exactly one conflict". I hold the recipe to his exact words, as in L-1 and L-2, so until
    he answers, exit 0 is a miss and the move takes the full plan (L-4's first two edits). That costs nothing today:
    every move so far conflicted in STATUS.md.
  - On his (a), L-4's clean path is used as worded. As first written, that path stopped at step 4, and its "HEAD in
    place of `--cached`" diffs would have read backwards.
- **Q4. Where does the line to him go? Both:** the chat at step 6, and the day's message.
  - It is the same one line in each, which fits "one line per use": the chat copy is the one he sees when it happens,
    and the day's message is the record.
  - Send no second message per use.

## 4. What I verified myself (read only)
| Claim (recipe) | My command and result |
|---|---|
| #67 alone (311c048..c1da1bd): E1 empty, E4 names docs/adr/0006 | E1 printed nothing; E4 printed docs/adr/0006-infrastructure-operations.md |
| #66 alone (28254a7..09faefa): E1-E4 empty | E1 and E4 printed nothing (E2 and E3 not re-run) |
| E7 today: "1 1" on BASE 07d673b | `git diff --numstat 07d673b HEAD -- docs/STATUS.md` printed "1 1" |
| (negative) #70 is outside | E1 on 07d673b..2483e9a printed 32 crates/ and packages/ paths |
| (negative) #73 is outside | by GitHub's file list, not by command (its objects are not local): E1 would print .claude/hooks/routine-guard.mjs and its test |
| E3 is "the widest list" | not so: main's advisor.md names V2_PHASE0_BRIEF too (L-1) |
| (new, L-3) one block | `git grep -c -E "^<<<<<<<" 079d8cdd… -- docs/STATUS.md` on today's trial tree printed ":1" |
| (new, L-6) main's own #40 row | `-G'^[|] #40 [|]'` on 07d673b HEAD named docs/STATUS.md (the branch's row changed); on 07d673b 2483e9a it printed nothing (main's row unchanged) |
| (new, L-4) the bars' direction | `git diff 07cc9b7 --shortstat b518460 -- . ':(exclude)docs/STATUS.md' ':(exclude).checkpoints/'` printed "17 insertions(+), 543 deletions(-)": reversed |
- Not re-run: E2 on c9bd268, E3 on c934fc0, and E6's 05:5x tree ids (a2f8c1ac, 151318fe).
- The recipe's step forms are MAIN-66's and MAIN-67's (main-66-plan.md steps 1-7; advisor-main-67.md M67-1 and
  M67-2), which advisor-main-67-head.md saw pass.
- I ran none of the recipe's write steps.

## 5. Findings (CLAUDE.md's scale)
| Id | Severity | What | Fix |
|---|---|---|---|
| ADV-LT-1 | High | E3 leaves out docs/V2_PHASE0_BRIEF.md, which main's advisor.md counts among the design and look docs. The recipe would take one in by the light path, looser than his "no … design doc" as main's rules read it. | L-1 |
| ADV-LT-2 | Medium | E1-E4 read only `<BASE> <SHA>`, but his limit is per merge. A merge that touched code and was undone later would pass. The content taken in stays docs only, so the reach is small. | L-2 |
| ADV-LT-3 | Medium | E6 counts CONFLICT lines (one per file), so two blocks in STATUS.md pass. Step 3 is written for one block. | L-3 |
| ADV-LT-4 | Medium | On a clean merge, git commits at step 2, and step 4's `git commit --no-edit` exits 1, so the recipe's own stop rule fires. Step 2's "the bars below use HEAD for `--cached`" reverses the diffs (tested: 17 / 543 for 543 / 17). Step 3's "(or "0 0")" cannot occur. Whether a clean merge is inside at all is his (LT-CLEAN). | L-4 |
| ADV-LT-5 | Low | No raw-mode read: a link or submodule under docs/ named `.md` would pass E1. | L-5 |
| ADV-LT-6 | Low | A change on main to its own #40 row would be dropped silently by step 3's swap. | L-6 |
| ADV-LT-7 | Low | A `.claude/` folder inside docs/ would pass E1 and E2. | L-7 |
| ADV-LT-8 | Low | The auto-merged docs are checked only by the total line counts, not file by file. | L-8 |
| ADV-LT-9 | Low | No whole-tree check after the merge (MAIN-66's C1 (c)). | L-9 |
| ADV-LT-10 | Low | No "As checked" header, and the resume line is not written out. | L-10 |
| ADV-LT-11 | Low | No rule for main moving after E0, which is today's case. Step 7's form is not named, nor what the lane may do before it. | L-11 |
| ADV-LT-12 | Low | An empty E0 log is not handled, and E8's miss (the lane busy) falls to the full plan where waiting is enough. | L-12 |

## 6. What I did not check
- No use of the recipe was run. Nothing was merged, fetched or saved.
- E1's glob on a dotted folder: by reading only (L-7 closes it either way).
- Whether `git merge` would open an editor on a clean merge in this tool's shell. I expect it does not without a
  terminal; L-4's last sentence covers it if it does.

## Notes (one line per check, as run)
- Read: the recipe (whole), history.md:8027-8066, answer-1914.md, advisor-main-66.md section 3 Q4 (:174-210),
  advisor.md (main) Risk tiers, check.mjs :1852-1906 and :2625-2641.
- E1/E4 re-runs on 311c048..c1da1bd and 28254a7..09faefa; E7 on 07d673b; E1 on 07d673b..2483e9a (32 paths).
- #73's files, by get_commit (GitHub): two .claude/hooks files and six docs paths. E1 would name the first two.

## Decisions needed
- **LT-CLEAN: may the lighter take-in also be used when main's move merges with no conflict at all?**
  - Why it matters: your limit says "the one conflict is STATUS.md's table". A move with no conflict is either inside
    that limit or a case for the full plan, and the recipe must not guess.
  - Options:
    - (a) Inside. A clean merge is used by the recipe's clean path, checked by the same bars.
    - (b) Outside. A move with no conflict takes the full plan, as today.
  - My lean: (a). A clean merge needs less hand work than the one-conflict case, and every bar still holds. Until you
    answer, the recipe treats it as (b), and nothing waits on it: every move so far conflicted in STATUS.md.

## For Mustafa
1. Answer LT-CLEAN, (a) or (b), whenever it suits you. Nothing waits on it.

DONE
