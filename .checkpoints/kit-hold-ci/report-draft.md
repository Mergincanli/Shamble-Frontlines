# Advisor report: kit-hold-ci (2026-11-18, branch claude/wonderful-hamilton-jdmlnf, base main 9b4b518)
The kit hold and two CI edits; pull request #60; risk tier 2 (the kit and CI); the branch stands in for
`cs/kit-hold-ci`. Built by the first builder chat (lane 1). All times are UTC, from a record.

## Goal
One small pull request with three things and nothing else, as Mustafa asked after the merge of #58:
1. The checkpoint script holds a push that changes a manifest or a lockfile until the security check of that change
   is recorded (his answer DEP-HOLD (a); his exception to the kit freeze, for this one change).
2. CI writes the generated types again and compares, and checks the TypeScript of `packages/types` with the strict
   flags (his answer CI-TYPES (a)).
3. The `toolchain` job runs with no cache (his answer CI-CUT (a), part 1).

Nothing a player sees, hears or feels.

## Outcome
DONE: written with the texts he said yes to, reviewed in one round, verified. It waits for the Advisor's check of
this report and then for Mustafa's go on the merge. No finding is Critical or High. **Two Medium findings of the
security review stay open as follow-ups**, by his word of 18:01:08 ("A later finding on them that is not Critical or
High becomes a follow-up"): they stand under "Open findings". What went wrong on the way comes first.

## What went wrong on the way
1. **A hole in my own changed lines stood on the pushed branch for 45 minutes** (17:18:16 to 18:03:10).
   - What: after his one change ("a manifest or lockfile is not held when its content in the push equals its content
     on main"), the script compared two reads of the same path that were made in two ways. Git reads a path that
     starts with ":" as a pattern (":/Cargo.lock" means the `Cargo.lock` at the top). So when main already held a
     manifest in a root folder named ":" with the same content as the root file, a push that changed the file in
     that folder was let through with no record. The text he first said yes to held that push.
   - Its reach: none. Main holds no such folder, and one can only arrive through a merge. The script ran on this
     branch only.
   - How it was found: the advisor agent's read of the changed lines (16:51 to about 17:10) found the first half
     (main read as a pattern) and gave the fix for it; that fix was written and pushed. Claude Code's automated
     review of the push then flagged the line beside it (17:21), and my probe in a scratch repository showed the
     release (17:29:40).
   - The fix: the rule's two lines became three and read both sides at the exact path; the last step of the new test
     builds exactly this case. Asked of him with the exact text at 17:48:13; his yes at 18:01:08; written 18:01;
     pushed 18:03:10 (commit 4877599). The probe on the written script: held.
   - Why it got through one read: the advisor agent and I each tried a file in such a folder that main does not
     have. Neither of us tried the case where main has it too.
2. **A count I gave him was off by one.** I told him the text he saw was 219 added lines; it was 218 (the
   workflow's 12 lines were counted as 13). Told to him at 17:48. Since then one method counts everything:
   `git diff --numstat`.

## Open findings (they stay, as follow-ups, by his word of 18:01:08)
1. **Medium (security-auditor): in two corners the hold checks another path than the one it pushes.** The script
   reads the list of changed files through a helper that trims the text and decodes it as UTF-8. So a manifest in a
   root folder whose name begins with a space (or a tab or another blank character), when it is the first changed
   file, is checked under the name without the blank. Shown in a scratch repository: main holds `a/Cargo.lock`;
   the push adds ` a/Cargo.lock` (a folder named " a") with new content and no record: "saved and pushed". The same
   for a folder name that is not valid UTF-8 when main holds its look-alike with the same content.
   - Its reach: tiny, and the same kind as the hole fixed by KIT-FIX. No such folder exists; the workspaces are
     `crates/*` and `packages/*`, so CI would not build a manifest in such a folder unless a root manifest changed
     too, and that change is held.
   - It comes from my "equal to main" lines meeting an older line of the script; the text he first said yes to
     would have held the first case with no record.
   - The follow-up: read that list raw (no trim, no decoding), with a test for a root folder " a" beside main's
     `a/Cargo.lock`.
2. **Medium (security-auditor): nothing makes the review of a pull request read the record.** The script's comment
   says "the review of the pull request reads it", but no rule names a reader, and the notes folder (the record
   with it) is deleted before a pull request is marked ready. A record written too early, or for other content,
   would go unchecked.
   - The follow-up (a kit change, so with his word): the ready report of a pull request that changes a manifest or
     a lockfile quotes each record (with the commit it was pushed in) and the security answer it names; the review
     checks every changed manifest at the head against those lines or main.
   - Until then, by practice and with no kit change: change-set 2's report does exactly that.

## His words, the questions put to him in chat, and his answers
His three messages of today stand word for word in `docs/DECISIONS.md` (by script, this pull request's docs pass):
15:38:13 (his go on #58 and his five answers), 16:39:23 and 18:01:08. The questions as I put them:
- **KIT-TEXT** (16:23:55, with the kit edit's diff in full and how the hold is released): "may the kit hold be
  written with the exact text above (and in the file)?" Options: (a) yes, as shown; (b) yes, with changes he names;
  (c) no. I recommended (a). His answer (16:39:23): "(b), yes, with one change. The change: a manifest or lockfile is
  not held when its content in the push equals its content on main. That content was merged with my go, so a branch
  that takes main in is not stopped by it. Say so in the rule line too. Everything else stays exactly as in that
  file. Add a test for the change and show it failing on a broken copy, as you did for the others. Your advisor
  agent reads the changed lines once. Then write the four files. Your ready report shows the changed lines; I do
  not need to see them before you write." The same message says yes to the planted change, to the reading of
  "nothing else in it", and to the stop rule for the new CI lines.
- **KIT-FIX** (17:48:13, with the fix's diff in full): "may I write the fix above into the two kit files?" Options:
  (a) yes, as shown; (b) yes, with changes he names; (c) no, leave it and record the hole as a known gap. I
  recommended (a). His answer (18:01:08): "(a), yes, as shown: the text of kit-fix2.diff as it stands in your notes
  at commit f7e232fb (45 lines, sha256 9197f070e456ac628703f4211380174ccdc037e8bcd4eb1a950da252b7a4d7f9). Write it
  into the two kit files. With this fix the kit lines are final for this pull request. A later finding on them that
  is not Critical or High becomes a follow-up."
- **KIT-REMOVED** (17:48:13): "should a manifest that main removed also pass when a branch takes main in?" Options:
  (a) leave it held; (b) also pass it (about 15 more lines, past the stop line of 300; the exact text shown first).
  The advisor agent recommended (b); I recommended (a) for this pull request. His answer: "(a). A manifest that main
  removed stays held. Put the question on the list for the quality gate v2's plan, together with the line of the
  approved text that still reads a path beginning with ":" as a pattern."
- **WATCH-60** (17:48:13): "should this chat watch pull request #60 for comments and CI results?" Options: (a) yes;
  (b) no, as for #58. I recommended (b). His answer: "(b), no."
- The size: "The size as you count it, 293 added lines against the stop line of 300, is accepted."

Also carried out from his message of 15:38:13, outside this pull request: #58 merged (9b4b518, 15:39:28, his words in
the merge message); the "Lane requests" issue opened as #59 (his LANE-ISSUE (a)); #58 not watched (WATCH-58 (b)).

## Plan and approval
- The plan: `.checkpoints/kit-hold-ci/plan.md` at commit CHECKPOINT-COMMIT. Planned size: 150 added lines; the stop
  line (twice the plan): 300.
- The advisor agent on the plan (15:46 to 16:07): APPROVE on eight conditions, all taken. On the revised kit text
  (16:18): REVISE, one test step never reached the hold; fixed and shown failing on a broken copy (16:22), which was
  its condition for APPROVE. On the lines changed after his KIT-TEXT answer (16:51 to about 17:10, one read, as he
  asked): REVISE, one Medium (main read as a pattern); its exact fix and test step applied and shown (17:12 to
  17:13), which was its condition for APPROVE. Its notes: `advisor-plan.md` in the same folder.
- His readings it ruled on: "main" is `origin/main` as the clone last fetched it; "content" is the file's bytes (the
  file mode is not compared; a link stays held); a file removed in the push stays held (later his KIT-REMOVED (a));
  the comment names him.
- "Nothing else in it" is read as: no other code, kit, gate or CI change and no work of change-set 2; the report and
  the records that are due ride along. His words (16:39:23): "your reading is right."

## What changed
Four files, 293 added lines and 2 removed (`git diff --numstat origin/main -- .claude .github`), against 150 planned.
His words (18:01:08): the size "is accepted."

| File | Added | What |
|---|---|---|
| `.claude/scripts/checkpoint.mjs` | 47 (1 removed) | the hold: one function (`heldDependencyFiles`), seven lines in `save`, the header's paragraph |
| `.claude/scripts/checkpoint.test.mjs` | 224 (1 removed) | six new tests (28 to 33); two imports; two helpers |
| `.claude/CHECKPOINTS.md` | 10 | section 3: one rule line |
| `.github/workflows/toolchain.yml` | 12 | `cache-mode: none`; the regenerate-and-compare lines; the two type-check lines |

**What the hold does.** `save` and the background saver commit the notes locally as before. Then, while what they
would push changes `Cargo.toml`, `Cargo.lock`, `package.json`, `package-lock.json` or `npm-shrinkwrap.json` (in any
folder) against the remote branch, they do not push: the files are named and `save` exits 1.
- **Released** by `.checkpoints/<slug>/dependency-check.md` in the commit to push, written after security-auditor's
  answer on exactly that content and naming that answer's notes file: one line per file, its sha256 and its path as
  `sha256sum` prints it (`deleted  <path>` for a removed file). The record releases only that content: when the
  file changes again, the hold is back.
- **Not held** (his one change): a plain file whose content in the push equals its content on `origin/main` at the
  same path.
- **Held whatever the record says:** what cannot be read, and an entry that is not a plain file (a link, a
  submodule).
- The five names are this chat's reading of "a manifest or a lockfile", for Cargo and npm. He saw them as a reading
  (16:23:55).

**Every rule sentence that changed** (all are new or stricter; none is looser):

| Where | Before (main 9b4b518) | After |
|---|---|---|
| `.claude/CHECKPOINTS.md` section 3 | no line on dependencies | a new line, "A dependency change is held": the script never pushes while what it would push changes one of the five files, compared with the remote branch (with main for a branch not yet pushed); released by the record; "Until then build nothing that holds the change, and push nothing by hand: a plain `git push` is not held."; a manifest or lockfile is not held when its content in the push equals its content on main (`origin/main` as last fetched) |
| `checkpoint.mjs`, header | "Exit codes: 0 saved or nothing to save · 1 a git step failed or a secret was found · 2 usage error or refused by a guard." | "... · 1 a git step failed, a secret was found or a dependency change is held · 2 usage error or refused by a guard." |
| `checkpoint.mjs`, header | no paragraph on dependencies | a new paragraph, "Dependency hold", with the same content as the rule line, and: "The script trusts that record; the review of the pull request reads it." |
| `checkpoint.mjs`, behaviour | `save` pushes every local commit of the branch; the saver pushes when only notes are ahead | both first ask whether a held file would be published; if so nothing is pushed |

No guard change, no change to `CLAUDE.md`, to `.claude/settings.json`, to an agent file or to `gate.yml`.

**The two CI edits** are byte for byte the text of `docs/reports/2026-11-18-cs1-types.md`, "The exact texts asked
for", blocks 1 to 3 (built by script from that report at 1bfe462; checked again by the advisor agent and by
infra-engineer).

## Evidence
**The kit's tests** (after the last write, 18:01:45 to 18:02:58):

| Test file | Main | This head |
|---|---|---|
| `.claude/scripts/checkpoint.test.mjs` | 27 pass | 33 pass |
| `.claude/hooks/routine-guard.test.mjs` | 1107 pass | 1107 pass |
| `.claude/hooks/settings.test.mjs` | 127 pass | 127 pass |
| `.claude/scripts/kept-logs.test.mjs` | 10 tests, 9 pass, 0 fail | the same |

`node tools/gate.mjs --no-smoke`: PASS (the smoke check ran in CI). The repository map check: OK.

**The new tests bite.** test-engineer made 49 broken copies of the script of this head, one change each, and ran
the test file against each (its table, shortened; "#n" is the test that fails):

| Broken copy of this head's script | Result |
|---|---|
| the hold removed | #28, #29, #30, #31, #32, #33 fail |
| the hold skipped for the background saver | #32 fails |
| the record read from the working tree, not from the commit | #28 fails |
| renames followed in the diff | #31, #32 fail |
| the record accepts any line, or any hash for a path | #28, #29 fail |
| the list of names emptied; each one of the five names dropped | all six fail; at least one fails for each name |
| a link treated as a plain file | #31 fails (#33 too when both reads do it) |
| the comparison with main dropped | #33 fails (step 1) |
| any plain file that main also has is released, whatever its content | #33 fails (step 2) |
| the push side, or both sides, read as a pattern in the comparison with main | #33 fails (step 4) |
| the base always main, never the remote branch | #29, #31, #32, #33 fail |
| "deleted" releases any entry that still exists | #31 fails |
| **main's side alone read as a pattern** | **survived: no test fails** |
| **"deleted" releases a plain file that still exists** | **survived** |
| **a local `main` compared, not `origin/main`** | **survived** |
| the hold moved below the saver's "code is ahead" branch; the comparison moved before the plain-file check | survived, and equal in behaviour (no difference to catch) |

- The six new tests fail against main's script (27 pass, 6 fail); main's 27 tests pass against the new script.
- The three copies that survived are behaviours **no test pins**. They hold by code reading and by probes (the
  advisor agent's and security-auditor's probes of ":" paths and of an unpushed local main were held); test-engineer
  wrote a scratch test for each, which passes on this head and fails on its copy, so each is a real gap in the
  tests, not in the script. They are follow-ups (his word of 18:01:08).
- **What I showed on older texts is history, not evidence about this head.** While the text changed I showed each
  new step failing on a broken copy of the text of that moment (16:22, 16:49, 17:13, 17:45). Two of those copies say
  nothing about this head: after the fix, moving the comparison before the plain-file check changes nothing, and
  the test's last step no longer catches a pattern read on main's side alone. The table above is the evidence.
- No flake in 14 full runs (5 one at a time, 8 four at a time). The kit's tests run in no CI job and not in the
  gate, so all kit evidence is local (Node 22.23.4, git 2.43.0).

**The replay of change-set 1's miss** (18:12:10 to 18:12:16; `replay-miss.sh` and its results in the notes): a
scratch clone with the remote branch at 55397c8, main at 22c6afa, the code commit 566d1d1 (three manifest and
lockfile changes) ahead and unpushed, and the notes of da22f96 on disk; then a foreground save.
- The script of this head: exit 1, nothing pushed, "the push would publish a dependency change with no recorded
  check: Cargo.lock, Cargo.toml, crates/protocol/Cargo.toml."
- Main's script on the same state: exit 0, "saved and pushed". That was the miss.

**CI, from real runs on this pull request** (the log lines are the job logs' own, read through GitHub's job-log
endpoint; the excerpts stand in `ci-evidence.md` in the notes):

| Head | What | Runs | Result |
|---|---|---|---|
| 94c197f | the clean tree, first run of the new lines | `toolchain` push 44357176860 (job 134988309631, 17:18:27 to 17:19:18); pull request 44357282401 | success; the log prints "Cache mode: none" (17:18:27.9099464Z) |
| 0c0e271 | the planted change | `toolchain` push 44357616602 (job 134989872637); pull request 44357633651 (job 134989932210) | failed at the compare step: "writing the generated types again changed:" and " M packages/types/src/generated/protocol/Join.ts" (17:21:20) |
| 415bd73 | the plant taken back (pushed after both plant runs had ended) | `toolchain` push 44357864275, pull request 44357881263; `gate` push 44357864589, pull request 44357881162 | all success |
| 4877599 | the fix | `toolchain` push 44365431438, pull request 44365446199; `gate` push 44365431235, pull request 44365445961 | all success |
| FINAL-HEAD | this report's head | PENDING | PENDING |

- **The stop rule** for the new CI lines (none had run on a runner before; if the clean run failed on one, or the log
  did not show "Cache mode: none": stop and tell him): not triggered.
- **The planted change** was one generated file committed with the executable bit (his yes, 16:39:23), tried locally
  first in a worktree of its own (16:52). It is the one difference that can reach the compare step today: the plain
  test run one line above fails first on a changed, missing, extra or non-plain file, and write mode rewrites the
  files with mode 0644.
- **What these runs prove:** the workflow with `cache-mode: none` is accepted and the runner receives that mode; git
  works inside the network cut; `SHAMBLE_WRITE_TYPES` reaches the tests and they rewrite the files; the compare step
  fails visibly; the two type-check lines pass on the clean tree.
- **What they do not prove:**
  - That the compare step catches a new untracked file, a deleted file or a file in a new subfolder: that rests on
    git's documented output, which infra-engineer showed on a scratch repository, not on a CI run.
  - That the type-check lines fail on a type error: not asked here; infra-engineer showed it locally (each run exits
    2 on a type error planted in its own files).
  - That the cache service refuses anything: no step of the job tries a cache operation, before or after the edit.
    The line closes the channel for later.
  - Neither CI check is a control against a hostile dependency: dependency code runs earlier in the same step as the
    same user and could silence both. They guard against honest mistakes.

## Reviews
One round, on the final text. PENDING: the verifier, the advisor agent.

- **security-auditor** (the kit hold, and a security read of the CI edits; 18:03:55 to about 18:31; about 85 probe
  runs in scratch repositories): nothing Critical or High; two Mediums (under "Open findings"), two Lows and wording.
  - What held in its probes: paths with ":" and pattern characters, spaces, quotes, a newline, non-ASCII names and
    letter case; renames, copies and merges; a link, a folder and a submodule; a mode change, an empty file and a
    65 MiB file; a file deleted and added again; a new branch; a missing `origin/main`; odd forms of the record;
    another task's slug; 45 saves racing; a branch that changes between the commit and the push.
  - Every git failure it injected ended held (seven places, and a hang killed after 5 s). Nothing fails open on an
    error.
  - For a push with no manifest change nothing changed against main's script (the outcome, the exit code, the skip
    mark, the secret scan, the saver's exits and its last save), with one exception: a new branch in a clone with
    no `origin/main`, or with unrelated history, is now held as unreadable.
  - The CI edits open nothing new. `cache-mode: none` closes the cache as a way from one run to another; the new
    lines run inside the cut on code that already runs there. What remains is CI-CUT part 2. One line it asked to
    have said: an earlier accepted Low of its review of change-set 1 assumed write mode ran only by hand; CI now
    runs it on every push, bounded by the throwaway machine and the cut.
  - Its Lows: a record that is a link or a folder still releases (only on purpose); with the git setting
    `push.followTags` on (unset here), a save would also push a tag on an in-between commit, and `toolchain` runs on
    tag pushes; wording (below).
  - Wording, where the rule line or the header says more or less than the code (the code errs towards holding in
    each): the message for an unreadable push advises writing a record, which cannot release that case (the way
    out is `git fetch origin main`); "as sha256sum prints it" is exactly `git show <commit>:<path> | sha256sum`
    (the script hashes the committed file; a path with a backslash or a newline never matches); "with main for a
    branch not yet pushed" is the point where the branch left `origin/main`; "never pushes" does not hold in the
    two corners of open finding 1; a link, a submodule and a file that became a folder can never be released.

- **test-engineer** (is it proven; 18:07:41 to about 18:39): proven for what is claimed, no High. Each of the six
  new tests fails on at least one broken copy of this head's script. Three Mediums, each a missing test (the three
  survivors in the table under "Evidence"); seven Lows (more missing tests; one test's look-alike file name pins
  neither end of the name pattern; the two saver tests delete their folder before they stop the saver, never seen to
  fail). It repeated the planted change in a worktree of its own: the clean tree is silent, the executable bit
  gives CI's line, and a content change fails one line earlier at the plain test run, so "a content change cannot
  reach the compare step" now rests on a run. Both of his asks on CI are proven as he worded them. Its warning,
  taken: the report must not call the three unpinned behaviours tested, nor use my older broken copies as evidence
  about this head.
- **infra-engineer** (the workflow edit; 17:47 to 18:07): APPROVE, no finding above Low. The text is byte for byte
  the report's blocks 1 to 3; nothing that stood is weakened (permissions, the order "fetch, then cut", `--locked`,
  `--offline`, the timeout). Six Lows, none a push: they are the "do not prove" lines above and four lines for the
  quality gate v2's list (below). Sources it read on 2026-11-18: GitHub's workflow syntax and dependency caching
  pages and its changelog of 2026-09-10 for `cache-mode`; TypeScript 6.0's release notes and compiler options for
  `--ignoreConfig`.

## Decisions and deviations
- **The chat's mode at the writes.** His words of 2026-11-13 ask for kit edits "with the chat in Accept edits". The
  session's record at 16:40 showed the mode "default", in which every edit asks him; I told him so before each
  write (16:55 and 17:48) and wrote in that mode. I did not read the mode again at the writes (17:15 and 18:01).
- **How the files were written.** Six copy commands from scratch copies that had been tested (four at 17:15, two at
  18:01), each checked afterwards: the files' diff equals the text he said yes to, hunk for hunk. His yes for each
  text is his chat message, not a prompt.
- **One read by the advisor agent, then a fix it had not read.** He asked for one read of the changed lines. That
  read gave a fix; the fix was written; the hole behind it needed a second fix, which he approved himself
  (KIT-FIX). The advisor agent reviews this report's head as it reviews every pushed head.
- **A refused command and the route taken** (16:51): the guard refused `git -c user.name=... commit` in the trial
  worktree ("no option before the subcommand"); route: a plain `git commit` with the repository's own identity
  (a local commit in that worktree, never pushed; the worktree is removed).
- **A stray scratch folder** (18:11:32): a replay script of mine made its scratch clone inside the repository's
  folder (`run-final-GbS2`, untracked); removed 20 seconds later; nothing of it was committed.
- **The saver** ran on main's script until 17:18, on the branch's first text until 18:03, on the fixed script since
  18:04; it was stopped and started again at each change. On the new script it has saved and pushed by itself.
- **Models:** this chat runs Fable 5.2 (checked 15:38 and 16:40). It wrote the kit edit under his DEP-HOLD ("my
  exception to the kit freeze, for this one change") and the two CI edits under his GATE-FOLLOW (b) with (i). The
  guard is not touched. Every agent ran on Opus 5.5 at effort max.
- **Change-set 2's plan** was drafted beside it by engine-architect (17:19 to about 18:09), as he allowed: a draft in the
  notes (`cs2-plan-draft.md`, 777 lines); nothing of it is built, no library is fetched, no manifest is touched. It
  is not part of this pull request's review. It is taken up from CHECKPOINT-COMMIT when change-set 2 starts.

## Not covered, and not verified
**Not covered by the hold, by his choice** ("Keep it to that: no guard change and no longer watch list now"):
- a plain `git push`, and a push through the GitHub tools;
- any other file that can bring code in (a toolchain file, a workflow's action pin, another package manager's
  lockfile);
- a check in CI (it goes on the quality gate v2's list).

**What a session could still do by accident** (security-auditor's list; no design here):
- push code by hand while a held checkpoint commit sits under it: the only sign is the script's one line, and for
  the saver a line in a background log;
- run the change before any push (`npm install`, `cargo build`, `npx`, `cargo install`): the hold only times the
  push;
- change files outside the five names that decide what CI fetches or runs (workflows, `rust-toolchain.toml`,
  `.cargo/config.toml`, `.npmrc`, `.gitmodules`, vendored source), or drop `--locked` or `npm ci` in a workflow;
- use another lockfile name or another letter case (not read by CI on Linux today);
- add and remove a dependency within one push: the commit in between goes out unheld (CI builds only pushed heads);
- rely on a stale view of the remote branch or of main;
- work on a branch from before this merge, or with a saver started before it: both run the old script.

**Known costs:**
- A manifest edit that changes no dependency (a description line) is held too: it takes a short answer of
  security-auditor first, then the record.
- A manifest that main removed stays held on a branch that takes main in (his KIT-REMOVED (a)).
- A link named like a manifest on main would stop every branch that takes main in; nothing can release it but its
  removal. None exists.
- While a push is held, the notes stay on this machine only.
- "Main" is `origin/main` as the clone last fetched it. A clone that has not fetched can hold a file that main has
  gained since, or pass content that main had at its last fetch (the advisor agent's probe).

**The script trusts the record.** It cannot tell who wrote it. The record and the answer it names travel in the same
push and stay in history; the review of the pull request reads them.

**Fail-safe corners:** a branch not yet pushed, in a clone with no `origin/main`, is held as unreadable. A true
record for a path that begins with ":" never matches (the line he kept reads such a path as a pattern), so such a
file stays held.

**By code reading and probes, not by a test** (test-engineer's list; the three in bold are its Mediums):
- **main's side of the comparison is read at the exact path** (the advisor agent's fix; security-auditor's ":"
  probes held);
- **a `deleted  <path>` line does not release a plain file that still exists;**
- **`origin/main` is compared, not a local `main`** (the advisor agent's probe: an unpushed local main is held);
- a plain file whose content equals a link's on main is held; a change of the file mode alone is not held when the
  content equals main's;
- "in any folder" for a manifest inside `.checkpoints/` and for a folder name that git quotes;
- "what cannot be read is held" in its one reachable case (an unpushed branch with no `origin/main`);
- for a branch not yet on the remote, the comparison with the point where it left main (not main's newer tip).

**His reason, "merged with my go":** it holds for every manifest on main today (they came in through #2, #5, #48
and #58). The PR Advisor's merge allow-list would let it merge a manifest under `docs/` or a Cargo file under
`tools/` without his go; no such file exists, and a guard change is not for this pull request (the advisor agent's
note).

**Not verified:** Windows and macOS (the tests ran on Linux, Node 22.23.4, git 2.43.0 here and git 2.56.1 on the
runner for CI); what the hold does in a clone with a case-insensitive file system.

## Follow-ups
**For the list of the quality gate v2's plan** (his words of 15:38:13 and 18:01:08):
1. A check in CI that refuses to build when the lockfile gains an entry with no recorded review (DEP-HOLD's option
   (c)). infra-engineer's line: first in every job that fetches, on push as well as pull request; it reads files
   only and runs nothing fetched.
2. CI-CUT part 2. infra-engineer's line: the job that runs the cut has no `uses:` step at all, so no post step runs
   after dependency code with the network and the token; a gate test fails if it gains one.
3. KIT-REMOVED: should a manifest that main removed also pass when a branch takes main in.
4. The line of the approved text that still reads a path beginning with ":" as a pattern
   (`const entry = git('ls-tree', rev, '--', f);`).
5. From infra-engineer: a guard for the case that `packages/types/src/generated` moves (the compare then checks
   nothing); a message when the type check's file list is empty; any workflow linter must accept `cache-mode`.
6. From security-auditor: the two open findings above; `push()` passes `--no-follow-tags`; the record is read
   only when it is a plain file; the wording lines (the exact hashing form, a fetch hint in the "could not be
   read" message, the branch point named, the entries that can never be released).
7. For his choice when that plan is written (security-auditor's list): hold plain pushes in the guard; a longer
   list of watched files; branches still on the old script; where held notes are kept.

**STATUS follow-ups** (their text: `docs/STATUS_ARCHIVE.md`, "Follow-ups"):
- security-auditor's two Mediums (under "Open findings"), its two Lows and its wording lines;
- test-engineer's ten lines: tests to add for the three unpinned behaviours (Medium) and for five smaller ones; the
  look-alike file name in test 29; the saver tests' clean-up order; and, for the quality gate v2's list, that the
  kit's four test files run in no CI job;
- infra-engineer's lines for the quality gate v2's list.

**Carried from #58**, unchanged: round 2's Lows of change-set 1; the two doc lines that claim more than the code
does (to be corrected in the next pull request that touches those files: this one does not).

## Risks
- The hold is a seatbelt in one script. A session that pushes by hand is not held; the rule line says so.
- A false hold costs time (the costs above), never safety.
- The kit lines changed three times in one afternoon (the text, his change, the fix). Each text has his yes and its
  tests; the review round and the verifier ran on the last one.

## Proposed next step
After the merge: change-set 2's plan goes to the advisor agent (the draft exists), with its four open questions for
Mustafa. Its first dependency step will be the first real use of the hold.

## The exact texts
His words (16:39:23): "Your ready report shows the changed lines". These are the lines that changed between the text
he answered on (`kit-edit.diff` at commit 3ebe654a, 262 lines, sha256 70756f3d...6ce5) and the files of this head:
75 added lines, none removed (the script 6, its tests 67, the rule line 2). They hold his one change, the advisor
agent's fix and the fix he said yes to as KIT-FIX.

KIT-CHANGE-DIFF

## Every line the docs pass moved (his OLD-LINES: "Its report lists each line it moved.")
PENDING

## Decisions needed
PENDING

## For Mustafa
PENDING
