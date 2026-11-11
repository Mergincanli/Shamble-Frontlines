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
- **Last update:** 2026-11-11 22:08 Amsterdam

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

2. His D1 message (2026-11-11 20:44:29 UTC, event ce7f1f4b-08bc-476f-862e-3c1d9362c40b): D1 (a) exactly as plan-check
   §3 proposed, nothing wider; his Windows run of the new gate on 76b4f7a PASSED (PowerShell 5.1, Node v24.17.0, base
   44446a5, "files at the root" twice, smoke 8/8 and 5/5, GATE PASS: into the report); his 2.3 finding (Playwright <
   1.60.0's installer hangs after 100% under Node >= 24.16.0; fine under 24.15.0; if ours stops after 100%: record it as
   that bug, tell him); next: the advisor agent's review, then the four kit lines and E8's ask in one message.
   plan-check.md updated (bbc43b5): D1 (a) quoted (checked equal to the capture), §3b the open PRs (#40 at 8c681cf
   shares DECISIONS, REPO_MAP, STATUS, STATUS_ARCHIVE with the docs step: Part 4's #40 route; #48 at d8186fd: no common
   file), the plan's order 6-11 with the kit ask before the build. His two messages captured by script from the
   session's events: `mustafa-2026-11-11-task.md` (4424 characters, event e018683d-30d9-49a0-b19d-ab279e7d23a1,
   20:21:02 UTC) and `mustafa-2026-11-11-d1.md` (1359 characters); `capture-message.mjs` (copy of
   `.gate/foundation-restructure/capture-message.mjs`). Toolkit: `npm ci --prefix tools` (159 packages; gl needs
   xvfb-run, as documented).

## Current step
The advisor agent reviews the plan check (background; notes `advisor-plan-check.md`). Done meanwhile (no tracked file
changed): the toolkit, the pinned browser, the baseline gate and kit tests, the evidence scripts.

## Next steps
1. On the advisor agent's verdict: APPROVE → the four kit lines and E8's ask to him in one message, with the exact text
   it approved (and the one line on which model answers the advisor tool: Opus 5.5 per the /advisor note); REVISE → fix
   the plan check, re-review.
2. After his yes: build in the plan's order 6, 7, 8, 9, 10, 11 (plan-check §7).
3. Step 6, the move (one commit, nothing else), gate with smoke, pushed alone; the draft PR opened on it.
4. Step 7, the commands in text (one commit). Step 8 waits for his chat yes on the four kit lines (one message).
5. Step 9, E1-E6 and E8; step 10, the docs; step 11, the reviews, the report, ready, stop for his go.

## Open findings
- (none yet)

## Key numbers
- The pinned browser: `node tools/node_modules/playwright/cli.js install chromium` (allowed at settings.json:12) downloaded
  Chromium 1194 and its headless shell to /opt/pw-browsers and exited 0, no hang after 100% (Node 22.23.4); the tools'
  Playwright resolves `/opt/pw-browsers/chromium-1194/chrome-linux/chrome`. No alias this time (2.1a's L-4 route not
  needed): the local smoke runs the pinned build.
- Baseline on main 76b4f7a (unchanged tree): `node tools/gate.mjs 76b4f7a` GATE PASS, +0 -0, smoke index 8/8, editor 5/5,
  header "base 76b4f7a = 76b4f7a (files at the root) | head = working tree (files at the root)".
- Kit tests on main: `node --test .claude/hooks/*.test.mjs .claude/scripts/checkpoint.test.mjs`: 1261 tests, 1261 pass,
  0 fail (E6's comparison count).
- His Windows run (his D1 message): the new gate on main 76b4f7a, PowerShell 5.1, Node v24.17.0, base 44446a5, "files at
  the root" twice, every check PASS, smoke 8/8 and 5/5, GATE PASS.
- Evidence scripts ready under `.gate/foundation-restructure/` (copies go to the notes when run): e1-e2.mjs, e3-run-2-1b.mjs
  (with e3-mutations.json and the recorded e3-results-f1.json copied from c8f4bb3, blobs 21c043f and 72ff476),
  e4-tuning.mjs, e5-saves.mjs, e6-commands.mjs.

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
- Saver: background task by9y0gd9z, started 21:47 Amsterdam, stops by 23:37 (110 min); output
  /tmp/claude-0/-home-user-Shamble-Frontlines/c0e5589d-9610-5933-a695-61d4b8cf6d47/tasks/by9y0gd9z.output; last check
  21:50: ok (watching).
- The advisor agent: the plan-check review, launched 21:49 Amsterdam (effort xhigh: it rules on ADR 0002 edits), notes
  `advisor-plan-check.md`.
- The browser install (`node tools/node_modules/playwright/cli.js install chromium`): background task byqyxhjfk,
  launched 21:50.
