# Advisor: U7's fix-pass snapshot read (14:50 UTC window, 2026-11-26; not a round)
Advisor agent, day session, effort xhigh, read only (no cargo). Acting by main's advisor.md. Snapshot
wip/u7-build-rs.patch sha256 6535c181…2728 (267,380 B, 5,727 lines: matches); notes wip/u7-build-rs-notes-1055.md
sha256 a65282fa…a34c (matches).
Plan: notes first (items, reds, greens, plants, run 4, slips); his answers and the addendum; the two summary.json
files; the load.rs hunk against r1.sh's load_settings; F3's legs; verdict.

## Verdict: MERGE (no fix item), on conditions U7-M-1 to U7-M-8 (section 4)
Every item of the fix pass is built and proven, but F3's brief-schedule plant, whose premise was wrong (section 2).
- **The end check, end-2.log, 12 of 12:** test 884/981 (+3 each); web 70/70; self 80/80; redact 0 findings in 454 items;
  refs 1,988.
- **Run 4 on the fixed tree:** its three runner headers carry the same diff and untracked hashes (551e9033…, b4811e75…).
  Marks 1-7 are met, with mark 7 now read on both halves; mark 8 is met under the restated bar; mark 9 is NOT-RUN with its
  stand-ins.
- **The record of his answers** (u7-brief.md:200-209) matches his words and both summary.json files.
- **The load.rs cross-check** finds no defect.
- **Findings:** nothing Critical or High; one process Medium (his interim rule broken once, read only); the rest Low.

## 1. The questions asked
- **Mark 8: a hand read is enough.** t5ii.mjs judges no mark: it holds no PASS, FAIL or verdict, and summary.json
  records values. So U7's report judges every mark, 1 to 9, against the brief's words from summary.json, and mark 8
  is read the same way.
  - Run 4: `m8_no_wt_timeout_fallbacks` 1 at `m8_no_wt_timeout_playing_ns` 4,067,000,000. That is later than 3 s, so the
    restated bar "a fallback later than 3 s, or none within 10 s" is met.
  - `m8_read_once_reads` 1.
  - The normal fallback (`m5_fallback`) at 1,564,100,000, inside mark 5's 1.5-3 s, so the planted bug is still caught.
  - A driver change now would move the tree and need a run 5. Not asked.
- **The record against his words** (history.md:8071-8072):
  - The restated words are his, word for word.
  - "The 3 s is twice the page's own 1.5 s timer, not a reading": his.
  - "restated on 26 November after measuring Chrome": his.
  - The numbers: run 3 at 1,601,100,000 and 4,064,900,000 (1.60 s and 4.06 s, his figures); run 4 at 1.56 s and 4.07 s.
  - "the planted bug is still caught": his.
  - (9) "NOT-RUN, never PASS, with the in-process tests standing in; the real mark read in U6d": his (a).
  - One clause is the session's reading, not his: "U6d comes after R1 (replan.md:22, :92), so R1 runs with mark 9 not
    run". It cites its source, so it stands (Low, no change needed).
- **Mark 9: is F3 as built a sound stand-in under his (a)? Yes, by name:**
  - **The one that discriminates:** F3's third leg in r3_late_wakes_a_1_s_stall_and_a_3_s_gap_draw_no_refusal: 100 ms
    each way, no clamp, a 2.1 s stall at step 659. Its plant (the old step counter) fails it with
    "RefusalRecord { slot: 0, seq: 649, kind: PressEdge }" (plant-f3d.log), and the fix passes it.
  - **Coverage, not discrimination:** F3's first two legs (the brief's schedule with and without the 4 ms clamp) and
    R3's press-script test (3,000 exact wakes, every slot counter 0, plants room-a and room-b caught). They show the fixed
    client draws no refusal under those schedules. Their plant cannot fail them: after a gap of 2 s or more the clock's
    samples age out (clock.rs:20, WINDOW_NS 2 s), so no command is stamped until a new sample.
  - **What the report must say:** in process, behind npt-server's real Decoder and npt-room; no jitter or loss; the
    brief's 3 s-gap leg cannot catch the old counter (the clock's window); the fix is shown by the narrow leg only; in
    Chromium the steady wake chain is clamped, so NC-2's case is narrow there. That is sound for "not run, with the
    in-process test standing in", and never a PASS.
  - **NC-2's netcode question:** counting presses from the commands issued gives at most one edge per command. Each
    command covers at least one tick, so the room's one-edge-per-tick rule (room/src/input.rs:286-297) holds. In steady
    running, commands track steps, so the press cadence is unchanged; after a gap, fewer commands give fewer presses,
    which is honest.
