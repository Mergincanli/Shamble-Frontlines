# Checkpoint: kit-v3-5 (F1, the guard's rename gap, agent memory included; Mustafa's GUARD-RENAME (c))

- **Branch:** `claude/sleepy-tesla-ofsxsr`, standing in for `cs/kit-v3-5`: this cloud session may push only there.
  - Worktree: `/tmp/claude-0/-home-user-Shamble-Frontlines/263eddf3-7643-5640-b1d5-39be43d3567c/scratchpad/a5`.
  - Local branch `kit-v3-5`, from main 44446a5 (PR #44, kit-v3-4, merged on his go). Notes committed by hand with
    [skip ci].
- **His words:** GUARD-RENAME (c) and "merge #44" of 2026-11-09, word for word in
  `.checkpoints/kit-v3-5/mustafa-2026-11-09-answers.md`: the guard fix, agent memory included, is the very next
  change-set; until it merges he pauses the PR Advisor routine (he does it himself: the session cannot edit a routine
  it did not create; at 18:20 UTC it was still enabled, next run 18:45 UTC).
- **What F1 is** (STATUS "From kit-v3-4", F1, and the two final-head reviews of #44, comments 6740050583 and
  6740560231): the PR Advisor's merge check reads both names of a renamed file; refuses a pr-advisor merge when the
  PR's `changed_files` is 3000 or more (SA-8); `protectedRel` protects `CLAUDE.local.md` and `AGENTS.md` anywhere;
  force-added `.claude/agent-memory-local/` files are refused in a merge (the advisor's new Critical, 2026-11-09);
  test rows. SA-5 (mechanical refusal of pr-advisor merges of `docs/adr/**` and the decision docs) only if he wants
  it. Once on main, advisor.md's tier-2 bracket ("it does not yet see ...") goes. Risk tier 2: his go; each guard
  edit asks him in a day session.
- **Also carried (docs, after #44):** STATUS and ROADMAP mark #44 merged; STATUS:44-45 still calls #44 a draft (the
  PR Advisor routine's Low); F4 to name agent memory and its "every gap is on the strict side" (the advisor's Low);
  the kit-v3-4 report's two Lows (mixed verdict rows counted by their first word; Appendix D's ddb06a5 and 2894a1f);
  his 2026-11-09 answers recorded.
- **Queued, his words:** his unit walk of 2026-11-09 (cb6636d, `mustafa-2026-11-09-unit-walk.md`, 2449 characters,
  byte-equal to the transcript) and his queue of 2026-11-10 (d410292, `mustafa-2026-11-10-queue.md`, 1484 characters):
  KIT-FREEZE-2 (the kit freezes after kit-v3-5 merges, until the foundation's gate step; exceptions: a broken main,
  STATUS-DIET, LANE-RULES), STATUS-DIET (docs), LANE-RULES (rules). This pass's docs part queues all of them in
  STATUS's "Queued, in Mustafa's words".
- **Main after #44:** the push run on 44446a5 (42673290304) lost its `gate` job on GitHub's side (the run read failure
  with only `smoke`, green); the gate on main's exact tree (a4 at 7c36d5a, the same tree) vs 626df67: PASS, +0 −0;
  one re-run (attempt 2, 2026-11-10 06:37 UTC): `gate` and `smoke` success. Main is green; it was never broken.
- **The PR Advisor routine:** paused by Mustafa (enabled false, 2026-11-09 18:22 UTC); it did not run at 18:45.
- **Facts:** `facts.md` B1-B8 done; the agent stopped on the org's spend limit (2026-11-09 18:30 UTC) and was resumed
  2026-11-10 for B9 (not found, tensions).
- **Done:** the facts (`facts.md`, B1-B9, 360 lines); plan revision 1 (`plan-rev1.md`, f6be38e): parts A-G
  (protectedRel gains CLAUDE.local.md and AGENTS.md; a merge-only `mergeProtected` for agent memory and nested
  `.claude/`; the file list reads both names via `@tsv`; the 3000 cap and a short-list check; test rows red then green,
  mutants; advisor.md tier-2 lines, CLAUDE.md:64, F4 if ruled; the docs part), decisions SA5-RIDE (his), F4-RIDE and
  MERGE-ONLY (the advisor's).
- **Done:** the advisor's plan review (`advisor-plan-rev1.md`, a36d369): REVISE, R1-R14, conditions 1-14 (letter
  case on every platform; every gitignored path; each clause needed alone; CANON's new names; check-run total_count;
  changed_files rows; pagination recorded (PR #40 fails closed here); the `@tsv` parse; exact rules texts; write-side
  limits recorded; F5 waits; the kit-v3-4 correction appended; small text; ask SA5-RIDE now). Rulings: F4-RIDE fold
  in; MERGE-ONLY approved. Plan revision 2 (`plan-rev2.md`, 890ed2d): C0-C5 clause table, C3 by `git check-ignore`.
- **Current step:** 3, the advisor's delta check of revision 2 (running), and SA5-RIDE asked of Mustafa; then the
  scratch build (tests red, guard, rules lines), the writes after his answer (one ask per kit file).
- **Open with Mustafa:** SA5-RIDE (the advisor's rewritten text); ORDER (unit walk vs STATUS-DIET first).
