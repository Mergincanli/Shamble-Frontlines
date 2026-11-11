# U4d report: the stand-in sim (npt-sim), with W1-S1's npt-proto half

Merged 2026-11-11 about 12:15 UTC into the spike branch by the main session (the commit "U4d: the stand-in sim …").
U4 wave 1, u4-build-plan.md §5 U4d (size M-L). For U4's report, which folds this one.

## Who built it, and how
- **The build:** a builder agent in workflow wf_f54246ff-e0d's copy `.claude/worktrees/wf_f54246ff-e0d-1` (base
  097f897): the World, the scenes (SS, ES, CO, TC), the recordings and their Replayer, the greybox, the hash, the RNG
  streams and the trajectories. Record: u4d-build.md (Plan, Progress, "Fix pass, round 1").
- **Review round 1** (u4d-review-{netcode,rust,arch,test}.md): netcode-architect REVISE (H1, vehicle parts written as
  world transforms, and five Mediums); test-engineer REVISE (five Mediums); rust-reviewer and engine-architect APPROVE
  WITH FINDINGS.
- **The advisor's rulings** (advisor-u4-wave1.md): W1-U4D (what the fix pass fixes, the press counters' wording, the
  two tuning choices, carry 13's split) and W1-S1 (the owner-only record; the freeze lifted for exactly that change).
- **The fix pass** (workflow wf_a8ff8550-67d; three agents, the spend limit cut it twice): every ruling item, each
  with its test and a plant that fails it by name (Q1-Q29).
- **W1-S1's npt-proto half,** by the main session (s1-proto.md): both GROUPS own-state rows gain OwnState; the codec
  refuses a record whose kind its section does not list (OutOfRange / Malformed); the GROUPS test's three checks; the
  -57 test sends the owner-only record; eight plants P1-P8, each failing by name.
- **Review round 2** on the combined copy `.gate/u4d-s1` (workflow wf_4b5ee472-66b; u4d-round2-{netcode,security,
  rust,arch,test}.md): all five APPROVE WITH FINDINGS. No Critical or High, so no round 3. Every round-1 finding the
  fix claims is closed. **SEC-U4a-1 (the soldier's owner-only state reaching other clients) and SEC-U4a-2 (the codec
  not enforcing the GROUPS tables) are CLOSED in writing** by the security-auditor.

## The merge condition (N2-M1, the netcode-architect's round 2)
"U4d's report states T-M1's window, the per-window rates, and that a 10-minute measured window would likely read
below 9.5."
- **T-M1's window:** 9,000 ticks (300 s) from tick 0 of seed 2's estimate-scene recording, with 48 soldiers and 15
  vehicles on every tick. Over that window ES meets all four PM-0001-2 bands: shots 70.776 a second (68.4-75.6),
  gameplay events 9.853 (9.5-10.5), live projectiles 32.300 (30.4-33.6), spawns 21.120 (20.064-22.176).
- **The per-window rates** (the fixer's measurement, gameplay events a second per 50 s window, at aim 1 in 10): 13.4,
  11.0, 10.1, 9.4, 9.7, 8.7. At 1 in 11, about 5 % lower.
- **A 10-minute measured window would read below 9.5.** The netcode-architect's estimate: about 8.5-9.3 a second
  after warm-up. The test-engineer measured it (round 2, probe X2: t_m1 over 18,000 ticks, 600 s, PM-0001-12's ES run
  length): FAILED, 7.578 a second over the 600 s, so about 5.30 a second over the second 300 s. Shots, spawns and live
  projectiles stay in band.
- **The cause:** the recording generator's dead reckoning moves its estimate at the scene's speed only; it ignores
  crouch, dropped commands, scripted move speeds, blocked moves and deaths, so its aim drifts off the soldiers.
- **Routing:** Mustafa, 2026-11-11: "The event-rate drift in the estimate scene: fix the scene before U4d2 pins the
  recordings. The 9.5 floor (PM-0001-2) does not move." Written as carry 52 (u4s-carries.md, §U4d2). No U4 run judges
  ES's event band, and the judge fails safe (INVALID, never a false pass).

## Other findings, by route
- **N2-M2, Medium** (netcode round 2): ES's 15 vehicles loop 40-82 m from the centre while ES needs every vehicle in
  every client's view on 99 % of snapshots. To the advisor: fix (a scene knob, ES's D-7 pin only) before U4d2 pins
  ES's hashes, or U4e's report shows its region covers them.
