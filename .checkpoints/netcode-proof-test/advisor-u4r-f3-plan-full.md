# Advisor: the plan review of U4r-F3 (workflow wf_8674561f-d44), 2026-11-14

Written by script, word for word, from the workflow's journal (advisor-u4r-f3-plan's StructuredOutput result). The plan: u4r-f3-plan.md. Its notes: advisor-u4r-f3-plan.md. Nothing below is the main session's.

## Verdict: APPROVE WITH CONDITIONS

## Conditions

1. 1. Words and places (Q1 (b)). INF-F2-1's sentence goes exactly as the plan's 2.1 gives it, right after INF-F1-1's sentence (header :13), in infra-engineer's words with the tag '(U4r-F2's review, INF-F2-1)'. INF-F2-2's first and third wordings go exactly as in 2.2. The second wording reads 'Both calls run whatever the first's end; a re-check stop before either call ends the set there, with no results/.' in the header (:36) and in the judge comment (:443-444). In that comment, 'each after its own re-check' and 'results/ is made only after both' stay, and the comment reads as one sentence.
2. 2. The split paragraph's clause (2.7) reads: 'projected about 11.5 GiB before the set (the four-point fit at F 1.504); at P0's measured bytes (3,248.3 MiB of logs), one call projects about 7.7 GiB by the six-point fit (8.2 GiB by the four-point fit: advisor-u4r-sets-r2)'. It must not say that the six-point fit gives 8.2 GiB (finding F3P-1).
3. 3. No throw after the runs (finding F3P-2). In judgeCall, everything F3 adds sits in one guard: logBytes and the on-disk projection line, then reading the --out file, JSON.parse and setParts. On any error the guard adds one line naming that error. Nothing it does can throw out of judgeCall or stop runs.txt being written. setParts also tolerates a missing parts, bites or runs array. Staging (iii) runs two cases, each on P3 (one call) and on P0's bite-runs call: text that is not JSON, and JSON of the wrong shape (`{}`). Each case must show: one error line; on P0, the measured call's parts lines still present; the file still copied; and the end and exit as HEAD's.
4. 4. The judge projection (Q3 (b)):
- It is checked right after the disk projection 'before the builds' and before mkdirSync(bin), from the start plan's bytes for each call.
- Each line names: the call; its projected log MiB; '302.2 + 2.3403 x MiB (the six-point fit: advisor-u4r-sets-r2)'; the projected peak MiB; the 10,240 MiB line; and either 'room' or 'over the line: stopped before the builds (advisor-u4r-sets-r1, Q2: to the advisor)'.
- The line printed before each call, from logBytes, uses the same form and only prints.
- The pre-check set has no judge, so it prints no judge line, and its runs.txt is unchanged.
5. 5. The judge block's order in runs.txt:
- A set without bites: the call's on-disk projection line, then `judge: ...` exactly as HEAD's, then the parts heading and its lines (only on exit 0).
- A set with bites: SPLIT_LINE first, unchanged. Then, for the measured call and then the bites call: its on-disk projection line, its judge line exactly as HEAD's (`judge (measured runs): ...` / `judge (bite runs): ...`), then its parts heading and lines (only on exit 0).
- This order replaces F2 condition 3 (b)'s 'directly above those two lines'.
- The cmp stagings assert this order.
6. 6. Evidence added to section 7 (finding F3P-3):
- (a) cmp-precheck: runs.txt and argv identical to HEAD's once times are masked.
- (b) fail-P3: no parts lines, and HEAD's exit line word for word. Both stagings are already in F2's harness.
- (c) The helper test's expected lines for all five committed verdict files come from an independent python3 -I reading of the files' fields, never from setParts itself. The counts are: P3 3 part lines and 6 why lines; B1 4 and 9; P2 3 and 4; P0 verdict.json 3 and 2, with no bite lines; P0 verdict-bites.json no part lines and 6 bite lines. Then cross-check them against u4r-report.md's one-table summary.
7. 7. RATES-1, in the form of Q2 (a):
- The rates: BYTES_10MIN_57 = { P0: 603_700_000, P2: 272_900_000, P3: 183_300_000, B1: 291_400_000 }; BYTES_10MIN_OTHER stays 56_000_000.
- The comment names: the date 2026-11-14; the five run folders; du -sb; each profile's largest run, saying that P0's is the bite run pos-4cm, which also sets the rate for P0's measured runs; the spread; the -17 bites' largest run, 55.84 MB, under the kept 56 MB; and the four ratios (P3 1.44, B1 1.504, P2 1.238, P0 1.057) as the cross-check.
- The card's note for BRAWL-1's disk plan states the runner's new start figures (P3 524.4, P2 780.8 and P0 3,614.6 MiB, each plus the 4,096 MiB floor), not the measured 519 / 779 / 3,248.
8. 8. Wherever the plan's pre-check sentence is carried (card, report), it reads: the folder holds the recording (16,950,444 B, the entry's bytes) and marks.json (372,637 B); the projection counted the recording plus three pairs' output (75,492 B), which goes into results/runs.txt; so the folder is 297,145 B over the projection, which is immaterial (finding F3P-4).
9. 9. Agents and the tested copy:
- The evidence agent runs on Opus with effort high, stated in the call. infra-engineer runs from its agent file, for one round.
- At most 3 agents at a time, counting any agent used for BRAWL-1's parallel drafting. No agent edits either script: the main session builds by Edit.
- The commit carries exactly the sha256s the stagings ran on. Any wording fix after the stagings re-runs the cmp stagings.
- The harness is not edited after its run (ADV-F2H-1).
10. 10. Before the commit: node --check on both scripts; redact-check alone, with its log kept; `node tools/gate.mjs --no-smoke`, with its log kept. After the push: CI green on the pushed head, then my review of the head. The card records the planned size (100 to 130 insertions, 20 to 30 deletions) beside the actual `git diff --stat`. No set runs on either script before my review of the head.

