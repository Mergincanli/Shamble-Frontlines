# advisor: review of the pushed head of #60 (kit-hold-ci), head e4156f1698d6c15fb0f56e3b4af3061c2906f2ad
Started 2026-11-18 19:04 UTC (date -u). Base main 9b4b518. Work from main's advisor.md (risk-tier-2 checklist).

## Plan of this review
1. Read the resume card, his three files of today, DECISIONS (11-13 Accept edits; 11-14; today's three), my plan notes.
2. A: git diff origin/main -- .claude .github; rebuild "his texts" (3ebe654a + his change + my fix + f7e232fb) and compare hunk for hunk; read the code in place.
3. Kit tests on the head (in a worktree of the head under .gate/, main's tests vs head's): guard, settings, checkpoint counts vs main.
4. Gate: full node tools/gate.mjs in a worktree of the head (day seat: I must run it myself).
5. B: read security-auditor F1/F3, test-engineer's three Mediums; probe F1 in a scratch repo; rate each.
6. C: read the session-record claims (mode at writes; the rm route); rule.
7. D: report sentence by sentence vs the record; quotes vs his files and DECISIONS by script.
8. E: STATUS/ROADMAP/ARCHIVE/REPO_MAP diff; moved lines byte for byte; other lanes' lines untouched.
9. F: git diff --stat origin/main -- . ':!.checkpoints'; nothing else in it.
10. H: rule on the final head.
11. Verdict.

## Findings (one line each, time from date -u)
- 19:06 Block 1 done: card read (steps 1-36); his three files read whole; each body equals its stated size and sha256 (1542 / 779 / 2618 B; the trailing newline aside); the merge-58 body stands byte for byte in 9b4b518's message.
- 19:06 G (from the main session, 19:06): on e4156f1 toolchain push 44376519102 + PR 44376533634 success; gate push 44376519103 gate + smoke success; gate PR 44376533569 gate success, smoke still running. The branch head moved on to a checkpoint commit (notes only, skip mark): no checks on it; the runs are on e4156f1.
- 19:06 Raw diff vs main: no link (120000) or submodule (160000); outside .checkpoints/ exactly 4 kit/CI files + DECISIONS, REPO_MAP, ROADMAP, STATUS, STATUS_ARCHIVE, the report. Commits touching code: 8bf4bee (3 kit), 94c197f (workflow), 0c0e271/415bd73 (plant + take-back of Join.ts), 4877599 (fix), a18a62b (DECISIONS), e4156f1 (docs + report).
- 19:07 A PASS: his texts re-extracted from git (3ebe654a kit-edit.diff 262 l. 70756f3d..; f7e232fb kit-fix2.diff 45 l. 9197f070..; ci-edit.diff 9a3b2945..); rebuilt by patch (scratchpad advf/rebuild.py): main + 3ebe654a text = "shown"; shown -> 8bf4bee adds only script +5 (2 header, 1 fn comment, 2 code lines with --literal-pathspecs), rules +2, tests +66 (test 33 with step 4), 0 removed; 8bf4bee + kit-fix2 == e4156f1 byte for byte (3 kit files); main + ci-edit == e4156f1 workflow; each of the 12 added workflow lines stands verbatim once in cs1 report @1bfe462 (l.283-305, section at l.279). Nothing else in the four files.
- 19:10 Gate in a worktree of e4156f1 (.gate/pr-60-advisor, node_modules linked from the main folder; tools/ unchanged by the PR): checks 1-4, 5b, 5c, 5d PASS (symbols +0 -0); 5a smoke could not start here (Playwright 1.56.1 wants chromium_headless_shell-1194; this machine has 1243 only): exit 2, a setup error, not a failure of the PR. Smoke rests on CI (push run 44376519103, job smoke success on e4156f1); it stands because the PR touches no legacy/ file and no gate file.
- 19:10 Kit tests (node v22.23.4): head guard 1107/1107, settings 127/127, checkpoint 33/33, kept-logs 10 (9 pass, 1 skipped, 0 fail); main (.gate/main-advisor): 1107, 127, 27, 10 (9 pass, 1 skipped). Same counts; checkpoint +6 new tests.
- 19:13 Cross-runs: main's 27 tests vs the head's script: 27/27; the head's 33 vs main's script: 27 pass, 6 fail (#28-#33).
- 19:13 F1 reproduced (scratchpad advf/f1.sh, scratch repos, head's script): root folder " a" with Cargo.lock=D beside main's a/Cargo.lock=C, no record: "saved and pushed"; control folder "b": held; " a" not first in byte order (a " 0/x" beside it): held; F1b "\xff/Cargo.lock"=D beside main's U+FFFD twin=C: "saved and pushed". So the reviewer's repro holds.
- 19:13 F1 exposure (by reading toolchain.yml + workspace globs): the escaping path always has a leading-blank or non-UTF-8 name; CI fetches with cargo fetch --locked / npm ci and builds --locked / --offline, so new fetched code needs a ROOT lockfile change (plain ASCII name: always read right, held); a new crate also changes Cargo.lock (members are listed). So the asset (no unreviewed dependency compiled in CI) is not reached in F1's corners.
