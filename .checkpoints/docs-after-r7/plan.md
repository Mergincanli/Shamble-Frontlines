# Plan: docs-after-r7 (docs only), revision 2

Revision 2 applies the advisor's REVISE of 2026-10-27 18:22 UTC (changes B.1–B.8, verbatim in
`cs/v2-research:.checkpoints/v2-research/advisor-amendments-verdict.md`, part B). What each change did is marked [B.n].

## Goal
STATUS and ROADMAP catch up with the work since 2026-10-17 (they were last updated then) and record the Advisor's
2026-10-27 decisions, including the one Mustafa asked for: kit-v3-1 recorded in STATUS as paused. The Night queue
(issue #16) gets the new night rule.

## Branch [B.1]
`cs/docs-after-r7`, from origin/main 07ab240, in its own git worktree (the main checkout stays on `cs/v2-research`
while research runs). This session pushes `cs/` branches (cs/v2-research, 8ff1c74), so no `claude/` stand-in. If the
push is refused, the PR body says so.

## Change type and scope
Docs only: `docs/STATUS.md` and `docs/ROADMAP.md`, plus issue #16 (its body, and one comment that quotes the Advisor's
2026-10-27 instructions). No game code, no rules files (`.claude/`, `CLAUDE.md`, `tools/` untouched), no new files.
Symbols +0 −0. The night rule lives in #16 and STATUS, not in `NIGHT_SHIFT.md` (that would be a rules change).

## The edits
docs-keeper writes STATUS and ROADMAP, checking every new fact against git, GitHub or the routine list first. Every
decision code gets its meaning in plain words on the line where it appears [B.3].

