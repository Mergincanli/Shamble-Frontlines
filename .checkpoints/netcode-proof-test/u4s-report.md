# U4s report: the seam types, the wire grid, the tick constants and the manifests

Written 2026-11-09 about 17:30 UTC by the main session, for the advisor (u4-build-plan.md §7.2: one review round per
sub-unit on its final diff, then a short report). Merged at f080551.

## What happened
- U4s, the U4 build plan's first sub-unit, is merged: the tick constants, the wire grid, the seam types and
  `HitHistory` in npt-sim, every manifest change wave 1 and wave 2 need, and the check.mjs greps.
- Three reviewers (netcode-architect, engine-architect, rust-reviewer) approved with findings, no high; a fix pass and
  a last small round closed every finding they asked to fix in U4s; the rust-reviewer checked both rounds (no
  regression). The last gap (G1a: the bite-id pattern missed 16 of marks.json's 114 ids) was fixed by the main session,
  with a test that fails on the old pattern.
- **A deviation from §7.2:** the plan names the main session as U4s's builder. One builder agent built it instead,
  alone in an isolated worktree with nothing else running (the same isolation), so the main session's context stayed
  free for the merges; the main session merged it and made the last fix.

## What was built (f080551)
- **npt-sim:** `TICK_HZ` (30) and the derived tick constants as const fns (`LOOP_CATCHUP` 7 at 30 Hz and 15 at 60 Hz;
  `ticks_for_ms`; the full-state-ack, ack-ring and settings-step ticks), each const-asserted equal to its function at
  `TICK_HZ`, the rate held in 1-1,000 by a const assert; `grid` (`Field`, `Config`, `to_grid`/`from_grid`/`snap`, the
  tie rule, a bites-only float-side floor defect); the seam (`SimCore`, `Command`, `Rewind`, `TickInput`,
  `EntityRecord` with `EntityKind::layout()` as the one per-kind field table, `HitVolumes`, `PortalStates`, `SimEvent`,
  `InputReports` outside the state hash); `HitHistory::for_ms` (refuses 0 ms; oldest evicted). The docs state what the
  sim reads (accepted fields only: `tick` and the view time belong to the room and the recording), the canonical
  command order, the grid/raw rule and the Rewind edge cases (their behaviour tests are carried to U4d).
- **Manifests:** libm =0.2.16 (already in Cargo.lock: no new package), rand_chacha and rand_core in npt-sim; npt-proto
  gains npt-sim and npt-link (plus rand dev dependencies for U4a's seeded tests, so no wave-1 lane edits a manifest);
  npt-client npt-link; npt-room npt-sim; `bites` features for every wave-1 and wave-2 crate with npt-lab's forwarding;
  npt-server's dev dependency on npt-client (condition 10). Cargo.lock: edges only, offline; `cargo tree -d`'s
  duplicate set unchanged.
- **Bite ids live with their sites** (arch F1; §4.14): -17's in npt-client, -41's at the sim's own-copy snap; -57's
  integer coarsening carried to U4a's `Quant`.
- **check.mjs** (now **10 steps**): the tick-constant grep (each derived constant is its own function at `TICK_HZ`; no
  re-derivation outside npt-sim; no literal `*_TICKS`), npt-room's float grep, npt-sim's dependency allow-list, a bite
  id found in two crates' sources refused (inline `#[cfg(test)] mod` blocks skipped; every marks.json id matched), and
  a `doc` step (`cargo doc --no-deps --workspace`, rustdoc warnings as errors, both builds). One doc link in npt-l1 made
  plain text (the only rustdoc warning in the 16 crates).
- **Public items** (A19): npt-sim's as listed above (the reviewers checked them against §4.1; five are field or config
  types §4.1's items need: `CoverVariant`, `EntityKind`, `ClampReport`, `grid::Field`, `grid::Config`); npt-client's
  bites-only `BITES` and `grid_with_bite`; nothing in npt-proto but dev dependencies.

## Proof
| What | Result |
|---|---|
| Reviews | netcode-architect, engine-architect, rust-reviewer: APPROVE WITH FINDINGS, no high (u4s-review-*.md) |
| Fix pass (items 1-11) and the last small round (G1, G2, the npt-sim membership message) | checked by the rust-reviewer, no regression (u4s-fix-review-rust.md, u4s-fix2-review-rust.md); G1a fixed by the main session |
| Bites | 9 at the build, 31 at the fix pass, the last round's plants (3 plants, 5 reverts), the G1a revert: each failing its own test or step for its own cause, every file restored by sha256 |
| check.mjs (main folder, own cache) | all 10 steps pass: 275 measured, 292 bites, 0 failed, 0 skips; self 48 of 48; doc 3.2 s; redact 157 items, 0 findings |
| Gate | GATE PASS with --no-smoke (5a: the container's Chromium/Playwright mismatch; CI runs the smoke) |

## Carried forward (u4s-carries.md: 33 entries by brief)
- Into U4a's: -57's integer coarsening in `Quant` with its ids; the "Channel only" grep for npt-proto and npt-client;
  pitch code -512 refused by the decoder; the netcode reviewer's finding 7 and nit 11.
- Into U4d's: the equal-hash test for inputs differing only in `tick` and the view time (netcode 1); the World's debug
  assert of the canonical command order (netcode 2); the grid/raw rule's test (netcode 4); the Rewind behaviours'
  tests (netcode 5); `HitVolumes`'s contents and its `Clone` (rust 2); `TickInput`'s spawn, teleport and attribute
  fields; s6's exhaustive match for `EntityKind::ALL`.
- Into U4b's, U4e's, U4h's and U4d2's or the main session's: as listed in the file (for example the 1 MB cap on
  `HitHistory` with `HitVolumes`'s size at U4h; a bite id's uniqueness beside carry 24 at U4e).
- Smaller notes for the main session: no self-test pins `STEPS.doc` (as for fmt, clippy and test); the doc step
  documents the host only (npt-tcpinfo's non-Linux stub never documented); `lintBothBuilds`'s "both linted" wording
  when only the bites build fails.

## Question for the advisor
- **NF-D4: drop `InputReports::non_finite` and reword U4d's test D-4?** Plain words: the plan has the sim refuse and
  count a non-finite input component, but `Command` carries integers only (move `[i8; 2]`, aim `[i32; 3]` in cm, so the
  room stays float-free), so nothing that crosses the seam can be non-finite and the counter can never count.
  - (a) Drop the counter and reword D-4 as "integer by type; a debug assert that the World holds no non-finite value
    each tick", plus an integer-overflow test of the command fields.
  - (b) Keep the counter, documented as always 0 (as built now), and D-4 as written (it cannot fail).
  - Recommendation: (a). It is a plan text change to U4d's D-4, so it is the advisor's.

## Decisions needed
- None for Mustafa. For the advisor: NF-D4, and whether the builder deviation needs anything more than this note.

## For Mustafa
Nothing.
