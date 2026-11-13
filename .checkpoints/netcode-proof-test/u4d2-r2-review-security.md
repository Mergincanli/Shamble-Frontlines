# U4d2 review round 2: security-auditor (read only; on what changed: FX-1 to FX-12)

Seat: the advisor's U4d2 round-1 ruling, Block 8, batch 1, security-auditor. Copy run in:
/home/user/Shamble-Frontlines/.claude/worktrees/u4d2-build (only through u4d2-run.sh; never edited). Read copy:
/home/user/Shamble-Frontlines/.gate/u4d2-r2. Round 1's tree: /home/user/Shamble-Frontlines/.gate/u4d2-r1. Logs:
/home/user/Shamble-Frontlines/.gate/u4d2/r2-sec-*.log. Probe scripts (text): this notes folder, sec-r2-*.mjs, sec-r2-*.rs
and sec-r2-home/<case>/config.toml, each importing the copy's check.mjs or loader.mjs and feeding it in-memory inputs
or those probe files (no edit of any copy). Built probe modules: .gate/u4d2/sec-probe/. This agent has no Edit tool:
the notes are rewritten whole with Write each time.

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

### Block 5: FX-3 and FX-6 in memory, and one near form built (done; r2-sec-scan.log, r2-sec-scan-esc.log)
- sec-r2-scan.mjs (shared lock; the copy's unsafeProblems on crateFiles' real list, one file swapped or added in
  memory): the real files 0 problems. FX-3: round 1's V1 (an `unsafe` block in begin's body) refused at
  lib.rs:101, its line; the same in dump_word's body refused at :203. FX-6: V4 whole (build.rs + `use std::include as
  inc;` + `inc!(concat!(env!("OUT_DIR"), "/gen.rs"))`) 2 problems (build.rs; lib.rs:1's `include`); each half alone
  refused by its own line; near forms refused: `use std::{include as inc};`, `use core::include as inc;` in
  npt-tcpinfo, `m!(include)` (the bare name as a macro argument), `std::include!(..)`, `r#include!(..)`, a raw-string
  include!, a quoted `"build"` key, an inline `build` key in npt-tcpinfo's manifest.
- LET THROUGH (0 problems): `m!(include!)` in begin's body, where `macro_rules! m { ($($t:tt)*) => { $($t)*
  ("../../../gen.rs") }; }` adds the path; the same with a space (`include !`); the same as a crate-level item in
  npt-tcpinfo. FX-6's lookahead `(?!\s*!)` (check.mjs:683) exempts `include` before any `!`, and pathProblems' rule
  (check.mjs:413) reads `include!` only before a bracket, so the path literal is read by neither.
- Built (r2-sec-scan.log, end): sec-r2-incl.rs, npt-hash-wasm's shape (`#![deny(unsafe_code)]`, the placed allow and
  `#[unsafe(no_mangle)]` on `begin`), with `m!(include!)` in begin's body naming sec-r2-incl-gen.txt, whose text is an
  `unsafe { core::ptr::read_volatile(&7u32) }` block: rustc for wasm32 exit 0, and begin(1, 3) = 9 (the included block
  ran). crateFiles lists only .rs and Cargo.toml (check.mjs:730), so a committed .txt file the macro names is read by
  no rule at all (crates/hash-wasm already holds a .txt, co-initial-dump.txt).
- C2R-2's escaped literal (sec-r2-scan-esc.mjs): `include!(".\x2e/.\x2e/.\x2e/gen.rs")` and the same #[path] pass
  (0 problems), as pathProblems' doc states (its known limit, "a literal with escapes there is review's",
  check.mjs:380-385); the literal must still end in ".rs" as written, so the file is a .rs outside crates/. (Two cases
  of the first probe that start with a backslash are refused as absolute: the probe's expectation was wrong, not the
  rule.)

### Block 6: check.test.mjs through the runner (done; r2-sec-selftest.log, -x, exit 0)
- node's summary: tests 71, pass 71, fail 0 (round 1's build 63; the fix pass reports 71). The FX-2, FX-4 and FX-5
  self-tests (packet-tests :279-334) assert deepEqual on the whole problem list, and FX-4's modules carry the helper's
  default clean section, so FX-2 cannot hide FX-4's refusal and no extra line could pass unseen. FX-6's self-test
  (:150-189) holds V4's two forms and no `m!(include!)` case (SEC-U4D2-R2-3).

