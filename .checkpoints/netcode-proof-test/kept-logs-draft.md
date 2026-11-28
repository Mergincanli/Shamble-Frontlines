# The kept-logs decision: a draft (the P0 ruling's step 5), 2026-11-14 about 23:15 UTC

Written by the main session for BRAWL-1's plan. The advisor's words (advisor-u4r-sets-r2-full.md, next step 5): "Draft
the kept-logs decision with BRAWL-1's plan: free space, the 5,385 MiB of kept logs, the refill figures, and options
within LEFTOVER-DELETE (a narrow kit rule, never a one-off delete). It goes to Mustafa under Decisions needed before the
re-run sets if the approved cleans cannot hold them."

Mustafa's rule (LEFTOVER-DELETE, 2026-11-09, verbatim in history.md): "leave them. 14 GB free is enough, and a one-off
delete past the deny list isn't a habit we start; if it's ever needed, it becomes a narrow kit rule instead." The
approved methods are `cargo clean --target-dir` and plain `git worktree remove` (never --force).

## 1. The disk today (df and du -sm, 23:09 UTC, nothing running)

- **Free:** 13,225 MiB.
- **The kept raw run logs** (.gate/netcode-proof-test/runs/, 7,380 MiB; never committed, plan §7.2; no approved method
  removes them):
  - U4r's sets: 5,385 MiB, made up of the pre-check 17, P3 519, B1 821, P2 779 and P0 3,249. Their results/ are
    committed and their rulings closed.
  - U2's two PM-0001-3 sets: 998 MiB each, 1,996 MiB together. Their results are committed.
- **Build caches** (cargo clean --target-dir removes them; each refills on the next build):
  - the shared target (agent copies and check.mjs runs): 1 MiB now. A full check.mjs refills about 9 GiB (its debug
    folder alone about 7.7 GiB);
  - the main folder's own cache (.gate/netcode-proof-test/target, the judged sets' release builds): 776 MiB.
- **Other leftovers that no approved method removes:**
  - .gate/u3c-review-target, 4,051 MiB: cargo refuses it, as it has no CACHEDIR.TAG;
  - the 23 merged copies that git refuses without --force, about 0.7 GiB;
  - builder and D4 evidence: .gate/u4r 887 MiB, .gate/u4r-d4 320 MiB, and smaller folders.

## 2. What BRAWL-1's lane and the re-run sets need

- **The build and review phase:** the builder's copy runs check.mjs in the shared target. That is about 9 GiB, so free
  falls to about 4.2 GiB, which is only just above the 4 GiB floor (brief section 4 rule 6: at least 4 GiB free before
  each build).
  - Nothing else fits in that phase: no 10-minute evidence run, about 0.5 GB at P0 and 0.2 GB at P3, and no reviewer's
    own build target.
- **After the merge:** (B) and (C) clean the shared target (both approved cleans), taking free back to about 13.2 GiB.
  Then come:
  - the two 10-minute evidence runs, about 0.7 GiB;
  - the main folder's release builds, about 0.8 to 1.0 GiB;
  - the re-run sets at U4r-F3's rates: P3 524, P2 781 and P0 3,615 MiB, about 4.9 GiB.
  - **Before P0:** about 13,225 - 700 - 1,000 - 524 - 781 = 10,220 MiB free. P0's start check needs 3,615 + 4,096 =
    7,711 MiB, so there is room.
  - **After P0:** about 6,600 MiB free.
- **The kept logs then:** 7,380 + about 5,600 (the evidence runs and the re-run sets) = about 13 GiB of raw logs. The
  next full refill of the shared target (U4f, U4g and U4h's builds) would then not fit: 6.6 GiB free against about
  9 GiB.

## 3. So: the approved cleans hold BRAWL-1's re-run sets, but not the units after them

- **For BRAWL-1:** the re-run sets fit with no delete, if the lane follows this order:
  1. build and review in the shared target, with no 10-minute run and no reviewer's own build target in that phase;
  2. merge;
  3. (B) and (C);
  4. the evidence runs, then the re-run sets, in the main folder's cache.
- **The risk:** the build phase runs at the floor, so one more refill (a second copy, or a reviewer's target) stops at
  the floor check. That is a stop, never a wrong result.
- **After BRAWL-1's sets:** U4g's judged B1 runs and the later units each need a shared-target refill (about 9 GiB)
  plus their own logs. With about 13 GiB of kept logs, they do not fit. So the decision is needed before U4g at the
  latest. If it is taken before BRAWL-1, BRAWL-1's build phase gains margin too.

## 4. Options within LEFTOVER-DELETE (for Mustafa, through the advisor)

**KEPT-LOGS.** Should a narrow kit rule let the session delete kept raw run logs whose results are committed and whose
rulings are closed?
- **Why it matters:** without it, the disk sits at its floor during BRAWL-1's build, and U4g's runs do not fit after
  BRAWL-1's re-run sets.
- **(a) Keep everything.** BRAWL-1 runs in the order of section 3. Before U4g, the question comes back with the same
  numbers.
- **(b) A narrow kit rule** (a `.claude/` change: tier 2, the advisor plus security-auditor, Mustafa's go in a day
  session). It allows one command form only: removing one run-set folder,
  `.gate/netcode-proof-test/runs/<set>-<commit12>/`.
  - **Conditions:**
    - its results/ folder is committed and pushed;
    - the set's ruling is closed in the notes;
    - its kept log under .gate/u4r-sets/ (the run lines) stays;
    - df before and after go on the card.
  - **What it frees:** U4r's 5,385 MiB and U2's 1,996 MiB.
  - **What is lost:** the raw logs, so those sets could no longer be judged again by hand. Their verdict files and
    runs.txt stay committed. Condition 5 already bars any re-run at those commits.
- **(c) The same rule, with an archive first.** Each set's raw logs are compressed into one file under .gate/ before the
  folder is removed. That keeps a re-judge possible. The advisor measured about 3.55 times smaller (one P3 run, zlib-6),
  so about 2.1 GiB of archives for the 7.4 GiB (advisor-brawl1-plan-full.md, condition 16, corrects this draft's earlier
  "5 to 10 times"). It costs the compression time (minutes), and the rule must name the archive step.
- **The advisor's ruling (Q7 (a)):** it goes to Mustafa now, and the lane does not wait. The advisor recommends (c), with
  the same rule also naming the old 4 GB build cache that cargo refuses (.gate/u3c-review-target, no evidence in it).
  The words for Mustafa are in advisor-brawl1-plan-full.md, "The KEPT-LOGS question".
- **The build-phase figure** is corrected too (condition 16): the measured full shared target is 8.2 GiB, so about 4.7
  GiB is free after the refill, not 4.2. Brief rule 6: 4 GB free before any wasm32 or release build, 2 GB before any
  other.
- **My recommendation:** (b), before BRAWL-1's build if Mustafa agrees, else before U4g.
  - The raw logs back no open ruling: U4r's and U2's results are committed and closed.
  - Condition 5 bars re-runs at those commits.
  - The kit rule names exactly one folder pattern, so it is not a habit of one-off deletes.
  - (c) is the choice if he wants re-judging kept possible.

## 5. Not in this draft

- Any delete itself: none happens before Mustafa's decision.
- .gate/u3c-review-target and the merged copies: LEFTOVER-DELETE already ruled that they stay. A rule for them would be
  a separate ask.