## The choices (the plan's section 10)

- **Q1, INF-F2-2's second wording** (b) 'a re-check stop before either call ends the set there, with no results/' **Why:** The flag is right. In the code (:445-457), a re-check stop before the second call comes after call 1 has run, and call 1's verdict.json then lies in the raw-log folder. So infra-engineer's 'ends the set before either' understates what has run, and INF-F2-2 exists to fix comments that overstate or understate. I ruled 'in infra-engineer's words' (ADV-Q), so I am the one who allows this one change of words, and I allow it. Both cases are now true.
- **Q2, RATES-1's form** (a) the largest measured run of each kind per profile, rounded up to 0.1 MB **Why:** My fix says: per-profile rates from these sets' measured whole-run bytes. A set ratio is a mean over its runs, so scaling the old rates by it understates the largest run. P3: 125.8 x 1.44 = 181.2 MB against s3's 183.3 MB. P0: 538.1 MB against pos-4cm's 603.6 MB. The per-run floor check must never understate. I re-measured every run folder with du -sb, and every figure in 2.5 holds. P0's single rate overstates its measured runs by about 366 MiB at the start check. That cost is in the safe direction, the plan's risks name it, and condition 7 carries it into BRAWL-1's disk plan. The ratios stay in the comment as the cross-check, as 'with the measured ratios' asks.
- **Q3, the judge projection** (b) printed, plus a stop at the start, before the builds, if any call projects over 10,240 MiB **Why:** This stop is not a new rule. It is my own rule from sets-r1 Q2 ('If it gives more, P0 waits'), put in code. It runs before mkdirSync(bin), so a stop leaves nothing behind that could block a re-run. That makes it the same shape as the disk start check (room('before the builds')). At the new rates the largest call projects 4,719 MiB, and the line falls at 4,246 MiB of logs, so a false stop is far away. I reject (c): it adds a third judge ending for a case that the rates and (b) already cover, and condition 3 keeps the call path free of new throws.
- **Q4, INF-F1-3** (a) it waits for the check.mjs follow-up, which then names the signal in both scripts **Why:** My step 3 said 'INF-F1-3 once check.mjs's run() returns the signal'. run() still returns { ok, out, stdout, status, timedOut } (check.mjs:1921). INF-R2-5's home is the check.mjs follow-up (advisor-u4r-r2-full.md, item 18), before PR #40 is marked ready. Its text already names r2's step(). Adding judgeCall's two phrases (:452, :454) there fits that home, and the report (:193, :306) already says so. Pulling it in here would touch check.mjs and check.test.mjs and split that follow-up in two. Until then, the printed projection and peak.py's maximum in the kept log tell an out-of-memory kill apart.
- **Q5, r2's scope** (a) INF-F1-5 only **Why:** r2 has no split. Its projection already uses its own suites' bytes: the record run wrote 119,159,321 B against 169,813,722 B projected, which I checked with du -sb. r2.txt already holds each suite's judge output whole, next to that suite's own verdict file. Its largest suite (24.8 MB) projects about 358 MiB of judge memory. No r2 run is planned.
- **Q6, the round** (a) infra-engineer, one round, as in F1 and F2 **Why:** This is risk tier 1 test tooling. RUNS-LINE only prints: it copies fields from the verdict files, which stay the record. Condition 6 (c) proves its lines against an independent reading of all five committed files. The runner's mechanics are infra-engineer's subject, and one round fits the review budget.

## Findings

### F3P-2 (Medium)

