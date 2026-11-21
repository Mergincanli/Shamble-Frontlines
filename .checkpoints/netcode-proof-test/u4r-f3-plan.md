# U4r-F3: the one runner change (plan for the advisor's review)

Written by the main session on 2026-11-14 at about 23:10 UTC. It answers the advisor's P0 ruling
(advisor-u4r-sets-r2-full.md), next step 3: "The one runner change (c-u4r-runs.mjs and c-u4r-r2.mjs): plan first, with
my plan review. Its list: INF-F2-1's header sentence; INF-F2-2's three wordings; INF-F1-5; RUNS-LINE; RATES-1 with the
measured ratios (P3 1.44, B1 1.504, P2 1.238, P0 1.057); a printed judge-peak projection for each call, using a fit from
the six judge points (the plan names the formula); INF-F1-3 once check.mjs's run() returns the signal. Then stagings,
infra-engineer's one round, CI and my review of the head. It lands before BRAWL-1's re-run sets." No edit comes before
your review. Nothing is running now: no set, no agent, and the shared target is cold.

The scripts as committed: c-u4r-runs.mjs at 791bc55 (sha256 18c1dcf6…, 486 lines) and c-u4r-r2.mjs at f1f131b (sha256
94564f8c…, 235 lines).

## 1. The list, and where each item goes

