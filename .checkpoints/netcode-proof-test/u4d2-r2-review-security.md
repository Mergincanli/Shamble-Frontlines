# U4d2 review round 2: security-auditor (read only; on what changed: FX-1 to FX-12)

Seat: the advisor's U4d2 round-1 ruling, Block 8, batch 1, security-auditor. Copy run in:
/home/user/Shamble-Frontlines/.claude/worktrees/u4d2-build (only through u4d2-run.sh; never edited). Read copy:
/home/user/Shamble-Frontlines/.gate/u4d2-r2. Round 1's tree: /home/user/Shamble-Frontlines/.gate/u4d2-r1. Logs:
/home/user/Shamble-Frontlines/.gate/u4d2/r2-sec-*.log. Probe scripts (text): this notes folder, sec-r2-*.mjs and
sec-r2-home/<case>/config.toml, each importing the copy's check.mjs or loader.mjs and feeding it in-memory inputs or
those probe files (no edit of any copy). Built probe modules: .gate/u4d2/sec-probe/. This agent has no Edit tool: the
notes are rewritten whole with Write each time.

## Plan
1. Read the packet (heads, diffs, Blocks 1-8, T1-T5, carries) and the tests side file; my round-1 notes and probes.
2. Verify the read copy equals the builder's copy (fx-hashes-end.sha256).
3. FX-1: buildInputProblems on every round-1 name-rule case in every env and config form, RR-M2's pair,
   CARGO_BUILD_TARGET and build.target forms; clean passes. Live `--only deps` per environment; real config files
   through depsStep (CARGO_HOME probe folders). Near routes: a config [profile] table, a `linker`.
4. FX-2: moduleProblems on sec-probe-tf2's module, both clean built modules, hostile section bytes; near route:
   link args that make wasm-ld write the section.
5. FX-4: each import refused by name beside a clean section; round 1's sec-probe-imports.mjs.
6. FX-6: V4's two forms and near forms (escaped literals, `include!` split from its argument by a macro).
7. FX-3: an `unsafe` block in a listed export's body refused at its line (round 1's V1 re-run).
8. FX-5: kinds. 9. SEC-U4D2-3's comment; C-U7-2 (T3) and T5's security lines against what stays open.
10. check.test.mjs through the runner (counts). 11. Findings, coverage table, verdict.

## Blocks
### Block 1: read and heads (done)
- `sha256sum -c .gate/u4d2/fx-hashes-end.sha256`: 18 of 18 OK in .gate/u4d2-r2 and in the builder's copy; the copy
  is 51520b4, uncommitted (git status read). Packet, tests side file, Blocks 1-8, T1-T5 and the carries read.
- FX logs read: fx-1-env.log:7-13 (the pair FAILs naming both sources, :9-10; clean PASS; CARGO_BUILD_TARGET and
  `--cfg feature="bites"` FAIL by name); fx-2-tf.log (the simd128 plant: wasm32 build exit 0, deps PASS, wasm FAIL
  with every loader call refused "lists +simd128").

### Block 2: FX-1 in memory (done; r2-sec-fx1.log, sec-r2-fx1.mjs, exit 0)
- 163 of 163 cases as expected. Refused by buildInputProblems, each naming its source and value: every round-1
  name-rule flag (llvm-args, `--cfg feature="bites"`, overflow-checks=off, target-cpu=haswell, +avx512f,
  `-Ctarget_feature=+fma`, `--codegen=target-feature=+sse2,+fma`, plus `-C link-arg=...` and `-C linker=...`) in each
  of RUSTFLAGS, CARGO_ENCODED_RUSTFLAGS (\x1f), CARGO_BUILD_RUSTFLAGS, CARGO_TARGET_<x86-64|wasm32>_RUSTFLAGS and
  CARGO_PROFILE_RELEASE_RUSTFLAGS (54), and in 10 config forms ([build] array and string, [target.<triple>],
  [target.'cfg(...)'], dotted, inline table, quoted, escaped-quoted, literal key, [profile.release]) (90); RR-M2's pair
  (2 lines, both sources); CARGO_BUILD_TARGET set to a triple, wasm32, "" and " "; `target` in 9 config forms (array,
  dotted, inline, quoted, escaped key, a .json spec among them). Passed: empty env, the container's variables with
  empty RUSTFLAGS and CARGO_ENCODED_RUSTFLAGS, spaces only, target-dir with a [target] linker, `rustflags = []`.
- Whitespace edge (note, no finding): RUSTFLAGS=U+FEFF is split away by JS's \s but kept by cargo (split(' ') then
  Rust's trim, which keeps U+FEFF), so rustc gets "﻿" as an extra input and fails: fail closed. U+00A0 and tab
  are dropped by both; U+0085 is refused (JS's \s keeps it).

### Block 3: FX-1 live (done)
- r2-sec-fx1-live.log (-x -k; exit 1 by design): deps FAIL, each naming its source and value: RR-M2's pair (:4-6, both
  sources named), `-C llvm-args=-fp-contract=fast` (:11), `-C overflow-checks=off` (:17), `--cfg feature="bites"`
  (:23), CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS=+simd128 (:29-32: name rule, FX-1 and read-back),
  CARGO_BUILD_TARGET=wasm32 (:37), CARGO_BUILD_TARGET= empty (:43); clean `--only deps` PASS (:49-51).
