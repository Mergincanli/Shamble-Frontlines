# Advisor: MAIN-70 plan check (main dae6d107, #79 cs4b-2 "The world", into PR #40's branch)
2026-11-28, day session. Advisor agent, read only: no cargo, no git write, one git per call, git first, no `git -C`.
I act by main's advisor.md (plan review list). Form: MAIN-68 rev 4 as checked (advisor-main-68b.md) and MAIN-69 as
checked (advisor-main-69.md, M69-1 to M69-6). Printing forms only (ADV-M69-0: `--quiet` shows nothing in this tool).

## Plan of this check
0. The plan: main-70-plan.md rev 0.1, sha256 ecac1d86...bdf (105 lines), against rev 4, MAIN-69 and M69-1..6.
1. The facts by git (no fetch): first-parent b8f02b0..dae6d107; #79's paths, modes, counts; kit/CI/spikes/design/mark
   pathspecs; DEP-HOLD (root manifests on the branch; Cargo.lock's registry adds); the trial merge and STATUS.md block;
   main's CI on dae6d107; refs.
2. Q1-Q3, the verdict, conditions, findings, Decisions needed, For Mustafa, Status line.

## Block 0. The plan
- Read whole at 02:06 UTC; sha256 ecac1d863fc09dc9da4286993bf93698d1d320e633ea3d165b84721304ec9bdf, 105 lines.
- HEAD now 2249d072 (an autosave on 4c936c3b, on the plan's c333b75); origin/main dae6d107fdbe942567104bcf1c9a2cbc6b3a1e0f.

## Block 1. The facts by git (02:06-02:2x UTC; no fetch; origin/main dae6d107 = `git ls-remote origin refs/heads/main`)
- **The merge.** `git log --oneline --first-parent b8f02b0..origin/main`: one line, dae6d107, #79. Parents b8f02b01...
  and b7145273fdaff2b2ee6eccafaf5e358840bc3572. Its message: "Merged at the expected head b7145273... on Mustafa's go for
  that head, given in the Builder V6 chat ... The advisor agent approved that head; gate, smoke and toolchain were
  green on it". Main's record (lane 1), not his word here. `git merge-base HEAD origin/main`: b8f02b0149cc... Holds.
- **What #79 changes.** `--raw --no-renames`: 107 paths, A 44, M 63, every new mode 100644 (no 120000, 160000, mode
  change or D). `--shortstat`: "107 files changed, 10839 insertions(+), 408 deletions(-)". Holds.
  - By area (my tally): crates/devtools 28, packages/client 15, packages/types 13, packages/sim-worker 13, crates/sim 8,
    crates/content 8, docs 7, crates/sim-wasm 4, packages/e2e 4, content/source 3, **content/maps 1, content/movement 1,
    Cargo.toml 1, Cargo.lock 1**. The plan's list sums to 103: it leaves out the last four (ADV-M70-1, Low).
- **Kit, CI, gate, spikes, design.** `git diff --name-only b8f02b0 origin/main --` CLAUDE.md .claude .github tools
  AGENTS.md CLAUDE.local.md .gitignore spikes docs/design docs/adr and the `**/CLAUDE.md`, `**/AGENTS.md`,
  `**/.claude/**`, `**/.npmrc` globs: prints only docs/adr/README.md (its own control). So no kit, CI, gate, spikes or
  design path. Mark rows (`-G'^[|] PM-'` on docs/): nothing.
  - docs/adr/README.md 7/4 (the register, a decision doc): #79's docs pass adds pointers to his VISION-RULE addition
    (DECISIONS.md entry of 2026-11-27 13:57:21 UTC); no `| PM-` row. Main's content byte for byte, as MAIN-68 took #75's
    8/6. On record; no lane edit.
  - Docs numstat: DECISIONS 105/0, REPO_MAP 49/22, ROADMAP 47/28, STATUS 15/15, STATUS_ARCHIVE 87/0, adr/README 7/4,
    reports/2026-11-27-cs4b2-world.md 254/0.
- **The kit both ways (the guard's merge rule).** `origin/main...HEAD` over the kit pathspec (with the globs): nothing;
  `HEAD...origin/main` the same with docs/STATUS.md added as control: prints only docs/STATUS.md. `git diff --name-only
  origin/main HEAD -- .claude CLAUDE.md .github tools AGENTS.md CLAUDE.local.md .gitignore` (+ docs/STATUS.md as
  control): only docs/STATUS.md. So `git merge origin/main` passes main's rule as at MAIN-69, and after it HEAD's kit
  still equals main's.
- **DEP-HOLD.** Main changes six dependency files: Cargo.toml, Cargo.lock, crates/{content,devtools,sim-wasm,sim}/
  Cargo.toml (the `**/Cargo.toml`, `**/package*.json`, `**/Cargo.lock`, toolchain and `.cargo` globs print exactly
  these). No package.json or package-lock.json moves.
  - The branch changes none: `git diff --name-only b8f02b0 HEAD -- Cargo.toml Cargo.lock crates packages package.json
    package-lock.json content` prints nothing; with docs/STATUS.md added it prints docs/STATUS.md (control).
  - Cargo.lock: 33 `+source = "registry+https://github.com/rust-lang/crates.io-index"` lines, no other source, one `-`
    line (the header). The tree is parry3d =0.31.1 (root Cargo.toml, `enhanced-determinism`, "the first outside crate
    inside the simulation (ADR 0004 part 10)") and its dependencies, plus ts-rs as a dev-dependency of content.
  - Reviewed in #79's lane (its report, docs/reports/2026-11-27-cs4b2-world.md on main): ":29-30 the remade batch
    resolved parry3d 0.31.1, safe_arch 1.2.0 and thiserror 2.0.21 ... (security-auditor approved its five digests)";
    :125 security-auditor's design reads "(the batch, the advisories, the test list)"; :161 round 1 PASS with three
    Lows; :219 follow-up 4B-HEAPLESS (security-auditor's dependency read); merged on his go.
  - The exception's code: checkpoint.mjs:23-24 (the comment) and :135 (`if (inPush && inPush ===
    blobAt('refs/remotes/origin/main')) return false;`), read at HEAD. Holds.
  - The spike stays out: spikes/netcode-proof/Cargo.toml:4 `[workspace]` (own workspace); no `path = "../../` in any
    spike manifest (the grep prints nothing; its form printed on :4/:6); root Cargo.toml:3 `members = ["crates/*"]`. So
    Stop B's dist (npt_client_wasm) does not build #79's crates.
- **The trial merge** (HEAD 2249d072, an autosave on the plan's c333b75): `git merge-tree --write-tree --name-only HEAD
  origin/main`: exit 1, tree fe1b72207934e047b2382a08c21796a3dc8d08f8 (the plan's 54c70bb0 was on c333b75; not bound),
  the conflicted-file line docs/STATUS.md, one CONFLICT line "CONFLICT (content): Merge conflict in docs/STATUS.md",
  auto-merging DECISIONS.md, REPO_MAP.md, STATUS.md, STATUS_ARCHIVE.md. Holds.
  - Markers (`git grep -n` over STATUS, STATUS_ARCHIVE, DECISIONS, REPO_MAP in the tree): only STATUS.md :33, :38, :41.
  - Ours :34-:37: this branch's #40 row, MAIN-WALL "none yet", "#75 (draft)", "#78 (draft)". Theirs :39-:40: main's #40
    row (2026-11-14 text) and "none yet | change-set 4b-2, The world". Holds.
  - `git diff --numstat b8f02b0 HEAD -- docs/STATUS.md`: "1 1", and `-U0` shows the two #40 rows. `git diff --name-only
    -G'^[|] #40 [|]' b8f02b0 origin/main -- docs/STATUS.md`: nothing (main's #40 row unchanged). Main's STATUS.md: 120
    lines; b8f02b0..main 15/15. Holds. So the rule's result is main's file with the #40 row swapped: 120 lines.
  - Trial tree vs main outside spikes/, .checkpoints/, docs/: nothing. docs/ vs main: the usual six names (STATUS 7/0 =
    the conflict block's extra lines). vs HEAD in spikes/, docs/design/, .checkpoints/: nothing.
  - `git diff --shortstat HEAD <tree> -- . ':(exclude)docs/STATUS.md'` = `git diff --shortstat b8f02b0 origin/main --
    . ':(exclude)docs/STATUS.md'` = "106 files changed, 10824 insertions(+), 393 deletions(-)". So step 5's stat bar
    should hold.
- **Main's CI on dae6d107** (gh api, read only): push runs on main, attempt 1, success: gate 46081134384 (23:12:08-
  23:12:51; jobs gate 140780126105, smoke 140780126466) and toolchain 46081134408 (job 140780125932, 23:12:11-23:22:00,
  9 min 49 s of 25). Holds. Two more runs on the same SHA are lane 1's branch push (46081292593/95), not main's.
- **Refs.** `git grep -a -l "spikes/"` outside spikes/, docs/, .checkpoints/, `*.md`: kept-logs.mjs and its test, on
  b8f02b0 and on main alike. The refs step counts `git ls-files` (check.mjs:2729-2735 at HEAD, not :2695-2700: the file
  grew since MAIN-69; ADV-M70-2, Low). So refs = N0 + 44. Holds.
- **Untracked and room.** `git ls-files --others --exclude-standard` under crates, packages, content, docs, .claude,
  .github and the root manifests: nothing. `git status --short`: only this file. Free disk 4,944 MiB (`df -m`, 02:2x);
  the light runs' bar is 4,096 (main-66-plan.md:120-121, as MAIN-68 read it). No cargo, rustc, build.mjs or check.mjs
  process runs (`pgrep -a -f`). Worktrees: .gate/stopb-a and stopb-b at Y 652e9513, **and .gate/stopb-c at Y0
  87cb8677**, which the plan's "Order" does not name (ADV-M70-3, Low).
- **"Why".** history.md:2055-2057 holds his words (MAIN-52); :8168 is TAKEIN-CODE (b); light-takein-recipe.md:9-10 the
  light recipe's limit. Holds.
