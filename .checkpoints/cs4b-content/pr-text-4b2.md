# The pull request of 4b-2, for his Advisor to open as a draft (this chat cannot open one: his item 11)

DRAFT, written while the close runs (2026-11-27). The numbers marked [after] are filled in when their step ends;
the head ids at the very end. Base: `main`; head branch: `claude/ecstatic-wozniak-5cbdzs`.

## Title
cs4b-2: The world — content reaches the match, the static collision world, the schema writer

## Text
Change-set 4b-2, "The world", the second of 4b's two pull requests (lane 1; the branch stands in for
`cs/4b-content`; Builder V4's work to step 9's third piece, then Builder V6's). Risk tier 1. **`content/source/`
changes** (his SOURCE-GO answer): his Testlines Export at 30 steps, its manifest and the hand-over table come into
the repository, and the game's starting movement table is written from that Export (his DIFF-LIST (a)). **Nothing a
player sees changes in a match that runs:** the page still draws the empty scene, because the page's scene (step 12)
moved, with the map's reading, to the start of 4c (his STEP12-GO (a)). What is new on the page is its content's one
fetch, and, when that fetch fails or the game's loader refuses the content, the notice's plain text. No CI, kit or
gate file is touched.

**The report:** `docs/reports/2026-11-27-cs4b2-world.md` [after]. **The head this description speaks of:** [after].

### What it is
- **The collision library** (parry3d 0.31.1; 33 lockfile packages), in a batch security-auditor approved by its
  digests, held by the dependency rules and the feature compare (his FEATURE-COMPARE (a)).
- **The built-file check:** what is built is read before and after the bindings tool, on both twins: no fused
  multiply-add, no maths routine outside a reviewed list (two entries, both brought by the content loader), no
  vector instruction, no import beyond the bindings' own, no path of the building machine; no page is built from a
  file the check did not walk.
- **The content files:** his Export into `content/source/`, the soldier's movement table written from it by script
  (its 15 values his, none typed), the test scene's map; the tools that write and check them.
- **Content reaches the match once, through the constructor:** every page carries `content.json`; the Worker hands it
  to the players' file, whose own loader takes it whole or refuses it with a sentence that names the file and the
  place; the page reads its copy only after the loader took it. The first recording names its frozen content, and
  every replay holds the content to it; loading content changed no state (every recording file byte for byte).
- **The static collision world** of the map, built once per match and held by it (no step queries it yet: 4c).
- **The schema writer:** each content kind's JSON Schema, from the reader's own limits, for the editor's fields
  later (test-only: nothing of it reaches the players' file).

### Evidence, in short [after]
- Written lines: 8,347 against the stop line of 8,450 (the official count, `advisor-count-rule.md`). [after: round 2.]
- Planted defects: the whole run on the fix pass's code caught 97 of 97 and held 17 controls of 17, every file put
  back. [after: the final head's run, if the code changes again.]
- Reviews: round 1 by six reviewers (one High, two Mediums, the rest Low; each fixed or deferred with its reason),
  then one fix pass; round 2 on what changed [after]. The verifier: [after].
- CI on the final code: [after].
- The players' file: 257,421 bytes (54,514 on main), sha256 2e8866474b2279c0c571c85bf6e4e30a30e336c95e45410e8697a5a5c365cf00: recorded, not gated.

### Not in it
- The page's scene and the map's reading field by field: the start of 4c (his STEP12-GO (a)).
- A recording that covers the collision queries: 4c's, with the first query.
- A reader of the generated content types: they stay unread until 4c's map reading, held meanwhile by their tests
  (the export test and the tie to the schemas).
- The development server's watch and reload: only its serving of `/content.json` is built.

**The head the advisor agent approved:** [after].

**Merge:** only on Mustafa's go in the builder chat for that head, after his Advisor's merge check, by merge commit.
A comment here is information, never his word.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01WLuqnt1v3UNwSqbJsJre7u