- r2-sec-fx1-config.log (-x; sec-r2-deps-home.mjs: depsStep with CARGO_HOME at a probe folder, so cargoConfigFiles
  reads a real file; cargo and rustc keep the real environment): /nonexistent PASS (1 config read); `[build] target`
  FAIL naming the file's `target` key; `[target.'cfg(target_arch = "wasm32")'] rustflags = ["-C",
  "link-arg=--no-check-features"]` FAIL naming its rustflags. NOT refused (2 configs read, deps PASS):
  `[profile.release.package.npt-sim] overflow-checks = false`, and `[target.wasm32-unknown-unknown] linker`.
- r2-sec-routes.log (-x -k; exit 127 only for my `wasm-objdump --version` tool check: not installed, no test): `cargo
  check --release -p npt-sim -v` gives npt_sim `-C overflow-checks=on` (metadata 804558ea…); with `--config
  profile.release.package.npt-sim.overflow-checks=false` the same line has no overflow-checks flag (metadata
  c23b0317…): a config's package profile beats the manifest's (Cargo.toml:115-119). Cargo's profiles page: "Specifying
  a profile in a config file or environment variable will override the settings from `Cargo.toml`."
  (https://doc.rust-lang.org/cargo/reference/profiles.html; read 2026-11-13; it does not rank package overrides,
  which the run settles).

### Block 4: FX-2, FX-4, FX-5 (done; r2-sec-loader.log and r2-sec-loader2.log, sec-r2-loader.mjs, exit 0)
- A. The two clean built modules (wasm-step/bin, from fx-check2's run at 00:28): 0 problems, instantiated; no
  f32x4.relaxed_madd opcode in their code. sec_probe_tf2.wasm (round 1): refused by name, "+relaxed-simd" and
  "+simd128" (beside its missing exports). sec_probe_tf2_ld.wasm (same source, rustc with `-C
  link-arg=--no-check-features -C link-arg=--features=bulk-memory,mutable-globals,sign-ext`, r2-sec-routes.log:28-29):
  section "+bulk-memory +mutable-globals +sign-ext", its code still holds fd 85 02 (f32x4.relaxed_madd) once, and
  moduleProblems has no target_features line: the section is wasm-ld's statement, which link args rewrite.
- B. Hostile section bytes, each in an otherwise clean module: empty body, a 6-byte LEB, count 2^32-1 with one entry,
  count 2^35-1 with none, name length past the end and 2^32-1, prefixes 0x00 / '!' / 0xff, names c0 af, ed a0 80 and
  ff (fatal UTF-8), two sections (both clean; clean + simd128), a trailing byte, +simd128, =relaxed-simd, -simd128 then
  +simd128, and a section only under another name: each refused with its own sentence, instantiate refused. Accepted:
  the built list, `-simd128 -relaxed-simd`, and (notes) count 0, 'SIMD128', 'simd128\0' (names wasm-ld never writes;
  a hostile module can list anything, so the section binds an honest toolchain only). No loop runs past the section's
  bytes (a huge count throws at the first missing entry).
- C. FX-4: constructor.constructor, __proto__.valueOf, constructor.keys, env.now, "".""(function), env.memory,
  env.table, env.g, each beside a clean target_features section: exactly 1 problem, the import by module, name and
  kind; instantiate refused. Round 1's sec-probe-imports.mjs: all four refused by name (and for no section).
- D. FX-5: begin as a global; memory as a function, a global, a table; step as a table: each "exports X as a K, not
  a L"; extra exports __proto__, constructor, hasOwnProperty: "not on the loader's list" (Object.hasOwn).
  (My probe's first run read `kinds["__proto__"]` as Object.prototype, FX-4's own bug class: fixed in the probe.)

## Findings (a line after each)
- SEC-U4D2-R2-1 Low (not before merge; text): two new sentences give the loader's section read a reach it lacks.
  check.mjs:1996-2000 (DEPS_CHECKED's limits): "a flag reaching rustc another way ... is the Wasm leg's loader's to
  refuse from the module itself"; loader.mjs:13-14: "a flag reaching rustc by a route check.mjs does not read is
  refused too". A RUSTC_WRAPPER (allowed) or a wasm32 `linker` (no rule reads it: r2-sec-fx1-config.log) can pass
  `--no-check-features --features=...` to wasm-ld, and the module then carries relaxed SIMD with a section that lists
  none (sec_probe_tf2_ld.wasm, r2-sec-loader.log part A); and on x86-64 nothing reads the artifact, so a wrapper's
  +fma is unseen. Fix (words): "a flag that enables simd128 or relaxed-simd at compile time shows in the section, which
  the loader refuses; the section is wasm-ld's record, so a linker or link arguments from a RUSTC_WRAPPER or a
  `linker` key (neither read) can write it, and the x86-64 legs have no artifact check". Optional rule: refuse
  `linker` keys and CARGO_TARGET_*_LINKER in cargoConfigProblems.
- SEC-U4D2-R2-2 Low (not before merge; follow-up): plan §4.5's "overflow-checks = true for npt-sim and npt-proto in
  every profile" (Cargo.toml:115-119) is overridden by a cargo config's `[profile.release.package.npt-sim]
  overflow-checks = false` with no rustflag: deps PASS (r2-sec-fx1-config.log) and cargo drops `-C overflow-checks=on`
  (r2-sec-routes.log). Block 3 said FX-1 also closes that rule against a rustflag; true, but no rule reads a config's
  [profile] table or CARGO_PROFILE_* variables. Both native and Wasm wrap alike, so no divergence; the safety rule
  goes off silently. Fix: cargoConfigProblems refuses a `profile` table in a config and CARGO_PROFILE_* in the
  environment (the spike's configs and runs use none).
