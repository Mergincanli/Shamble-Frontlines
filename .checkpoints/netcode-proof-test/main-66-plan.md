# MAIN-66: take main c1da1bd into the netcode branch (plan, 2026-11-25 16:2x UTC; brought up to 311c048 17:0x UTC, to c1da1bd 18:0x UTC)

## As approved
Not checked yet. The advisor agent's check runs in the 18:50-19:20 UTC window (LEAN-MODE item 3: new agent work only in
a reset's first 30 minutes), beside brief 3; it writes advisor-main-66.md. Nothing below runs before its verdict. His
word of 17:00 UTC (history.md:7988-7998, word for word there) brought this plan up to main 311c048, so that one merge
takes in #66, #68 and #69. Main then moved to c1da1bd by #67 (17:12:59 UTC, after his word; docs only): MAIN-52 and the
card's 18:51 fetch item bring the plan up to it, so the one merge takes in #66, #67, #68 and #69 (told to him in one
line, 18:0x UTC). His 17:00 word names #66, #68 and #69 only.

## Why
MAIN-52, Mustafa's words (history.md:2055-2057; TEST-FIRST's "main taken in when it changes code, CI or rules";
LEAN-MODE item 1 names "main taken in" on R1's path): main moved from 28254a7 (MAIN-65's) to c1da1bd by four merges:
#66 (09faefa7, 11:10:10 UTC: lane 2's preview hosting check, docs only), #68 (3ab9eae, 13:57:02 UTC: change-set 3b,
"The loop closes", merged on his tap of 13:56:26 UTC), #69 (311c048, 16:34:47 UTC: the guard fix, guard-env-fix;
merged at 16:34 UTC, his message of 17:00 UTC) and #67 (c1da1bd, 17:12:59 UTC: ADR 0006's small revision after the
preview hosting check, lane 2). #68 changes code (crates/client-endpoint, devtools, protocol, room,
sim, sim-wasm; packages/client, e2e, sim-worker, types), the root Cargo.toml and Cargo.lock, four crate manifests and
docs; #69 changes two kit files
(.claude/hooks/routine-guard.mjs and its test), docs/STATUS.md and docs/ROADMAP.md, and adds one report. #67 changes
seven docs files (ADR 0006, DECISIONS, REPO_MAP, ROADMAP, SPIKE_PLAN, STATUS, STATUS_ARCHIVE) and adds one report: no
code, CI, rule or manifest file, so alone it would not call for a take-in (MAIN-52). It comes in because step 2's `git
merge origin/main`, the form the guard passes in its own call, takes main's tip; this plan uses no other merge form.
By its merge message, #67 was merged from the Builder V2 chat on his go there, with the advisor agent's APPROVE of its
head 35adfbe and gate, smoke and toolchain green on it; by its report (docs/reports/2026-11-25-adr0006-header-row.md,
:151-157, :169), two review rounds: test-engineer APPROVE; infra-engineer REVISE (F1, High), then APPROVE; the
verifier FAIL (plan conformity), then PASS WITH NOTES. GitHub reports
PR #40 un-mergeable since #68 (the merge-conflict event of 13:57 UTC), so no pull-request CI runs on #40 until this
merge is in (push runs go on: gate.yml and toolchain.yml run on both events). U8's merge follows it (U4f's form: its
M3/M4 apply on the new head). Plan first, in MAIN-65's approved form: main-65-plan.md's steps 1-7 (main-64's,
main-61's), with M65-1 to M65-5 (advisor-main-65.md:16-160, its section 3 fallbacks included), M64-1 to M64-4
(advisor-main-64.md:15-48) and MAIN-61's C1-C6 (advisor-main-61.md:19-104) kept whole. Only the facts and numbers below
differ; "The numbers" gives every value those conditions bind with, so no MAIN-65 value stays in place (ADV-M65-2).

## Facts checked (16:1x-16:4x UTC; again on 311c048 at 17:0x UTC and on c1da1bd at 17:5x-18:0x UTC; read only)
- origin/main is c1da1bd24faae21794195e2384895460753a41eb (`git fetch origin main`, 17:5x UTC). The merge base is
  28254a73ab226379373e380eeee64336e58ef916 (MAIN-65's second parent: 39a0d04^2; `git merge-base HEAD c1da1bd`). HEAD
  equals the remote branch: 2935f0a3ee9870c966025e76f674a80a1244977a when this revision was written; its own save
  moves it. H0 is read at step 1.
- Main's CI on c1da1bd (its push, head_branch main): gate run 45662377295 (gate job 139375346631, smoke job
  139375347099) and toolchain run 45662377271 (job 139375347198), three check runs, all completed with success
  (17:13:05-17:18:09 UTC). A read by SHA may show more; only main's push runs count, as at MAIN-65.
- `git diff --name-status 28254a7 origin/main`: 92 paths, +11,561 / -631: 31 added, 61 modified, none deleted or
  renamed (`--diff-filter` A, M and DRCTUXB, each alone). Without docs/STATUS.md: 91 files, +11,512 / -582;
  STATUS.md's numstat is 49 / 49. HEAD's tree holds 1,886 files and merge-tree's 1,917 (1,886 + 31; both by shortstat
  against the empty tree); N0 is what step 1's base run reads (this plan's saves add notes first).
- Kit and CI: main now changes two kit files, .claude/hooks/routine-guard.mjs and .claude/hooks/routine-guard.test.mjs
  (#69: risk tier 2 on main; by its report, docs/reports/2026-11-25-guard-env-fix.md: security-auditor's round-2
  APPROVE, the verifier's re-run PASS and the advisor agent's plan approval; merged at 16:34 UTC, his message). Main
  changes no other `.claude/` file (settings.json and guard.mjs among them) and no CLAUDE.md, AGENTS.md, `.github/`,
  `tools/`, `legacy/` or `.gitignore` file (#67 changes none of these). This branch changes
  none of the guard's pathspec (main-61-plan.md:37-40): on `origin/main...HEAD` it prints nothing (on
  `HEAD...origin/main`, exactly the two guard files). That is the guard's
  own test for a plain merge in the main folder (routine-guard.mjs:1347-1351, unchanged by #69: "a plain merge takes
  main's kit files exactly only when the branch changed none since the merge base"), so `git merge origin/main` in its
  own call passes it, and after it `.claude/` equals main's. Main's checkpoint tests stay 33 (#69 leaves them); its
  guard tests are 1,247 (#69's report, "Evidence"). toolchain.yml is unchanged, so 3b's tests run on #40 as on main.
- The guard after the merge: settings.json's PreToolUse, PermissionRequest and ConfigChange hooks start a new node
  process at every call, which imports `${CLAUDE_PROJECT_DIR}/.claude/hooks/guard.mjs` from disk
  (settings.json:271-292), and guard.mjs imports routine-guard.mjs from its own folder (guard.mjs:42). #69 changes
  neither settings.json nor guard.mjs. So from the first call after step 2's `git merge` writes main's
  routine-guard.mjs into the main folder (before the commit), this session's calls are checked by main's fixed rules;
  the merge call itself by the old ones. Whether an agent's calls from its worktree resolve to the main folder's copy
  too is not tested.
- spikes/ and docs/design/: main changes nothing there. docs/adr/: #67 changes ADR 0006 only
  (0006-infrastructure-operations.md: the rows of PM-0006-2 and PM-0006-7, and prose outside the table in four other
  hunks). marks.json's 17 rows from that file are PM-0006-11 to -27 (`git grep` of its ids); #67's table hunk changes
  only the -2 and -7 rows and adds no other line starting `| PM-`. check.mjs's marks step finds each mark's row by
  `| <id> |` and compares its sha256 (check.mjs:1880-1905, :2632-2641), and takes its expected ids from ADR 0001
  (:1887), which main does not change. So the 81 marks rows hash as before, and U7's T-builder
  (its worktree and its snapshot to come) and U8's snapshot (wip/u8-build.patch: 43 files, all under
  spikes/netcode-proof/ops/) share no path with main's 92. docs/SPIKE_PLAN.md gains a five-line pointer in entry 6 (the
  preview hosting check, #66) and a six-line pointer after it (#67); item 1, this test, is untouched. The spike names
  SPIKE_PLAN.md only in comments (Cargo.toml:1, README.md) and in a refs-test fixture held in memory (check.test.mjs:1565).
- The root Cargo.toml: main changes one comment (wasm-bindgen's, :14-16); no member, exclude or dependency line.
- refs: `git grep -n -I "spikes/" origin/main -- crates packages Cargo.toml Cargo.lock package.json package-lock.json
  .gitignore .github` prints nothing; with binaries (`git grep -a -l "spikes/" origin/main -- . ':(exclude)spikes/'
  ':(exclude)docs/' ':(exclude).checkpoints/' ':(exclude)*.md'`) only kept-logs.mjs and its test (the exception).
- DEP-HOLD: main changes Cargo.lock, Cargo.toml, crates/client-endpoint/Cargo.toml, crates/devtools/Cargo.toml,
  crates/room/Cargo.toml and crates/sim-wasm/Cargo.toml (no package.json, no package-lock.json; #69 and #67 add none).
  In merge-tree's tree each equals main's, and so do `.claude/`, `.github/` and `tools/` (`git diff --stat origin/main
  f3a420bc -- Cargo.lock Cargo.toml crates packages package.json package-lock.json .claude .github tools` prints
  nothing), so the save is not held (checkpoint.mjs:23-24) while origin/main is still c1da1bd locally at the save.
  spikes/netcode-proof/Cargo.lock is not touched (brief 3's sha256 55604a5f…9f5d stands).
- `git merge-tree --write-tree --name-only HEAD c1da1bd` (at 2935f0a): tree f3a420bc7d1a92da6b7840dda3f2d57ce7d86075
  and one conflict, docs/STATUS.md (content); DECISIONS.md, REPO_MAP.md and STATUS_ARCHIVE.md merge on their own, and
  this branch's own edits in them come through as they are (patch-ids of 28254a7..HEAD against origin/main..tree,
  re-run on f3a420bc: DECISIONS f0822ab both ways, STATUS_ARCHIVE 348c3a6 both ways, REPO_MAP a75358f with -U0; as at
  MAIN-65).
- On that tree: `git diff --quiet origin/main f3a420bc -- . ':(exclude)spikes/' ':(exclude).checkpoints/'
  ':(exclude)docs/'` exits 0; `git diff --name-only origin/main f3a420bc -- docs/` prints MAIN-65's six names
  (DECISIONS, REPO_MAP, STATUS, STATUS_ARCHIVE, design/netcode-proof-messages, design/ws-reconnect); and
  `git diff --shortstat HEAD f3a420bc -- . ':(exclude)docs/STATUS.md'` reads "91 files changed, 11512 insertions(+),
  582 deletions(-)", main's own line.
- The conflict block (tree f3a420bc; markers at :24, :27, :31, by `git grep -n` on the tree):
  - Ours (:25-26): this branch's #40 row (its 2026-11-15 text; `git diff 28254a7 HEAD -- docs/STATUS.md` is 1 line in,
    1 out: the only STATUS.md line this branch changed), then the 3a "none yet" row as 28254a7 has it.
  - Theirs (:28-30): main's #40 row (its 2026-11-14 text, unchanged), then main's #67 row, then main's #69 row.
  - Main replaced the 3a row by the 3b and #66 rows at #68, took those out for the #69 row at #69, and added the #67
    row above it at #67. Main's file still lists #67 and #69 as in flight: main's text, taken as it is.
- main's docs/STATUS.md has 120 lines (c1da1bd, by numstat against the empty tree; #67's own numstat on it: 10 / 10).
- Main's 31 new paths have no untracked file in their way (`git ls-files --others --exclude-standard -- docs
  .claude/hooks crates packages Cargo.toml Cargo.lock` prints nothing). Free disk 15,060 MiB at 18:0x UTC (the light
  runs need 4,096).
- Running at merge time (new since MAIN-65):
  - U7's T-builder on brief 3 (launch-0850.md section 3, sent in the 18:50 window), on the pre-MAIN-66 head by design:
    main changes nothing under spikes/, and U7's merge applies its snapshot with `git apply --check` on the then head.
    Its cargo calls go through u7-run.sh, which holds the shared target's lane lock for each call (`-s`, or `-x` for a
    counted run: u7-run.sh:54-55). Its notes are in its own worktree, untracked there: nothing of it shows here.
  - The saver restarted after bd0efxc5i's end (about 18:07 UTC; from 16:16:54 for 110 minutes); the card names it.
  - The check-in trig_01EKqNqwZrQTVJ6caAyRrug5 (20:34 UTC) and the day-message trigger trig_01XVEDQzVLaAwy4Fer6AERdS
    (23:05 UTC).

## Steps (main-65-plan.md's steps 1-7, each in its own call; only what differs is written here)
Every command keeps his interim rule (history.md:7993): git and gh always start their own command, with no other
program in front of them.
1. As written (C1, C2, C5):
   - the resume line by Edit ("The numbers", C5 (a)), replacing card lines, not adding (M65-3);
   - the saver: if one runs, TaskStop, then C5 (b)'s proof (its output ends "watch stopped: terminated", or the reflog's
     last save is the "(terminated)" autosave, and `pgrep -af "checkpoint.mjs watch"` exits 1); if it ended on its own,
     its notice and pgrep exit 1 (M64-3), with no TaskStop. The card names which proof was used;
   - H0 named in the chat;
   - `git fetch origin` in its own call and main's CI read (C4), then the four pre-checks with origin/main = c1da1bd and
     base = 28254a7;
   - the base light run into .gate/main-66/light-base.log (run_in_background, timeout 7200000: C3), which gives N0, M0
     and the self count. It waits for the T-builder's lane lock with no limit, then up to 60 minutes until no cargo or
     rustc runs (main-run.sh:28-37).
2. `git merge origin/main`. Expected: exit 1 and "CONFLICT (content): Merge conflict in docs/STATUS.md", with no other
   CONFLICT line. `git status --short` shows `UU docs/STATUS.md` and main's other 91 paths staged (A 31, M 60). From
   the next call on, the guard's rules are main's (the Facts' guard line).
3. Resolve by Edit:
   - the block becomes this branch's #40 row, then main's #67 row and main's #69 row, in main's order, each word for
     word from its side;
   - the 3a row stays out, because main replaced it and this branch never changed it;
   - main's #40 row goes, as at MAIN-64 and MAIN-65; no other line changes.
   Then the marker grep and `wc -l docs/STATUS.md`: exactly 120, before `git add docs/STATUS.md`. After the add,
   `git diff --cached --numstat origin/main -- docs/STATUS.md` prints "1 1" (M65-4).
4. As written, with H0 and c1da1bd as the two parents (git's own message, no attribution lines; the card save carries
   them: ADV-M60H-5). Over H0 and the merge commit, the line without docs/STATUS.md is the 91-file line; the whole line
   is expected at "92 files changed, 11561 insertions(+), 631 deletions(-)" (STATUS.md's 49 / 49 as main's: the swapped
   #40 row is the same on both sides). If STATUS.md's line differs, step 3's content check governs.
5. a-c as written: GATE PASS (`--no-smoke`); "repo map OK", its two numbers recorded; the checkpoint tests 33 pass,
   0 fail. d: the light run (run_in_background, as in step 1): every count equals the base run's word for word, except
   HEAD, and refs at N0 + 31. e (new): `node --test .claude/hooks/routine-guard.test.mjs` (main's file
   now): every test passes, 0 fail, the count recorded (1,247 by #69's report). Outputs go under .gate/main-66/, kept
   until step 7 is DONE.
6. As written:
   - main's CI on c1da1bd read again;
   - the card save pushes (`wc -l` on the card first: 200 or fewer);
   - CI on the pushed head: gate, smoke and toolchain all green (toolchain now also runs 3b's new tests: the protocol's
     decoder inputs, the room's and the client endpoint's any-bytes tests, sim-wasm's panic-hook and step-allocation
     tests, the client's keyboard, keys and press-tally tests, the e2e loop and presses specs);
   - the saver restarts here, both trailers (the T-builder runs; from step 6 on, a hand save follows each step).
7. The advisor's check of the pushed head, as at MAIN-65 (advisor-main-65-head.md). It is new agent work, so it starts
   in the next reset's first 30 minutes (23:50 UTC to 00:20 UTC on 2026-11-26), with U8's merge read if that is ready.

## The numbers C1-C6, M64-1 and M65-2 bind with, for this take-in (in place of MAIN-65's)
- C1 (a), (b), 5d: the base run gives every count (refs N0, the full marks line, self S0 of S0, redact R0 of R0 with M0
  items, 0 findings); 5d equals them word for word, except HEAD, and refs = N0 + 31.
- C1 (c): docs/ prints the same six names. `git diff --name-only H0 HEAD` equals `git diff --name-only 28254a7
  origin/main` (92 names). Without docs/STATUS.md, both pairs' shortstat reads "91 files changed, 11512 insertions(+),
  582 deletions(-)"; the whole line is expected at "92 files changed, 11561 insertions(+), 631 deletions(-)".
- C2 (a): "beyond main's 92". The allowance is C2 (a)'s own rule: a ` M`, ` D` or `??` line for a path under
  .checkpoints/netcode-proof-test/, or for the card. Expected: the card, this plan, advisor-main-66.md, launch-0850.md
  (brief 3's fill), history.md (if he writes), msg-2026-11-26.md. A letter in the first column, or any other path
  beyond main's 92, stops.
- C2 (b): after `git add docs/STATUS.md`, each alone: `git diff --name-only --diff-filter=U` prints nothing; `git diff
  --cached --shortstat H0 -- . ':(exclude)docs/STATUS.md'` prints the 91-file line; `git diff --cached --name-only H0 --
  .checkpoints/ spikes/` prints nothing; and the anchor `git diff --cached --quiet
  f3a420bc7d1a92da6b7840dda3f2d57ce7d86075 -- . ':(exclude).checkpoints/' ':(exclude)docs/STATUS.md'` exits 0 (after
  the commit, `HEAD` in place of `--cached`). It exits 1 today, before the merge.
- C2 (c), C3: "builder 4" is U7's T-builder and its runner (u7-run.sh:54-55, `-s` or `-x` on the same lock). Its
  handback is only read until the push (no save, snapshot, `git apply` or check run on it). The check-in gets card
  edits only; the 23:05 trigger, if it comes inside the window, gets msg-2026-11-26.md by Edit only.
- C4: after `git fetch origin` in its own call, origin/main must be c1da1bd24faae21794195e2384895460753a41eb. Main's CI
  is main's own push runs: gate run 45662377295 (gate and smoke) and toolchain run 45662377271, three check runs, all
  completed with success.
- C5 (a): the resume line, word for word: "MAIN-66 approved (advisor-main-66.md), running steps 1-7. If `git rev-parse
  -q --verify MERGE_HEAD` prints c1da1bd24faae21794195e2384895460753a41eb, the merge is in progress: no save, no `git
  merge --abort`, no reset. If `git diff --name-only --diff-filter=U` names docs/STATUS.md, resume at step 3; otherwise
  resume at step 3's checks before the commit (C2 (b)), then step 4. If HEAD^2 is c1da1bd and HEAD is not on the remote
  yet, resume at step 5. advisor-main-66.md and this card may show unstaged: leave them; step 6's card save takes them.
  U7's T-builder works in its own worktree: nothing of it shows here. Anything else: the advisor rules."
- C5 (b): as step 1 says (TaskStop's proof, or M64-3's).
- C6: step 3's block is merged lines 24-31 (`<<<<<<< HEAD` :24, `=======` :27, `>>>>>>> origin/main` :31). Keep :25
  (ours #40, the 2026-11-15 text), :29 (main's #67 row) and :30 (main's #69 row). Drop :26 (the 3a row) and :28 (main's
  #40, the 2026-11-14 text). Step 5b's bar: exit 0 and "repo map OK", both numbers recorded (not worked out).
- Step 7's toolchain baseline (M64-4's fourth bullet): main's run 45662377271, job 139375347198.
- M65-2's fallbacks (advisor-main-65.md:81-119) stand with these values: N0 as H0's tree count if they differ; refs at
  5d = H0's tree count + 31, with `git diff --name-only --diff-filter=A H0 HEAD` listing main's 31 names; the staged
  tally decided after the add by `git diff --cached --name-status H0` = `git diff --name-status 28254a7 origin/main`
  (92 paths, 31 A, 61 M); STATUS.md's 120 with no tolerance; merge-tree's tree not bound (f3a420bc at this revision).
  5e's count: any fail stops; another total with 0 fail, while `git diff --quiet origin/main HEAD -- .claude/` exits 0,
  is recorded and goes on (step 7 reads it).

## Stop rules, risks, record
- main-61-plan.md's stop rules (:106-114) and risks (:116-128) stand, as at MAIN-64 and MAIN-65.
- No save, commit or push by hand from step 2 until step 6's card save (M65-1): the T-builder's handback, the check-in,
  the 23:05 trigger, a message from him (his words into history.md by Edit) or the Stop hook in that window is read and
  gets no save until step 6.
- New (Low): the guard's rules change inside the take-in (step 2 writes main's routine-guard.mjs; the Facts' guard
  line). The later steps' commands are plain forms. If one of them asks or is refused under the new rules, the step
  stops there, the merge stays as it is, and the advisor rules; no other form is tried.
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
- New (Low): main moved three times today (#68, #69, #67). If step 1's fetch finds origin/main other than c1da1bd, C4
  stops the take-in before step 2; Q4 asks the checker which values may then be re-derived without a new check.
- Disk: the light runs build nothing; main-run.sh's 4 GB floor stands. The T-builder's first cold build takes about
  6.1 GiB (u7-run.sh, incremental off: the card's Key numbers), so about 9 GiB stays free.
- Order with U8: MAIN-66 merges first; U8's merge follows on its own ruling (its M3/M4 apply on the new head). Nothing
  of U8's is staged during MAIN-66.
- The record, under "Choices taken without him": "main c1da1bd taken in now (#66, #67, #68 and #69 in one merge: his
  word of 17:00 UTC for #66, #68 and #69; #67 by MAIN-52 and the card's fetch item, told to him in one line), plan
  first (MAIN-66)", named in msg-2026-11-26.md, as MAIN-65 was in msg-2026-11-25.md.

## Questions for the checker
1. As M65-5 for U6c: may U8's M3-M9 run once step 6's CI is green, with step 7 reading MAIN-66 by its SHAs at 23:50 UTC
   (and U8's head with it)? U8's 43 files are all under spikes/netcode-proof/ops/, disjoint from main's 92.
2. Brief 3 goes out in the same window on the pre-MAIN-66 head. Does anything in this take-in require the T-builder to
   fast-forward again before U7's merge, or is the snapshot's `git apply --check` on the then head enough?
3. With #69 in, this session's calls are checked by main's fixed rules from the first call after step 2. Any condition
   on that (step 5e; the order of the later steps), and does the take-in need a security read? My lean: no, since #69
   had security-auditor's two rounds on main (round 2 APPROVE) and this merge takes it byte for byte.
4. Main moved three times today. If it moves again by a docs-only merge (no code, CI, rule, manifest or spikes/ file,
   and no docs/adr/ row that marks.json hashes) between your verdict and step 1, which values may I re-derive by the
   same commands under M65-2's style of fallback without a new check (C4's commit and runs, the counts, the anchor
   tree, the block's lines and the line count), and which need you again? My lean: all of those may be re-derived,
   recorded on the card before step 2, with one line to him; anything else stops for you.

## Change type, tier, size
- A merge of main: main's reviewed and merged text (92 files, +11,561 / -631) plus one conflict block resolved by hand;
  nothing else is edited by hand. #68 (3b): two review rounds by six reviewers, the verifier PASS, the advisor agent's
  approval, his browser check in Brave and Firefox, merged on his tap (its merge message). #66: the advisor agent's
  APPROVE and the PR Advisor's APPROVE on its head, merged on his go (its merge message). #69: risk tier 2 on main;
  security-auditor's round-2 APPROVE, the verifier's re-run PASS, the advisor agent's plan approval (its report);
  merged at 16:34 UTC (his message of 17:00 UTC). #67: two review rounds and the advisor agent's APPROVE of its head,
  CI green, merged from the Builder V2 chat on his go there (its merge message and report, as "Why" gives them).
- Risk tier 1 for PR #40 (a spike PR): after the merge, `.claude/`, `.github/` and `.gitignore` equal main's (C1 (c)'s
  first check), so the PR changes no kit file; the merge commit carries main's own tier-2 change as main merged it.
- Reviewers: the advisor (this plan, then the pushed head). Mustafa: nothing to check in Brave.
