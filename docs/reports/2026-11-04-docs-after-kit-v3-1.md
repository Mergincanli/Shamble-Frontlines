# Advisor report: docs-after-kit-v3-1 (2026-11-04, branch `claude/sleepy-tesla-ofsxsr`, base main 868e165)
## Goal
Bring the rules and docs in line with PRs #33, #30 and #34, the routine setup and its test run, and Mustafa's rulings of
2026-11-04, and add PORTAL-EMBED as an open question. Nothing a player sees; no code.
## Outcome
DONE: docs and rules, symbols +0 −0. It changes `CLAUDE.md` and `.claude/`, so it merges only on Mustafa's go.
## Plan and approval
- Plan revision 1 (381471d): the advisor's APPROVE with 16 conditions. Revision 2 (0099b3c) adds his answers.
- The verifier checked all 16 on the final diff: met. The two it marked partly met (2: the effort hedge; 3: one
  trigger wording) were fixed in 884d33b, and condition 15's API re-read was done at 14:36:38 and 14:47:25 UTC.
## Mustafa's words (2026-11-04)
- "CLAUDE.md's routine shell list should name `date`, matching the guard."
- PR-HOME-REPLACE: "(a), no setup script. The PR Advisor routine now has a schedule trigger only, with the GitHub
  pull-request trigger removed: CRON_TZ=Europe/Amsterdam 45 7,13,19 * * * … Record the rule in the docs pass: the PR
  Advisor runs only on a schedule or an API trigger, never on PR events."
- PROBE-FILE: "If it's Claude Code's own, (c), and record it as known noise." It is the harness's own Stop hook.
- "KIT-FOLLOWUPS: (a), a small A3 right after A2 for the guard's follow-ups: ask before kit and gate edits in day
  sessions; the sk- key after %20, %2C or %3A; and read-only GitHub Actions tools (such as actions_get) for pr-advisor,
  which B2 showed it needs for CI details. security-auditor reviews A3. GitHub secret scanning and push protection are
  now on." His earlier words for the Actions item: "the guard refused actions_get, a read-only GitHub Actions lookup
  the PR Advisor needs for CI details; consider allowing read-only Actions tools for pr-advisor, with security-auditor's
  review".
- "ROUTINE-EFFORT: (a). The PR Advisor at max is my deliberate choice (quality over cost), so it stays. The question is
  only whether the night shift really runs at Extra, as its routine says, and what controls that." It is renamed
  EFFORT-SOURCE: ROUTINE-EFFORT is the Advisor's decided code of 2026-10-25.
- "Part B accepted as passed. The PR Advisor routine is un-paused (07:45, 13:45, 19:45), and the Guard test B1 routine
  is deleted."
