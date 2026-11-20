# Advisor head check: U4f-F1's merge into the lane at 1faf894 (2026-11-20, from main's advisor.md)

Not a round (advisor-u4f-f1-pass.md:187-191, condition 2). Read only: git show/diff/log/reflog, sed -n, wc, sha256sum,
grep, df, du, pgrep, and GitHub's workflow-run listing. Nothing run from the change (no cargo, check.mjs, npt-lab,
npt-judge, run script); nothing written under spikes/ or .gate/. Lane head when I started: 2e20d29; it moved to d02612e
(02:15:13) and 02032ee (02:18:13) while I read, both `.checkpoints/netcode-proof-test.md` only (below).

## Plan
1. The merge commit's diff against the approved snapshot, and nothing else in it or after it.
2. The merge's evidence (gate.txt, m5-check.log) and the commit message's claims against them.
3. No marks, judge, SERIES.md or dependency change.
4. The card's record of the merge; FP-5's carries after the four Lows' fixes.
5. Readiness for -20's L1 re-run; CI on 1faf894.

## Block 1: the diff (DONE)
- `git diff ba3fa68 1faf894 -- spikes/ | sha256sum` = d0a66a68a16669a541624e3c23598b73ae25b61fd4984c7fbb2cf93a7d38829e,
  19,794 B: the snapshot my pass check approved (advisor-u4f-f1-pass.md:24-27). Also from cb7e374 (the builder's base;
  `git diff --quiet cb7e374 ba3fa68 -- spikes/` exit 0): the same hash.
- 1faf894 against its parent 78db75f: exactly six files, all mode 100644 M (`--raw`): room.rs 12/8, commands.rs 34/0,
  seam.rs 10/0, world.rs 39/16, tests/seam.rs 16/0, tests/world.rs 128/0 = +239 -24. Its whole diff hashes to d0a66a68
  too, so nothing outside spikes/ is in it. 78db75f (between ba3fa68 and 1faf894) touches only the card.
- After it: `git diff --quiet 1faf894 2e20d29 -- spikes/` exit 0, and the same to d02612e and 02032ee; all three commits
  change only `.checkpoints/netcode-proof-test.md`. `HEAD:spikes/netcode-proof` is tree d4f6cd41b8d7… at 1faf894,
  2e20d29 and d02612e (and 02032ee: spikes quiet).
- The src read matches the commit message: `Command::edges_taken` (seam.rs) = `now.wrapping_sub(last)`; run_command
  loops reload edges, refuses fire edges beyond the first (Ammunition when ammo 0, else Cooldown), throws per edge;
  run_one counts seen = advance, lost = advance - taken, doubled = taken on a repeat. Eight new tests (T1, the wrap
  test, T2 a-f), which matches +8.
- **Met.**

## Block 2: the merge's evidence (DONE)
- gate.txt: `node tools/gate.mjs --no-smoke` on ba3fa68 + the snapshot uncommitted, base f5030a3: checks 1-4 and 5b-5d
  PASS, 5a skipped, "GATE PASS".