- **SEC-U4d-R2-1, Low** (security round 2): an effect row's owner has no public home. Deferred to U4e by Mustafa,
  2026-11-11 ("defer it to U4e in writing. A Low never triggers a push."): carry 51.
- **For the advisor:** the fixer's points (A) the estimate scene's knobs `aim_one_in` 11 and `grenade_fuse_ms` 1,515
  (new stand-in tuning; `grenade_fuse_ms` also changes every bot-driven ES run's grenades), and (C) W1-S1's "(9
  fields)" is a counting slip for 8 (the ruling's own list and catalogue :137 have 8); the new public items `Weapon`,
  `Scene::weapon` and `Scene::health` (TUNE-1's run-header facts) for §4.1's npt-sim row (R2-L4, A2-L2, N2-L4).
- **The advisor's answers** (advisor-u4d-close.md, 2026-11-11): (A) the estimate scene's knobs are approved as stand-in
  tuning, with conditions (B1.1); N2-M1's fix and N2-M2's scene fix land as the main session's change U4d-F, before U4d2
  pins anything (B1.2, B1.3; carry 52); (C) was a slip, and its erratum is appended to advisor-u4-wave1.md (B1.4);
  `Weapon`, `Scene::weapon` and `Scene::health` are approved for §4.1's npt-sim row (B1.5); TUNE-1's measured load has
  its own bullet in this section (B1.6).
- **Follow-ups (Lows, nits; no push on their own):** rust L3, L6, L7, L9, N1, N2, N3, N5, R2-L1 to R2-L3, R2-N1;
  arch D, E (to U4d2), A2-L1, A2-L3 to A2-L5; test L2, N1, N2, T2-L1 (two invariants held only by D-7's re-pinned
  hash), T2-L2 (the decode cap's test goes with U5's carry 48), T2-L3; netcode N2-L1 (a test-backed U4e carry: the
  own-state section holds only the receiving slot's records), N2-L2, N2-L3 (notes for U4g, U4h, U4e), the round-1
  nits (PARENT redundant; a held-back command's Rewind may leave the ring; a non-finite ray is a Miss: carry 45);
  security's carries for U4e; round-1 SEC-U4a-4 and -5.
- **Carry 13** is met in part (the storage-identity test); its counting-allocator half is U5's (carry 48).
- **TUNE-1's measured load:** SS's kill rate is 0.272 a second over an S30 (the test-engineer's probe X3: 489 kills in
  1,800 s; D-6's 100 s give 28, 0.28 a second; probe F's 0.42 was before the fix pass), so respawns, and with them
  generation changes, are rare in SS. N-M1's magazine of 32,000 rounds lasts 2,194 s at one soldier's rate, so "no
  reloads" holds for a whole S30 (X3).

## Size (SIZE-CHECK, reported to Mustafa 2026-11-11)
About 7,250 lines for size M-L: code about 4,210 (world.rs 2,409, replay.rs 713, scene.rs 315, seam.rs +308, the
rest), tests about 3,030 (tests/world.rs 2,295, tests/replay.rs 659, tests/seam.rs +80); plus the npt-proto half,
+407 −56. Past its plan. Mustafa was told what grew, why, and the cheaper option (add nothing more to U4d; its open
tuning points go to the advisor).

## Checks on the merged tree
check.mjs, all 10 steps pass: measured 365 = 325 + 39 (U4d) + 1 (the npt-proto half), bites 412 = 366 + 45 + 1, self
52 of 52 (a counted run, 12:05:52-12:10:53 UTC: no other cargo or rustc before or after it, the 2 s touch loop, the
pre-run build "Compiling npt-sim" and "Compiling npt-proto" from the main folder); gate PASS (--no-smoke). The combined
copy gave 351 / 388 / 52 twice (the main session's run and round 2's test-engineer's), on the tree before U4c merged.
