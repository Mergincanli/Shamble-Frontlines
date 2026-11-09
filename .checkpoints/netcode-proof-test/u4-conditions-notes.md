# U4 conditions: applying advisor-u4.md to u4-build-plan.md (netcode-architect)

Started 2026-11-09. Base: u4-build-plan.md rev 2 at sha256 27e0315857e51bd5667605d032b57a0ce75701d0ba6a08bd2b9319315e3b4dc2
(verified on disk before any edit). No agent memory directory exists; no earlier notes file.

## Plan
1. Read u4-build-plan.md in full; map each condition 1-11 and nit (a)-(f) to its sections.
2. Apply edits one condition at a time, each marked; log each here as done.
3. Read plan.md at the named spots; draft u4-plan-md-edits.md (old text quoted exactly, new text, ruling).
4. Return the edit list.

## Log
- Read advisor-u4.md in full and u4-build-plan.md (901 lines) in full.
- Section map (approved text line numbers):
  - top line: after title (line 1).
  - c1: §4.1 seam-data row (:175, accessor type), §4.2 (:203-206 Rewind refusal + debug assert, :210-213 clamp report, :219-220 hash rule), §4.10 (:329-331 clamp twin, :332ff twin rule, counted_count), U4d tests (:554, :559-562 + new D-10). Watch: debug assert on out-of-ring Rewind blocks condition 1's unit test -> move assert to room's rewind_at (flag in return).
  - c2: U4r -30 bullet (:654-660), U4j builds/tests (:572-578), §4.15 both-uses (:439-441); stale PRE- text in §0 (:34), §0.1 T2 (:54), §4.1 npt-judge row (:189), risk 12 (:784-785); U4d2 comparer input (:587-588).
  - c3: §4.15 sources (:444-446), U4r -30 bullet.
  - c4: §4.12 -22 row (:381), U4j README §4 sentence, U4f -22 bullet (:675-676).
  - c5: U4r tests (:644-646), D-6 (:556-557).
  - c6: U4e tests/bites (:624-633).
  - c7: U4d2 (:589-607).
  - c8: §2 carry row; §6 U7 bullet (:731-732).
  - c9: U4r builds/tests (replay at U4 with a named test; chosen over moving).
  - c10: Lanes (:708-721); U4s manifests (:461-463).
  - c11: §10 preamble (:799-802), §11 (:899-901).
  - nits: a §9 (:793-794) + §0 :25, §0.1 T21 :73, risk 4 :769; b §4.15 header :433; c U4j README/tests; d U4e tests; e U4a (catalogue note); f U4j + §6 fixes.rs unit (:735).
