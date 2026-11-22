# U4f2 review round 2 (2026-11-21/22): the test seat (plus the rust seat's FP-8 items and step 2, under NPT-TRIM)

Read only; this file is my one write (no Edit tool here: rewritten whole with Write each time, as the rules allow).
Scope: the fix pass's diff 3441d47f..7c362a63 (round 1's snapshot e9a0ae3e on a289def against the final snapshot
e33d14f0 on a289def; 7 files, +638 -88), the builder's record u4f2-build.md Blocks 9-10 (:268-591), the logs under
.gate/u4f/. Rules: u4f2-panel-brief.md:65-79. Nothing run; every figure below is read from a file named beside it.
Line numbers are the worktree's (agent-aa62d31b6fa5b9807) unless a log is named.

## Plan (DONE)
1. Rulings (advisor-u4f2-r1.md :240-423; advisor-u4f2-parta.md whole), his F-B-COUNT (a) and NPT-TRIM
   (history.md:7544-7593), the record's Blocks 9-10.
2. The diff, file by file; FP-2's own change isolated from part A.
3. TE-U4F2-1's proof: the rule against his words, the f33 case's five cases, T-K7's extension, the plant, the restore.
4. Every new plant and restore. 5. FP-6, FP-7, FP-8's tests; FP-8's arithmetic and bounds.
6. R2-R5 against the acceptance lines; "report the number left out beside the check on every run".
7. Test-step time before R6; step 2's clippy and doc. 8. The departure. 9. Findings, table, verdict.

## Block 1: what I checked, and how (DONE)
- FP-2's own change: the diff of the committed snapshots `git diff f0da659 096a49b --
  .checkpoints/netcode-proof-test/wip/u4f2-build.patch` (f0da659, 2026-11-20 22:35, F-G's snapshot by time and content;
  096a49b, 2026-11-21 19:56, FP-2's; the main folder's wip patch is sha256 e33d14f0..., 190,245 B by `wc -c`). The
  record's sizes "190,233 B" and "184,639 B" read 12 B under `git cat-file -s` (190,245 and 184,651): a slip in the
  record's figure, no effect (the hash is what binds).
- The World's step order (sim world.rs:3-15, :2151-2199): inputs (0), movers (1), commands (2), shots (4) and
  projectiles (5) (the only `damage` calls: :1664, :1727; a death at :1698-1702), respawns (7), records (8). The lab's
  `run` scripts no spawn or attribute (`room.script` only in tests: logs.rs:1058, scenario.rs:3280, precheck.rs:381).
  So the forger's health in the records after tick t-1 (scenario.rs:1470-1473) is its state at tick t's command phase:
  the lab's flag is exactly his "as that command ran". T-K7's flag (bots lib.rs:2242-2269: the dead bit, asserted
  equal to health 0) is the same, plus its own test spawn (`spawn != Some(k)`, :2006-2011), which the lab never has.
- Callers of `kind_7_left_out`: bots lib.rs:180 (definition), :2082 (T-K7, `#[cfg(test)]`), lab forged.rs:106 (inside
  `Attribution::tick`; an Attribution exists only on a forged, non-twin run: scenario.rs:1368-1369). The scenario.rs
  wiring sits inside `if let Some(a) = attribution.as_mut()` (:1465-1474); outside it only the empty `dead_before`
  declaration (:1373).

