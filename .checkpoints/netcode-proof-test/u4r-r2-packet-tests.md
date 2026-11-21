# spikes/netcode-proof/check.test.mjs: git diff --no-index .gate/u4r-snap1/spikes/netcode-proof/check.test.mjs .gate/u4r-snap2/spikes/netcode-proof/check.test.mjs
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/check.test.mjs b/.gate/u4r-snap2/spikes/netcode-proof/check.test.mjs
index 782c33e..fe20714 100644
--- a/.gate/u4r-snap1/spikes/netcode-proof/check.test.mjs
+++ b/.gate/u4r-snap2/spikes/netcode-proof/check.test.mjs
@@ -526,13 +526,20 @@ test("unsafe_code: no #[path] or include! reaches outside its crate, the way npt
       f["crates/room/src/k.rs"] = code;
     });
     const line = code.startsWith("#") ? 1 : 3;
-    assert.deepEqual(p, [`crates/room/src/k.rs:${line} has a #[path] whose value is not a plain string literal, not a .rs file inside crates/room/`], code);
+    // The macro forms' attribute holds a metavariable too (carry 85 (c) as advisor-u4r-r1 widened it: FP-6 item 8).
+    const held = code.startsWith("#") ? [] : [metaHeld(`crates/room/src/k.rs:${line}`)];
+    assert.deepEqual(p, [...held, `crates/room/src/k.rs:${line} has a #[path] whose value is not a plain string literal, not a .rs file inside crates/room/`], code);
   }
 });
 
 /** One unsafe block, as rustc 1.99.1 compiles it once unsafe_code is relaxed. */
 const UNSAFE_FN = "pub fn f() -> u8 {\n    let x = 1u8;\n    unsafe { *(&raw const x) }\n}\n";
 
