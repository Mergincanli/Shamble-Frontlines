# Advisor: the plan review of U4r-F2 (workflow wf_4b7d75ec-24c), 2026-11-14

Written by script, word for word, from the workflow's journal (advisor-u4r-f2-plan's StructuredOutput result). The plan: u4r-f2-plan.md. Its notes: advisor-u4r-f2-plan.md. Nothing below is the main session's.

## Verdict: APPROVE WITH CONDITIONS

## Conditions

1. 1. Order of work around P2. Make no edit, committed or not, to any tracked file under spikes/netcode-proof (the script included) until P2's set has ended and its results/ are committed. P2 is running now (run l1-p2-s3 at 19:06 UTC, commit ab10f058a828). Its re-check before the judge reads `git status --porcelain --untracked-files=no -- spikes/netcode-proof` (c-u4r-runs.mjs:143-146, :365-368), so an uncommitted Edit of c-u4r-runs.mjs would stop P2 before its judge. The F2 commit lands after P2's results commit and before P0 starts. Nothing else is committed under spikes/ until P0 has ended. The harness and the real-judge check run after P2's set too (condition 5 needs the lane lock).
2. 2. When results/ is made. For a set with bites: re-check with `unchanged("the judge (measured runs)")`, then call 1, then re-check with `unchanged("the judge (bite runs)")`, then call 2. Only after both calls is results/ made, each verdict file copied on its own exit 0, and runs.txt written. A re-check stop before either call throws as today and leaves no results/ folder. Today results/ is only made after the judge (:432); the plan as written could leave verdict.json in results/ with no runs.txt. Harness staging (vi) must assert: the stop message names "the judge (bite runs)"; call 1 ran and call 2 did not; no results/ folder exists.
3. 3. Exact forms in runs.txt and on exit, for a set with bites. (a) The judge lines: `judge (measured runs): ...` and `judge (bite runs): ...`, each ending in one of condition 1's three endings (stdout; `stopped at <s> s (no verdict)`; `exit <n|by a signal> (no verdict): <output, trimmed>`). (b) Directly above those two lines, one line of fixed text (it never parses the judge's output). It says: the set is judged in two calls, each at most 10 GiB (U4r-F2); verdict.json holds the measured runs' verdicts (read its L1 P0 parts; its bites read 'no run yet' by design); verdict-bites.json holds the bite runs' verdicts (read its bites; every part reads '0 of n valid runs' by design). (c) The head's raw-logs line names the measured/ and bites/ subfolders. (d) If one or both calls did not exit 0, the exit line is `c-u4r-runs: ` followed by each failed call in order, written `the judge (<measured|bite> runs) stopped at <s> s` or `the judge (<measured|bite> runs) exited <n|by a signal>` and joined by '; ', then `: runs.txt written, with no verdict from that call (to the advisor)`, with 'those calls' when both failed. (e) A set without bites keeps today's head, single call, verdict.json, judge line and exit line, word for word.
4. 4. Harness additions, on top of the plan's (i) to (vi). (a) The argv and runs.txt comparison runs, as F1's did, on HEAD's and F2's copies for precheck, P0, P2, P3, B1 and r2 (c-u4r-r2.mjs imports spikeChanged and spikeReading from this file: c-u4r-r2.mjs:54). Precheck, P2, P3, B1 and r2 must be identical once times are masked. P0's npt-lab argv must be identical except that each --out gains measured/ or bites/. P0's single judge call becomes two, with the planned --logs and --out and the same --marks and --only (PM-0001-2 named in both). P0's runs.txt may differ only by the judge lines, the fixed line and the raw-logs line. (b) Both calls fail: both lines are written, both calls are named in the exit line, and no verdict file is in results/. (c) In (ii) and (iii): the verdict file of the call that exited 0 is in results/, and the failed call's is not. (d) The judge shim records start and end times, which show measured first, then bites, with no overlap.
5. 5. Replace the plan's real-judge check with a one-call-versus-two-calls comparison. Use /dev/shm copies of r2's p0, tick16 and whole-as-lost runs: all are in set u4r-r2 at P0, which mirrors P0's shared set; together about 30 MiB. Judge with r2's npt-judge (sha256 65acacfe...) and marks.json (sha256 9fb5be57...), always with --only PM-0001-2,PM-0001-17,PM-0001-57. Make three calls: all three runs in one folder laid out measured/ and bites/; measured/ alone; bites/ alone. A python3 -I comparison must show, for every mark: the measured-alone file's parts[] and report[] equal the combined file's; the bites-alone file's bites[] equal the combined file's; both -17 bites read shown. Run it under the lane lock after P2's set and before P0. Keep its log in .gate/u4r-f2/ and remove the /dev/shm copies afterwards. Drop the plan's P3-folder call: P3's own verdict.json already is that reading, and the call would cost about 1.5 GiB of judge memory plus 0.5 GiB of tmpfs.
6. 6. Before the commit: node --check; redact-check alone, as check.mjs runs it, with its log kept; `node tools/gate.mjs --no-smoke`, as F1's M7. After the push: CI green on the pushed head. The card records the planned size beside the actual `git diff --stat`.
7. 7. Before P0 starts, ruling steps 5 and 6 still apply, now to each call. Judge-apart P2's whole folder. Take F as the largest of P3's 1.44, B1's 1.504 and P2's ratio, and at least 1.5. Refit over the four points. Each call's projected peak is the fit at F times its group's projected MiB: 1,456.6 for measured, 1,616.8 for bites. If either is over 10 GiB, P0 waits and comes back to me. The card records both figures and the break-even F (about 2.49 for the bites call, 2.76 for measured, on today's fit). P0's runs.txt head must show the same sha256s as P3's and B1's: npt-lab measured b3b4a60d..., npt-lab bites 33e1f49f..., npt-judge 65acacfe..., marks.json 9fb5be57.... Any difference comes to me before P0 counts.
8. 8. What rides along (Q4). INF-F1-1's header sentence rides, in my words from advisor-u4r-f1-head-full.md:33. INF-F1-5, RUNS-LINE and RATES-1 do not. Their new home is one runner change after the P0 set, covering both run scripts, plan first, with my plan review, before the next judged set after U4r's. u4r-report.md's follow-ups list says so.
9. 9. Agents. The evidence agent runs on Opus with effort high, stated in the call. infra-engineer runs from its agent file. At most 3 agents at a time. Neither edits the script: the main session builds it with Edit.

## The choices (the plan's section 8)

- **Q1, the split's shape** (a) Two subfolders inside the set folder, only for a set with bites. **Why:** Only P0 has bites (bitesOf). P3, B1 and P2 are done or running, and later sets without bites (BRAWL-1's -17 L1 P3 re-run, U4g's B1) keep their <logs>/<run> layout and runs.txt word for word. (b) changes every set for no gain. The judge reads .jsonl at any depth (lib.rs log_files), and npt-lab's --out never enters a header (scenario.rs: no out param; create_dir_all at :771), so the logs' bytes do not depend on the folder.
- **Q2, after a failed first call** (a) Still run the second call, then write runs.txt and exit 1, naming each failed call (condition 3's exit line). A re-check stop is not a failed call: it still throws (condition 2). **Why:** The two calls' verdicts are independent (see the code check). A cut or stopped measured log then no longer voids the bites' verdict, and the reverse holds too. That follows condition 1's rule that the expensive results are always kept. The calls are sequential (check.mjs run() is spawnSync), so memory never stacks.
- **Q3, the names** (a) verdict.json (measured runs) and verdict-bites.json. **Why:** verdict.json keeps the meaning it has in P3's and B1's committed results: the set's measured runs. The report reads the L1 P0 parts there and the bites in verdict-bites.json. The fixed line of condition 3(b) states each file's partial-by-design lines.
- **Q4, which 'next change' items ride** INF-F1-1's header sentence: yes. INF-F1-5: no. RUNS-LINE: no (condition 3(b)'s fixed line instead). RATES-1: no. The three deferred items go to one runner change after P0 (condition 8). **Why:** INF-F1-1 is a comment with fixed words: no risk. INF-F1-5 changes the re-check that P0 itself depends on, and its fix lives in both scripts' sh() (trimming), while F2 touches c-u4r-runs.mjs only (r2 imports spikeReading). RUNS-LINE would parse verdict JSON right after a 70-minute set, where a throw would lose runs.txt, and -2's own part is 'SS runs', not 'L1 P0'. A constant label gives the reader the same warning at no risk. RATES-1 needs P0's own bytes. All three keep their deadline: before the next judged set after U4r's.
- **Q5, the round** (a) infra-engineer, one round, on the final diff and the evidence, as in F1. Then my review of the pushed head. **Why:** The whole P0 set rides on the runner's end-of-set mechanics. F1's infra round found a real Medium (the lock rule). One round fits the review budget, and P0 loses only minutes to it.

## Findings

### F2-1 (Medium)

- **Where:** u4r-f2-plan.md section 6 (Building: the main session, by Edit), against c-u4r-runs.mjs:143-146 and :365-368 in the running P2 set
- **Wrong:** The plan does not put the edit after P2's set. P2 (commit ab10f058a828) re-checks spikes/netcode-proof's tracked status before its judge. An uncommitted Edit of the tracked script would stop P2 there, with no results/ and a re-run needed at a new commit.
- **Fix:** Condition 1: no edit under spikes/netcode-proof until P2's set has ended and its results are committed. The F2 commit goes between that and P0's start.

### F2-2 (Medium)

- **Where:** u4r-f2-plan.md 2.3 (re-check before each call; each file copied 'on its own exit 0'), against c-u4r-runs.mjs:422-433
- **Wrong:** If the re-check before the second call throws after the first call's verdict was copied, results/u4r/<c12>/P0/ holds verdict.json with no runs.txt. Today a re-check stop leaves no results/ folder.
- **Fix:** Condition 2: make results/ and copy the verdicts only after both calls. A re-check stop throws as today and leaves no results/. Harness (vi) asserts it.

### F2-3 (Medium)

- **Where:** u4r-f2-plan.md 5.3 (the real-judge check)
- **Wrong:** Neither half proves the claim that matters: no call ever holds both groups, so 'same as one call' is never compared. The measured-only half re-reads P3's folder, which P3's own verdict.json already is, at about 1.5 GiB plus 0.5 GiB of tmpfs. The bites-only half repeats carry 62 (h)'s record.
- **Fix:** Condition 5: one call versus two calls on r2's same-set P0 runs (p0, tick16, whole-as-lost; about 30 MiB). Show parts[], report[] and bites[] equal between the combined file and the split files, under the lane lock, after P2.

### F2-4 (Low)

- **Where:** u4r-f2-plan.md 2.3 and 2.5 (runs.txt's two judge lines)
- **Wrong:** Each judge line prints each mark's overall verdict from half the runs. The measured line hides the bites ('no run yet'). The bites line reads 'L1 P0: 0 of 3 valid runs', or a bite's line, as -17's verdict. A reader of runs.txt alone would be misled, which is RUNS-LINE made worse. The head's raw-logs line also does not name the subfolders.
- **Fix:** Condition 3(b) and (c): one line of fixed text, with no parsing, saying what to read in each file; the raw-logs line names measured/ and bites/.

### F2-5 (Low)

- **Where:** u4r-f2-plan.md 5.2 (harness cases)
- **Wrong:** The unchanged-form comparison covers P3 only. c-u4r-r2.mjs imports this file (c-u4r-r2.mjs:54, :170), and precheck, P2 and B1 go through the shared code that F2 touches. There is no both-calls-fail case, and no check of call order or overlap.
- **Fix:** Condition 4: compare precheck, P0, P2, P3, B1 and r2 on HEAD's and F2's copies, plus the both-fail, which-file-kept and order stagings.

### F2-6 (Low)

- **Where:** u4r-f2-plan.md 5.1 and section 1
- **Wrong:** No gate run is named (F1 had gate --no-smoke plus CI). Nothing checks that P0 runs on the same binaries and marks as the other sets. The per-call projection is not tied to ruling step 5's P2 point and four-point refit.
- **Fix:** Conditions 6 and 7.

## Size

The plan says about 40 to 60 lines added in c-u4r-runs.mjs, and that is honest. My item-by-item count against HEAD 9bbceb7 (spike tree 866c323b): group field in planJudged about 3; subfolders and --out about 5; the judge call factored to run once or twice, with its line about 18; results ordering and copies about 5; runs.txt lines about 4; exit line about 6; header paragraph about 5. That gives about 45. The conditions add about 10 to 15 (fixed line, raw-logs line, INF-F1-1's sentence of about 4 comment lines, exit-line form). Expect roughly 50 to 70 insertions and 10 to 15 deletions, in one file. The harness, the scratch helper test and the real-judge comparison are not committed and do not count.

## For Mustafa

Nothing goes to you to decide or check. This is U4r-F2, a small change to one run script inside the netcode spike (risk tier 1). Nothing a player sees, hears or feels changes, no rule changes, and it is not a merge to main. In plain words: judging the biggest test set in one go would need about 11.5 GB of memory, over our 10 GB safety line. So its results are judged in two halves: the three main runs (about 5.5 GB) and the six 'deliberately broken' check runs (about 6.1 GB). I checked the judge's code: each half gives exactly the verdicts one pass would give, because no run's verdict reads another run. I approved it with nine exact conditions. The main ones: no edit until the set running now has finished (an edit made earlier would stop that set before its judge); a stop between the two halves leaves no half-written results; and one direct test on the real judge shows that one pass and two halves give the same answers. This is the 'small fix before that set' from my earlier heads-up. Decisions needed: none. For Mustafa: nothing.
