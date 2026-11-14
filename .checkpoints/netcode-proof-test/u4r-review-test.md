# U4r review, round 1, test-engineer seat (batch 1)

Snapshot under review: wip/u4r.patch sha256 572c4f69435da2ad6f62d54d4bcbc47b04d6329b5493e0faf78fcecfdc4a09c2 (checked by sha256sum, 2026-11-14).
Packet: u4r-packet.md sha256 cbd82485... (checked); tests file u4r-packet-tests.md sha256 d12555c8... (checked).
Read copy: /home/user/Shamble-Frontlines/.gate/u4r-snap1 (2fbec77 + patch). Logs of my runs, if any: /home/user/Shamble-Frontlines/.gate/u4r-review-test/.

## Plan (seat list, advisor-u4r-panel-full.md section 2)
1. r1..r10 and the 2.5 bites: each unit-shown, fails by name under its bite/plant.
   r1 twin+bite; r2 contract (cond 5 per U4R-Q4; U4R-Q11 doctored cases + ruling 3 pt 6 (1) sixth; exit codes r3pt6(3));
   r3 replay + floored-grid bite; r4 class audit + owner-to-all; r5 tps + 7.3 estimate; r6, r7; r8, r9 (carry 76);
   r10 (check.mjs carry 83-85 self-tests in check.test.mjs).
2. Ruling-added tests: dump-reader pin (r1p6); ring-edge pin + hash.rs header compare (r1p5); radius pin fails on None (r2p4a);
   late-command exact asserts (r2p4b); r6 with L from leg config + whole deviation list (r3p5); one-id-outside-PM (r4p2);
   parse cases --lead/--every-step with --record (r1p9a), precheck without --commit (r2p2(1)); wasmStep options case (r1p1(4));
   carry 63's two tests failing before refusals (U4R-Q13a); scratch helper place + teardown (r2p32).
3. Counts: base 11/491/561/71 -> end 11/526/603/76 (c5-end1.log, c5-end2.log). Each new test named with file and in --list.
   TE2-5: any check.mjs test-step failure during build recorded whole, not retried.
4. Evidence standing: builder logs labelled pre-commit (r2p33). c-u4r-r2.mjs and c-u4r-runs.mjs: suites' lines as run,
   amended tick16 expect text (r4p1), bites first (7.1), carry 62 (h) plain check added after record run (ruling 4 further).
5. marks/SERIES.md new rows (21): name, series, writer vs code (r3p8 (B),(D)).
6. Own earlier findings: carries 73 (TE-1), 83 (TE-L3, TE2-R2-2), 84 (TE2-R2-1), 85 (TE2-R2-3), 59 (T2-M1).
Method: read packet, packet tests, rulings, carries; then code in snap1. Runs only if a finding needs one.

## Progress / findings (a line per finding as found)