- m5-check.log: main-run.sh's header and footer form (main-run.sh:39-42, :80-85): the lane lock "held exclusively at
  01:54:18 UTC; idle after 0 s"; free 10570 MiB (above the 4 GB floor); HEAD ba3fa68; diff d0a66a68 (`git diff HEAD --
  .` from spikes/netcode-proof); untracked e3b0c442… (sha256 of nothing: no untracked file). `node check.mjs`
  01:54:18-02:13:49, exit 0: "check: all 11 steps pass"; test "642 tests passed, 0 failed, 0 reported a skip; bites
  build: 739"; wasm 27000 ticks, 0 mismatching both legs; marks "81 marks read and checked …; rows in this branch's
  docs/adr/: all unchanged"; self 78/78; redact 10/10.
- The run ended 02:13:49, before 1faf894 (02:14:03): "no commit of spikes/ until it ends" kept. 78db75f (01:54:36) is a
  card save only, so the tree checked is 1faf894's spikes tree.
- Provenance empty: this M5 ran check.mjs alone (U4f's M5 also ran cargo builds first, .gate/u4f-merge/m5-check.log:4-445).
  Condition 2 asked only check.mjs. The tree tie stands without it: the header's diff hash, the toucher, and 642 =
  634 + 8 (the new tests only exist in this tree). No finding.
- Commit message against these: snapshot, 6 files, +239 -24, gate PASS, 11 steps, 642/739 (+8 on 634/731:
  advisor-u4f-merge-head.md:107), wasm 0 mismatches; plan rev 1.1 (u4f-f1-plan.md:1); conditions 1-10
  (advisor-u4f-f1-plan.md:228); four seats APPROVE WITH FINDINGS (each review's Verdict section). All hold, except
  "its check of the pass (APPROVE)": my verdict was APPROVE WITH CONDITIONS (advisor-u4f-f1-pass.md:181).
  ADV-U4FF1H-1.

## Block 3: marks, judge, dependencies (DONE)
- `git diff --stat ba3fa68 1faf894 -- spikes/netcode-proof/marks spikes/netcode-proof/crates/judge`: empty. No path
  with marks, SERIES, README, judge, Cargo.toml, Cargo.lock, package*.json, .npmrc, .github, CLAUDE or .claude in
  ba3fa68..1faf894 or 1faf894..02032ee. DEP-HOLD untouched. **Met.**

## Block 4: the card and FP-5's carries (DONE)
- The card (at 02032ee, 200 lines, at the cap): "U4f-F1 MERGED at 1faf894" (:170-173) states the snapshot, the diff
  hash, gate PASS, 11 steps, 642/739, wasm 0, marks 81, both evidence paths. Two times are off: "pushed 02:15 UTC" (the
  branch's remote-tracking reflog: 1faf894 pushed at 02:14:10; 02:14:32 2e20d29; 02:15:15 d02612e) and
  "01:56-02:13:49" (the log: 01:54:18-02:13:49). ADV-U4FF1H-2.
- The four Lows from my pass check, fixed:
  - P-1: the card's readings PR line (:118-120) now carries "or takes at most one edge per command".
  - P-2: plan.md:1253's C-U6-19 ends "It pairs with C-U6-13 and C-U6-16: the same `JoinAccepted` change and the same
    review (:103-105, :111)".
  - P-3: u4s-carries.md:579-581 adds "taken only if open and cheap, else left for crates/sim's next opener (:147-150)".
  - P-4: "Choices taken without him" (:91-98) has my Block 6 entry (r1 :207-221) for the 2026-11-21 message: the round
    and pass, "0 presses lost" by design, the judged check to After the close, the first-command gap to U6 as C-U6-19
    before -19.
- Unchanged and still matching my pass check (Block 7): C-U4F2-9 and -10 (u4s-carries.md:548-552), the U4g, U4h and U5
  lines (:579-585, :587-589, :689-691), After the close (:131-132).
- FP-5 (i)-(v) are due in F1's record and the -20 L1 record (not written yet). (v) includes my pass check's correction
  of the FP-3 restore hash. **Met.**

## Block 5: readiness for -20's L1 re-run; CI (DONE)
- **CI on 1faf894: green.** gate push run 44659322035 and PR run 44659330605 (jobs `gate` and `smoke` success, done
  02:16:49), toolchain push 44659321688 and PR 44659330611 (success, 02:15:09). 1faf894 went up alone (reflog
  02:14:10), so its runs started. 2e20d29, d02612e and 02032ee are [skip ci] and spikes-equal; PR #40 touches no
  .github/ or gate file (`origin/main...1faf894`), so this green stands. PR #40's ready head still needs its own run
  (the card, :124).
- **Only F1 moved:** since 350677f (the -20 run's HEAD), outside results/, only F1's six files changed (`git diff
  --name-only 350677f 1faf894 -- spikes/ ':(exclude)spikes/netcode-proof/results'`). marks.json sha256 c1935a5b…22a9
  equals the kept copy's in u4f-20-350677f0aef2/. The judge and ops/runs/ are unchanged. So any difference from the
  kept logs is F1's or the build's.
- **State-neutral for -20, by reading:** sim refusals reach no log or summary. The lab writes only `rep.clamps` from the
  room's reports (lab/src/scenario.rs:1374-1390), and the summary's "press refused" is the room's own count
  (:2333). -20's bots press fire only (plan review :124), so the extra throw edges never arise.
- **Kept logs to compare against:** .gate/netcode-proof-test/runs/u4f-20-350677f0aef2/ (434 MiB, 24 files):
  measured/l1-20-{a,b}-s{1,2,3}/ and bites/bite-redundancy-1/, each with room.jsonl, client.jsonl and lab.jsonl, plus
  verdict.json, verdict-bites.json and the marks.json copy. Summary lines: results/u4f/350677f0aef2/pm-0001-20/runs.txt:23-29
  (seven lines, tracked).
- **Expected figures:** advisor-u4f-f1-plan.md:119-140, restated in advisor-u4f-f1-r1.md:191-205 and :257-261,
  unchanged.
- **Disk:** 10,570 MiB free at 02:16 UTC (df), not about 5,000: part-clean 3 is done (the shared target is now 36 KiB).
  The run builds in the main folder's cache, .gate/netcode-proof-test/target (777 MiB, release/ kept). The last -20
  folder took 434 MiB. The floor is 4,096. No cargo, rustc, check.mjs or npt-lab process was running at 02:16.
- **The run's commit:** c-u4f-runs.mjs names its folders and `--commit` from `git rev-parse HEAD`, and its header
  records `HEAD:spikes/netcode-proof`'s tree (c-u4f-r2.mjs:116-122; c-u4r-runs.mjs:194-197). The run will name a
  checkpoint commit, as 350677f was (a card save). "From the merge commit" holds by tree: see condition 2.

## Verdict
**APPROVE WITH FINDINGS.** 1faf894 is exactly the approved snapshot d0a66a68 (6 files, +239 -24) and nothing else. Every
later commit touches only the card. The gate PASS and the main-folder check.mjs run (11 of 11, 642/739, wasm 0, marks
81) are tied to that tree. No marks, judge, SERIES.md or dependency change. CI is green on 1faf894. The card records the
merge, and FP-5's carries match after the four Lows' fixes. -20's L1 re-run may start. Conditions for it:
1. Start it with no uncommitted tracked file (committed() refuses otherwise), under the lane lock, with
   CARGO_TARGET_DIR unset, as c-u4f-runs.mjs:4-8 says.
2. The -20 L1 record names the HEAD it ran at, and states that its header's spikes/netcode-proof tree is
   d4f6cd41b8d789a7d9ad00f729f3a32224d123c4, 1faf894's. Another tree stops the record.
3. Hold the run to my expected figures (advisor-u4f-f1-plan.md:119-140; r1 :191-205): summary lines equal to
   runs.txt:23-29 except wall time and binary sha256s; per-tick `h` equal to the kept logs; lost 0, doubled 0;
   presses B 432,000 each, A 432,003 / 432,003 / 432,009; -2 SS PASS 6/6; -20 A and B PASS 3/3; L2 NOT-RUN; the bite
   at 0.894544, its lost 3,614 to 0; the reorder row "not available". Any other difference stops for me. The record
   and F1's report carry FP-5 (i)-(v) and ADV-U4FF1H-1's correction.
PR #40 stays a draft and needs-mustafa (SA-F1-1 is an open security finding until C-U6-19).
Not checked by me: nothing was run (gate, cargo, check.mjs). Every pass figure is read from the logs, which are tied
to the tree by their diff hash and test counts. CI's runs are read from GitHub's job conclusions.

## Findings (CLAUDE.md's scale)
- **ADV-U4FF1H-1, Low**, 1faf894's commit message, line 11: "its check of the pass (APPROVE)". The verdict was APPROVE
  WITH CONDITIONS (advisor-u4f-f1-pass.md:181). Fix: no history rewrite. F1's report and the -20 L1 record give the
  verdict as written.
- **ADV-U4FF1H-2, Low**, .checkpoints/netcode-proof-test.md:170 and :173: "pushed 02:15 UTC" should be 02:14:10 (the
  reflog), and "01:56-02:13:49" should be 01:54:18-02:13:49 (m5-check.log:1, :4). Fix by Edit at the next save.
Neither needs a push of its own or a re-check by me.
