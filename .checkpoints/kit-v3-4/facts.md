# kit-v3-4 (A4) facts for the plan

Gathered read-only on 2026-11-07 in the worktree (branch `kit-v3-4` = main 69ea88e + one notes commit bbf2822; the
rules files equal origin/main's). Line numbers are the worktree's (= main's). Facts only; no decisions.

## 0. His words (docs/STATUS.md, copied exactly)
Source: `docs/STATUS.md:544-570` (worktree = origin/main, md5 equal). Verbatim, with line numbers:

```text
544:   - A4 (kit): the review policy, Mustafa, 2026-11-04. It waited for the kit freeze to lift (his A4-FREEZE (a),
545:     below: after the netcode proof test is under way; lifted on 2026-11-06 by his UNDER-WAY (a): Phase), then plan
546:     first; a kit change-set, so it needs his go:
547:     > A4, a small kit change-set after A3, plan first (Mustafa): a review policy inspired by Anthropic's own process.
548:     > 1. Risk tiers: Tier 0 docs only (CI + PR Advisor, auto-merge); Tier 1 game code and tools (CI + advisor; Mustafa's
549:     >    Brave check when players see or hear it); Tier 2 rules, CI, guard, secrets, infrastructure (advisor +
550:     >    security-auditor, always Mustafa's go); Tier 3 irreversible or money (Mustafa decides; agents only propose). Map
551:     >    today's "ready"/"needs-mustafa" classes onto these.
552:     > 2. Review budget: at most two review rounds per change; a third only for a Critical or High finding; everything else
553:     >    becomes a note or a follow-up. Nits never trigger a push.
554:     > 3. Rollback rule: if main breaks, revert the merge first, then fix in a new PR.
555:     > 4. Note for the foundation: fast checks on save (format, type checks for Rust and TypeScript), the full gate in CI.
556:   - A4's addition, Mustafa, 2026-11-04:
557:     > Addition to A4 (Mustafa): also include
558:     > 5. Per-tier review checklists in .claude/agents/advisor.md: for each tier, what to check and what to ignore.
559:     > 6. Notes for the foundation, next to the fast checks on save: one-click rollback of client deploys and of server
560:     >    versions; and, from the first playtests on, error reports and telemetry from players' browsers, next to the server
561:     >    metrics already planned. Record them where they belong (SPIKE_PLAN's foundation checks, ADR 0006) as notes, not
562:     >    designs.
563:   - A4-FREEZE, Mustafa, 2026-11-05, about 16:25 UTC (his chat message in the A2 and A3 day session,
564:     session_0173vVo7FcYUpbWgRW8oQyHU, answering A4-FREEZE, asked in that session's update; copied from
565:     `.checkpoints/kit-v3-3/queue/mustafa-2026-11-05-a4-b1.md`):
566:     > A4-FREEZE: (a). A4 waits until the netcode proof test is under way.
567:   - For A4's plan (the session's notes, not his words): check what ADR 0006 already plans (rollback, crash reports
568:     and telemetry, server metrics) before writing items 4 and 6 as notes; item 5's checklists must not contradict
569:     advisor.md's "Never" list or step 0; item 3: who may revert, and through a PR (main is protected); item 2: how
570:     a round is counted, and which severity scale; Tier 0: the PR Advisor's merge or GitHub auto-merge.
```


## 1. Merge classes and review flow today

**The two classes (advisor.md is the only place they are defined):**
- `.claude/agents/advisor.md:113-119`, step 4 "Classify":
  - **needs-mustafa** (:114-117): "it touches any escalation category above, or it is a major change worth a second
    opinion: a new system or mechanic, a large refactor of an engine file, a measured regression the PR accepts, a
    security finding left open, or a doubt your own checks can't settle headless. When unsure, or when both lists fit,
    it is needs-mustafa."
  - **ready** (:118-119): "nothing for human eyes. That covers docs, tools (not the gate's own files), tests, internal
    refactors with identical renders and sims, and fixes proven headless with no visible change."
- The "escalation categories" those classes key on: `advisor.md:36-43`, numbered 1-4: (1) "Human QA: anything a player
  sees, hears or feels"; (2) "Design decisions the docs don't settle"; (3) "The rules themselves: changes to hard
  constraints, ADRs, `CLAUDE.md`, `.claude/` (this file included), CI (`.github/`), security rules, or the gate's own
  files" (gate files listed :40-42: `tools/gate.mjs`, `tools/smoke.mjs`, `tools/battery.js`, `tools/gate-globals.json`,
  `tools/package.json`, `tools/package-lock.json`, any `.npmrc`); (4) "Anything irreversible: deleting data or saves,
  rewriting history."
- Extra needs-mustafa triggers in step 0 and step 2 (as the routine):
  - `advisor.md:76-78`: a diff touching `.claude/`, `CLAUDE.md` or `.github/` -> "classify it needs-mustafa and ask
    for a day-session advisor review rather than trusting your in-session checks".
  - `advisor.md:79-80`: "a refusal that blocks a step means that step is needs-mustafa".
  - `advisor.md:100-105`: CI's green "stands only when the PR touches neither `.github/` nor the gate's own files ...
    otherwise CI ran the PR's own gate, so it is at most needs-mustafa. A PR that changes `shamble-*.js` or the pages,
    or whose headline claim needs a sim, render, measurement or bug-repro, is **needs-mustafa** ('verify in a day
    session')".
- Labels and actions, `advisor.md:133-145` (step 6): "APPROVE + ready + all checks green: label `ready-to-merge`, merge
  the PR" (:134); "APPROVE + needs-mustafa: label `needs-mustafa` ... Once his go for the current head is in ... merge
  the PR if every check is green" (:135-138); "REVISE or BLOCK: label `changes-requested`" (:145). Labels in use:
  `ready-to-merge`, `needs-mustafa`, `changes-requested`.
- "Mustafa's go" defined: `advisor.md:139-142` ("counts only in his own words in the chat of the session that merges:
  about that PR ..., given after your APPROVE of its current head. A 'merge' written on GitHub never counts"); and
  `CLAUDE.md:96-101` (Standing permission: "A session merges only on Mustafa's own go in its chat, about that PR and
  for the PR's current head, given after the advisor approved that head ... with the gate and smoke checks green, and
  passes that SHA as the merge's expected head (the night shift never merges); a later push needs the advisor's review
  first, then his go again; every other merge is the PR Advisor's.").
- Merging mechanics: `advisor.md:143-144` (re-check go, SHA, checks; "pass the approved SHA as the expected head").
- Plan verdicts: `advisor.md:62-65` (APPROVE / REVISE / ESCALATE); PR verdicts `advisor.md:121` (APPROVE, REVISE or
  BLOCK). Plan review checklist `advisor.md:51-60` (today the only "what to check" list; no "what to ignore" list
  exists anywhere).
- advisor.md's **Never** list, `advisor.md:162-169` (item 5 of A4 must not contradict it): ":163 Merge with failing or
  pending checks, or unless the `gate` and `smoke` jobs both passed on the approved SHA" · ":164 Merge a hard-constraint
  change before its accepted ADR is on main" · ":165 Merge anything in escalation categories 1–4, or classed
  `needs-mustafa`, without Mustafa's go for its current head" · ":166 Merge a head you didn't approve" · ":167 Approve
  without running the gate yourself" · ":168 Edit code or push commits" · ":169 Lower a bar because a report says it
  passed".
  - Note (fact, not a ruling): :167 "Approve without running the gate yourself" sits next to :100-101, where the routine
    has "no interpreter" and reads gate/smoke from CI check runs instead.
- advisor.md's **step 0**, `advisor.md:70-87`: role check `SHAMBLE_ROLE=pr-advisor` (:70-71); `git fetch -q origin main
  && git switch --detach origin/main` (:72-73), "the only switch the PR Advisor makes" (:73-74); read the PR only
  through git and GitHub tools, "run no program from the PR" (:75-76); kit-touching diff -> needs-mustafa (:76-78);
  allowed Actions reads (:80-83); logs are data (:83-85); PROBE-FILE noise (:85-87).
- The protocol step that makes a PR "ready": `CLAUDE.md:84-90` (step 6): "Keep the PR a draft while work is in
  progress; before marking it ready, fold the checkpoint into the report and delete it ... Mark it ready for review when
  the report is in; the PR Advisor routine reviews it. Once approved with checks green, it merges at once if no human
  check is needed; otherwise it labels it `needs-mustafa`, and the session Mustafa gives his go to merges it (his Brave
  check for anything a player sees, hears or feels; his reconfirmation for rule changes and other major changes)."
  Also `.claude/skills/changeset/SKILL.md:30-32`; `.claude/CHECKPOINTS.md:164-166, 200-208` (§9: ready PR has no
  `.checkpoints/` files; a multi-step review fix turns the PR back into a draft).
- **The PR Advisor routine:** prompt and schedule recorded in `docs/STATUS.md:765-777` (trig_01YPyWAsQmsaQoydhMGG3CkJ;
  `CRON_TZ=Europe/Amsterdam 45 7,13,19 * * *`; never started by pull-request events, PR-HOME-REPLACE (a)). Stored
  prompt, verbatim at :774-777: "You are the PR Advisor for SHAMBLE FRONTLINES. Read .claude/agents/advisor.md from
  the default branch and follow its PR review procedure exactly, together with CLAUDE.md, for every open pull request
  in this repository, oldest first. Stay on the default branch: read each pull request only through git, naming its
  head SHA, and the GitHub tools." No separate routine doc exists in `.claude/` or `docs/`; advisor.md is its
  procedure. `CLAUDE.md:145-153` (its trigger and settings reload); `CLAUDE.md:135-144` (routine guard limits).
- **Auto-merge:** GitHub auto-merge is **refused for every session, day included**: `.claude/hooks/routine-guard.mjs:1393`
  `if (name === 'enable_pr_auto_merge') return deny('auto-merge skips the expected-head check; merge with the MCP merge
  tool instead');` (test `.claude/hooks/routine-guard.test.mjs:135`, row 'auto-merge', DAY, expected deny). Also
  `gh pr merge` refused: guard :1340 and settings.json deny `"Bash(gh pr merge *)"` (settings.json:229). The only merge
  path is the MCP `merge_pull_request` with the guard's `mergeCheck` (`routine-guard.mjs:1444-1466`): full 40-char
  `expectedHeadSha` (:1446), same repo, night shift refused (:1449: "the night shift never merges"), head equals SHA,
  not a fork, `gate` and `smoke` completed and green (:1454-1459); for pr-advisor, refused if the PR changes any
  protected file (:1460-1464: "rules, CI and gate changes merge from a day session on Mustafa's go"). The word
  "auto-merge" appears nowhere in the rules texts; only in STATUS:548 (his words) and :570 (session note).
- **Night shift's merge ban:** `.claude/NIGHT_SHIFT.md:60` ("never merge a pull request; never push to main; never
  change .claude/ or CLAUDE.md"); `CLAUDE.md:99` ("the night shift never merges"); `CLAUDE.md:119-120` (anything needing
  Mustafa, merges included, never goes in the Night queue); guard `routine-guard.mjs:1449`.

**Who may merge each class today:**
| Class | Condition | Who merges |
|---|---|---|
| ready | advisor APPROVE + all checks green (advisor.md:134) | the PR Advisor routine, at once (advisor.md:33-34, :134; CLAUDE.md:87, :100 "every other merge is the PR Advisor's") |
| needs-mustafa, player-facing | APPROVE + Mustafa's Brave check passed + his go in chat (advisor.md:127-129, :135-138) | the session in whose chat he says "merge #n" (CLAUDE.md:88, :96-99), or the PR Advisor after his go (advisor.md:135-138) |
| needs-mustafa, other (rules, kit, CI, gate, major) | APPROVE + his reconfirmation "merge #n" (advisor.md:130-131) | same; but for a PR touching `.claude/`, `CLAUDE.md`, `.github/` or gate files the guard refuses the PR Advisor's merge (routine-guard.mjs:1460-1464), so only a day session on his go |
| any, at night | — | never (NIGHT_SHIFT.md:60; guard :1449) |
| any, via GitHub auto-merge or `gh pr merge` | — | refused for all sessions (guard :1393, :1340; settings.json:229) |

- Practice check: every recent PR in STATUS "In flight" (e.g. #42, #43, #38) merged "on Mustafa's go" in a day session
  (STATUS:108, :110); the one recorded PR Advisor unattended merge is #34 (TEST-RUN B2, STATUS:114, :784-790).

## 2. Review rounds and severity words today

**"Review once" (the only round rule in the rules texts):**
- `CLAUDE.md:78-83` (protocol step 5): "Review, once, on the change-set's final diff (after fixes, only what changed):
  [reviewer list] ... Address or explicitly defer every finding." No cap on how many "after fixes" re-reviews follow.
- `CLAUDE.md:177-178` (Efficiency): "**Review once, on the final diff:** reviewer agents run on a change-set's final
  diff, not on every commit; after fixes, only on what changed. The gate still runs after every step."
- `.claude/skills/changeset/SKILL.md:25-26` (verifier "on the final diff ... and again after fixes. Fix and re-run until
  PASS") and `:27-29` (reviewers "once, on the final diff (after fixes, only on what changed) ... Address each finding,
  or defer it in writing with a reason").
- PR-side re-review: `advisor.md:91` ("If you reviewed this PR before, review only what changed since"); `:137-138`
  and `:145` ("A later push needs your review first, then his go again"; REVISE/BLOCK -> "The implementer fixes and
  pushes, and you review again"); `CLAUDE.md:99`. No limit on PR review rounds.
- Plan-side: `advisor.md:62-65` (REVISE -> fix the plan, no cap); `NIGHT_SHIFT.md:52-54` is the only cap anywhere: a
  night NEEDS PLAN item: "if it asks for changes, fix the plan and ask once more; if it escalates or still doesn't
  approve, list it under Decisions needed".
- `.claude/CHECKPOINTS.md:68-69`: "a review round of 5 reviewers runs as 3, then 2" (the word "round" as a set of
  reviewers run together, not a count limit). `CHECKPOINTS.md:207-208` (§9.4): "A review fix that takes more than one
  step turns the PR back into a draft".
- **"Review budget", "fix pass", "skeptic", "nit":** none appears in CLAUDE.md, advisor.md, CHECKPOINTS.md,
  NIGHT_SHIFT.md, any agent file or skill (grep, 2026-11-07). They exist only in STATUS and reports:
  - Mustafa's SCOPE-STOP, `docs/STATUS.md:185-193` (word for word at :185-192): "... then one final review (pass 4),
    and apply the review budget now: anything pass 4 finds below Critical or High becomes a STATUS follow-up, not
    another pass." (Also `docs/reports/2026-11-05-kit-v3-3.md:116`.)
  - Its use: STATUS:414-415 ("under Mustafa's review budget (below Critical or High: a follow-up, not code)"), :436;
    the kit-v3-3 report :42, :53, :58-60 ("checked by the verifier alone").
  - "skeptics": STATUS:110 (A3 row: "skeptics on six findings: five upheld, one refuted"), :951. A practice, not a rule.
  - "fix pass" / "pass N": STATUS:110 (A3: passes 1-4 plus fix passes 1-4; i.e. five review rounds after the plan).
  - "nit": STATUS:110 ("P3-8 (a nit)"), kit-v3-3 report :159-161.
- How a round is counted today: not defined anywhere. In practice (STATUS rows :108, :110): a "review round" = one set
  of reviewer agents on a head (e.g. B.2: "the review round on 26a5455", then "the one fix pass", then the advisor on
  the final head); A3 numbered "pass 1..4" for reviewer rounds and "fix pass 1..4" for fixes.

**Severity words (is there one scale? No):**
- The only definition-like line: `.claude/agents/security-auditor.md:52`: "Findings with severity (Critical / High /
  Medium / Low): what, one-line exploit sketch, evidence (file:line), fix." No meanings given for the levels.
- Every other reviewer: "ranked findings" with no scale: `advisor.md:124` ("Findings: ranked."), `art-director.md:37`,
  `engine-architect.md:49`, `graphics-engineer.md:38-39`, `infra-engineer.md:38`, `netcode-architect.md:61`,
  `rust-reviewer.md:36`, `test-engineer.md:36-37`, `perf-auditor.md:41` ("ranked by gain ÷ risk"); `verifier.md:39`
  (PASS | FAIL | PASS WITH NOTES, "Problems", no severity); `game-designer.md:31` (table, no severity).
- Verdict vocabularies differ: advisor plan APPROVE/REVISE/ESCALATE (`advisor.md:62-65`), advisor PR
  APPROVE/REVISE/BLOCK (:121); most reviewers APPROVE/REVISE/BLOCK; verifier PASS/FAIL/PASS WITH NOTES; perf-auditor
  OK/REGRESSION/IMPROVEMENT (`perf-auditor.md:38`); game-designer APPROVE/REVISE/ESCALATE (`game-designer.md:31`).
- In practice two scales are mixed in STATUS: Critical/High/Medium/Low (STATUS:110 pass 4: "P4-S4 (Critical ...)",
  "P4-S6 (Medium)", "P4-S7 and P4-S8 (Low)") and major/minor/nit (STATUS:110 pass 2-3: "SA-2 major", "P3-4 and P3-6
  (minor)", "P3-8 (a nit)", "skeptics upheld all four majors"). Mustafa's A4 item 2 uses "Critical or High" and
  "Nits".
- `advisor.md:47` "critical fixes only" (legacy frozen) is a different sense of "critical".

## 3. Rollback, revert and main

- **No rule anywhere says what to do when main breaks.** "revert", "roll back" and "main breaks" appear in none of
  CLAUDE.md, advisor.md, NIGHT_SHIFT.md, CHECKPOINTS.md or the agent files (grep, 2026-11-07), except
  `infra-engineer.md:37-38` (its review asks "how it is rolled back"). The phrase "main is protected" occurs only in the
  session's note `docs/STATUS.md:569` and in Mustafa's PROTECT-MAIN quote `STATUS:983`.
- **Branch protection (the docs' facts):** `docs/STATUS.md:983`: "GitHub branch protection on `main` is on (Mustafa,
  2026-10-17 ...). Rules, read with the GitHub REST API on 2026-10-17 ...: 'Shamble Ruleset 1' (active, targets the
  default branch `main`) blocks deleting and force-pushing `main`, requires a pull request (0 approvals) and requires
  the `gate` and `smoke` checks; a branch behind `main` may still merge (not strict); no classic protection rule. The
  `.claude/settings.json` deny rules stay a second layer (fragile by design)." Also `docs/ROADMAP.md:49` (turned on
  2026-10-17), `:58` ("a pull request with `gate` and `smoke` green"). Not re-read from GitHub in this pass.
  - Consequence (fact, from these lines): a revert of a merge must itself go through a PR with `gate` and `smoke`
    green; a direct push of a revert to main is blocked by the ruleset, by settings.json's deny rules
    (`.claude/settings.json:157-164`, e.g. `"Bash(git push origin main)"`, `"Bash(git push * HEAD:main)"`) and, in
    routines, by the guard (`CLAUDE.md:143` "it can't push to main or force-push").
  - Since main's own gate/smoke runs on push (`.github/workflows/gate.yml`, KT8 in `docs/LEGACY_INVENTORY.md:418`),
    "main breaks" is observable as a red CI run on a main commit (STATUS rows record "CI green on main", e.g. :108).
- **Force-push / history:** `CLAUDE.md:242` (Never: "force-push, hard-reset or rewrite history"); `advisor.md:43`
  (escalation 4: "rewriting history"); settings.json deny `"Bash(git push --force *)"` etc. (`settings.json:154-156`,
  `:165-168`), `"Bash(git reset --hard *)"` (:181), `"Bash(git rebase *)"` (:195), `"Bash(git commit *--amend*)"`.
- **`git revert` itself:** in no allow and no deny rule of settings.json (so in a day session it falls to the normal
  permission prompt); the guard does not read it: STATUS:867, stated limit L4: "patch-applying commands and HEAD moves:
  `git cherry-pick`, `am`, `apply`, `revert`, ... in any folder, the main folder included" (no kit ask for a kit file
  changed by a revert). In routines, any git subcommand outside the routine lists is refused (`CLAUDE.md:141-144`;
  NIGHT_SHIFT.md:60 bars the night from merging and pushing to main).
- **Who may merge to main (so who could merge a revert PR) today:** see section 1's table: the PR Advisor routine for
  `ready` PRs (but not a PR touching kit/CI/gate files: guard :1460-1464), a day session on Mustafa's go for
  `needs-mustafa`; never the night shift; always via the MCP merge tool with the approved SHA (guard :1444-1466;
  `gh pr merge` and auto-merge refused, :1340, :1393). A revert PR would need the advisor's APPROVE of its head
  (`CLAUDE.md:94-96`: "the advisor's APPROVE before anything merges to main"; `advisor.md:166` "Merge a head you
  didn't approve").
- **The plan protocol vs a revert:** `CLAUDE.md:240` (Never: "edit before the plan is approved"); `CLAUDE.md:56`
  ("Fix bugs before features"). No exception for an emergency revert exists.
- **The ADRs on reverting code:** `docs/adr/0002-code-structure-build-toolchain.md:1521` ("The restructure: a move of
  five files, reverted by reverting its one change-set"); `docs/adr/0004-simulation-core.md:1304` ("each change lands
  as its own change-set (CLAUDE.md), so backing out is a revert"). ADR 0006 has no rule about reverting merges on main.
- **v2 changes the required checks:** ADR 0002 part 9 plans one aggregate required check instead of `gate` and
  `smoke` (`docs/adr/0002-code-structure-build-toolchain.md:595`, :1407-1408: Mustafa names it in branch protection;
  PM-0002-57 at :1309). The guard (`routine-guard.mjs:1454`, `for (const nm of ['gate', 'smoke'])`) and
  `advisor.md:94, :163` hard-code `gate` and `smoke` today.
- NIGHT_SHIFT.md on a failing gate: `:62-63` ("if the gate fails and you can't fix it properly, stop that item and
  report it"); about a branch, not main.

## 4. What ADR 0006 and SPIKE_PLAN already plan

Status words: ADR 0006 is "**Status:** Proposed" (`docs/adr/0006-infrastructure-operations.md:3`), "Signed off by
Mustafa: as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted" (:7). Inside it, "(proposed)"
marks a single choice; "open: OQ-…" marks an open question. Same status for ADR 0002 and 0003 (each `:3` "Proposed")
and for `docs/SPIKE_PLAN.md:3-4` ("Proposed, with the six decision drafts ... not yet accepted"). "Mark" below = a pass
mark (PM-…) or a spike mark.

| His item | Where ADR 0006 (or a sibling draft) has it | Kind |
|---|---|---|
| One-click rollback of **client deploys** | Not designed. Part 15 (`0006:580-589`) and Rollback, "A bad release" (`0006:1552-1554`): "how the client host puts a previous deployment back was not read"; OQ-0006-14 (`0006:999-1015`, options (a) release window / (b) live releases, :1006-1011; "how the client host puts a previous deployment back was not read", :1011). Production deploy job: ADR 0002 part 10 (`0002:671-676`: "Production is deployed only by its own job, on pushes to `main`, from the artifact `main`'s CI built and tested, in its own environment `production`"); ADR 0006 :557. | open question (OQ-0006-14); no mark |
| One-click rollback of **server versions** | Part 15 interim position (`0006:584-586`, "(proposed)"): "a release is made only while no room runs, with the client and every VM's room binary switched together, and the previous room binary stays on each VM, so a bad server release is switched back the same way." Rollback section `0006:1552-1554`. OQ-0006-14 `0006:999-1015` (decider :1012-1014: "the Advisor for the model ...; Mustafa for when players may be refused"; when: "before the first online playtest", :1015); register row `docs/adr/README.md:153` ("LATER: the design session after the netcode proof test"). Migration step 4 (slice 3, `0006:1483-1487`): "the release model decided (OQ-0006-14)". | interim position, proposed; open question; no pass mark |
| **Error reports from players' browsers** | Part 10 "Crash reports" (`0006:431-457`): Sentry browser SDK with `wasmIntegration` (:431), EU org, `userInfo` off (:433-434); Reporting API `crash` reports to "our own report endpoint on the lobby" (:438-439), rate-limited, "(proposed; the security review, finding 14)" (:442); Mustafa creates the EU Sentry org (:445-447, OQ-0006-8); browser support and Wasm symbolication are spike marks (:449-451: R6-OBS-OQ3, R6-OBS-OQ4) "in the boundary spike if the EU organization exists by then, otherwise at the first online playtest's milestone". Options Q8 (`0006:666-672`, C1 and C2 "proposed"). Pass mark PM-0006-30 (`0006:1386`, a tab crash reported). ADR 0002 boundary spike item 10 (`0002:1174`, optional, only if the Sentry org exists). Timing: migration step 4, slice 3, "before the first online playtest ... Sentry set up in the EU" (`0006:1483-1486`); SPIKE_PLAN Later milestones `SPIKE_PLAN.md:552-553` ("R6-OBS-REC3, REC4 (before the first online playtest ...)"). | proposal + pass mark (PM-0006-30) + open questions (R6-OBS-OQ3, OQ4, OQ-0006-8) |
| **Telemetry from players' browsers** | ADR 0003 part 8, "Client field data" (`docs/adr/0003-rendering.md:422-427`): once per match the client reports backend, adapter info, quality tier, DPR, resolution, frame-time median/p95/over-budget; ADR 0006 part 10 stores it at the same lobby endpoint (`0006:439`, :452-455) and part 11 keeps it 14 days (`0006:462-463`). Timing: "switched on in slice 6" (`0003:1245`, migration step 10 at `0003:1410-1411`); no spike mark (`0003:1245`). Note: his words say "from the first playtests on"; the draft says slice 6. Server-side **balance telemetry** (match records) is part 12 (`0006:472-522`), slice 4 (`0006:1491-1492`), not browser telemetry. | proposal; no spike mark; timing differs from his words |
| **Server metrics (already planned)** | Part 9 "Logs and metrics" (`0006:398-430`): `tracing` JSON lines; `metrics` + `metrics-exporter-prometheus`, Prometheus on the staging VM, Grafana Cloud EU later (OQ-0006-8); the first metrics listed (:411-415: tick time histogram, overruns, per-phase time, sessions by transport, bytes, budget overruns, joins refused, room starts/ends/crashes, memory); alerts by email "(proposed ...)" (:421-423). Options Q7 (`0006:654-664`). Operations marks inside the netcode proof test (part 14, `0006:574-579`: "the metrics and their path, and the firewall"); spike item 2 at `0006:1212`. Live from slice 3 (`0006:1488-1489`: "logs and metrics live (part 9)"). | proposal + spike marks (netcode proof test) |
| **Logs** | Part 9 (`0006:398-401`, retention 14 days proposed, part 11 `0006:458-470`). | proposal |

**SPIKE_PLAN's foundation checks and the "fast checks on save" (his item 4):**
- `docs/SPIKE_PLAN.md:420-438`, "The foundation's acceptance checks (not a spike)": "every gate check bites; CI's
  required check cannot be fooled; the generated types hold in CI" (:424-425); pass marks PM-0002-35 to -69 (:427);
  size L, "six change-sets: restructure, toolchain, gate and CI, generated types, skeleton, previews" (:433); research
  carried: R5-TSLINT-REC1–REC4, R5-RUSTLINT-REC1–REC6, R5-CI-REC1–REC5, R5-GATE-REC1–REC4 (:434-438). **No word for
  fast checks on save, a per-edit hook, rollback, error reports or telemetry** appears in SPIKE_PLAN's foundation entry.
- Where fast checks on save already are: ADR 0002 part 7 (`docs/adr/0002-code-structure-build-toolchain.md:472-474`):
  "**The per-edit hook** checks only what changed (`tsc -b` on the touched project and ESLint on the file; `cargo
  check -p` on the crate), exits 2 on failure and stays fast; the full gate is the only judge. It is a `.claude/`
  change, so it lands in a kit change-set in phase 1 (nothing in `.claude/` changes in this PR)." It does not name
  format checks (rustfmt/Prettier) in the hook; format is gate stage G1 (`0002:570`: "`cargo fmt --all --check`;
  `prettier --check` on `packages/**`"), TypeScript G3 (`0002:572`: "`tsc -b`; ESLint typed lint"), Rust lint G2
  (:571). `--quick` (`0002:564-565`): "G0–G5 and the tests of changed crates and packages, for local use between
  steps, never for a commit". The full gate in CI: part 9 (`0002:562-`, :595 one final required job). Kind: proposal
  (ADR 0002 Proposed).
- SPIKE_PLAN's Later milestones (`SPIKE_PLAN.md:549-557`) lists R6-OBS-REC3, REC4 (crash reports, data rules) "before
  the first online playtest" and R6-TEL-REC1–REC3 at slice 4; it lists nothing on rollback or browser field data.
- Mustafa's item 6 says: "Record them where they belong (SPIKE_PLAN's foundation checks, ADR 0006) as notes, not
  designs." (STATUS:561-562). Sessions' constraint on ADR text: CLAUDE.md:240 (no edits before plan approval) and the
  ADRs are escalation category 3 (`advisor.md:39`: "changes to hard constraints, ADRs, ...").

## 5. The kit-edit guard

**Which files are "kit" and which are "gate" (one function decides both):** `.claude/hooks/routine-guard.mjs:32-43`,
`protectedRel(rel)`: true for any path whose last part is `CLAUDE.md`, `.npmrc` or `.mcp.json` (:35); anything under
`.github/` or `.git/` (:36); anything under `.claude/` except `.claude/agent-memory-local/` (:37-41; `.claude/worktrees/
<x>/…` is judged by its path inside that worktree, :39); and the gate files `GATE_FILES` (:26-27): `tools/gate.mjs`,
`tools/smoke.mjs`, `tools/battery.js`, `tools/gate-globals.json`, `tools/package.json`, `tools/package-lock.json`. So
CLAUDE.md, `.claude/agents/*.md`, `.claude/NIGHT_SHIFT.md`, `.claude/settings.json`, `.claude/CHECKPOINTS.md`,
`.claude/hooks/*`, `.claude/scripts/*`, `.claude/skills/*`, `.claude/rules/*` are all kit. `docs/` (STATUS, ROADMAP,
ADRs, SPIKE_PLAN, VISION_V2) is **not** kit or gate for the guard.
- The texts' definitions match: `CLAUDE.md:63-65` ("the guard's protected files: `.claude/` except agent memory, every
  `CLAUDE.md`, `.github/`, `tools/gate.mjs` and the other gate files"); `advisor.md:39-42` (escalation 3, with the
  same six gate files and any `.npmrc`); `advisor.md:101-102` (the routine's list of gate files).
- Day sessions also use `kitPath` (`routine-guard.mjs:97-108`; any worktree of this repository, e.g. under `.gate/` or
  the scratchpad, counts: "in any worktree of this repository", `CLAUDE.md:65`) and `kitDir` (:115-; a folder that
  holds kit files).

**What happens when a DAY session (no `SHAMBLE_ROLE`) edits CLAUDE.md, `.claude/agents/*.md`, `.claude/NIGHT_SHIFT.md`
or `.claude/settings.json`:**
- File tools (Edit, Write, NotebookEdit, MultiEdit): `dayEdit`, `routine-guard.mjs:931-940`, called at :388: the main
  session gets **ask** ("kit and gate files ask first in a day session: <rel>", :939); a subagent (`agent_id` set) is
  **refused** (:938, reason `KIT_AGENT` = "kit and gate files are edited by the main session, which asks Mustafa
  first", :741). Test row `routine-guard.test.mjs:137` ('day: edit CLAUDE.md asks (kit-v3-3 item 1)').
- Shell writes (redirects, cp, mv, tee, sed -i … as the guard reads them): `dayShellWrites`, :839-867, called at
  :692-693: ask for the main session, refusal for an agent; an unresolvable target (variable, glob, after an unknown
  cd) also asks / is refused (:847-860). D-ASK: a line in a form the guard does not read asks; an agent's is refused
  (STATUS:922).
- The guard "never allows anything: this guard only takes away" (:12-15); a guard exception lets a day session go on
  (:14-15; `guard.mjs:4-6`). The ask is printed only on PreToolUse with no `SHAMBLE_ROLE`, as a deny elsewhere
  (`guard.mjs:17-18`).
- The rule text: `CLAUDE.md:63-68`: "... asks Mustafa first, in any worktree of this repository (in auto mode, Claude
  Code's own classifier may refuse a `.claude/` edit before the prompt; make that edit in Accept edits or Manual
  mode); agents and teammates may not edit them. The ask is a seatbelt, not the wall: the PR review and Mustafa's go
  are the wall." Also `CLAUDE.md:222`, :226-227 ("never kit or gate files: the main session edits those, asking
  Mustafa first"); `advisor.md:157-158`.
- Mustafa's own permission mode: Auto (STATUS:182, PC-SETUP answer "1. Permission mode: [Auto]").
- Stated limits of the ask (STATUS:851-930): L1 scripts / inline code not inspected; L2 other writers; L3 user
  settings; L4 `git revert`, `cherry-pick`, `am`, `apply` etc. not read (:867-868); L5 auto mode and `.claude/`.
- Settings reload: in a day session a changed `.claude/settings.json` reloads into the running session at once
  (`CLAUDE.md:148-153`); the guard's ConfigChange block applies to routines only (`routine-guard.mjs:373-375`).

**What happens in a routine (`SHAMBLE_ROLE` set):** every kit/gate write is **refused**: `writePath`,
`routine-guard.mjs:403-407` ("routine (<role>): <rel> is a rules, CI or gate file; those change only in a daytime kit
change-set"), shell writers :501; the night commit and push checks and the night `git switch` check compare kit files
with origin/main (STATUS:110; `NIGHT_SHIFT.md:27-30`); `NIGHT_SHIFT.md:60-61` ("never change .claude/ or CLAUDE.md");
`CLAUDE.md:143-144`. A pr-advisor merge of a PR that changes any protected file is refused (:1460-1464). A
PermissionRequest in a routine is refused (:376-379).

**Settings itself:** `.claude/settings.json:152` `"ask": []` (empty; the day ask comes only from the guard hook);
hooks registered at :266-305 (PreToolUse, PermissionRequest, ConfigChange: the `node -e` bootstrap that imports
`guard.mjs`, `"onFailure": "block"`; PostToolUse `syntax-gate.mjs` on Edit|Write).

## 6. Tier, class and category words already in use

**In the rules (what his tiers would map onto):**
- **"Escalation categories 1-4"**, `advisor.md:36-43` (defined; see section 1), used at `advisor.md:48` ("escalation
  category 2"), `:114` ("any escalation category above"), `:165` ("escalation categories 1–4"). CLAUDE.md does not
  use the word "category"; it says "anything a player sees, hears or feels" (:88-89, :95-96) and "rule changes and
  other major changes" (:89).
- **Classes "ready" / "needs-mustafa"**, `advisor.md:113-119` (step 4 "Classify"), labels `ready-to-merge`,
  `needs-mustafa`, `changes-requested` (:134-145). CLAUDE.md:87-88 uses `needs-mustafa` and "no human check is
  needed".
- **kit-v2-1's "category 3"**: `docs/reports/2026-10-17-kit-v2-1.md:20-21` ("Category 3 adds `.github/` and every file
  the gate or CI runs as code or reads as settings (not the game files they check), including `tools/package*.json`
  and any `.npmrc`."); referred to at `docs/STATUS.md:985` ("kit-v2-1's category 3 (gate files need Mustafa's go)",
  a deferred Low: `tools/gate.mjs:333` and `tools/README.md:34` call `gate-globals.json` "reviewed data, not a gate
  tool").
- "kit" / "gate" files (the guard's protected set): see section 5. "kit change-set" is the usual name for a change-set
  that edits them (e.g. `routine-guard.mjs:407`, STATUS:546 "a kit change-set, so it needs his go").
- Change types (protocol step 4, `CLAUDE.md:69-77`): logic, visual / UI, gameplay / AI, performance, netcode,
  research; these set the evidence, not the merge class.
- Reports' "escalation category N" usage: `docs/reports/2026-11-01-v2-decision-drafts-appendix.md:299, :305, :449,
  :1098 ("The PR is needs-mustafa (ADRs, escalation category 3)"), :1116, :2116`; `docs/adr/0005-content-abilities-
  assets.md:94`.
- `docs/LEGACY_INVENTORY.md:413` (KT3, "Merge authority") describes the classes but is stale: it cites old CLAUDE.md
  and advisor.md line numbers and a routine "on GitHub events" (now schedule-only, STATUS:765-768).

**"Tier" is already taken elsewhere (possible name collision, a fact only):**
- Game tech tiers: `docs/LOOP_V2.md:68` ("`[DECIDED]` Tier 1, $100 each:"), :80 ("`[DEFAULT]` Tier 2"), :143, :185,
  :200; `docs/STATUS.md:743` ("a possible Tier 3 or hero unit"); `docs/ROADMAP.md:35` ("Officer (tier 2)");
  `docs/LEGACY_INVENTORY.md:95-96`; ADR 0006 part 12 "tech tier per team".
- Render quality tiers: `docs/adr/0003-rendering.md:121, :148, :324`.
- Rust target tier: `docs/research/r1-simulation-core.md:1024`.
- "Risk tier" / "Tier 0" appear only in his queued words (STATUS:43, :548-550, :570) and the report
  `docs/reports/2026-11-04-docs-after-kit-v3-1.md:30-32`.

**How his tiers line up with today's text (facts side by side, no mapping decided):**
| His tier (STATUS:548-550) | Nearest today | Differences visible in the text |
|---|---|---|
| Tier 0 "docs only (CI + PR Advisor, auto-merge)" | `ready` (advisor.md:118: "docs, tools (not the gate's own files), tests, ...") merged by the PR Advisor via the MCP merge tool (advisor.md:134) | GitHub auto-merge is refused by the guard for every session (`routine-guard.mjs:1393`); ADR edits are docs but escalation category 3 (advisor.md:39); `ready` also holds tools, tests, refactors |
| Tier 1 "game code and tools (CI + advisor; Mustafa's Brave check when players see or hear it)" | `ready` for tools/tests/headless fixes; `needs-mustafa` for player-facing (category 1) and for any `shamble-*.js` or page change when the routine reviews (advisor.md:103-105) | his words say "sees or hear"; CLAUDE.md:88-89 and advisor.md:37 say "sees, hears or feels" |
| Tier 2 "rules, CI, guard, secrets, infrastructure (advisor + security-auditor, always Mustafa's go)" | category 3 (advisor.md:39-42) -> needs-mustafa; guard refuses the PR Advisor's merge of such PRs (:1460-1464) | security-auditor is not a required reviewer of kit PRs today (CLAUDE.md:79 lists it for "network, server, storage, dynamic DOM, deps, CI"); "secrets" and "infrastructure" are not named in category 3; ADRs and hard constraints are in category 3 today |
| Tier 3 "irreversible or money (Mustafa decides; agents only propose)" | category 4 (advisor.md:43: "deleting data or saves, rewriting history"); `infra-engineer.md:40` ("Never: create accounts, buy services, add secrets or change repository or CI settings; those are Mustafa's"); ADR 0006 :24-25 ("every account, payment, secret and repository setting named below is Mustafa's to create") | "money" is in no escalation category today |
| Category 2 (design decisions the docs don't settle) | — | no tier of his names design decisions |

## 7. The docs answers carried in (XP-TERM, LOOK, the four Low notes on #43)

**His answers (2026-11-07, about 11:28 UTC), word for word, as saved in this branch's notes**
(`.checkpoints/kit-v3-4/mustafa-2026-11-07-answers.md:9-11`): "merge #43. XP-TERM: (a), all account progression (the
account level and Service points), never the in-match weapon XP: amps reset every match, so there's nothing to farm,
and cutting them would only make practice with bots feel worse. LOOK: (a)." The checkpoint (`.checkpoints/kit-v3-4.md`,
"Current step") says the advisor rules whether they ride A4's docs pass or a small docs change-set of their own.

**Where XP-TERM sits today (all still open in the text):**
- `docs/VISION_V2.md:71-75` (verbatim): ":71 On progression in private (rented) matches (Mustafa, 2026-11-06): / :72
  `[DECIDED]` With bots in the match: 25 % of normal XP; humans only: 100 %. / :73 `[OPEN]` Open, Advisor's
  suggestions: (1) weigh XP per action (kills on bots give little), so public matches with / :74 bot fillers aren't
  punished; (2) against friends trading easy wins: full XP only with a minimum number of humans / :75 per team, plus a
  daily XP cap on private matches." Header line `docs/VISION_V2.md:6-7` ("on 2026-11-07, his private-match XP item of
  2026-11-06 (§5)").
- `docs/STATUS.md:506-517`, follow-up "From vfx-xp-notes" (header :504-505), the XP-TERM entry: ":506 - **XP-TERM:**
  his private-match rule says "XP", and "XP" already names other things in the docs: the in-match ... :510 ... This
  pass maps his "XP" to none of them; the question goes to :511 him in its report's "Decisions needed" (XP-TERM). ...
  :516 ... an answer (a) to (c) would also settle :517 B.2's rented-servers follow-up above."
- `docs/STATUS.md:494-498` (B.2's rented-servers follow-up): ":496 ... His XP rule of 2026-11-06 for private (rented)
  matches (VISION_V2:71-75: 25 % of normal XP :497 with bots in the match, 100 % humans only) bears on it and does not
  answer it: which progression his "XP" means :498 waits on XP-TERM ("From vfx-xp-notes", below), and the design
  session for community servers decides the rest."
- `docs/reports/2026-11-07-vfx-xp-notes.md:57-61` ("Decisions needed", XP-TERM: options (a) all account progression
  (the account level and Service points); (b) Service points only; (c) the account level only; (d) also the in-match
  weapon XP. Recommendation: (a)); Risks :51 ("'XP' stays undefined until he answers XP-TERM; the drafts' account step
  waits on it"); For Mustafa :68-69.
- Register rows that cite his XP rule ("bears on it" clauses, per the report :21-22): `docs/adr/README.md:160`
  (OQ-0006-21) and `:183` (R2-ROOM-OQ7); STATUS:494-495 names R6-TEL-OQ2 (README:202, "whether bot matches and private
  matches count in balance rates is a product rule").

**Where LOOK sits today:**
- `docs/STATUS.md:518-525`, follow-up "LOOK" (":518 - **LOOK:** his "Comic flavour: ..." (EDITOR_V2 §7) :519 against
  LOOK_V2:63's `[DEFAULT]` "Comic impact touches for big moments," "used sparingly" (LOOK_V2 §7). ... :521 ... It also
  bears on OQ-0003-15 (the :522 legacy look touches, comic words first; README:103, which now points at it). No
  LOOK_V2 line is edited; for the :523 look brief's owners (Mustafa and the Advisor); the question goes to him in this
  pass's report (LOOK).").
- `docs/reports/2026-11-07-vfx-xp-notes.md:62-65` ("Decisions needed", LOOK: (a) "no, §7 stands until his VFX style
  guide and the look proof test"; (b) "yes, the look brief's owners update §7 in their own pass"; Recommendation:
  (a)). It does not name OQ-0003-15 (the advisor's Low note 4). Risks :52.
- `docs/adr/README.md:103` (OQ-0003-15 row, already "Mustafa (2026-11-06) bears on it, with his future VFX editor
  (EDITOR_V2 §7) ... Nothing is decided here.").
- CLAUDE.md lines 5-6 / 23-24 and advisor.md:48: the look is Mustafa's and the Advisor's; sessions never decide it.

**The advisor's four Low notes on PR #43's final head** (comment 6701970962, as the branch checkpoint
`.checkpoints/kit-v3-4.md` records them; the comment itself not re-read here):
1. **The vfx report's "STATUS:660"**: `docs/reports/2026-11-07-vfx-xp-notes.md:31`, verbatim: "- E4: STATUS:660's
   EDITOR_V2 citations moved to :94 and :95; ...". On main the line holding those citations is `docs/STATUS.md:681`
   ("`docs/EDITOR_V2.md:94` (§5: "It still runs by double-clicking editor.html") and `docs/EDITOR_V2.md:95` (§5: "if"),
   confirmed by grep; STATUS:660 holds other text ("the consistency pass (row CF-3).").
2. **"four register rows" (it is five):** `docs/STATUS.md:107` (vfx-xp-notes row): "... "bears on it" clauses in four
   register rows (`docs/adr/README.md:104`, `:122`, `:160` and `:183`: OQ-0003-16, OQ-0005-CD7, OQ-0006-21 and
   R2-ROOM-OQ7), no ADR draft or LOOK_V2 edited; ..."; `docs/ROADMAP.md:94`: "... "bears on it" clauses in four
   register rows (OQ-0003-16, OQ-0005-CD7, OQ-0006-21, R2-ROOM-OQ7; no ADR draft or LOOK_V2 edited); ...". The fifth is
   OQ-0003-15 at `docs/adr/README.md:103` (the report's own :21 lists five: ":103), OQ-0003-16 (:104), OQ-0005-CD7
   (:122 ...), OQ-0006-21 (:160) and R2-ROOM-OQ7 (:183)"; art-director AD-1 at report :42 "a fifth register clause").
3. **Why R6-TEL-OQ2, OQ-0005-AB3 and AB5 got no clause:** the vfx report says nothing about them (grep of
   `docs/reports/2026-11-07-vfx-xp-notes.md` for R6-TEL-OQ2, AB3, AB5: no hits). Their rows: `docs/adr/README.md:202`
   (R6-TEL-OQ2, "DESIGN: Mustafa | whether bot matches and private matches count in balance rates is a product rule"),
   `:69` (UPGRADE-SCOPE: levelling, OQ-0005-AB3, quotes "XP from kills, assists and objective work"), `:66`
   (WEAPON-UNLOCKS, OQ-0005-AB5). STATUS:507-508 cites :69 (OQ-0005-AB3) as one of the other "XP"s.
4. **The report's LOOK entry should name OQ-0003-15:** report :62-65 (above) does not; STATUS:521-522 does.
- The report has no "corrections after the merge" section yet (its sections: `## Goal` :2 ... `## For Mustafa` :66,
  `## Appendix A` :71, `## Appendix B` :97); the B.2 report got one from vfx-xp-notes (listed in the vfx report at :25, "The B.2 report:
  corrections after the merge"), a precedent for where notes 1, 3 and 4 could land.

## 8. Not found, not re-read, and tensions in the text

**Not found in the repository:**
- No separate PR Advisor routine doc or prompt file; its stored prompt is only recorded in `docs/STATUS.md:774-777`,
  and `advisor.md` §"PR review" is its procedure.
- No definition of a "review round", a review budget, a fix pass, skeptics or nits in any rules file (only in STATUS
  and reports, section 2).
- No severity scale with meanings; only security-auditor's four labels (`security-auditor.md:52`).
- No rollback or revert rule for main, and no "what to ignore" list for reviewers.
- No mention of GitHub auto-merge as an allowed path; whether the repository's "Allow auto-merge" setting is on is
  recorded nowhere.

**Not re-read live in this pass (read-only, no GitHub calls):** the branch-protection ruleset (STATUS:983 is the
2026-10-17 read); the advisor's PR #43 comment 6701970962 (taken from `.checkpoints/kit-v3-4.md`); the routines'
current settings (STATUS:752-777 is the 2026-11-04 read).

**Tensions between his A4 words and today's text (facts only, for the plan to address):**
1. Tier 0 says "auto-merge"; the guard refuses `enable_pr_auto_merge` for every session, day included
   (`routine-guard.mjs:1393`, reason: "auto-merge skips the expected-head check"), and `gh pr merge`
   (:1340, settings.json:229). The PR Advisor's own MCP merge with `expectedHeadSha` (advisor.md:134, :143-144) is
   today's unattended merge. Changing the guard is a Tier-2-style kit change (guard code + tests).
2. Tier 2 names security-auditor as a required reviewer of rules/CI/guard/secrets/infrastructure PRs; today
   CLAUDE.md:79 names it by subject ("network, server, storage, dynamic DOM, deps, CI"), not for all rules/kit PRs.
3. His Tier 1 says "when players see or hear it"; CLAUDE.md:88-89, :95-96 and advisor.md:37 say "sees, hears or
   feels".
4. ADRs and hard constraints are escalation category 3 today (advisor.md:39), i.e. needs-mustafa; his Tier 0 is "docs
   only". Whether an ADR or VISION/LOOP doc edit is Tier 0 is not settled by his words. Category 2 (design decisions)
   has no tier of its own.
5. "Money" (Tier 3) is in no escalation category today; category 4 covers deleting data and rewriting history.
6. Review budget ("at most two review rounds per change; a third only for a Critical or High finding") vs
   `CLAUDE.md:78-83` / changeset SKILL.md:25-26 ("Fix and re-run until PASS") and advisor.md:145 ("you review again",
   uncapped); only security-auditor uses Critical/High; others "ranked"; STATUS shows major/minor/nit used too.
7. advisor.md:167 ("Approve without running the gate yourself") vs the routine reading CI's checks
   (advisor.md:100-101) — an existing wording gap any per-tier checklist (item 5) sits next to.
8. Rollback rule ("revert the merge first"): main's ruleset requires a PR with `gate` and `smoke` (STATUS:983), so a
   revert is a PR; who may merge it follows today's classes (a revert of a kit PR touches kit files, so the PR Advisor
   cannot merge it: guard :1460-1464); `CLAUDE.md:240` "edit before the plan is approved" has no emergency
   exception; `git revert` is unread by the day guard (L4, STATUS:867).
9. "Tier" already names game tech tiers (LOOP_V2:68, :80; STATUS:743) and render quality tiers (ADR 0003).
10. Item 4 (fast checks on save) is already planned as ADR 0002 part 7's "per-edit hook" (`0002:472-474`), a
    `.claude/` change in a phase-1 kit change-set, without format checks in the hook (format is gate G1); SPIKE_PLAN's
    foundation entry (:420-438) does not mention it.
11. Item 6: browser telemetry (ADR 0003 client field data) is planned for slice 6 (`0003:1245`, :1410-1411), while his
    words say "from the first playtests on"; browser error reports are planned "before the first online playtest"
    (ADR 0006 :1483-1486); client-deploy rollback is "not read" and server rollback is an interim proposal under
    OQ-0006-14 (ADR 0006 :584-586, :999-1015).
12. The v2 gate plans one aggregate required check (ADR 0002 :595, :1407-1408) where advisor.md:94, :163 and the guard
    (:1454) hard-code `gate` and `smoke`.
