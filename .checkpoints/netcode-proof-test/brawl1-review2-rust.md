# BRAWL-1 round 2: rust-reviewer notes (2026-11-15)

Snapshot under review: wip/brawl1.patch sha256 c2561b6ca54c8cde994fcdad5954c63d332df73f2db038c12aba3d71a0daa49f (sha256sum checked at start).
Packet: brawl1-r2-packet.md sha256 e50a22f70ae876f962510693e03c414700200a7bf3a37a13ad31bf5fb5dfa8b5 (checked).
Round 1 snapshot: ee2973aa...; read copy round 1 .gate/brawl1-snap1; round 2 read copy .claude/worktrees/brawl1.
Logs: .gate/u4r/brawl1-review2-rust/lint-tests.log
(No Edit tool in this seat: this file is rewritten whole with Write at each save.)

## Plan
1. Read ruling's "Round 2" (my list) and "The fix pass, in order" (done).
2. Check read copy's `git diff HEAD | sha256sum` equals c2561b6c.
3. Diff round 1 snapshot to round 2 snapshot (diff the two read copies' changed files).
4. My list: RR-B1-1 (FP-3 test + P2); RR-B1-2 doc route; RR-B1-3 tie; RR-B1-4 rewords and cases; RR-B1-5 (wrap test + P4);
   RR-B1-7 (2) and EA-B1-2; fmt and clippy both builds; recording-path diff = docs + one visibility change.
5. RR-B1-6: no action (ruled). Check report words only.
6. Review changed lines; plant records P1-P4; FP-8 logs; FP-0 runner diffs; report section.
7. Result.

## Progress (all steps done)
- Read copy HEAD 352a245, `git diff HEAD | sha256sum` = c2561b6c (equal); round-1 copy .gate/brawl1-snap1 = ee2973aa.
  Files differing between copies: bots lib.rs, client lib.rs, client endpoint.rs, hash.rs, logs.rs, scenario.rs (6).
  Manifests, precheck.rs, SERIES.md unchanged (cmp). numstat now 1,139/127 = 1,266.
- Interdiff read whole (diff -u line counts: bots 139, client lib 10, endpoint 49, hash 7, logs 107, scenario 32).
- Logs read: fp-0-header (diff ee2973aa), fp-fmt-2 (fmt exit 0 at c2561b6c), fp-check-2 (fmt 0; clippy -D warnings exit 0
  in both builds; bots 3 pass 1 ign both builds; client 2 both; lab 5/6 both), final-fp-check (all 11 PASS, 538/616),
  final-fp-det (uptime first, PASS det-u4 227.3 s), final-fp-regen-tests (1f00d2d5 "as the manifest's"; 13 prints).
  Regenerated det recordings on disk hash 1ec25de7 / 6abf7643 (my sha256sum).
- FP-0 diffs read; `bash -n` both runners: exit 0, no output. u4r-run.sh sha 2f754d12 equals copy 2's start record.
  The "untracked" hash is of the porcelain name list (modified + untracked), not content; it still works as a tripwire
  (any untracked file changes it from the known 10-file value 17cf51d7, or from e3b0c442 at M3). Main folder's
  spikes/netcode-proof status and diff are empty today (e3b0c442 both), so M3's expectation is reachable. No finding.
- client lib.rs:250-251 hold SOLDIER_FLAGS / SOLDIER_DEAD (soldiers()' doc cites them right).
- Recording-path check (python difflib over the two copies, lines before each #[cfg(test)]): every changed line outside
  test modules is a comment (bots :59-62, :130-132, :250-253; client :246-249, :351-353; logs.rs :4-7 //!; scenario
  :370, :1432-1434) except scenario.rs:396 `fn new` (was pub(crate)). hash.rs change at :1561-1564 is inside tests
  (mod at :1418). HOLDS.
- EA-B1-2: only caller scenario.rs:389 (grep crates/, check.mjs, ops/). Clippy clean both builds. FIXED AS RULED.
- RR-B1-7 (2): scenario.rs:367-371, words equal to round 1's, no lone fragment. FIXED AS RULED. (1) deferred: logs.rs
  :584-599 unchanged (cast still at :596); FP-9 item 6 homes it.
- RR-B1-4: :1432-1434 reworded as ruled (true: burst block :1504-1530 follows); :2576 as ruled (true); cases :2588-2607,
  arithmetic re-done: 143,165,566 x 30 + 300 = 4,294,967,280; 143,165,567 -> 4,294,967,310; message equals format at
  :1515-1520. due_ns(4.29e9 ticks) ~ 4.29e18 ns < u64::MAX, so no debug overflow. FIXED AS RULED.
- RR-B1-3: tie bots lib.rs:513-526 (assert :519-524), Record.dead :502, counted :568-570, printed :675/:686, short copy
  `r.dead > 0` :700; client doc :246-249. npt-sim health is u16, set 0 with FLAG_DEAD at world.rs:1630-1634, both reset
  at revive :1176-1188, so the tie is exact. Dead counts 1,190 / 1,334 (short), ~8,000 per full run. FIXED AS RULED.
- RR-B1-1: logs.rs:871-967 test; P2 fails at :924 (0 vs 1). Room.jsonl both ticks [3, 1]; summary both forms. The
  `--record` addition is needed (parse refuses --logs 0 alone, scenario.rs:1486-1488; record only stored, :1456/:644).
  `Some(dir.as_path())` is required (Option<&Scratch> would not coerce). FIXED AS RULED.
- RR-B1-5: endpoint.rs:460-488; P4 fails at :483 (11 vs 22). Also kills `a.tick < b.tick` and last-entry mutants (ring
  order: slot 0 = tick 0 first, slot 31 = u32::MAX last). FIXED AS RULED.
- RR-B1-2: doc route at bots lib.rs:130-132 and :250. FIXED AS RULED, with RR2-B1-1 below.
- RR-B1-6: no action, as ruled (hash.rs:239 unchanged; FP-9 item 5 records it; ADV-PT-L3 follow-up).
- Plants: in-memory re-creation of each plant on the current files gives exactly the recorded planted sha256
  (P1 77be9b00, P2 b902a7ee, P3 7a77c71a, P4 6aeeec0a); current files equal the before/after values (83a8541b,
  83008fce, 2aca00c8). Logs: P1 short x2 fail at lib.rs:700 (Skips area; :699 + the planted line), full fails :750
  listing Skips all 3 seeds; P2 fails logs.rs:924 measured count 0 vs 1; P3 endpoint.rs:446; P4 endpoint.rs:483.
- My run, lint-tests.log (u4r-run.sh -x -k, 07:50:28-07:51:06 UTC, header diff c2561b6c, free 6,591 MiB), every exit 0:
  fmt --check; clippy -D warnings measured and --all-features; lab brawl1+fp3+r10_carry_82: 4 passed both builds;
  client brawl1: 2 passed both builds; bots tests::brawl1: 3 passed 1 ignored both builds (11.00 s; 10.82 s), short
  prints equal to the builder's (1575/1629 cm, dead 1190/1334).

## Findings (one line each, as found)
- RR2-B1-1 (Low): bots lib.rs:131-132 "so x² + y² stays below 2^31": at (-32,768, -32,768) it is 2^31 exactly
  (POSITION_XY min -32,768, sim grid.rs:109; quant.rs:212 accepts k = min). Fix: "is at most 2^31". No overflow either
  way; doc only; follow-up. (Builder flagged it, report section 6.)
- RR2-B1-2 (Low): report "FP-6 (a) ... Its only caller is :390": in round 2's copy the caller is scenario.rs:389 (the
  doc rewrap removed a line above it). Fix: ":389" in the report words. Report only.

## Verdict
APPROVE WITH FINDINGS: all seven round-1 findings fixed or deferred as ruled; two Lows (follow-ups, no push).
