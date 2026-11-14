# Advisor: conformance check of BRAWL-1's builder brief (condition 17), 2026-11-14

Inputs: brawl1-brief.md (the brief), brawl1-plan.md (the plan), advisor-brawl1-plan-full.md (my review: conditions
1-18, Q1-Q8). Task: every plan section (as amended) and every condition carried, word-faithful where a condition gives
exact words or numbers; the brief decides nothing new. Read-only; this file is my only write.

## Plan
1. Read the three inputs (done).
2. Facts the brief names: the runner's sha256 and its log-path check; the clean pair's "before" in
   spikes/netcode-proof/results/u4r-precheck-30/faf42aeb034c/runs.txt; the code lines the brief cites.
3. Plan sections 1-13 against the brief, one line each.
4. Conditions 1-18 against the brief, one line each, word for word where exact.
5. Q1-Q8 answers against the brief.
6. New decisions in the brief: where, and whether each may stay.
7. Verdict, exact edits, anything for Mustafa.

## Checks (a line per check as I go)

### Facts
- F1 runner: sha256sum = 6efb9157...729bc8, equal to the brief's. Log check `"$M"/.gate/u4r/*` accepts .gate/u4r/brawl1/.
  Worktree must be under .claude/worktrees/. The 4 GB check runs ONCE per call, before the first command (not "before
  every command" as brief 1.5 says): a fact error.
- F2 runs.txt: "## pair clean" at line 16 (header is c-u4r-runs.mjs's, with wall time); the npt-lab output follows
  (build line first; ends "result: PASS (9000 ticks, every World hash equal)" at :125). c-u4r-runs.mjs:509 runs exactly
  `precheck --manifest <m> --recording <rec> --commit <commit>` with no extra for the clean pair: condition 6's command holds.
- F3 code lines: precheck.rs:691 and :705-706 hold the "World::new(scene, ..)" strings; room.rs:691 is
  `self.roster.leave(slot);`; scenario.rs:1442 is the last-burst comment; crates/net/src/l2/socket_tests.rs:512 is
  `#[ignore = "..."]`; client lib.rs:162 `fn complete`, Filed::Hash at :140/:614; sim ticks.rs:35 `pub const fn
  ticks_for_ms`; world.rs:701 `World::new(scene, seed, first_tick)`; world.rs:120 FLAG_DEAD = 1 << 3; seam.rs:346 `pub
  area: u16`; scenario.rs:2154 "its slot was not freed"; roster.rs:80 is_live. All hold.
- F4 history: no commit touches spikes/netcode-proof/crates since faf42aeb; ops/runs last at 493b3d7 (U4r-F3). Holds.
- F5 rulings: N2-1's message in the brief equals advisor-u4r-r2-full.md:73; RR4-L1 `const MAX_ENTRY_TICKS: u32 = 36_000;`
  is the ruling's own text (:94), not new. RR4-L1's ruling also says "the three committed manifests still read"; the
  plan replaced it with "a synthetic entry at 36,000 passes" and my review missed it: carry both.
- F6 disk: df now 13,205 MiB (12.9 GiB). Brief says "about 13.2 GiB": wrong (13.2 - 8.2 = 5.0, not 4.7). Condition 16's
  4.7 GiB holds from 13,2xx MiB - 8.2 GiB.

### Plan sections (as amended)
- S1 goal: not carried (no goal or cause line). Edit E22.
- S2 steering: read item ok but drops "None when the newest complete tick has no own section" (E6); npt-bots ok (cond 4);
  leash drops "scaled to full command length" (E3), the sideways part's spec if kept (E4), TURN_EVERY/RETARGET_EVERY stay
  command counts (E5).
- S3 figure: ok except the summary text "in a building: N slot-ticks (slots …)" and room.jsonl's tick record (E10).
- S4.1 unit test: cited as "plan section 4.1's list" whose text names next_command and tests/ (replaced by cond 4): spell
  it out (E9). S4.2: drops the driver and the position source (E7). S4.3: main session's, named "not yours": ok.
- S5 re-pin: drops "The three synthetic entries in recordings.json do not move" (E11); -30 set and r2 texts: ok.
- S6 Part 1: EA-U4R-2 ok; EA-U4R-1 drops "the one-loop driver is not built" (E14); N2-1 ok; TE-R2-2/RR4-L2 drops the
  last tick's definition and "the builder words them, and they go in the record" (E12); RR4-L1 ok (+F5, E15); RR3-L2 (a)
  drops the unit test's content and "(finish reads it)" (E13); "Not in this lane" not carried (E18).
- S7 order: ok; (B) and (C) after the merge not named as not-yours (E21).
- S8 disk: two fact errors (E1, E2).
- S9 re-run sets: named not-yours: ok. S10 risks: covered by the stop rules. S11 tier/types: reviewers ok. S12 size: ok.
  S13: Q1-Q8 below.

### Conditions
- C1: bounds, 127^2, Kind::Brawl, tuning recorded: carried. Missing: the record's door-entry margin and spawn corners
  1,414 cm (E5).
- C2: word for word. Brief makes it a builder stop "before anything else": stricter than "before the 10-minute runs"
  (new; may stay: a re-tune would move the pins again).
- C3: (ii)-(v) carried; (i) drops "the largest lead a run takes (--lead's cap), as a step count with no time claim" (E8).
- C4: carried. C5: word for word. C6: carried; does not say (a)-(c) prove only scenario::run's half (E16).
- C7: builder part carried; the merge-step reproduction and "a new pin not reproduced comes to me whole" missing (E19).
- C8: "main-run.sh" -> "the same runner call" (u4r-run.sh: the builder's runner; may stay); "builds or runs" ->
  "running" (E17).
- C9: M-list carried; drops "on the merged tree" and "(--no-smoke, with the reason, if Chromium cannot start)" (E19).
- C10: carried. "(condition 10: you draft the rows)" misattributes the work split (E23; may stay).
- C11: carried; engine-architect's scope missing (E21). C12: word for word. C13: carried (log under .gate/u4r/brawl1/,
  inside .gate/: may stay). C14, C15: named not-yours: ok. C16: carried; fact errors E1, E2. C17: this check.
- C18: narrowed to "where they touch your work", words missing, and outside section 6's report list (E20).

### Q1-Q8
- Q1 via C1, Q2 via C5, Q3 via C7, Q4 via C13, Q5 via Part 1 item 3, Q6 this check, Q7 not the builder's (no clean:
  "a floor stop ... never cleaned around" holds), Q8 Part 1 first at the old pins. All carried.

### New decisions in the brief
- N1 binding order (review > plan > rulings): may stay (my review amends the plan).
- N2 -x also for det-u4.mjs; logs under .gate/u4r/brawl1/: may stay (runner rules; condition 8's no-other-lane).
- N3 the named-soldier stop "before anything else" (C2): may stay.
- N4 the builder drafts the SERIES.md rows: may stay; citation fixed by E23.
- N5 "uptime in the same runner call" (u4r-run.sh for main-run.sh): may stay.
- N6 wip/brawl1.patch at each snapshot; the start record (path, HEAD, status): may stay (process).

### Verdict
CONFORMS AFTER EDITS (E1-E23 below in the return). Nothing to Mustafa: KEPT-LOGS went with the plan review (Q7).
