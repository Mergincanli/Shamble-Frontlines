# Review packet: U4r round 2 (only what changed since snapshot 572c4f69: the fix pass FP-1 to FP-9 of the advisor's ruling on round 1, and FP-0's two runner diffs)

_Cut by `review_packet.py` from `u4r-r2-packet.json`. It holds sources only: code diffs, the approved plan's lines, the advisor's rulings, the last round's findings and Mustafa's words, each copied by line range. It holds no conclusion of the builder or the main session (no builder notes, brief, checkpoint card or summary). You stay free to read any file and run any test._

## Heads

- The snapshot reviewed in round 1: `.checkpoints/netcode-proof-test/wip/u4r.patch` sha256 572c4f69435da2ad6f62d54d4bcbc47b04d6329b5493e0faf78fcecfdc4a09c2 (26 files on 2fbec77; committed at b084998). Its read copy `.gate/u4r-snap1`: a detached worktree at 2fbec77 with that patch applied.
- The snapshot under review: `.checkpoints/netcode-proof-test/wip/u4r.patch` sha256 21ca46be23704131c77cc20981a4bfe866811b33f9ca853279f7ddbfb5d58bce (26 files on 2fbec77; committed at 450af1a). Its read copy `.gate/u4r-snap2`: a detached worktree at 2fbec77 with that patch applied; each of its 26 files equals the builder worktree's (`.claude/worktrees/agent-a11e2354169f39acf`, HEAD 2fbec77, the work uncommitted) by cmp.
- The diffs below compare `.gate/u4r-snap1/spikes/netcode-proof` with `.gate/u4r-snap2/spikes/netcode-proof` file by file: the 14 files that differ, found by comparing the two folders. The fix pass added no file under spikes/ and removed none.
- Counts at snapshot 572c4f69 (round 1's head): measured build 526 tests passed, bites build 603, self 76 of 76; check.mjs 11 steps.
- The fix pass's runs, all through the lane's runner `.checkpoints/netcode-proof-test/u4r-run.sh` (sha256 6efb91572cf06fd9ad6472c077cd39c9a59bfed8d4a3dac561992aafd5729bc8, after FP-0) in the builder worktree, logs under `.gate/u4r/`: c6-ceil-1.log, c6-det-u4.log, c6-end1.log, c6-end2.log, c6-end3.log, c6-fp12-1.log, c6-fp12-2.log, c6-fp3-1.log, c6-fp4-1.log, c6-fp4-2.log, c6-fp5-helpers.log, c6-fp5-wal.log, c6-fp6-1.log, c6-fp6-2.log, c6-fp6-3.log, c6-fp7-1.log, c6-plant-ceil.log, c6-plant-fp1.log, c6-plant-fp2i-b.log, c6-plant-fp2i.log, c6-plant-fp2ii.log, c6-plant-fp3.log, c6-plant-fp4.log, c6-regen-precheck.log.
- Among them, the check.mjs runs: c6-end1.log, c6-end2.log, c6-end3.log; FP-8's runs: c6-det-u4.log and c6-regen-precheck.log (round 1's det-u4.log and regen-precheck.log are older); the plant runs: c6-plant-ceil.log, c6-plant-fp1.log, c6-plant-fp2i-b.log, c6-plant-fp2i.log, c6-plant-fp2ii.log, c6-plant-fp3.log, c6-plant-fp4.log.
- FP-0's two runner diffs are copied below: the runners are outside the patch, and the main session edited them (each diff is `git diff` of the runner against its committed version).
- Round 1's packet, with the approved plan's lines, marks.json's and SERIES.md's rows, the catalogue and the carries as round 1 had them: `.checkpoints/netcode-proof-test/u4r-packet.md` and `.checkpoints/netcode-proof-test/u4r-packet-tests.md`.

## What changed (the diffs)

### spikes/netcode-proof/check.mjs
_`git diff --no-index .gate/u4r-snap1/spikes/netcode-proof/check.mjs .gate/u4r-snap2/spikes/netcode-proof/check.mjs`:  1 file changed, 29 insertions(+), 12 deletions(-)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/check.mjs b/.gate/u4r-snap2/spikes/netcode-proof/check.mjs
index ecbec86..461dee0 100644
--- a/.gate/u4r-snap1/spikes/netcode-proof/check.mjs
+++ b/.gate/u4r-snap2/spikes/netcode-proof/check.mjs
@@ -691,10 +691,14 @@ export function unsafeProblems(files, read) {
           );
         }
       }