### Block 7: the second near form, SEC-U4D2-3's limit, bounds (done; r2-sec-pathmeta.log, r2-sec-integers.log)
- r2-sec-pathmeta.log (shared lock; exit 0): a #[path] built from a `meta` fragment, `macro_rules! m { ($a:meta) =>
  { #[$a] mod g; }; }` and `m!(path = "x.txt");` in begin's body: 0 problems (pathProblems reads `path =` only inside
  an attribute's own text, check.mjs:402-411; here the attribute is `#[$a]`); the same at the crate level with
  "../../../gen.rs": 0; control, a plain `#[path = "x.txt"] mod g;` in begin's body: refused at lib.rs:101. Built:
  sec-r2-pathmeta.rs (deny at the crate, the placed allow on `begin`, the module declared inside its body from
  sec-r2-pathmeta-mod.txt, whose fn holds an unsafe block): rustc wasm32 exit 0 (00:58:38), no imports, begin(1, 3) = 9.
  A module declared in a fn body sits under that fn's lint levels, so the export's allow covers it.
- Neither deny-level crate defines a macro today (grep of crates/hash-wasm and crates/tcpinfo .rs: no macro_rules!),
  so "no macro_rules! in a deny-level exception crate" would close both near forms with one rule.
- r2-sec-integers.log (exit 0): SEC-U4D2-3's comment (check.mjs:217-224) is exact: `type u32 = f64;` and `use
  core::primitive::f64 as u32;` give 0 problems; control `feed(word: f64)` refused at lib.rs:118. r1-sec-rustc.log:12
  holds "feed: function (f64) -> (f64)", which the comment cites. moduleProblems (loader.mjs:97-123) reads names, kinds,
  imports and target_features, no type section: the comment's "export types are not read" holds.
- Bounds (security side): since round 1's tree, changed only Cargo.toml (the libm comment, 4 lines), check.mjs,
  check.test.mjs, crates/lab/src/lib.rs, crates/lab/src/hash.rs, crates/hash-wasm/loader.mjs and the new
  crates/hash-wasm/clippy.toml; hash-wasm/src/lib.rs and Cargo.lock equal round 1's (cmp). Loader exports unchanged;
  check.mjs adds `buildInputProblems` and `FAILED_STEP_DIR` and main's three optional parameters. FX-10's file:
  `<ROOT>/.gate/netcode-proof-test/failed-steps/<step>.log`, the step name checked against STEPS first (an
  Object.prototype name such as `--only constructor` throws in main and exits 2, never a pass: note, no finding).

### Block 8: T3 and T5 against what stays open (done)
- C-U7-2 (T3) carries SEC-U4D2-3's open half whole: moduleProblems reads each export's type from the type section and
  refuses any but i32, on the wasm-opt output, with the target_features section kept or its replacement stated. Its
  parenthesis "(U4d2's source scan reads the token `u32` only)" and T5's "reads the token `u32`, not the type" are
  close but not exact: EXPORT_INTEGERS is `u32` and `i32` (check.mjs:225). Wording only (SEC-U4D2-R2-4).
- T5's last bullet ("Open security finding: SEC-U4D2-3 (Low) ... Every other security finding of the review is fixed
  in this change") reads true for round 1 only; round 2's open security Lows (below) need naming there or a ruling
  that defers them, since advisor.md's PR review step 4 keeps a PR needs-mustafa for any open security finding.

## Findings (a line after each)
- SEC-U4D2-R2-1 Low (not before merge; text): two new sentences give the loader's section read a reach it lacks.
  check.mjs:1998-2002 (DEPS_CHECKED's limits): "a flag reaching rustc another way ... is the Wasm leg's loader's to
  refuse from the module itself"; loader.mjs:13-14: "a flag reaching rustc by a route check.mjs does not read is
  refused too". A RUSTC_WRAPPER (allowed: check.mjs:1400-1406) can pass `-C link-arg=--no-check-features -C
  link-arg=--features=...` with the flag, and the module then carries relaxed SIMD with a section that lists none
  (built: sec_probe_tf2_ld.wasm, r2-sec-loader.log part A); a wasm32 `linker` key (read by no rule:
  r2-sec-fx1-config.log) can do the same to the link step (by reasoning, not built). On x86-64 nothing reads the
  artifact, so a wrapper's +fma is unseen. Fix (words): "a flag that enables simd128 or relaxed-simd at compile time
  shows in the section, which the loader refuses; the section is wasm-ld's record, so link arguments from a
  RUSTC_WRAPPER or a `linker` key (neither read) can rewrite it, and the x86-64 legs have no artifact check". Optional
  rule: refuse `linker` keys and CARGO_TARGET_*_LINKER in cargoConfigProblems / buildInputProblems.
- SEC-U4D2-R2-2 Low (not before merge; follow-up): plan §4.5's "overflow-checks = true for npt-sim and npt-proto in
  every profile" (Cargo.toml:115-119) is overridden by a cargo config's `[profile.release.package.npt-sim]
  overflow-checks = false` with no rustflag: deps PASS (r2-sec-fx1-config.log) and cargo drops `-C overflow-checks=on`
  (r2-sec-routes.log). FX-1 closes that rule against a rustflag (a config's [profile.release] rustflags is refused,
  Block 2), but no rule reads a config's [profile] keys beyond rustflags (the CARGO_PROFILE_*_OVERFLOW_CHECKS
  variables not probed). Native and Wasm wrap alike, so no divergence; the safety rule goes off silently. Fix:
  cargoConfigProblems refuses any `profile` key in a cargo config (the spike's configs use none).
- SEC-U4D2-R2-3 Low (not before merge; follow-up or the next change to the scan): two macro forms bring code from an
  unread file under a listed export's allow in a deny-level crate. (a) `include!` handed to a macro as tokens:
  check.mjs:683's `/\binclude\b(?!\s*!)/` exempts `include` before any `!`, and check.mjs:413 reads include! only
  before a bracket, so `m!(include!)` with a macro_rules that appends `("<path>")` names a path neither rule reads
  (r2-sec-scan.log). (b) a #[path] from a `meta` fragment, `#[$a] mod g;` with `m!(path = "x.txt")` in the body
  (r2-sec-pathmeta.log). Both built in npt-hash-wasm's shape: rustc wasm32 exit 0, the file's unsafe block runs
  (begin(1, 3) = 9), and the file may be a .txt crateFiles never lists. Exploit sketch: a PR adds the macro and a
  src/x.txt with an unsafe block; deps PASSes. Same class and reach as SEC-U4D2-4 (a deliberate committed change past
  review; check.mjs:1990-1997 already names review as the wall inside the deny-level crates), so Low. Fix: refuse
  `macro_rules!` in the deny-level exception crates (neither defines one), or at least the lookahead
  `(?!\s*!\s*[([{])` at :683 and an attribute whose text is a metavariable (`#[$`); one self-test case each.
- SEC-U4D2-R2-4 Low (not before merge; text, the advisor's texts): T3's "(U4d2's source scan reads the token `u32`
  only)" and T5's "reads the token `u32`, not the type": the rule reads the tokens `u32` and `i32`
  (EXPORT_INTEGERS, check.mjs:225). Fix: "reads the type's token (`u32` or `i32`), not the type".

## Coverage (route into a build or the Wasm leg -> where U4d2 refuses it -> gap)
| Route | Refused where | Gap |
|---|---|---|
| a flag in any rustflags source (env, config, profile rustflags) | buildInputProblems (FX-1), by source and value | none found (163 cases, 9 live) |
| a set build target (CARGO_BUILD_TARGET, build.target) | buildInputProblems (FX-1) | none found |
| a negation in a source cargo skips (RR-M2) | FX-1: both sources named | closed |
| a flag through RUSTC_WRAPPER or a `linker` key | the loader's section read (FX-2), Wasm only | link args rewrite the section; x86-64 unread (R2-1) |
| a source-level `#[target_feature]` (SEC-U4D2-7) | FX-2: section lists it | the same link-arg gap (R2-1) |
| a config [profile.*.package] overflow-checks | none | R2-2 |
| a module import (Object.prototype names, a clock) | moduleProblems (FX-4), by name, before instantiate | closed |
| an export of another kind | moduleProblems (FX-5) | closed |
| an export typed f64 by alias | none (review; C-U7-2 at U7) | SEC-U4D2-3, carried |
| `unsafe` in a listed export's body | hashWasmProblems (FX-3), at its line | closed |
| a build script / renamed include! in a deny-level crate | unsafeProblems + layoutProblems (FX-6) | closed |
| include! as macro tokens; #[path] from a meta fragment | none | R2-3 |
| an escaped path literal | none (documented, review's) | C2R-2's known limit |

## Verdict
APPROVE WITH FINDINGS: every FX in this seat meets its ruled form and proof (FX-1, FX-2, FX-3, FX-4, FX-5, FX-6,
SEC-U4D2-3's comment); four Lows, none before the merge, none a push on its own. Open security findings after round 2:
SEC-U4D2-3 (carried, C-U7-2) and SEC-U4D2-R2-1 to R2-4 (follow-ups unless the advisor rules otherwise). The
test-engineer can settle R2-3 live by one planted run (`m!(include!)` plus a src/x.txt with an unsafe block in the
copy's npt-hash-wasm, `check.mjs --only deps` with -x: PASS expected while the gap stands).
