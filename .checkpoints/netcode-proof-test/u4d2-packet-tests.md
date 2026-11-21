# check.test.mjs (check.mjs's self-tests): git diff --no-index .gate/u4d2-prefix/spikes/netcode-proof/check.test.mjs .claude/worktrees/u4d2-build/spikes/netcode-proof/check.test.mjs
diff --git a/.gate/u4d2-prefix/spikes/netcode-proof/check.test.mjs b/.claude/worktrees/u4d2-build/spikes/netcode-proof/check.test.mjs
index 10442b7..fec574a 100644
--- a/.gate/u4d2-prefix/spikes/netcode-proof/check.test.mjs
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/check.test.mjs
@@ -13,9 +13,25 @@ import {
   BITE_ID,
   CHANNEL_ONLY,
   CLIPPY_TWINS,
+  H2_VECTOR,
+  HASH_MAP_ARM,
+  HASH_MAP_FIELD,
+  HASH_WASM_UNSAFE_FILE,
+  LEAF,
+  LOADER,
+  READ_BACK,
+  RECORDINGS_MANIFEST,
+  SIM_BUILDS,
+  STD_SIN_SCAN,
   STEPS,
+  UNSAFE_EXCEPTIONS,
+  WASM_BITES,
+  WASM_BUILDS,
   WASM_CRATES,
+  WASM_STEP_DIR,
   biteHomeProblems,
+  buildFlagProblems,
+  builtArtifact,
   cargoConfigProblems,
   cargoCounts,
   clippyTwinProblems,
@@ -26,6 +42,7 @@ import {
   exceptionLintProblems,
   expectedMarks,
   failedRun,
+  failingTests,
   layoutProblems,
   lexRust,
   linkUseProblems,
@@ -33,15 +50,34 @@ import {
   manifestKeyProblems,
   marksProblems,
   NOT_TICK_COUNTS,
+  readBackCheck,
+  readComparison,
+  readScan,
   referenceProblems,
   roomFloatProblems,
+  rustflagSources,
   SIM_DEPENDENCIES,
+  simFeatureCheck,
+  simSubtree,
   TICK_NAMED_SKIP,
   testBothBuilds,
   tickConstantProblems,
+  treeArgs,
   unsafeProblems,
+  wasmStep,
   workspaceLintProblems,
 } from "./check.mjs";
+import {
+  BEGIN,
+  HASH_WASM_EXPORTS,
+  LINKER_EXPORTS,
+  LOADER_EXPORTS,
+  STEP,
+  exportProblems,
+  instantiate,
+  moduleProblems,
+  replay,
+} from "./crates/hash-wasm/loader.mjs";
 
 const HERE = dirname(fileURLToPath(import.meta.url));
 
@@ -84,6 +120,7 @@ function clean() {
     "npt-log": n("serde", "serde_json"),
     "npt-judge": n("npt-log", "serde", "serde_json"),
     "npt-lab": n("npt-link", "npt-l1", "npt-sim", "npt-proto", "npt-room", "npt-server", "npt-client", "npt-log"),
+    "npt-hash-wasm": n("npt-sim"),
     quinn: n("quinn-proto", "tokio"),
     wtransport: n("quinn"),
     "tokio-tungstenite": n("tungstenite", "tokio"),
@@ -213,9 +250,32 @@ const placed = (f, between = "") =>
 /** npt-tcpinfo's crate-level deny of the two clippy unsafe lints, as lib.rs's first line. */
 const DENY = "#![deny(clippy::undocumented_unsafe_blocks, clippy::multiple_unsafe_ops_per_block)]\n";
 
-/** A clean crate tree for unsafeProblems: the two exceptions and two ordinary crates. */
+/** npt-hash-wasm's seven exports' signatures, as crates/hash-wasm/src/lib.rs writes them. */
+const EXPORT_SIGS = {
+  begin: "(bite: u32, arm_tick: u32) -> u32",
+  feed: "(word: u32)",
+  step: "() -> u32",
+  hash_lo: "() -> u32",
+  hash_hi: "() -> u32",
+  dump_len: "() -> u32",
+  dump_word: "(i: u32) -> u32",
+};
+/** One npt-hash-wasm export as its lib.rs writes it: the placed allow (a multi-line reason), its no_mangle, its body. */
+const exported = (name, sig = EXPORT_SIGS[name], between = "") =>
+  `#[allow(\n    unsafe_code,\n    reason = "the third exception (ADR 0002:264-265): this listed name; \\\n              integers only"\n)]\n` +
+  `${between}#[unsafe(no_mangle)]\npub extern "C" fn ${name}${sig} {\n    run()\n}\n`;
+/** npt-hash-wasm's clean lib.rs: a doc naming what it may not do, the seven exports, a helper. */
+const HASH_WASM_LIB =
+  "//! The seven exports; no `unsafe {}` block and no other #[allow(unsafe_code)] (a doc comment, not code).\n" +
+  `${HASH_WASM_EXPORTS.map((n) => exported(n)).join("")}fn run() -> u32 {\n    0\n}\n`;
+/** npt-hash-wasm's manifest as the fixtures give it: its own tables, unsafe_code denied. */
+const HASH_WASM_MANIFEST = '[package]\nname = "npt-hash-wasm"\n\n[lints.rust]\nunsafe_code = "deny"\n';
+
+/** A clean crate tree for unsafeProblems: the three exceptions and two ordinary crates. */
 function cleanCrates() {
   return {
+    "crates/hash-wasm/Cargo.toml": HASH_WASM_MANIFEST,
+    [HASH_WASM_UNSAFE_FILE]: HASH_WASM_LIB,
     "crates/client-wasm/Cargo.toml": '[package]\nname = "npt-client-wasm"\n\n[lints.rust]\nunsafe_code = "allow"\n',
     "crates/client-wasm/src/lib.rs": "pub fn create() {}\n",
     "crates/tcpinfo/Cargo.toml": '[package]\nname = "npt-tcpinfo"\n\n[lints.rust]\nunsafe_code = "deny"\n',
@@ -285,7 +345,7 @@ test("unsafe_code: an attribute relaxing it outside the two exceptions fails, in
     const p = unsafeWith((f) => {
       f["crates/net/src/planted.rs"] = code;
     });
-    assert.deepEqual(p, ["crates/net/src/planted.rs relaxes unsafe_code (only npt-client-wasm and npt-tcpinfo may)"], code);
+    assert.deepEqual(p, ["crates/net/src/planted.rs relaxes unsafe_code (only npt-client-wasm, npt-tcpinfo and npt-hash-wasm may)"], code);
   }
 });
 
@@ -483,7 +543,7 @@ test("unsafe code outside the two exception crates fails however the lint was re
     const n = code.match(/\bunsafe\b|\bglobal_asm\b/g).length;
     assert.deepEqual(
       p,
-      [`crates/room/src/evil.rs:${firstLine[what]} has unsafe code (${n} \`unsafe\` or \`global_asm\`; only npt-client-wasm and npt-tcpinfo may)`],
+      [`crates/room/src/evil.rs:${firstLine[what]} has unsafe code (${n} \`unsafe\` or \`global_asm\`; only npt-client-wasm, npt-tcpinfo and npt-hash-wasm may)`],
       what,
     );
   }
@@ -505,7 +565,7 @@ test("lexRust reads a char literal as one code point, so an emoji cannot hide co
   const p = unsafeWith((f) => {
     f["crates/room/src/emoji.rs"] = src;
   });
-  assert.deepEqual(p, ["crates/room/src/emoji.rs:4 has unsafe code (1 `unsafe` or `global_asm`; only npt-client-wasm and npt-tcpinfo may)"]);
+  assert.deepEqual(p, ["crates/room/src/emoji.rs:4 has unsafe code (1 `unsafe` or `global_asm`; only npt-client-wasm, npt-tcpinfo and npt-hash-wasm may)"]);
 });
 
 test("npt-tcpinfo: unsafe only inside srtt_us and unacked_and_unsent, unsafe_code named only by their allows", () => {
@@ -597,6 +657,7 @@ function tcpinfoLints(change) {
   const real = {
     [TCPINFO_MANIFEST]: readFileSync(join(HERE, TCPINFO_MANIFEST), "utf8"),
     "crates/client-wasm/Cargo.toml": readFileSync(join(HERE, "crates/client-wasm/Cargo.toml"), "utf8"),
+    "crates/hash-wasm/Cargo.toml": readFileSync(join(HERE, "crates/hash-wasm/Cargo.toml"), "utf8"),
   };
   const text = change(real[TCPINFO_MANIFEST]);
   const lineOf = (needle) => text.split("\n").findIndex((l) => l === needle) + 1;
@@ -748,11 +809,12 @@ test("a manifest naming unsafe-code (cargo's spelling of unsafe_code) is caught
   );
 });
 
-test("the two exceptions hold the workspace's other lints in their own tables, and their own level (review C2R-1)", () => {
+test("the three exceptions hold the workspace's other lints in their own tables, and their own level (review C2R-1; U4d2)", () => {
   const ws = readFileSync(join(HERE, "Cargo.toml"), "utf8");
   const T = "crates/tcpinfo/Cargo.toml";
   const W = "crates/client-wasm/Cargo.toml";
-  const real = { [T]: readFileSync(join(HERE, T), "utf8"), [W]: readFileSync(join(HERE, W), "utf8") };
+  const H = "crates/hash-wasm/Cargo.toml";
+  const real = { [T]: readFileSync(join(HERE, T), "utf8"), [W]: readFileSync(join(HERE, W), "utf8"), [H]: readFileSync(join(HERE, H), "utf8") };
   const read = (changed) => (p) => {
     if (!(p in real)) throw new Error(`no ${p}`);
     return changed[p] ?? real[p];
@@ -795,15 +857,24 @@ test("the two exceptions hold the workspace's other lints in their own tables, a
   assert.deepEqual(exceptionLintProblems(ws, read({ [W]: real[W].replace('disallowed_methods = "deny"\n', "") })), [
     `${W} has no [lints.clippy] disallowed_methods: the workspace's is "deny"`,
   ]);
-  // A lint the workspace adds must be added to both.
+  // A lint the workspace adds must be added to all three.
   const more = ws.replace('iter_over_hash_type = "deny"\n', 'iter_over_hash_type = "deny"\nundocumented_unsafe_blocks = "deny"\n');
   assert.deepEqual(exceptionLintProblems(more, read({})), [
     `${W} has no [lints.clippy] undocumented_unsafe_blocks: the workspace's is "deny"`,
     `${T} has no [lints.clippy] undocumented_unsafe_blocks: the workspace's is "deny"`,
+    `${H} has no [lints.clippy] undocumented_unsafe_blocks: the workspace's is "deny"`,
   ]);
   assert.deepEqual(exceptionLintProblems(ws, () => {
     throw new Error("absent");
-  }), [`${W} cannot be read`, `${T} cannot be read`]);
+  }), [`${W} cannot be read`, `${T} cannot be read`, `${H} cannot be read`]);
+  // npt-hash-wasm (the third, U4d2): deny, as npt-tcpinfo; its manifest at "allow" or "forbid" is refused by name, and
+  // so is a clippy key the workspace's tables do not hold, or one it drops.
+  const hw = (from, to) => exceptionLintProblems(ws, read({ [H]: real[H].replace(from, to) }));
+  for (const v of ['"allow"', '"forbid"', '"warn"']) assert.deepEqual(hw('unsafe_code = "deny"', `unsafe_code = ${v}`), [`${H} sets [lints.rust] unsafe_code = ${v}, not "deny"`], v);
+  assert.deepEqual(hw('iter_over_hash_type = "deny"\n', ""), [`${H} has no [lints.clippy] iter_over_hash_type: the workspace's is "deny"`]);
+  assert.deepEqual(hw("[lints.clippy]\n", '[lints.clippy]\nundocumented_unsafe_blocks = "allow"\n'), [
+    `${H} sets [lints.clippy] undocumented_unsafe_blocks = "allow", which the workspace does not`,
+  ]);
 });
 
 /** cargo metadata's shape for layoutProblems: members by folder, each with its targets ([kind, name, src, edition?]). */
@@ -1049,12 +1120,89 @@ const linkUse = (at, shown) =>
   `${at} names \`${shown}\` from npt_link: npt-proto and npt-client use npt-link for Channel only (catalogue item 2; ` +
   "u4-build-plan.md §4.1)";
 
+/** rustc 1.99.1's read-back of the target features per target in this container (`rustc --print cfg`; notes F1). */
+const CFG_BASE = {
+  "x86_64-unknown-linux-gnu": ["fxsr", "sse", "sse2"],
+  "wasm32-unknown-unknown": ["bulk-memory", "multivalue", "mutable-globals", "nontrapping-fptoint", "reference-types", "sign-ext"],
+};
+/**
+ * rustc 1.99.1's read-back given `flags` for `target`, as the advisor's U4d2 brief check ran it (A15; notes F21), as a
+ * runner result: a target feature of this target's is added or removed; one of the other target's only warns; a CPU
+ * name of this target's sets its list (haswell and native add avx, avx2 and fma on x86-64; bleeding-edge adds simd128
+ * and relaxed-simd on wasm32); a CPU name of the other target's empties wasm32's list and cuts x86-64's to sse, sse2.
+ */
+function cfgModel(target, flags) {
+  const x86 = target.startsWith("x86_64");
+  const own = x86 ? ["fma", "sse2", "sse3", "avx"] : ["simd128", "relaxed-simd", "sign-ext"];
+  let features = [...CFG_BASE[target]];
+  const options = [];
+  for (let i = 0; i < flags.length; i++) {
+    if (flags[i] === "-C" || flags[i] === "--codegen") options.push(flags[++i] ?? "");
+    else if (flags[i].startsWith("--codegen=")) options.push(flags[i].slice(10));
+    else if (flags[i].startsWith("-C")) options.push(flags[i].slice(2));
+  }
+  for (const o of options) {
+    const [name, value = ""] = [o.split("=")[0].replace(/_/g, "-"), o.split("=").slice(1).join("=")];
+    if (name === "target-cpu") {
+      if (["haswell", "native"].includes(value)) features = x86 ? [...CFG_BASE[target], "avx", "avx2", "fma"] : [];
+      else if (value === "x86-64") features = x86 ? [...CFG_BASE[target]] : [];
+      else if (["bleeding-edge", "mvp"].includes(value)) features = x86 ? ["sse", "sse2"] : value === "mvp" ? ["mutable-globals"] : [...CFG_BASE[target], "relaxed-simd", "simd128"];
+    } else if (name === "target-feature") {
+      for (const item of value.split(",")) {
+        const f = item.replace(/^[+-]/, "");
+        if (!own.includes(f)) continue;
+        features = item.startsWith("-") ? features.filter((x) => x !== f) : [...new Set([...features, f])];
+      }
+    }
+  }
+  const stdout = `debug_assertions\npanic="${x86 ? "unwind" : "abort"}"\n${features.sort().map((f) => `target_feature="${f}"`).join("\n")}\n`;
+  return { ok: true, status: 0, stdout, out: stdout };
+}
+/**
+ * One build's `cargo tree` output in treeArgs's form: the package, a dependency outside npt-sim's subtree (getrandom),
+ * npt-sim's subtree as the head's lock gives it (notes F4), then another package at npt-sim's depth; `extra` gives a
+ * package other features.
+ */
+const treeText = (pkg, bites, extra = {}) =>
+  `${[
+    `0${pkg} v0.0.0 (/w/crates/${pkg.slice(4)}) [${bites ? "bites" : ""}]`,
+    `1getrandom v0.2.17 [${extra.getrandom ?? ""}]`,
+    `1npt-sim v0.0.0 (/w/crates/sim) [${extra["npt-sim"] ?? (bites ? "bites" : "")}]`,
+    `2libm v0.2.16 [${extra.libm ?? "arch,default"}]`,
+    "2rand_chacha v0.10.0 []",
+    "3ppv-lite86 v0.2.21 [simd]",
+    `4zerocopy v0.8.63 [${extra.zerocopy ?? "simd"}]`,
+    "3rand_core v0.10.1 []",
+    "2rand_core v0.10.1 []",
+    "1serde v1.0.228 [std]",
+  ].join("\n")}\n`;
 /**
- * A spike folder for depsStep (the workspace manifest, the two exceptions, npt-room and a cargo config) and cargo
- * metadata's shape for it, from clean()'s graph: every member at crates/<name without npt->/ with one lib target there.
+ * A runner for depsStep's `cargo tree` and `rustc` calls: each tree from `trees["<pkg>[ bites]"]` (text), else
+ * treeText's; each read-back from cfgModel; `calls` records every call.
+ */
+function depsRunner({ trees = {}, cfg = cfgModel } = {}) {
+  const calls = [];
+  const ok = (stdout) => ({ ok: true, status: 0, stdout, out: stdout });
+  const runner = (cmd, args, cwd) => {
+    calls.push({ line: `${cmd} ${args.join(" ")}`, cwd });
+    if (cmd === "rustc" && args[0] === "--version") return ok("rustc 1.99.1 (58cf8f9c3 2026-10-13)\n");
+    if (cmd === "rustc" && args[0] === "--print") return cfg(args[3], args.slice(4));
+    if (cmd === "cargo" && args[0] === "tree") {
+      const key = `${args[args.indexOf("-p") + 1]}${args.includes("bites") ? " bites" : ""}`;
+      return ok(trees[key] ?? treeText(key.split(" ")[0], args.includes("bites")));
+    }
+    throw new Error(`depsRunner has no answer for ${cmd} ${args.join(" ")}`);
+  };
+  return { runner, calls };
+}
+
+/**
+ * A spike folder for depsStep (the workspace manifest, the three exceptions, npt-room and a cargo config) and cargo
+ * metadata's shape for it, from clean()'s graph: every member at crates/<name without npt->/ with one lib target there,
+ * npt-sim's package naming its three dependencies; and a runner for its `cargo tree` and `rustc` calls (depsRunner).
  * `plant` changes the files, `change` the graph.
  */
-function depsFixture(t, plant = () => {}, change = () => {}) {
+function depsFixture(t, plant = () => {}, change = () => {}, answers = {}) {
   const dir = mkdtempSync(join(tmpdir(), "npt-deps-"));
   t.after(() => rmSync(dir, { recursive: true, force: true }));
   const files = {
@@ -1063,6 +1211,8 @@ function depsFixture(t, plant = () => {}, change = () => {}) {
     "crates/tcpinfo/Cargo.toml": '[package]\nname = "npt-tcpinfo"\n\n[lints.rust]\nunsafe_code = "deny"\n\n[lints.clippy]\ndisallowed_types = "deny"\n',
     "crates/tcpinfo/src/lib.rs": `${DENY}mod linux;\n`,
     "crates/tcpinfo/src/linux.rs": `${placed("srtt_us")}${placed("unacked_and_unsent")}`,
+    "crates/hash-wasm/Cargo.toml": `${HASH_WASM_MANIFEST}\n[lints.clippy]\ndisallowed_types = "deny"\n`,
+    [HASH_WASM_UNSAFE_FILE]: HASH_WASM_LIB,
     "crates/room/Cargo.toml": '[package]\nname = "npt-room"\n\n[lints]\nworkspace = true\n',
     "crates/room/src/lib.rs": "pub fn f() {}\n",
     ".cargo/config.toml": '[build]\ntarget-dir = "t"\n',
@@ -1088,15 +1238,27 @@ function depsFixture(t, plant = () => {}, change = () => {}) {
     Object.assign(p, { source: null, manifest_path: join(folder, "Cargo.toml") });
     p.targets = [{ kind: ["lib"], name: p.name, src_path: join(folder, "src", "lib.rs"), edition: "2024" }];
   }
-  return { dir, meta: m, env: { CARGO_HOME: join(dir, "no-cargo-home") } };
+  const sim = m.packages.find((p) => p.name === "npt-sim");
+  if (sim) sim.dependencies = [...SIM_DEPENDENCIES.map((d) => ({ name: d, kind: null })), { name: "proptest", kind: "dev" }];
+  return { dir, meta: m, env: { CARGO_HOME: join(dir, "no-cargo-home") }, ...depsRunner(answers) };
 }
 
 test("the deps step is depsStep, and it calls every check: one plant per check, each reported (review C2R-5)", (t) => {
   assert.equal(STEPS.deps, depsStep, "check.mjs's deps step runs this function");
   const ok = depsFixture(t);
-  const clean = depsStep(ok.dir, ok.env, () => ({ ok: true, meta: ok.meta }));
+  const clean = depsStep(ok.dir, ok.env, () => ({ ok: true, meta: ok.meta }), ok.runner);
   assert.equal(clean.ok, true, clean.out);
-  assert.match(clean.out, /^held: .* \(6 crate files and 1 cargo config file\(s\) read\)$/);
+  assert.match(clean.out, /^held: .* \(8 crate files and 1 cargo config file\(s\) read; rustc 1\.99\.1 .*'s read-back, given no rustflags: .*; npt-sim's features, one set in npt-server, npt-lab, npt-hash-wasm: .*\)$/);
+  // The read-back and the features comparison run through the runner it is given, from the spike's folder.
+  assert.deepEqual(
+    ok.calls.map((c) => c.line),
+    [
+      "rustc --version",
+      ...READ_BACK.map((b) => `rustc --print cfg --target ${b.target}`),
+      ...[false, true].flatMap((bites) => SIM_BUILDS.map((b) => `cargo ${treeArgs(b, bites).join(" ")}`)),
+    ],
+  );
+  assert.ok(ok.calls.every((c) => c.cwd === ok.dir));
   const bad = depsFixture(
     t,
     (f) => {
@@ -1104,7 +1266,8 @@ test("the deps step is depsStep, and it calls every check: one plant per check,
       // A member's manifest too (review C2c N1: dropping depsStep's member-manifest loop had passed every test).
       f["crates/room/Cargo.toml"] += '\n[profile.dev]\nrustflags = ["--cap-lints", "warn"]\n';
       f["crates/tcpinfo/Cargo.toml"] = f["crates/tcpinfo/Cargo.toml"].replace('disallowed_types = "deny"\n', "");
-      f[".cargo/config.toml"] = `include = ["extra.toml"]\n${f[".cargo/config.toml"]}`;
+      // U4d2's build-flag check: a refused flag in a config's rustflags (the name scan; the read-back lists fma).
+      f[".cargo/config.toml"] = `include = ["extra.toml"]\n${f[".cargo/config.toml"]}rustflags = ["-C", "target-feature=+fma"]\n`;
       f["crates/room/src/evil.rs"] = UNSAFE_FN;
       // U4s's three greps (u4-build-plan.md §4.3, §4.11, §4.14).
       f["crates/room/src/ticks.rs"] = "pub const LOOP_CATCHUP: u32 = 7;\npub fn g() -> f64 {\n    0.0\n}\n";
@@ -1113,8 +1276,16 @@ test("the deps step is depsStep, and it calls every check: one plant per check,
       // U4a's two (u4-build-plan.md §2's Drift; u4s-carries.md carry 3).
       f["crates/proto/clippy.toml"] += "disallowed-methods = []\n";
       f["crates/proto/src/lib.rs"] = "use npt_link::Link;\n";
+      // U4d2's npt-hash-wasm rules: an unsafe block in its lib.rs.
+      f[HASH_WASM_UNSAFE_FILE] += "fn stray() {\n    unsafe {}\n}\n";
     },
-    (e) => (e["npt-sim"] = n("tokio")),
+    (e) => {
+      e["npt-sim"] = n("tokio");
+      // U4d2: npt-hash-wasm ships npt-sim only.
+      e["npt-hash-wasm"] = n("npt-sim", "serde");
+    },
+    // U4d2's features comparison: the server build's libm with one feature more.
+    { trees: { "npt-server": treeText("npt-server", false, { libm: "arch,default,std" }) } },
   );
   bad.meta.packages.find((p) => p.name === "npt-room").targets[0].src_path = join(bad.dir, "crates", "tcpinfo", "src", "linux.rs");
   bad.meta.packages.find((p) => p.name === "npt-proto").dependencies = [{ name: "npt-link", rename: "link" }];
@@ -1124,9 +1295,10 @@ test("the deps step is depsStep, and it calls every check: one plant per check,
   } catch {
     linked = false;
   }
-  const r = depsStep(bad.dir, bad.env, () => ({ ok: true, meta: bad.meta }));
+  const r = depsStep(bad.dir, bad.env, () => ({ ok: true, meta: bad.meta }), bad.runner);
   assert.equal(r.ok, false);
   const lines = r.out.split("\n");
+  const strayAt = `${HASH_WASM_UNSAFE_FILE}:${HASH_WASM_LIB.split("\n").length + 1}`;
   for (const [check, line] of [
     ["dependencyProblems", "leaf crate npt-sim depends on tokio"],
     ["workspaceLintProblems", 'Cargo.toml has no [workspace.lints.rust] unsafe_code = "forbid" for its crates to inherit'],
@@ -1138,30 +1310,51 @@ test("the deps step is depsStep, and it calls every check: one plant per check,
     ],
     ["layoutProblems", "npt-room's lib target npt-room compiles crates/tcpinfo/src/linux.rs, outside its folder crates/room/"],
     ["cargoConfigProblems", `${join(bad.dir, ".cargo", "config.toml")} has a key \`include\`: it loads a config file this check does not read`],
-    ["unsafeProblems", "crates/room/src/evil.rs:3 has unsafe code (1 `unsafe` or `global_asm`; only npt-client-wasm and npt-tcpinfo may)"],
+    ["unsafeProblems", "crates/room/src/evil.rs:3 has unsafe code (1 `unsafe` or `global_asm`; only npt-client-wasm, npt-tcpinfo and npt-hash-wasm may)"],
     ["tickConstantProblems", notAPath("crates/room/src/ticks.rs:1", "LOOP_CATCHUP", "7")],
     ["roomFloatProblems", roomFloat("crates/room/src/ticks.rs:2", "f64", 2)],
     ["biteHomeProblems", biteTwice("PM-0001-17/client-rounding", "crates/client/src/lib.rs:1", "crates/sim/src/grid.rs:1")],
     ["clippyTwinProblems", clippyDrift("crates/proto/clippy.toml:4", "crates/sim/clippy.toml:4 (its end)")],
     ["linkUseProblems", linkUse("crates/proto/src/lib.rs:1", "npt_link::Link")],
     ["dependencyProblems (a renamed npt-link)", "npt-proto renames its npt-link dependency to link"],
+    // U4d2's checks (u4d2-brief.md 2.4 (b), (c), (d)).
+    [
+      "buildFlagProblems",
+      `${join(bad.dir, ".cargo", "config.toml")}'s rustflags passes "-C target-feature=+fma": it enables fma, which plan.md §4.5's build flags refuse`,
+    ],
+    [
+      "readBackCheck",
+      'rustc\'s read-back for x86_64-unknown-linux-gnu (given "-C target-feature=+fma") lists target_feature="fma", which plan.md §4.5\'s build flags refuse',
+    ],
+    [
+      "simFeatureCheck",
+      "npt-sim's libm v0.2.16 resolves [arch, default, std] in the npt-server (x86_64-unknown-linux-gnu, measured) build and [arch, default] in the npt-lab (x86_64-unknown-linux-gnu, measured) build",
+    ],
+    [
+      "unsafeProblems (npt-hash-wasm)",
+      `${strayAt} has \`unsafe\` outside an attribute; npt-hash-wasm's only unsafe is the #[unsafe(no_mangle)] of its listed exports (${HASH_WASM_EXPORTS.join(", ")}) in ${HASH_WASM_UNSAFE_FILE}`,
+    ],
+    ["dependencyProblems (npt-hash-wasm)", "npt-hash-wasm depends on serde"],
   ]) {
     assert.ok(lines.includes(line), `${check}'s plant is not reported:\n${r.out}`);
   }
   if (!linked) t.diagnostic("symbolic links cannot be made here: the link refusal is not exercised");
   else assert.ok(lines.includes("crates/room/src/k.rs is a symbolic link: the scan does not follow it, and rustc would"), r.out);
   // The environment it is given is the one checked (process.env when check.mjs runs it).
-  const runner = depsStep(ok.dir, { ...ok.env, CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER: "true" }, () => ({ ok: true, meta: ok.meta }));
+  const runner = depsStep(ok.dir, { ...ok.env, CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER: "true" }, () => ({ ok: true, meta: ok.meta }), ok.runner);
   assert.equal(runner.out, "CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER in the environment runs the test binaries in cargo's place");
   // And so is its CARGO_HOME (review C2b F3): a config there that must be refused is read from the CARGO_HOME given, not
   // from this process's ~/.cargo.
   const home = join(ok.dir, "cargo-home");
   mkdirSync(home);
   writeFileSync(join(home, "config.toml"), '[build]\nrustc = "/opt/other/rustc"\n');
-  const homed = depsStep(ok.dir, { ...ok.env, CARGO_HOME: home }, () => ({ ok: true, meta: ok.meta }));
+  const homed = depsStep(ok.dir, { ...ok.env, CARGO_HOME: home }, () => ({ ok: true, meta: ok.meta }), ok.runner);
   assert.equal(homed.out, `${join(home, "config.toml")} has a key \`rustc\`: it replaces rustc or wraps the workspace's rustc calls`);
+  // The environment's rustflags reach the name scan and the read-back (U4d2): RUSTFLAGS="-C target-cpu=native".
+  const native = depsStep(ok.dir, { ...ok.env, RUSTFLAGS: "-C target-cpu=native" }, () => ({ ok: true, meta: ok.meta }), ok.runner);
+  assert.ok(native.out.split("\n").includes('RUSTFLAGS in the environment passes "-C target-cpu=native": plan.md §4.5\'s build flags refuse target-cpu=native'), native.out);
   // cargo metadata failing fails the step with its output.
-  assert.deepEqual(depsStep(ok.dir, ok.env, () => ({ ok: false, out: "error: no Cargo.lock" })), { ok: false, out: "error: no Cargo.lock" });
+  assert.deepEqual(depsStep(ok.dir, ok.env, () => ({ ok: false, out: "error: no Cargo.lock" }), ok.runner), { ok: false, out: "error: no Cargo.lock" });
 });
 
 test("the test step counts and names a test that reported a skip (review C2R-9)", () => {
@@ -1696,3 +1889,750 @@ test("a failed build's summary names the build and its first error, not cargo's
   // Output with no error line says so.
   assert.equal(lastLine(failedRun(failed("warning: build failed\n"), "x").out), "x failed: its output has no error line (the lines above)");
 });
+
+/** The last `n` lines of a text: what main prints of a failed step (its last 40). */
+const tail = (text, n = 40) => text.trimEnd().split("\n").slice(-n).join("\n");
+
+/** hashWasmProblems' messages (U4d2). */
+const HW_LIST = `its listed exports (${HASH_WASM_EXPORTS.join(", ")}) in ${HASH_WASM_UNSAFE_FILE}`;
+const hwMisplaced = (where, on) =>
+  `${where} relaxes unsafe_code on ${on}; npt-hash-wasm may only put #[allow(unsafe_code, reason = "...")] directly on ${HW_LIST}`;
+const hwCount = (fn, k) => `npt-hash-wasm's export ${fn} has ${k} #[allow(unsafe_code, reason = ...)] attributes, not 1`;
+const hwUnsafe = (where, what, how) => `${where} has \`${what}\` ${how}; npt-hash-wasm's only unsafe is the #[unsafe(no_mangle)] of ${HW_LIST}`;
+const hwSig = (where, fn, what) =>
+  `${where} exports ${fn} with ${what}: npt-hash-wasm's exports take and give u32 or i32 only (the advisor's U4D2-Q7 ruling)`;
+
+test("npt-hash-wasm, the third exception: one allow on each listed export, its no_mangle the only unsafe, u32 or i32 only (U4d2, 2.4 (d) and (e))", () => {
+  const F = HASH_WASM_UNSAFE_FILE;
+  const at = HASH_WASM_LIB.split("\n").length;
+  const lineOf = (text, start) => text.split("\n").findIndex((l) => l.startsWith(start)) + 1;
+  const hw = (change) => unsafeWith(change);
+  assert.deepEqual(hw(() => {}), [], "the clean fixture");
+  // The real crate holds (its seven exports as built).
+  const real = crateFiles(join(HERE, "crates")).files;
+  assert.ok(real.includes(F) && real.includes("crates/hash-wasm/Cargo.toml"));
+  assert.deepEqual(unsafeProblems(real, (f) => readFileSync(join(HERE, f), "utf8")), [], "the spike's own crates");
+  // plan :706's bite: a stray `unsafe {}` block, refused by file:line (rustc refuses it too, under the crate's deny).
+  assert.deepEqual(hw((f) => (f[F] += "fn stray() {\n    unsafe {}\n}\n")), [hwUnsafe(`${F}:${at + 1}`, "unsafe", "outside an attribute")]);
+  // An #[unsafe(no_mangle)] export not on the list, with its allow: the allow is misplaced, its unsafe refused.
+  const extra = 'pub extern "C" fn extra() -> u32 {';
+  assert.deepEqual(hw((f) => (f[F] += exported("extra", "() -> u32"))), [
+    hwMisplaced(`${F}:${at}`, `\`${extra}\``),
+    hwUnsafe(`${F}:${at + 5}`, "unsafe", `in #[unsafe(no_mangle)] on \`${extra}\``),
+  ]);
+  // A listed export with no allow, or two (a second stacked on step).
+  const bare = `#[unsafe(no_mangle)]\npub extern "C" fn begin${EXPORT_SIGS.begin} {\n    run()\n}\n`;
+  assert.deepEqual(hw((f) => (f[F] = HASH_WASM_LIB.replace(exported("begin"), bare))), [hwCount("begin", 0)]);
+  assert.deepEqual(hw((f) => (f[F] = HASH_WASM_LIB.replace(exported("step"), exported("step", EXPORT_SIGS.step, '#[allow(unsafe_code, reason = "twice")]\n')))), [
+    hwCount("step", 2),
+  ]);
+  // An allow on a helper; an inner allow; cfg_attr, expect or warn on an export; unsafe_code named outside an attribute.
+  assert.deepEqual(hw((f) => (f[F] += '#[allow(unsafe_code, reason = "a helper")]\nfn helper() {}\n')), [hwMisplaced(`${F}:${at}`, "`fn helper() {}`")]);
+  assert.deepEqual(hw((f) => (f[F] = `#![allow(unsafe_code)]\n${HASH_WASM_LIB}`)), [hwMisplaced(`${F}:1`, "its whole module or crate (#!)")]);
+  const beginLine = '`pub extern "C" fn begin(bite: u32, arm_tick: u32) -> u32 {`';
+  for (const attr of ["#[cfg_attr(all(), allow(unsafe_code))]\n", '#[expect(unsafe_code, reason = "x")]\n', '#[warn(unsafe_code, reason = "x")]\n', "#[allow(unsafe_code)]\n"]) {
+    const p = hw((f) => (f[F] = HASH_WASM_LIB.replace(exported("begin"), `${attr}${bare}`)));
+    assert.deepEqual(p, [hwMisplaced(`${F}:2`, beginLine), hwCount("begin", 0)], attr);
+  }
+  assert.deepEqual(hw((f) => (f[F] += "lax!(unsafe_code);\n")), [`${F}:${at} names unsafe_code outside an attribute (a macro can build the attribute from it)`]);
+  // Any other unsafe: an unsafe fn, an unsafe extern block, export_name or link_section in place of no_mangle, global_asm!.
+  assert.deepEqual(hw((f) => (f[F] += "pub unsafe fn f() {}\n")), [hwUnsafe(`${F}:${at}`, "unsafe", "outside an attribute")]);
+  assert.deepEqual(hw((f) => (f[F] += 'unsafe extern "C" {\n    fn now() -> u32;\n}\n')), [hwUnsafe(`${F}:${at}`, "unsafe", "outside an attribute")]);
+  for (const other of ['unsafe(export_name = "begin2")', 'unsafe(link_section = ".x")']) {
+    const p = hw((f) => (f[F] = HASH_WASM_LIB.replace("#[unsafe(no_mangle)]\npub extern \"C\" fn begin", `#[${other}]\npub extern "C" fn begin`)));
+    assert.deepEqual(p, [hwUnsafe(`${F}:7`, "unsafe", `in #[${other}] on ${beginLine}`)], other);
+  }
+  assert.deepEqual(hw((f) => (f[F] += 'core::arch::global_asm!("");\n')), [hwUnsafe(`${F}:${at}`, "global_asm", "outside an attribute")]);
+  // The written invariant's second clause (the advisor's U4D2-Q7 ruling): u32 or i32 in and out.
+  const feedAt = `${F}:${lineOf(HASH_WASM_LIB, 'pub extern "C" fn feed')}`;
+  assert.deepEqual(hw((f) => (f[F] = HASH_WASM_LIB.replace(exported("feed"), exported("feed", "(word: f64)")))), [hwSig(feedAt, "feed", "parameter word: f64")]);
+  const loAt = `${F}:${lineOf(HASH_WASM_LIB, 'pub extern "C" fn hash_lo')}`;
+  assert.deepEqual(hw((f) => (f[F] = HASH_WASM_LIB.replace(exported("hash_lo"), exported("hash_lo", "() -> u64")))), [hwSig(loAt, "hash_lo", "result u64")]);
+  assert.deepEqual(hw((f) => (f[F] = HASH_WASM_LIB.replace(exported("step"), exported("step", "() -> i32")))), [], "i32 is lowered as u32 is");
+  // Outside its own file, even a placed allow on a listed name is misplaced.
+  assert.deepEqual(hw((f) => (f["crates/hash-wasm/src/other.rs"] = exported("hash_hi"))), [
+    hwMisplaced("crates/hash-wasm/src/other.rs:1", '`pub extern "C" fn hash_hi() -> u32 {`'),
+    hwUnsafe("crates/hash-wasm/src/other.rs:6", "unsafe", 'in #[unsafe(no_mangle)] on `pub extern "C" fn hash_hi() -> u32 {`'),
+  ]);
+  // Its dependencies: npt-sim only; nothing depends on it, in any kind; it is a leaf crate and the third exception.
+  assert.deepEqual(problemsWith((e) => (e["npt-hash-wasm"] = n("npt-sim", "serde"))), ["npt-hash-wasm depends on serde"]);
+  assert.deepEqual(problemsWith((e) => (e["npt-hash-wasm"] = [])), ["npt-hash-wasm does not depend on npt-sim"]);
+  assert.deepEqual(problemsWith((e) => e["npt-lab"].push(["npt-hash-wasm", "dev"])), ["npt-lab depends on npt-hash-wasm"]);
+  assert.deepEqual(problemsWith((e) => (e["npt-hash-wasm"] = [...n("npt-sim"), ["proptest", "dev"]])), [], "a dev dependency does not ship");
+  assert.ok(LEAF.includes("npt-hash-wasm") && WASM_CRATES.includes("npt-hash-wasm"));
+  assert.deepEqual(UNSAFE_EXCEPTIONS, { "crates/client-wasm/": "npt-client-wasm", "crates/tcpinfo/": "npt-tcpinfo", "crates/hash-wasm/": "npt-hash-wasm" });
+});
+
+/**
+ * A hand-assembled WebAssembly module, a few dozen bytes: each of `names` exported, a function () -> i32 returning 0,
+ * but `memory`, a memory of one page; with `importFn`, a function env.now imported first.
+ */
+function wasmModule(names, { importFn = false } = {}) {
+  const leb = (n) => {
+    const out = [];
+    do {
+      let b = n & 0x7f;
+      n >>>= 7;
+      if (n) b |= 0x80;
+      out.push(b);
+    } while (n);
+    return out;
+  };
+  const str = (s) => [...leb(s.length), ...Buffer.from(s)];
+  const section = (id, body) => [id, ...leb(body.length), ...body];
+  const fns = names.filter((x) => x !== "memory");
+  const imported = importFn ? 1 : 0;
+  const body = [0, 0x41, 0, 0x0b];
+  return new Uint8Array([
+    0x00, 0x61, 0x73, 0x6d, 1, 0, 0, 0,
+    ...section(1, [1, 0x60, 0, 1, 0x7f]),
+    ...(importFn ? section(2, [1, ...str("env"), ...str("now"), 0x00, 0]) : []),
+    ...section(3, [...leb(fns.length), ...fns.map(() => 0)]),
+    ...(names.includes("memory") ? section(5, [1, 0x00, 1]) : []),
+    ...section(7, [...leb(names.length), ...names.flatMap((x) => (x === "memory" ? [...str(x), 0x02, 0] : [...str(x), 0x00, ...leb(imported + fns.indexOf(x))]))]),
+    ...section(10, [...leb(fns.length), ...fns.flatMap(() => [...leb(body.length), ...body])]),
+  ]);
+}
+
+test("the loader's list is the seven exports and wasm-ld's memory; an export more is refused by name, and a module that imports does not instantiate (U4d2, 2.2, 2.4 (d))", () => {
+  assert.deepEqual(HASH_WASM_EXPORTS, ["begin", "feed", "step", "hash_lo", "hash_hi", "dump_len", "dump_word"]);
+  assert.deepEqual(LINKER_EXPORTS, { memory: "memory" });
+  assert.deepEqual(LOADER_EXPORTS, { ...Object.fromEntries(HASH_WASM_EXPORTS.map((x) => [x, "function"])), memory: "memory" });
+  assert.deepEqual([BEGIN, STEP], [{ BEGUN: 0, REFUSED: 1 }, { NEED_MORE: 0, STEPPED: 1, DONE: 2, FAILED: 3 }]);
+  const listed = [...HASH_WASM_EXPORTS, "memory"];
+  assert.ok(wasmModule([...listed, "extra"]).length < 256, "a few hundred bytes at most, built here");
+  assert.deepEqual(exportProblems(wasmModule(listed)), [], "the clean module");
+  assert.deepEqual(exportProblems(wasmModule([...listed, "extra"])), ['the module exports "extra" (a function), which is not on the loader\'s list']);
+  assert.deepEqual(exportProblems(wasmModule(listed.filter((x) => x !== "dump_word"))), ['the module does not export "dump_word"']);
+  assert.match(exportProblems(new Uint8Array([0, 1, 2, 3]))[0], /^not a WebAssembly module: /);
+  assert.deepEqual(moduleProblems(new WebAssembly.Module(wasmModule(listed))), []);
+  // Refused before instantiating; and with an empty import object a module that imports a function fails.
+  assert.throws(() => instantiate(new WebAssembly.Module(wasmModule([...listed, "extra"]))), /^Error: refused: the module exports "extra"/);
+  assert.throws(
+    () => instantiate(new WebAssembly.Module(wasmModule(listed, { importFn: true }))),
+    /refused: the module does not instantiate with no imports \(it imports env\.now \(function\)\)/,
+  );
+  assert.equal(typeof instantiate(new WebAssembly.Module(wasmModule(listed))).begin, "function");
+});
+
+test("the loader streams a recording in chunks, numbers its ticks from the first, and names the recording and the last tick on a trap or a failure (U4d2, 2.2)", (t) => {
+  const dir = mkdtempSync(join(tmpdir(), "npt-loader-"));
+  t.after(() => rmSync(dir, { recursive: true, force: true }));
+  const path = join(dir, "r.rec");
+  // 200,003 bytes: 50,001 words, the last zero-padded, so more than one 64 KiB chunk.
+  writeFileSync(path, Buffer.alloc(200_003, 1));
+  const fake = (steps, trapAt = Infinity) => {
+    const fed = [];
+    let feeds = 0;
+    let n = 0;
+    return {
+      fed,
+      exports: {
+        begin: () => 0,
+        feed: (w) => {
+          feeds++;
+          if (fed.length < 2 || feeds === 50_001) fed.push(w >>> 0);
+        },
+        step: () => {
+          if (feeds === 0) return STEP.NEED_MORE;
+          if (n >= trapAt) throw new WebAssembly.RuntimeError("unreachable");
+          if (n < steps && feeds >= (n + 1) * 10_000) return (n++, STEP.STEPPED);
+          return feeds === 50_001 ? (n < steps ? STEP.NEED_MORE : STEP.DONE) : STEP.NEED_MORE;
+        },
+        hash_hi: () => -1,
+        hash_lo: () => n,
+        dump_len: () => 2,
+        dump_word: (i) => (i ? -2 : 7),
+      },
+    };
+  };
+  const ok = fake(5);
+  const r = replay(ok.exports, { path, name: "TC-s3", firstTick: 0xffff_fffe, dumpAt: 0 });
+  assert.equal(r.ok, true, r.error);
+  assert.deepEqual(r.lines, [
+    "h TC-s3 4294967294 ffffffff00000001",
+    "h TC-s3 4294967295 ffffffff00000002",
+    "h TC-s3 0 ffffffff00000003",
+    "h TC-s3 1 ffffffff00000004",
+    "h TC-s3 2 ffffffff00000005",
+    "stepped 5",
+  ]);
+  assert.equal(r.dump, "dump TC-s3 0\nhash ffffffff00000003\n00000007\nfffffffe\n");
+  assert.deepEqual(ok.fed, [0x01010101, 0x01010101, 0x00010101], "little-endian words, the last zero-padded");
+  // A trap, a FAILED status, a file that ends inside a tick, a dump tick never stepped, a refused begin.
+  const trap = replay(fake(5, 3).exports, { path, name: "SS-s1", firstTick: 10 });
+  assert.equal(trap.error, "SS-s1: the Wasm leg trapped after tick 12: unreachable");
+  const failedStep = fake(5);
+  failedStep.exports.step = () => STEP.FAILED;
+  assert.equal(replay(failedStep.exports, { path, name: "SS-s1", firstTick: 0 }).error, "SS-s1: step() returned FAILED after no tick");
+  assert.equal(replay(fake(9).exports, { path, name: "ES-s2", firstTick: 0 }).error, "ES-s2: the file ends inside a tick, after tick 4");
+  assert.equal(replay(fake(5).exports, { path, name: "ES-s2", firstTick: 0, dumpAt: 99 }).error, "ES-s2: no tick 99 was stepped, so no dump was taken (after tick 4)");
+  const refused = fake(5);
+  refused.exports.begin = () => 1;
+  assert.match(replay(refused.exports, { path, name: "SS-s1", firstTick: 0, bite: 1, arm: 5 }).error, /^SS-s1: begin\(1, 5\) returned 1, not BEGUN/);
+  assert.equal(trap.lines.at(-1), `failed ${trap.error}`);
+});
+
+test("the build flags: +fma, target-cpu=native, +simd128 and +relaxed-simd refused in every form and source; -fma, +sse2 and target-cpu=x86-64 pass the name scan (U4d2, 2.4 (c) (i))", () => {
+  const scan = (env, configs = []) => buildFlagProblems(rustflagSources(env, configs));
+  const forms = (o) => [["-C", o], [`-C${o}`], ["--codegen", o], [`--codegen=${o}`]];
+  const sources = [
+    ["RUSTFLAGS", " "],
+    ["CARGO_ENCODED_RUSTFLAGS", "\x1f"],
+    ["CARGO_BUILD_RUSTFLAGS", " "],
+    ["CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS", " "],
+    ["CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS", " "],
+  ];
+  const cases = [];
+  for (const f of ["fma", "simd128", "relaxed-simd"]) {
+    for (const o of [`target-feature=+${f}`, `target-feature=+sse2,+${f}`, `target_feature=+${f}`, `target-feature=-${f},+${f}`, `target-feature=${f}`]) {
+      cases.push([o, `it enables ${f}, which plan.md §4.5's build flags refuse`]);
+    }
+  }
+  for (const o of ["target-cpu=native", "target_cpu=native"]) cases.push([o, "plan.md §4.5's build flags refuse target-cpu=native"]);
+  for (const [o, why] of cases) {
+    for (const tokens of forms(o)) {
+      for (const [k, sep] of sources) {
+        assert.deepEqual(scan({ PATH: "/bin", [k]: tokens.join(sep) }), [`${k} in the environment passes ${JSON.stringify(`-C ${o}`)}: ${why}`], `${k}=${tokens}`);
+      }
+      // A cargo config's rustflags, as an array and as a string, in [build] and in a target table.
+      for (const text of [`[build]\nrustflags = ${JSON.stringify(tokens)}\n`, `[target.wasm32-unknown-unknown]\nrustflags = ${JSON.stringify(tokens.join(" "))}\n`]) {
+        assert.deepEqual(scan({}, [{ path: "c.toml", text }]), [`c.toml's rustflags passes ${JSON.stringify(`-C ${o}`)}: ${why}`], text);
+      }
+    }
+  }
+  // The three controls: no refused flag is named.
+  for (const flags of ["-C target-feature=-fma", "-C target-feature=+sse2", "-C target-cpu=x86-64", "-C opt-level=3 -D warnings"]) {
+    assert.deepEqual(scan({ RUSTFLAGS: flags }), [], flags);
+  }
+  // Each source is read: the sources' names and flags as rustc gets them.
+  assert.deepEqual(rustflagSources({ RUSTFLAGS: " -C  a=1 ", CARGO_ENCODED_RUSTFLAGS: "-C b=2\x1f-C\x1fc=3", HOME: "/root" }, [{ path: "c.toml", text: 'rustflags = "-C d=4"\n[x]\nrustflags = ["-C", "e=5"]\n' }]), [
+    { where: "RUSTFLAGS in the environment", flags: ["-C", "a=1"] },
+    { where: "CARGO_ENCODED_RUSTFLAGS in the environment", flags: ["-C b=2", "-C", "c=3"] },
+    { where: "c.toml's rustflags", flags: ["-C", "d=4"] },
+    { where: "c.toml's rustflags", flags: ["-C", "e=5"] },
+  ]);
+});
+
+/** readBackCheck's refusal of a read-back that lists none of a target's own features. */
+const nonVacuous = (target, present, given) =>
+  `rustc's read-back for ${target} (${given}) lists no target_feature="${present}": it read nothing a build for it has (a CPU ` +
+  "name meant for the other target, or a parse that reads nothing), so it proves nothing";
+const listsRefused = (target, f, given = "given no rustflags") =>
+  `rustc's read-back for ${target} (${given}) lists target_feature="${f}", which plan.md §4.5's build flags refuse`;
+
+test("rustc's read-back fails the step on fma (x86-64) or simd128 and relaxed-simd (wasm32), on a list with none of its own features, and on a failed run; every rustflag of every source reaches it (U4d2, 2.4 (c) (ii))", () => {
+  const X = "x86_64-unknown-linux-gnu";
+  const W = "wasm32-unknown-unknown";
+  const check = (env, cfg = cfgModel, configs = []) => {
+    const r = depsRunner({ cfg });
+    return { ...readBackCheck(rustflagSources(env, configs), r.runner, "/w"), calls: r.calls };
+  };
+  assert.deepEqual(READ_BACK, [
+    { target: X, refused: ["fma"], present: "sse2" },
+    { target: W, refused: ["simd128", "relaxed-simd"], present: "sign-ext" },
+  ]);
+  const clean = check({});
+  assert.deepEqual(clean.problems, []);
+  assert.equal(
+    clean.line,
+    `rustc 1.99.1 (58cf8f9c3 2026-10-13)'s read-back, given no rustflags: ${X} fxsr, sse, sse2; ${W} bulk-memory, multivalue, mutable-globals, nontrapping-fptoint, reference-types, sign-ext`,
+  );
+  // Recorded read-backs that list a refused feature, each alone.
+  const plus = (target, f) => (t, flags) => {
+    const r = cfgModel(t, flags);
+    return t === target ? { ...r, stdout: `${r.stdout}target_feature="${f}"\n`, out: `${r.stdout}target_feature="${f}"\n` } : r;
+  };
+  assert.deepEqual(check({}, plus(X, "fma")).problems, [listsRefused(X, "fma")]);
+  assert.deepEqual(check({}, plus(W, "simd128")).problems, [listsRefused(W, "simd128")]);
+  assert.deepEqual(check({}, plus(W, "relaxed-simd")).problems, [listsRefused(W, "relaxed-simd")]);
+  // A read-back missing sse2 or sign-ext; one that lists nothing; one that exits non-zero.
+  const minus = (target, f) => (t, flags) => {
+    const r = cfgModel(t, flags);
+    const stdout = t === target ? r.stdout.replace(`target_feature="${f}"\n`, "") : r.stdout;
+    return { ...r, stdout, out: stdout };
+  };
+  assert.deepEqual(check({}, minus(X, "sse2")).problems, [nonVacuous(X, "sse2", "given no rustflags")]);
+  assert.deepEqual(check({}, minus(W, "sign-ext")).problems, [nonVacuous(W, "sign-ext", "given no rustflags")]);
+  assert.deepEqual(check({}, () => ({ ok: true, status: 0, stdout: "", out: "" })).problems, [nonVacuous(X, "sse2", "given no rustflags"), nonVacuous(W, "sign-ext", "given no rustflags")]);
+  const broken = (t, flags) => (t === W ? { ok: false, status: 1, stdout: "", out: "error: Error loading target specification\n" } : cfgModel(t, flags));
+  assert.deepEqual(check({}, broken).problems, [`rustc --print cfg --target ${W} (given no rustflags) failed: error: Error loading target specification`]);
+  // The live plants' flags (2.7), through rustc's behaviour as the advisor's check ran it (A15; cfgModel): a CPU name
+  // the name scan cannot see.
+  assert.deepEqual(check({ RUSTFLAGS: "-C target-cpu=haswell" }).problems, [listsRefused(X, "fma", 'given "-C target-cpu=haswell"'), nonVacuous(W, "sign-ext", 'given "-C target-cpu=haswell"')]);
+  assert.deepEqual(check({ CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS: "-C target-cpu=bleeding-edge" }).problems, [
+    listsRefused(W, "simd128", 'given "-C target-cpu=bleeding-edge"'),
+    listsRefused(W, "relaxed-simd", 'given "-C target-cpu=bleeding-edge"'),
+  ]);
+  // The three controls through the whole check: -fma and +sse2 pass; target-cpu=x86-64 fails on wasm32's read-back.
+  for (const f of ["-C target-feature=-fma", "-C target-feature=+sse2"]) {
+    assert.deepEqual([...buildFlagProblems(rustflagSources({ RUSTFLAGS: f }, [])), ...check({ RUSTFLAGS: f }).problems], [], f);
+  }
+  assert.deepEqual(buildFlagProblems(rustflagSources({ RUSTFLAGS: "-C target-cpu=x86-64" }, [])), []);
+  assert.deepEqual(check({ RUSTFLAGS: "-C target-cpu=x86-64" }).problems, [nonVacuous(W, "sign-ext", 'given "-C target-cpu=x86-64"')]);
+  // The argv pin: four sources, each a distinct harmless flag; every one is in each rustc call, which runs where told.
+  const env = {
+    RUSTFLAGS: "-C debuginfo=0",
+    CARGO_ENCODED_RUSTFLAGS: "-C\x1fopt-level=1",
+    CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS: "-C codegen-units=1",
+  };
+  const pinned = check(env, cfgModel, [{ path: "c.toml", text: '[build]\nrustflags = ["-C", "overflow-checks=on"]\n' }]);
+  assert.deepEqual(pinned.problems, []);
+  const cfgCalls = pinned.calls.filter((c) => c.line.startsWith("rustc --print cfg"));
+  assert.deepEqual(cfgCalls.map((c) => c.line.split(" ").slice(0, 5).join(" ")), [`rustc --print cfg --target ${X}`, `rustc --print cfg --target ${W}`]);
+  for (const c of cfgCalls) {
+    for (const f of ["debuginfo=0", "opt-level=1", "codegen-units=1", "overflow-checks=on"]) assert.ok(c.line.includes(`-C ${f}`), `${f} in ${c.line}`);
+  }
+  assert.ok(pinned.calls.every((c) => c.cwd === "/w"));
+});
+
+test("npt-sim's resolved features are one set in the server, npt-lab and npt-hash-wasm builds, like for like; a difference anywhere in its subtree fails (U4d2, 2.4 (b), PM-0004-74)", () => {
+  const sim = {
+    id: "sim",
+    name: "npt-sim",
+    dependencies: [...SIM_DEPENDENCIES.map((d) => ({ name: d, kind: null })), { name: "proptest", kind: "dev" }],
+  };
+  const meta = { packages: [sim], workspace_members: ["sim"] };
+  const check = (trees) => {
+    const r = depsRunner({ trees });
+    return { ...simFeatureCheck(meta, r.runner, "/w"), calls: r.calls };
+  };
+  const subtree = "libm [arch, default], rand_chacha [], ppv-lite86 [simd], zerocopy [simd], rand_core []";
+  const clean = check({});
+  assert.deepEqual(clean.problems, []);
+  assert.equal(clean.line, `npt-sim's features, one set in npt-server, npt-lab, npt-hash-wasm: measured: npt-sim [], ${subtree}; bites: npt-sim [bites], ${subtree}`);
+  // The pairing pinned: three measured trees, then three bites trees, each build's package and target; a measured list
+  // is never paired with a bites list (npt-sim's own `bites` differs between the sets and passes).
+  assert.deepEqual(SIM_BUILDS, [
+    { pkg: "npt-server", target: "x86_64-unknown-linux-gnu" },
+    { pkg: "npt-lab", target: "x86_64-unknown-linux-gnu" },
+    { pkg: "npt-hash-wasm", target: "wasm32-unknown-unknown" },
+  ]);
+  assert.deepEqual(
+    clean.calls.map((c) => c.line),
+    [false, true].flatMap((bites) =>
+      SIM_BUILDS.map((b) => `cargo tree --locked -e normal --target ${b.target} -p ${b.pkg}${bites ? " --features bites" : ""} --no-dedupe --prefix depth --format {p} [{f}]`),
+    ),
+  );
+  const S = "npt-server (x86_64-unknown-linux-gnu, measured)";
+  const L = "npt-lab (x86_64-unknown-linux-gnu, measured)";
+  const HW = "npt-hash-wasm (wasm32-unknown-unknown, measured)";
+  // PM-0004-74's bite: the server build's libm with one feature more, named with both builds and the features.
+  assert.deepEqual(check({ "npt-server": treeText("npt-server", false, { libm: "arch,default,std" }) }).problems, [
+    `npt-sim's libm v0.2.16 resolves [arch, default, std] in the ${S} build and [arch, default] in the ${L} build`,
+    `npt-sim's libm v0.2.16 resolves [arch, default, std] in the ${S} build and [arch, default] in the ${HW} build`,
+  ]);
+  // A transitive dependency only (zerocopy, below rand_chacha through ppv-lite86), in the bites set: a walk of npt-sim's
+  // direct dependencies alone would pass it. Every pair is compared, so each build it sets apart is named.
+  assert.deepEqual(check({ "npt-hash-wasm bites": treeText("npt-hash-wasm", true, { zerocopy: "simd,std" }) }).problems, [
+    "npt-sim's zerocopy v0.8.63 resolves [simd] in the npt-server (x86_64-unknown-linux-gnu, bites) build and [simd, std] in the npt-hash-wasm (wasm32-unknown-unknown, bites) build",
+    "npt-sim's zerocopy v0.8.63 resolves [simd] in the npt-lab (x86_64-unknown-linux-gnu, bites) build and [simd, std] in the npt-hash-wasm (wasm32-unknown-unknown, bites) build",
+  ]);
+  // ADR 0004:1184's control: a feature on a package outside npt-sim's subtree passes.
+  assert.deepEqual(check({ "npt-server": treeText("npt-server", false, { getrandom: "std" }) }).problems, []);
+  // npt-sim's own features differing within one set fail (2.7's live plant: hash-wasm's npt-sim with `bites`).
+  assert.deepEqual(check({ "npt-hash-wasm": treeText("npt-hash-wasm", false, { "npt-sim": "bites" }) }).problems, [
+    `npt-sim's npt-sim v0.0.0 resolves [] in the ${S} build and [bites] in the ${HW} build`,
+    `npt-sim's npt-sim v0.0.0 resolves [] in the ${L} build and [bites] in the ${HW} build`,
+  ]);
+  // Not vacuous: a tree with no npt-sim subtree, or one missing a dependency npt-sim's manifest names, fails.
+  const noSim = "0npt-lab v0.0.0 (/w/crates/lab) []\n1serde v1.0.228 [std]\n";
+  assert.deepEqual(check({ "npt-lab": noSim }).problems, [`the ${L} build's tree holds no npt-sim subtree, so its features prove nothing`]);
+  const noLibm = treeText("npt-hash-wasm", false).replace("2libm v0.2.16 [arch,default]\n", "");
+  assert.deepEqual(check({ "npt-hash-wasm": noLibm }).problems, [
+    `the ${HW} build's npt-sim subtree lacks libm, which crates/sim/Cargo.toml names`,
+    `npt-sim's libm v0.2.16 resolves [arch, default] in the ${S} build and is not in its npt-sim subtree in the ${HW} build`,
+    `npt-sim's libm v0.2.16 resolves [arch, default] in the ${L} build and is not in its npt-sim subtree in the ${HW} build`,
+  ]);
+  // A cargo tree that fails is a problem, not a pass.
+  const r = depsRunner();
+  const failing = (cmd, args, cwd) => (args.includes("npt-lab") ? { ok: false, status: 101, stdout: "", out: "error: no Cargo.lock\n" } : r.runner(cmd, args, cwd));
+  assert.deepEqual(simFeatureCheck(meta, failing, "/w").problems, [`cargo tree for the ${L} build failed: error: no Cargo.lock`, "cargo tree for the npt-lab (x86_64-unknown-linux-gnu, bites) build failed: error: no Cargo.lock"]);
+  // simSubtree reads the first npt-sim to the next line at its depth or above.
+  assert.deepEqual([...simSubtree(treeText("npt-server", false)).keys()], ["npt-sim v0.0.0", "libm v0.2.16", "rand_chacha v0.10.0", "ppv-lite86 v0.2.21", "zerocopy v0.8.63", "rand_core v0.10.1"]);
+  assert.equal(simSubtree(noSim), null);
+});
+
+test("the test step names every failing test from libtest's failures: list beside its summary, in the lines main prints (carry 65; j-r2 B2.3)", () => {
+  // libtest's stdout for a failing binary: the panics' failures: block, then the names' one.
+  const libtest = (names) =>
+    `running ${names.length + 1} tests\ntest a ... ok\n${names.map((x) => `test ${x} ... FAILED`).join("\n")}\n\nfailures:\n\n` +
+    `${names.map((x) => `---- ${x} stdout ----\n\nthread '${x}' panicked at src/lib.rs:1:1:\nboom\n`).join("\n")}\n\nfailures:\n` +
+    `${names.map((x) => `    ${x}`).join("\n")}\n\ntest result: FAILED. 1 passed; ${names.length} failed; 0 ignored; 0 measured; 0 filtered out\n\n`;
+  // `run` joins stdout before stderr, and stderr's Compiling and Running lines push the names out of main's 40.
+  const stderr = `${Array.from({ length: 60 }, (_, i) => `   Compiling c${i} v0.0.0`).join("\n")}\n     Running unittests src/lib.rs (x)\nerror: test failed, to rerun pass \`-p npt-lab --lib\`\n`;
+  const failing = (names) => ({ ok: false, status: 101, stdout: libtest(names), out: `${libtest(names)}${stderr}` });
+  assert.ok(!tail(failing(["tests::h4_one"]).out).includes("    tests::h4_one"), "without the line, main's 40 lines miss the name");
+  // One failing test.
+  let s = testBothBuilds(recorder([failing(["tests::h4_one"])]).runner);
+  assert.ok(tail(s.out).includes("the measured build's failing tests (libtest's failures: list): tests::h4_one"), tail(s.out));
+  assert.equal(lastLine(s.out), "the measured build's tests (1 tests passed, 1 failed, 0 reported a skip) failed: error: test failed, to rerun pass `-p npt-lab --lib`");
+  // Two failing tests in one test binary, both named.
+  s = testBothBuilds(recorder([failing(["hash::tests::h5_a", "probe::tests::h7_b"])]).runner);
+  assert.ok(tail(s.out).includes("the measured build's failing tests (libtest's failures: list): hash::tests::h5_a, probe::tests::h7_b"), tail(s.out));
+  // A failure in the bites build only, named with the build.
+  const passed = { ok: true, status: 0, stdout: "test result: ok. 5 passed; 0 failed; 0 ignored\n", out: "test result: ok. 5 passed; 0 failed; 0 ignored\n" };
+  s = testBothBuilds(recorder([passed, failing(["hash::tests::h6_the_bites_build"])]).runner);
+  assert.ok(tail(s.out).includes("the bites build's failing tests (libtest's failures: list): hash::tests::h6_the_bites_build"), tail(s.out));
+  assert.match(lastLine(s.out), /^the bites build's tests \(measured build: 5 tests passed, 0 failed, 0 reported a skip; bites build: 1 tests passed, 1 failed, 0 reported a skip\) failed: /);
+  // A failed run with no failures: list (a build error) says so; failingTests reads only the names' list.
+  s = testBothBuilds(recorder([{ ok: false, status: 101, stdout: "", out: "error[E0425]: cannot find value `x`\n" }]).runner);
+  assert.ok(tail(s.out).includes("the measured build: no failing test in libtest's failures: list (a build or harness error: the lines above)"));
+  assert.deepEqual(failingTests(libtest(["x::y", "x::z"])), ["x::y", "x::z"]);
+  assert.deepEqual(failingTests("test result: ok. 1 passed; 0 failed\n"), []);
+});
+
+/** The wasm step's folders and copies, as it lays them out under WASM_STEP_DIR. */
+const WS = {
+  bin: join(WASM_STEP_DIR, "bin"),
+  rec: join(WASM_STEP_DIR, "recordings"),
+  meas: join(WASM_STEP_DIR, "measured"),
+  hm: join(WASM_STEP_DIR, "hash-map"),
+  sin: join(WASM_STEP_DIR, "std-sin"),
+};
+const LAB = join(WS.bin, "npt-lab");
+const LAB_BITES = join(WS.bin, "npt-lab-bites");
+const WASM_M = join(WS.bin, "npt-hash-wasm.wasm");
+const WASM_B = join(WS.bin, "npt-hash-wasm-bites.wasm");
+const RECS = [
+  { name: "SS-s1", first: 0 },
+  { name: "ES-s2", first: 1_000_000 },
+  { name: "TC-s3", first: 4_294_962_796 },
+];
+const COUNTS3 = "SS-s1 9000, ES-s2 9000, TC-s3 9000";
+/** A compare_text line (npt-lab's): equal, or a first mismatch at `rec` `tick` with `field` (or none). */
+const cmpLine = (a, b, { compared = 27_000, mismatching = 0, rec, tick, field, counts = COUNTS3 } = {}) =>
+  mismatching === 0
+    ? `${a} against ${b}: ${compared} ticks compared, 0 mismatching, equal (${counts})`
+    : `${a} against ${b}: ${compared} ticks compared, ${mismatching} mismatching; first mismatch at entry 7: ${rec} tick ${tick} hash ` +
+      `00000000000000aa against ${rec} tick ${tick} hash 00000000000000bb; ${field ? `field ${field} (word 9): 1 and 2` : "no field (no dump at that tick)"} (${counts})`;
+const NOT_CATCHABLE_SCAN = "# std-sin's scan\nnot catchable at this site on x86-64 and wasm32\nscanned on: the test's tree\nlibc: ldd (test) 2.39\n";
+/** A scan with two catches, listed out of the manifest's order (ES-s2 comes before TC-s3 there, so ES-s2's is armed). */
+const CATCH_SCAN = "catch TC-s3 4294962800 vehicle[0].pos.y\ncatch ES-s2 1000123 vehicle[3].pos.y\nlibc: ldd (Ubuntu GLIBC 2.39-0ubuntu8.6) 2.39\n";
+
+/**
+ * A recorded wasm step: a runner that answers each command as cargo, npt-lab and the loader do (an equal run, the
+ * hash-map bite caught natively and not catchable on wasm32, std-sin by `scan`), `over[kind]` replacing one answer,
+ * and an in-memory io (the manifest and the scan data read, the files each command writes known to exist).
+ */
+function wasmWorld({ over = {}, scan = NOT_CATCHABLE_SCAN } = {}) {
+  const calls = [];
+  const files = new Set();
+  const copies = [];
+  const manifest = { recordings: RECS.map((r) => ({ name: r.name, first_tick: r.first, ticks: 9_000 })) };
+  const arg = (args, k) => (args.includes(k) ? args[args.indexOf(k) + 1] : null);
+  const kindOf = (cmd, args) => {
+    if (cmd === "cargo") return `build ${args.includes("npt-hash-wasm") ? "wasm" : "lab"}${args.includes("bites") ? " bites" : ""}`;
+    if (cmd === process.execPath) {
+      const out = arg(args, "--out");
+      if (out.startsWith(WS.hm)) return `hash-map wasm ${out.endsWith("wasm-1.lines") ? 1 : 2}`;
+      if (out.startsWith(WS.sin)) return "std-sin wasm";
+      if (arg(args, "--dump-at") === "end") return `h2 ${arg(args, "--name")}`;
+      return `${arg(args, "--dump-at") ? "wasm dump" : "wasm"} ${arg(args, "--name")}`;
+    }
+    if (["bites", "vectors"].includes(args[1])) return args[1];
+    const where = arg(args, "--out") ?? arg(args, "--ours");
+    const area = where.startsWith(WS.hm) ? "hash-map " : where.startsWith(WS.sin) ? "std-sin " : "";
+    if (args[1] !== "compare") return `${area}legs`;
+    if (area === "hash-map ") return `hash-map compare ${arg(args, "--theirs-name") === "Wasm leg 2" ? "ww" : "nw"}`;
+    return `${area}compare${!area && args.includes("--theirs-dump") ? " field" : ""}`;
+  };
+  const artifact = (name, kind, path) =>
+    `${JSON.stringify({ reason: "compiler-artifact", target: { name, kind: [kind] }, executable: kind === "bin" ? path : null, filenames: [path] })}\n`;
+  const answer = (kind, args) => {
+    const name = arg(args, "--name");
+    const said = {
+      "build lab": { stdout: artifact("npt-lab", "bin", "/t/release/npt-lab") },
+      "build lab bites": { stdout: artifact("npt-lab", "bin", "/t/release/npt-lab") },
+      "build wasm": { stdout: artifact("npt_hash_wasm", "cdylib", "/t/wasm32-unknown-unknown/release/npt_hash_wasm.wasm") },
+      "build wasm bites": { stdout: artifact("npt_hash_wasm", "cdylib", "/t/wasm32-unknown-unknown/release/npt_hash_wasm.wasm") },
+      legs: {
+        stdout:
+          "wrote the header-only recordings vector-SS, vector-ES, vector-CO, vector-TC (seed 1, first tick 0, no tick) for h2\n" +
+          `${RECS.map((r) => `regenerated ${r.name}: 100 bytes, sha256 ab, as the manifest's\n`).join("")}${cmpLine("native leg 1", "native leg 2")}\n`,
+        writes: [...RECS.map((r) => join(WS.rec, `${r.name}.rec`)), ...["SS", "ES", "CO", "TC"].map((s) => join(WS.rec, `vector-${s}.rec`))],
+      },
+      compare: { stdout: `${cmpLine("native leg 1", "Wasm leg 1")}\n` },
+      vectors: { stdout: "h2: CO's wasm32 dump is the committed vector (260 words, digest b432ac332054c77b); every scene's initial dump and hash are equal natively and on wasm32 (SS 6488, ES 1944, CO 260, TC 340)\n" },
+      bites: { stdout: "1 PM-0004-21/hash-map\n2 PM-0004-21/std-sin\n3 PM-0001-41/server-speed-1pct\n4 PM-0001-41/server-quant-differs\n" },
+      "hash-map legs": {
+        status: 1,
+        stdout:
+          "leg 1 order digest 1111111111111111 (PM-0004-21/hash-map armed at SS-s1 tick 1234)\nleg 2 order digest 2222222222222222 (PM-0004-21/hash-map armed at SS-s1 tick 1234)\n" +
+          `${cmpLine("native leg 1", "native leg 2", { compared: 9_000, mismatching: 7_766, rec: "SS-s1", tick: 1234, field: HASH_MAP_FIELD, counts: "SS-s1 9000" })}\n`,
+      },
+      "hash-map compare nw": {
+        status: 1,
+        stdout:
+          `native leg 1 digest 1111111111111111 (${HASH_MAP_FIELD} at words [6488, 6489] of its dump at SS-s1 tick 1234)\n` +
+          `Wasm leg 1 digest 3333333333333333 (${HASH_MAP_FIELD} at words [6488, 6489] of its dump at SS-s1 tick 1234)\n` +
+          `${cmpLine("native leg 1", "Wasm leg 1", { compared: 9_000, mismatching: 7_766, rec: "SS-s1", tick: 1234, field: HASH_MAP_FIELD, counts: "SS-s1 9000" })}\n`,
+      },
+      "hash-map compare ww": {
+        stdout:
+          `Wasm leg 1 digest 3333333333333333 (${HASH_MAP_FIELD} at words [6488, 6489] of its dump at SS-s1 tick 1234)\n` +
+          `Wasm leg 2 digest 3333333333333333 (${HASH_MAP_FIELD} at words [6488, 6489] of its dump at SS-s1 tick 1234)\n` +
+          `${cmpLine("Wasm leg 1", "Wasm leg 2", { compared: 9_000, counts: "SS-s1 9000" })}\n`,
+      },
+      "std-sin legs": { stdout: `${cmpLine("native leg 1", "native leg 2", { compared: 9_000, counts: "ES-s2 9000" })}\n` },
+      "std-sin compare": {
+        status: 1,
+        stdout: `${cmpLine("native leg 1", "Wasm leg 1", { compared: 9_000, mismatching: 8_000, rec: "ES-s2", tick: 1_000_123, field: "vehicle[3].pos.y", counts: "ES-s2 9000" })}\n`,
+      },
+    }[kind];
+    if (said) return said;
+    // The loader's runs.
+    const armed = args.includes("--bite") ? `, bite ${arg(args, "--bite")} armed at tick ${arg(args, "--arm")}` : "";
+    const dumped = args.includes("--dump-at") ? `, its dump at ${arg(args, "--dump-at")} written` : "";
+    const ticks = kind.startsWith("h2 ") ? 0 : 9_000;
+    return { stdout: `wasm32 leg: ${name}: ${ticks} ticks stepped${armed}${dumped}\n`, writes: [arg(args, "--out")] };
+  };
+  const io = {
+    reset: () => files.clear(),
+    mkdir: () => {},
+    copy: (from, to) => copies.push([from, to]),
+    exists: (p) => files.has(p),
+    read: (p) => {
+      if (p === RECORDINGS_MANIFEST) return JSON.stringify(manifest);
+      if (p === STD_SIN_SCAN) return scan;
+      throw new Error(`ENOENT ${p}`);
+    },
+    libc: () => "glibc 2.39 (Node's report)",
+  };
+  let clock = 0;
+  const runner = (cmd, args, cwd) => {
+    const kind = kindOf(cmd, args);
+    calls.push({ kind, cwd, line: `${cmd === process.execPath ? "node" : cmd} ${args.join(" ")}` });
+    const base = answer(kind, args);
+    const r = over[kind] ? { ...base, ...over[kind](base, args) } : base;
+    for (const w of r.writes ?? []) files.add(w);
+    clock += 1_500;
+    const status = r.status ?? 0;
+    return { ok: status === 0, status, stdout: r.stdout ?? "", out: `${r.stdout ?? ""}${r.stderr ?? ""}` };
+  };
+  const result = wasmStep(runner, io, () => clock);
+  return { ...result, calls, copies, kinds: calls.map((c) => c.kind) };
+}
+
+test("the wasm step's builds, legs and comparisons, in order; a clean run passes and says each pair, h2, each bite and each call's seconds (U4d2, 2.4 (a) and (g))", () => {
+  const w = wasmWorld();
+  assert.equal(w.ok, true, w.out);
+  const loader = (wasm, r, out, extra = "") => `node ${LOADER} --wasm ${wasm} --rec ${join(WS.rec, `${r.name}.rec`)} --name ${r.name} --first-tick ${r.first} --out ${out}${extra}`;
+  const manifestArgs = `--manifest ${RECORDINGS_MANIFEST} --dir ${WS.rec}`;
+  const hmBite = `--bite ${WASM_BITES.hashMap} --arm ${HASH_MAP_ARM} --on SS-s1`;
+  const hmCompare = (ours, theirs, a, b) =>
+    `${LAB_BITES} hash compare --ours ${join(WS.hm, `${ours}.lines`)} --theirs ${join(WS.hm, `${theirs}.lines`)} ${manifestArgs} --ours-dump ${join(WS.hm, `${ours}.dump`)} --theirs-dump ${join(WS.hm, `${theirs}.dump`)} --digest-field ${HASH_MAP_FIELD} ${hmBite} --ours-name ${a} --theirs-name ${b}`;
+  assert.deepEqual(
+    w.calls.map((c) => c.line),
+    [
+      ...WASM_BUILDS.map((b) => `cargo ${b.args.join(" ")}`),
+      `${LAB} hash ${manifestArgs} --out ${WS.meas}`,
+      ...RECS.map((r) => loader(WASM_M, r, join(WS.meas, `wasm-${r.name}.lines`))),
+      `${LAB} hash compare --ours ${join(WS.meas, "native-1.lines")} --theirs ${RECS.map((r) => join(WS.meas, `wasm-${r.name}.lines`)).join(",")} ${manifestArgs} --ours-name native leg 1 --theirs-name Wasm leg 1`,
+      ...["SS", "ES", "CO", "TC"].map((s) => loader(WASM_M, { name: `vector-${s}`, first: 0 }, join(WS.meas, `vector-${s}.lines`), ` --dump-at end --dump-out ${join(WS.meas, `vector-${s}.dump`)}`)),
+      `${LAB} hash vectors --dir ${WS.rec} --dumps ${WS.meas} --committed ${H2_VECTOR}`,
+      `${LAB_BITES} hash bites`,
+      `${LAB_BITES} hash ${manifestArgs} --out ${WS.hm} --only SS-s1 --bite ${WASM_BITES.hashMap} --arm ${HASH_MAP_ARM} --dump-at ${HASH_MAP_ARM}`,
+      ...[1, 2].map((k) => loader(WASM_B, RECS[0], join(WS.hm, `wasm-${k}.lines`), ` --bite 1 --arm ${HASH_MAP_ARM} --dump-at ${HASH_MAP_ARM} --dump-out ${join(WS.hm, `wasm-${k}.dump`)}`)),
+      hmCompare("native-1", "wasm-1", "native leg 1", "Wasm leg 1"),
+      hmCompare("wasm-1", "wasm-2", "Wasm leg 1", "Wasm leg 2"),
+    ],
+  );
+  assert.ok(w.calls.every((c) => c.cwd === HERE), "every call runs from the spike's folder");
+  // The four release builds, each copied out of cargo's own path before the next replaces it.
+  assert.deepEqual(w.copies, [
+    ["/t/release/npt-lab", LAB],
+    ["/t/release/npt-lab", LAB_BITES],
+    ["/t/wasm32-unknown-unknown/release/npt_hash_wasm.wasm", WASM_M],
+    ["/t/wasm32-unknown-unknown/release/npt_hash_wasm.wasm", WASM_B],
+  ]);
+  assert.deepEqual(
+    WASM_BUILDS.map((b) => b.args.join(" ")),
+    [
+      "build --locked --release -p npt-lab --message-format=json",
+      "build --locked --release -p npt-lab --features bites --message-format=json",
+      "build --locked --release --target wasm32-unknown-unknown -p npt-hash-wasm --message-format=json",
+      "build --locked --release --target wasm32-unknown-unknown -p npt-hash-wasm --features bites --message-format=json",
+    ],
+  );
+  const last = lastLine(w.out);
+  for (const said of [
+    `native leg 1 against native leg 2: 27000 ticks compared, 0 mismatching, equal (${COUNTS3})`,
+    `native leg 1 against Wasm leg 1: 27000 ticks compared, 0 mismatching, equal (${COUNTS3})`,
+    "h2 equal",
+    `hash-map: native leg 1 - native leg 2 caught at SS-s1 tick 1234, ${HASH_MAP_FIELD} (digests 1111111111111111, 2222222222222222)`,
+    `native leg 1 - Wasm leg 1 caught at SS-s1 tick 1234, ${HASH_MAP_FIELD} (digests 1111111111111111, 3333333333333333)`,
+    "Wasm leg 1 - Wasm leg 2 not exercised (digests equal, 3333333333333333); not catchable on wasm32 (its two Wasm digests are equal)",
+    "std-sin: not catchable at this site on x86-64 and wasm32 (the scan: scanned on: the test's tree; libc: ldd (test) 2.39)",
+    "seconds: build npt-lab 1.5, build npt-lab-bites 1.5,",
+  ]) {
+    assert.ok(last.includes(said), `${said}\nin: ${last}`);
+  }
+  assert.equal((last.match(/ 1\.5(?:,|$)/g) ?? []).length, w.calls.length, "each timed call's seconds");
+  assert.deepEqual(STEPS.wasm.length, 0, "STEPS.wasm(runner) defaults its runner");
+  assert.deepEqual(Object.keys(STEPS), ["fmt", "clippy", "clippy-wasm32", "doc", "test", "deps", "wasm", "refs", "marks", "self", "redact"]);
+  // The std-sin catch path: armed at the scan's first catch in the manifest's order, native leg 1 and Wasm leg 1.
+  const c = wasmWorld({ scan: CATCH_SCAN });
+  assert.equal(c.ok, true, c.out);
+  const sinArgs = "--bite PM-0004-21/std-sin --arm 1000123";
+  assert.deepEqual(c.calls.slice(-3).map((x) => x.line), [
+    `${LAB_BITES} hash ${manifestArgs} --out ${WS.sin} --only ES-s2 ${sinArgs} --dump-at 1000123`,
+    loader(WASM_B, RECS[1], join(WS.sin, "wasm-1.lines"), ` --bite 2 --arm 1000123 --dump-at 1000123 --dump-out ${join(WS.sin, "wasm-1.dump")}`),
+    `${LAB_BITES} hash compare --ours ${join(WS.sin, "native-1.lines")} --theirs ${join(WS.sin, "wasm-1.lines")} ${manifestArgs} --ours-dump ${join(WS.sin, "native-1.dump")} --theirs-dump ${join(WS.sin, "wasm-1.dump")} ${sinArgs} --on ES-s2 --ours-name native leg 1 --theirs-name Wasm leg 1`,
+  ]);
+  assert.ok(lastLine(c.out).includes("std-sin: caught at ES-s2 tick 1000123, vehicle[3].pos.y, as the scan found (the scan's other catches: TC-s3 tick 4294962800)"), lastLine(c.out));
+});
+
+test("the wasm step fails on every outcome that is not an equal, counted run, naming what failed within the lines main prints (U4d2, 2.4 (a))", () => {
+  const fails = (world, ...said) => {
+    assert.equal(world.ok, false, world.out);
+    for (const s of said) assert.ok(tail(world.out).includes(s), `${s}\nnot in main's 40 lines:\n${tail(world.out)}`);
+  };
+  // A native-against-Wasm mismatch: the loader again with that tick's dump, the compare again, which names the field.
+  const mismatch = (field) => ({
+    compare: () => ({ status: 1, stdout: `${cmpLine("native leg 1", "Wasm leg 1", { mismatching: 3, rec: "ES-s2", tick: 1_000_500 })}\n` }),
+    "compare field": () => ({ status: 1, stdout: `${cmpLine("native leg 1", "Wasm leg 1", { mismatching: 3, rec: "ES-s2", tick: 1_000_500, field })}\n` }),
+  });
+  const nw = wasmWorld({ over: mismatch("soldier[3].pos.x") });
+  fails(nw, "native leg 1 against Wasm leg 1: 3 mismatching; first mismatch at ES-s2 tick 1000500", "field soldier[3].pos.x (word 9)");
+  assert.deepEqual(nw.kinds.slice(9, 11), ["wasm dump ES-s2", "compare field"]);
+  assert.ok(nw.calls[9].line.endsWith(`--dump-at 1000500 --dump-out ${join(WS.meas, "wasm-ES-s2-1000500.dump")}`));
+  // Leg 1 against leg 2.
+  fails(
+    wasmWorld({ over: { legs: (b) => ({ status: 1, stdout: b.stdout.replace(cmpLine("native leg 1", "native leg 2"), cmpLine("native leg 1", "native leg 2", { mismatching: 1, rec: "TC-s3", tick: 4_294_967_000, field: "soldier[0].hp" })) }) } }),
+    "native leg 1 against native leg 2: 1 mismatching; first mismatch at TC-s3 tick 4294967000",
+    "field soldier[0].hp",
+  );
+  // A SHA-256 or length that differs at the regeneration: the recording named; h2 still runs; the legs on it do not.
+  const regen = wasmWorld({
+    over: {
+      legs: () => ({
+        status: 1,
+        stdout: "wrote the header-only recordings vector-SS, vector-ES, vector-CO, vector-TC (seed 1, first tick 0, no tick) for h2\n",
+        stderr: "npt-lab hash: ES-s2: regenerated 16700849 bytes with sha256 aa; the manifest has 16700849 bytes with sha256 bb\n",
+        writes: [join(WS.rec, "SS-s1.rec"), ...["SS", "ES", "CO", "TC"].map((s) => join(WS.rec, `vector-${s}.rec`))],
+      }),
+    },
+  });
+  fails(regen, "the recordings manifest: ES-s2: regenerated 16700849 bytes with sha256 aa; the manifest has 16700849 bytes with sha256 bb");
+  assert.ok(regen.kinds.includes("wasm SS-s1") && !regen.kinds.includes("wasm ES-s2") && regen.kinds.includes("vectors"));
+  // A trap, and a FAILED status: the recording and the last tick stepped.
+  fails(
+    wasmWorld({ over: { "wasm TC-s3": () => ({ status: 1, stdout: "", stderr: "loader: TC-s3: the Wasm leg trapped after tick 4294962900: unreachable\n" }) } }),
+    "Wasm leg 1: loader: TC-s3: the Wasm leg trapped after tick 4294962900: unreachable",
+  );
+  fails(
+    wasmWorld({ over: { "wasm SS-s1": () => ({ status: 1, stdout: "", stderr: "loader: SS-s1: step() returned FAILED after tick 41\n" }) } }),
+    "Wasm leg 1: loader: SS-s1: step() returned FAILED after tick 41",
+  );
+  // 0 ticks compared; a short leg (the recording, both counts).
+  fails(wasmWorld({ over: { compare: () => ({ stdout: "native leg 1 against Wasm leg 1: 0 ticks compared (nothing to compare) ()\n" }) } }), "native leg 1 against Wasm leg 1: 0 ticks compared, not the manifest's 27000");
+  fails(wasmWorld({ over: { "wasm ES-s2": (b) => ({ stdout: b.stdout.replace("9000 ticks", "8999 ticks") }) } }), "Wasm leg 1: ES-s2 stepped 8999 ticks, the manifest's 9000");
+  fails(
+    wasmWorld({ over: { compare: () => ({ status: 1, stdout: `${cmpLine("native leg 1", "Wasm leg 1", { compared: 26_999, mismatching: 1, rec: "x", tick: 1, counts: "SS-s1 9000, ES-s2 8999, TC-s3 9000" })}\n` }) } }),
+    "native leg 1 against Wasm leg 1: ES-s2 has 8999 ticks compared, the manifest's 9000",
+  );
+  // An h2 vector word that differs: its index and field.
+  fails(
+    wasmWorld({
+      over: {
+        vectors: () => ({
+          status: 1,
+          stdout: "h2: CO's wasm32 dump differs from the committed vector at word 17 (soldier[2].pos.x): 3f800000 committed, 3f800001 on wasm32\nh2: the vectors differ (the lines above)\n",
+        }),
+      },
+    }),
+    "h2: CO's wasm32 dump differs from the committed vector at word 17 (soldier[2].pos.x)",
+  );
+  // The loader's refusal of an export more, by name.
+  fails(
+    wasmWorld({ over: { "wasm SS-s1": () => ({ status: 1, stdout: "", stderr: `loader: ${WASM_M}: refused: the module exports "extra" (a function), which is not on the loader's list\n` }) } }),
+    'refused: the module exports "extra" (a function)',
+  );
+  // A clean-half leg carrying an order digest or a bite: a bites build's output given to the measured half.
+  fails(wasmWorld({ over: { legs: (b) => ({ stdout: `leg 1 order digest 1234 (x)\n${b.stdout}` }) } }), "a clean-half native leg wrote an order digest");
+  fails(wasmWorld({ over: { "wasm ES-s2": (b) => ({ stdout: b.stdout.replace("stepped", "stepped, bite 1 armed at tick 1234") }) } }), "Wasm leg 1: ES-s2's leg carries a bite");
+  // A build that fails, or that names no artifact.
+  fails(wasmWorld({ over: { "build wasm bites": () => ({ status: 101, stdout: "", stderr: "error[E0425]: cannot find value `x`\nerror: could not compile\n" }) } }), "the release build of npt-hash-wasm (wasm32, bites) failed: error: could not compile");
+  fails(wasmWorld({ over: { "build lab": () => ({ stdout: "" }) } }), "the release build of npt-lab (x86-64, measured) names no bin artifact in cargo's JSON messages");
+});
+
+test("the wasm step judges PM-0004-21's two bites: each caught at its tick and field, or not exercised; anything else fails (U4d2, 2.4 (a) item 6)", () => {
+  const fails = (world, ...said) => {
+    assert.equal(world.ok, false, world.out);
+    for (const s of said) assert.ok(tail(world.out).includes(s), `${s}\nnot in:\n${tail(world.out)}`);
+  };
+  const nwAt = (o) => () => ({
+    status: 1,
+    stdout:
+      `native leg 1 digest 1111111111111111 (${HASH_MAP_FIELD} at words [6488, 6489] of its dump at SS-s1 tick 1234)\n` +
+      `Wasm leg 1 digest 3333333333333333 (${HASH_MAP_FIELD} at words [6488, 6489] of its dump at SS-s1 tick 1234)\n` +
+      `${cmpLine("native leg 1", "Wasm leg 1", { compared: 9_000, counts: "SS-s1 9000", ...o })}\n`,
+  });
+  // hash-map: a catch at another tick, at another field, and a miss (digests differ, no tick does): each fails.
+  fails(wasmWorld({ over: { "hash-map compare nw": nwAt({ mismatching: 3, rec: "SS-s1", tick: 1235, field: HASH_MAP_FIELD }) } }), "the first mismatch is SS-s1 tick 1235, field bite[0].hash_map_order, not SS-s1 tick 1234");
+  fails(wasmWorld({ over: { "hash-map compare nw": nwAt({ mismatching: 3, rec: "SS-s1", tick: 1234, field: "soldier[0].alive" }) } }), "the first mismatch is SS-s1 tick 1234, field soldier[0].alive, not SS-s1 tick 1234");
+  fails(wasmWorld({ over: { "hash-map compare nw": nwAt({}) } }), "hash-map, native leg 1 against Wasm leg 1: digests 1111111111111111 and 3333333333333333 differ, but the first mismatch is none");
+  // Digests equal, yet a tick differs: fails.
+  fails(
+    wasmWorld({ over: { "hash-map compare ww": (b) => ({ status: 1, stdout: b.stdout.replace(cmpLine("Wasm leg 1", "Wasm leg 2", { compared: 9_000, counts: "SS-s1 9000" }), cmpLine("Wasm leg 1", "Wasm leg 2", { compared: 9_000, mismatching: 2, rec: "SS-s1", tick: 5000, field: "soldier[1].pos.x", counts: "SS-s1 9000" })) }) } }),
+    "hash-map, Wasm leg 1 against Wasm leg 2: digests equal (3333333333333333: not exercised), yet 2 ticks differ",
+  );
+  // No pair exercised (every pair's digests equal): fails.
+  const same = (a, b) => (base) => ({ status: 0, stdout: base.stdout.replace(/digest \w{16}/g, "digest 4444444444444444").replace(/^(.* against .*): .*$/m, `${cmpLine(a, b, { compared: 9_000, counts: "SS-s1 9000" })}`) });
+  fails(
+    wasmWorld({ over: { "hash-map legs": same("native leg 1", "native leg 2"), "hash-map compare nw": same("native leg 1", "Wasm leg 1"), "hash-map compare ww": same("Wasm leg 1", "Wasm leg 2") } }),
+    "hash-map: no pair was exercised (every pair's digests equal)",
+  );
+  // The native digests equal (not exercised) while native and Wasm differ and are caught: one pair exercised passes.
+  const oneExercised = wasmWorld({
+    over: {
+      "hash-map legs": same("native leg 1", "native leg 2"),
+      "hash-map compare nw": (b) => ({ stdout: b.stdout.replace("native leg 1 digest 1111111111111111", "native leg 1 digest 4444444444444444") }),
+    },
+  });
+  assert.equal(oneExercised.ok, true, oneExercised.out);
+  assert.ok(lastLine(oneExercised.out).includes("native leg 1 - native leg 2 not exercised (digests equal, 4444444444444444); native leg 1 - Wasm leg 1 caught"));
+  // std-sin, armed at the scan's catch: caught at its tick and field passes (the pin test); at another tick, another
+  // field, or a miss fails, naming the libc the step ran on beside the scan's.
+  const sinAt = (o) => () => ({ status: 1, stdout: `${cmpLine("native leg 1", "Wasm leg 1", { compared: 9_000, counts: "ES-s2 9000", ...o })}\n` });
+  const libc = "this step ran on glibc 2.39 (Node's report), the scan on libc: ldd (Ubuntu GLIBC 2.39-0ubuntu8.6) 2.39";
+  fails(wasmWorld({ scan: CATCH_SCAN, over: { "std-sin compare": sinAt({ mismatching: 9, rec: "ES-s2", tick: 1_000_124, field: "vehicle[3].pos.y" }) } }), "NOT caught at the scan's ES-s2 tick 1000123, vehicle[3].pos.y: first mismatch ES-s2 tick 1000124", libc);
+  fails(wasmWorld({ scan: CATCH_SCAN, over: { "std-sin compare": sinAt({ mismatching: 9, rec: "ES-s2", tick: 1_000_123, field: "vehicle[3].pos.x" }) } }), "field vehicle[3].pos.x", libc);
+  fails(wasmWorld({ scan: CATCH_SCAN, over: { "std-sin compare": sinAt({}) } }), "NOT caught at the scan's ES-s2 tick 1000123, vehicle[3].pos.y: first mismatch none", libc);
+  // The scan data: "not catchable" passes and is printed (the pin test); a file with neither fails.
+  fails(wasmWorld({ scan: "scanned on: nothing\n" }), `${STD_SIN_SCAN} records neither a catch on a manifest recording nor "not catchable at this site on x86-64 and wasm32"`);
+  assert.deepEqual(readScan(CATCH_SCAN), {
+    catches: [
+      { recording: "TC-s3", tick: 4_294_962_800, field: "vehicle[0].pos.y" },
+      { recording: "ES-s2", tick: 1_000_123, field: "vehicle[3].pos.y" },
+    ],
+    notCatchable: false,
+    record: ["libc: ldd (Ubuntu GLIBC 2.39-0ubuntu8.6) 2.39"],
+  });
+  // The two ids are named by check.mjs; their indexes come from `npt-lab hash bites`, never a copy of World::BITES.
+  assert.deepEqual(WASM_BITES, { hashMap: "PM-0004-21/hash-map", stdSin: "PM-0004-21/std-sin" });
+  fails(wasmWorld({ over: { bites: () => ({ stdout: "1 PM-0004-21/hash-map\n" }) } }), "npt-lab hash bites lists no PM-0004-21/std-sin (World::BITES)");
+  const reordered = wasmWorld({ over: { bites: () => ({ stdout: "1 PM-0001-41/server-speed-1pct\n2 PM-0004-21/std-sin\n3 PM-0004-21/hash-map\n" }) } });
+  assert.ok(reordered.calls.find((c) => c.kind === "hash-map wasm 1").line.includes("--bite 3 --arm 1234"));
+});
+
+test("readComparison and builtArtifact read npt-lab's lines and cargo's JSON as they are written (U4d2)", () => {
+  const c = readComparison(`x\n${cmpLine("native leg 1", "Wasm leg 1", { mismatching: 4, rec: "SS-s1", tick: 12, field: "soldier[0].pos.x" })}\n`, "native leg 1", "Wasm leg 1");
+  assert.deepEqual({ ...c, line: undefined, counts: [...c.counts] }, {
+    line: undefined,
+    compared: 27_000,
+    mismatching: 4,
+    equal: false,
+    first: { recording: "SS-s1", tick: 12 },
+    at: "SS-s1 tick 12 hash 00000000000000aa against SS-s1 tick 12 hash 00000000000000bb; field soldier[0].pos.x (word 9): 1 and 2",
+    field: "soldier[0].pos.x",
+    counts: [["SS-s1", 9000], ["ES-s2", 9000], ["TC-s3", 9000]],
+  });
+  assert.equal(readComparison("nothing\n", "a", "b"), null);
+  const end = readComparison("a against b: 9 ticks compared, 1 mismatching; first mismatch at entry 8: SS-s1 tick 8 hash 01 against its end; the lists step other ticks or one ends (SS-s1 9)\n", "a", "b");
+  assert.deepEqual([end.first, end.field, end.mismatching], [null, null, 1]);
+  const json = `${JSON.stringify({ reason: "compiler-message" })}\n${JSON.stringify({ reason: "compiler-artifact", target: { name: "npt_lab", kind: ["lib"] }, filenames: ["/l.rlib"] })}\n${JSON.stringify({ reason: "compiler-artifact", target: { name: "npt-lab", kind: ["bin"] }, executable: "/t/npt-lab" })}\n`;
+  assert.equal(builtArtifact(json, { target: "npt-lab", kind: "bin" }), "/t/npt-lab");
+  assert.equal(builtArtifact(json, { target: "npt_hash_wasm", kind: "cdylib" }), null);
+});
