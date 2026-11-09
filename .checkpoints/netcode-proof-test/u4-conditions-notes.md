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
