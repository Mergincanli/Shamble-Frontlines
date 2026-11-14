# test-engineer, U4r review round 2 (PR #40), notes

Snapshot under review: wip/u4r.patch sha256 21ca46be23704131c77cc20981a4bfe866811b33f9ca853279f7ddbfb5d58bce (checked 2026-11-14).
Packet u4r-r2-packet.md sha256 b1526bde61fe... (checked); tests file 637249a5... (checked); runner u4r-run.sh 6efb9157... (checked).
Read copies: .gate/u4r-snap2 (2fbec77 + patch 21ca46be), .gate/u4r-snap1 (572c4f69).
Free space at start: 4.6 GiB (df -h). No Edit tool in this seat: notes rewritten whole with Write.

## Plan
My list (ruling "Round 2", Who reads what): every new test and plant in FP-1 to FP-4, T-3, T-4, T-5,
FP-5's helper test and its whole-as-lost suite, FP-6's added cases, FP-8's regeneration evidence and labels.
Plus: confirm T-1..T-7 fixed/deferred as ruled with file:line in snap2.

1. Read packet (diff), packet-tests, builder report's "Fix pass, round 1: report". DONE
2. FP-1 test + plant. DONE
3. FP-2 r8 + plants (i), (i)-b, (ii). DONE
4. FP-3 hash.rs cases, parse case, plant. DONE
5. FP-4 verdict test, plant. DONE
6. T-3, T-4, T-5. DONE
7. FP-5 helper test + log; whole-as-lost suite + hand runs. NEXT
8. FP-6 added cases (NC-U4R-2, EA-U4R-3, RR3-L7, SEC-U4R-5/-6 self-tests, re-pins of tests 21/22/24, EA-U4R-9). PARTLY
9. FP-8: end checks, det-u4, regen, labels. DONE
10. Runs the report names, each in its own log. PARTLY

## Results so far
- Plants fp1, fp2i, fp2i-b, fp2ii, fp3, fp4, ceil: each fails by name (logs read). Planted lines all present in snap2
  (scenario.rs:415 Kick send, :418 `*s = None`, :499 room.leave; hash.rs:267-272 profile; precheck.rs:657-662 ended; check.mjs:2359 Math.ceil).
- r8 body unchanged since its plants (Kick->Inputs gap 15 lines in snap2 :2063/:2078 and in the plant logs 1973(+6)/1993(+1)).
- T-1 fixed as ruled: snap2 scenario.rs:2072 (before), :2078-2082 (assert), plant c6-plant-fp2i-b.log at "the first's Inputs reach no room call", ack_refused 5 / ack_refused_commands 15.
- T-2 fixed as ruled: c6-det-u4.log PASS det-u4 (SS-P2-s1, ES-P3-s2 regenerated, 18000 ticks equal); c6-regen-precheck.log "regenerated SS-P0-s4: 16950444 bytes, sha256 6933b56a... as the manifest's". Label: tree fingerprint 63713cf3...; I recomputed the same fingerprint on .gate/u4r-snap2 = 63713cf3..., so these logs (and end checks A, B) ran on the reviewed snapshot 21ca46be. Old logs labelled in the report (u4r-build.md:2049-2051).
- T-3 fixed as ruled: check.test.mjs:2927-2950 (child /bin/sh :2930, run :2932, limit 200, same asserts).
- T-4 fixed as ruled: scenario.rs:1774-1782 (fate_of(&a).is_some(); room > 0 && client > 0); tick_differences :1729-1752 returns [room, client], equal lengths per process required.
- T-5 fixed as ruled: contract.rs:452-462, :467; scenario.rs:1903-1914, :1922, :1948 (guards held as `_guard`, Drop keeps on panic :1630-1637).
- T-6: verdict half fixed (precheck.rs:851-893, fp4 test, plant c6-plant-fp4.log); contract half deferred as ruled (u4r-build.md:1945-1947 lists it).
- T-7: NEXT (c-u4r-r2.mjs suite).
- Counts: c5-end2 526/603/76, list 617 -> c6-end2/end3 531/609/77, list 623 (+5/+6/+1, six new Rust tests named in c6-end3.log:420-448).
- End check 1 (c6-end1.log:174) crashed: spawnSync fractional timeout from performance.now edit; deterministic defect, root-caused, fixed with Math.ceil (check.mjs:2359), new self-test (check.test.mjs:2918-2925) and plant (c6-plant-ceil.log). Not a flake retried green. Edge: t0 >= deadline is skipped before (check.mjs:2350-2353), so ceil >= 1.
- Runner touches every crate file every 2 s (u4r-run.sh toucher), hence the 12:47:29 mtimes in the builder worktree: benign.
- Note only (rust seat's FP-3): due_ns (scenario.rs:130-132) truncates u128 -> u64 with `as`; the burst check bounds sums, not due_ns.
- Note only (infra seat): det-u4 npt-lab hash 187.8 s -> 220.3 s (+17%), non-cargo 223.7 s of 240 s.

## Findings
- TE-R2-1 (Low): fp1's "(a): the new endpoint is no replacement" (scenario.rs:2209-2212) cannot fail: Room::join's
  `replaced` (room.rs:686) reaches `ended` only through drop_slot (scenario.rs:408-421), which finds no session once the
  closed path removed it (:496). Proof: c6-plant-fp1.log, the roster still held slot 0 under the leave plant (so
  room.join returned replaced = true), yet "(a)'s new session 1" and the assert held. The doc (:2166-2169) claims "and
  not as a replacement".
