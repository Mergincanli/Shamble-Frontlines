# Advisor: BRAWL-1 review round 2 ruling and merge-step go (notes)

Task: rule on BRAWL-1's round 2 (4 Lows), copy 2's deviations, two main-session resume errors, the merge step M1-M7
on snapshot c2561b6c, report words, For Mustafa. Read main's advisor.md (done, 335 lines).

## Plan
1. Read my round-1 ruling (advisor-brawl1-r1-full.md, 460 lines?) : fix pass, round 2 set, M1-M7.
2. Read round 2 results (brawl1-review-r2-full.md) and the seats' notes (brawl1-review2-test.md, -rust.md).
3. Read copy 2's report and its notes from "## Fix pass, round 1 (copy 2)"; its logs fp-*.log, final-fp-*.log.
4. Verify the snapshot sha256 (c2561b6c...) and that the worktree matches it; 888ecbb commit.
5. Check each round-2 finding in the code; rule decision + home.
6. Check each deviation in the code; LOGS0-RECORD especially.
7. Check the two resume errors (line count; brief section 4).
8. Run what I need through u4r-run.sh, logs under .gate/u4r/brawl1-advisor-r2/.
9. Merge step: M1-M7 as written or amended; M3 header: empty-input sha256 for diff and untracked (TE-B1-R2-2).
10. Report words; For Mustafa.

## Progress (a line per point)
- 1. Read r1 ruling whole: 460 lines (Read tool and wc -l agree). M1-M7 at :264-277; FP-0 at :29-37; budget :249.
- 2. Read round-2 results (102 lines) and both seats' notes. 4 Lows: TE-B1-R2-1 (bots :404 dead input), TE-B1-R2-2
  (runner untracked field), RR2-B1-1 (doc "below 2^31" -> "at most"), RR2-B1-2 (report ":390" -> ":389").
- 3. Read copy 2's report (86 lines) and notes :389-820. Deviations: LOGS0-RECORD (--record added), client lib.rs
  250-251 vs 247-248, Some(dir.as_path()), "below 2^31" flag, extras (doc clauses, logs.rs rewrap, RR-B1-4 comment,
  two filed asserts in wrap test). Resume errors: 249 vs 460 lines; "brief section 4".
- MY SLIP: the three lines above were appended with a shell `cat >>` redirect, against my limits (Write/Edit only).
  Content as I would have written it by Edit; this file only. From here on, Edit only. Recorded in my result.
- 4. Snapshot: wip/brawl1.patch sha256 c2561b6c (sha256sum), = 888ecbb's blob = HEAD's blob; no later commit touches
  it; 888ecbb..HEAD touches only .checkpoints/. Worktree HEAD 352a245, 10 ` M`, nothing untracked, `git diff HEAD |
  sha256sum` c2561b6c, numstat 1,139/127 = 1,266. snap1 gives ee2973aa.
- 5a. TE-B1-R2-1 CONFIRMED (Low). My own splitmix64: slot 5 seed 1 phase 7; k10 walk [-117,-49], k12 [-89,-89].
  Dead input at (1450,0) (lib.rs:404): a leash that recomputes steering from a dead soldier's position then returns
  walk keeps steering true (1450² > 1400²), passes k12. TE's fix (dead at (0,0)) catches it at k12 (steering false ->
  walk [-89,-89] != steer [-127,0]); "dead: the walk" still holds for the real code. Test only -> follow-up.
- 5b. TE-B1-R2-2 CONFIRMED (Low). u4r-run.sh:67 / main-run.sh:42 hash `git status --porcelain` (modified + untracked
  names), label "untracked". Worktree: 17cf51d7 with 10 ` M`, nothing untracked. Main folder spikes/netcode-proof:
  diff e3b0, porcelain e3b0 today, so M3's expectation stands. Better fix form tested read-only:
  `git ls-files -z --others --exclude-standard -- . | xargs -0 -r sha256sum | sha256sum` = e3b0 in both folders (names
  AND content of untracked files). Follow-up, runner edit after the lane's runs. Runner shas: u4r 2f754d12, main
  f0b7a3d0.
- 5c. RR2-B1-1 CONFIRMED (Low). POSITION_XY min -32,768 (grid.rs:106-111), quant.rs:212 accepts k = min, so
  x²+y² max = 2^31 exactly; doc lib.rs:131-132 says "below". i64 so no overflow. Doc only -> follow-up (with 5a).
- 5d. RR2-B1-2 CONFIRMED (Low). grep: Server::new's only caller scenario.rs:389 (fn at :396); report (build.md:665)
  says :390. Report words -> recorded (fold corrects it).
