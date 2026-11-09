# U4s build proof: rust-reviewer

Worktree: /home/user/Shamble-Frontlines/.claude/worktrees/wf_aa2cc979-004-1 (uncommitted; base c8abc857)
Plan: u4-build-plan.md §4.1-§4.4, §4.11, §5 U4s (:492-508), §7.1; plan.md §4.5 (determinism), §4.8 (pins).
Rules: CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target on every cargo call; touch crates before
each build; df -h /home/user before each build (stop < 2 GB); --locked offline; git only in the copy (no -C/-c);
Write/Edit only; commit nothing; bites in place, one at a time, sha256 restored.
Tooling note: the Edit tool is not available in this session, so this notes file is rewritten whole with Write, and the
plants go through my small script u4s-rust-plant.py (this folder): in place, reverse replacement, sha256 checked
against the baseline; no copy of any source file is made.

## Plan
1. Record sha256 of every file I will plant in (baseline); git status of the copy.
2. Run 1: touch, df, `node spikes/netcode-proof/check.mjs` unchanged; record the nine step lines and counts.
3. Bites (one at a time, in place; run; record failure by name; restore; sha256 check):
   the plan's two (G1 grid floor, H1 ring one short), one more ring variant (H2), check.mjs self-test bites
   (C1-C10, builder's and mine), real-tree deps plants (R1-R3).
4. Review the Rust: unsafe, determinism (floats, libm, hash maps, const fns), overflow, allocation, wasm32;
   libm =0.2.16 pin, features, Cargo.lock (no new package), cargo tree -d; a native vs wasm32 run of the grid.
5. Final: touch, df, check.mjs again; fingerprint equal to step 1.
6. Findings + verdict. DONE.

## Progress
(started 2026-11-09)
- Step 1: base c8abc857 (git log -1); 11 M + 5 new source + 2 builder notes; no target/ in the copy. Fingerprint of
  every file under spikes/netcode-proof and .checkpoints (391 files, sorted sha256sum | sha256sum):
  074e11d25c14f29b9e0c4826a463bf9ad3ae5b6aebe092d534c2806233968858. Per file: grid.rs 2446152d..., history.rs
  d4351e08..., ticks.rs b5bbcbf2..., check.mjs 74f75b74..., check.test.mjs 09d67b79..., room lib.rs fd4db3af...,
  Cargo.lock 3cb230ca..., seam.rs 6dd94205..., tests/seam.rs 6cba21bd....
- Run 1 (touch; df 13G free; CARGO_TARGET_DIR shared; CARGO_NET_OFFLINE=true added so a fetch would error): exit 0.
  PASS fmt 0.3 s; clippy 2.8 s; clippy-wasm32 0.5 s; test 80.5 s "measured build: 273 tests passed, 0 failed, 0
  reported a skip; bites build: 288 tests passed, 0 failed, 0 reported a skip"; deps 0.6 s; refs "528 tracked files";
  marks "81 marks"; self "45 of 45"; redact "10 of 10 tests passed; 156 item(s) checked, 0 finding(s)". "check: all 9
  steps pass". Counts = 267+6, 281+7, 43+2: as expected.
- libm 0.2.16 (registry source): features default=["arch"]; arch_enabled = arch && !force-soft-floats
  (configure.rs:93-101); wasm32 arch module needs intrinsics_enabled = unstable-intrinsics (configure.rs:82-91,
  arch/mod.rs:12-16), so on wasm32 stable NOTHING from arch; on x86-64 (sse2) arch gives sqrt/sqrtf (sqrtsd asm) and
  fma/fmaf with RUNTIME cpuid dispatch (arch/x86/fma.rs:9-27). All correctly rounded -> identical results. The
  workspace comment (Cargo.toml:50-53) lists "sqrt, floor, ceil, trunc and rint": wrong list (nit).
- --list (touch, df 13G each): measured tests/seam.rs 6 (s1, s2 x2, s3, s4, s5), lib 0, doc 0; bites 7 (+
  the_grids_bites_are_listed_chosen_per_value_and_each_moves_values_off_the_wire_grid).
- G1 grid floor (grid.rs:146 round -> floor): 6 ran; FAILED s2_from_then_to... ("POSITION_XY: -32767 came back from
  -3.2767e2"), s2_to_errs... ("-3.27675e2 (scaled -32767.498779296875) went to -32768, -0.501220703125 steps off"),
  and s3 ("0.125"). Restored 2446152d == baseline.
- H1 ring one short (history.rs:22 capacity - 1): 6 ran; FAILED s4 only ("capacity 64: 64 held after 64 pushes").
  Restored d4351e08 == baseline.
- H2 (mine) ring one short at get (history.rs:69 age + 1 >= len): FAILED s4 ("capacity 64: tick 4294967255 held") and
  s5 (seam.rs:380, the stub's one-tick ring). Restored d4351e08 == baseline.
- check.mjs self-test bites, full `node --test check.test.mjs` each (45 tests), restored 74f75b74 == baseline each:
  C1 call counts as derived (builder's): not ok 44 only. C2 TICK_HZ anywhere first (builder's): not ok 44.
  C3 (mine) `static` not read: not ok 44. C4 (mine) tick grep reads comments unmasked: not ok 44.
  C5 (mine) exponent literal alternative dropped: not ok 45. C6 (mine) float grep reads crates/room/ (tests too):
  not ok 45. C7 (mine) lookbehind dropped (0x1e3 -> 1e3): not ok 45. C8 float literals missed (builder's): not ok 37
  AND 45 (builder ran 45 alone). C9 tick grep unhooked: not ok 37. C10 float grep unhooked: not ok 37.
- Real tree, `node check.mjs --only deps` (touch, df 13G each): R1 `pub const SCALE: f32 = 0.5;` in room lib.rs:
  FAIL deps "crates/room/src/lib.rs:7 has `f32` (2 float token(s))". R2 LOOP_CATCHUP = 7: FAIL deps
  "crates/sim/src/ticks.rs:36 sets LOOP_CATCHUP from `7`". R3 (mine) `pub const TICK_HZ: u32 = 30;` in room: FAIL deps
  "crates/room/src/lib.rs:7 sets TICK_HZ from `30` outside crates/sim/src/". All restored == baseline; status: all
  five plant targets == baseline.
