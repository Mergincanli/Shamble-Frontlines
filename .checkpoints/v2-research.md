# Checkpoint: v2-research
- **Goal:** R1–R6 research files (`docs/research/r1-…` to `r6-…`), each in the README's 8-part shape, verified as
  the plan says, plus the README status column and the report · plan: `.checkpoints/v2-research/plan.md` (advisor
  APPROVE with 14 conditions, 2026-10-27, `advisor-verdict.md`) and `plan-amendments.md` (the Advisor's 2026-10-27
  instructions in this session), approved by the advisor with 8 conditions (`advisor-amendments-verdict.md`, part A)
- **Branch:** cs/v2-research (session_0173vVo5q59ameR6cnN9BENg; the main checkout, never switched while research runs)
  · **Base:** origin/main 07ab240 (R7 merged; `docs/LEGACY_INVENTORY.md` checked present)
- **Last update:** 2026-10-27 22:45 Amsterdam

## Steps done
1. Plan and verdict recovered verbatim from the R7 session's transcript (`provenance.md`): one review round, APPROVE
   with 14 conditions at 11:36 UTC; final plan written at 11:41 UTC with the conditions applied.
2. Branch created from origin/main 07ab240 after PR #27 merged (17:36 UTC); R7's three files present · 8ff1c74
   (first push of cs/v2-research: the proxy accepts cs/ pushes).
