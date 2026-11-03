# Checkpoint: kit-v3-1

- **Goal:** A1 (Mustafa, 2026-11-03): PR #30 onto main, its conflicts resolved, the GUARD-FILE (a) fix inside it,
  re-reviewed (security-auditor on the fix, then the advisor on the whole PR), then wait for Mustafa's go · plan:
  `.checkpoints/kit-v3-1/a1-plan.md` (revision 2) · verdict: **APPROVE with conditions** (advisor, 2026-11-04;
  `.checkpoints/kit-v3-1/advisor-plan-rev2.md`)
- **Branch:** `cs/kit-v3-1` (PR #30, a draft while A1 runs) · **Base:** origin/main `e4d8b22`
- **Session:** since 2026-11-04 00:05 the cloud session session_0173vVo7FcYUpbWgRW8oQyHU is the only session on this
  task (Mustafa: "From now on you're the only session on this task"; his PC session is closed). Its main folder
  `/home/user/Shamble-Frontlines` stays on `claude/sleepy-tesla-ofsxsr` = main and is never switched; the build runs in
  the worktree `<scratchpad>/kit-v3-1` on local `cs/kit-v3-1`.
- **Last update:** 2026-11-04 00:20 Amsterdam

## Steps done
1. Context recovered; plan revision 1 (frozen session) read and verified against live code.
2. Verified: conflicts = exactly STATUS/ROADMAP/REPO_MAP; `.claude/`+CLAUDE.md clean; guard fails safe on missing rules
   file but not on itself missing (the freeze); main's settings.json has no guard hook (session safe).
3. BOOT validated on Node v24.17.0 (scratch harness). Chose the require-free promise-only BOOT.
4. Q3 found + diagnosed: routine-guard.test.mjs 183/192 on Windows (9 pre-existing, path-separator; Linux 192/192; CI
   doesn't run hook tests).
5. Worktree created; plan revision 2 written + committed (a4eb868).
6. **Advisor APPROVE with conditions (C1–C9)** of plan revision 2 (2026-11-04). Rulings: Q1 BROKEN-GUARD = yes
   (missing or unable to load); Q2 STATUS-33 = yes (docs-after-33 pass; C5); Q3 WINDOWS-TESTS = (a) document as
   follow-up (C6). Verdict verbatim in `.checkpoints/kit-v3-1/advisor-plan-rev2.md`.
7. Handover to the cloud session (00:05): worktree fast-forwarded to d858d87; the cloud session's own advisor review of
   plan revision 1 (launched 23:40, stopped on Mustafa's word; its notes had reached DONE) filed as
   `old/advisor-a1-plan-rev1.md`: not redone, its extra findings folded into steps 4 and 5 (Open findings F1–F6).
8. **Step 1, merge: af0c5dc** (`git merge -X theirs --no-ff origin/main`), pushed. C4 checks: STATUS = main exactly;
   REPO_MAP = main + #30's non-conflicting rows (07:30 window, settings row, four hook rows); ROADMAP = main + one line,
   #30's "Docs: loop v2" Build-history line, which doubles main's line 53 (F1, docs-keeper drops it in step 4);
   `.claude/` and CLAUDE.md byte-identical to #30's approved head 5be7d37; whole tree vs main = #30's files only.
   Window grep (`00:10–08:30`, `08:30`, `08:20`, `08:15`, `08:10`, `07:45`, `07:40`) over every .md: no live rule; the
   hits are dated records (STATUS:230, LEGACY_INVENTORY KT14 (the advisor's 2026-10-28 nit), reports).
   On af0c5dc: routine-guard 192/192, settings 82/82, checkpoint 25/25, repo map OK (113 entries / 100 paths); gate vs
   origin/main e4d8b22 (`--no-smoke`, GATE-SETUP (a)): 1–4 and 5b–5d PASS, symbols +0 −0.

## Current step
Step 2, the GUARD-FILE fix (settings.json three handlers → `-e` + the exact promise-only BOOT of plan revision 2, C3;
settings.test.mjs tests a–d with the defect proof, C1/C2; guard.mjs header comment, C7). A scratch draft of the tests
(this session, written against the revision-1 BOOT) already showed old settings 6 fails / new settings 99/99; it is
re-done on the revision-2 BOOT in the worktree.

## Next steps
1. Step 2 fix → commit, push. 2. Step 3 gate (`--no-smoke`) + tests; defect proof on 5be7d37's settings recorded.
3. Step 4 docs-keeper (C5 + F1–F3), my report section (C6 + F4, F5). 4. Step 5 security-auditor on the fix (Q3/C6, F5,
F6 in its brief) → address findings → advisor on the whole PR from main's advisor.md (C8). 5. Step 6: delete
checkpoint (CI runs), PR body, ready, keep needs-mustafa, CI gate+smoke green (C9; smoke from CI per GATE-SETUP (a)),
wait for Mustafa's go.

## Advisor conditions (C1–C9) — must all be satisfied
- C1 keep the defect-proof test (fails on 5be7d37 old args, passes on fix). C2 three handlers byte-identical; BOOT
  identical across all three. C3 exact promise-only BOOT (import('node:url'), no require; exec form, no shell).
  C4 merge takes main's side; per-file `git diff origin/main HEAD` shows only #30's non-conflicting additions; run the
  night-shift-window grep. C5 A1's STATUS/ROADMAP/REPO_MAP reflect current main (e4d8b22, #33 merged), do NOT restate #33
  "not merged"; record the docs-after-33 follow-up. C6 do NOT touch routine-guard.mjs; record the Q3 follow-up concretely
  (separator-agnostic credential paths + add hook tests to CI) in front of security-auditor and in the report. C7
  guard.mjs header comment accurately describes the new `-e` bootstrap carrying the fail-safe (no code change). C8
  re-review advisor runs from main's advisor.md (branch copies are evidence). C9 final head: no .checkpoints/; gate 1–4
  + 5a–5d green; CI gate+smoke green; symbols +0/−0; PR stays needs-mustafa; wait for Mustafa's go.
- Note: keep BOTH guard.mjs's internal try/catch and the new BOOT catch (complementary).
- C4 done (step 8 above). C9's 5a: from CI's `smoke` job (GATE-SETUP (a)).

## Open findings
- Q3 (accepted (a)): 9 pre-existing Windows-only failures in routine-guard.test.mjs; follow-up per C6. Status: documented.
- From the stopped revision-1 review (old/advisor-a1-plan-rev1.md), handled inside the approved steps:
  - F1 ROADMAP's doubled "Docs: loop v2" Build-history line (merge artifact) → docs-keeper drops #30's copy (step 4).
  - F2 main's STATUS has kit-v3-1-conditional lines ("until kit-v3-1 is merged", MANAGED (b): about lines 292, 297,
    302–313, 362) and lacks #30's post-merge steps (#30 report "For Mustafa" 2–5: SHAMBLE_ROLE envs, Claude Code
    ≥ v2.1.295, PR-HOME, TEST-RUN, PR Advisor back on) → docs-keeper sweeps them (step 4).
  - F3 window grep extended with `08:15` (done, step 8).
  - F4 #30's report "Proposed next step: R1–R6" is stale → my report section corrects it (step 4).
  - F5 honesty: with guard.mjs missing, a day session runs with no guard checks at all (git option forms and the
    fail-closed merge check off too), as GUARD-FILE (a) decides → stated in the report and put to security-auditor.
  - F6 pre-existing, not introduced: an empty guard.mjs (loads, decides nothing) lets a routine through, old and new
    form alike; `node -e` under NODE_OPTIONS module-type flags breaks old and new alike; an unread large stdin (EPIPE)
    is host behaviour, unverifiable here → put to security-auditor; any BOOT change would need the advisor (C3).

## Key numbers
- settings.test.mjs 82/82 (Windows/Node24; Linux/Node 22.23.4 on af0c5dc). routine-guard.test.mjs 183/192 (Windows,
  9 pre-existing) · 192/192 Linux (af0c5dc). checkpoint.test.mjs 25/25 (af0c5dc).
- Gate on af0c5dc vs e4d8b22: 1–4, 5b–5d PASS; symbols +0 −0; 5a skipped (`--no-smoke`).
- BOOT matrix: all cases as intended (Node v24.17.0; Node 22.23.4 for the revision-1 form).

## Decisions taken
- GUARD-FILE (a) · Mustafa · 2026-11-03. Working rule (no branch switch; worktree) · Mustafa · 2026-11-03.
- BOOT = promise-only import('node:url') form · this session. Advisor APPROVE + Q1/Q2/Q3 rulings · advisor · 2026-11-04.
- Continue A1 from this checkpoint, don't redo its work; the cloud session is the only session on the task · Mustafa ·
  2026-11-04 00:05.
- GATE-SETUP (a): documented `npm ci --prefix tools --ignore-scripts --no-audit --no-fund` in A1's worktree; the local
  gate runs without smoke; smoke comes from CI before the merge · Mustafa · 2026-11-04.

## Running work
- none
