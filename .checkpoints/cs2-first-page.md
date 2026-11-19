# Checkpoint: cs2-first-page (reopened for the last pass)
- **Goal:** change-set 2 of the build plan, "First page" (foundation step 2.5, first half), pull request #61. His
  order of work (2026-11-18, 20:20:09 UTC): "Then change-set 2: its plan, its review, the build and its pull request,
  then stop for my go."
- **Branch:** claude/wonderful-hamilton-jdmlnf (stands in for cs/cs2-first-page; lane 1, the first builder chat) ·
  **Base:** main b763ba3e5f34c67fc4f875cfbd9266167fb095f3
- **Reopened:** 2026-11-19 13:43 UTC (from the clock). **Last update:** 2026-11-19 13:48 UTC (from the clock).

## Why this card exists again
The first card was folded into the report and deleted in commit 29ab052 (`.claude/CHECKPOINTS.md` section 9). It and
its notes stay readable at commit 929b14624767390074268748a021260953f3897e
(`git show 929b146:.checkpoints/cs2-first-page.md`; the list: `git ls-tree -r --name-only 929b146 -- .checkpoints`).
The report that replaced it: `docs/reports/2026-11-19-cs2-first-page.md`.
At 13:36:37 UTC Mustafa sent new words (below). They add a docs-only pass before the pull request is marked ready,
so this small card and its notes exist until that pass is done; then they are folded and deleted again.

## His words that govern this pass (read the file; never a substitute)
`cs2-first-page/mustafa-2026-11-19-1200-four-things.md` (his message of 2026-11-19T13:36:37.137Z, whole, by script,
2494 bytes, sha256 49d984dc...3f1a51; his label: "My words of 2026-11-19, 12:00 (Amsterdam time)"):
- "SIZE-STOP-2: (a). The stop line moves to 4,700. I do not take tests out to fit a number. This is the last move of
  this line: what round 2 asks beyond it, and is not Critical or High, becomes a follow-up for change-set 3."
- "The night you lost to an approval prompt is accepted as you wrote it down, with your lesson."
- "Points 1 to 3 change nothing in change-set 2; point 4 adds only lines to the zip's README."
- Point 4: "Put the Firefox steps in the zip's README and say what I paste back. The game has to work in Chrome, the
  Chromium-based browsers and Firefox. Safari and Mac come later."