## Block 2: TE-U4F2-1's proof (DONE)
- **The rule against his words** ("Leave kind 7's presses out of that count when the server's own state shows its
  soldier dead as that command or the one before it ran"): forged.rs:99-113 decides at an item's first run, with
  `kind_7_left_out(dead now, dead at the first run of seq - 1)`; `None` when d-1 never ran; left-out items leave both
  `sent_count` (:184) and `unrejected_count` (:153), as the (a) text he chose words it ("the 1,000-per-kind check still
  applies to what is left"); `dead_left_out_count` on every record (:188), 0 for other kinds (the loop's `f.kind == 7`
  filter, :101). The flag is the server's World (Block 1). Matches.
- **The f33 case's five cases** (forged.rs:446-464; PASS u4f2-fp2-tests.log:102, 0.00 s): item 2 dead at d (out),
  item 4 dead at d-1 and alive at d (out), item 6 alive at both with no record (unrejected), item 8 alive with a record
  (rejected), item 10 with d-1 never run (not out). Each wrong answer moves the triple off [3, 1, 2]: case 1 or 2 wrong
  gives [4, 2, 1]; case 3 [2, 0, 3] or [3, 0, 2]; case 4 [3, 2, 2]; case 5 [2, 1, 3]. So all five bite, though cases 1
  and 2 fail with the same triple (one test name, not one message each: fine).
- **T-K7 extended** (bots lib.rs:2066-2110; PASS u4f2-fp2-tests.log:121-122, 1.83 s): every item outside the window
  carries its record (unrejected 0); left out 3, printed: (n 12, d dead), (576, d dead), (588, d-1 dead, d alive, no
  record); cases [46, 2, 1, 1, 1]; every tick's hash equal (the twin still covers every press, the left-out ones too).
  T-K7 applies the shared rule to its own World flags; it cannot run npt-lab's Attribution (the record's design point,
  u4f2-build.md:498-501). The lab's Attribution meets real World deaths only in R2 (Block 4; TE-U4F2-R2-2).
- **The plant** ("the window ignored", `false && (...)`): u4f2-fp2-plant.log, one `-x -k` call on tree 6bae3c94...:
  the f33 case fails by name at forged.rs:463 ([5, 3, 0] against [3, 1, 2], :27-30) and T-K7 by name at lib.rs:2101
  (n 12, 576, 588 unrejected outside the window, :69-74). Restore: step 2's tree (u4f2-fp2-end.log:3) is 4e15fbde...,
  the pre-plant tree of u4f2-fp2-tests.log:3, and the untracked hash abb7799a... is unchanged throughout. Proven.
- **At length (R2):** kind 7 left out A 18, B 8, unrejected 0 (u4f2-r2.log:43, :58). R2's four fate logs equal
  R2-pre's (6bb87ea5..., 1852a5b8..., 603bf064..., b0b9808d...: u4f2-r2.log:21-30 against u4f2-r2pre.log:21-30), so
  the same items ran; R2's unrejected 0 puts every unrecorded item inside the left-out set, and 18 = 18 (8 = 8) makes
  the two sets equal: every item R2-pre read unrejected was a dead-window item and no recorded item was left out. The
  judge's "every forged command rejected and counted" passes at 0 on both (u4f2-r2/verdict.json:289-293, :328-332).
- **Gaps (Low):** no test meets the lab's own death wiring (TE-U4F2-R2-2); the "first run only" guard and the kind-7
  filter have no negative case (TE-U4F2-R2-3).

