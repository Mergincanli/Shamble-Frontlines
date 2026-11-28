# Advisor report: cs4c1-scene (2026-11-28, branch `claude/ecstatic-wozniak-5cbdzs`, base main dae6d107)

Change-set 4c-1, "The page's scene", the first of 4c's two pull requests (his SIZE-4C (b)); lane 1; the chat he calls
Builder V6. The branch stands in for `cs/4c-soldier`; **his Advisor opens the pull request from a branch of its own
at exactly the final head** (his item 6 c), so this chat's later pushes for 4c-2 do not move it. Risk tier 1,
needs-mustafa: his Brave check, and step 0 writes into decision docs (`docs/LOOP_V2.md`, `docs/EDITOR_V2.md`, the
register). No CI, kit or gate file; no file under `crates/` (the players' file is unchanged); no `content/` file.

The notes this report names (`.checkpoints/cs4c-soldier.md`, the card; `.checkpoints/cs4c-soldier/`; and
`.checkpoints/cs4-walk/`, the walk plan and his saved messages) leave the head in its last commit
(`.claude/CHECKPOINTS.md` §9); they stand whole at commit [after: the commit before the last], read with
`git show <that commit>:<path>`. They come back on the branch in 4c-2's first commit.

## Goal
He sees the world before he walks in it: the page reads the test scene's map field by field and draws it as plain
grey blocks, a higher top never darker, the street and the grass in greys of their own, under an overview camera
that frames the whole map; the readout names the content files and counts the scene. And his words of 27 and 28
November go into the register and the design docs (step 0), before any other docs commit of 4c.

## Outcome
DONE for step 0 and step S. Two review rounds (all six reviewers by the end, the second round's two APPROVE; one
Medium, art-director's AD-4, deferred to 4c-2's step 4, and Lows left as follow-ups) and one fix pass; the verifier
[after]; the advisor agent reads the final head, which holds
this report, and its verdict comes in the builder chat and on the board. His Brave check decides.

## Plan and approval
The plan sheet's revision 2 (`reread-sheet.md`, sections D and G), approved for step 0 and step S by the advisor
agent (`advisor-reread-4c.md`: APPROVE with conditions 1 to 9; steps 1 to 7 not yet, its item 10). Each condition:
(1) the inventory's marker is the thrown sentence `INVENTORY_UNKNOWN` (`render/inventory.ts:11`), which the
build's marker check names (`build-checks.mjs:204-212`); (2) until he answered SIZE-4C only step S and notes went on the
branch (no code or docs commit between c1de41e5 and step 0's af789ef3); (3) STREET-GREY was its own question, and he
answered (b), "I look at it at the scene's Brave check"; (4) the hold of about 1,665 lines, told to him before the
count passed 1,420; (5) art-director joined the five reviewers on a narrow brief; (6) his Brave-then-Firefox list,
each step with its expected result, is art-director's (below, "For Mustafa"); (7) step 0's checks: the byte check of
LOOP_V2 and EDITOR_V2 against main (`loop-check-4c.mjs`), BOARD-WORDS (b)'s method before it ran, section 14's tie to
his message's digest (`section14-4c.mjs`), each pointer at the end of its sentence (LOOP_V2:16 moved there in the
docs pass, the security review's note); (8) step 0 waited for PASTE-BREAKS, BOARD-WORDS and DESIGN-CLASH (all
answered in his message sent at 11:43:45 UTC, which gives 12:10 Amsterdam as its time) and landed (af789ef3, 7cbf8226) before any other docs commit of 4c; (9) risk tier 1,
needs-mustafa.

## What changed (31 files outside the notes at 9283d302, 1,974 lines added and 186 removed, against main dae6d107; this report is one more)
- **Step 0** (af789ef3, 7cbf8226; by script): `docs/DECISIONS.md` gains his five messages of 27 and 28 November
  (`decisions-4c.mjs`: 14, 16, 7,117, 1,284 and 4,516 bytes, each byte for byte; the merge route without its line 5,
  by BOARD-WORDS (b)); `docs/LOOP_V2.md` gains his section 14 (517 words and signs, once, last), his dated pointer at
  eight lines and the header's one sentence; `docs/EDITOR_V2.md` one pointer (:47); `docs/adr/README.md` one dated
  line (his DESIGN-CLASH (a), with his own words at the silo line).
- **Step S** (bc7720f3, c1de41e5): `render/scene.ts` (new, in place of `empty-scene.ts`: the lights, the camera, the
  map's floor with flat faces turned outward, greys by height, the grid, the overview, disposal), `render/inventory.ts`
  (new, development builds only), `render/three-picture.ts` (`setContent`, `inventory`), `picture.ts`
  (`PictureContent`, `SceneObject`), `app/content.ts` (`readMap`, field by field, closed key sets), `app/boot.ts`,
  `overlay/report.ts` (the content and scene rows), `scripts/build-checks.mjs` (the markers), the Worker's
  `local-map.ts` (`LOCAL_MAP`, so the page takes in nothing of `recording.ts`); the tests beside each, the new
  `content-map.test.mjs`, the browser specs (the drawn scene against the built map, corner for corner).
- **The fix pass** (2b2f44c2) and one comment (0645432f): the overview's search bounded and the framing moved into
  `scene.ts` (`frameCamera`), the resize's width held at one pixel; tests of the ground and patches, the solids'
  closed surfaces, the nearest fit, the grid's extent, the greys' whole range, every drawn corner; the map's words
  tied to the loader's; the inventory's kinds typed; the page's own refusal named in the readout ("refused").
- **The docs pass** (9283d302, docs-keeper's): STATUS, ROADMAP, the archive, REPO_MAP; LOOP_V2:16's pointer moved.
- **Beyond the plan's symbol table** (engine-architect's table 1.10): from step S, the exported types `Floor` and
  `FloorKind` (`render/scene.ts`) and `ContentFileRow` and `SceneRows` (`overlay/report.ts`), the browser specs'
  `SceneObject` type and `scene()` (`e2e/tests/skeleton/report.ts`), and `LOCAL_MAP` in a module of its own
  (`sim-worker/src/local-map.ts`; the plan had it only re-exported); from the fix pass, `frameCamera`, `VIEW_MARGIN`
  and the `Overview` type (exported; the overview's answer may now be `null`), `MOST_DOUBLINGS` (a module constant),
  `KINDS` and `isKind` in the inventory, `MAP_FORMS`, `MAP_TOWARDS` and `MAP_GROUNDS` (exported),
  `PageContent.refused` and the readout's "refused" row. The gate's symbol counts are the legacy files' (+0 -0;
  GATE PASS with smoke at 9283d302, `.gate/gate-4c1-docs.log`); nothing under `legacy/` changes.

## Evidence
- **Size:** 1,565 written lines (`count-4c.mjs` on `git diff --numstat dae6d107 0645432f -- crates packages`):
  the count after each code commit, bc7720f3 1,277, c1de41e5 1,288, the fix pass 1,564 (its own diff adds 340 lines),
  0645432f 1,565 (its diff adds 2); under the hold of 1,665; 2.1 times its recount of 740.
- **The full local checks** on the fix pass's files (12:29 UTC): ALL LOCAL CHECKS PASSED (the page's 176 unit tests,
  the Worker's 53, the types' 14, 29 Chromium tests in 232 s). **CI:** on 2b2f44c2 gate, smoke and toolchain green
  (runs 46145018063, 46145018054; toolchain 9 min 53 s); on 0645432f gate, smoke and toolchain green (runs
  46146007607, 46146007608; toolchain 9 min 3 s; Chromium 29 in 3.6 min, Firefox 5). The verifier's own run [after].
  The final head's runs [after].
- **Planted defects** (`plants-4c.py`, `plants-4c.txt`, on 2b2f44c2): **45 caught of 45, 3 controls held of 3**,
  every file put back (7 files by sha256); F1 (the search that never ends) caught by the run's outer timeout. Earlier,
  on acd7d4f: 28 of 28 once the volume check (c1de41e5) caught the ramp-face plant the first run missed.
- **The picture** (the WebGL 2 fallback on the container's software renderer): 32 draw calls, 267 triangles, 162
  lines, 32 geometries, 3 textures, 6 programs (the empty floor's were 3, 3, 62, 3, 3, 6), the same whether the map
  arrives before or after the first frame (`counts-order-4c.mjs`). perf-auditor: about 4.1 KB more allocated a frame
  with the test scene, all inside three.js's per-object drawing, none in this change's code; the hand-off unchanged
  at about 93 B a frame; the camera fit about 22 µs a resize; each floor swap frees every geometry and material.
- **The builds:** the players' file unchanged (no file under `crates/`; 257,421 bytes); the development page's script
  840,441 bytes against main's 832,897 (+7,544, +2,718 with gzip -9: the verifier, at this code; perf-auditor's
  +7,344 was measured at c1de41e5); the production page holds `inventory(){return null}` and neither development
  marker.
- **His words:** `decisions-4c.mjs check`, `section14-4c.mjs check` and `loop-check-4c.mjs` pass at 9283d302; STATUS's
  quotes of his item 12 b and c equal his saved message byte for byte (`quote-check-12bc.mjs`).
- **The after picture:** [after: the verifier's retake].
- **Not verified:** WebGPU (the container's Chromium has no WebGPU adapter, so every count and picture is the WebGL 2
  fallback); a real GPU; Brave; the scene in Firefox (CI's Firefox project runs the rules and replay specs only); the
  one-time pipeline build when the map arrives (GE-F4); a window 0 wide on WebGPU.

## Reviews
Round 1 on c1de41e5, six reviewers, three at a time, then one fix pass (2b2f44c2); round 2 on what changed, by the
two that asked for changes in their lane, then the one comment (0645432f). Every finding: `review-s-round.md`.
Round 1's verdicts: graphics-engineer REVISE (GE-F1), perf-auditor REVISE (only for P-1), test-engineer REVISE (TE-1);
engine-architect, security-auditor and art-director (on its narrow brief) APPROVE.
- **The Medium found four times** (graphics-engineer's GE-F1, engine-architect's EA-1, perf-auditor's P-1,
  security-auditor's S-1): the overview's search for a camera distance never ended at an aspect of 0 or NaN, so a
  window of no width froze the page. Fixed: the search tries at most 20 distances and gives none past them, the
  camera stays as it was then, the resize holds the width at one pixel; tests at 0, NaN and the largest map.
- **test-engineer's TE-1 (Medium):** the ground and the patches had no test of their corners, height or facing:
  fixed. Its Lows TE-2 to TE-8 fixed, among them the framing moved where a Node test reaches it (TE-7).
- **art-director's AD-1 (Medium, his checklist): fixed**, its steps 1, 2, 3 and 6 rewritten (the zip's README and
  "For Mustafa"). **The other Lows fixed:** G-3 / EA-2 and GE-F5 (a map raised above 0), EA-3 to EA-6, S-2 (an own
  `__proto__` key), the security review's note on LOOP_V2:16 (moved in the docs pass).
- **Deferred to STATUS** (each with its owner): GE-F2 (the depth range on a large map), GE-F3 with P-2 (draw calls on
  the largest map), GE-F4 (WebGPU pipelines on the map's arrival), AD-4 (Medium: the readout over a window under
  about 600 px tall, until step 4's follow camera); AD-2 and AD-3 go to his Brave check.
- **Round 2: graphics-engineer APPROVE** (GE-F1 and GE-F5 closed; one comment reworded, 0645432f); **test-engineer
  APPROVE** (every finding closed; five Low gaps, TE2-1 to TE2-5, to STATUS). No round 3. No reviewer read 0645432f
  (a comment); the advisor agent reads it on the final head.
- **The verifier:** FAIL on the report's first draft, for its words only (two Mediums: art-director's two Mediums told
  as Lows; the symbols beyond the plan named in part; and five Lows: a size measured at c1de41e5, the fix pass's
  share of the count, two citations, his message's time), each corrected; every number it checked held (its own
  full local checks, the count, his words, five of the plants against the log and two of its own, eleven "fixed"
  rows in the code, the docs' rows, the after picture). [after: its re-check of the corrections.] Its one new test
  gap goes to STATUS as V-1 (Low): the street's and the grass's greys are tested as different, not for which is
  darker (a swap passes all 11 scene tests; the code's street 0x40 under the grass's 0x50 matches his step 3).

## Decisions and deviations
- His, in his message of 2026-11-28 (it gives 12:10 Amsterdam as its time; sent at 11:43:45 UTC, 12:43 Amsterdam;
  `docs/DECISIONS.md`): SIZE-4C (b) and the one-branch way (item 6);
  STREET-GREY (b); BOARD-WORDS (b); PASTE-BREAKS (a); DESIGN-CLASH (a); DIGEST-NOTE (a). The advisor agent's: the
  hold, the count with generated TypeScript in, the six reviewers.
- Found in the build: the plants' first run missed a ramp drawn with another side's faces, so the test now holds each
  solid's enclosed volume (c1de41e5); the yard's 0.55 m and 0.56 m steps share a grey, so the rule tested is "never
  darker", the design's words.
- **Slips of this chat** (each corrected when found): times written without reading the clock (01:08 for 00:57,
  02:06 for 02:04, two placeholders); a commit id written from memory (3e1c8a3b for bfe447a3); `date` and `sleep` in
  front of git in two reads; a notes commit (bfe447a3) holding a whole message's digest and length, found by the
  automated security review, the method fixed, left as it is by his DIGEST-NOTE (a); told to him "no change at 6.3
  and 9.576 m/s" for 4c-2's swept wall pass, which block 10 measured otherwise (below); question 13 said his
  sandbox's maps never met its case, wrong (his metrics room's window wall), corrected at 13:58 Amsterdam; a time in
  the plan sheet written as 14:20 without reading the clock (it was 14:15), corrected at once. An agent's:
  game-designer read his Testlines repository by its public address before his TESTLINES yes (told to him).

## For 4c-2, said here so nothing is lost (not in this pull request)
- His DEPART-4C (a) asks the report to show PM-0004-20's and PM-0004-35's old and new wording side by side: that
  wording comes with 4c-2's movement step, so 4c-2's report shows it. PM-0004-60 and PM-0004-20's 48-soldier run stay
  open in STATUS (quoted there).
- The swept wall pass against his sandbox's push-out (information, not a question: game-designer's block 8): the same
  in the open and on every face; at corners up to 0.069 m at his walk (one case, rounding the 3.05 m ledge while
  pushing another soldier), 4 micrometres at his sprint; on the roofs it holds him one tick at the ridge (paths up to
  0.51 m apart, the same result).

## Risks
1. Nothing has been seen on WebGPU or a real GPU; his Brave check is the first look.
2. The browser run: 232 s locally and 3.6 minutes in CI, under the 5-minute limit that 4c-2's specs press on; the CI
   Builder raises it after Tuesday 1 December, 20:00 (his CHROME-TIME).
3. Draw calls grow with the map (about 579 on the largest the loader allows; GE-F3 with P-2).
4. In a window under about 600 px tall the readout darkens the back of the scene (AD-4) until step 4.

## Proposed next step
Change-set 4c-2, the soldier (steps 1 to 7), on this branch on top of this head (his item 6 b), once the advisor
agent approves its plan.

## Decisions needed
- **HEADROOM-FEET** (question 13 of `question-2026-11-28-before-4c.md`): may the game differ from his sandbox where
  the top of a block at or below the soldier's feet would stop him like a wall (his metrics room has the case at his
  Export's sizes: along the 3.0 m window wall's top he is stopped 0.4 m before the stretch over the window)? Why: it
  decides where he can walk on upper floors. Options: (a) never a wall at or below his feet; (b) exactly as his
  sandbox. Recommended: (a). Needed before 4c-2's movement step; until he answers, step 1 builds both ways as a switch,
  whose default the advisor agent rules (this chat proposes (b), his sandbox's rule, by his MOVE-RULES (a)).

## For Mustafa
1. **Your Brave check of the scene** on the final head [after: its full id]: the zip in the builder chat; about 6
   minutes, in Brave, then once in Firefox (art-director's steps, the zip's README): (1) a maximised window: the whole
   map from above at an angle, the readout naming the backend, the commit [after] and "errors: 0"; (2) the blocks:
   the two houses with their alley, the cover wall, the yard's steps, ledges, thin wall, platforms with ramps and six
   posts; height in steps of brightness, every top lighter than its sides; (3) the ground: the street and the yard's
   lane a little darker than the grass (your STREET-GREY); (4) the readout's two new lines; (5) "Copy report",
   pasted into the chat; (6) the same in Firefox, and once more in Brave with `?backend=webgl` if both said WebGPU.
   One line to know: EDITOR_V2:47's pointer reads "(2026-11-27: see §14)" and does not name LOOP_V2, where §14 is.
2. **Your go for the merge** of that head, after your Advisor opens the pull request from a branch of its own at it,
   by merge commit with that head as the expected one; the title and text: `pr-text-4c1.md` in the notes.
3. **HEADROOM-FEET**, above, before 4c-2's movement step.
