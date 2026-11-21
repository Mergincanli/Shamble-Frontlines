# engine-architect: U4s build review (module boundaries)

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_aa2cc979-004-1 (uncommitted; read only)
Plan: u4-build-plan.md §4.1-§4.4, §4.11, §5 U4s; advisor-u4.md (condition 10).

## Plan
1. git status / git diff in the worktree; list changed files.
2. Manifests: the four new edges vs ADR 0002:186-191 and plan.md §4.2; no other edge; bites features and npt-lab
   forwarding (condition 10 a/b); npt-server dev-dep on npt-client; Cargo.lock delta.
3. npt-sim stays free of everything above it (deps, uses).
4. Public items vs §4.1 and plan.md §4.7 (incl. the five extra types).
5. check.mjs greps: tickConstantProblems (§4.3) and roomFloatProblems (§4.11), wired into the deps step; self-tests.
6. Boundary sanity: seam signatures vs §4.2 (step, reports, Rewind, HitHistory), state ownership.
7. Findings + verdict.

## Notes (one line per finding as found)
- Step 1 done: 11 modified, 5 new (sim src x4, sim tests/seam.rs) + builder notes; base c8abc85.
- Step 2: edges exactly proto->{sim,link}, client->link, room->sim, server dev->client; lock +12 lines match. ADR
  0002:186-191 (protocol row holds the transport interface, which the spike splits into npt-link) allows each. Holds.
- Step 2: bites: sim [], proto->sim, room/client->sim+proto, server->sim+proto+room, lab->l1,sim,proto,room,server,
  client. Condition 10 (a) and (b) met. Server dev-dep on client: not shipped (closures skip dev, check.mjs:45-46).
- Step 3: npt-sim deps libm, rand_chacha, rand_core; sources import nothing outside the crate. Holds today, but no
  check pins it (dependencyProblems check.mjs:68-127 has allow-lists for judge, tcpinfo only) -> F4 low.
- Step 4: pub items within §4.1 rows 2-4; the five extra types are field/element types or grid's own -> approve.
- F1 (medium): grid::Config bites: -57's three act in to_grid (float), stated site encoder/Quant holds integers only
  (room float-free; record carries grid ints): grid.rs:2,:17,:191,:193-199. -17 and -41 share Bite::Floor; ids of other
  crates' sites live in npt-sim.
- F2 (medium): cross-lane contracts open in npt-sim: EntityRecord per-kind field order (seam.rs:98-100) set by U4a's
  table in proto, filled by U4d in sim; VELOCITY provisional (grid.rs:117-127) decided by U4a, file owned by U4d.
- F3 (low): InputReports::non_finite (seam.rs:172-173) has no producer: Command integers only (seam.rs:25-29); U4d D-4.
- F5 (low): proto->link, client->link "Channel only" unenforced.
- F6 (low): tick grep: closed name list (check.mjs:517), re-derivation outside sim allowed (:550), clean fixture has
  LOOP_CATCHUP_ALIAS = 7 (check.test.mjs:1258) vs plan.md §4.6 "no literal 7 anywhere".
- F7 (low): U4a's seeded tests need a PRNG; proto has no dev-deps; adding one in wave 1 edits Cargo.lock (A9).
- F8 (low): float grep's known limit; clippy::float_arithmetic exists (cargo clippy --explain, stable here).
- N1-N3 nits: Command's claimed view time in the sim; Config's shape by feature; step's borrowed return.
- Verdict: APPROVE WITH FINDINGS (F1, F2 settled before wave 1's briefs).

DONE (2026-11-09).
