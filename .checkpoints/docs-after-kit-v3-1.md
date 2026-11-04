# Checkpoint: docs-after-kit-v3-1 (the docs pass after PRs #33, #30 and #34)

- **Branch:** `claude/sleepy-tesla-ofsxsr`. It stands in for `cs/docs-after-kit-v3-1`, because this cloud session may
  push only there. Built in the worktree `/tmp/claude-0/-home-user-Shamble-Frontlines/263eddf3-7643-5640-b1d5-39be43d3567c/scratchpad/docs-pass`
  (local branch `docs-after-kit-v3-1`, started from main 868e165, which contains the branch's old head a432692, so the
  push is a fast-forward). The session's main folder stays on its own branch (Mustafa's working rule, 2026-11-03).
  The checkpoint saver pushes only a branch of the same name, so this file is committed by hand, checkpoint-only
  commits marked `[skip ci]`.
- **Started:** 2026-11-04, after TEST-RUN part B passed (B1: Mustafa, 13:20 UTC; B2: the PR Advisor's "Run now",
  13:26:48–13:35:54 UTC, checked row by row from its transcript, session_0173vVo7uY6BfxLBnYjEBJLV).
- **Current step:** 2, the build. Plan revision 1 (381471d) got the advisor's **APPROVE with 16 conditions** (its words in
  `.checkpoints/docs-after-kit-v3-1/advisor-plan-rev1.md`); with Mustafa's answers below they make plan revision 2. The
  advisor checks the conditions on the final diff (no second plan round).

## Mustafa's answers (2026-11-04, after plan revision 1), word for word
- "KIT-FOLLOWUPS: (a), a small A3 right after A2 for the guard's follow-ups: ask before kit and gate edits in day
  sessions; the sk- key after %20, %2C or %3A; and read-only GitHub Actions tools (such as actions_get) for pr-advisor,
  which B2 showed it needs for CI details. security-auditor reviews A3. GitHub secret scanning and push protection are
  now on."
- "ROUTINE-EFFORT: (a). The PR Advisor at max is my deliberate choice (quality over cost), so it stays. The question is
  only whether the night shift really runs at Extra, as its routine says, and what controls that."
- "Part B accepted as passed. The PR Advisor routine is un-paused (07:45, 13:45, 19:45), and the Guard test B1 routine
  is deleted."
- Earlier the same day: "TEST-RUN B2 passed … Docs pass: record parts A and B and #34's merge."

## Plan revision 2: what changes from revision 1
- **Names (conditions 1 and 16):** the effort question is EFFORT-SOURCE (ROUTINE-EFFORT is the Advisor's decided code of
  2026-10-25, STATUS line 334); his read-only Actions item belongs to **A3, the guard's follow-ups** (his KIT-FOLLOWUPS
  answer), never "A2". In this file A1/A2 below still mean the CLAUDE.md and advisor.md edits.
- **EFFORT-SOURCE, answered with evidence (2026-11-04):**
  - the night shift's last run (06:11 UTC, session_0173vVo8bhZGeK4bVFXQQEtM) has `session_context.effort_level: "xhigh"`,
    its page's "Extra" (Mustafa);
  - TEST-RUN B2 (session_0173vVo7uY6BfxLBnYjEBJLV) has `effort_level: "max"`, his choice;
  - this day session (session_0173vVo7FcYUpbWgRW8oQyHU): record `effort_level: "max"`, `flag_settings.effortLevel: "max"`;
    its launch log `/tmp/env-manager.log` "Added gateway arg to command" (flag `--effort`); `/tmp/claude-command` and
    `/tmp/claude-code.log` read `--effort max`; `CLAUDE_EFFORT=max`.
  - So the effort is the one set on the routine (or session), passed by the cloud harness as `--effort`; neither run
    used `.claude/settings.json`'s `high`. Tonight's first night-shift run (00:10) is the next record to read.
- **A1 (CLAUDE.md):** the effort sentence says the above, without a general precedence rule (condition 2); the shell
  list adds `date` and "(the PR Advisor runs no program at all)" and no longer claims to match the guard, which also
  allows python on main's tracked scripts for the night shift (condition 4: a gap for A3's plan); the trigger sentence
  gives the known reason only (condition 3): "so every run starts on main's rules; a pull-request start could load the
  PR head's settings and hooks before step 0 switches to main"; one wording everywhere: it has a schedule trigger only,
  can also be started by hand ("Run now", as B2 was) or through the API, and never by pull-request events.
