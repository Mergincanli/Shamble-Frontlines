# U4d2 brief: drafter's notes (rust-reviewer, 2026-11-12)

No Edit tool in this agent: this file is rewritten whole with Write at each save (the task's rule).

## Plan
1. Read the model brief u4e-brief.md (structure: sections 0-8) and its approval (advisor-u4e-brief.md).
2. Read u4-build-plan.md §5 U4d2 (:666-702), §5's Lanes (:840 on), §4.1, §4.2, §4.15, and every section U4d2 cites.
3. Read plan.md §4.1 layout, §4.2 named exceptions, the lines U4d2 names.
4. Read the carries (u4s-carries.md: U4d2 section 29/41/42/52/65/66; entry 34; the recordings section 67/71/72);
   u4a-build.md and u4ab-*.md for entry 34.
5. Read the rulings: advisor-u4.md (cond 2, 7, 8, 10b), advisor-u4-wave1.md (W1-TARGET :558-587 + U4d2 lines),
   advisor-u4d-close.md (B1.2, B1.3, B2), advisor-u4df.md (R1, R3, R4.1, R6), advisor-u4j-r2.md (B2.3),
   advisor-u4e-brief.md (F2-ROUTE, L4).
6. Read the resume card's "Rules in force" (netcode-proof-test.md) and history.md "Decisions taken" (2026-11-11).
7. Read u4e-run.sh.
8. Read the code: check.mjs, check.test.mjs, Cargo.toml, crates/ (sim Replayer/Recording/GENERATOR/recordings; lab
   subcommands; proto/Cargo.toml; l1 FateLog::for_each_canonical; tcpinfo unsafe pattern), marks.json rows,
   ops/runs/c-u2-pm-0001-3.mjs, docs/adr rows.
9. Draft the brief by section (0..8), saving after each section; log choices and conflicts here as found.

## Sources read (file:line)
- u4e-brief.md (982 lines, rev 3.3): structure §0-8; §1 :139-172; §2 :176-564; §3 :568-717; §4 :721-781; §5 :785-801;
  §6 :805-834; §7 :838-912; §8 :916-982.
- u4-build-plan.md (1,079 lines at the main folder's head): §5 U4d2 :666-702; §4.1 :178-203 (hash-wasm row :202; npt-lab
  `hash` :198; Recording/Replayer :185); §4.2 :205-249; §4.14 PM-0004-21 hash-map row :476; §4.15 :478-504; U4d's
  row :602-643 (D-2 :612-614 "the judged check is U4d2's release-build step"; D-7 :629 "cross-target half is U4d2's";
  bites :640-642 "the std sin site (its proof is U4d2's)"); U4r :740-793 (HashCompare reuse :766-767; the step's
  5-minute fallback det-u4.mjs :790-792); Lanes :840-875 (wave 2 :863-867; U4s's manifest lines :841-845); §6 U7 :885-887
  (C-U7-1); §7.1 :896-908; §7.2 :910-918; §2 C-U7-1 row :159, Features row :157; RQ4/RQ5 :117-118; T9 :69, T22 :82,
  A8 :91, A12 :95, A13 :96; Q3 :981-985; Q21 :1040-1042 (U4d2 builds from the plan; this brief is beyond it).
- plan.md §4.1 layout :366-419 (hash-wasm line :392-393); §4.2 :421-454 (three exceptions :435-447; hash-wasm's
  :442-446; "How they are read" :449-450); §4.5 :527-551 (build flags :533-535; features :536; the check :546-551,
  incl. "once in local Chromium (a rough check)" :548 and the two bites :551).
- u4s-carries.md: §U4d2 :407-446 (29 :408-411, 41 :412-414, 42 :415-417, 52 :418-428, 65 :429-435, 66 :436-446);
  entry 34 :62-69; recordings section :351-379 (67 :352-369, 71 :370-374, 72 :375-379).
- Entry 34: met by U4a (u4a-build.md:125-126; history.md:1335; u4ab-review-test.md:90; u4ab-review-rust.md:27). So carry
  42's condition ("If U4a's report does not list entry 34 as met") is false: U4d2 does not take entry 34.

## Choices where the plan is silent
(filled as I go)

## Conflicts found (plan vs rulings vs code)
- K1. u4-build-plan.md:675 "the release build with §4.5's flags": the build plan's own §4.5 (:292-304) is "Baselines,
  groups and the window" and holds no flags; plan.md §4.5:533-535 holds them. Read as plan.md §4.5 (question).

## Progress
- [ ] sources read (model, plan, plan.md, carries done; rulings next)
- [ ] brief §0-1
- [ ] brief §2
- [ ] brief §3
- [ ] brief §4-6
- [ ] brief §7-8