1. **`docs/STATUS.md`**, using the sections it has [B.2]:
   - **"Last updated" line:** 2026-10-27; v2 phase 0 in progress.
   - **Phase:** a new first paragraph, **"v2 phase 0 (V2_PHASE0_BRIEF)"**: the v2 rebuild (VISION_V2.md and
     V2_PHASE0_BRIEF.md added 2026-10-24); kit-v3-0 merged (PR #26, f2ccc13, 2026-10-25); docs-after-kit-v2-2 merged
     (PR #25, 4c24d07, 2026-10-27); R7, the legacy inventory, merged (PR #27, 07ab240, 2026-10-27); R1–R6 research
     started on `cs/v2-research` (wave 1: R1 simulation core and R2 netcode; daytime only); the synthesis
     (ENGINE_OVERVIEW and the ADR drafts 0001–0006) comes next, then a stop for Mustafa and the Advisor. The legacy
     game is frozen. The existing "Phase 0" paragraph is relabelled **"legacy phase 0 (environment)"**; the old
     "Queue" and "Greybox phase" paragraphs are marked superseded by v2 (paused in issue #16), not deleted.
   - **In flight**, new rows: R1–R6 research (running); docs-after-r7 (this PR); and kit-v3-1 with this text [B.4]:
     > kit-v3-1 (write guards for kit and gate files, a builder agent, a default effort, the secret-pattern fix) |
     > `cs/kit-v3-1` | **paused**: "shelved" by the Advisor, 2026-10-27; this session leaves the branch untouched |
     > Waiting on: Mustafa and the Advisor. Note: the R7 session was still committing to this branch at 18:01 UTC, on
     > an earlier Advisor message (17:32 UTC: finish it there with a smaller, fixed scope and open its PR). Which
     > instruction holds is open (see "Decisions pending").
     Done rows: kit-v3-0 (PR #26), docs-after-kit-v2-2 (PR #25), R7 (PR #27). Older done rows stay.
   - **Decisions pending**, following the pattern at line 67 [B.2]:
     - "Taken by the Advisor 2026-10-27:" R7-MARKS (a): the inventory's marks are working recommendations, changed when
       their ADR draft comes up; rows marked "look brief pending" stay pending until the look brief exists ·
       MANAGED (b): "Mustafa is not an admin of the university's Claude plan", a kit-v3-1 question recorded for when it
       returns (its question is on the kit-v3-1 branch, which this session may not open, so nothing more is written)
       · kit-v3-1 shelved · research in the daytime only until kit-v3-1 returns and is merged · the night shift takes
       only items that need no web access, and lists any permission-prompt stop in its morning report · the PR Advisor
       routine off for now, the advisor agent reviewing PRs in the day session · R6 covers hosting, costs, monitoring
       and telemetry; the security baseline waits.
     - "Taken by the Advisor 2026-10-27 (R7 session):" BRANCH-2 (a): research on `cs/v2-research`, kit-v3-1 on its own
       branch, kit-v3-1 lands first if they collide · SPEND-1 (b): full checks for R1 and R2 and for anything that
       becomes a decision, spot checks for R3–R6, review rounds stop at nits.
     - "Taken by the Advisor 2026-10-25" (link: issue #16 comment 6396928581): KV3-MERGE (a): merge kit-v3-0 ·
       ROUTINE-EFFORT (b): night code changes go through a high-effort builder agent, the night's main session only
       orchestrates · SPIKE-SCOPE (a): once the drafts are approved, the legacy constraints apply to `legacy/` only and
       `spikes/` is throwaway · WRITE-GUARD (a): write guards for kit and gate files, kit changes by day · DOMAINS-ACCEPT
       (a): the web allow list is accepted, but research moves to the night only once WRITE-GUARD is merged.
     - Open: **KIT31-STATE**: is kit-v3-1 paused (this session's instruction, about 17:37 UTC) or being finished in the
       R7 session (its 17:32 UTC instruction)? Mustafa and the Advisor decide.
     - The ADR-001..003 entries get one line: the v2 drafts 0001–0006 replace them (CLAUDE.md, "Decisions pending").
   - **Routines**, a new short section [B.2]: Night shift (trig_019LtRqmAdzddyk6K3e3kDJz, hourly from 00:10 to 08:10
     Amsterdam, its own session): until kit-v3-1 returns and is merged, only items that need no web access, and a
     permission-prompt stop is listed in the morning report; research runs in the daytime only. PR Advisor
     (trig_01YPyWAsQmsaQoydhMGG3CkJ): to be off for now (the Advisor, 2026-10-27); an agent may not switch it (the API
     refused: it was created outside an agent), so Mustafa switches it off; meanwhile the advisor agent reviews PRs in
     the day session, and nothing merges without Mustafa's "merge #<n>".
   - **Open bugs and risks:** R7's three risks (marks used before Mustafa confirms them, now answered by R7-MARKS (a);
     some ANIMATION_PLAN §6 numbers can't be re-run; line numbers go stale when the legacy files move to `legacy/`);
     the PR Advisor routine stays on until Mustafa switches it.
2. **`docs/ROADMAP.md`:**
   - "Recent work", terse, newest first: R1–R6 started; R7 (PR #27); docs-after-kit-v2-2 (PR #25); kit-v3-0 (PR #26);
     the v2 pivot (VISION_V2 and the brief, 2026-10-24). The oldest entries fold into "Build history" if the list
     passes 12 (the file's hygiene rule).
   - "Next / open" [B.5]: the line-62 queue ("N1 (next)") is marked superseded by v2, pointing to STATUS and the brief,
     the same way STATUS's queue is. "Phases" stays as it is.
3. **Issue #16** [B.6], edited by the session after the advisor approves this plan:
   - **One new comment first**, in the pattern of the 2026-10-25 one: "Advisor verdicts, 2026-10-27", relayed by
     Mustafa in the day session's chat, the six instruction bullets quoted in full and unchanged, then "What this
     means" in three lines. The body links to it as the source.
   - **Body, the sentence** "Until kit-v3-1 is merged, research runs in the daytime only ([Advisor verdicts,
     2026-10-25](…#issuecomment-6396928581)). Last updated: 2026-10-25." **becomes** "kit-v3-1 is paused (the Advisor,
     2026-10-27: \"shelved\"). Until kit-v3-1 returns and is merged, research runs in the daytime only ([Advisor
     verdicts, 2026-10-25](…#issuecomment-6396928581); [Advisor verdicts, 2026-10-27](<new comment>)). Last updated:
     2026-10-27."
   - **Body, the "Rules" list** gets one bullet: "Until kit-v3-1 returns and is merged, the night shift takes only
     items that need no web access, and a night run that stops at a permission prompt lists that prompt in its morning
     report ([Advisor verdicts, 2026-10-27](<new comment>))."
   - **Body, queue item 1** "in progress as R7 in [PR #27](…) (draft; implementer session, branch
     `claude/bold-faraday-i49gug`, v2 phase 0 brief §7.3); night shift: skip." **becomes** "done as R7, merged in [PR
     #27](…) (main `07ab240`, 2026-10-27)." It stays a plain bullet; no item is added.
   - **Reversible** [B.7]: the PR body carries the old and the new text of every changed sentence.

## Evidence and report [B.8]
- No report file (earlier docs-only change-sets had none): the PR body is the report: what changed; what each fact was
  checked against; the gate result; the issue #16 diff; Decisions needed; For Mustafa.
- Gate: PASS on the branch, run locally (`tools/node_modules` is installed in the main checkout with
  `npm ci --omit=optional`; the worktree links to it for the run and the link is removed after), plus CI's `gate` and
  `smoke` checks on the PR. The advisor runs the gate itself at PR review, from the main checkout's toolkit.
- Repo map check (no paths added or removed).

## Reviewers
The advisor agent, on the PR's head, in this session. docs-keeper writes STATUS and ROADMAP.

## Merge path (accepted as planned)
A draft PR, so the still-enabled PR Advisor routine skips it. The advisor agent reviews its head here. On Mustafa's
"merge #<n>" (before or after he switches the routine off), the session marks it ready and merges with the approved
full SHA as the expected head, with `gate` and `smoke` green on it.

## Risks
1. The kit-v3-1 state may move: if a kit-v3-1 PR opens, STATUS's kit-v3-1 row changes before this merges.
2. Line drift in STATUS (it is long): docs-keeper greps before each edit.
3. The issue #16 edit is outward-facing: its exact text is in this plan and the old text is kept in the PR body.

## For Mustafa
Nothing to check in Brave. Switch the PR Advisor routine off; answer KIT31-STATE; say "merge #<n>" once the advisor
approves.

## Conditions (advisor APPROVE of revision 2, 2026-10-27 21:26 UTC; verbatim in `advisor-verdicts.md`)
1. KV3-MERGE reads: "KV3-MERGE (a): the session merged kit-v3-0 on Mustafa's own 'merge #26', given after the advisor
   approved its final head, not on his earlier advance go."
2. DOMAINS-ACCEPT reads: "DOMAINS-ACCEPT (a): six documentation hosts where anyone can publish (github.com,
   raw.githubusercontent.com, docs.rs, crates.io, developer.valvesoftware.com, datatracker.ietf.org) stay on the list of
   sites fetched without a prompt, with the risk recorded. This holds only once WRITE-GUARD is merged; until then
   research runs in the daytime only."
3. WRITE-GUARD reads: "WRITE-GUARD (a): write limits for kit and gate files, enforced in settings (rules that ask
   first, or deny rules if routines let ask-first rules pass silently), plus the secret-pattern fix; kit changes by day."
4. The gate claim is honest. The advisor's premise (no Playwright, so no local smoke) does not hold on this machine:
   `tools/smoke.mjs:121` loads `require('playwright')`, which resolves to the pre-installed Playwright 1.63.0
   (`/opt/node-tools/node_modules/playwright`) with its Chromium build 1243 (`/opt/pw-browsers`), and the local gate ran
   5a and passed (index 8/8, editor 5/5). This environment says not to install another Playwright version (it would not
   find this machine's browser), so 1.56.1 is not installed. The PR body says: the local smoke ran on the pre-installed
   1.63.0; CI's `smoke` check runs the pinned 1.56.1. The advisor is told before its PR review.
5. No symlink: the worktree has its own `tools/node_modules`, a real folder from `npm ci --omit=optional`, matched by
   `node_modules/` in `.gitignore`. Before each commit and push, `git status --porcelain` shows only `docs/STATUS.md`,
   `docs/ROADMAP.md` and `.checkpoints/`; the final head has no `.checkpoints/` files.
6. STATUS names the session: "the v2 day session (session_0173vVo5q59ameR6cnN9BENg)", never "this session" or "the day
   session".
