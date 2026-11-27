# 4c, "The first soldier": the sheet for the advisor agent's re-read, revision 1 (2026-11-28, main dae6d107)

_Written by the main session (Builder V6, lane 1) from the approved walk plan (`.checkpoints/cs4-walk/plan.md`,
revision 4: section 6 and the sections it names), three reads against today's code (`engine-architect-read.md`,
`netcode-architect-read.md`, `rust-reviewer-read.md` in this folder) and his words. The walk plan's section 3, item 1:
"Each later part starts only after the advisor agent has read this plan again against that day's main ... That
re-read carries the part's exact symbols to add and to remove, taken from the designs' tables as they stand that
day." **No file outside `.checkpoints/` is edited before the advisor agent's verdict.**_

**Status of this revision: being written.** Sections D and E fill in as the three reads finish; until then they
say "pending".

## A. Since the plan was approved (2026-11-25)
- **Merged:** 4a (pull request #70, main 2483e9a), 4b-1 (#75, b9aad01), 4b-2 (#79, dae6d107, at head b7145273 on his go
  of 2026-11-27 22:50:52 UTC). Written lines against the designs' count: 4a 3,300 (2.4 times), 4b-1 7,164 (2.17),
  4b-2 8,408 (2.7).
- **His answers since then that reach 4c** (each word for word in `docs/DECISIONS.md`):
  - STEP12-GO (a): 4b's step 12, the page's scene with the map's reading field by field, is 4c's first building step.
  - GREY-COLOURS (a): the scene's colours are LOOP_V2 section 9's default, "plain blocks, with height read through
    brightness (higher is lighter)", in neutral greys; the stand-in is one mid grey with a darker gun.
  - STEP-60 (a) and DIFF-LIST (a): the movement values are his Export at 30 steps, already in
    `content/movement/soldier.json`.
  - FEATURE-COMPARE (a), condition 4: "the recorded match exercises the collision queries from the first change-set
    that calls them, so native, Chrome and Firefox are compared on the real thing and not only on 10,000 rays": 4c is
    that change-set (follow-up 4B-QUERY-RECORDING).
  - DEPART-4 (a) with its two notes; EARLY-LOOK-ORDER (b); SIZE-4B's point 2 as a standing habit: before step 1 of
    each part, the card names what leaves first if the count runs high, and he hears in one sentence where the count
    stands at two thirds of the stop line.
