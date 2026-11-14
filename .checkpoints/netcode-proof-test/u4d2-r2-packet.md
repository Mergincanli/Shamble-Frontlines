# Review packet: U4d2 round 2 (on what changed: the fix pass FX-1 to FX-12 after the advisor's round-1 ruling)

_Cut by `review_packet.py` from `u4d2-r2-packet.json`. It holds sources only: code diffs, the approved plan's lines, the advisor's rulings, the last round's findings and Mustafa's words, each copied by line range. It holds no conclusion of the builder or the main session (no builder notes, brief, checkpoint card or summary). You stay free to read any file and run any test._

## Heads

- Base: round 1's reviewed tree, the read copy `.gate/u4d2-r1` (51520b4 with round 1's snapshot patch applied: wip/u4d2.patch at e2f2a5b, sha256 a526ef8e…; `git show e2f2a5b:.checkpoints/netcode-proof-test/wip/u4d2.patch`).
- U4d2's copy after the fix pass: `.claude/worktrees/u4d2-build` (branch u4d2-build at 51520b4; uncommitted). The same change as a patch on 51520b4: `.checkpoints/netcode-proof-test/wip/u4d2.patch` (committed at cb27931; 18 files; sha256 9e71cda0…; `git apply --check` exits 0 on 51520b4 and on the spike head, whose spikes/ is 1b4a3c8's). Its 18 files' sha256: `.gate/u4d2/fx-hashes-end.sha256` (`sha256sum -c` in the copy: 18 of 18 OK).
- The diffs below compare round 1's tree with the fixed copy. Files the fix pass did not change show no diff: `cmp` finds Cargo.lock, every crate's Cargo.toml, crates/hash-wasm/src/lib.rs, recordings.json, std-sin-scan.txt and co-initial-dump.txt byte-equal to round 1's. The fix pass's runs are under `.gate/u4d2/` as fx-*.log (fx-check1.log and fx-check2.log: its two counted check.mjs runs).
- The spike branch since round 1: A13 at 1b4a3c8 (crates/server/clippy.toml gains the HashMap and HashSet type bans; CI's gate runs 43330155579 and 43330163225 green). U4e merged at c88eb31 before round 1.

## What changed (the diffs)

### The workspace manifest (Cargo.toml)
_`git diff --no-index .gate/u4d2-r1/spikes/netcode-proof/Cargo.toml .claude/worktrees/u4d2-build/spikes/netcode-proof/Cargo.toml`:  1 file changed, 4 insertions(+), 4 deletions(-)._

````diff
diff --git a/.gate/u4d2-r1/spikes/netcode-proof/Cargo.toml b/.claude/worktrees/u4d2-build/spikes/netcode-proof/Cargo.toml
index 858efb7..6d29813 100644
--- a/.gate/u4d2-r1/spikes/netcode-proof/Cargo.toml
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/Cargo.toml
@@ -50,10 +50,10 @@ rand_core = { version = "=0.10.1", default-features = false }
 sha2 = "=0.11.1"
 # U4s: npt-sim's one source of transcendentals (plan §4.5: ADR 0004's float allow-list, "transcendentals through libm
 # only"), pure Rust; already in Cargo.lock through quinn-proto's fastbloom, so no new package. Default features kept
-# (`arch`: sqrt and sqrtf by x86-64's `sqrtsd`, and fma and fmaf with run-time CPU dispatch, on x86-64; nothing on
-# wasm32 stable; all correctly rounded: libm 0.2.16's configure.rs:82-101 and arch/x86/fma.rs:9-27, the rust review),
-# the set fastbloom already turns on in the server build, so npt-sim's libm resolves one feature set on native and
-# wasm32 (plan §4.5 "Features"; check.mjs's deps step compares the sets).
+# (`arch`: sqrtf by x86-64's `sqrtss` and sqrt by its `sqrtsd`, and fma and fmaf with run-time CPU dispatch, on x86-64;
+# nothing on wasm32 stable; all correctly rounded: libm 0.2.16's configure.rs:82-101, arch/x86.rs:13 and :26 and
+# arch/x86/fma.rs:9-27, the rust review), the set fastbloom already turns on in the server build, so npt-sim's libm
+# resolves one feature set on native and wasm32 (plan §4.5 "Features"; check.mjs's deps step compares the sets).
 libm = "=0.2.16"
 # U3: the transports and L2 (plan §4.1's npt-net; PM-0001-35's floors: quinn-proto >= 0.11.18, rustls >= 0.23.46,
 # ring >= 0.17.12, tokio >= 1.54.1; U0 resolved these versions).
````

### check.mjs
_`git diff --no-index .gate/u4d2-r1/spikes/netcode-proof/check.mjs .claude/worktrees/u4d2-build/spikes/netcode-proof/check.mjs`:  1 file changed, 145 insertions(+), 23 deletions(-)._

````diff
diff --git a/.gate/u4d2-r1/spikes/netcode-proof/check.mjs b/.claude/worktrees/u4d2-build/spikes/netcode-proof/check.mjs
index 059733d..c015186 100644
--- a/.gate/u4d2-r1/spikes/netcode-proof/check.mjs
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/check.mjs
@@ -23,7 +23,7 @@
 
 import { spawnSync } from "node:child_process";
 import { createHash } from "node:crypto";
-import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync } from "node:fs";
+import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
 import { homedir } from "node:os";
 import { dirname, join, relative, resolve, sep } from "node:path";
 import { fileURLToPath, pathToFileURL } from "node:url";
@@ -198,6 +198,15 @@ export const UNSAFE_EXCEPTIONS = {
 };
 /** The unsafe_code level each exception's own [lints.rust] gives, its other lints being the workspace's (review C2R-1). */
 const EXCEPTION_LEVELS = { "crates/client-wasm/": "allow", "crates/tcpinfo/": "deny", "crates/hash-wasm/": "deny" };
+/**
+ * Why a deny-level exception (npt-tcpinfo, npt-hash-wasm) may have no build script and may name `include` only as a
+ * macro call (the advisor's U4d2 round-1 ruling, FX-6: SEC-U4D2-4, C2R-2's known limit): code a build script writes,
+ * brought in by an `include!` the scan cannot see by its name (`use std::include as inc;`), would sit inside a listed
+ * function's allow, where rustc compiles unsafe code the scan never reads.
+ */
+const DENY_EXCEPTION_WHY =
+  "code a build script writes, brought in by a renamed include!, would sit inside a listed function's allow, where " +
+  "rustc compiles unsafe code the scan never reads (SEC-U4D2-4)";
 /** The exceptions' names as a sentence lists them: "a, b and c". */
 const exceptionNames = () => {
   const names = Object.values(UNSAFE_EXCEPTIONS);
@@ -208,6 +217,10 @@ export const HASH_WASM_UNSAFE_FILE = "crates/hash-wasm/src/lib.rs";
 /**
  * The types an export's parameters and result may have (the advisor's U4D2-Q7 ruling): rustc lowers both to Wasm's
  * i32, so every export has integer signatures only (the third exception's written invariant).
+ * Known limit (SEC-U4D2-3; the advisor's U4d2 round-1 ruling, FX-11): the scan reads the type's token, not the type.
+ * A module-local `type u32 = f64;` or `use core::primitive::f64 as u32;` passes it, and the built module's export
+ * types are not read (Wasm then types the export `(f64) -> (f64)`: r1-sec-rustc.log); review is the wall until the
+ * loader reads the type section (Q7 (b), C-U7-2).
  */
 const EXPORT_INTEGERS = ["u32", "i32"];
 /** The one attribute whose `unsafe` npt-hash-wasm may have: `#[unsafe(no_mangle)]`, sitting on a listed export. */
@@ -366,8 +379,10 @@ const lineAt = (text, offset) => text.slice(0, offset).split("\n").length;
  * given at the call).
  * Known limit (review C2R-2): the literal is read as written, its escapes not decoded (`.\x2e/` is `../` to rustc),
  * and include! is found by its name only (`use std::include as inc;` hides it). Code such a path brings into a crate
