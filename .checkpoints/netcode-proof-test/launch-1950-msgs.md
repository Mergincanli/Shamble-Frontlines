# The 19:50 UTC window (2026-11-26): the order and the messages, drafted 17:2x UTC by the main session
LEAN-MODE: new agents, and resumed agents with new work, only 19:50-20:20 UTC; at most 3 at once. This file governs
over the 19:51 wake-up's stored prompt (trig_01RkWH8Kv4KcuVUVmiKMH7uC, written before MAIN-68's rev 2).
- **Step 0, each its own call:**
  - `date -u`;
  - resetsAt (list_events, kinds rate_limit_event): the reset seen, and the next one read (probably 00:50 UTC);
  - `df -m .`; `git rev-parse HEAD` (fill `<head>`); `git status --short --branch`: clean, with no merge of mine in
    progress (else the launches wait for its push, inside 20:20);
  - `git ls-remote origin refs/heads/main`: 3b949c1a9f9c99d6d54570a0f2805c3d65661d8b. If main moved, MAIN-68's plan is
    brought up to the new tip first (R68-1's rule), and C waits for that;
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

## C. To the advisor a2b7636a57dff4206 (summary: "MAIN-68 rev 2's check")
Your one check of MAIN-68's plan rev 2 (main-68-plan.md, sha256 `<fill>`), brought up by your R68-1 to R68-8
(advisor-main-68.md). Main moved again after your verdict: #74 (ci-preview-handover), merged 15:24 UTC on his tap at
3b949c1a9f9c99d6d54570a0f2805c3d65661d8b. It changes .github/workflows/toolchain.yml (two steps that run on pull requests
only: the build check and the `preview-site` upload) and packages/e2e/playwright.config.ts (Firefox runs the replay
test), with a report. So the take-in now carries #70, #73 and #74 (45 paths, A 23, M 22), and it has two conflicts:
docs/STATUS.md, and docs/STATUS_ARCHIVE.md (both sides added a bullet under "## Open bugs and risks"). origin/main is
already fetched locally at 3b949c1: read by git, no fetch. Read only #73's and #74's halves and the new numbers (your
section 3 stands for #70's): the Facts, the two blocks (C6), R68-3 to R68-7 as applied, #74's step-6 bar and step 7's
M67-3 list. Answer its Q1-Q4. Write .checkpoints/netcode-proof-test/advisor-main-68b.md: APPROVE, APPROVE WITH CONDITIONS
(each worded to apply word for word) or REVISE. End your turn as soon as the verdict is in the file.

## A. To the advisor a0530735048f1620a (summary: "U7's merge plan check")
Your check of U7's merge plan (u7-merge-plan.md, sha256 `<fill>`), written in U8's M0-M9 form with your U7-M-1 to U7-M-8
(advisor-u7-snapshot.md section 4), as your item 5 asked ("else I bring it at 19:50"). One departure from U7-M-6: the
`ops` step is the first change after the merge, not inside it. The plan's Facts say why: M6's `git apply -R --index`,
and a commit equal to the snapshot. That change carries U8's fix 2 install. Check the plan against your conditions and
answer its Q1-Q3. Note also: main moved (#73, #74), and MAIN-68 (rev 2) takes them in before U7's merge (U7-M-1); after
it, #73's guard rules check every call. Write .checkpoints/netcode-proof-test/advisor-u7-merge.md: APPROVE, APPROVE WITH
CONDITIONS (each worded to apply word for word) or REVISE. End your turn as soon as the verdict is in the file.

## The third slot
- **If he answered U8F1-SIZE:** resume U8-F1's builder a7fe1b9f436f36bb1 (its worktree kept, detached at 77ffc95), with
  his answer word for word and what it changes in its stop and its items. Its first steps, each its own call:
  `git rev-parse --show-toplevel` (under .claude/worktrees/); `git rev-parse HEAD` =
  77ffc95101e186f75c57a1e8770b3ab7a9442820; `git status --short` shows only its own 1a and 1b changes and its notes.
  Then it builds on from u8-f1-build.md and u8-f1/ (the drafts), the brief as checked. If his answer is (d), the advisor
  re-rules C6 first, and the builder moves one window.
- **Else, if Builder 2's merge reached its step 9:** NB2-C10's reviewer read (advisor-nb2-merge.md:121-126).
- **Else** the slot stays empty.

## After C and A end (Builder 2's merge pushed 18:32 at 23e9e92, its merge 48efc9c; added 18:3x)
- NB2's step 9: the advisor's check of the pushed head by SHA (MAIN-65 step 7's form), with step 8's re-read bars
  (no conflict path beyond docs/STATUS.md and docs/STATUS_ARCHIVE.md; CI = the push runs) and the record in
  .gate/nb2-merge/ (step1-record.txt, check.log, the two rejudge-47 files). It takes the first slot that frees.
- NB2-C10's reviewer read (one reviewer agent on 72651a3c's last 67 judge lines: advisor-nb2-merge.md:121-126), in the
  next slot that frees, before NB2's report; then -47's pack.
