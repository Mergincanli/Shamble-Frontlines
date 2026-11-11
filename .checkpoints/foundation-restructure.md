# Checkpoint: foundation-restructure (foundation step 2.1b, the move)
- **Goal:** 2.1b under plan revision 3, steps 6 to 11: the five legacy files into `legacy/` with `git mv` (bytes
  unchanged, one pushed commit holding nothing else), the commands in text, the four kit lines (on his chat yes only),
  E1-E6 and E8, the docs (with his SCOPE (a) in ADR 0002's Status line and his render-budget note queued in
  DECISIONS.md), the reviews, the report, the PR ready, then stop for his go. Plan: `.checkpoints/foundation-restructure/plan.md`
  (revision 3, copied byte for byte from c8f4bb382d80d2248a3aef75f864753428bfac8b, blob d0b00ec). Task: his message of
  2026-11-11 opening this session (session_019YNMUWdGJZoQFUuuvvBP1Y, his personal account).
- **Branch:** `claude/clever-fermi-oudsfq` (this session may push only here; stands in for `cs/foundation-restructure`).
  **Base:** main 76b4f7a (PR #50 merged, 2.1a). **OLD** (the independent judge for E2 and E3): main 44446a5.
- **Never:** push to `claude/netcode-proof-test-plan-hcky5g` (#40) or `claude/laughing-pasteur-fwya24` (#48); touch
  `spikes/`; touch a gate file; edit a settings file, hook or allow-list; write a kit line before his chat yes.
- **Last update:** 2026-11-11 21:37 Amsterdam

## Steps done
0. Step 0 (this commit): model checked with get_session: `session_context.model` claude-opus-5-5,
   `last_served_model` claude-opus-5-5. The advisor tool is on, but the `/advisor` note says this conversation keeps
   Opus 5.5 behind it until /clear or /compact (Fable 5.2 not yet the one advising). Read: CLAUDE.md, CHECKPOINTS.md,
   STATUS.md, DECISIONS.md (2026-11-10 and 2026-11-11), the 2.1a report, the old checkpoint, plan.md, the advisor
   agent's 2.1a head review (2.1b's conditions: F-G Brave format, F-H E8 by the main session after one ask, F-I E4 with
   A's copies, F-J restart from main, Q-A1 the register, Q-A4 verifier.md:28). plan.md copied (blob d0b00ec).

1. Plan check (119b71d): `plan-check.md` in this folder. Every 2.1b target found; drift is line numbers only (CLAUDE.md
   :41-42 → :43-44, START_HERE +1, the STATUS split foreseen by the plan), except **D1**: plan step 10 names "the
   register's OQ-0002-2 row" and the register has none, at 44446a5 or 76b4f7a (it lists only questions routed to him or
   the Advisor; OQ-0002-2's decider is the restructure plan). New hits since OLD: 64 files / 747 vs 63 / 728, all
   HISTORY, NAME or TESTDATA. IE-2, IE-3, IE-4, IE-7, SA-L1 folded in (plan-check §4).

## Current step
Stopped on D1 (his rule: "If the plan and today's main disagree on more than a line number, stop and tell me in three
lines."), waiting for his answer. Nothing built.

## Next steps
1. After his D1 answer: write his choice into plan-check.md exactly as he picks it, nothing wider; add #40's and #48's
   current file lists (plan Part 4: re-read the open PRs' files before each build step); tell the advisor agent plainly
   that step 8 now runs after 9 and 10, and why.
2. The advisor agent reviews the plan check (APPROVE / REVISE / ESCALATE). The four kit lines and E8's ask go to him in
   one message after its APPROVE, with the exact text it approved.
3. Step 6, the move (one commit, nothing else), gate with smoke, pushed alone; the draft PR opened on it.
4. Step 7, the commands in text (one commit). Step 8 waits for his chat yes on the four kit lines (one message).
5. Step 9, E1-E6 and E8; step 10, the docs; step 11, the reviews, the report, ready, stop for his go.

## Open findings
- (none yet)

## Key numbers
- (none yet)

## Decisions taken
- Checkpoint slug `foundation-restructure` (the same task as 2.1a; plan 1.1 restarts the task from main) · the session
  · 2026-11-11.

## Advisor-tool consultations (his words of 2026-11-11: when, the question, what changed)
1. 2026-11-11 21:35 Amsterdam · the plan check done, before committing to the approach (his moments 1 and 4): does D1
   trigger his three-line stop, and should the kit ask ride the same message? · changed: stopped on D1 at once (no
   advisor-agent review and no build first); the kit lines and E8's ask held back for one message after the advisor
   agent's APPROVE; the three lines written as the one "Decisions needed" entry with options (a)-(c); ADR 0002's
   ":1596/:1626 open → answered" edits named inside option (a) instead of assumed; #40's and #48's file lists and the
   step-8 order deviation queued for the advisor agent's brief.

## Refusals (guard, deny list, classifier: what, and the route taken)
- (none yet)

## Running work
- (none)
