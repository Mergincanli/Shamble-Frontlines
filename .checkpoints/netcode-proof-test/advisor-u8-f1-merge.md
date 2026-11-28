# Advisor: U8-F1's merge plan (u8-f1-merge-plan.md), checked 2026-11-27

Plan: read main's advisor.md; the merge plan whole; the patch's hash; u8-f1-build.md's fix-pass HAND-BACK;
u7-merge-plan.md's M5 done line; advisor-u7-head.md sections 3 and 6 and "Item 6, re-ruled"; u8f1-m4-cmp.mjs. Then:
1 the verdict, 2 Q1 (M5's bars, figure by figure), 3 Q2 (the two carried Lows and the stop), 4 Q3 (no C1, no M2b,
M3a kept), 5 Q4 (u8-report.md's section); Decisions needed, For Mustafa, the status line. Written block by block.

**Read:**
- main's advisor.md (unchanged since c9bd268);
- u8-f1-merge-plan.md (147 lines);
- wip/u8-f1-build.patch, sha256 4d6a2484...f7164 (matches M1);
- u8-f1-build.md:389-end (the fix pass's HAND-BACK, "Status: COMPLETE 2026-11-27T01:14:16Z");
- u7-merge-plan.md:208-213 (U7's M5);
- advisor-u7-head.md:48-62 and :177-235;
- u8f1-m4-cmp.mjs (32 lines: read only, no git, exit 0 only for 6 paths byte-equal at mode 644);
- check.mjs:2695-2719, the refs and redact steps;
- u8-report.md:57-75.

## 1. Verdict: APPROVE WITH CONDITIONS (C1-C3)
The form is U7's M0-M9 with M3a and M6's refreshed undo, as advisor-u7-head.md section 3 words them. Its six-file
patch matches the fix pass's HAND-BACK: 105 added, 12 removed, +93; the six files and their restore hashes.
- M2's "unchanged since 77ffc95" check, M4's byte and mode compare, and M8's numstat equality tie the commit to the
  tree the builder checked.
- Two bars in M5 would force M6's undo on a legitimate move (C1).
- The two carried Lows meet a stop question that is his (C2, and U8-FILLS below).
- u8-report.md's section lacks a few lines (C3).
- The re-check at sending holds: `git diff --quiet 46c964f HEAD -- spikes/` exit 0 keeps the 01:2x apply check good on
  a5f8498, and M0's part-clean comes first at 7,796 MiB.

## 2. Q1: M5's bars, figure by figure
The rule: a figure that reads only spikes/ content takes U7's M5 count strictly, because M0's `git diff --quiet
6968215 H0 -- spikes/` (exit 0) makes that content U7's merge's. A figure that reads anything else takes M0's base
run at H0. A count that U8-F1's six files should move takes the base count plus that move.
| Figure | Bar | Why |
|---|---|---|
| ops | 104 tests, 104 pass, 0 fail, strict | U7's M5's 101 on unchanged ops/, plus U8-F1's three new tests; the fix pass adds none |
| wrappers | "315 ok, 0 failed", strict | The builder's end check had 315 on U8-F1's r1.sh; U7's merge touched nothing under ops/ |
| test | 931 measured and 1,028 bites passed, 0 failed, 0 skipped, strict with U7's M5 | crates/ unchanged since U7's merge; U8-F1 touches no Rust |
| web | 70 of 70, strict with U7's M5 | web/ unchanged |
| wasm | 27,000 ticks native-native and native-Wasm, 0 mismatching, the hash-map bite caught, strict with U7's M5 | crates/ unchanged |
| fmt, clippy, clippy-wasm32, doc, deps | PASS; deps' held line as U7's | crates/ unchanged |
| self | 80 of 80, equal to M0's base run (expected 80) | check.test.mjs is under spikes/; the base run gives it at H0 |
| marks | 81, rows unchanged, equal to M0's base run | the step also reads this branch's docs/adr/ (check.mjs:2702-2711), which spikes/ does not cover |
| refs | **= N0 from M0's base run, exactly; not 2,139** | the step counts `git ls-files` over the whole repository (check.mjs:2695-2700), notes included. Notes committed since U7's M5 move the count, so U7's 2,139 would force M6 on a legitimate move. U8-F1 adds no tracked file, and nothing is committed between the base run and M5 (the no-save window), so M5 must equal N0 |
| redact | **11 of 11 tests and 0 findings, strict; items equal to M0's base run's count (expected 464)** | the scan reads the spike folder on disk (check.mjs:2714-2718), not git. U8-F1 adds lines, not files, so its six files must move the item count by 0 and add no finding. If M5's count differs from the base's, name the files on disk under spikes/netcode-proof that differ (for example, any written by an earlier check.mjs step). A drop with no named cause is M6 |

**C1: M5's text in the plan reads, in place of its "each count equal to U7's M5" list:**
- "ops: 104 tests, 104 pass, 0 fail; wrappers: '315 ok, 0 failed'.
- check.mjs: all 12 steps pass.
  - test, web and wasm: equal to U7's M5 (931 and 1,028 passed, 0 failed, 0 skipped; 70 of 70; 27,000 ticks, 0
    mismatching).
  - self, marks and refs: equal to M0's base run (refs = N0).
  - redact: 11 of 11 tests, 0 findings, and its items equal to the base run's count. A difference is named with the
    files on disk that explain it; an unexplained drop is M6.
- Any other differing count is named; unless U8-F1's six ops/ files explain it, M6."
- M0's base-run bullet: "2,139 expected" becomes "the count recorded as N0 (2,139 at U7's M5; notes since may move it)".

## 3. Q2: the two carried Lows (SEC-U8F1-3's wording, SEC-U8F1-4's Stop B record)
**C2, the ruling:** both go onto PR #40's closing list now, as security findings left open, and u8-report.md names
them.
- Why now: a security finding not fixed in the PR counts as left open, wherever it sits (advisor.md, step 4). Units add
  theirs as they merge, and U8-F1 merges with these two open.
- If they are built later, the line that builds them takes them off the list, naming its commit.
- Their risk until then is small:
  - SEC-U8F1-3: a hooks path set only in Bash's profile, with the push run in Bash. The prompt's push is PowerShell's,
    and every command prompts him.
  - SEC-U8F1-4: Stop B's comparison is less exact. The re-read line still runs before every push.

**Building them at Stop B's fill would pass U8's 6,282 stop, and so would the fill itself.** That makes the question
his: U8-FILLS, below.
- Each Low needs one new line: R1.md:88 is at 120 columns, and step 2 needs a seventh line (u8-f1-build.md, "Carried").
- Separately, the approved brief's own Stop B fill adds lines to U8's files. ops/runs/r1-dist.sha256 holds comments
  only until Stop B, when it gains one `sha256  path` line per dist file (seven by U7's dist names). Y, X and the URLs
  are filled in place, net 0.
- With U8 at exactly 6,282, the fill alone passes the stop if fills count.
- Nothing in his U8F1-SIZE (a) words says whether data fills count. Reading that is re-reading his stop.

**Meanwhile:**
- The merge goes on: it adds nothing past +93.
- No fill and no building of the two Lows until he answers U8-FILLS.
- Stop B's preparation that writes no U8 file (the two-path dist build, the price read, the laptop reads) goes on.

## 4. Q3: no C1, no M2b, M3a kept: agree
- **No C1:** C1 tests save() and config.mjs against the judge (advisor-u7-brief.md:118-119). U8-F1's six files touch
  neither ops/browser/ nor the judge, and run no browser leg.
- **No M2b:** the form adds it for patches that change tracked files under crates/; this one changes none.
- **M3a kept:** it is cheap and harmless. After the base run through main-run.sh touches crates/, `git update-index
  --refresh` re-reads stat data and prints "needs update" only for real content changes, so its bar holds. M6's
  refreshed undo stays as written.
- **One addition, Low, for M9's worktree step** under "Item 6, re-ruled":
  - The builder's worktree also holds the scratch folder `.checkpoints/netcode-proof-test/u8-f1/` untracked (drafts.md,
    list-draft.test.mjs, pin-draft.test.mjs, hooks-proof.mjs, missing-log.mjs), beside u8-f1-build.md.
  - Step 1's notes save copies each of those files to the main folder's same path, sha256-equal to the worktree's.
  - ffcheck then expects exactly those, the six differing ops paths, and nothing else.
  - Otherwise the fast-forward leaves staged additions, and the fallback (b) applies.

## 5. Q4: u8-report.md's U8-F1 section
**C3: M8's list gains these lines (each a pointer, no new paper):**
1. **The rounds and rulings:**
   - the head read: advisor-u8-head.md, APPROVE WITH FINDINGS; ADV-U8H-1 to -4; SR-1 and SR-2 weighed Low;
   - his RESULTS-PUSH (b) and S14-PUSH (a), at history.md:8068-8106;
   - the brief check: advisor-u8-f1-brief.md, C1-C7;
   - the security round: security-u8-f1.md;
   - its ruling: advisor-u8-f1-sec.md;
   - the stop's moves: 6,230, then 6,282 by U8F1-SIZE (a).
2. **What leaves the server now,** one line for his Stop B read: `collect/`, `probe/presence.txt`,
   `ds-selftest/ws_split.txt`, `bringup/facts.txt`, `s14/` whole and `s14-redact.txt`. The new prints are integers
   only, and the policy reads are numbers. The s14 folder's own check runs with no VM list (S14-VMS), and the hook
   checks the whole push with it.
3. **The D10 and DEP-HOLD lines:** no marks.json or judge change; no manifest or lockfile; ops/browser's
   package.json and package-lock.json unchanged.
4. **The first run against the merged trees:** the builder's end checks ran at 77ffc95, so M5 is U8-F1's first run
   against NB2's and U7's merges.
5. **The security findings left open, for PR #40's closing list:** SEC-U8F1-3 and SEC-U8F1-4 (C2); SEC-SB-9's rest,
   as before. logq's non-object line is a STATUS follow-up, not a security finding.
6. **ADV-U8H-4, applied in the same edit** (my head read asked for it; it is not done):
   - :63-64's "U6c's names (NC-R2-4, read by test 7 on the merged code: M5 above)" becomes "U6c's names: test 7's
     item 4 reads fixtures; on real output, the head read's run of probe_presence on U6c's 24 Nov log (P0 present, P1
     at the drop rule)".
   - The open list gains U7's alignments (the URL contract in config.mjs, `node build.mjs` in rs.yaml), checked at
     U7's merge: their state as found.
   - :70-71's "For the advisor's head read" line points to advisor-u8-head.md (read).
7. **The two wordings for his Stop B read** (u8-f1-build.md, "Open"):
   - "runs on `<results>\r1\s14` with no VM list" (S14-VMS, choice 25);
   - the save's shorter reason.

## Decisions needed
- **U8-FILLS.**
  - The question: do Stop B's fills count against U8's 6,282-line stop? The fills are the seven expected dist
    SHA-256 lines in r1-dist.sha256; the commit, X and the URLs are filled in place. And may the two one-line security
    Lows carried by U8-F1 (SEC-U8F1-3's "in the same PowerShell" wording, SEC-U8F1-4's Stop B record of the hooks path
    and `git --version`) be built at that fill?
  - Why it matters: U8 sits exactly at 6,282, and the approved brief's own fill adds about seven lines. If fills
    count, Stop B's fill itself passes his stop, and the two Lows stay open security findings until built.
  - Options:
    - (a) the fills are data the approved brief requires, outside the stop; the two Lows are built at the fill and
      count, so the stop becomes 6,284;
    - (b) the fills stay outside the stop; the two Lows stay on PR #40's closing list and R1 runs without them;
    - (c) everything counts, so the stop becomes about 6,291 (6,282, plus about 7 fill lines, plus 2).
  - Lean: (a). The fills are values the plan has always said are filled at Stop B, not growth, and two lines close
    two security findings before R1.

## For Mustafa
1. Answer U8-FILLS before Stop B's fill (Tue 1 Dec). Until then the main session neither fills U8's files nor builds
   the two Lows; the merge itself goes on.
2. For the day's message, his to reverse: M5's refs and redact bars are taken from the base run on the day, not from
   U7's numbers, since notes move the repository's file count.

Status: COMPLETE 2026-11-27 06:00 UTC
