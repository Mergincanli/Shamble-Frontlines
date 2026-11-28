# U4r-F2: the P0 set judged in two calls, each at most 10 GiB (plan for the advisor's review)

Written by the main session on 2026-11-14 at about 19:05 UTC, while the P2 set runs. It answers the advisor's sets
ruling (advisor-u4r-sets-r1-full.md): Q2 ("If it gives more, P0 waits: the main session plans the smallest change that
keeps every judge call at or under 10 GiB (for example, the P0 set's six bite runs judged in a second call, or shorter
bite runs if the marks' setup checks allow), and brings the plan to me before any edit") and next step 5. No edit
comes before your review.

## 1. Goal and the numbers behind it

- **The judge's memory**, measured apart under the lane lock (judge-apart.sh, peak.py; .gate/u4r-judge-apart/): one
  P3 run (170.6 MiB of logs) took 641.1 MiB; the P3 folder (518.4 MiB) 1,507.8 MiB; the B1 folder (820.2 MiB)
  2,260.6 MiB. These three judge-only points fit about 215 MiB plus 2.49 times the log MiB.
- **P0's logs:** 3,073.3 MiB by the runner's projection. F is at least 1.504 (B1's 820.2 over 545.3; P3's ratio is
  1.44; P2's comes when its set ends). So P0's logs come to about 4,622 MiB, or more if P2's ratio is higher.
  - **One call:** about 11.5 GiB of judge memory, over the 10 GiB line.
- **The smallest change:** the same nine runs, settings, binaries and marks. Only the judge's grouping changes:
  the three measured runs in one call, the six bite runs in a second.
  - **Two calls at F = 1.5:** about 2,184 MiB of logs, so 5.5 GiB; and about 2,424 MiB, so 6.1 GiB.
  - **At F = 2.0:** 7.3 and 8.1 GiB. Both stay under the line with a margin.
- **Why not shorter bite runs:** marks.json gives the bites no duration. But brief 7.1 and 7.3 plan every judged run at
  18,335 ticks, and tick16's start is 65,551 - (warm-up + measured ticks) (U4R-Q7 (a)). Shortening them changes the
  approved run plan and a ruled formula. The split changes neither.

## 2. The change: ops/runs/c-u4r-runs.mjs only

1. **planJudged:** each run gets a group, "measured" (l1-p0-s1 to -s3) or "bites" (bite-*). Only the P0 set has
   bites (bitesOf).
2. **The raw-log layout for a set with bites:** the measured runs go under `<logs>/measured/<run>` and the bite runs
   under `<logs>/bites/<run>`. A set without bites keeps `<logs>/<run>`, so P3, B1 and P2 are unchanged. The set
   folder's exists-check, the floor order and the marks copy at `<logs>/marks.json` stay as they are. The two subfolders
   are made inside the set folder after the first run's floor, as the set folder is today.
3. **The judge for a set with bites, two calls, each through check.mjs's run() with RUN_LIMIT_MS:**
   - (1) `--logs <logs>/measured --out <logs>/verdict.json`;
   - (2) `--logs <logs>/bites --out <logs>/verdict-bites.json`.
   - Both take `--marks <logs>/marks.json --only <listedOf(profile)>`, which keeps JDG-ONLY-1's interim rule
     (PM-0001-2 is listed).
   - INF-R2-1's scoped re-check, `unchanged()`, runs before each call.
   - Each result file is copied into results/u4r/<commit12>/P0/ only on its own exit 0.
   - runs.txt gets one judge line per call: "judge (measured runs): …" and "judge (bite runs): …". Each line has
     condition 1's three endings: stdout; "stopped at <s> s (no verdict)"; "exit <n> (no verdict): <output>".
   - Both calls run whatever the first one's end. runs.txt is then written, and the script exits 1 if either call did
     not exit 0, naming which one.
   - A set without bites keeps today's single call, verdict.json and judge line, word for word.
4. **The header comment:** a new paragraph says a set with bites is judged in two calls, and why (this ruling, Q2).
5. **The reading, for the report (no code):** -17's and -57's L1 P0 parts are read from verdict.json, and their bites
   from verdict-bites.json. Each file's per-mark overall line is partial: verdict.json shows the bites as "no run
   yet", and verdict-bites.json shows the parts as "0 of 3 valid runs". So the report reads part by part and bite by
   bite, as RUNS-LINE already asks.

## 3. Risks

- **A call over 10 GiB anyway:** see section 1. The extrapolation is about 3 times (820 to about 2,400 MiB), not the
  one call's 5.6 times.
- **Validity of -17's bites:** pre:PM-0001-2 is judged on each bite run itself, in the bites call, with PM-0001-2
  listed. That is the same reading as one call, since a bite's validity never reads another run (eval.rs:958-996,
  checked again in the build).
- **A partial verdict read as whole:** the reading rule in item 5, and two plainly named files.
- **The layout:** only the P0 set's raw logs move into the two subfolders. The judge finds .jsonl files at any depth
  (judge/src/lib.rs log_files), so each call sees exactly its group.
- **INF-R2-1:** a commit between the two calls is caught by the re-check before the second call.

## 4. Change type and tier

Risk tier 1: a spike ops script, with no game code, no kit or gate file, and no marks or judge change. Type: test
tooling, so the evidence is a harness on the real script plus one real-judge check.

## 5. Evidence (kept under .gate/u4r-f2/; scratch scripts under .checkpoints/netcode-proof-test/u4r-f2/)

1. `node --check` on the script, and redact-check alone on the spike folder, as check.mjs runs it, with its log kept
   (ADV-F1H-2's lesson).
2. **The F1 harness, copied to .gate/u4r-f2/harness/ and adapted** (its shims record argv; nothing real is run):
   - (i) a staged P0 set: the layout, two judge calls with the right `--logs`, `--out` and `--only`, both verdict
     files copied, and runs.txt's two judge lines;
   - (ii) the first call exits 1: the second still runs, runs.txt is written with both lines, exit 1;
   - (iii) the same with the second call failing;
   - (iv) in a RUN_LIMIT_MS = 2,000 copy, a stopped first call;
   - (v) a staged P3 set: one call, the `<logs>/<run>` layout, and runs.txt's form as HEAD's (diff of the two
     runs.txt with the times masked);
   - (vi) the scoped re-check before the second call: a tracked edit under spikes/netcode-proof between the calls
     stops it, staged with real git in a /dev/shm scratch repository, as F1's condition 2.
3. **One real-judge check** (the sets' npt-judge, sha256 65acacfe…, on /dev/shm copies):
   - a bites-only folder: r2's whole-as-lost and tick16 runs (P0 bite runs) shows both bites and reads the parts
     "0 of 3 valid runs";
   - a measured-only folder: the P3 set's three runs reads -57 L1 P3 PASS and no bite run.
   - Together these show each call's half reads as item 5 says.
4. **The helper test** (scratch, never committed): planJudged("P0") gives the groups, and planJudged("P3") gives none.

## 6. Who builds, reviews and checks

- **Building:** the main session, by Edit.
- **The harness:** an evidence agent runs it, in the background, reading its notes and the diff first.
- **The one round:** infra-engineer reads the runner's mechanics, as in F1 (choice Q5).
- **Then:** one commit (the script and the card), push, CI, and your review of the head.
- **Then:** the free-space steps (B) and (C), only as far as R needs, and the P0 set.

## 7. Planned size (SIZE-CHECK)

About 40 to 60 lines added in c-u4r-runs.mjs: the groups, the layout, the second call, its line and the header.
The planned size goes on the card beside the actual one.

## 8. Choices for you

- **Q1, the split's shape:** (a) two subfolders inside the set folder, for a set with bites only (recommended); or (b)
  two subfolders for every set.
- **Q2, after a failed first call:** (a) still run the second, then write runs.txt and exit 1 (recommended, since it
  gives the bites' verdict anyway); or (b) stop at the first failure.
- **Q3, the names:** (a) verdict.json (measured) and verdict-bites.json (recommended: verdict.json keeps its meaning
  for the parts); or (b) verdict-measured.json and verdict-bites.json.
- **Q4, which "runner's next change" items ride:**
  - INF-F1-1's header sentence, in your words (recommended yes: a comment only).
  - INF-F1-5's untrimmed status read (recommended yes: small).
  - RUNS-LINE (recommended yes: runs.txt prints each listed mark's set part, and from the bites call each bite).
  - RATES-1 (recommended no: P0's rate can only come from the P0 set; your Q3 ruling puts it after P0, with R
    covering P0).
- **Q5, the round:** (a) infra-engineer, one round, as F1 (recommended); or (b) your review of the head only.
