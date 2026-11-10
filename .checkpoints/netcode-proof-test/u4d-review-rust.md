# U4d build review: rust-reviewer (read only)

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_f54246ff-e0d-1 (base 097f897, uncommitted).
Plan: u4-build-plan.md (sha 7ec1b324 per builder; plan now 0d69332c after F1) §5 U4d, §4.1-4.3, §4.13-4.15; advisor-u4.md U4s ruling; u4s-carries.md (U4d entries).

## Plan
1. Read carries (U4d entries) and builder notes.
2. Read diffs: seam.rs, history.rs, lib.rs, grid.rs, tests/seam.rs.
3. Read new files: rng, hash, trajectory, greybox, scene, world, replay; tests world, replay.
4. Check: float allow-list (sim/clippy.toml) vs uses; libm only; NaN assert; HashMap/clock; fixed pools / no per-tick alloc; integer overflow (as casts, wrapping, sat); hand-written Clone of HitVolumes; wasm32 parity (usize, f32 ops, fma, mul_add); safety (panics on recording decode, unbounded alloc, unsafe).
5. Findings + verdict.

## Progress
- 07:08 UTC: steps 1-3 mostly done (carries, notes, seam/history/lib/grid diffs, rng, hash, greybox, scene, trajectory, world read). File sha256s match the builder's final list.
- OPS: an orphaned touch loop (pid 15338, started 06:38, the builder's step-8 command) still touches every file under the worktree's crates/ every 2 s. Content unchanged; it forces rebuilds. Report to main session; not killed (read only).
- Candidate findings so far:
  - F-claimed: World::claimed (last press counters seen) decides later ticks but is outside write_state/hash (doc of write_state says "every field that decides a later tick" then excepts it). Needed for D-10's twin; plan §4.2 says sim input state holds what the server accepted, never the client's claim. Also not restorable for U4g rollback. Severity: medium (design; netcode to rule) / low for Rust.
  - F-finite: D-4's finite check walks write_state only; pre (records' raw), movers, blend boxes not checked. Low.
  - F-stream-idx: rngs[Stream::X as usize - 1] relies on discriminant = STREAMS position + 1; no const assert. Nit.
  - F-fired-cap: fired Vec::with_capacity(256) can grow past 256 with catch-up (48x8 commands); one-time growth. Low.
- RESUME (after usage-limit cut): sha256s still match builder list; touch loop pid 15338 no longer running. Re-reading files to finish steps 3-5 (replay, tests, world detail).
