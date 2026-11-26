# The 19:50 UTC window (2026-11-26): the order and the messages, drafted 17:2x UTC by the main session; brought up 18:5x-19:0x UTC (main 2671595, MAIN-68 rev 3; the three remaining messages written out)
LEAN-MODE: new agents, and resumed agents with new work, only 19:50-20:20 UTC; at most 3 at once. This file governs
over the 19:51 wake-up's stored prompt (trig_01RkWH8Kv4KcuVUVmiKMH7uC, written before MAIN-68's rev 2).
- **Step 0, each its own call:**
  - `date -u`;
  - resetsAt (list_events, kinds rate_limit_event): the reset seen, and the next one read (probably 00:50 UTC);
  - `df -m .`; `git rev-parse HEAD` (fill `<head>`); `git status --short --branch`: clean, with no merge of mine in
    progress (else the launches wait for its push, inside 20:20);
  - `git ls-remote origin refs/heads/main`: 2671595fc823750a83c58f5b5b122128eb500387 (#76, read 18:4x; rev 3 is on it).
    If main moved again, MAIN-68's plan is brought up to the new tip first (R68-1's rule), and C waits for that;
  - `sha256sum` of main-68-plan.md and u7-merge-plan.md, filled into C's and A's messages.
- **Order:** C and A together at 19:5x. The third slot, in this order: U8-F1's builder (resume a7fe1b9f436f36bb1) only if
  he has answered U8F1-SIZE; else NB2-C10's reviewer if Builder 2's merge reached its step 9; else empty. After the
  launches: save the card, and set the next window's wake-up from resetsAt.
- **Every message carries, word for word:** "git and gh always start their own command, with no other program in front
  of them." and "Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never
  `cargo fmt` writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with
  Write." C and A: read only, except their verdict files; no cargo; no commits (the main session saves); never switch
  the main folder; never `cd` into another worktree; one simple command per call; no `git -C`, no `git -c`, no shell
  loops.

## C. To the advisor a2b7636a57dff4206 (summary: "MAIN-68 rev 3's check")
Your one check of MAIN-68's plan rev 3 (main-68-plan.md, sha256 `<fill>`), brought up by your R68-1 to R68-8
(advisor-main-68.md). Main moved twice after your verdict:
- #74 (ci-preview-handover), merged 15:24 UTC on his tap at 3b949c1a9f9c99d6d54570a0f2805c3d65661d8b. It changes
  .github/workflows/toolchain.yml (two steps that run on pull requests only: the build check and the `preview-site`
  upload) and packages/e2e/playwright.config.ts (Firefox runs the replay test), with a report.
- #76 (MAIN-WALL's record), merged 18:00 UTC on his go at 2671595fc823750a83c58f5b5b122128eb500387: 8 docs .md paths
  (DECISIONS, REPO_MAP, ROADMAP, STATUS, STATUS_ARCHIVE and TECHNICAL modified, one report modified, one added), no kit,
  CI, code, spike, ADR or design path. The wall itself is a GitHub setting on main (his click, about 17:15 UTC).

So the take-in now carries #70, #73, #74 and #76 (47 paths, A 24, M 23). It has two conflicts:
- docs/STATUS.md: the #40 rows, ci-after-3a's row, and #76's MAIN-WALL row;
- docs/STATUS_ARCHIVE.md: both sides added under "## Open bugs and risks"; main's side is now a MAIN-WALL bullet, then
  the guard-wrappers bullet.

origin/main is already fetched locally at 2671595: read it by git, with no fetch.

Your section 3 stands for #70's half. Read only #73's, #74's and #76's halves and the new numbers:
- the Facts and the two blocks (C6);
- R68-3 to R68-7 as applied;
- #74's step-6 bar and step 7's M67-3 list;
- #76's lines.

The plan's "What rev 3 changes" list is relative to rev 2, which you have not read: use it as an index only.

Answer its Q1-Q5, and give Q3's answer as a condition worded to apply word for word. Write
.checkpoints/netcode-proof-test/advisor-main-68b.md: APPROVE, APPROVE WITH CONDITIONS (each worded to apply word for
word) or REVISE. End your turn as soon as the verdict is in the file.

