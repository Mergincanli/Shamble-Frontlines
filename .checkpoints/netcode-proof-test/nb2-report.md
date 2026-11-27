# NB2, Netcode Builder 2's judge work (MOVED and U6j) merged into the netcode branch: the merge report
Merged on 2026-11-26 by the main session: merge 48efc9c59b06347c79d0e9ecbdcc78c6992eebde (parents H0 22cd65d7 and
Builder 2's tip 0e621dae6e5b009a4cf00b6401b46d771b2215a3), pushed in 23e9e92d. Plan nb2-merge-plan.md, approved with
conditions NB2-C1 to NB2-C12 (advisor-nb2-merge.md). The advisor agent read the pushed head (advisor-nb2-head.md: APPROVE
WITH FINDINGS, all Low). His word: history.md:7909 (2026-11-25 00:07 UTC, item 4); the prompt's terms, NB2-PROMPT (a):
history.md:7965; his "go" to the named command: history.md:8123-8131 (18:01:32 UTC). Written 2026-11-27, after
review-nb2-c10.md (NB2-C10) came in; it goes out with NB2-ONEWAY's report, in a separate block (NB2B-9).

## What it is
Builder 2's branch claude/practical-ride-igo41n, both parts in one merge (Builder 2's card, netcode-judge.md:2-6):
- **Part a, MOVED:** the judge and marks.json report a pass mark moved to the real game, or a moved part of one, as
  MOVED, never as PASS.
- **Part b, U6j:** the judge learns 19 figures and three waiting judge changes.
- **Size:** 46 files, +6,951/-208. 24 are spike files under crates/judge/ and marks/ (+4,712/-208); 22 are Builder 2's
  notes. 32 files are added, 14 modified.
- **Kit:** no manifest, lockfile, kit, CI or gate file. The guard's full pathspec over the range prints nothing.

## Evidence on the merged tree
- **The tree is exactly the two parents' merge:** 5565f253, the tree a fresh merge of the two parents gives.
  - Every spike file equals the tip's.
  - SERIES.md differs from the tip's by U6c's 51 added lines alone.
  - marks.json is the tip's (sha256 c3396db2…).
- **His first check, the whole check:** 11 of 11 steps; 908 and 1,005 tests, 0 failed, 0 skipped.
  - refs 2,037 = 2,005 + the 32 added files.
  - redact 431 = 421 + 10 items, 0 findings.
  - marks 81, rows unchanged; self 78 of 78; wasm as U8's run.
- **His second check, the PM-0001-47 re-judge on the real logs:** MOVED on both calls (measured and bites), exit 0, with
  the lines the plan check predicted. Both out files read "verdict": "MOVED".
- **CI on 23e9e92:**
  - gate and smoke: success.
  - toolchain: success on its one re-run. Attempt 1 failed while downloading a tool ("curl: (22)" with a 504 from
    GitHub's server), before any size or hash check.
  - Pull-request runs could not start then: PR #40 conflicted with main in docs/STATUS.md until MAIN-68. They ran on
    MAIN-68's pushed head 7971da0, which contains this merge, all green (advisor-main-68-head.md:100-104; `git
    merge-base --is-ancestor 48efc9c 7971da0` exit 0).
- **Four departures, each accepted by the head read** (advisor-nb2-head.md section 2):
  - step 4's bars re-based on U8's merge run;
  - step 8's two bars re-read ("mergeable" was out of NB2's reach; push runs only);
  - `-F` in place of three `-m`;
  - the toolchain's one re-run.

## Reviews: what covered each part (his rule of 2026-11-15, history.md:2125-2127)
- **Part a:** b22c30cd, read by test-engineer and rust-reviewer. Its fix pass 0e0f6547 had the advisor agent's read only.
- **Part b:** a3476e6c, read by both reviewers.
  - The fix pass 7c3855c7 with 67892d39 (marks.json's five lines; his 24 November item 4, quoted at
    advisor-nb2-merge.md:133-134): both reviewers' re-check on 67892d39, and the advisor agent.
- **72651a3c's last 67 lines:** the advisor agent's APPROVE, his main Advisor's check, and NB2-C10's reviewer read.
  - That read is the test-engineer's (review-nb2-c10.md): APPROVE WITH FINDINGS, one Medium and four Lows.
  - The test-engineer's re-check had proposed that guard.
- **The merge itself** changes no judge or marks.json byte (step 3's empty diff against the tip).

## Findings and where they go
- **M1 (Medium; review-nb2-c10.md): the reason text at figures.rs:722-724, as merged, is wrong.**
  - A session that left properly but whose first tick came late is told "the stop, not a leave".
  - The verdict (INVALID) is right; only the text is wrong.
  - Builder 2's lane: a forward fix, plan first, with a test for that path. It was handed to Mustafa to paste into
    Builder 2's chat (msg-2026-11-27.md, For Mustafa item 3).
  - Builder 2's later tip 29a90199 does not fix it (NB2B-8).
- **L1 to L4 (Low):** a lookup only half pinned by a test, the stop's phase unpinned, a typo and an unwrapped line.
  Builder 2's lane, as forward fixes.
- **ADV-NB2H-1 to -5 (Low, records):**
  - the gate output was not kept (CI's gate and smoke stand for it);
  - old plan lines stand under the As-approved header (this report uses the corrected words);
  - the pull-request CI gap is closed above;
  - NB2-C10 and NB2-C11 are closed by this report;
  - the -47 folder's pack waits until after NB2-ONEWAY's step 5 (NB2B-7).

## Not verified
- **PM-0001-57 was not re-judged (NB2-C11).** Its re-plan line ("-57's folder still PASS") is not runnable as worded.
  - Every kept set holds one profile, so the mark reads NOT-RUN and only its own part reads PASS.
  - His item 4 names only -47.
  - The follow-up, in the part-level form: one kept P3 set (291 MB), `--only PM-0001-57`, with "L1 P3: PASS" in the
    out file. It happens at PR #40's close verification, or in an earlier unit that judges a kept set anyway
    (advisor-nb2-merge.md:178-187).
- **The reviewer's "fails without it" claims were traced by hand.** review-nb2-c10.md ran no cargo and no plant. The
  plants named in 72651a3c's commit message are Builder 2's record.
- **Step 6's gate output was not kept** (ADV-NB2H-1).

## Decisions needed
None.

## For Mustafa
Nothing new. M1's paste block for Builder 2's chat went out in msg-2026-11-27.md, For Mustafa item 3.