| Item | Its source | c-u4r-runs.mjs | c-u4r-r2.mjs |
|---|---|---|---|
| INF-F2-1's header sentence | u4r-f2-review-full.md; ADV-Q in advisor-u4r-f2-head-full.md | yes (2.1) | no: r2 has no split |
| INF-F2-2's three wordings | u4r-f2-review-full.md | yes (2.2) | no: the comments are the split's |
| INF-F1-5, the untrimmed status read | u4r-f1-review-full.md (it names both scripts' lines) | yes (2.3) | yes (3) |
| RUNS-LINE | advisor-u4r-sets-r1-full.md | yes (2.4) | Q5 (recommended no) |
| RATES-1 | advisor-u4r-sets-r1-full.md; the ratios: advisor-u4r-sets-r2-full.md Q3 (5) | yes (2.5) | no: section 3 |
| The judge's projected peak, per call | advisor-u4r-sets-r2-full.md, next step 3 | yes (2.6) | Q5 (recommended no) |
| INF-F1-3, the signal's name | u4r-f1-review-full.md; INF-R2-5 in advisor-u4r-r2-full.md | waits (section 4, Q4) | waits (section 4, Q4) |

## 2. The changes in c-u4r-runs.mjs

### 2.1 INF-F2-1's header sentence

It goes right after INF-F1-1's sentence (ADV-Q), in infra-engineer's words, with a source tag. The header's :10-14 then
read:

> `flock -x .gate/shared-target/.lane.lock node spikes/netcode-proof/ops/runs/c-u4r-runs.mjs …`. If this script dies
> while a call runs, the lane lock is freed at once (no npt-lab or npt-judge call holds it: advisor-u4r-f1-head,
> INF-F1-1, measured), while the call, in its own session, runs on with no limit: to stop a set, end the call's group
> first (`kill -9 -- -<pid>`, the pid from `pgrep -a npt-lab; pgrep -af npt-judge`), then the script. **In a set with
> bites (P0), ending the measured judge call's group starts the bite-runs call at once: pgrep again and end that call's
> group too; the script then writes runs.txt and exits 1 on its own. Kill the script only when no npt-lab or npt-judge
> call is left (U4r-F2's review, INF-F2-1).** Each npt-lab call and the judge are bounded at RUN_LIMIT_MS
> (advisor-u4r-r2, INF-R2-4); the builds are not.

### 2.2 INF-F2-2's three wordings, in infra-engineer's words

1. The header (:35-36), "No run's verdict reads another run, so each call's verdicts are what one call gives.", becomes:
   "No run's verdict reads another run for -2's, -17's and -57's checks (no against threshold, no ws-practice rule,
   figures that read one run); a mark that compares runs (a twin figure) needs one call."
2. The header (:36) "Both calls always run, each after its own re-check;" and the judge comment (:444) "both always run"
   become: "Both calls run whatever the first's end; a re-check stop ends the set before either, with no results/."
   - **One flag (Q1):** "before either" fits a re-check stop before the first call. It does not fit one before the
     second call: the first call has then run, and its verdict.json lies in the raw-log folder, though there is no
     results/. INF-F2-2 is about comments that overstate or understate what the code does, so I propose one change of
     words: "a re-check stop before either call ends the set there, with no results/."
3. The stop paragraph (:66-69), "runs.txt is written with the run lines and no verdict", becomes "… with the run lines
   and no verdict from that call".

### 2.3 INF-F1-5: the untrimmed status read

- **The cause:** sh() trims both ends (:256), so the re-check's status reading drops the first line's leading space. A
  stop message then quotes "M spikes/…" for " M spikes/…", which loses the staged-or-unstaged column.
- **The change:** sh() gains a fourth parameter, keepLead (default false), and returns `r.stdout.trimEnd()` when it is
  set. Only the re-check's readings pass it: `spikeReading((args) => sh("git", args, ROOT, true))` at :323. The line
  carries a one-line comment naming INF-F1-5.
- **Unchanged:**
  - the tree reading (rev-parse prints no leading space);
  - the start check's tracked() and untracked reads, which test only emptiness (an untracked line starts with "??");
  - the compare itself. Both readings go through the same function, and the start reading is always empty, because the
    start check refuses a tracked change.

### 2.4 RUNS-LINE: each part and bite of the set, in runs.txt

- **What is wrong today:** runs.txt's judge line is the judge's own summary: each mark's overall verdict and its first
  "why". That can hide the set's part. P3's runs.txt reads "PM-0001-57 NOT-RUN L1 P0: 0 of 3 valid runs", while -57's
  L1 P3 part is a PASS.
- **The change:** a new exported pure helper, setParts(verdict), turns a parsed verdict file into lines. For each mark,
  it takes each part, then each bite, that holds at least one run of that call (`runs.length > 0`). Each line gives the
  verdict (part) or status (bite), each run with its validity, and the part's why lines, indented.
  - Every word is copied from the file's fields: mark, name, verdict, why, runs[].run, runs[].valid, bites[].id,
    bites[].status. Nothing is counted or judged here.
  - "A part that holds a run of this call" is the set's own part with no names written into the script: -2's "SS runs",
    and -17's, -57's and -18's "L1 <profile>". In a split set's verdict-bites.json the parts hold no runs, so only the
    bites print; in its verdict.json the bites hold none, so only the parts print.
- **Where:** in judgeCall, after a call that exits 0, the runner reads its own --out file and adds a heading line plus
  setParts' lines under that call's judge line, in runs.txt and on stdout.
  - A call that is stopped or exits non-zero writes no verdict file, so it gets no parts lines, as today.
  - If the file cannot be parsed, a single line names the error. The set goes on: the file is still copied, and the end
    is as today.
- **What it gives, from the committed files** (P3's verdict.json; P0's verdict-bites.json):

```
judge, the set's parts and bites (verdict.json: each part and bite that holds a run of this call, as the file gives them):
  PM-0001-2 SS runs: FAIL (l1-p3-s1 valid, l1-p3-s2 valid, l1-p3-s3 valid)
    SS runs: run l1-p3-s1: everyone in view on at least 99 % of each client's snapshots = 0.352778, needs >= 0.99
    (and its other two why lines)
  PM-0001-17 L1 P3: INVALID (l1-p3-s1 invalid, l1-p3-s2 invalid, l1-p3-s3 invalid)
    L1 P3: run l1-p3-s1: pre:PM-0001-2: everyone in view on at least 99 % of each client's snapshots = 0.352778, needs >= 0.99
    (and its other two why lines)
  PM-0001-57 L1 P3: PASS (l1-p3-s1 valid, l1-p3-s2 valid, l1-p3-s3 valid)

judge (bite runs), the set's parts and bites (verdict-bites.json: …):
  PM-0001-17/tick16: invalid (bite-tick16 invalid)
  PM-0001-17/client-rounding: invalid (bite-client-rounding invalid)
  PM-0001-17/whole-as-lost: invalid (bite-whole-as-lost invalid)
  PM-0001-57/pos-4cm: shown (bite-pos-4cm valid)
  PM-0001-57/yaw-12bit: shown (bite-yaw-12bit valid)
  PM-0001-57/pitch-8bit: shown (bite-pitch-8bit valid)
```

- **The fixed SPLIT_LINE stays** (condition 3 (b) of your F2 plan review). It still never parses the judge's stdout.
  setParts reads the verdict file the judge wrote, never its stdout.

### 2.5 RATES-1: the rates from these sets' measured runs

**The measurements** are du -sb of each kept run folder, taken today under .gate/netcode-proof-test/runs/. The table
gives bytes; MB is 10^6 B.

| Profile | Runs whose header lists PM-0001-57 | The largest | Old rate |
|---|---|---|---|
| P3 | 178,922,504 / 181,429,340 / 183,256,573 | 183.26 MB | 125.8 MB |
| B1 | 285,578,840 / 283,081,680 / 291,324,004 | 291.32 MB | 190.6 MB |
| P2 | 271,216,673 / 272,819,713 / 272,351,303 | 272.82 MB | 219.8 MB |
| P0 | measured runs 514,005,562 / 514,868,190 / 520,269,430; -57's bites pitch-8bit 518,771,189, yaw-12bit 567,497,932, pos-4cm 603,630,452 | 603.63 MB | 509.1 MB |

The other runs are P0's three -17 bites, with no -57 series: 55,602,928 / 55,840,142 / 55,602,244. Their old rate is
56 MB.

**The change** (Q2 (a)):
- BYTES_10MIN_57 becomes the largest run of each profile, rounded up to 0.1 MB: `{ P0: 603_700_000, P2: 272_900_000,
  P3: 183_300_000, B1: 291_400_000 }`.
- BYTES_10MIN_OTHER stays at 56,000,000, which is above all three measured runs (55.84 MB at most).
- The comment above them names this source: the date, the folders, du -sb, the largest run, the spread, and the four
  ratios.
- The header's projection bullet (:53-56) names RATES-1 and "U4r's judged sets' largest measured runs" in place of "the
  builder's measured rates". PAIR_BYTES stays the builder's figure.

**The check** (in MiB, as runs.txt prints them):

| Set | Old projection | New projection | Measured (du -sb) | New / old | Measured / old (the ruled ratios) |
|---|---|---|---|---|---|
| P3 | 359.9 | 524.4 | 518.8 | 1.457 | 1.441 |
| B1 | 545.3 | 833.7 | 820.5 | 1.529 | 1.505 |
| P2 | 628.9 | 780.8 | 778.9 | 1.242 | 1.239 |
| P0 | 3,073.3 | 3,614.6 | 3,248.7 | 1.176 | 1.057 |

**Why the largest run, not the old rates times the ratios** (Q2 (b)):
- A set's ratio is its mean over its runs, so scaling by it understates the largest run in the set.
  - P3 125.8 x 1.44 = 181.2 MB, against s3's 183.3 MB.
  - P0 509.1 x 1.057 = 538.1 MB, against pos-4cm's 603.6 MB.
- The per-run floor check, before each run, uses that run's own bytes. It must not understate.

**The pre-check is unchanged.** It projected 17,025,936 B (its recording's manifest bytes plus three pairs' output) and
wrote 17,323,081 B. The only difference is the marks.json copy, 372,637 B, which every set folder holds and no
projection counts: immaterial against the 4 GiB floor.

### 2.6 The judge's projected peak, for each call

**The formula** is your six-point fit: peak MiB = 302.2 + 2.3403 x the MiB of logs the call reads. I reproduced it from
the judge-apart logs (.gate/u4r-judge-apart/*.log, peak.py's ru_maxrss), with residuals of -90 to +70 MiB:

| Point | Logs (MiB) | Peak (MiB) | The fit (MiB) |
|---|---|---|---|
| one P3 run | 170.6 | 641.1 | 701.5 |
| P3 folder | 518.4 | 1,507.8 | 1,515.4 |
| B1 folder | 820.2 | 2,260.6 | 2,221.7 |
| P2 folder | 778.6 | 2,173.4 | 2,124.3 |
| P0 measured half | 1,477.4 | 3,829.8 | 3,759.7 |
| P0 bites half | 1,770.9 | 4,356.7 | 4,446.7 |

New exported names:
- JUDGE_FIT `{ a: 302.2, b: 2.3403 }`;
- JUDGE_LINE_MIB 10,240 (your 10 GiB line);
- judgePeakMiB(bytes);
- a line maker, judgeRoom(bytes, what);
- logBytes(dir): the sum of the file sizes under a folder, at any depth, as the judge finds its .jsonl files at any
  depth. The six points' MiB are the logs only. Before a call, the folder holds only the logs and the 0.36 MiB
  marks.json copy (a one-call set), and no verdict file yet.

**Where it is printed:**
1. **At the start**, right after the disk projection "before the builds", one line per call from the plan's projected
   bytes for that call's runs. One call reads all the runs; a split set's measured call reads the measured group and its
   bites call the bites group. These lines go into runs.txt's head as "projection: judge … ".
   - Under Q3 (b), the set stops there, before the builds, with nothing made, if any call's projection is above 10,240
     MiB.
   - At the new rates the largest call is P0's bites call: 1,887.4 MiB, so 4,719 MiB. The line falls at 4,246 MiB of
     logs per call.
2. **Before each judge call**, after its re-check, one line from the bytes on disk (logBytes of the call's --logs
   folder), in runs.txt beside that call's judge line. It only prints, at every Q3 choice except (c).

The projections at the new rates:

| Call | Logs (MiB) | Projected peak (MiB) |
|---|---|---|
| P3 | 524.4 | 1,530 |
| B1 | 833.7 | 2,253 |
| P2 | 780.8 | 2,129 |
| P0 measured | 1,727.2 | 4,344 |
| P0 bites | 1,887.4 | 4,719 |

### 2.7 The header's other lines, kept true

- **The split paragraph (:34-35)** says "where one call over the whole folder would project about 11.5 GiB". That was
  the four-point fit at F 1.504, before the set. One clause keeps it true: "projected about 11.5 GiB before the set (the
  four-point fit at F 1.504; at P0's measured bytes, the six-point fit gives about 8.2 GiB: advisor-u4r-sets-r2)". The
  split stays, as your ruling's step 6 runs P0 split.
- **Two new bullets:**
  - RUNS-LINE's: under each judge call that exits 0, runs.txt prints each part and bite holding a run of that call, as
    the verdict file gives them; nothing is judged there.
  - The projection's: the formula, its source, and where it prints, with Q3's stop if chosen.
- **The exported helpers' list (:78-79)** adds the judge's projected peak and the verdict file's parts lines.

## 3. The change in c-u4r-r2.mjs

- **INF-F1-5 only:** the same keepLead parameter in r2's own sh() (:114-118), passed only by its re-check's readings
  (:170).
- **Why the rest does not apply:**
  - **RATES-1:** r2's projection is already its own suites' whole bytes (du -sb of the builder's 13 folders plus
    whole-as-lost at the p0 suite's size). Its record run at e59ece9ecafc wrote 119,159,321 B (113.6 MiB) against its
    162 MiB projection, 0.70 of it.
  - **INF-F2-1 and INF-F2-2:** r2 has no split.
  - **RUNS-LINE:** r2.txt already holds each suite's judge output whole, and each suite has its own verdict file beside
    it. No next r2 run is planned.
  - **The judge's projected peak:** r2's largest suite folder is 24.8 MB, which projects about 358 MiB.

## 4. INF-F1-3: why it waits

- **Where it stands:** check.mjs's run() returns { ok, out, stdout, status, timedOut }, with no signal (check.mjs:1921).
  Both scripts can only write "by a signal" today.
- **Why it is not here:** INF-R2-5, the change that makes run() return `signal: r.signal` with a self-test case, is ruled
  into the check.mjs follow-up (advisor-u4r-r2-full.md, item 18), together with SEC-U4R-R2-1, SEC-U4R-R2-2, TE-R2-3 and
  INF-U4R-10 (2). That follow-up comes before PR #40 is marked ready.
- **Q4 (a), recommended:** INF-F1-3 waits for that follow-up, and the follow-up's plan then names the signal in both
  scripts: judgeCall's two "by a signal" phrases (:452 and :454) and r2's step() (:157).
- **Until then, an out-of-memory kill is still told apart:**
  - the judge projection (2.6) is printed before each call;
  - peak.py's whole-call maximum is in the kept log;
  - the 16 GiB machine (no swap) has about 3.4 times the largest projected call free.
- **Q4 (b):** pull INF-R2-5 into this change. That touches check.mjs and check.test.mjs and splits the check.mjs
  follow-up in two.

## 5. Risks

- **RUNS-LINE misreads a verdict file.**
  - Every word is copied from the file's fields, with no logic.
  - The real-file check (section 7, item 2) runs setParts on all five committed verdict files and compares each line
    with the report's one-table summary.
  - The verdict files stay the record. The report reads them, never runs.txt alone.
- **The judge projection stops a set wrongly** (Q3 (b)). It fires only above 4,246 MiB of logs in one call. The largest
  call at the new rates is 1,887 MiB.
- **RATES-1 asks for more free space.** P0's start check now needs 3,614.6 MiB plus the 4 GiB floor, about 541 MiB more
  than before. BRAWL-1's disk plan (your step 4) uses these rates.
- **RATES-1 understates at BRAWL-1's commit.** With every soldier kept in view, a -57 series could grow.
  - P0's valid runs s2 and s3, which had everyone in view, wrote 514.9 and 520.3 MB against s1's 514.0. So the effect
    seen is small.
  - The 4 GiB floor absorbs about 10 % (about 360 MiB on P0).
  - BRAWL-1's own 10-minute SS P0 seed-1 and SS P3 runs measure the new bytes before the re-run sets. A run above its
    rate comes to you before those sets.
- **INF-F1-5** changes only the readings' leading whitespace. Both sides of the compare change alike.
- **The tested copy** (F2's lesson, ADV-F2H-1):
  - The commit carries exactly the sha256 the stagings ran on.
  - A wording fix after the stagings means the cmp stagings run again before the commit.
  - The harness is not edited after its run, or its sha256 change is recorded.
- **Nothing runs on these scripts** until BRAWL-1's re-run sets, so no set straddles the change.

## 6. Change type and tier

Risk tier 1: two spike ops scripts. There is no game code, no kit or gate file, and no change to the marks, the judge or
check.mjs (unless Q4 (b) is chosen). Type: test tooling, so the evidence is a harness on the real scripts plus the
real-file check.

## 7. Evidence (kept under .gate/u4r-f3/; scratch under .gate/u4r-f3/scratch/, never under .checkpoints/)

1. **The basics:** `node --check` on both scripts; redact-check alone on the spike folder, as check.mjs runs it; and
   `node tools/gate.mjs --no-smoke`. Each log is kept (F2's condition 6).
2. **The pure helper test** (scratch, never committed):
   - (a) setParts on P3's, B1's and P2's verdict.json, and on P0's verdict.json and verdict-bites.json, all as committed.
     It gives exactly the lines of section 2.4, each matched against u4r-report.md's one-table summary.
   - (b) judgePeakMiB at the six points gives the fit column of 2.6, and judgeRoom's line at 4,246 and 4,247 MiB falls
     on each side of the line.
   - (c) projectedBytes(planJudged(P)) for each profile gives section 2.5's new column, at least the measured bytes.
   - (d) spikeChanged with an untrimmed reading quotes " M spikes/…" with its leading space.
3. **Stagings on the real scripts**: F2's harness, with F1's r2 stagings, copied to .gate/u4r-f3/harness/ and adapted.
   Its shims record argv, nothing real runs, and its judge stand-in writes a fixture verdict file copied from the
   committed ones.
   - (i) **cmp-P3, cmp-B1, cmp-P2 and cmp-P0:** runs.txt against HEAD's, with the times masked.
     - The only differences are the head's judge projection lines, each call's on-disk projection line, and the parts
       lines.
     - The judge's argv, the layout and the exit lines equal HEAD's.
     - P0's parts lines come from both files: the measured parts from one, the bites from the other.
   - (ii) **fail-P0-measured, fail-P0-bites, fail-both, and lim-P0** (a copy with RUN_LIMIT_MS = 2,000): a call that
     did not exit 0 gets no parts lines, the other call's lines are present, and the exit lines are HEAD's.
   - (iii) **parts-unreadable:** the stand-in writes text that is not JSON and exits 0. One line names the parse error,
     the file is still copied, and the end is as today.
   - (iv) **big-plan,** under Q3 (b): a copy with BYTES_10MIN_57.P0 raised so the bites call projects over 10,240 MiB.
     The set stops before the builds: no build call, no bin, raw-log or results folder, and the line printed.
   - (v) **recheck-untrimmed,** with real git in a /dev/shm scratch repository, as F1's and F2's recheck stagings: an
     unstaged edit under spikes/netcode-proof before the judge. The stop message quotes " M spikes/netcode-proof/…" with
     its leading space. Then the same for r2 (its recheck staging) and a cmp-r2 against HEAD's r2.txt (no difference
     beyond the masked times).
4. **The size**, on the card beside the plan's.

## 8. Who builds, reviews and checks

- **Building:** the main session, by Edit, after your verdict.
- **The harness:** an evidence agent runs it in the background. It reads its notes and the diff first (USAGE-SAVINGS).
- **The one round:** infra-engineer reads the runner's mechanics, as in F1 and F2.
- **Then:**
  - one commit (both scripts and the card);
  - push;
  - CI;
  - your review of the head.
- **Meanwhile:** BRAWL-1's plan (your step 4) is drafted in parallel. It shares no file with this change.

## 9. Planned size (SIZE-CHECK)

- **The whole change:** about 100 lines added and 25 removed. c-u4r-runs.mjs takes about 95 of the additions and 22 of
  the removals: the header lines, the rates and their comment, setParts, the judge projection, judgeCall's lines and
  keepLead. c-u4r-r2.mjs takes about 4 and 2.
- **"Well past its plan"** is more than twice that: more than about 250 changed lines.
- The planned size goes on the card beside the actual one.

## 10. Choices for you

- **Q1, INF-F2-2's second wording:**
  - (a) infra-engineer's words as written ("a re-check stop ends the set before either, with no results/");
  - (b) "a re-check stop before either call ends the set there, with no results/" (recommended). It is also true of a
    stop between the calls.
- **Q2, RATES-1's form:**
  - (a) the largest measured run of each kind per profile, rounded up to 0.1 MB (recommended);
  - (b) the old rates times the four set ratios, which understate the largest run (section 2.5).
- **Q3, the judge projection:**
  - (a) printed only (your words);
  - (b) printed, and at the start the set stops before the builds if any call's projection from the projected bytes is
    above 10,240 MiB (recommended). It costs one check before anything is made, and it puts your 10 GiB line in code
    before 70 minutes of runs, not after.
  - (c) (b) plus a "not run" ending before a call whose on-disk projection is above the line: a third judge ending, for
    a case that the rates and (b) make unlikely.
- **Q4, INF-F1-3:**
  - (a) it waits for the check.mjs follow-up, which then names the signal in both scripts (recommended);
  - (b) INF-R2-5 is pulled into this change.
- **Q5, r2's scope:**
  - (a) INF-F1-5 only (recommended: section 3);
  - (b) also RUNS-LINE and the judge projection in r2.
- **Q6, the round:**
  - (a) infra-engineer, one round, as F1 and F2 (recommended);
  - (b) also test-engineer, on whether RUNS-LINE's lines are proven.
