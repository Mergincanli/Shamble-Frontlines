# BRAWL-1 review round 2: test-engineer seat (notes)

Snapshot under review: wip/brawl1.patch sha256 c2561b6ca54c8cde994fcdad5954c63d332df73f2db038c12aba3d71a0daa49f (checked by sha256sum at start; the read copy .claude/worktrees/brawl1 gives the same `git diff HEAD | sha256sum`).
Packet: brawl1-r2-packet.md sha256 e50a22f70ae876f962510693e03c414700200a7bf3a37a13ad31bf5fb5dfa8b5 (checked).
Round 1 snapshot: ee2973aa...; round 1 read copy .gate/brawl1-snap1 (gives ee2973aa); round 2 read copy .claude/worktrees/brawl1.
My log: .gate/u4r/brawl1-review2-test/r2-tests.log (u4r-run.sh -x -k; header diff c2561b6c; exit 0).
(No Edit tool in this seat: this file is rewritten whole with Write at each save.)

## Plan (done)
1. Confirm both read copies' hashes. 2. Interdiff snap1 vs worktree. 3. The ruling's test-engineer list (a)-(g). 4. FP-0 runner diffs. 5. FP-9 report words. 6. My own run. 7. Verdict.

## Progress log
- Both read copies checked: snap1 = ee2973aa..., worktree = c2561b6c... (10 files ` M`, nothing untracked). numstat 1,139/127 = 1,266.
- Interdiff read whole (bots lib.rs, client lib.rs, endpoint.rs, logs.rs, scenario.rs, hash.rs; manifests, precheck.rs, SERIES.md unchanged between snapshots).
- #[test] counts snap1 -> snap2: bots lib.rs 3->4, endpoint.rs 14->15, logs.rs 2->3, scenario.rs 20->20, hash.rs 12->12, client lib.rs 2->2: exactly +3, none removed. Matches 535->538 / 613->616 (final-fp-check.log:9).
- Plant file hashes: current bots lib.rs 83a8541b, logs.rs 83008fce, client lib.rs 2aca00c8, manifests a2faf3ea / 5ccf57f0: all equal to the "before/after" records.
- Planted sha256 values REPRODUCED by a no-write pipeline (awk/sed | sha256sum) from the reviewed files: P1 77be9b00 (`        return walk;` inserted before :255), P2 b902a7ee (:588 -> `.filter(|_| false)`), P3 7a77c71a (:359 Hash filter removed), P4 6aeeec0a (:360-366 -> `.max_by_key(|t| t.tick)?;`). Each plant was exactly the ruled line on the reviewed bytes.
- FP-0: both .diff files one line each, as ruled; `bash -n` both exit 0, silent. Every fix-pass log header carries the diff line; final-fp-* show c2561b6c; fp-0-header shows ee2973aa; plant logs show their own planted-tree hashes. Main folder's spikes/netcode-proof porcelain is empty now (e3b0), so M3's expectation is reachable.
- P1 (fp-plant-1.log): short copy fails in both builds at planted lib.rs:700 (= clean :699, the area assert_eq) on Skips (7,728 soldier-ticks, first (64395, 11, 1)); EveryTick half 0 under the plant (closed circle, 2,175 cm), recorded as the baseline. Full test fails at planted :750 listing Skips for all 3 seeds and EveryTick for seeds 1, 2. Bite rule holds on area; door-entry bar correctly not added.
- P2 (fp-plant-2.log): fails at logs.rs:924 "measured: one slot-tick / left: 0 / right: 1". P3 (fp-plant-3.log): fails at endpoint.rs:446 with tick 105's record. P4 (fp-plant-4.log): fails at endpoint.rs:483 (last assert) with u32::MAX's record. All as ruled.
- FP-2 command sequence re-computed by my own splitmix64 (python -I, no file): phase 7; walks k0-3 [-117,49], k4-7 [-127,0], k8-11 [-117,-49], k12-15 [-89,-89], k16-18 [-49,-117]; toward_centre(1450,200) = [-125,-17], (1480,30) = [-126,-2]. Each new check's expected value differs from the twin's walk; LEASH_OUT pinned to exactly 1,400 with `>`.
- FP-8: final-fp-check.log 11 PASS, 538/616. final-fp-det.log PASS 227.3 s, uptime first, non-cargo 213.3 s; det-u4's regenerated files (worktree .gate/netcode-proof-test/det-u4/recordings/) sha256 = 1ec25de7... / 6abf7643... (my sha256sum), equal to det-recordings.json:4-5; regeneration refuses a mismatch before "as the manifest's" (hash.rs:484-491). final-fp-regen-tests.log:21 1f00d2d5 as the manifest's. 13 printed lines IDENTICAL to round 1 after the stated normalisation (my own diff).
- My run r2-tests.log (07:51-07:52 UTC, header diff c2561b6c): bots brawl1 3 passed 1 ignored both builds (11.17 s, 10.82 s); client brawl1 2 passed both builds; lab figure + fp3 + r10_carry_82 (x2) 4 passed both builds; full test release 1 passed 24.31 s. All 4 short and 9 full prints byte-identical to final-fp-regen-tests.log. Free 6,595 MiB at start.
- HEALTH is an exact integer field 0..1023 (seam.rs:214), so the tie `dead == (grid[6] == 0)` has no quantisation edge.
- Dead figures quantised at ~29 soldier-ticks per death (respawn 1 s, scene.rs:186): equal 8,091 in three runs = equal death counts.
- Scratch helper: per test name + pid, removed at start (scenario.rs:1734-1747): no leftover-folder flake for create_new.
- LOGS0-RECORD: summary_line reads only `s.logs` (scenario.rs:1615), so the added `--record` cannot change the branch under test; option (a) is sound.