## Block 3: every new plant and its restore (DONE)
| Plant | Fails by name | Restore shown by |
|---|---|---|
| FP-1 (fire held on a reload item's d-1) | T-K7 lib.rs:2026 then, f15 kind 7 :1835 (u4f2-fp1-plant2.log:18-45) | the final `rest` (`k == 1 || !reload(n + k)`) and T-K7 passing on every later tree |
| FP-3 (`Some(3)` from two ticks) | fp3 at lib.rs:1609, the 60 Hz pin (u4f2-fp3-plant.log:24) | fp3 passing in u4f2-fg-tests2.log |
| FP-5 plant 9 (right form) | fp5 at lib.rs:2168; f16 22 fail, A and B (record :319-326) | fp5 and f16 passing in u4f2-fp2-tests.log |
| FP-6 a / b (each slot filter) | forged.rs:365 "the room's" / :368 "the sim's" (u4f2-fp6-plant-a.log:42, -b.log:42) | untracked hash back to aef9b178... (u4f2-fp7-tests.log:3); b's line :368 shows a was out before b ran |
| FP-7 plant 10 (room kind 4 off) | 22 fail, each naming "slot 3 (kind 4)" (record :343-346) | room input.rs absent from the fix pass's diff |
| FP-4 (lag client standing) | f20 at scenario.rs:4258 "a burst tick moves" (u4f2-fp4-plant.log:24) | `lane_walk` in the final scenario.rs:1248-1250 |
| FP-8 a / b (counted sent + 1; dropped 0) | f16 kinds 8, 9, 11 at :4552 and the fp8 case / fp8 at forged.rs:408 (u4f2-fp8-plant-b.log:44) | untracked back to 83a612c8... (u4f2-fp-end.log:3) |
| F-G (`every >= r`) | fp3 at lib.rs:1631 "2 and 3 refused at 30 Hz" (u4f2-fg-plant.log:24) | tree a9027b71... on every early run (u4f2-r0-times.log:3 on) |
| FP-2 (window ignored) | f33 case forged.rs:463 and T-K7 lib.rs:2101 | tree 4e15fbde... (Block 2) |

Note: the runner's `diff` hash covers tracked files only; forged.rs is untracked, so its plants show in the
`untracked` hash (FP-6, FP-8), as the table reads them. Every plant bites by name and every restore holds.

## Block 4: FP-6, FP-7, FP-8 (with the rust seat's arithmetic and bounds) and R2-R5 (DONE)
- **FP-6** (forged.rs:401-416): room kind 10 and the sim's Cooldown on SLOT + 1 each leave item 2 unrejected; both
  plants bite (Block 3). Met.
- **FP-7** (scenario.rs:4446-4497): the culprit on a fate-entry difference by link, on a hash difference by the slots
  whose Soldier records differ; `world_hashes` and `soldiers` are pushed in the same block (:1586-1596), so index i
  matches. Plant 10 names slot 3 (kind 4). Met.
- **FP-8, bounds:** the parser takes 2 to 1,000,000 (scenario.rs:2277-2283; c_u4f2_1 refuses "1000001" and
  "18446744073709551615", takes "1000000"); `forger` refuses when `every.checked_mul(8)` fails (lib.rs:588); the only
  product of `every` in the crate is kind 10's period `every x 8` or `x 4` (lib.rs:997), so 8 bounds both; f15 pins
  `u64::MAX / 8 + 1` refused for every kind. `fire_rest_at`'s `interval_us as u64 * tick_hz as u64` cannot overflow
  (two u32s); SS 68,571 us gives 2,057,130 and 4,114,260 units, so 3 and 5; 66,666 and 66,667 us at 30 Hz give
  1,999,980 (None) and 2,000,010 (3), as fp3 pins. `items.len() - left_out.len()` (forged.rs:184) cannot underflow:
  `left_out` takes only keys of `items` (:107) and nothing removes from `items`. Met.
- **FP-8, counts:** `counted_count` on kinds 8, 9, 11 only (forged.rs:189-193), equal to `sent_count` in f16
  (scenario.rs:4563-4566) and 1,523 = 1,523 in R2 and R3; a reused sequence adds to `unrejected_count` (fp8 case,
  sent 1, unrejected 1); the dropped count on every record (fp8 case, 3). Met.
- **FP-8, the zeros** (scenario.rs:4517-4543): no nonzero count found, but the exemptions go by slot number in every
  suite, so the clamp pair keeps one counter unchecked on slots 0-10 (TE-U4F2-R2-4, Low).
- **FP-8, projection** (c-u4f-runs.mjs:65-67, :84-85): 65,188,828 B is the sum of hand-long2's three logs
  (20,063,654 + 71,027 + 45,054,147: `ls -la`), and c-u4r-runs.mjs's 56 MB is 14.1 % under it. Met.
- **FP-8, the rest:** the -21 rows' "in a building: 0" (c-u4f-r2.mjs:83-84), RefusalRecord's docs (room.rs:43-48),
  `twin()`'s doc (lib.rs:603-610), the one-lap limit (forged.rs:19-22). Met by reading.
- **R2** (advisor-u4f2-r1.md:281-285): every kind sent at least 1,000 (least 1,505 A and 1,515 B, kind 7's after the
  left-out; u4f2-r2/verdict.json:278-281, :317-320); every `unrejected_count` 0 (u4f2-r2.log:37-62); kind 7's left out
  18 and 8, every other kind 0; twin mismatch 0 (judge 0, :296-299, :335-338; 18,300 ticks equal, u4f2-r2.log:67, :70);
  shots 685.771667 and 691.523333 (verdict.json:64, :136), every per-run PM-0001-2 check passing; slot 6 under B with
  its command missing on 0 of 18,000 measured ticks and `commands_repeat_count` 0 (u4f2-r2.log:63-64; the lab writes
  no `commands_rerun_count`, so the reader's proxy, which covers the measured ticks only); `counted_count` = `sent_count`
  for 8, 9, 11. "PM-0001-2 passing" cannot be met by 2 of its 3 SS seeds and no ES run (NOT-RUN by design): the
  per-run band checks are the reading. All met.
- **R3** (r1:287-288): kind 1 the only kind unrejected, 1,523 of 1,523; kind 7 unrejected 0, left out 10, sent 1,513
  (u4f2-r3.log:23-33); shots 673 from the summary line (:20), 3 over the 670 line, the thinnest run (TE-U4F2-6's
  band, his); the judge reads "every forged command rejected and counted" 1,523, failing as the bite should
  (verdict-bites.json:172-174). Met.
- **R4** (r1:289-290): `lagswitch_excess_move_um` max 600,000 on 3 ticks, `lagswitch_cmds_count` max 11
  (u4f2-r4-bite.log:23). Met.
- **R5** (r1:291-294; parta :97-105): every set-21 line and r2's p1, p7, p14, p17 at short settings, exit 0; the twin
  checks pass on both options; the fast-clock per-tick hashes equal hand-a's, 450 ticks each (u4f2-r5-bites.log:70,
  :73); R5b's forged lines, both judge calls, every run's fate log equal to its early run's (record :532-546). The
  measured judge call read only R5b's new folder (the early non-forged lines' verdicts stand in u4f2-r5/verdict.json):
  the ruling's "may", and no PM-0001-21 part spans the two folders. Met.
