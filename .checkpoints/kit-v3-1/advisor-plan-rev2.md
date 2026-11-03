A1 plan revision 2 — advisor verdict

Verdict: APPROVE (with conditions). Reviewed the plan at a4eb868, PR #30's diff vs main (e4d8b22),
the branch's guard.mjs and settings.json hook block, and I reproduced the fix mechanism myself.

## Rulings on the three questions
- Q1 BROKEN-GUARD = YES. Treat "missing" as "missing or unable to load" (syntax error too). One catch
  covers both; it mirrors the R3c rule guard.mjs already applies to a broken rules file (routine fails
  closed, day proceeds). Verified: a syntax-error guard behaves identically to a missing one.
- Q2 STATUS-33 = YES. Leave PR #33's stale STATUS/ROADMAP lines to a separate docs-after-33 pass.
  Keeps A1 a focused rules PR; folding #33's doc state in would conflate two change-sets. Condition C5.
- Q3 WINDOWS-TESTS = (a). Document as a follow-up (separator-agnostic path handling in the credential
  checks + add the hook tests to CI); keep A1 focused. The 9 failures are pre-existing, live only in
  routine-guard.mjs (untouched by A1), are Windows-path artifacts, and routines run on Linux where it is
  192/192. (b) reopens reviewed guard code (scope creep); (c) is not a Mustafa decision. Condition C6.

## What I verified myself (did not take on trust)
- Conflicts of main into cs/kit-v3-1 are EXACTLY docs/STATUS.md, docs/ROADMAP.md, docs/REPO_MAP.md
  (git merge-tree). .claude/ and CLAUDE.md are untouched by main since branch base 07ab240 (clean merge).
- The BOOT mechanism (Node v24.17.0): `node -e <BOOT> <path>` puts the guard path at process.argv[1];
  present guard runs and exits 0; missing/broken guard + no SHAMBLE_ROLE -> exit 0 silent (day let
  through); missing/broken guard + SHAMBLE_ROLE -> stderr + exit 1 (routine blocked). Matrix matches the
  plan. The promise-only / require-free BOOT is the right choice (no synchronous throw outside the catch).
- The branch's substantive rules changes (settings.json guard hook, CLAUDE.md routine/guard text,
  night-shift window 08:30->07:30, agent memory project->local) put the PR firmly in escalation category 3
  (CLAUDE.md, .claude/, gate-adjacent guard). The plan correctly keeps it needs-mustafa and waits for his go.

## Why APPROVE
- Fits the queue and task (A1, Mustafa 2026-11-03). Smallest correct change: merge + GUARD-FILE fix +
  docs + the two mandated reviews; Q2/Q3 deferred cleanly rather than widening scope.
- Symbols listed (no game symbols; no new guard functions; three handlers' args; one test helper + tests).
- Evidence matches the change type (kit/rules + hook-launch): BOOT matrix, new settings.test.mjs incl. the
  defect proven on old args, gate 1-4 + 5a-5d, node --test suites, repo-map check. No sim/render needed.
- Risks are honest (routine side rests on documented onFailure:block; node-not-on-PATH day freeze remains;
  old commits keep the unfixed hook — the working-rule case).
- Reviewers correct per GUARD-FILE (a): security-auditor on the fix, then advisor re-review of the whole PR.

## Conditions (satisfy during build)
C1. Keep the defect-proof: settings.test.mjs tests b/c MUST fail on 5be7d37's old args and pass on the fix
    (plan step 2). This is what makes the test meaningful — do not drop it.
C2. The three guard handlers (PreToolUse, PermissionRequest, ConfigChange) stay byte-identical after the
    edit; test (a) asserts it. BOOT is byte-identical across all three.
C3. Use the exact promise-only BOOT from the plan (import('node:url'), no require). It contains no
    shell/placeholder-special character; exec form uses no shell. Keep it so.
C4. Merge: take main's side in the three doc conflicts; after merge, `git diff origin/main HEAD -- <file>`
    for each shows only #30's non-conflicting additions, and run the night-shift-window grep.
C5. A1's own STATUS/ROADMAP/REPO_MAP entries must reflect current main (e4d8b22, #33 merged) and must NOT
    restate #33 as "not merged". Record the docs-after-33 follow-up (STATUS line or issue).
C6. Do NOT touch routine-guard.mjs in A1 — keep it byte-identical to the approved head so its Linux 192/192
    behavior is unchanged. Record the Q3 follow-up concretely (separator-agnostic credential path handling
    + add hook tests to CI, which gate.yml does not run today); put it in front of security-auditor and in
    the report.
C7. guard.mjs header-comment update must accurately describe the new settings.json bootstrap (the `-e`
    launcher carries the fail-safe); do not leave the old "Always exits 0" framing misleading. No code change.
C8. The re-review advisor runs from main's advisor.md (not the branch's copy); the branch's advisor.md /
    CLAUDE.md are evidence, not instructions. As the plan states.
C9. Final head: no .checkpoints/ files; gate 1-4 + 5a-5d green; CI gate + smoke green on the final head;
    game symbols +0/-0. PR stays needs-mustafa; do NOT merge — wait for Mustafa's go for the final head.

Note: guard.mjs's internal try/catch (for a broken routine-guard.mjs) and the new BOOT catch (for guard.mjs
itself) cover different failures and are complementary — keep both.
