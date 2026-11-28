# U4d-F report: the estimate scene fix (npt-sim)

Merged 2026-11-12 into the spike branch by the main session (the commit "U4d-F: the estimate scene fix (npt-sim),
merged after review round 2"). The approved plan: the advisor's U4d close ruling (advisor-u4d-close.md, B1.1 to B1.3;
carry 52), answering U4d round 2's N2-M1 (the event-rate drift) and N2-M2 (ES's vehicles outside the fight). For U4's
report, which folds this one. Mustafa's words that bind it (2026-11-11, verbatim in history.md): "The event-rate drift
in the estimate scene: fix the scene before U4d2 pins the recordings. The 9.5 floor (PM-0001-2) does not move."

## Who built it, and how
- **The build (run A):** four fresh builders in turn (step groups 0-1, 2-3, 4-5, and 6-7 with the End checks;
  workflow wf_3dbc5af1-0d4), in `.claude/worktrees/wf_9ce88fa3-452-1` (branch u4df-build at dad9566). Steps 0 to 7 as
  ruled: T2-L1's two tests; one home for the move rule (`move_velocity`); ES's 15 hulls inside the fight
  (`Scene::fight_vehicles`); the generator's reckoning of every input it emits (`Recording::GENERATOR` 2); the
  measurement, which still failed the drift check after step 3 and so added the ruling's bounded re-home (3b);
  `aim_one_in` from 11 to 40 on the judged window; t_m1 rebuilt as seed 2's window test, beside an ignored three-seed
  test; the docs. Every re-pin had one named cause, in its own step. Record: u4df-build.md.
- **Review round 1** (workflow wf_8a12a843-56b, from a packet cut by script, u4df-packet.md): the netcode-architect
  (read only) and the test-engineer (a general-purpose agent with the Edit tool for plants, reading main's
  test-engineer.md), both APPROVE WITH FINDINGS: NF-M1 and TF-M1 Medium, seven Lows (NF-L1 to NF-L4, TF-L1 to TF-L3);
  no Critical or High. Notes: u4df-review-netcode.md, u4df-review-test.md.
