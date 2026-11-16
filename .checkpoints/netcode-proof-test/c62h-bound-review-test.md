# C62H-BOUND build review, round 1: test-engineer (effort xhigh)

Read only. Diff 044fab188419e8859dbca699d851563c44c078c9..42b964618eed741f65ac04c1e5a06da0a82524d0 in
/home/user/Shamble-Frontlines/.claude/worktrees/c62h-bound (branch c62h-bound-build, HEAD 42b9646, clean). No cargo,
no build, no test run. (No Edit tool in this agent: this file is rewritten whole with Write after each block.)
Scratch scripts (read-only python3 -I): /tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/scratchpad/
mcheck.py, scheck.py, fx_scan.py, bitecmp.py.

## Plan
1. Read brief (sha 7936074c... checked), texts v3, freeze, design, rulings, step1-4 returns, build notes.
2. 13:35 read: marks.json, judge (figures.rs, engine.rs, fixtures.rs, fixture files); sha before/after; stat at close.
3. SERIES.md and README.md against texts v3.
4. Plants CB-P1..CB-P18 logs. 5. Test counts. 6. B-2. 7. The four scripts and their dry logs. 8. Return.
All blocks done.

## The 13:35 read line (his rule, history.md:2113-2129)
- Reader: test-engineer, this file.
- marks.json sha256 before (044fab1) 9fb5be578b27d4c378a890d297c5ba636af0ac239bb6d50eae157ed44884a3c5; after (42b9646 and
  the worktree file) c1935a5b996fb0aaff82b0b3c1c604255e1534f6ca7e8d2e55a9f472cd3b22a9.
- marks.json lines read (built file): PM-0001-17 :3845-4047 whole; check snap_lost_unexplained_count :3983-3993; check
  snap_whole_despite_link_loss_count :3994-4004; bite PM-0001-17/link-lost-overstated :4035-4038. Own check: block
  shas = freeze (cea24dd8..., d5945f92...); each once; :3983-4004 == check-objects and :4035-4038 == bite-object byte
  for byte; json(built) == json(base) + exactly the two checks after 'L1 P0 loses no group' and the bite after
  whole-as-lost. PASS.
- Judge lines read: figures.rs :200-536 (context: slot_hashes, decoded, coverage), :538-698 (Bound, LINK_GAPS, bound,
  both figures), :805-830 (FIGURES entries); engine.rs :1-10, :1296-1340 (hash_log), :1555-1700 (the six c62hb tests
  and helpers); eval.rs :1176-1233 and :1440-1484 (context: gaps beside a fail; bite status); fixtures.rs :13-40,
  :200-300, :380-440; fixtures/README.md :19-35; fixtures/pm-0001-17/base/p2-room.jsonl and p2-client.jsonl,
  lost-group/p2-room.jsonl (diff vs base: slot 1 tick 2 only), every -17 case's file list.
- git diff --stat 044fab1..42b9646 -- marks/marks.json crates/judge at the close:
  fixtures/README.md 6 (+3 -3); base/p2-room.jsonl 6 (+3 -3); lost-group/p2-room.jsonl 7 (+7); src/figures.rs 172 (+172);
  tests/engine.rs 150 (+149 -1); tests/fixtures.rs 21 (+16 -5); marks.json 26 (+26); 7 files, 376 insertions, 12 deletions.
- B-2's changed answers: hashed-and-incomplete INVALID -> FAIL (A 1; coverage gap kept in the run's gaps); base and
  lost-group unchanged (NOT-RUN, run valid, 0 mismatches; 6 / 5 compared); none-compared still FAIL, now A 6 beside the
  empty comparison. No other existing judge test's assertion moved: the only removed lines in judge tests are the
  BiteStatus import (engine.rs:4), the doc line (fixtures.rs:390) and the hashed-and-incomplete tuple (-4); judge
  src/main.rs, eval.rs, marks.rs, tests/cli.rs untouched. Own scan (fx_scan.py) agrees: A 0 / B 0 on every other L1
  -17 case; short-lists gaps (INVALID before and after).

## Blocks
- SERIES.md 6898e119 (rows :291, :292, :322, :378 whole once; headers :258 (38), :372 (19)); README.md f3c3c804 (word
  diff = the sentence's 73 words, nothing removed; refilled :67-82 <= 120). PASS.
- Figures vs frozen rows: A = sent && link == Some(0) over measured lost pairs (BTreeSet); B = link >= 1 over measured
  whole pairs, sentinel out, no-room-record skipped; eight gaps, first by row index; slot_hashes' refusals first.
- Plants: 18 logs; each bit as section 4 says (P1 sent 180; P12 202; P13 120; P2 two Ok(0); P3, P4 Ok(0); P5 "measured
  link-lost sum is 0"; P6 (28, 29); P7 "E2 is 0"; P8 (Fail 1, Fail 1); P9 valid Pass 0; P10 B 1; P14, P15 A 1; P16 note
  gap 2's words; P17 note gap 8's words; P18 valid Pass 0; P11 5 FAIL, P0 7 bites). Pre-plant shas = committed blobs.
- Counts: 545/625, 551/631, 552/632, 552/632 (check logs). New test names in the runner's own output, both builds.
- Scripts: insert (hash-gated writes, refuses twice), built check (negative 12 PASS/13 FAIL; b01d3ac 25/0), compare
  (self-test 4/4), set check (dry on fc411116c1f1: 29 of 29 new-named lines FAIL). Set check's PHRASE regex and
  bite-line comparison exercised on the short test's real lines (bitecmp.py): whole-as-lost differs in exactly the two
  fields (K=16) -> ok; a planted third field -> FAIL; overstated vs clean -> no field differs.

## Findings (all Low)
- TE-CBH-1 set check bite_rule reads only 3 named checks; his "every other check passing" (2 lines).
- TE-CBH-2 set check's bite-line comparisons never shown to fail on content in its own record (about 20 lines).
- TE-CBH-3 eight-gaps test asserts "no other gap's phrase" only for case 1 (1-2 lines).
- TE-CBH-4 bound()'s own length gaps (hs lists, snap_incomplete) have no bite; brief :276's parenthetical pins
  decoded()'s, not bound()'s (about 6 lines).
- TE-CBH-5 c-u4r-runs.mjs:14-15 "(P0)" stale operator text (1-6 lines).
- TE-CBH-6 fixtures.rs:390-391 terse; fixtures/README.md:23 "E2" undefined there (2-3 lines).
- TE-CBH-7 negative snap_incomplete entry reported with gap 7's phrase, not "an entry below 0"; note text only
  (2 lines).

## Verdict
APPROVE WITH FINDINGS (7 Low; none triggers a push on its own).
