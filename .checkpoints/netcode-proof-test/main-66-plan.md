# MAIN-66: take main 3ab9eae into the netcode branch (plan, 2026-11-25 16:2x-16:4x UTC)

## As approved
Not checked yet. The advisor agent's check runs in the 18:50-19:20 UTC window (LEAN-MODE item 3: new agent work only in
a reset's first 30 minutes), beside brief 3; it writes advisor-main-66.md. Nothing below runs before its verdict.

## Why
MAIN-52, Mustafa's words (history.md:2055-2057; TEST-FIRST's "main taken in when it changes code, CI or rules";
LEAN-MODE item 1 names "main taken in" on R1's path): main moved from 28254a7 (MAIN-65's) to 3ab9eae by two merges, #66
(09faefa7, 11:10:10 UTC: lane 2's preview hosting check, docs only) and #68 (3ab9eae, 13:57:02 UTC: change-set 3b, "The
loop closes", merged on his tap of 13:56:26 UTC). #68 changes code (crates/client-endpoint, devtools, protocol, room,
sim, sim-wasm; packages/client, e2e, sim-worker, types), the root Cargo.toml and Cargo.lock, four crate manifests and
docs. GitHub reports PR #40 un-mergeable since #68 (the merge-conflict event of 13:57 UTC), so no pull-request CI runs
on #40 until this merge is in (push runs go on: gate.yml and toolchain.yml run on both events). U8's merge follows it
(U4f's form: its M3/M4 apply on the new head). Plan first, in MAIN-65's approved form: main-65-plan.md's steps 1-7
(main-64's, main-61's), with M65-1 to M65-5 (advisor-main-65.md:16-160, its section 3 fallbacks included), M64-1 to
M64-4 (advisor-main-64.md:15-48) and MAIN-61's C1-C6 (advisor-main-61.md:19-104) kept whole. Only the facts and numbers
below differ; "The numbers" gives every value those conditions bind with, so no MAIN-65 value stays in place
(ADV-M65-2).

## Facts checked (16:1x-16:4x UTC, read only)
- origin/main is 3ab9eaecf42e617578c083b7fda01fbdf8f004e2 (`git fetch origin main`, 16:1x). The merge base is
  28254a73ab226379373e380eeee64336e58ef916 (MAIN-65's second parent: 39a0d04^2). HEAD equals the remote branch:
  972f638518e65cf871976d84ef8a0ad56b58a999 when this plan was written; this plan's own save moves it. H0 is read at
  step 1.
- Main's CI on 3ab9eae (its push, head_branch main): gate run 45628804541 (gate job 139256118145, smoke job
  139256117019) and toolchain run 45628802465 (job 139256106111), three check runs, all completed with success
  (13:57:07-14:02:09 UTC). A read by SHA may show more; only main's push runs count, as at MAIN-65.
- `git diff --name-status 28254a7 origin/main`: 87 paths, +10,884 / -601: 29 added, 58 modified, none deleted or
  renamed. Without docs/STATUS.md: 86 files, +10,836 / -553; STATUS.md's numstat is 48 / 48. HEAD's tree holds 1,884
  files and merge-tree's 1,913 (1,884 + 29); N0 is what step 1's base run reads (this plan's saves add notes first).
- Kit and CI: main changes no `.claude/`, CLAUDE.md, AGENTS.md, `.github/`, `tools/`, `legacy/` or `.gitignore` file
  (`git diff --name-only 28254a7 origin/main` over those paths and the `**/` glob forms: empty). This branch changes
  none of them: the guard's full pathspec (main-61-plan.md:37-40) on `origin/main...HEAD` prints nothing. So a plain
  `git merge origin/main` in its own call passes the guard. Main's checkpoint tests stay 33 (no `.claude/` change since
  #65). toolchain.yml is unchanged, so 3b's tests run on #40 as they do on main.
- spikes/, docs/adr/, docs/design/: main changes nothing there. So the 81 marks rows hash as before, and U7's T-builder
  (its worktree and its snapshot to come) and U8's snapshot (wip/u8-build.patch: 43 files, all under
  spikes/netcode-proof/ops/) share no path with main's 87. docs/SPIKE_PLAN.md gains a five-line pointer in entry 6 (the
  preview hosting check); item 1, this test, is untouched.