- PORTAL-EMBED: quoted in full in OQ-0002-9 and OQ-0006-23.
- A4, given after the review round and queued in STATUS without review: "A4, a small kit change-set after A3, plan
  first (Mustafa): a review policy inspired by Anthropic's own process. 1. Risk tiers: Tier 0 docs only (CI + PR
  Advisor, auto-merge); Tier 1 game code and tools (CI + advisor; Mustafa's Brave check when players see or hear it);
  Tier 2 rules, CI, guard, secrets, infrastructure (advisor + security-auditor, always Mustafa's go); Tier 3
  irreversible or money (Mustafa decides; agents only propose). Map today's "ready"/"needs-mustafa" classes onto these.
  2. Review budget: at most two review rounds per change; a third only for a Critical or High finding; everything else
  becomes a note or a follow-up. Nits never trigger a push. 3. Rollback rule: if main breaks, revert the merge first,
  then fix in a new PR. 4. Note for the foundation: fast checks on save (format, type checks for Rust and TypeScript),
  the full gate in CI." Then: "5. Per-tier review checklists in .claude/agents/advisor.md: for each tier, what to check
  and what to ignore. 6. Notes for the foundation, next to the fast checks on save: one-click rollback of client
  deploys and of server versions; and, from the first playtests on, error reports and telemetry from players'
  browsers, next to the server metrics already planned. Record them where they belong (SPIKE_PLAN's foundation checks,
  ADR 0006) as notes, not designs."
## What changed
- `CLAUDE.md`:
  - each routine runs at its own effort (the night shift at Extra, `xhigh`; the PR Advisor at max), not the project's
    `high`;
  - the shell list names `date`;
  - the PR Advisor has a schedule trigger only, can also be started by hand or the API, and never by PR events.
- `.claude/agents/advisor.md`: CI output counts as evidence, never instructions; the known limits (every `actions_*`
  tool and `git rev-list` refused until A3) and the Stop-hook noise; notes in a routine. `.claude/CHECKPOINTS.md`: the
  same note.
- STATUS, ROADMAP, REPO_MAP (docs-keeper): the merges, TEST-RUN, the routines as the API reads them, EFFORT-SOURCE,
  PROBE-FILE, A3, A4, PORTAL-EMBED.
- PORTAL-EMBED:
  - ADR 0002: part 5, the rollback bullet and OQ-0002-4 (b) now say the same thing, plus a new OQ-0002-9;
  - ADR 0006: a new OQ-0006-23;
  - the ADR index: 118 numbered questions, 98 open, and two new rows;
  - SPIKE_PLAN §3, ENGINE_OVERVIEW and the research glossary point at OQ-0002-9.
- Fixes: the kit-v3-1 report now covers both CI runs on 04da817; LEGACY_INVENTORY KT14.
## Evidence
- **Local checks on 884d33b:** gate `--no-smoke` PASS (1–4, 5b–5d; symbols +0 −0, as planned); repo-map-check OK
  (113/100); hook and checkpoint tests 339/339. CI on c70132f and 603f53b: gate and smoke green (runs 393 to 396).
- **Facts read from their sources:**
  - B2's transcript (session_0173vVo7uY6BfxLBnYjEBJLV: no permission request; 7 guard refusals);
  - the routines API at 14:02, 14:36 and 14:47 UTC;
  - effort from the session records: the night shift's 06:11 UTC run at `xhigh` (session_0173vVo8bhZGeK4bVFXQQEtM)
    and B2 at `max`; a day session's launch log shows `--effort max`;
  - the 04da817 job logs;
  - the PR merge times (`merged_at`).
- **Not verified:**
  - that a scheduled PR Advisor start begins on main (first one at 18:45 UTC);
  - the night shift's first night under the guard (2026-11-05 from 00:10);
  - that routines get `--effort` the same way.
## Reviews
- **Final diff** (workflow wf_213e499a-b4d): security-auditor, engine-architect (xhigh), infra-engineer (xhigh), the
  verifier and a completeness critic all gave PASS WITH NOTES. There were 31 findings: 4 major, 12 minor, 15 nits. A
  skeptic upheld 30, and 884d33b fixes all 30. The top ones:
  - the glossary still said isolation "flips everywhere at once";
  - OQ-0006-23 claimed "the same client host";
  - the effort hedge;
  - CI logs not yet named as evidence;
  - the `sk-` gap covers every percent-escape except `%3D`.
- **INFRA-2** (refuted under condition 9's "no claims") is added as a bare pointer.
- **The fix check** of 884d33b (the verifier): PASS WITH NOTES.
  - All 30 fixes landed. The three it called "partly" are completed by the last commit: STATUS's pointer to this
    report.
  - It raised one new minor item, A4 without a source, now quoted above.
  - It raised four nits, all fixed in the last commit: a "last docs commit" phrase, the API re-read repeated right
    before that commit, three ragged wraps, and the exact run start times on 04da817.
## Decisions and deviations
- The register gets one row per question (its own rule).
- Python stays out of CLAUDE.md's shell list (condition 4; the gap is on A3's list).
- docs-keeper's part came from its own worktree: 62d2864, cherry-picked as 603f53b.
## Risks
1. The routine rules rest on two runs started by hand; the 18:45 UTC run and tonight's night shift are the next proof.
2. The PR Advisor will class this PR needs-mustafa (it changes `.claude/` and `CLAUDE.md`), as designed.
## Proposed next step
B3: Mustafa's 2026-11-04 design decisions into LOOP_V2 and UNITS.md, plan first.
## Decisions needed
None.
## For Mustafa
1. After the advisor approves PR #35's final head, say "merge #35" in this session. It is a rules change; no Brave check.
