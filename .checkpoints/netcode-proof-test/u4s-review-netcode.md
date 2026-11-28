# U4s review: netcode-architect (read-only)

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_aa2cc979-004-1 (uncommitted; base c8abc857)
Plan: u4-build-plan.md §4.1-§4.4, §4.11, §5 U4s (:492-508); advisor-u4.md conditions 1, 10(a)/(b).

## Plan
1. git status / git diff --stat in the worktree; confirm base and file set.
2. seam.rs vs §4.2 + ADR 0004 (step signature, HitHistory lent &, integer Rewind, reports() outside hash, no World).
3. history.rs vs §4.2/§4.9 (fixed ring, capacity from ticks_for_ms, eviction, get by tick).
4. ticks.rs vs §4.3 (TICK_HZ 30, LOOP_CATCHUP TICK_HZ/4, ticks_for_ms ceil, derived constants, 30/60 values).
5. grid.rs vs §4.4 (1 cm, yaw 16, pitch 10 over ±90, tie rule, error bound, config value not static).
6. tests/seam.rs s1-s5 + bites; check.mjs greps (tick, float).
7. Manifests vs U4s list + condition 10; nothing of U4a-U4h pulled forward.
8. Findings + verdict.

## Notes (one line per finding as found)
- Steps 1-7 read: lib.rs, ticks.rs, grid.rs, seam.rs, history.rs, tests/seam.rs, check.mjs/check.test.mjs diffs, all
  manifests, Cargo.lock diff (+12, exactly the named edges), builder notes. No source outside crates/sim/src changed;
  no World, Quant, Schedule, room/client code: nothing of U4a-U4h built.
- Verified OK: step(&mut self, &TickInput, &HitHistory) -> &[SimEvent] = §4.2; Rewind {u32, u8, CoverVariant} integers;
  reports() outside hash by doc; ClampReport not a SimEvent; TICK_HZ 30, LOOP_CATCHUP = hz/4 (7/15), ticks_for_ms
  ceil in u64 saturating, 60/120, 32/64, 30/60 pinned in s1; grid XY 16b ±327.68, Z 12b 0-40.95, yaw 16b wrap,
  pitch 10b ±511 (both ends exact, step 0.17613 deg < -57's 0.18), round() on ADR 0004:262-263 allow-list, ties away
  from zero pinned s3; s2 exhaustive 201,727; Config per value, bites-only; HitHistory get/push index math correct incl.
  u32 wrap and non-contiguous guard; F116's Input fields match Command (buttons u16, presses 4xu8, move 2xi8, aim cm).
- F1 (medium) seam.rs:17-33: Command hands the sim client-claimed tick/view_tick/view_fraction beside the accepted
  Rewind; nothing says the sim must not read them (condition 1: sim keeps what the server accepted). Lag-comp cap
  bypass hazard. Fix: drop view_* from the seam Command (P0 recordings with 0 replacements recover them from the
  Rewind), or doc "sim never reads" + D-9/D-10 case (differ only in those fields -> equal hashes).
- F2 (medium) seam.rs:65-67: "in the order the room runs them" leaves arrival order open; -30's two legs compare
  per-tick server hashes. Fix: canonical order (slot ascending, then command tick within slot), sim debug-asserts.
- F3 (low) seam.rs:172-173 non_finite unreachable (integer Command); reword D-4, drop or doc the counter; record the
  reading of §4.2's list for the advisor.
- F4 (low) seam.rs:98-103 grid/raw invariant unstated; -41/server-quant-differs unit proof (§4.14) and -57's pairing
  (§4.6 T20) need raw = pre-snap value, grid = wire Config.to_grid(raw), apart from the snap's Config.
- F5 (low) seam.rs:45-57, :65-67: Option<Rewind> None on a firing command vs a refused Rewind must be one behaviour
  (D-10 twin = refused part removed); "a tick the ring does not hold" must name tick_a+1 when fraction > 0.
- F6 (low) history.rs:19-28 new(capacity_ticks) admits a literal; A5. Fix: from ms via ticks_for_ms.
- F7 (low) grid.rs:117-127 + seam.rs:98-108 + seam.rs:53-54: VELOCITY step, FIELDS = 12 / per-kind order, fraction 8
  bits fixed in crates/sim (U4d's in wave 1) though §4.2/§4.4 leave them to U4a's table: record as givens in U4a's brief.
- F8 (low) check.mjs TICK_DERIVED closed list; suggest *_TICKS name rule.
- F9 (low) seam.rs:115-119 HitVolumes derived Clone: clone_from not field-wise (ADR 0004 part 3); fields pub.
- F10-F12 nits: fire bit/press index constant; pitch -512 refused by U4a decoder; SimEvent has no tick (ADR 0004
  part 19) - room stamps it.
- F1 revised: §4.15 (u4-build-plan.md:460-462, :466-469) puts tick, view tick and fraction in the recorded Command and
  the -30 pre-check re-encodes them, so the builder followed the plan; fix = keep the fields, state in seam.rs that the
  sim never reads them (it reads the Rewind), U4d test. Side nit: §4.15 sizes move at 4 B, Command's is 2 B (F116).
- Verdict: APPROVE WITH FINDINGS (no high; F1, F2 doc sentences in seam.rs before merge, their tests into U4d's brief).
DONE (2026-11-09)
