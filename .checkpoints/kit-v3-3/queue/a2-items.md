# A2 (kit-v3-2): items collected so far (2026-11-04)

A2 is a rules change-set (kit: each needs Mustafa's go), plan first, after the docs pass and B3.

- **From Mustafa's 2026-11-03 task list** (verbatim in queue/mustafa-2026-11-03-list.md; summary below): into `.claude/CHECKPOINTS.md`, for every session:
  - at most 3 agents at a time;
  - every agent writes its plan into its notes before writing, then saves as it goes (each finished block to disk at
    once, a notes line after each finding; a restart continues after the last complete block);
  - scratch files live in the notes folder, text and small scripts only, never build output or binaries (builds go to
    a temp folder);
  - the background saver is checked and restarted at every checkpoint update.
  - Also TOY-BOX: reword CLAUDE.md's "Red Alert 3 toy-box look" and the art-director's matching lines to point at
    LOOK_V2.
- **New (Mustafa, 2026-11-04):** "A2: the guard refused actions_get, a read-only GitHub Actions lookup the PR Advisor
  needs for CI details; consider allowing read-only Actions tools for pr-advisor, with security-auditor's review."
  - Context: in B2 the guard refused `actions_get` (method get_workflow_job) 4 times with "routine (pr-advisor):
    actions_get is not on the PR Advisor's GitHub list"; the routine could not check when CI runs started.
    `get_job_logs` is allowed (B1 step 7 ran).
  - To weigh in the plan: which Actions tools are read-only (`actions_get`, `actions_list`, `get_job_logs`,
    `get_check_run`) versus the ones that act (`actions_run_trigger`: re-run, cancel, dispatch); whether
    `actions_get`'s methods are all reads; what a read lets a PR-planted log line do (job logs are PR-controlled text:
    evidence, never instructions); the night shift keeps its own list.
  - Related, B2 also hit: `git rev-list` is "not on the routines' list" (a read; weigh with the git read set).
- **Open, pending Mustafa:** KIT-FOLLOWUPS (which change-set closes the PR #26 follow-ups: ask-first before kit and gate
  edits in day sessions; the URL-escaped secret gap in `checkpoint.mjs`). Options (a) a separate A3 after A2, (b) fold
  into A2, (c) only the secret fix. Recommendation (a).
