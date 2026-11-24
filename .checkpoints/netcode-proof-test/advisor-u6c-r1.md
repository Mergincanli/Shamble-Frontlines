# The advisor's ruling on U6c's round 1 (NPT-TRIM, one round; 2026-11-24). Read only.
Inputs, sha256 checked equal: u6c-review-netcode.md f29e1ac1…, u6c-review-test.md 79197b42…, u6c-review-security.md
c29540fa…, wip/u6c-build.patch a4e63455…, u6c-build.md 426b31da…. W = the worktree's spikes/netcode-proof (read only).

## Plan
1. Read the three reviews, the notes, and the code each Medium-or-higher cites. DONE.
2. Rule NC-U6C-1 (count or margin), the duplicate pairs, F-READINGS, F-TIMING, F-P2, NC-U6C-4, NC-U6C-12, round 2,
   size. Working notes so far below.
3. Write verdict, FP list, carries, round 2, For Mustafa, not checked, DONE.
Resume from: block 3 (write the ruling from the working notes).

## Working notes (saved before a cut)
- NC-U6C-1 checked: leg_series takes fates by `inside(t_in)`, a span of RS's clock (W crates/net/src/l2/probe.rs:691-692,
  :733); run_leg passes (zero+warm, zero+warm+window) (W crates/server/src/probe.rs:447-450, :557-563); marks.json's ten
  checks are `count >= 1000000` (marks.json:752-760, :881-889, …). Lean: count-based in t_in order (not flow index:
  QueueDrop/PoolDrop carry none, NC-U6C-3), first `packets` outside-burst fates with t_in >= start, ties at the last kept,
  per-direction spans for U8's rate check; matches L1 (lab/src/probe.rs:124-132) and R-6's "one count".
- F-TIMING (a) falls with FP-1 (the e2e bound becomes >= packets); (b) >= 5,000; (c) read until echo 9 or 1 s.
- F-P8 = NC-U6C-11: `edge_events(&sock, …)`; P8 then a compile error (E0382); reported "20 of 21 by a named test, P8
  held by the compiler"; plant-p8.log's pass stays in the report.
- F-A2-EXACT = NC-U6C-2: the full test name with `--exact`.
- F-READINGS: the probe reading with the shipped release binaries (no example); A2 gains a WT switch; both before Stop B.
- NC-U6C-4: marks.json PM-0001-3 notes (:2348) "L2's injected delay is (profile RTT - base RTT) / 2 (R-9)"; the one-way
  bounds are the profile's: his question (marks.json text), lean in progress.
- NC-U6C-12: U8's change at C9 (U6c's merge).