- Sources read (for accuracy of the edits): marks.json PM-0001-2 checks (soldiers >= 48, smallest team >= 16,
  snapshot clients >= 48, big vehicles >= 15, very large >= 4, projectiles min >= 300, shots mean 665-735, portal changes
  mean 0.95-1.05, everyone_in_view >= 0.99) and PM-0001-30 (parts, late-zero, report buffer_depth_ticks_count, bites);
  marks.json -21 clamp checks (event.clamp_kind.counted_count "never refused or counted"; SERIES.md:144); README
  :190-197 (late-zero; pre: sentence); catalogue :240-253 (item 2 already says "on both sides (plan E7)"; item 3 names
  the counting allocator); checkpoint :269-271 (U2's 35 runs), :305-306 (C3 line); check.mjs:135 UNSAFE_EXCEPTIONS;
  l1 lib.rs:114-138 (P0 = 0 ms, 0 jitter, 0 loss); link lib.rs:203-205 (Loopback rtt Some(0)); ADR 0002:264-265,
  :500-505; plan.md §4.1-§4.7 (:365-611) and §12.1 (:1216-1227).
- Decisions while applying (flag in return):
  - c1: the out-of-ring Rewind's debug assert moves from the sim to the room's rewind_at (else condition 1's unit test
    panics in a debug build); the sim's accessor named `SimCore::reports()` -> `&InputReports`.
  - c3: a fourth runner recording, SS P0 seed 4, used by the pre-check only; the determinism step keeps its three.
  - c9: `replay` stays at U4 with a named test (so plan.md:590-591 needs no edit); bite: the re-decoder's grid
    config floored.
  - c10: U4e's dev dependency on npt-link (Loopback) is added by U4s with the other manifest changes, so U4e touches no
    Cargo.lock (U4d2 owns it in wave 2).
- DONE top line (after the title).
- CORRECTION to the line above: the top line was NOT applied then (no Edit tool in this run). Edits now go through a
  replace-once Python helper (scratchpad), each old text asserted to occur exactly once.
- DONE batch 1 (8 edits): top line; c1 in §4.1 seam row, §4.2 (reports accessor; Rewind refusal + assert moved to
  rewind_at; clamp report; no-trace rule), §4.10 (clamp twin via reports(); twin = forged minus refused part;
  counted_count refusals only), U4d D-10. Open point for U4d's builder: press counters (cumulative) vs "accepted, not
  claimed" state; D-10 runs past the next press to catch it.
- DONE batch 2 (11 edits): c2 in §0 bullet, §0.1 T2 row, §4.1 npt-judge row, §4.15 both-uses, U4d2 (HashCompare takes
  two per-tick hash lists), U4r -30 bullet (rewritten: comparer, late_count, VOID, bites, headers list no mark,
  runs.txt read by the advisor, NOT-RUN until U6), U4j builds (PRE- dropped; loopback one line) and tests
  (--check-marks on marks.json), risk 12; c3 in §4.15 sources (fourth recording SS P0 seed 4) and U4r (setup
  condition, RTT-before-first-sample argument, buffer depth); nit b (§4.15 header); nit c (U4j README hs null).
- DONE batch 3 (13 edits): c8 (§2 row C-U7-1; §6 U7 bullet); c4 (§4.12 -22 row; U4j README §4 sentence; U4f -22
  bullet); c5 (D-6 exact thresholds; U4r contract test passes on each precondition suite); c7 (U4d2 read-back refuses
  with bite; third exception per ADR 0002:264-265, UNSAFE_EXCEPTIONS, plan.md §4.2 before merge; loader-list bite);
  c6 + nit d (U4e: -18 bite end to end; client wrong-channel drop); c9 (U4r: replay = R-8's x86-64 re-decode, its U4
  test with a floored-grid bite); c3 (U4r: four runner recordings); nit f (U4j README/SERIES format; §6 fixes.rs unit).
- DONE batch 4 (11 edits): nit a (§0, §0.1 T21, risk 4, §9: 35 runs for -3, about 285); c10 (U4s manifests; Lanes
  step 0, wave 2, reviewers xhigh in the 3-agent limit, main session on committed state); nit e (U4a: catalogue §6
  item 3 note); c11 (§10 preamble, §11).
- Total so far: 43 replace-once edits to u4-build-plan.md. Next: review the diff against aebe8ea for anything missed or
  unmarked, then the plan.md edits file.
- DONE re-wrap of four lines my edits made over-long (U4r tests; §0; §4.15 Fields; U4e Bites). Hunk check: every diff
  hunk against aebe8ea carries a condition or nit marker (43 hunks). Next: u4-plan-md-edits.md.
- DONE u4-plan-md-edits.md: 13 plan.md edits (1 c7(c) §4.2 three exceptions; 2 §4.1 hash-wasm layout line, consistency;
  3 Q7 §4.7 RoomControl; 4 Q19 E7 ClientEndpoint ns; 5 Q19 host.ts next_due_ns; 6 Q19 §4.7 step order next_due_ns; 7
  Q13+Q6 §4.4 per-tick inputs; 8 Q25 §4.4 clamp "(in the sim)"; 9 Q6 plan.md:562 loss to U6; 10 c8 U7 row C-U7-1; 11-12
  risk 11 §4.7 SimCore/HitHistory to npt-sim; 13 risk 11 §4.2 proto -> sim edge) + 2 checkpoint changes (C3 line; C-U7-1
  entry). Every Old block counted exactly once in its target (plan.md sha256 daf25395...; the checkpoint as on disk);
  no two plan.md Old blocks overlap.
- FINISHED. u4-build-plan.md now 50 replace-once edits over 5 batches + 1 re-wrap batch (47 applied edits counting
  re-wraps as part of their condition), every hunk against aebe8ea marked.
- CORRECTION to the line above: 43 substantive replace-once edits (batches 1-4: 8 + 11 + 13 + 11) plus 4 re-wraps of
  lines those edits made over-long = 47 operations. Final u4-build-plan.md: 1012 lines, sha256 f7961cfe...; 43 diff
  hunks against aebe8ea, each marked.