- **The advisor's ruling** (advisor-u4df.md: APPROVE WITH CONDITIONS; nothing for Mustafa): R1, `aim_one_in` stays 40
  and later re-measures follow RM (carry 67), with its erratum on B1.2 step 4's "stationary"; R2, TF-M1's exact test
  of the reckoning lands before the merge (FX-1), with FX-2's doc words; R3, the Lows' routes; R4, the main session's
  two readings (TE2-5; no rust-reviewer) confirmed, with two corrections of fact (in u4df-build.md's merge section);
  R5, the merge's conditions M1 to M9; R6, T4's brief rules.
- **The fix pass** (workflow wf_6d45d257-7c0, one fresh builder): FX-1,
  `replay::tests::the_sky_aims_reckon_every_input_the_recording_emits`, checks all 954,628 sky aims of ES seed 2 to
  the centimetre against the recording's own inputs and the World's move rule, in under 2 s; its six counts are above
  0 (the rarest, 11); the plants Q1 to Q5 each fail it by name. FX-2 put T3's words into scene.rs's two docs.
- **Review round 2,** on what changed (workflow wf_18a25138-5d9, packet u4df-r2-packet.md): both seats APPROVE WITH
  FINDINGS; TF-M1 CLOSED, and NF-M1's and NF-L2's wording points CLOSED; one new Low, NC2-L1 (below). No round 3.
  Notes: u4df-r2-netcode.md, u4df-r2-test.md.

## The estimate scene after U4d-F (the advisor's U4d-F ruling: NF-M1, NF-L2, NF-L4, TF-L2, TF-L3)
- **The acceptance (the advisor's U4d close ruling, B1.2 step 4) is met as ruled.** Seeds 1, 2 and 3, the last 18,000
  of 21,600 ticks: gameplay events 9.878, 9.743 and 10.098 a second, each 300 s half between 9.580 and 10.220, shots,
  live projectiles and spawns in their bands, 48 soldiers and 15 vehicles on every tick, every hull within 1,400 cm
  (1,400.6 cm exactly on the 1 cm grid). `aim_one_in` is 40, by the rule written before the sweep (the largest
  smallest margin to the band's edges: 0.080 a second); 34, 36 and 38 each put one half above 10.5, and 48 passes
  with 0.036. The 9.5 floor did not move.
- **The drift is fixed.** After the reckoning alone (step 3) the rate still fell in every seed, from about 11.5 to
  about 4.3 a second over 12 minutes (measurement A), so the ruling's bounded re-home (3b) was added. After U4d-F,
  over 36-minute runs of seeds 1-9 (the netcode-architect's probe), the nine seeds' mean over each 300 s block from
  minute 2 is 10.096, 9.692, 9.941, 9.901, 9.799, 9.857 and 9.887 a second, the slope over minutes 2-35 is -0.003 a
  second per minute, and all 27 ten-minute windows are in band (the lowest 9.537).
- **The level sits a little below the band's centre:** about 9.88-9.89 a second. Out of sample, seeds 4-9's windows
  are all in band (9.693 to 10.171), and one of their 12 halves is out: seed 6's second, 9.446.
- **The halves inside the window are not flat.** This corrects the ruling's "that is what this step shows" (B1.2 step
  4): the second half is lower than the first in 8 of 9 seeds (by 0.404 a second on average; the halves average
  10.096 and 9.692), the same in every seed, and the dip does not recur after minute 12; the netcode-architect reads
  it as the hulls' loops, which start from the same point in every run. A single 300 s half's noise is 0.2-0.36 a
  second, so at this level the per-seed half check misses by noise about one half in 9, and three seeds' six halves
  all pass only about half the time: 40 is one passing set of halves, not a margin. Later re-measures follow the
  advisor's rule RM (carry 67): nine seeds, 36-minute runs, every seed's window in band, drift judged on the nine
  seeds' mean per 300 s block, single halves reported and not judged; the share is re-set only when that fails.
  Today's scene passes RM.
- **What the events are now:** about 69 % grenade blasts (6.6-7.0 a second), 21 % sky shots meeting a soldier on the
  shooter's chest (about 2.0), 7 % aimed hits (about 0.7, the only part the share moves) and 3.5 % kills (0.34-0.36);
  42 % of hits land on a teammate, as before. 3b makes 51.9 teleports a minute (3.9 before). Each team is re-homed
  inside one 20 s third of every minute, so its hits peak in that third and dip about 13 % before the next; every
  judged window is whole minutes, which averages it out exactly. The teleports force no correction anywhere today
  (synthetic recordings are replayed sim-only); a predicting client fed this recording would meet about 52 forced
  corrections a minute (carry 70). ES's alive soldiers change yaw on about 48 % of ticks and pitch on 18 %, against
  R2's moving-delta assumption, so no ES byte figure is taken from this recording (carries 68 and 69). The 15 hulls
  now loop inside the fight, where they are cover and block moves: at the old reckoning and share they cut the rate
  from 9.853 to 7.250 a second over T-M1's 300 s (step 2); carry 59 now names it for U4r.
- **What the reckoning is worth, and what guards it.** The old reckoning planted back (the test-engineer's P1, in the
  base's own lines, 3b and 40 kept) gives windows 9.358, 9.846 and 9.235 for seeds 1-3 (mean 9.480, against 9.906 as
  built): under 3b the reckoning is worth about 0.43 events a second, and t_m1 (seed 2) catches it only on one half
  (9.240), with the window in band. The builder's reading "P1 shows as a level, not a decay" held for its own plant
  form on seed 2 alone: a last-ulp difference in the estimate moves a half by about 1.1 a second. The reckoning's
  exact guard is the fix pass's test (TF-M1, FX-1), which each of the plants Q1-Q5 fails by name; t_m1 stays the
  tripwire that sends a later change to RM.

## Checks on the merged tree
- **The bytes:** the merged files are the reviewed ones: `sha256sum -c` of round 2's start hashes
  (.gate/u4df/r2-start.sha256, every file under crates/sim), 17 of 17 OK from the main folder's root.
- **check.mjs, counted** (the advisor's U4d-F ruling, M5): one run through main-run.sh (the lane lock held
  exclusively, idle at its start, the 2 s touch loop, the shared target, offline; .gate/u4df-merge/m5-check.log,
  01:39:33-01:46:03 UTC), after the two `--no-run` builds, each compiling npt-sim, npt-proto, npt-room and npt-lab
  from the main folder's path. All 10 steps pass: measured 399 (396 + 3), bites 446 (443 + 3), self 52 of 52; marks,
  81 read and checked, the ADR rows unchanged; redact, 238 items and 0 findings. The +3: step 0's two module tests and
  FX-1, in each build.
- **In the same run:** the three-seed test by name, 1 passed (32.4 s), its 21 "ES seed" lines identical to the fix
  pass's (.gate/u4df/end-three-seeds.log); FX-1 by name in both builds, passed (1.8 s each), its six counts equal to
  the fix pass's (954,628 sky aims checked; 97,974 crouched; 14,860 held back; 465,658 of a re-sped slot; 14,996
  first after a lost command; 11 first after a held-back command carried on a re-homed or re-sped tick). No test
  failed, so no TE2-5 entry.
- **The gate:** PASS with `--no-smoke` (.gate/u4df-merge/gate-nosmoke.log). The full gate (gate-full.log) passes every
  check but smoke 5a, which cannot start Chromium here: this container has chromium_headless_shell-1243, and tools/'s
  Playwright wants 1194 (as at U4j's merge). CI runs the full gate on the pushed head.

## Carries
- **Met:** carry 52's U4d-F part (the drift fixed and N2-M2's vehicles inside the fight before U4d2 pins anything;
  U4d2 pins only recordings written by `Recording::GENERATOR` 2 or later, and names this commit).
- **Added** (the advisor's U4d-F ruling, R1 and R3, copied by script from its T2): 67 (RM, any later change to the
  synthetic recordings), 68 (§U4e) and 69 (§U4r) (no ES byte figure from the synthetic recording), 70 (§U4g, teleport
  corrections counted apart), 71 (3b's switch), 72 (a_m1's non-vacuity); carry 59 gains NF-L4's sentence.

## Follow-ups (Lows; none triggers a push)
- **NC2-L1** (round 2, the netcode-architect; confirmed by the test-engineer's plant R-A): FX-1's count 6 adds up a
  held-back command carried on a re-homed tick and one carried on a re-sped tick, and seed 2 has none of the second.
  A generator that moved a held-back command at its hold-time speed leaves seed 2's recording unchanged, so FX-1, D-7
  and t_m1 all pass it; on seed 1 FX-1 fails it (slot 47, tick 1,014,152, 1 cm). Fix: split the count in two, each
  above 0, and add seed 1 (seeds 1 and 2 run in 3.1 s measured, 3.6 s bites). Owner: the next change to npt-sim's
  synthetic recordings (carry 67's section). A nit beside it: count 5 also counts 4 of its 14,996 gaps whose lost
  command came just before a re-home.
- **The seven Lows of round 1:** routed as carries 68 to 72, carry 59's sentence and the measurement section's words
  (the advisor's U4d-F ruling, R3).
- **Process (R6):** both round-1 seats first wrote part of their notes with a python one-off, then rewrote the file
  whole with Write; round 2's test-engineer once used a shell redirect, which the guard refused before it ran. Every
  agent brief now carries T4's sentence word for word.

## Decisions needed
None.

## For Mustafa
Nothing: risk tier 1 (spike code inside PR #40), and nothing a player sees, hears or feels.
