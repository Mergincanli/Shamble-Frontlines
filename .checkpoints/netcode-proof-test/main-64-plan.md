# MAIN-64: take main 1ec6f6c into the netcode branch (plan, 2026-11-24 10:3x UTC)

## Why
MAIN-52, Mustafa's words (history.md:2055-2057; TEST-FIRST's "main taken in when it changes code, CI or rules"): main
moved from f5030a3 (MAIN-61's) to 1ec6f6c, PR #64 (CI-WASM, merged 10:15 UTC). It changes CI
(`.github/workflows/toolchain.yml`, +97), code in packages/client and packages/e2e, `.gitignore` and docs. GitHub has
reported PR #40 as un-mergeable since then (the merge-conflict event of about 10:16 UTC). So no CI runs on #40 until
this merge is in, and U6c's merge waits on it (the M2 re-check). Plan first, in MAIN-61's approved form: main-61-plan.md's
steps 1-7, with its conditions C1-C6 (advisor-main-61.md:19-104) kept whole. Only the facts and numbers below differ.

## Facts checked (10:2x UTC, read only)
- origin/main is 1ec6f6c3852d2922989f299bfc053c403d0e52d3. The merge base is f5030a33185b1ea1b15329174e82823f0215f993
  (MAIN-61's second parent). HEAD equals the remote branch: a4186ca7e643ae18d13aac9b2ebe208a46965540 when this plan was
  written. H0 is read at step 1, after this plan's own save.
- Main's CI on 1ec6f6c (push): gate run 45373057216 and toolchain run 45373056953, both completed with success
  (10:15:35-10:17:41 UTC).
- `git diff --name-status f5030a3 origin/main`: 19 paths, +2,174 / -99. 3 added (docs/reports/2026-11-19-ci-wasm.md,
  docs/reports/2026-11-19-cs2-browser-check.md, packages/e2e/node-names.d.ts), 16 modified, none deleted or renamed.
  So the tracked-file count after the merge is N0 + 3.
- Kit and CI: main changes `.github/workflows/toolchain.yml`, and no `.claude/`, CLAUDE.md, AGENTS.md or tools/ file.
  This branch changes none of them: the guard's full pathspec (main-61-plan.md:37-40) prints nothing. So a plain
  `git merge origin/main` in its own call passes the guard.
- `.gitignore` gains `dist-*/`:
  - this branch tracks no path under such a folder (`git ls-files ':(glob)**/dist-*/**'` prints nothing);
  - U7's dist-bites folder is already under .gate/ (u7-t-build-t5i.patch:8, :79);
  - U8's "dist-accept" is a command word, not a path.
- Marks: main changes no docs/adr file, so the 81 rows hash as before.
- refs: `git grep -n -I "spikes/" origin/main -- crates packages Cargo.toml Cargo.lock package.json package-lock.json
  .gitignore .github` prints nothing.
- DEP-HOLD: main changes no Cargo.toml, Cargo.lock, package.json or package-lock.json, so the hold has nothing new to
  hold.
- Main's checkpoint tests: 33, since main has changed no `.claude/` file since #61.
- `git merge-tree --write-tree --name-only HEAD origin/main` gives one conflict, docs/STATUS.md (content).
  DECISIONS.md, REPO_MAP.md and STATUS_ARCHIVE.md merge on their own; both sides changed them since f5030a3.
- The conflict block (merge-tree's tree f4dce49, "In flight"):
  - Ours: this branch's #40 row (its 2026-11-15 text; `git diff f5030a3 HEAD -- docs/STATUS.md` shows it is the only
    line this branch changed), then main's #61 row as f5030a3 had it.
  - Theirs: main's #40 row (its 2026-11-14 text, unchanged), the new #64 row and the new "#63, #62" row.
  - Main took the #61 row out, because #61 merged.
- Of U6c's review seats, security and test are done. The netcode seat still runs, read only, on U6c's worktree. The
  seats write only their notes (u6c-review-*.md) in the main folder. These files, and only
  .checkpoints/netcode-proof-test/ files, may show as unstaged. No step stages them, and the merge touches none of them.

## Steps (main-61-plan.md's steps 1-7, each in its own call; only what differs is written here)
1. As written (C1, C2, C5):
   - the resume line by Edit;
   - the saver b1bd3pcdl stopped by TaskStop, and its end proved;
   - H0 named in the chat;
   - the four pre-checks, with origin/main = 1ec6f6c and base = f5030a3;
   - the base light run into .gate/main-64/light-base.log, which gives N0, M0 and the self count.
2. `git merge origin/main`. Expected: exit 1 and "CONFLICT (content): Merge conflict in docs/STATUS.md", with no other
   CONFLICT line. `git status --short` shows `UU docs/STATUS.md` and main's other 18 paths staged (A 3, M 15).
3. Resolve by Edit:
   - the block becomes this branch's #40 row, then main's #64 row, then its "#63, #62" row, each word for word from its
     side;
   - #61's row stays out, because main took it out and this branch never changed it;
   - no other line changes.
   Then the marker grep and `git add docs/STATUS.md`. Then `wc -l docs/STATUS.md`: at most 120.
4. As written, with H0 and 1ec6f6c as the two parents.
5. a-c as written. Repo map: "repo map OK", with its two numbers recorded. d's bar as written, with refs at N0 + 3.
   Outputs go under .gate/main-64/.
6. As written:
   - main's CI on 1ec6f6c read again;
   - the card save pushes;
   - CI on the pushed head: gate, smoke and toolchain all green (toolchain now runs CI-WASM's steps: the page built,
     the boundary tests, the type checks and the 12 browser tests);
   - the saver restarted with both trailers.
7. The advisor's check of the pushed head, as at MAIN-61.

## Stop rules, risks, record
- main-61-plan.md's stop rules (:106-114) and risks (:116-128) stand, with "builder 4" read as "the netcode seat".
  - Builder 4 wrote and held the lane lock; the netcode seat only reads.
  - Its runner calls (u6c-run.sh) may take the shared lock, so the light runs wait for them, as the runner does.
- New risk (Low): toolchain on PR #40 now runs CI-WASM's steps. This branch changes nothing those steps read: it
  differs from main only in spikes/, .checkpoints/, docs/design/, the #40 row, and its own lines in DECISIONS.md,
  REPO_MAP.md and STATUS_ARCHIVE.md. A red there is main's or the runner's, and the advisor rules before any further
  step.
- The record, under "Choices taken without him": "main 1ec6f6c taken in now, plan first (MAIN-64)", named in today's
  message, as MAIN-61 was.

## Change type, tier, size
- A merge of main: main's reviewed and merged text (19 files, +2,174 / -99; #64 had two review rounds, the verifier and
  the advisor agent: STATUS's #64 row), plus one conflict block resolved by hand. Nothing else is edited by hand.
- Reviewers: the advisor (this plan, then the pushed head). Mustafa: nothing to check in Brave.