- * under the workspace's forbid is still refused by rustc if it holds unsafe code or an allow of the lint; into
- * npt-tcpinfo, only review stops it.
+ * under the workspace's forbid is still refused by rustc if it holds unsafe code or an allow of the lint. Into
+ * npt-tcpinfo or npt-hash-wasm (deny, not forbid), whose listed functions' allows would cover it, unsafeProblems
+ * refuses the two routes found (SEC-U4D2-4): a build script, whose output an include! brings in, and `include` named
+ * as a path; a literal with escapes there is review's.
  */
 function pathProblems(f, crate, code, mask, attrs) {
   const problems = [];
@@ -630,6 +645,9 @@ function inheritsWorkspaceLints(text) {
  * - in npt-hash-wasm, any relaxing but one placed allow on each listed export, any `unsafe` but a listed export's
  *   `#[unsafe(no_mangle)]`, a listed export whose parameters or result are not u32 or i32 (hashWasmProblems), and a
  *   listed export with no allow or two;
+ * - in npt-tcpinfo and npt-hash-wasm (EXCEPTION_LEVELS' deny), a build.rs at the crate's root, a `build` key in its
+ *   manifest, and `include` named other than as an include!, include_str! or include_bytes! call (the advisor's U4d2
+ *   round-1 ruling, FX-6: SEC-U4D2-4); layoutProblems refuses a custom-build target there too;
  * - in every crate, a #[path] or include! whose literal names anything but a .rs file inside the crate's folder
  *   (pathProblems, with its known limit), the way another crate could compile npt-tcpinfo's file under its own name.
  * npt-client-wasm's .rs files are not scanned for unsafe code: its manifest allows the lint for the whole crate.
@@ -659,7 +677,22 @@ export function unsafeProblems(files, read) {
       }
       if (crate === "crates/tcpinfo/") problems.push(...tcpinfoProblems(f, code, mask, attrs, sites, state));
       if (crate === "crates/hash-wasm/") problems.push(...hashWasmProblems(f, code, mask, attrs, hashSites));
+      if (EXCEPTION_LEVELS[crate] === "deny") {
+        const name = UNSAFE_EXCEPTIONS[crate];
+        if (f === `${crate}build.rs`) problems.push(`${f} is a build script in ${name}, which may have none: ${DENY_EXCEPTION_WHY}`);
+        for (const m of mask.matchAll(/\binclude\b(?!\s*!)/g)) {
+          problems.push(
+            `${f}:${lineAt(mask, m.index)} names \`include\` other than as an include!, include_str! or include_bytes! call ` +
+              `(a renamed include!, as \`use std::include as inc;\`, hides the call from the include! rule), which ${name} may not: ` +
+              DENY_EXCEPTION_WHY,
+          );
+        }
+      }
       problems.push(...pathProblems(f, crate, code, mask, attrs));
+    } else if (f === `${crate}Cargo.toml` && EXCEPTION_LEVELS[crate] === "deny") {
+      if (tomlValues(tomlScan(text), "build").length) {
+        problems.push(`${f} has a key \`build\`: a build script in ${UNSAFE_EXCEPTIONS[crate]}, which may have none: ${DENY_EXCEPTION_WHY}`);
+      }
     } else if (f === `${crate}Cargo.toml` && !(crate in UNSAFE_EXCEPTIONS)) {
       // The name is looked for outside comments, strings included (a quoted key is the lint to cargo; a string that
       // names it fails closed), in tomlScan's `clean`: a `#` inside a string is no comment there (review C2c N2's
@@ -1244,6 +1277,8 @@ export function layoutProblems(meta) {
       else if (!src.endsWith(".rs")) problems.push(`${what} compiles ${rel(src)}, not a .rs file the scan reads`);
       if (t.edition !== "2024") problems.push(`${what} is edition ${t.edition}, not 2024`);
       if (t.kind.includes("proc-macro")) problems.push(`${what} is a proc-macro crate, whose expansions the lint and the scan cannot see`);
+      const denied = Object.keys(EXCEPTION_LEVELS).some((c) => EXCEPTION_LEVELS[c] === "deny" && UNSAFE_EXCEPTIONS[c] === p.name);
+      if (denied && t.kind.includes("custom-build")) problems.push(`${what} is a build script in ${p.name}, which may have none: ${DENY_EXCEPTION_WHY}`);
     }
   }
   return problems;
@@ -1505,12 +1540,57 @@ export function buildFlagProblems(sources) {
   return problems;
 }
 
+/**
+ * Every rustflags source that holds a flag, whatever the flag, and every build target set outside the builds' own
+ * `--target` (the advisor's U4d2 round-1 ruling, FX-1: RR-M2 with TE-M1, SEC-U4D2-5, SEC-U4D2-6), as sentences naming
+ * the source and its value:
+ * - each source rustflagSources reads (every environment variable whose name ends in RUSTFLAGS, and every `rustflags`
+ *   key of the cargo configs read) with a flag in it. Cargo gives rustc the first non-empty one of
+ *   CARGO_ENCODED_RUSTFLAGS, RUSTFLAGS, the target tables and build.rustflags and drops the others
+ *   (https://doc.rust-lang.org/cargo/reference/config.html, build.rustflags), so a read-back given all of them could
+ *   take a flag's negation from a source cargo drops (`-Ctarget-feature=-simd128,-relaxed-simd` in RUSTFLAGS beside
+ *   CARGO_ENCODED_RUSTFLAGS=-Ctarget-cpu=bleeding-edge passed while the module cargo built held both: RR-M2), and a
+ *   flag no name rule knows (`--cfg feature="bites"`, `-C overflow-checks=off`, `-C llvm-args=...`) reached rustc unseen
+ *   by both (SEC-U4D2-5). The spike's builds use none, so with every source empty the read-back is given exactly what
+ *   cargo gives rustc: no flag;
+ * - CARGO_BUILD_TARGET in the environment, whatever its value, and a `target` key in a cargo config (build.target; read
+ *   in any table or form, as this file reads config keys, which fails closed): either builds npt-lab's native legs for
+ *   another target than the x86-64 one the read-back and the features comparison read (SEC-U4D2-6).
+ */
+export function buildInputProblems(env, configs) {
+  const problems = [];
+  for (const { where, flags } of rustflagSources(env, configs)) {
+    if (!flags.length) continue;
+    problems.push(
+      `${where} holds ${JSON.stringify(flags.join(" "))}: no rustflags source may hold a flag (cargo uses one source ` +
+        "and drops the rest, so the read-back could not be given what rustc gets: the advisor's U4d2 round-1 ruling, FX-1)",
+    );
+  }
+  if (typeof env.CARGO_BUILD_TARGET === "string") {
+    problems.push(
+      `CARGO_BUILD_TARGET in the environment sets the build target (${JSON.stringify(env.CARGO_BUILD_TARGET)}): the native ` +
+        "builds take cargo's default target, the x86-64 one the read-back and the features comparison read",
+    );
+  }
+  for (const { path, text } of configs) {
+    if (tomlValues(tomlScan(text), "target").length) {
+      problems.push(
+        `${path} has a key \`target\` (build.target): it sets the build target, and the native builds take cargo's ` +
+          "default target, the x86-64 one the read-back and the features comparison read",
+      );
+    }
+  }
+  return problems;
+}
+
 /**
  * rustc's read-back of the target features (the advisor's condition 7 (a): it fails the step, never only reports; the
  * advisor's U4D2-Q4 ruling): `rustc --print cfg --target <triple>` run from the spike's folder (rust-toolchain.toml
- * picks the toolchain cargo uses), given every rustflag of every source read, a superset of the one source cargo uses,
- * which fails closed (a CPU name meant for one target empties or cuts the other's list: rustc 1.99.1, the advisor's
- * U4d2 brief check, A15). Per target: the features it must not list, and the one a read-back that read anything lists
+ * picks the toolchain cargo uses), given every rustflag of every source read. The deps step refuses every source that
+ * holds a flag (buildInputProblems: the advisor's U4d2 round-1 ruling, FX-1), so whenever the step can pass the
+ * read-back is given no flag, which is exactly what cargo gives rustc; given flags, it still reads them (a CPU name
+ * meant for one target empties or cuts the other's list: rustc 1.99.1, the advisor's U4d2 brief check, A15), on a step
+ * that already fails. Per target: the features it must not list, and the one a read-back that read anything lists
  * (non-vacuity: a parse that reads nothing never passes).
  */
 export const READ_BACK = [
@@ -1764,11 +1844,15 @@ export function marksProblems(marks, readFile) {
   return problems;
 }
 
-function run(cmd, args, cwd, env = process.env) {
-  const r = spawnSync(cmd, args, { cwd, env, encoding: "utf8", maxBuffer: 1 << 28 });
+/**
+ * One command, waited for: `timeout` (ms; 0, none) stops it with SIGTERM once passed, and the result says so
+ * (`timedOut`: spawnSync's ETIMEDOUT), so a hung call is a named failure, not a lock held forever (INF-4).
+ */
+function run(cmd, args, cwd, env = process.env, timeout = 0) {
+  const r = spawnSync(cmd, args, { cwd, env, encoding: "utf8", maxBuffer: 1 << 28, ...(timeout ? { timeout } : {}) });
   // `stdout` alone is kept for the steps that parse it: cargo writes progress lines ("Blocking waiting for file
   // lock", "Downloaded …") to stderr, and they must never reach a JSON parser.
-  return { ok: r.status === 0, out: `${r.stdout ?? ""}${r.stderr ?? ""}`, stdout: r.stdout ?? "", status: r.status };
+  return { ok: r.status === 0, out: `${r.stdout ?? ""}${r.stderr ?? ""}`, stdout: r.stdout ?? "", status: r.status, timedOut: r.error?.code === "ETIMEDOUT" };
 }
 
 /**
@@ -1912,8 +1996,10 @@ export function testBothBuilds(runner = run) {
  *   only (C2R-2). The compiler covers both in the crates under the forbid; inside npt-tcpinfo and npt-hash-wasm only
  *   review does.
  * - The build-flag scan and rustc's read-back read the rustflags sources above, not a build script's
- *   `cargo::rustc-flags` or a RUSTC_WRAPPER's own flags; the read-back is given every source's flags at once (a superset
- *   of what cargo passes, which fails closed: the advisor's U4D2-Q4 ruling).
+ *   `cargo::rustc-flags` or a RUSTC_WRAPPER's own flags. No source may hold a flag (buildInputProblems: the advisor's
+ *   U4d2 round-1 ruling, FX-1), so on a step that passes the read-back is given no flag, which is what cargo gives
+ *   rustc from those sources; a flag reaching rustc another way, or a source-level `#[target_feature]`, is the Wasm
+ *   leg's loader's to refuse from the module itself (its target_features section, FX-2).
  * - Inputs that can lower lints and are not read here: a cargo config's [env] table (CLIPPY_ARGS, C2R-4 G, or any
  *   other variable it sets for rustc), what a member's build script prints (`cargo::rustc-env`, C2R-4 H), a
  *   RUSTC_WRAPPER program (allowed, for sccache) and a rust-toolchain.toml `path`. Lint capping by any of them
@@ -1943,10 +2029,12 @@ const DEPS_CHECKED =
   "npt-tcpinfo relaxes it only by the #[allow] on srtt_us and unacked_and_unsent, has unsafe code only inside " +
   "them, and keeps lib.rs's deny of the two clippy unsafe lints, and npt-hash-wasm relaxes it only by one #[allow] on " +
   "each of its seven listed exports, whose #[unsafe(no_mangle)] is its only unsafe code, each taking and giving u32 " +
-  "or i32 only; the environment and the cargo configs read pass no " +
+  "or i32 only; neither npt-tcpinfo nor npt-hash-wasm has a build script or names `include` but as a macro call; " +
+  "the environment and the cargo configs read pass no " +
   "lint-lowering rustflags and set no clippy or fmt alias, no other rustc or workspace wrapper, no config include and " +
-  "no test runner, and the environment has no RUSTC_BOOTSTRAP or cargo channel override; no rustflags source enables " +
-  "fma, simd128 or relaxed-simd or sets target-cpu=native, and rustc's read-back given every flag read lists sse2 and " +
+  "no test runner, and the environment has no RUSTC_BOOTSTRAP or cargo channel override; no rustflags source holds a " +
+  "flag (so none enables fma, simd128 or relaxed-simd or sets target-cpu=native) and neither the environment nor a " +
+  "cargo config sets the build target, and rustc's read-back, given what cargo gives rustc, lists sse2 and " +
   "no fma for x86-64 and sign-ext and no simd128 or relaxed-simd for wasm32; npt-sim's resolved features, its own " +
   "and every package's below it, are one set in the npt-server, npt-lab and npt-hash-wasm (wasm32) builds, measured " +
   "with measured and bites with bites, each holding npt-sim's own dependencies; every member lives in " +
@@ -1999,6 +2087,7 @@ export function depsStep(here = HERE, env = process.env, metadata = cargoMetadat
     ...clippyTwinProblems(read),
     ...linkUseProblems(files, read),
     ...buildFlagProblems(sources),
+    ...buildInputProblems(env, configs),
     ...readBack.problems,
     ...features.problems,
   ];
@@ -2119,6 +2208,8 @@ export function readScan(text) {
   return { catches, notCatchable, record };
 }
 
+/** The wasm step's time limit on each of its calls but its cargo builds: the step's whole 5-minute budget. */
+const WASM_CALL_LIMIT_MS = 300_000;
 /** The real file operations of the wasm step (check.test.mjs passes its own), paths under the spike for `read`. */
 const WASM_IO = {
   reset: (dir) => {
@@ -2164,10 +2255,15 @@ export function wasmStep(runner = run, io = WASM_IO, now = Date.now, dir = WASM_
   const problems = [];
   const lines = [];
   const timed = [];
+  // Every call but the four cargo builds has WASM_CALL_LIMIT_MS (the advisor's U4d2 round-1 ruling, FX-7: INF-4): a
+  // hung leg would hold the lane lock for every other lane's cargo. The builds have none: under the exclusive lock no
+  // other cargo holds theirs, and a cold build's time is INF-1's to measure, not to cut.
   const call = (label, cmd, args) => {
     const t0 = now();
-    const r = runner(cmd, args, HERE);
+    const limit = cmd === "cargo" ? 0 : WASM_CALL_LIMIT_MS;
+    const r = runner(cmd, args, HERE, undefined, limit);
     timed.push(`${label} ${((now() - t0) / 1000).toFixed(1)}`);
+    if (r.timedOut) problems.push(`${label}: still running after ${limit / 1000} s, so it was stopped (the wasm step's limit on each call but its four cargo builds)`);
     return r;
   };
   const failure = (r) => lastLines(r.out, 1).trim() || `exit ${r.status}`;
@@ -2345,7 +2441,13 @@ export function wasmStep(runner = run, io = WASM_IO, now = Date.now, dir = WASM_
       bites.stdSin = caught
         ? `caught at ${recording} tick ${tick}, ${field}, as the scan found`
         : `NOT caught at the scan's ${recording} tick ${tick}, ${field}: ${c ? `first mismatch ${c.first ? `${c.first.recording} tick ${c.first.tick}, field ${c.field ?? "none"}` : (c.at ?? "none")}` : "no comparison was written"}`;
-      if (!caught) problems.push(`std-sin: ${bites.stdSin}; this step ran on ${io.libc()}, the scan on ${scan.record.find((l) => /libc|ldd/i.test(l)) ?? "a libc it does not name"}`);
+      if (!caught) {
+        problems.push(
+          `std-sin: ${bites.stdSin}; this step ran on ${io.libc()}, the scan on ${scan.record.find((l) => /libc|ldd/i.test(l)) ?? "a libc it does not name"}; ` +
+            "glibc picks its sinf by the CPU's features when it loads (an IFUNC), so on another CPU or libc the scan is re-run " +
+            "there (u4d2-brief.md 2.4 (a) item 6; INF-3)",
+        );
+      }
       if (catches.length > 1) bites.stdSin += ` (the scan's other catches: ${catches.slice(1).map((x) => `${x.recording} tick ${x.tick}`).join(", ")})`;
     } else if (scan?.notCatchable) {
       bites.stdSin = `not catchable at this site on x86-64 and wasm32 (the scan: ${record}); no std-sin leg runs (the scratch plant stood in once: u4d2-brief.md 2.7)`;
@@ -2404,26 +2506,46 @@ function lastLines(text, n) {
   return text.trimEnd().split("\n").slice(-n).join("\n");
 }
 
-export function main(argv) {
+/**
+ * Where a failed step's whole output is kept (the advisor's U4d2 round-1 ruling, FX-10: TE-L4): `<step>.log` in the
+ * checkout's .gate/ (gitignored), one per step, removed when the step starts and written when it fails, so a panic
+ * above the 40 lines main prints (B2.3's "name and panic") survives the run.
+ */
+export const FAILED_STEP_DIR = join(ROOT, ".gate", "netcode-proof-test", "failed-steps");
+
+/**
+ * The command: each step (all, or `--only`'s) from `steps`, its PASS or FAIL line printed through `log`; a failed
+ * step's last 40 lines, then the path of the file in `keep` that holds its whole output.
+ */
+export function main(argv, steps = STEPS, log = console.log, keep = FAILED_STEP_DIR) {
   let only = null;
   for (let i = 0; i < argv.length; i++) {
     if (argv[i] === "--only" && i + 1 < argv.length) only = argv[++i].split(",");
     else throw new Error(`unknown argument ${argv[i]}`);
   }
-  const names = only ?? Object.keys(STEPS);
-  for (const n of names) if (!STEPS[n]) throw new Error(`no step ${n}; the steps: ${Object.keys(STEPS).join(", ")}`);
+  const names = only ?? Object.keys(steps);
+  for (const n of names) if (!steps[n]) throw new Error(`no step ${n}; the steps: ${Object.keys(steps).join(", ")}`);
   let failed = 0;
   for (const n of names) {
+    const file = join(keep, `${n}.log`);
+    rmSync(file, { force: true });
     const started = Date.now();
-    const r = STEPS[n]();
+    const r = steps[n]();
     const secs = ((Date.now() - started) / 1000).toFixed(1);
-    console.log(`${r.ok ? "PASS" : "FAIL"}  ${n.padEnd(13)} ${secs} s  ${lastLines(r.out, 1)}`);
+    log(`${r.ok ? "PASS" : "FAIL"}  ${n.padEnd(13)} ${secs} s  ${lastLines(r.out, 1)}`);
     if (!r.ok) {
       failed++;
-      console.log(lastLines(r.out, 40).replace(/^/gm, "      "));
+      log(lastLines(r.out, 40).replace(/^/gm, "      "));
+      try {
+        mkdirSync(keep, { recursive: true });
+        writeFileSync(file, r.out.endsWith("\n") ? r.out : `${r.out}\n`);
+        log(`      the step's whole output (${r.out.split("\n").length} lines): ${file}`);
+      } catch (e) {
+        log(`      the step's whole output could not be kept in ${file}: ${e.message}`);
+      }
     }
   }
-  console.log(failed ? `check: ${failed} step(s) failed` : `check: all ${names.length} steps pass`);
+  log(failed ? `check: ${failed} step(s) failed` : `check: all ${names.length} steps pass`);
   return failed ? 1 : 0;
 }
 
````

### npt-hash-wasm (crates/hash-wasm)
_`git diff --no-index .gate/u4d2-r1/spikes/netcode-proof/crates/hash-wasm .claude/worktrees/u4d2-build/spikes/netcode-proof/crates/hash-wasm`:  2 files changed, 90 insertions(+), 10 deletions(-)._

````diff
diff --git a/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/hash-wasm/clippy.toml b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/hash-wasm/clippy.toml
new file mode 100644
index 0000000..c64bb4a
--- /dev/null
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/hash-wasm/clippy.toml
@@ -0,0 +1,16 @@
+# npt-hash-wasm's clock and hash-map bans: it is a leaf crate (check.mjs's LEAF; u4-build-plan.md §5 U4d2), and plan.md
+# §4.2's leaf rule says "Clock reads are banned in them by clippy (§4.5)" (the advisor's U4d2 round-1 ruling, FX-12:
+# RR-L2, EA-5). The form is npt-lab's and npt-room's: the two type bans are npt-sim's, the clock ban npt-room's; no
+# tokio lines, as the crate has no tokio. It hashes npt-sim's state, which takes no order from a hash map and reads no
+# clock, and the Node loader gives it nothing to import. The workspace sets disallowed_types and disallowed_methods to
+# deny, and the crate's own lint tables keep both.
+disallowed-types = [
+    { path = "std::collections::HashMap", reason = "hash-map ban (GLOSSARY §7): iteration order is not deterministic" },
+    { path = "std::collections::HashSet", reason = "hash-map ban (GLOSSARY §7): iteration order is not deterministic" },
+    { path = "std::time::Instant", reason = "the clock ban: time is passed in" },
+    { path = "std::time::SystemTime", reason = "the clock ban: time is passed in" },
+]
+disallowed-methods = [
+    { path = "std::time::Instant::now", reason = "the clock ban" },
+    { path = "std::time::SystemTime::now", reason = "the clock ban" },
+]
diff --git a/.gate/u4d2-r1/spikes/netcode-proof/crates/hash-wasm/loader.mjs b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/hash-wasm/loader.mjs
index ac07a10..972fd86 100644
--- a/.gate/u4d2-r1/spikes/netcode-proof/crates/hash-wasm/loader.mjs
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/hash-wasm/loader.mjs
@@ -4,10 +4,15 @@
 // step to compare with the native legs (npt-lab's `hash compare`, HashCompare).
 //   node crates/hash-wasm/loader.mjs --wasm <file.wasm> --rec <file.rec> --name <recording> --first-tick <tick>
 //        --out <lines file> [--bite <index> --arm <tick>] [--dump-at <tick|end> --dump-out <dump file>]
-// - It lists the module's exports with WebAssembly.Module.exports and refuses, before instantiating, any name not on
-//   LOADER_EXPORTS (or of another kind), naming it, and any listed name the module lacks (the spike's third unsafe
-//   exception's test: the advisor's condition 7 (b)). It instantiates with an empty import object, so a module that
-//   imports anything fails (no host clock or random source can reach the Wasm leg).
+// - Before instantiating, it refuses the module (moduleProblems), naming each reason: an export whose name is not on
+//   LOADER_EXPORTS or whose kind is not the listed one, a listed name the module lacks (the spike's third unsafe
+//   exception's test: the advisor's condition 7 (b)); every import, by module and name (WebAssembly.Module.imports),
+//   so no host clock or random source can reach the Wasm leg (the advisor's U4d2 round-1 ruling, FX-4: an import
+//   object `{}` would have given a module importing `constructor.constructor` JS's Function); and a target_features
+//   section that lists simd128 or relaxed-simd as used or required, or none, or more than one, or one that does not
+//   parse (FX-2: plan.md §4.5's "simd128 and relaxed-simd off", read from the module itself, so a source-level
+//   `#[target_feature]` or a flag reaching rustc by a route check.mjs does not read is refused too). It instantiates
+//   with `Object.create(null)`, an import object with nothing in it, not even Object.prototype's names.
 // - It streams the recording in: the file's bytes as little-endian 32-bit words, the last zero-padded, fed in chunks
 //   of 64 KiB with `step` called until NEED_MORE between them, never the whole file before the first step.
 // - Per tick it writes `h <recording> <tick> <hash>` (the hash as 16 hex digits, from hash_hi and hash_lo), the ticks
@@ -41,9 +46,53 @@ export const STEP = { NEED_MORE: 0, STEPPED: 1, DONE: 2, FAILED: 3 };
 /** The bytes read from the recording between steps. */
 const BLOCK_BYTES = 1 << 16;
 
+/** The target features plan.md §4.5 leaves off on wasm32, refused in a module's target_features section as `+` or `=`. */
+const REFUSED_FEATURES = ["simd128", "relaxed-simd"];
+
+/**
+ * A target_features custom section's body read (the WebAssembly tool conventions' Linking.md, "Target Features
+ * Section", which wasm-ld writes: a LEB128 count, then per entry a prefix byte, `+` used, `-` disallowed or `=`
+ * required, and the feature's name as a LEB128 length and UTF-8 bytes): { features: [{ prefix, name }] }, or
+ * { error } naming why it does not parse (a length past the end, a prefix of another byte, a name that is no UTF-8, or
+ * bytes after the last entry).
+ */
+function targetFeatures(bytes) {
+  let at = 0;
+  const leb = (what) => {
+    let n = 0;
+    for (let shift = 0; shift < 35; shift += 7) {
+      if (at >= bytes.length) throw new Error(`it ends inside ${what}`);
+      const b = bytes[at++];
+      n += (b & 0x7f) * 2 ** shift;
+      if (!(b & 0x80)) return n;
+    }
+    throw new Error(`${what} runs past 5 bytes`);
+  };
+  try {
+    const count = leb("its entry count");
+    const features = [];
+    for (let k = 0; k < count; k++) {
+      if (at >= bytes.length) throw new Error(`it ends before entry ${k + 1} of ${count}`);
+      const prefix = String.fromCharCode(bytes[at++]);
+      if (!["+", "-", "="].includes(prefix)) throw new Error(`entry ${k + 1}'s prefix is byte ${prefix.charCodeAt(0)}, not +, - or =`);
+      const length = leb(`entry ${k + 1}'s name length`);
+      if (at + length > bytes.length) throw new Error(`entry ${k + 1}'s name runs past its end`);
+      const name = new TextDecoder("utf-8", { fatal: true }).decode(bytes.subarray(at, at + length));
+      at += length;
+      features.push({ prefix, name });
+    }
+    if (at !== bytes.length) throw new Error(`${bytes.length - at} byte(s) follow its last entry`);
+    return { features };
+  } catch (e) {
+    return { error: e.message };
+  }
+}
+
 /**
  * The loader's refusal of a compiled module, as sentences (empty when it may be instantiated): every export whose name
- * is not on LOADER_EXPORTS, or whose kind is not the listed one, by name, and every listed name it lacks.
+ * is not on LOADER_EXPORTS, or whose kind is not the listed one, by name, and every listed name it lacks; every import,
+ * by module and name; and its target_features section (targetFeatures) when it lists simd128 or relaxed-simd with `+`
+ * or `=`, or when the module has none, or more than one, or one that does not parse.
  */
 export function moduleProblems(module) {
   const problems = [];
@@ -55,6 +104,21 @@ export function moduleProblems(module) {
   for (const name of Object.keys(LOADER_EXPORTS)) {
     if (!exports.some((e) => e.name === name)) problems.push(`the module does not export ${JSON.stringify(name)}`);
   }
+  for (const { module: from, name, kind } of WebAssembly.Module.imports(module)) {
+    problems.push(`the module imports ${JSON.stringify(`${from}.${name}`)} (a ${kind}): the loader gives a module nothing to import`);
+  }
+  const sections = WebAssembly.Module.customSections(module, "target_features");
+  if (sections.length !== 1) {
+    problems.push(`the module has ${sections.length} target_features sections, not one, so its target features cannot be read (wasm-ld writes one)`);
+  } else {
+    const read = targetFeatures(new Uint8Array(sections[0]));
+    if (read.error) problems.push(`the module's target_features section does not parse: ${read.error}`);
+    for (const { prefix, name } of read.features ?? []) {
+      if (REFUSED_FEATURES.includes(name) && prefix !== "-") {
+        problems.push(`the module's target_features section lists ${prefix}${name}, which plan.md §4.5 leaves off (simd128 and relaxed-simd)`);
+      }
+    }
+  }
   return problems;
 }
 
@@ -70,17 +134,17 @@ export function exportProblems(bytes) {
 }
 
 /**
- * A module the loader accepts, instantiated with an empty import object: its exports. A module that imports anything
- * fails to instantiate (a LinkError or TypeError, rethrown naming the imports).
+ * A module the loader accepts (moduleProblems: no import among the rest, refused by name before this point), instantiated
+ * with `Object.create(null)`, an import object with no names at all: its exports. A module that still fails to
+ * instantiate is refused with the engine's message.
  */
 export function instantiate(module) {
   const problems = moduleProblems(module);
   if (problems.length) throw new Error(`refused: ${problems.join("; ")}`);
   try {
-    return new WebAssembly.Instance(module, {}).exports;
+    return new WebAssembly.Instance(module, Object.create(null)).exports;
   } catch (e) {
-    const imports = WebAssembly.Module.imports(module).map((i) => `${i.module}.${i.name} (${i.kind})`);
-    throw new Error(`refused: the module does not instantiate with no imports (it imports ${imports.join(", ") || "nothing"}): ${e.message}`);
+    throw new Error(`refused: the module does not instantiate: ${e.message}`);
   }
 }
 
````

### npt-lab (crates/lab)
_`git diff --no-index .gate/u4d2-r1/spikes/netcode-proof/crates/lab .claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab`:  2 files changed, 93 insertions(+), 21 deletions(-)._

````diff
diff --git a/.gate/u4d2-r1/spikes/netcode-proof/crates/lab/src/hash.rs b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab/src/hash.rs
index 36d761a..6bb355d 100644
--- a/.gate/u4d2-r1/spikes/netcode-proof/crates/lab/src/hash.rs
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab/src/hash.rs
@@ -26,10 +26,11 @@
 //! - `compare` compares two lists by [`HashCompare`], each read from its lines files (`--ours` and `--theirs` each
 //!   name one file or several, comma-separated, read in order: the loader writes one per recording), each recording's
 //!   tick count checked against the manifest. At the first mismatch it replays that recording natively to that tick in
-//!   this process (with the bite, if given)
-//!   and names the first differing field: against its own dump by `StateDump::first_difference` when `--ours-dump` is
-//!   not given, or between the two dump files given, word by word, named by its own dump's `StateDump::field`.
-//!   `--digest-field` prints each given dump's words under that field (the hash-map bite's folded order).
+//!   this process (with the bite, if given, armed as the legs arm it: on `--on`'s recording, by default the first the
+//!   lists hold) and names the first differing field: against its own dump by `StateDump::first_difference` when
+//!   `--ours-dump` is not given, or between the two dump files given, word by word, named by its own dump's
+//!   `StateDump::field`. `--digest-field` prints each given dump's words under that field (the hash-map bite's folded
+//!   order), or "layouts differ (<n> and <m> words)" for a dump whose length is not this build's own.
 //! - `vectors` compares each scene's initial dump and hash on wasm32 (the loader's `vector-<scene>.dump` files in
 //!   `--dumps`) with this build's, and CO's with the committed vector file (h2 (ii)).
 //! - `bites` prints `World::BITES` in order, numbered from 1 as npt-hash-wasm's `begin` takes them (bites builds only).
@@ -422,6 +423,12 @@ fn bitten_name(arm: Option<&Arm>, replayed: &[&Entry]) -> Result<Option<String>,
     }
 }
 
+/// The bite a replay of `e` runs with: `arm`'s when `e` is the recording it is armed on (`on`, bitten_name's), else
+/// none. The legs and `compare` arm it alike, one default (the engine review's EA-9).
+fn arm_for<'a>(arm: Option<&'a Arm>, on: Option<&str>, e: &Entry) -> Option<&'a Arm> {
+    arm.filter(|_| on == Some(e.name.as_str()))
+}
+
 /// A line of a leg's (or a Wasm leg's) per-tick list: `h <recording> <tick> <hash>`.
 pub(crate) fn hash_line(h: &TickHash) -> String {
     format!("h {} {} {:016x}", h.recording, h.tick, h.hash)
@@ -881,10 +888,7 @@ fn serve(mode: &Mode) -> Result<(), Failure> {
         let bytes = std::fs::read(&path)
             .map_err(|err| Failure::Failed(format!("{}: {err}", path.display())))?;
         let words = le_words(&bytes);
-        let mut rp = replayer(
-            arm.as_ref()
-                .filter(|_| on.as_deref() == Some(e.name.as_str())),
-        )?;
+        let mut rp = replayer(arm_for(arm.as_ref(), on.as_deref(), e))?;
         let mut digest_written = false;
         let mut chunks = words.chunks(CHUNK_WORDS);
         loop {
@@ -951,13 +955,13 @@ fn serve(mode: &Mode) -> Result<(), Failure> {
     out.flush().map_err(io)
 }
 
-/// This build's own replay of `entry` to `tick`, its dump then (with the bite, armed as given, when it is `--on`'s).
+/// This build's own replay of `entry` to `tick`, its dump then, with `arm`'s bite armed when given (its caller passes
+/// arm_for's, so only on the recording the bite is armed on).
 fn replay_to(dir: &str, e: &Entry, tick: u32, arm: Option<&Arm>) -> Result<StateDump, Failure> {
     let path = rec_path(dir, &e.name);
     let bytes = std::fs::read(&path)
         .map_err(|err| Failure::Failed(format!("{}: {err}", path.display())))?;
-    let armed = arm.filter(|a| a.on.as_deref().is_none_or(|n| n == e.name));
-    let mut rp = replayer(armed)?;
+    let mut rp = replayer(arm)?;
     for w in le_words(&bytes) {
         rp.feed(w);
     }
@@ -981,6 +985,22 @@ fn read_dump(path: &str) -> Result<DumpFile, Failure> {
     parse_dump(&read_text(path)?, path).map_err(Failure::Failed)
 }
 
+/// What `--digest-field` prints for one dump's `words`: the words at `at` (the field's indexes in this build's own
+/// dump, `own_len` words long), as one u64 when there are two (the low word first, as npt-sim's hash writes it), or
+/// "layouts differ (<n> and <m> words)" when the dump's length is not this build's own: its words at those indexes
+/// would be other fields' (the rust review's RR-L3: P6's word-size plant).
+fn digest_text(own_len: usize, at: &[usize], words: &[u32]) -> String {
+    if words.len() != own_len {
+        return format!("layouts differ ({own_len} and {} words)", words.len());
+    }
+    let picked: Vec<u32> = at.iter().filter_map(|&i| words.get(i).copied()).collect();
+    match picked.as_slice() {
+        [lo, hi] => format!("{:016x}", u64::from(*hi) << 32 | u64::from(*lo)),
+        [] => "none".to_owned(),
+        other => hex_words(other),
+    }
+}
+
 /// `compare`: two lines files, and at a mismatch the field diff.
 fn compare(mode: &Mode) -> Result<(Vec<String>, bool), Failure> {
     let Mode::Compare {
@@ -1012,22 +1032,20 @@ fn compare(mode: &Mode) -> Result<(Vec<String>, bool), Failure> {
     let mut problems = count_problems(&names[0], &lo, &listed);
     problems.extend(count_problems(&names[1], &lt, &listed));
     let entry = |name: &str| entries.iter().find(|e| e.name == name);
+    // The bite is armed as the legs arm it: on `--on`'s recording, else the first the lists hold (EA-9).
+    let on = bitten_name(arm.as_ref(), &listed)?;
+    let armed = |e: &Entry| arm_for(arm.as_ref(), on.as_deref(), e);
     if let Some(f) = digest_field
         && let Some(d) = dumps.iter().flatten().next()
         && let (Some(t), Some(e)) = (d.tick, entry(&d.recording))
     {
-        let own = replay_to(dir, e, t, arm.as_ref())?;
+        let own = replay_to(dir, e, t, armed(e))?;
         let at: Vec<usize> = (0..own.words().len())
             .filter(|&i| own.field(i).as_deref() == Some(f.as_str()))
             .collect();
         for (k, d) in dumps.iter().enumerate() {
             let Some(d) = d else { continue };
-            let words: Vec<u32> = at.iter().filter_map(|&i| d.words.get(i).copied()).collect();
-            let digest = match words.as_slice() {
-                [lo, hi] => format!("{:016x}", u64::from(*hi) << 32 | u64::from(*lo)),
-                [] => "none".to_owned(),
-                other => hex_words(other),
-            };
+            let digest = digest_text(own.words().len(), &at, &d.words);
             lines.push(format!(
                 "{} digest {digest} ({f} at words {:?} of its dump at {} tick {t})",
                 names[k], at, d.recording
@@ -1050,7 +1068,7 @@ fn compare(mode: &Mode) -> Result<(Vec<String>, bool), Failure> {
                 (None, ..) => Some(format!("no field ({} is not in the manifest)", a.recording)),
                 (Some(_), _, None) => None,
                 (Some(e), None, Some(t)) if ours_dump.is_none() => {
-                    let own = replay_to(dir, e, a.tick, arm.as_ref())?;
+                    let own = replay_to(dir, e, a.tick, armed(e))?;
                     Some(match own.first_difference(&t) {
                         Some(d) => {
                             format!("{} (ours: this build's replay to that tick)", diff_text(&d))
@@ -1062,7 +1080,7 @@ fn compare(mode: &Mode) -> Result<(Vec<String>, bool), Failure> {
                     Some("no field (ours' dump is not at that tick)".to_owned())
                 }
                 (Some(e), Some(o), Some(t)) => {
-                    let own = replay_to(dir, e, a.tick, arm.as_ref())?;
+                    let own = replay_to(dir, e, a.tick, armed(e))?;
                     Some(match words_difference(&own, &o, &t) {
                         Some(d) => format!(
                             "{} (both dumps given, named by this build's)",
@@ -1388,6 +1406,60 @@ mod tests {
             })),
             Err(Failure::Usage(_))
         ));
+        // One default for `--on` (EA-9): `compare`, like the legs, arms the bite on the first recording the lists hold
+        // (bitten_name), or on `--on`'s, and on no other (arm_for).
+        let (first, _) = short();
+        let mut second = first.clone();
+        second.name = "TC-s9".to_owned();
+        let arm = Arm {
+            id: "PM-0004-21/hash-map".to_owned(),
+            tick: 1_234,
+            on: None,
+        };
+        let on = bitten_name(Some(&arm), &[&first, &second]).unwrap();
+        assert_eq!(on.as_deref(), Some("TC-s8"));
+        assert_eq!(arm_for(Some(&arm), on.as_deref(), &first), Some(&arm));
+        assert_eq!(arm_for(Some(&arm), on.as_deref(), &second), None);
+        let named = Arm {
+            on: Some("TC-s9".to_owned()),
+            ..arm.clone()
+        };
+        let on = bitten_name(Some(&named), &[&first, &second]).unwrap();
+        assert_eq!(arm_for(Some(&named), on.as_deref(), &first), None);
+        assert_eq!(arm_for(Some(&named), on.as_deref(), &second), Some(&named));
+        assert!(
+            bitten_name(
+                Some(&Arm {
+                    on: Some("XX".to_owned()),
+                    ..arm
+                }),
+                &[&first]
+            )
+            .is_err()
+        );
+    }
+
+    #[test]
+    fn a_digest_field_of_a_dump_of_another_length_says_the_layouts_differ() {
+        // RR-L3: the field's indexes are this build's own dump's, so a dump of another layout (P6's word-size plant
+        // made the Wasm dump 2 words longer) would be read at other fields' words; no digest is read from it.
+        let own: Vec<u32> = (0..10).collect();
+        let at = [6, 7];
+        assert_eq!(
+            digest_text(own.len(), &at, &own),
+            format!("{:016x}", (7u64 << 32) | 6)
+        );
+        let longer: Vec<u32> = (0..12).collect();
+        assert_eq!(
+            digest_text(own.len(), &at, &longer),
+            "layouts differ (10 and 12 words)"
+        );
+        assert_eq!(
+            digest_text(own.len(), &at, &own[..8]),
+            "layouts differ (10 and 8 words)"
+        );
+        assert_eq!(digest_text(own.len(), &[], &own), "none");
+        assert_eq!(digest_text(own.len(), &[2], &own), "2");
     }
 
     #[test]
diff --git a/.gate/u4d2-r1/spikes/netcode-proof/crates/lab/src/lib.rs b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab/src/lib.rs
index 3a789bb..dbfd0d5 100644
--- a/.gate/u4d2-r1/spikes/netcode-proof/crates/lab/src/lib.rs
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab/src/lib.rs
@@ -8,7 +8,7 @@
 /// One tick's entry in a leg's per-tick hash list: the recording (or run) it replays, the tick and the state hash.
 #[derive(Clone, Debug, PartialEq, Eq)]
 pub struct TickHash {
-    /// The recording's name, as its manifest names it (or a run's, for U4r's legs).
+    /// The recording both legs replay (the same name on both legs), as its manifest names it.
     pub recording: String,
     /// The tick the hash is of.
     pub tick: u32,
````

### check.test.mjs (check.mjs's self-tests)
_ 1 file changed, 279 insertions(+), 35 deletions(-); the diff itself is in `.checkpoints/netcode-proof-test/u4d2-r2-packet-tests.md` (`git diff --no-index .gate/u4d2-r1/spikes/netcode-proof/check.test.mjs .claude/worktrees/u4d2-build/spikes/netcode-proof/check.test.mjs`)._

## What it is judged against

### The advisor's round-1 ruling, Blocks 1 to 8: the findings' routes, the fix pass's scope, bounds and evidence, round 2's seats (advisor-u4d2-r1.md)
_`.checkpoints/netcode-proof-test/advisor-u4d2-r1.md` lines 68-469 (sha256 9978b43b7639), as they are:_

````text
## Rulings (one block per item of the brief)

### Block 1. Item 1: the four Mediums marked before the merge (done)
**RR-M2 with TE-M1 (the read-back's superset fails open): the finding holds; REVISE, in the fix pass as FX-1 and FX-2.**
- Proven twice by run on the copy (r1-rr-superset.log:145-148, r1-te-tp2-flags.log run 3: deps PASS with
  CARGO_ENCODED_RUSTFLAGS=-Ctarget-cpu=bleeding-edge and RUSTFLAGS=-Ctarget-feature=-simd128,-relaxed-simd; the control
  FAILs), and the module cargo builds in that environment carries +simd128 and +relaxed-simd
  (r1-rr-superset-build.log:14). So condition 7 (a)'s "fails the step ... never only reports" did not hold for that
  case, and the step's limits text (readBackCheck's doc: "a superset of the one source cargo uses, which fails closed")
  is untrue. Severity stays Medium (its reviewers'): it needs two crafted rustflags sources, and no runner, config or
  session here sets any.
- **My error, corrected in writing.** My U4D2-Q4 (a) ruling said the superset "is a superset, fail-closed". A superset
  of flags is not a superset of features: a flag can take a feature away, and cargo drops every source but the first
  non-empty one. The rest of Q4 (a) stands: the read-back runs from spikes/netcode-proof, and nothing re-implements
  cargo's choice of source.
- **The form (FX-1, which is also SEC-U4D2-5's and SEC-U4D2-6's fix):** the deps step refuses every rustflags source
  that holds a flag, whatever the flag: each environment variable rustflagSources reads (every name ending RUSTFLAGS)
  and each `rustflags` key of each cargo config it reads, each refusal naming the source and its value. It also refuses
  a set CARGO_BUILD_TARGET and any `build.target` key in those configs. The spike's builds use no rustflags
  (e1-check.log: "given no rustflags") and pass no target config, so this refuses nothing a lawful build needs. It is
  stricter than plan §5 U4d2's four named flags, never looser: those four stay refused. The read-back stays as the plan
  asks ("read back with `rustc --print cfg` per target ... never only reports"), with its non-vacuity. When the step can
  pass, it is now given exactly what cargo gives rustc (no flags); readBackCheck's doc and DEPS_CHECKED's limits say
  so, and the words "which fails closed" go. No per-source read-back is needed, because no source may hold a flag.
  buildFlagProblems' named-flag messages may stay beside the new rule.
- **Second half (FX-2, which is also SEC-U4D2-7's fix):** the loader's `moduleProblems` reads the module's
  `target_features` custom section (`WebAssembly.Module.customSections(module, "target_features")`). It refuses the
  module when that section lists simd128 or relaxed-simd with the prefix `+` or `=`, when it has no such section or
  more than one, or when the section does not parse, each by name. This reads the artifact itself, so it also catches
  what no source read can: source-level `#[target_feature]` (Block 2) and a flag that reaches rustc by a route check.mjs
  does not read. Both built modules list neither feature today (the rust seat's wasminfo run), so this refuses no
  lawful build.
- **Proof:** see Block 8, FX-1 and FX-2.

**RR-M1 with SEC-U4D2-2 and TE-M2 (one point): the finding holds; REVISE, in the fix pass as FX-3.** TP4
(r1-te-tp4-body-unsafe.log) shows that an `unsafe` read inside `begin`'s body passes rustc, clippy -D warnings and the
unit tests, and is refused only by the deps scan (lib.rs:101). TP10 shows that a scan regression letting it through
leaves 63 of 63 self-tests and the live deps step green. So the scan is that case's only guard, and nothing pins it.
The form is the seats': one self-test case (an `unsafe {}` inside a listed export's body, refused by hashWasmProblems
at its file and line); the deps fixture's stray-unsafe plant also placed in an export's body (beside the new-fn case,
which stays); and TP4's live plant once, with the wasm32 build's exit 0 beside the deps FAIL. Round 2's test seat
re-plants TP10's regression, and the new case must fail by name.

**SEC-U4D2-1 (Object.prototype imports): the finding holds; REVISE, in the fix pass as FX-4.** r1-sec-imports.log:5-7:
a module importing `constructor.constructor` (JS's Function) instantiates with moduleProblems []. The loader's own
words ("a module that imports anything fails", loader.mjs:9-10, :77-78) are false, and the env.now self-test passes
because `env` is not on Object.prototype. The form: `moduleProblems` refuses every entry of
`WebAssembly.Module.imports(module)`, by module and name, before anything is instantiated, and `instantiate` passes
`Object.create(null)`. Self-tests: modules importing `constructor.constructor`, `__proto__.valueOf` and `env.now`, each
refused by name. The loader's header and instantiate's doc then say what is true.

**INF-1 (no cold time was measured): the finding holds; report text before the merge, and the cold figure measured
after the merge without any exception to §7.1. Nothing goes to Mustafa.**
- **Why the brief's "cold" could not be met (my miss as well):** the brief defines cold as "its first run: the release
  builds not yet in the shared target". But the shared target always held other lanes' release dependencies (tokio,
  quinn, ring and rustls were already there at b3-lab1 16:47), and Mustafa's rule (history.md:411-412, "point all
  copies at one shared Cargo target folder") with §7.1 items 1 and 7 leaves a copy no empty target. My brief check did
  not see that.
- **The route that needs no exception:** W1-TARGET item 3 (advisor-u4-wave1.md:576-580): "Between waves, with no lane
  running, the main session may empty the shared folder by `cargo clean --target-dir .gate/shared-target`, an approved
  method ... §7.1 item 7 ... binds the copies, not the main session." Wave 2 ends with U4d2's merge (U4e merged at
  c88eb31). So, after U4d2's merge and before U4r's brief, with no lane running, the main session empties the shared
  target and runs `node check.mjs --only wasm` once through main-run.sh (the lock exclusive, the shared target,
  offline). That run's seconds are the step's cold figure: every dependency is built for both targets, and the
  toolchain and the registry cache stay as they are. It feeds U4r's Evidence decision (plan :800-802: "in check.mjs if
  the step stays within 5 minutes, else through ops/runs/det-u4.mjs"). It is M9's last item. If it exceeds 300 s, that
  comes to me as U4r's det-u4.mjs question, not as a U4d2 defect.
- **The words before the merge (u4d2-report.md, T5):** every figure is labelled "warm for dependencies, the workspace
  rebuilt by the touch loop". The first release builds are given (b3-lab1, npt-lab 16.04 s; b2-hw-test3, npt-hash-wasm
  wasm32 3.57 s), with the infra seat's bound of about 80 s against 300 s. The report says why no cold run was taken
  before the merge, and where the cold figure comes from (M9). The budget verdict stands on the warm figures: 40.0 to
  46.5 s over seven runs.
- **The rust seat's scratch target.** Its `--target-dir .gate/u4d2-rr-target` build (r1-rr-superset-build.log:3,
  38 MB) was a cargo call in the builder's copy off the shared target, against §7.1 item 1's letter, which the infra
  seat saw. Its reason was sound: it kept a SIMD-built module out of the shared target, where a later run could have
  read it. It is recorded as a process slip in T5. The folder goes by `cargo clean --target-dir` (an approved method)
  with the copies at M9. No later seat or builder does it again: each round-2 brief says "the shared target only".

Block 1 written 2026-11-12.

### Block 2. Item 2: SEC-U4D2-7 (source-level target features reach the Wasm leg): this fix pass, FX-2. APPROVE as a finding.
- The facts hold (r1-sec-tf.log, r1-sec-tf2.log:6). Under `#![deny(unsafe_code)]` and with no rustflag, a safe
  `#[target_feature(enable = "relaxed-simd")]` function compiles for wasm32 and emits `f32x4.relaxed_madd`, and the
  linked module's `target_features` section lists +relaxed-simd and +simd128. No scan, ban or read-back sees it. The
  exposure is npt-sim's since U4s, but U4d2's step is the one that says simd128 and relaxed-simd are off, so U4d2 makes
  that true.
- **Route: the fix pass, not a carry.** It is FX-2's own check: the same reading of the section, failing closed when
  the section is missing, so it costs nothing beyond RR-M2's second half. A carry beside C-U7-1 would leave the step
  claiming a property it does not check until U7.
- **Not taken:** the optional source scan (refusing `target_feature` attributes and `core::arch` or `std::arch` paths in
  the crates the Wasm leg compiles). Reading the artifact is the decisive check for the Wasm leg. On x86-64, calling a
  `#[target_feature]` function from code built without that feature needs `unsafe` (Rust's target_feature 1.1 rules,
  my reading), which npt-sim forbids and the scan refuses in npt-hash-wasm, so the native legs are not exposed this way.
- **Its live plant (FX-2's proof):** a safe `#[target_feature(enable = "simd128")]` function (or relaxed_madd, as the
  security seat's probe used), reached from a listed export in crates/hash-wasm/src/lib.rs by Edit. The wasm32 build
  exits 0, the deps step passes, and the loader refuses the module naming the feature, so the wasm step FAILs. Restore
  it and check the sha256.
- C-U7-2 (T3) adds that U7's check of wasm-opt's output runs this same refusal, so U7 must keep the section or say how
  it checks the features.

Block 2 written 2026-11-12.

### Block 3. Item 3: the Lows that touch a build's integrity or a verdict (done; labels stand; each rides the push the Mediums need anyway)
- **SEC-U4D2-5 (`--cfg feature="bites"`, `-C overflow-checks=off`, `-C llvm-args` pass the name rule): fix pass, FX-1.**
  The rule "every rustflags source that holds a flag is refused" is the seat's own fix. It closes all three, and also
  the plan.md §4.5 rule a rustflag could override ("`overflow-checks = true` for npt-sim and npt-proto in every
  profile"). Self-tests: each of the three in RUSTFLAGS, and a config file's `rustflags`, each FAILs by name.
- **SEC-U4D2-6 (CARGO_BUILD_TARGET or `build.target` builds npt-lab for another target): fix pass, FX-1,** in the
  cheaper of the two forms. The deps step refuses either, read as it reads the rustflags sources. Adding
  `--target x86_64-unknown-linux-gnu` to the step's native builds would move their artifacts, the six pre-builds'
  provenance and the shared target's layout; that is not needed when the input is refused. Self-tests: one of each.
- **INF-3 (the std-sin catch depends on the CPU through glibc's IFUNC `sinf`): the wording in the fix pass (FX-9), the
  data's CPU record a follow-up.** It never gives a false pass, only a false FAIL whose message names the libc alone.
  The step's std-sin failure line adds that glibc picks `sinf` by the CPU's features when it loads, so on another CPU or
  libc the scan is re-run there (u4d2-brief.md 2.4 (a) item 6). The scan file's format does not change now. Follow-up
  (card): when the scan is next re-run (a toolchain, glibc or machine change), its record names the CPU model and its
  fma and avx2 flags.
- **INF-4 (no child call of the wasm step has a timeout; a hang holds the exclusive lane lock): fix pass, FX-7.** Every
  child call of the step except its four cargo builds runs with a timeout of 300 s: the loader legs, the `npt-lab hash`
  legs and compares, vectors and bites. That is the step's whole budget, and each call takes under 5 s today
  (b8-wasm1.log's split). A timeout is a FAIL naming the call and the limit. The cargo builds are left without one: under
  the exclusive lock no other cargo can hold theirs, and a cold build's time is INF-1's to measure, not to cut. Self-test:
  a fake io whose call reports a timeout, and the FAIL line names it.
- **RR-L3 (`--digest-field` reads every dump at the native layout's indexes): fix pass, FX-8.** When a dump's length
  is not this build's own, `hash compare` prints "layouts differ (<n> and <m> words)" in place of a digest. One unit
  test (two dumps of different lengths). This answers the builder's open choice: P6's word-size plant, the one bite whose
  output this line exists to read, now reads true. EA-9 rides the same edit (Block 5).

Block 3 written 2026-11-12.

### Block 4. Item 4: the other Lows, each one's route (done; labels stand; none pushes on its own)
The rule I apply: a Low rides the fix pass when it is cheap, inside U4d2's lane, and in a file the pass edits anyway, or
when it is the exception's own missing test; a Low whose natural home is a later unit's change goes to that unit as a
carry, with its words in T2 or T3; the rest are card follow-ups the report names.
- **RR-L1 (carry 29's comment says `sqrtsd` for sqrtf): fix pass, FX-11.** Cargo.toml's libm comment says sqrtf by
  `sqrtss` and sqrt by `sqrtsd` (libm 0.2.16 src/math/arch/x86.rs:13 and :26, the seat's read). Carry 29's own text is
  a reviewer's quoted words and stays as it is. The report lists carry 29 as met, with this correction named.
- **RR-L2 with EA-5 (npt-hash-wasm, a LEAF crate, has no clippy.toml, so plan.md §4.2's "Clock reads are banned in them
  by clippy" is false for it): fix pass, FX-12.** A new crates/hash-wasm/clippy.toml holds the HashMap and HashSet type
  bans and the std clock bans in the room's and lab's form (no tokio lines: hash-wasm has no tokio), its header citing
  plan.md §4.2's leaf rule. That makes the plan.md sentence true without a plan.md edit, which the other option needed.
- **RR-L4 with EA-1 (replay_to feeds the whole recording, so the drain is quadratic; the Wasm leg hashes every tick
  twice): a card follow-up.** The step takes 40-46 s of its 300 s. If U4r's Evidence decision (plan :800-802) or the
  cold figure (INF-1) brings time into question, first feed replay_to by chunks, as `serve` does, and cache the hash
  per `step` in hash-wasm's State (no export change).
- **RR-L5 (manifest names unchecked; `ticks` reaches `Vec::with_capacity` before the length and SHA-256 check): U4r's
  carry, entry 82 (T2), with EA-8.** U4r rewrites the manifest's reader for its runner recordings, so the checks go in
  with that change. The manifest is committed data, and today the step fails closed.
- **SEC-U4D2-3 (the signature scan reads the token `u32`, not the type): its comment in the fix pass (FX-11), and
  Q7 (b)'s type-section read as C-U7-2 (T3).** The comment beside EXPORT_INTEGERS states the limit (a module-local
  `type u32 = f64;` or `use core::primitive::f64 as u32;` passes the token scan; the artifact's export types are not
  read). The type read is about 60 lines of binary parsing, and its natural home is U7's browser-safe `moduleProblems`,
  which must also run on wasm-opt's output (C-U7-1). It stays an open security finding (Low) until U7: the report lists
  it.
- **SEC-U4D2-4 (a build script plus a renamed `include!` reaches inside an export's allow): fix pass, FX-6.** The deps
  step refuses a build script in each exception crate whose level is deny (a build.rs file, a `build` key in its
  manifest, or a custom-build target in `cargo metadata`). Neither has one today (read), and without one `env!("OUT_DIR")`
  does not compile. It also refuses `include` used as a path (not as an `include!`, `include_str!` or `include_bytes!`
  call) in those crates' sources. pathProblems' doc (check.mjs:367-370) names npt-hash-wasm beside npt-tcpinfo.
  Self-tests: V4's two forms from r1-sec-scan.log, each refused by name.
- **SEC-U4D2-6:** Block 3 (FX-1).
- **SEC-U4D2-8 with TE-L1 (the loader's kind refusal is untested; TP11 shows 63 of 63 pass with it disabled): fix pass,
  FX-5.** Two self-test cases: `begin` exported as a global, and `memory` exported as a function. Round 2's test seat
  re-plants TP11's disable, and both must fail by name. It is the third exception's own test, which ADR 0002:264-265
  asks for.
- **INF-2 (on Windows the copied executable loses its `.exe`): a card follow-up.** plan.md:668 puts this check in this
  container, so nothing runs it on Windows. If it is ever run there, the fix is to keep `extname(path)` in the copied
  name, with a check.test.mjs case.
- **INF-5 (a counted run on the merged tree takes about 11 minutes): M5's wording** (Block 9: in the background with a
  limit of at least 1,500,000 ms, U4e round 2's Q4 form; the wasm step re-timed from that run's printed split).
- **INF-6 (U7's Chromium leg and wasm-opt rerun have no runner yet): C-U7-2 (T3).** U7's brief names how its tools
  come (pinned downloads) and that a CI home would be .github/ work (risk tier 2, Mustafa's go).
- **INF-7 (README.md lists neither the wasm step nor its prerequisites): a follow-up for docs-keeper at U4's close.**
  The README is outside U4d2's lane (Lanes), and it was stale before U4d2 ("15 crates").

Block 4 written 2026-11-12.

### Block 5. Item 5: the engine-architect's and the test-engineer's findings (done)
**The engine-architect's nine Lows (APPROVE WITH FINDINGS accepted; labels stand):**
- **EA-1:** with RR-L4, a card follow-up (Block 4).
- **EA-2 (the status numbers live in lib.rs and in loader.mjs; ticks numbered from `--first-tick`): C-U7-2 (T3).**
  U7 splits the loader. The numbers then get one home, or a test that binds lib.rs's consts to the loader's. Today a
  renumbering fails closed, caught at the step's first end-to-end run (loader.mjs:158).
- **EA-3 (check.mjs reads npt-lab's verdicts from their English lines, never their exit codes): U4r's carry, entry 83
  (T2).** Every path fails closed today. At U4r's change to the step (its recordings: EA-8), the clean half also
  requires exit 0 from `npt-lab hash` and `hash compare`.
- **EA-4 (U7 can reuse the .wasm file, not the loader): C-U7-2 (T3).** Building the split now would be a feature added
  "for later", which the phase's rules refuse.
- **EA-5:** with RR-L2, FX-12 (Block 4).
- **EA-6 (TickHash.recording's doc invites legs named by run ids; HashCompare compares by position): the doc in the fix
  pass (FX-11), the rest in U4r's carry 83.** lab/src/lib.rs's doc says "the recording both legs replay (the same name
  on both legs)". U4r's -30 pre-check aligns its two lists by tick and says where its printed comparison comes from
  (compare_text is private to the binary). My condition 2 holds as built: HashCompare reads no file and knows no
  recording.
- **EA-7 (the public items to record): T1 (§4.1's rows, in the merge commit) and the report.** The report lists
  check.mjs's 23 new exports by count, each imported by check.test.mjs. The loader's three exports that no file imports
  (forEachChunk, parse, main) stay as they are and are named in T1, and U7's split (C-U7-2) decides their form. Removing
  `export` now would be churn ahead of that.
- **EA-8 (the manifest holds only the generator's inputs; U4r's recordings change its format): U4r's carry, entry 82
  (T2),** with RR-L5.
- **EA-9 (`hash compare` and the legs arm `--on` differently): fix pass, FX-8.** One default, the legs' (the bite armed
  on the first recording replayed), with h6's test following. hash.rs is open for RR-L3 anyway.

**The test-engineer's findings (REVISE accepted; labels stand).** Its runs settle each point by exit code and by
name, and its end state is clean: 17 of 17 sha256 equal, `diff -rq` against the read copy empty, crates/sim untouched.
I re-checked the 17 hashes myself (Notes).
- **TE-M1** is RR-M2, fixed as FX-1 and FX-2 (Block 1). Its negation case becomes a self-test, and round 2 re-runs TP2
  run 3, which must FAIL by the source rule.
- **TE-M2** is RR-M1 with SEC-U4D2-2, fixed as FX-3 (Block 1). Round 2 re-plants TP4 and TP10.
- **TE-L1** is SEC-U4D2-8: FX-5. **TE-L2** supports SEC-U4D2-3: its comment (FX-11), and the type read in C-U7-2.
- **TE-L3 (the std-sin verdict never re-shows that the bite caused the mismatch): U4r's carry, entry 83 (T2).** It is
  sound for the committed catch (SS-s1 tick 2 lies inside the hash-map pair's equal prefix, ticks 0 to 1,233). The guard
  is needed only once the catch moves, which only a re-run scan or a new recording can do. U4r's recordings are the
  first such change.
- **TE-L4 (carry 65 names the failing test, but its panic is lost to the 40-line tail): fix pass, FX-10.** It is cheap,
  in the lane, and it is what TE-M3's route needs. When any step fails, check.mjs writes the step's whole output to a
  file under the checkout's .gate/netcode-proof-test/ (one per step, replaced at each run, as the wasm step's folder
  is), and its FAIL lines name the file. Self-test: a recorded failing test run whose panic block lies above the last
  40 lines; the file holds it, and the printed lines name the path.
- **TE-M3 (npt-net's `l2::delay::tests::a_shutdown_s_never_completes_ends_at_its_deadline_without_spinning` failed
  once in the bites build under the counted run's load; it failed the same way in U3c): Medium, not before U4d2's
  merge. APPROVE as a finding.** It is not U4d2's, and this is shown, not assumed. U4d2 changes no npt-net file. The
  test (delay_tests.rs:2551-2603) reads no file; it counts pump runs after a 300 ms wait on a two-worker tokio runtime.
  npt-net's tests that read marks/SERIES.md (names.rs:143, delay_names.rs:449, socket_tests.rs:390) are fixed
  `contains` checks, which T4's added row cannot fail and which cannot fail once in many runs of one build: B2.3's
  reasoning for U4j holds here.
  - **What the merge's counted run does if it recurs (B2.3's rule; M6):** check.mjs's FAIL is never retried into green
    unnamed. That crate is re-run at once through main-run.sh with `--no-fail-fast`, its whole output kept under
    .gate/u4d2-merge/, with FX-10's file of the failed step copied beside it, and the test's name and panic go into the
    notes, tagged TE2-5. Because this test reads no U4d2 file and is in no crate U4d2 changes, it does not stop the
    merge. But the merge goes ahead only on a whole counted run in which all 11 steps pass, never on a test-step-only
    run. Any failure in npt-hash-wasm, npt-lab or npt-sim, in check.mjs's self-tests, or in a test that reads U4d2's
    files, stops the merge and comes to me.
  - **Where its cause goes (B2.3 (b)'s last sentence: "that test's cause is found and fixed in a planned change of its
    own"):** a planned change of the main session's (npt-net is U3's code, and no U4 lane holds crates/net), plan first
    with me, after U4d2's merge and before U4r's merge, so that U4r's counted and judged runs do not meet it unnamed. Its
    first step is the panic: FX-10's saved output from the next failure, or a loop of the bites build's workspace tests
    under load as B2.3 (b) ran them, now watching this test by name. That tells which assertion fails
    (`polls_after_deadline == 1`, `polls <= 3`, `s_calls <= polls`, or the 30 s hang guard) and why it fails under
    load. Never a retry, and never a looser bound without its cause. The card records it as TE2-5 (c).
  - **B2.3's rule after U4d2's merge:** its "until carry 65 is met" is read as met for the name only. The rule stays in
    force until TE-M3's cause is found and fixed. With FX-10, a first failure keeps its panic, so the rule's "name and
    panic" can be met from that failure itself.
- **Its other results** (the builder's six plants and TP1 to TP11 each failing by name where they should; h1 to h8
  asserting what they claim; no wrong-reason path for the hash-map verdict): accepted as evidence. They rest on runner
  logs with exit codes and libtest's or node's own summary lines. I read only the lines cited above, and none
  contradicts the seat.

Block 5 written 2026-11-12.

### Block 6. Item 6: the builder's points (done)
- **TE2-5 mid-build (62 of 63 in check.test.mjs, a failure its step plan did not name): outside the stop clause's
  letter. The outcome stands; it is recorded as a process slip (Low) and nothing is redone.** The clause (u4d2-brief.md
  :707-710) says "a failure ... in check.test.mjs ... stops your step group (report it), unless your step's plan names
  it as expected (a pin your step re-pins, or a test a later step rebuilds)". The failing test (check.test.mjs:205-222)
  was neither of those, so the step group should have stopped and reported. "My own unfinished code" is not one of the
  clause's exceptions, and it should not become one: the clause exists for the moment a builder judges its own failure.
  The judgement was right here, and that is proven, not assumed. The test is byte-identical to `<base>`'s (the test
  seat's sed diff). The fix was in the builder's new rule (check.mjs:138's `&& all.has("npt-sim")`). TP9 (r1-te-tp9-te25.log)
  shows that undoing the fix makes the unchanged test fail by name, so the test still pins what it pinned. The
  fix-pass brief carries the clause as written, and the report records the slip in T5. The builder's other TE2-5
  entries (h2's placeholder vector, the manifest's zero hashes, its own test errors) were named in advance by design or
  were failures in its own new tests: within the clause.
- **The three early `npt-lab hash` runs under the shared lock (b3-lab1 to b3-lab3; rule 2 asks `-x`): a process slip
  (Low), recorded, no redo.** None is a reported result. Each printed a placeholder's expected failure, and the three
  SHA-256 values it supplied were checked under `-x` by b3-lab4 ("as the manifest's") and by every counted run since
  (manifest `check()` binds length and SHA-256). A run from another copy's artifact could only have printed a value
  that those `-x` runs would then refuse.
- **rustfmt's diffs applied by Edit after the cut: within the write rule** ("never `cargo fmt` writing files: apply
  rustfmt's diff by Edit"); b8-rust2 shows fmt clean. Nothing to rule.
- **Its open choices:** the cold run is INF-1 (Block 1: report text now, the cold figure after the between-waves clean,
  no exception). `--digest-field` at the native layout's indexes is RR-L3 (Block 3: FX-8).

Block 6 written 2026-11-12.

### Block 7. Item 7: the main session's points (done)
1. **The restamp's stale plan cites: confirmed, with one addition.** At 3f58ff0d (the plan at 51520b4) the brief's
   section 1 item 6 (:141-143) and its :173 read §5 U4d2 at :666-712, Lanes at :850-888, §6's U7 line at :898-900 and
   §7 at :908-931 (headings read: 666/714, 850/890, 898, 908/933). **Also stale in the same item (:143):** "U4r's lines
   that reuse your work (:766-767, :790-792)", which read :776-777 and :800-802 at 3f58ff0d (the same text, read at
   both stamps). §4.1, §4.11, §4.14, §4.15, U4d's lines and §2's rows did not move. Nothing was built short because of
   it. The builder built §5 U4d2 to its end, Q11's two plants included (P5, P6). The seats judged against the whole
   section, and the arch seat found every path inside the Lanes line's U4d2 list. The brief is a closed record of what
   was built, so no re-fold is needed: T5 says it in one line. For the fix pass's brief, cite the plan by section
   heading, not by line.
2. **Carry 66's header param row: the main session writes it, in U4d2's merge commit, by script from T4 below, applied
   before M5's counted run.** The marks step and npt-net's name tests read SERIES.md. marks/ is in no wave-2 lane
   (Lanes), and U4e's merge carried its own SERIES.md row the same way (advisor-u4e-r1.md T4). The builder's proposed
   words are kept, with the Values cell saying what the hash is. The Used-by cell is "-3 (recorded)", the form the
   table's other recorded params use.
3. **A13: right not to wait; it is now committed, and I checked the commit.** 1b4a3c8 changes one file,
   crates/server/clippy.toml (`git show --stat`: 5 in, 1 out). Its blob equals the working tree's (a82dd3e7…). spikes/
   is otherwise unchanged since c88eb31 (`git diff --quiet`, exit 0). The diff is the room file's two type entries
   plus a header naming Q9 (a), §4.11 and A13 (read). The plant log shows clippy on the main folder's npt-server
   refusing a scratch HashMap at lib.rs:20 and :21, exit 101 (a13-plant.log:13-36). The control exits 0. check.mjs
   counted passed all 10 steps, 473 / 541 / 52 of 52, marks 81 (a13-check-counted.log). Its log has no provenance lines
   because no pre-build ran; for a clippy.toml-only change the plant log's "Checking npt-server (<main folder>)" line is
   the provenance that matters. Why no wait: crates/server is in no open lane (U4e merged), U4d2 changes no server
   file, and check.mjs reads no server/clippy.toml (CLIPPY_TWINS is sim and proto only). Left: CI's gate and smoke on
   the pushed head, which M2 checks. U4d2's merge then applies on a head that holds it (M2 names it).
4. **The "two exceptions" text: confirmed, with one more place.** None outside U4d2's lane. Inside it there are three,
   all in check.test.mjs: :328 (a test name), :364 (a comment) and **:529** ("unsafe code outside the two exception
   crates fails however the lint was relaxed (review S1)", a test name). Each says "three", or names what it means, in
   FX-11. Renaming a test changes no count.
5. **The merge's counts: confirmed as the method, with the fix pass's deltas added.** On c88eb31 (473 / 541 / 52 of 52,
   10 steps; A13 left them unchanged), U4d2's round-1 deltas (+17, +19, +11, +1 step) give 490 / 560 / 63 of 63 and 11
   steps. The fix pass adds its own (FX-8's unit test, +1 in each build; its new self-tests). M5 expects 490, 560 and
   63 plus the fix pass's deltas, each named, the sums shown. Interactions only the merged run can show are named in M5:
   the features comparison now walks U4e's npt-server library, and the wasm step's npt-lab release build carries U4e's
   crates. The counted run goes in the background with a limit of at least 1,500,000 ms (INF-5; U4e round 2's Q4).

Block 7 written 2026-11-12.

### Block 8. Item 8: the fix pass, its builder, and round 2's seats. APPROVE on these terms.
**Scope (each item with its proof; nothing else):**
- **FX-1** RR-M2's first half, SEC-U4D2-5 and SEC-U4D2-6 (Blocks 1 and 3). The deps step refuses every rustflags source
  that holds a flag, and a set CARGO_BUILD_TARGET or a `build.target` key. readBackCheck's doc and DEPS_CHECKED's limits
  say the read-back is exact because no source may hold a flag. Self-tests: RR-M2's negation pair (both sources named);
  `--cfg feature="bites"`, `-C overflow-checks=off` and `-C llvm-args=-fp-contract=fast` in RUSTFLAGS; a config file's
  `rustflags`; CARGO_BUILD_TARGET; `build.target`; each FAILs by name, and an environment with none passes. Live:
  r1-rr-superset.log's first environment (`--only deps`) now FAILs, naming both sources.
- **FX-2** RR-M2's second half and SEC-U4D2-7 (Blocks 1 and 2). `moduleProblems` refuses a `target_features` section
  that lists simd128 or relaxed-simd (`+` or `=`), and a module with no such section, with more than one, or with one
  that does not parse. Self-tests: hand-built modules (check.test.mjs's wasmModule helper) for +simd128, +relaxed-simd,
  no section, two sections, a truncated section, and the clean list accepted. Live: Block 2's target_feature plant
  (build exit 0, the step FAILs on the loader's refusal), and the two clean modules accepted.
- **FX-3** RR-M1, SEC-U4D2-2 and TE-M2 (Block 1): the self-test case, the fixture plant in an export's body, and TP4's
  live plant once (the wasm32 build exits 0, deps FAILs at the line).
- **FX-4** SEC-U4D2-1 (Block 1): every import refused by name before instantiating; `Object.create(null)`; three
  self-tests; the loader's header and instantiate's doc corrected.
- **FX-5** SEC-U4D2-8 and TE-L1 (Block 4): the two kind cases.
- **FX-6** SEC-U4D2-4 (Block 4): no build script in a deny-level exception crate; `include` refused as a path there;
  pathProblems' doc; V4's two forms as self-tests.
- **FX-7** INF-4 (Block 3): a 300 s timeout on each of the wasm step's non-cargo child calls; a timeout is a named FAIL;
  one self-test.
- **FX-8** RR-L3 and EA-9 (Blocks 3 and 5): "layouts differ (<n> and <m> words)" in place of a digest, with one unit
  test; one default for `--on`, the legs', with h6 following.
- **FX-9** INF-3 (Block 3): the std-sin failure line's IFUNC sentence.
- **FX-10** TE-L4 (Block 5): a failed step's whole output kept in a file under .gate/netcode-proof-test/ (one per step,
  replaced at each run), named in its FAIL lines; one self-test.
- **FX-11** texts: check.test.mjs:328, :364 and :529's "two" (Block 7, point 4); Cargo.toml's libm comment, sqrtf by
  `sqrtss` (RR-L1); SEC-U4D2-3's limit beside EXPORT_INTEGERS; TickHash.recording's doc (EA-6).
- **FX-12** RR-L2 and EA-5 (Block 4): crates/hash-wasm/clippy.toml.
- **Not the builder's:** the report, marks/SERIES.md (T4), the plan files and the carries (T1-T3), INF-1's figures and
  words (T5), and A13 (done).

**Bounds.** Files only in U4d2's lane, and of those only: check.mjs, check.test.mjs, crates/hash-wasm (loader.mjs,
src/lib.rs only for FX-3's and FX-2's restored plants, and the new clippy.toml), crates/lab/src/hash.rs and
crates/lab/src/lib.rs (a doc line), and Cargo.toml's libm comment. No Cargo.lock change, no manifest change, no new
public Rust item, and no new loader or check.mjs export beyond what a self-test imports (each one named in the record).
No change to recordings.json, std-sin-scan.txt or co-initial-dump.txt, and no crates/sim file, not even as a plant.
Carry 52 holds: the recordings are U4d-F's generator's, and PM-0001-2's 9.5 floor does not move. Any of these needed,
or a test that cannot be made to fail under its plant, stops that item: the builder reports it, and it comes to me.

**Who builds:** one builder (general-purpose, Opus, effort xhigh), in the same worktree (`.claude/worktrees/u4d2-build`,
uncommitted on 51520b4), reading its record (u4d2-build.md), this ruling's Blocks 1 to 8 and the copy's diff first,
as U4e's fix pass did. It works under W1-TARGET's lock (the lane lock held for each cargo run, the shared target only,
offline) through u4d2-run.sh, and counts in the three-agent limit. Its brief carries, word for word, u4d2-brief.md
section 4's binding sentences: the write rule (T4 item 1's sentence), the plant rule (B6), the stop clause with the
crates as named there, B2.3's rule, "No retry for the `wasm` step", and Mustafa's builders' limits. It also carries
this ruling's note on the stop clause (Block 6: an unnamed failure stops the step group, whoever's code caused it). It
cites the plan by section heading, never by line. Its plan is written first into the record's new section "Fix pass,
round 1", then a notes line after each FX.

**Its evidence (in the record):** for each FX, the plant's diff or the environment, the run (`-x -k`), the step, test
or self-test that FAILED by name, the restore by Edit, and the file's sha256 equal to its fixed form. Then fmt
(rustfmt's diff applied by Edit), clippy in both builds and for wasm32, and doc. Then check.mjs counted twice in the
copy, all 11 steps, with the counts as deltas on 416 / 465 / 63 of 63 and each new test named, and the wasm step's
printed split from both runs. At the end, the sha256 of every changed or new file under spikes/, in `sha256sum` form
with paths from the copy's root, saved as .gate/u4d2/fx-hashes-end.sha256.

**Then the main session** re-cuts the snapshot by worktree_patch.py (wip/u4d2.patch replaced, committed), checks that
the copy's files equal fx-hashes-end.sha256, and cuts round 2's packet by review_packet.py on what changed (round 1's
patch at e2f2a5b, a526ef8e…, against the new one), with this ruling as the "judged against" half.

**Round 2's seats, on what changed** (Opus at xhigh; counted in the three-agent limit; each brief carries T4 item 1's
write rule and "the shared target only"; batches as round 1's, so that no timing run meets a plant):
- **Batch 1** (runs through the runner in the builder's copy while nobody plants):
  - **security-auditor:** FX-1 against every source form it probed (r1-sec-scan.log's name-rule cases, RR-M2's pair,
    CARGO_BUILD_TARGET); FX-2 on sec-probe-tf2's module (refused) and on both clean modules (accepted); FX-4 on
    sec-probe-imports' modules (each refused by name); FX-6 on V4's forms; FX-3's case; SEC-U4D2-3's comment and
    C-U7-2's words.
  - **rust-reviewer:** FX-1 against cargo's rustflags and build-target rules; FX-2's section parse (LEB128 lengths, the
    prefixes, a missing or duplicated section); FX-8 (panics, casts) and FX-12 (clippy in both builds and targets);
    FX-11's libm comment against libm 0.2.16; r1-rr-superset.log's first environment, which must FAIL now.
  - **infra-engineer:** FX-7 (which calls, the limit, a hang's FAIL line) and FX-10 (the file's place, its replacement
    at each run, its size); the step's time after the fix pass (`--only wasm` with `-x`, twice); T5's INF-1 words
    against its finding.
- **Batch 2:**
  - **test-engineer** (with Edit, in the copy): its start hashes first, as .gate/u4d2/r2-hashes-start.sha256; each FX's
    proof re-shown in its own form where it doubts the builder's; TP2 run 3, TP4, TP10 and TP11 re-planted, each now
    failing by name; one counted check.mjs (in the background, with a limit of at least 1,500,000 ms).
  - **engine-architect** (read only): T1's §4.1 rows against the final diff (a loader export the fix pass adds or drops
    is named there); FX-2's and FX-4's place in the loader against C-U7-2's split; carries 82 and 83 and C-U7-2's words
    against the code.
- **Round 2's outcome:** APPROVE, or APPROVE WITH FINDINGS with Lows only, lets the merge go ahead on M1-M9; its Lows
  become follow-ups that the main session records. A Medium comes to me first and becomes a follow-up or a carry, never
  a third round. A High or a Critical comes to me for a third round on it alone (the review budget's one exception);
  one still open after that stops U4d2 for Mustafa.

Block 8 written 2026-11-12.

````

### The advisor's round-1 ruling, the texts T1 to T5 (advisor-u4d2-r1.md)
_`.checkpoints/netcode-proof-test/advisor-u4d2-r1.md` lines 548-649 (sha256 9978b43b7639), as they are:_

````text
## The texts (each copied by script from its code block; never retyped)
Each anchor below was found exactly once at the head (6c97718), counted with `grep -cF`. Carry numbers 82 and 83 and
the name C-U7-2 are free (no entry, no mention).

### T1. u4-build-plan.md §4.1: npt-lab's row and npt-hash-wasm's row (the merge commit)
(a) The row `| npt-lab | subcommands `scenario` (with `--record`), `replay` and `hash` | plan.md:590-591 |` becomes:
```
| npt-lab | subcommands `scenario` (with `--record`), `replay` and `hash` (`hash` with its modes: the two native legs, `compare`, `vectors` and `bites`); in the library, `HashCompare` (its constructor `of` and its variants `Equal`, `Empty` and `Mismatch`) and `TickHash` (the per-tick entry `of` takes); in the run header, the param `fate_log_v1_datagram_sha256` | plan.md:590-591; the advisor's condition 2 (`HashCompare` is U4r's -30 comparer, so it lives in the library and reads no file); §4.6's "0 compared fails" (`Empty`, as npt-judge's `eval::Outcome::Empty`); carry 66 (the v1 datagram hash, for §5 U4c's L-c8); the advisor's U4d2 round-1 ruling (EA-7) |
```
(b) The row `| a new crate | `npt-hash-wasm` (cdylib; integer-only exports) | U4-WASM (B) |` becomes:
```
| a new crate | `npt-hash-wasm` (cdylib): its seven `#[unsafe(no_mangle)] pub extern "C"` exports `begin`, `feed`, `step`, `hash_lo`, `hash_hi`, `dump_len` and `dump_word`, integer-only (u32), every const private, beside wasm-ld's `memory`; its Node loader crates/hash-wasm/loader.mjs, exporting `HASH_WASM_EXPORTS` (the export list's one home, which check.mjs imports), `LINKER_EXPORTS`, `LOADER_EXPORTS`, `BEGIN`, `STEP`, `moduleProblems`, `exportProblems`, `instantiate`, `forEachChunk`, `replay`, `parse` and `main` | U4-WASM (B); plan.md §4.2's third `unsafe` exception (ADR 0002:264-265); U7 splits the loader for the browser (C-U7-2); the advisor's U4d2 round-1 ruling (EA-7) |
```

### T2. u4s-carries.md §U4r, after entry 77 (its last), above "## The next change to crates/judge (whoever makes it; its first step)" (the merge commit)
```
82. **EA-8 with RR-L5, the recordings manifest (u4d2-review-arch.md, EA-8; u4d2-review-rust.md, RR-L5; the advisor's
    U4d2 round-1 ruling, Blocks 4 and 5):** "U4r's runner recordings join crates/lab/recordings.json (§4.15), whose
    entries today hold only the synthetic generator's inputs, read by hash.rs's `manifest` and generated by its
    `regenerate_all`: U4r changes that schema (a kind or script key, and the dispatch both functions need), and its
    brief says so. With that change, the reader checks each entry's name against `[A-Za-z0-9-]+` and bounds `ticks` by
    `bytes / 12` before it generates anything (today a bad name reaches a file path and the `h <recording> <tick>
    <hash>` line, and a large `ticks` reaches `Vec::with_capacity` before the length and SHA-256 check)."
83. **EA-6, EA-3 and TE-L3, the comparer for -30 and the wasm step's verdicts (u4d2-review-arch.md, EA-3 and EA-6;
    u4d2-review-test.md, TE-L3; the advisor's U4d2 round-1 ruling, Block 5):** "(a) For -30's pre-check, U4r gives
    `HashCompare::of` two lists that name the same recording on both legs and line up tick for tick (it compares by
    position, names included), and says where its printed comparison line comes from (`compare_text` is private to
    npt-lab's binary). (b) At U4r's change to check.mjs's `wasm` step (its recordings: entry 82), the clean half also
    requires exit 0 from `npt-lab hash` and `npt-lab hash compare` (today the verdict is read from their English lines
    alone, which fails closed), with one fixture case each. (c) In the same change, the std-sin verdict re-shows that
    the bite causes the mismatch it judges: the bitten native leg's first difference from a clean bites-build native leg
    is the committed (recording, tick, field), or the step FAILs when the committed catch is not on the manifest's
    first recording before the hash-map bite's arming tick (today it is sound only because SS-s1's tick 2 lies inside
    the hash-map pair's equal prefix)."
```

### T3. plan.md's U7 row: C-U7-2 (the merge commit)
The text `or states why not |` (found once, at the end of the U7 row's last cell) becomes the text below, on the row's
one line (no "|" added inside the cell):
```
or states why not. **C-U7-2** (the advisor's U4d2 round-1 ruling, 2026-11-12: EA-2, EA-4, INF-6, SEC-U4D2-3 and TE-L2): U7's Chromium hash check loads U4d2's npt-hash-wasm.wasm as it is (no imports; the seven integer exports and `memory`), and splits crates/hash-wasm/loader.mjs into a browser-safe module (the export list, its one home kept; the status numbers, with one home or a test binding them to lib.rs's constants; `moduleProblems` with its import, kind and `target_features` refusals; the replay loop over a chunk source) beside the Node script; `moduleProblems` also reads each export's type from the module's type section and refuses any but i32 (U4d2's source scan reads the token `u32` only), and it runs on the wasm-opt output C-U7-1 checks, so U7 keeps that output's `target_features` section or states how it checks the features; U7's brief names how its Chromium leg and wasm-opt get pinned tools (no CI job runs spikes/; a CI home is .github/ work, risk tier 2, Mustafa's go) |
```

### T4. marks/SERIES.md, the header-params table, after `fate_log_sha256`'s row (the merge commit, before M5's run)
After the line `| `fate_log_sha256` | hex string | -3 |` (found once, :397 at the head), insert:
```
| `fate_log_v1_datagram_sha256` | hex string: the SHA-256 of v1's tag and the v2 fate log's datagram entries (carry 66), which §5 U4c's L-c8 compares with U2's committed hashes | -3 (recorded) |
```

### T5. u4d2-report.md's section on the review's rulings (new file, beside u4e-report.md)
Copied by script as it is. Each `<…>` is a value the main session fills from the named source (round 2's notes, the
fix pass's record, or the merged tree's M5 lines), and the notes name the log and line of each fill; no other word
changes.
```
## What U4d2's review settled (the advisor's U4d2 rulings: round 1, advisor-u4d2-r1.md; round 2, <round 2's ruling file>)
- **Round 1** (five seats on one head, 51520b4 plus the snapshot at e2f2a5b; the spend limit cut the second batch at
  20:52 UTC, after the architect's and the test seat's plans and with no plant in place, and both resumed at 22:53 on
  the same bytes, so one round): rust-reviewer, security-auditor and test-engineer REVISE, infra-engineer and
  engine-architect APPROVE WITH FINDINGS; no Critical or High; six Mediums (RR-M2 = TE-M1; RR-M1 = SEC-U4D2-2 = TE-M2;
  SEC-U4D2-1; SEC-U4D2-7; INF-1; TE-M3). One fix pass (FX-1 to FX-12), then round 2 on what changed: <round 2's
  verdicts>.
- **The build-flag guard was fail-open for one case, now closed (RR-M2; Medium; fixed).** The read-back gave rustc every
  rustflags source at once, but cargo uses only the first non-empty one, so a negation in a source cargo skips hid an
  enabling flag in the one it used: the deps step passed while the module built so listed +simd128 and +relaxed-simd.
  The advisor's U4D2-Q4 ruling had called that superset fail-closed; it was not. Since the fix the deps step refuses
  every rustflags source that holds a flag (the spike's builds use none), and a set CARGO_BUILD_TARGET or
  `build.target`, so the read-back reads exactly what cargo gives rustc; and the loader refuses a module whose
  `target_features` section lists simd128 or relaxed-simd, or that has none, which also covers a safe
  `#[target_feature]` function reaching the Wasm leg with no flag and no `unsafe` (SEC-U4D2-7; Medium; fixed).
- **The third exception's guards (RR-M1, SEC-U4D2-1; Medium; fixed).** An `unsafe` block inside a listed export's body
  is accepted by rustc under the export's own allow, so only the deps scan refuses it: that case now has its self-test
  and was shown live (build exit 0, deps FAIL at its line). The loader's empty import object inherited
  Object.prototype, so a module importing `constructor.constructor` instantiated; it now refuses every import by name
  and instantiates with a null-prototype object. The kind refusal has its two self-tests (SEC-U4D2-8); a build script
  and `include` used as a path are refused in the deny-level exception crates (SEC-U4D2-4).
- **The step's time (INF-1; Medium; report text).** Every figure here is warm for dependencies, the workspace rebuilt
  by the runner's touch loop: <the fix pass's and round 2's wasm step seconds>, and <the merged tree's> on the merged
  tree, against a 5-minute budget. No cold run was taken before the merge: the shared target always held other lanes'
  release dependencies, and Mustafa's one-folder rule gives a copy no empty target. The first release builds in the
  lane took 16.04 s (npt-lab) and 3.57 s (npt-hash-wasm, wasm32), and the infra seat's bound for a cold step is about
  80 s. The cold figure comes from the first run after the between-waves clean of the shared target (W1-TARGET item 3):
  <its seconds, from the card>.
- **An npt-net test that fails under load (TE-M3; Medium; not U4d2's).** `l2::delay::tests::a_shutdown_s_never_
  completes_ends_at_its_deadline_without_spinning` failed once in the test seat's counted run (bites build) and passed
  on the immediate re-run; it failed the same way in U3c. It reads no U4d2 file and is in no crate U4d2 changes, so it
  did not stop the merge (B2.3's rule); its cause is a planned change of the main session's before U4r's merge (TE2-5
  (c)). Since FX-10, a failed step's whole output is kept in a file, so the next failure keeps its panic.
- **Process slips, none weakening the evidence.** The builder judged an unnamed check.test.mjs failure inside its step
  instead of stopping (its own new rule repeated a sentence the test pins once; the test is unchanged and still bites:
  TP9); three early `npt-lab hash` runs took the shared lock where `-x` was asked (none a reported result; each value
  re-checked under `-x`); the rust seat built one module in a scratch target off the shared one (38 MB, removed by
  `cargo clean --target-dir`) to keep a SIMD-built module out of it. The builder's brief cited four plan spans, and
  U4r's two lines, at the plan's earlier stamp; the build covered §5 U4d2 whole.
- **Carries.** Met: 29 (Cargo.toml's libm comment; sqrtf is `sqrtss`, sqrt `sqrtsd`, a correction to the carry's own
  words), 41, 52 (only recordings by U4d-F's generator, adb1bab), 65 (the failing tests named; their panics kept since
  FX-10), 66 (<the three v1 datagram hashes and their commands>); 42 does not apply (entry 34 was met by U4a). Added:
  82 and 83 (U4r), C-U7-2 (U7).
- **Open security finding:** SEC-U4D2-3 (Low): the integer-signature rule reads the token `u32`, not the type; the
  limit is written beside the rule, and the type-section read is U7's (C-U7-2). Every other security finding of the
  review is fixed in this change.
```

````

## The carries this packet holds

_Rule: every entry of `.checkpoints/netcode-proof-test/u4s-carries.md` whose text matches `^(29|34|41|42|52|65|66|67|71|72)\. \*\*|U4d2`._

### Carry 34
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 62-69 (sha256 efdd7b6bd358), as they are:_

````text
34. **rust fix2 nits (u4s-fix2-review-rust.md:90-93; the advisor's U4s ruling, condition 2):** "U4a addresses each of
    four check.mjs nits, by a fix listed in its report (a deferral leaves this entry unmet, and entry 42 applies): nothing in check.test.mjs pins `STEPS.doc` or its
    `-D warnings` (as for fmt, clippy and test); the doc step documents the host build only (npt-tcpinfo's non-Linux
    stub is never documented); `lintBothBuilds` says 'both linted' when only the bites build fails; a measured-build
    doc failure's one-line summary is cargo's last line ('warning: build failed, waiting for other jobs to
    finish...'), with the detail in the lines below it. Their home is U4a's brief (wave 1's only check.mjs editor:
    u4-build-plan.md Lanes, wave 1) or, if U4a's report does not list this entry as met, U4d2's (entry 42); never the
    main session while U4a owns check.mjs."
````

### Carry 39
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 169-179 (sha256 efdd7b6bd358), as they are:_

````text
39. **rust fix review nit (u4s-fix-review-rust.md:114-115; the advisor's U4s ruling, condition 2):** "A tick count
    needed outside npt-sim goes into npt-sim as `<name>_at` plus its constant (carry 18's route), and the crate uses it
    by path. Why: check.mjs:627 lets an initializer that merely names `TICK_HZ` through, and nothing pins such a value
    at 60 Hz." (Route, the advisor's F2 ruling, 2026-11-09: the plan names no new tick count for U4e; its counts are
    `FULL_STATE_ACK_TICKS` and `ACK_RING_TICKS`, in npt-sim since U4s, and its deadlines and limits run in ns on the
    injected now. The Input bucket's refill (2 × TICK_HZ a second) and the window's per-tick budget (256 kbps) follow
    carry 6's pattern: the type takes the rate as an argument, production callers pass `npt_sim::TICK_HZ`, a test pins
    each at 30 and 60 Hz, and no const or static outside npt-sim is derived from TICK_HZ. If U4e finds it needs a new
    tick count or a named constant derived from the rate, it writes no local form and records the need in its report;
    the main session adds it by carry 18's route in its own commit once U4d2 has merged, and the site and U4e's merge
    follow it.)
````

### Carry 67
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 401-418 (sha256 efdd7b6bd358), as they are:_

````text
67. **NF-M1 and TF-L2, the ES re-measure RM (u4df-review-netcode.md, u4df-review-test.md; the advisor's U4d-F
    ruling, R1):** "Before the merge of any change that moves the estimate scene's synthetic recording (D-7's ES
    digest) or the World's outcome on it (t_m1's printed line differs from the base's), and at once whenever t_m1
    fails, the change's owner runs RM and its report gives the numbers. (1) Runs: seeds 1 to 9, each a 36-minute ES
    recording (64,800 ticks from tick 1,000,000), replayed and counted as tests/world.rs's `EsRun` counts (an
    `#[ignore]` test run by name, or a scratch crate on npt-sim's public API outside every copy). (2) The floor, run by
    run: each seed's window (minutes 2-11) meets PM-0001-2's four ES bands (shots 68.4-75.6 a second, gameplay events
    9.5-10.5, live projectiles 30.4-33.6 a tick, spawns 20.064-22.176 a second), with 48 soldiers and 15 vehicles on
    every tick and every hull within 1,400 cm of the cluster's centre. (3) Drift: the nine seeds' mean gameplay-event
    rate over each 300 s block from minute 2 to minute 31 (six blocks) lies in 9.5-10.5; the least-squares slope of the
    nine seeds' mean per-minute rate over minutes 2-35 is reported beside it. (4) Every seed's two 300 s halves of its
    window are reported, not judged: a single half's noise is 0.2-0.36 a second at ES's level. (5) `aim_one_in` is
    re-set only if (2) or (3) fails at the current value, then to the value whose nine-seed mean window lies nearest
    10.0 with (2) and (3) passing at it, never on three seeds' in-sample margins; the numbers come to the advisor
    before D-7 or t_m1 is re-pinned for it. (6) If t_m1 fails while (2) and (3) pass, neither the share nor t_m1
    changes: the numbers come to the advisor, who rules on t_m1's form. The band and its 9.5 floor do not move
    (Mustafa, 2026-11-11; carry 52 quotes him). At U4d-F's merge the scene passes RM: windows 9.693-10.171 on seeds
    1-9; blocks 10.096, 9.692, 9.941, 9.901, 9.799 and 9.857 (u4df-review-netcode.md [10]; .gate/u4df/nc-long.log)."
````

### Carry 71
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 419-423 (sha256 efdd7b6bd358), as they are:_

````text
71. **NF-L3, 3b's switch (u4df-review-netcode.md; the advisor's U4d-F ruling, R3):** "The generator's re-home (3b) is
    switched on by a scene's `aim_one_in` above 1 (replay.rs's `synthetic`; scene.rs's `aim_one_in` doc says so), and
    only ES sets it above 1. A change that would let another scene set `aim_one_in` above 1, or set ES's to 1, first
    gives the re-home its own pub(crate) scene knob (its period, 0 for none), in that same change and with the
    advisor's approval of its symbols. Until then no knob is added."
````

### Carry 72
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 424-428 (sha256 efdd7b6bd358), as they are:_

````text
72. **TF-L1, a_m1's non-vacuity (u4df-review-test.md; the advisor's U4d-F ruling, R3):** "a_m1
    (tests/world.rs:940) is not vacuous only because SS seed 5's 1,200 ticks hold about two deaths (58 dead
    soldier-ticks: the test-engineer's U4d-F probe), and two generator-only plants took them away. Before the merge of
    any change that moves SS's synthetic recording, a_m1 is made independent of the recording's draws: it scripts its
    deaths through its own inputs, or runs long enough for at least 10, and asserts that count by name."
````

### Carry 29
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 501-504 (sha256 efdd7b6bd358), as they are:_

````text
29. **rust 5 (nit):** "The workspace Cargo.toml's libm comment says `arch` gives 'hardware sqrt, floor, ceil, trunc and
    rint'. Per the rust review (libm 0.2.16 configure.rs:82-101, arch/x86/fma.rs:9-27) it gives sqrt and sqrtf (x86-64
    `sqrtsd`) and fma and fmaf with run-time CPU dispatch on x86-64, and nothing on wasm32 stable; all correctly
    rounded. Reword it at the next edit of that file."
````

### Carry 41
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 505-507 (sha256 efdd7b6bd358), as they are:_

````text
41. **proto's bites comment (the advisor's U4s ruling, condition 2):** "At the next edit of crates/proto/Cargo.toml
    (U4d2 or later), reword its bites comment (:13-14), which still says the grid's bites are chosen through its
    config: after carry 2, npt-proto's bites are its own."
````

### Carry 42
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 508-510 (sha256 efdd7b6bd358), as they are:_

````text
42. **entry 34's second home (the advisor's U4s ruling, condition 2):** "If U4a's report does not list entry 34 as
    met, U4d2's brief carries entry 34's text word for word, U4d2 in place of U4a (U4d2 owns check.mjs in wave 2:
    u4-build-plan.md Lanes)."
````

### Carry 52
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 511-521 (sha256 efdd7b6bd358), as they are:_

````text
52. **N2-M1, Medium (u4d-round2-netcode.md; Mustafa, 2026-11-11: "The event-rate drift in the estimate scene: fix the
    scene before U4d2 pins the recordings. The 9.5 floor (PM-0001-2) does not move."):** "Before U4d2 pins any
    recording or hash, the estimate scene's event-rate drift is fixed: the recording's dead reckoning (replay.rs's
    generator) ignores crouch, dropped commands, scripted move speeds, blocked moves and deaths, so the event rate
    falls over a run (per 50 s window: 13.4, 11.0, 10.1, 9.4, 9.7, 8.7 a second at 1 in 10; the test-engineer's probe
    X2, seed 2 at 1 in 11: 7.578 a second over 600 s). The fix and the re-calibration of `aim_one_in` are measured on
    the judged window (a 10-minute measured window after warm-up, ADR 0001:1600; 3 seeds), not on T-M1's 300 s from
    tick 0. PM-0001-2's floor of 9.5 events a second does not move. The form, the measurement and the owner are the
    advisor's U4d close ruling (advisor-u4d-close.md, B1.2 and B1.3): the main session's change U4d-F lands the fix,
    with ES's vehicles inside the fight (N2-M2), before U4d2's brief is final. U4d2 pins only recordings written by
    U4d-F's `Recording::GENERATOR` or later, and ES's hashes only from U4d-F's scene; its report names U4d-F's commit."
````

### Carry 65
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 522-528 (sha256 efdd7b6bd358), as they are:_

````text
65. **TE2-5's check.mjs half (the advisor's U4j round-2 ruling, B2.3):** "check.mjs's test step names every failing
    test. When a cargo test run fails, the lines `main` prints under its FAIL hold each failing test's name from
    libtest's `failures:` list (on the run's stdout) beside the summary line. Today `run` joins stdout before stderr
    (check.mjs:1402) and `main` prints a failed step's last 40 lines (:1649), so the names never show (the U4j
    fixer's check.mjs run 2: u4j-build.md:322-325). A check.test.mjs self-test feeds a recorded failing run and
    asserts the name is printed. The main session may make this change sooner, in its own commit; U4d2's report then
    lists entry 65 as met."
````

### Carry 66
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 529-539 (sha256 efdd7b6bd358), as they are:_

````text
66. **U4c's netcode F2, Medium (u4c-review-netcode.md:53-54; L-c8's judged re-run, u4c-lc8-rerun.md; the advisor's U4e
    brief ruling, F2-ROUTE):** "npt-lab's probe also writes the SHA-256 of (v1's tag + the v2 fate log's datagram
    entries), the hash §5 U4c's L-c8 compares with U2's committed 913db8e9…, feabf2f0… and b8a04f2f… for P2, B1 and P4
    seed 1, beside the whole log's SHA-256: in the run log's header and in its summary line, so that -3's script
    (ops/runs/c-u2-pm-0001-3.mjs) records it unchanged. It reads the log through npt-l1's public
    `FateLog::for_each_canonical` and edits no crates/l1 file; v1's tag and the datagram entry's form are those U4c's
    test `v1s_tag_and_the_v2_logs_datagram_entries_hash_as_v1s_log_did` reads (l1/tests/l1.rs:481-532). U4d2's
    report shows the three values from probe runs of those three runs (the -3 set's own arguments), each equal to
    U2's, with the commands it ran. Until then L-c8's hash half rests on U4c's scratch harness through equal
    whole-log hashes (u4c-lc8-rerun.md). If U4d2's brief is final without this entry, U4r's brief takes it word for
    word, U4r in place of U4d2."
````

## The carries this packet left out

Ids: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 30, 31, 32, 33, 35, 36, 37, 38, 40, 43, 44, 45, 46, 47, 48, 49, 50, 51, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 68, 69, 70, 73, 74, 75, 76, 77, 78, 79, 80, 81.