- **"Report the number left out beside the check on every run":** the lab writes `dead_left_out_count` in every
  forged, non-twin run's records, beside `sent_count` and `unrejected_count` (a twin writes none: scenario.rs:4567-4570).
  No output a set produces carries it: verdict.json shows the check's value only (u4f2-r2/verdict.json:289-293), and
  runs.txt and r2.txt carry npt-lab's summary line, which lacks it (u4f2-r2.log:21; c-u4f-runs.mjs:163). The record
  states it for R2 (A, B), R3 and R5b's forged A and B, not for R5b's p1, p7, p14, p17 or its 11 forged bites
  (u4f2-r5b.log:45-71 read only forged A and B). TE-U4F2-R2-1 (Medium).
- **What -21's set report must print**, for each forged-suite run of the set (the forged part's measured runs, A and B
  at each seed; the 11 forged no-* bites) and of the r2 rows p1, p7, p14 and p17: kind 7's `dead_left_out_count`
  beside the judge's "every forged command rejected and counted" value for that run, with kind 7's `sent_count` (after
  the left-out) and `unrejected_count`; one line that every other kind's count is 0 on every record (or the
  exceptions); for each twin, "no forged_kind record: a twin forges nothing; the twin check covers every press, the
  left-out ones included"; the clamp, fast-clock and lag-switch runs: not applicable. And the reading in a line of its
  own (F-B-COUNT (a), Mustafa 2026-11-21, reviewed in round 2 by the netcode and test seats).