## Round-1 findings state
- TE-B1-1: fixed as ruled. bots lib.rs:565; P1 recorded, bite rule held on area; bar not needed.
- TE-B1-2: fixed as ruled. logs.rs:872-967, P2 at :924; bots area test lib.rs:704-732.
- TE-B1-3: fixed as ruled. lib.rs:380-388, :408-413, :418-429, :338-341.
- TE-B1-4: fixed as ruled. endpoint.rs:437-450 + P3; :459-488 + P4 (the wrap case replaces 127/128 as ruled).
- TE-B1-5: deferred as ruled (recorded). brawl1-build.md:587-591 withdraws the obstacle reading (my mechanism was wrong); U4g words in the ruling.
- TE-B1-6: fixed as ruled (by the ruling's tie, not my separate kill test). lib.rs:518-524, :502, :568-570, :700; client lib.rs:245-249.
- TE-B1-7: fixed as ruled. u4r-run.sh:67, main-run.sh:42; .diff files; every fix-pass log carries the line.
- TE-B1-8: deferred as ruled. brawl1-build.md:592-594; follow-up home :615-616.

## New findings
- TE-B1-R2-1 (Low): bots lib.rs:404 + :408-413. The kept-state check cannot tell "dead keeps the leash's state" from "a dead soldier's position updates it": the dead input (1450, 0) is past the outer radius, so a leash that recomputes `steering` from a dead soldier's position, then returns the walk, still has steering true at k12 and passes the whole unit test. Near-equivalent mutant in practice (homes 10 m out, scene.rs:153), so Low. Fix: lib.rs:404 `give(Some(([1_450, 0, 150], true)))` -> `give(Some(([0, 0, 150], true)))`, messages unchanged; walk at k10 [-117,-49] != toward_centre(0,0) [0,0] so "dead: the walk" still can fail, and k12 then fails for the update-from-position, ignore-dead and dead-resets mutants. Follow-up.
- TE-B1-R2-2 (Low): u4r-run.sh:67, main-run.sh:42 (my own round-1 form). The "untracked" field hashes the whole porcelain status, modified files included: every fix-pass log reads "untracked 17cf51d7..." while nothing is untracked, and no hash covers an untracked file's content. M3's e3b0 expectation still holds. Fix (later runner edit, outside the change set): `untracked $(git ls-files --others --exclude-standard -- . | sha256sum | cut -d' ' -f1)`. Follow-up.

## Verdict
APPROVE WITH FINDINGS: two Lows, both follow-ups; no push; no Critical or High; nothing opens round 3.