+/** Carry 85 (c)'s refusal of an attribute holding a metavariable (advisor-u4r-r1, FP-6 item 8: SEC-U4R-5), at `where`. */
+function metaHeld(where) {
+  return `${where} has an attribute holding a metavariable (\`$\` between \`#[\` or \`#![\` and its \`]\`): a macro could build a #[path] or an allow from a fragment that this scan does not read`;
+}
+
 test("unsafe code outside the three exception crates fails however the lint was relaxed (review S1)", () => {
   const relaxedElsewhere = {
     // Relaxed by RUSTFLAGS=-Aunsafe_code, --cap-lints warn or a cargo config: no attribute to see, the block still shows.
@@ -548,9 +555,11 @@ test("unsafe code outside the three exception crates fails however the lint was
       f["crates/room/src/evil.rs"] = code;
     });
     const n = code.match(/\bunsafe\b|\bglobal_asm\b/g).length;
+    // A macro-built allow's attribute holds a metavariable: refused as well (FP-6 item 8).
+    const held = what.includes("macro-built allow") ? [metaHeld("crates/room/src/evil.rs:3")] : [];
     assert.deepEqual(
       p,
-      [`crates/room/src/evil.rs:${firstLine[what]} has unsafe code (${n} \`unsafe\` or \`global_asm\`; only npt-client-wasm, npt-tcpinfo and npt-hash-wasm may)`],
+      [`crates/room/src/evil.rs:${firstLine[what]} has unsafe code (${n} \`unsafe\` or \`global_asm\`; only npt-client-wasm, npt-tcpinfo and npt-hash-wasm may)`, ...held],
       what,
     );
   }
@@ -585,6 +594,8 @@ test("npt-tcpinfo: unsafe only inside srtt_us and unacked_and_unsent, unsafe_cod
   assert.deepEqual(lax, [
     "crates/tcpinfo/src/linux.rs:" + (at + 9) + " names unsafe_code outside an attribute (a macro can build the attribute from it)",
     outside(`crates/tcpinfo/src/linux.rs:${at + 5}`),
+    // Its attribute holds a metavariable (FP-6 item 8).
+    metaHeld(`crates/tcpinfo/src/linux.rs:${at + 2}`),
   ]);
   // In the tests, an example or the stub: outside the two functions.
   for (const file of ["crates/tcpinfo/src/tests.rs", "crates/tcpinfo/examples/probe.rs", "crates/tcpinfo/src/stub.rs"]) {
@@ -1115,15 +1126,18 @@ test("carry 85 (a), (b) and (d): a linker for any target and a profile outside t
   assert.deepEqual(cargoConfigProblems({ CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS: "false" }, []), [
     "CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS in the environment sets a profile key (\"false\"): it can drop npt-sim's -C overflow-checks=on, which plan §4.5 asks for in every profile",
   ]);
-  const profile = "c.toml has a `profile` key or table: it can drop npt-sim's -C overflow-checks=on, which plan §4.5 asks for in every profile";
+  // Each refusal names its line and text (advisor-u4r-r1, FP-6 item 9: SEC-U4R-6).
+  const profile = (line, said) =>
+    `c.toml:${line} has a profile key or table (${JSON.stringify(said)}): it can drop npt-sim's -C overflow-checks=on, which plan §4.5 asks for in every profile`;
   for (const text of [
     "[profile.release.package.npt-sim]\noverflow-checks = false\n",
     "profile.release.overflow-checks = false\n",
     '["profile".dev]\nopt-level = 1\n',
     "profile = { release = { debug = true } }\n",
   ]) {
-    assert.deepEqual(config(text), [profile], text);
+    assert.deepEqual(config(text), [profile(1, text.split("\n")[0])], text);
   }
+  assert.deepEqual(config('[build]\njobs = 2\n\n[profile.dev]\nopt-level = 1\n'), [profile(4, "[profile.dev]")], "its line");
   // Neither name in a string value or inside another key's name.
   assert.deepEqual(config('[build]\ntarget-dir = "profile/linker"\nprofiles = 1\nlinkers = 2\n'), []);
   // This container's environment and the spike's own config, as the deps step reads them, pass.
@@ -2081,10 +2095,13 @@ test("npt-tcpinfo and npt-hash-wasm, the deny-level exceptions, have no build sc
   assert.deepEqual(unsafeWith((f) => (f[F] = `${HASH_WASM_LIB}m!(include!);\n`)), [includeAt(`${F}:${HASH_WASM_LIB.split("\n").length}`, "npt-hash-wasm")]);
   assert.deepEqual(unsafeWith((f) => (f["crates/tcpinfo/src/lib.rs"] += 'const K: &str = include ! ("k.rs");\n')), [includeAt("crates/tcpinfo/src/lib.rs:8", "npt-tcpinfo")]);
   assert.deepEqual(unsafeWith((f) => (f[F] = `${HASH_WASM_LIB}include!{"k.rs"}\ninclude!["k.rs"];\n`)), []);
-  // Carry 85 (c), every crate: an attribute whose text starts with a metavariable, either form; none in the spike.
-  const metaAt = (where) => `${where} has an attribute whose text starts with a metavariable (\`#[$\` or \`#![$\`): a macro could build a #[path] from a \`meta\` fragment that this scan does not read`;
+  // Carry 85 (c), every crate, as advisor-u4r-r1 widened it (FP-6 item 8: SEC-U4R-5): an attribute holding a
+  // metavariable anywhere in its text, either form; none in the spike.
+  const metaAt = (where) => `${where} has an attribute holding a metavariable (\`$\` between \`#[\` or \`#![\` and its \`]\`): a macro could build a #[path] or an allow from a fragment that this scan does not read`;
   const meta = "macro_rules! m {\n    ($p:meta) => {\n        #[$p]\n        mod x;\n    };\n}\nmacro_rules! n {\n    ($p:meta) => {\n        # ! [ $p ]\n    };\n}\n";
   assert.deepEqual(unsafeWith((f) => (f["crates/net/src/lib.rs"] += meta)), [metaAt("crates/net/src/lib.rs:6"), metaAt("crates/net/src/lib.rs:12")]);
+  const inner = "macro_rules! k {\n    ($a:meta) => {\n        #[cfg_attr(all(), $a)]\n        mod y;\n    };\n}\n";
+  assert.deepEqual(unsafeWith((f) => (f["crates/net/src/lib.rs"] += inner)), [metaAt("crates/net/src/lib.rs:6")]);
   // A `build` key in the manifest, in either form cargo reads; a custom-build target in cargo metadata.
   const manifestKey = (crate) => `crates/${crate}/Cargo.toml has a key \`build\`: a build script in npt-${crate}, which may have none: ${why}`;
   assert.deepEqual(unsafeWith((f) => (f["crates/hash-wasm/Cargo.toml"] = HASH_WASM_MANIFEST.replace("\n\n[lints.rust]", '\nbuild = "gen.rs"\n\n[lints.rust]'))), [manifestKey("hash-wasm")]);
@@ -2898,11 +2915,21 @@ test("carry 84 (c), the shared deadline with a fake runner and clock: a first ca
   assert.equal(spent.kinds.filter((k) => !k.startsWith("build ")).length, 1, "only the first call ran");
 });
 
+test("the shared deadline on a fractional clock (wasmStep's default is performance.now: advisor-u4r-r1, FP-6 item 11) gives every call a whole limit, as spawnSync takes it", () => {
+  // `npt-lab hash` takes 1,500.4 ms, so the time left for every later call is fractional.
+  const w = wasmWorld({ over: { legs: (b) => ({ ...b, ms: 1_500.4 }) } });
+  assert.equal(w.ok, true, w.out);
+  const timedCalls = w.calls.filter((c) => !c.kind.startsWith("build "));
+  for (const c of timedCalls) assert.ok(Number.isInteger(c.limit) && c.limit > 0, `${c.kind}: ${c.limit}`);
+  assert.equal(timedCalls[1].limit, 298_500, "300,000 - 1,500.4, rounded up");
+});
+
 test("carry 84 (b) and (c): run() stops a timed call with every process it started (a child that starts a grandchild and sleeps, limit 200 ms)", () => {
-  const child =
-    'console.log("child " + process.pid); const g = require("node:child_process").spawn(process.execPath, ["-e", "setTimeout(() => {}, 30000)"], { stdio: "inherit" }); console.log("grandchild " + g.pid); setTimeout(() => {}, 30000);';
+  // A /bin/sh child, which starts far inside the limit (advisor-u4r-r1, FP-6 item 7: T-3; a node child took 203.7-204.0
+  // ms in all, so one slow start would fail a check that is never retried).
+  const child = "echo child $$; sleep 30 & echo grandchild $!; wait";
   const t0 = Date.now();
-  const r = run(process.execPath, ["-e", child], HERE, process.env, 200);
+  const r = run("/bin/sh", ["-c", child], HERE, process.env, 200);
   const ms = Date.now() - t0;
   assert.equal(r.timedOut, true, `run() reports the timeout: ${JSON.stringify(r)}`);
   assert.ok(ms < 10_000, `run() returned after ${ms} ms, not the child's 30 s`);

# spikes/netcode-proof/crates/server/tests/schedule.rs: git diff --no-index .gate/u4r-snap1/spikes/netcode-proof/crates/server/tests/schedule.rs .gate/u4r-snap2/spikes/netcode-proof/crates/server/tests/schedule.rs
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/crates/server/tests/schedule.rs b/.gate/u4r-snap2/spikes/netcode-proof/crates/server/tests/schedule.rs
index 2f463d7..aa50b56 100644
--- a/.gate/u4r-snap1/spikes/netcode-proof/crates/server/tests/schedule.rs
+++ b/.gate/u4r-snap2/spikes/netcode-proof/crates/server/tests/schedule.rs
@@ -344,8 +344,10 @@ mod bites {
         Schedule::with_bite(hz, cap, id).expect("npt-server plants it")
     }
 
+    /// npt-server's four bite ids, each in marks.json: three are the schedule's (`Schedule::with_bite`) and one the
+    /// pump's (`Decoder::with_bite`: U4R-Q6 (a)), which the schedule refuses.
     #[test]
-    fn the_three_bites_are_planted_here_under_their_marks_json_ids() {
+    fn npt_server_s_four_bites_are_listed_under_their_marks_json_ids() {
         assert_eq!(
             npt_server::BITES,
             [TIMER_PERIOD, UNCAPPED, UNCOUNTED, WALLCLOCK]

# FP-5's helper test (a scratch file in the builder worktree's notes folder, never committed; the ruling's FP-5 EVIDENCE): git diff --no-index /dev/null .claude/worktrees/agent-a11e2354169f39acf/.checkpoints/netcode-proof-test/u4r-fp5-helpers.test.mjs
diff --git a/.claude/worktrees/agent-a11e2354169f39acf/.checkpoints/netcode-proof-test/u4r-fp5-helpers.test.mjs b/.claude/worktrees/agent-a11e2354169f39acf/.checkpoints/netcode-proof-test/u4r-fp5-helpers.test.mjs
new file mode 100644
index 0000000..d767084
--- /dev/null
+++ b/.claude/worktrees/agent-a11e2354169f39acf/.checkpoints/netcode-proof-test/u4r-fp5-helpers.test.mjs
@@ -0,0 +1,72 @@
+// Scratch test (advisor-u4r-r1, FP-5's evidence; never committed): the run scripts' pure helpers.
+// Run: node --test .checkpoints/netcode-proof-test/u4r-fp5-helpers.test.mjs (from the worktree's root).
+import assert from "node:assert/strict";
+import { readFileSync } from "node:fs";
+import { dirname, join } from "node:path";
+import { test } from "node:test";
+import { fileURLToPath } from "node:url";
+import {
+  FLOOR,
+  LATE_COMMAND_LINE,
+  nameProblems,
+  overflowFlag,
+  pairAsExpected,
+  planJudged,
+  planPrecheck,
+  projectedBytes,
+  roomFor,
+} from "../../spikes/netcode-proof/ops/runs/c-u4r-runs.mjs";
+import { SUITES, keyProblems, r2Projected, roomFor as r2RoomFor } from "../../spikes/netcode-proof/ops/runs/c-u4r-r2.mjs";
+
+const SPIKE = join(dirname(fileURLToPath(import.meta.url)), "..", "..", "spikes", "netcode-proof");
+const marks = JSON.parse(readFileSync(join(SPIKE, "marks", "marks.json"), "utf8"));
+/** Today's free space as the runner logged it at this copy's first call (.gate/u4r/c6-fp12-1.log: 4780 MiB). */
+const TODAY = 4780 * 1024 ** 2;
+
+test("the projection: a P0 set at today's free space is refused; each set's plan and total printed", () => {
+  const p0 = planJudged("P0", marks);
+  const total = projectedBytes(p0);
+  const r = roomFor(TODAY, total, "u4r-P0");
+  console.log(`P0 set: ${p0.map((x) => `${x.run} ${x.bytes}`).join(", ")}; ${r.line}`);
+  assert.equal(r.ok, false, r.line);
+  assert.ok(r.line.includes("free 4780 MiB, projected") && r.line.endsWith("no room, stopped before it"), r.line);
+  for (const profile of ["P2", "P3", "B1"]) {
+    const plan = planJudged(profile, marks);
+    console.log(`${profile} set: ${plan.length} runs, ${roomFor(TODAY, projectedBytes(plan), `u4r-${profile}`).line}`);
+  }
+  const pre = planPrecheck({ name: "SS-P0-s4", bytes: 16_950_444 });
+  console.log(`precheck set: ${roomFor(TODAY, projectedBytes(pre), "u4r-precheck-30").line}`);
+  assert.equal(projectedBytes(pre), 16_950_444 + 3 * 25_164);
+  // One run's own floor: free minus that run's bytes.
+  assert.equal(roomFor(FLOOR + 10, 10, "x").ok, true);
+  assert.equal(roomFor(FLOOR + 10, 11, "x").ok, false);
+  const r2 = r2Projected();
+  console.log(`r2 set: ${SUITES.length} suites, ${r2RoomFor(TODAY, r2, "r2").line}`);
+  assert.equal(r2, 145_000_000 + 24_813_722);
+});
+
+test("the name check: '../x' is refused by name; a manifest name and a bite tail pass", () => {
+  assert.deepEqual(nameProblems([["the pre-check entry's name", "../x"]]), [
+    "the pre-check entry's name \"../x\" is not one or more of A-Z, a-z, 0-9 and -: refused before any use",
+  ]);
+  assert.equal(nameProblems([["bite tail", undefined]]).length, 1, "a bite id with no tail");
+  assert.deepEqual(nameProblems([["a", "SS-P0-s4"], ["b", "whole-as-lost"]]), []);
+  assert.deepEqual(keyProblems(), [], "r2's suite keys");
+  assert.equal(keyProblems([{ key: "../x" }]).length, 1);
+});
+
+test("the summary line's flag: overflow resyncs 3 on a -17 run is flagged; 0, or a run listing neither -17 nor -18, is not", () => {
+  const line = "l1-p0-s1 SS P0 seed 1: 18300 ticks (0 after); lead 2; ...; stale 0; overflow resyncs 3; heals 0; ...";
+  assert.equal(overflowFlag(line, ["PM-0001-2", "PM-0001-17", "PM-0001-57"]), " (overflow resyncs: to the advisor before it counts, carry 58)");
+  assert.equal(overflowFlag(line, ["PM-0001-2", "PM-0001-18"]).length > 0, true);
+  assert.equal(overflowFlag(line.replace("resyncs 3", "resyncs 0"), ["PM-0001-17"]), "");
+  assert.equal(overflowFlag(line, ["PM-0001-2", "PM-0001-57"]), "");
+});
+
+test("the late-command pair: only its whole line is as expected; a VOID by the RTT rule is not", () => {
+  assert.equal(pairAsExpected("late-command", "VOID", `leg L1: ...\n${LATE_COMMAND_LINE}`), true);
+  assert.equal(pairAsExpected("late-command", "VOID", "result: VOID (L1 RTT rule: client 3: no first-sample instant)"), false);
+  assert.equal(pairAsExpected("late-command", "VOID", "result: VOID (L1 late_count 1; L1 refused Rewinds logged 2)"), false);
+  assert.equal(pairAsExpected("clean", "PASS", "result: PASS (9000 ticks, every World hash equal)"), true);
+  assert.equal(pairAsExpected("wallclock", "FAIL", "result: VOID (L1 late_count 1)"), false);
+});