- **The load.rs cross-check: no defect.**
  - U7's two bots hunks change no field. load.rs:44-45 takes MAX_LEAD from `npt_client::HONEST_MAX_LEAD`, which is 9
    (patch :695), as the old literal was; lib.rs:1334 is a test.
  - r1.sh's load_settings (:343-349) against the lane's LoadSettings (load.rs:197-222, `deny_unknown_fields`):
    - the 24 keys equal the 24 fields;
    - machine's 5 equal LgMachine's (:185-191);
    - each value fits its type;
    - each fits parse's ranges (:260-296): room "NN" ≤ 31, server an IPv4, ports non-zero, mode run, calibrate or
      probe, transport wt or ws (the call's argument, not the room's `both`), clients 1-64, duration_s ≥ 1, commit 40
      hex, pin_hex 64 hex (the probe's 64 zeros included), tick_hz 30, scene SS, behaviour brawl.
  - No test pins this pairing today: ADV-U7S-6, a carry.

## 2. Each item, red and green
- **F1:** plants (i) and (ii) fail with 85,000,667 and 51,667,334 ns (plant-f1i.log, plant-f1ii.log); green 14 of 14
  (fix-rs-3.log).
- **F2, NC-6, TE-8, TE-9, F4, NC-4's reap, NC-5's helper, F5's counter:** one batch of 13 plants (plant-web.log): web
  57 of 70, exactly the 13 new tests failing, and self 79 of 80; then all six files restored by `sha256sum -c`.
- **TE-7:** the u64::MAX plant fails in 10.14 s instead of hanging (plant-te7.log). The refused-by-name test has no
  plant (Low, recorded).
- **TE-10:** dry runs show `--commit` required and t5i.mjs's `clean()`. `fail`'s call to `reap` is not shown; only
  `reap` is tested (Low, recorded).
- **F5:** the probe's control reads "page csp 1; worker csp 1", and run 4 reads 0 on every page. Three shipped call sites
  are in the dist (t5-dist/main.js and worker.js, by grep) but no test or browser enters them: `countCsp(document, …)`,
  `countCsp(globalThis, …)` and `forgetFragment(location, history)`. So mark 7's two halves are read with the wiring
  inferred (ADV-U7S-2).
- **F3:** the brief's plant premise was wrong (mine). Its plant after the 3 s gap could not fail (the clock's 2 s
  window). The builder found why and added the leg where it does fail. This is a departure for the report, not a defect
  (ADV-U7S-1).
- **Size:** about 5,193 added (+462 in this pass against my estimate of about 300), under 6,000.

## 3. Findings (CLAUDE.md's scale)
| Id | Sev | What | Taker |
|---|---|---|---|
| ADV-U7S-3 | Medium | His interim rule broken once: the last `git status` ran after `cd`, `sha256sum` and `ls` in one command (notes 1055 :176 (4)). It only read, and moved no evidence. | One line to him in the day message; the next brief to this builder repeats the rule and asks for one command per call |
| ADV-U7S-4 | Low | The Write/Edit-only rule broken once (`tee` of a hash list into the scratchpad, outside the repo); one no-op shell loop; foreground `tail -f --pid` waits, bounded, not the background form. None touched evidence. | The report's slips; the same line to him |
| ADV-U7S-1 | Low | F3's brief plant could not bite (the brief's premise); the narrow leg meets the intent | The report (a departure) |
| ADV-U7S-2 | Low | F5 and NC-5's three call sites are in the dist and never exercised, so mark 7 reads 0 with the wiring inferred | The report; a real-page control at the next T5 (ii) run, if one comes |
| ADV-U7S-5 | Low | U8's ops/browser/config.mjs:60 comment ("no replaceState") is stale after NC-5 | U8's next change (U8-F1 if it is in its files) |
| ADV-U7S-6 | Low | No test pins r1.sh's load_settings to LoadSettings (M5's class); the read above finds them equal | U8's next change, an item-8-like test, inside its 6,230 stop |
| ADV-U7S-7 | Low | `fail` → `reap` not shown; TE-7's refused-by-name test has no plant | The report |

