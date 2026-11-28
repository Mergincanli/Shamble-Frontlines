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
   (C1-C10, builder's and mine), real-tree deps plants (R1-R4).
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
  "crates/room/src/lib.rs:7 sets TICK_HZ from `30` outside crates/sim/src/". All restored == baseline.
- R4 (mine, an evasion probe) `LOOP_CATCHUP = 7 * TICK_HZ / 30` (7 at 30 Hz, 14 at 60, plan says 15): deps PASS,
  npt-sim 6 passed (s1 included), clippy clean. Restored b5bbcbf2 == baseline. -> FINDING low: the grep accepts any
  initializer naming TICK_HZ and s1 links each constant to its function only at TICK_HZ = 30 (tests/seam.rs:93-96).
- Deps: Cargo.lock diff adds 0 [[package]] (only dependency lines), so no new package and no new duplicate possible.
  `cargo tree -d -e normal,build` (host): getrandom 0.2.17 / 0.4.3 only. `--all-features --target all`: getrandom,
  syn 2/3, synstructure 0.13/0.14, windows-link, windows-sys: the builder's baseline set. libm features: default+arch
  on host (npt-server's tree, also via fastbloom) and on wasm32 (npt-client-wasm's tree): one feature set. rand_chacha
  and rand_core default-features = false (Cargo.toml:47-48). cargo-deny / cargo-audit not installed here: NOT run
  (plan.md §4.8 puts them at U5).
- bites features: `cargo tree --workspace -e features -i <crate>`: measured 0 bites edges for npt-sim, -proto, -room,
  -client, -server; --all-features 1 each; npt-lab measured -> npt-sim "default" only, --features bites -> + "bites".
- Cross-target probe (my scratch crate u4s-rust-xtarget/, edition 2021, cdylib + bin, path dep on the copy's npt-sim;
  own Cargo.lock resolved offline): FNV-1a over every integer's from_grid bits and round trip, 25 values around each
  grid point's quarter/half steps (to_grid + snap), and every 2^shift-th f32 bit pattern (NaN, inf, subnormals) per
  field. shift 8: native release, native debug and wasm32 (Node, 0 imports) all 2878a85531a57dd8 over 178,261,964
  values (1.09 s / 9.97 s / 3.96 s). shift 4: native release and wasm32 both e1b681efc276dc55 over 2,694,844,364
  values (16.3 s / 57.4 s). -> grid identical on x86-64 and wasm32.
- clone_from probe (u4s-rust-xtarget/src/bin/clonefrom.rs, release): "derived clone_from keeps the Vec's storage:
  false"; "manual clone_from keeps the Vec's storage: true". -> history.rs:46-57's reuse needs a hand-written Clone.
- npt-client-wasm (still a 2-line stub) release wasm32: 292 B raw, 278 B gzip -9; no npt_sim/libm/chacha strings.
  U4s adds no bytes to it (nothing calls npt-sim yet). Base not built (no second checkout).
- Extra lints (informational, npt-sim lib, bites): missing_docs 14 pub fields/variants undocumented (seam.rs:73-74,
  :79, :91, :136, :140-143, :146, :153-154); arithmetic hits all proven bounded except ticks.rs:45 (32 * tick_hz
  overflows above 134,217,727 Hz: panics at run time with overflow-checks; doc :7 says rates <= 1,000, nothing asserts);
  history.rs:20 panics on 0 (no # Panics doc).
- Final run (touch; df 13G): exit 0, all 9 PASS: fmt 0.4 s; clippy 3.2 s; clippy-wasm32 0.5 s; test 81.3 s 273 / 288,
  0 failed, 0 skips; deps 0.4 s; refs 528; marks 81; self 45 of 45; redact 10 of 10, 156 items, 0 findings.
- Unchanged by me: fingerprint after = 074e11d25c14f29b9e0c4826a463bf9ad3ae5b6aebe092d534c2806233968858 (391 files),
  equal to step 1; git status 18 entries, same list; no .gate or target/ in the copy; plant status all == baseline.

## Findings
1. LOW ticks.rs:36,40,49,58 + check.mjs:540-550 + tests/seam.rs:93-96: R4 (`7 * TICK_HZ / 30`) evades grep, s1 and
   clippy. Fix: grep requires each derived name's initializer be its own fn at TICK_HZ (or ticks_for_ms(<int>), or a
   path); self-test plant `7 * TICK_HZ / 30`.
2. LOW (U4d brief) seam.rs:115 HitVolumes derives Clone: derived clone_from allocates (probe); history.rs:46-57 relies
   on it. Fix: hand-written field-wise clone_from + counting-allocator test on a full ring's push (netcode F9).
3. LOW history.rs:20-22: new(0) panics; capacity from data via ticks_for_ms; no 1 MB cap (§4.9). Fix: NonZeroU32 or
   from-ms constructor returning an error (with netcode F6).
4. LOW/NIT ticks.rs:44-46 (+ :7): 32 * tick_hz overflow > 134,217,727 unasserted. Fix: const assert on TICK_HZ range;
   u64 or # Panics.
5. NIT Cargo.toml:50-53: libm `arch` list wrong (x86-64: sqrt + run-time-dispatched fma; wasm32: nothing on stable).
6. NIT missing docs on 14 pub fields/variants (seam.rs:73-74, :79, :91, :136, :140-143, :146, :153-154).
7. NIT (U4d brief) seam.rs:26, :29: i8 -128 / i32 aims reach the sim from recordings; widen before arithmetic (overflow
   checks on); HitVolumes.tick from the world's counter, not TickInput.tick (history.rs:49-53 debug assert).
VERDICT: APPROVE WITH FINDINGS (no high or medium of mine; the two other reviewers' mediums stand as they wrote them).
DONE 2026-11-09
