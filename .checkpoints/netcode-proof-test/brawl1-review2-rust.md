# BRAWL-1 round 2: rust-reviewer notes (2026-11-15)

Snapshot under review: wip/brawl1.patch sha256 c2561b6ca54c8cde994fcdad5954c63d332df73f2db038c12aba3d71a0daa49f (sha256sum checked at start).
Packet: brawl1-r2-packet.md sha256 e50a22f70ae876f962510693e03c414700200a7bf3a37a13ad31bf5fb5dfa8b5 (checked).
Round 1 snapshot: ee2973aa...; read copy round 1 .gate/brawl1-snap1; round 2 read copy .claude/worktrees/brawl1.
Logs (if any): .gate/u4r/brawl1-review2-rust/

## Plan
1. Read ruling's "Round 2" (my list) and "The fix pass, in order" (done).
2. Check read copy's `git diff HEAD | sha256sum` equals c2561b6c.
3. Diff round 1 snapshot to round 2 snapshot (interdiff of the two patches / diff the two read copies' changed files).
4. My list:
   - RR-B1-1 through FP-3's test (logs.rs) + P2 record.
   - RR-B1-2 doc route (bots lib.rs toward_centre, leash docs).
   - RR-B1-3 tie (bots soldiers() assert, dead count, client SOLDIER_FLAGS doc line).
   - RR-B1-4 rewords and two fp3 cases (scenario.rs).
   - RR-B1-5 through the wrap test (endpoint.rs) + P4.
   - RR-B1-7 (2) doc rewrap, EA-B1-2 narrowing.
   - fmt and clippy in both builds (FP-8 check log; maybe my own lint run).
   - Library diff on recording path: only doc comments and the one visibility change.
5. RR-B1-6: no action (ruled). Check report words only.
6. Review changed lines; plant records P1-P4; FP-8 logs; FP-0 runner diffs; report section.
7. Result.

## Progress
