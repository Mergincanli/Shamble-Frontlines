# The advisor agent's returned verdict on plan revision 1 (f535752), as it came back to the main session

Source: the advisor agent's final report to this session (effort xhigh), 2026-11-10, copied here by the main session so
the plan's quotes of it can be checked against a file (the fold check's finding: the verdict was saved nowhere). Its
working notes are `advisor-plan-rev1.md` (this folder). Text as returned, unchanged.

```text
**Verdict: APPROVE** for plan revision 1 at f5357520329a1ce76b7bf3ec3a873ea86f378cbc, with the conditions below. I checked first that `git show f535752:.checkpoints/foundation-toolchain/plan.md` matches the working copy: no difference.

The plan answers all four of Mustafa's asks, and its substance holds. Every change needed is small: a fix to the wording of a question for him, two corrections to the build steps, and some tightening of the CI job. No new plan review is needed if you fold conditions 1 to 5 in as written. Condition 1 must go into the plan before FDN-NOCODE is sent to Mustafa.

Risk tier 2 is right (a new `.github/` workflow, `CLAUDE.md`, maybe `advisor.md`), and so are the reviewers: security-auditor, infra-engineer, rust-reviewer, test-engineer and engine-architect. No Brave check is needed. It merges on his go from a day session, after 2.1b.

**What I checked myself**
- **The guard's marks:** I ran main's two guard tests on 35 paths with my own script (`.checkpoints/foundation-toolchain/advisor-marks.mjs`). One says whether routines may write a path (`protectedRel`), the other whether the PR Advisor may merge it (`mergeAllowed`). The results match Part 5 exactly.
- **Cited lines all hold:** `CLAUDE.md:20-22`, `:45`, `:51-53`; `advisor.md:107`, `:322-323`; ADR 0002 `:569`, `:1289-1291`, `:1314`, `:1432-1438`; STATUS `:680-690`, `:700-710`; the guard `:28-29`, `:35-48`, `:84-100`, `:1486`. PR #40 at 411eeee: its `rust-toolchain.toml` has the plan's four keys, the `check.mjs` line ranges are right, and the spike has its own Cargo workspace and no `package.json`.
- **Version claims, against the npm registry today:** TypeScript's latest is 7.0.2 and the 6.0 line ends at 6.0.3; Vitest 5.1.0 needs Node "^22.12.0 || ^24.0.0 || >=26.0.0"; @playwright/test 1.65.0 needs Node ">=20". All as the plan says.
- **rustup, run locally:** `rustup toolchain install --help` says it installs "by default the active toolchain", so the no-argument install from `rust-toolchain.toml` works as planned.
- **The tools behind the network cut:** in an Ubuntu 24.04.5 container (the runner's OS), `setpriv` and `unshare` ship in util-linux 2.39.3, a package every install has. `setpriv` takes a group name for `--regid` (`<gid|group>`), so `nogroup` works.
- **The network research:** every claim the plan rests on is quoted with its web address. Two are inferences (a package without a `test` script fails; a new namespace holds only `lo`), and the plan's L3 and E2 prove them. Assumptions A1 to A3 are proved by the build.

**Conditions**
1. **FDN-NOCODE's text** (problems F-1 and F-2, Medium):
   - It must cover three lines, not two: `CLAUDE.md:52` ("Phase 0 writes no v2 code" plus the exception) is the same kind of edit as `:20-22` and `advisor.md:107`. Split K2 in two: K2a, the heading scoped to `legacy/`, stays covered by "the lines that scope the legacy constraints"; K2b, the `:52` exception, moves into FDN-NOCODE.
   - It must also quote the SCOPE option (a) text he picked (`git show d389d91:.checkpoints/foundation/plan.md`, lines 524-536): "...and 2.2's kit edit scopes the constraints and adds your 2026-11-10 decision as the exception to the "no v2 code" lines". Say too that his own reply does not repeat that clause.
   - It should open in plain words: "your freeze exception for the foundation (FREEZE-FDN)".
   - It should say the answer is needed before step 5 only, not for the build.
2. **Step 5's "merge main"** (F-8, Medium): CLAUDE.md's Never list counts `merge` as switching the main folder; only `git merge --ff-only origin/main` is allowed there. Do a non-fast-forward merge in a worktree under `.gate/` and push from there.
3. **Concurrency and the planted-failure runs** (F-9, Medium): state `cancel-in-progress` for the new workflow. Push each planted failure (M1a, M1b, M2, M3) alone and wait until its run is red at the expected step. Only then push its `git revert`. If the revert is pushed early, the red run can be cancelled and the evidence lost.
4. **security-auditor's brief** must name the dependency review: the lockfile's package count, the packages with install scripts, and every `resolved` address pointing at the npm registry. CLAUDE.md's Never line on running unreviewed code from the internet is met the way `tools/` meets it today: a pinned lockfile plus this review.
5. **Before K2 is written in step 5**, check that ADR 0002's Status line on main records part 1 as accepted (2.1b's docs pass). `advisor.md:323` forbids merging a hard-constraint change before its accepted ADR is on main.
6. **Low items.** Fold these in, or record them as follow-ups; none blocks:
   - F-3: judge the probe inside the cut on a TCP connection; print the DNS lookup (`getent`) result but don't judge on it. A local DNS service can answer through a socket file that the namespace does not cut, which would turn the job red for nothing.
   - F-5: make the job check that `rustup show active-toolchain` names `rust-toolchain.toml`, and print `rustc --version` inside the cut too. The image's default Rust equals the pin, so the version check alone can't show where the toolchain came from.
   - F-10: bring `lo` up as root inside the namespace before dropping privileges. The probe is unchanged, and 2.5's local server will need loopback.
   - F-4 (optional): the control step depends on crates.io being up.
   - F-6: add a Part 5 row for CLAUDE.md's Layout lines (`:41-43`): either a K5 edit under "paths in CLAUDE.md", or "none: REPO_MAP carries it".
   - F-7: Part 8 should say each agent runs at "its file's effort"; engine-architect's file says `xhigh`, not `high`.
   - F-11: local cargo runs should use `--target-dir .gate/foundation-toolchain/target` and clean it afterwards (CHECKPOINTS §10). CI keeps `/target/`.
   - In plan 1.3, add `engine-architect.md:52` and `graphics-engineer.md:41`. They only bind those agents' own writing, but brief both on his decision since they review 2.2.
   - The report should say that `V2_PHASE0_BRIEF.md:20-21` and `:144` stay stale; that file belongs to the claude.ai Advisor.

**Rulings**
- **Q2.2-a (G0's exit-2 mark):** (a) is final. G0 is a stage of the gate v2, and PM-0002-39 already runs it under `--prove`. 2.2 keeps the pin check and M2, which fail before any cargo call. The register entry is a decision doc, so it needs his go.
- **Q2.2-c (Node):** (a), Node 22, is final. Add a dated STATUS follow-up: one change-set that moves `tools/` (gate files, so it asks Mustafa) and v2 to Node 26 together, after 2.3 and merged before 2027-03-31. Node 22's end of life is 2027-04-30.
- **The three changes from the sketch:** all approved.
  - Lint tools move to 2.3: no check uses them in 2.2, and leaving them out means fewer packages to trust.
  - `.gitignore` takes two lines: `/target/` (anchored) and `test-results/` (unanchored) are right.
  - The job gets its own file: `gate.yml` stays byte-identical, as he asked, and 2.3 replaces the new file.
- **Planted-failure commits on the change-set's own branch:** approved, with condition 3. Also: reverts are new commits (never force-push or reset), none of them carries `[skip ci]`, none touches a protected file, the final diff against main contains none of them, and the report lists each run's id and failing step.
- **Plan 1.3:** holds. No rule forbids 2.2 beyond what his decision, SCOPE (a) and FREEZE-FDN cover. The brief's own `:22` already names "Phase 1: the repository restructure, toolchain, CI gate v2".
- **Part 5:** complete on the guard's marks, which I reproduced. Add the K2 split (condition 1) and the Layout row (F-6).
- **FDN-NOCODE:** needed. His prompt asks him about each kit edit, and his FREEZE-FDN words don't name these lines. Fix it per condition 1.
- **The rules' globs left to 2.5:** justified. After 2.1b they match only `legacy/` files, and 2.2 adds no page and no `shamble-*.js`. The FREEZE-FDN option (a) he picked listed the globs, so 2.5 can still ask.
- **The workflow draft (4.2):** holds up:
  - Permissions: only `contents: read`.
  - Pins: the same action SHAs and `ubuntu-24.04` as `gate.yml`, `persist-credentials: false`.
  - No secrets, no caches, no `pull_request_target`.
  - The cut: `sudo`, `unshare`, then `setpriv` run in the right order.
  - The probe and its control: a real control, the same check outside the cut and inside it.
  - Failure modes: M2 fails at the pin check's `sed` before any cargo call; M3 fails at `cargo fetch --locked`.

  It still needs conditions 3 and 6 (F-3, F-5, F-10).
```

(The report went on with its notes' paths, a "Decisions needed" entry for FDN-NOCODE rewritten per condition 1, and a
"For Mustafa" list; both are carried into the plan's own last sections.)
