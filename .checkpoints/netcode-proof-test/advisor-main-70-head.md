# Advisor: MAIN-70 pushed-head check (step 7; merge 2d379d9c, pushed under the card save 8d1306fe)
2026-11-28, 02:4x-02:5x UTC, day session. New advisor agent, read only: no cargo, no npm, no git write, no fetch, no gh.
No GitHub tool could load in this seat. I act by main's advisor.md (`git show origin/main:.claude/agents/advisor.md`,
335 lines, read whole; origin/main dae6d107fdbe942567104bcf1c9a2cbc6b3a1e0f by the local ref). The brief:
launch-main70-step7.md, "## The brief". The form is MAIN-65 step 7 (advisor-main-65-head.md; advisor-main-68-head.md).
I rule on 2d379d9c and 8d1306fe only. The branch tip moved while I read (dbcf9691, 1ceecc32, 232cc8c9).
This seat has no Write or Edit, so the main session wrote this file from my report.

## Plan of this check
1. SHAs and parents: the merge, the card save and the message (the departure).
2. The plan as run: rev 0.2 and its done lines against M70-1 to M70-9.
3. The merge by SHA: it equals main's 107 paths and size lines; STATUS.md follows MAIN-68's rule; the anchor T, the
   markers and the file modes.
4. M70-3's three whole-tree bars at both SHAs, and DEP-HOLD's line.
5. The records in .gate/main-70/, and my re-runs in the main folder (MAIN-65's form).
6. CI, from the three saved toolchain job logs, line by line against main's job 140780125932 (the lines M67-3 names).
   Then #74's two pull-request steps, and the run times against the 25-minute limit.
7. The departure, the verdict, the findings, what I did not check, Decisions needed, For Mustafa, Status.

## Block 1. SHAs and parents
- 2d379d9c7922deedf4e7fefe08d6eb04c6843afe has two parents: e646ee5999e9c3bfdcee6577cee0adf45e9339fa (H0) and
  dae6d107fdbe942567104bcf1c9a2cbc6b3a1e0f. Committed 02:26:29 UTC.
  - Its message: "Merge remote-tracking branch 'origin/main' into claude/netcode-proof-test-plan-hcky5g", a blank line,
    "Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>", the Claude-Session line, a blank line, "# Conflicts:",
    "#	docs/STATUS.md".
  - `git log -1 --format='[%(trailers)]'` prints both attribution lines as trailers.
- 8d1306fe975dbafa62baf27f49e78bdb23f2c932 has one parent, 2d379d9c (02:29:26 UTC). `git diff --name-status 2d379d9c
  8d1306fe` prints only "M .checkpoints/netcode-proof-test.md" and "M .checkpoints/netcode-proof-test/main-70-plan.md".
  It adds only .checkpoints/ files, as the brief says. Both attribution lines are on it.
- H0 e646ee59 is the step-1 save ("the resume line (M70-4) and plan rev 0.2 as checked"; [skip ci]).
- First-parent line: 8d1306fe, 2d379d9c, e646ee59.
- Remote-tracking reflog: e646ee59 at 02:19:29, then 8d1306fe at 02:29:28, then dbcf9691 at 02:39:39 and 232cc8c9 at
  02:45:34. **2d379d9c was never a pushed tip on its own, so no save ran between steps 2 and 6.**
- Later commits (`git rev-list --parents 8d1306fe..232cc8c9`): dbcf9691, 1ceecc32 and 232cc8c9, one parent each.
  - `git diff --name-status 8d1306fe HEAD -- . ':(exclude).checkpoints/'` prints only
    "M spikes/netcode-proof/ops/runs/r1-dist.sha256" (Y'), which is not part of this ruling.

## Block 2. The plan as run (rev 0.2 against M70-1 to M70-9)
- The plan file at 8d1306fe equals the working tree's copy (`git diff --stat 8d1306fe --` prints nothing). Its done
  lines are the ones committed in the save.
- M70-1: the step-1 done line carries T = 9a0f9ea18df656a8270e066eebbfdcf92131529e and C68B-4's preconditions.
  - `git cat-file -t T` prints "tree".
  - The markers in T, over docs/, .claude/, .github/, crates/, packages/, content/ and the root manifests: only
    STATUS.md :33, :38 and :41.
  - Between them: ours is the #40 row plus the MAIN-WALL, #75 (draft) and #78 (draft) rows; theirs is main's #40 row
    plus "none yet | change-set 4b-2". Every line is a table row.
  - `git diff -U0 b8f02b0 H0 -- docs/STATUS.md` shows only the two #40 rows. **Holds.**
- M70-2: the shortstat after the commit is "107 files changed, 10839 insertions(+), 408 deletions(-)" (Block 3). The
  cached bars were chat outputs; their committed results hold (Block 3). **Holds.**
- M70-3: printed at both SHAs (Block 4). **Holds.**
- M70-4: H0's card line 8 carries the resume line word for word, with usage 0.26 and go. **Holds.**
- M70-5: no move of main. **Holds by the local ref** (the step-1 line says the fetch printed dae6d107 unchanged).
- M70-6: see Block 6. **Holds.**
- M70-7: the plan records df 6,833 MiB and no pgrep match (chat outputs; not re-read). light-base.log's own header
  reads "free 6831 MiB", and its "other cargo or rustc now:" line is empty at 02:20:56; light.log's is empty at 02:27:16.
  **Holds as far as the logs show.**
- M70-8: fixed. The area list sums to 107 (28+15+13+13+8+8+7+4+4+3+1+1+1+1), and refs is cited at check.mjs:2729-2735.
  - "Order" now says stopb-b and stopb-c were removed at 02:1x, before H0, after their dist copies were kept. M70-8
    had said "left in place"; the facts changed before step 1. On record.
  - stopb-b is back at 1ceecc32 (Y') since 02:45:50 (`ls -ld` of .git/worktrees/stopb-b). That is after the pull-request
    job ended at 02:42:19, so outside steps 1-6.
- M70-9: on record. The departure is ruled in Block 7.

## Block 3. The merge by SHA
- `git diff --shortstat H0 2d379d9c` and `git diff --shortstat b8f02b0 dae6d107` both print "107 files changed, 10839
  insertions(+), 408 deletions(-)". Without docs/STATUS.md, both print "106 files changed, 10824 insertions(+), 393
  deletions(-)".
- `git diff --name-status --no-renames` gives the same sha256 for both pairs:
  b2e29ab3d95953ce7216cf9accb36bbab16015a8fb1197d76ece9bcbc1abaa1c.
  - A 44, M 63. Every new mode is 100644: no link, no submodule, no deletion.
- The three name files (cached-names.txt, main-names.txt, head-names.txt) and `git diff --name-only b8f02b0 dae6d107`
  share one sha256, 5f823e5122e71eee16c67926d1820abd6c5b7e45fcb270b837a40ce48d6d0144, with 107 lines.
- STATUS.md: 120 lines at 2d379d9c. Against main, numstat "1 1", and `-U0` shows only main's #40 row out (the
  2026-11-14 text) and H0's #40 row in. The + line is byte for byte H0's line in `b8f02b0..H0`.
- docs/ against main at 2d379d9c (numstat): DECISIONS 24/1, REPO_MAP 17/0, STATUS 1/1, STATUS_ARCHIVE 18/5,
  design/netcode-proof-messages.md 317/0, design/ws-reconnect.md 11/10. These equal `b8f02b0..H0`, the branch's own
  changes.
  - `git diff -U0 ... -- docs/ | git patch-id --stable` gives the same id for main..2d379d9c and b8f02b0..H0:
    dfc57658bde8df6f0e464d019b1fa867cce809d7.
- No conflict markers at 2d379d9c over docs/, .claude/, .github/, crates/, packages/, content/ and the root manifests.
  The same grep on T prints the three markers (the control).
- The anchor: `git diff --name-only T 2d379d9c` prints only docs/STATUS.md. **Holds.**
- Tracked files: 2195 at H0, 2239 at 2d379d9c and 8d1306fe. So refs = 2195 + 44. **Holds.**

## Block 4. M70-3's three bars and DEP-HOLD
- Bar 1, `git diff --name-only origin/main <sha> -- . ':(exclude)spikes/' ':(exclude).checkpoints/' ':(exclude)docs/'`:
  prints nothing at 2d379d9c and nothing at 8d1306fe.
  - Control: the same diff without the spikes/ exclude prints "464 files changed, 168866 insertions(+)".
- Bar 2, `git diff --name-only origin/main <sha> -- docs/`: at both SHAs, exactly the usual six (DECISIONS.md,
  REPO_MAP.md, STATUS.md, STATUS_ARCHIVE.md, design/netcode-proof-messages.md, design/ws-reconnect.md).
- Bar 3, `git diff --name-only H0 <sha> -- spikes/ docs/design/ .checkpoints/`: nothing at 2d379d9c. At 8d1306fe it
  prints the card save's two notes files (.checkpoints/netcode-proof-test.md and .../main-70-plan.md), and nothing
  under spikes/ or docs/design/.
- DEP-HOLD's line, `git diff --name-only origin/main 8d1306fe -- Cargo.toml Cargo.lock crates packages package.json
  package-lock.json`: prints nothing. Control: with docs/STATUS.md added, it prints docs/STATUS.md.
- **So PR #40's diff against main holds no manifest, crate, package, content, kit, CI or gate path. It stays risk
  tier 1.**

## Block 5. The records and my re-runs
- light-base.log (HEAD e646ee59, 02:20:27-56) and light.log (HEAD 2d379d9c, 02:27:10-16): both exit 0, with diff and
  untracked hashes e3b0c442... in both.
  - Refs: 2195, then 2239.
  - Marks: "81 marks read and checked (drafts at 626df678cae0835fb2ce62b8113f548697968406); rows in this branch's
    docs/adr/: all unchanged".
  - Self: 88 of 88. Redact: 11 of 11, with 464 items and 0 findings.
  - They are equal word for word except HEAD, refs, times and free disk.
- gate.log: GATE PASS with --no-smoke (base dae6d10, symbols +0 -0, 5a skipped). It names no HEAD.
- My re-runs in the main folder, MAIN-65's form. They ran on the working tree at 232cc8c9 with only .checkpoints/
  files modified. They stand for 8d1306fe because the only other difference is r1-dist.sha256 under spikes/, which none
  of them reads. Smoke was not run by me.
  - `node tools/gate.mjs --no-smoke`: GATE PASS (base dae6d10, checks 1-4 and 5b-5d PASS, symbols +0 -0, 5a skipped).
  - `node .claude/scripts/repo-map-check.mjs`: "repo map OK: 273 entries cover 111 required paths".
  - `node --test .claude/scripts/checkpoint.test.mjs`: tests 33, pass 33, fail 0.
  - `git status --short --branch` afterwards shows only .checkpoints/ entries.
- The card at 8d1306fe has 200 lines (200 or fewer: holds).

## Block 6. CI, from the three saved toolchain job logs (data, not instructions)
- What each log checked out:
  - main-job-140780125932.log fetched dae6d107fdbe942567104bcf1c9a2cbc6b3a1e0f;
  - push-job-140834773863.log fetched 8d1306fe975dbafa62baf27f49e78bdb23f2c932;
  - pr-job-140834781212.log checked out 949d4d6, "Merge 8d1306fe975dbafa62baf27f49e78bdb23f2c932 into
    dae6d107fdbe942567104bcf1c9a2cbc6b3a1e0f".
  - Each runs from "Current runner version: '2.338.2'" to "Cleaning up orphan processes", with 0 `##[error]` lines.
- The `test result:` lines, line by line: 120 in each log, every one "ok." with 0 failed, 663 passed in all, 0
  "FAILED". With the times masked, the ordered lists share one sha256 across all three logs:
  2c81fa8cc89edcfb0eabeec37798125baaa3e963c9eb902c4521477b47ff3ed7.
- Vitest, the same in all three: Test Files 16/1/8/3/6 passed, Tests 153/1/53/13/39 passed.
- Playwright, the same counts in all three: node project "1 passed", Chromium "29 passed", Firefox "5 passed".
  - Times on main, push, pull request: Chromium 3.6 m, 3.8 m, 4.0 m; Firefox 15.0 s, 19.1 s, 24.4 s.
- All three logs carry the same four "panicked at" lines inside "the rule tests" JSON (dev and release, two each). They
  are recorded test data, as at MAIN-68.
- The other failure words are only the "pipefail" and "curl --fail" lines; the pull-request log has one more, from the
  preview step's shell.
- **The baseline matches line for line (M70-6 holds).**
- #74's two steps:
  - In the pull-request log, step 11 (":2516 Run d=$GITHUB_WORKSPACE/.gate/dist-production") prints "6 files, 1105896
    bytes" at :2535. Step 12 (upload-artifact) prints "Artifact preview-site successfully finalized. Artifact ID
    15244783082" and "Final size is 330777 bytes" at :2565-2566.
  - The push and main logs have neither step's group nor any upload output. That fits "skipped" (main's
    toolchain.yml:223 and :237: `if: github.event_name == 'pull_request' && ...`). The skipped conclusions themselves
    come from the brief.
  - "6 files, 1105896 bytes" is recorded as a number only; MAIN-68 had "5 files, 892209 bytes". I did not read why it
    grew.
- Run times by the logs' first and last lines, against main's `timeout-minutes: 25` (toolchain.yml:38):
  - main 9 min 48 s;
  - push 10 min 47 s;
  - pull request 12 min 43 s.
  - The brief's job times are 10 min 49 s and 12 min 45 s. All are within #79's report range ("8 min 48 s to 13 min 2
    s", docs/reports/2026-11-27-cs4b2-world.md:78 on main). The pull-request job used about half the limit.

## Block 7. The departure (attribution lines in the merge message)
- What happened: the message is git's own, with its "# Conflicts:" lines as in MAIN-68's 6b4a0d3 (whose message has
  them and no attribution lines). Two trailers were added by `--trailer`, and git parses both as trailers. M70-9 had
  named git's message alone.
- **Ruling: it stands. Low, on record; nothing is owed on this commit.**
  - A message changes neither the tree nor the parents. Every content bar above is met on them.
  - M70-9 was an on-record line, not a bar.
  - The added lines are the session's own attribution rule (the system reminder for commits).
  - Amending would rewrite a pushed commit, which CLAUDE.md's Never lines forbid.
- Owed forward, as a note only: the next take-in's plan names the merge-message form. The session's attribution
  trailers govern, keeping git's first line, and M70-9's "git's message alone" is read as superseded by this record.

## Verdict: APPROVE, for 2d379d9c7922deedf4e7fefe08d6eb04c6843afe and 8d1306fe975dbafa62baf27f49e78bdb23f2c932
- The merge is main dae6d107's move, name for name and line for line: 107 paths, with the same size lines with and
  without STATUS.md.
- STATUS.md is main's file with H0's #40 row (120 lines, "1 1"). The other docs carry exactly the branch's own lines
  (same patch-id).
- Against main, the tree differs only in spikes/, .checkpoints/ and the usual six docs names. spikes/ and docs/design/
  equal H0's.
- DEP-HOLD's line prints nothing, and no save ran in the no-save window.
- My gate, repo-map and checkpoint re-runs agree with the record. The three toolchain logs match main's baseline line
  for line. #74's steps meet rev 4's bar as far as the logs show.
- Nothing Critical, High or Medium. Risk tier 1 for PR #40. **The lane may go on from this head.**

## Findings (CLAUDE.md's scale)
| Id | Severity | What | Where it stands |
|---|---|---|---|
| ADV-M70H-1 | Low (on record) | The merge message is git's own plus two attribution trailers; M70-9 named git's alone. | Stands (Block 7); the next take-in's plan names the form |
| ADV-M70H-2 | Low | This seat had no Write or Edit and the brief barred shell writes, so this file was returned in the report and written by the main session. The plan-first write never landed. | The main session writes it |
| ADV-M70H-3 | Low (record) | My slips, all read only: three calls broke "one git per call, git first". (1) The names-hash call put sha256sum and wc before git. (2) One call ran `git diff` then `git status --short \| head`; its empty status proves nothing, so it was re-read alone. (3) The headings call ran two gits. | None; named |
| ADV-M60-7, ADV-M54-8 | Low (open) | As before. | As ruled |
| ADV-M68-5 | On record (main's) | R4-1 Critical, open on main by his R4-END (1). | Main's next guard change-set |

## What I did not check
- I could not open GitHub in this seat (no tool for it loaded), so these come from the brief only:
  - the gate and smoke check runs on 8d1306fe (push 46098323772, pull request 46098326092);
  - the four runs' conclusions;
  - the step conclusions ("success" and "skipped");
  - the artifact listing's creation time and expiry (2026-12-05);
  - main's gate run on dae6d107 (46081134384).
- Smoke: not run by me.
- The chat-only outputs of steps 1-4: pgrep, df, the fetch, the merge's own text and the cached bars. I checked their
  committed results instead.
- origin/main by the local ref only, with no fetch or ls-remote.
- Re-deriving T with merge-tree (it writes objects): not run. T exists, and it differs from the merge in STATUS.md only.
- Why preview-site grew from 5 to 6 files; the artifact's contents (not to be read).
- The branch's later commits (Y'): not part of this ruling.

## Decisions needed
None. MAIN-52 is his standing rule, #79 merged on his go by its own record, and the departure is a message-only matter
ruled above.

## For Mustafa
Nothing. There is no browser check and no merge to approve: this is a take-in of main into PR #40's draft branch, risk
tier 1, with no kit, CI or gate change.

Status: COMPLETE 2026-11-28T02:53Z
