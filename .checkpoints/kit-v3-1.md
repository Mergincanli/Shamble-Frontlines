# Checkpoint: kit-v3-1

- **Goal:** A1 (Mustafa, 2026-11-03): PR #30 onto main, its conflicts resolved, the GUARD-FILE (a) fix inside it,
  re-reviewed (security-auditor on the fix, then the advisor on the whole PR), then wait for Mustafa's go · plan:
  `.checkpoints/kit-v3-1/a1-plan.md` (revision 2) · verdict: **APPROVE with conditions** (advisor, 2026-11-04;
  `.checkpoints/kit-v3-1/advisor-plan-rev2.md`)
- **Branch:** `cs/kit-v3-1` (PR #30), head after plan commits `a4eb868` · **Base:** origin/main `e4d8b22`
- **Session:** main folder `C:\dev\shamble-fresh` stays on `main`; build in worktree `<scratchpad>/kit-v3-1`
- **Last update:** 2026-11-04 ~00:05 Amsterdam

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

## Current step
BUILD step 1: merge origin/main (e4d8b22) into cs/kit-v3-1, taking main's side in the three doc conflicts (-X theirs),
then verify (C4) and commit.

## Next steps
1. Step 1 merge (in hand). 2. Step 2 GUARD-FILE fix (settings.json 3 handlers → `-e` + promise-only BOOT; settings.test
   tests a–d incl. the defect-proof C1; guard.mjs header comment C7). 3. Gate+tests. 4. docs-keeper (C5). 5. Reviews:
   security-auditor on the fix (with Q3/C6 in front of it) → advisor on the whole PR from main's advisor.md (C8).
   6. Delete checkpoint, report, mark ready, keep needs-mustafa, wait for Mustafa's go (C9).

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

## Open findings
- Q3 (accepted (a)): 9 pre-existing Windows-only failures in routine-guard.test.mjs; follow-up per C6. Status: documented.

## Key numbers
- settings.test.mjs 82/82 (Windows/Node24). routine-guard.test.mjs 183/192 (Windows, 9 pre-existing) · 192/192 Linux.
- BOOT matrix: all cases as intended (Node v24.17.0).

## Decisions taken
- GUARD-FILE (a) · Mustafa · 2026-11-03. Working rule (no branch switch; worktree) · Mustafa · 2026-11-03.
- BOOT = promise-only import('node:url') form · this session. Advisor APPROVE + Q1/Q2/Q3 rulings · advisor · 2026-11-04.

## Running work
- none