## Block 5: step 2, the test-step time, the departure (DONE)
- **Step 2** (u4f2-fp2-end.log, tree 4e15fbde..., R2's and R3's tree): `cargo fmt --check`, clippy host measured and
  `--all-features`, clippy wasm32 (check.mjs's WASM_CRATES) both, `cargo doc` host and wasm32 both, each exit 0
  (:5, :93, :219, :244, :256, :288, :320, :336, :352). Met.
- **The test step against 1,207 s, before R6:** round 1's counted runs 1,124.2 s and 1,125.1 s (end-a3, end-b). The
  fix pass adds per build, alone in debug: T-K7 1.83 s, c_u4f2_1 +4.17 s (was 0.00), fp5 0.13 s, fp3, fp6, the two
  fp8 cases and the fp2 case 0.00 s (u4f2-fp2-tests.log, record :395-399, :518-520); f20 12.84 s against 12.61 s for
  f28's lag-switch run of the same shape (u4f2-times-rest.log:45; no earlier f20 alone time); f16 x 24 57.87 s against
  58.08 s; R0 no rise from `Summary::soldiers` (46.61 s against 48.67 s). About 6 s a build, so about 1,131 s against
  1,207 s, inside the 30 s allowance (r1:389): a projection, read whole only by R6.
- **The departure** (parta :102-105): FP-2's diff also touches bots lib.rs (one `pub const fn` and T-K7) and
  scenario.rs's f16 (test code). Reading the snapshot diff: the only non-test lines outside forged.rs and SERIES.md are
  `kind_7_left_out` (reached from the lab only through `Attribution::tick`, built only on forged non-twin runs) and the
  wiring inside `if let Some(a) = attribution` (Block 1). No command, datagram, sim, room, lag-switch, fast-clock,
  clamp, bite or forger path changes, and no `cfg(feature = "bites")` line is added. So no non-forged line can reach
  FP-2's code. The lag-switch A proof confirms it on the line with the widest P1 path: its lab.jsonl:53 fate-log
  sha256 (0576dea2de31df67172f515e5887c60832ba7cb789f5be11064f300bac25...) is the same in both runs (u4f2-r5 and
  u4f2-r5c, read here), and 481 per-tick hashes are equal (u4f2-r5-lagproof.log:23); R5b's forged lines and R2 also
  kept their early fate logs. One equal line alone would not prove the clamp, fast-clock or bite lines; the call-site
  reading does, and the two together suffice. **Ruling asked: no re-run of R4 or R5's non-forged lines;** record the
  departure in U4f2's report (the letter widened by one predicate, chosen before the build so one plant fails both
  tests).

## Findings (ranked)
- **TE-U4F2-R2-1 (Medium): his "report the number left out beside the check on every run" has no carrier in what a set
  produces, and the record misses it on 15 forged runs.** u4f2-r2/verdict.json:289-293 (the check's value, no
  left-out count); c-u4f-runs.mjs:163 and c-u4f-r2.mjs:176 (runs.txt and r2.txt carry the summary line, which lacks it:
  u4f2-r2.log:21); c-u4f-r2.mjs:61 (p1's expect names the check, not the count); u4f2-r5b.log:45-71 (read only for
  forged A and B). What goes wrong: -21's set report, built from these files, would show "rejected and counted: 0"
  with no left-out count beside it, which leaves out half of his instruction. Fix: c-u4f-runs.mjs (set 21) and
  c-u4f-r2.mjs (p1, p7, p14, p17) add to each forged run's line kind 7's left-out, sent and unrejected counts, read
  from its lab.jsonl `forged_kind` record (about 10 to 20 lines, `node --check`, one dry read on R2's folder; no lab
  log changes, so nothing re-runs); the set report prints Block 4's table; the build record gets the 15 missing R5b
  figures from the logs (no run). Minimum, if deferred: that table written into the set step's acceptance lines. A
  route through the judge's verdict.json (a marks.json `report` entry, like PM-0001-2's "effects per soldier") would
  change marks.json: Mustafa's.
- **TE-U4F2-R2-2 (Low): the lab's own death wiring has no test in the step.** scenario.rs:1465-1474 (`dead_before` from
  the records after the tick, health field 6, slots under FORGERS, assigned after `a.tick`); f16's
  `dead_left_out_count` 0 (scenario.rs:4557-4562) holds trivially, since its short pair kills no forger; T-K7 checks
  the shared rule on its own flags. Only R2's 610 s run (outside check.mjs) shows the wiring right (Block 2). What goes
  wrong: a later change (the assignment moved before `a.tick`, a wrong field) passes check.mjs; a too-wide window
  hides an unrecorded item. Fix: a small pure `dead_forgers(&[EntityRecord]) -> Vec<u8>` with a hand-made-record case
  and a comment pinning the order (about 15 to 25 lines), or an f16-style short forged run with slot 6 killed by a
  scripted set-up (heavier). Follow-up.
