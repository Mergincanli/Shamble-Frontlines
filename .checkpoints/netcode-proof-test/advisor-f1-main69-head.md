# Advisor: the head reads of F1 (c16777e, pushed in 9fbc4b4) and MAIN-69 (merge 8ec5dc07, pushed in d9fb0dd), 2026-11-27

Plan: main's advisor.md (changed since c9bd268?). Block 1, F1: the commit's parents, files, numstat and diff against
my advisor-ops-reword-head.md block 2; operator-rewordings.md's last section; CI on 9fbc4b4. Block 2, MAIN-69:
main-69-plan.md rev 0.2 and advisor-main-69.md (M69-1 to M69-6); the merge's parents and what it brings; the
whole-tree bars; the toolchain job's lines against main's job 140547396571 (and 140114848934); #74's two pull-request
steps and the preview-site artifact in run 46018454213; the run times. Then Decisions needed, For Mustafa, the status
line. Printing forms only, quoted.

- **advisor.md:** main's last change is still c9bd268 (2026-11-12).
- **Heads, printed:** origin/main b8f02b0149cc…; origin's branch 06a2b982…; local HEAD 3ffe76e6…. The one local commit
  above origin changes only .checkpoints/netcode-proof-test.md (`git diff --name-only 06a2b98 3ffe76e`), a notes save
  not yet pushed.

## 1. F1 (c16777e): it stands; nothing owed
- **The commit:** c16777e629a6…, parent 9696897b…, and 9fbc4b42…, parent c16777e…. Its subject names the new rule.
- **numstat:** "3 3 spikes/netcode-proof/ops/operator/PROMPT-R1.md", one file, net zero.
- **Its diff** is my block 2's sentence word for word: "Unless its saved last line reads `… checked, 0 finding(s)`, no
  push (an empty file included): stop and tell him." The rest of :127-129 is rewrapped with no word changed, and
  :124-126 are untouched (`git diff c16777e^ c16777e`).
