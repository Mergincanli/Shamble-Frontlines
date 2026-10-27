# Checkpoint: v2-research
- **Goal:** R1–R6 research files (`docs/research/r1-…` to `r6-…`), each in the README's 8-part shape, verified as
  the plan says, plus the README status column and the report · plan: `.checkpoints/v2-research/plan.md` (advisor
  APPROVE with 14 conditions, 2026-10-27, `advisor-verdict.md`) and `plan-amendments.md` (the Advisor's 2026-10-27
  instructions in this session; advisor check: `advisor-amendments-verdict.md` once in)
- **Branch:** cs/v2-research (session_0173vVo5q59ameR6cnN9BENg; the main checkout, never switched while research runs)
  · **Base:** origin/main 07ab240 (R7 merged; `docs/LEGACY_INVENTORY.md` checked present)
- **Last update:** 2026-10-27 19:20 Amsterdam

## Steps done
1. Plan and verdict recovered verbatim from the R7 session's transcript (`provenance.md`): one review round, APPROVE
   with 14 conditions at 11:36 UTC; final plan written at 11:41 UTC with the conditions applied.
2. Branch created from origin/main 07ab240 after PR #27 merged (17:36 UTC); R7's three files present · 8ff1c74
   (first push of cs/v2-research: the proxy accepts cs/ pushes).

## Current step
3. Wave 1 units running since 19:18 Amsterdam, 11 background agents (model opus, effort xhigh), one per unit:
   engine-architect r1-ecs, r1-tick, r1-determinism, r1-boundary-worker, r1-abilities, r1-data-hotreload;
   netcode-architect r2-sync, r2-budgets, r2-interest, r2-transport, r2-rooms. Notes `<agent>-<topic>-<unit>.md`,
   drafts `…-draft.md`, all in `.checkpoints/v2-research/`. Each prompt carries the plan's rules (no spikes, allow
   list, no recommendation on unread sources, proposed pass marks, no look decision, write only notes and draft).
   Advisor check of the amendments and of the docs-after-r7 plan running (notes `advisor-amendments-review.md`).
   Saver running (background, 110 minutes from 19:18).

## Next steps
1. Count the 11 unit results; restart any unit that came back empty with the same prompt.
2. Assemble: engine-architect → `docs/research/r1-simulation-core.md`, netcode-architect → `docs/research/r2-netcode.md`.
3. Verify: verifier on R1 (high), verifier on R2 (xhigh), rust-reviewer on R1 + R2's server stack (xhigh),
   security-auditor on R2's protocol (xhigh).
4. Fix (owners, xhigh); a second verifier pass only where a fix changed a recommendation, a number or a source.
5. Wave 1 tally (shape followed, share of claims unsupported, what went unread); then wave 2 (R3, R4, R5, R6 with the
   amended R6 scope), the consistency critic, docs-keeper, gate, report, PR.

## Per-topic table
| Topic | Owner | Units | Drafts | Assembled | Verification | Notes |
|---|---|---|---|---|---|---|
| R1 | engine-architect | 6 | 0/6 | no | pending | wave 1 |
| R2 | netcode-architect | 5 | 0/5 | no | pending | wave 1 |
| R3 | graphics-engineer | 1 | 0/1 | no | pending | wave 2 |
| R4 | engine-architect | 1 | 0/1 | no | pending | wave 2 |
| R5 | test-engineer (+ infra-engineer CI part) | 1 | 0/1 | no | pending | wave 2 |
| R6 | infra-engineer | 1 | 0/1 | no | pending | wave 2; security baseline deferred (Advisor) |

## Open findings
- kit-v3-1: the Advisor's word in this session (2026-10-27, about 17:37 UTC) is "shelved"; the R7 session was still
  committing to `cs/kit-v3-1` at 18:01 UTC on an earlier Advisor message (17:32 UTC: finish it there on Opus 4.8,
  open its PR). For Mustafa to settle; this session does not touch that branch.
- The PR Advisor routine (trig_01YPyWAsQmsaQoydhMGG3CkJ) could not be switched off by an agent (created via the
  HTTP API); Mustafa switches it off at its routine page.

## Key numbers
- Wave 1 agents planned: 11 units + 2 assemblers + 4 checkers + 2 fixers + up to 2 second passes (about 21).

## Decisions taken
- BRANCH-2 (a) and SPEND-1 (b): the Advisor, 2026-10-27 (R7 session; `request-and-decisions.md`).
- kit-v3-1 shelved; research daytime only and night shift no-web items only until it returns; PR Advisor routine off,
  the advisor agent reviews PRs in the day session; R7-MARKS (a) with look-brief rows pending; MANAGED (b); R6 without
  the security baseline: the Advisor, 2026-10-27, this session (`plan-amendments.md`).
- PR #27 (R7) merged by the R7 session at 17:36 UTC on Mustafa's go (merge 07ab240).

## Running work
- Wave 1 units: 11 background agents launched 2026-10-27 19:18 Amsterdam (list under "Current step"); a cut-off one
  is restarted with the same prompt and goes on from its notes.
- Advisor (amendments + docs-after-r7 plan): background agent, launched 19:18.
- Saver: `checkpoint.mjs watch v2-research`, background, started 19:18, stops after 110 minutes; restart it if agents
  still run.
- Day-window check: send_later trig_01Veu82e8opzfVF7b6ZKAHwp at 22:50 UTC (23:50 Amsterdam): stop anything still
  running before 00:10 and schedule the 08:35 resume.
