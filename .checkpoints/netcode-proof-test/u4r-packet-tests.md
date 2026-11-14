# check.test.mjs (check.mjs's self-tests): git diff --no-index spikes/netcode-proof/check.test.mjs .gate/u4r-snap1/spikes/netcode-proof/check.test.mjs
diff --git a/spikes/netcode-proof/check.test.mjs b/.gate/u4r-snap1/spikes/netcode-proof/check.test.mjs
index d1d2039..782c33e 100644
--- a/spikes/netcode-proof/check.test.mjs
+++ b/.gate/u4r-snap1/spikes/netcode-proof/check.test.mjs
@@ -5,7 +5,7 @@ import assert from "node:assert/strict";
 import { spawnSync } from "node:child_process";
 import { createHash } from "node:crypto";
 import { mkdirSync, mkdtempSync, readFileSync, readdirSync, readlinkSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
-import { tmpdir } from "node:os";
+import { homedir, tmpdir } from "node:os";
 import { dirname, join, relative } from "node:path";
 import { test } from "node:test";
 import { fileURLToPath } from "node:url";
@@ -34,6 +34,7 @@ import {
   buildFlagProblems,
   buildInputProblems,
   builtArtifact,
+  cargoConfigDirs,
   cargoConfigProblems,
   cargoCounts,
   clippyTwinProblems,
@@ -58,6 +59,7 @@ import {
   readScan,
   referenceProblems,
   roomFloatProblems,
+  run,
   rustflagSources,
   SIM_DEPENDENCIES,
   simFeatureCheck,
@@ -1032,7 +1034,8 @@ test("another rustc, a workspace wrapper, a config include and a test runner are
   };
   assert.deepEqual(cargoConfigProblems(normal, []), []);
   const config = (text) => cargoConfigProblems({}, [{ path: "c.toml", text }]);
-  assert.deepEqual(config('[build]\nrustc-wrapper = "sccache"\ntarget-dir = "t"\n[target.x86_64-unknown-linux-gnu]\nlinker = "cc"\n'), []);
+  // A target table passes; its `linker` key no longer does (carry 85 (a): the test below).
+  assert.deepEqual(config('[build]\nrustc-wrapper = "sccache"\ntarget-dir = "t"\n[target.x86_64-unknown-linux-gnu]\nrustflags = []\n'), []);
   // C2R-8: another compiler, or a wrapper around the workspace crates' rustc calls.
   for (const k of ["RUSTC", "CARGO_BUILD_RUSTC", "RUSTC_WORKSPACE_WRAPPER", "CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER"]) {
     assert.deepEqual(cargoConfigProblems({ ...normal, [k]: "/opt/other/rustc" }, []), [
@@ -1098,6 +1101,44 @@ test("RUSTC_BOOTSTRAP, cargo's channel override, and a manifest's cargo-features
   assert.deepEqual(manifestKeyProblems(own), []);
 });
 
+test("carry 85 (a), (b) and (d): a linker for any target and a profile outside the manifests are refused, naming the source and value; CARGO_HOME is read as cargo reads it; this container's environment and the spike's own config pass", () => {
+  // (a) A linker: an environment variable whose name ends in _LINKER, or a `linker` key in any table or form.
+  assert.deepEqual(cargoConfigProblems({ PATH: "/bin", CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER: "/opt/ld" }, []), [
+    'CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER in the environment sets a linker ("/opt/ld"): its arguments can rewrite the target_features section the Wasm loader reads',
+  ]);
+  const config = (text) => cargoConfigProblems({}, [{ path: "c.toml", text }]);
+  const linker = (v) => `c.toml has a key \`linker\` (${v}): it sets a linker, whose arguments can rewrite the target_features section the Wasm loader reads`;
+  assert.deepEqual(config('[target.wasm32-unknown-unknown]\nlinker = "rust-lld"\n'), [linker('"rust-lld"')]);
+  assert.deepEqual(config('target.x86_64-unknown-linux-gnu.linker = "cc"\n'), [linker('"cc"')]);
+  assert.deepEqual(config('[host]\n"linker" = "cc"\n'), [linker('"cc"')]);
+  // (b) A profile: CARGO_PROFILE_* in the environment; a `profile` table, quoted table, dotted key or key in a config.
+  assert.deepEqual(cargoConfigProblems({ CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS: "false" }, []), [
+    "CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS in the environment sets a profile key (\"false\"): it can drop npt-sim's -C overflow-checks=on, which plan §4.5 asks for in every profile",
+  ]);
+  const profile = "c.toml has a `profile` key or table: it can drop npt-sim's -C overflow-checks=on, which plan §4.5 asks for in every profile";
+  for (const text of [
+    "[profile.release.package.npt-sim]\noverflow-checks = false\n",
+    "profile.release.overflow-checks = false\n",
+    '["profile".dev]\nopt-level = 1\n',
+    "profile = { release = { debug = true } }\n",
+  ]) {
+    assert.deepEqual(config(text), [profile], text);
+  }
+  // Neither name in a string value or inside another key's name.
+  assert.deepEqual(config('[build]\ntarget-dir = "profile/linker"\nprofiles = 1\nlinkers = 2\n'), []);
+  // This container's environment and the spike's own config, as the deps step reads them, pass.
+  assert.deepEqual(cargoConfigProblems(process.env, [{ path: ".cargo/config.toml", text: readFileSync(join(HERE, ".cargo", "config.toml"), "utf8") }]), []);
+  // (d) CARGO_HOME as cargo's home crate reads it: an empty one as unset, a relative one against the folder cargo runs
+  // in, read through check.mjs's own function (cargo never runs with these values: u4r-brief.md section 4 rule 7).
+  const start = join(HERE, "crates", "lab");
+  const home = (env) => cargoConfigDirs(start, env).at(-1);
+  assert.equal(home({}), join(homedir(), ".cargo"));
+  assert.equal(home({ CARGO_HOME: "" }), join(homedir(), ".cargo"), "an empty CARGO_HOME is unset");
+  assert.equal(home({ CARGO_HOME: "../h" }), join(HERE, "crates", "h"), "a relative CARGO_HOME, against the folder cargo runs in");
+  assert.equal(home({ CARGO_HOME: "/opt/cargo" }), "/opt/cargo");
+  assert.ok(cargoConfigDirs(start, {}).includes(join(HERE, ".cargo")), "and every folder above it");
+});
+
 /** The tick grep's message for a tick-count name (not a derived one) set from `init` at `at` (u4-build-plan.md §4.3). */
 const fromLiteral = (at, name, init) =>
   `${at} sets ${name} from \`${init}\`: a tick-counted value is a const fn of npt_sim::TICK_HZ, never a literal ` +
@@ -2024,8 +2065,9 @@ test("npt-tcpinfo and npt-hash-wasm, the deny-level exceptions, have no build sc
   // Its second: `use std::include as inc;`, and `inc!(concat!(env!("OUT_DIR"), "/gen.rs"));` in begin's body, which the
   // include! rule cannot see by its name; and a raw identifier's form in npt-tcpinfo.
   const includeAt = (where, name) =>
-    `${where} names \`include\` other than as an include!, include_str! or include_bytes! call (a renamed include!, as ` +
-    `\`use std::include as inc;\`, hides the call from the include! rule), which ${name} may not: ${why}`;
+    `${where} names \`include\` other than directly before \`!\` and an opening bracket, as an include!, include_str! or ` +
+    `include_bytes! call is written (a renamed include!, as \`use std::include as inc;\`, or one a macro builds, as ` +
+    `\`m!(include!)\`, hides the call from the include! rule), which ${name} may not: ${why}`;
   const renamed = `use std::include as inc;\n${HASH_WASM_LIB.replace(BEGIN_HEAD, `${BEGIN_HEAD}    inc!(concat!(env!("OUT_DIR"), "/gen.rs"));\n`)}`;
   assert.deepEqual(unsafeWith((f) => (f[F] = renamed)), [includeAt(`${F}:1`, "npt-hash-wasm")]);
   assert.deepEqual(unsafeWith((f) => (f["crates/tcpinfo/src/lib.rs"] += "use core::r#include as i;\n")), [includeAt("crates/tcpinfo/src/lib.rs:8", "npt-tcpinfo")]);
@@ -2034,6 +2076,15 @@ test("npt-tcpinfo and npt-hash-wasm, the deny-level exceptions, have no build sc
   const calls = 'include!("k.rs");\nconst V: &str = include_str!("../x.txt");\nconst B: &[u8] = include_bytes!("../y.bin");\n// include\nconst S: &str = "include";\n';
   assert.deepEqual(unsafeWith((f) => (f[F] = `${HASH_WASM_LIB}${calls}`)), []);
   assert.deepEqual(unsafeWith((f) => (f["crates/net/src/lib.rs"] += "use std::include as inc;\n")), []);
+  // Carry 85 (c): `include` must be directly before `!` and an opening bracket: `m!(include!)` (the bare name handed to
+  // a macro, which can build the call) and `include ! (...)` are refused; `include!{...}` and `include![...]` pass.
+  assert.deepEqual(unsafeWith((f) => (f[F] = `${HASH_WASM_LIB}m!(include!);\n`)), [includeAt(`${F}:${HASH_WASM_LIB.split("\n").length}`, "npt-hash-wasm")]);
+  assert.deepEqual(unsafeWith((f) => (f["crates/tcpinfo/src/lib.rs"] += 'const K: &str = include ! ("k.rs");\n')), [includeAt("crates/tcpinfo/src/lib.rs:8", "npt-tcpinfo")]);
+  assert.deepEqual(unsafeWith((f) => (f[F] = `${HASH_WASM_LIB}include!{"k.rs"}\ninclude!["k.rs"];\n`)), []);
+  // Carry 85 (c), every crate: an attribute whose text starts with a metavariable, either form; none in the spike.
+  const metaAt = (where) => `${where} has an attribute whose text starts with a metavariable (\`#[$\` or \`#![$\`): a macro could build a #[path] from a \`meta\` fragment that this scan does not read`;
+  const meta = "macro_rules! m {\n    ($p:meta) => {\n        #[$p]\n        mod x;\n    };\n}\nmacro_rules! n {\n    ($p:meta) => {\n        # ! [ $p ]\n    };\n}\n";
+  assert.deepEqual(unsafeWith((f) => (f["crates/net/src/lib.rs"] += meta)), [metaAt("crates/net/src/lib.rs:6"), metaAt("crates/net/src/lib.rs:12")]);
   // A `build` key in the manifest, in either form cargo reads; a custom-build target in cargo metadata.
   const manifestKey = (crate) => `crates/${crate}/Cargo.toml has a key \`build\`: a build script in npt-${crate}, which may have none: ${why}`;
   assert.deepEqual(unsafeWith((f) => (f["crates/hash-wasm/Cargo.toml"] = HASH_WASM_MANIFEST.replace("\n\n[lints.rust]", '\nbuild = "gen.rs"\n\n[lints.rust]'))), [manifestKey("hash-wasm")]);
@@ -2532,17 +2583,27 @@ const cmpLine = (a, b, { compared = 27_000, mismatching = 0, rec, tick, field, c
 const NOT_CATCHABLE_SCAN = "# std-sin's scan\nnot catchable at this site on x86-64 and wasm32\nscanned on: the test's tree\nlibc: ldd (test) 2.39\n";
 /** A scan with two catches, listed out of the manifest's order (ES-s2 comes before TC-s3 there, so ES-s2's is armed). */
 const CATCH_SCAN = "catch TC-s3 4294962800 vehicle[0].pos.y\ncatch ES-s2 1000123 vehicle[3].pos.y\nlibc: ldd (Ubuntu GLIBC 2.39-0ubuntu8.6) 2.39\n";
+/** A scan whose first catch in the manifest's order is on SS-s1 before HASH_MAP_ARM, as the committed scan's is. */
+const CATCH_SCAN_SS = "catch ES-s2 1000123 vehicle[3].pos.y\ncatch SS-s1 2 vehicle[8].pos.y\nlibc: ldd (Ubuntu GLIBC 2.39-0ubuntu8.6) 2.39\n";
 
 /**
  * A recorded wasm step: a runner that answers each command as cargo, npt-lab and the loader do (an equal run, the
- * hash-map bite caught natively and not catchable on wasm32, std-sin by `scan`), `over[kind]` replacing one answer,
- * and an in-memory io (the manifest and the scan data read, the files each command writes known to exist).
+ * hash-map bite caught natively and not catchable on wasm32, std-sin by `scan`: caught and re-shown at its first catch
+ * in the manifest's order), `over[kind]` replacing one answer (`ms`: the call's time, 1,500 by default), and an
+ * in-memory io (the manifest `manifestPath` holding `recs`, and the scan data, read; the files each command writes
+ * known to exist). `options` goes to wasmStep as its fifth argument.
  */
-function wasmWorld({ over = {}, scan = NOT_CATCHABLE_SCAN } = {}) {
+function wasmWorld({ over = {}, scan = NOT_CATCHABLE_SCAN, recs = RECS, manifestPath = RECORDINGS_MANIFEST, options } = {}) {
   const calls = [];
   const files = new Set();
   const copies = [];
-  const manifest = { recordings: RECS.map((r) => ({ name: r.name, first_tick: r.first, ticks: 9_000 })) };
+  const manifest = { recordings: recs.map((r) => ({ name: r.name, first_tick: r.first, ticks: 9_000 })) };
+  const all = { compared: 9_000 * recs.length, counts: recs.map((r) => `${r.name} 9000`).join(", ") };
+  const order = recs.map((r) => r.name);
+  const sc = readScan(scan)
+    .catches.filter((c) => order.includes(c.recording))
+    .sort((x, y) => order.indexOf(x.recording) - order.indexOf(y.recording))[0] ?? { recording: "ES-s2", tick: 1_000_123, field: "vehicle[3].pos.y" };
+  const sinAt = { compared: 9_000, mismatching: 8_000, rec: sc.recording, tick: sc.tick, field: sc.field, counts: `${sc.recording} 9000` };
   const arg = (args, k) => (args.includes(k) ? args[args.indexOf(k) + 1] : null);
   const kindOf = (cmd, args) => {
     if (cmd === "cargo") return `build ${args.includes("npt-hash-wasm") ? "wasm" : "lab"}${args.includes("bites") ? " bites" : ""}`;
@@ -2557,6 +2618,7 @@ function wasmWorld({ over = {}, scan = NOT_CATCHABLE_SCAN } = {}) {
     const where = arg(args, "--out") ?? arg(args, "--ours");
     const area = where.startsWith(WS.hm) ? "hash-map " : where.startsWith(WS.sin) ? "std-sin " : "";
     if (args[1] !== "compare") return `${area}legs`;
+    if (arg(args, "--theirs-name") === "std-sin native leg 1") return "std-sin re-show";
     if (area === "hash-map ") return `hash-map compare ${arg(args, "--theirs-name") === "Wasm leg 2" ? "ww" : "nw"}`;
     return `${area}compare${!area && args.includes("--theirs-dump") ? " field" : ""}`;
   };
@@ -2572,10 +2634,10 @@ function wasmWorld({ over = {}, scan = NOT_CATCHABLE_SCAN } = {}) {
       legs: {
         stdout:
           "wrote the header-only recordings vector-SS, vector-ES, vector-CO, vector-TC (seed 1, first tick 0, no tick) for h2\n" +
-          `${RECS.map((r) => `regenerated ${r.name}: 100 bytes, sha256 ab, as the manifest's\n`).join("")}${cmpLine("native leg 1", "native leg 2")}\n`,
-        writes: [...RECS.map((r) => join(WS.rec, `${r.name}.rec`)), ...["SS", "ES", "CO", "TC"].map((s) => join(WS.rec, `vector-${s}.rec`))],
+          `${recs.map((r) => `regenerated ${r.name}: 100 bytes, sha256 ab, as the manifest's\n`).join("")}${cmpLine("native leg 1", "native leg 2", all)}\n`,
+        writes: [...recs.map((r) => join(WS.rec, `${r.name}.rec`)), ...["SS", "ES", "CO", "TC"].map((s) => join(WS.rec, `vector-${s}.rec`))],
       },
-      compare: { stdout: `${cmpLine("native leg 1", "Wasm leg 1")}\n` },
+      compare: { stdout: `${cmpLine("native leg 1", "Wasm leg 1", all)}\n` },
       vectors: { stdout: "h2: CO's wasm32 dump is the committed vector (260 words, digest b432ac332054c77b); every scene's initial dump and hash are equal natively and on wasm32 (SS 6488, ES 1944, CO 260, TC 340)\n" },
       bites: { stdout: "1 PM-0004-21/hash-map\n2 PM-0004-21/std-sin\n3 PM-0001-41/server-speed-1pct\n4 PM-0001-41/server-quant-differs\n" },
       "hash-map legs": {
@@ -2597,11 +2659,9 @@ function wasmWorld({ over = {}, scan = NOT_CATCHABLE_SCAN } = {}) {
           `Wasm leg 2 digest 3333333333333333 (${HASH_MAP_FIELD} at words [6488, 6489] of its dump at SS-s1 tick 1234)\n` +
           `${cmpLine("Wasm leg 1", "Wasm leg 2", { compared: 9_000, counts: "SS-s1 9000" })}\n`,
       },
-      "std-sin legs": { stdout: `${cmpLine("native leg 1", "native leg 2", { compared: 9_000, counts: "ES-s2 9000" })}\n` },
-      "std-sin compare": {
-        status: 1,
-        stdout: `${cmpLine("native leg 1", "Wasm leg 1", { compared: 9_000, mismatching: 8_000, rec: "ES-s2", tick: 1_000_123, field: "vehicle[3].pos.y", counts: "ES-s2 9000" })}\n`,
-      },
+      "std-sin legs": { stdout: `${cmpLine("native leg 1", "native leg 2", { compared: 9_000, counts: `${sc.recording} 9000` })}\n` },
+      "std-sin compare": { status: 1, stdout: `${cmpLine("native leg 1", "Wasm leg 1", sinAt)}\n` },
+      "std-sin re-show": { status: 1, stdout: `${cmpLine("clean native leg 1", "std-sin native leg 1", sinAt)}\n` },
     }[kind];
     if (said) return said;
     // The loader's runs.
@@ -2616,7 +2676,7 @@ function wasmWorld({ over = {}, scan = NOT_CATCHABLE_SCAN } = {}) {
     copy: (from, to) => copies.push([from, to]),
     exists: (p) => files.has(p),
     read: (p) => {
-      if (p === RECORDINGS_MANIFEST) return JSON.stringify(manifest);
+      if (p === manifestPath) return JSON.stringify(manifest);
       if (p === STD_SIN_SCAN) return scan;
       throw new Error(`ENOENT ${p}`);
     },
@@ -2630,11 +2690,11 @@ function wasmWorld({ over = {}, scan = NOT_CATCHABLE_SCAN } = {}) {
     const base = answer(kind, args);
     const r = over[kind] ? { ...base, ...over[kind](base, args) } : base;
     for (const w of r.writes ?? []) files.add(w);
-    clock += 1_500;
+    clock += r.ms ?? 1_500;
     const status = r.timedOut ? null : (r.status ?? 0);
     return { ok: status === 0, status, stdout: r.stdout ?? "", out: `${r.stdout ?? ""}${r.stderr ?? ""}`, timedOut: Boolean(r.timedOut) };
   };
-  const result = wasmStep(runner, io, () => clock);
+  const result = wasmStep(runner, io, () => clock, undefined, options);
   return { ...result, calls, copies, kinds: calls.map((c) => c.kind) };
 }
 
@@ -2695,16 +2755,37 @@ test("the wasm step's builds, legs and comparisons, in order; a clean run passes
   assert.equal((last.match(/ 1\.5(?:,|$)/g) ?? []).length, w.calls.length, "each timed call's seconds");
   assert.deepEqual(STEPS.wasm.length, 0, "STEPS.wasm(runner) defaults its runner");
   assert.deepEqual(Object.keys(STEPS), ["fmt", "clippy", "clippy-wasm32", "doc", "test", "deps", "wasm", "refs", "marks", "self", "redact"]);
-  // The std-sin catch path: armed at the scan's first catch in the manifest's order, native leg 1 and Wasm leg 1.
-  const c = wasmWorld({ scan: CATCH_SCAN });
+  // The std-sin catch path: armed at the scan's first catch in the manifest's order, native leg 1 and Wasm leg 1, then
+  // the re-show (carry 83 (c)): the hash-map half's native leg 1, clean before its arming tick, against the bitten
+  // native leg, its field named by compare's own clean replay against the bitten leg's dump.
+  const c = wasmWorld({ scan: CATCH_SCAN_SS });
   assert.equal(c.ok, true, c.out);
-  const sinArgs = "--bite PM-0004-21/std-sin --arm 1000123";
-  assert.deepEqual(c.calls.slice(-3).map((x) => x.line), [
-    `${LAB_BITES} hash ${manifestArgs} --out ${WS.sin} --only ES-s2 ${sinArgs} --dump-at 1000123`,
-    loader(WASM_B, RECS[1], join(WS.sin, "wasm-1.lines"), ` --bite 2 --arm 1000123 --dump-at 1000123 --dump-out ${join(WS.sin, "wasm-1.dump")}`),
-    `${LAB_BITES} hash compare --ours ${join(WS.sin, "native-1.lines")} --theirs ${join(WS.sin, "wasm-1.lines")} ${manifestArgs} --ours-dump ${join(WS.sin, "native-1.dump")} --theirs-dump ${join(WS.sin, "wasm-1.dump")} ${sinArgs} --on ES-s2 --ours-name native leg 1 --theirs-name Wasm leg 1`,
+  const sinArgs = "--bite PM-0004-21/std-sin --arm 2";
+  assert.deepEqual(c.calls.slice(-4).map((x) => x.line), [
+    `${LAB_BITES} hash ${manifestArgs} --out ${WS.sin} --only SS-s1 ${sinArgs} --dump-at 2`,
+    loader(WASM_B, RECS[0], join(WS.sin, "wasm-1.lines"), ` --bite 2 --arm 2 --dump-at 2 --dump-out ${join(WS.sin, "wasm-1.dump")}`),
+    `${LAB_BITES} hash compare --ours ${join(WS.sin, "native-1.lines")} --theirs ${join(WS.sin, "wasm-1.lines")} ${manifestArgs} --ours-dump ${join(WS.sin, "native-1.dump")} --theirs-dump ${join(WS.sin, "wasm-1.dump")} ${sinArgs} --on SS-s1 --ours-name native leg 1 --theirs-name Wasm leg 1`,
+    `${LAB_BITES} hash compare --ours ${join(WS.hm, "native-1.lines")} --theirs ${join(WS.sin, "native-1.lines")} ${manifestArgs} --theirs-dump ${join(WS.sin, "native-1.dump")} --ours-name clean native leg 1 --theirs-name std-sin native leg 1`,
   ]);
-  assert.ok(lastLine(c.out).includes("std-sin: caught at ES-s2 tick 1000123, vehicle[3].pos.y, as the scan found (the scan's other catches: TC-s3 tick 4294962800)"), lastLine(c.out));
+  assert.ok(
+    lastLine(c.out).includes("std-sin: caught at SS-s1 tick 2, vehicle[8].pos.y, as the scan found; re-shown against a clean bites-build native leg (the scan's other catches: ES-s2 tick 1000123)"),
+    lastLine(c.out),
+  );
+  // Carry 83 (c)'s failing cases: a catch not on the manifest's first recording before HASH_MAP_ARM (no clean leg
+  // covers it), and a re-show whose first difference is not the scan's.
+  const notFirst = "std-sin: the scan's catch ES-s2 tick 1000123 is not on the manifest's first recording (SS-s1) before the hash-map bite's arming tick 1234";
+  for (const scan of [CATCH_SCAN, "catch SS-s1 1234 vehicle[8].pos.y\n"]) {
+    const w2 = wasmWorld({ scan });
+    assert.equal(w2.ok, false, w2.out);
+    assert.ok(tail(w2.out).includes(scan === CATCH_SCAN ? notFirst : "the scan's catch SS-s1 tick 1234 is not on the manifest's first recording"), tail(w2.out));
+    assert.ok(!w2.kinds.includes("std-sin re-show"), "no re-show runs");
+  }
+  const off = wasmWorld({
+    scan: CATCH_SCAN_SS,
+    over: { "std-sin re-show": () => ({ status: 1, stdout: `${cmpLine("clean native leg 1", "std-sin native leg 1", { compared: 9_000, mismatching: 9, rec: "SS-s1", tick: 3, field: "vehicle[8].pos.y", counts: "SS-s1 9000" })}\n` }) },
+  });
+  assert.equal(off.ok, false, off.out);
+  assert.ok(tail(off.out).includes("std-sin: the re-show (the bitten native leg against a clean bites-build native leg) first differs at SS-s1 tick 3, field vehicle[8].pos.y, not at the scan's SS-s1 tick 2, vehicle[8].pos.y"), tail(off.out));
 });
 
 test("the wasm step fails on every outcome that is not an equal, counted run, naming what failed within the lines main prints (U4d2, 2.4 (a))", () => {
@@ -2781,19 +2862,104 @@ test("the wasm step fails on every outcome that is not an equal, counted run, na
   fails(wasmWorld({ over: { "build lab": () => ({ stdout: "" }) } }), "the release build of npt-lab (x86-64, measured) names no bin artifact in cargo's JSON messages");
 });
 
-test("the wasm step gives each call but its four cargo builds 300 s, and a call stopped by it fails the step naming the call and the limit (U4d2 round 1, FX-7: INF-4)", () => {
+test("the wasm step's calls but its four cargo builds share one 300 s deadline from the first of them, each given what is left; a call stopped by it fails the step naming the call (carry 84 (a); before it U4d2 round 1, FX-7: INF-4)", () => {
   const w = wasmWorld();
   assert.equal(w.ok, true, w.out);
-  for (const c of w.calls) assert.equal(c.limit, c.kind.startsWith("build ") ? 0 : 300_000, `${c.kind}: ${c.limit}`);
+  // The fake clock moves 1,500 ms a call: the i-th call after the builds is given 300,000 - 1,500 i ms.
+  const timedCalls = w.calls.filter((c) => !c.kind.startsWith("build "));
+  timedCalls.forEach((c, i) => assert.equal(c.limit, 300_000 - 1_500 * i, `${c.kind}: ${c.limit}`));
   assert.equal(w.calls.filter((c) => c.limit === 0).length, 4, "the four cargo builds alone have no limit");
-  const stopped = (label) => `${label}: still running after 300 s, so it was stopped (the wasm step's limit on each call but its four cargo builds)`;
-  // A Wasm leg that hangs, and the native legs' call: each fails the step, named within main's 40 lines.
+  const stopped = (label, left) =>
+    `${label}: still running at the step's deadline (300 s shared by every call but its cargo builds, from the first of them; ${left} s were left for it), so it was stopped with every process it started`;
+  // A Wasm leg that hangs: the step fails naming it, within main's 40 lines.
   const hung = wasmWorld({ over: { "wasm ES-s2": () => ({ timedOut: true, stdout: "" }) } });
   assert.equal(hung.ok, false, hung.out);
-  assert.ok(tail(hung.out).split("\n").includes(stopped("Wasm ES-s2")), tail(hung.out));
-  const legs = wasmWorld({ over: { legs: () => ({ timedOut: true, stdout: "", writes: [] }) } });
+  assert.ok(tail(hung.out).split("\n").includes(stopped("Wasm ES-s2", "297.0")), tail(hung.out));
+});
+
+test("carry 84 (c), the shared deadline with a fake runner and clock: a first call that takes the whole limit is stopped, and the next calls are not started, each named as skipped", () => {
+  // `npt-lab hash`, the first call after the builds, runs the whole 300 s and is stopped (it had regenerated the
+  // recordings first).
+  const w = wasmWorld({ over: { legs: (b) => ({ timedOut: true, ms: 300_000, stdout: "", writes: b.writes }) } });
+  assert.equal(w.ok, false, w.out);
+  assert.deepEqual(w.kinds, ["build lab", "build lab bites", "build wasm", "build wasm bites", "legs"], "nothing starts after the stopped call");
+  const said = w.out.split("\n");
+  assert.ok(said.some((l) => l.startsWith("npt-lab hash: still running at the step's deadline") && l.includes("300.0 s were left for it")), w.out);
+  for (const label of ["Wasm SS-s1", "Wasm ES-s2", "Wasm TC-s3", "npt-lab bites"]) {
+    assert.ok(
+      said.includes(`${label}: not started (skipped): npt-lab hash was stopped at the step's deadline (300 s shared by every call but its cargo builds, from the first of them)`),
+      `${label}\n${w.out}`,
+    );
+  }
+  // A call that ends exactly at the deadline is not stopped, but the time is spent: the next is skipped as such.
+  const spent = wasmWorld({ over: { legs: (b) => ({ ...b, ms: 300_000 }) } });
+  assert.equal(spent.ok, false, spent.out);
+  assert.ok(spent.out.split("\n").includes("Wasm SS-s1: not started (skipped): the step's deadline had passed (300 s shared by every call but its cargo builds, from the first of them)"), spent.out);
+  assert.equal(spent.kinds.filter((k) => !k.startsWith("build ")).length, 1, "only the first call ran");
+});
+
+test("carry 84 (b) and (c): run() stops a timed call with every process it started (a child that starts a grandchild and sleeps, limit 200 ms)", () => {
+  const child =
+    'console.log("child " + process.pid); const g = require("node:child_process").spawn(process.execPath, ["-e", "setTimeout(() => {}, 30000)"], { stdio: "inherit" }); console.log("grandchild " + g.pid); setTimeout(() => {}, 30000);';
+  const t0 = Date.now();
+  const r = run(process.execPath, ["-e", child], HERE, process.env, 200);
+  const ms = Date.now() - t0;
+  assert.equal(r.timedOut, true, `run() reports the timeout: ${JSON.stringify(r)}`);
+  assert.ok(ms < 10_000, `run() returned after ${ms} ms, not the child's 30 s`);
+  const pids = ["child", "grandchild"].map((k) => Number(r.out.match(new RegExp(`^${k} (\\d+)$`, "m"))?.[1]));
+  assert.ok(pids.every((p) => p > 0), `both started and said their pid: ${r.out}`);
+  // Left means still running: a process gone, or a zombie (dead, its exit status not yet collected by its new parent),
+  // is not left. SIGKILL is delivered at once, but /proc may show the state a moment later.
+  const running = (pid) => {
+    try {
+      return readFileSync(`/proc/${pid}/stat`, "utf8").split(") ")[1][0] !== "Z";
+    } catch {
+      return false;
+    }
+  };
+  const until = Date.now() + 2_000;
+  while (pids.some(running) && Date.now() < until);
+  assert.deepEqual(pids.map(running), [false, false], `child ${pids[0]}, grandchild ${pids[1]}`);
+});
+
+test("carry 83 (b): the clean half requires exit 0 from npt-lab hash and from its compare, whatever their lines say", () => {
+  const legs = wasmWorld({ over: { legs: (b) => ({ ...b, status: 1 }) } });
   assert.equal(legs.ok, false, legs.out);
-  assert.ok(tail(legs.out).split("\n").includes(stopped("npt-lab hash")), tail(legs.out));
+  assert.ok(tail(legs.out).split("\n").includes("npt-lab hash: exit 1, not 0 (the clean half requires exit 0 as well as its lines)"), tail(legs.out));
+  const compare = wasmWorld({ over: { compare: (b) => ({ ...b, status: 1 }) } });
+  assert.equal(compare.ok, false, compare.out);
+  assert.ok(
+    tail(compare.out).split("\n").includes("npt-lab hash compare (native leg 1 against Wasm leg 1): exit 1, not 0 (the clean half requires exit 0 as well as its lines)"),
+    tail(compare.out),
+  );
+  // Their lines alone said equal: only the exit code fails each.
+  for (const w of [legs, compare]) assert.equal(w.out.split("\n").filter((l) => /exit 1, not 0/.test(l)).length, 1, w.out);
+});
+
+test("wasmStep's two options (the advisor's U4r build ruling, point 1 (4)): a non-default manifest reaches every npt-lab hash call and the loader's recordings; the bites half off starts no bites-build call", () => {
+  const DET = "crates/lab/det-recordings.json";
+  const recs = [
+    { name: "SS-P2-s1", first: 0 },
+    { name: "ES-P3-s2", first: 1_000_000 },
+  ];
+  const w = wasmWorld({ recs, manifestPath: DET, options: { manifest: DET, bitesHalf: false } });
+  assert.equal(w.ok, true, w.out);
+  const hashCalls = w.calls.filter((c) => c.line.startsWith(`${LAB} hash`) && !c.line.startsWith(`${LAB} hash vectors`));
+  assert.deepEqual(hashCalls.map((c) => c.kind), ["legs", "compare"]);
+  for (const c of hashCalls) assert.ok(c.line.includes(` --manifest ${DET} --dir ${WS.rec}`) && !c.line.includes(RECORDINGS_MANIFEST), c.line);
+  const wasmLegs = w.calls.filter((c) => c.kind.startsWith("wasm "));
+  assert.deepEqual(
+    wasmLegs.map((c) => c.line),
+    recs.map((r) => `node ${LOADER} --wasm ${WASM_M} --rec ${join(WS.rec, `${r.name}.rec`)} --name ${r.name} --first-tick ${r.first} --out ${join(WS.meas, `wasm-${r.name}.lines`)}`),
+  );
+  assert.ok(w.calls.every((c) => !c.line.includes("--features bites") && !c.line.startsWith(LAB_BITES) && !c.line.includes(WASM_B)), "no bites-build call");
+  assert.deepEqual(w.kinds.filter((k) => k.startsWith("build ")), ["build lab", "build wasm"]);
+  const last = lastLine(w.out);
+  assert.ok(last.includes("native leg 1 against Wasm leg 1: 18000 ticks compared, 0 mismatching, equal (SS-P2-s1 9000, ES-P3-s2 9000)"), last);
+  assert.ok(last.includes("hash-map: not run: the bites half is off (wasmStep's bitesHalf option); std-sin: not run: the bites half is off (wasmStep's bitesHalf option)"), last);
+  // The defaults: check.mjs's own manifest and the bites half.
+  const d = wasmWorld();
+  assert.ok(d.calls.some((c) => c.line.startsWith(LAB_BITES)) && d.calls.every((c) => !c.line.includes(DET)));
 });
 
 test("the wasm step judges PM-0004-21's two bites: each caught at its tick and field, or not exercised; anything else fails (U4d2, 2.4 (a) item 6)", () => {

# npt-bots' tests (crates/bots/tests/behaviour.rs, new): git diff --no-index /dev/null .gate/u4r-snap1/spikes/netcode-proof/crates/bots/tests/behaviour.rs
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/crates/bots/tests/behaviour.rs b/.gate/u4r-snap1/spikes/netcode-proof/crates/bots/tests/behaviour.rs
new file mode 100644
index 0000000..6dbe069
--- /dev/null
+++ b/.gate/u4r-snap1/spikes/netcode-proof/crates/bots/tests/behaviour.rs
@@ -0,0 +1,236 @@
+//! U4r's tests of npt-bots' `Behaviour` (u4r-brief.md 2.2): a real `ClientEndpoint` fed crafted room messages, each
+//! encoded with npt-proto as the room sends it, and the commands it then sends read back from its `Input` datagrams.
+
+use npt_bots::Behaviour;
+use npt_client::ClientEndpoint;
+use npt_link::Channel;
+use npt_proto::{
+    Direction, EntityDelta, FullState, Join, JoinAccepted, Message, Snapshot, decode, encode,
+};
+use npt_sim::{Command, EntityKind, EntityRecord, Scene};
+
+fn bytes(m: &Message) -> Vec<u8> {
+    let mut out = Vec::new();
+    encode(m, &mut out).unwrap_or_else(|e| panic!("{}: {e}", m.name()));
+    out
+}
+
+fn soldier(index: u16, x: i32) -> EntityRecord {
+    let mut grid = [0; EntityRecord::FIELDS];
+    grid[0] = x;
+    grid[2] = 0;
+    grid[6] = 1_000;
+    EntityRecord {
+        kind: EntityKind::Soldier,
+        index,
+        generation: 0,
+        area: 0,
+        grid,
+        raw: [0; EntityRecord::FIELDS],
+    }
+}
+
+/// The room's `JoinAccepted` for slot 0 at tick `t`.
+fn accepted(t: u32) -> (Channel, Vec<u8>, u64) {
+    let m = Message::JoinAccepted(JoinAccepted {
+        slot: 0,
+        tick: t,
+        ..JoinAccepted::default()
+    });
+    (Channel::Control, bytes(&m), 0)
+}
+
+/// The full state of tick `t`: the client's own soldier and soldiers 1 and 20.
+fn full(t: u32) -> (Channel, Vec<u8>, u64) {
+    let m = Message::FullState(FullState {
+        tick: t,
+        last_run: None,
+        own: vec![soldier(0, 0)],
+        entities: vec![soldier(1, 100), soldier(20, 2_000)],
+    });
+    (Channel::FullState, bytes(&m), 0)
+}
+
+/// One snapshot group of tick `t` against the full state `age` ticks back: soldier 1 moved to `x`.
+fn snap(t: u32, age: u16, x: i32) -> (Channel, Vec<u8>, u64) {
+    let delta = EntityDelta::between(&soldier(1, x), Some(&soldier(1, 100))).expect("a change");
+    let m = Message::Snapshot(Snapshot {
+        tick: t,
+        baseline_age: age,
+        group: 0,
+        groups: 1,
+        entities: vec![delta],
+        ..Snapshot::default()
+    });
+    (Channel::Datagram, bytes(&m), 0)
+}
+
+/// The commands of the `Input` the endpoint's next step sends (newest first), or none.
+fn sent(e: &mut ClientEndpoint) -> Vec<Command> {
+    let out = e.step();
+    out.datagrams
+        .iter()
+        .find_map(
+            |d| match decode(Direction::ClientToRoom, Channel::Datagram, d) {
+                Ok(Message::Input(i)) => Some(i.commands),
+                _ => None,
+            },
+        )
+        .unwrap_or_default()
+}
+
+fn endpoint(lead: u32) -> ClientEndpoint {
+    ClientEndpoint::new(Join::default(), npt_sim::TICK_HZ).with_lead(lead)
+}
+
+#[test]
+fn b1_no_command_before_a_snapshot_then_one_per_step_whose_newest_tick_moved() {
+    let scene = Scene::stress();
+    let mut b = Behaviour::brawl(&scene, 0, 7);
+    let mut e = endpoint(5);
+    assert!(
+        !b.step(&mut e, &[accepted(100)], 0, &[]),
+        "joined, no snapshot yet: no command"
+    );
+    assert!(sent(&mut e).is_empty());
+    assert!(
+        b.step(&mut e, &[full(100)], 33, &[]),
+        "the full state's tick is the newest: a command"
+    );
+    let c = sent(&mut e);
+    assert_eq!(c.len(), 1);
+    assert_eq!(c[0].tick, 105, "stamped newest + lead");
+    assert_eq!(
+        (c[0].view_tick, c[0].view_fraction),
+        (100, 0),
+        "the view is the newest tick"
+    );
+    assert!(
+        !b.step(&mut e, &[], 66, &[]),
+        "the newest tick did not move: no command"
+    );
+    assert_eq!(b.skipped(), 1);
+    assert!(b.step(&mut e, &[snap(101, 1, 150)], 99, &[]));
+    let c = sent(&mut e);
+    assert_eq!(c[0].tick, 106);
+    assert_eq!(
+        c[0].seq,
+        c[1].seq.wrapping_add(1),
+        "one new command, the older resent"
+    );
+    assert_eq!((b.given(), b.skipped()), (2, 1));
+    assert_eq!(
+        c[0].buttons & Command::FIRE,
+        Command::FIRE,
+        "the brawl holds fire"
+    );
+}
+
+#[test]
+fn b2_every_step_gives_a_command_at_each_step_after_the_first_snapshot() {
+    let scene = Scene::stress();
+    let mut b = Behaviour::brawl(&scene, 0, 7).every_step();
+    let mut e = endpoint(5);
+    assert!(!b.step(&mut e, &[accepted(100)], 0, &[]));
+    for i in 0..5 {
+        let r = if i == 0 { vec![full(100)] } else { vec![] };
+        assert!(b.step(&mut e, &r, 33 * (i + 1), &[]), "step {i}");
+    }
+    assert_eq!((b.given(), b.skipped()), (5, 0));
+    let c = sent(&mut e);
+    assert!(
+        c.iter().all(|c| c.tick == 105),
+        "every one stamped for the same tick"
+    );
+}
+
+/// Every pressable counter moves by at most one edge from one command to the next (U4R-Q14, A4).
+fn at_most_one_edge(cmds: &[Command]) -> bool {
+    cmds.windows(2).all(|w| {
+        w[0].presses
+            .iter()
+            .zip(&w[1].presses)
+            .all(|(a, b)| b.wrapping_sub(*a) <= 1)
+    })
+}
+
+/// The commands a behaviour gives over `n` steps, each newest tick moving by one.
+fn commands(mut b: Behaviour, n: u32) -> Vec<Command> {
+    let mut e = endpoint(2);
+    b.step(&mut e, &[accepted(100)], 0, &[]);
+    b.step(&mut e, &[full(100)], 1, &[]);
+    let mut out = sent(&mut e);
+    for i in 1..n {
+        let view = [(1, [100 + i as i32, 0, 0]), (20, [2_000, i as i32, 0])];
+        b.step(
+            &mut e,
+            &[snap(100 + i, 1, 100 + i as i32)],
+            u64::from(i) * 33,
+            &view,
+        );
+        out.push(sent(&mut e)[0]);
+    }
+    out
+}
+
+#[test]
+fn b3_each_press_counter_moves_by_at_most_one_edge_per_command() {
+    let scene = Scene::stress();
+    for name in ["presses", "clamp", "steady"] {
+        let b = Behaviour::script(&scene, 3, 11, name).expect("a script");
+        let cmds = commands(b, 40);
+        assert!(at_most_one_edge(&cmds), "{name}");
+        if name == "presses" {
+            let edges = cmds
+                .windows(2)
+                .filter(|w| w[0].presses != w[1].presses)
+                .count();
+            assert!(edges >= 15, "the presses script presses: {edges} edges");
+        }
+    }
+    assert!(at_most_one_edge(&commands(
+        Behaviour::brawl(&scene, 3, 11),
+        40
+    )));
+    assert!(Behaviour::script(&scene, 3, 11, "cheat").is_none());
+}
+
+#[test]
+fn b4_the_same_seed_gives_the_same_commands_and_another_seed_others() {
+    let scene = Scene::stress();
+    let a = commands(Behaviour::brawl(&scene, 5, 1), 60);
+    let b = commands(Behaviour::brawl(&scene, 5, 1), 60);
+    let c = commands(Behaviour::brawl(&scene, 5, 2), 60);
+    assert_eq!(a, b);
+    assert_ne!(a, c);
+    let room_check = |c: &Command| {
+        let [x, y] = c.move_xy.map(i32::from);
+        x * x + y * y <= 127 * 127
+    };
+    assert!(a.iter().all(room_check), "every move inside x² + y² ≤ 127²");
+}
+
+#[test]
+fn b5_the_brawl_aims_at_an_enemy_in_view_never_a_teammate() {
+    let scene = Scene::stress();
+    // Slot 0 is team 0: slot 1 is its teammate, slot 20 an enemy (3 x 16).
+    assert_eq!(
+        (scene.team(0), scene.team(1), scene.team(20)),
+        (Some(0), Some(0), Some(1))
+    );
+    let cmds = commands(Behaviour::brawl(&scene, 0, 9), 90);
+    for (i, c) in cmds.iter().enumerate().skip(1) {
+        let enemy = [2_000, i as i32, 100];
+        assert_eq!(
+            c.aim, enemy,
+            "command {i} aims at slot 20, raised into its box"
+        );
+    }
+    let clamp = commands(Behaviour::script(&scene, 0, 9, "clamp").expect("clamp"), 5);
+    assert!(
+        clamp
+            .iter()
+            .all(|c| c.aim[0].abs() == 32_000 && c.aim[1].abs() == 32_000),
+        "the clamp script aims far beyond the reach"
+    );
+}

# npt-server's schedule tests (crates/server/tests/schedule.rs): git diff --no-index spikes/netcode-proof/crates/server/tests/schedule.rs .gate/u4r-snap1/spikes/netcode-proof/crates/server/tests/schedule.rs
diff --git a/spikes/netcode-proof/crates/server/tests/schedule.rs b/.gate/u4r-snap1/spikes/netcode-proof/crates/server/tests/schedule.rs
index 01c465e..2f463d7 100644
--- a/spikes/netcode-proof/crates/server/tests/schedule.rs
+++ b/.gate/u4r-snap1/spikes/netcode-proof/crates/server/tests/schedule.rs
@@ -337,6 +337,8 @@ mod bites {
     const TIMER_PERIOD: &str = "PM-0004-67/timer-period";
     const UNCAPPED: &str = "PM-0004-67/uncapped-catchup";
     const UNCOUNTED: &str = "PM-0004-67/uncounted-drop";
+    /// The pump's (U4R-Q6 (a)): in npt-server's list, never the schedule's.
+    const WALLCLOCK: &str = "PM-0001-30/wallclock";
 
     fn bitten(hz: u32, cap: u32, id: &str) -> Schedule {
         Schedule::with_bite(hz, cap, id).expect("npt-server plants it")
@@ -344,7 +346,10 @@ mod bites {
 
     #[test]
     fn the_three_bites_are_planted_here_under_their_marks_json_ids() {
-        assert_eq!(npt_server::BITES, [TIMER_PERIOD, UNCAPPED, UNCOUNTED]);
+        assert_eq!(
+            npt_server::BITES,
+            [TIMER_PERIOD, UNCAPPED, UNCOUNTED, WALLCLOCK]
+        );
         let marks = include_str!("../../../marks/marks.json");
         for id in npt_server::BITES {
             assert!(
@@ -352,7 +357,12 @@ mod bites {
                 "{id} is not in marks.json"
             );
         }
-        for other in ["PM-0004-67/no-such-bite", "PM-0001-17/client-rounding", ""] {
+        for other in [
+            "PM-0004-67/no-such-bite",
+            "PM-0001-17/client-rounding",
+            "",
+            WALLCLOCK,
+        ] {
             assert!(Schedule::with_bite(30, 7, other).is_none(), "{other}");
         }
     }
