# Plan: docs-after-v2-research (docs only), revision 1 (DRAFT: the sweep's findings are not folded in yet)

## Goal
The docs catch up with PR #28's merge (R1–R6 research, merge commit b6effde, 2026-10-31) and record the decisions of
2026-10-31, word for word in `decisions-2026-10-31.md`:
- Mustafa: MODES-3 (a) totals, the biggest match 24v24 = 48 · PREVIEW-WALL (a) private from the first preview that
  reaches the test game server · SEC-SPREAD (a) server-only spread · ABL-PREDICT later, after the Feel Lab test.
- The Advisor: BASELINE-WHEN (b): ADR 0006 lists the security baseline as open, with temporary positions; the baseline
  is researched before the first online playtest, in its own fresh session.
The decision drafts (the next step) must find these answers where they start: STATUS and the research folder.

## Branch
`cs/docs-after-v2-research`, from origin/main b6effde, in the main checkout (nothing else runs there now); first commit
1bd662d (the checkpoint, pushed). This session pushes `cs/` branches (cs/v2-research, merged as #28).

## Change type and scope
Docs only: `docs/STATUS.md`, `docs/ROADMAP.md`, `docs/research/README.md`; and one line of `docs/VISION_V2.md` §3 if
the advisor allows it (item 4). No game code; no rules files (`.claude/`, `CLAUDE.md`, `tools/` untouched); no new
files (the checkpoint is deleted before ready). Symbols +0 −0. The research topic files, `GLOSSARY.md`,
`UNREAD_SOURCES.md` and the v2-research report stay as merged: they are verified, dated records, and the advisor's #28
review sends its one wording fix there to the synthesis ("Fix in the synthesis").

## The edits
docs-keeper writes STATUS, ROADMAP and the research README, checking every new fact against git, GitHub or the
decisions file first. Every decision code gets its meaning in plain words on the line where it appears.

1. **`docs/STATUS.md`**, in the sections it has:
   - **Header line 3:** "Last updated: 2026-10-31"; R1–R6 research merged (PR #28, b6effde); drop "(PR #28, not merged
     yet)".
   - **Phase, the v2 paragraph:** R1–R6 research joins "Done" (PR #28, b6effde, 2026-10-31; docs only); "Done, waiting
     to merge" goes. "Next": the decision drafts (`docs/ENGINE_OVERVIEW.md`, `docs/adr/0001`–`0006`), starting from the
     2026-10-31 answers (Decisions pending); then a stop for Mustafa and the Advisor.
   - **In flight:** the R1–R6 row becomes "merged: PR #28, b6effde, 2026-10-31, on Mustafa's 'merge #28' after the
     advisor's APPROVE of 854d42f" | done | none. A new first row: docs-after-v2-research (this PR) | waiting on the
     advisor's review of its head, then Mustafa's "merge #<n>".
   - **Targets:** a v2 bullet first, from VISION_V2 §3 read with MODES-3 (a): the start modes are two teams with 2–8
     players in total (1v1 to 4v4) and three teams with 6–18 in total (2v2v2 to 6v6v6); the goal is 12v12v12 or
     24v24; the largest mode is 24v24, 48 players; one faction per team, squads of 4, bots fill empty slots; design for
     48+ players (CLAUDE.md). The old bullet (12–16 per team, 12v12v12 to 16v16v16, milestones 2–8 → 16 → full scale)
     is kept, marked "legacy target (before v2), superseded", as STATUS does with its other legacy paragraphs.
   - **Decisions pending:**
     - New first group, "Taken by Mustafa 2026-10-31 (in the v2 day session's chat, with his 'merge #28')": MODES-3 (a):
       VISION_V2 §3's starting sizes count players in total, so the largest mode is 24v24, 48 players (not 18v18v18,
       54) · PREVIEW-WALL (a): per-PR previews that reach the test game server sit behind a Cloudflare login on his
       account from the first one · SEC-SPREAD (a): weapon spread stays known only to the server; tracers use a
       similar-looking spread that the hit corrects (the research's default) · ABL-PREDICT: answered later, after the
       Feel Lab test (it stays under "Waiting").
     - New group, "Taken by the Advisor 2026-10-31 (relayed by Mustafa)": BASELINE-WHEN (b): ADR 0006 lists the security
       baseline as open, with temporary positions; the baseline is researched before the first online playtest, in
       its own fresh session.
     - "Waiting": keeps ABL-PREDICT (with "after the Feel Lab test") and NS-2; the answered entries leave it. The "Next
       after PR #28" line becomes "Next: the decision drafts", with what they carry forward: (1) the synthesis plan
       verifies every ADR claim and number, R3–R6's included (research plan condition 9; the advisor's #28 review,
       finding 4); (2) R2-BUD-REC6's reason holds only at N = 54: at N = 48 every 60 Hz tick fits, the re-flood tick by
       0.05 ms (8.28 of 8.33 ms, GLOSSARY §1) (finding 2); (3) where a research mark gives values for 48 and 54, the
       drafts use 48.
     - New open item, **VISION-WORDING**: `CLAUDE.md:7` and `.claude/agents/netcode-architect.md:17` read VISION_V2 §3
       per team ("two teams of 2–8 and three teams of 6–18"); under MODES-3 (a) they need the totals reading. They are
       rules files, so a kit change-set aligns them, with Mustafa's go (kit-v3-1 lands first if they collide, BRANCH-2
       (a)).
   - **Open bugs and risks, R1–R6 research:** risk 1 ("If MODES-3 picks 54 players…") becomes: "MODES-3 (a), 2026-10-31,
     keeps the largest mode at 48 players, so the marks written for 48 hold; the 60 Hz server tick on a re-flood tick
     fits by 0.05 ms (8.28 of 8.33 ms, GLOSSARY §1), so a 60 Hz choice has almost no room." Risks 2–4 stay.