- Points 1 to 3 (SLICE-FEEL, PROJ-DRAW (a), lane 2's list on issue #59) are for the plans of change-sets 4, 8 and 9.
His earlier words: `docs/DECISIONS.md` (five entries of this change-set, by script, commit 88225ee).

## Standing rules (short; the full list: the old card at 929b146 and `docs/DECISIONS.md`)
- This chat is Fable 5.2; every agent Opus 5.5, reviewers and the advisor agent at effort max; the advisor tool off.
- No kit, gate or CI file is edited without his yes on the exact text, with the chat in Accept edits.
- At most 3 agents. Never push to another lane's branch; never open `spikes/`; never read `/root/.claude/uploads/`.
- No code change in this pass: the size stands at 4,477 added lines against the stop line of 4,700 (his last move).
- No other file, version or checksum of code from the internet (TOOL-SOURCE): no Firefox is fetched here.
- Times from the clock (`date -u` in the same command); his words by script, never retyped.
- Merge only with the GitHub tool, on his go in chat for the named head, after the advisor agent's APPROVE of it.

## State when reopened
- Head 5172dfba8a423869f2e0b7b67c5384f738db7b31, pushed, tree clean. Code final since b27323a (the zip's commit
  7d8e2de2e holds the same code). Pull request #61: draft.
- The advisor agent (task a9aa7e61d3ebfb467, Opus 5.5, effort max) answered on 5172dfb (its hand-back reached the
  main session after 13:43 UTC): APPROVE for that head once CI's gate, smoke and toolchain are green on it; all
  eight findings of its review of 29ab052 closed; two Low wording fixes ride this pass (report: "two autosaves
  (39664c2, e9f31f0)" becomes "two checkpoint saves"; the bullet on the two questions names the three asks of
  SIZE-STOP-2: 07:57:10, 11:07:17 and 11:07:44 UTC). Both are applied in the working tree. It reads the delta from
  5172dfb to the new head next (continue it with SendMessage to its task id). Notes:
  `.gate/advisor-cs2/advisor-final.md`.
- The zip of the first send: `.gate/cs2-first-page/send/shamble-v2-02-first-page.zip` (245,260 bytes, sha256
  b458b7ca...e8ab), not yet sent to him. Its staged files: `.gate/cs2-first-page/zip/shamble-v2-02-first-page-7d8e2de2e/`.

## Steps of this pass
1. [x] His words saved by script (13:42 UTC): `cs2-first-page/mustafa-2026-11-19-1200-four-things.md`.
2. [x] The zip's README has the Firefox steps (F1 to F7) and what he pastes back (`cs2-first-page/make-zip-2.py`,
   its output: `cs2-first-page/make-zip-2.txt`; 13:44:01 UTC). The eight built files are byte for byte those of the
   first zip; README.txt (53 lines before, 86 now, every old line stands, in order) and with it SHA256SUMS.txt
   differ. New zip: `.gate/cs2-first-page/zip/shamble-v2-02-first-page-7d8e2de2e-firefox-2/shamble-v2-02-first-page.zip`,
   245,953 bytes, sha256 c49d8a7f9b599bc5d9855fca0c782650a796b0f54ec4cf284badb5f2a920c5f4; copied to
   `.gate/cs2-first-page/send/`. The 12 browser tests against its unpacked copy: 12 passed (49.4 s, 13:44:07 to
   13:44:59 UTC; allocation 52 bytes per frame, 515 per block). The script's first run (13:43:45 UTC) stopped at its
   own line-width check after it had copied the files: a leftover folder `...-7d8e2de2e-firefox` lies in `.gate/`
   (scratch, not deleted: folder trees are not removed without his word). Fixed: the checks run before anything is
   written, and the limit is the first README's own widest line (105).
   Firefox was read, not run: `cs2-first-page/firefox-reading.md`.
3. [x] `docs/DECISIONS.md`: his words of 13:36:37 UTC as the newest entry, by script
   (`cs2-first-page/decisions-2.mjs`, 13:45:48 UTC; all seven checks yes). Not committed yet.
4. [ ] The follow-ups from his points 1 to 4 into `docs/STATUS_ARCHIVE.md` (the block of PR #61) and STATUS's
   "For Mustafa" line (Brave and once Firefox).
5. [ ] The report: outcome, decisions, not verified (Firefox was not run here), follow-ups, the new zip's size and
   sha256, the Firefox steps in "For Mustafa".
6. [ ] Commit by path, push by hand. The advisor agent reads the delta 5172dfb..new head.
7. [ ] Fold this card into the report, delete `.checkpoints/cs2-first-page*` in the last commit (no "[skip ci]"),
   push; the advisor agent's verdict on that head; CI green on it (gate with smoke, toolchain).
8. [ ] Pull request #61's description (verdict lines with the full head, the commits that hold the notes, CI runs,
   what is not verified, the zip's sha256); mark it ready; send the zip and the screenshot in the chat; stop for his
   browser checks and his go.

## Current step
Steps 4 and 5 (the archive's follow-ups, STATUS, the report); the report's edits are under way, uncommitted.

## Open findings
None above Low.

## Key numbers
4,477 added lines of code (stop line 4,700). Browser tests 12 passed. First zip 245,260 bytes (never sent); the zip to send 245,953 bytes.

## Decisions open
None of his. A question for later, with the CI pull request's text: when the browser tests also run in Firefox
(that needs a Firefox build from the internet: his word first).