- **TE-U4F2-R2-3 (Low): two negative cases missing from the fp2 case.** forged.rs:102-104 (a sequence's re-run is
  skipped: "first run only") and :101 (`f.kind == 7`). Untested: an item that first runs alive with no record and is
  re-run while dead stays unrejected; another kind's forger dead at its items' runs keeps `dead_left_out_count` 0 and
  its `sent_count`. R2's other kinds' 0 shows nothing unless those forgers died near their items, which is not
  shown. Plants: drop the `continue`; drop `f.kind == 7 &&`. About 10 lines in the fp2 case. No reach in a judged run
  today (slot 6's re-runs 0 at P0; both lines right by reading). Follow-up.
- **TE-U4F2-R2-4 (Low): f16's zeros check exempts by slot number in the clamp pair too.** scenario.rs:4518-4531 (`match
  s + 1` in every suite) against the clamp pair (scenario.rs:4151-4163: 16 clients, slot 0 the clamp, the rest
  honest): slots 0-10 each keep one counter unchecked, against the ruling's "every honest client's, 0"
  (advisor-u4f2-r1.md:410). Fix: `own` empty unless `suite == "forged"` (about 2 lines; f16 re-runs, nothing else).
- **TE-U4F2-R2-5 (Low): kind 7's tightest accepted interval has no World-level case.** F-G takes `rest + 1` (4 at 30
  Hz: lib.rs:590); f15 checks it at the command level (lib.rs:1870), T-K7 only at 12 (:1939). Outside -21's set (12);
  any failure there would be a false FAIL, never a false pass. Fix: run T-K7's body at `rest + 1` as well (about 10
  lines, about +1 s), or a record line "intervals 4 to 11 unproven in the World". Follow-up.
- **TE-U4F2-R2-6 (Low): record lines.** The snapshot sizes read 12 B low (Block 1); R2's "PM-0001-2 passing" is
  NOT-RUN by design (the per-run checks are the reading); the B re-run proxy (`input_missing`) covers 18,000 measured
  ticks, not the warm-up's 300 (24 of kind 7's items are forged there); R3's 673 is the summary line's whole number.
  Record lines in U4f2's report; no code.

## Claim, proof, gap
| Claim | Proof | Gap |
|---|---|---|
| FP-2 builds his (a) | forged.rs:99-113, :153, :184, :188; bots lib.rs:180; the flag is the World at the command phase (Block 1) | none in the rule |
| The f33 case proves all five cases | forged.rs:446-464, PASS; each wrong case moves the triple | re-run and other-kind negatives (R2-3) |
| T-K7 extended | lib.rs:2066-2110, PASS, left out 3 named | shared rule only, not the lab's Attribution or flags (R2-2) |
| One plant fails both by name, restored | u4f2-fp2-plant.log:27-30, :69-74; tree 4e15fbde before and after | none |
| R2 on its lines | u4f2-r2.log:37-70; verdict.json:278-339 | PM-0001-2 NOT-RUN by design; proxy over measured ticks (R2-6) |
| R2-pre's 18 and 8 were dead-window items | equal fate logs; 18 = 18, 8 = 8, unrejected 0 | none |
| R3, R4, R5 on their lines | u4f2-r3.log:20-33; u4f2-r4-bite.log:23; u4f2-r5-bites.log:70, :73; R5b logs | shots 673, 3 over 670 (his band) |
| The number left out reported on every run | the lab's records on every forged run | not in any set output; 15 R5b runs unstated (R2-1) |
| FP-6, FP-7 | fp6 and plants :365, :368; plant 10 naming slot 3 | none |
| FP-8 bounds and arithmetic | c_u4f2_1; f15; lib.rs:588, :997; forged.rs:107, :184 | none |
| FP-8 counts and projection | f16 :4563-4566 and plant a; fp8 cases and plant b; 65,188,828 B | clamp pair exemptions (R2-4) |
| FP-3, F-G, FP-4, FP-5 | pins and plants (Block 3); R4 | no World case at rest + 1 (R2-5) |
| Step 2 clean | u4f2-fp2-end.log, all exit 0 | none |
| Test step under 1,207 s | about 1,131 s projected | R6 reads it whole |
| Non-forged lines unchanged (the departure) | call sites (Block 1) and the lag-switch A proof | none: no re-run |

## Verdict: APPROVE WITH FINDINGS
No Critical or High: no third round on my account. One Medium (TE-U4F2-R2-1) to be fixed or deferred in writing before
-21's set; five Lows as follow-ups or record lines (none triggers a push under NPT-TRIM). Nothing here touches a pass
mark, the judge, marks.json, money or main, except R2-1's judge route if chosen (a marks.json report entry: Mustafa's).