- **The proof is in the done lines** (operator-rewordings.md, last section):
  - Q0: 21 of 21 and 464 items, 0 findings.
  - Q1: 21 of 21; redact 464 items, 0 findings; numstat 3/3 only; GATE PASS; repo map OK; the three lines printed with
    their lengths.
  - Q2: `git diff --cached --name-only` printed only PROMPT-R1.md; the saver was stopped first, with `pgrep -c`
    printing 0.
  - The two r1plan pins (the hooks span on one line, :125's "no VM list") still hold. I re-ran r1plan in the previous
    read, and :125 and the hooks span are untouched in this diff.
  - All in printing forms.
- **CI on 9fbc4b4** (`gh run view`, each printed): gate push 46016904132, toolchain push 46016904135, gate
  pull_request 46016913053 and toolchain pull_request 46016913183. Each reads conclusion "success", attempt 1, headSha
  9fbc4b42….
- **Since F1, outside notes:** only MAIN-69's six paths, as `git diff --name-only c16777e 3ffe76e` prints. They are read
  in block 2.

## 2. MAIN-69 (merge 8ec5dc07): APPROVE
- **Parents:** `git log -1 --format='%H %P'` gives 8ec5dc077b32… with parents c7950799… (H0) and b8f02b01… (main), and
  d9fb0dda…, parent 8ec5dc07….
- **The merge brings only main's six paths:**
  - `git diff --name-only H0 8ec5dc07` prints .github/workflows/toolchain.yml, docs/DECISIONS.md, docs/REPO_MAP.md,
    docs/ROADMAP.md, docs/STATUS.md and docs/reports/2026-11-27-ci-time.md.
  - `--stat` reads "6 files changed, 134 insertions(+), 4 deletions(-)", equal to `git diff --stat b9aad01 b8f02b0`'s
    line.
- **The anchor:** `git rev-parse 8ec5dc07^{tree}` printed 85fcce48e0d7…, the plan's T from step 1's `merge-tree`.
- **M69-1, the whole-tree bars, at the pushed head d9fb0dd:**
  - outside spikes/, .checkpoints/ and docs/ against main: printed nothing;
  - docs/ against main: printed the usual six (DECISIONS.md, REPO_MAP.md, STATUS.md, STATUS_ARCHIVE.md,
    design/netcode-proof-messages.md, design/ws-reconnect.md);
  - `H0 8ec5dc07 -- spikes/ docs/design/ .checkpoints/`: printed nothing.
  - The controls in the same call printed six names, so the empty results are real.
  - PR #40's diff against main holds no .github/ path: it stays tier 1.
- **M69-5:** the manifests form (Cargo.lock, Cargo.toml, crates, packages, package.json, package-lock.json) against
  main printed nothing. `.github/ .claude/ CLAUDE.md tools/` against main also printed nothing.
- **What d9fb0dd adds over the merge:** six notes paths only, all under .checkpoints/.
- **The done lines** (main-69-plan.md, steps 1-6) agree with these reads:
  - usage 0.89 and 0.90, go;
  - the saver stopped, `pgrep -c` printing 0;
  - git's "Merge made by the 'ort' strategy.";
  - GATE PASS; "repo map OK: 246 entries" (main's new report entry);
  - checkpoint tests 33 of 33;
  - the light runs: refs 2,163 then 2,164 = N0 + 1 (the new report); marks 81; self 80; redact 464 items, 0 findings.
  - M69-1 to M69-6 are applied as written (the plan's "As checked" section).
- **CI on d9fb0dd** (`gh run view`, each printed; all attempt 1, headSha d9fb0dda…, success):
  - gate push 46018443843 (14:43:17-14:44:47);
  - toolchain push 46018443909 (14:43:17-14:51:54);
  - gate pull_request 46018453424 (14:43:21-14:44:36);
  - toolchain pull_request 46018454213 (14:43:21-14:51:10).
- **M69-2, step 7's baseline, compared line for line.** I took M67-3's lines from each job's log: every `test result:`
  line, every vitest `Test Files` and `Tests` line, and every Playwright "passed" line. I removed only the timings
  (cargo's "finished in …s" and Playwright's "(3.4m)").
  - The four jobs hash equal, ccd0639fdd47…: main's 140547396571, PR #40's pull-request job 140571570351, its push
    job 140571535294, and MAIN-68's 140114848934.
  - Each has 103 `test result:` lines, all "ok." (`grep -c 'test result: ok\.'` printed 103 on the pull-request job),
    552 passed in sum.
  - The vitest pairs are 12/95, 1/1, 8/51, 2/7 and 5/33. The Playwright lines are "1 passed", "18 passed" and "4
    passed".
  - The only line that differs is Playwright's run time: 3.4 m on main and on the pull-request job, 3.5 m on the push
    job, 3.8 m on MAIN-68's.
- **#74's two pull-request steps,** by rev 4's step-6 bar:
  - On the pull-request run 46018454213, steps 11 ("Check that the tested build holds only plain files and folders")
    and 12 ("Hand the tested build to the preview workflow") both succeeded.
  - The check printed "5 files, 892209 bytes" (14:51:04).
  - The artifact listing shows "preview-site 250469 expired=false 2026-12-04T14:51:04Z".
  - On the push run 46018443909 both steps were skipped, and its artifact count is 0.
  - Main's workflows are gate.yml and toolchain.yml only (`git ls-tree origin/main .github/workflows/`), so no preview
    starts.
- **The time limit:** toolchain.yml:38 reads `timeout-minutes: 25` at d9fb0dd and on main.
  - The toolchain jobs took 7 min 45 s on the pull request (140571570351, 14:43:24-14:51:09) and 8 min 33 s on the push
    (140571535294, 14:43:20-14:51:53), against 25 minutes.
  - Main's own job took 7 min 30 s.
- **Owed:** nothing. The one record line, "main b8f02b0 taken in (#78 ci-time: the toolchain job's time limit 15 to 25
  minutes, and docs), plan first (MAIN-69)", goes to him as main-69-plan.md says, in the chat and the day's message.

## Decisions needed
None.

## For Mustafa
1. Nothing to do.
2. For the day's message: MAIN-69's record line (main b8f02b0 taken in), and F1's reworded push rule, which he sees at
   Stop B.

Status: COMPLETE 2026-11-27T15:58Z