-      // Carry 85 (c): no attribute in any crate whose text starts with a metavariable, so a #[path] (or an allow) built
-      // from a macro's `meta` fragment cannot bring an unread file under a listed export's allow.
-      for (const m of mask.matchAll(/#\s*!?\s*\[\s*\$/g)) {
-        problems.push(`${f}:${lineAt(mask, m.index)} has an attribute whose text starts with a metavariable (\`#[$\` or \`#![$\`): a macro could build a #[path] from a \`meta\` fragment that this scan does not read`);
+      // Carry 85 (c), as advisor-u4r-r1 widened it (FP-6 item 8: SEC-U4R-5): no attribute in any crate holds a
+      // metavariable (`$`) anywhere in its text, from `#[` or `#![` to its matching `]` in the mask, so neither a #[path]
+      // nor an allow built from a macro's fragment (`#[$p]`, `#[cfg_attr(all(), $a)]`) can bring an unread file under a
+      // listed export's allow.
+      for (const a of attrs) {
+        if (mask.slice(a.bodyStart, a.bodyEnd).includes("$")) {
+          problems.push(`${f}:${lineAt(mask, a.start)} has an attribute holding a metavariable (\`$\` between \`#[\` or \`#![\` and its \`]\`): a macro could build a #[path] or an allow from a fragment that this scan does not read`);
+        }
       }
       problems.push(...pathProblems(f, crate, code, mask, attrs));
     } else if (f === `${crate}Cargo.toml` && EXCEPTION_LEVELS[crate] === "deny") {
@@ -1492,7 +1496,17 @@ export function cargoConfigProblems(env, configs) {
       tomlHeaderHeads(scan).includes("profile") ||
       /(?:^|[\s.{,])profile[ \t]*[.=]/m.test(scan.mask) ||
       scan.strings.some((s) => s.value === "profile" && /^[ \t]*[.=]/.test(scan.mask.slice(s.end)) && /(?:^|[\s.{,[])$/.test(scan.mask.slice(0, s.start)));
-    if (profile) problems.push(`${path} has a \`profile\` key or table: it can drop npt-sim's -C overflow-checks=on, which plan §4.5 asks for in every profile`);
+    if (profile) {
+      // Its line and text, as the refusals above name their value (advisor-u4r-r1, FP-6 item 9: SEC-U4R-6): the first
+      // place a header, a bare or dotted key, or a quoted key names `profile`.
+      const header = scan.mask.match(/^[ \t]*\[\[?[ \t]*profile[ \t]*[.\]]/m);
+      const key = scan.mask.match(/(?:^|[\s.{,])profile[ \t]*[.=]/m);
+      const quoted = scan.strings.find((s) => s.value === "profile" && /^[ \t]*[.=]/.test(scan.mask.slice(s.end)) && /(?:^|[\s.{,[])$/.test(scan.mask.slice(0, s.start)));
+      const found = [header?.index, key ? key.index + key[0].indexOf("profile") : undefined, quoted?.start].filter((i) => i !== undefined);
+      const line = lineAt(scan.mask, found.length ? Math.min(...found) : 0);
+      const said = text.split("\n")[line - 1].trim();
+      problems.push(`${path}:${line} has a profile key or table (${JSON.stringify(said)}): it can drop npt-sim's -C overflow-checks=on, which plan §4.5 asks for in every profile`);
+    }
     if (tomlValues(scan, "include").length || tomlHeaderHeads(scan).includes("include")) {
       problems.push(`${path} has a key \`include\`: it loads a config file this check does not read`);
     }
@@ -1882,7 +1896,8 @@ export function marksProblems(marks, readFile) {
 }
 
 /**
- * One command, waited for: `timeout` (ms; 0, none) stops it with SIGTERM once passed, and the result says so
+ * One command, waited for: `timeout` (ms; 0, none) stops it with SIGKILL once passed (a SIGTERM the call handles could
+ * hold it past the deadline: advisor-u4r-r1, FP-6 item 11, INF-U4R-10 (1)), and the result says so
  * (`timedOut`: spawnSync's ETIMEDOUT), so a hung call is a named failure, not a lock held forever (INF-4). A call with a
  * timeout runs in its own process group (spawnSync's `detached`: the child calls setsid), and once it is stopped that
  * whole group is killed (SIGKILL to the group), so every process it started goes with it (carry 84 (b): spawnSync's
@@ -1892,7 +1907,7 @@ export function marksProblems(marks, readFile) {
  * loader.mjs`). check.test.mjs runs it on real processes (carry 84 (c)).
  */
 export function run(cmd, args, cwd, env = process.env, timeout = 0) {
-  const r = spawnSync(cmd, args, { cwd, env, encoding: "utf8", maxBuffer: 1 << 28, ...(timeout ? { timeout, detached: true } : {}) });
+  const r = spawnSync(cmd, args, { cwd, env, encoding: "utf8", maxBuffer: 1 << 28, ...(timeout ? { timeout, detached: true, killSignal: "SIGKILL" } : {}) });
   const timedOut = r.error?.code === "ETIMEDOUT";
   if (timedOut && r.pid) {
     try {
@@ -2084,7 +2099,7 @@ const DEPS_CHECKED =
   "them, and keeps lib.rs's deny of the two clippy unsafe lints, and npt-hash-wasm relaxes it only by one #[allow] on " +
   "each of its seven listed exports, whose #[unsafe(no_mangle)] is its only unsafe code, each taking and giving u32 " +
   "or i32 only; neither npt-tcpinfo nor npt-hash-wasm has a build script or names `include` but directly before `!` " +
-  "and an opening bracket; no attribute's text starts with a metavariable; " +
+  "and an opening bracket; no attribute holds a metavariable; " +
   "the environment and the cargo configs read pass no " +
   "lint-lowering rustflags and set no clippy or fmt alias, no other rustc or workspace wrapper, no linker, no profile, " +
   "no config include and " +
@@ -2287,8 +2302,8 @@ const WASM_IO = {
 
 /**
  * The determinism check (u4-build-plan.md §5 U4d2; plan.md §4.5:546-551; ADR 0002:500-505), through `runner` for every
- * command (cargo, npt-lab, Node), `io` for its files and `now` for its clock (main's, Date.now, which times each call;
- * npt-lab reads none):
+ * command (cargo, npt-lab, Node), `io` for its files and `now` for its clock (by default monotonic, performance.now,
+ * which times each call and the shared deadline: advisor-u4r-r1, FP-6 item 11, INF-U4R-10 (3); npt-lab reads none):
  * 1. the release builds (WASM_BUILDS), each artifact's path read from cargo's JSON and copied out;
  * 2. `npt-lab hash` (measured): the manifest's recordings regenerated into the step's folder and checked by length and
  *    SHA-256, then its two native legs in lock step, leg 1 against leg 2;
@@ -2318,7 +2333,7 @@ const WASM_IO = {
  * run (by default they do). ops/runs/det-u4.mjs runs the clean half on crates/lab/det-recordings.json with the bites
  * half off; check.mjs runs the defaults.
  */
-export function wasmStep(runner = run, io = WASM_IO, now = Date.now, dir = WASM_STEP_DIR, { manifest = RECORDINGS_MANIFEST, bitesHalf = true } = {}) {
+export function wasmStep(runner = run, io = WASM_IO, now = () => performance.now(), dir = WASM_STEP_DIR, { manifest = RECORDINGS_MANIFEST, bitesHalf = true } = {}) {
   const problems = [];
   const lines = [];
   const timed = [];
@@ -2339,7 +2354,9 @@ export function wasmStep(runner = run, io = WASM_IO, now = Date.now, dir = WASM_
         return { ok: false, out: "", stdout: "", status: null, timedOut: false, skipped: true };
       }
     }
-    const limit = cmd === "cargo" ? 0 : deadline - t0;
+    // Whole ms, as spawnSync takes them (the monotonic clock is fractional); at least 1 while time is left, never 0
+    // (no limit).
+    const limit = cmd === "cargo" ? 0 : Math.ceil(deadline - t0);
     const r = runner(cmd, args, HERE, undefined, limit);
     timed.push(`${label} ${((now() - t0) / 1000).toFixed(1)}`);
     if (r.timedOut) {
````

### spikes/netcode-proof/check.test.mjs
_ 1 file changed, 36 insertions(+), 9 deletions(-); the diff itself is in `.checkpoints/netcode-proof-test/u4r-r2-packet-tests.md` (`git diff --no-index .gate/u4r-snap1/spikes/netcode-proof/check.test.mjs .gate/u4r-snap2/spikes/netcode-proof/check.test.mjs`)._

### spikes/netcode-proof/crates/bots/src/lib.rs
_`git diff --no-index .gate/u4r-snap1/spikes/netcode-proof/crates/bots/src/lib.rs .gate/u4r-snap2/spikes/netcode-proof/crates/bots/src/lib.rs`:  1 file changed, 7 insertions(+), 5 deletions(-)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/crates/bots/src/lib.rs b/.gate/u4r-snap2/spikes/netcode-proof/crates/bots/src/lib.rs
index 0519f46..1e4aa54 100644
--- a/.gate/u4r-snap1/spikes/netcode-proof/crates/bots/src/lib.rs
+++ b/.gate/u4r-snap2/spikes/netcode-proof/crates/bots/src/lib.rs
@@ -26,8 +26,8 @@ use npt_client::ClientEndpoint;
 use npt_link::Channel;
 use npt_sim::{Command, Scene};
 
-/// Move vectors 22.5 degrees apart round the circle, each inside the room's x² + y² ≤ 127² check: a soldier walking
-/// through them, one step every [`TURN_EVERY`] commands, walks a circle about 2 m across at the stress scene's speed.
+/// Move vectors 22.5 degrees apart round the circle, each inside the room's x² + y² ≤ 127² check: a soldier walks
+/// through them, one step every [`TURN_EVERY`] commands.
 const MOVES: [[i8; 2]; 16] = [
     [127, 0],
     [117, 49],
@@ -46,9 +46,10 @@ const MOVES: [[i8; 2]; 16] = [
     [89, -89],
     [117, -49],
 ];
-/// Commands between two turns of the move vector.
+/// Commands between two turns of the move vector: a count of commands given, with no time or speed in it (a cadence in
+/// time would need a rate-derived constant from npt-sim: carry 39; advisor-u4r-r1, FP-7 item 1).
 const TURN_EVERY: u64 = 4;
-/// Commands a brawler keeps its target before it picks again (about 1 s at 30 commands a second).
+/// Commands a brawler keeps its target before it picks again: a count of commands given, with no time in it.
 const RETARGET_EVERY: u64 = 30;
 /// How far above a target's feet the brawl aims, cm (inside a soldier's box).
 const AIM_ABOVE_FEET_CM: i32 = 100;
@@ -172,7 +173,8 @@ impl Behaviour {
     /// One client step at `now_ns` on the host's clock (u4r-brief.md 2.2; the module text): `received` goes to the
     /// endpoint first, then a command if one is due. `view` is what the host says this client sees at its newest tick:
     /// (slot, position in the position grid's cm) of other soldiers; the open-loop scripts never read it. Returns
-    /// whether a command was given.
+    /// whether a command was given. A host that computes a view must feed the endpoint itself and pass `&[]` (the view
+    /// is of the newest tick the endpoint holds after what it was fed: advisor-u4r-r1, FP-7 item 4).
     pub fn step(
         &mut self,
         endpoint: &mut ClientEndpoint,
````

### spikes/netcode-proof/crates/lab/src/contract.rs
_`git diff --no-index .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/contract.rs .gate/u4r-snap2/spikes/netcode-proof/crates/lab/src/contract.rs`:  1 file changed, 6 insertions(+), 5 deletions(-)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/contract.rs b/.gate/u4r-snap2/spikes/netcode-proof/crates/lab/src/contract.rs
index 4221f73..414e55f 100644
--- a/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/contract.rs
+++ b/.gate/u4r-snap2/spikes/netcode-proof/crates/lab/src/contract.rs
@@ -443,27 +443,28 @@ pub(crate) fn main(argv: &[String]) -> Result<Vec<String>, (u8, String)> {
 #[cfg(test)]
 mod tests {
     use super::*;
-    use crate::scenario::tests::{scratch, settings};
+    use crate::scenario::tests::{Scratch, scratch, settings};
 
     const SERIES: &str = include_str!("../../../marks/SERIES.md");
 
     /// One short L1 P0 run's logs (SS, 48 clients so the shot band applies, 6 s of warm-up, 2 s measured, its header
-    /// listing -17).
-    fn short_run() -> Vec<Log> {
+    /// listing -17), with its scratch guard, so a failing caller keeps them (advisor-u4r-r1, FP-6 item 6: T-5).
+    fn short_run() -> (Scratch, Vec<Log>) {
         let dir = scratch("r2");
         let s = settings(
             &dir,
             "--seed 2 --profile P0 --warmup-s 6 --measured-s 2 --after-max-s 10 --marks PM-0001-17",
         );
         crate::scenario::run(&s).expect("the run");
-        read(&dir).expect("its logs")
+        let logs = read(&dir).expect("its logs");
+        (dir, logs)
     }
 
     /// r2's checker passes the clean run, and each of U4R-Q11's four doctored copies fails it by name, and so do two
     /// more: the shot band, and a mirror breach (the advisor's U4r build ruling 3, point 6 (1)).
     #[test]
     fn r2_the_checker_passes_a_clean_run_and_fails_each_doctored_copy_by_name() {
-        let logs = short_run();
+        let (_guard, logs) = short_run();
         let clean = check(&logs, SERIES);
         for l in &clean.lines {
             println!("r2 clean: {l}");
````

### spikes/netcode-proof/crates/lab/src/hash.rs
_`git diff --no-index .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/hash.rs .gate/u4r-snap2/spikes/netcode-proof/crates/lab/src/hash.rs`:  1 file changed, 113 insertions(+), 2 deletions(-)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/hash.rs b/.gate/u4r-snap2/spikes/netcode-proof/crates/lab/src/hash.rs
index 7a4a92f..7212591 100644
--- a/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/hash.rs
+++ b/.gate/u4r-snap2/spikes/netcode-proof/crates/lab/src/hash.rs
@@ -229,6 +229,12 @@ pub(crate) struct Entry {
 /// The fewest bytes one tick of a recording takes (its tick and four counts: replay.rs's tick header), so no
 /// recording of `bytes` holds more than `bytes / 12` ticks (carry 82).
 const TICK_HEADER_BYTES: u64 = 12;
+/// The most bytes an entry's recording may hold: 64 MiB, four times today's largest entry (SS-P0-s4, 16,950,444 B); a
+/// larger recording comes to the advisor first (advisor-u4r-r1, FP-3: RR3-L1).
+const MAX_ENTRY_BYTES: u64 = 64 * 1024 * 1024;
+/// The profiles a runner entry may name (`npt-lab scenario --profile`'s), each checked by name before its settings
+/// reach `scenario::record` (advisor-u4r-r1, FP-3: RR3-M1 = SEC-U4R-2).
+const RUNNER_PROFILES: [&str; 6] = ["P0", "P1", "P2", "P3", "P4", "B1"];
 
 /// Whether `name` is a manifest name: one or more of `A-Z`, `a-z`, `0-9` and `-` (carry 82: it reaches a file path and
 /// the `h <recording> <tick> <hash>` line).
@@ -236,8 +242,9 @@ fn name_ok(name: &str) -> bool {
     !name.is_empty() && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
 }
 
-/// The manifest's entries, in its order. Each entry's name and its `ticks` against its `bytes` are checked here,
-/// before anything is generated (carry 82).
+/// The manifest's entries, in its order. Each entry's name, its `ticks` against its `bytes` and its `bytes` against 64
+/// MiB are checked here, before anything is generated (carry 82), and a runner entry's profile by name, its `ticks`
+/// against its warm-up and measured seconds, and its `span_ms` against the room's (advisor-u4r-r1, FP-3).
 pub(crate) fn manifest(text: &str) -> Result<Vec<Entry>, String> {
     let v: Value =
         serde_json::from_str(text).map_err(|e| format!("the manifest is not JSON: {e}"))?;
@@ -315,6 +322,47 @@ pub(crate) fn manifest(text: &str) -> Result<Vec<Entry>, String> {
                 entry.name, entry.ticks, entry.bytes
             ));
         }
+        if entry.bytes > MAX_ENTRY_BYTES {
+            return Err(format!(
+                "manifest entry {}: bytes {} is above {MAX_ENTRY_BYTES} (64 MiB, four times today's largest entry): a \
+                 larger recording comes to the advisor first",
+                entry.name, entry.bytes
+            ));
+        }
+        if let Kind::Runner {
+            profile,
+            warmup_s,
+            measured_s,
+            ..
+        } = &entry.kind
+        {
+            if !RUNNER_PROFILES.contains(&profile.as_str()) {
+                return Err(format!(
+                    "manifest entry {}: profile {profile:?} is not one of P0, P1, P2, P3, P4 or B1",
+                    entry.name
+                ));
+            }
+            let hz = u64::from(npt_sim::TICK_HZ);
+            let ticks = warmup_s
+                .checked_add(*measured_s)
+                .and_then(|s| s.checked_mul(hz));
+            if ticks != Some(u64::from(entry.ticks)) {
+                return Err(format!(
+                    "manifest entry {}: ticks {} is not (warmup_s {warmup_s} + measured_s {measured_s}) x {hz} ({})",
+                    entry.name,
+                    entry.ticks,
+                    ticks.map_or("an overflow".to_owned(), |t| t.to_string())
+                ));
+            }
+            if entry.span_ms != crate::scenario::ROOM_HISTORY_MS {
+                return Err(format!(
+                    "manifest entry {}: span_ms {} is not the room's ROOM_HISTORY_MS {}",
+                    entry.name,
+                    entry.span_ms,
+                    crate::scenario::ROOM_HISTORY_MS
+                ));
+            }
+        }
         out.push(entry);
     }
     Ok(out)
@@ -1539,6 +1587,69 @@ mod tests {
         assert_eq!(kept, 0);
     }
 
+    /// advisor-u4r-r1, FP-3 (RR3-M1 = SEC-U4R-2, SEC-U4R-3, RR3-L1): before anything is generated, a runner entry's
+    /// profile is checked by name, its ticks against (warmup_s + measured_s) x TICK_HZ and its span_ms against the room's,
+    /// and every entry's bytes against 64 MiB; each refusal names the entry and the value. The committed manifests read.
+    #[test]
+    fn r10_fp3_a_runner_entry_s_profile_ticks_and_span_and_any_entry_s_bytes_are_checked() {
+        let text = "{\"recordings\": [{\"name\": \"SS-P2-s1\", \"kind\": \"runner\", \"profile\": \"P2\", \
+                    \"clients\": 48, \"warmup_s\": 10, \"measured_s\": 290, \"scene\": 1, \"seed\": 1, \
+                    \"first_tick\": 0, \"ticks\": 9000, \"span_ms\": 2000, \"generator\": 0, \"bytes\": 999999, \
+                    \"sha256\": \"00\"}]}";
+        assert!(manifest(text).is_ok(), "the clean runner entry");
+        let refused = |from: &str, to: &str, why: &str| {
+            let e = manifest(&text.replace(from, to)).expect_err(why);
+            assert!(e.starts_with("manifest entry SS-P2-s1: "), "{why}: {e}");
+            e
+        };
+        let e = refused(
+            "\"profile\": \"P2\"",
+            "\"profile\": \"P2 --record x\"",
+            "the profile \"P2 --record x\"",
+        );
+        assert!(
+            e.contains("profile \"P2 --record x\" is not one of P0, P1, P2, P3, P4 or B1"),
+            "{e}"
+        );
+        let e = refused(
+            "\"ticks\": 9000",
+            "\"ticks\": 8999",
+            "ticks 8,999 with 10 + 290",
+        );
+        assert!(
+            e.contains("ticks 8999 is not (warmup_s 10 + measured_s 290) x 30 (9000)"),
+            "{e}"
+        );
+        let e = refused("\"span_ms\": 2000", "\"span_ms\": 1000", "span_ms 1,000");
+        assert!(
+            e.contains("span_ms 1000 is not the room's ROOM_HISTORY_MS 2000"),
+            "{e}"
+        );
+        let e = refused(
+            "\"bytes\": 999999",
+            "\"bytes\": 67108865",
+            "bytes 67,108,865",
+        );
+        assert!(e.contains("bytes 67108865 is above 67108864"), "{e}");
+        assert!(
+            manifest(&text.replace("\"bytes\": 999999", "\"bytes\": 67108864")).is_ok(),
+            "bytes at 64 MiB"
+        );
+        for (name, committed) in [
+            ("recordings.json", include_str!("../recordings.json")),
+            (
+                "det-recordings.json",
+                include_str!("../det-recordings.json"),
+            ),
+            (
+                "precheck-recordings.json",
+                include_str!("../precheck-recordings.json"),
+            ),
+        ] {
+            assert!(manifest(committed).is_ok(), "{name} reads");
+        }
+    }
+
     #[test]
     fn h6_usage_errors_are_refused() {
         let base = "--manifest m --dir d --out o";
````

### spikes/netcode-proof/crates/lab/src/lib.rs
_`git diff --no-index .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/lib.rs .gate/u4r-snap2/spikes/netcode-proof/crates/lab/src/lib.rs`:  1 file changed, 5 insertions(+), 4 deletions(-)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/lib.rs b/.gate/u4r-snap2/spikes/netcode-proof/crates/lab/src/lib.rs
index 8bc3479..bca64ab 100644
--- a/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/lib.rs
+++ b/.gate/u4r-snap2/spikes/netcode-proof/crates/lab/src/lib.rs
@@ -1,9 +1,10 @@
 //! npt-lab: ADR 0002's devtools analogue for the netcode proof test (plan §4.7; engine EA-3, EA-11).
 //!
-//! `SimClock` (integer ns) and `L1Scenario`, which steps one server and 48 headless clients on one simulated clock in a
-//! fixed order per instant (due packets, the server's I/O step, the server tick if due, due client frames, due client
-//! ticks); the x86-64 re-decode and replay tool for the Arm rule (`ReplayX86`); the native-against-Wasm hash comparer
-//! ([`HashCompare`], U4d2). Nothing depends on this crate. Built in units U2 and U4.
+//! This library holds the native-against-Wasm hash comparer ([`HashCompare`], U4d2) and its [`TickHash`]. The npt-lab
+//! binary holds `SimClock` (integer ns) and the L1 runner, plan §4.7's `L1Scenario`, built as `scenario::run` with its
+//! `Server` (one server and 48 headless clients on one simulated clock, in a fixed order per instant: due packets, the
+//! server's I/O step, the server tick if due, due client frames, due client ticks), and the x86-64 re-decode and replay
+//! tool for the Arm rule (`replay::ReplayX86`). Nothing depends on this crate. Built in units U2 and U4.
 
 /// One tick's entry in a leg's per-tick hash list: the recording both legs replay (its manifest name), the tick and the
 /// state hash.
````

### spikes/netcode-proof/crates/lab/src/logs.rs
_`git diff --no-index .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/logs.rs .gate/u4r-snap2/spikes/netcode-proof/crates/lab/src/logs.rs`:  1 file changed, 32 insertions(+), 15 deletions(-)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/logs.rs b/.gate/u4r-snap2/spikes/netcode-proof/crates/lab/src/logs.rs
index 66761d6..a104ea4 100644
--- a/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/logs.rs
+++ b/.gate/u4r-snap2/spikes/netcode-proof/crates/lab/src/logs.rs
@@ -159,7 +159,10 @@ pub(crate) struct Logs {
     wraps: BTreeMap<&'static str, u64>,
     /// carry 62 (h)'s bite, PM-0001-17/whole-as-lost (the advisor's U4r build ruling, point 4): on every client, its
     /// first tick resolved whole in the measured phase filed as lost (null, `snap_incomplete` 1); per slot, that tick.
+    /// Bites builds only, so the measured binary holds no bite code (advisor-u4r-r1, FP-6 item 3: RR3-L4).
+    #[cfg(feature = "bites")]
     whole_as_lost: bool,
+    #[cfg(feature = "bites")]
     bitten_at: Vec<Option<u64>>,
     pub(crate) counts: Counts,
 }
@@ -190,6 +193,8 @@ impl Logs {
         whole_as_lost: bool,
     ) -> Result<Logs, String> {
         let n = meta.clients as usize;
+        #[cfg(not(feature = "bites"))]
+        let _ = whole_as_lost;
         Ok(Logs {
             room: open(dir, "room", meta)?,
             client: open(dir, "client", meta)?,
@@ -207,7 +212,9 @@ impl Logs {
             written: None,
             track: RoomTrack::default(),
             wraps: BTreeMap::new(),
+            #[cfg(feature = "bites")]
             whole_as_lost,
+            #[cfg(feature = "bites")]
             bitten_at: vec![None; n],
             counts: Counts::default(),
         })
@@ -298,6 +305,22 @@ impl Logs {
         }
     }
 
+    /// The whole-as-lost bite's site (bites builds only): a whole decode (not the sentinel) of slot `s`'s first measured
+    /// tick, and every later filing of that tick, goes in as lost; `w` the filing's tick, `h` its hash.
+    #[cfg(feature = "bites")]
+    fn bitten(&mut self, s: usize, w: u64, h: u64) -> bool {
+        let as_lost = self.whole_as_lost
+            && h != 0
+            && match self.bitten_at[s] {
+                Some(b) => b == w,
+                None => self.phases.get(&w) == Some(&Phase::Measured),
+            };
+        if as_lost {
+            self.bitten_at[s] = Some(w);
+        }
+        as_lost
+    }
+
     /// What one client step filed: (tick, hash) pairs (0 the sentinel) and the runs of ticks it resolved lost.
     pub(crate) fn filed(&mut self, slot: u8, hashes: &[(u32, u64)], lost: &[(u32, u32)]) {
         let s = usize::from(slot);
@@ -311,23 +334,17 @@ impl Logs {
                 self.counts.conflicts += 1;
                 continue;
             }
-            // The bite: a whole decode (not the sentinel) of the client's first measured tick, and every later filing
-            // of that tick, goes in as lost.
-            let as_lost = self.whole_as_lost
-                && h != 0
-                && match self.bitten_at[s] {
-                    Some(b) => b == w,
-                    None => self.phases.get(&w) == Some(&Phase::Measured),
-                };
-            if as_lost {
-                self.bitten_at[s] = Some(w);
+            #[cfg(feature = "bites")]
+            if self.bitten(s, w, h) {
+                self.entry(w)[s] = Res::Lost;
+                self.resolved(s, w);
+                continue;
             }
             let e = &mut self.entry(w)[s];
-            *e = match (*e, as_lost) {
-                (_, true) => Res::Lost,
-                (Res::Open | Res::Lost, false) => Res::Hash(h),
-                (Res::Hash(x), false) if x == h => Res::Hash(x),
-                (Res::Hash(_), false) => Res::Hash(0),
+            *e = match *e {
+                Res::Open | Res::Lost => Res::Hash(h),
+                Res::Hash(x) if x == h => Res::Hash(x),
+                Res::Hash(_) => Res::Hash(0),
             };
             self.resolved(s, w);
         }
````

### spikes/netcode-proof/crates/lab/src/precheck.rs
_`git diff --no-index .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/precheck.rs .gate/u4r-snap2/spikes/netcode-proof/crates/lab/src/precheck.rs`:  1 file changed, 108 insertions(+), 28 deletions(-)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/precheck.rs b/.gate/u4r-snap2/spikes/netcode-proof/crates/lab/src/precheck.rs
index 1eef179..8daf08e 100644
--- a/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/precheck.rs
+++ b/.gate/u4r-snap2/spikes/netcode-proof/crates/lab/src/precheck.rs
@@ -1,6 +1,7 @@
 //! The -30 pre-check (u4r-brief.md 7.2; plan :772-796; the advisor's conditions 2 and 3; carry 40): a recorded npt-lab
 //! test, never a judged id. Replay clients send the SS P0 recording's commands to one room over Loopback and to another
-//! over L1 P0, in RQ8's order on one SimClock each, and `HashCompare::of` compares the two rooms' per-tick World hashes.
+//! over L1 P0, in RQ8's order, each leg on its own simulated clock (integer ns), and `HashCompare::of` compares the two
+//! rooms' per-tick World hashes.
 //!
 //! - **The tick base:** both legs' `World::new(scene, recording.seed, recording.first_tick)`, so each leg's first tick
 //!   is the recording's first (`run_leg` refuses a tick that is not the recording's), run at SimClock instant 0 (U4b's
@@ -33,7 +34,8 @@
 //!   client's first take), and the refused Rewinds per command.
 //! - **The setup condition and the RTT rule:** per leg, `late_count` (the room's `late`), `rewinds_refused`, the commands
 //!   run off their stamp, and [`rtt_rule`]; the tolerance's replacements and the cap's clips are not built at U4r (0 by
-//!   construction: rewind.rs:1-2). Any of the first three above 0, or the RTT rule on the L1 leg, makes the pair VOID.
+//!   construction: rewind.rs:1-2). Any of the first three above 0, a refused Rewind logged per command, a session ended
+//!   on the leg (its counters cannot be read), or the RTT rule on the L1 leg, makes the pair VOID.
 
 use std::collections::BTreeMap;
 
@@ -162,12 +164,38 @@ struct ReplayClient {
     /// Commands sent so far: `cmds[..next]`; `cmds[unrun..next]` sent and not yet run.
     next: usize,
     unrun: usize,
-    /// The late-command bite: the tick T whose command and every later one are held until T has run.
+    /// The late-command bite: the tick T whose command and every later one are held until T has run. Bites builds
+    /// only, so the measured binary holds no bite code (advisor-u4r-r1, FP-6 item 3: RR3-L4).
+    #[cfg(feature = "bites")]
     hold: Option<u32>,
+    #[cfg(feature = "bites")]
     released: bool,
 }
 
 impl ReplayClient {
+    /// The late-command bite's hold (bites builds only): the next command, stamped T or later, waits while T has not run.
+    #[cfg(feature = "bites")]
+    fn held(&self, ran: u32) -> bool {
+        self.hold
+            .is_some_and(|t| ran < t && self.cmds[self.next].tick >= t)
+    }
+
+    /// The late-command bite's release (bites builds only): once T has run, T first (it ran already: option B reran the
+    /// last command there), the later ones after; (the first command to send, the delayed command's tick).
+    #[cfg(feature = "bites")]
+    fn release(&mut self, ran: u32, from: usize) -> (usize, Option<u32>) {
+        if let Some(t) = self.hold
+            && ran >= t
+            && !self.released
+        {
+            self.released = true;
+            if let Some(i) = self.cmds.iter().position(|c| c.tick >= t) {
+                return (from.min(i), Some(self.cmds[i].tick));
+            }
+        }
+        (from, None)
+    }
+
     /// RQ8's step 5 at `now`, after tick `ran` ran: the endpoint decodes what arrived and sends its `Join` and `Pong`s;
     /// once joined, the commands due (stamped at most `ran + lead`) go in this client's own `Input`.
     fn step(&mut self, now: u64, ran: u32, lead: u32) -> Option<u32> {
@@ -190,29 +218,20 @@ impl ReplayClient {
             }
         }
         self.endpoint.slot()?;
-        let held = |c: &Command, hold: Option<u32>| hold.is_some_and(|t| ran < t && c.tick >= t);
-        while self.next < self.cmds.len()
-            && self.cmds[self.next].tick <= ran.wrapping_add(lead)
-            && !held(&self.cmds[self.next], self.hold)
-        {
+        while self.next < self.cmds.len() && self.cmds[self.next].tick <= ran.wrapping_add(lead) {
+            #[cfg(feature = "bites")]
+            if self.held(ran) {
+                break;
+            }
             self.next += 1;
         }
         while self.unrun < self.next && self.cmds[self.unrun].tick <= ran {
             self.unrun += 1;
         }
-        let mut from = self.unrun;
-        let mut delayed = None;
-        if let Some(t) = self.hold
-            && ran >= t
-            && !self.released
-        {
-            // The bite's release: T first (it ran already: option B reran the last command there), the later ones after.
-            self.released = true;
-            if let Some(i) = self.cmds.iter().position(|c| c.tick >= t) {
-                from = from.min(i);
-                delayed = Some(self.cmds[i].tick);
-            }
-        }
+        #[cfg(feature = "bites")]
+        let (from, delayed) = self.release(ran, self.unrun);
+        #[cfg(not(feature = "bites"))]
+        let (from, delayed) = (self.unrun, None);
         let mut commands: Vec<Command> = Vec::new();
         for c in self.cmds[from..self.next].iter().rev() {
             if commands.len() == Limits::REDUNDANCY_CAP
@@ -289,17 +308,17 @@ pub(crate) fn run_leg(
     )
     .with_hashes(CONTENT, [0; 32]);
     let mut server = Server::new(room, Quant::default(), RttSource::Link);
-    let mut late_bite = false;
     if let Some(id) = bite {
         let b = Bites::of(id)?;
         if b.pump {
             server.decoder_bite = Some(id.to_owned());
-        } else if b.late_command {
-            late_bite = true;
-        } else {
+        } else if !b.late_command {
             return Err(format!("--bite {id}: not a -30 pre-check bite"));
         }
     }
+    // The late-command bite's replay client (bites builds only: a measured build's `Bites::of` refused every id above).
+    #[cfg(feature = "bites")]
+    let late_bite = bite == Some(crate::scenario::LAB_BITES[1]);
     let mut schedule = Schedule::new(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP);
     let net = L1Net::new(rec.seed);
     let mut listener = net.listener();
@@ -314,6 +333,7 @@ pub(crate) fn run_leg(
                 Box::new(c)
             }
         };
+        #[cfg(feature = "bites")]
         let hold = if late_bite && clients.is_empty() {
             let t0 = rec.ticks.first().map_or(rec.first_tick, |t| t.tick);
             Some(
@@ -334,7 +354,9 @@ pub(crate) fn run_leg(
             cmds: list.clone(),
             next: 0,
             unrun: 0,
+            #[cfg(feature = "bites")]
             hold,
+            #[cfg(feature = "bites")]
             released: false,
         });
     }
@@ -480,9 +502,10 @@ pub(crate) fn run_leg(
     Ok(o)
 }
 
-/// The pair's result (7.2): VOID if either leg breaks the setup condition (late, rewinds refused, a command off its
-/// stamp) or the RTT rule, or nothing was compared; else PASS when the comparer names no differing tick, FAIL when it
-/// names one. Returns (result, reason).
+/// The pair's result (7.2): VOID if either leg breaks the setup condition (late, rewinds refused, refused Rewinds logged
+/// per command, a command off its stamp) or the RTT rule, or a session ended on it (its counters, read at the leg's end,
+/// cannot be read once its slot is gone: advisor-u4r-r1, FP-4), or nothing was compared; else PASS when the comparer
+/// names no differing tick, FAIL when it names one. Returns (result, reason).
 pub(crate) fn verdict(a: &LegOutcome, b: &LegOutcome, cmp: &HashCompare) -> (&'static str, String) {
     let mut void = Vec::new();
     for (leg, o) in [("loopback", a), ("L1", b)] {
@@ -492,6 +515,15 @@ pub(crate) fn verdict(a: &LegOutcome, b: &LegOutcome, cmp: &HashCompare) -> (&'s
         if o.rewinds_refused > 0 {
             void.push(format!("{leg} rewinds_refused {}", o.rewinds_refused));
         }
+        if o.refused_logged > 0 {
+            void.push(format!("{leg} refused Rewinds logged {}", o.refused_logged));
+        }
+        if !o.ended.is_empty() {
+            void.push(format!(
+                "{leg} sessions ended on the leg {:?}: their counters cannot be read",
+                o.ended
+            ));
+        }
         if o.off_stamp > 0 {
             void.push(format!("{leg} commands off their stamp {}", o.off_stamp));
         }
@@ -812,6 +844,54 @@ mod tests {
         );
     }
 
+    /// advisor-u4r-r1, FP-4 (NC-U4R-3, T-6's verdict half): `verdict` on doctored outcomes, one VOID cause each on the
+    /// L1 leg (late, rewinds refused, refused Rewinds logged alone, off-stamp, the RTT rule, a session ended), each named;
+    /// and on clean legs the comparer decides: Empty VOID, Equal PASS, Mismatch FAIL.
+    #[test]
+    fn fp4_each_void_cause_alone_gives_void_by_name_and_the_comparer_decides_a_clean_pair() {
+        let clean = LegOutcome::default();
+        let equal = HashCompare::Equal { entries: 3 };
+        type Doctor = fn(&mut LegOutcome);
+        let causes: [(&str, Doctor); 6] = [
+            ("L1 late_count 1", |o| o.late = 1),
+            ("L1 rewinds_refused 2", |o| o.rewinds_refused = 2),
+            ("L1 refused Rewinds logged 3", |o| o.refused_logged = 3),
+            ("L1 commands off their stamp 4", |o| o.off_stamp = 4),
+            ("L1 RTT rule: client 5: planted", |o| {
+                o.rtt_void = Some("client 5: planted".to_owned());
+            }),
+            (
+                "L1 sessions ended on the leg [(6, \"closed\")]: their counters cannot be read",
+                |o| o.ended = vec![(6, "closed")],
+            ),
+        ];
+        for (named, doctor) in causes {
+            let mut l1 = LegOutcome::default();
+            doctor(&mut l1);
+            let (result, why) = verdict(&clean, &l1, &equal);
+            assert_eq!(
+                (result, why.as_str()),
+                ("VOID", named),
+                "the cause {named:?}"
+            );
+        }
+        assert_eq!(verdict(&clean, &clean, &HashCompare::Empty).0, "VOID");
+        assert_eq!(
+            verdict(&clean, &clean, &equal),
+            ("PASS", "3 ticks, every World hash equal".to_owned())
+        );
+        let mismatch = HashCompare::Mismatch {
+            index: 1,
+            ours: None,
+            theirs: None,
+            compared: 3,
+            mismatching: 1,
+        };
+        let (result, why) = verdict(&clean, &clean, &mismatch);
+        assert_eq!(result, "FAIL", "{why}");
+        assert!(why.starts_with("first differing index 1 "), "{why}");
+    }
+
     /// r6, the pre-check's pacing test (7.2's pacing rule): a short clean replay of the SS P0 recording over both legs
     /// gives `late_count` 0 and `rewinds_refused` 0 on both, every command run at its stamped tick, the room's measured
     /// depth L - 1 at every take after each client's first (input.rs:28-29: the newest buffered command's tick minus
````

### spikes/netcode-proof/crates/lab/src/replay.rs
_`git diff --no-index .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/replay.rs .gate/u4r-snap2/spikes/netcode-proof/crates/lab/src/replay.rs`:  1 file changed, 60 insertions(+), 3 deletions(-)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/replay.rs b/.gate/u4r-snap2/spikes/netcode-proof/crates/lab/src/replay.rs
index 241ef0d..4def2cb 100644
--- a/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/replay.rs
+++ b/.gate/u4r-snap2/spikes/netcode-proof/crates/lab/src/replay.rs
@@ -261,15 +261,26 @@ pub(crate) fn run(s: &Settings) -> Result<Outcome, Failure> {
     let fail = Failure::Failed;
     let client = read_log(&s.logs.join("client.jsonl")).map_err(fail)?;
     let h = &client.header;
+    // The header's client count as a slot count, 1 to 255, before any .bytes file is read or any list sized
+    // (advisor-u4r-r1, FP-6 item 4: RR3-L7).
+    let clients = u8::try_from(h.clients)
+        .ok()
+        .filter(|c| *c > 0)
+        .ok_or_else(|| {
+            fail(format!(
+                "the run's client log header names {} clients: 1 to 255",
+                h.clients
+            ))
+        })?;
     let rx = ReplayX86::of(h, s.bite.as_deref()).map_err(Failure::Usage)?;
-    let n = usize::try_from(h.clients).map_err(|_| fail("clients above usize".to_owned()))?;
+    let n = usize::from(clients);
     let mut by_tick: BTreeMap<u64, Vec<Option<u64>>> = BTreeMap::new();
     let mut o = Outcome {
         clients: n,
         ..Outcome::default()
     };
-    for slot in 0..n {
-        let s8 = u8::try_from(slot).map_err(|_| fail(format!("slot {slot} above 255")))?;
+    for s8 in 0..clients {
+        let slot = usize::from(s8);
         let p = s.bytes.join(format!("client-{slot}.bytes"));
         let bytes = std::fs::read(&p).map_err(|e| fail(format!("{}: {e}", p.display())))?;
         let steps = read_steps(&bytes).map_err(|e| fail(format!("{}: {e}", p.display())))?;
@@ -356,6 +367,52 @@ mod tests {
         (dir, bytes)
     }
 
+    /// advisor-u4r-r1, FP-6 item 4 (RR3-L7): a run's client log whose header names 256 clients is refused as a failure
+    /// before any .bytes file is read (the bytes folder given does not exist).
+    #[test]
+    fn fp6_a_header_of_256_clients_is_refused_before_any_bytes_file_is_read() {
+        let dir = scratch("replay-256");
+        std::fs::create_dir_all(&*dir).expect("the folder");
+        let header = Header {
+            schema: npt_log::SCHEMA,
+            run: "t".to_owned(),
+            set: "t".to_owned(),
+            role: "measured".to_owned(),
+            marks: Vec::new(),
+            machine: crate::machine::container(),
+            build: crate::machine::build(crate::scenario::tests::COMMIT).expect("a build"),
+            bite: None,
+            layer: "L1".to_owned(),
+            profile: "P0".to_owned(),
+            seed: 1,
+            tick_hz: npt_sim::TICK_HZ,
+            clients: 256,
+            scene: "SS".to_owned(),
+            process: "client".to_owned(),
+            slot: 0,
+            date: "2026-11-14".to_owned(),
+            params: BTreeMap::new(),
+        };
+        let f = std::fs::File::create(dir.join("client.jsonl")).expect("the log");
+        Writer::new(f, &header)
+            .expect("its header")
+            .finish(Ended::Clean)
+            .expect("its footer");
+        let s = Settings {
+            logs: dir.to_path_buf(),
+            bytes: dir.join("no-such-bytes-folder"),
+            out: dir.join("out"),
+            bite: None,
+        };
+        match run(&s) {
+            Err(Failure::Failed(m)) => {
+                assert_eq!(m, "the run's client log header names 256 clients: 1 to 255")
+            }
+            Err(Failure::Usage(m)) => panic!("a usage error: {m}"),
+            Ok(o) => panic!("replayed: {o:?}"),
+        }
+    }
+
     /// The bytes file's reader takes back what `write_step` wrote, and refuses a cut-short file and an unknown channel.
     #[test]
     fn r3_the_bytes_file_reads_back_step_by_step() {
````

### spikes/netcode-proof/crates/lab/src/scenario.rs
_`git diff --no-index .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs .gate/u4r-snap2/spikes/netcode-proof/crates/lab/src/scenario.rs`:  1 file changed, 283 insertions(+), 69 deletions(-)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs b/.gate/u4r-snap2/spikes/netcode-proof/crates/lab/src/scenario.rs
index 460a3c9..4865e70 100644
--- a/.gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs
+++ b/.gate/u4r-snap2/spikes/netcode-proof/crates/lab/src/scenario.rs
@@ -7,8 +7,8 @@
 //!    `Err`, `Decoder::reject`), the pump called again after a `Join` (it returns after one), `receive` for every other
 //!    message of a joined session, and `pump_report` with the bytes the pump sent;
 //! 3. the server tick if due, through U4b's `Schedule::new(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP)` (carry 6), then
-//!    each session's datagrams and reliable messages onto its link, each reliable message on the channel its type's
-//!    row of `Limits::TYPES` names;
+//!    each session's datagrams and reliable messages onto its link, each reliable message on the one stream channel
+//!    that decodes it (`stream_of`; the paragraph below);
 //! 4. the clients' 60 Hz frames: no work until U4g's `Interp`, so the runner adds no instant for them;
 //! 5. each client whose next step is due, in slot order: what its link delivered since its last step (polled into its
 //!    inbox at every instant: the socket's buffer), its `Behaviour::step`, then `ClientEndpoint::step`, whose datagrams
@@ -19,10 +19,12 @@
 //! is U4f's cheat), and each arrival is stamped on it. L1's RTT is its own RFC 9002 estimate.
 //!
 //! **Session handling** (carry 76): when `Room::join` reports a replacement, the lab sends the old session
-//! `Kick(replaced)` and drops its `Decoder` and link before it routes anything else; when the room kicks a client or
-//! takes its `Leave`, the lab drops that session. **Reliable messages' channel:** each is decoded on both stream
-//! channels with the `Quant` it was coded with, and exactly one must take it (never a silent fall-back to Control).
-//! **Tokens** are seeded per slot and never logged.
+//! `Kick(replaced)` and drops its `Decoder` and link before it routes anything else (the `Kick(replaced)` is sent once;
+//! on L1 the old link is dropped in the same instant, so a lost first transmission is not resent: best effort); when
+//! the room kicks a client or takes its `Leave`, the lab drops that session; a closed decoder or link ends the session
+//! and frees the room's slot (`Room::leave`, as its doc has it: "a closed link"). **Reliable messages' channel:** each
+//! is decoded on both stream channels with the `Quant` it was coded with, and exactly one must take it (never a silent
+//! fall-back to Control). **Tokens** are seeded per slot and never logged.
 
 use std::collections::BTreeMap;
 use std::io::Write;
@@ -31,7 +33,7 @@ use std::path::PathBuf;
 use npt_bots::Behaviour;
 use npt_client::ClientEndpoint;
 use npt_l1::{L1Net, Profile};
-use npt_link::{Channel, Link};
+use npt_link::{Channel, Link, LinkState};
 use npt_log::{Ended, EventRecord, Phase};
 use npt_proto::{Direction, Join, Kick, Message, Quant};
 use npt_room::Room;
@@ -57,7 +59,8 @@ pub(crate) const ROOM_HISTORY_MS: u32 = 2_000;
 /// Carry 75's bound: every client is admitted before the room has stamped this many events past its start.
 pub(crate) const ADMIT_EVENTS_BOUND: u64 = 32_768;
 /// The highest feet a scripted spawn or teleport may set, cm: World's stated clamp bound (TUNE-2; world.rs:653-660),
-/// the position grid's top less the fire origin's 140 cm above the feet, read from npt-sim's grid.
+/// the position grid's top less the fire origin's 140 cm above the feet, a copy of npt-sim's private `CHEST_CM`
+/// (sim/src/world.rs:79; its `FEET_Z_BOUND_CM`, :71), with the grid's top read from npt-sim's grid.
 pub(crate) const PLACEMENT_Z_BOUND_CM: i32 = npt_sim::grid::POSITION_Z.max() - 140;
 
 /// The simulated clock (plan.md:617): integer ns, which only moves forward.
@@ -211,7 +214,9 @@ impl Bites {
             client: npt_client::BITES.contains(&id) || proto,
             decoder: proto,
             l1: L1Net::BITES.contains(&id),
-            schedule: npt_server::BITES.contains(&id),
+            // npt-server's list also holds the pump's id, which the schedule never plants (advisor-u4r-r1, FP-6 item 2:
+            // EA-U4R-3 = RR3-L8): the schedule's own answer decides.
+            schedule: Schedule::with_bite(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP, id).is_some(),
             pump: Decoder::new(
                 Quant::default(),
                 npt_sim::TICK_HZ,
@@ -263,12 +268,10 @@ impl Bites {
     fn world(&self, scene: &Scene, seed: u64, first_tick: u32, arm_tick: u32) -> World {
         #[cfg(feature = "bites")]
         if self.world
-            && let Some(w) = self
-                .id
-                .as_deref()
-                .and_then(|id| World::with_bite(scene, seed, first_tick, id, arm_tick))
+            && let Some(id) = self.id.as_deref()
         {
-            return w;
+            return World::with_bite(scene, seed, first_tick, id, arm_tick)
+                .expect("Bites::of checked that this instance plants it");
         }
         let _ = arm_tick;
         World::new(scene, seed, first_tick)
@@ -301,9 +304,10 @@ impl Bites {
     fn net(&self, seed: u64) -> L1Net {
         #[cfg(feature = "bites")]
         if self.l1
-            && let Some(net) = self.id.as_deref().and_then(|id| L1Net::with_bite(seed, id))
+            && let Some(id) = self.id.as_deref()
         {
-            return net;
+            return L1Net::with_bite(seed, id)
+                .expect("Bites::of checked that this instance plants it");
         }
         L1Net::new(seed)
     }
@@ -311,12 +315,10 @@ impl Bites {
     fn schedule(&self) -> Schedule {
         #[cfg(feature = "bites")]
         if self.schedule
-            && let Some(s) = self
-                .id
-                .as_deref()
-                .and_then(|id| Schedule::with_bite(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP, id))
+            && let Some(id) = self.id.as_deref()
         {
-            return s;
+            return Schedule::with_bite(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP, id)
+                .expect("Bites::of checked that this instance plants it");
         }
         Schedule::new(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP)
     }
@@ -487,10 +489,16 @@ impl Server {
             let Some(x) = self.sessions[i].as_mut() else {
                 continue;
             };
-            if x.decoder.is_closed() {
-                let slot = x.slot;
+            // Carry 76 (advisor-u4r-r1, FP-1): after the pump drained what had arrived, a closed decoder or link ends
+            // the session, and the room's slot is freed when this session holds it (`Room::leave`: "a closed link").
+            if x.decoder.is_closed() || x.link.state() == LinkState::Closed {
+                let (id, slot) = (x.id, x.slot);
                 self.sessions[i] = None;
                 if let Some(slot) = slot {
+                    if self.owner.get(&slot) == Some(&id) {
+                        self.room.leave(slot);
+                        self.owner.remove(&slot);
+                    }
                     self.ended.push((slot, "closed"));
                 }
                 continue;
@@ -575,7 +583,9 @@ pub(crate) struct Settings {
 }
 
 /// A runner recording made again (carry 82; u4r-brief.md 2.1 (ii)): `npt-lab scenario --record` at these settings, the
-/// brawl, no log kept, its bytes handed back.
+/// brawl, no log kept, its bytes handed back. Its argv holds these flags only, each value one item (never a line split
+/// on whitespace: advisor-u4r-r1, FP-3), so no manifest value can add a flag, and the advisor's U4r build ruling 1,
+/// point 9 (a)'s refusal of `--lead` and `--every-step` holds on this path by construction.
 pub(crate) fn record(
     scene: &Scene,
     profile: &str,
@@ -585,13 +595,43 @@ pub(crate) fn record(
     warmup_s: u64,
     measured_s: u64,
 ) -> Result<Vec<u8>, String> {
-    let line = format!(
-        "--run regenerate --set regenerate --date 2000-01-01 --commit {} --out - --scene {} --profile {profile} \
-         --seed {seed} --clients {clients} --tick-start {first_tick} --warmup-s {warmup_s} --measured-s {measured_s}",
+    let (commit, seed, clients, first_tick, warmup_s, measured_s) = (
         "0".repeat(40),
-        scene.name()
+        seed.to_string(),
+        clients.to_string(),
+        first_tick.to_string(),
+        warmup_s.to_string(),
+        measured_s.to_string(),
     );
-    let argv: Vec<String> = line.split_whitespace().map(str::to_owned).collect();
+    let argv: Vec<String> = [
+        "--run",
+        "regenerate",
+        "--set",
+        "regenerate",
+        "--date",
+        "2000-01-01",
+        "--commit",
+        &commit,
+        "--out",
+        "-",
+        "--scene",
+        scene.name(),
+        "--profile",
+        profile,
+        "--seed",
+        &seed,
+        "--clients",
+        &clients,
+        "--tick-start",
+        &first_tick,
+        "--warmup-s",
+        &warmup_s,
+        "--measured-s",
+        &measured_s,
+    ]
+    .into_iter()
+    .map(str::to_owned)
+    .collect();
     let mut s = parse(&argv)?;
     s.logs = false;
     s.record_mem = true;
@@ -846,6 +886,9 @@ pub(crate) fn run(s: &Settings) -> Result<Summary, Failure> {
         // (2) the server's I/O step.
         server.io(now);
         for (slot, session, tick) in std::mem::take(&mut server.admitted) {
+            // Carry 75's bound is checked on logged runs only: `events_stamped` counts what `room_tick` returns, which
+            // a no-log run (--logs 0, or `npt-lab hash`'s regeneration) does not call, so there it stays 0
+            // (advisor-u4r-r1, FP-7 item 6: RR3-L2 (b)).
             if events_stamped >= ADMIT_EVENTS_BOUND {
                 return Err(fail(format!(
                     "slot {slot} admitted after {events_stamped} events past the event start (carry 75's bound \
@@ -1319,9 +1362,23 @@ pub(crate) fn parse(argv: &[String]) -> Result<Settings, String> {
     let event_start = small(num(take("event-start"), "event-start", 0)?, "event-start")?;
     let cmd_start = small(num(take("cmd-start"), "cmd-start", 0)?, "cmd-start")?;
     let hz = u64::from(npt_sim::TICK_HZ);
-    let warmup_ticks = num(take("warmup-s"), "warmup-s", 10)? * hz;
-    let measured_ticks = num(take("measured-s"), "measured-s", 60)? * hz;
-    let after_max_ticks = num(take("after-max-s"), "after-max-s", 30)? * hz;
+    // Checked products and sums (advisor-u4r-r1, FP-3: SEC-U4R-3, RR3-L3): each overflow a usage error naming its
+    // option, so `run`'s plain sums of these lengths cannot overflow.
+    let mut ticks_of = |k: &str, default: u64| -> Result<u64, String> {
+        let s = num(take(k), k, default)?;
+        s.checked_mul(hz).ok_or(format!(
+            "--{k} {s}: its ticks ({s} x {hz}) overflow 64 bits"
+        ))
+    };
+    let warmup_ticks = ticks_of("warmup-s", 10)?;
+    let measured_ticks = ticks_of("measured-s", 60)?;
+    let after_max_ticks = ticks_of("after-max-s", 30)?;
+    let measured_end = warmup_ticks
+        .checked_add(measured_ticks)
+        .ok_or("--warmup-s and --measured-s: their ticks' sum overflows 64 bits".to_owned())?;
+    measured_end
+        .checked_add(after_max_ticks)
+        .ok_or("--after-max-s: the run's last tick overflows 64 bits".to_owned())?;
     let behaviour = take("behaviour").unwrap_or_else(|| "brawl".to_owned());
     if !["brawl", "presses", "clamp", "steady"].contains(&behaviour.as_str()) {
         return Err(format!(
@@ -1382,6 +1439,18 @@ pub(crate) fn parse(argv: &[String]) -> Result<Settings, String> {
                 .to_owned(),
         );
     }
+    // The last burst's tick, checked before `burst_starts` sums it (FP-3: RR3-L3), which bounds every sum there.
+    if profile_name == "B1" && bursts > 0 {
+        (bursts - 1)
+            .checked_mul(burst_every_s)
+            .and_then(|x| x.checked_add(burst_first_s))
+            .and_then(|x| x.checked_mul(hz))
+            .and_then(|x| x.checked_add(warmup_ticks))
+            .ok_or(format!(
+                "--bursts {bursts}, --burst-first-s {burst_first_s} and --burst-every-s {burst_every_s}: the last \
+                 burst's tick overflows 64 bits"
+            ))?;
+    }
     let profile = match profile_name.as_str() {
         "B1" => Profile::b1(&burst_starts(
             seed,
@@ -1396,6 +1465,13 @@ pub(crate) fn parse(argv: &[String]) -> Result<Settings, String> {
         None => Bites::default(),
         Some(id) => Bites::of(id)?,
     };
+    // advisor-u4r-r1, FP-6 item 1 (NC-U4R-2): an id whose only instance is the -30 pre-check's late-command replay
+    // client is planted by no instance of a scenario run, so it is a usage error here (u4r-brief.md 2.1).
+    if let (true, Some(id)) = (bites.late_command, &bite) {
+        return Err(format!(
+            "--bite {id} is the -30 pre-check's (npt-lab precheck)"
+        ));
+    }
     let lead = lead_override.unwrap_or_else(|| lead_for(&profile));
     let burst_spacing_ns = if profile.bursts_ns.is_empty() {
         0
@@ -1630,30 +1706,33 @@ pub(crate) mod tests {
     }
 
     /// r1's comparison: the fate logs' SHA-256s, the room's per-tick World hash `h` and `hs.server_hash`, and the
-    /// clients' `hs.decoded_hash` and `tick.s.snap_incomplete`, over the same ticks. `Ok` with the ticks compared, or
-    /// the first difference.
-    fn twin_differences(a: &Path, b: &Path) -> Result<usize, String> {
+    /// clients' `hs.decoded_hash` and `tick.s.snap_incomplete`, over the same ticks. `Ok` with the tick records compared
+    /// per process (room, client), or the first difference.
+    fn twin_differences(a: &Path, b: &Path) -> Result<[usize; 2], String> {
         if let Some(e) = fate_difference(a, b) {
             return Err(e);
         }
         tick_differences(a, b)
     }
 
+    /// A run's `fate_log` record's SHA-256, if its lab log holds one.
+    fn fate_of(d: &Path) -> Option<serde_json::Value> {
+        read(d, "lab")
+            .events
+            .iter()
+            .find(|e| e.ev == "fate_log")
+            .map(|e| e.f["sha256"].clone())
+    }
+
     fn fate_difference(a: &Path, b: &Path) -> Option<String> {
-        let fate = |d: &Path| {
-            read(d, "lab")
-                .events
-                .iter()
-                .find(|e| e.ev == "fate_log")
-                .map(|e| e.f["sha256"].clone())
-        };
-        (fate(a) != fate(b)).then(|| format!("fate logs {:?} and {:?}", fate(a), fate(b)))
+        (fate_of(a) != fate_of(b))
+            .then(|| format!("fate logs {:?} and {:?}", fate_of(a), fate_of(b)))
     }
 
-    /// The per-tick half of r1's comparison.
-    fn tick_differences(a: &Path, b: &Path) -> Result<usize, String> {
-        let mut compared = 0;
-        for process in ["room", "client"] {
+    /// The per-tick half of r1's comparison: the tick records compared per process (room, client).
+    fn tick_differences(a: &Path, b: &Path) -> Result<[usize; 2], String> {
+        let mut compared = [0; 2];
+        for (i, process) in ["room", "client"].into_iter().enumerate() {
             let (x, y) = (read(a, process), read(b, process));
             if x.ticks.len() != y.ticks.len() {
                 return Err(format!(
@@ -1670,7 +1749,7 @@ pub(crate) mod tests {
                 {
                     return Err(format!("{process}: tick {} (and {}) differs", p.t, q.t));
                 }
-                compared += 1;
+                compared[i] += 1;
             }
         }
         Ok(compared)
@@ -1690,11 +1769,22 @@ pub(crate) mod tests {
     #[test]
     fn r1_two_processes_of_one_seed_give_equal_fate_logs_and_per_tick_hashes() {
         let [a, b] = two_legs("r1", false);
-        let compared = twin_differences(&a, &b).unwrap_or_else(|e| panic!("r1: {e}"));
-        assert!(compared > 200, "ticks compared: {compared}");
+        // advisor-u4r-r1, FP-6 item 5 (T-4): both fates are there (None against None is no comparison), and each
+        // process's half compared records in both legs (its count, not the sum).
+        assert!(
+            fate_of(&a).is_some(),
+            "the first leg's lab log holds its fate_log record"
+        );
+        let [room, client] = twin_differences(&a, &b).unwrap_or_else(|e| panic!("r1: {e}"));
+        assert!(
+            room > 0 && client > 0,
+            "tick records compared: room {room}, client {client}"
+        );
+        assert!(room + client > 200, "ticks compared: {room} + {client}");
         assert_eq!(digest(&a), digest(&b), "slot order in both");
         println!(
-            "r1: {compared} tick records compared, equal; connect order digest {}",
+            "r1: {room} room and {client} client tick records compared, equal; fate log {:?}; connect order digest {}",
+            fate_of(&a),
             digest(&a)
         );
     }
@@ -1808,8 +1898,9 @@ pub(crate) mod tests {
         );
     }
 
-    /// r4's run: SS, 48 clients in 3 x 16, P2, 6 s of warm-up and 2 s measured.
-    fn r4_run(seed: u64, bite: &str) -> Summary {
+    /// r4's run: SS, 48 clients in 3 x 16, P2, 6 s of warm-up and 2 s measured; its scratch guard is handed back, so a
+    /// failing caller keeps the logs (advisor-u4r-r1, FP-6 item 6: T-5).
+    fn r4_run(seed: u64, bite: &str) -> (Scratch, Summary) {
         let dir = scratch(&format!(
             "r4-{seed}{}",
             if bite.is_empty() { "" } else { "-bite" }
@@ -1818,7 +1909,8 @@ pub(crate) mod tests {
             &dir,
             &format!("--seed {seed} --profile P2 --warmup-s 6 --measured-s 2 {bite}"),
         );
-        run(&s).unwrap_or_else(|e| panic!("r4 seed {seed}: {e:?}"))
+        let o = run(&s).unwrap_or_else(|e| panic!("r4 seed {seed}: {e:?}"));
+        (dir, o)
     }
 
     /// r4, the class audit (catalogue §6 at U4, item 5): every group the encoder writes into each client's snapshots
@@ -1827,7 +1919,7 @@ pub(crate) mod tests {
     #[test]
     fn r4_every_group_is_one_its_class_lets_that_client_see_on_three_seeds() {
         for seed in 1..=3 {
-            let o = r4_run(seed, "");
+            let (_guard, o) = r4_run(seed, "");
             let a = &o.audit;
             println!(
                 "r4 seed {seed}: {} ticks; audited owner {} (own-state records), spatial {} checked against the \
@@ -1853,7 +1945,7 @@ pub(crate) mod tests {
     #[cfg(feature = "bites")]
     #[test]
     fn r4_bite_owner_to_all_fails_the_audit_by_name() {
-        let o = r4_run(1, "--role bite --bite catalogue-5/owner-to-all");
+        let (_guard, o) = r4_run(1, "--role bite --bite catalogue-5/owner-to-all");
         let first = o.audit.breaches.first().cloned().unwrap_or_default();
         println!(
             "r4 bite: {} breaches (16 texts kept); the first: {first}",
@@ -1871,6 +1963,18 @@ pub(crate) mod tests {
         server: &mut Server,
         slot: u8,
         now: &mut u64,
+    ) -> (npt_link::Loopback, ClientEndpoint, u64) {
+        let (mut c, mut e, id) = join_only(server, slot, now);
+        deliver(server, &mut c, &mut e, *now);
+        (c, e, id)
+    }
+
+    /// `joined`'s join alone: the session accepted and its `Join` taken by the server's I/O step, before the room's
+    /// next tick.
+    fn join_only(
+        server: &mut Server,
+        slot: u8,
+        now: &mut u64,
     ) -> (npt_link::Loopback, ClientEndpoint, u64) {
         let (s, mut c) = npt_link::Loopback::pair(MAX_DATAGRAM);
         let id = server.accept(Box::new(s), *now);
@@ -1881,7 +1985,6 @@ pub(crate) mod tests {
         }
         *now += 1;
         server.io(*now);
-        deliver(server, &mut c, &mut e, *now);
         (c, e, id)
     }
 
@@ -1900,13 +2003,6 @@ pub(crate) mod tests {
         e.input(None, &got, now + 1);
     }
 
-    fn room_counts(server: &Server, slot: u8) -> u64 {
-        server.room.client(slot).map_or(0, |c| {
-            let k = c.counters();
-            k.commands_buffered + k.commands_repeat + k.late + k.buffer_full + k.press_refused
-        })
-    }
-
     fn send_input(c: &mut npt_link::Loopback, e: &mut ClientEndpoint, now: u64) {
         e.input(Some(npt_sim::Command::default()), &[], now);
         for d in e.step().datagrams {
@@ -1937,15 +2033,17 @@ pub(crate) mod tests {
         Server::new(room, Quant::default(), RttSource::Link)
     }
 
-    /// r8 (carry 76): a second endpoint joins with slot 0's token and bind while the first is live; after it the
-    /// first's Inputs and `Leave` reach no room call, and the second keeps the slot.
+    /// r8 (carry 76): a second endpoint joins with slot 0's token and bind while the first is live; the first is sent
+    /// `Kick(replaced)` once; after it the first's Inputs and `Leave` reach no room call (the slot's whole counters
+    /// unchanged), and the second keeps the slot.
     #[test]
     fn r8_a_replaced_session_s_inputs_and_leave_reach_no_room_call() {
         let mut server = lab_room(4);
         let mut now = 1_000;
         let (mut c1, mut e1, first) = joined(&mut server, 0, &mut now);
         assert_eq!(server.owner.get(&0), Some(&first));
-        let (_c2, _e2, second) = joined(&mut server, 0, &mut now);
+        // The second join, read right after it, before the room's next tick (FP-2 (b)).
+        let (mut c2, mut e2, second) = join_only(&mut server, 0, &mut now);
         assert_eq!(
             server.owner.get(&0),
             Some(&second),
@@ -1955,14 +2053,30 @@ pub(crate) mod tests {
             server.ended.contains(&(0, "replaced")),
             "the first was told and dropped"
         );
-        let before = room_counts(&server, 0);
+        // advisor-u4r-r1, FP-2 (b): the first is told once, on Control, `Kick(replaced)`.
+        let mut control = Vec::new();
+        while let Some(r) = c1.poll_recv(now) {
+            if r.channel == Channel::Control {
+                control.push(r.bytes);
+            }
+        }
+        assert_eq!(control.len(), 1, "one Control-channel message to the first");
+        assert_eq!(
+            Quant::default().decode(Direction::RoomToClient, Channel::Control, &control[0]),
+            Ok(Message::Kick(Kick::Replaced)),
+            "the first's Control message is Kick(replaced)"
+        );
+        deliver(&mut server, &mut c2, &mut e2, now);
+        // FP-2 (a): the slot's whole counters (`ClientCounters`, ack_refused and ack_refused_commands among them);
+        // nothing ticks inside the loop.
+        let before = server.room.client(0).map(|c| c.counters().clone());
         for _ in 0..5 {
             now += 33_000_000;
             send_input(&mut c1, &mut e1, now);
             server.io(now + 1);
         }
         assert_eq!(
-            room_counts(&server, 0),
+            server.room.client(0).map(|c| c.counters().clone()),
             before,
             "the first's Inputs reach no room call"
         );
@@ -2031,6 +2145,73 @@ pub(crate) mod tests {
         );
     }
 
+    /// FP-1's three checks of a session that ended closed on `slot`: the room's slot freed, no owner, and `(slot,
+    /// "closed")` in `ended`; each that fails, by name.
+    fn closed_checks(server: &Server, slot: u8) -> Vec<String> {
+        let mut failed = Vec::new();
+        if server.room.client(slot).is_some() {
+            failed.push(format!(
+                "the room still holds slot {slot}: its slot was not freed"
+            ));
+        }
+        if server.owner.contains_key(&slot) {
+            failed.push(format!("the lab's owner map still holds slot {slot}"));
+        }
+        if !server.ended.contains(&(slot, "closed")) {
+            failed.push(format!("ended holds no ({slot}, \"closed\")"));
+        }
+        failed
+    }
+
+    /// Carry 76's boundary (advisor-u4r-r1, FP-1: NC-U4R-1 = SEC-U4R-1): a session whose decoder closes, (a) on a
+    /// control-stream frame it refuses as malformed, or whose link closes, (b) its client end dropped, ends, and the
+    /// room's slot is freed (`Room::leave`: "a closed link"); after (a) a new endpoint with that slot's token and bind is
+    /// admitted, and not as a replacement. Each case's checks are collected, so a failure names its case.
+    #[test]
+    fn fp1_a_closed_decoder_or_link_ends_the_session_and_frees_the_room_s_slot() {
+        // (a) A frame the pump refuses with an error other than WrongChannel or WrongDirection: `close_malformed`.
+        let mut a = lab_room(4);
+        let mut now = 1_000;
+        let (mut c, _e, _) = joined(&mut a, 0, &mut now);
+        let bad = [0xffu8; 3];
+        let refused = Quant::default()
+            .decode(Direction::ClientToRoom, Channel::Control, &bad)
+            .expect_err("the decoder refuses the frame");
+        assert!(
+            !matches!(
+                refused,
+                npt_proto::DecodeError::WrongChannel | npt_proto::DecodeError::WrongDirection
+            ),
+            "refused as {refused:?}: a forged kind, which the pump counts and keeps"
+        );
+        c.send_reliable(Channel::Control, &bad, now);
+        now += 1;
+        a.io(now);
+        let case_a = closed_checks(&a, 0);
+        let (_c2, _e2, again) = joined(&mut a, 0, &mut now);
+        // (b) The client end dropped: the server end's link reads Closed.
+        let mut b = lab_room(4);
+        let mut now_b = 1_000;
+        let (c, _e, _) = joined(&mut b, 0, &mut now_b);
+        drop(c);
+        b.io(now_b + 1);
+        let case_b = closed_checks(&b, 0);
+        println!("fp1: case (a) {case_a:?}; case (b) {case_b:?}; (a)'s new session {again}");
+        assert!(
+            case_a.is_empty() && case_b.is_empty(),
+            "case (a), the decoder closed: {case_a:?}; case (b), the link closed: {case_b:?}"
+        );
+        assert_eq!(
+            a.owner.get(&0),
+            Some(&again),
+            "(a): the new endpoint is admitted"
+        );
+        assert!(
+            !a.ended.contains(&(0, "replaced")),
+            "(a): the new endpoint is no replacement"
+        );
+    }
+
     /// The reliable messages' channel (the module text): exactly one stream channel takes each, never a fall-back.
     #[test]
     fn s4_a_reliable_message_goes_on_the_one_channel_that_takes_it() {
@@ -2236,6 +2417,39 @@ pub(crate) mod tests {
         }
     }
 
+    /// advisor-u4r-r1, FP-3 (SEC-U4R-3, RR3-L3): a length whose ticks overflow 64 bits is a usage error naming its
+    /// option, never a wrapped length.
+    #[test]
+    fn fp3_a_length_whose_ticks_overflow_is_a_usage_error_naming_its_option() {
+        let line = format!(
+            "--run t --set t --date 2026-11-14 --commit {COMMIT} --out x --scene SS --seed 1 --profile P0 \
+             --measured-s 1000000000000000000"
+        );
+        let argv: Vec<String> = line.split_whitespace().map(str::to_owned).collect();
+        let e = parse(&argv).expect_err("--measured-s 10^18 x 30 overflows");
+        assert!(e.starts_with("--measured-s 1000000000000000000: "), "{e}");
+    }
+
+    /// advisor-u4r-r1, FP-6 items 1 and 2 (NC-U4R-2; EA-U4R-3 = RR3-L8; bites builds only): `scenario` refuses the
+    /// pre-check's late-command id as a usage error, and `Bites::of` gives the pump's wallclock id to the pump alone,
+    /// never to the schedule (whose `with_bite` refuses it).
+    #[cfg(feature = "bites")]
+    #[test]
+    fn fp6_the_late_command_id_is_refused_by_scenario_and_the_wallclock_id_is_the_pump_s_alone() {
+        let line = format!(
+            "--run t --set t --date 2026-11-14 --commit {COMMIT} --out x --scene SS --seed 1 --profile P0 \
+             --role bite --bite PM-0001-30/late-command"
+        );
+        let argv: Vec<String> = line.split_whitespace().map(str::to_owned).collect();
+        let e = parse(&argv).expect_err("the pre-check's id");
+        assert_eq!(
+            e,
+            "--bite PM-0001-30/late-command is the -30 pre-check's (npt-lab precheck)"
+        );
+        let b = Bites::of("PM-0001-30/wallclock").expect("the pump's id");
+        assert!(!b.schedule && b.pump, "{b:?}");
+    }
+
     /// The measured phase's sum of `tick.s.snap_incomplete` in a run's client log, and whether its lab log holds -57's
     /// series.
     fn incomplete_and_57(dir: &Path) -> (i64, bool) {
````

### spikes/netcode-proof/crates/server/tests/schedule.rs
_ 1 file changed, 3 insertions(+), 1 deletion(-); the diff itself is in `.checkpoints/netcode-proof-test/u4r-r2-packet-tests.md` (`git diff --no-index .gate/u4r-snap1/spikes/netcode-proof/crates/server/tests/schedule.rs .gate/u4r-snap2/spikes/netcode-proof/crates/server/tests/schedule.rs`)._

### spikes/netcode-proof/ops/runs/c-u4r-r2.mjs
_`git diff --no-index .gate/u4r-snap1/spikes/netcode-proof/ops/runs/c-u4r-r2.mjs .gate/u4r-snap2/spikes/netcode-proof/ops/runs/c-u4r-r2.mjs`:  1 file changed, 137 insertions(+), 60 deletions(-)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/ops/runs/c-u4r-r2.mjs b/.gate/u4r-snap2/spikes/netcode-proof/ops/runs/c-u4r-r2.mjs
index b8df242..7653965 100644
--- a/.gate/u4r-snap1/spikes/netcode-proof/ops/runs/c-u4r-r2.mjs
+++ b/.gate/u4r-snap2/spikes/netcode-proof/ops/runs/c-u4r-r2.mjs
@@ -4,67 +4,109 @@
 // the record, before -17's judged runs and before carry 62 (h)'s setup check is decided (7.1).
 //   node spikes/netcode-proof/ops/runs/c-u4r-r2.mjs --date YYYY-MM-DD
 // - Each suite's three lines exactly as the builder ran them (.gate/u4r/c3-r2-p0.log, c3-r2-suites.log and
-//   c3-r2-bites.log, U4r's copy 3; then copy 5's runs of ruling 3), but for --date, --commit and --out: `npt-lab
-//   scenario` (SS or ES, 48 clients, seed 1, a 10 s warm-up and 20 s measured unless the suite names other lengths),
-//   then `npt-lab contract --logs <its folder> --series marks/SERIES.md`, then `npt-judge --marks marks/marks.json
-//   --logs <its folder> --only <the run's marks> --out <suite>-verdict.json`.
+//   c3-r2-bites.log, U4r's copy 3; then copy 5's runs of ruling 3), but for --date, --commit, --out and the copies of
+//   marks.json and SERIES.md: `npt-lab scenario` (SS or ES, 48 clients, seed 1, a 10 s warm-up and 20 s measured unless
+//   the suite names other lengths), then `npt-lab contract --logs <its folder> --series <SERIES.md's copy>`, then
+//   `npt-judge --marks <marks.json's copy> --logs <its folder> --only <the run's marks> --out <suite>-verdict.json`.
 // - The suites: the ten measured ones of copy 3 (p0, p2, p3, b1; -21's option-B stand-ins forged, clamp, fast-clock and
 //   lag-switch; es, carry 59's; evstart, carry 75's); ruling 3's point 3 pair (the tick16 bite at tick_start 64,651, so
 //   the 16-bit wrap falls at measured tick 585, 15 before the measured end; and its clean partner at 64,936, the wrap at
-//   measured tick 300); and point 4's bitten B1 run (PM-0001-18/no-fallback; since point 8 (A) the mirror keeps the
-//   frame the room's stale copy codes against). Copy 3's three earlier tick16 tries (tick_start 65,506) stay builder
-//   evidence and are not re-made.
+//   measured tick 300); point 4's bitten B1 run (PM-0001-18/no-fallback; since point 8 (A) the mirror keeps the frame
+//   the room's stale copy codes against); and carry 62 (h)'s bite, PM-0001-17/whole-as-lost, on the p0 suite's
+//   settings (advisor-u4r-r1, FP-5 (i): T-7; the builder's .gate/u4r/whole-as-lost/ is a labelled dry check only).
+//   Copy 3's three earlier tick16 tries (tick_start 65,506) stay builder evidence and are not re-made.
 // - Every command runs whatever the one before gave, each exit code recorded: a FAIL is a result here, never a stop and
-//   never re-run (condition 5). Expected, by ruling 3: b1 fails PM-0001-2's everyone-in-view share, so its -17 and -18
-//   are INVALID and not counted (point 1: npt-room's, fixed in U4g); es fails ES's reliable-events band (point 2: U4h's);
+//   never re-run (condition 5). Each command runs through check.mjs's run() with a 600 s limit, in its own process
+//   group, which is killed whole at the limit: its exit line then reads "stopped at <s> s", a result, never a retry
+//   (FP-5 (f): INF-U4R-11). Expected, by ruling 3: b1 fails PM-0001-2's everyone-in-view share, so its -17 and -18 are
+//   INVALID and not counted (point 1: npt-room's, fixed in U4g); es fails ES's reliable-events band (point 2: U4h's);
 //   b1-nofallback fails -18's max check and is INVALID by pre:PM-0001-2 while the room is unfixed (points 4 and 8 (A)).
 //   Each suite's line in r2.txt states what is expected beside what came; the main session reads both.
 // - Run it under the lane lock, exclusive (u4r-brief.md section 4 rule 2: it holds bite runs, and its outputs are a
 //   record): `bash <runner> -x <copy> <log> node ops/runs/c-u4r-r2.mjs --date ...` or the main folder's own runner.
-// - It refuses uncommitted changes to tracked files (every log names its commit). Its outputs go under
-//   .gate/netcode-proof-test/r2-<commit12>/ (gitignored): one folder per suite (its three logs), <suite>-verdict.json
-//   beside it, the binaries under bin/, and r2.txt (the commit, the binaries' SHA-256s, then per suite each command, its
-//   exit code, its wall seconds and its output). Never overwritten. It stops before a run when less than 4 GB is free
-//   (section 4 rule 6's floor). The binaries: npt-lab's measured and bites builds and npt-judge, built --release
-//   --locked, each artifact's path read from cargo's --message-format=json messages (check.mjs's builtArtifact) and
-//   copied out before the next build replaces it.
+// - Its order (FP-5 (a): INF-U4R-2): the usage and git checks (no uncommitted change to a tracked file, and no untracked
+//   file under spikes/netcode-proof/crates: every log names its commit); each suite's key checked as a name (one or more
+//   of A-Z, a-z, 0-9 and -: FP-5 (h)); the projection; each release build (--release --locked --offline), with at least
+//   4 GiB free before each (section 4 rule 6); the projection again; only then the output folder,
+//   .gate/netcode-proof-test/r2-<commit12>/ (gitignored, never overwritten): one folder per suite (its three logs),
+//   <suite>-verdict.json beside it, the copies of marks/marks.json and marks/SERIES.md (their sha256s in r2.txt's head:
+//   FP-5 (e)), and r2.txt (the commit, the binaries' SHA-256s, then per suite each command, its exit line and its
+//   output). The binaries go to .gate/netcode-proof-test/bin/r2-<commit12>/, which no exists-check reads, so a stop
+//   before the first suite leaves nothing that blocks a re-run at that commit. Before r2.txt is written, HEAD and the
+//   tracked files' status are checked against their start values (FP-5 (e)).
+// - The projection (FP-5 (b), (c)): the builder's own 13 suites took about 145 MB (u4r-build.md, copy 5's state: the
+//   r2 folder; their 13 folders sum to 115,746,371 B, du -sb in copy 6), plus whole-as-lost at the p0 suite's size
+//   (24,813,722 B); it starts only when free space minus that total is at least 4 GiB, printing both; before each suite,
+//   free space minus that suite's own measured size must be at least 4 GiB.
+// - The binaries: npt-lab's measured and bites builds and npt-judge, each artifact's path read from cargo's
+//   --message-format=json messages (check.mjs's builtArtifact) and copied out before the next build replaces it.
+// - Its pure helpers are exported (the projection, the name check); main runs only when this file is the entry module.
 
 import { spawnSync } from "node:child_process";
 import { createHash } from "node:crypto";
 import { copyFileSync, existsSync, mkdirSync, readFileSync, statfsSync, writeFileSync } from "node:fs";
-import { dirname, join } from "node:path";
+import { dirname, join, resolve } from "node:path";
 import { fileURLToPath } from "node:url";
-import { builtArtifact } from "../../check.mjs";
+import { builtArtifact, run } from "../../check.mjs";
 
 const SPIKE = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
 const ROOT = join(SPIKE, "..", "..");
 const GATE = join(ROOT, ".gate", "netcode-proof-test");
-/** The free space below which no run starts, bytes (section 4 rule 6's 4 GB floor). */
-const FLOOR = 4 * 1024 ** 3;
+/** The free space a build and a suite must leave, bytes (section 4 rule 6's 4 GB floor, as 4 GiB). */
+export const FLOOR = 4 * 1024 ** 3;
+/** Each command's limit, ms (FP-5 (f)): the longest suite command the builder ran took well under it. */
+const STEP_LIMIT_MS = 600_000;
 const MARKS_17 = "PM-0001-2,PM-0001-17,PM-0001-57";
 const B1 = "--profile B1 --seed 1 --tick-start 63736 --warmup-s 10 --measured-s 30 --bursts 2 --burst-first-s 5 --burst-every-s 15";
+const P0 = "--scene SS --profile P0 --seed 1 --tick-start 63736 --warmup-s 10 --measured-s 20";
+/** The builder's 13 suites' bytes, about 145 MB (the module text), and the p0 suite's own (whole-as-lost's projection). */
+export const SUITES_13_BYTES = 145_000_000;
+const P0_SUITE_BYTES = 24_813_722;
+/** A suite key, a folder name (FP-5 (h)): one or more of A-Z, a-z, 0-9 and -. */
+const NAME = /^[A-Za-z0-9-]+$/;
 
 /**
  * The suites, in the order the builder ran them: `build` names the npt-lab build (`lab` measured, `bites`), `run` the
  * run id, `args` the scenario's settings after --out, `marks` its header's marks (the judge's --only), `expect` ruling
- * 3's or copy 3's outcome.
+ * 3's or copy 3's outcome, `bytes` the builder's folder of that suite (du -sb, copy 6; whole-as-lost: the p0 suite's).
  */
-const SUITES = [
-  { key: "p0", build: "lab", run: "u4r-r2-p0-1", args: "--scene SS --profile P0 --seed 1 --tick-start 63736 --warmup-s 10 --measured-s 20", marks: MARKS_17, suite: "r2-p0", expect: "contract PASS; PM-0001-2's SS checks, -17 (0 mismatches) and -57 pass on the run" },
-  { key: "p2", build: "lab", run: "u4r-r2-p2-1", args: "--scene SS --profile P2 --seed 1 --tick-start 63736 --warmup-s 10 --measured-s 20", marks: MARKS_17, suite: "r2-p2", expect: "contract PASS; PM-0001-2, -17 and -57 pass" },
-  { key: "p3", build: "lab", run: "u4r-r2-p3-1", args: "--scene SS --profile P3 --seed 1 --tick-start 63736 --warmup-s 10 --measured-s 20", marks: MARKS_17, suite: "r2-p3", expect: "contract PASS; PM-0001-2, -17 and -57 pass (carry 62 (g): P3, the reorder step on)" },
-  { key: "b1", build: "lab", run: "u4r-r2-b1-1", args: `--scene SS ${B1}`, marks: `${MARKS_17},PM-0001-18`, suite: "r2-b1", expect: "contract PASS; PM-0001-2 FAIL (everyone in view 0.273333, needs >= 0.99), so -17 and -18 L1 B1 INVALID by pre:PM-0001-2, recorded and not counted; -57 passes (ruling 3, point 1: npt-room's, fixed in U4g)" },
-  { key: "forged", build: "lab", run: "u4r-r2-forged-1", args: "--scene SS --profile P0 --seed 1 --warmup-s 10 --measured-s 20 --behaviour presses", marks: "PM-0001-2", suite: "forged", expect: "contract PASS; PM-0001-2 passes" },
-  { key: "clamp", build: "lab", run: "u4r-r2-clamp-1", args: "--scene SS --profile P0 --seed 1 --warmup-s 10 --measured-s 20 --behaviour clamp", marks: "PM-0001-2", suite: "clamp", expect: "contract PASS; PM-0001-2 passes" },
-  { key: "fast-clock", build: "lab", run: "u4r-r2-fast-clock-1", args: "--scene SS --profile P0 --seed 1 --warmup-s 10 --measured-s 20 --behaviour steady", marks: "PM-0001-2", suite: "fast-clock", expect: "contract PASS; PM-0001-2 passes" },
-  { key: "lag-switch", build: "lab", run: "u4r-r2-lag-switch-1", args: "--scene SS --profile P1 --seed 1 --warmup-s 10 --measured-s 20 --behaviour steady", marks: "PM-0001-2", suite: "lag-switch", expect: "contract PASS; PM-0001-2 passes" },
-  { key: "es", build: "lab", run: "u4r-r2-es-1", args: "--scene ES --profile P0 --seed 1 --warmup-s 10 --measured-s 20", marks: "PM-0001-2", suite: "r2-es", expect: "contract PASS; PM-0001-2 FAIL by ES's reliable events (50.05 a second against 9.5-10.5), the other 11 ES checks pass; no ES run counts at U4r (ruling 3, point 2: U4h's)" },
-  { key: "evstart", build: "lab", run: "u4r-r2-evstart-1", args: "--scene SS --profile P0 --seed 1 --tick-start 63736 --event-start 65533 --warmup-s 10 --measured-s 20", marks: "PM-0001-2,PM-0001-17", suite: "r2-evstart", expect: "contract PASS; PM-0001-2 and -17 (0 mismatches) pass (carry 75)" },
-  { key: "tick16", build: "bites", run: "u4r-r2-tick16-4", args: "--scene SS --profile P0 --seed 1 --tick-start 64651 --warmup-s 10 --measured-s 20 --after-max-s 5 --role bite --bite PM-0001-17/tick16", marks: "PM-0001-2,PM-0001-17", suite: "r2-tick16", expect: "contract PASS (0 mirror breaches); PM-0001-2 passes; -17's bite run FAILs by decoded_hash_mismatch_count with at least 48 mismatches; after_capped as it comes (false when the room's overflow resync resolves the measured ticks before the cap, true when the cap path of ruling 3, point 7 (a) ends the run); carry 62 (h)'s measured sum is the bite's own (tick16d: 720 = 48 x 15 post-wrap measured ticks) (ruling 3, point 3, as ruling 4, point 1 reads it; carry 73)" },
-  { key: "tick16-clean", build: "lab", run: "u4r-r2-tick16-clean-1", args: "--scene SS --profile P0 --seed 1 --tick-start 64936 --warmup-s 10 --measured-s 20 --after-max-s 5", marks: MARKS_17, suite: "r2-tick16-clean", expect: "contract PASS (0 mirror breaches); PM-0001-2, -17 (0 mismatches) and -57 pass; the after phase resolves without the cap (ruling 3, point 3)" },
-  { key: "b1-nofallback", build: "bites", run: "u4r-r2-b1-nofallback-1", args: `--scene SS ${B1} --role bite --bite PM-0001-18/no-fallback`, marks: `${MARKS_17},PM-0001-18`, suite: "r2-b1", expect: "contract PASS (0 mirror breaches, ruling 3, point 8 (A)); -18's max check FAILs (a heal later than 500 ms); INVALID by pre:PM-0001-2 is expected while npt-room is unfixed (ruling 3, points 4 and 8 (A))" },
+export const SUITES = [
+  { key: "p0", build: "lab", run: "u4r-r2-p0-1", args: P0, marks: MARKS_17, suite: "r2-p0", bytes: P0_SUITE_BYTES, expect: "contract PASS; PM-0001-2's SS checks, -17 (0 mismatches) and -57 pass on the run" },
+  { key: "p2", build: "lab", run: "u4r-r2-p2-1", args: "--scene SS --profile P2 --seed 1 --tick-start 63736 --warmup-s 10 --measured-s 20", marks: MARKS_17, suite: "r2-p2", bytes: 11_483_900, expect: "contract PASS; PM-0001-2, -17 and -57 pass" },
+  { key: "p3", build: "lab", run: "u4r-r2-p3-1", args: "--scene SS --profile P3 --seed 1 --tick-start 63736 --warmup-s 10 --measured-s 20", marks: MARKS_17, suite: "r2-p3", bytes: 6_509_038, expect: "contract PASS; PM-0001-2, -17 and -57 pass (carry 62 (g): P3, the reorder step on)" },
+  { key: "b1", build: "lab", run: "u4r-r2-b1-1", args: `--scene SS ${B1}`, marks: `${MARKS_17},PM-0001-18`, suite: "r2-b1", bytes: 13_900_772, expect: "contract PASS; PM-0001-2 FAIL (everyone in view 0.273333, needs >= 0.99), so -17 and -18 L1 B1 INVALID by pre:PM-0001-2, recorded and not counted; -57 passes (ruling 3, point 1: npt-room's, fixed in U4g)" },
+  { key: "forged", build: "lab", run: "u4r-r2-forged-1", args: "--scene SS --profile P0 --seed 1 --warmup-s 10 --measured-s 20 --behaviour presses", marks: "PM-0001-2", suite: "forged", bytes: 2_761_291, expect: "contract PASS; PM-0001-2 passes" },
+  { key: "clamp", build: "lab", run: "u4r-r2-clamp-1", args: "--scene SS --profile P0 --seed 1 --warmup-s 10 --measured-s 20 --behaviour clamp", marks: "PM-0001-2", suite: "clamp", bytes: 2_734_499, expect: "contract PASS; PM-0001-2 passes" },
+  { key: "fast-clock", build: "lab", run: "u4r-r2-fast-clock-1", args: "--scene SS --profile P0 --seed 1 --warmup-s 10 --measured-s 20 --behaviour steady", marks: "PM-0001-2", suite: "fast-clock", bytes: 2_761_213, expect: "contract PASS; PM-0001-2 passes" },
+  { key: "lag-switch", build: "lab", run: "u4r-r2-lag-switch-1", args: "--scene SS --profile P1 --seed 1 --warmup-s 10 --measured-s 20 --behaviour steady", marks: "PM-0001-2", suite: "lag-switch", bytes: 2_878_953, expect: "contract PASS; PM-0001-2 passes" },
+  { key: "es", build: "lab", run: "u4r-r2-es-1", args: "--scene ES --profile P0 --seed 1 --warmup-s 10 --measured-s 20", marks: "PM-0001-2", suite: "r2-es", bytes: 2_737_959, expect: "contract PASS; PM-0001-2 FAIL by ES's reliable events (50.05 a second against 9.5-10.5), the other 11 ES checks pass; no ES run counts at U4r (ruling 3, point 2: U4h's)" },
+  { key: "evstart", build: "lab", run: "u4r-r2-evstart-1", args: "--scene SS --profile P0 --seed 1 --tick-start 63736 --event-start 65533 --warmup-s 10 --measured-s 20", marks: "PM-0001-2,PM-0001-17", suite: "r2-evstart", bytes: 2_766_566, expect: "contract PASS; PM-0001-2 and -17 (0 mismatches) pass (carry 75)" },
+  { key: "tick16", build: "bites", run: "u4r-r2-tick16-4", args: "--scene SS --profile P0 --seed 1 --tick-start 64651 --warmup-s 10 --measured-s 20 --after-max-s 5 --role bite --bite PM-0001-17/tick16", marks: "PM-0001-2,PM-0001-17", suite: "r2-tick16", bytes: 2_937_440, expect: "contract PASS (0 mirror breaches); PM-0001-2 passes; -17's bite run FAILs by decoded_hash_mismatch_count with at least 48 mismatches; after_capped as it comes (false when the room's overflow resync resolves the measured ticks before the cap, true when the cap path of ruling 3, point 7 (a) ends the run); carry 62 (h)'s measured sum is the bite's own (tick16d: 720 = 48 x 15 post-wrap measured ticks) (ruling 3, point 3, as ruling 4, point 1 reads it; carry 73)" },
+  { key: "tick16-clean", build: "lab", run: "u4r-r2-tick16-clean-1", args: "--scene SS --profile P0 --seed 1 --tick-start 64936 --warmup-s 10 --measured-s 20 --after-max-s 5", marks: MARKS_17, suite: "r2-tick16-clean", bytes: 24_813_880, expect: "contract PASS (0 mirror breaches); PM-0001-2, -17 (0 mismatches) and -57 pass; the after phase resolves without the cap (ruling 3, point 3)" },
+  { key: "b1-nofallback", build: "bites", run: "u4r-r2-b1-nofallback-1", args: `--scene SS ${B1} --role bite --bite PM-0001-18/no-fallback`, marks: `${MARKS_17},PM-0001-18`, suite: "r2-b1", bytes: 14_647_138, expect: "contract PASS (0 mirror breaches, ruling 3, point 8 (A)); -18's max check FAILs (a heal later than 500 ms); INVALID by pre:PM-0001-2 is expected while npt-room is unfixed (ruling 3, points 4 and 8 (A))" },
+  { key: "whole-as-lost", build: "bites", run: "u4r-r2-whole-as-lost-1", args: `${P0} --role bite --bite PM-0001-17/whole-as-lost`, marks: "PM-0001-17", suite: "r2-whole-as-lost", bytes: P0_SUITE_BYTES, expect: "carry 62 (h)'s bite: measured incomplete equal to the client count (48), as in the builder's .gate/u4r/whole-as-lost/ dry check; -17's verdict as it comes until marks.json holds ruling 4's plain check" },
 ];
 
+/** The set's projected bytes (FP-5 (b)): the 13 suites' figure plus whole-as-lost's, or the suites' own sum if larger. */
+export function r2Projected(suites = SUITES) {
+  const sum = suites.reduce((s, x) => s + x.bytes, 0);
+  return Math.max(SUITES_13_BYTES + P0_SUITE_BYTES, sum);
+}
+
+/** Each suite key that is not a name, as a sentence (FP-5 (h)). */
+export const keyProblems = (suites = SUITES) =>
+  suites.filter((s) => !NAME.test(s.key)).map((s) => `suite key ${JSON.stringify(s.key)} is not one or more of A-Z, a-z, 0-9 and -: refused before any use`);
+
+/** Whether `free` bytes leave the 4 GiB floor after `projected` more, with the line that prints both. */
+export function roomFor(free, projected, what) {
+  const ok = free - projected >= FLOOR;
+  const mib = (b) => (b / 1024 ** 2).toFixed(0);
+  return {
+    ok,
+    line: `${what}: free ${mib(free)} MiB, projected ${mib(projected)} MiB, so ${mib(free - projected)} MiB after it against the 4 GiB floor (4096 MiB): ${ok ? "room" : "no room, stopped before it"}`,
+  };
+}
+
 function sh(cmd, args, cwd = SPIKE) {
   const r = spawnSync(cmd, args, { cwd, encoding: "utf8", maxBuffer: 1 << 28 });
   if (r.status !== 0) throw new Error(`${cmd} ${args.join(" ")}: exit ${r.status}\n${r.stdout}${r.stderr}`);
@@ -73,74 +115,109 @@ function sh(cmd, args, cwd = SPIKE) {
 
 const sha256 = (path) => createHash("sha256").update(readFileSync(path)).digest("hex");
 
-/** One release build, its artifact's path read from cargo's JSON messages and copied to `to`. */
+function freeBytes() {
+  const s = statfsSync(ROOT);
+  return s.bavail * s.bsize;
+}
+
+/** Stops unless free space minus `projected` bytes leaves the floor (0: a build's own check, 4 GiB free). */
+function floor(what, projected = 0) {
+  const r = roomFor(freeBytes(), projected, what);
+  if (!r.ok) throw new Error(r.line);
+}
+
+/** One release build, at least 4 GiB free before it, its artifact's path read from cargo's JSON messages and copied to `to`. */
 function build(args, target, to) {
-  const out = sh("cargo", ["build", "--release", "--locked", ...args, "--message-format=json"]);
+  floor(`the release build of ${target}${args.includes("bites") ? " (bites)" : ""}`);
+  const out = sh("cargo", ["build", "--release", "--locked", "--offline", ...args, "--message-format=json"]);
   const path = builtArtifact(out, { target, kind: "bin" });
   if (!path) throw new Error(`cargo build ${args.join(" ")} names no ${target} executable in its JSON messages`);
   copyFileSync(path, to);
   return to;
 }
 
-/** Stops before a run when less than FLOOR bytes are free where the logs go. */
-function floor(what) {
-  const s = statfsSync(ROOT);
-  const free = s.bavail * s.bsize;
-  if (free < FLOOR) throw new Error(`${what}: ${(free / 1024 ** 3).toFixed(1)} GB free, below the 4 GB floor: stopped before it`);
-}
-
-/** One command, run from the spike whatever it gives: its line, exit code, wall seconds and output. */
+/**
+ * One command, run from the spike whatever it gives, through check.mjs's run() with STEP_LIMIT_MS (its own process
+ * group, killed whole at the limit: FP-5 (f)): its line, its exit line ("stopped at <s> s" for a stopped one) and output.
+ */
 function step(exe, args) {
   const t0 = Date.now();
-  const r = spawnSync(exe, args, { cwd: SPIKE, encoding: "utf8", maxBuffer: 1 << 28 });
+  const r = run(exe, args, SPIKE, process.env, STEP_LIMIT_MS);
   const secs = ((Date.now() - t0) / 1000).toFixed(1);
-  const code = r.status ?? `signal ${r.signal}`;
-  return [`$ ${[exe, ...args].join(" ")}`, `exit ${code} (wall ${secs} s)`, `${r.stdout ?? ""}${r.stderr ?? ""}`.trimEnd()];
+  const exit = r.timedOut ? `stopped at ${secs} s` : `exit ${r.status ?? "by a signal"} (wall ${secs} s)`;
+  return [`$ ${[exe, ...args].join(" ")}`, exit, r.out.trimEnd()];
 }
 
 function main(argv) {
   const date = argv.includes("--date") ? argv[argv.indexOf("--date") + 1] : undefined;
   if (!/^\d{4}-\d{2}-\d{2}$/.test(date ?? "") || argv.length !== 2) throw new Error("usage: c-u4r-r2.mjs --date YYYY-MM-DD");
-  if (sh("git", ["status", "--porcelain", "--untracked-files=no"])) {
-    throw new Error("tracked files have uncommitted changes: commit first, since every log names its commit");
-  }
+  const tracked = () => sh("git", ["status", "--porcelain", "--untracked-files=no"]);
+  if (tracked()) throw new Error("tracked files have uncommitted changes: commit first, since every log names its commit");
+  const untracked = sh("git", ["status", "--porcelain", "--untracked-files=all", "--", "crates"]);
+  if (untracked) throw new Error(`files under spikes/netcode-proof/crates are not committed (${untracked.split("\n").join("; ")}): commit first, since every log names its commit`);
   const commit = sh("git", ["rev-parse", "HEAD"]);
+  const start = { commit, status: tracked() };
   const out = join(GATE, `r2-${commit.slice(0, 12)}`);
   if (existsSync(out)) throw new Error(`${out} exists: a run set is never overwritten`);
-  const bin = join(out, "bin");
+  const bad = keyProblems();
+  if (bad.length) throw new Error(bad.join("; "));
+  const total = r2Projected();
+  const room = (when) => {
+    const r = roomFor(freeBytes(), total, `r2: the ${SUITES.length} suites' projected logs, ${when}`);
+    console.log(r.line);
+    if (!r.ok) throw new Error(r.line);
+    return r.line;
+  };
+  const before = room("before the builds");
+  const bin = join(GATE, "bin", `r2-${commit.slice(0, 12)}`);
   mkdirSync(bin, { recursive: true });
   const exe = {
     lab: build(["-p", "npt-lab"], "npt-lab", join(bin, "npt-lab")),
     bites: build(["-p", "npt-lab", "--features", "bites"], "npt-lab", join(bin, "npt-lab-bites")),
   };
   const judge = build(["-p", "npt-judge"], "npt-judge", join(bin, "npt-judge"));
+  const after = room("after the builds");
+  mkdirSync(out, { recursive: true });
+  const copies = {};
+  for (const f of ["marks.json", "SERIES.md"]) {
+    copies[f] = join(out, f);
+    copyFileSync(join(SPIKE, "marks", f), copies[f]);
+  }
   const lines = [
     `# U4r's r2, re-made from commit ${commit} on ${date} (the advisor's U4r build ruling 3, point 6 (4))`,
     `npt-lab (measured build) sha256 ${sha256(exe.lab)}`,
     `npt-lab (bites build) sha256 ${sha256(exe.bites)}`,
     `npt-judge sha256 ${sha256(judge)}`,
+    `marks.json sha256 ${sha256(copies["marks.json"])}, SERIES.md sha256 ${sha256(copies["SERIES.md"])} (copied into the output folder at the start; contract and the judge read the copies)`,
+    `projection: ${before}`,
+    `projection: ${after}`,
     "",
   ];
   for (const s of SUITES) {
-    floor(`suite ${s.key}`);
+    floor(`suite ${s.key}`, s.bytes);
     const logs = join(out, s.key);
     const said = [
       `## suite ${s.key} (the ${s.build === "lab" ? "measured" : "bites"} build); expected: ${s.expect}`,
       ...step(exe[s.build], ["scenario", "--run", s.run, "--set", "u4r-r2", "--date", date, "--commit", commit, "--out", logs, ...s.args.split(" "), "--marks", s.marks, "--suite", s.suite]),
-      ...step(exe[s.build], ["contract", "--logs", logs, "--series", "marks/SERIES.md"]),
-      ...step(judge, ["--marks", "marks/marks.json", "--logs", logs, "--only", s.marks, "--out", join(out, `${s.key}-verdict.json`)]),
+      ...step(exe[s.build], ["contract", "--logs", logs, "--series", copies["SERIES.md"]]),
+      ...step(judge, ["--marks", copies["marks.json"], "--logs", logs, "--only", s.marks, "--out", join(out, `${s.key}-verdict.json`)]),
       "",
     ];
     console.log(said.join("\n"));
     lines.push(...said);
   }
+  if (sh("git", ["rev-parse", "HEAD"]) !== start.commit || tracked() !== start.status) {
+    throw new Error("HEAD or the tracked files changed since the set started: stopped before r2.txt is written (FP-5 (e))");
+  }
   writeFileSync(join(out, "r2.txt"), `${lines.join("\n")}\n`);
   console.log(`written: ${join(".gate", "netcode-proof-test", `r2-${commit.slice(0, 12)}`, "r2.txt")}`);
 }
 
-try {
-  main(process.argv.slice(2));
-} catch (e) {
-  console.error(`c-u4r-r2: ${e.message}`);
-  process.exit(1);
+if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
+  try {
+    main(process.argv.slice(2));
+  } catch (e) {
+    console.error(`c-u4r-r2: ${e.message}`);
+    process.exit(1);
+  }
 }
````

### spikes/netcode-proof/ops/runs/c-u4r-runs.mjs
_`git diff --no-index .gate/u4r-snap1/spikes/netcode-proof/ops/runs/c-u4r-runs.mjs .gate/u4r-snap2/spikes/netcode-proof/ops/runs/c-u4r-runs.mjs`:  1 file changed, 233 insertions(+), 58 deletions(-)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/ops/runs/c-u4r-runs.mjs b/.gate/u4r-snap2/spikes/netcode-proof/ops/runs/c-u4r-runs.mjs
index ca29c1a..3178e9b 100644
--- a/.gate/u4r-snap1/spikes/netcode-proof/ops/runs/c-u4r-runs.mjs
+++ b/.gate/u4r-snap2/spikes/netcode-proof/ops/runs/c-u4r-runs.mjs
@@ -3,20 +3,27 @@
 // model). One script for every set, the set named first:
 //   node spikes/netcode-proof/ops/runs/c-u4r-runs.mjs precheck --date YYYY-MM-DD
 //   node spikes/netcode-proof/ops/runs/c-u4r-runs.mjs judged --profile P0|P2|P3|B1 --date YYYY-MM-DD
+// - How it is run (advisor-u4r-r1, FP-5 (d): INF-U4R-8): in the main folder, with CARGO_TARGET_DIR unset (plan §7.1
+//   item 6: the judged runs build in the main folder's own cache), not through main-run.sh; it refuses a set
+//   CARGO_TARGET_DIR, since a shared-target run comes to the advisor first (u4r-brief.md section 12 item 8). The main
+//   session's notes say whether it held the lane lock (`flock -x` on .gate/shared-target/.lane.lock) for the set.
 // - precheck (set u4r-precheck-30; 7.2): SS P0 seed 4's recording made again by `npt-lab scenario --record` at its
 //   crates/lab/precheck-recordings.json entry's settings (the set stops if that run's rewinds refused are above 0: 7.1),
 //   then three pairs by `npt-lab precheck`, each checking the file against that entry first: clean (the measured
 //   build), PM-0001-30/wallclock and PM-0001-30/late-command (the bites build), each bite on the L1 leg. runs.txt holds
 //   every pair's lines whole (both legs' records, the comparer's output and the result) and no verdict: the judge
-//   never reads these runs.
+//   never reads these runs. The late-command pair is as expected only when its whole result line is
+//   `result: VOID (L1 late_count 1)` (precheck's own test asserts that line: FP-5 (g)).
 // - judged (set u4r-<profile>; 7.1): the profile's three measured runs (seeds 1 to 3; SS, 48 clients, L1, a 10 s
 //   warm-up, 600 s measured, tick_start 63,736; B1 with 10 bursts, every 60 s from 30 s into the measured phase), their
 //   headers listing PM-0001-2 (every L1 part's pre:PM-0001-2), PM-0001-17 and PM-0001-57 (-57's parts are on -17's
 //   runs), and PM-0001-18 on B1; then, in the bites build at seed 1 with the same settings, one run per bite of those
-//   marks (marks.json's lists, read at run time) whose part has this profile: -17's and -57's on P0, each header
+//   marks (marks.json's lists, read from the set's copy) whose part has this profile: -17's and -57's on P0, each header
 //   listing PM-0001-2 and the bite's own mark; then `npt-judge --only` on the set's folder. One profile per call keeps
 //   each set's logs and the judge's memory to one profile's (the advisor's U4r build ruling, point 3: the main session
-//   plans the disk and judges each set's folder apart).
+//   plans the disk and judges each set's folder apart). A run whose summary line counts an overflow resync above 0 and
+//   whose header lists -17 or -18 is marked on its runs.txt line: it goes to the advisor before it counts (carry 58;
+//   FP-5 (g)).
 // - The tick16 bite run (the advisor's U4r build ruling 3, point 3): its tick_start is computed from its own lengths,
 //   65,551 - (warm-up ticks + measured ticks) (47,251 for 300 + 18,000), so the 16-bit wrap falls 15 ticks before the
 //   measured phase's end and the measured window ends before the bitten room evicts; every other -17 run, clean and the
@@ -25,23 +32,37 @@
 //   moves to U4g with the room's fix; ws-budget-x2 is L2's. The three B1 measured runs stay in this set for -57's L1 B1
 //   part, which has no precondition; their -17 and -18 verdicts are recorded INVALID (pre:PM-0001-2's everyone-in-view
 //   share, npt-room's recovery after each burst) and never counted.
-// - It refuses uncommitted changes to tracked files (every log names its commit); raw logs go under
-//   .gate/netcode-proof-test/runs/<set>-<commit12>/ (one folder per run), results under results/u4r/<commit12>/<profile>/
-//   or results/u4r-precheck-30/<commit12>/; neither is ever overwritten. It stops before a run when less than 4 GB is
-//   free (u4r-brief.md section 4 rule 6's floor).
-// - Both npt-lab builds and npt-judge are built --release --locked, each artifact's path read from cargo's
+// - Its order (FP-5 (a): INF-U4R-2): the usage and git checks (no uncommitted change to a tracked file, and no untracked
+//   file under spikes/netcode-proof/crates: every log names its commit); the CARGO_TARGET_DIR refusal; the names (the
+//   pre-check entry's name and each bite id's tail, each one or more of A-Z, a-z, 0-9 and -, before any use: FP-5 (h));
+//   the projection; each release build, with at least 4 GiB free before each (u4r-brief.md section 4 rule 6); the
+//   projection again; only then the set's folders. The binaries go to .gate/netcode-proof-test/bin/<set>-<commit12>/,
+//   which no exists-check reads, so a stop before the first run leaves nothing that blocks a re-run at that commit.
+// - The projection (FP-5 (b), (c): INF-U4R-3): each run's log bytes from the builder's measured rates (BYTES_10MIN_57,
+//   BYTES_10MIN_OTHER, PAIR_BYTES), the set's total checked before the builds and after them: it starts only when free
+//   space minus the total is at least 4 GiB, printing both; before each run, free space minus that run's projected
+//   bytes must be at least 4 GiB, else it stops before that run.
+// - Raw logs go under .gate/netcode-proof-test/runs/<set>-<commit12>/ (one folder per run), results under
+//   results/u4r/<commit12>/<profile>/ or results/u4r-precheck-30/<commit12>/; neither is ever overwritten. Right after
+//   the raw-log folder is made, marks/marks.json is copied into it (its sha256 in runs.txt's head); the bite list is read
+//   from that copy and the judge is given it. Before the judge, HEAD and the tracked files' status are checked against
+//   their start values, and the set stops if either changed (FP-5 (e): INF-U4R-9). runs.txt's head names the target
+//   (the main folder's own cache) and the uptime and load average at the start and at the end.
+// - Both npt-lab builds and npt-judge are built --release --locked --offline, each artifact's path read from cargo's
 //   --message-format=json messages (check.mjs's builtArtifact, as the wasm step reads its builds) and copied out before
-//   the next build replaces it: so it is right under the main folder's own cache (CARGO_TARGET_DIR unset, as plan §7.1
-//   item 6 has judged runs) and under the shared target alike.
+//   the next build replaces it.
 // - Each npt-lab process is timed (the wall time around the call) and its seconds go on its runs.txt line beside its
 //   ticks (npt-lab's summary line), for the ticks-per-second figure (2.4 r5, 7.3).
 // - No run logs its clients' bytes (`scenario --bytes`): no part these sets judge reads a replay log (only an x86_64
 //   replay log of an L2 Arm run is read: judge/src/figures.rs:222-233).
+// - Its pure helpers are exported (the projection, the name check, the summary line's flag, the pairs' check); main
+//   runs only when this file is the entry module.
 
 import { spawnSync } from "node:child_process";
 import { createHash } from "node:crypto";
 import { copyFileSync, existsSync, mkdirSync, readFileSync, statfsSync, writeFileSync } from "node:fs";
-import { dirname, join } from "node:path";
+import { loadavg, uptime } from "node:os";
+import { dirname, join, resolve } from "node:path";
 import { fileURLToPath } from "node:url";
 import { builtArtifact } from "../../check.mjs";
 
@@ -67,10 +88,116 @@ const TICK16_START = 65_536 + 15 - (WARMUP_S + MEASURED_S) * TICK_HZ;
 const L2_ONLY = ["PM-0001-18/ws-budget-x2"];
 /** The bites whose judged run is not U4r's (the advisor's U4r build ruling 3, point 1: -18's no-fallback, to U4g). */
 const NOT_AT_U4R = ["PM-0001-18/no-fallback"];
-/** The free space below which no run starts, bytes (section 4 rule 6's 4 GB floor). */
-const FLOOR = 4 * 1024 ** 3;
+/** The free space a build and a run must leave, bytes (section 4 rule 6's 4 GB floor, as 4 GiB). */
+export const FLOOR = 4 * 1024 ** 3;
 /** precheck-recordings.json's scene ids (Scene::by_id). */
 const SCENES = { 1: "SS", 2: "ES", 3: "CO", 4: "TC" };
+/** A name that reaches a file path or a run id (FP-5 (h): SEC-U4R-7): one or more of A-Z, a-z, 0-9 and -. */
+const NAME = /^[A-Za-z0-9-]+$/;
+/**
+ * One judged run's projected log bytes over a 10-minute measured phase (18,300 ticks with its warm-up and after phase),
+ * from the builder's measured bytes per measured tick (u4r-build.md:739-751; MB = 10^6 B): a run whose header lists
+ * PM-0001-57 writes -57's three series (the advisor's U4r build ruling, point 3), P0 509.1 MB, P2 219.8 MB, P3 125.8
+ * MB, B1 190.6 MB; any other run about 3,050 B a measured tick at every profile, 56 MB.
+ */
+export const BYTES_10MIN_57 = { P0: 509_100_000, P2: 219_800_000, P3: 125_800_000, B1: 190_600_000 };
+export const BYTES_10MIN_OTHER = 56_000_000;
+/**
+ * One -30 pre-check pair's output (its lines, kept for runs.txt), at most 25,164 B in the builder's pre-check runs
+ * (.gate/u4r/c4-b12-3.log:18-365: the clean, wallclock and late-command pairs, 24,840, 25,164 and 25,093 B); the
+ * recording itself is its manifest entry's bytes.
+ */
+export const PAIR_BYTES = 25_164;
+/** The late-command pair's one expected result line (precheck.rs, the late-command bite's test). */
+export const LATE_COMMAND_LINE = "result: VOID (L1 late_count 1)";
+
+/** Whether `s` is a name (NAME). */
+export const nameOk = (s) => typeof s === "string" && NAME.test(s);
+
+/** Each [what, value] whose value is not a name, as a sentence naming both (FP-5 (h)). */
+export function nameProblems(named) {
+  return named
+    .filter(([, v]) => !nameOk(v))
+    .map(([what, v]) => `${what} ${JSON.stringify(v)} is not one or more of A-Z, a-z, 0-9 and -: refused before any use`);
+}
+
+/**
+ * One run's projected log bytes (FP-5 (b)): its profile's figure when its header lists PM-0001-57 (the largest, P0's,
+ * for a profile the table lacks), else BYTES_10MIN_OTHER, scaled by its measured seconds over 600.
+ */
+export function runBytes({ profile, marks, measuredS }) {
+  const per10 = marks.includes("PM-0001-57") ? (BYTES_10MIN_57[profile] ?? BYTES_10MIN_57.P0) : BYTES_10MIN_OTHER;
+  return Math.ceil((per10 * measuredS) / 600);
+}
+
+/** The set's projected bytes: its runs' sum. */
+export const projectedBytes = (plan) => plan.reduce((s, r) => s + r.bytes, 0);
+
+/**
+ * Whether `free` bytes leave the 4 GiB floor after `projected` more (FP-5 (b), (c)), with the line that prints both.
+ */
+export function roomFor(free, projected, what) {
+  const ok = free - projected >= FLOOR;
+  const mib = (b) => (b / 1024 ** 2).toFixed(0);
+  return {
+    ok,
+    line: `${what}: free ${mib(free)} MiB, projected ${mib(projected)} MiB, so ${mib(free - projected)} MiB after it against the 4 GiB floor (4096 MiB): ${ok ? "room" : "no room, stopped before it"}`,
+  };
+}
+
+/**
+ * The mark a run's summary line takes (FP-5 (g): NC-U4R-6): its "overflow resyncs N" above 0 on a run whose header
+ * lists PM-0001-17 or PM-0001-18 goes to the advisor before the run counts (carry 58); "" otherwise.
+ */
+export function overflowFlag(line, marks) {
+  const n = Number(line.match(/overflow resyncs (\d+)/)?.[1] ?? 0);
+  return n > 0 && marks.some((m) => m === "PM-0001-17" || m === "PM-0001-18") ? " (overflow resyncs: to the advisor before it counts, carry 58)" : "";
+}
+
+/**
+ * Whether a pair's output is as expected (FP-5 (g)): the late-command pair only when its whole result line is
+ * LATE_COMMAND_LINE; the clean and wallclock pairs by the result's first word.
+ */
+export function pairAsExpected(pair, expected, out) {
+  const line = out.match(/^result: .*$/m)?.[0] ?? "";
+  return pair === "late-command" ? line === LATE_COMMAND_LINE : line.match(/^result: (\S+)/)?.[1] === expected;
+}
+
+/** The marks a judged set's headers list (the module text). */
+const listedOf = (profile) => ["PM-0001-2", "PM-0001-17", "PM-0001-57", ...(profile === "B1" ? ["PM-0001-18"] : [])];
+
+/**
+ * The bites a judged set runs, from marks.json's lists: one run each at seed 1 on P0, the profile of a part that reads
+ * it (-17's and -57's); -18's none at U4r (ruling 3, point 1: to U4g), so a B1 set runs no bite.
+ */
+export function bitesOf(profile, marks) {
+  const listed = listedOf(profile);
+  const bitesOn = (mark) => (mark === "PM-0001-18" ? null : "P0");
+  const out = [];
+  for (const m of marks.marks.filter((x) => listed.includes(x.id) && x.id !== "PM-0001-2" && bitesOn(x.id) === profile)) {
+    for (const b of (m.bites ?? []).filter((x) => !L2_ONLY.includes(x.id) && !NOT_AT_U4R.includes(x.id))) out.push({ id: b.id, mark: m.id });
+  }
+  return out;
+}
+
+/** The judged set's runs, in order, each with its projected bytes (the module text; FP-5 (b)). */
+export function planJudged(profile, marks) {
+  const listed = listedOf(profile);
+  const runs = SEEDS.map((s) => ({ run: `l1-${profile.toLowerCase()}-s${s}`, exe: "lab", seed: s, marks: listed, extra: ["--role", "measured", "--marks", listed.join(",")], tickStart: TICK_START }));
+  for (const b of bitesOf(profile, marks)) {
+    const marksOf = ["PM-0001-2", b.mark];
+    runs.push({ run: `bite-${b.id.split("/")[1]}`, exe: "labBites", seed: 1, marks: marksOf, extra: ["--role", "bite", "--bite", b.id, "--marks", marksOf.join(",")], tickStart: b.id === TICK16 ? TICK16_START : TICK_START });
+  }
+  return runs.map((r) => ({ ...r, what: r.run, bytes: runBytes({ profile, marks: r.marks, measuredS: MEASURED_S }) }));
+}
+
+/** The pre-check set's outputs, each with its projected bytes (FP-5 (b)): the recording, then the three pairs. */
+export function planPrecheck(entry) {
+  return [
+    { what: `the recording ${entry.name}`, bytes: entry.bytes },
+    ...["clean", "wallclock", "late-command"].map((p) => ({ what: `the ${p} pair`, bytes: PAIR_BYTES })),
+  ];
+}
 
 function sh(cmd, args, cwd = ROOT) {
   const r = spawnSync(cmd, args, { cwd, encoding: "utf8", maxBuffer: 1 << 28 });
@@ -80,22 +207,28 @@ function sh(cmd, args, cwd = ROOT) {
 
 const sha256 = (path) => createHash("sha256").update(readFileSync(path)).digest("hex");
 
-/** One release build, its artifact's path read from cargo's JSON messages and copied to `to`. */
+/** The free bytes where the logs go. */
+function freeBytes() {
+  const s = statfsSync(ROOT);
+  return s.bavail * s.bsize;
+}
+
+/** Stops unless free space minus `projected` bytes leaves the floor (0: a build's own check, 4 GiB free). */
+function floor(what, projected = 0) {
+  const r = roomFor(freeBytes(), projected, what);
+  if (!r.ok) throw new Error(r.line);
+}
+
+/** One release build, at least 4 GiB free before it, its artifact's path read from cargo's JSON messages and copied to `to`. */
 function build(args, target, to) {
-  const out = sh("cargo", ["build", "--release", "--locked", ...args, "--message-format=json"], SPIKE);
+  floor(`the release build of ${target}${args.includes("bites") ? " (bites)" : ""}`);
+  const out = sh("cargo", ["build", "--release", "--locked", "--offline", ...args, "--message-format=json"], SPIKE);
   const path = builtArtifact(out, { target, kind: "bin" });
   if (!path) throw new Error(`cargo build ${args.join(" ")} names no ${target} executable in its JSON messages`);
   copyFileSync(path, to);
   return to;
 }
 
-/** Stops before a run when less than FLOOR bytes are free where the logs go. */
-function floor(what) {
-  const s = statfsSync(ROOT);
-  const free = s.bavail * s.bsize;
-  if (free < FLOOR) throw new Error(`${what}: ${(free / 1024 ** 3).toFixed(1)} GB free, below the 4 GB floor: stopped before it`);
-}
-
 /** One npt-lab process, timed: its summary line with the wall seconds. */
 function timed(exe, args) {
   const t0 = Date.now();
@@ -104,6 +237,9 @@ function timed(exe, args) {
   return { out, secs };
 }
 
+/** The machine's uptime and load average now (runs.txt's head: FP-5 (d)). */
+const machineNow = () => `uptime ${uptime().toFixed(0)} s, load average ${loadavg().map((x) => x.toFixed(2)).join(" ")}`;
+
 function main(argv) {
   const set = argv[0];
   const opt = (k) => (argv.includes(k) ? argv[argv.indexOf(k) + 1] : undefined);
@@ -112,15 +248,43 @@ function main(argv) {
   const usage = "usage: c-u4r-runs.mjs precheck --date YYYY-MM-DD | judged --profile P0|P2|P3|B1 --date YYYY-MM-DD";
   if (!/^\d{4}-\d{2}-\d{2}$/.test(date ?? "") || !["precheck", "judged"].includes(set)) throw new Error(usage);
   if (set === "judged" && !PROFILES.includes(profile)) throw new Error(usage);
-  if (sh("git", ["status", "--porcelain", "--untracked-files=no"])) {
-    throw new Error("tracked files have uncommitted changes: commit first, since every log names its commit");
+  const tracked = () => sh("git", ["status", "--porcelain", "--untracked-files=no"]);
+  if (tracked()) throw new Error("tracked files have uncommitted changes: commit first, since every log names its commit");
+  const untracked = sh("git", ["status", "--porcelain", "--untracked-files=all", "--", "crates"], SPIKE);
+  if (untracked) throw new Error(`files under spikes/netcode-proof/crates are not committed (${untracked.split("\n").join("; ")}): commit first, since every log names its commit`);
+  if (process.env.CARGO_TARGET_DIR !== undefined) {
+    throw new Error(
+      `CARGO_TARGET_DIR is set (${JSON.stringify(process.env.CARGO_TARGET_DIR)}): this script runs in the main folder with its own cache (plan §7.1 item 6); a shared-target run comes to the advisor first (u4r-brief.md section 12 item 8)`,
+    );
   }
   const commit = sh("git", ["rev-parse", "HEAD"]);
+  const start = { commit, status: tracked(), machine: machineNow() };
   const short = commit.slice(0, 12);
   const name = set === "precheck" ? "u4r-precheck-30" : `u4r-${profile}`;
   const logs = join(GATE, "runs", `${name}-${short}`);
   const results = set === "precheck" ? join(SPIKE, "results", "u4r-precheck-30", short) : join(SPIKE, "results", "u4r", short, profile);
   for (const d of [logs, results]) if (existsSync(d)) throw new Error(`${d} exists: a run set is never overwritten`);
+  const marksText = readFileSync(join(SPIKE, "marks", "marks.json"), "utf8");
+  const marksSha = createHash("sha256").update(marksText).digest("hex");
+  const manifest = join(SPIKE, "crates", "lab", "precheck-recordings.json");
+  const entry = set === "precheck" ? JSON.parse(readFileSync(manifest, "utf8")).recordings[0] : null;
+
+  // The names, before any use (FP-5 (h)).
+  const named = set === "precheck" ? [["the pre-check entry's name", entry?.name]] : bitesOf(profile, JSON.parse(marksText)).map((b) => [`bite ${JSON.stringify(b.id)}'s tail`, b.id.split("/")[1]]);
+  const bad = nameProblems(named);
+  if (bad.length) throw new Error(bad.join("; "));
+
+  // The projection, before the builds and after them (FP-5 (b)).
+  const planOf = (marks) => (set === "precheck" ? planPrecheck(entry) : planJudged(profile, marks));
+  const plan = planOf(JSON.parse(marksText));
+  const total = projectedBytes(plan);
+  const room = (when) => {
+    const r = roomFor(freeBytes(), total, `${name}: the set's ${plan.length} runs' projected logs, ${when}`);
+    console.log(r.line);
+    if (!r.ok) throw new Error(r.line);
+    return r.line;
+  };
+  const before = room("before the builds");
 
   // Both builds of npt-lab, each copied out before the next build replaces it, and the judge.
   const bin = join(GATE, "bin", `${name}-${short}`);
@@ -128,78 +292,87 @@ function main(argv) {
   const lab = build(["-p", "npt-lab"], "npt-lab", join(bin, "npt-lab"));
   const labBites = build(["-p", "npt-lab", "--features", "bites"], "npt-lab", join(bin, "npt-lab-bites"));
   const judge = set === "judged" ? build(["-p", "npt-judge"], "npt-judge", join(bin, "npt-judge")) : null;
+  const after = room("after the builds");
+
+  // The set's folder, and its copy of marks.json, which the bite list and the judge read (FP-5 (e)).
   mkdirSync(logs, { recursive: true });
+  const marksCopy = join(logs, "marks.json");
+  copyFileSync(join(SPIKE, "marks", "marks.json"), marksCopy);
+  if (sha256(marksCopy) !== marksSha) throw new Error("marks/marks.json changed since the set started: stopped before the first run");
+  const runs = planOf(JSON.parse(readFileSync(marksCopy, "utf8")));
   const head = [
     `commit ${commit}`,
+    "target: the main folder's own cache (CARGO_TARGET_DIR unset: plan §7.1 item 6)",
     `npt-lab (measured build) sha256 ${sha256(lab)}`,
     `npt-lab (bites build) sha256 ${sha256(labBites)}`,
     ...(judge ? [`npt-judge sha256 ${sha256(judge)}`] : []),
+    `marks.json sha256 ${marksSha} (copied into the raw-log folder at the set's start; the bite list and the judge read that copy)`,
     `raw logs: .gate/netcode-proof-test/runs/${name}-${short}/ (not committed: plan §7.2)`,
+    `projection: ${before}`,
+    `projection: ${after}`,
+    `start: ${start.machine}`,
   ];
   const common = ["--set", name, "--date", date, "--commit", commit];
   const lines = [];
+  const unchanged = (what) => {
+    if (sh("git", ["rev-parse", "HEAD"]) !== start.commit || tracked() !== start.status) {
+      throw new Error(`HEAD or the tracked files changed since the set started: stopped before ${what} (FP-5 (e))`);
+    }
+  };
 
   if (set === "precheck") {
-    const manifest = join(SPIKE, "crates", "lab", "precheck-recordings.json");
-    const [e] = JSON.parse(readFileSync(manifest, "utf8")).recordings;
-    const rec = join(logs, `${e.name}.rec`);
-    floor(`the recording ${e.name}`);
+    const rec = join(logs, `${entry.name}.rec`);
+    floor(runs[0].what, runs[0].bytes);
     const made = timed(lab, [
-      "scenario", "--run", `record-${e.name}`, ...common, "--out", join(logs, `record-${e.name}`), "--scene", SCENES[e.scene],
-      "--profile", e.profile, "--seed", String(e.seed), "--clients", String(e.clients), "--tick-start", String(e.first_tick),
-      "--warmup-s", String(e.warmup_s), "--measured-s", String(e.measured_s), "--logs", "0", "--suite", "record", "--record", rec,
+      "scenario", "--run", `record-${entry.name}`, ...common, "--out", join(logs, `record-${entry.name}`), "--scene", SCENES[entry.scene],
+      "--profile", entry.profile, "--seed", String(entry.seed), "--clients", String(entry.clients), "--tick-start", String(entry.first_tick),
+      "--warmup-s", String(entry.warmup_s), "--measured-s", String(entry.measured_s), "--logs", "0", "--suite", "record", "--record", rec,
     ]);
     console.log(made.out);
-    lines.push(`# the recording (npt-lab scenario --record at the manifest entry's settings; wall ${made.secs} s)`, made.out, `${e.name} sha256 ${sha256(rec)}`);
+    lines.push(`# the recording (npt-lab scenario --record at the manifest entry's settings; wall ${made.secs} s)`, made.out, `${entry.name} sha256 ${sha256(rec)}`);
     const refused = Number(made.out.match(/rewinds refused (\d+)/)?.[1] ?? Number.NaN);
-    if (refused !== 0) throw new Error(`${e.name}'s run refused ${refused} rewinds: the set stops before the pre-check (7.1; carry 40)`);
+    if (refused !== 0) throw new Error(`${entry.name}'s run refused ${refused} rewinds: the set stops before the pre-check (7.1; carry 40)`);
     const pairs = [
       { pair: "clean", exe: lab, extra: [], expected: "PASS" },
       { pair: "wallclock", exe: labBites, extra: ["--bite", "PM-0001-30/wallclock", "--bite-leg", "L1"], expected: "FAIL" },
       { pair: "late-command", exe: labBites, extra: ["--bite", "PM-0001-30/late-command", "--bite-leg", "L1"], expected: "VOID" },
     ];
-    for (const p of pairs) {
-      floor(`the ${p.pair} pair`);
+    for (const [i, p] of pairs.entries()) {
+      floor(runs[i + 1].what, runs[i + 1].bytes);
       const r = timed(p.exe, ["precheck", "--manifest", manifest, "--recording", rec, "--commit", commit, ...p.extra]);
       const result = r.out.match(/^result: (\S+)/m)?.[1] ?? "none";
-      const said = `## pair ${p.pair} (wall ${r.secs} s): result ${result}, expected ${p.expected}${result === p.expected ? "" : " (NOT as expected: to the advisor)"}`;
+      const expected = p.pair === "late-command" ? `${p.expected}, the whole line "${LATE_COMMAND_LINE}"` : p.expected;
+      const said = `## pair ${p.pair} (wall ${r.secs} s): result ${result}, expected ${expected}${pairAsExpected(p.pair, p.expected, r.out) ? ", as expected" : " (NOT as expected: to the advisor)"}`;
       console.log(said);
       lines.push(said, r.out);
     }
+    unchanged("runs.txt is written");
     mkdirSync(results, { recursive: true });
     const top = [`# u4r-precheck-30: the -30 pre-check (u4r-brief.md 7.2), run in this container on ${date}; no verdict (no judged id)`];
-    writeFileSync(join(results, "runs.txt"), `${[...top, ...head, "", ...lines].join("\n")}\n`);
+    writeFileSync(join(results, "runs.txt"), `${[...top, ...head, `end: ${machineNow()}`, "", ...lines].join("\n")}\n`);
     console.log(`written: ${join("spikes", "netcode-proof", "results", "u4r-precheck-30", short)}/runs.txt`);
     return;
   }
 
-  const marks = JSON.parse(readFileSync(join(SPIKE, "marks", "marks.json"), "utf8"));
-  const listed = ["PM-0001-2", "PM-0001-17", "PM-0001-57", ...(profile === "B1" ? ["PM-0001-18"] : [])];
   const settings = (tickStart) => ["--scene", "SS", "--profile", profile, "--tick-start", String(tickStart), "--warmup-s", String(WARMUP_S), "--measured-s", String(MEASURED_S), "--suite", name];
-  const one = (exe, run, seed, extra, tickStart = TICK_START) => {
-    floor(run);
-    const r = timed(exe, ["scenario", "--run", run, ...common, "--out", join(logs, run), "--seed", String(seed), ...settings(tickStart), ...extra]);
-    const said = `${r.out} (wall ${r.secs} s)`;
+  for (const r of runs) {
+    floor(r.what, r.bytes);
+    const exe = r.exe === "lab" ? lab : labBites;
+    const t = timed(exe, ["scenario", "--run", r.run, ...common, "--out", join(logs, r.run), "--seed", String(r.seed), ...settings(r.tickStart), ...r.extra]);
+    const said = `${t.out} (wall ${t.secs} s)${overflowFlag(t.out, r.marks)}`;
     console.log(said);
     lines.push(said);
-  };
-  for (const s of SEEDS) one(lab, `l1-${profile.toLowerCase()}-s${s}`, s, ["--role", "measured", "--marks", listed.join(",")]);
-  // The bites: one run each at seed 1 on P0, the profile of a part that reads it (-17's and -57's); -18's none at U4r
-  // (ruling 3, point 1: to U4g), so a B1 set runs no bite. The tick16 run at TICK16_START (ruling 3, point 3).
-  const bitesOn = (mark) => (mark === "PM-0001-18" ? null : "P0");
-  for (const m of marks.marks.filter((x) => listed.includes(x.id) && x.id !== "PM-0001-2" && bitesOn(x.id) === profile)) {
-    for (const b of (m.bites ?? []).filter((x) => !L2_ONLY.includes(x.id) && !NOT_AT_U4R.includes(x.id))) {
-      one(labBites, `bite-${b.id.split("/")[1]}`, 1, ["--role", "bite", "--bite", b.id, "--marks", `PM-0001-2,${m.id}`], b.id === TICK16 ? TICK16_START : TICK_START);
-    }
   }
+  unchanged("the judge");
   const verdict = join(logs, "verdict.json");
-  const judged = sh(judge, ["--marks", join(SPIKE, "marks", "marks.json"), "--logs", logs, "--only", listed.join(","), "--out", verdict]);
+  const judged = sh(judge, ["--marks", marksCopy, "--logs", logs, "--only", listedOf(profile).join(","), "--out", verdict]);
   console.log(judged);
   mkdirSync(results, { recursive: true });
   copyFileSync(verdict, join(results, "verdict.json"));
   const top = [
     `# ${name}: -17's, -57's${profile === "B1" ? " and -18's" : ""} L1 ${profile} part(s), run in this container on ${date} (u4r-brief.md 7.1)`,
     ...head,
+    `end: ${machineNow()}`,
     `judge: ${judged}`,
     "",
     "# one line per run (npt-lab's own summary line and its wall time; the judge reads the logs, never these lines)",
@@ -208,9 +381,11 @@ function main(argv) {
   console.log(`written: ${join("spikes", "netcode-proof", "results", "u4r", short, profile)}/verdict.json and runs.txt`);
 }
 
-try {
-  main(process.argv.slice(2));
-} catch (e) {
-  console.error(`c-u4r-runs: ${e.message}`);
-  process.exit(1);
+if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
+  try {
+    main(process.argv.slice(2));
+  } catch (e) {
+    console.error(`c-u4r-runs: ${e.message}`);
+    process.exit(1);
+  }
 }
````

### spikes/netcode-proof/ops/runs/det-u4.mjs
_`git diff --no-index .gate/u4r-snap1/spikes/netcode-proof/ops/runs/det-u4.mjs .gate/u4r-snap2/spikes/netcode-proof/ops/runs/det-u4.mjs`:  1 file changed, 8 insertions(+), 5 deletions(-)._

````diff
diff --git a/.gate/u4r-snap1/spikes/netcode-proof/ops/runs/det-u4.mjs b/.gate/u4r-snap2/spikes/netcode-proof/ops/runs/det-u4.mjs
index a0ba0f4..450f52f 100644
--- a/.gate/u4r-snap1/spikes/netcode-proof/ops/runs/det-u4.mjs
+++ b/.gate/u4r-snap2/spikes/netcode-proof/ops/runs/det-u4.mjs
@@ -10,15 +10,16 @@
 // - The step's rules hold here as they are wasmStep's own: `npt-lab hash` and its compare must exit 0 (carry 83 (b));
 //   every call but the cargo builds shares one 300 s deadline from the first of them, and a call stopped by it takes
 //   every process it started (carry 84).
-// - It prints as check.mjs's main prints a step (PASS or FAIL, the step's seconds and its summary: each pair, h2 and
-//   each call's seconds; on a FAIL its last 40 lines and the file under .gate/ that keeps its whole output) and exits 0
-//   only on PASS. A FAIL is never re-run into a PASS (u4r-brief.md section 4's no-retry rule for the wasm step).
+// - It runs check.mjs's `deps` step first, then the clean half (advisor-u4r-r1, FP-6 item 10: INF-U4R-5). It prints as
+//   check.mjs's main prints its steps (PASS or FAIL, each step's seconds and its summary: for the clean half each pair,
+//   h2 and each call's seconds; on a FAIL its last 40 lines and the file under .gate/ that keeps its whole output) and
+//   exits 0 only when both pass. A FAIL is never re-run into a PASS (u4r-brief.md section 4's no-retry rule for the wasm step).
 // - Run it as check.mjs is run: under the lane lock, with the session's CARGO_TARGET_DIR (the step's cargo builds read
 //   their artifacts' paths from cargo's JSON messages, so either cache serves).
 
 import { dirname, join } from "node:path";
 import { fileURLToPath } from "node:url";
-import { FAILED_STEP_DIR, main, wasmStep } from "../../check.mjs";
+import { FAILED_STEP_DIR, STEPS, main, wasmStep } from "../../check.mjs";
 
 const SPIKE = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
 const ROOT = join(SPIKE, "..", "..");
@@ -27,7 +28,9 @@ const DET_MANIFEST = "crates/lab/det-recordings.json";
 /** Its own folder under .gate/ (gitignored), emptied at each run as the step empties its own. */
 const DET_DIR = join(ROOT, ".gate", "netcode-proof-test", "det-u4");
 
-const steps = { "det-u4": () => wasmStep(undefined, undefined, undefined, DET_DIR, { manifest: DET_MANIFEST, bitesHalf: false }) };
+// check.mjs's deps step first (advisor-u4r-r1, FP-6 item 10: INF-U4R-5), so this exits 0 only when the builds pass the
+// checks check.mjs would make of them and the clean half passes.
+const steps = { deps: STEPS.deps, "det-u4": () => wasmStep(undefined, undefined, undefined, DET_DIR, { manifest: DET_MANIFEST, bitesHalf: false }) };
 try {
   process.exitCode = main(process.argv.slice(2), steps, console.log, FAILED_STEP_DIR);
 } catch (e) {
````

### FP-5's helper test (a scratch file in the builder worktree's notes folder, never committed; the ruling's FP-5 EVIDENCE)
_ 1 file changed, 72 insertions(+); the diff itself is in `.checkpoints/netcode-proof-test/u4r-r2-packet-tests.md` (`git diff --no-index /dev/null .claude/worktrees/agent-a11e2354169f39acf/.checkpoints/netcode-proof-test/u4r-fp5-helpers.test.mjs`)._

## What it is judged against

### FP-0: u4r-run.sh against its committed version
_`.checkpoints/netcode-proof-test/u4r-run.sh.inf-u4r-4.diff` lines 1-20 (sha256 3d90c33c716c), as they are:_

````text
diff --git a/.checkpoints/netcode-proof-test/u4r-run.sh b/.checkpoints/netcode-proof-test/u4r-run.sh
index b040c7f..8678438 100644
--- a/.checkpoints/netcode-proof-test/u4r-run.sh
+++ b/.checkpoints/netcode-proof-test/u4r-run.sh
@@ -60,12 +60,12 @@ while pgrep -x cargo >/dev/null || pgrep -x rustc >/dev/null; do
   fi
   sleep 1
 done
-avail=$(df -BG --output=avail /home/user | tail -1 | tr -dc 0-9)
+avail=$(df -B1 --output=avail /home/user | tail -1 | tr -dc 0-9)
 {
   echo "lock $LOCK held ($mode) at $(date -u +%T) UTC; idle after ${i} s"
-  echo "pwd $(pwd); free ${avail} GB; HEAD $(git rev-parse --short HEAD); CARGO_TARGET_DIR $CARGO_TARGET_DIR"
+  echo "pwd $(pwd); free $((avail/1048576)) MiB; HEAD $(git rev-parse --short HEAD); CARGO_TARGET_DIR $CARGO_TARGET_DIR"
 } > "$log"
-if [ "$avail" -lt 4 ]; then
+if [ "$avail" -lt $((4*1024*1024*1024)) ]; then
   echo "STOP: under 4 GB free" >> "$log"
   exit 3
 fi
````

### FP-0: main-run.sh against its committed version
_`.checkpoints/netcode-proof-test/main-run.sh.inf-u4r-4.diff` lines 1-20 (sha256 ddf70f28931b), as they are:_

````text
diff --git a/.checkpoints/netcode-proof-test/main-run.sh b/.checkpoints/netcode-proof-test/main-run.sh
index d9ce0e1..9901cf6 100644
--- a/.checkpoints/netcode-proof-test/main-run.sh
+++ b/.checkpoints/netcode-proof-test/main-run.sh
@@ -35,12 +35,12 @@ while pgrep -x cargo >/dev/null || pgrep -x rustc >/dev/null; do
   fi
   sleep 1
 done
-avail=$(df -BG --output=avail /home/user | tail -1 | tr -dc 0-9)
+avail=$(df -B1 --output=avail /home/user | tail -1 | tr -dc 0-9)
 {
   echo "lock $LOCK held exclusively at $(date -u +%T) UTC; idle after ${i} s"
-  echo "pwd $(pwd); free ${avail} GB; HEAD $(cd "$M" && git rev-parse --short HEAD)"
+  echo "pwd $(pwd); free $((avail/1048576)) MiB; HEAD $(cd "$M" && git rev-parse --short HEAD)"
 } > "$log"
-if [ "$avail" -lt 4 ]; then
+if [ "$avail" -lt $((4*1024*1024*1024)) ]; then
   echo "STOP: under 4 GB free" >> "$log"
   exit 3
 fi
````

### The advisor's ruling on round 1: the fix pass, round 2 and each finding
_`.checkpoints/netcode-proof-test/advisor-u4r-r1-full.md` lines 1-316 (sha256 7b552e8c9d6f), as they are:_

````text
# Advisor: the ruling on U4r review round 1 (workflow wf_549d69bf-7f3), 2026-11-14

Written by script, word for word, from the workflow's journal (advisor-u4r-r1's StructuredOutput result). Its notes: advisor-u4r-r1.md; the findings: u4r-review-r1-full.md. Nothing below is the main session's.

## The fix pass, in order

1. THE COPY AND ITS RULES (the main session sends this with the items below). A fresh builder copy (copy 6), in u4r-copy5-prompt.md's form with its resume part brought up to date. It starts from the reviewed snapshot: wip/u4r.patch, sha256 572c4f69..., applied on 2fbec77. Notes: u4r-build.md, 1,586 lines, plus a new section 'Fix pass, round 1'. Brief section 4's rules hold throughout. Every change is made with the Edit tool. Every run goes through the runner with -x, df first, 4 GiB before any release or wasm32 build. Each plant is restored by Edit, and the file's sha256 is checked equal to its pre-plant value. Counts are deltas from 526/603/76, and each new test is named with its file. A check.mjs test-step failure is recorded whole under TE2-5 and never retried. The Lows below ride this push, which the Mediums need anyway (A4: a Low never triggers a push). At the end the main session cuts a new snapshot by script, and round 2 reviews its diff against 572c4f69.
2. FP-0 (the main session, before copy 6 starts; INF-U4R-4; this ruling approves the runner edit). In .checkpoints/netcode-proof-test/u4r-run.sh (:63, :68) and main-run.sh (:38, :43), replace the df -BG read with `avail=$(df -B1 --output=avail /home/user | tail -1 | tr -dc 0-9)`. Stop on `[ "$avail" -lt $((4*1024*1024*1024)) ]`, and log `free $((avail/1048576)) MiB` where the line said `free ${avail} GB`. Use the Edit tool only. Keep each change as a .diff beside its runner, in INF2-4's form. Run `bash -n` on each runner; it must print nothing. Copy 6's first runner log must show the MiB line. No other runner line changes.
3. FP-1 (NC-U4R-1 = SEC-U4R-1, with EA-U4R-7's owner half). In scenario.rs Server::io, the end-of-session branch (:488-497): end a session when `x.decoder.is_closed()` or `x.link.state() == LinkState::Closed`, after the pump has drained what had arrived. Take its id and slot and set `self.sessions[i] = None`. If the slot is Some and `self.owner.get(&slot) == Some(&id)`, call `self.room.leave(slot)` and `self.owner.remove(&slot)`. Then push `(slot, "closed")`. The module text (:21-25) adds that a closed decoder or link ends the session and frees the room's slot, as Room::leave's doc ('a closed link') has it. TEST: one new test beside r8 and r9 (Loopback, lab_room), with two cases. (a) A joined session sends one control-stream frame that `Quant::default().decode(Direction::ClientToRoom, Channel::Control, ..)` refuses with an error other than WrongChannel or WrongDirection; assert that refusal first. Then call server.io and assert: `server.room.client(slot)` is None, `server.owner` holds no entry for the slot, and `server.ended` holds (slot, "closed"). Then a new endpoint joins with that slot's token and bind: assert it is admitted (owner holds the new session id) and that `ended` gains no (slot, "replaced"). (b) A joined session whose client end is dropped, then server.io: the same three asserts. PLANT: delete the `self.room.leave(slot)` call; both cases must fail by name. Restore by Edit; sha256 checked.
4. FP-2 (T-1; with SEC-U4R-4 = EA-U4R-6). In r8 (scenario.rs:1943-1979): (a) replace the room_counts compare with the whole counters struct. Before the five Inputs: `let before = server.room.client(0).map(|c| c.counters().clone());`. After them: `assert_eq!(server.room.client(0).map(|c| c.counters().clone()), before, "the first's Inputs reach no room call")`. Nothing ticks inside the loop. Remove room_counts if nothing else uses it. (b) Right after the second join, drain c1 and assert that exactly one Control-channel message is there and that it decodes (Direction::RoomToClient, Channel::Control, Quant::default()) to Message::Kick(Kick::Replaced). The module text (:21-23) adds: 'the Kick(replaced) is sent once; on L1 the old link is dropped in the same instant, so a lost first transmission is not resent (best effort)'. TWO PLANTS, each restored by Edit with its sha256 checked. (i) In drop_slot, keep `self.ended.push((slot, why))` but skip `*s = None`: r8 must fail at 'the first's Inputs reach no room call'. (ii) Delete the Kick(replaced) send (:409-414): r8 must fail at the Kick assert.
5. FP-3 (RR3-M1 = SEC-U4R-2; with SEC-U4R-3, RR3-L1 and RR3-L3). (a) In hash.rs manifest(), before anything is generated, add these refusals, each naming the entry and the value. A runner entry whose profile is not one of P0, P1, P2, P3, P4 or B1. A runner entry unless `warmup_s.checked_add(measured_s).and_then(|s| s.checked_mul(u64::from(npt_sim::TICK_HZ))) == Some(u64::from(ticks))`. A runner entry whose span_ms is not scenario's ROOM_HISTORY_MS (2,000). Any entry whose bytes is above 67,108,864 (64 MiB: four times today's largest entry, 16,950,444 B); a larger recording comes to the advisor. (b) In scenario.rs record(), build argv as a Vec<String> of separate items ("--run", "regenerate", ..., "--profile", profile, "--seed", seed, ...). Never use format! followed by split_whitespace. Its doc says the argv holds only these flags, so ruling 1 point 9 (a)'s refusal of --lead and --every-step holds on this path by construction. (c) In scenario.rs parse(): use checked_mul for the three `* hz` products (warm-up, measured, after-max) and checked_add for warm-up + measured and for + after-max. When bursts > 0, compute the last burst's tick with checked operations before burst_starts is called: warmup_ticks + (burst_first_s + (bursts - 1) x burst_every_s) x hz. Each overflow is a usage error naming its option. TESTS: hash.rs unit cases beside carry 82's: profile "P2 --record x" refused, naming the entry; ticks 8,999 with 10 + 290 refused; span_ms 1,000 refused; bytes 67,108,865 refused; the three committed manifests (recordings.json, det-recordings.json, precheck-recordings.json) still read. One parse case: --measured-s 1000000000000000000 refused as a usage error naming --measured-s. PLANT: remove the profile check; the "P2 --record x" case must fail by name. Restore by Edit; sha256 checked.
6. FP-4 (NC-U4R-3, with T-6's verdict half). In precheck.rs verdict (:486-504), add two VOID causes. First, a leg's refused_logged above 0, beside rewinds_refused: '<leg> refused Rewinds logged N'. Second, a non-empty `ended` on a leg: '<leg> sessions ended on the leg [..]: their counters cannot be read'. A client whose session ends on a leg can then never read as 0 late. Update verdict's doc. TEST: one unit test of verdict, with LegOutcome::default() doctored once per cause (late, rewinds_refused, refused_logged alone, off_stamp, rtt_void, ended); each gives VOID and names its cause. Also HashCompare::Empty gives VOID, Equal gives PASS and Mismatch gives FAIL. PLANT: remove the `ended` cause; its case must fail by name. Restore by Edit; sha256 checked.
7. FP-5 (the run scripts: INF-U4R-2 and INF-U4R-3, with INF-U4R-8, INF-U4R-9, INF-U4R-11, NC-U4R-6, SEC-U4R-7 and T-7), in ops/runs/c-u4r-runs.mjs and c-u4r-r2.mjs. (a) ORDER, in both scripts: the usage check and the git checks; refuse an untracked file under spikes/netcode-proof/crates (`git status --porcelain --untracked-files=all -- crates`); then (d)'s refusal (c-u4r-runs.mjs only); then (h)'s name checks; then (b)'s up-front projection. Next, each release build, with floor() (at least 4 GiB free: section 4 rule 6) before each one. Then (b)'s projection again. Only then create any set folder (logs, results, out). Binaries go to a folder that no exists-check reads; in c-u4r-r2.mjs, move bin out of `out`, for example to GATE/bin/r2-<commit12>. A stop before the first run must leave nothing that blocks a re-run at that commit. (b) PROJECTION: a constant table citing u4r-build.md:739-748. Per 10-minute run listing PM-0001-57: P0 509.1 MB, P2 219.8 MB, P3 125.8 MB, B1 190.6 MB. Any other run: 56 MB. Scale each by the run's measured seconds over 600. For the precheck set, use the recording's manifest bytes plus the pairs' output, as measured in your own earlier pre-check runs (cite the log). For c-u4r-r2.mjs, use your own 13-suite figure (about 145 MB) plus the new suite at the p0 suite's size. Refuse to start unless free minus the projected total is at least 4 GiB, and print both figures. (c) PER RUN: the floor before each run becomes 'free minus that run's projected bytes is at least 4 GiB'. (d) c-u4r-runs.mjs: the header says it runs in the main folder with CARGO_TARGET_DIR unset (plan §7.1 item 6), not through main-run.sh, and that the main session's notes say whether it held the lane lock (flock -x on .gate/shared-target/.lane.lock) for the set. It refuses a set CARGO_TARGET_DIR: a shared-target run comes to the advisor first (brief section 12 item 8). Its cargo builds get --offline. runs.txt's head gains 'target: the main folder's own cache' and the uptime and load average at start and at end (os.uptime(), os.loadavg()). c-u4r-r2.mjs also gets --offline; it keeps running under the runner (no CARGO_TARGET_DIR refusal). (e) Right after the set folder is created, copy marks/marks.json (plus marks/SERIES.md for c-u4r-r2.mjs) into it and write their sha256s in the runs.txt or r2.txt head. Read the bite list from that copy, and pass the copies to the judge and to contract. Before the judge (c-u4r-runs.mjs) and before writing r2.txt (c-u4r-r2.mjs), re-check `git rev-parse HEAD` and `git status --porcelain --untracked-files=no` against their start values, and stop if either changed. (f) c-u4r-r2.mjs step(): run each command through check.mjs's exported run() with a 600 s limit, so it gets its own process group, killed whole. A stopped command's exit line reads 'stopped at <s> s'. A stop is a result, never a retry (condition 5). (g) In c-u4r-runs.mjs one(), parse /overflow resyncs (\d+)/. When the figure is above 0 on a run listing PM-0001-17 or PM-0001-18, append ' (overflow resyncs: to the advisor before it counts, carry 58)'. For the late-command pair, print 'as expected' only when the whole result line equals 'result: VOID (L1 late_count 1)'. (h) Before any use, refuse the precheck entry's name unless it matches /^[A-Za-z0-9-]+$/, and each bite id's tail unless it matches /^[A-Za-z0-9-]+$/, each by name. (i) T-7: c-u4r-r2.mjs gains a whole-as-lost suite: the bites build, the p0 suite's SS P0 settings, --role bite --bite PM-0001-17/whole-as-lost, --marks PM-0001-17. Its expect text: 'carry 62 (h)'s bite: measured incomplete equal to the client count (48), as in the builder's .gate/u4r/whole-as-lost/ dry check; -17's verdict as it comes until marks.json holds ruling 4's plain check'. EVIDENCE: each script runs main() only when it is the entry module and exports its pure helpers (the projection, the name check, the summary-line flag, the late-command line check). A scratch node:test file in the notes folder, never committed, asserts each helper: a P0 set at today's free space is refused; '../x' is refused; a summary line with 'overflow resyncs 3' on a -17 run is flagged; 'result: VOID (L1 RTT rule: ...)' is not 'as expected'. Its output goes in the report. Run `node --check` on both scripts, and run the new suite's three npt-lab and npt-judge lines by hand on the copy, as section 5 item 5 asks, with their output in the notes.
8. FP-6 (small code Lows riding files the Mediums open). 1. NC-U4R-2: scenario::parse refuses, as a usage error, a --bite whose only instance is late_command ('--bite PM-0001-30/late-command is the -30 pre-check's (npt-lab precheck)'); one parse case. 2. EA-U4R-3 = RR3-L8: in Bites::of, set `schedule: Schedule::with_bite(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP, id).is_some()`. In schedule(), world() and net(), replace the quiet fallback with `.expect("Bites::of checked that this instance plants it")`, as room() and client() already do. Add one test assertion: Bites::of on the pump's wallclock id gives schedule false and pump true. 3. RR3-L4: put cfg(feature = "bites") on logs.rs's whole_as_lost and bitten_at and on the as_lost branch, and on precheck.rs's hold and released, the hold and release branches and late_bite's computation, so the measured binary holds no bite code (plan.md:630-631); cfg_attr allow(dead_code) where a field turns dead in the measured build, as decoder_bite has. Clippy and tests stay green in both builds, with counts unchanged. 4. RR3-L7: in replay.rs, before the loop, apply `u8::try_from(h.clients)` and refuse 0 and anything above 255 as a Failed error, before any .bytes file is read; one unit case with a header of 256 clients. 5. T-4: in the clean r1, assert that fate(a) is Some, that each process's tick record count is above 0 in both legs, and that the client half compared at least one record (count per process, not the sum). 6. T-5: contract.rs short_run() returns (Scratch, Vec<Log>) and scenario.rs r4_run returns (Scratch, Summary); each test holds the guard until its last assert. 7. T-3: in check.test.mjs's real-process test, make the child `/bin/sh -c 'echo child $$; sleep 30 & echo grandchild $!; wait'`, at the same 200 ms limit with the same asserts. 8. SEC-U4R-5: in check.mjs, refuse in every crate any metavariable (`$`) anywhere inside an attribute's text, from `#[` or `#![` to its matching `]` in the mask (no attribute holds one today). Correct the comment at :694-695 and the problem text to match. Add one self-test case with `#[cfg_attr(all(), $a)]`. 9. SEC-U4R-6: at check.mjs:1495, the profile refusal names the line and the matched text: `${path}:${line} has a profile key or table (${JSON.stringify(text)})`. Update the self-test's expected line. 10. INF-U4R-5: in det-u4.mjs, import STEPS from check.mjs and use `const steps = { deps: STEPS.deps, "det-u4": ... }`, so it exits 0 only when both pass. 11. INF-U4R-10, points (1) and (3): run() passes killSignal: "SIGKILL" with detached: true, and its doc's 'SIGTERM' changes to match. wasmStep's default clock becomes monotonic (`() => performance.now()`). Its printed lines stay unchanged, and the fake-clock tests stay as they are. 12. EA-U4R-9: rename server/tests/schedule.rs's pin test to `npt_server_s_four_bites_are_listed_under_their_marks_json_ids`, with a doc line: three are the schedule's and one the pump's (U4R-Q6 (a)).
9. FP-7 (docs only, no behaviour). 1. NC-U4R-7: bots/src/lib.rs:49-52 document TURN_EVERY and RETARGET_EVERY as pure command counts, with no time or speed claim. The report names the need under carry 39: a rate-derived constant from npt-sim, by carry 18's route. 2. EA-U4R-1: precheck.rs:3 reads 'on its own simulated clock (integer ns)'. lib.rs:3-5 says SimClock and the runner live in the npt-lab binary, and that the runner is scenario::run with Server. The report's public-items line reads: 'L1Scenario is built as scenario::run plus Server (scenario.rs:340-545, :702-1158); the pre-check's run_leg is a second loop with 7.2's pacing, sharing Server for step 2'. 3. EA-U4R-4: scenario.rs:59-60 cites npt-sim's private CHEST_CM (sim/src/world.rs:79). The report lists PLACEMENT_Z_BOUND_CM beside INTEREST_RADIUS_CM and ROOM_HISTORY_MS as copies of private npt-sim constants, this one unpinned. 4. EA-U4R-5: bots/src/lib.rs:172-175 adds: 'a host that computes a view must feed the endpoint itself and pass &[]'. 5. EA-U4R-8: scenario.rs:10-11 reads 'each reliable message on the one stream channel that decodes it (`stream_of`; the paragraph below)'. The report states the route and why: no npt-proto pub item reads a framed message's type byte. 6. RR3-L2 (b): at the ADMIT_EVENTS_BOUND check, scenario.rs says carry 75's bound is checked on logged runs only, and not on a no-log run (--logs 0, or `npt-lab hash`'s regeneration).
10. FP-8 (the end checks, after every edit above; T-2 = INF-U4R-1, and section 5). 1. Section 5's end checks: check.mjs twice, all 11 steps passing in each run, with the counts as deltas and every new test named. 2. Then, through the runner with -x, df first and the 4 GiB floor, run each of these once: `node ops/runs/det-u4.mjs` (now with deps), and `npt-lab hash --manifest crates/lab/precheck-recordings.json`, as regen-precheck.log ran it. Both PASS lines go in the report, each log labelled with the patch's sha256 at the moment it ran. A SHA-256 mismatch: stop. No re-pin and no retry; bring the log whole to the advisor, with the first differing tick. 3. The report labels the old det-u4.log (01:08-01:11 UTC, before ruling 3's mirror.rs and scenario.rs changes) and regen-precheck.log (2026-11-13 20:27, copy 1) with the tree each ran on. From now on, each builder log is labelled by the patch sha256 at its run (or by the time of the last source edit before it). 4. Both regenerations join section 5's end checks for any later copy.
11. FP-9 (report words, no code; the report must be on disk before round 2). 1. RR3-L5: /tmp/npt-lab-tests holds 124 folders: the 93 that ruling 2 counted, plus 31 from the builder's c3-reg runs at 00:29-00:34 UTC, made before the helper changed. None was deleted (Mustafa's LEFTOVER-DELETE). 2. INF-U4R-7: the wasm step's figures, each labelled: warm, with uptime; after a restart, page cache cold and dependencies warm. Say that M9's cold figure (every release artifact built from nothing) was not measured for U4r's tree, bounded by U4d2's 61.6 s plus U4r's added builds. Give det-u4.mjs's time with its non-cargo sum against 240 s. Use the fix pass's own end-check figures, beside round 1's 39.6 s, 41.9 s and 200.5 s (non-cargo 190.9 s). 3. EA-U4R-2: name the three sites that name npt_sim::World (logs.rs:97, :455, :578; precheck.rs:277) and the two room constructions as a deviation from U4-SEAM (u4-build-plan.md:162-163), with its follow-up. 4. List every follow-up this ruling names, for STATUS, each with its home: NC-U4R-4 (U4g's brief); NC-U4R-5 with EA-U4R-7's logs half (the first logged replacement: U4f or U6); EA-U4R-1's shared loop and EA-U4R-2's constructor (the next npt-lab lane, before the pre-check's re-run after U4h); EA-U4R-4 (the next lane that opens crates/sim); EA-U4R-5 (the U6 brief's bots'-view follow-up); NC-U4R-7's derivation (carry 18's route); RR3-L2 (a) (the next npt-lab lane); RR3-L6 (measured inside ruling 1 point 3 (4)'s judge timing); T-6's contract half (the next unit that edits contract.rs); INF-U4R-6 (U4h's brief); INF-U4R-10 (2) (check.mjs's next change); SEC-U4R-4's U6 half (U6's TickLoop carry). 5. T-7's correction of ruling 4's further finding, in the advisor's words: the main session's npt-judge --only PM-0001-17 check reads the record run's whole-as-lost folder, and the builder's folder is a labelled dry check only.

## Round 2

Round 2 is the second and last round, unless a Critical or High is open after it. Five seats at xhigh, at most three at a time, in two batches that count as one round. Batch 1: netcode-architect, security-auditor, test-engineer. Batch 2: rust-reviewer, infra-engineer.

Subject, only what changed:
- the diff from snapshot 572c4f69 to the fix pass's new snapshot, cut by script as round 1's packet was;
- the plant logs;
- the two check.mjs end checks;
- the det-u4.mjs and precheck-regeneration logs (FP-8);
- FP-0's two runner diffs;
- the builder's report, which must be on disk before round 2 starts (INF-U4R-7, RR3-L5 and EA-U4R-1/-2/-4/-8 are report words).

Each seat confirms its own round-1 findings fixed as ruled, or deferred as ruled, with file:line, and reviews the changed lines. Nothing unchanged is reviewed again.

Who reads what:
- netcode-architect: FP-1's session ending, FP-4's verdict causes, FP-5 (g) and NC-U4R-2's refusal.
- security-auditor: FP-1 (the boundary, both halves), FP-3 (the manifest and argv, the bounds), FP-5 (h), and SEC-U4R-5/-6 in check.mjs.
- test-engineer: every new test and plant in FP-1 to FP-4, T-3, T-4, T-5, FP-5's helper test and its whole-as-lost suite, FP-6's added cases, and FP-8's regeneration evidence and labels.
- rust-reviewer: FP-3, RR3-L4's cfg gating in both builds, RR3-L7, and EA-U4R-3/RR3-L8 with the new expects.
- infra-engineer: FP-0, FP-5 (a) to (f), INF-U4R-5, INF-U4R-10 (1) and (3), and FP-8's times against their budgets.

engine-architect sits out: no public item, crate dependency or seam change is ruled. rust-reviewer reads its EA-U4R-3, and the main session reads its doc and rename items (EA-U4R-1, -4, -5, -8, -9) itself. engine-architect rejoins only if the new snapshot adds a pub item, a dependency or a change to the seam.

The budget: a round-2 finding below High becomes a STATUS follow-up named in the report (a Low never triggers a push). A Critical or High opens round 3 for that finding alone, and one still open after round 3 stops the change-set for Mustafa. Not rounds: the gate after every step, and my review of the pushed head, which always runs.

## For Mustafa

Round 1 found no Critical or High finding. All 10 Mediums are confirmed: two real code defects (a closed connection leaves a ghost player slot in the room; recordings manifest text can inject extra settings), a weak test, an unproven recording re-run, and four disk-safety problems in the run scripts and runners. All are fixed in one pass. Of the 40 Lows, 26 ride the same pass, 9 are fixed in part, and 5 become follow-ups. Nothing in this ruling changes a word or decision of yours, a pass bar or a mark. The only words corrected are mine (ruling 4, about which folder the whole-as-lost check reads). After the fix pass no security finding stays open in U4r. The one exception is a design line carried to U6.

Decisions needed:
- SIZE-U4R. U4r has grown well past its plan, and your SIZE-CHECK rule (2026-11-10: 'stop and tell me in three lines: what grew, why, and the cheaper option') was not applied to it. What grew: about 7,600 lines (1,286 inserted into 13 files, 6,290 in 13 new files) for a unit sized M. U4c and U4j, also sized M, came to about 3,000. Why: the runner (scenario.rs, 2,304 lines with its tests), the -30 pre-check (977), the log writer (786), the contract checker (632), the mirror (490), the replay (429), the run scripts and check.mjs's carries 83-85, plus four build rulings' fixes. The fix pass adds more, mostly tests. Options: (a) the fix pass as ruled: the 10 Mediums plus the small Lows in the same files; (b) cheaper: the Mediums, the one Low tied to a Medium (NC-U4R-3) and the security Lows (SEC-U4R-3 to -7) only, with every other Low a STATUS follow-up. Why it matters: your rule asks for a stop before more growth. My recommendation is (a). The riding Lows sit in files the Mediums already open. Several of them guard the hours-long record runs, which cannot be re-run at the same commit. Deferring them means a later unit re-opens and re-reviews the same files.

For Mustafa:
1. Answer SIZE-U4R with (a) or (b). The main session sends you the three lines and starts the fix pass on your answer. Nothing else is needed now: no merge, no Brave check, no other decision. PR #40 stays a draft.

A heads-up, not a decision: 4.68 GiB is free. The P0 judged set needs about 3.2 GB on top of the 4 GB floor, so the main session will first free space by the approved part-cleans (ruling 1 point 3). If that is not enough, it comes to you under that ruling's point 5 (your LEFTOVER-DELETE route).

## Each finding

### NC-U4R-1 (Medium; fix now)

- **Builder:** FP-1
- **Why:** Confirmed in the read copy. scenario.rs:488-497 drops the session and pushes (slot, 'closed') but never calls room.leave or owner.remove. pump.rs:414-421 closes a joined session on any bad stream message (close_malformed, :364-367). Room::leave (room.rs:689-694) is the only call that frees the slot, and its doc names 'a closed link'. No honest U4r run reaches this path. But U4f's hostile clients and the pre-check's Server run on this glue, and carry 76 calls it a security boundary. So it is a real defect with a small reach today. It is the same defect as SEC-U4R-1, fixed once.

### NC-U4R-2 (Low; fix now)

- **Builder:** FP-6 item 1
- **Why:** Confirmed. Bites::of counts late_command in `any` (scenario.rs:226-236), and only precheck.rs:297 reads it. So `scenario --bite PM-0001-30/late-command` runs clean under a bite id. Brief 2.1 makes an id that no instance of the run plants a usage error. No script passes it today. It rides the parse edit FP-3 makes anyway.

### NC-U4R-3 (Low; fix now)

- **Builder:** FP-4
- **Why:** Confirmed. precheck.rs:455-462 reads late and rewinds_refused once, at the leg's end, with unwrap_or_default, so a slot that is gone reads 0. verdict (:486-504) ignores both refused_logged and `ended`. FP-1 makes a closed session leave the room, which makes the gone-slot case reachable on that path too, so it is fixed in the same pass, with T-6's verdict tests. VOID on an ended session is my reading of 7.2: when a client's counters cannot be read, the setup condition is unshown, and that is VOID, never PASS or FAIL. No current result changes: the existing test asserts that `ended` is empty (:846).

### NC-U4R-4 (Low; follow-up)

- **Where it goes:** U4g's brief, written by the main session: either the lab writes one `tick` counter_wrap record when the room's tick crosses a multiple of 65,536, or the advisor re-reads -58's count against the names actually reported. A STATUS follow-up until then.
- **Why:** Confirmed. The room and the client push only five names (input.rs:86, :104; room.rs:765; client lib.rs:661, :673). -58's check counts at least 6, the server tick's low 16 bits among them. All of -58 is judged after U4g (U4R-Q5), so this is not a U4r defect. Carry 77 (a), one record per name, is met.

### NC-U4R-5 (Low; follow-up)

- **Where it goes:** A STATUS follow-up for the first unit whose runs log a replacement (U4f's hostile clients or U6's reconnect): key `ended` and `gone` by session, or reset the slot in Logs::admit when a new session takes it.
- **Why:** Confirmed: drop_slot pushes (slot, 'replaced'), and run() calls logs.gone for every entry, whatever its reason. No U4r run replaces a session, and today the failure is closed: a later filing counts as a conflict and the log ends crash, or the coverage rule finds a gap, so the judge reads INVALID, never a wrong PASS. Keying by session needs a design for a later join's ticks (carry 62 (d)), and that design belongs to the next unit. This is the same finding as EA-U4R-7's logs half.

### NC-U4R-6 (Low; fix now)

- **Builder:** FP-5 (g)
- **Why:** Confirmed. c-u4r-runs.mjs:181-184 records each summary line and flags nothing. Carry 58 sends any overflow resync on a judged -17 or -18 run to the advisor before the run counts, and ruling 4 point 1 (main-session item 4) expects about 48 on the tick16 run. :164 compares only the result's first word, while precheck's own test asserts the whole line 'result: VOID (L1 late_count 1)' (:932). It rides the script edit FP-5 makes anyway.

### NC-U4R-7 (Low; fix now)

- **Builder:** FP-7 item 1
- **Where it goes:** Deriving the constants from the rate: carry 18's route, named under carry 39 in the report.
- **Why:** Confirmed. bots lib.rs:50 and :52 are command counts, but their docs claim a time ('about 1 s') and a speed. That is the local rate-derived form brief 2.6 (carry 39) bars. Nothing judged reads the cadence, and the bots stay deterministic at any rate. The doc reword rides FP-7. The report names the need under carry 39, so the main session can route an npt-sim item by carry 18 if a time-based cadence is ever wanted.

### T-1 (Medium; fix now)

- **Builder:** FP-2 (a) and plant (i)
- **Why:** Confirmed by reading; not run. room_counts (scenario.rs:1903-1908) leaves out ack_refused and ack_refused_commands. A misrouted Input from the replaced endpoint acknowledges a tick that the fresh AckRing never sent, so receive_input returns at room.rs:783-788 after raising only those two counters. The 'Inputs reach no room call' assert therefore cannot fail for the misroute it names. r8 as a whole still catches that glue plant later, at the Leave assert, so carry 76's evidence holds overall: Medium, not High. The fix is sound: ClientCounters derives PartialEq and Eq (room.rs:63-64), pump_report changes no counter (:699-708), and nothing ticks inside the loop.

### T-2 (Medium; fix now)

- **Builder:** FP-8
- **Why:** Confirmed. det-u4.log ran 01:08:04-01:11:41 UTC. Ruling 3's 8 (A) then changed mirror.rs keep() and scenario.rs's call to it at 01:44-01:49 (u4r-build.md:1215-1222). That is on the bots' view path, which feeds the recorded inputs. SS-P0-s4 was last regenerated in regen-precheck.log (2026-11-13 20:27, copy 1), and c5-end1/c5-end2 regenerate only recordings.json's three synthetic recordings. So no run at this head shows the runner recordings byte for byte. Both paths fail closed at the merge, so Medium. The fix pass also edits record(), so both regenerations must run on its final tree anyway. Same as INF-U4R-1.

### T-3 (Low; fix now)

- **Builder:** FP-6 item 7
- **Why:** Confirmed (check.test.mjs:2901-2923). A node child must start and spawn its grandchild inside 200 ms, and the builder's logs show 203.7-204.0 ms in total. TE2-5 forbids retrying a self-step failure, so one slow start would stop a merge check. A /bin/sh child keeps the same limit and the same asserts.

### T-4 (Low; fix now)

- **Builder:** FP-6 item 5
- **Why:** Confirmed. fate_difference (scenario.rs:1642-1651) gives None when both fates are None. `compared` (:1653-1677) is summed over room and client and checked once against 200 (:1694), which the room's ticks alone clear. It rides the scenario.rs test edits.

### T-5 (Low; fix now)

- **Builder:** FP-6 item 6
- **Why:** Confirmed. short_run (contract.rs:452-460) and r4_run (scenario.rs:1812-1822) return only their results, so the Scratch guard is dropped before the caller's asserts run. A failing r2 or r4 therefore keeps no logs, which goes against my ruling 2 point 32 ('kept on failure').

### T-6 (Low; fix now)

- **Builder:** FP-4 (the verdict cases)
- **Where it goes:** Contract half: a STATUS follow-up for the next unit that edits crates/lab/src/contract.rs. It adds doctored cases for carry 60's missing header param, a missing class_audit event, a duplicate admit, a log npt-log refuses (exit 1) and a usage error (exit 2).
- **Why:** Confirmed. The pre-check verdict's VOID causes for rewinds_refused, commands off their stamp, the RTT rule and Empty are never exercised, and the contract checker has no negative case for five paths. The verdict half is fixed now with NC-U4R-3 (FP-4), because LegOutcome derives Default (precheck.rs:121), which makes the cases cheap. The contract half adds five doctored cases to a file that no Medium opens, so it waits.

### T-7 (Low; fix now)

- **Builder:** FP-5 (i)
- **Why:** Confirmed. This corrects my ruling 4's further finding. That ruling had the main session judge the builder's .gate/u4r/whole-as-lost/ before -17's judged runs, while ruling 2 point 33 says the builder's logs name 2fbec77 for uncommitted code. No script re-makes that run: c-u4r-r2.mjs's SUITES has no such suite. My words now read: 'runs npt-judge --only PM-0001-17 on the record run's whole-as-lost folder (c-u4r-r2.mjs's suite, run from the merge commit) and on its tick16 folder; the builder's .gate/u4r/whole-as-lost/ is a labelled dry check only'. These words are the advisor's, not Mustafa's.

### SEC-U4R-1 (Medium; fix now)

- **Builder:** FP-1
- **Why:** The same defect as NC-U4R-1, confirmed. The exploit sketch holds: input.rs:127-141 repeats last_run with no bound. Its link-closed half is accepted too: Room::leave's contract names 'a closed link'. Loopback and L1 report Closed only when an end is dropped, and no judged run does that mid-run (run() keeps every client until the stop), so no judged result moves. It is a security finding, so it is fixed in the PR, not deferred.

### SEC-U4R-2 (Medium; fix now)

- **Builder:** FP-3
- **Why:** The same defect as RR3-M1, confirmed. hash.rs:275-280 takes `profile` as any string, and record() (scenario.rs:588-594) formats it into a line and then splits on whitespace. Manifest text can therefore add --record or --bytes, whose files and folders are created before any SHA check, or change the regeneration's settings. That walks around carry 82's own name check. Committed, reviewed text bounds the reach, so Medium. It is a security finding, fixed in the PR.

### SEC-U4R-3 (Low; fix now)

- **Builder:** FP-3 (a) and (c)
- **Why:** Confirmed. Nothing ties warmup_s and measured_s to `ticks` before generation (hash.rs:283-284, :312), and parse's `* hz` is unchecked (scenario.rs:1322-1324), with no release overflow-checks for npt-lab. All three runner entries satisfy (10 + 290) x 30 = 9,000 ticks, so the new relation refuses nothing current. It rides FP-3, in the same function. Same class as RR3-L1 and RR3-L3.

### SEC-U4R-4 (Low; fix now)

- **Builder:** FP-2 (b), plant (ii) and the module-text line
- **Where it goes:** U6 half: U6's TickLoop carry (SEC-U4E-5's U6 half). The old connection's stream is flushed and finished before it is closed. A replaced session's Kick gets one transmission once its link is dropped on L1.
- **Why:** Confirmed. drop_slot sends the Kick and drops the link at once (:409-416). L1's close cancels resends (l1/src/lib.rs:1685-1696). r8 never polls c1. On Loopback the Kick still arrives after the server end drops, because Drop clears only the dropping side, so the new assert is sound there. Carry 76's two tests do not name the Kick's arrival, hence Low. It rides r8's FP-2 edit. Same as EA-U4R-6.

### SEC-U4R-5 (Low; fix now)

- **Builder:** FP-6 item 8
- **Why:** Confirmed in check.mjs:694-698. The rule matches only an attribute whose text starts with `$` (/#\s*!?\s*\[\s*\$/). A `#[cfg_attr(all(), $a)]` carrying `path = ...` passes the scan, as the seat's probe log and its rustc build of that form show. So the class SEC-U4D2-R2-3 (b) raised stays open under my narrow carry 85 (c) wording, and the comment at :694-695 claims more than the rule does. No attribute in the spike holds a `$` today, so the wider rule refuses nothing current. It is a security finding, fixed in the PR.

### SEC-U4R-6 (Low; fix now)

- **Builder:** FP-6 item 9
- **Why:** Confirmed. The refusal at check.mjs:1495 names only the file. Its twins at :1471 and :1488 name their value, and carry 85 asks each refusal to name its source and value. It rides the check.mjs edit FP-6 makes anyway.

### SEC-U4R-7 (Low; fix now)

- **Builder:** FP-5 (h)
- **Why:** Confirmed. c-u4r-runs.mjs:144-148 builds the --record file and the --out folder from precheck-recordings.json's `name` before hash::manifest's name check runs. :192 builds a run folder from the bite id's tail. Both come from committed text, so Low. It rides the script edit FP-5 makes anyway.

### EA-U4R-1 (Low; fix now)

- **Builder:** FP-7 item 2
- **Where it goes:** A STATUS follow-up for the next npt-lab lane, before the -30 pre-check's re-run after U4h (carry 40): one shared loop for RQ8's steps 1-3.
- **Why:** Confirmed. There is no L1Scenario item. precheck.rs:350 runs a plain u64 `now`, while its doc (:3) says SimClock, and lib.rs:3-5 places items where they are not. The doc corrections and the report's public-items line are made now. Sharing one loop would refactor the pre-check's run order, and its evidence would then have to be made again, so that part is kept out of this fix pass.

### EA-U4R-2 (Low; follow-up)

- **Where it goes:** A STATUS follow-up for the same next npt-lab lane as EA-U4R-1, before the pre-check's re-run after U4h: `pub(crate) type Sim = npt_sim::World` and one server constructor used by run and run_leg.
- **Why:** Confirmed. npt_sim::World is named in logs.rs:97, :455 and :578 and in precheck.rs:277, and the room is built in two places, against U4-SEAM (u4-build-plan.md:162-163). Refactoring both construction paths inside a fix pass risks behaviour, and no current result differs. The report names the deviation and its three sites now (FP-9).

### EA-U4R-3 (Low; fix now)

- **Builder:** FP-6 item 2
- **Why:** Confirmed. scenario.rs:214 sets `schedule` from npt_server::BITES, which now also holds the pump's wallclock id, while Schedule::with_bite returns None for that id. schedule(), world() and net() fall back quietly, while room() and client() expect. Same as RR3-L8. It rides the Bites::of edit FP-6 makes anyway.

### EA-U4R-4 (Low; fix now)

- **Builder:** FP-7 item 3
- **Where it goes:** A STATUS follow-up for the next unit whose lane opens crates/sim: make the clamp bound (or the fire-origin height) a pub npt-sim item, read by npt-lab.
- **Why:** Confirmed. PLACEMENT_Z_BOUND_CM (scenario.rs:59-61) copies npt-sim's private CHEST_CM (world.rs:79), with no cite, and s1 pins it only against the literal 3,955. The cite and the report line are added now. A pub npt-sim item needs a lane that opens crates/sim.

### EA-U4R-5 (Low; fix now)

- **Builder:** FP-7 item 4
- **Where it goes:** The U6 brief's bots'-view follow-up (ruling 1 point 7): one route for feeding the endpoint.
- **Why:** Confirmed. The only real host feeds the endpoint itself and passes &[] (scenario.rs:962-968), while the tests use the `received` route (behaviour.rs:92-165). A doc line now; keeping a single route changes npt-bots' public Behaviour, which waits.

### EA-U4R-6 (Low; fix now)

- **Builder:** FP-2 (b) and plant (ii)
- **Where it goes:** Its U6 line: as SEC-U4R-4.
- **Why:** The same as SEC-U4R-4, confirmed: deleting the Kick send at :409-414 keeps today's r8 green.

### EA-U4R-7 (Low; fix now)

- **Builder:** FP-1 (owner half)
- **Where it goes:** The logs half: NC-U4R-5's follow-up.
- **Why:** Confirmed. The owner half (the closed path at :488-495 never removes `owner`) is fixed by FP-1. The logs half (`ended` keyed by slot, so logs.gone runs on a replacement) is NC-U4R-5 and waits for the same reasons.

### EA-U4R-8 (Low; fix now)

- **Builder:** FP-7 item 5
- **Why:** Confirmed. The code never reads Limits::TYPES: the only match is the comment at :11. stream_of tries both stream channels (:133-144), so the module doc contradicts itself. A doc reword, plus the route stated in the report.

### EA-U4R-9 (Low; fix now)

- **Builder:** FP-6 item 12
- **Why:** Confirmed. server/tests/schedule.rs:348's name says three bites, and :350 asserts four. A rename with a doc line; the test count is unchanged.

### RR3-M1 (Medium; fix now)

- **Builder:** FP-3
- **Why:** Confirmed, and it resolves the seat's REQUEST CHANGES. Beyond SEC-U4R-2's half, parse refuses --lead and --every-step only when record.is_some() (:1377), but record() sets record_mem after parse, so ruling 1 point 9 (a)'s refusal does not hold on the regeneration path. Once argv is built from fixed flags with the profile checked by name, no manifest value can carry a flag. The gap then closes by construction, so no new refusal is needed beyond FP-3.

### RR3-L1 (Low; fix now)

- **Builder:** FP-3 (a) and (c)
- **Why:** Confirmed. For a runner entry, carry 82's ticks <= bytes/12 limits nothing (hash.rs:283-284, :312), and `bytes` itself is unbounded, so `ticks` can still reach Vec::with_capacity near u32::MAX: RR-L5's limit is incomplete. The cap at 64 MiB is four times today's largest entry (16,950,444 B; recordings.json's largest is 16,700,849 B). It rides FP-3, in the same function.

### RR3-L2 (Low; fix now)

- **Builder:** FP-7 item 6 (part (b))
- **Where it goes:** Part (a): a STATUS follow-up for the next npt-lab lane. On a no-log run `filed` keeps only what `started` needs.
- **Why:** Confirmed by reading. (a) On a no-log run, `filed` grows `res` for the whole run, about 7 MB for a 9,000-tick regeneration: bounded, so it waits. (b) events_stamped is counted only when logs are written (scenario.rs:928-932), so carry 75's bound goes unchecked on --logs 0 and on record() runs. A doc line now says so.

### RR3-L3 (Low; fix now)

- **Builder:** FP-3 (c)
- **Why:** Confirmed. parse's three `* hz` products, warm-up + measured, + after_max and burst_starts' sums use plain arithmetic on command-line text. Cargo.toml turns release overflow-checks on only for npt-sim and npt-proto. It rides FP-3's parse edit. One checked computation of the last burst's tick before burst_starts bounds burst_starts' own sums.

### RR3-L4 (Low; fix now)

- **Builder:** FP-6 item 3
- **Why:** Confirmed. logs.rs:316 (the as_lost branch) and precheck.rs:193 and :317 (the hold) have no cfg. plan.md:630-631 says the measured builds 'never contain' bites, and code that is unreachable is still contained. npt-server gates its own site (pump.rs:451-452). The change is mechanical, and the counts are unchanged.

### RR3-L5 (Low; fix now)

- **Builder:** FP-9 item 1
- **Why:** Confirmed: I counted 124 folders in /tmp/npt-lab-tests myself. That is the 93 from ruling 2 plus 31 from the builder's c3-reg runs at 00:29-00:34 UTC, made before the helper changed. Report words only; nothing is deleted (LEFTOVER-DELETE).

### RR3-L6 (Low; follow-up)

- **Where it goes:** The main session reports read_log's share of npt-judge's time in ruling 1 point 3 (4)'s measurement on one P0 run's logs. If it matters, the next npt-log change makes it one pass or uses borrowed keys. A STATUS follow-up.
- **Why:** An estimate, not a measurement: the 30-50 % was not run. read_log does parse each line twice (log lib.rs:578-579), but the cost can be measured at no extra run, inside the judge timing that ruling 1 point 3 (4) already requires.

### RR3-L7 (Low; fix now)

- **Builder:** FP-6 item 4
- **Why:** Confirmed. replay.rs:265 sizes vec![None; n] from the u32 header, and the u8 check (:272) runs per slot inside the loop, after slot 0's ticks have already allocated. Three lines and one case.

### RR3-L8 (Low; fix now)

- **Builder:** FP-6 item 2
- **Why:** The same as EA-U4R-3, confirmed: schedule.rs:252-257 lists four ids, while with_bite searches only BITE_IDS' three.

### INF-U4R-1 (Medium; fix now)

- **Builder:** FP-8
- **Why:** The same as T-2, confirmed by times and by code path: the bots' view comes from the mirror (scenario.rs:963-968), and their commands feed the recorded tick_input. The builder logs all carry one label for different code states. A re-pin without a ruling is not allowed: a SHA-256 mismatch comes to me whole.

### INF-U4R-2 (Medium; fix now)

- **Builder:** FP-5 (a)
- **Why:** Confirmed. c-u4r-runs.mjs creates the bin folder (:127), runs three release builds (:128-130) and creates the logs folder (:131), all before the first floor() (:146 or :180). c-u4r-r2.mjs creates out/bin (:111) before its builds and its first floor() (:125). A floor stop therefore leaves a folder that the exists-check (:123 and :109) turns into a refusal of every re-run at that commit. Sessions may not delete it (LEFTOVER-DELETE), so the only way on is a new commit. Rule 6 also asks for 4 GB before every release build.

### INF-U4R-3 (Medium; fix now)

- **Builder:** FP-5 (a) to (c)
- **Why:** Confirmed. floor() checks only that 4 GiB is free now. Ruling 1 point 3 (3) stops a set when the next run's projected logs would take free space below 4 GB, and the main session cannot apply that inside one script call. At today's 4.68 GiB, the P0 set (about 3.2 GB at u4r-build.md:739-748's rates) would stop after run 2, with free space under the floor and the commit blocked.

### INF-U4R-4 (Medium; fix now)

- **Builder:** FP-0 (the main session, not the builder)
- **Why:** Confirmed. df -B1 reads 5,021,802,496 B (4.68 GiB), while df -BG reads '5G'. So the runners' 4 GB floor acts at about 3.01 GiB. The floor is the advisor's W1-TARGET rule (advisor-u4-wave1.md:577), not Mustafa's words, so this is not High. No run so far went below 4 GiB (the logs' '5 GB' were 4.7-4.9 GiB). The runners are outside the patch, so the main session edits them, and this ruling approves that runner edit.

### INF-U4R-5 (Low; fix now)

- **Builder:** FP-6 item 10
- **Why:** Confirmed. det-u4.mjs:30 runs wasmStep without the deps step, so run on its own it could PASS on builds that check.mjs would refuse. Adding deps costs about 4 s (c5-end1 deps 3.8 s) and does not touch the 240 s non-cargo sum.

### INF-U4R-6 (Low; follow-up)

- **Where it goes:** U4h's brief (the unit whose recording joins det-recordings.json): det-u4.mjs prints its own non-cargo sum and the line 'over 240 s: to the advisor (ruling 1 point 1)', without changing wasmStep's default print. A STATUS follow-up.
- **Why:** Confirmed: the sum is added by hand today (190.9 s). But the proposed change to wasmStep's summary would break ruling 1 point 1 (4): 'With the defaults it runs and prints exactly what it does now'. The sum only matters once U4h's recording (carry 87) grows it. The report states today's 190.9 s of 240 s.

### INF-U4R-7 (Low; fix now)

- **Builder:** FP-9 item 2
- **Why:** Confirmed. c5-end2's 41.9 s was measured on a cold machine with warm dependencies (u4r-build.md:1525), and M9's 'cold' means every release artifact built from nothing. Report words only, updated with the fix pass's own end-check figures.

### INF-U4R-8 (Low; fix now)

- **Builder:** FP-5 (d)
- **Why:** Confirmed. c-u4r-runs.mjs's header says nothing about how it is run, and run under main-run.sh it would build in the shared target, which brief section 12 item 8 sends to the advisor first. runs.txt records no target and no load, and cargo gets no --offline. It rides FP-5.

### INF-U4R-9 (Low; fix now)

- **Builder:** FP-5 (e)
- **Why:** Confirmed. The commit is checked once (:115; r2 :104), but marks.json is read live at the end of a set that runs for hours (:176, :196; r2 :130-131). A marks.json commit made during a set (carry 62 (h)'s check, ruling 4) would judge it against a file other than the commit runs.txt names. It rides FP-5.

### INF-U4R-10 (Low; fix now)

- **Builder:** FP-6 item 11 (points 1 and 3)
- **Where it goes:** Point (2): a STATUS follow-up for check.mjs's next change: a group kill after a timed call that did not exit 0, with pid reuse weighed.
- **Why:** Confirmed (check.mjs:1894-1906, :2321). (1) spawnSync's default SIGTERM can hang a call past the deadline if the child handles that signal. (3) Date.now is the wall clock. Both are one-line, safe changes; wasmStep uses now() only for differences. (2), a group kill after every timed call, carries a pid-reuse risk on an empty group and goes beyond carry 84 (b)'s 'stopped by the deadline', so it waits.

### INF-U4R-11 (Low; fix now)

- **Builder:** FP-5 (f)
- **Why:** Confirmed. c-u4r-r2.mjs:93-99 spawns its 39 commands with no timeout under the exclusive lane lock, and u4r-run.sh:53's flock has no -w. A hang would hold every lane until someone kills it by hand: INF-4's case, which carry 84 fixed for the wasm step only. It rides FP-5.
````

### Round 1's findings, the six seats' results word for word
_`.checkpoints/netcode-proof-test/u4r-review-r1-full.md` lines 1-402 (sha256 ec61dfbd38fa), as they are:_

````text
# U4r review round 1: the six seats' results (workflow wf_a3541dc5-9e1), 2026-11-14

Written by script, word for word, from the workflow's journal. Snapshot reviewed: wip/u4r.patch 572c4f69...; packet u4r-packet.md cbd82485.... Each seat's notes: u4r-review-<seat>.md. Nothing below is the main session's.

## review-netcode (APPROVE WITH FINDINGS; snapshot 572c4f69435d)

**Seat:** netcode-architect (batch 1), U4r round 1, PR #40, spike branch claude/netcode-proof-test-plan-hcky5g. Read copy /home/user/Shamble-Frontlines/.gate/u4r-snap1 (2fbec77 plus the patch). Notes file: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-review-netcode.md. Two read-only Python probes over the builder's pre-commit logs in .gate/u4r/r2 are kept beside the notes (u4r-review-netcode-probe1.py and probe2.py). They print only and write nothing. No cargo or check.mjs run was needed, so nothing went through the runner.

What I checked and found sound:
- The runner follows RQ8's order at each instant: accept, pump, tick and send, clients in slot order, then the inbox poll. SimClock never goes back, and L1 never delivers a packet in the instant it was sent.
- The header holds every parameter the brief lists, plus the lead, the counter starts, weapon and health, the event cap, the burst starts and spacing, the features, the bite and the binary SHA-256.
- The bite dispatch includes the pump (ruling 2 point 5 (b)).
- The client writer meets carry 62 (a) to (f). The -18 heal record takes neither the sentinel nor a full state's tick.
- Pre-check: the late-command bite gives exactly one late command and no command off its stamp. The wallclock bite changes only the aim and never by 0. The depth relation and r6's whole-list compare at the configured lead are in (ruling 3 point 5). The result lines are the record (ruling 2 points 1 and 2).
- Mirror: checked and unchecked records are counted apart with a full breach counter (ruling 1 point 2). Baselines are looked up by the room's tick (ruling 3 point 7 (b)), and the last acknowledged frame is kept (ruling 3 point 8 (A); the bitten B1 run shows 0 breaches).
- The after-phase cap exception is keyed on the bite id (ruling 3 point 7 (a)).
- c-u4r-runs.mjs: the tick16 bite run starts at 47,251, every other run at 63,736, there is no -18 bite run, and the B1 runs stay for -57.
- c-u4r-r2.mjs carries the amended tick16 expect text (ruling 4 point 1).
- The npt-server edits are in bites builds only, the lint allow sits on the site alone, and the field rule holds. Against .claude/rules/netcode.md, the measured build is unchanged.

Log figures recomputed from the logs, all matching the rulings:
- B1 worst slot: 246 of 900 measured ticks (0.273333).
- Bitten B1: 25 fast heals (14 at age 0, 11 at ages 33 or 34) and 71 slow heals, each after a full state; slowest 1,688,286,323 ns.
- ES: 50.05 reliable events a second, 1,440 shots in 20 s.
- tick16d: 885 ticks below the wrap; the wrap falls at measured tick 585; everyone in view on 600 of 600 measured ticks; 46 after-phase ticks; the cap was not reached; 0 breaches.
- tick16-clean: everyone in view on every measured tick, 0 after-phase ticks.
- Every clean short run has under 1 % of its commands late (highest 0.617 %).
- D13's lines match ruling 1 point 8.

Could not check:
- The report (item 5): it is not on disk yet. u4r-build.md stops at 'Next: r5 and the end checks'. I checked the ruled statements against the logs only; their wording in the report stays the main session's read.
- r5, section 5's end checks, the record runs from the merge commit, and the full 9,000-tick pre-check pair. Only r6's 120-tick pair has been run.

### NC-U4R-1 (Medium)

- **Where:** spikes/netcode-proof/crates/lab/src/scenario.rs:490-497 (Server::io's closed branch); room/src/room.rs:689-695 (Room::leave); server/src/pump.rs:419-421 and :364-367 (close_malformed)
- **Wrong:** Carry 76 calls the lab's session handling a security boundary. When the pump closes a session that has already joined (any malformed stream message: close_malformed sends Kick(malformed) and sets State::Closed), the lab drops the session and records (slot, "closed"), but it never calls Room::leave(slot) and never removes the slot from `owner`. The room keeps a ghost slot: the roster still treats it as live, option B reruns its last command every tick (fill_input, room.rs:841-876), and it keeps being encoded and queuing events until an overflow kick. Room::leave's own doc names this case: 'The slot's session ended (a `Leave`, a closed link)'.
- **Evidence:** In the snapshot, scenario.rs:490-497: `if x.decoder.is_closed() { let slot = x.slot; self.sessions[i] = None; if let Some(slot) = slot { self.ended.push((slot, "closed")); } continue; }`. There is no room.leave and no owner.remove. Leave is the only path that frees the room's slot (through room.receive -> Room::leave, room.rs:739). Kick frees the roster entry inside the room (room.rs:318-323). pump.rs:417-421 closes a joined session on any stream decode error other than WrongChannel or WrongDirection. No U4r run reaches it, because honest clients never send malformed streams. U4f's hostile clients run on this same glue, and so does the pre-check's Server.
- **Fix:** In the closed branch: `if let Some(slot) = slot { if self.owner.get(&slot) == Some(&x.id) { self.room.leave(slot); self.owner.remove(&slot); } self.ended.push((slot, "closed")); }`. Add a unit test beside r8 and r9: a joined session sends a malformed Control frame, and afterwards room.client(slot) is None and the slot can be joined again without a replacement. The test must fail by name under a plant that removes the leave call.

### NC-U4R-2 (Low)

- **Where:** spikes/netcode-proof/crates/lab/src/scenario.rs:226 and :229-236 (Bites::of); scenario::run never reads `late_command`
- **Wrong:** `scenario --role bite --bite PM-0001-30/late-command` is accepted: Bites::of's `any` includes late_command. But only precheck.rs:297 acts on that flag. The run's header names the bite, and nothing is planted: a bite run that cannot bite. The brief says an id that no instance of the run plants is a usage error (2.1, exit 2).
- **Evidence:** grep late_command in crates/lab/src: scenario.rs:184 (the field), :226 (set), :236 (counted in `any`), precheck.rs:297 (the only reader). run() (scenario.rs:702-1156) never reads s.bites.late_command.
- **Fix:** In scenario::parse, after Bites::of, refuse with a usage error any id whose only instance is late_command: "--bite PM-0001-30/late-command is the -30 pre-check's (npt-lab precheck)". Add one parse-test case.

### NC-U4R-3 (Low)

- **Where:** spikes/netcode-proof/crates/lab/src/precheck.rs:455-462 (each leg's end counters) and :486-501 (verdict)
- **Wrong:** `late` and `rewinds_refused`, which decide VOID, are read once, from the room's per-client counters at the end of the leg. A client whose session was dropped or kicked on that leg reads as 0 (`room.client(slot)` is None, so unwrap_or_default). A late command, or a refused Rewind, from a client dropped later would then be invisible to the VOID rule, and the pair could read PASS. `refused_logged`, read after every tick, is printed but not used by the verdict.
- **Evidence:** precheck.rs:456-462: `let k = server.room.client(c.slot).map(|x| x.counters().clone()).unwrap_or_default(); o.late += k.late; o.rewinds_refused += k.rewinds_refused;`. verdict (:489-494) reads only o.late and o.rewinds_refused. The rise in logs.rs room_tick (:504-505) is per tick, but it serves only the scenario. A clean replay has no kick, so no current result changes.
- **Fix:** Add up each leg's late count per tick, as the scenario's room log does (or snapshot each client's counters when its session ends, from `ended`). VOID on max(rewinds_refused, refused_logged).

### NC-U4R-4 (Low)

- **Where:** spikes/netcode-proof/crates/lab/src/logs.rs:270-283 (wrapped); room/src/input.rs:86, :104; room/src/room.rs:765; client/src/lib.rs:661, :673; marks/marks.json:4204-4210
- **Wrong:** Carry 77 (a) is met: one record per counter name. But the room and the client report only five names between them: cmd_seq, press, ack, tick_offset and event_seq. -58's check counts counter_wrap records `>= 6` and lists the server tick's low 16 bits among its six counters. Nothing writes a wrap for the tick, so on this writer -58 can never pass. This is U4g's (U4R-Q5: all of -58 is judged after U4g), so it is a line for U4g's brief, not a U4r defect. Confidence medium: which wire counter -58 means by 'acknowledgement numbers' is not settled here.
- **Evidence:** grep for wraps.push and pending_wraps.push: input.rs:86 CMD_SEQ, :104 PRESS, room.rs:765 ACK, client lib.rs:661 TICK_OFFSET, :673 EVENT_SEQ. The client's CMD_SEQ (:644) is only used as a widen16 name and never pushed. marks.json:4204-4210: 'every 16-bit or narrower counter wrapped in the run (server tick's low 16 bits, command sequence, press counters, the header's command acknowledgements, event numbers, acknowledgement numbers)', count >= 6.
- **Fix:** Add a line to U4g's brief (main session): the lab writes one `tick` counter_wrap record when the room's tick crosses a multiple of 65,536 (logs.wrapped("tick", t, ph) in run's tick loop), or the advisor re-reads -58's count against the names actually reported. No change at U4r.

### NC-U4R-5 (Low)

- **Where:** spikes/netcode-proof/crates/lab/src/scenario.rs:410-418 (drop_slot pushes (slot, "replaced")), :857-859 and :925-927 (logs.gone for every ended entry); logs.rs:263-268, :358-370
- **Wrong:** When a session is replaced, the slot continues under the new session. But run() marks the whole slot gone in the client writer, and writable() then stops waiting for that slot. On any run with a replacement, the new session's ticks are written early as null with incomplete 0, and its later filings for ticks already written count as conflicts, so the log ends `crash` (or has coverage gaps). No U4r run has a replacement (each client sends one Join), and r8 tests only Server. U6's reconnect runs would hit it.
- **Evidence:** drop_slot: `self.ended.push((slot, why))` with why "replaced". run(): `for (slot, _) in std::mem::take(&mut server.ended) { logs.gone(slot); }` ignores the reason. Logs::gone's doc reads '(replaced, kicked or left): its later ticks are not waited for'. writable() returns true for a gone slot (logs.rs:361-362).
- **Fix:** In run(), call logs.gone(slot) only when why != "replaced". Alternatively, key `gone` by session and reset it in Logs::admit when a new session takes the slot.

### NC-U4R-6 (Low)

- **Where:** spikes/netcode-proof/ops/runs/c-u4r-runs.mjs:179-186 (one) and :164-165 (pre-check expectation)
- **Wrong:** Carry 58 says an overflow resync in any judged -17 or -18 run is reported and comes to the advisor before the run counts. Ruling 4 point 1 item 4 expects about 48 resyncs on the judged tick16 bite run, all in the after phase. The script records each summary line, which contains 'overflow resyncs N', but flags nothing, unlike its own '(NOT as expected: to the advisor)' line for the pre-check. Also, the pre-check's expectation check matches only the first word: a late-command pair that is VOID for another reason (for example the RTT rule) would print 'expected VOID' with no flag, while 7.2 shows the bite only when the pair is VOID by late_count alone.
- **Evidence:** c-u4r-runs.mjs:181-184 pushes `${r.out} (wall ...)` without parsing it. :164 `r.out.match(/^result: (\S+)/m)` compared with p.expected 'VOID'. precheck.rs's own test asserts the exact line 'result: VOID (L1 late_count 1)' (:932), but the record run does not.
- **Fix:** In one(), parse /overflow resyncs (\d+)/ and, when it is above 0 on a run listing PM-0001-17 or PM-0001-18, append ' (overflow resyncs: to the advisor before it counts, carry 58)' to the line. For the late-command pair, compare the whole result line with 'result: VOID (L1 late_count 1)' before writing 'as expected'.

### NC-U4R-7 (Low)

- **Where:** spikes/netcode-proof/crates/bots/src/lib.rs:49-52 (TURN_EVERY, RETARGET_EVERY)
- **Wrong:** RETARGET_EVERY = 30 is documented as 'about 1 s at 30 commands a second'. TURN_EVERY = 4 sets a circle 'about 2 m across at the stress scene's speed'. Both are counts derived from the rate, written as local constants, which carry 39's rule (brief 2.6) bars outside npt-sim. At another TICK_HZ the brawl's retarget time and walk circle would change. Nothing judged reads the aim cadence, and the bots stay deterministic at any rate, hence Low.
- **Evidence:** lib.rs:50 `const TURN_EVERY: u64 = 4;`, :52 `const RETARGET_EVERY: u64 = 30;` with the doc 'Commands a brawler keeps its target before it picks again (about 1 s at 30 commands a second)'. Brief 2.6: 'A tick count or a named constant derived from the rate: no local form; the need goes in your report'.
- **Fix:** Either reword both docs as pure command counts with no time or speed claim, or derive RETARGET_EVERY from npt_sim's functions (for example npt_sim::ticks_for_ms(1_000)). Name the need in the report under carry 39 so the main session can add it by carry 18's route.

## review-test (APPROVE WITH FINDINGS; snapshot 572c4f69435d)

**Seat:** test-engineer (batch 1), U4r round 1, PR #40. Notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-review-test.md. Held: the counts (42 new Rust tests, 7 of them bites-only: +35 measured to 526 and +42 bites to 603; +5 self-tests to 76; all 42 appear in c5-end2.log's --list). No TE2-5 entry was needed: c5-end1 and c5-end2 are the build's only full check.mjs runs, and each passed all 11 steps. The suite lines in c-u4r-r2.mjs match the builder's logs. The tick16 expect text is ruling 4 point 1's, word for word. npt-judge and precheck exit 0 on FAIL or VOID. Every ruling-added test asserts what its ruling asks: the radius pin fails on None, the ring-edge pin is exact, the late-command test has its three exact asserts, r6 takes L from the leg and compares the whole list, and the one-id-outside-PM, p9, no --commit, options case, dump reader, h5 header compare and FU-3/FU-7 before-refusal tests are all in place. All 21 SERIES rows have a writer. My own carries 73, 83 (a)-(d), 84 (a)-(c) and 85 (a)-(d) are met. Not checked: the builder's report is not on disk, so its pre-commit labels (ruling 2 point 33), its r4 wording and its r5 estimate are unread. I made no plant or run: 4.7 GB was free, the runner takes only .claude/worktrees/ copies, and the panel says never plant. T-1 is therefore reasoned from the code. Deviations: one shell redirect created and at once deleted an empty .tmp file in the notes folder. I also ran one read-only node timing probe in the session scratchpad.

### T-1 (Medium)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs:1903-1908 (room_counts) and :1958-1968 (r8's Inputs assert)
- **Wrong:** r8 asserts that a replaced session's Inputs reach no room call, but that assert cannot fail for this misroute. room_counts adds up only commands_buffered, commands_repeat, late, buffer_full and press_refused. A misrouted Input from the replaced session is refused by the room's ack check, and that refusal moves only ack_refused and ack_refused_commands, which room_counts leaves out. So carry 76's Inputs half is unproven. The Leave half and the glue-state asserts do bite.
- **Evidence:** The replaced endpoint acknowledged its join FullState as group 0 of tick T1 (client/src/lib.rs:802), so its Input carries ack_groups 1. The replacing ClientSlot starts with a fresh AckRing (room/src/room.rs:644) that never sent T1. ack() therefore returns NeverSent (room/src/ack.rs:161-171), and receive_input returns at room.rs:783-788 after raising only ack_refused and ack_refused_commands. b11-plant-r8.log fails at the glue's `ended` assert (old line :1741), never at the Inputs assert. Under a plant that keeps `ended.push` but skips `*s = None` (scenario.rs:416), r8 fails only later, at the Leave assert (:1971-1974). Reasoned from code; not run.
- **Fix:** In r8, compare the whole counters struct before and after the loop: `server.room.client(0).map(|c| c.counters().clone())`. ClientCounters derives PartialEq and Eq (room.rs:63-64), and nothing ticks inside the loop. Then show it bites under the plant that skips `*s = None` in drop_slot, with the test now failing at "the first's Inputs reach no room call". Restore by Edit and check the sha256.

### T-2 (Medium)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/det-recordings.json and precheck-recordings.json (their sha256 pins); .gate/u4r/det-u4.log; .gate/u4r/regen-precheck.log
- **Wrong:** No run at the snapshot under review has regenerated the three committed runner recordings (SS-P2-s1, ES-P3-s2 and SS-P0-s4) against their pinned SHA-256s. check.mjs's wasm step reads only recordings.json, which holds the synthetic three. So the claim that the runner recordings regenerate byte for byte rests on older trees.
- **Evidence:** det-u4.log ran 01:08-01:11 UTC (copy 4). Copy 5 then changed mirror.rs's frame keep, and the brawl's view comes from the mirror (u4r-build.md:1217-1222, written 01:44-01:49). SS-P0-s4 was regenerated in full only once, in regen-precheck.log (copy 1, 20:27). Every later pre-check run used a 120-tick recording made inside the test or a short manifest (c4-b12-3.log, b12/precheck-short.json). c5-end1 and c5-end2 hold no regeneration of either manifest. Both paths fail closed after the merge (det-u4.mjs in the merge step; precheck's SHA check stops the c-u4r-runs.mjs precheck set), so this is Medium, not High.
- **Fix:** Before the report, run each of these once at the head under the runner with -x, df first, the 4 GB floor: node ops/runs/det-u4.mjs, and npt-lab hash --manifest crates/lab/precheck-recordings.json, as regen-precheck.log did. Report both PASS lines. Label det-u4.log and regen-precheck.log with the tree each ran on. Add both to section 5's end checks for any later copy.

### T-3 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/check.test.mjs:2901-2923 (carry 84 (b) and (c), the real-process test), pid assert at :2910
- **Wrong:** The test depends on timing. A node child must start, require child_process, spawn the grandchild and print both pids inside the 200 ms limit. A slow start on a loaded machine fails `pids.every(p > 0)`, and TE2-5 forbids retrying a self-step failure.
- **Evidence:** In the builder's logs the test takes 203.7 to 204.0 ms, which is the limit plus the kill (c4-plant-*.log, e.g. ok 71 at duration_ms 203.70). My idle-machine probe: a node child reaches both pid lines 20-30 ms after its own start (load average 0.02, 4 vCPUs). That margin shrinks under load.
- **Fix:** Keep the 200 ms limit but make the child a fast-starting shell: run('/bin/sh', ['-c', 'echo child $$; sleep 30 & echo grandchild $!; wait'], HERE, process.env, 200). Keep the same asserts that neither pid is left running.

### T-4 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs:1642-1651 (fate_difference), :1653-1677 (tick_differences), :1694 (compared > 200)
- **Wrong:** In the measured build, r1's clean twin test passes even if neither lab log has a fate_log event, because the comparison is None against None. It also passes if the client log has no tick records, because the room's 240+ ticks alone clear `compared > 200`. Only the bites-build bite test would notice a missing fate event.
- **Evidence:** fate_difference returns `(fate(a) != fate(b)).then(...)`, which is None when both are None. compared is summed over room and client together, then checked once against 200.
- **Fix:** In the clean r1, assert that fate(a) is Some, that each process's tick record count is above 0, and that the client half compared at least one record.

### T-5 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/contract.rs:452-460 (short_run); scenario.rs:1812-1822 (r4_run)
- **Wrong:** Both helpers drop their Scratch guard when they return, before the calling test's asserts run. So a failing r2 or r4 assert keeps no logs, which goes against ruling 2 point 32 (removed on pass, kept on failure).
- **Evidence:** `let dir = scratch("r2"); ... read(&dir).expect("its logs")` returns only the Vec<Log>, and the guard's Drop (scenario.rs:1554-1561) runs then, while no panic is in flight. r4_run likewise returns only the Summary.
- **Fix:** Return the Scratch with the result, e.g. `fn short_run() -> (Scratch, Vec<Log>)` and `fn r4_run(..) -> (Scratch, Summary)`, and keep the guard alive in each test until its last assert.

### T-6 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/contract.rs:464-591; precheck.rs:486-504 (verdict)
- **Wrong:** Some failure paths have no negative case. The contract checker has none for carry 60's missing header param, a missing class_audit event or breaches_count, a duplicate admit, a log that npt-log refuses (exit 1) or a usage error (exit 2). The pre-check verdict's VOID causes for rewinds_refused, commands off their stamp, the RTT rule and an Empty comparer are never exercised: only late (late-command), PASS (r6) and FAIL (wallclock) are.
- **Evidence:** The six doctored cases are: admit gone, footer gone, a SERIES row gone, late_count gone, the shot band halved, and one breach. r7 tests rtt_rule alone, never verdict.
- **Fix:** Add one doctored case each: remove params.weapon_reach_cm (expect the carry 60 line), drop the class_audit event, duplicate one admit, write a non-JSON line into a log (expect exit 1), and call main with no --logs (expect exit 2). Add a verdict unit test with one doctored LegOutcome per VOID cause and HashCompare::Empty.

### T-7 (Low)

- **Where:** advisor-u4r-build4-full.md:32 (ruling 4, further finding) against advisor-u4r-build2-full.md:40-42 (ruling 2, point 33); .gate/u4r-snap1/spikes/netcode-proof/ops/runs/c-u4r-r2.mjs:52-66
- **Wrong:** Ruling 4 has the main session judge the builder's .gate/u4r/whole-as-lost/ logs to show the bite as Shown before -17's judged runs. Ruling 2 point 33 says those logs name 2fbec77 for uncommitted code, and that the record is a bitten run re-made from the merge commit. No script makes that re-made run, so the record step rests on a hand-typed command line.
- **Evidence:** c-u4r-r2.mjs's SUITES has no whole-as-lost suite. The builder's command line exists only in whole-as-lost.log and in u4r-build.md:469-473.
- **Fix:** Read ruling 4's whole-as-lost check as run on the re-made folder (the builder's logs as a labelled dry check only). Add a whole-as-lost suite to c-u4r-r2.mjs: bites build, SS P0 short, --role bite --bite PM-0001-17/whole-as-lost, --marks PM-0001-17, expected measured incomplete equal to the client count. The record run then makes it, and npt-judge --only PM-0001-17 reads that folder after the main session's marks.json commit.

## review-security (APPROVE WITH FINDINGS; snapshot 572c4f69435d)

**Seat:** security-auditor (batch 1), U4r round 1, PR #40 (claude/netcode-proof-test-plan-hcky5g). Notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-review-security.md. Probes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-review-security-probe.mjs, /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-review-security-cfgattr.rs (+ -mod.txt). Log: /home/user/Shamble-Frontlines/.gate/u4r-review-security/probe-scan.log. The five items: (1) carry 76: met as worded; r8 and r9 tell a broken glue from a working one. (2) carry 85 (a) to (e): built with self-tests; this container's environment and the spike's .cargo/config.toml pass cargoConfigProblems and buildInputProblems (probe: [] and []); loader.mjs header fixed. (3) carry 63: FU-3 CLOSED, FU-7 CLOSED (each test fails without its refusal; nothing else in crates/log changed). (4) U4R-Q6 (a): the wall-clock read is in bites builds only, the allow sits on wallclock_aims alone, and no other SystemTime or Instant read exists in npt-server, npt-lab or npt-bots. (5) paths built from manifest text: see SEC-U4R-2, -3, -7. Rule breaks (mine, touching only my notes file): one shell heredoc append and one python3 write, against the Write/Edit-only rule. This run has no Edit tool, so the file was otherwise rewritten whole with Write. I ran no cargo and no check.mjs step: u4r-run.sh only accepts worktrees under .claude/worktrees/. I did run node on check.mjs's pure exported functions (main is guarded) and a plain rustc of a 15-line probe into .gate/u4r-review-security/. Not checked: the failing-under-plant runs of r8, r9 and r10; L1 delivery of the Kick under loss (by reading only).

### SEC-U4R-1 (Medium)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs:490-496 (Server::io, the decoder-closed branch)
- **Wrong:** In seat (carry 76's boundary), beyond the carry's literal words. When the decoder closes a joined session, the lab drops the session but never ends the room's slot: no room.leave(slot), and `owner` still holds the slot. The roster stays live and option B reruns the last command with no bound. Exploit sketch: a client holds FIRE, then sends one malformed control-stream frame. Its session is closed (Kick(malformed)), but its soldier keeps running that last command every tick for the rest of the match, with no connection left to kick. The room also keeps encoding snapshots for the dead slot.
- **Evidence:** pump.rs:416-421 closes a joined session on a bad stream message. scenario.rs:490-496 only sets sessions[i] = None and pushes (slot, "closed"). The Leave path at :474-481 also removes owner, but the room is told there by receive(Leave). Room::leave's own contract (room.rs:689): "The slot's session ended (a `Leave`, a closed link): the slot is free". input.rs:127-141: take() returns last_run as a repeat whenever no command is stamped, with no bound. grep of crates/lab/src for `.leave(` and `.state()`: no call, so a closed link is never noticed either.
- **Fix:** In the closed branch (scenario.rs:490-496), when x.slot is Some(slot), call self.room.leave(slot) and self.owner.remove(&slot) before pushing (slot, "closed"). Treat `x.link.state() == LinkState::Closed` the same way. Add a test next to r8 and r9: a joined Loopback session sends one malformed control-stream message, then server.io; assert room.client(slot).is_none(), owner has no slot, and ended holds (slot, "closed"). The test must fail with the leave call removed.

### SEC-U4R-2 (Medium)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs:588-595 (record) with crates/lab/src/hash.rs:271-285 (Kind::Runner's profile)
- **Wrong:** A runner entry's `profile` is read as any string and pasted into a command line that record() splits on whitespace. Manifest text can therefore add any scenario flag that is not already given, including flags that create files and folders. This walks around the very check carry 82 added (keeping manifest text out of file paths). Exploit sketch: a det-recordings.json entry with "profile": "P2 --record /some/dir/x.rec --bytes /other/dir". `npt-lab hash` (check.mjs's wasm step, det-u4.mjs) regenerates it and creates /some/dir and x.rec plus /other/dir/client-N.bytes before any SHA check. "P2 --lead 8" or "--event-start N" silently changes the regenerated recording.
- **Evidence:** hash.rs:275-280 takes e.get("profile").as_str() with no check. scenario.rs:588-594 does format!("... --profile {profile} --seed ...") and then line.split_whitespace(). parse (scenario.rs:1270-1281) takes any `--name value` pair once, and --record and --bytes are known options (:1339-1340). run() then does create_dir_all(parent) and File::create_new(path) for --record (:1112-1124), and create_dir_all(dir) plus client-{slot}.bytes for --bytes (:766-775). Profile::by_name only sees the first token ("P2").
- **Fix:** (1) In hash.rs manifest(), refuse a runner entry by name unless its profile is a name parse accepts ("B1", or Profile::by_name(p).is_some()), before anything is generated. (2) In scenario::record(), build argv as a Vec<String> of separate elements ("--profile".into(), profile.to_owned(), ...) and never format then split_whitespace, so no manifest value can become a flag. (3) Add an r10 case: an entry with "profile": "P2 --record x" is refused by manifest().

### SEC-U4R-3 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/hash.rs:283-284 and :312-317; crates/lab/src/scenario.rs:1322-1324
- **Wrong:** For a runner entry, carry 82's ticks <= bytes/12 bound limits nothing: record() never reads `ticks`, and warmup_s and measured_s are unbounded and not tied to ticks. Exploit sketch: "measured_s": 100000000 with ticks 9000 makes `npt-lab hash` run a 48-client scenario for about 3 billion ticks before check() compares the tick count. With an overflowing value, parse's `* hz` wraps silently in release (npt-lab has no overflow-checks) or panics in debug.
- **Evidence:** hash.rs:283-284: warmup_s: num("warmup_s")?, measured_s: num("measured_s")? (any u64). The bound at hash.rs:312 compares only entry.ticks with bytes/12. The recording stops at measured_end = (warmup_s + measured_s) x 30 (scenario.rs:821, :1029-1030). check() reads r.ticks.len() only after generation (hash.rs:376-396). parse: num(...)? * hz unchecked (scenario.rs:1322-1324). Cargo.toml:116-120 turns on overflow-checks for npt-sim and npt-proto only.
- **Fix:** In manifest(), for a runner entry, require (warmup_s + measured_s) x npt_sim::TICK_HZ (checked_add and checked_mul) to equal ticks, refused by name, so the bytes/12 bound then also bounds the run. In parse, use checked_mul for the three `* hz` products and refuse an overflow as a usage error. Add one r10 case each.

### SEC-U4R-4 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs:406-420 (drop_slot) and :1943-1976 (r8)
- **Wrong:** The Kick(replaced) that carry 76 asks for is sent but never shown to arrive. r8 never polls the old endpoint for it. On L1 the old link is dropped in the same instant, and that drop cancels every retransmission timer, so a lost first send is never retried. The old client may never learn it was replaced, and nothing documents this. U6 copies this glue, and on QUIC, closing straight after a stream write can discard the write.
- **Evidence:** drop_slot calls x.link.send_reliable(Control, Kick bytes, now) and then sets *s = None at once (:411-416). L1's End::drop calls Net::close, which clears the end's inbox and, for each space, sets timer = None and generation += 1 (l1/src/lib.rs:1685-1696, :1761-1765). r8 asserts owner, ended, room counts and client(0), never a Kick at c1 (scenario.rs:1946-1975).
- **Fix:** In r8, poll c1 after the second join and assert exactly one Control message that decodes (Direction::RoomToClient) to Message::Kick(Kick::Replaced). Say in scenario.rs's module text (:21-23) that the Kick is best effort on L1 (the link is closed in the same instant, so it is not retransmitted). Add to U6's TickLoop carry (SEC-U4E-5's U6 half) that the old connection's stream is flushed and finished before it is closed.

### SEC-U4R-5 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/check.mjs:694-698 (carry 85 (c)'s metavariable rule)
- **Wrong:** This is a finding against the ruling's narrow wording of carry 85 (c), with evidence. The rule refuses only an attribute whose text starts with a metavariable. A #[path] whose whole meta comes from a `meta` fragment inside cfg_attr passes, so the class SEC-U4D2-R2-3 (b) raised stays open. The comment at :694-695 claims such a #[path] "cannot bring an unread file under a listed export's allow". Exploit sketch: a PR adds `macro_rules! m { ($a:meta) => { #[cfg_attr(all(), $a)] mod g; } }` and `m!(path = "x.txt");` inside an allowed export's body in npt-hash-wasm, plus a src/x.txt holding an unsafe block. The deps step passes.
- **Evidence:** My probe (u4r-review-security-probe.mjs, log .gate/u4r-review-security/probe-scan.log) runs unsafeProblems over the read copy's real crate files with npt-hash-wasm's lib.rs extended in memory. Baseline 0 problems; `#[$a]` 1 new problem (refused); plain #[path] 1 (refused); `#[cfg_attr(all(), $a)]` 0 new problems; `#[cfg_attr(all(), $($t)*)]` 0. rustc 1.99.1 (`rustc --edition 2024`) builds u4r-review-security-cfgattr.rs (this macro form, its module read from a .txt file), exit 0, and the binary prints "module from the .txt file says 7". grep of the spike's crates: no attribute holds a `$` today.
- **Fix:** Refuse any metavariable anywhere inside an attribute's text, from `#[` or `#![` to its matching `]` in the mask (no attribute in the spike holds one, so nothing current is refused). Or, as SEC-U4D2-R2-3 first proposed, refuse macro_rules! in the two deny-level exception crates (neither defines one). Correct the comment at check.mjs:694-695 and the problem text to match. Add a self-test case with `#[cfg_attr(all(), $a)]`.

### SEC-U4R-6 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/check.mjs:1495
- **Wrong:** Carry 85 asks that each refusal name its source and value. The config `profile` refusal names only the file, with no line, key text or value, so a reader of a FAIL cannot see which table or key tripped it. Its environment twin (:1471) and the linker refusal (:1488) both name their value.
- **Evidence:** check.mjs:1495: problems.push(`${path} has a \`profile\` key or table: it can drop npt-sim's -C overflow-checks=on, ...`). Compare :1471 (`${k} in the environment sets a profile key (${JSON.stringify(v)})`) and :1488 (`has a key \`linker\` (${...JSON.stringify(v.join(" "))})`). check.test.mjs's carry 85 test pins the message without a value.
- **Fix:** Name the line and the matched text, e.g. `${path}:${line} has a profile key or table (${JSON.stringify(text of the header or key)})`, using the match index the three detectors already have. Update the self-test's expected line.

### SEC-U4R-7 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/ops/runs/c-u4r-runs.mjs:144-148 and :192
- **Wrong:** The run script builds file paths from committed JSON text before any name rule reads it. This is the same class as carry 82, but in the JavaScript layer. precheck-recordings.json's `name` becomes the --record file and the --out folder of a scenario run, which writes the file before `npt-lab precheck` reads the manifest through hash::manifest's name check. marks.json's bite id tail becomes a run folder. Exploit sketch: an entry named "../../../x" makes the precheck set write a recording outside .gate/netcode-proof-test/runs/.
- **Evidence:** c-u4r-runs.mjs:144 reads `const [e] = JSON.parse(...).recordings`, :145 `rec = join(logs, `${e.name}.rec`)`, and :148 passes `--out join(logs, `record-${e.name}`)` and `--record rec` to `npt-lab scenario`. The name rule (hash.rs:235-237) runs only later, in `npt-lab precheck --manifest` (precheck.rs:599-600). :192 builds `bite-${b.id.split("/")[1]}` as a run id and folder name (join(logs, run) at :182).
- **Fix:** Before any use, refuse e.name unless it matches /^[A-Za-z0-9-]+$/, and the bite id's tail unless it matches /^[A-Za-z0-9-]+$/, each by name. Or have the script call `npt-lab precheck`'s manifest check (or `npt-lab hash`'s reader) on precheck-recordings.json before the scenario run.

## review-arch (APPROVE WITH FINDINGS; snapshot 572c4f69435d)

**Seat:** engine-architect (batch 2), U4r round 1, PR #40. Notes, with the seat's own count of public items and every check made: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-review-arch.md

### EA-U4R-1 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/precheck.rs:266-479 (loop :350-472, `let mut now = 0u64` :350; doc :3); scenario.rs:702-1158 (SimClock :819, :1062); lib.rs:3-5
- **Wrong:** No item called `L1Scenario` exists, and RQ8's per-instant order is written out twice. scenario::run is the runner and uses SimClock. precheck::run_leg has its own loop for steps 1-3 and 5 on a plain u64 `now`. Only step 2 is shared, through `Server`. precheck.rs:3 says the legs run "in RQ8's order on one SimClock each", but the code does not use SimClock. lib.rs:3-5 says the library holds SimClock, L1Scenario and ReplayX86, but they live in the binary and L1Scenario does not exist. If a later unit changes step 1 or step 3 in one loop and not the other, the -30 pre-check's re-run after U4h (brief section 8) runs a different order without anyone noticing.
- **Evidence:** A grep for `L1Scenario` finds only doc comments (lib.rs:3, scenario.rs:1). A grep for `SimClock` outside doc comments finds only scenario.rs:65-77, :819, :1062. precheck.rs:350 `let mut now = 0u64;` and :471 `now = next.max(now + 1);`. Both loops call listener.poll_accept, server.io and due_ticks, then room.tick and take/send each on their own (scenario.rs:842-858, :862-944 against precheck.rs:353-445).
- **Fix:** In run_leg, replace the u64 with `let mut clock = SimClock::default();` and `clock.advance_to(next.max(clock.now() + 1))`, so precheck.rs:3 becomes true (or reword :3 to 'on its own simulated clock, integer ns'). Reword lib.rs:3-5 to say these live in the npt-lab binary and that the runner is `scenario::run` with `Server`. The report's public-items line should say: L1Scenario is built as scenario::run plus Server (scenario.rs:340-545, :702-1158); the pre-check's run_leg is a second loop with 7.2's pacing, sharing Server for step 2. Card follow-up: one shared loop for steps 1-3 before the pre-check is re-run after U4h.

### EA-U4R-2 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs:40, :704-726; logs.rs:24, :97, :455, :578; precheck.rs:48, :277-299
- **Wrong:** Brief 2.1 (:356) and the plan's U4-SEAM rule (u4-build-plan.md :162-163) say npt-lab names `npt_sim::World` once. U4r names it as a type in three modules and builds the room in two places. scenario::run calls Bites::world, Room::new(n), with_event_start and Bites::room. precheck::run_leg calls World::new and Room::new(cmds.len()) with neither. A later room setting added to the runner (for example U4h's ring or cover) would be missing from the pre-check's rooms.
- **Evidence:** logs.rs:97 `world: &World`, :455 `room: &Room<World>`, :578 `world: &World`; precheck.rs:277 `let world = World::new(&scene, rec.seed, rec.first_tick);` and :281-290 its own Room::new and Server::new; scenario.rs:706-723. At base, npt-lab named World only for bite items in hash.rs (:366-382 and tests).
- **Fix:** In scenario.rs add `pub(crate) type Sim = npt_sim::World;` and a single `pub(crate) fn server(scene: &Scene, seed: u64, first_tick: u32, clients: usize, bites: &Bites) -> Server` that does what scenario.rs:706-726 does. run and run_leg call it (run_leg passes Bites::default() and then sets decoder_bite as it does now), and logs.rs takes `&Sim` / `&Room<Sim>`. Or keep the code and have the report name the three sites as a deviation from U4-SEAM, with a follow-up for the next npt-lab lane.

### EA-U4R-3 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs:214 and :311-322 (also :263-275, :301-309)
- **Wrong:** Bites::of sets `schedule: npt_server::BITES.contains(&id)`. That list now also holds the pump's PM-0001-30/wallclock, so `schedule` is true for a bite the Schedule does not plant. schedule() then quietly falls back to a clean Schedule. world() and net() also fall back quietly, while room() and client() `.expect`. The flag means 'the crate lists it', not 'this instance plants it', which is the dependence on a list npt-lab does not own that ruling 2 point 5 (b) objected to. A list that disagrees with its instance would give a clean run under a bite id instead of a refusal.
- **Evidence:** schedule.rs (patch): `pub const BITES: [&str; 4] = [BITE_IDS[0].0, BITE_IDS[1].0, BITE_IDS[2].0, crate::pump::WALLCLOCK_BITE]`. tests/schedule.rs: `Schedule::with_bite(30, 7, WALLCLOCK)` is None. scenario.rs:313-321: `if self.schedule && let Some(s) = ...Schedule::with_bite(...) { return s; } Schedule::new(...)`.
- **Fix:** Set `schedule: Schedule::with_bite(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP, id).is_some()`, the same probe `pump` uses at :215-224. In schedule(), world() and net(), replace the quiet fallback with `.expect("Bites::of checked that this instance plants it")`, as room() and client() already do.

### EA-U4R-4 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs:59-61 and test s1 :1719-1735
- **Wrong:** PLACEMENT_Z_BOUND_CM is `POSITION_Z.max() - 140`. The 140 is a copy of npt-sim's private CHEST_CM (world.rs:79), with no cite. s1 checks the copy against a literal 3,955, not against npt-sim, so a change to CHEST_CM would leave carry 49's refusal on a stale bound and s1 still passing. This is a third copy of a private constant from a merged crate, beside INTEREST_RADIUS_CM (ruling 1 point 2) and ROOM_HISTORY_MS (ruling 1 point 5). Unlike those two, it has no ruling and no follow-up.
- **Evidence:** world.rs:79 `const CHEST_CM: i32 = 140;` (private). scenario.rs:61 `npt_sim::grid::POSITION_Z.max() - 140`. scenario.rs:1722-1724 `assert_eq!(PLACEMENT_Z_BOUND_CM, 3_955, ...)`.
- **Fix:** Cite world.rs:79 in the doc at scenario.rs:59-60. List the copy in the report beside the other two as unpinned. Add a card follow-up for the next unit whose lane opens crates/sim: make the clamp bound (or the fire-origin height) a pub npt-sim item, and have npt-lab read it instead of the literal.

### EA-U4R-5 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/bots/src/lib.rs:172-196; host scenario.rs:962-968; tests bots/tests/behaviour.rs:92-165
- **Wrong:** Behaviour::step takes `received` and feeds it to the endpoint itself, and it also takes `view`, documented as what the client sees 'at its newest tick'. A host that passes `received` cannot know the newest tick before step changes it, so its view would be one snapshot stale. The only real host therefore feeds the endpoint itself and passes `&[]`. The tests exercise the `received` route; production uses the other one. So two places own 'feed the endpoint', and the tested path is not the used path.
- **Evidence:** scenario.rs:962 `c.endpoint.input(None, &received, host);`, :964 `mirror.view(c.slot, newest, &mut view)`, :968 `c.behaviour.step(&mut c.endpoint, &[], host, &view)`. behaviour.rs:92, :97, :113, :137 call step with non-empty `received` and an empty view.
- **Fix:** Keep one route. Either drop `received` from step (the host feeds the endpoint, as the lab does) and move the tests to that form, or have step take the view as a closure of the newest tick. If this waits, add to bots/src/lib.rs:172-175: 'a host that computes a view must feed the endpoint itself and pass &[]', and name it in the U6 brief's bots'-view follow-up (ruling 1 point 7).

### EA-U4R-6 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs:1943-1979 (r8); send at :409-414
- **Wrong:** Carry 76 requires the lab to send the old session Kick(replaced). The code does, but r8 checks only `server.ended` and `owner`, never what reached the old client. A change that deletes the Kick send at :409-414 keeps r8 green. Also, on L1, dropping the link straight after the send stops all resends, so on a lossy profile the old client may never get its only Kick. That is acceptable under the carry's wording, but U6 should know it.
- **Evidence:** r8 asserts `server.owner.get(&0)`, `server.ended.contains(&(0, "replaced"))` and room counts. It never calls c1.poll_recv after the second join. L1 close: 'the link sends nothing more, resends and acknowledgements included' (l1/src/lib.rs:1683-1695).
- **Fix:** In r8, after the second join, poll c1 and assert that exactly one Control message decodes to `Message::Kick(Kick::Replaced)`. Show the test failing by name with :409-414 removed, then restore by Edit and check the sha256. Add a line to U6's session-handling carry: a replaced session's Kick gets a single transmission once its link is dropped.

### EA-U4R-7 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs:857-859 with Server::io :444, :472 and drop_slot :405-419; logs.rs:263-268; closed path scenario.rs:488-495
- **Wrong:** `Server.ended` is keyed by slot, not session. On a replacement, io records the new session's admission and then pushes (slot, "replaced") for the old one. run then calls `logs.gone(slot)` for every entry, so the slot is marked gone even though the new session still holds it, and the logs stop waiting for its ticks. No U4r run replaces a session (r8 runs without logs), but a later rejoin, such as a U4f cheat or U6, would get logs that skip a live client. Separately, `owner` is not removed when a session's decoder closes.
- **Evidence:** scenario.rs:444 `self.admitted.push((slot, x.id, tick));`, :472 `self.drop_slot(slot, keep, "replaced", now);` (pushes (slot, "replaced") at :417), :857-859 `for (slot, _) in std::mem::take(&mut server.ended) { logs.gone(slot); }`. logs.rs:263-268: 'its later ticks are not waited for'. :488-495 ends the session without `self.owner.remove(&slot)`.
- **Fix:** In run (both places that drain `ended`, :857 and :937), skip replacements: `for (slot, why) in std::mem::take(&mut server.ended) { if why != "replaced" { logs.gone(slot); } }`. Or key `ended` by session id. Add `self.owner.remove(&slot);` to the closed path at :492-494.

### EA-U4R-8 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs:9-11 against :24-25 and stream_of :133-144
- **Wrong:** The module doc's step 3 says each reliable message goes on 'the channel its type's row of `Limits::TYPES` names', which is the route brief 2.1 (2) and room.rs:603-606 give. The code never reads Limits::TYPES. stream_of decodes each message on both stream channels and takes the one that accepts it, as the doc's later paragraph (:24-25) says. The behaviour is sound and fails closed, but the doc contradicts itself and the change of route is not stated anywhere.
- **Evidence:** A grep for `TYPES` in lab/src/scenario.rs finds only the comment at :11. stream_of: `match (ok(Channel::Control), ok(Channel::FullState))`.
- **Fix:** Reword :10-11 to 'each reliable message on the one stream channel that decodes it (`stream_of`; the paragraph below)'. The report states the route and why: no npt-proto pub item reads a framed message's type byte.

### EA-U4R-9 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/server/tests/schedule.rs:348
- **Wrong:** The pin test is still named `the_three_bites_are_planted_here_under_their_marks_json_ids`, but it now checks four ids, one of them the pump's. A failure output would name the wrong count and the wrong home.
- **Evidence:** schedule.rs:348 is the test name; :350 asserts `npt_server::BITES == [TIMER_PERIOD, UNCAPPED, UNCOUNTED, WALLCLOCK]`.
- **Fix:** Rename it to `npt_server_s_four_bites_are_listed_under_their_marks_json_ids`, with a doc line saying three are the schedule's and one the pump's (U4R-Q6 (a)).

## review-rust (REQUEST CHANGES; snapshot 572c4f69435d)

**Seat:** rust-reviewer (batch 2), U4r round 1, PR #40. Notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-review-rust.md. Checked: all the Rust in the diff (crates/lab/src, crates/bots, crates/server pump/schedule/tests, crates/log/src/lib.rs) for panics on input, overflow, determinism, cfg(bites) gating and memory. Also the npt-server edits under U4R-Q6 (a), the npt-log edits under U4R-Q13 (a), my own earlier findings (RR-L5, RR2-L1/L2/L3/L4 (a)), section 3.3's carries 6 and 14, and the scratch helper. Met: (1) The pump bite sits on that one instance's decoded Inputs (pump.rs:449-453). Its clippy allow covers only wallclock_aims (:232-246). It changes aim[0] only, never tick, sequence, slot, buttons, presses, view time or ack. A measured build is unchanged, and BITES holds 4 entries, pinned in server/tests/schedule.rs:347-368. (2) npt-log: both refusals are present (:307-403, :410-416, :578), and nothing else in crates/log changed. Writer::new runs check_header (:455), and nesting depth is capped by serde_json's recursion limit. (3) bots: no clock and no hash map; the clippy.toml bans fire (b4-plant-clock.log, b4-plant-hashmap.log). (4) Lab determinism: only BTreeMap and Vec; the one HashMap is r1's test-only bite (scenario.rs:684-689). (5) Carry 82: hash.rs:235-237, :266-271, :312-317, dispatch :271-292 and :332-361 (a gap remains: F2). Carry 83 (d): the test at hash.rs:1655-1749 fails under the builder's plant (c4-plant-83d.log). Carry 85: (a) check.mjs:1436-1438, :1470, :1487-1488 and loader.mjs:13-16; (d) check.mjs:1790-1799; (e) check.mjs:1576. Carry 6: scenario.rs:317, :321 and precheck.rs:303. Carry 14's U4r half: scenario.rs:56 and :815, pinned by p5 (:2064-2115) and by check() (hash.rs:365-396). (6) The scratch helper (scenario.rs:1544-1579) puts its folders under target/<profile>/npt-lab-tests, removes them when a test passes and keeps them when it fails (the planted failure's folder hash-83d-3588 is still there). No lab source touches /tmp. Not checked: I ran nothing. u4r-run.sh accepts only a worktree under .claude/worktrees/ and a log under .gate/u4r/, so the read copy, a .gate/ worktree and .gate/u4r-review-rust/ logs cannot go through it, and only 4.7 GB was free. F1, F2, F3 and F8 are therefore shown by reading the code, and F7's cost is an estimate, not a measurement.

### RR3-M1 (Medium)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs:588-597 (record), :1377-1384 (the --lead/--every-step refusal); crates/lab/src/hash.rs:273-282 (the runner entry's profile)
- **Wrong:** record() builds the scenario's argv with format! and then split_whitespace. The runner entry's `profile` comes from the manifest as any JSON string and nothing checks it. Whitespace in it therefore adds scenario flags to the regeneration: `--lead 3`, `--every-step 1` and `--behaviour steady` change the recording's settings; `--bytes <dir>` or `--record <file>` write files at a path the manifest picks (the same class of problem as carry 82's name check); in a bites build, `--role twin --bite <id>` plants a bite. parse refuses --lead and --every-step only when `record.is_some()`, but record() never sets `record` (it sets record_mem after parse), so ruling 1 point 9 (a)'s refusal does not hold on `npt-lab hash`'s regeneration path, which det-u4.mjs uses. The manifest still pins the SHA-256, but a manifest whose SHA was made the same way passes.
- **Evidence:** scenario.rs:588-594: `let line = format!("... --scene {} --profile {profile} --seed {seed} ..."); let argv: Vec<String> = line.split_whitespace()...`. :595-597: `let mut s = parse(&argv)?; s.logs = false; s.record_mem = true;`. :1377: `if (every_step || lead_override.is_some()) && record.is_some()`. hash.rs:275-281 copies `profile` with only `.as_str()`. parse accepts lead, every-step, behaviour, bytes, record, role and bite as long as each is given once (:1270-1281). c-u4r-runs.mjs:148-150 passes argv as an array, so it is not affected. Found by reading; not run (the runner's constraints, see seat).
- **Fix:** In hash.rs manifest(), refuse a runner `profile` other than P0, P1, P2, P3, P4 or B1, by name. In record(), build argv as a Vec<String> of separate items ("--profile", profile, ...), never by splitting text. In parse or record(), also refuse --lead and --every-step when the run is a recording kept in memory (record_mem). Add one manifest test case: a profile containing a space is refused.

### RR3-L1 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/hash.rs:271-292, :312-317; scenario.rs:1322-1324
- **Wrong:** Carry 82's `ticks <= bytes / 12` limits nothing for a runner entry: warmup_s and measured_s decide how long the regeneration runs, and nothing bounds them or checks them against `ticks` before generating. parse's `* hz` is unchecked: it panics in debug builds and wraps in release. A typo such as measured_s 2900 runs ten times longer before check() fails on the tick count. `bytes` itself is also unbounded, so for a synthetic entry with a huge `bytes`, `ticks` can still reach Vec::with_capacity at up to u32::MAX. That is my own RR-L5 fix's limit.
- **Evidence:** hash.rs:283-284 reads `warmup_s: num("warmup_s")?, measured_s: num("measured_s")?` with no relation to `ticks`. :312 `if u64::from(entry.ticks) > entry.bytes / TICK_HEADER_BYTES`. The run's length comes from scenario.rs:1322-1323 (`num(...)? * hz`). check() at :365-396 compares the tick count only after generation.
- **Fix:** In manifest(), for Kind::Runner, require warmup_s.checked_add(measured_s).and_then(|s| s.checked_mul(TICK_HZ)) == Some(ticks) and span_ms == ROOM_HISTORY_MS, refused by name. Cap `bytes` (for example at 64 MiB) for every entry. In parse, use checked_mul with a usage error.

### RR3-L2 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/logs.rs:285-288, :302-347, :435-447; scenario.rs:928-932, :849-854
- **Wrong:** Two gaps on the no-log path (--logs 0 and record()). (a) `filed()` adds one Vec<Res> per tick to `res`, and only write_client_tick removes entries. flush never reaches it without room_tick, because `phases` stays empty, so the map grows for the whole run: about 7 MB for a 9,000-tick regeneration at 48 clients. It is bounded, but it is held for nothing. (b) events_stamped is only counted inside `if s.logs`, so carry 75's admission bound is never checked on a --logs 0 or record() run.
- **Evidence:** scenario.rs:988 calls `logs.filed(...)` on every run. :928-932: `if s.logs { events_stamped += logs.room_tick(...) }`. logs.rs:463 inserts into `phases` only in room_tick, and flush (:437-441) iterates `phases`. Found by reading.
- **Fix:** (a) When Logs has no folder, make `filed` keep only first and last (all `started` needs), or skip `res`. (b) Count the World's events (dump_events, or the room's own count) whether or not logs are written, or document that carry 75's bound applies to logged runs only.

### RR3-L3 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs:1263, :1322-1324, :821, :1038
- **Wrong:** Unchecked u64 arithmetic on operator command-line text: `num(..)? * hz`, burst_starts' `warmup_ticks + first_s * hz + i * every_s * hz`, `warmup_ticks + measured_ticks` and `measured_end + after_max_ticks`. Debug builds panic; release builds wrap silently, because npt-lab has no release overflow-checks (Cargo.toml turns them on only for npt-sim and npt-proto). A wrapped value gives a wrong run with no error.
- **Evidence:** Cargo.toml [profile.release.package.*] lists only npt-sim and npt-proto. The lines quoted above use plain `*` and `+`.
- **Fix:** Use checked_mul and checked_add in parse and burst_starts, and return a usage error (exit 2) naming the option.

### RR3-L4 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/logs.rs:160-163, :316-331; precheck.rs:165-167, :193-215, :317-327
- **Wrong:** npt-lab's own bites are compiled into the measured build. The whole-as-lost writer (logs.rs) and the late-command hold (precheck.rs) are not under cfg(feature = "bites"). They are only unreachable, because Bites::of refuses every id in a measured build. plan.md:630-631 says bites are "compiled only into the bites feature build ... The measured builds never contain them", and npt-server gates its own site the same way.
- **Evidence:** logs.rs:316 `let as_lost = self.whole_as_lost && h != 0 && ...` has no cfg. precheck.rs:193 `let held = |c, hold| ...` and :317 `let hold = if late_bite && ...` have no cfg. scenario.rs:245-250 is the measured build's Bites::of, which always errors. pump.rs:451-452 shows the gated form: `#[cfg(feature = "bites")] let i = if self.wallclock {...}`.
- **Fix:** cfg-gate the `whole_as_lost` and `bitten_at` fields and the as_lost branch in logs.rs, and the `hold` and `released` fields and the hold and release branches in precheck.rs, so the measured binary holds no bite code. Or record a ruling that unreachable code meets plan.md:630-631.

### RR3-L5 (Low)

- **Where:** /tmp/npt-lab-tests (ruling 2 point 32's 'the 93 old /tmp folders')
- **Wrong:** The 93 folders that ruling 2 counted were left alone, but /tmp/npt-lab-tests now holds 124. Another 31 were made after the ruling by the builder's test runs with the old helper, before point 3 (ii) was applied. A report that says '93, untouched' would be wrong about what is on disk.
- **Evidence:** `find /tmp/npt-lab-tests -maxdepth 1 -mindepth 1 -type d ! -newermt '2026-11-14 00:28:00' | wc -l` gives 93; `-newermt` gives 31, all with pid suffix 9579 or 15700, dated 00:29:45 to 00:34:28. .gate/u4r/c3-reg.log:40 starts `cargo test --locked -p npt-lab -p npt-bots` at 00:29:40. None are newer than 00:34:28; the new helper's only kept folder is .gate/shared-target/debug/npt-lab-tests/hash-83d-3588, from the planted 83 (d) failure.
- **Fix:** The report states 124 folders: the 93 plus 31 from the builder's c3-reg runs at 00:29 to 00:34, before the helper changed, none deleted (Mustafa's LEFTOVER-DELETE). No code change.

### RR3-L6 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/log/src/lib.rs:334-403, :578
- **Wrong:** read_log now parses every line twice. The NoDuplicateKey pass allocates a String per key and a BTreeSet per object, then serde_json::from_str builds the Value. On -57's P0 lab logs (about 24 KB per line, about 450 MB per 10-minute run: ruling 1's fact check) this adds CPU time to every judge and contract read. Memory does not change. The cost has not been measured.
- **Evidence:** :578 `no_duplicate_key(&text).map_err(err)?;` comes before :579 `serde_json::from_str(&text)`. :381 `map.next_key::<String>()` and :380 `BTreeSet::new()` run per object. My estimate is +30-50% of read_log's time on number-heavy lines; not run.
- **Fix:** Measure it inside the main session's judge timing (ruling 1 point 3 (4)). If it matters: deserialize keys as borrowed &str with a String fallback for escaped keys, or do one pass that builds the Value and refuses duplicates while building it.

### RR3-L7 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/replay.rs:265-279
- **Wrong:** n = header.clients (u32) sizes `vec![None; n]` for every tick. The u8 check at :272 fails only at slot 256, after slot 0's ticks have already allocated vec![None; n]. A client log whose header says clients 4,000,000,000 aborts on allocation instead of returning an error.
- **Evidence:** :265 `let n = usize::try_from(h.clients)`; :272 `u8::try_from(slot)` inside the loop; :278 `by_tick.entry(w).or_insert_with(|| vec![None; n])`.
- **Fix:** Before the loop, `u8::try_from(h.clients)` and refuse 0, or anything above 255, as a Failed error.

### RR3-L8 (Low)

- **Where:** .gate/u4r-snap1/spikes/netcode-proof/crates/lab/src/scenario.rs:214; crates/server/src/schedule.rs:252-257
- **Wrong:** Bites::of sets `schedule` from npt_server::BITES, which now also holds the pump's id. So `schedule` is true for PM-0001-30/wallclock, although Schedule::with_bite returns None for it. This is harmless today, because Bites::schedule() falls back to Schedule::new (:311-322), but the flag says the schedule is bitten when it is not.
- **Evidence:** :214 `schedule: npt_server::BITES.contains(&id)`; schedule.rs:252-257 `BITES: [&str; 4] = [..., crate::pump::WALLCLOCK_BITE]`; with_bite searches BITE_IDS, which has 3 entries (:263).
- **Fix:** Compute the flag the way `pump` is computed: `schedule: Schedule::with_bite(npt_sim::TICK_HZ, npt_sim::LOOP_CATCHUP, id).is_some()`.

## review-infra (APPROVE WITH FINDINGS; snapshot 572c4f69435d)

**Seat:** infra-engineer (batch 2), U4r review round 1, PR #40 (claude/netcode-proof-test-plan-hcky5g). Notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-review-infra.md. Probe log: /home/user/Shamble-Frontlines/.gate/u4r-review-infra/probe-1.log. All file:line cites are in the read copy /home/user/Shamble-Frontlines/.gate/u4r-snap1/spikes/netcode-proof unless a path says otherwise. What I checked: (1) Carry 84 (a) to (c) in wasmStep and run() is met. There is one 300 s deadline from the first non-cargo call (check.mjs:2331-2349). Each call is given the time that is left. Once a call is stopped or the time is spent, the remaining calls are skipped and named (:2337-2339). A stopped call is named with the seconds it had left (:2345-2347). It runs in its own process group, and that group is killed whole with SIGKILL, with the two limits written down (:1884-1907). Both self-tests are present, and the builder's plants make each one fail by name (u4r-build.md:990-999). My probe ran run() on the test's own child 50 times, with limits from 200 ms down to 50 ms. It reported a timeout every time, and no child or grandchild was left running (probe-1.log). (2) det-u4.mjs meets ruling 1 point 1 (5). It imports wasmStep with crates/lab/det-recordings.json and the bites half off, in its own folder (.gate/netcode-proof-test/det-u4). It prints through check.mjs's main and exits 0 only on PASS (det-u4.mjs:21-36). check.test.mjs has the options case. (3) Times against budgets. The wasm step took 39.6 s warm (c5-end1.log:174) and 41.9 s after a restart (c5-end2.log:174), both far under 200 s. det-u4 took 200.5 s, of which non-cargo calls were 190.9 s, under 240 s (det-u4.log:24). (4) The run scripts: c-u4r-runs.mjs and c-u4r-r2.mjs refuse uncommitted tracked changes, name their .gate/ folders by the commit's first 12 hex digits, never overwrite a set, and delete nothing. What I could not check: no report is on disk yet (u4r-build.md:1539-1541), so section 5 item 4 and return item 9 are counted from the notes and logs only. I could not re-run det-u4.mjs or check.mjs either: u4r-run.sh:36-43 refuses any copy outside .claude/worktrees/ and any log outside .gate/u4r/. Disk, reported but not settled: 5,025,054,720 B free at 07:59 UTC (df -h 4.7G). The main folder's cache .gate/netcode-proof-test/target is 6.8G, .gate/netcode-proof-test/runs 2.0G, shared-target 8.2G and .gate/u4r 867M. The projected judged sets, at the rates in u4r-build.md:739-748, are about 3.2 GB for P0, 0.66 GB for P2, 0.38 GB for P3 and 0.57 GB for B1, about 4.8 GB in all. Free space today is 0.68 GiB above the 4 GB floor. Cost per run: check.mjs takes 11.9 and 12.6 min under the exclusive lock (c5-end1, c5-end2). det-u4 adds about 3.6 min to every merge on the plan's crate list. There is no CI and no money cost, and the runs are offline under the runners.

### INF-U4R-1 (Medium)

- **Where:** det-u4.log (/home/user/Shamble-Frontlines/.gate/u4r/det-u4.log:1-26, the only det-u4 run, 01:08-01:11 UTC) against crates/lab/src/mirror.rs:86-96 and crates/lab/src/scenario.rs:877-878, :963-968
- **Wrong:** The only det-u4.mjs PASS, its 200.5 s and its 190.9 s of non-cargo calls come from a run made before ruling 3's code changes. Ruling 3 changed mirror.rs keep() and how scenario.rs calls it (8 (A)) and the burst parameters (8 (D)), both on the path that rebuilds the runner recordings. Nothing in check.mjs rebuilds the det-recordings.json entries, so neither end check covers them. As it stands, the evidence for this head is an inference, and the notes label every log the same way ('tree 2fbec77 plus the uncommitted U4r changes'), whichever code state it was made on.
- **Evidence:** det-u4.log:1 lock at 01:08:04, :26 exit 0 at 01:11:41. u4r-build.md:1215-1222: 8 (A) and 8 (D) were written 01:44-01:49 UTC. The bots' view comes from the mirror: scenario.rs:963-968 (`mirror.view(c.slot, newest, &mut view)` then `c.behaviour.step(..., &view)`). The bots' commands feed the room's tick_input, which the recording stores (scenario.rs:867-868). Before ruling 3, keep() held 34 frames (FRAMES_KEPT: advisor-u4r-build3-full.md:62). It now keeps 34 frames other than the pin, plus the pin's frame (mirror.rs:89-95). With last_acked inside the ring, that makes 35 frames, so a client whose newest tick is 34 behind now gets a view where it used to get none. That changes its bot's inputs and so the bytes that det-recordings.json:4-5 pins by SHA-256. It is very likely still a PASS: P2 and P3 have 150 ms of delay (l1/src/lib.rs:191), so reaching age 34 would take about 28 lost snapshots in a row. Still, no run on this head shows it.
- **Fix:** Either run `node ops/runs/det-u4.mjs` once on the final tree through the runner with -x (df first, 4 GB floor), with its own log under .gate/u4r/, and report that line. Or have the report say: 'det-u4.mjs PASS, 200.5 s (non-cargo 190.9 s, W 3.1 s), run at 01:11 UTC before ruling 3's mirror.rs and scenario.rs changes and not re-run; U4r's merge-step det-u4 run is the first on the final code, and a SHA-256 failure there means re-pinning, never a retry.' Whichever you choose, give each builder log a label that tells code states apart: the time of the last source edit before it, or the patch's sha256 at that moment.

### INF-U4R-2 (Medium)

- **Where:** ops/runs/c-u4r-runs.mjs:126-131 (and the first floor() at :146, :162, :180); ops/runs/c-u4r-r2.mjs:108-116 (first floor() at :125)
- **Wrong:** Both scripts make three release builds before any free-space check, although section 4 rule 6 requires at least 4 GB free before every release build. Both also create the set's folder before the first check: c-u4r-runs.mjs makes the bin folder at :127 and the logs folder at :131, and c-u4r-r2.mjs makes out/bin at :111. A floor stop at the first run therefore leaves a folder behind, and any re-run at that commit is then refused ('exists: a run set is never overwritten', c-u4r-runs.mjs:123, c-u4r-r2.mjs:109). Sessions may not delete it (LEFTOVER-DELETE), so the only way on is a new commit.
- **Evidence:** c-u4r-runs.mjs:128-130 build the measured npt-lab, the bites npt-lab and npt-judge, then :131 makes the logs folder, then :180 is the first floor() for a judged set (:146 for precheck). c-u4r-r2.mjs:111 makes the folder, :113-116 build, and :125 is the first floor(). Judged runs use the main folder's cache with CARGO_TARGET_DIR unset (brief section 12 item 8), so they are not run under main-run.sh, and main-run.sh's df check (main-run.sh:38-46) never runs ahead of them. Free space is 4.68 GiB now (df -B1 at 07:59 UTC).
- **Fix:** In both scripts, move `floor("the release builds")` above the first mkdirSync (c-u4r-runs.mjs:126, c-u4r-r2.mjs:110), and call it again before each build call (c-u4r-runs.mjs:128-130, c-u4r-r2.mjs:113-116). Create the set's log folder only after the up-front check of INF-U4R-3 passes, so a stop before the first run leaves nothing that blocks a re-run.

### INF-U4R-3 (Medium)

- **Where:** ops/runs/c-u4r-runs.mjs:93-97 (floor) and :179-194 (one call runs the whole set); ops/runs/c-u4r-r2.mjs:86-90
- **Wrong:** floor() only checks that at least 4 GiB is free now. It does not check that the next run's logs will leave 4 GiB free. A whole set runs in one call, so the main session cannot check df between runs as ruling 1 point 3's stop rule (3) requires ('stops if the next run's projected logs would take free space below 4 GB'). A run may therefore push free space below the floor. The next floor() then stops the set halfway, and the folder it leaves blocks a re-run at that commit (c-u4r-runs.mjs:123).
- **Evidence:** Log size per 10-minute run with -57 listed: P0 509.1 MB, P2 219.8 MB, P3 125.8 MB, B1 190.6 MB; without -57 about 56 MB (u4r-build.md:739-748). The P0 set as c-u4r-runs.mjs:186-194 builds it: 3 measured runs at about 509 MB, 3 -57 bite runs at about 509 MB, and 2 or 3 -17 bite runs at about 56 MB, about 3.2 GB in all. At today's 5,025,054,720 B (4.68 GiB), and before allowing anything for the builds: run 1 leaves about 4.2 GiB, which passes floor(); run 2 leaves about 3.7 GiB, which is below the floor; floor() then stops run 3. The outcome is a set with 2 of its 8 or 9 runs, free space under the floor, and the commit blocked.
- **Fix:** Before the builds, compute the set's projected log bytes: the number of runs times the per-profile rate (509.1, 219.8, 125.8 or 190.6 MB for runs listing -57, 56 MB otherwise; a constant table citing u4r-build.md:739-748). Add a build allowance. Refuse to start, without creating any folder, unless free space minus that total is at least 4 GiB, and print the projection. Keep the per-run floor as a backstop that checks free space minus that run's projected bytes. Do the same in c-u4r-r2.mjs (its 13 suites came to about 145 MB in the builder's run).

### INF-U4R-4 (Medium)

- **Where:** OUT OF SEAT AND OUT OF THE PATCH: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-run.sh:63, :68 and main-run.sh:38, :43
- **Wrong:** Both runners read free space with `df -BG`, which rounds up to the next GiB, and then stop only if that figure is below 4. Section 4 rule 6's floor of 4 GB before every wasm32 or release build (every check.mjs and det-u4 run) is therefore enforced as about 3.01 GiB, and the logs show free space higher than it is.
- **Evidence:** My reading at 07:59 UTC: `df -B1 --output=avail /home/user` gives 5025054720 (4.68 GiB), while `df -BG --output=avail /home/user` gives 5G. The builder's logs show the same gap: c5-end2.log:2 says 'free 5 GB' where the notes say 'df 4.7G' (u4r-build.md:1519), and det-u4.log:2 says 'free 5 GB' where the notes say 'df 4.9G' (u4r-build.md:1008). By contrast, the new scripts' floor() reads exact bytes (statfs bavail x bsize, c-u4r-runs.mjs:94-95).
- **Fix:** This is the main session's script (INF2-4's lineage), not U4r's lane. Read exact bytes: `avail=$(df -B1 --output=avail /home/user | tail -1 | tr -dc 0-9)`, then `[ "$avail" -lt $((4*1024*1024*1024)) ]`, and log it as `free $((avail/1048576)) MiB`. Make the same two-line change in both runners, through the advisor's approved route for runner edits.

### INF-U4R-5 (Low)

- **Where:** ops/runs/det-u4.mjs:30 (steps holds det-u4 alone); check.mjs:2592-2594 (deps runs before wasm 'so the flags its builds use are checked first')
- **Wrong:** det-u4.mjs runs wasmStep's release builds without the deps step. In check.mjs, the deps step runs first to refuse rustflags, a linker, a profile override or a target set from the environment or a cargo config. Run on its own as its header shows (det-u4.mjs:5), det-u4 can then PASS on builds that check.mjs would refuse. At U4r's merge it runs after check.mjs in the same main-run.sh call, which covers it there, but not when someone runs it directly.
- **Evidence:** det-u4.mjs:30: `const steps = { "det-u4": () => wasmStep(...) }`. STEPS.deps is exported (check.mjs:2584, :2592). The builder's det-u4 run was a runner call of its own, without check.mjs (det-u4.log:3-26).
- **Fix:** `import { STEPS } from "../../check.mjs"` and use `const steps = { deps: STEPS.deps, "det-u4": ... }` (about 3-4 s more, c5-end1.log deps 3.8 s), so the script exits 0 only when both pass. Or have its header say it is run only after check.mjs in the same runner call.

### INF-U4R-6 (Low)

- **Where:** check.mjs:2576-2578 (summary); ops/runs/det-u4.mjs
- **Wrong:** Ruling 1 point 1 sends det-u4 to the advisor once its non-cargo calls pass 240 s of the shared 300 s. Neither the summary nor det-u4 prints that sum or the share of the deadline used, so every later merge on the plan's crate list depends on someone adding 11 rounded 'seconds:' entries by hand. U4h's TC B1 recording (carry 87) will grow that sum.
- **Evidence:** det-u4.log:24 prints only the individual seconds ('npt-lab hash 187.8, Wasm SS-P2-s1 2.3, ...'). The builder added them up by hand to 190.9 s (u4r-build.md:1017-1018).
- **Fix:** In wasmStep, add up the seconds of the non-cargo calls and add '; non-cargo <s> s of the 300 s deadline' to the summary. In det-u4.mjs, when that sum exceeds 240 s, print the line 'over 240 s: to the advisor (ruling 1 point 1)' (still exit 0 if PASS). Pin it with one fake-clock case.

### INF-U4R-7 (Low)

- **Where:** report section 5 item 4 / return item 9 (not on disk yet); evidence: /home/user/Shamble-Frontlines/.gate/u4r/c5-end2.log:1-2, :174; u4r-build.md:1519-1525
- **Wrong:** 'Cold binds' on M9's basis means every release artifact is built from nothing (the card, netcode-proof-test.md:729). The 41.9 s figure is cold for the machine only (uptime 5 min) and warm for the dependencies (builds 9.4, 9.4, 1.3 and 1.5 s). If the report calls it 'the cold figure', it misstates its basis. The uptime appears only in the notes, not in the logs.
- **Evidence:** c5-end2.log:174 'PASS  wasm  41.9 s'. The builder's notes: 'cold machine, warm dependencies' (u4r-build.md:1525). U4d2's M9 cold figure was 61.6 s with builds of 41.8 s (card :727-729). The runners print no uptime (u4r-run.sh:64-67).
- **Fix:** Report words: 'wasm step 39.6 s (warm, uptime about 1 h, c5-end1.log) and 41.9 s (after a restart, uptime 5 min: page cache cold, dependencies warm, c5-end2.log). M9's cold figure (all release artifacts from nothing) was not measured for U4r's tree; bound: U4d2's 61.6 s plus U4r's added builds, far under 200 s. det-u4.mjs 200.5 s warm (uptime 56 min), non-cargo 190.9 s of 240 s; route (c), ruling 1 point 1.' Optionally have the runners log `cat /proc/uptime` in their header.

### INF-U4R-8 (Low)

- **Where:** ops/runs/c-u4r-runs.mjs:1-39 (header), :132-138 (runs.txt head), :84-85 (build)
- **Wrong:** Unlike c-u4r-r2.mjs:22-23 and det-u4.mjs:16-17, c-u4r-runs.mjs says nothing about how to run it: whether it holds the lane lock, and which target it uses. Run under main-run.sh it would build in the shared target (main-run.sh:17 exports CARGO_TARGET_DIR), which brief section 12 item 8 sends to the advisor first. Run bare, it holds no lock, so other lanes can share the 4 vCPUs. That inflates the per-run wall seconds the script records for r5's ticks per second (:36-37, :181-182), and a CPU-heavy set can slow another lane's det-u4 toward its 300 s deadline. runs.txt records neither the target, nor the load, nor whether a lock was held. Its cargo builds also lack --offline unless CARGO_NET_OFFLINE is set.
- **Evidence:** c-u4r-runs.mjs:85 `sh("cargo", ["build", "--release", "--locked", ...])` uses the inherited environment. runs.txt's head (:132-138) holds the commit, the SHA-256s and the raw-log path only.
- **Fix:** Header: 'Run it in the main folder with CARGO_TARGET_DIR unset (plan §7.1 item 6), not through main-run.sh; hold the lane lock for the whole set (`flock -x <shared-target>/.lane.lock node ...`) so the wall seconds are on an otherwise idle machine.' Add a runs.txt head line `target: ${process.env.CARGO_TARGET_DIR ?? "the main folder's own cache"}; uptime and load: <os.uptime(), os.loadavg()>`. Refuse a set CARGO_TARGET_DIR unless a `--shared-target` flag names the advisor's ok. Pass `--offline` to cargo.

### INF-U4R-9 (Low)

- **Where:** ops/runs/c-u4r-runs.mjs:115-118, :176, :196; ops/runs/c-u4r-r2.mjs:104-107, :130-131
- **Wrong:** The commit label is checked once, at the start. The judge reads the live marks/marks.json at the end of a set that runs for hours (:196), and the bite list is read from it at :176. c-u4r-r2.mjs reads marks.json and SERIES.md afresh for every suite. If one of them changes in the main folder during the set (for example carry 62 (h)'s marks.json commit, ruling 4 point 4), the verdict is computed against a file that is not the commit runs.txt names. `--untracked-files=no` also lets an untracked file through that cargo picks up by itself (a crate's build.rs) without any tracked edit.
- **Evidence:** c-u4r-runs.mjs:196: `sh(judge, ["--marks", join(SPIKE, "marks", "marks.json"), ...])` runs after every run of the set. c-u4r-r2.mjs:130-131 pass `marks/SERIES.md` and `marks/marks.json` by path for each suite. The git check is at :115 / :104 only.
- **Fix:** At the start, copy marks.json (and SERIES.md for r2) into the set folder, write their sha256 into the runs.txt / r2.txt head, and pass the copies to the judge and the contract. Before the judge, check that `git rev-parse HEAD` and `git status --porcelain --untracked-files=no` still give the start values, and stop if they do not. Also refuse an untracked file under spikes/netcode-proof/crates (`git status --porcelain -- crates` with untracked files listed).

### INF-U4R-10 (Low)

- **Where:** check.mjs:1895-1903 (run), :2321 (wasmStep's `now = Date.now`)
- **Wrong:** Carry 84 (b) is met, but run() stays fragile at three points. (1) spawnSync keeps its default killSignal (SIGTERM) and waits for the direct child to exit before run() can kill the group, so a child that handles or blocks SIGTERM hangs the call past the deadline. (2) The group is killed only on ETIMEDOUT, so a timed call that ends any other way can leave its leg processes running, unseen by the runners' idle wait (cargo and rustc only). (3) The deadline uses Date.now, the wall clock, so a clock step during a run can lengthen or shorten every remaining call's limit.
- **Evidence:** check.mjs:1895 `spawnSync(cmd, args, { ..., ...(timeout ? { timeout, detached: true } : {}) })` sets no killSignal. :1897 `if (timedOut && r.pid)`. :2321 `now = Date.now`. Today npt-lab and Node both exit on SIGTERM, which is why probe 1 found 0 of 50 processes left (probe-1.log).
- **Fix:** Pass `killSignal: "SIGKILL"` with `detached: true`. After every timed call, whatever its outcome, run `try { process.kill(-r.pid, "SIGKILL") } catch {}` (ESRCH ignored), and keep `timedOut` as it is. Have WASM_IO or main pass a monotonic clock (`() => performance.now()`) as wasmStep's `now`. The fake-clock tests stay as they are.

### INF-U4R-11 (Low)

- **Where:** ops/runs/c-u4r-r2.mjs:93-99 (step) with its header :22-23; /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-run.sh:53 (out of patch)
- **Wrong:** c-u4r-r2.mjs is to be run under the exclusive lane lock, but its 39 commands (13 suites x 3) have no timeout. If one hangs, the lock is held until someone kills it by hand: this is INF-4's case, which FX-7 and carry 84 fixed for the wasm step only. Other lanes' runners then wait on `flock` with no -w, so without any limit.
- **Evidence:** c-u4r-r2.mjs:95 `spawnSync(exe, args, { cwd: SPIKE, encoding: "utf8", maxBuffer: 1 << 28 })` passes no timeout. u4r-run.sh:53 `flock "$mode" 9` has no -w. Each suite is 20-30 s measured plus at most 5 s after (c-u4r-r2.mjs:53-65).
- **Fix:** Give each step() a timeout, for example 10 x the suite's game seconds plus 120 s (or 600 s flat), using check.mjs's exported run() so the call gets its own group and the group kill. Record 'stopped at <s> s' as that command's exit line. A stop is a result, never a retry, which keeps condition 5.
````

### Mustafa's words of 2026-11-14 (his SIZE-U4R answer (a) sets the fix pass's scope)
_`.checkpoints/netcode-proof-test/history.md` lines 2040-2058 (sha256 0f4044cd1900), as they are:_

````text
- **Mustafa, 2026-11-14 about 11:07 UTC, after his /compact (verbatim):** "You were just compacted to cut usage. Read
  the resume card in .checkpoints/netcode-proof-test.md first and continue from its next steps. Restart the saver. Do
  not redo finished work.
  My words of 2026-11-14.
  1. SIZE-U4R: (a). The fix pass as your advisor ruled it in advisor-u4r-r1-full.md: the 10 Mediums and the small Lows
     in the same files. Your reports of this morning said "Decisions needed: none", but that ruling's "For Mustafa" asks
     me this question, and the card does not carry it. Put it on the card and in history.md with this answer. When the
     fix pass is done, tell me in one line how many lines U4r has then.
  2. SIZE-CHECK from now on. My rule of 2026-11-10 was not applied to U4r. Check a unit's size at every snapshot and
     write it on the card beside its planned size. "Well past its plan" means more than twice the plan. Where the plan
     gives only a size letter, take what earlier units of that letter came to (M: about 3,000 lines). When a unit passes
     that, stop and tell me in three lines, as the rule says.
  3. Do not wait for my /compact. This morning the work stood still from about 08:31 UTC until this message. Ask me as
     before. If I have not typed /compact within 20 minutes of your asking, refresh the card and carry on: the system
     compacts the chat by itself, and after that you read the card first.
  4. Main is now 4460ef5b8d6dc8566b29c15ded75c6f0c8be386b: PR #52 is merged (foundation step 2.3's first docs pass,
     eight files under docs/, no code and no kit, gate or CI file). Do not merge it in now. Take main in when main
     changes code, CI or rules, and once before PR #40 is marked ready.
  Record these words in history.md under my decisions."
````

## The carries this packet holds

_Rule: every entry of `.checkpoints/netcode-proof-test/u4s-carries.md` whose text matches `^(18|39|40|58|60|62|75|76|77|82|83|84|85|87)\. \*\*`._

### Carry 18
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 129-132 (sha256 c74e52553bd8), as they are:_

````text
18. **this pass (the tick grep):** "A new tick-count constant in crates/sim/src joins check.mjs's `TICK_DERIVED` as
    `<name in lower case>_at(TICK_HZ)`, its function pinned at 30 and 60 Hz in s1's `DERIVED` table and tied by a const
    assert in ticks.rs; the deps step refuses any other form (no mechanical check ties s1's table to `TICK_DERIVED`:
    review keeps the two lists equal)."
````

### Carry 39
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 169-179 (sha256 c74e52553bd8), as they are:_

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

### Carry 40
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 256-264 (sha256 c74e52553bd8), as they are:_

````text
40. **O1 (u4-conditions-check.md:42-45; advisor-u4.md:343-347; the advisor's U4s ruling, O1; its last sentence the main
    session's words for the advisor's W1-U4E ruling, U4E-Q5, approved with the clause 'for the tolerance and the cap' by
    the advisor's U4d close ruling):** "U4r's brief, which comes to the advisor, states that at U4r the room has neither
    the view-time tolerance nor the cap (both land in U4h: u4-build-plan.md §5 U4e's `rewind_at` and §5 U4h), so the -30
    pre-check's setup condition (0 view-time replacements and 0 clipped shots on both legs: the advisor's condition 3)
    holds by construction at U4r for the tolerance and the cap, and its RTT argument cannot be tested there; and it
    either re-runs the pre-check pair after U4h merges (6 runs) or says why not (the advisor's lean is the re-run). The
    room's count of commands that `rewind_at` gives no Rewind, because the ring does not hold their claimed view time's
    tick, counts as a replacement in that setup condition: a pre-check leg with any is VOID (U4E-Q5)."
````

### Carry 58
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 269-273 (sha256 c74e52553bd8), as they are:_

````text
58. **U4E-Q6's two conditions (advisor-u4-wave1.md:476-479; the advisor's W1-U4E ruling; homed by the advisor's U4d
    close ruling):** "U4r's contract test logs each client's relevant gameplay-event rate and its event queue's
    high-water mark per burst. An overflow resync in any judged -17 or -18 run (U4r's, or a later unit's) is reported,
    and those runs come to the advisor before they count: net F121 warns that a cap set too low resyncs every client
    on every burst and changes -18's heal path."
````

### Carry 60
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 284-288 (sha256 c74e52553bd8), as they are:_

````text
60. **TUNE-1's header (advisor-u4-wave1.md:252-253; the advisor's W1-U4D ruling; homed by the advisor's U4d close
    ruling):** "Each run's header records the scene's weapon (interval, damage, reach, magazine, reload, spread) and
    its soldiers' health, read from `Scene::weapon()` and `Scene::health()`. U4r's runner writes them into npt-lab's
    L1 headers, and the contract test shows them in every header. U6's npt-server header does the same (plan.md's U6
    row)."
````

### Carry 62
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 289-316 (sha256 c74e52553bd8), as they are:_

````text
62. **U4j's log rules, U4r's runner and logs (the advisor's U4j ruling, "Carries", U4r, with FX-5 (a), FX-10, FX-12
    and R2.4; its round-2 ruling, B2.1: NC-U4J-R2-N1 and N6):** "U4r's logs meet marks/README.md §1's `hs` rules as
    U4j merged them, and the judge's coverage rule (SERIES.md's `decoded_hash_mismatch_count` row): (a) each name
    only in its own process's log: `server_hash` in the room's (process `room`), `decoded_hash` in the clients'
    (`client` on L1, `bots` on L2), `replay_hash` only in a `replay` log on x86_64; (b) every `hs` list and every
    `tick.s.snap_incomplete` list holds exactly the room header's `clients` entries, at the room's slot; (c) the
    writer buffers each client's ticks until every earlier tick is resolved, and writes a tick lost only once none of
    its groups can still decode (once it has left the client's frame ring, or at the stop: on L1 the runner may
    decide it there); a hash is never dropped, else the log ends `crash`; (d) the head: every client is admitted,
    and its first snapshot resolved, in the warm-up, before the room's first measured hash for its slot (a client
    admitted later is -19's join: C-U6-11); (e) the tail: after the measured phase the clients keep logging until
    every measured tick is resolved, a tick still in flight at the stop is not written, and the room's log covers
    every snapshot the room sent (it sends none of a tick it does not log); (f) a run and its twin log `h` over the
    same ticks; (g) the contract test shows `decoded_hash_mismatch_count` resolving with no gap on every -17 suite, a
    short L1 P3 suite with the reorder step on among them (no `crash` footer, 0 mismatches, no gap), and, if U4e's
    client drops stale groups (entry 61 (d)), reports its drop count beside `decoded_hash_compared_count` on -58's
    P3 suite; (h) NC-10 (the advisor's U4j ruling, R2.4): the contract test reports `tick.s.snap_incomplete`'s sum on
    each short L1 P0 run. If it is 0 on every one, U4r adds to -17's L1 P0 part, with a bite (a client that writes
    null and incomplete 1 for a refused group), before -17's judged runs: `{"name": "setup: L1 P0 loses no group (no
    client-tick left incomplete)", "series": "tick.s.snap_incomplete", "stat": "sum", "window": "measured", "op": "==",
    "value": 0, "parts": ["L1 P0"], "setup": true}`. If it is not 0, the cause goes to the advisor first. (Both the
    reported sum and the check read the measured phase: each join's ticks between its full state and that full state's
    acknowledgement carry no snapshot, so the client resolves them lost by design in the warm-up: the advisor's U4e
    round-1 ruling, R5, NC-L10.) marks/ is not U4r's lane unless its brief gives it this one check; else the main
    session adds it in its own commit."
    **(h) amended (the advisor's U4r build ruling, point 4; advisor-u4r-build1-full.md):** the bite's wording now reads
    "(a client writer that files a tick it decoded whole as lost: null and incomplete 1)", its id
    PM-0001-17/whole-as-lost; the check's JSON stays word for word. U4r's brief 2.5 row (:643) reads this way.
````

### Carry 75
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 350-358 (sha256 c74e52553bd8), as they are:_

````text
75. **NC-M5 and A3, the client's event-number base (u4e-review-netcode.md, Block h; u4e-review-arch.md, A3; the test
    seat's probe Q-10; the advisor's U4e round-1 ruling, R3):** "U4e's client rebuilds each event's 16-bit number near
    its own start (`ClientEndpoint::with_starts`, 0 by default), and the wire carries no event base (`JoinAccepted` and
    `FullState` have none), so a client whose start is 32,768 or more from the room's counter rebuilds every event
    number wrong, and the hash reads them (Q-10). U4r's lab gives each endpoint, through `with_starts`, the event start
    it gave the room (`Room::with_event_start`) and its command start, and admits every client before the room has
    stamped 32,768 events past that start (about 30 s of SS's events); a later admission comes to the advisor first
    (the room exposes no event counter). The contract test shows `decoded_hash_mismatch_count` 0 on a run whose event
    start is far from 0 (-58/lt-event-seq's, 3 below the wrap, among them). The wire field is U6's (C-U6-13)."
````

### Carry 76
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 359-366 (sha256 c74e52553bd8), as they are:_

````text
76. **A6 and SEC-U4E-5, ending a session (u4e-review-arch.md, A6; u4e-review-security.md, SEC-U4E-5; the advisor's U4e
    round-1 ruling, R3):** "npt-room decides a replacement and a `Kick(overflow)` but sends the old connection nothing,
    npt-server's `Decoder` has no call that ends a session, and `Room::receive` takes any message by slot alone, so the
    caller's session handling is a security boundary. U4r's lab, when `Room::join` reports a replacement, sends the old
    session `Kick(replaced)` and drops its `Decoder` and link before it routes anything else; when the room kicks a
    client or takes its `Leave`, it drops that session. Two tests: a second endpoint joining with slot 0's token and
    bind while the first is live, after which the first's Inputs and `Leave` reach no room call and the second keeps
    the slot; and a kicked session whose later datagrams reach no room call."
````

### Carry 77
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 367-376 (sha256 c74e52553bd8), as they are:_

````text
77. **U4r's readings of U4e's room and client (u4e-review-netcode.md, NC-L2, NC-L4, NC-L9 and NC-L10; the advisor's U4e
    round-1 ruling, R4 and R5):** "(a) -58's wrap check counts `event.counter_wrap` records (marks.json -58), and the
    room and the client report wraps per client slot (`ClientSlot::wraps`, `StepOut::wraps`): U4r writes one record per
    counter name that wrapped (SERIES.md's row: 'per counter that wrapped'), never one per client, so one counter's 48
    per-client wraps cannot pass the check. (b) Under PM-0001-18/no-fallback, a burst whose last acknowledged baseline
    is exactly 32 ticks old at the first tick after it logs `event.burst_eviction` 1, yet the bitten room codes that
    tick from the defaults (its stale copy is taken only once B's ring slot is overwritten) and heals: U4r's report
    counts the bitten runs' bursts at that age apart. (c) The contract test reports each client's
    `ClientEndpoint::stale()` sum on each short L1 run (groups of ticks already resolved when they left the frame ring;
    no SERIES.md row); a sum above 0 on L1 comes to the advisor."
````

### Carry 82
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 377-386 (sha256 c74e52553bd8), as they are:_

````text
82. **EA-8 with RR-L5, the recordings manifest (u4d2-review-arch.md, EA-8; u4d2-review-rust.md, RR-L5; the advisor's
    U4d2 round-1 ruling, Blocks 4 and 5):** "U4r's runner recordings join crates/lab/recordings.json (§4.15), whose
    entries today hold only the synthetic generator's inputs, read by hash.rs's `manifest` and generated by its
    `regenerate_all`: U4r changes that schema (a kind or script key, and the dispatch both functions need), and its
    brief says so. With that change, the reader checks each entry's name against `[A-Za-z0-9-]+` and bounds `ticks` by
    `bytes / 12` before it generates anything (today a bad name reaches a file path and the `h <recording> <tick>
    <hash>` line, and a large `ticks` reaches `Vec::with_capacity` before the length and SHA-256 check)."
    **Amended (the advisor's U4r build ruling, point 1; advisor-u4r-build1-full.md):** "U4r's runner recordings join
    crates/lab/recordings.json" now reads "join crates/lab/det-recordings.json, in recordings.json's schema"; the rule
    (kind key, dispatch, name and ticks checks) is unchanged.
````

### Carry 83
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 387-401 (sha256 c74e52553bd8), as they are:_

````text
83. **EA-6, EA-3 and TE-L3, the comparer for -30 and the wasm step's verdicts (u4d2-review-arch.md, EA-3 and EA-6;
    u4d2-review-test.md, TE-L3; the advisor's U4d2 round-1 ruling, Block 5; with EA2-3, RR2-L3 and TE2-R2-2: the
    advisor's U4d2 round-2 ruling, Block 2):** "(a) For -30's pre-check, U4r gives `HashCompare::of` two lists that
    name the same recording on both legs and line up tick for tick (it compares by position, names included), and says
    where its printed comparison line comes from (`compare_text` is private to npt-lab's binary); TickHash's struct doc
    then says what its field doc says, 'the recording both legs replay' (today it still says 'the recording (or run) it
    replays'). (b) At U4r's change to check.mjs's `wasm` step (its recordings: entry 82), the clean half also requires
    exit 0 from `npt-lab hash` and `npt-lab hash compare` (today the verdict is read from their English lines alone,
    which fails closed), with one fixture case each. (c) In the same change, the std-sin verdict re-shows that the bite
    causes the mismatch it judges: the bitten native leg's first difference from a clean bites-build native leg is the
    committed (recording, tick, field), or the step FAILs when the committed catch is not on the manifest's first
    recording before the hash-map bite's arming tick (today it is sound only because SS-s1's tick 2 lies inside the
    hash-map pair's equal prefix). (d) One npt-lab bin test runs `hash compare` with a bite and no `--on` over two
    recordings and asserts that only the first listed is armed (EA-9's one default, the legs'; today no test pins it:
    compare arming every recording again leaves every test and the step green, r2-te-rr2-l3-compare-arm.log)."
````

### Carry 84
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 402-416 (sha256 c74e52553bd8), as they are:_

````text
84. **INF2-1 with INF2-2 and TE2-R2-1, the wasm step's time bound and a stopped call's processes
    (u4d2-r2-review-infra.md, INF2-1 and INF2-2; u4d2-r2-review-test.md, TE2-R2-1; the advisor's U4d2 round-2 ruling,
    Block 1):** "At U4r's change to check.mjs's `wasm` step (entries 82 and 83), or in ops/runs/det-u4.mjs if U4r's
    Evidence decision moves the step's runs there: (a) the step's calls other than its cargo builds share one deadline,
    WASM_CALL_LIMIT_MS (300 s) from the first of them: each is given what is left, and once one is stopped the rest are
    not started, each named as skipped (today each call has its own 300 s, so a hang in npt-sim's replay makes 12 calls
    each wait out 300 s, 60 min under the exclusive lane lock: r2-inf-hang.log). (b) A call stopped by the deadline
    takes every process it started with it (today spawnSync's timeout signals only the direct child, so `npt-lab
    hash`'s two leg processes can keep running after their parent is stopped, unseen by the runners' idle wait, which
    looks for cargo and rustc only), by a documented means (its own process group, killed whole, or another) or by one
    whose effect (c)'s second test proves. (c) Two self-tests: the shared deadline, with a fake runner and clock (a
    first call that takes the whole limit; the next calls skipped and named); and run() itself on real processes (a
    child that starts a grandchild and sleeps, with a limit of about 200 ms: run() returns `timedOut`, and neither
    process is left), which also pins run()'s timeout wiring (today dropping it leaves 71 of 71 green:
    r2-te-inf2-2-run-timeout.log)."
````

### Carry 85
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 417-440 (sha256 c74e52553bd8), as they are:_

````text
85. **The deps step's unread inputs and two overclaims (u4d2-r2-review-security.md, SEC-U4D2-R2-1 to -3;
    u4d2-r2-review-rust.md, RR2-L1, RR2-L2 and RR2-L4 (a); u4d2-r2-review-test.md, TE2-R2-3; the advisor's U4d2 round-2
    ruling, Block 2):** "At U4r's change to check.mjs (entries 82 to 84), the deps step, each refusal naming its source
    and value, with one self-test case each, and the container's environment and the spike's own config still passing:
    (a) refuses a linker set for any target, by an environment variable whose name ends in _LINKER or by a `linker` key
    in any table of a cargo config read, as COMPILER_ENV and COMPILER_KEYS refuse a compiler (today
    CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER passes: r2-rr-superset.log run 3); and DEPS_CHECKED's limits and
    crates/hash-wasm/loader.mjs's header (the one edit U4r makes in crates/hash-wasm) say what the target_features read
    gives: a feature enabled at compile time shows in the section, which the loader refuses, but the section is the
    linker's record, so link arguments from a RUSTC_WRAPPER (allowed, for sccache) can rewrite it (built:
    sec_probe_tf2_ld.wasm, r2-sec-loader.log), and the x86-64 legs have no artifact check (today both say a flag from a
    route check.mjs does not read is refused too). (b) It refuses any `profile` key in a cargo config read and any
    environment variable whose name starts with CARGO_PROFILE_ (today a config's `[profile.release.package.npt-sim]
    overflow-checks = false` passes and drops npt-sim's `-C overflow-checks=on`, which plan §4.5 asks for in every
    profile: r2-sec-fx1-config.log, r2-sec-routes.log). (c) In npt-tcpinfo and npt-hash-wasm it refuses `include` other
    than directly before `!` and an opening bracket, and in every crate an attribute whose text starts with a
    metavariable (`#[$` or `#![$`; none today), so that neither `m!(include!)` nor a #[path] built from a `meta`
    fragment brings an unread file under a listed export's allow (today both pass, and deps' held line says every
    include! literal names a .rs file: r2-te-sec-r2-3-macro-include.log, r2-sec-pathmeta.log). (d) It reads an empty
    CARGO_HOME as unset and a relative one against the folder cargo runs in, as cargo does (today an empty one hides
    ~/.cargo/config.toml from FX-1's refusal and from the read-back: r2-rr-cargohome.log). (e) Its text beside
    buildInputProblems says which source cargo uses: CARGO_ENCODED_RUSTFLAGS if set, even empty; else RUSTFLAGS if set,
    even empty; else the matching target tables' flags, if any; else build.rustflags (today 'the first non-empty
    one')."
````

### Carry 87
_`.checkpoints/netcode-proof-test/u4s-carries.md` lines 572-578 (sha256 c74e52553bd8), as they are:_

````text
87. **U4R-Q9, the TC B1 seed 3 recording (the advisor's U4r brief check):** U4h records TC B1 seed 3 with `scenario
    --record` and its TC shooter, adds it to crates/lab/recordings.json under carry 82's schema, and states the `wasm`
    step's time with it against the 5-minute budget.
    **Amended (the advisor's U4r build ruling, point 1; advisor-u4r-build1-full.md):** from "adds it to" on, it now
    reads "adds it to crates/lab/det-recordings.json under carry 82's schema, and states det-u4.mjs's time with it, its
    non-cargo calls against carry 84's shared 300 s deadline (over 240 s it comes to the advisor, RR-L4's remedy
    weighed first)".
````

## The carries this packet left out

Ids: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 59, 61, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 78, 79, 80, 81, 86, 88, 89.