- **Asked on 2026-11-28, open** (`question-2026-11-28-before-4c.md`): MOVE-RULES, NO-SOURCE, RESUME-FLASH, TUNE-HOME
  (the plan's four before 4c), SIZE-4C, BOARD-WORDS, PASTE-BREAKS, DESIGN-CLASH.
- **4b-2's follow-ups placed in 4c** (`docs/STATUS_ARCHIVE.md` lines 2278 to 2300): 4B-TYPES-READER, 4B-MEMBER-KINDS
  and 4B-BOOT-CALLS at step S; 4B-QUERY-RECORDING, 4B-RAY-COUNTS and 4B-RAY-WASM at steps 1 and 6; 4B-WORLD-ROOM at
  steps 1 and 2; 4B-LIMIT-IMPORT at step 5; 4B-CHROMIUM-TIME is a risk (section G). The advisor agent's acceptance of
  security-auditor's two developer-machine risks goes into STATUS as a dated note at 4c's docs pass.

## B. The steps, in order, and what each waits for
| Step | What | Waits for (his open answers) | Designs' count (code / tests) |
|---|---|---|---|
| 0 | The first docs commit: his words of 2026-11-27 into the register; LOOP_V2's section 14 and its pointers | the advisor agent's APPROVE; PASTE-BREAKS (section 14), BOARD-WORDS (his message of 23:59), DESIGN-CLASH (two lines) | docs only |
| S | The page's scene, with the map's reading field by field (4b's old step 12) | none (GREY-COLOURS is answered); SIZE-4C sets its stop line, held to about 1,420 until then | 230 + about 70 / 260 + about 70 |
| 1 | The simulation's movement | MOVE-RULES (rows 1 to 5 and 8), NO-SOURCE | 1,240 / 1,090 |
| 2 | The command path | pending netcode-architect's read | 385 / 530 |
| 3 | The snapshot's soldier and the block | pending the reads | 320 / 340 |
| 4 | The page: the stand-in, the follow camera, input, the aim line, the stamina bar | RESUME-FLASH | 800 / 540 |
| 5 | The camera panel and the camera's content file (the early look after it) | TUNE-HOME | 360 / 300 |
| 6 | The movement and lane recordings, and every mark that rides them | step 1 | 320 / 230 |
| 6b | The paced replay in the real Worker | step 6 | 130 / 110 |
| 6c | The hostile commands; the far-corner map | step 2 | 30 / 180 |
| 7 | The page's browser tests | RESUME-FLASH (the hidden-tab test) | 0 / 410 |
The counts are the plan's; the reads recount them against today's code (section E).

## C. Step 0, the first docs commit (exact)
After the advisor agent's APPROVE. It waits for PASTE-BREAKS, BOARD-WORDS and DESIGN-CLASH, since nothing in step S
needs it; if his answers come after step S's code, step 0 still lands before any other docs commit of 4c.
- **`docs/DECISIONS.md`**, newest first, by a script (`decisions-4c.mjs` in this folder: 4b-2's `decisions-4b2.mjs`,
  readable at 59f6cc7, with these entries and the same `write` and `check`), from the saved files, never typed:
  1. `cs4-walk/mustafa-2026-11-27-go-merge-79.md`: his go for the merge of 4b-2 at b7145273 ("you have my go").
  2. `cs4-walk/mustafa-2026-11-27-may-merge.md`: his word that the chat may merge it ("You can merge it").
  3. `cs4-walk/mustafa-2026-11-27-design-notes.md`: his design notes on resource points and repair bots, whole.
  4. His message of 23:59 Amsterdam (MERGE-ROUTE (a), how 4c starts), only as BOARD-WORDS says.
  STEP12-GO is already there (`docs/DECISIONS.md:183`): the script refuses a message that is in the file.
- **`docs/LOOP_V2.md`** (his items 2 and 3, placed by docs-keeper, which writes no design of its own):
  - Section 14 after section 13: the saved message's lines 61 to 128 (from "## 14. Resource points and repair bots"
    to the line before "9. "), cut by `chat-words.mjs part` and never typed. By PASTE-BREAKS (a): each line "- " joined
    to the line after it and each line "  " to the line after it, nothing else; a check proves that the placed text's
    words and signs are the message's, in order.
  - The dated pointer "(2026-11-27: resource points replace open fields, see §14)" at the lines of sections 1, 3, 9
    and 11 that speak of the harvest field in the centre or the safe field near each base: line 15 (section 1), 32
    and 34 (section 3), 150, 154 and 156 (section 9), 181 (section 11). The old text stays.
  - The header's "Updated" line (line 5) gains one sentence that names section 14.
  - By DESIGN-CLASH: `docs/EDITOR_V2.md:45` and `docs/LOOP_V2.md:52` as he answers.
- **`docs/adr/README.md`** (the register): one dated line that names LOOP_V2's section 14.
- **Evidence:** `decisions-4c.mjs check` (every quoted line byte for byte, each saved file against its own size and
  sha256); the section-14 check; `git diff --stat` touching only those three files (four with EDITOR_V2); the gate.

## D. Step S, the page's scene (exact) — pending engine-architect's first block

## E. Steps 1 to 7 against today's code — pending the three reads

## F. Sizes, the ladder and the stop lines
- By the designs' count: steps 1 to 7, 7,315; step S about 630; together about 7,950. At the measured rates (2.17
  to 2.7): about 17,250 to 21,450 written lines. The plan's stop line, 16,450, was set before step S moved in: the
  line with it inside is his (SIZE-4C).
- **What leaves first if the count runs high** (his SIZE-4B point 2): the plan's rung, the far-corner map (PM-0004-39)
  to 4d at 13,150 written lines, told to him before it moves. Nothing else moves; at a stop line the chat stops and he
  decides. Under SIZE-4C (b) or (c), each pull request gets its own rungs, written here when he has answered.
- Until he answers SIZE-4C, step S is held to about 1,420 written lines (its planned size times 1.25, the lowest line
  any option gives it), and its reviews wait, since under (a) they run on 4c's final diff.
- The count is taken at each step's end by 4b-2's method: `git diff --numstat` against the base, `crates/` and
  `packages/` only, the generated and recorded data files counted as 4b-2 counted them (`count-4b2.mjs` at 59f6cc7).

## G. Evidence, reviewers, his checks, risks
- **Step S:** a visual change. Before and after renders of the real page (the empty scene on main, the scene on the
  branch); the render counts before and after with their method; the browser tests; the gate. Reviewers on its diff:
  art-director, perf-auditor, engine-architect (the interface file), security-auditor (content text shown on the
  page), test-engineer. His Brave check: the street corner and the training yard in greys.
- **Steps 1 to 7:** pending the reads.
- **Risks:** CI's Chromium run uses 3.6 to 3.9 of its 5 minutes (4B-CHROMIUM-TIME): 4c's browser tests may pass it,
  and the fix is a CI edit, the CI Builder's, with his yes first. Others pending the reads.

## H. What I ask of the advisor agent — written when D and E are in