- **A2 (advisor.md):** known limits name every `actions_*` tool (B2 met `actions_get` four times; the read pattern
  `^(get_|list_|search_)|_read$` refuses `actions_list` too) and hold until A3 decides; "Your notes" fixed (no file
  tool in a routine's shell; the routine's session writes notes with Write under `.checkpoints/`; the advisor agent has
  no Write tool, so as a routine's subagent it returns its notes in its report; a known limit for A3); the heading says
  what the stored prompt says ("every open pull request …, oldest first"), as "every open PR that step 1 does not skip,
  oldest first" (condition 5).
- **C (PORTAL-EMBED):** "web portal", never a bare "portal" (condition 8); ADR 0002 says one thing in four places: part 5
  (the new sentence and "flips everywhere at once" qualified), the failure-mode bullet (lines 1486–1489) and OQ-0002-4
  option (b) (line 1023), each with a short qualifier pointing at OQ-0002-9 and nothing on how a fallback is built
  (condition 6); OQ-0002-9 marks the widening to B3, C2 and W3 as the advisor's reading and records W3's one
  consequence (W2 is its only non-isolated fallback; if W2 fails its jitter mark, the question returns to Mustafa with
  the spike's numbers) (condition 7); OQ-0006-23 also names OQ-0006-3 and OQ-0006-15, with no storage or cookie claims
  (condition 9); C5 and C6 cite OQ-0002-9 rather than restate the rule.
- **B (STATUS, ROADMAP):** the API state as read, with its time (condition 15: PR Advisor `enabled: true`, updated
  14:01:31 UTC, next run 18:45 UTC; "Guard test B1" gone; read 2026-11-04 about 14:02 UTC, re-read before the last docs
  commit); KIT-FOLLOWUPS (a) and A3's list (his three items, plus three B2 found for A3's plan to weigh: `git rev-list`,
  the python gap, the advisor agent's notes without Write); EFFORT-SOURCE's answer; one line next to ROUTINE-EFFORT (b):
  its premise (a routine's main session at medium) no longer holds, the (b) rule stays (condition 1); one clause that
  V2_PHASE0_BRIEF §3 step 2 ("effort high", "Keep its triggers") is outdated by the 2026-11-04 facts and rulings, the
  brief not edited (condition 10).
- **D:** D1 names each run and extends the correction note "(PR #34; PR #35: the second run's times)" (condition 13);
  D2 is in, dated, citing NIGHT_SHIFT.md lines 2–3 and 16 (condition 14).
- **Agents (condition 11):** all Opus 5.5; engine-architect xhigh (frontmatter); infra-engineer passed `effort: "xhigh"`
  (ADR text); docs-keeper, security-auditor and the verifier high (frontmatter). docs-keeper edits B in its own
  worktree (CLAUDE.md: parallel implementation only in isolated worktrees), merged here after.
- **Gate (condition 12):** `npm ci` in `tools/` in this worktree (GATE-SETUP (a) precedent), then
  `node tools/gate.mjs --no-smoke` PASS with symbols +0 −0; smoke from CI.

## Step log
1. Plan revision 1, 381471d; the advisor's APPROVE with 16 conditions (2026-11-04 ~14:00 UTC).

## Goal
Bring the rules and docs in line with what happened since PR #33:
- PRs #33, #30 and #34 merged;
- the routine setup done and tested (TEST-RUN part A, B1, B2);
- Mustafa's 2026-11-04 rulings: the routine shell list names `date`; the PR Advisor runs only on a schedule or an API
  trigger; PROBE-FILE (c), recorded as known noise;
- what B2 showed: routines run at effort max, and two read limits;
- PORTAL-EMBED as an open question (Mustafa, 2026-11-04).

Docs and rules only: no code, symbols +0 −0. It takes over STATUS's docs-after-33 row (the advisor's ruling Q2 kept
that out of A1).

## Change type and sign-off
Rules plus docs: `CLAUDE.md` and `.claude/agents/advisor.md` change, so it is needs-mustafa. It merges on his go in
this session after the advisor approves the final head (full SHA), with `gate` and `smoke` green. Nothing a player
sees, hears or feels: no Brave check.

## Edits (exact)

### A. Rules (main session)
**A1. `CLAUDE.md`, "Models and effort", the Routines bullet** (line 130 on main):
- "at `effortLevel: "high"` (set in `.claude/settings.json`, read at session start)" becomes "at effort max: the
  cloud harness starts Claude Code with `--effort max`, and a command-line flag beats the project's
  `effortLevel: "high"` (seen 2026-11-04 in B2's session record, `effort_level: max`, and in this session's container
  launch log)". If Mustafa answers ROUTINE-EFFORT (a) and the lookup finds a setting, the line names it.
- The shell list "git, gh, `npm ci` and node on main's scripts" becomes "git, gh, `npm ci`, node on main's scripts and
  `date`" (Mustafa, 2026-11-04). This matches the guard and `.claude/NIGHT_SHIFT.md` line 8.
- One new sentence: "The PR Advisor routine runs only on a schedule or an API trigger, never on pull-request events
  (Mustafa, 2026-11-04): a pull-request event would start it from the PR's own head commit, under the PR's rules,
  while a schedule or API run starts from the default branch."

**A2. `.claude/agents/advisor.md`, "PR review":**
- The heading (line 67) "(as the PR Advisor routine: one run per ready PR or new push)" becomes "(as the PR Advisor
  routine: each run, on its schedule or an API trigger, takes every open PR, oldest first)".
- Step 0, after "a refusal that blocks a step means that step is needs-mustafa.", add: "Known limits (TEST-RUN B2,
  2026-11-04): the guard refuses `actions_get` (CI run and job details) and `git rev-list`. Read CI from the PR's
  check runs and the jobs' logs, and say in the review what that leaves unchecked."
