# MAIN-68: the values for main b9aad01 (derived 2026-11-26 19:5x-20:0x UTC by the main session, read only)
For C's verdict (advisor-main-68b.md) and rev 4. Nothing here is bound until the verdict says how it applies. Rev 3
(main-68-plan.md, on 2671595, sha256 a6ff0bc4…) is unchanged; these lines replace its values one for one.

## Main's tip and the two new merges
- `git ls-remote origin refs/heads/main` (19:52) and `git fetch origin main` (19:5x): origin/main
  b9aad01954be666340a7ec5c6e9fc9353b11abf1. First-parent 07d673b..b9aad01: #70, #73, #74, #76, #77, #75. Merge base
  07d673b.
- #77 (47bb79e9f5ecd54c32d6cf82131b9aebf5421ec3, 18:52:17 UTC), docs/TECHNICAL.md and the main-wall report only. Its
  message, word for word: "main-wall: T10's day-session half recorded in the report and TECHNICAL (docs only, risk tier
  0). Merged by the PR Advisor routine after its APPROVE of head 4e8a87e8b527f4762d22893fa3b379f85d71825a (class ready:
  no human check needed), with gate and smoke green on that SHA. The PR Advisor's first merge under main's wall (T10's
  PR Advisor half)."
- #75 (b9aad01, 18:53:13 UTC), code. Its message, word for word: "cs4b-1: Content — the game's own content loader,
  proven before any value rests on it". It records no go or approval. What main records:
  - STATUS.md:36 (main's text, written before the merge): "risk tier 1, no browser check; reviewed (two rounds, no
    Critical or High); the verifier and the advisor agent's verdict on the final head are in its report | his go for the
    head the advisor agent approves".
  - STATUS.md:49: "4b-1 (#75), reviewed (its report): his go."
  - Its report (docs/reports/2026-11-26-cs4b1-content.md:57-58): "It waits for the advisor agent's verdict on the final
    head and for his go." The report's :421 asks "Your go for the merge of pull request #75, for the head the advisor
    agent approved".
  - Under the wall every session acts as his account, so the merge itself does not show whose go it was. This take-in
    brings in main's reviewed text byte for byte and needs no reading of that go.

## The counts (07d673b..b9aad01)
- `git diff --name-status`: 87 paths, A 60, M 27. `--no-renames --diff-filter=DRCTUXB`: nothing. `--raw` (2671595..
  b9aad01): every file 100644, the added ones from 000000.
- Whole: "87 files changed, 51544 insertions(+), 271 deletions(-)". Without docs/STATUS.md: "86 files changed, 51498
  insertions(+), 225 deletions(-)". Without both STATUS files: "85 files changed, 50926 insertions(+), 223 deletions(-)".
- Docs and lock numstat: Cargo.lock 5/2; DECISIONS 745/0; REPO_MAP 58/20; ROADMAP 132/63; STATUS 46/46; STATUS_ARCHIVE
  572/2; TECHNICAL 9/0; UNITS 10/0; adr/README.md 8/6; reports cs4a-recording 355/0, ci-preview-handover 126/0,
  cs4b1-content 427/0, guard-wrappers 89/0, main-wall 84/0. Main's STATUS.md: 120 lines.
- The 60 added: rev 3's 24, plus #75's 36 (crates/content: Cargo.toml and 10 sources; devtools: src/bin/time_report.rs,
  src/content_proofs.rs, src/time_report.rs, tests aim_reach, content_fuzz, content_proofs, dependency_rules, time_unit,
  vectors/ seven files; crates/sim/src/time.rs; docs/reports/2026-11-26-cs4b1-content.md; packages/client/scripts/ four
  files; packages/e2e/tests/rules/content.spec.ts; packages/sim-worker/boundary content.test.mjs, plain-file.test.mjs,
  plain-file.ts).
