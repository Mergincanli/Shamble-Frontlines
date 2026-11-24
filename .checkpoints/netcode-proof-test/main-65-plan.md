# MAIN-65: take main 28254a7 into the netcode branch (plan, 2026-11-24 22:5x UTC)

## As approved
The advisor's check (advisor-main-65.md, ends DONE; 22:56-23:17 UTC): **APPROVE WITH CONDITIONS M65-1 to M65-5** on this
plan's text fd6e8c4c…. Its own words (advisor-main-65.md:16-160: the conditions, the fallbacks of its section 3 and
U6c's terms of its section 4) govern wherever this plan differs. In short: no save from step 2 until step 6's card save
(M65-1); this take-in's numbers and the resume line (M65-2); the card at 200 lines or fewer (M65-3); risk tier 1, `wc -l`
before the add, .gate/main-65/ kept until step 7 (M65-4); U6c's M3-M9 may start once step 6's CI is green (M65-5).

## Why
MAIN-52, Mustafa's words (history.md:2055-2057; TEST-FIRST's "main taken in when it changes code, CI or rules"; LEAN-MODE
item 1 names "main taken in" on R1's path): main moved from 1ec6f6c (MAIN-64's) to 28254a7, PR #65 (change-set 3a,
"rules and boundaries", merged 22:40 UTC on his tap). It changes code (crates/sim, crates/devtools, crates/sim-wasm,
packages/client, packages/sim-worker, packages/e2e), Cargo.lock, package-lock.json, two manifests, `.gitignore` and docs.
GitHub reports PR #40 as un-mergeable since then (the merge-conflict event of 22:41 UTC), so no CI runs on #40 until
this merge is in, and U6c's merge waits on it (U6c's M3 re-runs `git apply --check` on the post-MAIN-65 head). Plan
first, in MAIN-64's approved form: main-64-plan.md's steps 1-7 (which are main-61-plan.md's), with MAIN-64's conditions
M64-1 to M64-4 (advisor-main-64.md:15-48) and MAIN-61's C1-C6 (advisor-main-61.md:19-104) kept whole. Only the facts
and numbers below differ.

## Facts checked (22:4x-22:5x UTC, read only)
- origin/main is 28254a73ab226379373e380eeee64336e58ef916. The merge base is 1ec6f6c3852d2922989f299bfc053c403d0e52d3
  (MAIN-64's second parent). HEAD equals the remote branch: 40a348197e1ef0037b2205fc833a1bb305fb7291 when this plan was
  written. H0 is read at step 1, after this plan's own save.
- Main's CI on 28254a7 (push): gate run 45503161434 and toolchain run 45503161333, both completed with success
  (22:40:42-22:42:53 UTC).
- `git diff --name-status 1ec6f6c origin/main`: 64 paths, +4,677 / -317: 23 added, 41 modified, none deleted or
  renamed. N0 is the tracked-file count step 1's base light run reads (1,822 at 40a3481; this plan's own commit and
  step 1's save add notes files before it), and refs are N0 + 23 after the merge.
- Kit and CI: main changes no `.claude/`, CLAUDE.md, AGENTS.md, `.github/` or `tools/` file. This branch changes none of
  them: the guard's full pathspec (main-61-plan.md:37-40) prints nothing. So a plain `git merge origin/main` in its own
  call passes the guard. Main's checkpoint tests stay 33.
- `.gitignore` gains `packages/sim-worker/pkg-rules/` and `__pycache__/`. This branch tracks nothing under the first;
  it tracks three files under `__pycache__/` folders, all in .checkpoints/ (`git ls-files ':(glob)**/__pycache__/**'`:
  netcode-proof-test/__pycache__/u4c-test-plant.cpython-313.pyc, netcode-proof-test/u3c-fix2-c1b-finish/__pycache__/
  bites.cpython-313.pyc, netcode-proof-test/u4c-round2-test/__pycache__/r2-plant.cpython-313.pyc). Ignore rules do not
  apply to tracked files, so they stay tracked and unchanged; new ones are ignored; PR #40's close drops .checkpoints/.
- Marks: main changes no docs/adr file, so the 81 rows hash as before.
- refs: `git grep -n -I "spikes/" origin/main -- crates packages Cargo.toml Cargo.lock package.json package-lock.json
  .gitignore .github` prints nothing.
- DEP-HOLD: main changes Cargo.lock, crates/devtools/Cargo.toml, package-lock.json and packages/sim-worker/package.json.
  After the merge each equals main's (`git diff --stat origin/main 5f57b5ae -- Cargo.lock package-lock.json
  crates/devtools/Cargo.toml packages/sim-worker/package.json Cargo.toml package.json` prints nothing), so the save is
  not held (checkpoint.mjs:23-24: "a branch that takes main in is not stopped by it").
- `git merge-tree --write-tree --name-only HEAD origin/main` gives tree 5f57b5ae194d604a649b8c56719d9735bfe2fdbe and one
  conflict, docs/STATUS.md (content). DECISIONS.md, REPO_MAP.md and STATUS_ARCHIVE.md merge on their own.
- The conflict block (tree 5f57b5ae, :24-31; markers at :24, :28, :31):
  - Ours (:25-27): this branch's #40 row (its 2026-11-15 text; `git diff 1ec6f6c HEAD -- docs/STATUS.md` is 1 line
    in, 1 out: the only line this branch changed), then main's #64 row and its "#63, #62" row as 1ec6f6c had them.
  - Theirs (:29-30): main's #40 row (its 2026-11-14 text, unchanged), then the new "none yet" row (change-set 3a).
  - Main took the #64 and "#63, #62" rows out.
- main's docs/STATUS.md has 120 lines.
- Running at merge time: the advisor agent for U6c's round-2 ruling (from 22:50, read only; it writes only
  advisor-u6c-r2.md). Only .checkpoints/netcode-proof-test/ files may show as unstaged; no step stages them, and the
  merge touches none of them.

## Steps (main-64-plan.md's steps 1-7, each in its own call; only what differs is written here)
1. As written (C1, C2, C5):
   - the resume line by Edit;
   - no saver to stop: it ended at 19:47 UTC on its own ("watch stopped: time is up", its task output), so M64-3's
     proof is that notice and `pgrep` exit 1, with no TaskStop;
   - H0 named in the chat;
   - the four pre-checks, with origin/main = 28254a7 and base = 1ec6f6c;
   - the base light run into .gate/main-65/light-base.log, which gives N0, M0 and the self count.
2. `git merge origin/main`. Expected: exit 1 and "CONFLICT (content): Merge conflict in docs/STATUS.md", with no other
   CONFLICT line. `git status --short` shows `UU docs/STATUS.md` and main's other 63 paths staged (A 23, M 40).
3. Resolve by Edit:
   - the block becomes this branch's #40 row, then main's "none yet" (3a) row, each word for word from its side;
   - the #64 and "#63, #62" rows stay out, because main took them out and this branch never changed them;
   - main's #40 row goes, as at MAIN-64; no other line changes.
   Then the marker grep and `git add docs/STATUS.md`. Then `wc -l docs/STATUS.md`: exactly 120, and STATUS.md differs
   from main's in the #40 row alone.
4. As written, with H0 and 28254a7 as the two parents (git's own message, no attribution lines; the card save carries
   them: ADV-M60H-5). C2 (b), observed at 22:5x on a merge preview of 92fdf65 (tree 81aa7f1e…): without
   docs/STATUS.md, `git diff --shortstat HEAD 81aa7f1e -- . ':(exclude)docs/STATUS.md'` and the same over
   `1ec6f6c origin/main` both give "63 files changed, 4644 insertions(+), 284 deletions(-)". Step 4 checks that line
   again over H0 and the merge commit, and reads docs/STATUS.md's own line as is (main's is +33/-33, so the whole stat is
   expected at 64 files, +4,677/-317); if STATUS.md's line differs, step 3's content check (main's file but the #40
   row) governs.
5. a-c as written. Repo map: "repo map OK", with its two numbers recorded. d's bar as written: every count equals the
   base run's word for word, except HEAD, and refs at N0 + 23. Outputs go under .gate/main-65/.
6. As written:
   - main's CI on 28254a7 read again;
   - the card save pushes;
   - CI on the pushed head: gate, smoke and toolchain all green (toolchain now also runs 3a's tests: the rules scan,
     the ChaCha8 and float vectors, the panic hook, wasm-rules.spec.ts);
   - the saver restarts with U6c's merge (its M5 is the next long run), not here.
7. The advisor's check of the pushed head, as at MAIN-64. It is new work for an agent, so under LEAN-MODE item 3 it
   starts in the next reset's first 30 minutes (03:50-04:20 UTC), together with U6c's merge read if that is ready.

## Stop rules, risks, record
- main-61-plan.md's stop rules (:106-114) and risks (:116-128) stand, as at MAIN-64.
- The ruling's advisor only reads; it takes no lane lock (it runs no cargo, test or plant), so the light runs need not
  wait for it.
- New risk (Low): toolchain on PR #40 now also runs 3a's steps. This branch changes nothing those steps read: it
  differs from main only in spikes/, .checkpoints/, docs/design/, the #40 row, and its own lines in DECISIONS.md,
  REPO_MAP.md and STATUS_ARCHIVE.md. A red there is main's or the runner's, and the advisor rules before any further
  step.
- Order with U6c: MAIN-65 merges first (CI needs a mergeable PR); U6c's merge then follows its ruling, its M3 re-checking
  the apply on the new head. Nothing of U6c's is staged during MAIN-65. **Question for the checker:** may U6c's M3-M9
  run once step 6's CI is green, with step 7 reading MAIN-65's merge commit by its SHA at 03:50 (and U6c's head with
  it), or must U6c wait for step 7? The answer moves U6c's merge by about four hours.
- No save, commit or push by hand from step 2 until step 6's card save (M65-1): an agent's notice, a trigger or the
  Stop hook in that window is read and gets no save until step 6.
- The record, under "Choices taken without him": "main 28254a7 taken in now, plan first (MAIN-65)", named in tonight's
  message, msg-2026-11-25.md, as MAIN-64 was in it.

## Change type, tier, size
- A merge of main: main's reviewed and merged text (64 files, +4,677 / -317; #65 had two review rounds by five
  reviewers and a third by security-auditor on one fix, the verifier's PASS, the advisor agent's APPROVE, and his
  browser check of 21:48:49 UTC: STATUS's 3a row), plus one conflict block resolved by hand. Nothing else
  is edited by hand.
- Reviewers: the advisor (this plan, then the pushed head). Mustafa: nothing to check in Brave.
