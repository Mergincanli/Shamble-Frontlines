# test-engineer, U4r review round 2 (PR #40), notes

Snapshot under review: wip/u4r.patch sha256 21ca46be23704131c77cc20981a4bfe866811b33f9ca853279f7ddbfb5d58bce (checked 2026-11-14).
Packet u4r-r2-packet.md sha256 b1526bde61fe... (checked); tests file 637249a5... (checked); runner u4r-run.sh 6efb9157... (checked).
Read copies: .gate/u4r-snap2 (2fbec77 + patch 21ca46be), .gate/u4r-snap1 (572c4f69).
Free space at start: 4.6 GiB (df -h). No Edit tool in this seat: notes rewritten whole with Write.
No run, build or plant of my own: everything below is from the code and the builder's logs (.gate/u4r/c6-*.log), plus
read-only git/sha256 commands in the builder worktree and .gate/u4r-snap2.

## Plan (all done)
My list (ruling "Round 2", Who reads what): every new test and plant in FP-1 to FP-4, T-3, T-4, T-5, FP-5's helper test
and its whole-as-lost suite, FP-6's added cases, FP-8's regeneration evidence and labels. Plus T-1..T-7 states.

## Results
- Plants fp1, fp2i, fp2i-b, fp2ii, fp3, fp4, ceil: each fails by name (logs read). Planted lines all present in snap2
  (scenario.rs:415 Kick send, :418 `*s = None`, :499 room.leave; hash.rs:267-272 profile; precheck.rs:657-662 ended;
  check.mjs:2359 Math.ceil).
- r8 body unchanged since its plants (Kick->Inputs gap 15 lines in snap2 :2063/:2078 and in the plant logs 1973(+6)/1993(+1)).
  precheck.rs fp4 at :872 vs plant log :845: shift explained by FP-6 item 3 and FP-7 item 2 edits above it, after 11:30.
- FP-2's test change (join_only, drain before the tick) is not a weakening: under plant (i) the old placement failed at
  the Kick count because deliver's output went to the kept old session; the new placement reaches the Inputs assert.
- Counts: c5-end2 526/603/76, list 617 -> c6-end2/end3 531/609/77, list 623 (+5/+6/+1; six new Rust tests named in
  c6-end3.log:420-448; schedule pin renamed, c6-end3.log:847).
- End check 1 (c6-end1.log:174) crashed: spawnSync fractional timeout from the performance.now edit. Deterministic,
  root-caused, fixed (check.mjs:2359), pinned by a new self-test (check.test.mjs:2918-2925) with its plant
  (c6-plant-ceil.log:432-438). Not a flake retried green. t0 >= deadline is skipped first (check.mjs:2350-2353), so ceil >= 1.
- FP-8 labels: tree fingerprint 63713cf3...; I recomputed it on .gate/u4r-snap2 (same method:
  `{ git diff HEAD -- spikes; sha256sum <13 untracked, sorted>; } | sha256sum`) = 63713cf3..., and on the builder
  worktree now = 63713cf3.... So end checks A and B, c6-det-u4.log and c6-regen-precheck.log ran on snapshot 21ca46be.
- Runner touches every crate file every 2 s (u4r-run.sh toucher), hence the 12:47:29 mtimes: benign.
- Scripts last edited 11:35:03 / 11:36:20, before c6-fp5-helpers.log (11:36:48): helper evidence current; both equal snap2 by cmp.
- Whole-as-lost hand run (c6-fp5-wal.log:23-38, 11:37) predates FP-6 item 3's refactor of the bite's site (11:39-11:47),
  but p4 (scenario.rs:2507, bites build) covers that site in both end checks. Hand-run command = c-u4r-r2.mjs:201-203's.
- TE2-5 re-pin (c6-fp6-1.log: 21, 22, 24 failed): the re-pins add the new refusal and keep every old expected refusal,
  so no old check loses its proof (check.test.mjs diff).
- Note only (rust seat, FP-3): due_ns (scenario.rs:130-132) truncates u128 -> u64 with `as`; the burst check bounds
  burst_starts' sums, not due_ns's product.
- Note only (infra seat): det-u4 npt-lab hash 187.8 s -> 220.3 s (+17%); non-cargo 223.7 s of 240 s; that call is 220 s
  of the 300 s shared deadline.

## Round-1 findings
- T-1 fixed as ruled: scenario.rs:2072 (before), :2078-2082 (whole-counters assert); plant c6-plant-fp2i-b.log:21-24.
- T-2 fixed as ruled: c6-det-u4.log:24-26; c6-regen-precheck.log:20-21; labels u4r-build.md:1909-1914, :2042-2051.
- T-3 fixed as ruled: check.test.mjs:2930 (/bin/sh child), :2932 (200 ms); durations 202.5-203.1 ms.
- T-4 fixed as ruled: scenario.rs:1774-1782; tick_differences :1729-1752.
- T-5 fixed as ruled: contract.rs:452-462, :467; scenario.rs:1903-1914, :1922, :1948; Drop :1630-1637.
- T-6 verdict half fixed (precheck.rs:851-893, c6-plant-fp4.log:26-29); contract half deferred as ruled (u4r-build.md:1945-1947).
- T-7 fixed as ruled: c-u4r-r2.mjs:87; c6-fp5-wal.log:23-38; advisor's words u4r-build.md:1951-1953.

## Findings
- TE-R2-1 (Low): fp1's rejoin asserts (scenario.rs:2204-2212) cannot fail for the leave plant: Room::join's `replaced`
  (room.rs:686) reaches `ended` only through drop_slot (scenario.rs:408-421), which finds no session once the closed
  path removed it (:496). c6-plant-fp1.log: under the plant "(a)'s new session 1" and both asserts held. Doc :2166-2169
  claims "and not as a replacement". closed_checks (:2150-2164) reads room.client only, never the roster.
  Fix: in closed_checks add `server.room.roster().is_live(slot)` as a failure ("the roster still holds slot N")
  (room.rs:568, roster.rs:80); drop or keep the `ended` assert but stop it carrying the claim.
- TE-R2-2 (Low): three new parse refusals have no case: the B1 last-burst check (scenario.rs:1443-1454), the
  warm-up + measured sum (:1376-1378) and + after-max (:1379-1381). Only --measured-s's product is tested (:2423-2432).
- TE-R2-3 (Low): FP-6 item 11's `killSignal: "SIGKILL"` (check.mjs:1910) has no test that fails without it: T-3's
  /bin/sh child dies on SIGTERM too. Node docs: a child that handles SIGTERM keeps spawnSync waiting until it exits.
  Fix: start T-3's child with `trap '' TERM;` (same 200 ms limit, same asserts): under SIGTERM the call would outlive
  the `ms < 10_000` assert.

## Verdict
APPROVE WITH FINDINGS (three Lows; budget: STATUS follow-ups, no push).