2. **`docs/ROADMAP.md`:**
   - **Recent work:** the R1–R6 entry becomes MERGED (PR #28, b6effde, 2026-10-31, on Mustafa's "merge #28" after the
     advisor's APPROVE of 854d42f); a terse new entry for this change-set with the 2026-10-31 answers in one line.
     Hygiene rule: 8–12 entries, the oldest folded into "Build history".
   - **Next / open:** a new first bullet for v2 phase 0: next, the decision drafts; the security baseline is researched
     before the first online playtest, in its own fresh session (BASELINE-WHEN (b)); ABL-PREDICT waits for a Feel Lab
     test. Where things stand: STATUS.
3. **`docs/research/README.md`:** after the opening paragraph ("Nothing here is decided…"), one short paragraph:
   "Answered since (2026-10-31; STATUS, 'Decisions pending'): MODES-3 (a), the largest mode is 24v24, 48 players;
   PREVIEW-WALL (a), previews that reach the test game server are private from the first one; SEC-SPREAD (a), weapon
   spread stays server-only; BASELINE-WHEN (b), ADR 0006 lists the security baseline as open. ABL-PREDICT waits for a
   Feel Lab test. The topic files and GLOSSARY.md keep the questions as written; the decision drafts apply the
   answers."
4. **`docs/VISION_V2.md` §3, line 43** (the advisor rules on this item): the file is "written by Mustafa and the
   Advisor", and its own tag means "[DECIDED] Mustafa decided it". The line is where the per-team reading began (two
   rules files and the research's MODES-3 question). Options: (a) the session appends Mustafa's answer in brackets,
   nothing else changed: "two teams with 2–8 players, and three teams with 6–18 players (in total: 1v1 to 4v4, and
   2v2v2 to 6v6v6; MODES-3 (a), Mustafa, 2026-10-31)."; (b) VISION_V2 is left to Mustafa and the Advisor, and the
   wording goes to them under "Decisions needed". Recommendation: (a), since it records his own answer in his words'
   place and stops the misreading at its source.
5. **Issue #16 (the Night queue):** unchanged, because nothing new can run unattended: the security baseline research
   is web research by day in its own session (research stays in the daytime only until kit-v3-1 is merged). [Pending
   the sweep's issue #16 check.]

## Evidence and report
- No report file (docs-only change-sets have none): the PR body is the report: what changed, what each fact was
  checked against, the gate result, Decisions needed, For Mustafa.
- Gate: PASS on the branch (`node tools/gate.mjs`, run in the main checkout, which has `tools/node_modules`), plus CI's
  `gate` and `smoke` checks on the PR head. Repo-map check (no paths added or removed).
- Before the PR review, a read-only verification workflow over the final diff (ultracode): a fact checker (every changed
  line's facts against git, GitHub and the decisions file), a wording checker (each answer says what Mustafa and the
  Advisor said, nothing added; every code explained on its line), and a completeness critic (anything still stale;
  anything this plan lists but the diff lacks). docs-keeper fixes what they find; the session fixes the README or
  VISION_V2 line if it made them.

## Reviewers
The advisor agent: this plan, then the PR's head, in this session. docs-keeper writes STATUS, ROADMAP and the README.

## Merge path
A draft PR while work is in progress. The advisor reviews its head here; the checkpoint is folded into the PR body and
deleted in the last commit (CHECKPOINTS.md §9; that commit runs CI); the PR is marked ready; on Mustafa's "merge #<n>"
(docs only: no Brave check) the session merges with the approved full SHA as the expected head, with `gate` and `smoke`
green on it. A push after the APPROVE needs a new review and a new go.

## Risks
1. If the advisor rules (b) on item 4, VISION_V2 §3 stays ambiguous until Mustafa and the Advisor edit it.
2. kit-v3-1 may land with the per-team wording in CLAUDE.md; STATUS keeps VISION-WORDING open until a kit change-set
   fixes it.
3. STATUS is long and its line numbers drift: docs-keeper greps before each edit.
4. The thin 60 Hz margin at N = 48 is recorded for the drafts, not decided here.

## For Mustafa
Nothing to check in Brave. NS-2 is still his step. Say "merge #<n>" once the advisor has approved its head.