- **Where:** u4r-f3-plan.md 2.4 'If the file cannot be parsed, a single line names the error'; the planned judgeCall changes (c-u4r-runs.mjs :445-455)
- **Wrong:** The guard covers JSON.parse only. If the file holds JSON of the wrong shape (`{}`), setParts throws a TypeError, because `for..of` cannot read an object. logBytes can also throw in principle. Either throw would leave judgeCall, so no runs.txt is written after a 70-minute set. That is exactly the risk my F2 plan review gave (Q4) for not taking RUNS-LINE then.
- **Fix:** Conditions 3 and 6: one guard around logBytes, the read, the parse and setParts that can only add an error line; setParts tolerates missing arrays; staging (iii) runs both a non-JSON case and a `{}` case, on P3 and on P0's bite-runs call.

### F3P-3 (Low)

- **Where:** u4r-f3-plan.md section 7 item 3 (the staging list) and item 2 (a)
- **Wrong:** Three gaps in the evidence. The list has no cmp-precheck, though the pre-check shares `head` and sh() with the changed path (F2's condition 4 (a) had it). It has no fail-P3, though the one-call judgeCall path changes. And 2 (a) gives expected lines only for P3 and P0's bites file, while the other three files have none written down, which leaves room for setParts to check itself.
- **Fix:** Condition 6: add cmp-precheck and fail-P3 (both are already in F2's harness), and take the expected lines for all five files from an independent python3 -I reading, with the counts given.

### F3P-1 (Low)

- **Where:** u4r-f3-plan.md 2.7, the split paragraph's new clause ('the six-point fit gives about 8.2 GiB'); also my own sets-r2 ruling Q3, RR3-L6's figures, as carried in u4r-report.md:520
- **Wrong:** P0's logs total 3,406,088,069 B, which is 3,248.3 MiB. The six-point fit gives 302.2 + 2.3403 x 3,248.3 = 7,904 MiB, about 7.7 GiB. The 8.2 GiB figure is the four-point fit's (212.3 + 2.5062 x 3,248.3 = 8,353 MiB). My ruling placed the sentence right after both fits without naming which one it used. That ambiguity is my error, and the plan carried it into a committed comment.
- **Fix:** Condition 2 gives the clause's words. The report's line at :520 gains '(by the four-point fit; about 7.7 GiB by the six-point fit)'. Card and report only: no push for this on its own.

### F3P-4 (Low)

- **Where:** u4r-f3-plan.md 2.5, 'The pre-check is unchanged' ('The only difference is the marks.json copy, 372,637 B')
- **Wrong:** The folder holds SS-P0-s4.rec (16,950,444 B, which is entry.bytes) plus marks.json (372,637 B), 17,323,081 B in all. The projection was entry.bytes plus three pairs' output (75,492 B), and that output goes into results/runs.txt, not into the folder. So the difference is 297,145 B, not 372,637 B. The conclusion (immaterial, rates unchanged) still holds.
- **Fix:** Condition 8's wording wherever the sentence is carried.

### F3P-5 (Low)

- **Where:** u4r-f3-plan.md 2.6 and 2.4 (where the lines print) and section 8 (who builds)
- **Wrong:** The plan leaves four things unspecified: the exact order of the new lines in runs.txt (SPLIT_LINE would no longer sit directly above both judge lines, as F2's condition 3 (b) says); the form of judgeRoom's line; that the pre-check prints no judge line; and the evidence agent's model and effort, which CLAUDE.md's 'Models and effort' requires to be stated.
- **Fix:** Conditions 4, 5 and 9.

## Size

I expect 100 to 130 insertions and 20 to 30 deletions in two files. c-u4r-runs.mjs takes about 95 to 125 insertions and 18 to 27 deletions: the header lines, the rates and their comment, setParts, JUDGE_FIT, JUDGE_LINE_MIB, judgePeakMiB, judgeRoom, logBytes, the start stop, judgeCall's guarded lines, keepLead and the import line. c-u4r-r2.mjs takes about 3 to 4 insertions and 1 to 2 deletions: keepLead and its re-check call. The plan's 100 / 25 sits inside that range. 'Well past its plan' means more than about 250 changed lines.

## For Mustafa

Nothing goes to you. There is nothing to decide or check.

What happened: before the bot fix's re-run sets, the test-run script gets one small update, and I approved its plan with a few exact conditions. After each run, it will now print each test's own result for that setting. It will make its disk-space estimate from the real sizes of today's logs. And before it judges, it will print how much memory the judge is expected to need: a set whose estimate is over the 10 GB safety line refuses to start before anything is built.

I checked every number in the plan myself, against the kept logs and the measured memory points. All but two held. Two small text errors are now conditions: a memory figure credited to the wrong fit, and a disk-difference sentence. I also added one safety rule: nothing new in the script may stop it writing its results file after a 70-minute set.

This is a spike-branch tool change. It is not a merge to main, and nothing a player sees changes.

Decisions needed: none.

For Mustafa: nothing.