- Kit and CI: 2671595..b9aad01 changes nothing under .claude/, .github/, tools/, spikes/ or docs/design/, and no
  CLAUDE.md, CLAUDE.local.md, AGENTS.md or .gitignore. The guard's pathspec both ways: `origin/main...HEAD` nothing;
  `HEAD...origin/main` the same three (#73's two, #74's toolchain.yml).
- No `| PM-` row moves under docs/ (`-G'^[|] PM-'` over 2671595..b9aad01: nothing).

## Dependencies (DEP-HOLD: the ruling is C's)
- Cargo.lock (2671595..b9aad01): adds `[[package]] name = "content" version = "0.0.0"` (no source and no checksum line:
  a workspace crate), and devtools' dependency list gains "content" and loses "client-endpoint" and "room". No external
  package is added or removed.
- crates/content/Cargo.toml (new, 13 lines); crates/devtools/Cargo.toml 3/2. No package.json or package-lock.json.
- After the merge, the root Cargo.lock and every root manifest equal main's byte for byte; spikes/netcode-proof/Cargo.lock
  and the spike's manifests are untouched.

## Main's CI on b9aad01 (push runs, attempt 1, all success)
- gate run 45879443239: gate job 140114837556 (18:53:34-18:53:54 UTC), smoke job 140114837338 (18:53:35-18:54:49).
- toolchain run 45879443258: job 140114848934 (18:53:35-19:03:18; build-and-test 18:53:56-19:03:16; #74's two
  pull-request steps skipped). Step 7's baseline in M67-3's form.
- #77's runs on 47bb79e (gate 45879322864, toolchain 45879322888) were cancelled by #75's push a minute later.

## The trial merge (HEAD 07cb835)
- `git merge-tree --write-tree --name-only HEAD origin/main`: exit 1, tree 6497e29d6f32d96393b847bd184ad7b80a9cb3ea; two
  CONFLICT lines (docs/STATUS.md, docs/STATUS_ARCHIVE.md); auto-merging DECISIONS.md, REPO_MAP.md, STATUS.md and
  STATUS_ARCHIVE.md.
- STATUS.md's block, markers :34, :37, :41. Ours: :35 (#40, this branch's 2026-11-15 text) and :36 (ci-after-3a's row).
  Theirs: :38 (main's #40, its 2026-11-14 text), :39 (MAIN-WALL's "none yet" row) and :40 (#75's "(draft)" row, main's
  text as it is). By rev 3's rule: keep :35, :39, :40 in that order; drop :36 and :38; `wc -l` 120.
- STATUS_ARCHIVE.md's block, markers :2385, :2395, :2421, right under "## Open bugs and risks" (:2384). Ours: :2386-2394,
  the netcode proof test bullet (9 lines). Theirs: :2396-2409, the MAIN-WALL bullet (14 lines), then :2410-2420, the
  guard-wrappers bullet (11 lines). The CI smoke bullet follows at :2422. Keep ours, then theirs; drop only the markers.
- Measured on the trial tree: `git diff --shortstat HEAD 6497e29d… -- . ':(exclude)docs/STATUS.md'
  ':(exclude)docs/STATUS_ARCHIVE.md'` prints "85 files changed, 50926 insertions(+), 223 deletions(-)" (main's own), and
  its numstat gives DECISIONS 745/0 and REPO_MAP 58/20 (main's own). Against main outside spikes/, .checkpoints/ and
  docs/: nothing.
- refs: `git grep -a -l "spikes/" origin/main` outside spikes/, docs/, .checkpoints/ and *.md: kept-logs.mjs and its test
  only. Untracked in main's paths: nothing (18:5x; not re-read for #75's new folders: crates/content and the vectors).

## Rev 3's values, one for one
- C4: origin/main b9aad01954be666340a7ec5c6e9fc9353b11abf1; CI as above.
- C5 (a): the SHA b9aad01954be666340a7ec5c6e9fc9353b11abf1 and "HEAD^2 is b9aad01".
- Step 2: main's other 85 paths staged (A 60, M 25). Step 3: STATUS.md exactly 120 lines; the numstat "1 1"; the archive
  "18 5" (H0's own). Step 4: parents H0 and b9aad01.
- Step 5d: refs = N0 + 60. Step 5e: 1,551 expected (no `.claude/` change since #73).
- C1 (c): 87 names; the 85-file line without both STATUS files; the whole line expected at "87 files changed, 51544
  insertions(+), 271 deletions(-)"; docs/ against main the usual six names.
- C2 (b): the 85-file line. M65-2's fallbacks: the 60 added; the staged tally 87 paths (A 60, M 27); STATUS.md's 120.
- Step 6: toolchain on PR #40 now also runs #75's tests (the content crate's, devtools' content_proofs, content_fuzz,
  dependency_rules, time_unit and aim_reach, the e2e content spec, sim-worker's content and plain-file boundary tests).
- Step 7's baseline: run 45879443258, job 140114848934.
- Free disk 7,705 MiB at 19:52 (the light runs need 4,096).
