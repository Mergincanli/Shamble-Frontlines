# infra-engineer, review round 1 of kit-hold-ci (PR #60), file .github/workflows/toolchain.yml only

Head reviewed: branch claude/wonderful-hamilton-jdmlnf; the workflow edit is commit 94c197f (12 added lines), base main 9b4b518.
Memory: .claude/agent-memory-local/infra-engineer/ does not exist (nothing to recall).

## Plan
1. Q1 byte check: extract blocks 1 to 3 of "The exact texts asked for" from `git show 1bfe462:docs/reports/2026-11-18-cs1-types.md`
   (and main's copy), compare byte for byte with the added lines of 94c197f (script, not by eye), and check placement.
2. Q2 cache-mode: read GitHub's docs page for `jobs.<job_id>.cache-mode` (docs.github.com), note date; check every step of the
   job for a cache restore/save (setup-node default package-manager-cache, rust toolchain, anything else); what the log line
   proves and does not; cost/side effects.
3. Q3 git inside the cut: read the whole job (user, env -i, HOME, nogroup), reason about safe.directory/ownership/index
   refresh, try locally on a scratch copy under .gate/ with a different uid if possible; set -euo pipefail behaviour of
   `changed=$(...)` command substitution failure.
4. Q4 compare lines: test on a scratch git repo which changes `git status --porcelain --untracked-files=all -- <path>` reports
   (content, mode, untracked, deleted, new subfolder, ignored file, core.fileMode=false, assume-unchanged), and what the
   generator writes (does it delete stale files?). Judge the plant.
5. Q5 tsc lines: word splitting / globbing of $strict $shipped $tests; empty list guard under set -e; `--ignoreConfig` in
   TypeScript 6.0 docs (typescriptlang.org); can both runs check nothing.
6. Q6 regressions: diff the job against main for the cut, permissions, order, --locked/--offline, timeout.
7. Q7 two lines each for the gate v2 list.
8. Read ci-evidence.md, plan.md "Evidence", resume card steps 12-17, Mustafa's words files.

## Findings (each block written as found)
