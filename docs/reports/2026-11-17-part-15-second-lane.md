# Advisor report: part-15-second-lane (2026-11-17, branch claude/wonderful-hamilton-jdmlnf, base main d7dad97)
The docs change between change-set 0 (#56) and change-set 1; pull request #57; docs only; the branch stands in for
`cs/part-15-second-lane`.

## Goal
Put Part 15 of the build plan, "The second lane", on main with Mustafa's six answers, so that both builder chats work
from one record of who owns which change-sets, folders and files. Nothing a player sees.

## Outcome
DONE: built, reviewed and pushed; it waits for Mustafa's go on the merge.

## Plan and approval
- His words that set the scope (22:59:16 UTC): "Keep it to Part 15's text, my answers by script, the pointers they
  need and the rules file's line on gun files. It also brings the status page and the roadmap up to date for this
  merge."
- The advisor agent on the plan, 23:12 UTC: APPROVE on nine conditions, all met: "What it adds" carries his additions
  in full; RECORD-FILES binds this change-set already; the section names the two lines of Part 15 his answers
  overtake and marks this chat's readings as readings; the pointers name the plan's line that changes; ADR 0002's
  pointer holds facts and his words only; every written line counts toward this change-set's stop line of 300.

## What changed
- `docs/reports/2026-11-17-build-plan-slice-1-part-15.md` (new, by script): "His answers" (each quoted, with what it
  adds), Part 15's text as he read it (copied from commit ea851a8, not edited), his two messages with their sha256.
- `docs/DECISIONS.md`: two entries by script (his answers, 22:10:34 UTC; his go on the merge of #56, 22:59:16 UTC)
  and two dated pointers in this lane's earlier entries.
- Pointers: ADR 0002's package list (his word on a seventh package); two in the plan report's "Since the plan".
  `docs/research/asset-export-rules.md`, rule AX-R1: his answer GUN-FILES in place of "until he answers".
- `docs/STATUS.md`, `docs/STATUS_ARCHIVE.md`, `docs/ROADMAP.md`, `docs/REPO_MAP.md` (docs-keeper): #56 merged
  (d7dad97, 23:00:12 UTC, on his go); #57's row; the six follow-ups from change-set 0 and Part 15.

## Evidence
- `build.mjs --check`: both entries equal his saved messages byte for byte; Part 15's text in the report equals commit
  ea851a8's; each fenced block matches its sha256. Every quote of his in the written lines is in his two messages
  (`quotes-check.mjs`); the other quoted names and lines are exact against their own sources (the verifier). The plan in
  the build plan's report still equals commit 68addf9.
- RECORD-FILES: every line docs-keeper removed from STATUS and ROADMAP was written by change-set 0 (blame: 94aaae8,
  6a31650, 4f20643); the netcode lane's lines (#40's row, its Waiting bullet, the DOCS-54 entry) are byte for byte.
- Main after the merge of #56: `gate` with `smoke` (run 856) and `toolchain` (run 155) green on d7dad97.
- Size: 220 written lines against this change-set's stop line of 300; 15 of them were added after the reviews, for their
  findings and to complete this report; 339 copied lines are not counted (Part 15's text 257, his two messages in two
  places 82).
- Gate: `node tools/gate.mjs --no-smoke` PASS; the repository map check passes. Smoke cannot start in this container
  (Chromium build 1194 wanted, 1243 installed): CI runs it on the last commit.

## Reviews
- docs-keeper's pass, 23:52 UTC: done within the rule above. The verifier, 00:11 UTC (2026-11-18): PASS, seven Low, all
  fixed in the last commits. It extracted his two messages again from the transcript (equal) and read main's CI runs.
  Not verified by it: the bytes he received; smoke in this container.
- The advisor agent on the final content, 00:11 UTC (2026-11-18): APPROVE at 3e4fea1, five Low, all fixed in the last
  commits; its word on the final head is in the pull request's description.

## Decisions and deviations
- His go on the merge of #56 is recorded in DECISIONS by this change-set: this plan's reading of his "Keep it to"
  sentence, because the go holds what he accepts with the merge; the advisor agent agreed.
- Part 15's text is not edited. His answers overtake two of its lines: its title's "it waits for Mustafa's answers",
  and VFX-ROUTE's option (a), where it says the copy "may need changes".
- The rules file is lane 2's since change-set 0's merge (Part 15, 15.2); lane 1 edits its one line on gun files on
  his word of 22:59:16 UTC. `docs/EDITOR_V2.md`'s pointer is left to lane 2's docs change for the look carry (15.5).
- Left as found, because they are another lane's lines: ROADMAP's "Recent work" now holds 13 entries; STATUS's line on
  main's CI still names 82c605e; #55's row left STATUS in #56's docs pass without an archive row (its merge is still
  recorded in ROADMAP). See OLD-LINES below.
- This change-set's notes (its plan, the reviewers' notes, the scripts) are deleted in the last commit; the commit
  before it holds them.

## Risks
1. Lane 2 has Part 15 only once he gives it the file; each of its units starts from the point 15.2 names, and it edits
   nothing here before it has taken main in.
2. The VFX runtime accepts three.js 0.187 only, while the game pins "the newest release when the look's first
   measurements start" (Part 15, 15.5).
3. The count of written lines takes the status archive's moved row as written; it is the cautious side.

## Proposed next step
Change-set 1, Types (foundation step 2.4).

## Decisions needed
**OLD-LINES: who may fold old lines in the record files that no running lane wrote?** Why it matters: his answer
RECORD-FILES lets a pull request touch only its own lines; ROADMAP holds 13 recent entries against docs-keeper's 8 to 12
and gains one per merge, and STATUS is at 120 of 120 lines. Options: (a) any lane's docs pass may move such lines by
script, byte for byte, never reworded; (b) only the lane that wrote them, so older lines stay; (c) ask him each time.
Recommendation: (a). It does not block this merge.

## For Mustafa
1. Your go on merging pull request #57, after the advisor agent's review of its final head. No Brave check.
2. Part 15's items that stay yours: give lane 2 Part 15 and ask for its list of needs from the block before change-set
   4's plan is reviewed; tell lane 2 which chat on your PC makes the model files; have the VFX chat export an effect
   folder for the VFX proof, and later the effects for the slice's five weapons; pass the Shamble VFX findings to the
   VFX chat.
3. Answer OLD-LINES when it suits you.