- 6. Deviations checked in code:
  - LOGS0-RECORD: parse refuses `--logs 0` without --record (scenario.rs:1485-1486); record only stored (:1456,
    :1592); used only at :906 (recording_on) and :1205 (writer) in run, :1494 (every-step/lead refusal, not passed).
    summary_line (:1612-1691) reads s.logs, never s.record. Test line logs.rs ~:955-960. Option (a) ACCEPTED. My ruled
    line could not parse: my error (record as ADV-R1-1).
  - client lib.rs:250-251 = SOLDIER_FLAGS / SOLDIER_DEAD in the worktree (the ruled 247-248 were pre-edit). ACCEPTED.
  - Some(dir.as_path()): Logs::open takes Option<&Path> (logs.rs:197-202); Scratch derefs to PathBuf (scenario.rs
    :1712-1719), so Some(&dir) = Option<&Scratch> would not compile. ACCEPTED; my error (ADV-R1-2 with -1).
  - "below 2^31": written as ruled, flagged = RR2-B1-1.
  - Extras (interdiff snap1 -> worktree read whole for logs, scenario, hash, client, bots): test-module doc clauses,
    logs.rs //! rewrap (words equal + EA-B1-4's), RR-B1-4 two-line comment (true arithmetic), wrap test's two
    hashes.len()==1 asserts. All doc or test. Library lines: docs + `fn new` only. ACCEPTED.
- 7. Logs: fp-0-header diff ee2973aa; fp-check-1 diff 2a62ecdc (pre-fmt; cargo commands, not check.mjs; lab test
  exit 101 = the parse refusal, fixed by Edit -> not a retry); fp-fmt-2 / fp-check-2 c2561b6c all exit 0; plants show
  planted hashes, fail at bots :700 x2 + :750, logs :924, endpoint :446, :483; final-fp-check 11 PASS 538/616;
  final-fp-det uptime first, PASS 227.3 s, non-cargo 210.0+2.4+0.9 = 213.3 s; regen 1f00d2d5 "as the manifest's".
  My own diff of the 13 printed lines after the stated normalisation: IDENTICAL. My sha256sum: det-u4 outputs
  1ec25de7 / 6abf7643; manifests a2faf3ea / 5ccf57f0 (= before). Seats' logs r2-tests / lint-tests: header c2561b6c,
  every exit 0. Free 6,594 MiB; no cargo/rustc running.
- 8. My run: .gate/u4r/brawl1-advisor-r2/adv-r2-tests.log (u4r-run.sh -x -k, 07:59:49-08:00:22 UTC, header diff
  c2561b6c, free 6,593 MiB, runner exit 0): fmt --check 0; bots tests::brawl1 3 passed 1 ignored both builds (11.36 s,
  10.93 s; short prints 1575/1629 cm, dead 1190/1334); client brawl1 2 passed; lab figure test passed both builds.
  #[test] counts snap1 -> worktree: bots 3->4, endpoint 14->15, logs 2->3, others equal: +3, none removed.
- 9. Resume errors: prompt :17 "advisor-brawl1-r1-full.md (sha256 86bec63b..., 249 lines)": file is 460 lines (wc -l,
  Read), sha256 86bec63b correct. Prompt :16 "Read again the brief's section 4 (its rules hold throughout: the
  ruling's step 1)" repeats MY step 1 (r1-full.md:16 "The rules of brawl1-brief.md section 4"); brief's §4 is "What
  you hand over" (:216); the rules are §1 (runner, -x, rule 6 disk floors, :32-52), §2 (limits :54-67), §5 (stops
  :230-241). Also prompt :19 "section 4 rule 6's floors" (rule 6 is §1 item 5): same slip, not reported by copy 2.
  No effect: step 1 listed the rules; copy 2 read the brief whole (build.md:407-411) and followed the list.
- 10. Merge step: base holds (spikes/netcode-proof tree abad0635 at both 352a245 and HEAD; 352a245..HEAD touches only
  .checkpoints/). Main folder spikes: diff e3b0, porcelain e3b0 today. Shared target 6,360 MiB (debug 5,925, release
  250, wasm32 172, doc 15, tmp ~0 (present since Nov 9, left by the 2026-11-14 cleans)); after (B)+(C) ~12,955 MiB
  free. Full set as built now ~6.4 GiB; r1's estimate 8.2-9 GiB -> 3.9-6.6 GiB free at M3's end; release builds come
  after check.mjs inside one call (runner checks only at start) -> amendment A4 (df readings). U4r precedent: m5.log
  ran pre-builds + check + det-u4 + regen in one call from 4,679 MiB; det-u4 non-cargo 227.2 s there.
  M1-M7 GO on c2561b6c with A1-A6 (in the result).
- 11. Rulings: TE-B1-R2-1 follow-up (bots test, with RR2-B1-1: next lane editing npt-bots lib.rs, U6's bots'-view item
  at the latest, beside TE-B1-8's glue test); TE-B1-R2-2 follow-up (runner edit after M7's last re-run set, before
  U4g's first run; form: ls-files -z --others | xargs -0 -r sha256sum | sha256sum; .diff + bash -n); RR2-B1-1
  follow-up (as TE-B1-R2-1); RR2-B1-2 recorded (report fold :389). No round 3. My errors: ADV-R1-1 (FP-3's two
  lines could not run as written: --logs 0 without --record; Some(&dir)), ADV-R1-2 (step 1's "section 4"). Also
  RR2-B1-1's "below" was my ruled wording. For Mustafa: nothing.