- The root Cargo.toml: main changes one comment (wasm-bindgen's, :14-16); no member, exclude or dependency line.
- refs: `git grep -n -I "spikes/" origin/main -- crates packages Cargo.toml Cargo.lock package.json package-lock.json
  .gitignore .github` prints nothing; with binaries (`git grep -a -l "spikes/" origin/main -- . ':(exclude)spikes/'
  ':(exclude)docs/' ':(exclude).checkpoints/' ':(exclude)*.md'`) only kept-logs.mjs and its test (the exception).
- DEP-HOLD: main changes Cargo.lock, Cargo.toml, crates/client-endpoint/Cargo.toml, crates/devtools/Cargo.toml,
  crates/room/Cargo.toml and crates/sim-wasm/Cargo.toml (no package.json, no package-lock.json). In merge-tree's tree
  each equals main's (`git diff --stat origin/main eb11edf7 -- Cargo.lock Cargo.toml crates packages package.json
  package-lock.json` prints nothing), so the save is not held (checkpoint.mjs:23-24) while origin/main is still 3ab9eae
  locally at the save. spikes/netcode-proof/Cargo.lock is not touched (brief 3's sha256 55604a5f…9f5d stands).
- `git merge-tree --write-tree --name-only HEAD origin/main` (at 972f638): tree eb11edf7f4ef5c442da5fd819b11ddb998bdd6a4
  and one conflict, docs/STATUS.md (content); DECISIONS.md, REPO_MAP.md and STATUS_ARCHIVE.md merge on their own, and
  this branch's own edits in them come through as they are (patch-ids of 28254a7..HEAD against origin/main..tree:
  DECISIONS f0822ab both ways, STATUS_ARCHIVE 348c3a6 both ways, REPO_MAP a75358f with -U0; as at MAIN-65).
- On that tree: `git diff --quiet origin/main eb11edf7 -- . ':(exclude)spikes/' ':(exclude).checkpoints/'
  ':(exclude)docs/'` exits 0; `git diff --name-only origin/main eb11edf7 -- docs/` prints MAIN-65's six names
  (DECISIONS, REPO_MAP, STATUS, STATUS_ARCHIVE, design/netcode-proof-messages, design/ws-reconnect); and
  `git diff --shortstat HEAD eb11edf7 -- . ':(exclude)docs/STATUS.md'` reads "86 files changed, 10836 insertions(+),
  553 deletions(-)", main's own line.
- The conflict block (tree eb11edf7; markers at :24, :27, :31):
  - Ours (:25-26): this branch's #40 row (its 2026-11-15 text; `git diff 28254a7 HEAD -- docs/STATUS.md` is 1 line in,
    1 out: the only STATUS.md line this branch changed), then the 3a "none yet" row as 28254a7 has it.
  - Theirs (:28-30): main's #40 row (its 2026-11-14 text, unchanged), main's new "none yet" row (change-set 3b), then
    its #66 row.
  - Main replaced the 3a row by the 3b row and added the #66 row.
- main's docs/STATUS.md has 120 lines.
- Main's 29 new paths have no untracked file in their way (`git ls-files --others --exclude-standard -- crates packages
  docs Cargo.toml Cargo.lock` prints nothing). Free disk 15,074 MiB at 16:2x UTC (the light runs need 4,096).
- Running at merge time (new since MAIN-65):
  - U7's T-builder on brief 3 (launch-0850.md section 3, sent in the 18:50 window), on the pre-MAIN-66 head by design:
    main changes nothing under spikes/, and U7's merge applies its snapshot with `git apply --check` on the then head.
    Its cargo calls go through u7-run.sh, which holds the shared target's lane lock for each call (`-s`, or `-x` for a
    counted run: u7-run.sh:54-55). Its notes are in its own worktree, untracked there: nothing of it shows here.
  - The saver bd0efxc5i (from 16:16:54 UTC for 110 minutes: ends about 18:07), or a saver restarted after it.
  - The check-in trig_01EKqNqwZrQTVJ6caAyRrug5 (20:34 UTC) and the day-message trigger trig_01XVEDQzVLaAwy4Fer6AERdS
    (23:05 UTC).

## Steps (main-65-plan.md's steps 1-7, each in its own call; only what differs is written here)
1. As written (C1, C2, C5):
   - the resume line by Edit ("The numbers", C5 (a)), replacing card lines, not adding (M65-3);
   - the saver: if one runs, TaskStop, then C5 (b)'s proof (its output ends "watch stopped: terminated", or the reflog's
     last save is the "(terminated)" autosave, and `pgrep -af "checkpoint.mjs watch"` exits 1); if it ended on its own,
     its notice and pgrep exit 1 (M64-3), with no TaskStop. The card names which proof was used;
   - H0 named in the chat;
   - `git fetch origin` in its own call and main's CI read (C4), then the four pre-checks with origin/main = 3ab9eae and
     base = 28254a7;
   - the base light run into .gate/main-66/light-base.log (run_in_background, timeout 7200000: C3), which gives N0, M0
     and the self count. It waits for the T-builder's lane lock with no limit, then up to 60 minutes until no cargo or
     rustc runs (main-run.sh:28-37).
2. `git merge origin/main`. Expected: exit 1 and "CONFLICT (content): Merge conflict in docs/STATUS.md", with no other
   CONFLICT line. `git status --short` shows `UU docs/STATUS.md` and main's other 86 paths staged (A 29, M 57).
3. Resolve by Edit:
   - the block becomes this branch's #40 row, then main's "none yet" (3b) row, then main's #66 row, each word for word
     from its side;
   - the 3a row stays out, because main replaced it and this branch never changed it;
   - main's #40 row goes, as at MAIN-64 and MAIN-65; no other line changes.
   Then the marker grep and `wc -l docs/STATUS.md`: exactly 120, before `git add docs/STATUS.md`. After the add,
   `git diff --cached --numstat origin/main -- docs/STATUS.md` prints "1 1" (M65-4).
4. As written, with H0 and 3ab9eae as the two parents (git's own message, no attribution lines; the card save carries
   them: ADV-M60H-5). Over H0 and the merge commit, the line without docs/STATUS.md is the 86-file line; the whole line
   is expected at "87 files changed, 10884 insertions(+), 601 deletions(-)" (STATUS.md's 48 / 48 as main's: the swapped
   #40 row is the same on both sides). If STATUS.md's line differs, step 3's content check governs.
5. a-c as written: GATE PASS (`--no-smoke`); "repo map OK", its two numbers recorded; the checkpoint tests 33 pass,
   0 fail. d: the light run (run_in_background, as in step 1): every count equals the base run's word for word, except
   HEAD, and refs at N0 + 29. Outputs go under .gate/main-66/, kept until step 7 is DONE.
6. As written:
   - main's CI on 3ab9eae read again;
   - the card save pushes (`wc -l` on the card first: 200 or fewer);
   - CI on the pushed head: gate, smoke and toolchain all green (toolchain now also runs 3b's new tests: the protocol's
     decoder inputs, the room's and the client endpoint's any-bytes tests, sim-wasm's panic-hook and step-allocation
     tests, the client's keyboard, keys and press-tally tests, the e2e loop and presses specs);
   - the saver restarts here, both trailers (the T-builder runs; from step 6 on, a hand save follows each step).
7. The advisor's check of the pushed head, as at MAIN-65 (advisor-main-65-head.md). It is new agent work, so it starts
   in the next reset's first 30 minutes (23:50 UTC to 00:20 UTC on 2026-11-26), with U8's merge read if that is ready.

## The numbers C1-C6, M64-1 and M65-2 bind with, for this take-in (in place of MAIN-65's)
- C1 (a), (b), 5d: the base run gives every count (refs N0, the full marks line, self S0 of S0, redact R0 of R0 with M0
  items, 0 findings); 5d equals them word for word, except HEAD, and refs = N0 + 29.
- C1 (c): docs/ prints the same six names. `git diff --name-only H0 HEAD` equals `git diff --name-only 28254a7
  origin/main` (87 names). Without docs/STATUS.md, both pairs' shortstat reads "86 files changed, 10836 insertions(+),
  553 deletions(-)"; the whole line is expected at "87 files changed, 10884 insertions(+), 601 deletions(-)".
- C2 (a): "beyond main's 87". The allowance is C2 (a)'s own rule: a ` M`, ` D` or `??` line for a path under
  .checkpoints/netcode-proof-test/, or for the card. Expected: the card, this plan, advisor-main-66.md, launch-0850.md
  (brief 3's fill), history.md (if he writes), msg-2026-11-26.md. A letter in the first column, or any other path
  beyond main's 87, stops.
- C2 (b): after `git add docs/STATUS.md`, each alone: `git diff --name-only --diff-filter=U` prints nothing; `git diff
  --cached --shortstat H0 -- . ':(exclude)docs/STATUS.md'` prints the 86-file line; `git diff --cached --name-only H0 --
  .checkpoints/ spikes/` prints nothing; and the anchor `git diff --cached --quiet
  eb11edf7f4ef5c442da5fd819b11ddb998bdd6a4 -- . ':(exclude).checkpoints/' ':(exclude)docs/STATUS.md'` exits 0 (after
  the commit, `HEAD` in place of `--cached`). It exits 1 today, before the merge.
- C2 (c), C3: "builder 4" is U7's T-builder and its runner (u7-run.sh:54-55, `-s` or `-x` on the same lock). Its
  handback is only read until the push (no save, snapshot, `git apply` or check run on it). The check-in gets card
  edits only; the 23:05 trigger, if it comes inside the window, gets msg-2026-11-26.md by Edit only.
- C4: after `git fetch origin` in its own call, origin/main must be 3ab9eaecf42e617578c083b7fda01fbdf8f004e2. Main's CI
  is main's own push runs: gate run 45628804541 (gate and smoke) and toolchain run 45628802465, three check runs, all
  completed with success.
- C5 (a): the resume line, word for word: "MAIN-66 approved (advisor-main-66.md), running steps 1-7. If `git rev-parse
  -q --verify MERGE_HEAD` prints 3ab9eaecf42e617578c083b7fda01fbdf8f004e2, the merge is in progress: no save, no `git
  merge --abort`, no reset. If `git diff --name-only --diff-filter=U` names docs/STATUS.md, resume at step 3; otherwise
  resume at step 3's checks before the commit (C2 (b)), then step 4. If HEAD^2 is 3ab9eae and HEAD is not on the remote
  yet, resume at step 5. advisor-main-66.md and this card may show unstaged: leave them; step 6's card save takes them.
  U7's T-builder works in its own worktree: nothing of it shows here. Anything else: the advisor rules."
- C5 (b): as step 1 says (TaskStop's proof, or M64-3's).
- C6: step 3's block is merged lines 24-31 (`<<<<<<< HEAD` :24, `=======` :27, `>>>>>>> origin/main` :31). Keep :25
  (ours #40, the 2026-11-15 text), :29 (main's "none yet" row, change-set 3b) and :30 (#66). Drop :26 (the 3a row) and
  :28 (main's #40, the 2026-11-14 text). Step 5b's bar: exit 0 and "repo map OK", both numbers recorded (not worked
  out).
- Step 7's toolchain baseline (M64-4's fourth bullet): main's run 45628802465, job 139256106111.
- M65-2's fallbacks (advisor-main-65.md:81-119) stand with these values: N0 as H0's tree count if they differ; refs at
  5d = H0's tree count + 29, with `git diff --name-only --diff-filter=A H0 HEAD` listing main's 29 names; the staged
  tally decided after the add by `git diff --cached --name-status H0` = `git diff --name-status 28254a7 origin/main`
  (87 paths, 29 A, 58 M); STATUS.md's 120 with no tolerance; merge-tree's tree not bound (eb11edf7 at the plan).

## Stop rules, risks, record
- main-61-plan.md's stop rules (:106-114) and risks (:116-128) stand, as at MAIN-64 and MAIN-65.
- No save, commit or push by hand from step 2 until step 6's card save (M65-1): the T-builder's handback, the check-in,
  the 23:05 trigger, a message from him (his words into history.md by Edit) or the Stop hook in that window is read and
  gets no save until step 6.
- New (Low): U7's T-builder runs beside the take-in. Its base is the pre-MAIN-66 head by design (brief 3 must go out in
  the 18:50 window, and this take-in waits for its check in the same window); it needs nothing from main, and main
  touches nothing under spikes/. While a light run holds the lane lock exclusively (minutes: refs, marks, self,
  redact), the builder's next u7-run.sh call waits silently: `flock` blocks before anything is printed (u7-run.sh:55;
  its default mode is shared, `-x` only for counted runs, check.mjs runs, list or clippy confirmations, bites and
  plants: :3-5, :25-29), and its log header comes only once the lock is held (:66-70). So brief 3 tells the builder to
  run u7-run.sh calls in the background or with a long timeout, and to read a call with no log yet as waiting.
- New (Low): toolchain on PR #40 now also runs 3b's tests. This branch changes nothing those tests read: it differs
  from main only in spikes/, .checkpoints/, docs/design/, the #40 row, and its own lines in DECISIONS.md, REPO_MAP.md
  and STATUS_ARCHIVE.md. A red there is main's or the runner's, and the advisor rules before any further step.
- Disk: the light runs build nothing; main-run.sh's 4 GB floor stands. The T-builder's first cold build takes about
  6.1 GiB (u7-run.sh, incremental off: the card's Key numbers), so about 9 GiB stays free.
- Order with U8: MAIN-66 merges first; U8's merge follows on its own ruling (its M3/M4 apply on the new head). Nothing
  of U8's is staged during MAIN-66.
- The record, under "Choices taken without him": "main 3ab9eae taken in now, plan first (MAIN-66)", named in
  msg-2026-11-26.md, as MAIN-65 was in msg-2026-11-25.md.

## Questions for the checker
1. As M65-5 for U6c: may U8's M3-M9 run once step 6's CI is green, with step 7 reading MAIN-66 by its SHAs at 23:50 UTC
   (and U8's head with it)? U8's 43 files are all under spikes/netcode-proof/ops/, disjoint from main's 87.
2. Brief 3 goes out in the same window on the pre-MAIN-66 head. Does anything in this take-in require the T-builder to
   fast-forward again before U7's merge, or is the snapshot's `git apply --check` on the then head enough?

## Change type, tier, size
- A merge of main: main's reviewed and merged text (87 files, +10,884 / -601) plus one conflict block resolved by hand;
  nothing else is edited by hand. #68 (3b): two review rounds by six reviewers, the verifier PASS, the advisor agent's
  approval, his browser check in Brave and Firefox, merged on his tap (main's STATUS 3b row; its merge message). #66:
  the advisor agent's APPROVE and the PR Advisor's APPROVE on its head, merged on his go (its merge message).
- Risk tier 1 (a spike PR). After the merge, .github/ and .gitignore equal main's; C1 (c)'s first check proves it.
- Reviewers: the advisor (this plan, then the pushed head). Mustafa: nothing to check in Brave.