## A. To the advisor a0530735048f1620a (summary: "U7's merge plan check")
Your check of U7's merge plan (u7-merge-plan.md, sha256 `<fill>`), written in U8's M0-M9 form with your U7-M-1 to U7-M-8
(advisor-u7-snapshot.md section 4), as your item 5 asked ("else I bring it at 19:50"). One departure from U7-M-6: the
`ops` step is the first change after the merge, not inside it. The plan's Facts say why: M6's `git apply -R --index`,
and a commit equal to the snapshot. That change carries U8's fix 2 install. Check the plan against your conditions and
answer its Q1-Q3. Note also: main moved (#73, #74, #76), and MAIN-68 (rev 3, on main 2671595) takes them in before U7's
merge (U7-M-1); after it, #73's guard rules check every call. Builder 2's merge came first (48efc9c, 18:02 UTC). Write .checkpoints/netcode-proof-test/advisor-u7-merge.md: APPROVE, APPROVE WITH
CONDITIONS (each worded to apply word for word) or REVISE. End your turn as soon as the verdict is in the file.

## The third slot: U8-F1's builder a7fe1b9f436f36bb1 (he answered (a) at 17:35; summary "U8-F1: resume on his (a)")
His answer to U8F1-SIZE, word for word (2026-11-26 17:35 UTC, history.md:8108-8121): "8F1-SIZE before 19:50 UTC if you
want its builder to resume in that window;  (a)." Option (a) was your first priced option: a new stop over the whole,
6,282 (measured: built +26, drafts +67). So U8's stop is 6,282 and U8-F1 adds at most 93 net lines, tests included.
Everything is built as drafted, with nothing carried, and a build past 6,282 stops again with three lines (what grew,
why, the cheaper option). The brief's section 0 now says so (/home/user/Shamble-Frontlines/.checkpoints/
netcode-proof-test/u8-f1-brief.md:12-18, the main session's Edit, read only: "The 41 are net" now reads "The 93 are
net"). Nothing else in the brief changed; C1-C7 as applied stand.

First, each its own call, in your own worktree:
- `git rev-parse --show-toplevel` (under .claude/worktrees/);
- `git rev-parse HEAD` = 77ffc95101e186f75c57a1e8770b3ab7a9442820;
- `git status --short` shows only your 1a and 1b changes and your notes.

Any other answer: stop and report it, changing nothing.

Then build on from your u8-f1-build.md and the u8-f1/ drafts (drafts.md and its scripts), the brief as checked: 1c, 1d,
1e with its pin test, then items 2 and 3, each with its test passing before the next. The ops tests run as
`node --test 'ops/**/*.test.mjs'` (Node 22.23 fails on `node --test ops/`).

The rules:
- No cargo. No commits: the main session takes the snapshot.
- Never touch the main folder (/home/user/Shamble-Frontlines): a merge may run there in this window. Read only, its
  brief file excepted. Never `cd` into another worktree.
- One simple command per call; no `git -C`, no `git -c`, no shell loops.
- git and gh always start their own command, with no other program in front of them.
- Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt`
  writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write.

The hand-back, in u8-f1-build.md:
- per file, added, removed and net (the brief's numstat form), against 93;
- the tests, the plants and the redact check;
- anything carried, and what is open.

End your turn when it is in the file.

## Step 9: to the advisor a10848a10f73ed39e (NB2's plan checker; summary "NB2's pushed head"), in the first slot that frees
NB2-MERGE's step 9: your check of the pushed head by SHA, in MAIN-65 step 7's form (as advisor-main-67-head.md did for
MAIN-67).

The merge and its push:
- The merge is 48efc9c59b06347c79d0e9ecbdcc78c6992eebde, with parents H0 22cd65d7d32f654742753867712883910429722f and
  T 0e621dae6e5b009a4cf00b6401b46d771b2215a3. It was made 18:02-18:05 UTC on his go of 18:01 (history.md:8123-8131),
  with the command named to him first (step1-record.txt:21-23).
- It was pushed in the card save 23e9e92d11a8bc78f7aa9d0a2862790f1e7d0736 at 18:32.
- The branch head at launch is `<head>`. Since 23e9e92 there are notes saves only: `git diff --name-only 23e9e92
  <head>` names only .checkpoints/ files.

The record, in .gate/nb2-merge/:
- step1-record.txt and merge-msg.txt;
- light-base.log;
- check.log: 11 of 11; 908 and 1005 tests; refs 2037; redact 431 items, 0 findings;
- rejudge-47-measured and rejudge-47-bites (.log and .json): PM-0001-47 reads MOVED in both.

Step 8's two bars, as re-worded with the advisor tool at 15:1x:
- (a) No conflict path beyond docs/STATUS.md and docs/STATUS_ARCHIVE.md. At 18:5x, against main 2671595 (fetched
  18:4x), `git merge-tree --write-tree --name-only` with the head names only those two; MAIN-68 resolves them.
- (b) CI is the push runs on 23e9e92:
  - gate 45876736547, jobs gate 140105605763 and smoke 140105605489: success;
  - toolchain 45876736642: attempt 1 failed at the wasm-bindgen archive fetch (GitHub's HTTP 504). Its one re-run
    (PR #40's rule), attempt 2 with job 140106992032, succeeded at 18:40.

Check NB2-C1 to NB2-C12 as met on the pushed head.

How to read:
- By SHA only (git show, diff and log of SHAs): MAIN-68 may be running in the main folder while you read, so never read
  its working tree for these files.
- No fetch, no cargo, no tests.
- One simple command per call; no `git -C`, no `git -c`, no shell loops.
- git and gh always start their own command, with no other program in front of them.
- Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt` writing
  files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write.

Write .checkpoints/netcode-proof-test/advisor-nb2-head.md: APPROVE, APPROVE WITH FINDINGS or REVISE, each finding with
its severity. End your turn as soon as the verdict is in the file.

## NB2-C10: a new test-engineer (opus, effort xhigh, read only, no worktree; summary "NB2-C10 judge read"), next slot
NB2-C10 (advisor-nb2-merge.md:121-139) asks for one read-only reviewer's read of 72651a3c's last judge lines, before
NB2-MERGE's report. It does so under Mustafa's rule of 2026-11-15 (history.md:2127): every marks.json or judge change
gets a reviewer's read before its unit closes.

Read `git diff 67892d39 72651a3c -- spikes/netcode-proof/crates/judge spikes/netcode-proof/marks`: 67 lines, namely
figures.rs's no-tail guard, two engine tests, and README and SERIES text. Read as much of the judge around it as you need,
by `git show 72651a3c:<path>`. These lines are merged unchanged at 48efc9c: the merge changes no judge or marks.json byte.

Answer, by reading:
- Is the guard right?
- Would each new test fail without it?
- Does any README or SERIES line claim more than the code does?
- Can anything here make a verdict wrong?

How to read:
- Read only: no edits but your file, no cargo, no tests run, no fetch.
- By SHA only: a merge may be running in the main folder.
- One simple command per call; no `git -C`, no `git -c`, no shell loops.
- git and gh always start their own command, with no other program in front of them.
- Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt` writing
  files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write.

Write .checkpoints/netcode-proof-test/review-nb2-c10.md: APPROVE, APPROVE WITH FINDINGS or REVISE, each finding with its
severity (CLAUDE.md's scale) and file:line. End your turn when the file is written.

## Order after the launches (the advisor tool, 18:5x: keep the gap between C's verdict and the merge short)
- C's verdict lands: MAIN-68's steps 1-6 start at once, its conditions applied first by Edit, word for word. The two
  launches below are one call each and do not wait on the merge; each reads by SHA only.
- The first slot that frees (likely A's) gets step 9's message; the next (likely C's) gets NB2-C10's. Both launch only
  before 20:20 UTC (LEAN-MODE); a launch that misses 20:20 waits for the 00:50 window.
- -47's pack after NB2's report.