- Same place, PROBE-FILE (c): "If the run leaves files in the checkout (notes under `.checkpoints/`), Claude Code on
  the web's own Stop hook asks once to commit and push them. Ignore it: the PR Advisor never pushes, and the guard
  refuses the push."

### B. Docs (docs-keeper: STATUS, ROADMAP, REPO_MAP)
- **B1. STATUS header:**
  - "Last updated" becomes 2026-11-04.
  - The merged list adds #33 (e4d8b22, 2026-11-03), #30 (e2b5799, 2026-11-04 10:21 UTC) and #34 (868e165,
    2026-11-04 13:35 UTC, merged by the PR Advisor routine).
  - In flight: this pass.
- **B2. STATUS Phase:**
  - The decision drafts are merged (PR #33).
  - "Next" stays the netcode proof test ("after PR #33 merges" becomes "now").
  - The "After PR #30 merges" block becomes a done list:
    1. The two role environments exist and both routines point at them (Mustafa, 2026-11-04).
    2. PR-HOME was replaced, with no setup script: the PR Advisor has a schedule trigger only,
       `CRON_TZ=Europe/Amsterdam 45 7,13,19 * * *`, and its connectors are removed (PR-HOME-REPLACE (a)).
    3. TEST-RUN passed:
       - part A: the "Guard test" routine on the night-shift environment;
       - B1: 13:20 UTC, second try;
       - B2: the routine reviewed and merged PR #34 on its own.
    4. The PR Advisor comes back on when Mustafa un-pauses it.
- **B3. STATUS "In flight":**
  - The v2-decision-drafts row becomes merged.
  - The kit-v3-1 row becomes merged (PR #30, e2b5799, on Mustafa's "merge #30" after the advisor's APPROVE of
    7a23132). It drops "a draft while A1 runs" (the advisor's #30 finding 3).
  - The docs-after-33 row is folded into this pass.
  - New rows: part-b-test (PR #34, 868e165, merged by the routine: TEST-RUN B2) and docs-after-kit-v3-1 (this pass).
- **B4. STATUS "Routines"** (facts read with the routines API, 2026-11-04 about 13:45 UTC):
  - **Night shift** (trig_01QpV6y6UaG1Na2q3jumRDxh): `CRON_TZ=Europe/Amsterdam 10 0-7 * * *`, enabled, Opus 5.5. Its
    first night under the guard starts 2026-11-05 00:10.
  - **PR Advisor** (trig_01YPyWAsQmsaQoydhMGG3CkJ): schedule only (`45 7,13,19`), no setup script, no connectors,
    Opus 5.5, its prompt quoted. Paused until part B passed; part B passed 2026-11-04, so Mustafa un-pauses it.
  - **"Guard test B1"** (trig_01XmLByUmTibCu5XXQR7iN26, daily 09:41 Amsterdam): a test routine, which Mustafa
    disables or deletes.
  - The old PR Advisor session from the PR #26 event stopped at step 0 and Mustafa archived it.
  - The old "Night shift" (trig_019LtRqmAdzddyk6K3e3kDJz) is still disabled, waiting for its deletion after one good
    night.
  - Known noise (PROBE-FILE (c)): the Stop hook is Claude Code on the web's own (`~/.claude/stop-hook-git-check.sh`,
    registered by the harness; the repository defines none). It asks a session to commit and push uncommitted
    changes, untracked files that are not ignored, and unpushed or unsigned commits. B1's probe
    (`.checkpoints/part-b/probe.md`) set it off. The guard refused the push, and the hook asks only once per stop.
    B2 left no files and did not set it off.
  - Effort: routines run at effort max (A1).
- **B5. STATUS "Open bugs and risks":**
  - The kit-v3-1 entries stay (the Q3 follow-up; no guard checks in a day session whose `guard.mjs` can't load).
  - New: the PR #26 follow-ups, closed by the change-set that KIT-FOLLOWUPS picks:
    - asking before kit and gate edits: done for routines by #30; day sessions have none;
    - the URL-escaped secret gap is open: `checkpoint.mjs`'s generic `sk-` pattern misses `%20`, `%2C` and `%3A`
      before `sk-` (only `%3D` is handled); Anthropic `sk-ant-` keys are caught; GitHub push protection is on (Mustafa).
  - New: B2's read limits (the A2 known limits).
- **B6. STATUS "Decisions pending":**
  - PORTAL-EMBED, shaped like the COMMUNITY-SERVERS line;
  - KIT-FOLLOWUPS and ROUTINE-EFFORT, while open.
- **B7. ROADMAP:**
  - the phase line (line 29) and "v2, next" (line 69);
  - Kit v3.1 becomes MERGED (PR #30, e2b5799, 2026-11-04);
  - v2 decision drafts becomes MERGED (PR #33, e4d8b22, 2026-11-03, by squash);
  - new entries: TEST-RUN part B (PR #34) and this pass.
- **B8. REPO_MAP:** only what is stale (docs-keeper checks); `repo-map-check.mjs` passes.

### C. PORTAL-EMBED (main session; engine-architect and infra-engineer review)
Mustafa's words (2026-11-04): "PORTAL-EMBED (new, keep the door open; don't design it): the game may also launch on
web portals such as CrazyGames and Poki, inside their iframe, sharing our servers and player pool. Add it as an open
question to the README (and ADR 0002 and 0006) in the docs pass. The client must keep working inside a third-party
iframe, so the boundary spike may only pick a design that needs cross-origin isolation (the SharedArrayBuffer ring,
B3) if a non-isolated fallback stays available."
"The README" is read as `docs/adr/README.md`, the index that keeps the open-question register.
- **C1. ADR 0002, "Open questions":** a new OQ-0002-9 (PORTAL-EMBED), shaped like OQ-0006-21 (COMMUNITY-SERVERS).
  - his words, verbatim;
  - why it matters: his rule;
  - options: none weighed ("keep the door open; don't design it");
  - decider: Mustafa;
  - what decides it: no timing is set;
  - carries: no research id;
  - related: OQ-0002-4 (whether production turns isolation on); R1-BND-OQ5 and OQ-0006-12 (the other direction:
    third-party content inside the game page); OQ-0006-23.
- **C2. ADR 0002 part 5, the isolation bullet:** after "(OQ-0002-4).", add: "Whatever the spike measures, a design
  that needs isolation (B3, C2 or W3) is picked only if a non-isolated fallback stays available: the client must keep
  working inside a third-party iframe (PORTAL-EMBED, Mustafa 2026-11-04; OQ-0002-9)." engine-architect checks it
  against the same bullet's "then the default flips everywhere at once".
- **C3. ADR 0006, "Open questions":** a new OQ-0006-23 (PORTAL-EMBED), shaped the same way, for the portal side:
  - launches inside the portals' iframes, sharing our servers and player pool;
  - not researched;
  - related: OQ-0006-12, OQ-0006-21, OQ-0002-9.
- **C4. `docs/adr/README.md`:**
  - the drafts table:
    - 0002 becomes 9 numbered, 8 open ("-9, PORTAL-EMBED, added 2026-11-04");
    - 0006 becomes 23 numbered, 18 open ("-23, PORTAL-EMBED, added 2026-11-04");
  - the total becomes 118 numbered, 98 open, naming the new pair;
  - one row in "Decisions the drafts ask for" for OQ-0002-9 and OQ-0006-23 (PORTAL-EMBED), decider Mustafa, after
    OQ-0006-21's row.
- **C5. `docs/SPIKE_PLAN.md` §3 (the boundary spike):** the C2 rule as one bullet. This is beyond Mustafa's list
  (README, 0002, 0006), but the spike's own plan needs the rule; the advisor rules on keeping it.
- **C6. `docs/ENGINE_OVERVIEW.md` line 68:** after "is measured against it in the boundary spike", add "; it can be
  picked only with a non-isolated fallback (PORTAL-EMBED)". Also beyond his list, for consistency; the advisor rules.

### D. Small fixes
- **D1. `docs/reports/2026-10-27-kit-v3-1.md` line 169** (B2's nit): "(09:30:25–41 UTC)" becomes "(09:30:25–41 UTC in
  one run, 09:30:23–38 in the other)". I re-read both job logs first.
- **D2. `docs/LEGACY_INVENTORY.md` KT14** (optional; the advisor's 2026-10-28 nit, carried in its #30 review, note
  7): a "since then" note. One "Night shift" routine now starts hourly from 00:10 to 07:10 Amsterdam time, with the
  window to 07:30 (kit-v3-1).

## Not in this pass
- Changing the guard's read limits (`actions_get`, `git rev-list`): they are recorded, not changed. A guard change is
  a kit change-set (KIT-FOLLOWUPS or later).
- The PR #26 follow-ups themselves (KIT-FOLLOWUPS).
- B3 (Mustafa's 2026-11-04 design decisions): its own change-set, next.
- The EDITOR_V2 and VISION_V2 notes (docs B1 and B2 of the 2026-11-03 list).

## Evidence
- CI `gate` and `smoke` green on the final head, with gate symbols +0 −0. There is no local gate: this worktree has no
  `tools/node_modules`, and the change is docs only.
- `node .claude/scripts/repo-map-check.mjs` OK.
- `node --test .claude/hooks/settings.test.mjs .claude/hooks/routine-guard.test.mjs .claude/scripts/checkpoint.test.mjs`
  unchanged: 122, 192 and 25.
- Greps:
  - no "fires on pull request events" or "one run per ready PR or new push" left in the live docs;
  - CLAUDE.md's routine shell list names `date`;
  - the README's counts equal the OQ headings counted in ADR 0002 and 0006.
- Every fact cites its source (SHAs, times, routine reads, session ids). The verifier spot-checks them.

## Reviewers (once, on the final diff)
- docs-keeper writes B.
- engine-architect reviews C1, C2, C5 and C6; infra-engineer reviews C3.
- security-auditor reviews A1 and A2 (the routine rules lines).
- The verifier checks the evidence and the facts.
- The advisor reviews this plan now and the final head at the end.

## Risks
1. advisor.md changes, so this PR's reviews use main's advisor.md (a general agent reading it from main). The PR
   Advisor routine would classify the PR needs-mustafa (it touches `.claude/` and `CLAUDE.md`) and ask for a
   day-session review. It merges on Mustafa's go in this session.
2. The effort line rests on two observations; ROUTINE-EFFORT may refine it.
3. PORTAL-EMBED must stay a question: no options, no design (his words).
4. KIT-FOLLOWUPS and ROUTINE-EFFORT are open. The docs record them as open; if Mustafa answers before the build ends,
   the answers land here.
5. STATUS is long and drifts. docs-keeper edits it; the verifier spot-checks.
6. The night shift's first guarded night starts 2026-11-05 00:10. Nothing here changes what it may run: the shell
   list only gains `date`, which the guard already allows.

## Steps
1. Plan revision 1 (this file), then the advisor's review.
2. Edits A, C and D (main session) and B (docs-keeper), one after another in this worktree.
3. Evidence, reviews, fixes.
4. Report (the PR body); delete this checkpoint; mark the PR ready; the advisor's APPROVE of the final head;
   Mustafa's go; merge.

## Decisions
- PORTAL-EMBED (Mustafa, 2026-11-04): an open question, not designed. The README is read as `docs/adr/README.md`.
- PR-HOME-REPLACE (a) (Mustafa, 2026-11-04): no setup script; schedule trigger only.
- PROBE-FILE (c) (Mustafa, 2026-11-04): the Stop hook is Claude Code's own, recorded as known noise.

## Open findings
- None yet.