3. Advisor check of the amendments: APPROVE with 8 conditions (part A of `advisor-amendments-verdict.md`), 18:22 UTC.
4. Wave 1 units launched 19:18 Amsterdam; all 11 cut off by the org spend limit at about 19:25 (HTTP 429, "resets
   9:20pm UTC"), each with its notes saved (19–93 lines). Resumed 22:24 Amsterdam by message, each from its own
   transcript and notes, with one added instruction: exact quoted source text for every finding a recommendation
   rests on; a claim seen only in WebFetch's summary is "inferred".
5. Gate on this branch: PASS, all 8 checks (`tools/node_modules` installed with `npm ci --omit=optional`, so the
   smoke test uses the pre-installed Playwright 1.63 and its Chromium; toolkit pins 1.56.1).

## Current step
6. Wave 1 units running (resumed 22:24 Amsterdam): engine-architect r1-ecs, r1-tick, r1-determinism,
   r1-boundary-worker, r1-abilities, r1-data-hotreload; netcode-architect r2-sync, r2-budgets, r2-interest,
   r2-transport, r2-rooms. Model opus, effort xhigh. Prompts saved verbatim: `prompts/<agent>-<topic>-<unit>.txt`
   (advisor condition 1). Notes `<agent>-<topic>-<unit>.md`, drafts `…-draft.md`, all in `.checkpoints/v2-research/`.

## Next steps
1. Count the 11 unit results (a draft file each, ending sources lists); restart any unit with no draft: its saved
   prompt plus "resume from your notes at <notes path>".
2. Assemble (save each prompt first under `prompts/`): engine-architect → `docs/research/r1-simulation-core.md`,
   netcode-architect → `docs/research/r2-netcode.md`; nothing new claimed; the prompts say the inventory's marks are
   working recommendations and "look brief pending" rows stay pending (advisor condition A6).
3. Verify: verifier on R1 (high), verifier on R2 (xhigh), rust-reviewer on R1 + R2's server stack (xhigh),
   security-auditor on R2's protocol (xhigh). The verifier counts a claim confirmed only with the exact quoted source
   text (condition A8); default "unsupported" when it can't confirm.
4. Fix (owners, xhigh); a second verifier pass only where a fix changed a recommendation, a number or a source.
5. Wave 1 tally (shape followed, share of claims unsupported, what went unread); then wave 2 (R3, R4, R5, R6 with the
   amended R6 scope), the consistency critic, docs-keeper, gate, report, PR.

## Carried forward (into the report, the synthesis and later prompts)
- Condition 9: "the synthesis plan verifies every ADR claim and number, including those from R3–R6".
- The security baseline is deferred by the Advisor (2026-10-27): the report's coverage table shows it as "deferred by
  the Advisor, 2026-10-27", not as a gap; it goes to the synthesis: ADR 0006 lists it as open, or it is researched
  before 0006 is accepted (condition A3, A4).
- R1's cross-origin isolation question (the headers shared memory needs) stays in R1; R6's hosting part states, per
  host, whether it can send those headers (condition A4).
- If docs-after-r7 has not merged when research reaches its docs-keeper step, condition 11 applies in full: this PR
  adds the earlier STATUS and ROADMAP entries itself (condition A5).
- The research PR's "For Mustafa" says it merges only on his "merge #<n>", because the PR Advisor routine is off
  (condition A7).
- Night stop (condition A2): background agents have no workflow cache. At 00:10 Amsterdam, stop every agent still
  running and list which; after 08:30, restart each with its saved prompt plus "resume from your notes at <path>".
- `checkpoint.mjs save -m` without the "checkpoint(<slug>):" prefix (the script adds it; 8ff1c74 has it twice).

## Per-topic table
| Topic | Owner | Units | Drafts | Assembled | Verification | Prompts |
|---|---|---|---|---|---|---|
| R1 | engine-architect | 6 | 6/6 | assembling (since 22:37 Amsterdam) | pending | `prompts/engine-architect-r1-*.txt` (6) |
| R2 | netcode-architect | 5 | 4/5 (rooms, interest, transport, budgets) | no | pending | `prompts/netcode-architect-r2-*.txt` (5) |
| R3 | graphics-engineer | 1 | 0/1 | no | pending | wave 2 |
| R4 | engine-architect | 1 | 0/1 | no | pending | wave 2 |
| R5 | test-engineer (+ infra-engineer CI part) | 1 | 0/1 | no | pending | wave 2 |
| R6 | infra-engineer | 1 | 0/1 | no | pending | wave 2; security baseline deferred (Advisor) |
Agent ids are internal to this session and are not written here; the saved prompt plus the notes path is the restart
key (condition A1).

## Open findings
- KIT31-STATE (for Mustafa): the Advisor's word in this session (about 17:37 UTC) is "kit-v3-1 is shelved"; the R7
  session was still committing to `cs/kit-v3-1` at 18:01 UTC on an earlier Advisor message (17:32 UTC: finish it there
  on Opus 4.8 with a smaller scope and open its PR). This session does not touch that branch.
- The PR Advisor routine (trig_01YPyWAsQmsaQoydhMGG3CkJ) could not be switched off by an agent (created via the
  HTTP API); Mustafa switches it off at its routine page.
- Spend: the org's spend limit cut all 11 units at once about 7 minutes after launch; reset at 21:20 UTC.

## Key numbers
- Wave 1 agents planned: 11 units + 2 assemblers + 4 checkers + 2 fixers + up to 2 second passes (about 21).

## Decisions taken
- BRANCH-2 (a) and SPEND-1 (b): the Advisor, 2026-10-27 (R7 session; `request-and-decisions.md`).
- kit-v3-1 shelved; research daytime only and night shift no-web items only until it returns; PR Advisor routine off,
  the advisor agent reviews PRs in the day session; R7-MARKS (a) with look-brief rows pending; MANAGED (b); R6 without
  the security baseline: the Advisor, 2026-10-27, this session (`plan-amendments.md`).
- Amendments approved with 8 conditions: the advisor agent, 2026-10-27 18:22 UTC (`advisor-amendments-verdict.md`).
- PR #27 (R7) merged by the R7 session at 17:36 UTC on Mustafa's go (merge 07ab240).

- KIT31-STATE (b) and SRC-ACCESS (a): the Advisor, relayed by Mustafa about 21:36 UTC: the R7 session finishes kit-v3-1
  on Opus 5.5 and its PR goes to Mustafa ("shelved" meant "not in this session"); unread sources stay open questions
  for now, the full list comes with the wave-1 tally, and the Advisor looks for another copy of any source a
  recommendation hinges on. The assembler prompts list those recommendations (rule 5 added before launch).
- PR Advisor routine: off (Mustafa, 21:35 UTC; get_trigger reads enabled false).

## Running work
- Wave 1 units: 11 background agents, launched 19:18, cut off 19:25 (spend limit), resumed 22:24 Amsterdam.
- Saver: `checkpoint.mjs watch v2-research`, background, restarted 22:24 Amsterdam; stops after 110 minutes.
- Day-window check: send_later trig_01Veu82e8opzfVF7b6ZKAHwp at 22:50 UTC (23:50 Amsterdam): stop anything still
  running before 00:10 and schedule the 08:35 resume.
