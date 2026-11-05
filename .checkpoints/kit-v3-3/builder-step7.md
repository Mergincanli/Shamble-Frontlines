Task: kit-v3-3 builder, plan rev2 section 8 step 7: rule texts of sections 4.5-4.7 (CHECKPOINTS.md, advisor.md, CLAUDE.md, NIGHT_SHIFT.md, changeset skill, START_HERE), without WT-NIGHT parts.

Plan (blocks, in order):
B1 read plan-rev2 sections 4.5-4.7, 6 (E7), 8 step 7
B2 read advisor-e10.md, e10-docs.md, mustafa-words.md, advisor-plan-rev1 / advisor-build-step4 relevant bits
B3 edit CHECKPOINTS.md
B4 edit advisor.md
B5 edit CLAUDE.md
B6 edit NIGHT_SHIFT.md (not :26)
B7 edit changeset skill
B8 edit START_HERE
B9 line-length check, E7 greps
B10 node tests, repo-map-check
B11 answer

Notes:
- B1 done: plan-rev2 §0-§3, §4.5-§4.7, §6 E7, §8 step 7 read. Next B2.
- B2 done: advisor-e10 (E10-1..9; Never bullet keeps his reason word for word), mustafa-words, step-4 ruling grep (R1-R4 touch tests/code, not step-7 texts). Next B3.
- finding: plan's Never bullet paraphrases his words ("move ... that ref's"); task says word for word from mustafa-words.md -> use his exact rule+reason sentences, keep plan's git examples and WT-UPDATE "still allowed" forms (not the WT-NIGHT exception).
- B3 done: CHECKPOINTS.md :84-86 (worktree agents' notes: session's own checkout), :89 (memory: local clause, 'until A3' removed), :190-193 (§8 step 2 worktree resume, no WT-NIGHT clause). Next B4 advisor.md.
- B4 done: advisor.md step 0 (+only-switch sentence; Known limits -> CI details, list = ACTIONS_READ routine-guard.mjs:889-892), step 2 day seat worktree, Your notes (E10-2 text). memory: local :9 kept. Next B5 CLAUDE.md.
- B5 done: CLAUDE.md step 3 appended (C4/E10-1/E10-6), :134-137 routine guard clause + python, :144-150 split reload text (E10-3), :212 and :217-218 agents never kit/gate, Never bullet appended (his rule+reason verbatim, no exception clause). Next B6 NIGHT_SHIFT.
- B6 done: NIGHT_SHIFT.md :8-10 node or python on main's tracked scripts (rewrapped; old :8-9 were 121/122 chars). :26 handoff ("Check out that branch and resume from the checkpoint") left as is: waits for WT-NIGHT.
- B7 done: SKILL.md :11-13 branch from origin/main (fetch; switch -c or worktree).
- B8 done: START_HERE.md :21-23 cut from origin/main, fetch first, never move main folder (merged into the existing (`CLAUDE.md`, `/changeset`) bracket). Next B9 line lengths + E7.
- fix: two Edits left long lines (CHECKPOINTS.md :90 137 chars, CLAUDE.md :142 171 chars) -> rewrapped; all added lines now <=120 (awk over git diff -U0: 0 long).
- check: Mustafa's rule+reason (mustafa-words.md, whitespace-normalised, minus the leading "never" that the "## Never" heading supplies) found verbatim in CLAUDE.md: true.
- B9 done, E7 results:
  - worktree rule: CLAUDE.md:246-251 (Never bullet), CHECKPOINTS.md:190-193 (§8 step 2), advisor.md:96 (day seat) + step 0 only-switch sentence, START_HERE.md:22-24, SKILL.md:11-13 (origin/main + "CLAUDE.md, Never").
  - "until A3": 0 hits in the six step-7 files; 2 hits elsewhere: docs/STATUS.md:623 (A3 queue item 3 text; docs-keeper's §4.8 pass, step 8) and docs/reports/2026-11-04-docs-after-kit-v3-1.md:49 (a dated past report: history, leave).
  - python: CLAUDE.md:142 yes, NIGHT_SHIFT.md:8 yes; guard refusals NO: routine-guard.mjs:650 "...npm ci and node on main's scripts" and :652 "...npm ci, node (main's scripts) and date" still name node only. Plan §4.1 (:291-292) and §5 (:501, assert) put that in the code steps; step-4 builder flagged it as not in its list; not built yet. Not my file: OPEN for the main session.
  - "keeps the hook settings it loaded" in CLAUDE.md: 0.
  - WT-NIGHT: NIGHT_SHIFT.md:27 (was :26) "Check out that branch and resume from the checkpoint" unchanged; Never bullet has no exception clause.
Next B10 tests + repo-map.
- B10 done: node --test routine-guard.test.mjs settings.test.mjs checkpoint.test.mjs -> 528 tests, 528 pass, 0 fail (no test file touched in step 7). repo-map-check: "repo map OK: 113 entries cover 100 required paths", exit 0.
- Final hunks (head line numbers): CHECKPOINTS.md 84-86, 89-93, 190-193; advisor.md 73-85, 95-96, 154-159; CLAUDE.md 63-68, 140-144, 148-153, 222, 226-227, 246-251; NIGHT_SHIFT.md 8-10; SKILL.md 11-13; START_HERE.md 22-24.
- No test edited; no commit/push; no WT-NIGHT part built.
DONE
