# check.test.mjs (check.mjs's self-tests): git diff --no-index .gate/u4d2-r1/spikes/netcode-proof/check.test.mjs .claude/worktrees/u4d2-build/spikes/netcode-proof/check.test.mjs
diff --git a/.gate/u4d2-r1/spikes/netcode-proof/check.test.mjs b/.claude/worktrees/u4d2-build/spikes/netcode-proof/check.test.mjs
index fec574a..d1d2039 100644
--- a/.gate/u4d2-r1/spikes/netcode-proof/check.test.mjs
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/check.test.mjs
@@ -13,6 +13,7 @@ import {
   BITE_ID,
   CHANNEL_ONLY,
   CLIPPY_TWINS,
+  FAILED_STEP_DIR,
   H2_VECTOR,
   HASH_MAP_ARM,
   HASH_MAP_FIELD,
@@ -31,6 +32,7 @@ import {
   WASM_STEP_DIR,
   biteHomeProblems,
   buildFlagProblems,
+  buildInputProblems,
   builtArtifact,
   cargoConfigProblems,
   cargoCounts,
@@ -47,6 +49,7 @@ import {
   lexRust,
   linkUseProblems,
   lintBothBuilds,
+  main,
   manifestKeyProblems,
   marksProblems,
   NOT_TICK_COUNTS,
@@ -264,6 +267,8 @@ const EXPORT_SIGS = {
 const exported = (name, sig = EXPORT_SIGS[name], between = "") =>
   `#[allow(\n    unsafe_code,\n    reason = "the third exception (ADR 0002:264-265): this listed name; \\\n              integers only"\n)]\n` +
   `${between}#[unsafe(no_mangle)]\npub extern "C" fn ${name}${sig} {\n    run()\n}\n`;
+/** begin's first line in the fixtures (its body follows): where U4d2 round 1's FX-3 plants an `unsafe` block. */
+const BEGIN_HEAD = `pub extern "C" fn begin${EXPORT_SIGS.begin} {\n`;
 /** npt-hash-wasm's clean lib.rs: a doc naming what it may not do, the seven exports, a helper. */
 const HASH_WASM_LIB =
   "//! The seven exports; no `unsafe {}` block and no other #[allow(unsafe_code)] (a doc comment, not code).\n" +
@@ -325,7 +330,7 @@ test("lexRust blanks comments and keeps literals, offsets and lines", () => {
   assert.match(mask, /^a\s+\nb\s+x<'a>\s+e$/);
 });
 
-test("unsafe_code: an attribute relaxing it outside the two exceptions fails, in every form", () => {
+test("unsafe_code: an attribute relaxing it outside the three exceptions fails, in every form", () => {
   const planted = [
     "#[allow(unsafe_code)]\nfn f() {}\n",
     "#[expect(unsafe_code)]\nfn f() {}\n",
@@ -361,7 +366,7 @@ test("unsafe_code: a manifest that sets it or drops the workspace lints fails",
     f["crates/sim/Cargo.toml"] = '[package]\nname = "npt-sim"\nworkspace = true\n';
   });
   assert.deepEqual(dropped, ["crates/sim/Cargo.toml does not inherit the workspace lints ([lints] workspace = true)"]);
-  // The two exceptions' manifests set their own level (the workspace forbids it): exceptionLintProblems reads those.
+  // The three exceptions' manifests set their own level (the workspace forbids it): exceptionLintProblems reads those.
   assert.deepEqual(
     unsafeWith((f) => (f["crates/tcpinfo/Cargo.toml"] = '[package]\nname = "npt-tcpinfo"\n\n[lints.rust]\nunsafe_code = "deny"\n')),
     [],
@@ -526,7 +531,7 @@ test("unsafe_code: no #[path] or include! reaches outside its crate, the way npt
 /** One unsafe block, as rustc 1.99.1 compiles it once unsafe_code is relaxed. */
 const UNSAFE_FN = "pub fn f() -> u8 {\n    let x = 1u8;\n    unsafe { *(&raw const x) }\n}\n";
 
-test("unsafe code outside the two exception crates fails however the lint was relaxed (review S1)", () => {
+test("unsafe code outside the three exception crates fails however the lint was relaxed (review S1)", () => {
   const relaxedElsewhere = {
     // Relaxed by RUSTFLAGS=-Aunsafe_code, --cap-lints warn or a cargo config: no attribute to see, the block still shows.
     "rustflags or --cap-lints": UNSAFE_FN,
@@ -1119,6 +1124,16 @@ const clippyDrift = (copyAt, homeAt) =>
 const linkUse = (at, shown) =>
   `${at} names \`${shown}\` from npt_link: npt-proto and npt-client use npt-link for Channel only (catalogue item 2; ` +
   "u4-build-plan.md §4.1)";
+/** buildInputProblems' messages (the advisor's U4d2 round-1 ruling, FX-1): a source holding flags; a build target set. */
+const holds = (where, flags) =>
+  `${where} holds ${JSON.stringify(flags)}: no rustflags source may hold a flag (cargo uses one source and drops the ` +
+  "rest, so the read-back could not be given what rustc gets: the advisor's U4d2 round-1 ruling, FX-1)";
+const targetEnv = (value) =>
+  `CARGO_BUILD_TARGET in the environment sets the build target (${JSON.stringify(value)}): the native builds take ` +
+  "cargo's default target, the x86-64 one the read-back and the features comparison read";
+const targetKey = (path) =>
+  `${path} has a key \`target\` (build.target): it sets the build target, and the native builds take cargo's default ` +
+  "target, the x86-64 one the read-back and the features comparison read";
 
 /** rustc 1.99.1's read-back of the target features per target in this container (`rustc --print cfg`; notes F1). */
 const CFG_BASE = {
@@ -1276,8 +1291,9 @@ test("the deps step is depsStep, and it calls every check: one plant per check,
       // U4a's two (u4-build-plan.md §2's Drift; u4s-carries.md carry 3).
       f["crates/proto/clippy.toml"] += "disallowed-methods = []\n";
       f["crates/proto/src/lib.rs"] = "use npt_link::Link;\n";
-      // U4d2's npt-hash-wasm rules: an unsafe block in its lib.rs.
-      f[HASH_WASM_UNSAFE_FILE] += "fn stray() {\n    unsafe {}\n}\n";
+      // U4d2's npt-hash-wasm rules: an unsafe block in its lib.rs, in a new fn and (U4d2 round 1, FX-3) inside a listed
+      // export's body, where that export's own allow lets rustc compile it.
+      f[HASH_WASM_UNSAFE_FILE] = `${HASH_WASM_LIB.replace(BEGIN_HEAD, `${BEGIN_HEAD}    unsafe {}\n`)}fn stray() {\n    unsafe {}\n}\n`;
     },
     (e) => {
       e["npt-sim"] = n("tokio");
@@ -1298,7 +1314,9 @@ test("the deps step is depsStep, and it calls every check: one plant per check,
   const r = depsStep(bad.dir, bad.env, () => ({ ok: true, meta: bad.meta }), bad.runner);
   assert.equal(r.ok, false);
   const lines = r.out.split("\n");
-  const strayAt = `${HASH_WASM_UNSAFE_FILE}:${HASH_WASM_LIB.split("\n").length + 1}`;
+  // The new fn's block follows the fixture's lib.rs, one line longer for the block in begin's body.
+  const strayAt = `${HASH_WASM_UNSAFE_FILE}:${HASH_WASM_LIB.split("\n").length + 2}`;
+  const bodyAt = `${HASH_WASM_UNSAFE_FILE}:${HASH_WASM_LIB.split("\n").indexOf(BEGIN_HEAD.trimEnd()) + 2}`;
   for (const [check, line] of [
     ["dependencyProblems", "leaf crate npt-sim depends on tokio"],
     ["workspaceLintProblems", 'Cargo.toml has no [workspace.lints.rust] unsafe_code = "forbid" for its crates to inherit'],
@@ -1322,6 +1340,7 @@ test("the deps step is depsStep, and it calls every check: one plant per check,
       "buildFlagProblems",
       `${join(bad.dir, ".cargo", "config.toml")}'s rustflags passes "-C target-feature=+fma": it enables fma, which plan.md §4.5's build flags refuse`,
     ],
+    ["buildInputProblems (U4d2 round 1, FX-1)", holds(`${join(bad.dir, ".cargo", "config.toml")}'s rustflags`, "-C target-feature=+fma")],
     [
       "readBackCheck",
       'rustc\'s read-back for x86_64-unknown-linux-gnu (given "-C target-feature=+fma") lists target_feature="fma", which plan.md §4.5\'s build flags refuse',
@@ -1334,6 +1353,10 @@ test("the deps step is depsStep, and it calls every check: one plant per check,
       "unsafeProblems (npt-hash-wasm)",
       `${strayAt} has \`unsafe\` outside an attribute; npt-hash-wasm's only unsafe is the #[unsafe(no_mangle)] of its listed exports (${HASH_WASM_EXPORTS.join(", ")}) in ${HASH_WASM_UNSAFE_FILE}`,
     ],
+    [
+      "unsafeProblems (npt-hash-wasm, inside an export's body: U4d2 round 1, FX-3)",
+      `${bodyAt} has \`unsafe\` outside an attribute; npt-hash-wasm's only unsafe is the #[unsafe(no_mangle)] of its listed exports (${HASH_WASM_EXPORTS.join(", ")}) in ${HASH_WASM_UNSAFE_FILE}`,
+    ],
     ["dependencyProblems (npt-hash-wasm)", "npt-hash-wasm depends on serde"],
   ]) {
     assert.ok(lines.includes(line), `${check}'s plant is not reported:\n${r.out}`);
@@ -1963,34 +1986,114 @@ test("npt-hash-wasm, the third exception: one allow on each listed export, its n
   assert.deepEqual(UNSAFE_EXCEPTIONS, { "crates/client-wasm/": "npt-client-wasm", "crates/tcpinfo/": "npt-tcpinfo", "crates/hash-wasm/": "npt-hash-wasm" });
 });
 
+test("npt-hash-wasm: an `unsafe` block inside a listed export's body is refused at its line, where that export's own allow lets rustc compile it (U4d2 round 1, FX-3: RR-M1, SEC-U4D2-2, TE-M2)", () => {
+  const F = HASH_WASM_UNSAFE_FILE;
+  // TP4's form (r1-te-tp4-body-unsafe.log: rustc, clippy -D warnings and the unit tests accept it; the scan alone
+  // refuses it), and a bare block, as the first line of each listed export's body.
+  for (const name of HASH_WASM_EXPORTS) {
+    const head = `pub extern "C" fn ${name}${EXPORT_SIGS[name]} {\n`;
+    for (const block of ["    unsafe {}\n", "    let w = unsafe { core::ptr::read_volatile(&0u32) };\n"]) {
+      const planted = HASH_WASM_LIB.replace(head, `${head}${block}`);
+      assert.notEqual(planted, HASH_WASM_LIB, name);
+      const line = planted.split("\n").indexOf(block.trimEnd()) + 1;
+      assert.deepEqual(unsafeWith((f) => (f[F] = planted)), [hwUnsafe(`${F}:${line}`, "unsafe", "outside an attribute")], `${name}: ${block}`);
+    }
+  }
+  // Deeper in a body: inside a nested block, a closure and an unsafe fn item declared within it.
+  for (const inner of ["    if true {\n        unsafe {}\n    }\n", "    let f = || unsafe { 0 };\n", "    unsafe fn g() {}\n"]) {
+    const planted = HASH_WASM_LIB.replace(BEGIN_HEAD, `${BEGIN_HEAD}${inner}`);
+    const line = planted.split("\n").findIndex((l, i) => i > 0 && l.includes("unsafe") && !l.includes("no_mangle") && !l.includes("unsafe_code")) + 1;
+    assert.deepEqual(unsafeWith((f) => (f[F] = planted)), [hwUnsafe(`${F}:${line}`, "unsafe", "outside an attribute")], inner);
+  }
+});
+
+test("npt-tcpinfo and npt-hash-wasm, the deny-level exceptions, have no build script and name `include` only as a macro call: V4's two forms are refused by name (U4d2 round 1, FX-6: SEC-U4D2-4)", () => {
+  const F = HASH_WASM_UNSAFE_FILE;
+  const why =
+    "code a build script writes, brought in by a renamed include!, would sit inside a listed function's allow, where rustc " +
+    "compiles unsafe code the scan never reads (SEC-U4D2-4)";
+  // V4's first form (r1-sec-scan.log; sec-probe-scan.mjs): a build.rs writing `\x75nsafe` code into OUT_DIR, which the
+  // scan cannot see in its string.
+  const buildRs =
+    'fn main() {\n    let out = std::env::var("OUT_DIR").unwrap();\n' +
+    '    let code = "{ let _ = \\x75nsafe { core::ptr::read_volatile(&0u32) }; }";\n' +
+    '    std::fs::write(format!("{out}/gen.rs"), code).unwrap();\n}\n';
+  for (const crate of ["hash-wasm", "tcpinfo"]) {
+    assert.deepEqual(unsafeWith((f) => (f[`crates/${crate}/build.rs`] = buildRs)), [`crates/${crate}/build.rs is a build script in npt-${crate}, which may have none: ${why}`]);
+  }
+  // Its second: `use std::include as inc;`, and `inc!(concat!(env!("OUT_DIR"), "/gen.rs"));` in begin's body, which the
+  // include! rule cannot see by its name; and a raw identifier's form in npt-tcpinfo.
+  const includeAt = (where, name) =>
+    `${where} names \`include\` other than as an include!, include_str! or include_bytes! call (a renamed include!, as ` +
+    `\`use std::include as inc;\`, hides the call from the include! rule), which ${name} may not: ${why}`;
+  const renamed = `use std::include as inc;\n${HASH_WASM_LIB.replace(BEGIN_HEAD, `${BEGIN_HEAD}    inc!(concat!(env!("OUT_DIR"), "/gen.rs"));\n`)}`;
+  assert.deepEqual(unsafeWith((f) => (f[F] = renamed)), [includeAt(`${F}:1`, "npt-hash-wasm")]);
+  assert.deepEqual(unsafeWith((f) => (f["crates/tcpinfo/src/lib.rs"] += "use core::r#include as i;\n")), [includeAt("crates/tcpinfo/src/lib.rs:8", "npt-tcpinfo")]);
+  // The macros by their own names are pathProblems' (an in-crate .rs literal passes); `include` in prose, a string or
+  // another crate is not this rule's (the workspace's forbid makes rustc refuse unsafe code there).
+  const calls = 'include!("k.rs");\nconst V: &str = include_str!("../x.txt");\nconst B: &[u8] = include_bytes!("../y.bin");\n// include\nconst S: &str = "include";\n';
+  assert.deepEqual(unsafeWith((f) => (f[F] = `${HASH_WASM_LIB}${calls}`)), []);
+  assert.deepEqual(unsafeWith((f) => (f["crates/net/src/lib.rs"] += "use std::include as inc;\n")), []);
+  // A `build` key in the manifest, in either form cargo reads; a custom-build target in cargo metadata.
+  const manifestKey = (crate) => `crates/${crate}/Cargo.toml has a key \`build\`: a build script in npt-${crate}, which may have none: ${why}`;
+  assert.deepEqual(unsafeWith((f) => (f["crates/hash-wasm/Cargo.toml"] = HASH_WASM_MANIFEST.replace("\n\n[lints.rust]", '\nbuild = "gen.rs"\n\n[lints.rust]'))), [manifestKey("hash-wasm")]);
+  assert.deepEqual(unsafeWith((f) => (f["crates/tcpinfo/Cargo.toml"] = `package.build = true\n${f["crates/tcpinfo/Cargo.toml"]}`)), [manifestKey("tcpinfo")]);
+  for (const crate of ["hash-wasm", "tcpinfo"]) {
+    const m = layoutMeta({ [`crates/${crate}`]: [["lib", `npt_${crate}`, "src/lib.rs"], ["custom-build", "build-script-build", "build.rs"]] });
+    assert.deepEqual(layoutProblems(m), [`npt-${crate}'s custom-build target build-script-build is a build script in npt-${crate}, which may have none: ${why}`]);
+  }
+  assert.deepEqual(layoutProblems(layoutMeta({ "crates/bots": [["bin", "npt-bots", "src/main.rs"], ["custom-build", "build-script-build", "build.rs"]] })), [], "another crate's build script");
+  // The spike's own crates have neither.
+  const real = crateFiles(join(HERE, "crates")).files;
+  assert.deepEqual(unsafeProblems(real, (f) => readFileSync(join(HERE, f), "utf8")), []);
+});
+
+/** The target_features entries rustc 1.99.1's release builds of npt-hash-wasm carry (the rust seat's wasminfo run). */
+const BUILT_FEATURES = ["+bulk-memory", "+bulk-memory-opt", "+call-indirect-overlong", "+multivalue", "+mutable-globals", "+nontrapping-fptoint", "+reference-types", "+sign-ext"];
+/** LEB128, as WebAssembly's binary format writes an unsigned integer. */
+const leb128 = (n) => {
+  const out = [];
+  do {
+    let b = n & 0x7f;
+    n >>>= 7;
+    if (n) b |= 0x80;
+    out.push(b);
+  } while (n);
+  return out;
+};
+const wasmName = (s) => [...leb128(Buffer.byteLength(s)), ...Buffer.from(s)];
+/** A target_features section's body: a count, then each entry's prefix byte and name. */
+const featureBody = (entries) => [...leb128(entries.length), ...entries.flatMap((f) => [f.charCodeAt(0), ...wasmName(f.slice(1))])];
+
 /**
- * A hand-assembled WebAssembly module, a few dozen bytes: each of `names` exported, a function () -> i32 returning 0,
- * but `memory`, a memory of one page; with `importFn`, a function env.now imported first.
+ * A hand-assembled WebAssembly module, a few hundred bytes: each of `names` exported, a function () -> i32 returning 0,
+ * but `memory`, a memory of one page; `kinds` exports a name as another kind (`global`, an i32 global; `function`, for
+ * `memory` too, and then the module has no memory); `imports` imports one function per [module, field], first;
+ * `features` is its target_features section's entries (two of BUILT_FEATURES by default: wasm-ld writes one), and
+ * `sections`, when given, its target_features sections' bodies in place of that one ([] for none).
  */
-function wasmModule(names, { importFn = false } = {}) {
-  const leb = (n) => {
-    const out = [];
-    do {
-      let b = n & 0x7f;
-      n >>>= 7;
-      if (n) b |= 0x80;
-      out.push(b);
-    } while (n);
-    return out;
-  };
-  const str = (s) => [...leb(s.length), ...Buffer.from(s)];
-  const section = (id, body) => [id, ...leb(body.length), ...body];
-  const fns = names.filter((x) => x !== "memory");
-  const imported = importFn ? 1 : 0;
+function wasmModule(names, { imports = [], kinds = {}, features = ["+mutable-globals", "+sign-ext"], sections } = {}) {
+  const section = (id, body) => [id, ...leb128(body.length), ...body];
+  const kindOf = (x) => kinds[x] ?? (x === "memory" ? "memory" : "function");
+  const fns = names.filter((x) => kindOf(x) === "function");
+  const globals = names.filter((x) => kindOf(x) === "global");
   const body = [0, 0x41, 0, 0x0b];
+  const exported = (x) => {
+    const k = kindOf(x);
+    if (k === "memory") return [...wasmName(x), 0x02, 0];
+    if (k === "global") return [...wasmName(x), 0x03, ...leb128(globals.indexOf(x))];
+    return [...wasmName(x), 0x00, ...leb128(imports.length + fns.indexOf(x))];
+  };
   return new Uint8Array([
     0x00, 0x61, 0x73, 0x6d, 1, 0, 0, 0,
     ...section(1, [1, 0x60, 0, 1, 0x7f]),
-    ...(importFn ? section(2, [1, ...str("env"), ...str("now"), 0x00, 0]) : []),
-    ...section(3, [...leb(fns.length), ...fns.map(() => 0)]),
-    ...(names.includes("memory") ? section(5, [1, 0x00, 1]) : []),
-    ...section(7, [...leb(names.length), ...names.flatMap((x) => (x === "memory" ? [...str(x), 0x02, 0] : [...str(x), 0x00, ...leb(imported + fns.indexOf(x))]))]),
-    ...section(10, [...leb(fns.length), ...fns.flatMap(() => [...leb(body.length), ...body])]),
+    ...(imports.length ? section(2, [...leb128(imports.length), ...imports.flatMap(([m, f]) => [...wasmName(m), ...wasmName(f), 0x00, 0])]) : []),
+    ...section(3, [...leb128(fns.length), ...fns.map(() => 0)]),
+    ...(names.some((x) => kindOf(x) === "memory") ? section(5, [1, 0x00, 1]) : []),
+    ...(globals.length ? section(6, [...leb128(globals.length), ...globals.flatMap(() => [0x7f, 0x00, 0x41, 0, 0x0b])]) : []),
+    ...section(7, [...leb128(names.length), ...names.flatMap(exported)]),
+    ...section(10, [...leb128(fns.length), ...fns.flatMap(() => [...leb128(body.length), ...body])]),
+    ...(sections ?? [featureBody(features)]).flatMap((b) => section(0, [...wasmName("target_features"), ...b])),
   ]);
 }
 
@@ -2006,15 +2109,73 @@ test("the loader's list is the seven exports and wasm-ld's memory; an export mor
   assert.deepEqual(exportProblems(wasmModule(listed.filter((x) => x !== "dump_word"))), ['the module does not export "dump_word"']);
   assert.match(exportProblems(new Uint8Array([0, 1, 2, 3]))[0], /^not a WebAssembly module: /);
   assert.deepEqual(moduleProblems(new WebAssembly.Module(wasmModule(listed))), []);
-  // Refused before instantiating; and with an empty import object a module that imports a function fails.
+  // Refused before instantiating; and a module that imports a function is refused, naming it (FX-4's form since U4d2's
+  // round 1: every import refused by moduleProblems, not left to an empty import object).
   assert.throws(() => instantiate(new WebAssembly.Module(wasmModule([...listed, "extra"]))), /^Error: refused: the module exports "extra"/);
   assert.throws(
-    () => instantiate(new WebAssembly.Module(wasmModule(listed, { importFn: true }))),
-    /refused: the module does not instantiate with no imports \(it imports env\.now \(function\)\)/,
+    () => instantiate(new WebAssembly.Module(wasmModule(listed, { imports: [["env", "now"]] }))),
+    /^Error: refused: the module imports "env\.now" \(a function\): the loader gives a module nothing to import$/,
   );
   assert.equal(typeof instantiate(new WebAssembly.Module(wasmModule(listed))).begin, "function");
 });
 
+test("the loader reads the module's target_features section: simd128 or relaxed-simd used or required is refused, and so is a module with none, two or one that does not parse; the built modules' list passes (U4d2 round 1, FX-2)", () => {
+  const listed = [...HASH_WASM_EXPORTS, "memory"];
+  const problems = (opts) => moduleProblems(new WebAssembly.Module(wasmModule(listed, opts)));
+  // The built modules' list (both builds), and the two features named disallowed (`-`), pass.
+  assert.deepEqual(problems({ features: BUILT_FEATURES }), []);
+  assert.deepEqual(problems({ features: [...BUILT_FEATURES, "-simd128", "-relaxed-simd"] }), []);
+  // Used (`+`) or required (`=`): refused by name.
+  const lists = (f) => `the module's target_features section lists ${f}, which plan.md §4.5 leaves off (simd128 and relaxed-simd)`;
+  for (const f of ["+simd128", "+relaxed-simd", "=simd128", "=relaxed-simd"]) assert.deepEqual(problems({ features: [...BUILT_FEATURES, f] }), [lists(f)], f);
+  // sec-probe-tf2's module's list (r1-sec-tf2.log: one safe #[target_feature(enable = "relaxed-simd")] fn): both named.
+  assert.deepEqual(problems({ features: ["+relaxed-simd", "+simd128", ...BUILT_FEATURES] }), [lists("+relaxed-simd"), lists("+simd128")]);
+  assert.throws(() => instantiate(new WebAssembly.Module(wasmModule(listed, { features: ["+simd128"] }))), /^Error: refused: the module's target_features section lists \+simd128,/);
+  // None, or two (wasm-ld writes one; a module stripped of it says nothing of its features).
+  const sections = (k) => `the module has ${k} target_features sections, not one, so its target features cannot be read (wasm-ld writes one)`;
+  assert.deepEqual(problems({ sections: [] }), [sections(0)]);
+  assert.deepEqual(problems({ sections: [featureBody(BUILT_FEATURES), featureBody(BUILT_FEATURES)] }), [sections(2)]);
+  // One that does not parse: a name cut short, a count cut short, an entry missing, a prefix of another byte, a byte
+  // after the last entry, a name that is no UTF-8.
+  const parse = (why) => `the module's target_features section does not parse: ${why}`;
+  const whole = featureBody(["+sign-ext"]);
+  assert.deepEqual(problems({ sections: [whole.slice(0, -2)] }), [parse("entry 1's name runs past its end")]);
+  assert.deepEqual(problems({ sections: [[0x80]] }), [parse("it ends inside its entry count")]);
+  assert.deepEqual(problems({ sections: [[2, ...whole.slice(1)]] }), [parse("it ends before entry 2 of 2")]);
+  assert.deepEqual(problems({ sections: [[1, 0x2a, ...wasmName("simd128")]] }), [parse("entry 1's prefix is byte 42, not +, - or =")]);
+  assert.deepEqual(problems({ sections: [[...whole, 0]] }), [parse("1 byte(s) follow its last entry")]);
+  const notUtf8 = problems({ sections: [[1, 0x2b, 2, 0xc3, 0x28]] });
+  assert.equal(notUtf8.length, 1, notUtf8.join("\n"));
+  assert.ok(notUtf8[0].startsWith(parse("")), notUtf8[0]);
+  // exportProblems (the bytes' path) reads it too.
+  assert.deepEqual(exportProblems(wasmModule(listed, { sections: [] })), [sections(0)]);
+});
+
+test("the loader refuses every import by module and name before instantiating, Object.prototype's names included (U4d2 round 1, FX-4: SEC-U4D2-1)", () => {
+  const listed = [...HASH_WASM_EXPORTS, "memory"];
+  const imports = (m, f) => `the module imports ${JSON.stringify(`${m}.${f}`)} (a function): the loader gives a module nothing to import`;
+  // sec-probe-imports' modules (r1-sec-imports.log): given `{}`, constructor.constructor (JS's Function),
+  // __proto__.valueOf and constructor.keys had linked with moduleProblems []; env.now had failed only by chance.
+  for (const [m, f] of [["constructor", "constructor"], ["__proto__", "valueOf"], ["constructor", "keys"], ["env", "now"]]) {
+    const mod = new WebAssembly.Module(wasmModule(listed, { imports: [[m, f]] }));
+    assert.deepEqual(moduleProblems(mod), [imports(m, f)], `${m}.${f}`);
+    assert.throws(() => instantiate(mod), (e) => e.message === `refused: ${imports(m, f)}`, `${m}.${f}`);
+  }
+  assert.deepEqual(moduleProblems(new WebAssembly.Module(wasmModule(listed, { imports: [["env", "now"], ["constructor", "constructor"]] }))), [
+    imports("env", "now"),
+    imports("constructor", "constructor"),
+  ]);
+  assert.equal(typeof instantiate(new WebAssembly.Module(wasmModule(listed))).begin, "function", "a module with no import instantiates");
+});
+
+test("the loader refuses a listed name exported as another kind: begin as a global, memory as a function (U4d2 round 1, FX-5: SEC-U4D2-8, TE-L1)", () => {
+  const listed = [...HASH_WASM_EXPORTS, "memory"];
+  const problems = (kinds) => moduleProblems(new WebAssembly.Module(wasmModule(listed, { kinds })));
+  assert.deepEqual(problems({ begin: "global" }), ['the module exports "begin" as a global, not a function']);
+  assert.deepEqual(problems({ memory: "function" }), ['the module exports "memory" as a function, not a memory']);
+  assert.throws(() => instantiate(new WebAssembly.Module(wasmModule(listed, { kinds: { begin: "global" } }))), /^Error: refused: the module exports "begin" as a global, not a function$/);
+});
+
 test("the loader streams a recording in chunks, numbers its ticks from the first, and names the recording and the last tick on a trap or a failure (U4d2, 2.2)", (t) => {
   const dir = mkdtempSync(join(tmpdir(), "npt-loader-"));
   t.after(() => rmSync(dir, { recursive: true, force: true }));
@@ -2186,6 +2347,44 @@ test("rustc's read-back fails the step on fma (x86-64) or simd128 and relaxed-si
   assert.ok(pinned.calls.every((c) => c.cwd === "/w"));
 });
 
+test("no rustflags source may hold a flag and no build target may be set, so the read-back is given what cargo gives rustc; RR-M2's negation pair fails naming both sources (U4d2 round 1, FX-1)", (t) => {
+  // RR-M2's pair (r1-rr-superset.log): cargo used CARGO_ENCODED_RUSTFLAGS alone, so the negation in RUSTFLAGS hid
+  // bleeding-edge's simd128 and relaxed-simd from a read-back given both; each source is refused by name.
+  const pair = { CARGO_ENCODED_RUSTFLAGS: "-Ctarget-cpu=bleeding-edge", RUSTFLAGS: "-Ctarget-feature=-simd128,-relaxed-simd" };
+  const pairLines = [holds("CARGO_ENCODED_RUSTFLAGS in the environment", "-Ctarget-cpu=bleeding-edge"), holds("RUSTFLAGS in the environment", "-Ctarget-feature=-simd128,-relaxed-simd")];
+  assert.deepEqual(buildInputProblems(pair, []), pairLines);
+  // SEC-U4D2-5: flags the name rule does not know, each refused here (and passed by buildFlagProblems).
+  for (const flags of ['--cfg feature="bites"', "-C overflow-checks=off", "-C llvm-args=-fp-contract=fast"]) {
+    assert.deepEqual(buildInputProblems({ PATH: "/bin", RUSTFLAGS: flags }, []), [holds("RUSTFLAGS in the environment", flags)], flags);
+    assert.deepEqual(buildFlagProblems(rustflagSources({ RUSTFLAGS: flags }, [])), [], `${flags}: the name rule alone passes it`);
+  }
+  // Every source form: CARGO_BUILD_RUSTFLAGS, a target's variable, a config's rustflags in [build] and in a target table.
+  assert.deepEqual(buildInputProblems({ CARGO_BUILD_RUSTFLAGS: "-C opt-level=1" }, []), [holds("CARGO_BUILD_RUSTFLAGS in the environment", "-C opt-level=1")]);
+  assert.deepEqual(buildInputProblems({ CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS: "-C debuginfo=0" }, []), [
+    holds("CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS in the environment", "-C debuginfo=0"),
+  ]);
+  const config = (text) => buildInputProblems({}, [{ path: "c.toml", text }]);
+  assert.deepEqual(config('[build]\nrustflags = ["-C", "overflow-checks=off"]\n'), [holds("c.toml's rustflags", "-C overflow-checks=off")]);
+  assert.deepEqual(config('[target.x86_64-unknown-linux-gnu]\nrustflags = "-C debuginfo=0"\n'), [holds("c.toml's rustflags", "-C debuginfo=0")]);
+  // SEC-U4D2-6: a build target set by the environment (whatever its value) or by a config's build.target, in any form.
+  assert.deepEqual(buildInputProblems({ CARGO_BUILD_TARGET: "wasm32-unknown-unknown" }, []), [targetEnv("wasm32-unknown-unknown")]);
+  assert.deepEqual(buildInputProblems({ CARGO_BUILD_TARGET: "" }, []), [targetEnv("")]);
+  for (const text of ['[build]\ntarget = "aarch64-unknown-linux-gnu"\n', 'build.target = ["x86_64-unknown-linux-musl"]\n', 'build = { target = "x" }\n']) {
+    assert.deepEqual(config(text), [targetKey("c.toml")], text);
+  }
+  // None of it: this container's cargo environment, empty sources, the spike's own config, a target table, target-dir.
+  const normal = { PATH: "/usr/bin", HOME: "/root", CARGO_HTTP_CAINFO: "/root/.ccr/ca-bundle.crt", RUSTUP_HOME: "/root/.rustup", RUST_BACKTRACE: "1" };
+  assert.deepEqual(buildInputProblems({ ...normal, CARGO_TARGET_DIR: "/t", CARGO_NET_OFFLINE: "true", RUSTFLAGS: "", CARGO_ENCODED_RUSTFLAGS: "" }, []), []);
+  const own = { path: "spike/.cargo/config.toml", text: readFileSync(join(HERE, ".cargo", "config.toml"), "utf8") };
+  assert.deepEqual(buildInputProblems(normal, [own]), []);
+  assert.deepEqual(config('[target.x86_64-unknown-linux-gnu]\nlinker = "cc"\n[build]\ntarget-dir = "t"\nrustflags = []\n[env]\nX = "target = y"\n'), []);
+  // Through depsStep (review C2R-5's rule: a check unhooked from the step fails a test): the pair fails the step.
+  const f = depsFixture(t);
+  const r = depsStep(f.dir, { ...f.env, ...pair }, () => ({ ok: true, meta: f.meta }), f.runner);
+  assert.equal(r.ok, false);
+  for (const line of pairLines) assert.ok(r.out.split("\n").includes(line), r.out);
+});
+
 test("npt-sim's resolved features are one set in the server, npt-lab and npt-hash-wasm builds, like for like; a difference anywhere in its subtree fails (U4d2, 2.4 (b), PM-0004-74)", () => {
   const sim = {
     id: "sim",
@@ -2282,6 +2481,30 @@ test("the test step names every failing test from libtest's failures: list besid
   assert.deepEqual(failingTests("test result: ok. 1 passed; 0 failed\n"), []);
 });
 
+test("a failed step's whole output is kept in its file under the checkout's .gate/, named in its FAIL lines, and removed when the step next runs (U4d2 round 1, FX-10: TE-L4)", (t) => {
+  assert.equal(FAILED_STEP_DIR, join(HERE, "..", "..", ".gate", "netcode-proof-test", "failed-steps"));
+  const dir = mkdtempSync(join(tmpdir(), "npt-failed-"));
+  t.after(() => rmSync(dir, { recursive: true, force: true }));
+  // A failed test step as testBothBuilds returns it: the panic, then 60 lines of cargo's stderr, then the summary, so
+  // main's 40 lines miss the panic (TE-M3's case: r1-te-final-check.log).
+  const panic = "thread 'l2::delay::tests::a_shutdown' panicked at crates/net/src/l2/delay_tests.rs:2590:5:\nassertion `left == right` failed\n  left: 2\n right: 1";
+  const out =
+    `running 1 test\ntest l2::delay::tests::a_shutdown ... FAILED\n\nfailures:\n\n---- l2::delay::tests::a_shutdown stdout ----\n${panic}\n\n` +
+    `failures:\n    l2::delay::tests::a_shutdown\n\ntest result: FAILED. 0 passed; 1 failed\n` +
+    `${Array.from({ length: 60 }, (_, i) => `   Compiling c${i} v0.0.0`).join("\n")}\nthe bites build's tests (x) failed: error: test failed`;
+  const printed = [];
+  assert.equal(main(["--only", "test"], { test: () => ({ ok: false, out }) }, (l) => printed.push(l), dir), 1);
+  const file = join(dir, "test.log");
+  assert.ok(!printed.join("\n").includes("panicked at"), "main's 40 lines miss the panic");
+  assert.equal(readFileSync(file, "utf8"), `${out}\n`, "the file holds the whole output, the panic included");
+  assert.ok(printed.includes(`      the step's whole output (${out.split("\n").length} lines): ${file}`), printed.join("\n"));
+  assert.equal(printed.at(-1), "check: 1 step(s) failed");
+  // The next run of the step removes it first (it passes now); another step's file stays as it was.
+  writeFileSync(join(dir, "deps.log"), "an older run's\n");
+  assert.equal(main(["--only", "test"], { test: () => ({ ok: true, out: "measured build: 1 tests passed" }) }, () => {}, dir), 0);
+  assert.deepEqual(readdirSync(dir), ["deps.log"]);
+});
+
 /** The wasm step's folders and copies, as it lays them out under WASM_STEP_DIR. */
 const WS = {
   bin: join(WASM_STEP_DIR, "bin"),
@@ -2400,15 +2623,16 @@ function wasmWorld({ over = {}, scan = NOT_CATCHABLE_SCAN } = {}) {
     libc: () => "glibc 2.39 (Node's report)",
   };
   let clock = 0;
-  const runner = (cmd, args, cwd) => {
+  // `limit` is the call's timeout (U4d2 round 1, FX-7); an answer with `timedOut` reports the run stopped by it.
+  const runner = (cmd, args, cwd, env, limit) => {
     const kind = kindOf(cmd, args);
-    calls.push({ kind, cwd, line: `${cmd === process.execPath ? "node" : cmd} ${args.join(" ")}` });
+    calls.push({ kind, cwd, limit, line: `${cmd === process.execPath ? "node" : cmd} ${args.join(" ")}` });
     const base = answer(kind, args);
     const r = over[kind] ? { ...base, ...over[kind](base, args) } : base;
     for (const w of r.writes ?? []) files.add(w);
     clock += 1_500;
-    const status = r.status ?? 0;
-    return { ok: status === 0, status, stdout: r.stdout ?? "", out: `${r.stdout ?? ""}${r.stderr ?? ""}` };
+    const status = r.timedOut ? null : (r.status ?? 0);
+    return { ok: status === 0, status, stdout: r.stdout ?? "", out: `${r.stdout ?? ""}${r.stderr ?? ""}`, timedOut: Boolean(r.timedOut) };
   };
   const result = wasmStep(runner, io, () => clock);
   return { ...result, calls, copies, kinds: calls.map((c) => c.kind) };
@@ -2557,6 +2781,21 @@ test("the wasm step fails on every outcome that is not an equal, counted run, na
   fails(wasmWorld({ over: { "build lab": () => ({ stdout: "" }) } }), "the release build of npt-lab (x86-64, measured) names no bin artifact in cargo's JSON messages");
 });
 
+test("the wasm step gives each call but its four cargo builds 300 s, and a call stopped by it fails the step naming the call and the limit (U4d2 round 1, FX-7: INF-4)", () => {
+  const w = wasmWorld();
+  assert.equal(w.ok, true, w.out);
+  for (const c of w.calls) assert.equal(c.limit, c.kind.startsWith("build ") ? 0 : 300_000, `${c.kind}: ${c.limit}`);
+  assert.equal(w.calls.filter((c) => c.limit === 0).length, 4, "the four cargo builds alone have no limit");
+  const stopped = (label) => `${label}: still running after 300 s, so it was stopped (the wasm step's limit on each call but its four cargo builds)`;
+  // A Wasm leg that hangs, and the native legs' call: each fails the step, named within main's 40 lines.
+  const hung = wasmWorld({ over: { "wasm ES-s2": () => ({ timedOut: true, stdout: "" }) } });
+  assert.equal(hung.ok, false, hung.out);
+  assert.ok(tail(hung.out).split("\n").includes(stopped("Wasm ES-s2")), tail(hung.out));
+  const legs = wasmWorld({ over: { legs: () => ({ timedOut: true, stdout: "", writes: [] }) } });
+  assert.equal(legs.ok, false, legs.out);
+  assert.ok(tail(legs.out).split("\n").includes(stopped("npt-lab hash")), tail(legs.out));
+});
+
 test("the wasm step judges PM-0004-21's two bites: each caught at its tick and field, or not exercised; anything else fails (U4d2, 2.4 (a) item 6)", () => {
   const fails = (world, ...said) => {
     assert.equal(world.ok, false, world.out);
@@ -2600,6 +2839,11 @@ test("the wasm step judges PM-0004-21's two bites: each caught at its tick and f
   fails(wasmWorld({ scan: CATCH_SCAN, over: { "std-sin compare": sinAt({ mismatching: 9, rec: "ES-s2", tick: 1_000_124, field: "vehicle[3].pos.y" }) } }), "NOT caught at the scan's ES-s2 tick 1000123, vehicle[3].pos.y: first mismatch ES-s2 tick 1000124", libc);
   fails(wasmWorld({ scan: CATCH_SCAN, over: { "std-sin compare": sinAt({ mismatching: 9, rec: "ES-s2", tick: 1_000_123, field: "vehicle[3].pos.x" }) } }), "field vehicle[3].pos.x", libc);
   fails(wasmWorld({ scan: CATCH_SCAN, over: { "std-sin compare": sinAt({}) } }), "NOT caught at the scan's ES-s2 tick 1000123, vehicle[3].pos.y: first mismatch none", libc);
+  // The miss says why a CPU or libc change can cause it (U4d2 round 1, FX-9: INF-3): glibc's sinf is chosen at load.
+  fails(
+    wasmWorld({ scan: CATCH_SCAN, over: { "std-sin compare": sinAt({}) } }),
+    `${libc}; glibc picks its sinf by the CPU's features when it loads (an IFUNC), so on another CPU or libc the scan is re-run there (u4d2-brief.md 2.4 (a) item 6; INF-3)`,
+  );
   // The scan data: "not catchable" passes and is printed (the pin test); a file with neither fails.
   fails(wasmWorld({ scan: "scanned on: nothing\n" }), `${STD_SIN_SCAN} records neither a catch on a manifest recording nor "not catchable at this site on x86-64 and wasm32"`);
   assert.deepEqual(readScan(CATCH_SCAN), {
