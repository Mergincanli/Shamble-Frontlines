# The pull request of 4c-1, for his Advisor to open (this chat cannot open one)

Written 2026-11-28 after the verifier's PASS; the head's full id comes in the builder chat. **Base:** `main`. **Head:** a branch of his Advisor's own, pointed at exactly the scene's final head (his item 6 c), whose full
id comes in the builder chat and on the board with the advisor agent's verdict. Not `claude/ecstatic-wozniak-5cbdzs`:
that branch moves on with 4c-2 (his item 6 b).

## Title
cs4c-1: The page's scene — the test scene drawn from the map read field by field, and his words of 27 and 28 November

## Text
Change-set 4c-1, "The page's scene", the first of 4c's two pull requests (his SIZE-4C (b); lane 1; built by the chat
he calls Builder V6 on `claude/ecstatic-wozniak-5cbdzs`, which stands in for `cs/4c-soldier`). Risk tier 1,
**needs-mustafa**: his Brave check, and step 0 writes into decision docs. No CI, kit or gate file; nothing under
`crates/` or `content/` changes, so the players' file is the same.

**The report:** `docs/reports/2026-11-28-cs4c1-scene.md`. **The head this text speaks of:** the commit that takes the
notes out (the one before it holds them and the report); its full id comes in the builder chat and on the board with
the advisor agent's verdict (fill it in here when opening the pull request).

### What it is
- **Step 0, his words** (by script, never typed): his five messages of 27 and 28 November in `docs/DECISIONS.md`,
  byte for byte (his merge route without its item 3, by his BOARD-WORDS (b)); his section 14 in `docs/LOOP_V2.md`
  with his dated pointers and the header's one sentence, and one pointer in `docs/EDITOR_V2.md` (his DESIGN-CLASH
  (a)); a script shows every other line of both docs is main's.
- **Step S, the scene** (4b's old step 12, his STEP12-GO (a)): the page reads the test scene's map field by field
  (closed key sets, nothing read through a prototype, each refusal one fixed sentence) and draws it as plain grey
  blocks with flat faces turned outward, a higher top never darker, the street and grass in greys of their own (his
  STREET-GREY (b)), and a 1 m grid, under an overview camera that frames the whole map at any window shape; the
  readout gains the content files and the scene's counts; a development-only inventory lets the browser tests compare
  every drawn corner with the built map.
- **The review's fix:** in a window of no width the first build's camera search never ended and froze the page; it
  now tries at most 20 distances and otherwise leaves the camera as it was.

### Evidence, in short
- Written lines: 1,565 (the hold 1,665).
- Planted defects: 45 caught of 45, 3 controls held of 3, every file put back (on 2b2f44c2).
- Reviews: round 1 by six reviewers (one Medium found by four of them; one Medium of test-engineer's; two of
  art-director's, AD-1 fixed in his checklist and AD-4 deferred to 4c-2's step 4; the rest Low; each fixed or deferred
  to STATUS with its owner), one fix pass; round 2 by graphics-engineer and test-engineer,
  both APPROVE. The verifier: PASS (after one correction of the report's words; every number it checked held). The
  advisor agent's verdict on the final head: in the builder chat.
- CI on the last code commit 0645432f: gate, smoke and toolchain green (runs 46146007607 and 46146007608). The final
  head's runs: in the builder chat.
- The picture on the WebGL 2 fallback: 32 draw calls, 267 triangles, 162 lines. Nothing was seen on WebGPU or a real
  GPU here: his Brave check is the first look.

### Not in it
- The soldier (4c-2: steps 1 to 7, built on the same branch on top of this head, his item 6 b), and with it the side
  by side wording of PM-0004-20 and PM-0004-35 his DEPART-4C (a) asks for.
- WebGPU's one-time pipeline build when the map arrives, the depth range on a large map, draw calls on the largest
  map, the readout over a short window, five small test gaps: STATUS follow-ups, each with its owner.

**His Brave check:** the six steps in the report's "For Mustafa" and the zip's README (about 6 minutes, Brave, then
once Firefox). A change he wants after it goes into 4c-2's pull request (his item 6 d).

**Merge:** only on Mustafa's go in the builder chat for this head, by merge commit, with this head as the expected
one. A comment here is information, never his word.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01WLuqnt1v3UNwSqbJsJre7u