## 4. U7's merge: its conditions (for u7-merge-plan.md, U8's M0-M9 form)
- **U7-M-1, the order.** Main has moved by #70 (code; 2483e9a), so MAIN-68's full take-in comes first (MAIN-52). M0
  stops on any later code, CI or rules move. Builder 2's merge (on his "ready") may come before or after, never
  overlapping, and its 46 files share none of U7's 43.
- **U7-M-2, M1 and M2.**
  - M1: the snapshot's sha256 is 6535c18130cad99881b798a332973b37215c1edad27929dd54178efc00342728.
  - M2: each of the 43 paths is unchanged on the lane since 80b6748 (`git diff --quiet 80b6748 H0 --` the 43 paths). The
    snapshot holds no manifest or lockfile (grep: none).
- **U7-M-3, the tie to run 4, before M4.** The main session runs `bash
  /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u7-run.sh <worktree> <log under .gate/u7/> true`. Its
  header must read "diff 551e9033…; untracked b4811e75…", run 4's tree. Then M3: each of the 43 applied files
  cmp-equal to the worktree's, with the same mode.
- **U7-M-4, M5,** in one main-run.sh call, cheap runs first:
  - `node --test 'ops/**/*.test.mjs'`, then `bash ops/tests/wrappers.test.sh`, then `env CARGO_INCREMENTAL=0 node
    check.mjs` (12 steps), all PASS;
  - the counts recorded as found: test against end-2's 884/981 (MAIN-68 and Builder 2 may move them), web 70, self 80,
    marks 81, redact 0 findings, refs N0 plus the snapshot's new files;
  - ops/browser/node_modules as for U8 (choice 15 (i)).
- **U7-M-5, C1 again** on run 4's 6-mds `pages[0]`, in u8-c1.mjs's form through main-run.sh: a verdict, never a read
  error. NC-6 changed the Recorder. Run 4's records hold the same two kinds, `event` and `second`.
- **U7-M-6, the `ops` step (choice 15 (ii)):** at U7's merge (M4b) or in the first change after it, before Stop B's
  fill. check.test.mjs:2785's list holds "web" and "ops"; STEPS gains `ops`.
- **U7-M-7, u7-report.md:**
  - marks 1-8 read from run 4's summary.json against the brief's words, mark 8 under his restated words with both runs'
    numbers;
  - mark 9 NOT-RUN, with the stand-ins and their limits as section 1 gives them;
  - the wording items (NC-3, NC-8, TE-6, TE-11's records, T5ii-C's note);
  - the departures: F3's plant; HONEST_MAX_LEAD in bots/load.rs and server/tests/carry.rs, outside the file list;
    `npt_client::wire`'s re-exports; C6 reworded; TE-11's limits literal kept (FULL_STATE_MAX is `pub(crate)` in
    npt-net);
  - the carries: TE-5 to Stop B; U6d's per-slot list and T5i-2; CLIPPY_TWINS; DROP-not-REJECT to R8 and Stop B;
    ADV-U7S-5 and -6 to U8's next change;
  - the slips (ADV-U7S-3, -4);
  - the size, about 5,193 of 6,000.
- **U7-M-8, after the push:** CI on the pushed head (gate with gate and smoke, toolchain; push and pull request). Then
  my read of the pushed head in a window, as for U8 (not a round).

## Decisions needed
None. His U7-MARK8B (a) and U7-MARK9 (a) are applied as he wrote them.

## For Mustafa
Nothing to act on. One line for the day message: a builder ran one read-only `git status` after other programs in one
command, against his interim rule; no evidence was touched, and the next brief repeats the rule.

## Notes
- 14:58-15:2x: snapshot and notes sha256s match. Notes 1055 read :100-178 (the fix pass whole). history.md:8068-8088
  read. u7-brief.md:196-209 read. Run 3 and run 4 summary.json read (m5, m7, m8, m9). Run 4's 6-mds `pages[0]` read: the
  same record kinds, and `page_csp_violations` sits in the driver's counters, not in the records. The patch's bots
  hunks read (:249-276; HONEST_MAX_LEAD = 9 at :695). load.rs:182-299 and r1.sh:343-349 (the lane) compared. t5ii.mjs
  grep: no PASS, FAIL or verdict.
- Not checked: the header hashes recomputed from the patch (U7-M-3 does it at the merge), any plant log beyond the
  notes' lines, the dist grep of the three call sites (from the notes).

DONE
