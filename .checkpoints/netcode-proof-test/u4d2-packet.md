# Review packet: U4d2 round 1 (the Wasm-in-Node hash and the determinism step: npt-hash-wasm and its loader, npt-lab's `hash` subcommand and `HashCompare`, check.mjs's wasm step and rules; carries 29, 41, 52, 65 and 66)

_Cut by `review_packet.py` from `u4d2-packet.json`. It holds sources only: code diffs, the approved plan's lines, the advisor's rulings, the last round's findings and Mustafa's words, each copied by line range. It holds no conclusion of the builder or the main session (no builder notes, brief, checkpoint card or summary). You stay free to read any file and run any test._

## Heads

- The build's `<base>`: bfda3cd on the spike branch (main merged in; the advisor's U4d2 brief check, L3: P6). U4d2's worktree HEAD is 51520b4, whose spikes/ equals bfda3cd's and 28a85e7's (`git diff --stat bfda3cd 51520b4 -- spikes/` is empty). Counts at `<base>`: check.mjs 10 steps, measured 399, bites 446, self 52 of 52.
- U4d2's copy: `.claude/worktrees/u4d2-build` (branch u4d2-build at 51520b4; the change is uncommitted there: 9 tracked files changed and 8 new, all under spikes/netcode-proof). The same change as a patch: `.checkpoints/netcode-proof-test/wip/u4d2.patch` (committed at e2f2a5b; sha256 a526ef8e…).
- The read copy `.gate/u4d2-r1`: a worktree at 51520b4 with that patch applied by `git apply`; `diff -rq` of its spikes/ against U4d2's copy's spikes/ finds no difference, and the 17 files' sha256 are equal.
- U4d2 was built on `<base>`. U4e merged into the spike branch later, at c88eb31 (npt-room, npt-server's library, npt-client); `git apply --check` of wip/u4d2.patch on c88eb31 exits 0.
- The diffs below compare the base copy `.gate/u4d2-prefix` (a worktree at 51520b4) with U4d2's copy; new folders are compared with the empty folder `.gate/u4e-empty`. check.test.mjs's diff is in the side file `.checkpoints/netcode-proof-test/u4d2-packet-tests.md`.
- The build's runs are under `.gate/u4d2/` (logs written by the runner `.checkpoints/netcode-proof-test/u4d2-run.sh`): the two counted check.mjs runs e1-check.log and e2-check.log, the `--list` e3-h7.log, the wasm-step runs b8-wasm1.log and b8-wasm2.log, the std-sin scans b8-scan1.log and b8-scan2.log, the six plants p1-features.log to p6-usize.log, the runner's `-k` demonstration k-demo-with.log and k-demo-without.log, and the block runs b1 to b8.

## What changed (the diffs)

### The workspace manifest (Cargo.toml)
_`git diff --no-index .gate/u4d2-prefix/spikes/netcode-proof/Cargo.toml .claude/worktrees/u4d2-build/spikes/netcode-proof/Cargo.toml`:  1 file changed, 20 insertions(+), 13 deletions(-)._

````diff
diff --git a/.gate/u4d2-prefix/spikes/netcode-proof/Cargo.toml b/.claude/worktrees/u4d2-build/spikes/netcode-proof/Cargo.toml
index 36fc8e5..858efb7 100644
--- a/.gate/u4d2-prefix/spikes/netcode-proof/Cargo.toml
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/Cargo.toml
@@ -20,6 +20,7 @@ members = [
     "crates/judge",
     "crates/lab",
     "crates/tcpinfo",
+    "crates/hash-wasm",
 ]
 
 [workspace.package]
@@ -49,8 +50,10 @@ rand_core = { version = "=0.10.1", default-features = false }
 sha2 = "=0.11.1"
 # U4s: npt-sim's one source of transcendentals (plan §4.5: ADR 0004's float allow-list, "transcendentals through libm
 # only"), pure Rust; already in Cargo.lock through quinn-proto's fastbloom, so no new package. Default features kept
-# (`arch`: hardware sqrt, floor, ceil, trunc and rint, each exact, where the target has them), the set fastbloom already
-# turns on in the server build, so npt-sim's libm resolves one feature set on native and wasm32 (plan §4.5 "Features").
+# (`arch`: sqrt and sqrtf by x86-64's `sqrtsd`, and fma and fmaf with run-time CPU dispatch, on x86-64; nothing on
+# wasm32 stable; all correctly rounded: libm 0.2.16's configure.rs:82-101 and arch/x86/fma.rs:9-27, the rust review),
+# the set fastbloom already turns on in the server build, so npt-sim's libm resolves one feature set on native and
+# wasm32 (plan §4.5 "Features"; check.mjs's deps step compares the sets).
 libm = "=0.2.16"
 # U3: the transports and L2 (plan §4.1's npt-net; PM-0001-35's floors: quinn-proto >= 0.11.18, rustls >= 0.23.46,
 # ring >= 0.17.12, tokio >= 1.54.1; U0 resolved these versions).
@@ -72,21 +75,25 @@ futures-util = { version = "=0.3.34", default-features = false, features = ["sin
 
 [workspace.lints.rust]
 # forbid, the compiler backstop (u3c review C2R-1): in a crate that inherits these lints no #[allow] can relax it
-# (rustc refuses one, a macro-built one too, with E0453), so unsafe code there does not compile. Two crates are ADR
+# (rustc refuses one, a macro-built one too, with E0453), so unsafe code there does not compile. Three crates are ADR
 # 0002:264-265's exceptions ("an exception needs a written invariant, a test and an #[allow] with its reason") and
 # carry their own [lints] tables instead, every other lint as here: npt-client-wasm allows unsafe_code for the whole
 # crate, for wasm-bindgen's generated code (U7); npt-tcpinfo denies it, and its two pub functions in src/linux.rs,
 # srtt_us and unacked_and_unsent (getsockopt SOL_SOCKET, then getsockopt TCP_INFO or ioctl SIOCOUTQ, on a TCP socket
-# only), each carry one #[allow(unsafe_code, reason = "...")] (U3c, R20 (a)). check.mjs's deps step checks that this
-# line is forbid; that every other member inherits these lints and sets no unsafe_code (nor unsafe-code); that the two
-# exceptions' tables equal these key for key but for unsafe_code (allow, deny), a lint given in any form its reader
-# does not read being refused (a lint's own [lints.<tool>.<lint>] table; a quoted, dotted, hyphenated or uppercase
-# name); that no manifest has a cargo-features or rustflags key; that, as its scan reads the crates' files, no other
-# crate relaxes the lint or has unsafe code, and npt-tcpinfo relaxes it only by those two allows and has unsafe code
-# only inside them; that no #[path] or include! literal, manifest target path or symbolic link compiles a file outside
-# its crate; and that the environment and cargo configs it reads pass no lint-lowering rustflags and set no clippy or
-# fmt alias, other rustc, workspace wrapper, config include or test runner, and that the environment sets neither
-# RUSTC_BOOTSTRAP nor cargo's channel override (__CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS). That scan guards against
+# only), each carry one #[allow(unsafe_code, reason = "...")] (U3c, R20 (a)); npt-hash-wasm denies it, and its seven
+# exports in src/lib.rs (begin, feed, step, hash_lo, hash_hi, dump_len and dump_word, integers only), each carry one
+# #[allow(unsafe_code, reason = "...")] for their #[unsafe(no_mangle)], its only unsafe (U4d2). check.mjs's deps step
+# checks that this line is forbid; that every other member inherits these lints and sets no unsafe_code (nor
+# unsafe-code); that the three exceptions' tables equal these key for key but for unsafe_code (allow, deny, deny), a
+# lint given in any form its reader does not read being refused (a lint's own [lints.<tool>.<lint>] table; a quoted,
+# dotted, hyphenated or uppercase name); that no manifest has a cargo-features or rustflags key; that, as its scan reads
+# the crates' files, no other crate relaxes the lint or has unsafe code, npt-tcpinfo relaxes it only by those two
+# allows and has unsafe code only inside them, and npt-hash-wasm relaxes it only by one allow on each listed export,
+# whose #[unsafe(no_mangle)] is its only unsafe, with u32 or i32 parameters and results; that no #[path] or include!
+# literal, manifest target path or symbolic link compiles a file outside its crate; and that the environment and
+# cargo configs it reads pass no lint-lowering rustflags and set no clippy or fmt alias, other rustc, workspace
+# wrapper, config include or test runner, and that the environment sets neither RUSTC_BOOTSTRAP nor cargo's channel
+# override (__CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS). That scan guards against
 # an accidental relaxation; review is the wall against a deliberate one. What it does not check is listed above
 # DEPS_CHECKED in check.mjs (among them a cargo config's [env], a build script's output, a RUSTC_WRAPPER program and
 # which toolchain runs).
````

### Cargo.lock
_`git diff --no-index .gate/u4d2-prefix/spikes/netcode-proof/Cargo.lock .claude/worktrees/u4d2-build/spikes/netcode-proof/Cargo.lock`:  1 file changed, 7 insertions(+)._

````diff
diff --git a/.gate/u4d2-prefix/spikes/netcode-proof/Cargo.lock b/.claude/worktrees/u4d2-build/spikes/netcode-proof/Cargo.lock
index 2720623..137ab7d 100644
--- a/.gate/u4d2-prefix/spikes/netcode-proof/Cargo.lock
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/Cargo.lock
@@ -799,6 +799,13 @@ dependencies = [
  "npt-client",
 ]
 
+[[package]]
+name = "npt-hash-wasm"
+version = "0.0.0"
+dependencies = [
+ "npt-sim",
+]
+
 [[package]]
 name = "npt-host"
 version = "0.0.0"
````

### npt-proto's manifest (crates/proto/Cargo.toml)
_`git diff --no-index .gate/u4d2-prefix/spikes/netcode-proof/crates/proto/Cargo.toml .claude/worktrees/u4d2-build/spikes/netcode-proof/crates/proto/Cargo.toml`:  1 file changed, 2 insertions(+), 2 deletions(-)._

````diff
diff --git a/.gate/u4d2-prefix/spikes/netcode-proof/crates/proto/Cargo.toml b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/proto/Cargo.toml
index af05788..53cdf0f 100644
--- a/.gate/u4d2-prefix/spikes/netcode-proof/crates/proto/Cargo.toml
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/proto/Cargo.toml
@@ -10,8 +10,8 @@ publish.workspace = true
 workspace = true
 
 [features]
-# The planted defects (plan §4.7, E19), bites builds only; npt-sim's too, since the wire grid's bites are chosen through
-# its config value (u4-build-plan.md §4.4, §4.11).
+# The planted defects (plan §4.7, E19), bites builds only; npt-sim's too, so a bites build of npt-proto compiles its
+# dependency's own bites in (after carry 2, npt-proto's bites are its own: u4-build-plan.md §4.4, §4.11).
 bites = ["npt-sim/bites"]
 
 [dependencies]
````

### npt-client-wasm's manifest (crates/client-wasm/Cargo.toml)
_`git diff --no-index .gate/u4d2-prefix/spikes/netcode-proof/crates/client-wasm/Cargo.toml .claude/worktrees/u4d2-build/spikes/netcode-proof/crates/client-wasm/Cargo.toml`:  1 file changed, 3 insertions(+), 2 deletions(-)._

````diff
diff --git a/.gate/u4d2-prefix/spikes/netcode-proof/crates/client-wasm/Cargo.toml b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/client-wasm/Cargo.toml
index 060807c..29fbc29 100644
--- a/.gate/u4d2-prefix/spikes/netcode-proof/crates/client-wasm/Cargo.toml
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/client-wasm/Cargo.toml
@@ -6,8 +6,9 @@ edition.workspace = true
 rust-version.workspace = true
 publish.workspace = true
 
-# Not the workspace lints: wasm-bindgen's generated code needs `unsafe` (one of the workspace's two unsafe_code
-# exceptions, with npt-tcpinfo's two functions; the workspace Cargo.toml's note), and the workspace forbids it. So
+# Not the workspace lints: wasm-bindgen's generated code needs `unsafe` (one of the workspace's three unsafe_code
+# exceptions, with npt-tcpinfo's two functions and npt-hash-wasm's seven exports, the third; the workspace Cargo.toml's
+# note), and the workspace forbids it. So
 # [lints.rust] allows it for this crate, and every other lint is the workspace's, key for key (check.mjs's deps step
 # checks both, and refuses a lint given in any form its reader does not read, such as a lint's own table or a
 # hyphenated name).
````

### npt-hash-wasm (crates/hash-wasm, new: Cargo.toml, src/lib.rs with its tests, loader.mjs, co-initial-dump.txt)
_`git diff --no-index .gate/u4e-empty .claude/worktrees/u4d2-build/spikes/netcode-proof/crates/hash-wasm`:  4 files changed, 1076 insertions(+)._

````diff
diff --git a/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/hash-wasm/Cargo.toml b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/hash-wasm/Cargo.toml
new file mode 100644
index 0000000..30d7463
--- /dev/null
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/hash-wasm/Cargo.toml
@@ -0,0 +1,34 @@
+[package]
+name = "npt-hash-wasm"
+description = "A check-only cdylib over npt-sim's replayer with integer-only exports, which Node runs for the native-against-Wasm check"
+version.workspace = true
+edition.workspace = true
+rust-version.workspace = true
+publish.workspace = true
+
+# Not `workspace = true`: the workspace forbids unsafe_code, which no #[allow] can relax (E0453), and edition 2024's
+# `#[unsafe(no_mangle)]` is unsafe code to that lint. So this crate has its own tables, as npt-tcpinfo has (the spike's
+# third unsafe exception, ADR 0002:264-265; the advisor's condition 7): [lints.rust] denies the lint, so each of the
+# seven exports in src/lib.rs can carry one #[allow(unsafe_code, reason = "...")], and nothing else in the crate relaxes
+# it. Every other lint is the workspace's, key for key. check.mjs's deps step checks the level, the tables against the
+# workspace's key for key (it refuses a lint given in any form its reader does not read: a lint's own
+# [lints.<tool>.<lint>] table, or a quoted, dotted, hyphenated or uppercase name), where the seven allows sit, and that
+# the exports take and give u32 or i32 only.
+[lints.rust]
+unsafe_code = "deny"
+
+[lints.clippy]
+disallowed_types = "deny"
+disallowed_methods = "deny"
+iter_over_hash_type = "deny"
+
+[lib]
+crate-type = ["cdylib"]
+
+[features]
+# The bites build (plan §4.7): npt-sim's planted defects, armed per instance through `begin` (u4-build-plan.md §5 U4d2).
+bites = ["npt-sim/bites"]
+
+[dependencies]
+# npt-sim only, and nothing depends on this crate (check.mjs's deps step); no dev dependency.
+npt-sim.workspace = true
diff --git a/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/hash-wasm/co-initial-dump.txt b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/hash-wasm/co-initial-dump.txt
new file mode 100644
index 0000000..01535f2
--- /dev/null
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/hash-wasm/co-initial-dump.txt
@@ -0,0 +1,266 @@
+# PM-0004-21's cross-target vector (u4-build-plan.md §5 U4d2, T22; ADR 0004:1142; u4d2-brief.md 2.6 h2): the canonical
+# words of CO's initial world, `World::new(&Scene::course(), 1, 0)` (D-7's form, sim/tests/replay.rs:326-337: 260
+# words), one word a line as 8 hex digits, and below this header their digest, the state hash (`StateDump::hash`,
+# FNV-1a-64 over the words' little-endian bytes: D-7's pin 12984630011248297851). npt-hash-wasm's unit test h2 (i)
+# checks it natively; check.mjs's `wasm` step checks it on wasm32 (h2 (ii)).
+digest b432ac332054c77b
+00000000
+00000000
+00000000
+00000001
+00000000
+00000000
+00000064
+c0400000
+c1900000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000258
+000186a0
+0000001e
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000001
+00000000
+00000001
+00000000
+00000001
+00000000
+000001f4
+00000001
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
+00000000
diff --git a/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/hash-wasm/loader.mjs b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/hash-wasm/loader.mjs
new file mode 100644
index 0000000..ac07a10
--- /dev/null
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/hash-wasm/loader.mjs
@@ -0,0 +1,239 @@
+#!/usr/bin/env node
+// npt-hash-wasm's Node loader (u4-build-plan.md §5 U4d2; u4d2-brief.md 2.2): runs one recording through the wasm32
+// build of npt-hash-wasm and writes its per-tick hashes in the line form npt-lab's legs write, for check.mjs's `wasm`
+// step to compare with the native legs (npt-lab's `hash compare`, HashCompare).
+//   node crates/hash-wasm/loader.mjs --wasm <file.wasm> --rec <file.rec> --name <recording> --first-tick <tick>
+//        --out <lines file> [--bite <index> --arm <tick>] [--dump-at <tick|end> --dump-out <dump file>]
+// - It lists the module's exports with WebAssembly.Module.exports and refuses, before instantiating, any name not on
+//   LOADER_EXPORTS (or of another kind), naming it, and any listed name the module lacks (the spike's third unsafe
+//   exception's test: the advisor's condition 7 (b)). It instantiates with an empty import object, so a module that
+//   imports anything fails (no host clock or random source can reach the Wasm leg).
+// - It streams the recording in: the file's bytes as little-endian 32-bit words, the last zero-padded, fed in chunks
+//   of 64 KiB with `step` called until NEED_MORE between them, never the whole file before the first step.
+// - Per tick it writes `h <recording> <tick> <hash>` (the hash as 16 hex digits, from hash_hi and hash_lo), the ticks
+//   numbered from --first-tick as the recording's header does (`step` gives a status only; the world's own tick is in
+//   the hashed state, so a leg that stepped other ticks mismatches by hash), and at the end `stepped <n>`.
+// - With --dump-at it writes the dump's words at that tick (or after the last, `end`) through dump_len and dump_word:
+//   `dump <recording> <tick|end>`, `hash <16 hex>`, then one word a line as 8 hex digits (npt-lab's dump form).
+// - A bite comes in as its index in World::BITES, from 1 (`npt-lab hash bites` lists them), with its arming tick.
+// - A trap (WebAssembly.RuntimeError: the wasm32 build aborts on a panic), a FAILED status, a refused `begin`, a file
+//   that ends inside a tick or a dump tick never stepped ends it with exit 1, naming the recording and the last tick
+//   stepped; a usage error exits 2. Each run is its own Node process (check.mjs runs it through process.execPath).
+
+import { closeSync, openSync, readFileSync, readSync, writeFileSync } from "node:fs";
+import { pathToFileURL } from "node:url";
+
+/** npt-hash-wasm's seven exports (crates/hash-wasm/src/lib.rs), each a function: the third exception's list. */
+export const HASH_WASM_EXPORTS = ["begin", "feed", "step", "hash_lo", "hash_hi", "dump_len", "dump_word"];
+/**
+ * The linker's own exports in the release build of a wasm32-unknown-unknown cdylib: `memory`, the module's linear
+ * memory, which wasm-ld exports by default (rustc 1.99.1's release build of npt-hash-wasm exports it and nothing else
+ * beside the seven; it imports nothing).
+ */
+export const LINKER_EXPORTS = { memory: "memory" };
+/** Every export the loader accepts, by name, with its kind. */
+export const LOADER_EXPORTS = { ...Object.fromEntries(HASH_WASM_EXPORTS.map((n) => [n, "function"])), ...LINKER_EXPORTS };
+
+/** `begin`'s results (crates/hash-wasm/src/lib.rs). */
+export const BEGIN = { BEGUN: 0, REFUSED: 1 };
+/** `step`'s results, one per npt_sim::replay::Step (crates/hash-wasm/src/lib.rs). */
+export const STEP = { NEED_MORE: 0, STEPPED: 1, DONE: 2, FAILED: 3 };
+/** The bytes read from the recording between steps. */
+const BLOCK_BYTES = 1 << 16;
+
+/**
+ * The loader's refusal of a compiled module, as sentences (empty when it may be instantiated): every export whose name
+ * is not on LOADER_EXPORTS, or whose kind is not the listed one, by name, and every listed name it lacks.
+ */
+export function moduleProblems(module) {
+  const problems = [];
+  const exports = WebAssembly.Module.exports(module);
+  for (const { name, kind } of exports) {
+    if (!Object.hasOwn(LOADER_EXPORTS, name)) problems.push(`the module exports ${JSON.stringify(name)} (a ${kind}), which is not on the loader's list`);
+    else if (LOADER_EXPORTS[name] !== kind) problems.push(`the module exports ${JSON.stringify(name)} as a ${kind}, not a ${LOADER_EXPORTS[name]}`);
+  }
+  for (const name of Object.keys(LOADER_EXPORTS)) {
+    if (!exports.some((e) => e.name === name)) problems.push(`the module does not export ${JSON.stringify(name)}`);
+  }
+  return problems;
+}
+
+/** The loader's refusal of a module's bytes (moduleProblems); bytes that are no WebAssembly module are refused too. */
+export function exportProblems(bytes) {
+  let module;
+  try {
+    module = new WebAssembly.Module(bytes);
+  } catch (e) {
+    return [`not a WebAssembly module: ${e.message}`];
+  }
+  return moduleProblems(module);
+}
+
+/**
+ * A module the loader accepts, instantiated with an empty import object: its exports. A module that imports anything
+ * fails to instantiate (a LinkError or TypeError, rethrown naming the imports).
+ */
+export function instantiate(module) {
+  const problems = moduleProblems(module);
+  if (problems.length) throw new Error(`refused: ${problems.join("; ")}`);
+  try {
+    return new WebAssembly.Instance(module, {}).exports;
+  } catch (e) {
+    const imports = WebAssembly.Module.imports(module).map((i) => `${i.module}.${i.name} (${i.kind})`);
+    throw new Error(`refused: the module does not instantiate with no imports (it imports ${imports.join(", ") || "nothing"}): ${e.message}`);
+  }
+}
+
+/**
+ * Calls `onChunk(words)` with the file's bytes as little-endian 32-bit words, BLOCK_BYTES at a time, the last word
+ * zero-padded; stops early when `onChunk` returns false.
+ */
+export function forEachChunk(path, onChunk) {
+  const fd = openSync(path, "r");
+  const buf = Buffer.alloc(BLOCK_BYTES + 3);
+  let carry = 0;
+  try {
+    for (;;) {
+      const n = readSync(fd, buf, carry, BLOCK_BYTES, null);
+      const have = carry + n;
+      if (n === 0) {
+        if (carry > 0) {
+          buf.fill(0, carry, 4);
+          onChunk([buf.readUInt32LE(0)]);
+        }
+        return;
+      }
+      const whole = have - (have % 4);
+      const words = new Array(whole / 4);
+      for (let i = 0; i < words.length; i++) words[i] = buf.readUInt32LE(i * 4);
+      carry = have - whole;
+      buf.copy(buf, 0, whole, have);
+      if (onChunk(words) === false) return;
+    }
+  } finally {
+    closeSync(fd);
+  }
+}
+
+const hex = (n, digits) => (n >>> 0).toString(16).padStart(digits, "0");
+
+/**
+ * One recording through an instance's exports: { ok, lines, stepped, dump, error }. `lines` are the per-tick lines and
+ * the closing `stepped <n>` (or `failed <why>`); `dump` the requested dump's text, if it was taken.
+ */
+export function replay(exports, { path, name, firstTick, bite = 0, arm = 0, dumpAt = null }) {
+  const lines = [];
+  let stepped = 0;
+  let status = STEP.NEED_MORE;
+  let dump = null;
+  let error = null;
+  const tickOf = (i) => (firstTick + i) >>> 0;
+  const last = () => (stepped ? `tick ${tickOf(stepped - 1)}` : "no tick");
+  const hash = () => `${hex(exports.hash_hi(), 8)}${hex(exports.hash_lo(), 8)}`;
+  const takeDump = (at) => {
+    const n = exports.dump_len() >>> 0;
+    const words = new Array(n);
+    for (let i = 0; i < n; i++) words[i] = `${hex(exports.dump_word(i), 8)}\n`;
+    return `dump ${name} ${at}\nhash ${hash()}\n${words.join("")}`;
+  };
+  const begun = exports.begin(bite, arm) >>> 0;
+  if (begun !== BEGIN.BEGUN) {
+    error = `${name}: begin(${bite}, ${arm}) returned ${begun}, not BEGUN: the module refused the bite (a measured build, or an index outside World::BITES)`;
+  } else {
+    try {
+      forEachChunk(path, (words) => {
+        for (const w of words) exports.feed(w);
+        for (;;) {
+          status = exports.step() >>> 0;
+          if (status !== STEP.STEPPED) break;
+          const tick = tickOf(stepped);
+          stepped++;
+          lines.push(`h ${name} ${tick} ${hash()}`);
+          if (dumpAt === tick) dump = takeDump(tick);
+        }
+        return status === STEP.NEED_MORE;
+      });
+      if (status === STEP.FAILED) error = `${name}: step() returned FAILED after ${last()}`;
+      else if (status === STEP.NEED_MORE) error = `${name}: the file ends inside a tick, after ${last()}`;
+      else if (status !== STEP.DONE) error = `${name}: step() returned ${status}, no status it gives, after ${last()}`;
+      else if (dumpAt === "end") dump = takeDump("end");
+      else if (dumpAt !== null && dump === null) error = `${name}: no tick ${dumpAt} was stepped, so no dump was taken (after ${last()})`;
+    } catch (e) {
+      if (!(e instanceof WebAssembly.RuntimeError)) throw e;
+      error = `${name}: the Wasm leg trapped after ${last()}: ${e.message}`;
+    }
+  }
+  lines.push(error ? `failed ${error}` : `stepped ${stepped}`);
+  return { ok: !error, lines, stepped, dump, error };
+}
+
+const USAGE =
+  "usage: node loader.mjs --wasm <file.wasm> --rec <file.rec> --name <recording> --first-tick <tick> --out <lines file> " +
+  "[--bite <index> --arm <tick>] [--dump-at <tick|end> --dump-out <dump file>]";
+
+/** The command's options: each `--name value` once. */
+export function parse(argv) {
+  const opts = {};
+  for (let i = 0; i < argv.length; i += 2) {
+    const flag = argv[i];
+    if (!flag.startsWith("--") || i + 1 >= argv.length) throw new Error(`${flag}: every option is --name value`);
+    const k = flag.slice(2);
+    if (Object.hasOwn(opts, k)) throw new Error(`${flag} given twice`);
+    opts[k] = argv[i + 1];
+  }
+  const known = ["wasm", "rec", "name", "first-tick", "out", "bite", "arm", "dump-at", "dump-out"];
+  for (const k of Object.keys(opts)) if (!known.includes(k)) throw new Error(`unknown option --${k}`);
+  for (const k of ["wasm", "rec", "name", "first-tick", "out"]) if (!opts[k]) throw new Error(`--${k} is required`);
+  const u32 = (k, v) => {
+    if (!/^\d+$/.test(v) || Number(v) > 0xffff_ffff) throw new Error(`--${k} ${JSON.stringify(v)} is not a whole number below 2^32`);
+    return Number(v);
+  };
+  if (("bite" in opts) !== ("arm" in opts)) throw new Error("--bite and --arm go together");
+  if (("dump-at" in opts) !== ("dump-out" in opts)) throw new Error("--dump-at and --dump-out go together");
+  const bite = "bite" in opts ? u32("bite", opts.bite) : 0;
+  if ("bite" in opts && bite === 0) throw new Error("--bite is World::BITES's index from 1");
+  return {
+    wasm: opts.wasm,
+    path: opts.rec,
+    name: opts.name,
+    firstTick: u32("first-tick", opts["first-tick"]),
+    out: opts.out,
+    bite,
+    arm: "arm" in opts ? u32("arm", opts.arm) : 0,
+    dumpAt: !("dump-at" in opts) ? null : opts["dump-at"] === "end" ? "end" : u32("dump-at", opts["dump-at"]),
+    dumpOut: opts["dump-out"] ?? null,
+  };
+}
+
+/** The command: 0 on a recording replayed, 1 on a refusal or a failed replay, 2 on a usage error. */
+export function main(argv) {
+  let o;
+  try {
+    o = parse(argv);
+  } catch (e) {
+    console.error(`loader: ${e.message}\n${USAGE}`);
+    return 2;
+  }
+  let exports;
+  try {
+    exports = instantiate(new WebAssembly.Module(readFileSync(o.wasm)));
+  } catch (e) {
+    console.error(`loader: ${o.wasm}: ${e.message}`);
+    return 1;
+  }
+  const r = replay(exports, o);
+  writeFileSync(o.out, `${r.lines.join("\n")}\n`);
+  if (r.dump !== null) writeFileSync(o.dumpOut, r.dump);
+  if (!r.ok) {
+    console.error(`loader: ${r.error}`);
+    return 1;
+  }
+  const armed = o.bite ? `, bite ${o.bite} armed at tick ${o.arm}` : "";
+  const dumped = r.dump !== null ? `, its dump at ${o.dumpAt} written` : "";
+  console.log(`wasm32 leg: ${o.name}: ${r.stepped} ticks stepped${armed}${dumped}`);
+  return 0;
+}
+
+if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
+  process.exitCode = main(process.argv.slice(2));
+}
diff --git a/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/hash-wasm/src/lib.rs b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/hash-wasm/src/lib.rs
new file mode 100644
index 0000000..b1a9bcf
--- /dev/null
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/hash-wasm/src/lib.rs
@@ -0,0 +1,537 @@
+//! npt-hash-wasm: npt-sim's one replay driver ([`npt_sim::replay::Replayer`]) behind seven integer-only exports, built
+//! as a wasm32 cdylib that Node runs for the native-against-Wasm hash check (u4-build-plan.md §5 U4d2; plan §4.5's
+//! determinism check). check.mjs's `wasm` step feeds a recording's words to the Wasm build through the Node loader
+//! beside this file (loader.mjs) and compares its per-tick hashes with npt-lab's native legs. Nothing depends on this
+//! crate; it depends on npt-sim alone; its `bites` feature forwards to npt-sim's.
+//!
+//! **The exports** (each `extern "C"`, every parameter and result a u32):
+//! - `begin(bite, arm_tick) -> u32`: drops any replay begun before and starts a fresh one: for `bite` 0 a plain
+//!   [`npt_sim::replay::Replayer::new`]; for `bite` k in 1..=4 (bites builds only) `Replayer::with_bite` of
+//!   `World::BITES[k - 1]`, armed at `arm_tick`. Returns `BEGUN` (0), or `REFUSED` (1) for a k outside 1..=4 or any
+//!   k above 0 in the measured build, and then changes nothing (a replay begun before stays as it was). The scene and
+//!   the seed come from the recording's header, the one reader of which is npt-sim's replay.rs.
+//! - `feed(word)`: the recording's next 4 bytes, little-endian. Before any `begin` it does nothing.
+//! - `step() -> u32`: runs the next tick if the words fed hold it whole: `NEED_MORE` (0), `STEPPED` (1), `DONE` (2)
+//!   or `FAILED` (3), the four [`npt_sim::replay::Step`]s. Before any `begin` it returns `FAILED` and changes nothing.
+//!   At `NEED_MORE`, feeding more words and stepping again goes on; after `DONE` it returns `DONE` again; after
+//!   `FAILED` it returns `FAILED` again (the replayer's own rule), until the next `begin`.
+//! - `hash_lo() -> u32`, `hash_hi() -> u32`: the low and the high 32 bits of the last tick's state hash; 0 before the
+//!   header has been read (no world yet) and before any `begin`.
+//! - `dump_len() -> u32`: takes the last tick's dump once, keeps it, and returns its length in words; 0 (and an empty
+//!   kept dump) before the header and before any `begin`. `dump_word(i) -> u32`: word i of the dump `dump_len` kept;
+//!   0 at or past its end. A `begin` empties the kept dump.
+//!
+//! The state lives in a `thread_local!` cell, never a `static mut`: on wasm32 (no atomics) it is one static; natively
+//! each test thread has its own, so the unit tests share none.
+//!
+//! **The spike's third `unsafe` exception** (the first two are npt-client-wasm's and npt-tcpinfo's), granted by the
+//! advisor under ADR 0002:264-265's rule: "an exception needs a written invariant, a test and an `#[allow]` with its
+//! reason" (the advisor's condition 7; plan.md §4.2). In edition 2024 an export's `#[no_mangle]` takes the
+//! `#[unsafe(no_mangle)]` form, which the unsafe_code lint fires on; the workspace forbids that lint, which no allow can
+//! relax, so this crate's manifest has its own lint tables (unsafe_code denied, every other lint the workspace's), and
+//! each of the seven exports carries one `#[allow(unsafe_code, reason = "...")]` directly on it. No `unsafe` block,
+//! unsafe fn, impl, trait or extern block exists here.
+//! - **The written invariant**, which each allow's reason repeats in short: no exported name collides with another
+//!   symbol, and every export has integer signatures only (u32 parameters and results).
+//! - **What it was checked against** (the advisor's U4D2-Q8 ruling): on wasm32-unknown-unknown, the module's only other
+//!   exports are the linker's (loader.mjs lists them); natively, this container's glibc 2.39 has one symbol of these
+//!   names, the compat `step@GLIBC_2.2.5` (`nm -D`), a non-default version that no new link binds, and the native
+//!   build is never loaded as a library (its unit tests call the functions in-process).
+//! - **The test**: loader.mjs lists the module's exports with `WebAssembly.Module.exports` and refuses, before
+//!   instantiating, any name not on its list (check.test.mjs pins the list and the refusal); check.mjs's deps step
+//!   refuses an export whose parameters or result are not u32 or i32 (the advisor's U4D2-Q7 ruling), any `unsafe` but
+//!   a listed export's `#[unsafe(no_mangle)]`, and any relaxing of the lint but the seven placed allows.
+
+use std::cell::RefCell;
+
+use npt_sim::replay::{Replayer, Step};
+
+/// `begin`'s results.
+const BEGUN: u32 = 0;
+const REFUSED: u32 = 1;
+/// `step`'s results, one per [`Step`].
+const NEED_MORE: u32 = 0;
+const STEPPED: u32 = 1;
+const DONE: u32 = 2;
+const FAILED: u32 = 3;
+
+/// The replay begun last, and the dump `dump_len` kept.
+struct State {
+    replayer: Option<Replayer>,
+    dump: Vec<u32>,
+}
+
+thread_local! {
+    static STATE: RefCell<State> = const {
+        RefCell::new(State {
+            replayer: None,
+            dump: Vec::new(),
+        })
+    };
+}
+
+/// The replayer `begin` makes for `bite` (0: none; k: `World::BITES[k - 1]`), or `None` to refuse it.
+fn replayer_for(bite: u32, arm_tick: u32) -> Option<Replayer> {
+    if bite == 0 {
+        return Some(Replayer::new());
+    }
+    with_bite(bite, arm_tick)
+}
+
+#[cfg(feature = "bites")]
+fn with_bite(bite: u32, arm_tick: u32) -> Option<Replayer> {
+    let k = usize::try_from(bite).ok()?.checked_sub(1)?;
+    let id = npt_sim::World::BITES.get(k)?;
+    Replayer::with_bite(id, arm_tick)
+}
+
+#[cfg(not(feature = "bites"))]
+fn with_bite(_bite: u32, _arm_tick: u32) -> Option<Replayer> {
+    None
+}
+
+/// Starts a fresh replay, with no bite (0) or `World::BITES[bite - 1]` armed at `arm_tick` (bites builds only).
+#[allow(
+    unsafe_code,
+    reason = "the third exception (ADR 0002:264-265): #[unsafe(no_mangle)] exports this listed name; no exported name \
+              collides with another symbol, and every export has integer signatures only"
+)]
+#[unsafe(no_mangle)]
+pub extern "C" fn begin(bite: u32, arm_tick: u32) -> u32 {
+    let Some(replayer) = replayer_for(bite, arm_tick) else {
+        return REFUSED;
+    };
+    STATE.with_borrow_mut(|s| {
+        s.replayer = Some(replayer);
+        s.dump.clear();
+    });
+    BEGUN
+}
+
+/// Feeds the recording's next 4 bytes, little-endian.
+#[allow(
+    unsafe_code,
+    reason = "the third exception (ADR 0002:264-265): #[unsafe(no_mangle)] exports this listed name; no exported name \
+              collides with another symbol, and every export has integer signatures only"
+)]
+#[unsafe(no_mangle)]
+pub extern "C" fn feed(word: u32) {
+    STATE.with_borrow_mut(|s| {
+        if let Some(r) = &mut s.replayer {
+            r.feed(word);
+        }
+    });
+}
+
+/// Runs the next tick if its words are whole: `NEED_MORE`, `STEPPED`, `DONE` or `FAILED`.
+#[allow(
+    unsafe_code,
+    reason = "the third exception (ADR 0002:264-265): #[unsafe(no_mangle)] exports this listed name; no exported name \
+              collides with another symbol, and every export has integer signatures only"
+)]
+#[unsafe(no_mangle)]
+pub extern "C" fn step() -> u32 {
+    STATE.with_borrow_mut(|s| match s.replayer.as_mut().map(Replayer::step) {
+        None | Some(Step::Failed(_)) => FAILED,
+        Some(Step::NeedMore) => NEED_MORE,
+        Some(Step::Stepped(_)) => STEPPED,
+        Some(Step::Done) => DONE,
+    })
+}
+
+/// The last tick's state hash, `shift` bits down, as a u32 (0 before the header).
+fn hash_bits(shift: u32) -> u32 {
+    STATE.with_borrow(|s| {
+        let h = s
+            .replayer
+            .as_ref()
+            .and_then(Replayer::state_hash)
+            .unwrap_or(0);
+        (h >> shift) as u32
+    })
+}
+
+/// The low 32 bits of the last tick's state hash.
+#[allow(
+    unsafe_code,
+    reason = "the third exception (ADR 0002:264-265): #[unsafe(no_mangle)] exports this listed name; no exported name \
+              collides with another symbol, and every export has integer signatures only"
+)]
+#[unsafe(no_mangle)]
+pub extern "C" fn hash_lo() -> u32 {
+    hash_bits(0)
+}
+
+/// The high 32 bits of the last tick's state hash.
+#[allow(
+    unsafe_code,
+    reason = "the third exception (ADR 0002:264-265): #[unsafe(no_mangle)] exports this listed name; no exported name \
+              collides with another symbol, and every export has integer signatures only"
+)]
+#[unsafe(no_mangle)]
+pub extern "C" fn hash_hi() -> u32 {
+    hash_bits(32)
+}
+
+/// Takes the last tick's dump, keeps it, and returns its length in words.
+#[allow(
+    unsafe_code,
+    reason = "the third exception (ADR 0002:264-265): #[unsafe(no_mangle)] exports this listed name; no exported name \
+              collides with another symbol, and every export has integer signatures only"
+)]
+#[unsafe(no_mangle)]
+pub extern "C" fn dump_len() -> u32 {
+    STATE.with_borrow_mut(|s| {
+        s.dump = s
+            .replayer
+            .as_ref()
+            .and_then(Replayer::dump)
+            .map(|d| d.words().to_vec())
+            .unwrap_or_default();
+        u32::try_from(s.dump.len()).unwrap_or(u32::MAX)
+    })
+}
+
+/// Word `i` of the dump `dump_len` kept; 0 at or past its end.
+#[allow(
+    unsafe_code,
+    reason = "the third exception (ADR 0002:264-265): #[unsafe(no_mangle)] exports this listed name; no exported name \
+              collides with another symbol, and every export has integer signatures only"
+)]
+#[unsafe(no_mangle)]
+pub extern "C" fn dump_word(i: u32) -> u32 {
+    STATE.with_borrow(|s| {
+        usize::try_from(i)
+            .ok()
+            .and_then(|i| s.dump.get(i).copied())
+            .unwrap_or(0)
+    })
+}
+
+#[cfg(test)]
+mod tests {
+    //! h2 (i), PM-0004-21's committed vector natively, and h3, the exports against the replayer they wrap
+    //! (u4-build-plan.md §5 U4d2; u4d2-brief.md 2.6). Inline, so check.mjs's bite-home scan reads these test-only ids
+    //! as tests.
+
+    use super::*;
+    use npt_sim::replay::Recording;
+    use npt_sim::{Scene, SimCore, World};
+
+    /// The committed vector: CO's initial world (`World::new(&Scene::course(), 1, 0)`, D-7's form), one word a line
+    /// as 8 hex digits, below a `digest` line holding its state hash as 16 hex digits.
+    const VECTOR: &str = include_str!("../co-initial-dump.txt");
+
+    /// The vector file's digest and words.
+    fn vector() -> (u64, Vec<u32>) {
+        let mut digest = None;
+        let mut words = Vec::new();
+        for line in VECTOR.lines().map(str::trim) {
+            if line.is_empty() || line.starts_with('#') {
+                continue;
+            }
+            if let Some(d) = line.strip_prefix("digest ") {
+                digest = Some(u64::from_str_radix(d, 16).expect("16 hex digits"));
+            } else {
+                assert_eq!(line.len(), 8, "a word is 8 hex digits: {line:?}");
+                words.push(u32::from_str_radix(line, 16).expect("8 hex digits"));
+            }
+        }
+        (digest.expect("a digest line"), words)
+    }
+
+    /// Bytes as little-endian 32-bit words, the last zero-padded (sim/tests/replay.rs's `words`).
+    fn le_words(bytes: &[u8]) -> Vec<u32> {
+        bytes
+            .chunks(4)
+            .map(|c| {
+                let mut w = [0u8; 4];
+                w[..c.len()].copy_from_slice(c);
+                u32::from_le_bytes(w)
+            })
+            .collect()
+    }
+
+    /// The exports' hash, as one u64.
+    fn export_hash() -> u64 {
+        u64::from(hash_hi()) << 32 | u64::from(hash_lo())
+    }
+
+    /// The exports' dump, through `dump_len` and `dump_word`.
+    fn export_dump() -> Vec<u32> {
+        (0..dump_len()).map(|i| dump_word(i)).collect()
+    }
+
+    /// TC's short recording (sim/tests/replay.rs:514-515): 60 ticks across the u32 wrap.
+    fn short() -> Recording {
+        Recording::synthetic(&Scene::targets(), 8, u32::MAX - 20, 60, 2_000)
+    }
+
+    /// A recording with no tick: its header alone, so `step` builds the world and returns `DONE`.
+    fn header_only(scene: &Scene) -> Vec<u32> {
+        let rec = Recording {
+            scene: scene.id(),
+            seed: 1,
+            first_tick: 0,
+            span_ms: 2_000,
+            generator: 0,
+            ticks: Vec::new(),
+        };
+        le_words(&rec.encode())
+    }
+
+    /// Feeds `words` through the exports in chunks of `chunk`, stepping until `NEED_MORE` between them, and returns
+    /// each stepped tick's hash and dump, and the last status.
+    fn run_exports(words: &[u32], chunk: usize) -> (Vec<(u64, Vec<u32>)>, u32) {
+        let mut out = Vec::new();
+        let mut last = NEED_MORE;
+        for part in words.chunks(chunk) {
+            for w in part {
+                feed(*w);
+            }
+            loop {
+                last = step();
+                match last {
+                    STEPPED => out.push((export_hash(), export_dump())),
+                    NEED_MORE | DONE | FAILED => break,
+                    other => panic!("a status step does not give: {other}"),
+                }
+            }
+        }
+        (out, last)
+    }
+
+    /// The same through a `Replayer` fed every word first.
+    fn run_reference(mut r: Replayer, words: &[u32]) -> Vec<(u64, Vec<u32>)> {
+        for w in words {
+            r.feed(*w);
+        }
+        let mut out = Vec::new();
+        while let Step::Stepped(_) = r.step() {
+            out.push((
+                r.state_hash().expect("a world"),
+                r.dump().expect("a world").words().to_vec(),
+            ));
+        }
+        out
+    }
+
+    #[test]
+    fn h2_cos_initial_dump_is_the_committed_vector_natively_and_through_the_exports() {
+        let (digest, words) = vector();
+        let world = World::new(&Scene::course(), 1, 0);
+        let d = world.dump();
+        let shown: Vec<String> = d.words().iter().map(|w| format!("{w:08x}")).collect();
+        // The first word that differs, named by its field (h2's shown failure names the word: U4D2-Q11).
+        let n = d.words().len().max(words.len());
+        if let Some(i) = (0..n).find(|&i| d.words().get(i) != words.get(i)) {
+            let show = |w: Option<&u32>| w.map_or("none".to_owned(), |w| format!("{w:08x}"));
+            panic!(
+                "the committed vector differs at word {i} ({}): {} committed, {} built natively ({} words built, {} \
+                 committed); the file should read: digest {:016x}, then\n{}",
+                d.field(i).unwrap_or_else(|| "end".to_owned()),
+                show(words.get(i)),
+                show(d.words().get(i)),
+                d.words().len(),
+                words.len(),
+                d.hash(),
+                shown.join("\n")
+            );
+        }
+        assert_eq!(
+            (d.hash(), d.words()),
+            (digest, words.as_slice()),
+            "the committed file should read: digest {:016x}, then\n{}",
+            d.hash(),
+            shown.join("\n")
+        );
+        assert_eq!(world.state_hash(), digest);
+        assert_eq!(words.len(), 260, "D-7's 260 words");
+        // Through the exports, from a header-only recording.
+        assert_eq!(begin(0, 0), BEGUN);
+        let (stepped, last) = run_exports(&header_only(&Scene::course()), 5);
+        assert!(stepped.is_empty());
+        assert_eq!(last, DONE);
+        assert_eq!((export_hash(), export_dump()), (digest, words));
+    }
+
+    #[test]
+    fn h3_the_exports_give_the_replayers_own_hashes_and_dumps_tick_by_tick() {
+        let words = le_words(&short().encode());
+        let reference = run_reference(Replayer::new(), &words);
+        assert_eq!(reference.len(), 60);
+        // In chunks smaller than a tick and larger than many, so NEED_MORE falls inside ticks and between them.
+        for chunk in [7, 1_000, words.len()] {
+            assert_eq!(begin(0, 0), BEGUN);
+            let (got, last) = run_exports(&words, chunk);
+            assert_eq!(last, DONE, "chunk {chunk}");
+            assert_eq!(got, reference, "chunk {chunk}");
+            assert_eq!(step(), DONE, "DONE again after DONE");
+            assert_eq!(export_hash(), reference[59].0, "the last tick's hash stays");
+        }
+    }
+
+    #[test]
+    fn h3_each_call_before_begin_at_need_more_after_done_and_after_failed() {
+        // Before any begin: nothing to step, no hash, no dump; feed does nothing.
+        feed(1);
+        assert_eq!(step(), FAILED);
+        assert_eq!(
+            (hash_lo(), hash_hi(), dump_len(), dump_word(0)),
+            (0, 0, 0, 0)
+        );
+        // Begun, nothing fed (the word above went nowhere): NEED_MORE, and still no world.
+        assert_eq!(begin(0, 0), BEGUN);
+        assert_eq!(step(), NEED_MORE);
+        let words = le_words(&short().encode());
+        for w in &words[..10] {
+            feed(*w);
+        }
+        assert_eq!(step(), NEED_MORE, "10 of the header's 15 words");
+        assert_eq!((hash_lo(), hash_hi(), dump_len()), (0, 0, 0));
+        // The rest: every tick, then DONE, and DONE again.
+        for w in &words[10..] {
+            feed(*w);
+        }
+        let mut stepped = 0;
+        loop {
+            match step() {
+                STEPPED => stepped += 1,
+                DONE => break,
+                other => panic!("status {other}"),
+            }
+        }
+        assert_eq!(stepped, 60);
+        assert_eq!(step(), DONE);
+        // A header that is no recording: FAILED, again FAILED, and more words change nothing; begin starts afresh.
+        assert_eq!(begin(0, 0), BEGUN);
+        assert_eq!(dump_len(), 0, "begin empties the kept dump");
+        for _ in 0..15 {
+            feed(0xdead_beef);
+        }
+        assert_eq!(step(), FAILED);
+        assert_eq!(step(), FAILED);
+        for w in &words {
+            feed(*w);
+        }
+        assert_eq!(step(), FAILED, "after FAILED, FAILED until the next begin");
+        assert_eq!(begin(0, 0), BEGUN);
+        for w in &words {
+            feed(*w);
+        }
+        assert_eq!(step(), STEPPED);
+    }
+
+    #[test]
+    fn h3_dump_word_past_dump_len_is_0_and_dump_len_keeps_one_dump() {
+        assert_eq!(begin(0, 0), BEGUN);
+        for w in header_only(&Scene::targets()) {
+            feed(w);
+        }
+        assert_eq!(step(), DONE);
+        let n = dump_len();
+        assert_eq!(n, 340, "TC's initial dump: D-7's 340 words");
+        assert_ne!(dump_word(n - 1), 0xffff_ffff);
+        assert_eq!(
+            (dump_word(n), dump_word(n + 1), dump_word(u32::MAX)),
+            (0, 0, 0)
+        );
+        let first = World::new(&Scene::targets(), 1, 0).dump();
+        assert_eq!(export_dump(), first.words());
+    }
+
+    #[test]
+    fn h3_begin_refuses_an_index_outside_world_bites_and_changes_nothing() {
+        let words = le_words(&short().encode());
+        assert_eq!(begin(0, 0), BEGUN);
+        for w in &words {
+            feed(*w);
+        }
+        assert_eq!(step(), STEPPED);
+        let before = export_hash();
+        #[cfg(feature = "bites")]
+        let refused: Vec<u32> = vec![u32::try_from(World::BITES.len()).unwrap() + 1, 99, u32::MAX];
+        #[cfg(not(feature = "bites"))]
+        let refused: Vec<u32> = vec![1, 2, 3, 4, 5, u32::MAX];
+        for k in refused {
+            assert_eq!(begin(k, 0), REFUSED, "bite {k}");
+            assert_eq!(
+                export_hash(),
+                before,
+                "a refused begin keeps the replay begun before"
+            );
+        }
+        assert_eq!(step(), STEPPED, "the replay goes on");
+    }
+
+    #[cfg(feature = "bites")]
+    #[test]
+    fn h3_every_index_of_world_bites_begins_in_the_bites_build() {
+        for k in 1..=World::BITES.len() {
+            assert_eq!(begin(u32::try_from(k).unwrap(), 0), BEGUN, "bite {k}");
+        }
+    }
+
+    #[cfg(feature = "bites")]
+    #[test]
+    fn h3_an_armed_hash_map_index_matches_a_reference_replayer_less_the_folded_words() {
+        let id = "PM-0004-21/hash-map";
+        let k = World::BITES
+            .iter()
+            .position(|b| *b == id)
+            .expect("World plants it")
+            + 1;
+        let rec = short();
+        let arm = rec.first_tick.wrapping_add(30);
+        let words = le_words(&rec.encode());
+        let fold = "bite[0].hash_map_order";
+        let mut reference = Replayer::with_bite(id, arm).expect("a bite World plants");
+        for w in &words {
+            reference.feed(*w);
+        }
+        assert_eq!(begin(u32::try_from(k).unwrap(), arm), BEGUN);
+        for w in &words {
+            feed(*w);
+        }
+        let mut armed_ticks = 0;
+        loop {
+            let (a, b) = (reference.step(), step());
+            let Step::Stepped(tick) = a else {
+                assert_eq!((a, b), (Step::Done, DONE));
+                break;
+            };
+            assert_eq!(b, STEPPED, "tick {tick}");
+            let ours = STATE.with_borrow(|s| {
+                s.replayer
+                    .as_ref()
+                    .and_then(Replayer::world)
+                    .map(World::bite_order_digest)
+            });
+            let theirs = reference.world().map(World::bite_order_digest);
+            // `bite_order_digest` is None before the arming tick and Some from it, on both.
+            let from_arm = tick.wrapping_sub(rec.first_tick) >= 30;
+            assert_eq!(
+                ours.flatten().is_some(),
+                from_arm,
+                "the export's, tick {tick}"
+            );
+            assert_eq!(
+                theirs.flatten().is_some(),
+                from_arm,
+                "the reference's, tick {tick}"
+            );
+            armed_ticks += u32::from(from_arm);
+            // The two HashMaps take different RandomState keys: the dumps are compared less the folded words.
+            let r = reference.dump().expect("a world");
+            let keep: Vec<usize> = (0..r.words().len())
+                .filter(|&i| r.field(i).as_deref() != Some(fold))
+                .collect();
+            assert_eq!(r.words().len() - keep.len(), if from_arm { 2 } else { 0 });
+            let mine = export_dump();
+            assert_eq!(mine.len(), r.words().len(), "tick {tick}");
+            let less = |w: &[u32]| keep.iter().map(|&i| w[i]).collect::<Vec<u32>>();
+            assert_eq!(less(&mine), less(r.words()), "tick {tick}");
+        }
+        assert_eq!(armed_ticks, 30);
+    }
+}
````

### npt-lab (crates/lab: src with its tests, and the new clippy.toml, recordings.json and std-sin-scan.txt)
_`git diff --no-index .gate/u4d2-prefix/spikes/netcode-proof/crates/lab .claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab`:  7 files changed, 1796 insertions(+), 9 deletions(-)._

````diff
diff --git a/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab/clippy.toml b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab/clippy.toml
new file mode 100644
index 0000000..2354b05
--- /dev/null
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab/clippy.toml
@@ -0,0 +1,16 @@
+# npt-lab's clock and hash-map bans (u4-build-plan.md §4.11; the advisor's Q9 (a) and U4D2-Q10 rulings): the probe runs
+# on simulated time, and the `hash` subcommand's comparer and native legs take no order from a hash map and read no
+# clock (check.mjs's `wasm` step times its calls itself). The clock ban is npt-server's, as its clippy.toml words it;
+# the two type bans are npt-sim's. The workspace sets disallowed_types and disallowed_methods to deny.
+disallowed-types = [
+    { path = "std::collections::HashMap", reason = "hash-map ban (GLOSSARY §7): iteration order is not deterministic" },
+    { path = "std::collections::HashSet", reason = "hash-map ban (GLOSSARY §7): iteration order is not deterministic" },
+    { path = "std::time::Instant", reason = "the clock ban: time is passed in" },
+    { path = "std::time::SystemTime", reason = "the clock ban: time is passed in" },
+    { path = "tokio::time::Instant", reason = "the clock ban: time is passed in", allow-invalid = true },
+]
+disallowed-methods = [
+    { path = "std::time::Instant::now", reason = "the clock ban" },
+    { path = "std::time::SystemTime::now", reason = "the clock ban" },
+    { path = "tokio::time::Instant::now", reason = "the clock ban", allow-invalid = true },
+]
diff --git a/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab/recordings.json b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab/recordings.json
new file mode 100644
index 0000000..2ff8533
--- /dev/null
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab/recordings.json
@@ -0,0 +1,8 @@
+{
+  "about": "The recordings manifest (u4-build-plan.md §4.15; u4d2-brief.md 2.3; carry 52): the three synthetic recordings of D-2 (sim/tests/replay.rs:21-29), each regenerated by `npt-lab hash` with Recording::synthetic(scene, seed, first_tick, ticks, span_ms) under Recording::GENERATOR 2 (U4d-F, adb1bab) into the checkout's .gate/ (never committed) and checked against its byte length and SHA-256 here before any leg runs. An entry whose generator is not Recording::GENERATOR is refused before anything is generated. Scene ids: 1 SS, 2 ES, 3 CO, 4 TC (Scene::by_id).",
+  "recordings": [
+    {"name": "SS-s1", "scene": 1, "seed": 1, "first_tick": 0, "ticks": 9000, "span_ms": 2000, "generator": 2, "bytes": 16694921, "sha256": "ab3cc8423ed5f373c2539a31d900c83dad30ffe909508dbff2f3032424001458"},
+    {"name": "ES-s2", "scene": 2, "seed": 2, "first_tick": 1000000, "ticks": 9000, "span_ms": 2000, "generator": 2, "bytes": 16700849, "sha256": "e3257dbd2972c1caed67f53f5697985de24d7c77f7c74c0568f563963899e0c8"},
+    {"name": "TC-s3", "scene": 4, "seed": 3, "first_tick": 4294962796, "ticks": 9000, "span_ms": 2000, "generator": 2, "bytes": 799693, "sha256": "c018b6503adf5836767b498177aa0537532bbafdce2acbb8f7dccd9675adbfc5"}
+  ]
+}
diff --git a/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab/src/hash.rs b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab/src/hash.rs
new file mode 100644
index 0000000..36d761a
--- /dev/null
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab/src/hash.rs
@@ -0,0 +1,1412 @@
+//! npt-lab's `hash` subcommand (u4-build-plan.md §5 U4d2, §4.15; plan §4.5's determinism check): the recordings
+//! manifest regenerated and checked, two native legs in two processes, and the comparisons check.mjs's `wasm` step
+//! makes with the Node loader's Wasm legs (crates/hash-wasm/loader.mjs). It reads no clock and uses no hash map
+//! (crates/lab/clippy.toml bans both).
+//!
+//! ```text
+//! npt-lab hash --manifest <file> --dir <folder> --out <folder> [--only <recording>]
+//!              [--bite <id> --arm <tick> [--on <recording>]] [--dump-at <tick>]
+//! npt-lab hash compare --ours <lines[,lines...]> --theirs <lines[,lines...]> --manifest <file> --dir <folder>
+//!              [--ours-dump <file>] [--theirs-dump <file>] [--digest-field <field>]
+//!              [--ours-name <name>] [--theirs-name <name>] [--bite <id> --arm <tick> [--on <recording>]]
+//! npt-lab hash vectors --dir <folder> --dumps <folder> --committed <file>
+//! npt-lab hash bites
+//! ```
+//! - The first form writes each scene's header-only recording into `--dir` (`vector-<scene>.rec`: seed 1, first tick
+//!   0, no tick), for h2, and says so at once; then regenerates every manifest entry with `Recording::synthetic` there
+//!   (`<name>.rec`), failing on any length or SHA-256 that is not the manifest's, and refusing, before anything is
+//!   generated, an entry whose generator is not `Recording::GENERATOR` (carry 52); then runs two native legs, this
+//!   binary re-run through `std::env::current_exe()` (D-2's form, sim/tests/replay.rs:144-171), each replaying the
+//!   recordings (or `--only` one) through `Replayer` in lock step, and compares leg 1 with leg 2 by [`HashCompare`].
+//!   Each leg's per-tick lines go to `--out` (`native-1.lines`, `native-2.lines`: `h <recording> <tick> <hash>`). At
+//!   the first tick whose hashes differ, leg 2's dump goes to leg 1, which names the first field that differs
+//!   (`StateDump::first_difference`). With a bite (bites builds only) the recording `--on` (default: the first
+//!   replayed) runs with it armed at `--arm`, and each leg writes its order digest once armed; with `--dump-at`, each
+//!   leg's dump at that tick of that recording goes to `--out` (`native-1.dump`, `native-2.dump`).
+//! - `compare` compares two lists by [`HashCompare`], each read from its lines files (`--ours` and `--theirs` each
+//!   name one file or several, comma-separated, read in order: the loader writes one per recording), each recording's
+//!   tick count checked against the manifest. At the first mismatch it replays that recording natively to that tick in
+//!   this process (with the bite, if given)
+//!   and names the first differing field: against its own dump by `StateDump::first_difference` when `--ours-dump` is
+//!   not given, or between the two dump files given, word by word, named by its own dump's `StateDump::field`.
+//!   `--digest-field` prints each given dump's words under that field (the hash-map bite's folded order).
+//! - `vectors` compares each scene's initial dump and hash on wasm32 (the loader's `vector-<scene>.dump` files in
+//!   `--dumps`) with this build's, and CO's with the committed vector file (h2 (ii)).
+//! - `bites` prints `World::BITES` in order, numbered from 1 as npt-hash-wasm's `begin` takes them (bites builds only).
+//!
+//! Exit 0 when everything compared is equal, 1 on a mismatch or a failed run, 2 on a usage error (a bite asked of the
+//! measured build among them).
+
+use std::io::{BufRead, BufReader, Write};
+use std::path::{Path, PathBuf};
+use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
+
+use npt_lab::{HashCompare, TickHash};
+use npt_sim::replay::{Recording, Replayer, Step};
+use npt_sim::{FieldDiff, Scene, StateDump};
+use serde_json::Value;
+use sha2::{Digest, Sha256};
+
+pub(crate) const USAGE: &str = "npt-lab hash --manifest <file> --dir <folder> --out <folder> [--only <recording>] \
+[--bite <id> --arm <tick> [--on <recording>]] [--dump-at <tick>]\n       npt-lab hash compare --ours <lines[,lines...]> \
+--theirs <lines[,lines...]> --manifest <file> --dir <folder> [--ours-dump <file>] [--theirs-dump <file>] [--digest-field <field>] \
+[--ours-name <name>] [--theirs-name <name>] [--bite <id> --arm <tick> [--on <recording>]]\n       npt-lab hash vectors \
+--dir <folder> --dumps <folder> --committed <file>\n       npt-lab hash bites";
+
+/// Set in a leg's environment: its number, 1 or 2.
+const LEG: &str = "NPT_LAB_HASH_LEG";
+/// Words fed to a replayer between steps: the file streams in, never whole before the first step.
+const CHUNK_WORDS: usize = 16_384;
+/// The header-only recordings' ring span (the manifest's recordings' 2 s).
+const VECTOR_SPAN_MS: u32 = 2_000;
+
+/// How a run ended: `Usage` exits 2, `Failed` 1.
+#[derive(Debug, PartialEq, Eq)]
+pub(crate) enum Failure {
+    Usage(String),
+    Failed(String),
+}
+
+/// A planted defect armed on one recording (bites builds only).
+#[derive(Clone, Debug, PartialEq, Eq)]
+pub(crate) struct Arm {
+    id: String,
+    tick: u32,
+    on: Option<String>,
+}
+
+#[derive(Debug, PartialEq, Eq)]
+pub(crate) enum Mode {
+    Legs {
+        manifest: String,
+        dir: String,
+        out: String,
+        only: Option<String>,
+        arm: Option<Arm>,
+        dump_at: Option<u32>,
+    },
+    Compare {
+        ours: String,
+        theirs: String,
+        manifest: String,
+        dir: String,
+        ours_dump: Option<String>,
+        theirs_dump: Option<String>,
+        digest_field: Option<String>,
+        names: [String; 2],
+        arm: Option<Arm>,
+    },
+    Vectors {
+        dir: String,
+        dumps: String,
+        committed: String,
+    },
+    Bites,
+}
+
+/// The arguments after `hash`: an optional mode word, then `--name value` pairs, each once.
+pub(crate) fn parse(argv: &[String]) -> Result<Mode, String> {
+    let (mode, rest) = match argv.first().map(String::as_str) {
+        Some(m @ ("compare" | "vectors" | "bites")) => (m, &argv[1..]),
+        _ => ("legs", argv),
+    };
+    let mut map: Vec<(&str, &str)> = Vec::new();
+    let mut it = rest.iter();
+    while let Some(flag) = it.next() {
+        let name = flag
+            .strip_prefix("--")
+            .ok_or(format!("unexpected argument {flag:?}"))?;
+        let value = it.next().ok_or(format!("{flag} needs a value"))?;
+        if map.iter().any(|(k, _)| *k == name) {
+            return Err(format!("{flag} given twice"));
+        }
+        map.push((name, value));
+    }
+    let mut take = |k: &str| {
+        let at = map.iter().position(|(n, _)| *n == k)?;
+        Some(map.remove(at).1.to_owned())
+    };
+    let need = |v: Option<String>, k: &str| v.ok_or(format!("--{k} is required"));
+    let tick = |v: Option<String>, k: &str| -> Result<Option<u32>, String> {
+        v.map(|s| {
+            s.parse::<u32>()
+                .map_err(|_| format!("--{k} {s:?} is not a tick (0 to 4294967295)"))
+        })
+        .transpose()
+    };
+    let parsed = match mode {
+        "bites" => Mode::Bites,
+        "vectors" => Mode::Vectors {
+            dir: need(take("dir"), "dir")?,
+            dumps: need(take("dumps"), "dumps")?,
+            committed: need(take("committed"), "committed")?,
+        },
+        _ => {
+            let bite = take("bite");
+            let arm_tick = tick(take("arm"), "arm")?;
+            let on = take("on");
+            let arm = match (bite, arm_tick) {
+                (Some(id), Some(tick)) => Some(Arm { id, tick, on }),
+                (None, None) if on.is_none() => None,
+                _ => return Err("--bite and --arm go together (and --on needs them)".to_owned()),
+            };
+            if mode == "compare" {
+                Mode::Compare {
+                    ours: need(take("ours"), "ours")?,
+                    theirs: need(take("theirs"), "theirs")?,
+                    manifest: need(take("manifest"), "manifest")?,
+                    dir: need(take("dir"), "dir")?,
+                    ours_dump: take("ours-dump"),
+                    theirs_dump: take("theirs-dump"),
+                    digest_field: take("digest-field"),
+                    names: [
+                        take("ours-name").unwrap_or_else(|| "ours".to_owned()),
+                        take("theirs-name").unwrap_or_else(|| "theirs".to_owned()),
+                    ],
+                    arm,
+                }
+            } else {
+                Mode::Legs {
+                    manifest: need(take("manifest"), "manifest")?,
+                    dir: need(take("dir"), "dir")?,
+                    out: need(take("out"), "out")?,
+                    only: take("only"),
+                    arm,
+                    dump_at: tick(take("dump-at"), "dump-at")?,
+                }
+            }
+        }
+    };
+    if let Some((k, _)) = map.first() {
+        return Err(format!("unknown option --{k}"));
+    }
+    let wants_bite = match &parsed {
+        Mode::Bites => true,
+        Mode::Legs { arm, .. } | Mode::Compare { arm, .. } => arm.is_some(),
+        Mode::Vectors { .. } => false,
+    };
+    if wants_bite && !cfg!(feature = "bites") {
+        return Err(
+            "a bite asked of a measured build, which holds none (build npt-lab with --features bites)"
+                .to_owned(),
+        );
+    }
+    Ok(parsed)
+}
+
+/// One recordings manifest entry (u4-build-plan.md §4.15; crates/lab/recordings.json).
+#[derive(Clone, Debug, PartialEq, Eq)]
+pub(crate) struct Entry {
+    pub(crate) name: String,
+    pub(crate) scene: u8,
+    pub(crate) seed: u64,
+    pub(crate) first_tick: u32,
+    pub(crate) ticks: u32,
+    pub(crate) span_ms: u32,
+    pub(crate) generator: u32,
+    pub(crate) bytes: u64,
+    pub(crate) sha256: String,
+}
+
+/// The manifest's entries, in its order.
+pub(crate) fn manifest(text: &str) -> Result<Vec<Entry>, String> {
+    let v: Value =
+        serde_json::from_str(text).map_err(|e| format!("the manifest is not JSON: {e}"))?;
+    let list = v
+        .get("recordings")
+        .and_then(Value::as_array)
+        .ok_or("the manifest has no \"recordings\" list")?;
+    let mut out = Vec::new();
+    for (i, e) in list.iter().enumerate() {
+        let name = e
+            .get("name")
+            .and_then(Value::as_str)
+            .ok_or(format!("manifest entry {i} has no name"))?
+            .to_owned();
+        let num = |k: &str| {
+            e.get(k)
+                .and_then(Value::as_u64)
+                .ok_or(format!("manifest entry {name}: no whole number \"{k}\""))
+        };
+        let small = |k: &str| {
+            num(k).and_then(|n| {
+                u32::try_from(n)
+                    .map_err(|_| format!("manifest entry {name}: \"{k}\" {n} is above 32 bits"))
+            })
+        };
+        let entry = Entry {
+            scene: u8::try_from(num("scene")?)
+                .map_err(|_| format!("manifest entry {name}: no such scene"))?,
+            seed: num("seed")?,
+            first_tick: small("first_tick")?,
+            ticks: small("ticks")?,
+            span_ms: small("span_ms")?,
+            generator: small("generator")?,
+            bytes: num("bytes")?,
+            sha256: e
+                .get("sha256")
+                .and_then(Value::as_str)
+                .ok_or(format!("manifest entry {name}: no \"sha256\""))?
+                .to_owned(),
+            name,
+        };
+        if out.iter().any(|o: &Entry| o.name == entry.name) {
+            return Err(format!("manifest entry {} is named twice", entry.name));
+        }
+        out.push(entry);
+    }
+    Ok(out)
+}
+
+fn sha256_hex(bytes: &[u8]) -> String {
+    Sha256::digest(bytes)
+        .iter()
+        .map(|b| format!("{b:02x}"))
+        .collect()
+}
+
+/// The entry's recording, generated (after its generator is checked: `regenerate_all` checks every entry first).
+fn generate(e: &Entry) -> Result<Vec<u8>, String> {
+    let scene = Scene::by_id(e.scene).ok_or(format!("{}: no scene {}", e.name, e.scene))?;
+    Ok(Recording::synthetic(&scene, e.seed, e.first_tick, e.ticks, e.span_ms).encode())
+}
+
+/// Whether `bytes` are the entry's: its length and SHA-256.
+pub(crate) fn check(e: &Entry, bytes: &[u8]) -> Result<(), String> {
+    let sha = sha256_hex(bytes);
+    if bytes.len() as u64 != e.bytes || sha != e.sha256 {
+        return Err(format!(
+            "{}: regenerated {} bytes with sha256 {sha}; the manifest has {} bytes with sha256 {}",
+            e.name,
+            bytes.len(),
+            e.bytes,
+            e.sha256
+        ));
+    }
+    Ok(())
+}
+
+/// Every entry regenerated and checked, each handed to `keep` once it holds; an entry whose generator is not
+/// `Recording::GENERATOR` is refused before anything is generated (carry 52: only GENERATOR 2's recordings are pinned).
+pub(crate) fn regenerate_all(
+    entries: &[Entry],
+    mut keep: impl FnMut(&Entry, &[u8]) -> Result<(), String>,
+) -> Result<Vec<String>, String> {
+    for e in entries {
+        if e.generator != Recording::GENERATOR {
+            return Err(format!(
+                "{}: generator {}, not this build's Recording::GENERATOR {} (only its recordings are pinned)",
+                e.name,
+                e.generator,
+                Recording::GENERATOR
+            ));
+        }
+    }
+    let mut lines = Vec::new();
+    for e in entries {
+        let bytes = generate(e)?;
+        check(e, &bytes)?;
+        keep(e, &bytes)?;
+        lines.push(format!(
+            "regenerated {}: {} bytes, sha256 {}, as the manifest's",
+            e.name,
+            bytes.len(),
+            e.sha256
+        ));
+    }
+    Ok(lines)
+}
+
+/// Bytes as little-endian 32-bit words, the last zero-padded.
+fn le_words(bytes: &[u8]) -> Vec<u32> {
+    bytes
+        .chunks(4)
+        .map(|c| {
+            let mut w = [0u8; 4];
+            w[..c.len()].copy_from_slice(c);
+            u32::from_le_bytes(w)
+        })
+        .collect()
+}
+
+/// The scenes h2 reads, by name, and their header-only recordings' file names.
+fn vector_scenes() -> [Scene; 4] {
+    [
+        Scene::stress(),
+        Scene::estimate(),
+        Scene::course(),
+        Scene::targets(),
+    ]
+}
+
+fn header_only(scene: &Scene) -> Vec<u8> {
+    Recording {
+        scene: scene.id(),
+        seed: 1,
+        first_tick: 0,
+        span_ms: VECTOR_SPAN_MS,
+        generator: 0,
+        ticks: Vec::new(),
+    }
+    .encode()
+}
+
+/// A replayer for `bite` (bites builds only; parse refuses one in the measured build).
+fn replayer(arm: Option<&Arm>) -> Result<Replayer, Failure> {
+    let Some(a) = arm else {
+        return Ok(Replayer::new());
+    };
+    bitten(a)
+}
+
+#[cfg(feature = "bites")]
+fn bitten(a: &Arm) -> Result<Replayer, Failure> {
+    Replayer::with_bite(&a.id, a.tick).ok_or(Failure::Usage(format!(
+        "--bite {}: npt-sim's World plants no such bite (it plants {:?})",
+        a.id,
+        npt_sim::World::BITES
+    )))
+}
+
+#[cfg(not(feature = "bites"))]
+fn bitten(a: &Arm) -> Result<Replayer, Failure> {
+    Err(Failure::Usage(format!(
+        "--bite {}: a measured build holds no bites",
+        a.id
+    )))
+}
+
+#[cfg(feature = "bites")]
+fn order_digest(r: &Replayer) -> Option<u64> {
+    r.world().and_then(npt_sim::World::bite_order_digest)
+}
+
+#[cfg(not(feature = "bites"))]
+fn order_digest(_r: &Replayer) -> Option<u64> {
+    None
+}
+
+fn read_text(path: &str) -> Result<String, Failure> {
+    std::fs::read_to_string(path).map_err(|e| Failure::Failed(format!("{path}: {e}")))
+}
+
+fn rec_path(dir: &str, name: &str) -> PathBuf {
+    Path::new(dir).join(format!("{name}.rec"))
+}
+
+/// The entries a run replays: all, or `--only`'s.
+fn selected<'a>(entries: &'a [Entry], only: Option<&str>) -> Result<Vec<&'a Entry>, Failure> {
+    match only {
+        None => Ok(entries.iter().collect()),
+        Some(n) => entries
+            .iter()
+            .find(|e| e.name == n)
+            .map(|e| vec![e])
+            .ok_or(Failure::Usage(format!(
+                "--only {n}: no such recording in the manifest"
+            ))),
+    }
+}
+
+/// The recording a bite is armed on: `--on`'s, or the first replayed.
+fn bitten_name(arm: Option<&Arm>, replayed: &[&Entry]) -> Result<Option<String>, Failure> {
+    let Some(a) = arm else {
+        return Ok(None);
+    };
+    match &a.on {
+        Some(n) if replayed.iter().any(|e| &e.name == n) => Ok(Some(n.clone())),
+        Some(n) => Err(Failure::Usage(format!(
+            "--on {n}: not a recording this run replays"
+        ))),
+        None => Ok(replayed.first().map(|e| e.name.clone())),
+    }
+}
+
+/// A line of a leg's (or a Wasm leg's) per-tick list: `h <recording> <tick> <hash>`.
+pub(crate) fn hash_line(h: &TickHash) -> String {
+    format!("h {} {} {:016x}", h.recording, h.tick, h.hash)
+}
+
+pub(crate) fn parse_hash_line(line: &str) -> Option<TickHash> {
+    let mut p = line.strip_prefix("h ")?.split(' ');
+    let recording = p.next()?.to_owned();
+    let tick = p.next()?.parse().ok()?;
+    let hash = u64::from_str_radix(p.next()?, 16).ok()?;
+    p.next().is_none().then_some(TickHash {
+        recording,
+        tick,
+        hash,
+    })
+}
+
+/// The entries of one or more lines files, `paths` comma-separated and read in order (the loader writes one file per
+/// recording): their `h` lines; any other line, such as the loader's `stepped`, is skipped.
+fn read_lines(paths: &str) -> Result<Vec<TickHash>, Failure> {
+    let mut out = Vec::new();
+    for path in paths.split(',') {
+        for (i, l) in read_text(path)?.lines().enumerate() {
+            if l.starts_with("h ") {
+                out.push(parse_hash_line(l).ok_or(Failure::Failed(format!(
+                    "{path}:{}: not a hash line: {l:?}",
+                    i + 1
+                )))?);
+            }
+        }
+    }
+    Ok(out)
+}
+
+/// A dump file (the loader's and the legs'): `dump <recording> <tick|end>`, then `hash <16 hex>` (optional), then
+/// one word a line as hex.
+#[derive(Debug, PartialEq, Eq)]
+pub(crate) struct DumpFile {
+    pub(crate) recording: String,
+    pub(crate) tick: Option<u32>,
+    pub(crate) hash: Option<u64>,
+    pub(crate) words: Vec<u32>,
+}
+
+pub(crate) fn parse_dump(text: &str, what: &str) -> Result<DumpFile, String> {
+    let mut lines = text.lines();
+    let head = lines.next().unwrap_or("");
+    let mut p = head
+        .strip_prefix("dump ")
+        .ok_or(format!("{what}: no `dump <recording> <tick>` line"))?
+        .split(' ');
+    let recording = p.next().unwrap_or("").to_owned();
+    let tick = match p.next() {
+        Some("end") => None,
+        Some(t) => Some(t.parse().map_err(|_| format!("{what}: tick {t:?}"))?),
+        None => return Err(format!("{what}: no tick")),
+    };
+    let mut hash = None;
+    let mut words = Vec::new();
+    for l in lines {
+        if let Some(h) = l.strip_prefix("hash ") {
+            hash = Some(u64::from_str_radix(h, 16).map_err(|_| format!("{what}: hash {h:?}"))?);
+        } else if !l.is_empty() {
+            words.push(u32::from_str_radix(l, 16).map_err(|_| format!("{what}: word {l:?}"))?);
+        }
+    }
+    Ok(DumpFile {
+        recording,
+        tick,
+        hash,
+        words,
+    })
+}
+
+fn dump_text(recording: &str, tick: u32, hash: u64, words: &[u32]) -> String {
+    let mut s = format!("dump {recording} {tick}\nhash {hash:016x}\n");
+    for w in words {
+        s.push_str(&format!("{w:08x}\n"));
+    }
+    s
+}
+
+fn hex_words(words: &[u32]) -> String {
+    words
+        .iter()
+        .map(|w| format!("{w:x}"))
+        .collect::<Vec<_>>()
+        .join(" ")
+}
+
+fn parse_hex_words(s: &str) -> Option<Vec<u32>> {
+    if s.is_empty() {
+        return Some(Vec::new());
+    }
+    s.split(' ')
+        .map(|w| u32::from_str_radix(w, 16).ok())
+        .collect()
+}
+
+fn diff_text(d: &FieldDiff) -> String {
+    format!(
+        "field {} (word {}): {:08x} and {:08x}",
+        d.field, d.word, d.ours, d.theirs
+    )
+}
+
+/// The first word two word-only dumps differ at, named by `names` (this build's own dump of the same tick; npt-sim's
+/// `StateDump::field`), or `None` when they are equal: the dumps of two other processes arrive as words alone.
+fn words_difference(names: &StateDump, ours: &[u32], theirs: &[u32]) -> Option<FieldDiff> {
+    let n = ours.len().max(theirs.len());
+    (0..n)
+        .find(|&i| ours.get(i) != theirs.get(i))
+        .map(|i| FieldDiff {
+            word: i,
+            field: names.field(i).unwrap_or_else(|| "end".to_owned()),
+            ours: ours.get(i).copied().unwrap_or(0),
+            theirs: theirs.get(i).copied().unwrap_or(0),
+        })
+}
+
+/// Each recording's tick count in a list, in the order met.
+fn counts(list: &[TickHash]) -> Vec<(String, usize)> {
+    let mut out: Vec<(String, usize)> = Vec::new();
+    for h in list {
+        match out.last_mut() {
+            Some((n, k)) if *n == h.recording => *k += 1,
+            _ => out.push((h.recording.clone(), 1)),
+        }
+    }
+    out
+}
+
+fn counts_text(list: &[TickHash]) -> String {
+    counts(list)
+        .iter()
+        .map(|(n, k)| format!("{n} {k}"))
+        .collect::<Vec<_>>()
+        .join(", ")
+}
+
+/// Whether `list` holds each of `entries`' ticks, in order, exactly the manifest's count each (a short leg fails).
+fn count_problems(what: &str, list: &[TickHash], entries: &[&Entry]) -> Vec<String> {
+    let want: Vec<(String, usize)> = entries
+        .iter()
+        .map(|e| (e.name.clone(), e.ticks as usize))
+        .collect();
+    let got = counts(list);
+    if got == want {
+        return Vec::new();
+    }
+    vec![format!(
+        "{what} holds {} ticks, not the manifest's {}",
+        if got.is_empty() {
+            "no".to_owned()
+        } else {
+            counts_text(list)
+        },
+        want.iter()
+            .map(|(n, k)| format!("{n} {k}"))
+            .collect::<Vec<_>>()
+            .join(", ")
+    )]
+}
+
+/// One `HashCompare` as a line: `<a> against <b>: ...`.
+fn compare_text(names: &[String; 2], c: &HashCompare, field: Option<&str>) -> String {
+    let [a, b] = names;
+    match c {
+        HashCompare::Equal { entries } => {
+            format!("{a} against {b}: {entries} ticks compared, 0 mismatching, equal")
+        }
+        HashCompare::Empty => format!("{a} against {b}: 0 ticks compared (nothing to compare)"),
+        HashCompare::Mismatch {
+            index,
+            ours,
+            theirs,
+            compared,
+            mismatching,
+        } => {
+            let side = |h: &Option<TickHash>| match h {
+                Some(h) => format!("{} tick {} hash {:016x}", h.recording, h.tick, h.hash),
+                None => "its end".to_owned(),
+            };
+            format!(
+                "{a} against {b}: {compared} ticks compared, {mismatching} mismatching; first mismatch at entry \
+                 {index}: {} against {}; {}",
+                side(ours),
+                side(theirs),
+                field.unwrap_or("no field (no dump at that tick)")
+            )
+        }
+    }
+}
+
+/// A leg of the lock-step comparison: this binary re-run as a child.
+struct Leg {
+    k: u32,
+    child: Child,
+    stdin: ChildStdin,
+    out: BufReader<ChildStdout>,
+    digest: Option<String>,
+}
+
+enum LegLine {
+    Hash(TickHash),
+    End,
+}
+
+impl Leg {
+    fn spawn(k: u32, argv: &[String]) -> Result<Leg, Failure> {
+        let exe = std::env::current_exe()
+            .map_err(|e| Failure::Failed(format!("this binary's path: {e}")))?;
+        let mut child = Command::new(exe)
+            .arg("hash")
+            .args(argv)
+            .env(LEG, k.to_string())
+            .stdin(Stdio::piped())
+            .stdout(Stdio::piped())
+            .spawn()
+            .map_err(|e| Failure::Failed(format!("leg {k} did not start: {e}")))?;
+        let stdin = child.stdin.take().expect("piped");
+        let out = BufReader::new(child.stdout.take().expect("piped"));
+        Ok(Leg {
+            k,
+            child,
+            stdin,
+            out,
+            digest: None,
+        })
+    }
+
+    fn line(&mut self) -> Result<String, Failure> {
+        let mut line = String::new();
+        let n = self
+            .out
+            .read_line(&mut line)
+            .map_err(|e| Failure::Failed(format!("leg {}'s output: {e}", self.k)))?;
+        if n == 0 {
+            return Err(Failure::Failed(format!(
+                "leg {} ended without its last line",
+                self.k
+            )));
+        }
+        Ok(line.trim_end().to_owned())
+    }
+
+    fn next(&mut self) -> Result<LegLine, Failure> {
+        loop {
+            let l = self.line()?;
+            if let Some(d) = l.strip_prefix("o ") {
+                self.digest = Some(d.to_owned());
+                continue;
+            }
+            if l == "end" {
+                return Ok(LegLine::End);
+            }
+            return parse_hash_line(&l)
+                .map(LegLine::Hash)
+                .ok_or(Failure::Failed(format!("leg {} wrote {l:?}", self.k)));
+        }
+    }
+
+    fn send(&mut self, word: &str) -> Result<(), Failure> {
+        writeln!(self.stdin, "{word}")
+            .and_then(|()| self.stdin.flush())
+            .map_err(|e| Failure::Failed(format!("leg {}'s input: {e}", self.k)))
+    }
+
+    fn ask(&mut self, word: &str, prefix: &str) -> Result<String, Failure> {
+        self.send(word)?;
+        let l = self.line()?;
+        l.strip_prefix(prefix)
+            .map(str::to_owned)
+            .ok_or(Failure::Failed(format!(
+                "leg {} answered {word:?} with {l:?}",
+                self.k
+            )))
+    }
+
+    fn finish(mut self) -> Result<(), Failure> {
+        drop(self.stdin);
+        let status = self
+            .child
+            .wait()
+            .map_err(|e| Failure::Failed(format!("leg {}: {e}", self.k)))?;
+        if !status.success() {
+            return Err(Failure::Failed(format!("leg {} failed: {status}", self.k)));
+        }
+        Ok(())
+    }
+}
+
+fn write_file(path: &Path, text: &str) -> Result<(), Failure> {
+    std::fs::write(path, text).map_err(|e| Failure::Failed(format!("{}: {e}", path.display())))
+}
+
+fn lines_text(list: &[TickHash]) -> String {
+    let mut s = String::new();
+    for h in list {
+        s.push_str(&hash_line(h));
+        s.push('\n');
+    }
+    s
+}
+
+/// The first form: regenerate, check, write the header-only recordings, run the two legs, compare.
+fn legs(argv: &[String], mode: &Mode) -> Result<(Vec<String>, bool), Failure> {
+    let Mode::Legs {
+        manifest: m,
+        dir,
+        out,
+        only,
+        arm,
+        dump_at,
+    } = mode
+    else {
+        unreachable!("legs runs the first form")
+    };
+    let entries = manifest(&read_text(m)?).map_err(Failure::Failed)?;
+    let replayed = selected(&entries, only.as_deref())?;
+    let on = bitten_name(arm.as_ref(), &replayed)?;
+    // `--dump-at` names a tick of the bitten recording, or of `--only`'s.
+    let dump_on = on.clone().or_else(|| only.clone());
+    for d in [dir, out] {
+        std::fs::create_dir_all(d).map_err(|e| Failure::Failed(format!("{d}: {e}")))?;
+    }
+    // The header-only recordings first, said at once, so h2's vectors can run even if a regeneration fails.
+    let mut written = Vec::new();
+    for s in vector_scenes() {
+        let name = format!("vector-{}", s.name());
+        std::fs::write(rec_path(dir, &name), header_only(&s))
+            .map_err(|e| Failure::Failed(format!("{name}: {e}")))?;
+        written.push(name);
+    }
+    println!(
+        "wrote the header-only recordings {} (seed 1, first tick 0, no tick) for h2",
+        written.join(", ")
+    );
+    let mut lines = regenerate_all(&entries, |e, bytes| {
+        std::fs::write(rec_path(dir, &e.name), bytes)
+            .map_err(|err| format!("{}: {err}", rec_path(dir, &e.name).display()))
+    })
+    .map_err(Failure::Failed)?;
+    let mut a = Leg::spawn(1, argv)?;
+    let mut b = Leg::spawn(2, argv)?;
+    let (mut la, mut lb) = (Vec::new(), Vec::new());
+    let (mut a_done, mut b_done) = (false, false);
+    let mut field: Option<String> = None;
+    while !(a_done && b_done) {
+        let x = if a_done { None } else { Some(a.next()?) };
+        let y = if b_done { None } else { Some(b.next()?) };
+        let (ha, hb) = match (x, y) {
+            (Some(LegLine::Hash(ha)), Some(LegLine::Hash(hb))) => (Some(ha), Some(hb)),
+            (Some(LegLine::Hash(ha)), other) => {
+                b_done |= matches!(other, None | Some(LegLine::End));
+                (Some(ha), None)
+            }
+            (other, Some(LegLine::Hash(hb))) => {
+                a_done |= matches!(other, None | Some(LegLine::End));
+                (None, Some(hb))
+            }
+            _ => {
+                a_done = true;
+                b_done = true;
+                (None, None)
+            }
+        };
+        if let (Some(ha), Some(hb)) = (&ha, &hb)
+            && ha != hb
+            && field.is_none()
+        {
+            field = Some(
+                if (ha.recording.as_str(), ha.tick) == (hb.recording.as_str(), hb.tick) {
+                    let theirs = b.ask("d", "d ")?;
+                    let f = a.ask(&format!("c {theirs}"), "f ")?;
+                    format!("field {f}")
+                } else {
+                    "the legs stepped other ticks".to_owned()
+                },
+            );
+        }
+        if let (Some(ha), Some(hb), Some(t), Some(r)) = (&ha, &hb, dump_at, &dump_on)
+            && ha.tick == *t
+            && &ha.recording == r
+        {
+            for (leg, h, k) in [(&mut a, ha, 1), (&mut b, hb, 2)] {
+                let words = parse_hex_words(&leg.ask("d", "d ")?)
+                    .ok_or(Failure::Failed(format!("leg {k}'s dump")))?;
+                write_file(
+                    &Path::new(out).join(format!("native-{k}.dump")),
+                    &dump_text(&h.recording, h.tick, h.hash, &words),
+                )?;
+            }
+            lines.push(format!(
+                "dump at {r} tick {t}: each leg's written (native-1.dump, native-2.dump)"
+            ));
+        }
+        if let Some(h) = ha {
+            la.push(h);
+            a.send("n")?;
+        }
+        if let Some(h) = hb {
+            lb.push(h);
+            b.send("n")?;
+        }
+    }
+    let digests = [a.digest.clone(), b.digest.clone()];
+    a.finish()?;
+    b.finish()?;
+    write_file(&Path::new(out).join("native-1.lines"), &lines_text(&la))?;
+    write_file(&Path::new(out).join("native-2.lines"), &lines_text(&lb))?;
+    if let (Some(a), Some(r)) = (arm, &on) {
+        for (k, d) in digests.iter().enumerate() {
+            lines.push(format!(
+                "leg {} order digest {} ({} armed at {r} tick {})",
+                k + 1,
+                d.as_deref().unwrap_or("none"),
+                a.id,
+                a.tick
+            ));
+        }
+    }
+    let mut problems = count_problems("native leg 1", &la, &replayed);
+    problems.extend(count_problems("native leg 2", &lb, &replayed));
+    let c = HashCompare::of(&la, &lb);
+    let equal = matches!(c, HashCompare::Equal { .. });
+    lines.extend(problems.iter().cloned());
+    let names = ["native leg 1".to_owned(), "native leg 2".to_owned()];
+    lines.push(format!(
+        "{} ({})",
+        compare_text(&names, &c, field.as_deref()),
+        counts_text(&la)
+    ));
+    Ok((lines, equal && problems.is_empty()))
+}
+
+/// A leg: replays each recording in lock step with the parent (the protocol in the module's first form).
+fn serve(mode: &Mode) -> Result<(), Failure> {
+    let Mode::Legs {
+        manifest: m,
+        dir,
+        only,
+        arm,
+        ..
+    } = mode
+    else {
+        return Err(Failure::Usage("a leg runs the first form only".to_owned()));
+    };
+    let entries = manifest(&read_text(m)?).map_err(Failure::Failed)?;
+    let replayed = selected(&entries, only.as_deref())?;
+    let on = bitten_name(arm.as_ref(), &replayed)?;
+    let mut out = std::io::stdout().lock();
+    let mut input = std::io::stdin().lock().lines();
+    let io = |e: std::io::Error| Failure::Failed(format!("a leg's pipe: {e}"));
+    for e in &replayed {
+        let path = rec_path(dir, &e.name);
+        let bytes = std::fs::read(&path)
+            .map_err(|err| Failure::Failed(format!("{}: {err}", path.display())))?;
+        let words = le_words(&bytes);
+        let mut rp = replayer(
+            arm.as_ref()
+                .filter(|_| on.as_deref() == Some(e.name.as_str())),
+        )?;
+        let mut digest_written = false;
+        let mut chunks = words.chunks(CHUNK_WORDS);
+        loop {
+            let tick = match rp.step() {
+                Step::Stepped(tick) => tick,
+                Step::NeedMore => match chunks.next() {
+                    Some(c) => {
+                        for w in c {
+                            rp.feed(*w);
+                        }
+                        continue;
+                    }
+                    None => {
+                        return Err(Failure::Failed(format!(
+                            "{}: the file ends inside a tick",
+                            e.name
+                        )));
+                    }
+                },
+                Step::Done => break,
+                Step::Failed(why) => return Err(Failure::Failed(format!("{}: {why}", e.name))),
+            };
+            if !digest_written && let Some(d) = order_digest(&rp) {
+                writeln!(out, "o {d:016x}").map_err(io)?;
+                digest_written = true;
+            }
+            let hash = rp.state_hash().expect("a stepped replayer has a world");
+            let h = TickHash {
+                recording: e.name.clone(),
+                tick,
+                hash,
+            };
+            writeln!(out, "{}", hash_line(&h)).map_err(io)?;
+            out.flush().map_err(io)?;
+            loop {
+                let line = match input.next() {
+                    Some(l) => l.map_err(io)?,
+                    None => return Ok(()),
+                };
+                let dump = || rp.dump().expect("a stepped replayer has a world");
+                let (word, rest) = line.split_once(' ').unwrap_or((line.as_str(), ""));
+                match word {
+                    "n" => break,
+                    "q" => return Ok(()),
+                    "d" => writeln!(out, "d {}", hex_words(dump().words())).map_err(io)?,
+                    "c" => {
+                        let theirs = parse_hex_words(rest)
+                            .ok_or(Failure::Failed("the parent's words".to_owned()))?;
+                        match dump().first_difference(&theirs) {
+                            Some(d) => {
+                                writeln!(out, "f {}", diff_text(&d).trim_start_matches("field "))
+                            }
+                            None => writeln!(out, "f none: the dumps are equal"),
+                        }
+                        .map_err(io)?;
+                    }
+                    other => return Err(Failure::Failed(format!("the parent's word {other:?}"))),
+                }
+                out.flush().map_err(io)?;
+            }
+        }
+    }
+    writeln!(out, "end").map_err(io)?;
+    out.flush().map_err(io)
+}
+
+/// This build's own replay of `entry` to `tick`, its dump then (with the bite, armed as given, when it is `--on`'s).
+fn replay_to(dir: &str, e: &Entry, tick: u32, arm: Option<&Arm>) -> Result<StateDump, Failure> {
+    let path = rec_path(dir, &e.name);
+    let bytes = std::fs::read(&path)
+        .map_err(|err| Failure::Failed(format!("{}: {err}", path.display())))?;
+    let armed = arm.filter(|a| a.on.as_deref().is_none_or(|n| n == e.name));
+    let mut rp = replayer(armed)?;
+    for w in le_words(&bytes) {
+        rp.feed(w);
+    }
+    loop {
+        match rp.step() {
+            Step::Stepped(t) if t == tick => {
+                return Ok(rp.dump().expect("a stepped replayer has a world"));
+            }
+            Step::Stepped(_) => {}
+            other => {
+                return Err(Failure::Failed(format!(
+                    "{}: this build's replay never reached tick {tick} ({other:?})",
+                    e.name
+                )));
+            }
+        }
+    }
+}
+
+fn read_dump(path: &str) -> Result<DumpFile, Failure> {
+    parse_dump(&read_text(path)?, path).map_err(Failure::Failed)
+}
+
+/// `compare`: two lines files, and at a mismatch the field diff.
+fn compare(mode: &Mode) -> Result<(Vec<String>, bool), Failure> {
+    let Mode::Compare {
+        ours,
+        theirs,
+        manifest: m,
+        dir,
+        ours_dump,
+        theirs_dump,
+        digest_field,
+        names,
+        arm,
+    } = mode
+    else {
+        unreachable!("compare runs the compare form")
+    };
+    let entries = manifest(&read_text(m)?).map_err(Failure::Failed)?;
+    let (lo, lt) = (read_lines(ours)?, read_lines(theirs)?);
+    let dumps = [
+        ours_dump.as_deref().map(read_dump).transpose()?,
+        theirs_dump.as_deref().map(read_dump).transpose()?,
+    ];
+    // The recordings each list replays, in the manifest's order, each with the manifest's tick count.
+    let listed: Vec<&Entry> = entries
+        .iter()
+        .filter(|e| lo.iter().chain(&lt).any(|h| h.recording == e.name))
+        .collect();
+    let mut lines = Vec::new();
+    let mut problems = count_problems(&names[0], &lo, &listed);
+    problems.extend(count_problems(&names[1], &lt, &listed));
+    let entry = |name: &str| entries.iter().find(|e| e.name == name);
+    if let Some(f) = digest_field
+        && let Some(d) = dumps.iter().flatten().next()
+        && let (Some(t), Some(e)) = (d.tick, entry(&d.recording))
+    {
+        let own = replay_to(dir, e, t, arm.as_ref())?;
+        let at: Vec<usize> = (0..own.words().len())
+            .filter(|&i| own.field(i).as_deref() == Some(f.as_str()))
+            .collect();
+        for (k, d) in dumps.iter().enumerate() {
+            let Some(d) = d else { continue };
+            let words: Vec<u32> = at.iter().filter_map(|&i| d.words.get(i).copied()).collect();
+            let digest = match words.as_slice() {
+                [lo, hi] => format!("{:016x}", u64::from(*hi) << 32 | u64::from(*lo)),
+                [] => "none".to_owned(),
+                other => hex_words(other),
+            };
+            lines.push(format!(
+                "{} digest {digest} ({f} at words {:?} of its dump at {} tick {t})",
+                names[k], at, d.recording
+            ));
+        }
+    }
+    let c = HashCompare::of(&lo, &lt);
+    let field = match &c {
+        HashCompare::Mismatch {
+            ours: Some(a),
+            theirs: Some(b),
+            ..
+        } if (a.recording.as_str(), a.tick) == (b.recording.as_str(), b.tick) => {
+            let at = |d: &Option<DumpFile>| {
+                d.as_ref()
+                    .filter(|d| d.recording == a.recording && d.tick == Some(a.tick))
+                    .map(|d| d.words.clone())
+            };
+            match (entry(&a.recording), at(&dumps[0]), at(&dumps[1])) {
+                (None, ..) => Some(format!("no field ({} is not in the manifest)", a.recording)),
+                (Some(_), _, None) => None,
+                (Some(e), None, Some(t)) if ours_dump.is_none() => {
+                    let own = replay_to(dir, e, a.tick, arm.as_ref())?;
+                    Some(match own.first_difference(&t) {
+                        Some(d) => {
+                            format!("{} (ours: this build's replay to that tick)", diff_text(&d))
+                        }
+                        None => "no field: this build's dump there equals theirs".to_owned(),
+                    })
+                }
+                (Some(_), None, Some(_)) => {
+                    Some("no field (ours' dump is not at that tick)".to_owned())
+                }
+                (Some(e), Some(o), Some(t)) => {
+                    let own = replay_to(dir, e, a.tick, arm.as_ref())?;
+                    Some(match words_difference(&own, &o, &t) {
+                        Some(d) => format!(
+                            "{} (both dumps given, named by this build's)",
+                            diff_text(&d)
+                        ),
+                        None => "no field: the two dumps given are equal".to_owned(),
+                    })
+                }
+            }
+        }
+        HashCompare::Mismatch { .. } => Some("the lists step other ticks or one ends".to_owned()),
+        _ => None,
+    };
+    let equal = matches!(c, HashCompare::Equal { .. });
+    lines.extend(problems.iter().cloned());
+    lines.push(format!(
+        "{} ({})",
+        compare_text(names, &c, field.as_deref()),
+        counts_text(&lo)
+    ));
+    Ok((lines, equal && problems.is_empty()))
+}
+
+/// The committed vector file: its digest and words.
+pub(crate) fn committed_vector(text: &str) -> Result<(u64, Vec<u32>), String> {
+    let mut digest = None;
+    let mut words = Vec::new();
+    for l in text.lines().map(str::trim) {
+        if l.is_empty() || l.starts_with('#') {
+            continue;
+        }
+        if let Some(d) = l.strip_prefix("digest ") {
+            digest = Some(u64::from_str_radix(d, 16).map_err(|_| format!("digest {d:?}"))?);
+        } else {
+            words.push(u32::from_str_radix(l, 16).map_err(|_| format!("word {l:?}"))?);
+        }
+    }
+    Ok((digest.ok_or("no digest line")?, words))
+}
+
+/// `vectors`: h2 (ii), every scene's initial dump on wasm32 against this build's, and CO's against the committed file.
+fn vectors(mode: &Mode) -> Result<(Vec<String>, bool), Failure> {
+    let Mode::Vectors {
+        dir,
+        dumps,
+        committed,
+    } = mode
+    else {
+        unreachable!("vectors runs the vectors form")
+    };
+    let (digest, words) = committed_vector(&read_text(committed)?)
+        .map_err(|e| Failure::Failed(format!("{committed}: {e}")))?;
+    let mut lines = Vec::new();
+    let mut ok = true;
+    let mut sizes = Vec::new();
+    for s in vector_scenes() {
+        let name = format!("vector-{}", s.name());
+        let mut rp = Replayer::new();
+        let path = rec_path(dir, &name);
+        let bytes = std::fs::read(&path)
+            .map_err(|e| Failure::Failed(format!("{}: {e}", path.display())))?;
+        for w in le_words(&bytes) {
+            rp.feed(w);
+        }
+        if rp.step() != Step::Done {
+            return Err(Failure::Failed(format!(
+                "{name}: not a header-only recording"
+            )));
+        }
+        let own = rp.dump().expect("the header builds the world");
+        let own_hash = rp.state_hash().expect("the header builds the world");
+        let wasm = read_dump(
+            &Path::new(dumps)
+                .join(format!("{name}.dump"))
+                .display()
+                .to_string(),
+        )?;
+        sizes.push(format!("{} {}", s.name(), own.words().len()));
+        if let Some(d) = own.first_difference(&wasm.words) {
+            ok = false;
+            lines.push(format!(
+                "h2: {}'s initial dump differs on wasm32: {}",
+                s.name(),
+                diff_text(&d)
+            ));
+        }
+        if wasm.hash != Some(own_hash) {
+            ok = false;
+            lines.push(format!(
+                "h2: {}'s initial hash differs on wasm32: {:016x} natively, {} on wasm32",
+                s.name(),
+                own_hash,
+                wasm.hash.map_or("none".to_owned(), |h| format!("{h:016x}"))
+            ));
+        }
+        if s.name() == "CO" {
+            let n = words.len().max(wasm.words.len());
+            if let Some(i) = (0..n).find(|&i| words.get(i) != wasm.words.get(i)) {
+                ok = false;
+                lines.push(format!(
+                    "h2: CO's wasm32 dump differs from the committed vector at word {i} ({}): {} committed, {} on \
+                     wasm32",
+                    own.field(i).unwrap_or_else(|| "end".to_owned()),
+                    words.get(i).map_or("none".to_owned(), |w| format!("{w:08x}")),
+                    wasm.words.get(i).map_or("none".to_owned(), |w| format!("{w:08x}"))
+                ));
+            }
+            if wasm.hash != Some(digest) {
+                ok = false;
+                lines.push(format!(
+                    "h2: CO's wasm32 hash is not the committed digest {digest:016x}"
+                ));
+            }
+        }
+    }
+    lines.push(if ok {
+        format!(
+            "h2: CO's wasm32 dump is the committed vector ({} words, digest {digest:016x}); every scene's initial dump \
+             and hash are equal natively and on wasm32 ({})",
+            words.len(),
+            sizes.join(", ")
+        )
+    } else {
+        "h2: the vectors differ (the lines above)".to_owned()
+    });
+    Ok((lines, ok))
+}
+
+#[cfg(feature = "bites")]
+fn bites() -> Vec<String> {
+    npt_sim::World::BITES
+        .iter()
+        .enumerate()
+        .map(|(i, id)| format!("{} {id}", i + 1))
+        .collect()
+}
+
+#[cfg(not(feature = "bites"))]
+fn bites() -> Vec<String> {
+    Vec::new()
+}
+
+/// Runs `npt-lab hash` with the arguments after `hash`: its lines and whether everything compared was equal.
+pub(crate) fn run(argv: &[String]) -> Result<(Vec<String>, bool), Failure> {
+    let mode = parse(argv).map_err(Failure::Usage)?;
+    if std::env::var_os(LEG).is_some() {
+        serve(&mode)?;
+        return Ok((Vec::new(), true));
+    }
+    match &mode {
+        Mode::Legs { .. } => legs(argv, &mode),
+        Mode::Compare { .. } => compare(&mode),
+        Mode::Vectors { .. } => vectors(&mode),
+        Mode::Bites => Ok((bites(), true)),
+    }
+}
+
+#[cfg(test)]
+mod tests {
+    //! h5, the manifest, and h6, the subcommand's arguments (u4d2-brief.md 2.6).
+
+    use super::*;
+
+    fn argv(s: &str) -> Vec<String> {
+        s.split_whitespace().map(str::to_owned).collect()
+    }
+
+    /// A short entry, generated in the test: TC, 60 ticks across the wrap (sim/tests/replay.rs:514-515).
+    fn short() -> (Entry, Vec<u8>) {
+        let bytes = Recording::synthetic(&Scene::targets(), 8, u32::MAX - 20, 60, 2_000).encode();
+        let e = Entry {
+            name: "TC-s8".to_owned(),
+            scene: Scene::targets().id(),
+            seed: 8,
+            first_tick: u32::MAX - 20,
+            ticks: 60,
+            span_ms: 2_000,
+            generator: Recording::GENERATOR,
+            bytes: bytes.len() as u64,
+            sha256: sha256_hex(&bytes),
+        };
+        (e, bytes)
+    }
+
+    #[test]
+    fn h5_a_regenerated_entry_matches_its_recorded_sha256_and_one_changed_byte_fails_naming_it() {
+        let (e, bytes) = short();
+        assert_eq!(check(&e, &bytes), Ok(()));
+        let mut kept = Vec::new();
+        let lines = regenerate_all(std::slice::from_ref(&e), |e, b| {
+            kept.push((e.name.clone(), b.to_vec()));
+            Ok(())
+        })
+        .unwrap();
+        assert_eq!(kept, [("TC-s8".to_owned(), bytes.clone())]);
+        assert_eq!(lines.len(), 1);
+        let mut changed = bytes.clone();
+        changed[1_000] ^= 1;
+        let err = check(&e, &changed).unwrap_err();
+        assert!(
+            err.starts_with("TC-s8: regenerated 5421 bytes with sha256 "),
+            "{err}"
+        );
+        // The manifest's own digest changed: the regenerated bytes no longer match it, and nothing is kept.
+        let mut wrong = e.clone();
+        wrong.sha256 = sha256_hex(&changed);
+        let mut none = 0;
+        let err = regenerate_all(&[wrong], |_, _| {
+            none += 1;
+            Ok(())
+        })
+        .unwrap_err();
+        assert!(
+            err.contains("TC-s8") && err.contains("the manifest has"),
+            "{err}"
+        );
+        assert_eq!(none, 0);
+    }
+
+    #[test]
+    fn h5_a_generator_other_than_recording_generator_is_refused_before_anything_is_generated() {
+        let (good, _) = short();
+        let mut old = good.clone();
+        old.name = "TC-old".to_owned();
+        old.generator = Recording::GENERATOR - 1;
+        let mut kept = 0;
+        let err = regenerate_all(&[good, old], |_, _| {
+            kept += 1;
+            Ok(())
+        })
+        .unwrap_err();
+        assert!(
+            err.starts_with("TC-old: generator 1, not this build's Recording::GENERATOR 2"),
+            "{err}"
+        );
+        assert_eq!(kept, 0, "the good entry before it was not generated either");
+    }
+
+    #[test]
+    fn h5_the_committed_manifest_reads_and_names_three_generator_2_recordings() {
+        let text = include_str!("../recordings.json");
+        let entries = manifest(text).unwrap();
+        let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
+        assert_eq!(names, ["SS-s1", "ES-s2", "TC-s3"]);
+        assert!(
+            entries
+                .iter()
+                .all(|e| e.generator == Recording::GENERATOR && e.ticks == 9_000)
+        );
+        assert!(manifest("{\"recordings\": [{\"name\": \"x\"}]}").is_err());
+        assert!(manifest("[]").is_err());
+    }
+
+    #[test]
+    fn h6_usage_errors_are_refused() {
+        let base = "--manifest m --dir d --out o";
+        assert!(parse(&argv(base)).is_ok());
+        for bad in [
+            "--manifest m --dir d",
+            "--manifest m --dir d --out o --colour red",
+            "--manifest m --dir d --out o --dir e",
+            "--manifest m --dir d --out o --dump-at x",
+            "--manifest m --dir d --out o --dump-at -1",
+            "--manifest m --dir d --out o --arm 5",
+            "--manifest m --dir d --out o --on SS-s1",
+            "compare --ours a --manifest m --dir d",
+            "vectors --dir d --dumps e",
+            "--manifest",
+            "stray",
+        ] {
+            assert!(parse(&argv(bad)).is_err(), "{bad}");
+        }
+        assert!(parse(&argv("compare --ours a --theirs b --manifest m --dir d")).is_ok());
+        assert!(parse(&argv("vectors --dir d --dumps e --committed c")).is_ok());
+        assert_eq!(
+            parse_hash_line("h SS-s1 4294967295 00000000000000ff"),
+            Some(TickHash {
+                recording: "SS-s1".to_owned(),
+                tick: u32::MAX,
+                hash: 255
+            })
+        );
+        assert_eq!(parse_hash_line("h SS-s1 1 ff extra"), None);
+    }
+
+    #[cfg(not(feature = "bites"))]
+    #[test]
+    fn h6_a_bite_is_refused_in_the_measured_build() {
+        for bad in [
+            "--manifest m --dir d --out o --bite PM-0004-21/hash-map --arm 1234",
+            "compare --ours a --theirs b --manifest m --dir d --bite PM-0004-21/hash-map --arm 1",
+            "bites",
+        ] {
+            let err = parse(&argv(bad)).unwrap_err();
+            assert!(err.contains("measured build"), "{bad}: {err}");
+        }
+        assert!(matches!(run(&argv("bites")), Err(Failure::Usage(_))));
+    }
+
+    #[cfg(feature = "bites")]
+    #[test]
+    fn h6_the_bites_build_takes_a_bite_and_lists_world_bites_in_order() {
+        let m = parse(&argv(
+            "--manifest m --dir d --out o --bite PM-0004-21/hash-map --arm 1234 --on SS-s1",
+        ))
+        .unwrap();
+        assert!(matches!(
+            m,
+            Mode::Legs {
+                arm: Some(Arm { tick: 1_234, .. }),
+                ..
+            }
+        ));
+        let listed = bites();
+        assert_eq!(listed.len(), npt_sim::World::BITES.len());
+        assert_eq!(listed[0], "1 PM-0004-21/hash-map");
+        assert_eq!(listed[1], "2 PM-0004-21/std-sin");
+        assert!(matches!(
+            replayer(Some(&Arm {
+                id: "PM-0004-21/nothing".to_owned(),
+                tick: 0,
+                on: None
+            })),
+            Err(Failure::Usage(_))
+        ));
+    }
+
+    #[test]
+    fn a_dump_file_reads_back_and_two_word_dumps_are_named_by_this_builds_dump() {
+        let own = npt_sim::World::new(&Scene::course(), 1, 0).dump();
+        let text = dump_text("CO", 7, own.hash(), own.words());
+        let back = parse_dump(&text, "t").unwrap();
+        assert_eq!(
+            (back.tick, back.hash, back.words.as_slice()),
+            (Some(7), Some(own.hash()), own.words())
+        );
+        let mut other = own.words().to_vec();
+        other[3] ^= 1;
+        let d = words_difference(&own, own.words(), &other).unwrap();
+        assert_eq!((d.word, d.field.as_str()), (3, "soldier[0].alive"));
+        assert_eq!(words_difference(&own, own.words(), own.words()), None);
+        assert_eq!(
+            own.first_difference(&other).map(|d| d.field),
+            Some("soldier[0].alive".to_owned())
+        );
+    }
+}
diff --git a/.gate/u4d2-prefix/spikes/netcode-proof/crates/lab/src/lib.rs b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab/src/lib.rs
index eb42907..3a789bb 100644
--- a/.gate/u4d2-prefix/spikes/netcode-proof/crates/lab/src/lib.rs
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab/src/lib.rs
@@ -3,4 +3,158 @@
 //! `SimClock` (integer ns) and `L1Scenario`, which steps one server and 48 headless clients on one simulated clock in a
 //! fixed order per instant (due packets, the server's I/O step, the server tick if due, due client frames, due client
 //! ticks); the x86-64 re-decode and replay tool for the Arm rule (`ReplayX86`); the native-against-Wasm hash comparer
-//! (`HashCompare`). Nothing depends on this crate. Built in units U2 and U4.
+//! ([`HashCompare`], U4d2). Nothing depends on this crate. Built in units U2 and U4.
+
+/// One tick's entry in a leg's per-tick hash list: the recording (or run) it replays, the tick and the state hash.
+#[derive(Clone, Debug, PartialEq, Eq)]
+pub struct TickHash {
+    /// The recording's name, as its manifest names it (or a run's, for U4r's legs).
+    pub recording: String,
+    /// The tick the hash is of.
+    pub tick: u32,
+    /// The state hash after that tick.
+    pub hash: u64,
+}
+
+/// Two legs' per-tick hash lists compared, entry by entry in order (u4-build-plan.md §5 U4d2: the native-against-Wasm
+/// check, and U4r's -30 pre-check, which feeds it two legs' per-tick server hashes; plan.md §4.7). It reads no file and
+/// knows no recording: the lists are its whole input.
+#[derive(Clone, Debug, PartialEq, Eq)]
+pub enum HashCompare {
+    /// Both lists hold the same entries in the same order, this many (at least one).
+    Equal { entries: usize },
+    /// Both lists are empty: nothing was compared, which is never "equal".
+    Empty,
+    /// The lists differ. `index` is the first entry where they do: there the two entries differ in their recording,
+    /// tick or hash, or one list has ended (`None`). `compared` is the longer list's length, and `mismatching` how
+    /// many of those positions differ (an entry with no partner counts).
+    Mismatch {
+        index: usize,
+        ours: Option<TickHash>,
+        theirs: Option<TickHash>,
+        compared: usize,
+        mismatching: usize,
+    },
+}
+
+impl HashCompare {
+    /// Compares `ours` with `theirs`.
+    pub fn of(ours: &[TickHash], theirs: &[TickHash]) -> HashCompare {
+        let compared = ours.len().max(theirs.len());
+        if compared == 0 {
+            return HashCompare::Empty;
+        }
+        let differs = |i: usize| ours.get(i) != theirs.get(i);
+        let mismatching = (0..compared).filter(|&i| differs(i)).count();
+        match (0..compared).find(|&i| differs(i)) {
+            None => HashCompare::Equal { entries: compared },
+            Some(index) => HashCompare::Mismatch {
+                index,
+                ours: ours.get(index).cloned(),
+                theirs: theirs.get(index).cloned(),
+                compared,
+                mismatching,
+            },
+        }
+    }
+}
+
+#[cfg(test)]
+mod tests {
+    //! h4, `HashCompare` (u4d2-brief.md 2.6).
+
+    use super::*;
+
+    fn list(n: u32) -> Vec<TickHash> {
+        (0..n)
+            .map(|k| TickHash {
+                recording: if k < 20 { "SS-s1" } else { "ES-s2" }.to_string(),
+                tick: 1_000 + k,
+                hash: 0x9e37_79b9_7f4a_7c15_u64.wrapping_mul(u64::from(k) + 1),
+            })
+            .collect()
+    }
+
+    #[test]
+    fn h4_equal_lists_are_equal_over_every_entry() {
+        assert_eq!(
+            HashCompare::of(&list(40), &list(40)),
+            HashCompare::Equal { entries: 40 }
+        );
+    }
+
+    #[test]
+    fn h4_the_first_mismatch_is_found_above_index_0_with_both_entries_and_the_count() {
+        let mut theirs = list(40);
+        theirs[17].hash ^= 1;
+        theirs[30].hash ^= 1;
+        let ours = list(40);
+        assert_eq!(
+            HashCompare::of(&ours, &theirs),
+            HashCompare::Mismatch {
+                index: 17,
+                ours: Some(ours[17].clone()),
+                theirs: Some(theirs[17].clone()),
+                compared: 40,
+                mismatching: 2,
+            }
+        );
+    }
+
+    #[test]
+    fn h4_lists_of_different_lengths_mismatch_where_the_shorter_ends() {
+        let ours = list(40);
+        let theirs = list(25);
+        assert_eq!(
+            HashCompare::of(&ours, &theirs),
+            HashCompare::Mismatch {
+                index: 25,
+                ours: Some(ours[25].clone()),
+                theirs: None,
+                compared: 40,
+                mismatching: 15,
+            }
+        );
+        assert!(matches!(
+            HashCompare::of(&[], &theirs),
+            HashCompare::Mismatch {
+                index: 0,
+                ours: None,
+                compared: 25,
+                mismatching: 25,
+                ..
+            }
+        ));
+    }
+
+    #[test]
+    fn h4_a_tick_or_recording_out_of_step_is_a_mismatch_with_equal_hashes() {
+        let ours = list(10);
+        let mut tick = list(10);
+        tick[4].tick += 1;
+        assert!(matches!(
+            HashCompare::of(&ours, &tick),
+            HashCompare::Mismatch {
+                index: 4,
+                mismatching: 1,
+                ..
+            }
+        ));
+        let mut name = list(10);
+        name[6].recording = "TC-s3".to_string();
+        assert!(matches!(
+            HashCompare::of(&ours, &name),
+            HashCompare::Mismatch {
+                index: 6,
+                mismatching: 1,
+                ..
+            }
+        ));
+    }
+
+    #[test]
+    fn h4_0_entries_compared_is_its_own_result_never_equal() {
+        assert_eq!(HashCompare::of(&[], &[]), HashCompare::Empty);
+        assert_ne!(HashCompare::of(&[], &[]), HashCompare::Equal { entries: 0 });
+    }
+}
diff --git a/.gate/u4d2-prefix/spikes/netcode-proof/crates/lab/src/main.rs b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab/src/main.rs
index 6919e60..aa02208 100644
--- a/.gate/u4d2-prefix/spikes/netcode-proof/crates/lab/src/main.rs
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab/src/main.rs
@@ -1,15 +1,19 @@
 //! The npt-lab command (plan §4.7, §6.4): runs on this container's simulated time and writes npt-log results logs for
-//! npt-judge. Today it has one subcommand, `probe` (PM-0001-3's L1 half, unit U2); the scenario runner, the x86-64
-//! replay and the hash comparer join it in U4.
+//! npt-judge. Two subcommands: `probe` (PM-0001-3's L1 half, unit U2) and `hash` (U4d2: the recordings manifest
+//! regenerated and checked, the two native legs of the determinism check and its comparisons, in the release build;
+//! its own usage is in hash.rs); the scenario runner and the x86-64 replay join it in U4.
 //!
 //! ```text
 //! npt-lab probe --run <id> --set <set> --profile <P1..P4|B1> --seed <n> --date <YYYY-MM-DD> --commit <sha>
 //!               --out <file.jsonl> [--role measured|twin|bite] [--twin <run>] [--bite <id>]
 //!               [--packets <n>] [--rate <per second>] [--burst-at-s <s>]
+//! npt-lab hash ...
 //! ```
 //! A log is never overwritten: `--out` must not exist. Exit 0 on a written log, 1 when the run failed, 2 on a usage
-//! error.
+//! error. `probe`'s log header and summary line carry the whole fate log's SHA-256 and, since U4d2 (carry 66), the
+//! SHA-256 of v1's tag and the log's datagram entries, the hash U2's v1 logs had.
 
+mod hash;
 mod machine;
 mod probe;
 
@@ -114,6 +118,10 @@ fn run_probe(a: &Args) -> Result<String, String> {
         |(delivered, reordered): (u64, u64)| (reordered * 1000).checked_div(delivered).unwrap_or(0);
     let mut params: BTreeMap<String, serde_json::Value> = BTreeMap::new();
     params.insert("fate_log_sha256".into(), o.fate_log_sha256.clone().into());
+    params.insert(
+        "fate_log_v1_datagram_sha256".into(),
+        o.fate_log_v1_datagram_sha256.clone().into(),
+    );
     params.insert("reorder_up_permille".into(), permille(o.reorder[0]).into());
     params.insert(
         "reorder_down_permille".into(),
@@ -166,7 +174,7 @@ fn run_probe(a: &Args) -> Result<String, String> {
     let s = &o.summary;
     Ok(format!(
         "{} {} seed {}: {} sent each way; lost up {} down {}; mean one-way up {} ns down {} ns; mean round trip {} ns \
-         over {} samples; reorder up {} down {} of 1000; fate log {}",
+         over {} samples; reorder up {} down {} of 1000; fate log {}; v1 datagram fate log {}",
         a.run,
         profile.name,
         a.seed,
@@ -180,17 +188,39 @@ fn run_probe(a: &Args) -> Result<String, String> {
         permille(o.reorder[0]),
         permille(o.reorder[1]),
         &o.fate_log_sha256[..16],
+        &o.fate_log_v1_datagram_sha256[..16],
     ))
 }
 
 fn main() -> ExitCode {
     let argv: Vec<String> = std::env::args().skip(1).collect();
     let Some((cmd, rest)) = argv.split_first() else {
-        eprintln!("usage: {USAGE}");
+        eprintln!("usage: {USAGE}\n       {}", hash::USAGE);
         return ExitCode::from(2);
     };
+    if cmd == "hash" {
+        return match hash::run(rest) {
+            Ok((lines, equal)) => {
+                for l in lines {
+                    println!("{l}");
+                }
+                ExitCode::from(if equal { 0 } else { 1 })
+            }
+            Err(hash::Failure::Usage(e)) => {
+                eprintln!("npt-lab hash: {e}\nusage: {}", hash::USAGE);
+                ExitCode::from(2)
+            }
+            Err(hash::Failure::Failed(e)) => {
+                eprintln!("npt-lab hash: {e}");
+                ExitCode::from(1)
+            }
+        };
+    }
     if cmd != "probe" {
-        eprintln!("npt-lab: unknown command {cmd:?}\nusage: {USAGE}");
+        eprintln!(
+            "npt-lab: unknown command {cmd:?}\nusage: {USAGE}\n       {}",
+            hash::USAGE
+        );
         return ExitCode::from(2);
     }
     let args = match parse(rest) {
diff --git a/.gate/u4d2-prefix/spikes/netcode-proof/crates/lab/src/probe.rs b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab/src/probe.rs
index 02f28bd..933354c 100644
--- a/.gate/u4d2-prefix/spikes/netcode-proof/crates/lab/src/probe.rs
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab/src/probe.rs
@@ -45,6 +45,8 @@ pub struct Setup {
 pub struct Outcome {
     pub seconds: Vec<SecondRecord>,
     pub fate_log_sha256: String,
+    /// SHA-256 of v1's tag and the v2 fate log's datagram entries ([`v1_datagram_sha256`]; carry 66).
+    pub fate_log_v1_datagram_sha256: String,
     /// Per direction (up, down): (datagrams delivered, of them reordered), from the fate log.
     pub reorder: [(u64, u64); 2],
     /// Datagrams sent per direction, bursts included.
@@ -207,13 +209,20 @@ pub fn run(setup: &Setup) -> Result<Outcome, String> {
         t += interval;
     }
     drop(server);
-    let (fate_log_sha256, reorder) =
-        net.with_fate_log(|log| (sha256_hex(log), log.reorder_counts()));
+    let (fate_log_sha256, v1, reorder) = net.with_fate_log(|log| {
+        (
+            sha256_hex(log),
+            v1_datagram_sha256(log),
+            log.reorder_counts(),
+        )
+    });
+    let fate_log_v1_datagram_sha256 = v1?;
     let seconds = per_second(profile, interval, &up_delay, &down_delay, &rtt);
     let summary = summarize(&up_delay, &down_delay, &rtt);
     Ok(Outcome {
         seconds,
         fate_log_sha256,
+        fate_log_v1_datagram_sha256,
         reorder,
         sent: total,
         summary,
@@ -241,6 +250,38 @@ fn sha256_hex(log: &FateLog) -> String {
     h.finalize().iter().map(|b| format!("{b:02x}")).collect()
 }
 
+/// The fate log's canonical form's first bytes since U4c (v2), and v1's, which U2's committed hashes began with.
+const V2_TAG: &[u8] = b"npt-l1 fate log v2\n";
+const V1_TAG: &[u8] = b"npt-l1 fate log v1\n";
+/// A datagram's entry in the canonical form: 40 bytes, flow code 0 at byte 5 (`FateLog::for_each_canonical`).
+const DATAGRAM_ENTRY: usize = 40;
+
+/// SHA-256 of v1's tag followed by the v2 fate log's datagram entries, read through npt-l1's public
+/// `FateLog::for_each_canonical` (carry 66; u4c-review-netcode.md F2; L-c8): the v2 tag is skipped and the 40-byte
+/// entries with flow code 0 are kept (the acknowledgements v2 logs beside them are not), as l1/tests/l1.rs:479-505
+/// walks them. A datagram's entry is v1's form, so for the probe, whose v1 log held only datagrams, this is the hash
+/// v1's whole log had for the same run: U2's committed
+/// 913db8e9…, feabf2f0… and b8a04f2f… for P2, B1 and P4 at seed 1. A log whose form starts with another tag is refused.
+fn v1_datagram_sha256(log: &FateLog) -> Result<String, String> {
+    let mut h = Sha256::new();
+    h.update(V1_TAG);
+    let mut tag: Option<Vec<u8>> = None;
+    log.for_each_canonical(|b| {
+        if tag.is_none() {
+            tag = Some(b.to_vec());
+        } else if b.len() == DATAGRAM_ENTRY && b[5] == 0 {
+            h.update(b);
+        }
+    });
+    if tag.as_deref() != Some(V2_TAG) {
+        return Err(format!(
+            "the fate log's canonical form starts with {:?}, not v2's tag: the v1 datagram hash reads v2's form only",
+            tag.map(|t| String::from_utf8_lossy(&t).into_owned())
+        ));
+    }
+    Ok(h.finalize().iter().map(|b| format!("{b:02x}")).collect())
+}
+
 fn as_i64(v: u64) -> i64 {
     i64::try_from(v).unwrap_or(i64::MAX)
 }
@@ -460,6 +501,115 @@ mod tests {
         }
     }
 
+    /// SHA-256 of v1's tag and a log's datagram entries, by l1/tests/l1.rs:479-505's walk: the v2 tag first, then
+    /// every entry by its size and flow code (40 and 0 a datagram, 64 and 1 or 2 a stream piece, 56 and 3 an
+    /// acknowledgement), the datagrams hashed.
+    fn v1_walk(log: &FateLog) -> (String, [u64; 3]) {
+        let mut h = Sha256::new();
+        h.update(b"npt-l1 fate log v1\n");
+        let mut counts = [0u64; 3];
+        let mut tag = true;
+        log.for_each_canonical(|b| {
+            if tag {
+                assert_eq!(b, b"npt-l1 fate log v2\n");
+                tag = false;
+                return;
+            }
+            match (b.len(), b[5]) {
+                (40, 0) => {
+                    counts[0] += 1;
+                    h.update(b);
+                }
+                (64, 1 | 2) => counts[1] += 1,
+                (56, 3) => counts[2] += 1,
+                other => panic!("an entry of {} bytes with flow code {}", other.0, other.1),
+            }
+        });
+        let hex = h.finalize().iter().map(|b| format!("{b:02x}")).collect();
+        (hex, counts)
+    }
+
+    #[test]
+    fn h7_the_v1_datagram_hash_is_v1s_tag_and_the_logs_datagram_entries() {
+        // A probe's traffic on L1: datagrams both ways, as `run` sends them, at P2's loss and delay.
+        let net = L1Net::new(1);
+        let mut client = net.connect(Profile::p2(), MAX_DATAGRAM, 0);
+        let mut listener = net.listener();
+        let mut server: Option<Box<dyn Link>> = None;
+        for i in 0..3_000u64 {
+            let t = send_time(i, 100_000);
+            if server.is_none() {
+                server = listener.poll_accept(t).map(|(l, _)| l);
+            }
+            let s = server.as_mut().expect("accepted at once");
+            while s.poll_recv(t).is_some() {}
+            while client.poll_recv(t).is_some() {}
+            let d = |dir| Datagram {
+                dir,
+                index: i,
+                sent_ns: t,
+                echo: NONE,
+                hold_ns: 0,
+            };
+            assert_eq!(client.send_unreliable(&encode(&d(0)), t), SendResult::Sent);
+            assert_eq!(s.send_unreliable(&encode(&d(1)), t), SendResult::Sent);
+        }
+        drop(server);
+        net.with_fate_log(|log| {
+            // Every datagram has its entry; v2 also logs the acknowledgements beside them, which v1 had not.
+            let (walked, [datagrams, stream, acks]) = v1_walk(log);
+            assert_eq!((datagrams, stream), (6_000, 0));
+            assert!(
+                acks > 0,
+                "{acks} acknowledgements logged beside the datagrams"
+            );
+            assert_eq!(v1_datagram_sha256(log), Ok(walked.clone()));
+            // The whole log's hash starts with the v2 tag, so the two differ on the same entries.
+            assert_ne!(sha256_hex(log), walked);
+        });
+        // A probe run carries both, each 64 lowercase hex digits.
+        let o = run(&setup(Profile::p2(), 1, 10_000)).unwrap();
+        for h in [&o.fate_log_sha256, &o.fate_log_v1_datagram_sha256] {
+            assert!(
+                h.len() == 64
+                    && h.bytes()
+                        .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
+            );
+        }
+        assert_ne!(o.fate_log_sha256, o.fate_log_v1_datagram_sha256);
+    }
+
+    /// h7 (ii), carry 66's three values, run once by name on the release build for the report (the precedent:
+    /// sim/tests/world.rs:1188): P2, B1 and P4 at seed 1 with -3's arguments (1,000,000 datagrams each way at 10,000
+    /// a second; B1's burst at 50 s, as main.rs builds it), each SHA-256 of v1's tag and the datagram entries U2's
+    /// whole v1 log's (results/c-u2-pm-0001-3/81c6d01e371b/runs.txt:16, :34, :28 hold their first 16 digits).
+    #[test]
+    #[ignore = "carry 66's judged values: 3 probe runs of 1,000,000 datagrams each way; run by name (u4d2-brief.md 2.6 h7)"]
+    fn h7_carry_66_the_probes_v1_datagram_hashes_are_u2s_for_p2_b1_and_p4_seed_1() {
+        for (name, want) in [
+            (
+                "P2",
+                "913db8e914bdb773f6f19ac404c7098f711798790fcd1a861502502764b11ecc",
+            ),
+            (
+                "B1",
+                "feabf2f02d023fa9784b2f9630ec6e2ceeed0bcdd28507c203b0ba40de1c25bf",
+            ),
+            (
+                "P4",
+                "b8a04f2f8ddc0d39624d853730636c50ffb354ddb187938066fabc156c1f602b",
+            ),
+        ] {
+            let profile = crate::profile(name, 50).unwrap();
+            let o = run(&setup(profile, 1, PACKETS)).unwrap();
+            println!(
+                "{name} seed 1: fate log {}; v1 datagram fate log {}",
+                o.fate_log_sha256, o.fate_log_v1_datagram_sha256
+            );
+            assert_eq!(o.fate_log_v1_datagram_sha256, want, "{name} seed 1");
+        }
+    }
+
     #[cfg(not(feature = "bites"))]
     #[test]
     fn a_measured_build_refuses_a_bite() {
diff --git a/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab/std-sin-scan.txt b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab/std-sin-scan.txt
new file mode 100644
index 0000000..ecebce2
--- /dev/null
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/crates/lab/std-sin-scan.txt
@@ -0,0 +1,17 @@
+# PM-0004-21/std-sin's scan (u4-build-plan.md §5 U4d2; u4d2-brief.md 2.3 and 2.7; the advisor's U4D2-Q5 ruling, (a)):
+# the bitten World (f32::sin in place of libm::sinf in the vehicles' loops: sim/src/world.rs's sinf and
+# place_vehicles), armed at the first tick of each recording whose scene has vehicles (SS-s1 and ES-s2; TC-s3's scene
+# has none), its native leg against its Wasm leg, and the bitten World against the clean one on each target. A `catch`
+# line is <recording> <first tick the bitten native and Wasm legs differ> <the field there>. check.mjs's wasm step arms
+# std-sin at the first catch in the recordings manifest's order, on its recording, native leg 1 and Wasm leg 1, and
+# fails unless their first mismatch is that tick and field. The lines below the catches are the scan's record.
+catch SS-s1 2 vehicle[8].pos.y
+catch ES-s2 1000002 vehicle[8].pos.y
+scanned on: <base> bfda3cd2e4733efc3d3ec9f7c6c3c99bc6c2e8c4 with U4d2's code (uncommitted, in its worktree), for x86_64-unknown-linux-gnu and wasm32-unknown-unknown under Node v22.23.4
+rustc: rustc 1.99.1 (58cf8f9c3 2026-10-13)
+libc: ldd (Ubuntu GLIBC 2.39-0ubuntu8.9) 2.39
+SS-s1, armed at tick 0: native against Wasm, 9000 ticks compared, 1375 mismatching, the first at tick 2; native bitten against native clean, 1375 mismatching, the first at tick 2; Wasm bitten against Wasm clean, 0 mismatching (wasm32's std sin is libm's here)
+SS-s1, armed at tick 2 with both dumps: the first mismatch at tick 2, field vehicle[8].pos.y (word 2148): c159186a native, c1591869 on wasm32 (one ulp)
+ES-s2, armed at tick 1000000: native against Wasm, 9000 ticks compared, 1326 mismatching, the first at tick 1000002; native bitten against native clean, 1326 mismatching, the first at tick 1000002; Wasm bitten against Wasm clean, 0 mismatching
+ES-s2, armed at tick 1000002 with both dumps: the first mismatch at tick 1000002, field vehicle[8].pos.y (word 996): c0099f7a native, c0099f79 on wasm32 (one ulp)
+commands: the bites builds (cargo build --locked --release -p npt-lab --features bites; the same --target wasm32-unknown-unknown -p npt-hash-wasm); per recording R from its first tick F: npt-lab hash --manifest crates/lab/recordings.json --dir <recordings> --out <dir> --only R --bite PM-0004-21/std-sin --arm F; node crates/hash-wasm/loader.mjs --wasm <the bites npt_hash_wasm.wasm> --rec R.rec --name R --first-tick F --bite 2 --arm F --out <dir>/wasm-1.lines; the same two with no bite (the clean World); npt-lab hash compare for each pair; then armed at the first differing tick T with --dump-at T on both legs and compared with both dumps (--ours-dump, --theirs-dump), which names the field (the runner's logs: .gate/u4d2/b8-scan1.log and b8-scan2.log)
````

### check.mjs
_`git diff --no-index .gate/u4d2-prefix/spikes/netcode-proof/check.mjs .claude/worktrees/u4d2-build/spikes/netcode-proof/check.mjs`:  1 file changed, 820 insertions(+), 46 deletions(-)._

````diff
diff --git a/.gate/u4d2-prefix/spikes/netcode-proof/check.mjs b/.claude/worktrees/u4d2-build/spikes/netcode-proof/check.mjs
index 6d121d8..059733d 100644
--- a/.gate/u4d2-prefix/spikes/netcode-proof/check.mjs
+++ b/.claude/worktrees/u4d2-build/spikes/netcode-proof/check.mjs
@@ -9,26 +9,34 @@
 // `cargo metadata` shows, with every feature and target, and its unsafe_code rule, from the crates' files, the
 // manifests and the environment and cargo configs this command's cargo reads: DEPS_CHECKED says what it checks, and
 // the comment above it what it does not; with U4s's three greps: the tick constants of u4-build-plan.md §4.3,
-// npt-room's float-free sources, §4.11, and each bite id in one crate's sources, §4.14; and U4a's two: npt-proto's
-// clippy.toml against npt-sim's, §2, and npt-link's Channel alone in npt-proto and npt-client); refs (nothing outside
-// spikes/
-// references it, §4.2 and §4.9);
+// npt-room's float-free sources, §4.11, and each bite id in one crate's sources, §4.14; U4a's two: npt-proto's
+// clippy.toml against npt-sim's, §2, and npt-link's Channel alone in npt-proto and npt-client; and U4d2's: npt-sim's
+// resolved features in each build that runs it, the build flags with rustc's read-back of them, and npt-hash-wasm's
+// rules, the spike's third unsafe exception, §5 U4d2); wasm (the determinism check, plan.md §4.5 and u4-build-plan.md
+// §5 U4d2: release builds of npt-lab for x86-64 and npt-hash-wasm for wasm32, measured and bites; the recordings
+// manifest regenerated and checked; npt-lab's two native legs and the Node loader's Wasm leg over its recordings,
+// compared tick by tick, with the field diff at a first mismatch; PM-0004-21's cross-target vectors; and its two
+// bites, hash-map and std-sin, in the bites builds); refs (nothing outside spikes/ references it, §4.2 and §4.9);
 // marks (marks.json compiles in the judge, covers exactly the plan's marks, and every row it hashed is unchanged); self
 // (this file's own tests); redact (redact-check's tests, then a scan of the spike's folder). It runs every step and
 // exits 1 if any failed.
 
 import { spawnSync } from "node:child_process";
 import { createHash } from "node:crypto";
-import { readFileSync, readdirSync } from "node:fs";
+import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync } from "node:fs";
 import { homedir } from "node:os";
 import { dirname, join, relative, resolve, sep } from "node:path";
 import { fileURLToPath, pathToFileURL } from "node:url";
+import { HASH_WASM_EXPORTS } from "./crates/hash-wasm/loader.mjs";
 
 const HERE = dirname(fileURLToPath(import.meta.url));
 const ROOT = join(HERE, "..", "..");
 
-/** The leaf crates (plan §4.2): built for wasm32, free of every transport crate and async runtime. */
-export const LEAF = ["npt-link", "npt-l1", "npt-sim", "npt-proto", "npt-room", "npt-client", "npt-log"];
+/**
+ * The leaf crates (plan §4.2): built for wasm32, free of every transport crate and async runtime; npt-hash-wasm, the
+ * check-only cdylib over npt-sim, among them (u4-build-plan.md §5 U4d2), so the wasm32 clippy and doc steps take it.
+ */
+export const LEAF = ["npt-link", "npt-l1", "npt-sim", "npt-proto", "npt-room", "npt-client", "npt-log", "npt-hash-wasm"];
 /** Transport crates and async runtimes no leaf crate may reach (plan §4.2's list, with their own parts). */
 export const TRANSPORT = [
   "tokio",
@@ -119,6 +127,21 @@ export function dependencyProblems(meta) {
     if (name === "npt-net") continue;
     for (const d of n.deps) if (byId.get(d.pkg).name === "npt-tcpinfo") problems.push(`${name} depends on npt-tcpinfo`);
   }
+  // npt-hash-wasm (U4d2; u4-build-plan.md §5 U4d2): npt-sim is its only shipped dependency, and nothing depends on it,
+  // in any kind: a check-only cdylib that Node loads, whose exports are the spike's third unsafe exception. A missing
+  // npt-hash-wasm is said once, by the LEAF loop above (it is a leaf crate), and so is a missing npt-sim (below).
+  const hashWasmId = meta.workspace_members.find((id) => byId.get(id).name === "npt-hash-wasm");
+  if (hashWasmId) {
+    const node = meta.resolve.nodes.find((n) => n.id === hashWasmId);
+    const shippedDeps = (node?.deps ?? []).filter(isShipped).map((d) => byId.get(d.pkg).name);
+    for (const name of shippedDeps) if (name !== "npt-sim") problems.push(`npt-hash-wasm depends on ${name}`);
+    if (!shippedDeps.includes("npt-sim") && all.has("npt-sim")) problems.push("npt-hash-wasm does not depend on npt-sim");
+  }
+  for (const n of meta.resolve.nodes) {
+    for (const d of n.deps) {
+      if (byId.get(d.pkg).name === "npt-hash-wasm") problems.push(`${byId.get(n.id).name} depends on npt-hash-wasm`);
+    }
+  }
   // npt-sim (U4s; the arch review's F4): it ships with SIM_DEPENDENCIES alone, so the sim stays free of every crate
   // above it and of any dependency no review has read; a dev dependency does not ship (C10). A missing npt-sim fails
   // here too, not only in the LEAF loop above (it is a leaf crate), which says the same sentence: it is said once.
@@ -161,14 +184,34 @@ export function dependencyProblems(meta) {
 /**
  * The crates that may relax `unsafe_code`, by folder: ADR 0002:264-265's exceptions ("an exception needs a written
  * invariant, a test and an #[allow] with its reason"), named in the workspace Cargo.toml. The workspace forbids the
- * lint, so these two carry their own [lints] tables in place of `workspace = true` (EXCEPTION_LEVELS):
+ * lint, so these three carry their own [lints] tables in place of `workspace = true` (EXCEPTION_LEVELS):
  * npt-client-wasm allows it for the whole crate (wasm-bindgen's generated code), so its files are not scanned for it;
  * npt-tcpinfo denies it, which its two #[allow]s can relax (forbid could not), only through the attributes
- * TCPINFO_UNSAFE_FNS allows, and has `unsafe` only inside those functions.
+ * TCPINFO_UNSAFE_FNS allows, and has `unsafe` only inside those functions; npt-hash-wasm, the third (U4d2; the
+ * advisor's condition 7), denies it too, and relaxes it only by one #[allow] on each of its listed exports
+ * (HASH_WASM_EXPORTS, the Node loader's list), whose `#[unsafe(no_mangle)]` is its only `unsafe`.
  */
-export const UNSAFE_EXCEPTIONS = { "crates/client-wasm/": "npt-client-wasm", "crates/tcpinfo/": "npt-tcpinfo" };
+export const UNSAFE_EXCEPTIONS = {
+  "crates/client-wasm/": "npt-client-wasm",
+  "crates/tcpinfo/": "npt-tcpinfo",
+  "crates/hash-wasm/": "npt-hash-wasm",
+};
 /** The unsafe_code level each exception's own [lints.rust] gives, its other lints being the workspace's (review C2R-1). */
-const EXCEPTION_LEVELS = { "crates/client-wasm/": "allow", "crates/tcpinfo/": "deny" };
+const EXCEPTION_LEVELS = { "crates/client-wasm/": "allow", "crates/tcpinfo/": "deny", "crates/hash-wasm/": "deny" };
+/** The exceptions' names as a sentence lists them: "a, b and c". */
+const exceptionNames = () => {
+  const names = Object.values(UNSAFE_EXCEPTIONS);
+  return `${names.slice(0, -1).join(", ")} and ${names.at(-1)}`;
+};
+/** npt-hash-wasm's one file with unsafe code, its seven exports (u4-build-plan.md §5 U4d2; u4d2-brief.md 2.4 (d)). */
+export const HASH_WASM_UNSAFE_FILE = "crates/hash-wasm/src/lib.rs";
+/**
+ * The types an export's parameters and result may have (the advisor's U4D2-Q7 ruling): rustc lowers both to Wasm's
+ * i32, so every export has integer signatures only (the third exception's written invariant).
+ */
+const EXPORT_INTEGERS = ["u32", "i32"];
+/** The one attribute whose `unsafe` npt-hash-wasm may have: `#[unsafe(no_mangle)]`, sitting on a listed export. */
+const NO_MANGLE = /^\s*unsafe\s*\(\s*no_mangle\s*\)\s*$/;
 /** npt-tcpinfo's only relaxing attributes (u3c-plan.md 1.8 (a)): one on each of these functions, in this file. */
 export const TCPINFO_UNSAFE_FILE = "crates/tcpinfo/src/linux.rs";
 export const TCPINFO_UNSAFE_FNS = ["srtt_us", "unacked_and_unsent"];
@@ -201,9 +244,9 @@ const UNSAFE_CODE_NAME = /\bunsafe[-_]code\b/i;
  * the unsafe_code lint denies that edition 2024 lets go without the keyword (`#[no_mangle]`, `#[export_name]` and
  * `#[link_section]` need `#[unsafe(...)]` there; rustc 1.99.1 compiles a macro-built `#[allow($l)]` over
  * `global_asm!("")` with no `unsafe` in sight). Found in code, they are unsafe code whatever relaxed the lint: an
- * attribute this scan cannot read (one built by a macro), rustflags or `--cap-lints`. In every crate but the two
+ * attribute this scan cannot read (one built by a macro), rustflags or `--cap-lints`. In every crate but the three
  * exceptions the compiler refuses them first (the workspace's forbid: E0453 on any allow, a macro-built one included);
- * this scan is the second layer there, and inside npt-tcpinfo the only one.
+ * this scan is the second layer there, and inside npt-tcpinfo and npt-hash-wasm the only one.
  */
 const UNSAFE_CODE = /(?<!r#)\bunsafe\b|\bglobal_asm\b/g;
 /** The one form npt-tcpinfo may use: `allow(unsafe_code, reason = "...")` and nothing else in the attribute. */
@@ -224,8 +267,8 @@ const isPlacedAllow = (body) =>
  * blank the code after it.
  * Known limit (review C2R-1): this is not rustc's lexer. A first line `#!...` (a shebang, which rustc drops from any
  * source file) or a `\u{...}` escape with underscores past six characters puts it out of step, so text rustc compiles
- * can read here as a comment or a literal. Outside the two exception crates the workspace's forbid makes rustc refuse
- * unsafe code this misses; inside npt-tcpinfo (deny, not forbid) nothing but review does.
+ * can read here as a comment or a literal. Outside the three exception crates the workspace's forbid makes rustc refuse
+ * unsafe code this misses; inside npt-tcpinfo and npt-hash-wasm (deny, not forbid) nothing but review does.
  */
 export function lexRust(src) {
   const blank = (s) => s.replace(/[^\n]/g, " ");
@@ -451,6 +494,111 @@ function tcpinfoProblems(f, code, mask, attrs, sites, state) {
   return problems;
 }
 
+/**
+ * The item an outer attribute ending at `at` sits on, other outer attributes skipped, read for npt-hash-wasm's exports:
+ * `item` its first line as written (from `code`), `export` its name when it is `pub extern "C" fn <name>(` with the
+ * name on HASH_WASM_EXPORTS, else null. itemAfter reads `pub fn` only, and in the mask the `"C"` is blanked, so the ABI
+ * is read from `code`.
+ */
+function exportAfter(code, mask, at, attrs) {
+  let j = at;
+  for (;;) {
+    while (j < mask.length && /\s/.test(mask[j])) j++;
+    const next = attrs.find((a) => a.start === j && !a.inner);
+    if (!next) break;
+    j = next.end;
+  }
+  const item = code.slice(j).split("\n", 1)[0].trim() || "(nothing)";
+  const name = code.slice(j).match(/^pub\s+extern\s+"C"\s+fn\s+([A-Za-z_]\w*)\s*\(/)?.[1] ?? null;
+  return { item, export: name && HASH_WASM_EXPORTS.includes(name) ? name : null };
+}
+
+/**
+ * npt-hash-wasm's rules (the spike's third unsafe exception: u4-build-plan.md §5 U4d2; u4d2-brief.md 2.4 (d)), each
+ * broken one reported with its line, in tcpinfoProblems' pattern:
+ * - every relaxing of unsafe_code in the crate is an outer `#[allow(unsafe_code, reason = "...")]` in
+ *   HASH_WASM_UNSAFE_FILE sitting directly on a listed export, `pub extern "C" fn <one of HASH_WASM_EXPORTS>` (other
+ *   outer attributes may sit between), its reason not empty; anything else is a problem: an inner attribute, cfg_attr,
+ *   expect or warn, an allow on a helper or any other item, a mention in a string or a macro's text, `unsafe_code`
+ *   named in code outside an attribute; `sites` counts the placed allows per export (one each, checked by the caller);
+ * - every `unsafe` in the crate is the `unsafe` of a `#[unsafe(no_mangle)]` on a listed export in that file: any other
+ *   (a block, an unsafe fn, impl or trait, an `unsafe extern` block, `#[unsafe(export_name = ...)]`,
+ *   `#[unsafe(link_section = ...)]`, a no_mangle on an export not on the list) and `global_asm!` is a problem;
+ * - every listed export's parameters, and its result if it has one, are u32 or i32 (EXPORT_INTEGERS; the advisor's
+ *   U4D2-Q7 ruling): Wasm gets i32 for both, so the written invariant's "integer signatures only" holds.
+ */
+function hashWasmProblems(f, code, mask, attrs, sites) {
+  const problems = [];
+  const lists = `its listed exports (${HASH_WASM_EXPORTS.join(", ")}) in ${HASH_WASM_UNSAFE_FILE}`;
+  const inAttribute = (i) => attrs.some((a) => i >= a.start && i < a.end);
+  let inAttributes = 0;
+  for (const a of attrs) {
+    if (!mask.slice(a.bodyStart, a.bodyEnd).match(RELAX_UNSAFE)) continue;
+    const body = code.slice(a.bodyStart, a.bodyEnd);
+    inAttributes += body.match(RELAX_UNSAFE)?.length ?? 0;
+    const after = a.inner ? { item: "its whole module or crate (#!)", export: null } : exportAfter(code, mask, a.end, attrs);
+    const placed = f === HASH_WASM_UNSAFE_FILE && !a.inner && isPlacedAllow(body) && after.export;
+    if (placed) sites.set(after.export, (sites.get(after.export) ?? 0) + 1);
+    else {
+      const on = a.inner ? after.item : `\`${after.item}\``;
+      problems.push(
+        `${f}:${lineAt(code, a.start)} relaxes unsafe_code on ${on}; npt-hash-wasm may only put #[allow(unsafe_code, ` +
+          `reason = "...")] directly on ${lists}`,
+      );
+    }
+  }
+  const all = code.match(RELAX_UNSAFE)?.length ?? 0;
+  if (all > inAttributes) problems.push(`${f} names an unsafe_code relaxing outside an attribute (a string or a macro's text)`);
+  for (const m of mask.matchAll(/\bunsafe_code\b/g)) {
+    if (!inAttribute(m.index)) {
+      problems.push(`${f}:${lineAt(mask, m.index)} names unsafe_code outside an attribute (a macro can build the attribute from it)`);
+    }
+  }
+  // The one `unsafe` each listed export may have: its #[unsafe(no_mangle)]'s.
+  const allowed = new Set();
+  for (const a of attrs) {
+    const body = mask.slice(a.bodyStart, a.bodyEnd);
+    if (a.inner || f !== HASH_WASM_UNSAFE_FILE || !NO_MANGLE.test(body)) continue;
+    if (exportAfter(code, mask, a.end, attrs).export) allowed.add(a.bodyStart + body.indexOf("unsafe"));
+  }
+  for (const m of mask.matchAll(UNSAFE_CODE)) {
+    if (allowed.has(m.index)) continue;
+    const a = attrs.find((x) => m.index >= x.start && m.index < x.end);
+    const where = a
+      ? `in #[${code.slice(a.bodyStart, a.bodyEnd).trim()}] on \`${a.inner ? "its whole module or crate (#!)" : exportAfter(code, mask, a.end, attrs).item}\``
+      : "outside an attribute";
+    problems.push(
+      `${f}:${lineAt(mask, m.index)} has \`${m[0]}\` ${where}; npt-hash-wasm's only unsafe is the #[unsafe(no_mangle)] ` +
+        `of ${lists}`,
+    );
+  }
+  // Each listed export's signature: u32 or i32 in and out.
+  if (f === HASH_WASM_UNSAFE_FILE) {
+    for (const m of code.matchAll(/\bpub\s+extern\s+"C"\s+fn\s+([A-Za-z_]\w*)\s*\(/g)) {
+      if (!HASH_WASM_EXPORTS.includes(m[1]) || mask.slice(m.index, m.index + 3) !== "pub") continue;
+      const open = m.index + m[0].length - 1;
+      let depth = 0;
+      let close = open;
+      do {
+        if (mask[close] === "(") depth++;
+        else if (mask[close] === ")") depth--;
+        close++;
+      } while (depth > 0 && close < mask.length);
+      const params = mask.slice(open + 1, close - 1).split(",").map((p) => p.trim()).filter(Boolean);
+      const types = params.map((p) => p.slice(p.indexOf(":") + 1).trim());
+      const result = mask.slice(close).match(/^\s*(?:->\s*([^{]*?))?\s*(?:where\b[^{]*)?\{/)?.[1]?.trim();
+      const bad = [...types.map((t, k) => (EXPORT_INTEGERS.includes(t) ? null : `parameter ${params[k]}`)), ...(result === undefined || EXPORT_INTEGERS.includes(result) ? [] : [`result ${result}`])].filter(Boolean);
+      if (bad.length || !params.every((p) => p.includes(":"))) {
+        problems.push(
+          `${f}:${lineAt(code, m.index)} exports ${m[1]} with ${bad.join(" and ") || "a parameter it cannot read"}: npt-hash-wasm's ` +
+            "exports take and give u32 or i32 only (the advisor's U4D2-Q7 ruling)",
+        );
+      }
+    }
+  }
+  return problems;
+}
+
 /**
  * Whether a crate manifest has `[lints]` with `workspace = true`, read from tomlScan's mask, as lintTables reads, so a
  * header or a key inside a string or a comment is none (review C2c N2: the two lines moved into a multi-line string,
@@ -472,13 +620,16 @@ function inheritsWorkspaceLints(text) {
  * Every place under crates/ that breaks the unsafe_code rule, as a sentence; empty when none does (u3c-plan.md 1.8 (a)
  * and (d)). `files` are paths relative to the spike's folder (crates/<crate>/...): its .rs files and crate manifests.
  * What is caught, as lexRust reads the code (its known limit is there):
- * - outside the two exception crates, any `allow`, `expect` or `warn` of unsafe_code in a .rs file, comments included
+ * - outside the three exception crates, any `allow`, `expect` or `warn` of unsafe_code in a .rs file, comments included
  *   (and a comment or literal inside the attribute's list), and any `unsafe` or `global_asm!` in code;
- * - a manifest other than the two exceptions' that names unsafe_code outside its comments (UNSAFE_CODE_NAME:
+ * - a manifest other than the three exceptions' that names unsafe_code outside its comments (UNSAFE_CODE_NAME:
  *   `unsafe-code` too) or does not inherit the workspace lints, as read outside its strings and comments
  *   (inheritsWorkspaceLints; the exceptions' own tables are exceptionLintProblems');
  * - in npt-tcpinfo, any relaxing but the two placed allows, `unsafe` outside their two functions, the two clippy lints
  *   named outside lib.rs's crate-level deny or that deny missing (tcpinfoProblems), and either allow missing;
+ * - in npt-hash-wasm, any relaxing but one placed allow on each listed export, any `unsafe` but a listed export's
+ *   `#[unsafe(no_mangle)]`, a listed export whose parameters or result are not u32 or i32 (hashWasmProblems), and a
+ *   listed export with no allow or two;
  * - in every crate, a #[path] or include! whose literal names anything but a .rs file inside the crate's folder
  *   (pathProblems, with its known limit), the way another crate could compile npt-tcpinfo's file under its own name.
  * npt-client-wasm's .rs files are not scanned for unsafe code: its manifest allows the lint for the whole crate.
@@ -489,8 +640,9 @@ function inheritsWorkspaceLints(text) {
 export function unsafeProblems(files, read) {
   const problems = [];
   const sites = new Map();
+  const hashSites = new Map();
   const state = { rootDeny: false };
-  const others = Object.values(UNSAFE_EXCEPTIONS).join(" and ");
+  const others = exceptionNames();
   for (const f of files) {
     const crate = f.match(/^crates\/[^/]+\//)?.[0];
     if (!crate) continue;
@@ -506,6 +658,7 @@ export function unsafeProblems(files, read) {
         }
       }
       if (crate === "crates/tcpinfo/") problems.push(...tcpinfoProblems(f, code, mask, attrs, sites, state));
+      if (crate === "crates/hash-wasm/") problems.push(...hashWasmProblems(f, code, mask, attrs, hashSites));
       problems.push(...pathProblems(f, crate, code, mask, attrs));
     } else if (f === `${crate}Cargo.toml` && !(crate in UNSAFE_EXCEPTIONS)) {
       // The name is looked for outside comments, strings included (a quoted key is the lint to cargo; a string that
@@ -522,6 +675,10 @@ export function unsafeProblems(files, read) {
   if (!state.rootDeny) {
     problems.push(`${TCPINFO_ROOT} does not deny clippy::${TCPINFO_CLIPPY_LINTS.join(" and clippy::")} at the crate root`);
   }
+  for (const fn of HASH_WASM_EXPORTS) {
+    const n = hashSites.get(fn) ?? 0;
+    if (n !== 1) problems.push(`npt-hash-wasm's export ${fn} has ${n} #[allow(unsafe_code, reason = ...)] attributes, not 1`);
+  }
   return problems;
 }
 
@@ -990,7 +1147,7 @@ function lintTables(text, prefix, file) {
 
 /**
  * The workspace manifest's unsafe_code lint, which the whole rule rests on: `[workspace.lints.rust] unsafe_code =
- * "forbid"` (or `{ level = "forbid", priority = n }`), the compiler backstop (review C2R-1): every member but the two
+ * "forbid"` (or `{ level = "forbid", priority = n }`), the compiler backstop (review C2R-1): every member but the three
  * exceptions inherits it, and under it rustc refuses unsafe code and any allow of the lint (E0453), a macro-built one
  * included. Every line naming unsafe_code outside strings and comments, in any case and with `-` for `_` (cargo reads
  * `unsafe-code` as the same lint), must be that entry; a form lintTables does not read, or another level, place or
@@ -1011,11 +1168,11 @@ export function workspaceLintProblems(text) {
 }
 
 /**
- * The two exceptions' own lint tables (review C2R-1; replacing u3c-plan.md 1.8 (a)'s "keep [lints] workspace = true"
+ * The three exceptions' own lint tables (review C2R-1; replacing u3c-plan.md 1.8 (a)'s "keep [lints] workspace = true"
  * for npt-tcpinfo, the advisor's ruling of 2026-11-08): each crate's `[lints.<tool>]` tables hold exactly the
  * workspace's `[workspace.lints.<tool>]` keys and values, every tool and key, but unsafe_code, which is
- * EXCEPTION_LEVELS' level: "deny" for npt-tcpinfo (its two #[allow]s need a level they can relax), "allow" for
- * npt-client-wasm. Key for key as cargo reads the manifest: any lint given in a form lintTables does not read (a lint's
+ * EXCEPTION_LEVELS' level: "deny" for npt-tcpinfo (its two #[allow]s need a level they can relax) and for
+ * npt-hash-wasm (its seven, U4d2), "allow" for npt-client-wasm. Key for key as cargo reads the manifest: any lint given in a form lintTables does not read (a lint's
  * own sub-table, a quoted, dotted, hyphenated or uppercase name, a dotted key or an inline table elsewhere) is reported
  * with them (review C2b F1: a `[lints.clippy.iter-over-hash-type]` table with level "allow" had relaxed the clippy
  * deny unseen). So leaving the workspace's lints costs them nothing else. `read(path)` reads a file under the spike.
@@ -1283,6 +1440,218 @@ export function cargoConfigProblems(env, configs) {
   return problems;
 }
 
+/**
+ * Every rustflags source cargo may read for a build (https://doc.rust-lang.org/cargo/reference/config.html,
+ * build.rustflags: CARGO_ENCODED_RUSTFLAGS, RUSTFLAGS, the target tables, build.rustflags), each as { where, flags }
+ * with its flags as rustc would get them: the environment's *RUSTFLAGS variables (CARGO_ENCODED_RUSTFLAGS split at
+ * \x1f, the others at whitespace; CARGO_TARGET_<triple>_RUSTFLAGS and CARGO_BUILD_RUSTFLAGS among them) and every
+ * `rustflags` key of the cargo configs read (a string split at whitespace, an array as it is), in any table. A value
+ * this check cannot read is cargoConfigProblems' to report.
+ */
+export function rustflagSources(env, configs) {
+  const sources = [];
+  for (const [k, v] of Object.entries(env)) {
+    if (typeof v !== "string" || !/RUSTFLAGS$/.test(k)) continue;
+    sources.push({ where: `${k} in the environment`, flags: v.split(k === "CARGO_ENCODED_RUSTFLAGS" ? "\x1f" : /\s+/).filter(Boolean) });
+  }
+  for (const { path, text } of configs) {
+    for (const v of tomlValues(tomlScan(text), "rustflags")) {
+      if (v === null) continue;
+      sources.push({ where: `${path}'s rustflags`, flags: v.length === 1 ? v[0].split(/\s+/).filter(Boolean) : v });
+    }
+  }
+  return sources;
+}
+
+/** The target features plan.md §4.5's build flags leave off (baseline x86-64, simd128 and relaxed-simd off; no FMA). */
+const REFUSED_FEATURES = ["fma", "simd128", "relaxed-simd"];
+/** The CPU name plan.md §4.5 refuses: `target-cpu=native` builds for the machine that compiles. */
+const REFUSED_CPUS = ["native"];
+
+/**
+ * The refused build flags in the rustflags sources (u4d2-brief.md 2.4 (c) (i); the advisor's condition 7 (a); ADR
+ * 0002:256-258; plan.md §4.5): `+fma`, `+simd128` or `+relaxed-simd` in a target-feature list, and
+ * `target-cpu=native`, in every form rustc reads a codegen option (`-C opt`, `-Copt`, `--codegen opt`,
+ * `--codegen=opt`; the option split across two tokens or not; `_` for `-` in its name), each item of a comma list read;
+ * a feature named without a sign is read as enabled, which fails closed. As sentences, naming the source and the flag.
+ */
+export function buildFlagProblems(sources) {
+  const problems = [];
+  for (const { where, flags } of sources) {
+    const options = [];
+    for (let i = 0; i < flags.length; i++) {
+      const t = flags[i];
+      if (t === "-C" || t === "--codegen") {
+        if (i + 1 < flags.length) options.push(flags[++i]);
+      } else if (t.startsWith("--codegen=")) options.push(t.slice("--codegen=".length));
+      else if (t.startsWith("-C")) options.push(t.slice(2));
+    }
+    for (const o of options) {
+      const eq = o.indexOf("=");
+      const name = (eq < 0 ? o : o.slice(0, eq)).replace(/_/g, "-");
+      const value = eq < 0 ? "" : o.slice(eq + 1);
+      if (name === "target-feature") {
+        for (const item of value.split(",").map((s) => s.trim())) {
+          const feature = item.replace(/^[+-]/, "");
+          if (REFUSED_FEATURES.includes(feature) && !item.startsWith("-")) {
+            problems.push(`${where} passes ${JSON.stringify(`-C ${o}`)}: it enables ${feature}, which plan.md §4.5's build flags refuse`);
+          }
+        }
+      } else if (name === "target-cpu" && REFUSED_CPUS.includes(value)) {
+        problems.push(`${where} passes ${JSON.stringify(`-C ${o}`)}: plan.md §4.5's build flags refuse target-cpu=native`);
+      }
+    }
+  }
+  return problems;
+}
+
+/**
+ * rustc's read-back of the target features (the advisor's condition 7 (a): it fails the step, never only reports; the
+ * advisor's U4D2-Q4 ruling): `rustc --print cfg --target <triple>` run from the spike's folder (rust-toolchain.toml
+ * picks the toolchain cargo uses), given every rustflag of every source read, a superset of the one source cargo uses,
+ * which fails closed (a CPU name meant for one target empties or cuts the other's list: rustc 1.99.1, the advisor's
+ * U4d2 brief check, A15). Per target: the features it must not list, and the one a read-back that read anything lists
+ * (non-vacuity: a parse that reads nothing never passes).
+ */
+export const READ_BACK = [
+  { target: "x86_64-unknown-linux-gnu", refused: ["fma"], present: "sse2" },
+  { target: "wasm32-unknown-unknown", refused: ["simd128", "relaxed-simd"], present: "sign-ext" },
+];
+
+/** The read-back through `runner` (from `here`): { problems, line }, the line naming rustc's version and each list. */
+export function readBackCheck(sources, runner, here) {
+  const flags = sources.flatMap((s) => s.flags);
+  const given = flags.length ? `given ${JSON.stringify(flags.join(" "))}` : "given no rustflags";
+  const problems = [];
+  const lists = [];
+  const version = runner("rustc", ["--version"], here);
+  const rustc = version.ok ? (version.stdout || version.out).trim().split("\n")[0] : "rustc (its version did not print)";
+  if (!version.ok) problems.push(`rustc --version failed in ${here}: the read-back cannot run (${lastLines(version.out, 1)})`);
+  for (const { target, refused, present } of READ_BACK) {
+    const r = runner("rustc", ["--print", "cfg", "--target", target, ...flags], here);
+    if (!r.ok) {
+      problems.push(`rustc --print cfg --target ${target} (${given}) failed: ${lastLines(r.out, 1)}`);
+      continue;
+    }
+    const features = [...(r.stdout || r.out).matchAll(/^target_feature="([^"]+)"\r?$/gm)].map((m) => m[1]);
+    lists.push(`${target} ${features.join(", ") || "none"}`);
+    for (const f of refused) {
+      if (features.includes(f)) problems.push(`rustc's read-back for ${target} (${given}) lists target_feature="${f}", which plan.md §4.5's build flags refuse`);
+    }
+    if (!features.includes(present)) {
+      problems.push(
+        `rustc's read-back for ${target} (${given}) lists no target_feature="${present}": it read nothing a build for it has ` +
+          "(a CPU name meant for the other target, or a parse that reads nothing), so it proves nothing",
+      );
+    }
+  }
+  return { problems, line: `${rustc}'s read-back, ${given}: ${lists.join("; ")}` };
+}
+
+/**
+ * The builds whose npt-sim must resolve one feature set, like for like (ADR 0004:1184, PM-0004-74; u4-build-plan.md
+ * §5 U4d2; the advisor's U4D2-Q6 ruling): the server's, npt-lab's (the native legs of the determinism check) and
+ * npt-hash-wasm's for wasm32 (its Wasm leg); measured with measured, bites (`--features bites`) with bites.
+ */
+export const SIM_BUILDS = [
+  { pkg: "npt-server", target: "x86_64-unknown-linux-gnu" },
+  { pkg: "npt-lab", target: "x86_64-unknown-linux-gnu" },
+  { pkg: "npt-hash-wasm", target: "wasm32-unknown-unknown" },
+];
+
+/** The `cargo tree` arguments for one build: its normal edges, every package once per path, each with its features. */
+export const treeArgs = ({ pkg, target }, bites) => [
+  "tree",
+  "--locked",
+  "-e",
+  "normal",
+  "--target",
+  target,
+  "-p",
+  pkg,
+  ...(bites ? ["--features", "bites"] : []),
+  "--no-dedupe",
+  "--prefix",
+  "depth",
+  "--format",
+  "{p} [{f}]",
+];
+
+/**
+ * npt-sim's subtree in one `cargo tree` output (treeArgs's form: `<depth><name> v<version>[ (<path>)] [<features>]`):
+ * a Map from `<name> v<version>` to its features, sorted and joined, npt-sim's own included; null when npt-sim is not
+ * in the tree. The first npt-sim met is walked, down to the next line at its depth or above (--no-dedupe repeats every
+ * subtree in full, so none is cut short by a `(*)`); one build resolves one feature set per package, so every npt-sim
+ * in a tree is the same.
+ */
+export function simSubtree(text) {
+  const rows = text
+    .split("\n")
+    .map((l) => l.trimEnd().match(/^(\d+)(\S+) v(\S+)(?: \(.*\))? \[([^\]]*)\]$/))
+    .filter(Boolean);
+  const at = rows.findIndex((m) => m[2] === "npt-sim");
+  if (at < 0) return null;
+  const depth = Number(rows[at][1]);
+  const out = new Map();
+  for (let i = at; i < rows.length && (i === at || Number(rows[i][1]) > depth); i++) {
+    const [, , name, version, features] = rows[i];
+    out.set(`${name} v${version}`, features.split(",").map((s) => s.trim()).filter(Boolean).sort().join(", "));
+  }
+  return out;
+}
+
+/**
+ * npt-sim's resolved features compared across SIM_BUILDS (u4d2-brief.md 2.4 (b)), through `runner` (`cargo tree` from
+ * `here`): { problems, line }. Per feature set (measured, then bites), each pair of builds' npt-sim subtrees,
+ * package by package (npt-sim's own features and each package below it, transitive ones included): a package missing
+ * from one, or with other features, fails, naming it, the two builds and the features. Not vacuous: each
+ * subtree must hold npt-sim and each normal dependency npt-sim's own manifest names (`meta`'s npt-sim package, the
+ * cargo metadata the deps step takes). A measured tree is never compared with a bites tree.
+ */
+export function simFeatureCheck(meta, runner, here) {
+  const problems = [];
+  const sim = meta.packages.find((p) => p.name === "npt-sim" && meta.workspace_members.includes(p.id));
+  const direct = (sim?.dependencies ?? []).filter((d) => d.kind === null || d.kind === undefined).map((d) => d.name);
+  const shown = [];
+  for (const bites of [false, true]) {
+    const set = bites ? "bites" : "measured";
+    const name = (b) => `${b.pkg} (${b.target}, ${set})`;
+    const trees = [];
+    for (const b of SIM_BUILDS) {
+      const r = runner("cargo", treeArgs(b, bites), here);
+      if (!r.ok) {
+        problems.push(`cargo tree for the ${name(b)} build failed: ${lastLines(r.out, 1)}`);
+        continue;
+      }
+      const tree = simSubtree(r.stdout || r.out);
+      if (!tree) {
+        problems.push(`the ${name(b)} build's tree holds no npt-sim subtree, so its features prove nothing`);
+        continue;
+      }
+      const names = [...tree.keys()].map((k) => k.split(" ")[0]);
+      for (const d of direct) {
+        if (!names.includes(d)) problems.push(`the ${name(b)} build's npt-sim subtree lacks ${d}, which crates/sim/Cargo.toml names`);
+      }
+      trees.push({ b, tree });
+    }
+    // Every pair of builds in the set, so a difference names each build it sets apart.
+    for (let i = 0; i < trees.length; i++) {
+      for (let j = i + 1; j < trees.length; j++) {
+        const [p, q] = [trees[i], trees[j]];
+        for (const pkg of new Set([...p.tree.keys(), ...q.tree.keys()])) {
+          const [x, y] = [p.tree.get(pkg), q.tree.get(pkg)];
+          if (x === y) continue;
+          const show = (v) => (v === undefined ? "is not in its npt-sim subtree" : `[${v}]`);
+          problems.push(`npt-sim's ${pkg} resolves ${show(x)} in the ${name(p.b)} build and ${show(y)} in the ${name(q.b)} build`);
+        }
+      }
+    }
+    const first = trees[0];
+    if (first) shown.push(`${set}: ${[...first.tree].map(([k, v]) => `${k.split(" ")[0]} [${v}]`).join(", ")}`);
+  }
+  return { problems, line: `npt-sim's features, one set in ${SIM_BUILDS.map((b) => b.pkg).join(", ")}: ${shown.join("; ")}` };
+}
+
 /** The manifest keys manifestKeyProblems refuses, with why. */
 const MANIFEST_KEYS = {
   "cargo-features": "it unlocks unstable manifest keys (a profile's rustflags among them)",
@@ -1443,10 +1812,36 @@ const WASM_ARGS = ["--target", "wasm32-unknown-unknown", ...WASM_CRATES.flatMap(
  * its output, not cargo's last line ("warning: build failed, waiting for other jobs to finish..."; the rust fix2
  * review's nit, u4s-carries.md entry 34): `what` says which run failed. The detail stays in the lines above it.
  */
-export function failedRun(r, what, before = "") {
+export function failedRun(r, what, before = "", note = "") {
   const first = r.out.split("\n").find((l) => /^error(?:\[\w+\])?:/.test(l.trim()));
   const said = first ? first.trim() : "its output has no error line (the lines above)";
-  return { ...r, ok: false, out: `${before}${r.out}\n${what} failed: ${said}` };
+  return { ...r, ok: false, out: `${before}${r.out}${note}\n${what} failed: ${said}` };
+}
+
+/**
+ * The failing tests' names, from libtest's `failures:` lists in a test run's stdout (the list after the panics, one
+ * name a line, four spaces in), in order, each once (carry 65; the advisor's U4j round-2 ruling, B2.3: `run` joins
+ * stdout before stderr and main prints a failed step's last 40 lines, so without this line the names never show).
+ */
+export function failingTests(stdout) {
+  const names = [];
+  const lines = stdout.split("\n").map((l) => l.replace(/\r$/, ""));
+  for (let i = 0; i < lines.length; i++) {
+    if (lines[i] !== "failures:") continue;
+    for (let j = i + 1; j < lines.length && /^ {4}\S/.test(lines[j]); j++) {
+      const name = lines[j].trim();
+      if (!names.includes(name)) names.push(name);
+    }
+  }
+  return names;
+}
+
+/** The line a failed test run gets just above its summary: its failing tests by name, or why none is named. */
+function failingTestsLine(r, build) {
+  const names = failingTests(r.stdout ?? r.out);
+  return names.length
+    ? `\n${build}'s failing tests (libtest's failures: list): ${names.join(", ")}`
+    : `\n${build}: no failing test in libtest's failures: list (a build or harness error: the lines above)`;
 }
 
 /**
@@ -1489,13 +1884,16 @@ export function docBothBuilds(runner = run, base = process.env) {
 }
 
 /** Both builds' tests: the measured build (no features), then the bites build (every feature, the planted defects
- * compiled in: plan §4.7). Each count is reported, skips included (cargoCounts). Through `runner`, as lintBothBuilds. */
+ * compiled in: plan §4.7). Each count is reported, skips included (cargoCounts); a failed run names its failing tests
+ * just above its summary (failingTests: carry 65). Through `runner`, as lintBothBuilds. */
 export function testBothBuilds(runner = run) {
   const measured = runner("cargo", ["test", "--locked", "--workspace"], HERE);
-  if (!measured.ok) return failedRun(measured, `the measured build's tests (${cargoCounts(measured.out)})`);
+  if (!measured.ok) {
+    return failedRun(measured, `the measured build's tests (${cargoCounts(measured.out)})`, "", failingTestsLine(measured, "the measured build"));
+  }
   const bites = runner("cargo", ["test", "--locked", "--workspace", "--all-features"], HERE);
   const summary = `measured build: ${cargoCounts(measured.out)}; bites build: ${cargoCounts(bites.out)}`;
-  if (!bites.ok) return failedRun(bites, `the bites build's tests (${summary})`, measured.out);
+  if (!bites.ok) return failedRun(bites, `the bites build's tests (${summary})`, measured.out, failingTestsLine(bites, "the bites build"));
   return { ok: true, out: `${measured.out}${bites.out}\n${summary}`, status: bites.status };
 }
 
@@ -1507,11 +1905,15 @@ export function testBothBuilds(runner = run) {
  *
  * The static unsafe scan guards against an accidental relaxation; review, and U5's audit, are the wall against a
  * deliberate one. Between them stands the compiler: under the workspace's forbid, rustc refuses unsafe code and any
- * allow of the lint (E0453, a macro-built allow included) in every crate but the two exceptions, whatever the scan
+ * allow of the lint (E0453, a macro-built allow included) in every crate but the three exceptions, whatever the scan
  * misreads. Known limits, which the step does not report:
  * - lexRust is not rustc's lexer (a shebang line; a `\u{...}` escape with underscores past six characters: review
  *   C2R-1), and pathProblems reads a #[path] or include! literal with its escapes undecoded and include! by its name
- *   only (C2R-2). The compiler covers both in the crates under the forbid; inside npt-tcpinfo only review does.
+ *   only (C2R-2). The compiler covers both in the crates under the forbid; inside npt-tcpinfo and npt-hash-wasm only
+ *   review does.
+ * - The build-flag scan and rustc's read-back read the rustflags sources above, not a build script's
+ *   `cargo::rustc-flags` or a RUSTC_WRAPPER's own flags; the read-back is given every source's flags at once (a superset
+ *   of what cargo passes, which fails closed: the advisor's U4D2-Q4 ruling).
  * - Inputs that can lower lints and are not read here: a cargo config's [env] table (CLIPPY_ARGS, C2R-4 G, or any
  *   other variable it sets for rustc), what a member's build script prints (`cargo::rustc-env`, C2R-4 H), a
  *   RUSTC_WRAPPER program (allowed, for sccache) and a rust-toolchain.toml `path`. Lint capping by any of them
@@ -1524,23 +1926,30 @@ export function testBothBuilds(runner = run) {
  *   variable, a patched or rebuilt cargo or rustc) is out of the scan's reach.
  * - Unsafe code a dependency's macro expands into a crate is not linted by rustc (another crate's code); the
  *   dependencies are U5's audit.
- * - Too wide, not too narrow: the runner refusal covers every target triple, so a future wasm32 test runner
- *   (wasm-bindgen-test's `[target.wasm32-unknown-unknown] runner`) would be refused too, though the test step runs
- *   host tests only (no wasm test is planned; review C2b's note).
+ * - Too wide, not too narrow: the runner refusal covers every target triple, so a wasm32 test runner
+ *   (wasm-bindgen-test's `[target.wasm32-unknown-unknown] runner`) would be refused too, though the test step runs host
+ *   tests only (review C2b's note). The wasm step's Wasm legs need none: they are a Node script (crates/hash-wasm's
+ *   loader, run through process.execPath), not a cargo test, so the refusal stands (u4-build-plan.md §5 U4d2).
  */
 const DEPS_CHECKED =
   "held: no leaf crate reaches a transport or runtime; npt-client has no npt-net; npt-super and npt-host keep off the " +
   "sim, room, proto (and client) crates; npt-judge ships npt-log and serde only; npt-tcpinfo ships libc only and only " +
-  "npt-net uses it; npt-net has no direct libc; npt-sim ships libm, rand_chacha and rand_core only; nothing depends " +
-  "on npt-lab; the workspace forbids unsafe_code and " +
-  "every member inherits its lints but npt-client-wasm (allow) and npt-tcpinfo (deny), whose own lint tables are the " +
-  "workspace's otherwise, key for key, with no lint given in a form the reader does not read (a lint's own table, a " +
-  "quoted, dotted, hyphenated or uppercase name); no manifest has a cargo-features or rustflags key; as the scan " +
-  "reads crates/ (lexRust), no other crate relaxes unsafe_code or has unsafe code, " +
-  "and npt-tcpinfo relaxes it only by the #[allow] on srtt_us and unacked_and_unsent, has unsafe code only inside " +
-  "them, and keeps lib.rs's deny of the two clippy unsafe lints; the environment and the cargo configs read pass no " +
+  "npt-net uses it; npt-net has no direct libc; npt-sim ships libm, rand_chacha and rand_core only; npt-hash-wasm " +
+  "ships npt-sim only; nothing depends on npt-lab or npt-hash-wasm; the workspace forbids unsafe_code and " +
+  "every member inherits its lints but npt-client-wasm (allow), npt-tcpinfo (deny) and npt-hash-wasm (deny), whose own " +
+  "lint tables are the workspace's otherwise, key for key, with no lint given in a form the reader does not read (a " +
+  "lint's own table, a quoted, dotted, hyphenated or uppercase name); no manifest has a cargo-features or rustflags " +
+  "key; as the scan reads crates/ (lexRust), no other crate relaxes unsafe_code or has unsafe code, " +
+  "npt-tcpinfo relaxes it only by the #[allow] on srtt_us and unacked_and_unsent, has unsafe code only inside " +
+  "them, and keeps lib.rs's deny of the two clippy unsafe lints, and npt-hash-wasm relaxes it only by one #[allow] on " +
+  "each of its seven listed exports, whose #[unsafe(no_mangle)] is its only unsafe code, each taking and giving u32 " +
+  "or i32 only; the environment and the cargo configs read pass no " +
   "lint-lowering rustflags and set no clippy or fmt alias, no other rustc or workspace wrapper, no config include and " +
-  "no test runner, and the environment has no RUSTC_BOOTSTRAP or cargo channel override; every member lives in " +
+  "no test runner, and the environment has no RUSTC_BOOTSTRAP or cargo channel override; no rustflags source enables " +
+  "fma, simd128 or relaxed-simd or sets target-cpu=native, and rustc's read-back given every flag read lists sse2 and " +
+  "no fma for x86-64 and sign-ext and no simd128 or relaxed-simd for wasm32; npt-sim's resolved features, its own " +
+  "and every package's below it, are one set in the npt-server, npt-lab and npt-hash-wasm (wasm32) builds, measured " +
+  "with measured and bites with bites, each holding npt-sim's own dependencies; every member lives in " +
   "crates/ and builds only .rs files in its own folder, in edition 2024, none " +
   "a proc-macro, and every path dependency is a member; no symbolic link under crates/; every #[path] and include! " +
   "literal names a .rs file in its own crate; TICK_HZ is set from a literal once, in npt-sim; each derived tick " +
@@ -1556,11 +1965,13 @@ function cargoMetadata(here) {
 }
 
 /**
- * The deps step on the spike in `here`, with `env` as the environment and `metadata(here)` for cargo metadata: every
- * check it makes is called here, and STEPS.deps is this function (review C2R-5: check.test.mjs runs it on a fixture
- * with one plant per check, so a check unhooked from it fails a test).
+ * The deps step on the spike in `here`, with `env` as the environment, `metadata(here)` for cargo metadata and
+ * `runner` for the `cargo tree` and `rustc` calls of the features comparison and the read-back: every check it makes is
+ * called here, and STEPS.deps is this function (review C2R-5: check.test.mjs runs it on a fixture with one plant per
+ * check, so a check unhooked from it fails a test). Its summary line ends with the read-back's lists and npt-sim's
+ * features.
  */
-export function depsStep(here = HERE, env = process.env, metadata = cargoMetadata) {
+export function depsStep(here = HERE, env = process.env, metadata = cargoMetadata, runner = run) {
   const r = metadata(here);
   if (!r.ok) return r;
   const { meta } = r;
@@ -1570,6 +1981,9 @@ export function depsStep(here = HERE, env = process.env, metadata = cargoMetadat
   const workspace = read("Cargo.toml");
   const manifests = [{ path: "Cargo.toml", text: workspace }];
   for (const f of files) if (f.endsWith("/Cargo.toml")) manifests.push({ path: f, text: read(f) });
+  const sources = rustflagSources(env, configs);
+  const readBack = readBackCheck(sources, runner, here);
+  const features = simFeatureCheck(meta, runner, here);
   const problems = [
     ...dependencyProblems(meta),
     ...workspaceLintProblems(workspace),
@@ -1584,11 +1998,369 @@ export function depsStep(here = HERE, env = process.env, metadata = cargoMetadat
     ...biteHomeProblems(files, read),
     ...clippyTwinProblems(read),
     ...linkUseProblems(files, read),
+    ...buildFlagProblems(sources),
+    ...readBack.problems,
+    ...features.problems,
   ];
-  const counts = `${files.length} crate files and ${configs.length} cargo config file(s) read`;
+  const counts = `${files.length} crate files and ${configs.length} cargo config file(s) read; ${readBack.line}; ${features.line}`;
   return { ok: problems.length === 0, out: problems.join("\n") || `${DEPS_CHECKED} (${counts})` };
 }
 
+/** The recordings manifest (u4-build-plan.md §4.15; carry 52), the std-sin scan data and h2's committed vector. */
+export const RECORDINGS_MANIFEST = "crates/lab/recordings.json";
+export const STD_SIN_SCAN = "crates/lab/std-sin-scan.txt";
+export const H2_VECTOR = "crates/hash-wasm/co-initial-dump.txt";
+/** The Node loader of the Wasm legs. */
+export const LOADER = join(HERE, "crates", "hash-wasm", "loader.mjs");
+/** Where the wasm step's copies of its builds, its recordings and its legs' outputs go: the checkout's .gate/. */
+export const WASM_STEP_DIR = join(ROOT, ".gate", "netcode-proof-test", "wasm-step");
+/**
+ * The two bites the wasm step arms (PM-0004-21's, planted by U4d in npt-sim: plan.md §4.5:551, "one std `sin` and one
+ * hash-map iteration, each caught at the first tick it changes"), named here as ops/runs/c-u2-pm-0001-3.mjs names
+ * PM-0001-3's; each one's index (from 1, as npt-hash-wasm's `begin` takes it) is read from `npt-lab hash bites`.
+ */
+export const WASM_BITES = { hashMap: "PM-0004-21/hash-map", stdSin: "PM-0004-21/std-sin" };
+/** The hash-map bite's arming: the manifest's first recording at tick 1,234 (D-2's form, sim/tests/replay.rs:291). */
+export const HASH_MAP_ARM = 1_234;
+/** The state word its order is folded into (world.rs's write_state: two words under one name). */
+export const HASH_MAP_FIELD = "bite[0].hash_map_order";
+/**
+ * The step's release builds (u4d2-brief.md 2.4 (a) item 1: plan.md §4.5's flags, so none is added; wasm-opt is U7's,
+ * C-U7-1), each artifact read from cargo's JSON messages and copied out before the next build replaces it.
+ */
+export const WASM_BUILDS = [
+  { key: "lab", what: "npt-lab (x86-64, measured)", copy: "npt-lab", target: "npt-lab", kind: "bin", args: ["build", "--locked", "--release", "-p", "npt-lab", "--message-format=json"] },
+  { key: "labBites", what: "npt-lab (x86-64, bites)", copy: "npt-lab-bites", target: "npt-lab", kind: "bin", args: ["build", "--locked", "--release", "-p", "npt-lab", "--features", "bites", "--message-format=json"] },
+  {
+    key: "wasm",
+    what: "npt-hash-wasm (wasm32, measured)",
+    copy: "npt-hash-wasm.wasm",
+    target: "npt_hash_wasm",
+    kind: "cdylib",
+    args: ["build", "--locked", "--release", "--target", "wasm32-unknown-unknown", "-p", "npt-hash-wasm", "--message-format=json"],
+  },
+  {
+    key: "wasmBites",
+    what: "npt-hash-wasm (wasm32, bites)",
+    copy: "npt-hash-wasm-bites.wasm",
+    target: "npt_hash_wasm",
+    kind: "cdylib",
+    args: ["build", "--locked", "--release", "--target", "wasm32-unknown-unknown", "-p", "npt-hash-wasm", "--features", "bites", "--message-format=json"],
+  },
+];
+
+/** The artifact a build's JSON messages name: a bin's executable, a cdylib's .wasm file; null if none. */
+export function builtArtifact(stdout, { target, kind }) {
+  let found = null;
+  for (const line of stdout.split("\n")) {
+    if (!line.startsWith("{")) continue;
+    let m;
+    try {
+      m = JSON.parse(line);
+    } catch {
+      continue;
+    }
+    if (m.reason !== "compiler-artifact" || m.target?.name !== target || !m.target?.kind?.includes(kind)) continue;
+    found = kind === "bin" ? (m.executable ?? null) : ((m.filenames ?? []).find((f) => f.endsWith(".wasm")) ?? null);
+  }
+  return found;
+}
+
+/**
+ * One `<a> against <b>: ...` line of npt-lab's `hash` or `hash compare` (HashCompare's result, as compare_text writes
+ * it), read: { line, compared, mismatching, equal, first: { recording, tick } when both sides' first mismatching entries
+ * are the same tick, `at`: that entry as written, field: the first differing field when named, counts: each
+ * recording's ticks in its first list }; null when no such line was written (the run failed).
+ */
+export function readComparison(text, a, b) {
+  const head = `${a} against ${b}: `;
+  const line = text.split("\n").find((l) => l.startsWith(head));
+  if (!line) return null;
+  const body = line.slice(head.length);
+  const tail = body.match(/ \(([^()]*)\)$/);
+  const counts = new Map(
+    (tail?.[1] ?? "")
+      .split(", ")
+      .filter(Boolean)
+      .map((s) => [s.split(" ")[0], Number(s.split(" ")[1])]),
+  );
+  const rest = tail ? body.slice(0, tail.index) : body;
+  const compared = Number(rest.match(/^(\d+) ticks compared/)?.[1] ?? Number.NaN);
+  const mismatching = compared === 0 ? 0 : Number(rest.match(/, (\d+) mismatching/)?.[1] ?? Number.NaN);
+  const firstAt = rest.match(/; first mismatch at entry \d+: (.*)$/)?.[1] ?? null;
+  const sides = firstAt?.match(/^(\S+) tick (\d+) hash \S+ against (\S+) tick (\d+) hash \S+;/);
+  const first = sides && sides[1] === sides[3] && sides[2] === sides[4] ? { recording: sides[1], tick: Number(sides[2]) } : null;
+  return {
+    line,
+    compared,
+    mismatching,
+    equal: /, 0 mismatching, equal$/.test(rest),
+    first,
+    at: firstAt,
+    field: firstAt?.match(/; field (\S+) \(word \d+\)/)?.[1] ?? null,
+    counts,
+  };
+}
+
+/** The std-sin scan data (STD_SIN_SCAN; u4d2-brief.md 2.3): its catches (`catch <recording> <tick> <field>`), whether it
+ * says "not catchable at this site on x86-64 and wasm32", and its other lines (the scan's record). */
+export function readScan(text) {
+  const catches = [];
+  let notCatchable = false;
+  const record = [];
+  for (const raw of text.split("\n")) {
+    const l = raw.trim();
+    if (!l || l.startsWith("#")) continue;
+    const c = l.match(/^catch (\S+) (\d+) (\S+)$/);
+    if (c) catches.push({ recording: c[1], tick: Number(c[2]), field: c[3] });
+    else if (l === "not catchable at this site on x86-64 and wasm32") notCatchable = true;
+    else record.push(l);
+  }
+  return { catches, notCatchable, record };
+}
+
+/** The real file operations of the wasm step (check.test.mjs passes its own), paths under the spike for `read`. */
+const WASM_IO = {
+  reset: (dir) => {
+    rmSync(dir, { recursive: true, force: true });
+    mkdirSync(dir, { recursive: true });
+  },
+  mkdir: (dir) => mkdirSync(dir, { recursive: true }),
+  copy: (from, to) => copyFileSync(from, to),
+  exists: (path) => existsSync(path),
+  read: (path) => readFileSync(join(HERE, path), "utf8"),
+  libc: () => {
+    const v = process.report?.getReport?.()?.header?.glibcVersionRuntime;
+    return v ? `glibc ${v} (Node's report)` : "no glibc found by Node's report";
+  },
+};
+
+/**
+ * The determinism check (u4-build-plan.md §5 U4d2; plan.md §4.5:546-551; ADR 0002:500-505), through `runner` for every
+ * command (cargo, npt-lab, Node), `io` for its files and `now` for its clock (main's, Date.now, which times each call;
+ * npt-lab reads none):
+ * 1. the release builds (WASM_BUILDS), each artifact's path read from cargo's JSON and copied out;
+ * 2. `npt-lab hash` (measured): the manifest's recordings regenerated into the step's folder and checked by length and
+ *    SHA-256, then its two native legs in lock step, leg 1 against leg 2;
+ * 3. the Node loader, once per recording, its own process each (process.execPath): Wasm leg 1;
+ * 4. `npt-lab hash compare`: native leg 1 against Wasm leg 1, and at a first mismatch the loader again with that tick's
+ *    dump and the compare again, which names the field;
+ * 5. h2's vectors on wasm32: each header-only recording through the loader to its dump, then `npt-lab hash vectors`;
+ * 6. the bites half, in the bites builds: `npt-lab hash bites` for the indexes; PM-0004-21/hash-map on the manifest's
+ *    first recording at tick 1,234, four legs (native 1 and 2, Wasm 1 and 2, each its own process), three pairs by
+ *    D-2's rule (sim/tests/replay.rs:300-308): digests that differ must first differ at that recording, that tick,
+ *    HASH_MAP_FIELD; equal digests are "not exercised" and no tick may differ; no pair exercised fails; equal Wasm
+ *    digests are "not catchable on wasm32". PM-0004-21/std-sin by the scan data (STD_SIN_SCAN, the advisor's U4D2-Q5
+ *    ruling): a catch (recording R, tick T', field F) arms it at T' on R, native leg 1 and Wasm leg 1, whose first
+ *    mismatch must be (R, T', F); "not catchable at this site on x86-64 and wasm32" is printed with the scan's record,
+ *    and no std-sin leg runs.
+ * It passes only with every manifest recording's ticks compared (its count each, their sum per pair) and 0 mismatching
+ * for leg 1 against leg 2 and leg 1 against Wasm, h2's vectors equal and each bite's verdict as above. On a failure it
+ * still runs every comparison it has the legs for; its last lines are one line per pair, h2 and each bite, then each
+ * problem, then the summary, which main prints: the ticks per pair, the mismatches, each bite's outcome and each timed
+ * call's seconds.
+ */
+export function wasmStep(runner = run, io = WASM_IO, now = Date.now, dir = WASM_STEP_DIR) {
+  const problems = [];
+  const lines = [];
+  const timed = [];
+  const call = (label, cmd, args) => {
+    const t0 = now();
+    const r = runner(cmd, args, HERE);
+    timed.push(`${label} ${((now() - t0) / 1000).toFixed(1)}`);
+    return r;
+  };
+  const failure = (r) => lastLines(r.out, 1).trim() || `exit ${r.status}`;
+  const bin = join(dir, "bin");
+  const rec = join(dir, "recordings");
+  const meas = join(dir, "measured");
+  const hm = join(dir, "hash-map");
+  const sin = join(dir, "std-sin");
+  io.reset(dir);
+  for (const d of [bin, rec, meas, hm, sin]) io.mkdir(d);
+  // 1. The builds.
+  const art = {};
+  for (const b of WASM_BUILDS) {
+    const r = call(`build ${b.copy}`, "cargo", b.args);
+    const path = r.ok ? builtArtifact(r.stdout ?? "", b) : null;
+    if (!r.ok) problems.push(`the release build of ${b.what} failed: ${failure(r)}`);
+    else if (!path) problems.push(`the release build of ${b.what} names no ${b.kind} artifact in cargo's JSON messages`);
+    else {
+      art[b.key] = join(bin, b.copy);
+      io.copy(path, art[b.key]);
+    }
+  }
+  // The manifest's recordings, as the loader needs them (npt-lab checks the manifest's own rules).
+  let entries = [];
+  try {
+    const m = JSON.parse(io.read(RECORDINGS_MANIFEST));
+    entries = m.recordings.map((e) => ({ name: e.name, first: e.first_tick, ticks: e.ticks }));
+  } catch (e) {
+    problems.push(`${RECORDINGS_MANIFEST} cannot be read: ${e.message}`);
+  }
+  const total = entries.reduce((s, e) => s + e.ticks, 0);
+  const manifestArgs = ["--manifest", RECORDINGS_MANIFEST, "--dir", rec];
+  const loader = (wasm, e, out, extra = []) => [LOADER, "--wasm", wasm, "--rec", join(rec, `${e.name}.rec`), "--name", e.name, "--first-tick", String(e.first), "--out", out, ...extra];
+  const stepped = (r) => Number(r.out.match(/: (\d+) ticks stepped/)?.[1] ?? Number.NaN);
+  // A recording regenerated in this run (the folder is new each run, and npt-lab writes each file once it is checked).
+  const has = (e) => io.exists(join(rec, `${e.name}.rec`));
+  // One pair of the clean half: every recording at its count, the sum compared, 0 mismatching.
+  const judgePair = (what, c, r) => {
+    if (!c) {
+      problems.push(`${what}: no comparison was written${r ? `: ${failure(r)}` : " (no leg to compare)"}`);
+      return `${what}: not compared`;
+    }
+    if (c.compared !== total) problems.push(`${what}: ${c.compared} ticks compared, not the manifest's ${total}`);
+    for (const e of entries) {
+      if (c.counts.get(e.name) !== e.ticks) problems.push(`${what}: ${e.name} has ${c.counts.get(e.name) ?? 0} ticks compared, the manifest's ${e.ticks}`);
+    }
+    if (!c.equal) problems.push(`${what}: ${c.mismatching} mismatching; first mismatch at ${c.at ?? "no entry named"}`);
+    return c.line;
+  };
+  // 2-4. The clean half.
+  let vectors = [];
+  if (art.lab && entries.length) {
+    const legs = call("npt-lab hash", art.lab, ["hash", ...manifestArgs, "--out", meas]);
+    const regen = legs.out.match(/^npt-lab hash: (.*(?:the manifest has|not this build's Recording::GENERATOR).*)$/m)?.[1];
+    if (regen) problems.push(`the recordings manifest: ${regen}`);
+    if (/order digest/.test(legs.out)) problems.push("a clean-half native leg wrote an order digest: a bites build's output given to the measured half");
+    vectors = (legs.out.match(/^wrote the header-only recordings (.*) \(seed 1/m)?.[1] ?? "").split(", ").filter(Boolean);
+    lines.push(judgePair("native leg 1 against native leg 2", readComparison(legs.out, "native leg 1", "native leg 2"), legs));
+  } else if (!art.lab) problems.push("the clean half did not run: no measured npt-lab build");
+  if (art.lab && art.wasm && entries.some(has)) {
+    const theirs = [];
+    for (const e of entries.filter(has)) {
+      const out = join(meas, `wasm-${e.name}.lines`);
+      const r = call(`Wasm ${e.name}`, process.execPath, loader(art.wasm, e, out));
+      if (!r.ok) problems.push(`Wasm leg 1: ${failure(r)}`);
+      else if (stepped(r) !== e.ticks) problems.push(`Wasm leg 1: ${e.name} stepped ${stepped(r)} ticks, the manifest's ${e.ticks}`);
+      if (/\bbite\b/.test(r.out)) problems.push(`Wasm leg 1: ${e.name}'s leg carries a bite (a bites build's output given to the measured half)`);
+      if (io.exists(out)) theirs.push(out);
+    }
+    const compareArgs = ["hash", "compare", "--ours", join(meas, "native-1.lines"), "--theirs", theirs.join(","), ...manifestArgs];
+    const names = ["--ours-name", "native leg 1", "--theirs-name", "Wasm leg 1"];
+    const cr = theirs.length ? call("compare", art.lab, [...compareArgs, ...names]) : null;
+    let c = cr ? readComparison(cr.out, "native leg 1", "Wasm leg 1") : null;
+    const e = c?.first && !c.field ? entries.find((x) => x.name === c.first.recording) : null;
+    if (e) {
+      // The field diff: the loader again, with that tick's dump, then the compare naming the field.
+      const dump = join(meas, `wasm-${e.name}-${c.first.tick}.dump`);
+      const r = call(`Wasm dump ${e.name}`, process.execPath, loader(art.wasm, e, join(meas, `wasm-${e.name}-dump.lines`), ["--dump-at", String(c.first.tick), "--dump-out", dump]));
+      if (r.ok) c = readComparison(call("compare field", art.lab, [...compareArgs, "--theirs-dump", dump, ...names]).out, "native leg 1", "Wasm leg 1") ?? c;
+    }
+    lines.push(judgePair("native leg 1 against Wasm leg 1", c, cr));
+  } else problems.push(`the Wasm leg did not run: ${art.wasm ? "no recording was regenerated" : "no measured npt-hash-wasm build"}`);
+  // 5. h2's vectors on wasm32.
+  if (art.lab && art.wasm && vectors.length) {
+    for (const v of vectors) {
+      const r = call(`h2 ${v}`, process.execPath, loader(art.wasm, { name: v, first: 0 }, join(meas, `${v}.lines`), ["--dump-at", "end", "--dump-out", join(meas, `${v}.dump`)]));
+      if (!r.ok) problems.push(`h2: ${failure(r)}`);
+    }
+    const r = call("h2 vectors", art.lab, ["hash", "vectors", "--dir", rec, "--dumps", meas, "--committed", H2_VECTOR]);
+    const said = r.out.split("\n").filter((l) => l.startsWith("h2: "));
+    lines.push(...said);
+    const differs = said.filter((l) => l !== "h2: the vectors differ (the lines above)").map((l) => l.slice(4));
+    if (!r.ok) problems.push(`h2: the vectors differ or did not run: ${differs.join("; ") || failure(r)}`);
+  } else problems.push("h2: the vectors did not run (no build, or npt-lab wrote no header-only recording)");
+  // 6. The bites half.
+  const bites = { hashMap: "not run", stdSin: "not run" };
+  const index = {};
+  if (art.labBites && art.wasmBites && entries.some(has)) {
+    const listed = call("npt-lab bites", art.labBites, ["hash", "bites"]);
+    for (const m of listed.out.matchAll(/^(\d+) (\S+)$/gm)) index[m[2]] = Number(m[1]);
+    for (const id of Object.values(WASM_BITES)) if (!index[id]) problems.push(`npt-lab hash bites lists no ${id} (World::BITES)`);
+  } else problems.push(`the bites half did not run: ${art.labBites && art.wasmBites ? "no recording was regenerated" : "no bites builds"}`);
+  const rec0 = entries[0];
+  if (index[WASM_BITES.hashMap] && rec0 && !has(rec0)) problems.push(`hash-map: ${rec0.name} was not regenerated, so it did not run`);
+  if (index[WASM_BITES.hashMap] && rec0 && has(rec0)) {
+    const arm = String(HASH_MAP_ARM);
+    const bite = ["--bite", WASM_BITES.hashMap, "--arm", arm, "--on", rec0.name];
+    const legs = call("hash-map native legs", art.labBites, ["hash", ...manifestArgs, "--out", hm, "--only", rec0.name, "--bite", WASM_BITES.hashMap, "--arm", arm, "--dump-at", arm]);
+    const digests = { "native leg 1": legs.out.match(/^leg 1 order digest (\S+)/m)?.[1], "native leg 2": legs.out.match(/^leg 2 order digest (\S+)/m)?.[1] };
+    for (const k of [1, 2]) {
+      const r = call(`hash-map Wasm ${k}`, process.execPath, loader(art.wasmBites, rec0, join(hm, `wasm-${k}.lines`), ["--bite", String(index[WASM_BITES.hashMap]), "--arm", arm, "--dump-at", arm, "--dump-out", join(hm, `wasm-${k}.dump`)]));
+      if (!r.ok) problems.push(`hash-map Wasm leg ${k}: ${failure(r)}`);
+    }
+    const pair = (label, a, b, ours, theirs) => {
+      const r = call(label, art.labBites, ["hash", "compare", "--ours", join(hm, `${ours}.lines`), "--theirs", join(hm, `${theirs}.lines`), ...manifestArgs, "--ours-dump", join(hm, `${ours}.dump`), "--theirs-dump", join(hm, `${theirs}.dump`), "--digest-field", HASH_MAP_FIELD, ...bite, "--ours-name", a, "--theirs-name", b]);
+      for (const n of [a, b]) digests[n] ??= r.out.match(new RegExp(`^${n} digest (\\S+) \\(`, "m"))?.[1];
+      return readComparison(r.out, a, b);
+    };
+    const n12 = readComparison(legs.out, "native leg 1", "native leg 2");
+    const nw = pair("hash-map compare native-Wasm", "native leg 1", "Wasm leg 1", "native-1", "wasm-1");
+    const ww = pair("hash-map compare Wasm-Wasm", "Wasm leg 1", "Wasm leg 2", "wasm-1", "wasm-2");
+    // D-2's rule, per pair (sim/tests/replay.rs:300-308).
+    const verdicts = [];
+    let exercised = 0;
+    for (const [a, b, c] of [["native leg 1", "native leg 2", n12], ["native leg 1", "Wasm leg 1", nw], ["Wasm leg 1", "Wasm leg 2", ww]]) {
+      const [da, db] = [digests[a], digests[b]];
+      const what = `hash-map, ${a} against ${b}`;
+      if (!c || !da || !db || da === "none" || db === "none") {
+        problems.push(`${what}: ${c ? `a leg logged no order digest (${da ?? "none"}, ${db ?? "none"})` : "no comparison was written (the lines above)"}`);
+        verdicts.push(`${a} - ${b} not judged`);
+        continue;
+      }
+      if (c.compared !== rec0.ticks) problems.push(`${what}: ${c.compared} ticks compared, not ${rec0.name}'s ${rec0.ticks}`);
+      if (da === db) {
+        if (c.mismatching !== 0 || !c.equal) problems.push(`${what}: digests equal (${da}: not exercised), yet ${c.mismatching} ticks differ, first at ${c.at}`);
+        verdicts.push(`${a} - ${b} not exercised (digests equal, ${da})`);
+      } else {
+        exercised++;
+        const caught = c.first?.recording === rec0.name && c.first?.tick === HASH_MAP_ARM && c.field === HASH_MAP_FIELD;
+        if (!caught) {
+          problems.push(`${what}: digests ${da} and ${db} differ, but the first mismatch is ${c.first ? `${c.first.recording} tick ${c.first.tick}, field ${c.field ?? "none"}` : (c.at ?? "none")}, not ${rec0.name} tick ${HASH_MAP_ARM}, field ${HASH_MAP_FIELD}`);
+        }
+        verdicts.push(`${a} - ${b} ${caught ? `caught at ${rec0.name} tick ${HASH_MAP_ARM}, ${HASH_MAP_FIELD}` : "NOT caught as D-2's rule asks"} (digests ${da}, ${db})`);
+      }
+    }
+    if (exercised === 0) problems.push("hash-map: no pair was exercised (every pair's digests equal)");
+    const wasmEqual = digests["Wasm leg 1"] && digests["Wasm leg 1"] === digests["Wasm leg 2"];
+    bites.hashMap = `${verdicts.join("; ")}${wasmEqual ? "; not catchable on wasm32 (its two Wasm digests are equal)" : ""}`;
+    lines.push(`PM-0004-21/hash-map (${rec0.name} armed at tick ${HASH_MAP_ARM}): ${bites.hashMap}`);
+  }
+  if (index[WASM_BITES.stdSin]) {
+    let scan = null;
+    try {
+      scan = readScan(io.read(STD_SIN_SCAN));
+    } catch (e) {
+      problems.push(`${STD_SIN_SCAN} cannot be read: ${e.message}`);
+    }
+    const order = entries.map((e) => e.name);
+    const catches = (scan?.catches ?? []).filter((c) => order.includes(c.recording)).sort((x, y) => order.indexOf(x.recording) - order.indexOf(y.recording));
+    const record = scan?.record.join("; ") ?? "";
+    const e = catches.length ? entries.find((x) => x.name === catches[0].recording) : null;
+    if (e && !has(e)) problems.push(`std-sin: ${e.name} was not regenerated, so its leg did not run`);
+    else if (e) {
+      const { recording, tick, field } = catches[0];
+      const t = String(tick);
+      call("std-sin native legs", art.labBites, ["hash", ...manifestArgs, "--out", sin, "--only", recording, "--bite", WASM_BITES.stdSin, "--arm", t, "--dump-at", t]);
+      const r = call("std-sin Wasm", process.execPath, loader(art.wasmBites, e, join(sin, "wasm-1.lines"), ["--bite", String(index[WASM_BITES.stdSin]), "--arm", t, "--dump-at", t, "--dump-out", join(sin, "wasm-1.dump")]));
+      if (!r.ok) problems.push(`std-sin Wasm leg 1: ${failure(r)}`);
+      const c = readComparison(
+        call("std-sin compare", art.labBites, ["hash", "compare", "--ours", join(sin, "native-1.lines"), "--theirs", join(sin, "wasm-1.lines"), ...manifestArgs, "--ours-dump", join(sin, "native-1.dump"), "--theirs-dump", join(sin, "wasm-1.dump"), "--bite", WASM_BITES.stdSin, "--arm", t, "--on", recording, "--ours-name", "native leg 1", "--theirs-name", "Wasm leg 1"]).out,
+        "native leg 1",
+        "Wasm leg 1",
+      );
+      const caught = c?.first?.recording === recording && c.first.tick === tick && c.field === field;
+      bites.stdSin = caught
+        ? `caught at ${recording} tick ${tick}, ${field}, as the scan found`
+        : `NOT caught at the scan's ${recording} tick ${tick}, ${field}: ${c ? `first mismatch ${c.first ? `${c.first.recording} tick ${c.first.tick}, field ${c.field ?? "none"}` : (c.at ?? "none")}` : "no comparison was written"}`;
+      if (!caught) problems.push(`std-sin: ${bites.stdSin}; this step ran on ${io.libc()}, the scan on ${scan.record.find((l) => /libc|ldd/i.test(l)) ?? "a libc it does not name"}`);
+      if (catches.length > 1) bites.stdSin += ` (the scan's other catches: ${catches.slice(1).map((x) => `${x.recording} tick ${x.tick}`).join(", ")})`;
+    } else if (scan?.notCatchable) {
+      bites.stdSin = `not catchable at this site on x86-64 and wasm32 (the scan: ${record}); no std-sin leg runs (the scratch plant stood in once: u4d2-brief.md 2.7)`;
+    } else if (scan) {
+      problems.push(`${STD_SIN_SCAN} records neither a catch on a manifest recording nor "not catchable at this site on x86-64 and wasm32"`);
+    }
+    lines.push(`PM-0004-21/std-sin: ${bites.stdSin}`);
+  }
+  const pairs = lines.filter((l) => / against /.test(l) && !l.startsWith("PM-"));
+  const summary =
+    `${problems.length ? `${problems.length} problem(s); ` : ""}${pairs.join("; ") || "no pair compared"}; h2 ${lines.some((l) => l.startsWith("h2: CO's wasm32 dump is the committed vector")) ? "equal" : "not equal"}; ` +
+    `hash-map: ${bites.hashMap}; std-sin: ${bites.stdSin}; seconds: ${timed.join(", ")}`;
+  return { ok: problems.length === 0, out: [...lines, ...problems, summary].join("\n") };
+}
+
 // Each cargo step takes the runner it calls cargo through (`run` when main calls it), so check.test.mjs pins what each
 // step runs: its commands, its flags, -D warnings included (the rust fix2 review's nit, u4s-carries.md entry 34).
 export const STEPS = {
@@ -1600,6 +2372,8 @@ export const STEPS = {
   doc: docBothBuilds,
   test: testBothBuilds,
   deps: depsStep,
+  // After deps, so the flags its builds use are checked first (u4d2-brief.md 2.4 (a)).
+  wasm: (runner = run) => wasmStep(runner),
   refs: () => {
     const r = run("git", ["ls-files", "-z"], ROOT);
     if (!r.ok) return r;
````

### check.test.mjs (check.mjs's self-tests)
_ 1 file changed, 961 insertions(+), 21 deletions(-); the diff itself is in `.checkpoints/netcode-proof-test/u4d2-packet-tests.md` (`git diff --no-index .gate/u4d2-prefix/spikes/netcode-proof/check.test.mjs .claude/worktrees/u4d2-build/spikes/netcode-proof/check.test.mjs`)._

## What it is judged against

### The approved plan: §5 U4d2 (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 666-712 (sha256 52dd31860efc), as they are:_

````text
### U4d2, the Wasm-in-Node hash and the determinism step (depends on U4d and Q3; size S-M)
- **Builds (option B):** crates/hash-wasm, a cdylib depending on npt-sim only, its exports thin wrappers over
  `Replayer`: `begin(bite, arm_tick)` (the recording's header carries the scene and the seed, and the manifest's SHA-256
  binds each file to its entry: the advisor's U4D2-Q3 ruling), `feed(word)`, `step() -> status`, `hash_lo`, `hash_hi`,
  `dump_len`, `dump_word(i)`, each `#[unsafe(no_mangle)] extern "C"`, integers only; its `bites` feature forwards to
  npt-sim's. A Node script loads it with `WebAssembly.instantiate` and streams a recording in; npt-lab's `hash`
  subcommand, on the release build, runs the two native legs in two processes and `HashCompare` prints the first
  mismatching tick and the field diff. `HashCompare` takes two per-tick hash lists, so U4r's -30 pre-check reuses it on
  the two legs' per-tick server hashes (the advisor's condition 2). npt-lab gains its clippy.toml: §4.11's clock ban and
  Q9 (a)'s HashMap and HashSet type bans (the advisor's U4D2-Q10 ruling).
- **check.mjs** (U4d2 is its only editor in its wave), each with a self-test bite in check.test.mjs:
  - a `wasm` step: the release build with plan.md §4.5's flags (baseline x86-64, no `+fma` or `target-cpu=native`,
    simd128 and relaxed-simd off; wasm-opt is U7's: C-U7-1; the advisor's U4D2-Q2 ruling), the Node run (a script, so
    the runner refusal at check.mjs:1156-1158 stands, its note updated), the comparison; its wall time printed, budget 5
    minutes;
  - the features comparison, like for like: npt-sim's resolved features under the server build and under npt-lab's (the
    native legs' build) against the hash-wasm build (the advisor's U4D2-Q6 ruling), measured builds together and bites
    builds together (A12);
  - **the build-flag check (A8):** `+fma`, `target-cpu=native`, `+simd128` and `+relaxed-simd` refused in the sources
    the deps step already reads (environment rustflags and cargo configs: check.mjs:903-942), and the effective
    target features read back with `rustc --print cfg` per target: the step fails on `fma` (x86-64) and on `simd128`
    or `relaxed-simd` (wasm32) in that read-back and never only reports, with a self-test bite (a read-back that lists
    one of them) (the advisor's condition 7 (a));
  - **hash-wasm's rules (A13):** its `[lints.rust]` denies unsafe_code, each listed export carries one allow (the
    pattern npt-tcpinfo has: check.mjs:135-137), and the scan refuses any other `unsafe`; it joins LEAF and
    WASM_CRATES (check.mjs:25, :1110); it depends on npt-sim only and nothing depends on it (as tcpinfo's rule,
    check.mjs:94-102). plan.md §4.1 gains its layout line.
  - **the third exception follows ADR 0002:264-265** ("an exception needs a written invariant, a test and an
    `#[allow]` with its reason"; the advisor's condition 7 (b), (c)): the written invariant, in the crate doc and beside
    each allow: no exported name collides with another symbol, and every export has integer signatures only; the
    test: the Node loader lists the module's exports and refuses any not on the list (its bite below), and the deps scan
    refuses a listed export whose parameters or result are not u32 or i32 (its bite: an export taking an f64; the
    advisor's U4D2-Q7 ruling); check.mjs's `UNSAFE_EXCEPTIONS` (check.mjs:135) gains `crates/hash-wasm/`; and before the
    code merges, plan.md §4.2's "two named exceptions" becomes three, hash-wasm named (the main session's plan.md edit,
    u4-plan-md-edits.md).
- **Tests:** 0 mismatching ticks over the 27,000 synthetic ticks, native twice and Wasm once; PM-0004-21's cross-target
  vectors: one committed byte string's digest and one committed state's bytes equal on x86-64 and wasm32 (T22).
- **Bites:** the std `sin` (input from a scan for inputs where glibc's `sinf` and the wasm32 builtin differ; if none
  differs, recorded as not catchable on these targets and a 1-ulp planted change stands in: ADR 0004:1142); the hash-map
  (caught, or recorded as not catchable on wasm32, where std documents no seeding: marks S-HashMap); an extra feature on
  a shared dependency (PM-0004-74); each flag; a stray `unsafe` block in hash-wasm; an export not on the loader's list
  (the advisor's condition 7 (b)); and the vectors' shown failures: a word of the committed vector file changed, and a
  length written as `usize`'s bytes in npt-sim's walk `write_state` (ADR 0004:1142's word-size bite), both scratch
  plants, restored (the advisor's U4D2-Q11 ruling).
- **Reviewers:** rust-reviewer (the exception, Wasm), engine-architect (the boundary), test-engineer, infra-engineer
  (the step's time against its budget; no CI job runs spikes/: plan.md R-8; the advisor's U4d2 brief check, A1),
  security-auditor (the third `unsafe` exception and check.mjs's unsafe and rustflags scans: the same check, A12).
````

### The approved plan: §4.1, the public items (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 178-203 (sha256 52dd31860efc), as they are:_

````text
### 4.1 Public items: plan §4.7 plus these, each with its reason
| Crate | Added or changed | Why |
|---|---|---|
| npt-sim | `SimCore` (moved from npt-room) | U4-SEAM |
| npt-sim | `TICK_HZ`, `LOOP_CATCHUP`, the tick-constant functions and their derived constants (§4.3) | plan.md:544-546; A4, A5: the Worker reaches npt-sim, never npt-server |
| npt-sim | `grid` (the wire grid, each way, per field kind) | U4-OWNSNAP: the sim's own-copy snap and both codec ends need one function |
| npt-sim | seam data: `Command`, `Rewind`, `TickInput`, `EntityRecord`, `HitVolumes`, `HitHistory` (moved from npt-room), `PortalStates`, `SimEvent`; `InputReports` and `SimCore::reports` (the clamp reports and the refusal counters, outside the hash: the advisor's condition 1); the owner-only own-state kind (W1-S1); the records-to-volumes function (W1-U4D A-M1); `InputReports`' refusal records (W1-U4D N-M2); the plain-data types `Placement`, `AttributeChange`, `Attribute`, `ClampReport`, `VolumeBox`, `FieldDiff`, `Ray`, `RayHit`, `replay::Step`, `Weapon` (TUNE-1); `Scene`'s and `World`'s accessors (`World::new`, `scene`, `greybox`, `next_tick`, `dump`; `Scene::by_id`, `id`, `name`, `slots`, `team`, `stand_ins`, `with_stand_ins`, `weapon`, `health`); W1-PUB's other approved items: `StandIns::OFF` and `s1`, `HitVolumes::MAX_SOLDIERS`, `MAX_COVER` and `MAX_BYTES`, carry 17's button and press constants, `Greybox::dm_tri`, and, bites builds only, `World::BITES`, `World::with_bite` and `World::bite_order_digest`; `Scene::effect_rows` (an effect row's owner, the one public home: carry 51) | §4.2; A1, A2; the advisor's condition 1; the advisor's W1-S1, W1-U4D and W1-PUB rulings; TUNE-1: `Weapon`, `Scene::weapon` and `Scene::health` are the facts each run's header records (U4r's npt-lab headers and U6's npt-server header: carry 60); carry 51 (the advisor's U4d close ruling, B2) |
| npt-sim | `Recording` (with `GENERATOR`, `encode`, `decode` and `synthetic`), `StateDump`, `replay::Replayer` and its methods | RQ5's field diff; U4-REPLAY; A12's one driver; W1-PUB: `synthetic` is in the library because U4d2 regenerates the recordings (§4.15) |
| npt-sim | `World::predict`, `World::load_movers`, `World::ray_cast`; `Scene::estimate`, `Scene::course`, `Scene::targets` beside `Scene::stress` | ADR 0004:166-178; ADR 0001:258-260; ADR 0001:1470-1482 |
| npt-sim | `EntityKind::ALL` and `EntityKind::layout`; `grid::Field::integer`; `full_state_ack_ticks_at`; `HitHistory::for_ms` in place of `HitHistory::new`; bites builds only: `grid::Config::floor_rounding`, `OWN_COPY_SNAP_BITES` and `own_copy_snap_with_bite`. Removed: bites-only `grid::Config::BITES` and `grid::Config::with_bite` (the advisor's U4s ruling, condition 1 (a)) | u4s-carries.md carry 30: arch F2 (one home for the per-kind field order; one type for a layout's fields); rust 1 (each derived constant is `<name>_at(TICK_HZ)`); netcode F6, A5; arch F1 |
| npt-proto | `RoomControl` (unfeatured; N5) | catalogue item 6 (:253); U5's fuzz target (catalogue :280) |
| npt-proto | `Wrap` (one compare per width, called with its counter's name) | -58's per-counter bites (§4.14) |
| npt-proto | `client_hash`; the `EntityRecord` ↔ wire mapping | -17 (§4.6); A18 |
| npt-proto | dependencies on npt-sim (grid, seam types) and npt-link (`Channel`) | U4-OWNSNAP, A18; catalogue item 2 (arch F37); leaf to leaf |
| npt-proto | `Message`, `Direction`, `DecodeError`, `EncodeError`; the message and control types' field types (`EndReason`, `ProfileId`, `StepResult`, `TokenKind`); `Limits`' constants and `TYPES`; `Quant`'s `encode` and `decode`; `Wrap`'s compares and `widen16`; `EntityDelta`; `Class` and the two `GROUPS` tables; the catalogue audit's name and variant helpers; bites builds only: `BITES`, `Quant::with_bite`, `Wrap::plain_lt` | the signatures and fields of plan §4.7's items; §5 U4a's layout table; catalogue items 2, 3 and 6 (the advisor's W1-PUB ruling) |
| npt-client | a dependency on npt-link, for `Channel`; `StepOut`'s fields (`datagrams`, `reliable`, `next_due_ns`, `hashes`, `lost`, `wraps`); `ClientEndpoint`'s methods (`new`, `input`, `step`, `with_starts`, `with_lead`, `slot`, `newest_tick`, `last_run_command`, `wrong_channel`, `refused`, `stale`, `duplicates`); bites builds only, `BITES` (3 ids), `grid_with_bite` and `ClientEndpoint::with_bite` | as above; u4s-carries.md carry 30: arch F1; plan.md:486 (`StepOut`); carries 24, 61 (b) and (c); U4E-Q2, U4E-Q7; -58's counter wraps; carry 74 (`with_lead`, set by U4r's lab); the advisor's U4e brief ruling, A5 (the bites-only constructor); the advisor's U4e round-1 ruling, R2 (A5) |
| npt-room | `Room<S: SimCore>`; `InputBuffer::CMD_CATCHUP` and `buffer_depth_ticks`; `ClientCounters` (the type `ClientSlot::counters` returns, its fields the per-client counts); `Room`'s methods (`new`, `join`, `leave`, `receive`, `pump_report`, `tick`, `take_datagrams`, `take_control` handing out each reliable message's bytes, `client`, `last_tick`, `sim`, `roster`, `script`, `tick_input`, `reports`, `event_cap`, `with_event_cap`, `with_event_start`, `with_hashes`, `with_cover`); `ClientSlot`'s getters (`slot`, `counters`, `server_hash`, `burst_eviction`, `baseline_age`, `groups`, `bytes`, `budget`, `last_acked`, `own_section`, `overflow_resync`, `wraps`, `rewinds_refused`, `queue`, `headroom`, `buffer_depth_ticks`, `kicked`); `Roster::live` and `is_live`; bites builds only: `BITES` and `Room::with_bite` | U4-SEAM; RQ6; U4E-Q3 (the advisor's W1-PUB ruling); §4.2 (`script`, `reports`), §4.5 (`burst_eviction`, the window's pair), §4.9 (`with_cover`), §4.11 (`with_bite`); U4E-Q6, U4E-Q7, U4E-Q11, U4E-Q12; carries 23, 40, 54, 56, 57 and 61 (a); plan.md:508-509 (`take_control`: encoding stays on the tick); the advisor's U4e round-1 ruling, R2 (A1, A4, A5) |
| npt-server | `Decoder::pump` (the I/O step over `&mut dyn Link`, injected `now`, the ping sender, per-client RTT and bytes sent); `Schedule`, `Due`; `Dropped`, `LoopCounters`, `next_due_ns`, `counters`, `tick_hz`, `catchup`; `Decoder`'s methods (`new` taking an `RttSource`, `counters`, `is_closed`, `accept`, `reject`, `take_sent_bytes`, `rtt_ns`); `RttSource` (`Link`, `Pings`); `ConnLimits::new`, `input_refill_per_second`, `hold_cap_ns`; `Counters`' fields; the modules private, each item at the crate root; bites builds only: `Schedule::with_bite`, `BITES` | U4-SERVER; §4.3; A6, A7; the advisor's W1-PUB ruling; §4.9 (`RttSource`: the RTT by the session's kind), §4.10 (`Counters`, U4f's -21 series); carry 39's 30 and 60 Hz pins; the advisor's U4e round-1 ruling, R2 (A2, A5) |
| npt-l1 | `L1Net::next_event_ns` | C1' makes L1 event-driven |
| npt-bots (library) | `Behaviour` (the brawl, the open-loop scripts, the target-course shooter, the cheats, the never-acker), clock-injected | E12; U4-SERVER |
| npt-lab | subcommands `scenario` (with `--record`), `replay` and `hash` | plan.md:590-591 |
| npt-log | `TickRecord.hs`, with `TickRecord::set_hashes` and `TickRecord::hashes` | marks/README.md:147-148 (since U4j, README §1's `hs`) |
| npt-judge | `marks::Mark::ws_bound_line`, `marks::Check::ws_bound` and `marks::BoundNumber` (PM-0001-18's WebSocket bound, its line held verbatim); `marks::Rule::WsPractice` (the `ws-practice` rule); `eval::Outcome::Empty`, `eval::Outcome::fails` and `figures::Lack::Empty` (0 compared fails, never passes) | Q20 and the advisor's ws-docs R3-R12 (-18's split, `ws-practice`); the advisor's U4j FX-6 (g) (A19); §4.6's "0 compared fails" |
| npt-judge | nothing for -30's pre-check: the `PRE-` id, marks/prechecks.json and the `"kind": "precheck"` rule are dropped (the advisor's condition 2) | Q10 (T2); the advisor's condition 2 |
| a new crate | `npt-hash-wasm` (cdylib; integer-only exports) | U4-WASM (B) |
| not built at U4 | `BlockWriter` (U7); `TickLoop`, `PhaseTimers`, `ControlSocket`'s socket, `AddrTable`, the handshake cap (U6) | §6 |
````

### The approved plan: §4.11, determinism and hygiene outside npt-sim (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 392-407 (sha256 52dd31860efc), as they are:_

````text
### 4.11 Determinism and hygiene outside npt-sim (Q9)
- The HashMap and HashSet type bans join room's and client's clippy.toml; new clippy.toml files for npt-server and
  npt-lab, and for npt-bots (all three with the same HashMap and HashSet type bans, by Q9 (a), and with the clock ban,
  naming `std::time::Instant`, `std::time::SystemTime`, `tokio::time::Instant` and their `now`: A14). npt-bots' file is
  crate-wide; U6's runtime half and its npt-net glue then carry reviewed per-item allows, listed in U6's plan.
- **npt-room is float-free (A3):** interest, priority and the window work on grid integers; a check.mjs grep refuses
  `f32` and `f64` in crates/room/src (U4s, with its self-test bite).
- **Bites per instance (T10):** every bite is chosen by a constructor argument or a config value the caller passes
  (`World::with_bite`, `Room::with_bite`, the grid config), never a static, as `L1Net::with_bite` does
  (l1/src/lib.rs:856-868). A test runs a bite instance and a clean instance on parallel threads in one process.
- **Send (A25):** compile-time asserts that `Room<World>`, `Decoder` and `ClientEndpoint` are `Send`, so U6 can move
  them to the tick thread; none holds a `Link`.
- **Paths no L1 run reaches (T17):** a U4-built path whose only marks are L2, RS or ML parts gets a named unit test in
  U4, or moves to the unit that first runs it (AddrTable and the handshake cap move to U6).
- The runner's own twin test (two processes, same seed and scenario: equal fate-log hashes, equal per-tick server and
  client hashes); bite: a HashMap-ordered client order (marks F29).
````

### The approved plan: §4.14, bites that need a site or a run setting (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 453-476 (sha256 52dd31860efc), as they are:_

````text
### 4.14 Bites that need a site or a run setting (T4, T6, risk 3)
Each is planted where its code lives, shown to change state by a unit test in that sub-unit, and run judged in the
sub-unit that moves its mark.

| Bite | Its site | The check it fails | Run setting | Unit proof |
|---|---|---|---|---|
| -17/tick16, -58/tick16 | proto's header writer and the client's tick rebuild (the 16-bit value taken as the tick) | decoded hash: post-wrap frames logged under the wrong tick (an orphan pair) or as the sentinel | -17: `tick_start` = 63,736; -58: `ticks_below_wrap` = 30 | a frame at tick 65,537 hashes differently or is refused |
| -17/client-rounding | npt-client's call into `grid` (floor in place of nearest); the id lives in npt-client (`npt_client::BITES`, `grid_with_bite`), the grid holding only the defect (the advisor's U4s ruling, condition 1 (a)) | decoded hash | any -17 profile | the floor variant differs on about half of all position values |
| -18/no-fallback | the encoder's baseline choice, never falling back to the defaults | `burst_heal` (the fresh delta) | B1, ring ages 0-31 | a gap evicts B (on Loopback `npt_sim::ACK_RING_TICKS` tick periods, 1 s plus two tick periods at 30 Hz, since there a 1 s gap leaves B at the ring's edge: the advisor's U4E-Q14 ruling); `burst_eviction` = 1 |
| -21/not-older-than | the room's tick check (≥ in place of >) | fast clock's excess commands | the fast-clock suite, option A (marks.json -21 notes) | an equal-tick command runs |
| -21/no-catchup-cap | the room's per-tick command cap | forged: unrejected | the forged suite, option A, two datagrams in one tick | 16 commands run in one tick |
| -41/server-speed-1pct | the authority step's run speed x 1.01, not `predict`'s (Q24) | untraced corrections | CO, every profile | one moving tick differs |
| -41/server-quant-differs | the authority step's own-copy snap with another rounding than `grid`'s (Q23); the id lives at npt-sim's own-copy snap side (`OWN_COPY_SNAP_BITES`, `own_copy_snap_with_bite`), the grid holding only the defect (the advisor's U4s ruling, condition 1 (a)) | untraced corrections | CO | the snapped copy differs from the sent one |
| -45/buffer-one-short | Interp's slot count | L1 P2: extrapolated share < 1 % | P2; size = the clean P2 runs' logged largest delay in intervals + 2 - 1; the delay never clamped to the buffer | extrapolation on a fixed P2 arrival trace |
| -48/no-buffer-wait | the estimate without the input-buffer wait | view-hit share | where the buffer settles at 2 ticks or more (P3, option B, read from -20's logged depth; below 2, the advisor is asked before a fixed minimum is set) | honest view times replaced |
| -58/lt-cmd-seq | the room's sequence-forward compare | 0 commands refused | sequence 2 below its wrap | post-wrap commands refused |
| -58/lt-press | the room's press-edge compare | 0 presses lost or doubled | presses scripted through the wrap | an edge lost |
| -58/lt-tick-offset | the client's rebuild of the header's 16-bit 'last command run' sequence, where its replay starts (a plain `<` in place of its wrap-aware compare): U4e builds the rebuild, U4g's Predictor starts its replay there (the advisor's W1-N2 ruling; U4E-Q2) | 0 untraced corrections | command sequences crossing the wrap (as -58/lt-cmd-seq: a few steps below it) | U4e: the rebuilt sequence names the wrong command; U4g: the replay starts at the wrong command |
| -58/lt-event-seq | the client's event-number rebuild | decoded hash (event numbers are hashed: §4.6) | event numbers 3 below the wrap; SS's hits flow | a post-wrap event rebuilt wrong |
| -58/lt-ack | the room's rebuild of the 16-bit ack: a wrong rebuild names a snapshot never sent, and the check "the acknowledgement names a sent snapshot" refuses that datagram's commands | 0 commands refused | acks crossing the wrap | a post-wrap ack refused |
| -62/cues-from-replay | CueLog written from replays | plays at most once | suite `cues`: the replay skip off, a replay on every snapshot (as PM-0001-44: ADR 0001:1650) | a replayed shot plays twice |
| -62/server-echo | the room sending the owner its own cosmetic shot events | own cosmetic sent = 0 | `cues` | one event sent |
| -22/unbounded | the event queue's cap | headroom min = 0 | a full S30 of simulated time (marks.json -22) | headroom below 0 |
| PM-0004-21 hash-map | a bites-only HashMap of at least 32 keys whose whole iteration order folds into a state value | D-2 and U4d2: first mismatching tick = T | two processes; each logs its order digest | the two digests differ; equal digests mean "not exercised", never caught or missed |
````

### The approved plan: §4.15, recordings (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 478-504 (sha256 52dd31860efc), as they are:_

````text
### 4.15 Recordings (A1, A12, T9; Q4)
- **Fields:** a header (scene, seed, TICK_HZ, pool sizes, HitHistory's capacity (nit b)); then per tick a
  `TickInput`: per soldier one `Command` (slot 1 B, tick 4, sequence 2, buttons 2, press counters 4, move 2 (the advisor's U4s ruling, condition 1 (a)), aim in cm 12, view tick 4 and fraction 1) and its
  `Rewind` (present 1, tick_a 4, fraction 1, cover 1); spawns, teleports and scripted attribute changes when they occur.
  Fixed-width little-endian, no `usize` (plan.md:527-529).
- **Size:** 39 B per command (move is 2 B), so about 1,872 B a tick of commands for 48 soldiers before spawns and the
  tick's own header (the advisor's U4s ruling, condition 1 (a)); about 16.8 MB per 9,000 ticks and 50.5 MB for three,
  commands only (my arithmetic; U4d states the measured size). Fed to Wasm as 32-bit words: about 4.2 million calls
  each (the advisor's F1 ruling).
- **Both uses:** the sim-only replay steps npt-sim with each `TickInput` and the ring the driver rebuilds; -30's
  pre-check (a recorded npt-lab test, never judged: the advisor's condition 2) takes the SS P0 recording below (the
  advisor's condition 3) and re-encodes its commands as `Input` (stamped ticks, view times, sequences and press
  counters from the recording; the ack from what that leg's replay client decodes live) and lets the room compute its
  own Rewinds.
- **One driver:** `npt_sim::replay::Replayer` (feed words, step, per-tick hash and dump). npt-lab's `hash` subcommand
  runs it natively; npt-hash-wasm's exports wrap it; D-2 calls it in a test.
- **Sources:** U4d's three synthetic recordings (open-loop commands, Rewinds drawn inside the ring) from a seeded
  generator in npt-sim's library (`Recording::synthetic`), since U4d2's step regenerates them from the manifest,
  needing no room; U4r's three runner recordings (`scenario --record`: SS P2 seed 1, ES
  P3 seed 2, TC B1 seed 3), which hold the room's real Rewinds; and a fourth runner recording, **SS P0 seed 4**
  (`scenario --record`), the -30 pre-check's source (the advisor's condition 3). Replayed at P0, the other three's view
  times sit about one RTT older than P0's estimate, so the room would replace them on both legs, each from its own RTT;
  a recording made at P0 keeps them inside the tolerance. The determinism step keeps its three; the fourth serves the
  pre-check only.
- **Where they live:** regenerated at check time under .gate/ from a committed recordings manifest (scene, seed,
  script, generator version) with each file's SHA-256; a different hash fails the step and names the recording.
  Never committed as files.
````

### The approved plan: Lanes (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 850-888 (sha256 52dd31860efc), as they are:_

````text
### Lanes (at most 3 agents at a time, in worktrees, merged one at a time)
- **Step 0:** U4s alone (the main session or one builder), merged before anything else. It makes every manifest and
  Cargo.lock change wave 1 needs, so no wave-1 lane touches Cargo.toml or Cargo.lock. It also adds every wave-1
  crate's `bites` feature and npt-lab's forwarding to them (the advisor's condition 10 (a)), and the wave-2 manifest
  lines U4e's crates need (npt-room's and npt-client's `bites` features with npt-lab's forwarding; npt-server's dev
  dependency on npt-client), so that in wave 2 only U4d2 edits crates/lab and Cargo.lock (condition 10 (b)).
- **Wave 1:** U4a, U4c and U4d as three agents; U4b and U4j in the main session (or the first free lane). File
  ownership: U4a crates/proto and check.mjs; U4c crates/l1; U4d crates/sim; U4b crates/server/src/schedule.rs and
  server/clippy.toml; U4j crates/judge, crates/log and marks/. No file is shared. U4c's L-c8 judged re-run waits for
  U4j's merge.
  - **Freeze the seam's shapes for wave 1.** Through wave 1, U4d keeps unchanged every existing field, variant and
    width of `Command`, `Rewind`, `CoverVariant`, `EntityKind` and `EntityRecord` (FIELDS 12), and the five grid
    fields as built at f080551. It may add items (World, carry 17's button constants, `TickInput`'s new fields),
    complete `layout()` within FIELDS (carry 12), and drop `InputReports::non_finite` (NF-D4). A change to an
    existing item waits for U4d's merge and goes to the main session, as carry 1 already says for the grid. Why: U4a
    builds its codec and field tables against these types at the same time, so a silent change breaks the second
    merge. The words are the ruling's condition 3; the carries are u4s-carries.md's
    (the advisor's U4s ruling, condition 1 (d)). **Lifted for one change (the advisor's W1-S1 ruling, 2026-11-10):**
    U4d's fix pass appends one owner-only kind to `EntityKind` (last in `ALL`), holding velocity, the ammunition, the
    fire timer, the reload left and the move-speed and fire-interval attributes, and takes velocity and the weapon
    state out of SOLDIER's layout. `EntityRecord::FIELDS`, `Command`, `Rewind`, `CoverVariant` and the five grid fields
    stay frozen. U4a's half (the kind in both GROUPS tables' own-state rows, the codec's refusal of a kind its section
    does not list, and its test) is the main session's, in U4d's merge commit, after U4a has merged.
- **Wave 2:** U4d2 (after U4d) and U4e (after U4a, U4b, U4d) in parallel: U4d2 owns check.mjs and check.test.mjs,
  Cargo.toml's members and its lints and libm comments (carry 29), Cargo.lock, crates/hash-wasm, crates/lab (npt-lab's
  `hash`, carry 66's probe change and its clippy.toml), and two manifests' comment lines only: crates/proto/Cargo.toml's
  bites comment (carry 41) and crates/client-wasm/Cargo.toml's lints comment (the exceptions now three) (the advisor's
  U4D2-Q9 ruling); U4e owns crates/room, crates/server (beyond schedule.rs) and crates/client. In wave 2 only U4d2 edits
  crates/lab, its Cargo.toml included; U4e's tests live in its own crates (Loopback as a dev dependency: npt-server's
  tests, through its npt-link and the dev dependency on npt-client U4s adds), never in crates/lab (the advisor's
  condition 10 (b)).
- **Then** U4r (after U4c, U4d2, U4e, U4j) and its runs; then U4f, U4g and U4h in order (they share the room's and the
  client's files: never two at once).
- After each merge the next lane's base moves: its brief's counts are deltas, and the merge step recomputes the
  absolute counts on the merged tree.
- **Reviewers count toward the 3-agent limit** and run at effort xhigh, as netcode work (the advisor's condition
  10 (c)): a review round starts only when enough of the three slots are free.
- **The main session builds U4b and U4j only on committed state,** never with uncommitted edits while it merges a lane
  (the advisor's condition 10 (d)).
````

### The approved plan: §7.1, Mustafa's shared-target rule (u4-build-plan.md)
_`.checkpoints/netcode-proof-test/u4-build-plan.md` lines 909-921 (sha256 52dd31860efc), as they are:_

````text
### 7.1 Mustafa's shared-target rule (2026-11-09; checkpoint :404-413; as u3d-plan.md §7.1)
1. Every cargo call in each builder's worktree and each bite's scratch copy runs with
   `CARGO_TARGET_DIR=/home/user/Shamble-Frontlines/.gate/shared-target`; checked once after the first build.
2. Touch before each build: `find <copy>/spikes/netcode-proof/crates -type f -exec touch {} +`.
3. Each run's test count matches the brief's delta; `--list` names the new tests; a bite run shows its own test
   failing by name; a mismatch means another copy's artifact ran: touch, rebuild, never report that run.
4. After the last bite, touch and rebuild before any further claim.
5. Cargo's own lock serializes two copies' builds: expected.
6. The main folder keeps its own cache (.gate/netcode-proof-test/target): the merge checks, the gate and every judged
   L1 run happen there, with `CARGO_TARGET_DIR` unset.
7. No copy cleans the shared folder.
8. Every brief names the folder, items 2 and 3, and .claude/rules/netcode.md as binding on proto, room, client and
   server code.
````

### The proof test's plan: §4.1, npt-hash-wasm's layout line (plan.md)
_`.checkpoints/netcode-proof-test/plan.md` lines 392-393 (sha256 aac32c3d2802), as they are:_

````text
  crates/hash-wasm/    npt-hash-wasm (U4): a check-only cdylib over npt-sim with integer-only exports, which Node runs
                    for the native-against-Wasm check (§4.5); the workspace's third unsafe exception (§4.2)
````

### The proof test's plan: §4.2, the dependency rules and the unsafe exceptions (plan.md)
_`.checkpoints/netcode-proof-test/plan.md` lines 421-454 (sha256 aac32c3d2802), as they are:_

````text
### 4.2 Dependency rules (enforced by the spike's test command: `cargo tree` checks and one grep)
- **Leaf crates:** npt-link, npt-l1, npt-sim, npt-proto, npt-room, npt-client and npt-log depend on no tokio, quinn,
  wtransport, web-transport-quinn or tungstenite. Each passes
  `cargo check --locked --target wasm32-unknown-unknown`. Clock reads are banned in them by clippy (§4.5).
- **The client:** npt-client never depends on npt-net; the bots get transports by composing npt-client with npt-net.
- **The protocol and the sim:** npt-proto may depend on npt-sim, for the wire grid (one function for the sim's
  own-copy snap and both codec ends) and the seam's plain-data types (ADR 0002:188: the `protocol` crate uses "`sim`
  and `content` types only"; the advisor's Q23 ruling, 2026-11-09, advisor-u4.md).
- **npt-super** depends on no npt-sim, npt-room or npt-proto. **npt-host** depends on none of the sim, room, proto or
  client crates.
- **npt-judge** depends only on npt-log, plus serde ("serde" here and in §4.1 means serde and serde_json). Nothing
  depends on npt-lab.
- **npt-tcpinfo** depends on libc only (pinned =0.2.191, already in the lock through tokio, mio and quinn-udp), and
  only npt-net uses it (U3c; the advisor's C12).
- **Unsafe code** is forbidden in the workspace (`unsafe_code = "forbid"`, the compiler backstop of the advisor's
  ruling of 2026-11-08: no `#[allow]` can relax it in a crate that inherits the workspace's lints), with three named
  exceptions under ADR 0002:264-265's rule ("an exception needs a written invariant, a test and an `#[allow]` with its
  reason"), each carrying its own `[lints]` table equal to the workspace's but for unsafe_code:
  - npt-client-wasm (`allow`), for wasm-bindgen's generated code;
  - npt-tcpinfo (`deny`): its two functions (getsockopt TCP_INFO and ioctl SIOCOUTQ), each `#[allow]` on the function
    only, holding four unsafe blocks of one foreign call each, each with its SAFETY comment;
  - npt-hash-wasm (`deny`; U4, the advisor's Q3 ruling and condition 7, 2026-11-09, advisor-u4.md): the check-only
    cdylib over npt-sim that Node runs for the native-against-Wasm check (§4.5); each listed export carries one
    `#[allow]` for its `#[unsafe(no_mangle)]` attribute, and no `unsafe` block exists; its written invariant: no
    exported name collides with another symbol, and every export has integer signatures only; its test: the Node
    loader lists the module's exports and refuses any not on the list.
  The spike's test command checks all three, and greps that `allow(unsafe_code` appears nowhere else. npt-net stays
  free of unsafe code and libc (the advisor's M14).
- **How they are read** (the advisor's C10): with every feature and every target; dev dependencies do not count (they
  never ship), except for "nothing depends on npt-lab", which counts every kind.
- **No reference into `spikes/`** from outside it: a grep in the test command, until the foundation's G5 and G8 exist.
  It skips docs/, .checkpoints/ and Markdown, whose prose may name the folder; W-PR's workflow file, which runs the
  Safari leg, gets an exception that names it by path, in the same change (C11).
- **Narrow public surfaces:** each crate's public items are those in §4.7; everything else is `pub(crate)`.
````

### The proof test's plan: §4.5, the determinism rules and the native-against-Wasm check (plan.md)
_`.checkpoints/netcode-proof-test/plan.md` lines 527-551 (sha256 aac32c3d2802), as they are:_

````text
### 4.5 Determinism rules for npt-sim, and the native-against-Wasm check (engine EA-7, EA-8; ADR 0004 parts 4 and 7-9; ADR 0002 parts 2 and 7; GLOSSARY §7)
- **Floats:** ADR 0004's allow-list only, with its own min, max and clamp helpers; `rem_euclid`, `div_euclid` and `%`
  are off the list; transcendentals through `libm` only. NaN never enters state: finite checks at entry, and debug
  asserts each tick.
- **Bans (clippy):** the hash-map ban (`disallowed-types` plus `iter_over_hash_type`), in a `crates/sim/clippy.toml`
  copied from ADR 0002 part 7. The clock ban covers npt-sim, npt-proto, npt-room, npt-client and npt-l1.
- **Build flags:** baseline x86-64; no `+fma`, no `target-cpu=native`, no relaxed-simd; simd128 off; wasm-opt without
  `--fast-math`. `overflow-checks = true` for npt-sim and npt-proto in every profile, with `wrapping_` and `checked_`
  counters (the S30 soak starts the tick counter 1,800 below 65,536).
- **Features:** one feature set for npt-sim in the server and Wasm builds, compared with `cargo tree -e features`.
- **Randomness:** `ChaCha8Rng`, pinned; the seed passed in by the host; our own sampling functions; the authority-only
  spread stream kept apart.
- **Clippy and the wasm32 check run at every unit, from U1:** host and wasm32 for npt-sim, npt-proto, npt-client and
  npt-room.
- **The hash:**
  - computed by canonical serialization: field by field, fixed-width little-endian, no `usize`, floats by `to_bits`, a
    fixed unseeded hash, never std's `Hasher`;
  - spike-local: the sim core spike picks the real hash function (ADR 0004:350-351);
  - the client-held hash leaves out the authority-only streams.
- **The check (from U4):**
  - inputs: the 3 input recordings (3 × 5 min = 27,000 ticks);
  - each run natively twice in separate processes, once as Wasm in Node, and once in local Chromium (a rough check);
  - the report names the first mismatching tick, with a field diff;
  - pass mark: 0 mismatching ticks;
  - bites: one std `sin` and one hash-map iteration, each caught at the first tick it changes.
````

### The proof test's plan: §4.7, npt-lab's public items (plan.md)
_`.checkpoints/netcode-proof-test/plan.md` lines 617-617 (sha256 aac32c3d2802), as they are:_

````text
- **npt-lab:** `SimClock` (integer ns), `L1Scenario`, `ReplayX86`, `HashCompare` (netcode NC2-F9).
````

### The advisor's U4 plan ruling: condition 2 (advisor-u4.md)
_`.checkpoints/netcode-proof-test/advisor-u4.md` lines 90-101 (sha256 7b3e76ca09f3), as they are:_

````text
2. **-30's pre-check becomes a recorded test, not a judged id (Q10).**
   - Keep: both legs (Loopback and L1 P0), the replay clients, both bites and the suite `precheck-30`.
   - Drop from U4j: `PRE-` ids, marks/prechecks.json, the "kind": "precheck" rule and their tests.
   - npt-lab's comparer (built in U4d2) compares the two legs' per-tick server hashes and names the first tick that
     differs. `tick.v.late_count` must sum to 0 on both legs.
   - The bites: wallclock must produce a mismatch; late-command must make the pair void, never a pass.
   - The headers list no mark, so the judge never reads these runs.
   - Record results/u4r-precheck-30/<commit>/runs.txt and the comparer's output. I read them in U4r's review, as I did
     for U3d's self-test (advisor-u3d.md, U3D-JUDGE: nothing goes into marks.json).
   - If the Loopback leg writes an npt-log header, README §1 gains the layer value `loopback` in one line. No judge
     change.
   - -30 stays NOT-RUN until U6.
````

### The advisor's U4 plan ruling: conditions 7 and 8 (advisor-u4.md)
_`.checkpoints/netcode-proof-test/advisor-u4.md` lines 130-139 (sha256 7b3e76ca09f3), as they are:_

````text
7. **U4d2's checks refuse, and the new exception follows ADR 0002:264-265.**
   - (a) The `rustc --print cfg` read-back fails the step on fma (x86-64), simd128 or relaxed-simd (wasm32), with a
     self-test bite. It never only reports.
   - (b) hash-wasm gets a written invariant (no exported name collides with another symbol; integer signatures only)
     and a test (the Node loader lists the module's exports and refuses any not on the list).
   - (c) Before the code merges, plan.md §4.2's "two named exceptions" becomes three, and check.mjs's
     `UNSAFE_EXCEPTIONS` gains crates/hash-wasm/.
8. **New carry C-U7-1.** U4d2 does not cover wasm-opt (ADR 0002:500-505 uses "the exact file G6 ships"). U7 reruns
   U4d2's check on hash-wasm passed through the same wasm-opt step and flags as the shipped npt-client-wasm, or states
   why not. This goes into plan.md's U7 row and the checkpoint.
````

### The advisor's U4 plan ruling: condition 10, the lanes (advisor-u4.md)
_`.checkpoints/netcode-proof-test/advisor-u4.md` lines 143-149 (sha256 7b3e76ca09f3), as they are:_

````text
10. **Lanes.**
    - (a) U4s also adds every wave-1 crate's `bites` feature and npt-lab's forwarding to them.
    - (b) In wave 2, only U4d2 edits crates/lab, its Cargo.toml included. U4e's tests live in its own crates (Loopback
      as a dev dependency).
    - (c) Reviewers count toward the 3-agent limit and run at xhigh (netcode work).
    - (d) The main session builds U4b and U4j only on committed state, never with uncommitted edits while it merges a
      lane.
````

### The advisor's U4 plan ruling: Q3, U4-WASM (advisor-u4.md)
_`.checkpoints/netcode-proof-test/advisor-u4.md` lines 198-199 (sha256 7b3e76ca09f3), as they are:_

````text
- **Q3 U4-WASM:** (B), with conditions 7 and 8. The third unsafe exception is mine under ADR 0002:264-265, as
  npt-tcpinfo's was (advisor-u3c-plan.md:56).
````

### The advisor's F1 and F2 ruling: point (3) (advisor-u4.md)
_`.checkpoints/netcode-proof-test/advisor-u4.md` lines 450-458 (sha256 7b3e76ca09f3), as they are:_

````text
later takes (b): a new tick count, or a named constant (an array length, a const generic) that needs a value derived
from the rate: the builder writes no local form (no literal, no local formula, no local `ticks_for_ms` constant),
records the need in its report (name, definition, values at 30 and 60 Hz, the site) and builds the rest; the main
session then adds the constant by carry 18's route, as one commit of its own on committed state (condition 10 (d)): the
`_at` function, the constant and the const assert in ticks.rs; the re-export in lib.rs; s1's DERIVED row; check.mjs's
`TICK_DERIVED`; plan §4.1 and §4.3; timing: only when no lane owns crates/sim or check.mjs (in wave 1 after U4d and
U4a have merged; in wave 2 after U4d2 has merged), check.mjs and the gate green; the site is finished and the lane
merges after that commit; the needing lane's rust-reviewer reviews the commit; this costs nothing on the critical path
(U4e already waits for all of wave 1, and U4r already waits for U4d2). (4) (c) is not taken: pinning a local constant at
````

### The advisor's wave-1 ruling W1-PUB, the public items beyond §4.1 (advisor-u4-wave1.md)
_`.checkpoints/netcode-proof-test/advisor-u4-wave1.md` lines 353-435 (sha256 a45575aeb3b4), as they are:_

````text
### W1-PUB: the public items beyond §4.1's lists (A19)
**Ruling: APPROVE all three lists, and remove nothing.** Two nits are below, and §4.1's table is to be brought up to
date. Every item is one of three things:
- a type in the signature or a field of an item §4.1 or plan.md §4.7 names;
- the field-group layout table that §5 U4a itself lists;
- an item a planned test needs (the catalogue audit, carry 6's read-back).

None is a "for later" item.

- **U4a (npt-proto; its lib.rs:32-45, checked against the rust-reviewer's A19 list):**
  - `Message`, `Direction`, `DecodeError` and `EncodeError`: in `encode`'s and `decode`'s signatures.
  - The message structs, `EndReason`, `ProfileId`, `StepResult` and `TokenKind`: fields of the listed messages and
    controls.
  - `Limits`' constants and `Limits::TYPES`: `Limits` is listed, and catalogue items 2 and 3 test against `TYPES`.
  - `Quant::encode`, `Quant::decode` and the bites-only `Quant::with_bite` and `BITES`: the per-instance bites
    (§4.11).
  - `Wrap::after8`, `after16`, `after32` and `widen16`: §4.4's one compare per width, and the rebuild. `plain_lt` is
    bites-only.
  - `EntityDelta`'s `between` and `apply`: the `EntityRecord` mapping (§4.4).
  - `Class`, `Snapshot::GROUPS` and `FullState::GROUPS`: §5 U4a's "field-group layout table". W1-S1 makes the codec
    enforce them, so they become operative.
  - `Message::name`, `SessionControl::VARIANTS` and `variant`, `RoomControl::NAMES` and `name`: catalogue item 6's
    audit (approved, Q7).
  - Nit (rust F-A8, a follow-up): `Limits::GROUPS` and `Snapshot::GROUPS` share a name with two meanings. Rename the
    limit at the next edit of limits.rs.
- **U4b (npt-server's schedule.rs):**
  - `Dropped` and `LoopCounters`: types inside `Due` and `counters`.
  - `next_due_ns`: U6's timer, and the tests' "no tick owed" check.
  - `counters`: ADR 0004 part 5's monitoring.
  - `tick_hz` and `catchup`: carry 6's read-back.
  - Bites builds only: `Schedule::with_bite` and `BITES` (PM-0004-67's three, schedule.rs:251; lib.rs:13).
  - These match u4b-build.md:31-42 and schedule.rs:22-119 and :251-257.
  - Nit (a follow-up): each item has two paths (`npt_server::schedule::X` and `npt_server::X`: lib.rs:10, :13-14).
    Keep one at U4e's next edit of server/src/lib.rs.
- **U4d (npt-sim).** u4d-build.md holds no "For the advisor" section, so I ruled on the surface its lib.rs exports
  (worktree crates/sim/src/lib.rs:25-55, and each module's `pub` items):
  - Plain-data types of named items (plan.md §4.7: "with its plain-data input and output types"):
    - `Placement`, `AttributeChange` and `Attribute`: `TickInput`'s fields;
    - `ClampReport`: `InputReports`' field;
    - `VolumeBox`: `HitVolumes`' field;
    - `FieldDiff`: `StateDump::first_difference`'s result (RQ5's field diff);
    - `Ray` and `RayHit`: `ray_cast`'s argument and result;
    - `replay::Step`: `Replayer::step`'s result.
  - Named in plan.md §4.7: `Greybox`, `Areas`, `Portals`, `StandIns` and `trajectory_at`. Engine-architect F is
    right that the builder's gap list overstated this.
  - The constructors and accessors the room, lab and Wasm wrapper need:
    - `World::new`, `scene`, `greybox`, `next_tick` and `dump`;
    - `Scene::by_id`, `id`, `name`, `slots`, `team`, `stand_ins` and `with_stand_ins` (the header's scene id; the
      stand-ins' off switch on L1: §4.13);
    - `StandIns::OFF` and `s1`;
    - `HitVolumes::MAX_SOLDIERS`, `MAX_COVER` and `MAX_BYTES`;
    - `Recording::GENERATOR`, `encode`, `decode` and `synthetic`;
    - `Replayer`'s methods;
    - carry 17's button and press constants.
  - Bites-only: `World::BITES`, `with_bite` and `bite_order_digest`.
  - `Recording::synthetic` belongs in the library: U4d2 regenerates the recordings from the manifest (§4.15's text
    changes: W1-U4D).
  - `Greybox::dm_tri` stays public: the static graph without a World, for the room's AreaFlood. U4e's report says
    which route the room takes.
  - Nit (rides U4d's fix pass): `Recording` has two paths (lib.rs:17's `pub mod replay` and :28's re-export). Keep
    one.
  - `SimRng` stays crate-private, narrower than plan.md §4.7: no other crate draws from the sim's streams (§4.5; the
    spread stream is authority-only). Record it in U4d's report, and plan.md §4.7's npt-sim line at its next edit:
    "`SimRng` (crate-private: no other crate draws from the sim's streams; the advisor's W1-PUB ruling)".
- **§4.1's table (main session; text):**
  - npt-proto's rows gain one row: "`Message`, `Direction`, `DecodeError`, `EncodeError`; the message and control
    types' field types (`EndReason`, `ProfileId`, `StepResult`, `TokenKind`); `Limits`' constants and `TYPES`; `Quant`'s
    `encode` and `decode`; `Wrap`'s compares and `widen16`; `EntityDelta`; `Class` and the two `GROUPS` tables; the
    catalogue audit's name and variant helpers; bites builds only: `BITES`, `Quant::with_bite`, `Wrap::plain_lt` |
    the signatures and fields of §4.7's items; §5 U4a's layout table; catalogue items 2, 3 and 6 (the advisor's
    W1-PUB ruling)".
  - npt-server's row gains "`Dropped`, `LoopCounters`, `next_due_ns`, `counters`, `tick_hz`, `catchup`; bites builds
    only: `Schedule::with_bite`, `BITES`".
  - npt-sim's row gains:
    - "the plain-data types `Placement`, `AttributeChange`, `Attribute`, `ClampReport`, `VolumeBox`, `FieldDiff`, `Ray`,
      `RayHit`, `replay::Step`";
    - the owner-only own-state kind (W1-S1);
    - the records-to-volumes function (W1-U4D A-M1);
    - `InputReports`' refusal records (W1-U4D N-M2);
    - `Scene`'s and `World`'s accessors as listed above.
  - npt-room's row gains "bites builds only: `BITES`" (U4E-Q3).

W1-PUB written 2026-11-10.
````

### The advisor's wave-1 note W1-TARGET, the shared Cargo target (advisor-u4-wave1.md)
_`.checkpoints/netcode-proof-test/advisor-u4-wave1.md` lines 558-586 (sha256 a45575aeb3b4), as they are:_

````text
### W1-TARGET: the shared Cargo target (a note; one rule for wave 2's briefs)
- **What wave 1 showed.** Cargo keys a workspace member's artifacts without its folder and keeps dep-info paths
  package-relative (u4d-build.md, step 7). So a copy that builds after another copy's touch makes the other copy's
  stale sources look fresh, and the lanes ran each other's crates.
  - This happened at least five times: U4a 22:13, 22:14 and 22:55; U4d 06:35; and the test-engineers' runs A in
    u4ab-review-test.md and u4d-review-test.md.
  - Every time, the count check or a compile error caught it, and the run was not reported.
  - The 2 s touch loop narrows the window but does not close it: another lane can still build between a touch and
    this lane's fingerprint check.
- **The gap wave 2 opens.** Fix passes and re-runs often leave the test count unchanged (U4d's round 2 will). There
  the count check cannot tell this copy's code from another copy's.
- **What wave 2's briefs add** (agent briefs and notes, no kit edit, within Mustafa's one-folder rule):
  1. **A lock.** Every cargo, rustc or rustdoc call in every lane runs under `flock` on one file,
     `.gate/shared-target/.lane.lock`: shared (`flock -s`) for ordinary builds, exclusive (`flock -x`) for a counted
     run (the touch, check.mjs, the `--list` and clippy confirmations) and for each bite run. No other lane can then
     build inside a counted run. flock 2.39.3 is present.
  2. **Provenance.** A counted run's log must show "Compiling <crate> (<this copy's path>)" for every crate the lane
     changed, in each cargo invocation that builds it. A missing line means "not counted", whatever the count says.
  3. **Disk.** 12 GB is free, with the shared target at 4.7 GB. U4d2's wasm32 release build adds a target triple.
     Each lane checks for at least 4 GB free before a wasm32 or release build (the 2 GB floor stays for the rest).
     Between waves, with no lane running, the main session may empty the shared folder by `cargo clean --target-dir
     .gate/shared-target`, an approved method (LEFTOVER-DELETE). §7.1 item 7 ("No copy cleans the shared folder")
     binds the copies, not the main session.
  4. **After the merges.** The main session removes `.gate/u4a-prefix` and `.gate/u4b-prefix` (the resume card
     already says so). Wave 1's worktrees go by the standing procedure (history.md, 2026-11-08: leftovers on a local
     branch, then a plain `git worktree remove`).
- Nothing here needs Mustafa: it keeps his one-folder rule as written.

W1-TARGET written 2026-11-10.
````

### The advisor's U4d close ruling: B1.2 and B1.3, for carry 52 (advisor-u4d-close.md)
_`.checkpoints/netcode-proof-test/advisor-u4d-close.md` lines 42-180 (sha256 cbc7e1c9210b), as they are:_

````text
**B1.2 N2-M1's fix form (carry 52): the generator stays open-loop, and its dead reckoning follows every input the
generator itself emits. A bounded re-home is added only if the measurement still shows a drift. `aim_one_in` is set on
the judged window. The main session lands it as U4d-F, together with N2-M2's scene fix (B1.3), before U4d2 pins
anything.**

Why this form:
- **Closed loop (the generator steps a World to read the true positions): refused.** §4.15 makes the recordings
  open-loop commands that U4d2 regenerates from the manifest (u4-build-plan.md:493-494). A generator that ran the World
  would tie every recording's bytes to the World's float path: any World change would re-pin every recording, and a
  cross-target difference in the World would show up as a different recording, not as U4d2's first mismatching tick.
- **Re-home alone: not as the fix.** It masks the error and leaves the model wrong. It multiplies the recording's
  teleports (48 a minute against about 6 today), each an input that forces a correction, so it changes what the
  recording exercises. And a teleport puts the soldier on the exact home point, where the into-the-air shots meet
  overlapping boxes: an event source of its own, tied to the period.
- **Reckoning what the generator emits fixes the cause for the main terms** (my reading of replay.rs:402-432 against
  world.rs:1276-1320 and :1112-1121):
  - ES holds fire on every command (`always_fire`, replay.rs:368), so sprint never applies in ES (world.rs:1295);
  - crouch halves the speed on about one heading in 10 (replay.rs:425-429);
  - a scripted MoveSpeed of 400-800 cm/s replaces 600 for one slot every 311 ticks and stays (replay.rs:512-521);
  - one command in 64 is dropped while the estimate still moves (replay.rs:501-509).
  Crouch alone is a random walk of about 3.75 m per crouched heading (6 m/s × about 1.25 s × 0.5): tens of metres
  over 10 minutes. That is my estimate, not a measurement. Deaths reset the true position near home (a respawn at a
  team spawn point, world.rs:1712-1727), so they bound the error rather than grow it. Blocked moves (greybox solids,
  closed doors, the wall, vehicle parts: world.rs:1192-1239) the generator cannot see; the measurement decides whether
  they need the re-home.

**U4d-F, in order.** Each step runs the gate and npt-sim's suites. A pin changes only in the step that changes it, so
every re-pin has one named cause (T2-L1's lesson).
0. **T2-L1's two unit tests, first** (in world.rs's module). U4d-F re-pins D-7, which is the exact scenario T2-L1
   names, so its invariants get their own tests before any pin moves:
   - (a) `part_center` at hull yaw 90° maps offset (+120, 0, 100) cm to hull + (0, +120, 100) cm within 1 cm (the old
     place_vehicles formula's sign);
   - (b) in one step, a soldier that moved out of its old box is hit at its new place by a shot with no Rewind (or:
     phase 3's soldier boxes equal `hit_volumes_from` of the post-move records).
   The test-engineer's plants O1, O2 and O3 (u4d-round2-test.md [5]) each fail one of them by name. No pin changes.
1. **One home for the move rule.** A pub(crate) function in world.rs gives one command's per-tick move from the
   soldier's move speed, its buttons (crouch halves the speed; sprint × 3 / 2 only when not crouching and not holding
   fire: world.rs:1293-1301) and its move vector (the scale above 127). `run_command` and the generator both call it.
   This step is behaviour-neutral: every test and every D-7 pin stays unchanged.
2. **N2-M2: ES's vehicles** (B1.3). Only ES's D-7 pins change; SS's, CO's and TC's stay unchanged.
3. **The reckoning, in every scene** (one generator, one model). The estimate moves by step 1's function, with:
   - the generator's own buttons;
   - its own record of each slot's move speed: the scripted MoveSpeed it emits, clamped as `apply_inputs` clamps it,
     from the tick `apply_inputs` applies it, which is before that tick's commands (world.rs:1112-1121);
   - no move for a command it drops (a held-back command moves it once, when the World runs it);
   - teleports and spawns resetting it, as now.
   `Recording::GENERATOR` goes from 1 to 2 (replay.rs:285). D-7's recordings and after-tick states are re-pinned in
   every scene. D-6 (SS) and the CO and TC tests pass with their assertions unchanged.
4. **The measurement (the acceptance), before `aim_one_in` is set:**
   - **The recordings:** for seeds 1, 2 and 3, an ES recording of 21,600 ticks (12 minutes) from the fixed
     generator. The window is its last 18,000 ticks: 10 minutes, PM-0001-12's ES run length (ADR 0001:1600), after a
     2-minute warm-up (the shortest warm-up ADR 0001 states for a run: PM-0001-63, :1634). A stationary generator
     makes the warm-up's length immaterial; that is what this step shows.
   - **Over the window, each seed:** shots 68.4-75.6 a second, gameplay events 9.5-10.5 a second, live projectiles
     30.4-33.6, spawns 20.064-22.176 a second; 48 soldiers and 15 vehicles on every tick; every ES hull within
     1,400 cm of the centre (B1.3).
   - **Against drift:** each seed's gameplay-event rate over each 300 s half of the window is also in 9.5-10.5.
   - **Reported per seed:** the four rates over the window; the two halves; the twelve 60 s event rates over all 12
     minutes; T-M1's 300 s from tick 0 beside them (reported, never used to set anything); and X2's form (ticks
     0-17,999, seed 2) beside X2's 7.578.
   - **3b, only if the drift check fails after step 3:** the bounded re-home, in ES only. Each soldier is placed at its
     home by a recorded teleport once every 60 s (slot k on the ticks where e mod 1,800 = floor(k × 1,800 / 48)), and
     its estimate is reset with it; no new draws. Then measure again.
5. **`aim_one_in`** (or its finer form, B1.1) is set from step 4's three seeds, and step 4's checks pass at the value
   chosen.
6. **Tests.** t_m1 becomes the window test for seed 2 in the measured suite: 21,600 ticks, about 11 s by X2's 9 s for
   18,000. The three-seed form is an `#[ignore]` test, run once by name for the report. Plants, each failing by name:
   - the old reckoning restored (scene speed only) fails t_m1's drift check;
   - one ES hull at 40 m fails the hull bound.
7. **Docs:** scene.rs:77-79's measured numbers, :114 and :177 name the window; the grenade-fuse doc (B1.1);
   replay.rs:458-461's comment.
- **If no setting** in steps 4-5 puts every seed's window and both halves in band, U4d-F stops and brings the numbers
  to me. The 9.5 floor does not move (Mustafa, 2026-11-11): the scene is what changes, never the band.

**Who lands it, and how it is reviewed:**
- **Who:** the main session, itself or through one builder in a worktree from the spike head. crates/sim has no
  wave-2 owner (u4-build-plan.md, Lanes). It merges before U4d2's brief is final. U4d2 bases on that commit and pins
  nothing generated before it.
- **U4e** need not wait: it edits no crates/sim file and pins no recording. Any ES number in U4e's report comes from
  U4d-F's scene.
- **Its own review budget:** U4d-F is its own change, not a round 3 of U4d. netcode-architect and test-engineer
  (Opus 5.5, xhigh) review U4d-F's diff; rust-reviewer only if step 1 changes a World line beyond moving the speed rule
  into the function.
- **Evidence:** check.mjs's counted run with the new counts, the gate, and step 4's table. Risk tier 1 (spike code,
  inside PR #40). Nothing for Mustafa's eyes.
- **This block is U4d-F's approved plan,** for these files and symbols:
  - crates/sim/src/world.rs: the move function; ES's loop placement in `World::new`; T2-L1's two tests;
  - src/scene.rs: the in-fight hull knob, `ES_AIM_ONE_IN`, the docs;
  - src/replay.rs: `Recording::synthetic`, `Recording::GENERATOR`;
  - tests/world.rs: t_m1 and the ignored three-seed test;
  - tests/replay.rs: D-7's pins.
  Anything beyond them comes back to me first. The main session's checkpoint points to this block as the plan.
  SEC-U4d-R2-1's accessor does not ride U4d-F: Mustafa deferred it to U4e (carry 51).

**Carry 52 (u4s-carries.md, §U4d2): faithful to Mustafa's words.**
- His quote in the label is verbatim (history.md:284-285).
- The body says "fix" of the estimate scene and keeps the 9.5 floor, as he did. "Or hash" is the carry's own word, not
  his: it covers ES's hashes (N2-M2), and it is stricter than his words, never looser.
- Text change: the label lines (his words) stay as they are. In the body, replace the span from "(per 50 s window:" to
  the carry's closing quote with the block below (no word of his is in the span; rewrap within the entry):
```
(per 50 s window: 13.4, 11.0, 10.1, 9.4, 9.7, 8.7 a second at 1 in 10; the test-engineer's probe X2, seed 2 at 1 in
    11: 7.578 a second over 600 s). The fix and the re-calibration of `aim_one_in` are measured on the judged window (a
    10-minute measured window after warm-up, ADR 0001:1600; 3 seeds), not on T-M1's 300 s from tick 0. PM-0001-2's
    floor of 9.5 events a second does not move. The form, the measurement and the owner are the advisor's U4d close
    ruling (advisor-u4d-close.md, B1.2 and B1.3): the main session's change U4d-F lands the fix, with ES's vehicles
    inside the fight (N2-M2), before U4d2's brief is final. U4d2 pins only recordings written by U4d-F's
    `Recording::GENERATOR` or later, and ES's hashes only from U4d-F's scene; its report names U4d-F's commit."
```

**New carry for U4r, from N2-M1's other half (A2-L3 = T2-M1's carry):** the judged ES runs are U4r's, driven by
npt-bots (§5 U4r; marks.json's ES runs). No U4 check shows that the brawl meets ES's bands before PM-0001-12 counts,
and §4.12's contract test resolves PM-0001-2 on SS suites only. u4s-carries.md §U4r gains this entry (copy the
block's lines as they are):
```
59. **A2-L3 and T2-M1, ES under the bots (u4d-round2-arch.md, u4d-round2-test.md; the advisor's U4d close ruling):**
    "U4r's contract test also runs a short ES suite of the brawl `Behaviour` (48 clients) and resolves PM-0001-2's ES
    checks on it: shots, gameplay events, live projectiles and spawns in their bands (W1-U4D T-M1's numbers) and the
    share of snapshots whose client set holds all 47 other soldiers and all 15 vehicles. A band or share missed goes
    to the advisor before any ES run of PM-0001-12 counts. The ES runs themselves stay under PM-0001-2's counter
    script, run by run (ADR 0001:1480)."
```

**B1.3 N2-M2, ES's vehicles: fix the scene now, in U4d-F step 2, before any ES pin.**
- **The rule:** ES's 15 vehicles loop inside the fight, every hull within 1,400 cm of the cluster's centre on every
  tick (the bound D-6 holds SS's very large units to). A pub(crate) scene knob counts the hulls that loop in the
  fight (SS 4, its very large units; ES 15). SS's placement stays as it is (scene.rs:183 and world.rs:760-764 today
  put all of ES's at 40 + 3v m).
- **Why not "U4e's report shows its region covers them":**
  - The region's size is not set yet: neither plan fixes it, and ADR 0001 leaves interest's values as data set from
    R2-INT-REC1's marks when interest management is built (R2-INT-OQ7, :1957).
  - ES's premise is every vehicle in every client's set on at least 99 % of snapshots (ADR 0001:1475-1477). If it
    held only under one region setting, PM-0001-12's ES runs would depend on a tuning value.
  - With the hulls inside 14 m, no hull is much farther from any soldier than the soldiers are from each other (homes
    10 m out and 17.3 m apart; each soldier wanders about 8 m). So a region that holds all 47 other soldiers holds the
    vehicles too.
- **Order:** before `aim_one_in` is set. In the fight the hulls are cover (a part hit is no gameplay event:
  world.rs:1572-1590) and they block moves (world.rs:1226-1235), and both move the event rate.
- **The share** stays PM-0001-2's check on every ES run, and U4r's contract test shows it (carry 59).
````

### The advisor's U4d-F ruling: R1 (advisor-u4df.md)
_`.checkpoints/netcode-proof-test/advisor-u4df.md` lines 28-87 (sha256 842ec092a4b4), as they are:_

````text
### R1. NF-M1 (Medium) with TF-L2 (Low): 40 stays, the acceptance stands as met, the report says what the level and the halves are, and a later re-measure judges by RM. Nothing here needs Mustafa's word. APPROVE.

**The finding holds.** I re-derived the netcode-architect's 36-minute figures from nc-long.log's own per-minute counts
(seeds 1-9, 23:26:56-23:27:26 UTC, exit 0): the nine seeds' mean per 300 s block from minute 2 is 10.096, 9.692, 9.941,
9.901, 9.799, 9.857 and 9.887 (minutes 32-35); the slope over minutes 2-35 is -0.0032 a second per minute around a mean
of 9.882; all 27 ten-minute windows (minutes 2-11, 12-21, 22-31) are in band, the lowest 9.537; 6 of the 54 single
300 s blocks are out of band; and seeds 1-3's minutes 2-11 equal the official windows (9.878, 9.743, 10.098). The
test-engineer's run through the test's own `EsRun` (te-probe-seeds4-9.log) gives seed 6's second half 9.446, the same
figure. The label stays Medium.

**Why 40 stays:**
- The acceptance as I ruled it (B1.2 step 4) is met on seeds 1, 2 and 3, and three independent runs agree on every
  figure (the builder's s6-three-seeds.log and end-three-seeds.log, the netcode-architect's nc-suite.log, the
  test-engineer's te-base-suite.log). The rule for choosing the share (the largest smallest margin) was written down
  before the sweep chose, and no band moved. Nothing in the acceptance changed after the fact.
- What the finding shows is that my per-seed half check is noisier at this level than I assumed when I wrote it: a
  300 s half's noise is about 0.2-0.36 a second, so at a level of 9.88 one half in about 9 misses by noise, and three
  seeds' six halves all pass only about half the time. That makes 40 one passing set of halves, not a margin. It does
  not make 40 wrong: the windows, the quantity PM-0001-2 judges run by run ("on every ES run", ADR 0001:1480), are all
  in band on seeds 1-9 (9.693 to 10.171; standard deviation 0.166 across the nine).
- A re-tune now would buy little and cost a re-pin. The share moves only the aimed hits (about 0.7 of ES's 9.9 events
  a second at 40; blasts and sky shots meeting the shooter's chest make the rest), so 36 or 38 would lift the level by
  about 0.1 at most, would be chosen on the same noise, and would re-pin ES's recording and widen round 2.

**Does any of it change the acceptance in a way that needs Mustafa's word? No.**
- His words bind two things: the scene is fixed before U4d2 pins the recordings, and PM-0001-2's 9.5 floor does not
  move. The drift is fixed: before U4d-F the rate fell from about 11.5 to about 4.3 a second within 12 minutes in every
  seed (measurement A, s4-probe-11.log); after it, the rate is flat over 36 minutes and nine seeds. The floor is
  unchanged in every check, now and in RM: every judged window lies in 9.5-10.5.
- The per-seed half check is mine (B1.2 step 4, "Against drift"), not his and not PM-0001-2's. RM below replaces it,
  for later re-measures only, by a drift check on the nine seeds' mean. That changes my rule, not his floor: RM keeps
  the floor run by run on every seed's window and widens it from 3 seeds to 9.
- t_m1, the counted test (seed 2), keeps its half check exactly as built. For given code it is deterministic, and it
  passes today (halves 9.880 and 9.606, window 9.743). Its job from now on is the tripwire that sends a later change
  that moves ES to RM; it is no longer the reckoning's guard (R2, FX-1).

**TF-L2, my erratum.** B1.2 step 4 says "A stationary generator makes the warm-up's length immaterial; that is what
this step shows." The step did not show it: inside the judged window the second half is lower than the first in 8 of
9 seeds (mean -0.404 a second; the nine seeds' halves average 10.096 and 9.692), the same in every seed, and the dip
does not recur after minute 12. The netcode-architect reads it as the hulls' loops, which run on the World's elapsed
ticks from zero in every run, so every run's minutes 2-11 see the same arrangement; I accept that reading as likely,
not proven. What the step did show is that the decay is gone. The report says so (T1), beside NF-M1's numbers.

**How a later re-measure judges drift: RM (carry 67; its words in T2).** In short:
- **When:** before the merge of any change that moves ES's synthetic recording (D-7's ES digest) or the World's
  outcome on it (t_m1's printed line), and at once whenever t_m1 fails.
- **Runs:** seeds 1-9, each a 36-minute ES recording (64,800 ticks from tick 1,000,000), counted as `EsRun` counts.
- **The floor, run by run:** each seed's window (minutes 2-11) meets all four bands, with 48 soldiers and 15 vehicles
  on every tick and every hull within 1,400 cm.
- **Drift:** the nine seeds' mean gameplay-event rate over each 300 s block from minute 2 to minute 31 lies in
  9.5-10.5 (today: 10.096, 9.692, 9.941, 9.901, 9.799 and 9.857, so today's scene passes RM). The slope over minutes
  2-35 is reported beside it.
- **Reported, not judged:** every seed's two halves.
- **The share** is re-set only when the floor or the drift check fails at the current value, then to the value whose
  nine-seed mean window lies nearest 10.0 with both passing, never on three seeds' margins; the numbers come to me
  before D-7 or t_m1 is re-pinned for it.
- **t_m1 failing while RM passes:** the session changes neither the share nor t_m1 and brings RM's numbers to me; I
  rule on t_m1's form then.

R1 written 2026-11-12.
````

### The advisor's U4d-F ruling: R4.1, TE2-5 (advisor-u4df.md)
_`.checkpoints/netcode-proof-test/advisor-u4df.md` lines 213-231 (sha256 842ec092a4b4), as they are:_

````text
**R4.1 TE2-5: CONFIRMED, with two corrections of fact and one brief wording for next time.**
- **The reading holds:** every npt-sim failure in run A had a cause the plan names, and none was retried into green.
  - t_m1, the old 300 s test at the old share, failed in step 2 (7.250, the hulls), step 3 (8.566, the reckoning) and
    step 4 (12.716, 3b), each time with the same values in both builds; it went green only through the planned steps
    (step 5's share, then step 6's rebuild). B1.3's "both move the event rate" foresaw it.
  - d7 failed in steps 2, 3, 4 and 5 on exactly the pin each step re-pins (B1.2: "a pin changes only in the step that
    changes it"), and passed after each re-pin.
  - The step 4-5 builder named the stop clause and left the ruling to the main session (u4df-build.md:355-357).
- **Correction 1:** t_m1 failed in steps 2 to 4, not 2 to 5. At step 5's first run it passed (seed 2's 300 s at 40:
  10.440); step 5's only failure was d7's named re-pin (s5-first-measured.log).
- **Correction 2:** the separate "re-run at once" happened in steps 2, 4 and 5 (s2-rerun-*, s4-3b-rerun-*,
  s5-rerun-*). In step 3 the first runs were already the whole crate with `--no-fail-fast` in each build, their
  output saved and both failures named in the notes (s3-first-measured.log, s3-first-bites.log), but with no
  separate re-run and no TE2-5 tag. The rule's purpose held (each failure named, from the whole output, never retried
  into green); the record should not claim a step-3 re-run.
- **Stricter than B2.3:** yes. B2.3 stops a unit's merge on an unexplained failure in its own crate; the build brief
  stopped a step group on any npt-sim failure, which a change whose plan expects failures between its steps cannot
  meet. Not stopping was right. Next time the builder brief says so in advance (T4, item 2), so a builder does not
  have to choose between its brief and its plan.
````

### The advisor's U4d-F ruling: R6, the reviewers' shell writes (advisor-u4df.md)
_`.checkpoints/netcode-proof-test/advisor-u4df.md` lines 301-322 (sha256 842ec092a4b4), as they are:_

````text
### R6. Process: the reviewers' shell writes. A brief wording, no new rule. APPROVE as recorded.
- **What happened:** each reviewer first wrote part of its notes with a python one-off (the netcode-architect blocks
  [5]-[8], the test-engineer blocks [11]-[12]), then rewrote its notes whole with Write, and says so at the top of its
  notes. The standing rule (the card's "Rules in force", Standing: "write files with Write and Edit, never cd plus a
  redirect") was broken twice; neither seat hid it.
- **Harm: none found.** Only each seat's own notes file under .checkpoints/netcode-proof-test/ was written that way,
  the folder its role allows; the test-engineer's end check shows the copy byte-identical at the end (its 183-file
  hash list, [14]); I re-hashed the five changed files and they equal the builder's final list; the main folder's
  `git status` shows only this file new. The netcode-architect's probe crates live in its scratchpad, outside every
  copy, by its notes ([9]).
- **Why the rule matters anyway:** a file tool's write is what the guard and the permission check read; a shell write
  goes around them, and a python write can clobber a file the agent did not mean to touch.
- **What the briefs said:** the build brief carried the rule in words ("Write files with the Write and Edit tools only
  (no sed, no shell redirect, no python writes; ...)"); the review brief said only "you have no Edit tool, so rewrite
  the notes file whole with Write each time", which tells how, not what is forbidden, and the test-engineer had Edit
  and slipped too.
- **The ruling:** no rule beyond the standing one. Every agent brief from now on, reviewers' included, carries the
  build brief's sentence word for word (T4, item 1). An agent that slips says so at the top of its notes, as both did,
  and the main session checks `git status` and the copy's hashes after the run, as it did here. Severity: Low
  (process); no push of its own.

R6 written 2026-11-12.
````

### The advisor's U4d-F ruling: T4 (advisor-u4df.md)
_`.checkpoints/netcode-proof-test/advisor-u4df.md` lines 459-470 (sha256 842ec092a4b4), as they are:_

````text
### T4. The card's "Rules in force", a new bullet after the U4j round-2 ruling's
```
- **The advisor's U4d-F ruling (advisor-u4df.md, R1, R4.1 and R6):**
  1. Every agent brief, builders' and reviewers' alike, carries this sentence word for word: "Write files with the
     Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt` writing files: apply
     rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with Write."
  2. A builder brief's TE2-5 stop clause reads, with the unit's crate named: "a failure in <crate>, or in a test that
     reads <crate>'s files, stops your step group (report it), unless your step's plan names it as expected (a pin
     your step re-pins, or a test a later step rebuilds): then record it under TE2-5 with its cause and the step that
     ends it, and go on." B2.3's merge stop is unchanged.
  3. RM (carry 67) for any change that moves ES's synthetic recording or the World's outcome on it.
```
````

### The advisor's U4j round-2 ruling B2.3: TE2-5 (advisor-u4j-r2.md)
_`.checkpoints/netcode-proof-test/advisor-u4j-r2.md` lines 146-171 (sha256 7efa9410323f), as they are:_

````text
### B2.3 TE2-5, an unnamed npt-net failure retried until green: a FOLLOW-UP in two halves, and a rule from now on
- **The finding holds, and the name is gone for good.** The fixer's saved output of check.mjs run 2 (its
  scratchpad's check-run2.txt, 12:39 UTC) ends "bites build: 299 tests passed, 1 failed" and "error: test failed, to
  rerun pass `-p npt-net --lib`", with no test name. Why: check.mjs's `run` joins stdout before stderr
  (check.mjs:1402), and `main` prints only the last 40 lines of a failed step (:1649), so libtest's `failures:` list,
  on stdout, is above the tail; cargo stops at the first failing binary. Then npt-net's lib passed three runs alone
  and the next two check.mjs runs, and the fixer called it "a timing flake". A test that fails once in eight
  attempts (the fixer's four check.mjs runs and three runs alone, the test-engineer's one counted run) is a defect
  until its cause is known, and a later green proves nothing about it.
- **Not U4j's, and why that is shown, not assumed:** U4j touches no npt-net source. npt-net's tests do read U4j's
  marks/SERIES.md (include_str!, the three files in M6), but those are fixed `contains` checks on fixed bytes, which
  cannot fail once in eight runs of one build. So it does not hold U4j's merge (M6 keeps watch).
- **Half (a), check.mjs names every failing test:** u4s-carries.md entry 65, §U4d2 (check.mjs is U4d2's in wave 2:
  u4-build-plan.md, Lanes). The main session may do it sooner in its own commit, before U4d2's lane opens; U4d2's
  report then lists entry 65 as met.
- **Half (b), the cause:** the main session, before the next merge into the spike branch after U4j's (U4d-F's or
  U4e's): the bites build's workspace tests, as check.mjs's test step runs them but with `--no-fail-fast`
  (`cargo test --locked --workspace --all-features --no-fail-fast`), ten times through the shared-target runner, each
  run's whole output saved under .gate/te2-5/. Any failure's name and panic go into the notes, and that test's cause
  is found and fixed in a planned change of its own. If none fails in ten, that is recorded, and the rule below stays
  until a name is caught.
- **The rule, from now on, for the main session and every builder brief until entry 65 is met** (the card's "Rules
  in force" carries it as this ruling's): a FAIL in check.mjs's test step is never retried into green unnamed. The
  failing build's crate is re-run at once through the runner with `--no-fail-fast`, its whole output saved under
  .gate/, and the failing test's name and panic go into the notes, tagged TE2-5. A failure in a crate the unit
  changes, or in a test that reads its files, stops that unit's merge until it is explained.
````

### The advisor's U4j round-2 ruling B6: plants by Edit (advisor-u4j-r2.md)
_`.checkpoints/netcode-proof-test/advisor-u4j-r2.md` lines 283-306 (sha256 7efa9410323f), as they are:_

````text
## B6. The test-engineer's two process slips: neither weakens the evidence
- **Plants by whole-file Write, not by Edit in place.** One planted practice.rs and one planted figures.rs, each
  holding its plants behind a `ZZ_PLANT` switch; one run per plant set; a whole-file restore.
  - What would weaken it: a planted file that differs from the reviewed one beyond the plant lines (a run then tests
    other code), or a restore that is not exact (the copy reviewed afterwards, and merged, would not be the one
    planted from).
  - What rules both out:
    - each planted file was diffed against the original rebuilt from wip/u4j.patch (only the plant lines differed);
    - each restore was hash-checked (practice.rs back to e074d58d, figures.rs to e9282de6), and all 66 paths equalled
      the start hashes at step 7 and again after probe2 (u4j-round2-test.md);
    - each run's failures match its plant exactly (run 9's ROLE: nothing else failed; run 2's BAND: the three band
      tests; runs 5 to 8 and 10 to 11: the named test with the plant's own message), so no hidden extra change shows.
  - The one gap: the planted files' diffs were checked but not saved, so "only the plant lines" rests on the
    reviewer's word, backed by the failure pattern. It does not change a conclusion. M1 and M4 re-check the hashes
    independently before anything lands.
  - From now on: plants by Edit in place, or the planted file's diff saved beside the run's output.
- **Three notes lines appended with python** (and, in the other two seats, a shell heredoc), then each file rewritten
  whole with Write, its content kept. The rule (Write and Edit only) exists so the guard sees every write. The lines
  are notes, not evidence: every result in them comes from the runner's exit code and the test runner's own summary
  line, and the counted check.mjs run is reproduced by M5. No weakening.
  - The cause is the seats, not the agents: all three say their run had no Edit tool. The main session gives the next
    review workflow's agents the Edit tool, or tells them to rewrite their notes whole with Write from the start.
- **The netcode-architect's first read-only commands ran outside run.sh** (ls, git status, grep, a python read of
  marks.json): reads only; no build or test result depends on them.
````

### The advisor's check of the U4e brief: F2-ROUTE, for carry 66 (advisor-u4e-brief.md)
_`.checkpoints/netcode-proof-test/advisor-u4e-brief.md` lines 277-310 (sha256 3485a6c65b85), as they are:_

````text
### Block 6. F2-ROUTE: U4c's netcode F2 (Medium, open) (done)
**Ruling: a carry for U4d2, the next free entry (66 at this writing), in u4s-carries.md's "U4d2 or the main session"
section; U4r's brief takes it if U4d2's is final first. F2 keeps its Medium label and stays open until the carry is
met. L-c8's judged half PASS stands, and its hash half holds today on the evidence below.**
- What the evidence shows (u4c-lc8-rerun.md; u4c-build.md:79-84; u4c-review-test.md:95-99): the merged build's npt-lab
  runs of P2, B1 and P4 seed 1 print whole-v2-log hashes (43745683ab36d67b, 722f20713e18352e, 432693efb9698c05) equal
  to those of the builder's scratch logs of the same runs; on those logs' bytes the builder's scratch harness gave
  SHA-256(v1 tag + datagram entries) equal to U2's 913db8e9…, feabf2f0… and b8a04f2f…, in the builder's build and
  again in the test-engineer's own build. Equal whole-log hashes mean equal logs, so the merged build's datagram
  entries are U2's.
- What it lacks: the datagram-entry hash comes from an uncommitted scratch harness, so no later run of -3's set can show
  it without rebuilding that scratch. That is F2's point, and it still holds. It is a reproducibility gap, not a doubt
  about U4c: no Critical or High, no security finding.
- Why U4d2: the change is in crates/lab (probe.rs and main.rs), which is U4d2's alone in wave 2 (u4-build-plan.md :842,
  :862 as the file stands); U4d2 already edits npt-lab (its `hash` subcommand), so no new lane is opened. It needs no
  npt-l1 change: `FateLog::for_each_canonical` is public (l1/src/lib.rs:351), and U4c's committed test reads the
  same bytes through it (l1/tests/l1.rs:481-532). Not U4e: crates/lab is never U4e's (condition 10 (b)).
- Not a "for later" feature: it is the evidence form an approved test (L-c8) names; -3's L1 set was re-run because U4c
  changed npt-l1's model, and the next model change would need the same re-run.
- **The carry's words** (copy as they are):
```
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
```
- PR #40's report lists F2 as open (Medium, netcode-architect's, U4c), with carry 66 as its route.
````

### The advisor's check of the U4e brief: L4, the lane beside U4d2 (advisor-u4e-brief.md)
_`.checkpoints/netcode-proof-test/advisor-u4e-brief.md` lines 335-336 (sha256 3485a6c65b85), as they are:_

````text
- **L4. Launch as section 1 says:** one builder (general-purpose, opus, xhigh) in a worktree, under W1-TARGET's lock,
  beside U4d2's lane, with at most three agents at a time counting reviewers.
````

### The advisor's check of the U4d2 brief: findings A1 to A15, U4D2-Q1 to Q12, conditions L1 to L5 (advisor-u4d2-brief.md, whole)
_`.checkpoints/netcode-proof-test/advisor-u4d2-brief.md` lines 1-755 (sha256 155f3376b47c), as they are:_

````text
# The advisor's check of the U4d2 builder brief (rev 1.1), 2026-11-12

Seat: a day session's advisor (workflow agent, opus), on main's `.claude/agents/advisor.md` (read with `git show
origin/main:.claude/agents/advisor.md`). Read only: no build, no run, no commit. Branch
`claude/netcode-proof-test-plan-hcky5g` at f94b078 (the main folder's head). The brief: u4d2-brief.md rev 1.1, sha256
e6bcefc841dd963dd4ce9c4bd72b3dc62efa5bf11aea2eb4aa06c3c51b9268fa (929 lines), as the task names it.

## Plan
1. Read the brief whole (N/u4d2-brief.md rev 1.1), the check (N/u4d2-brief-check.md) and the drafter's notes
   (N/u4d2-brief-notes.md, "Revision for the check").
2. Read the plan's §5 U4d2 (:666-702) and the parts of the plan, plan.md and the ADRs the brief leans on.
3. Read my conditions on U4d2 (advisor-u4.md 2, 7, 8, 10 (b)), W1-TARGET and W1-PUB's U4d part, the U4d-F ruling's T4,
   j-r2's B2.3 and B6, e-ruling's F2-ROUTE and L1-L4, e-r1's R6 and R9 and T2/T6.
4. Read carries "## U4d2 or the main session" (29, 41, 42, 52, 65, 66) and the recordings section (67, 71, 72).
5. Coverage table: every item of §5 U4d2 and of my conditions against the brief, testable as worded.
6. The check's findings TB-1 to TB-20 against rev 1.1 (TB-1 to TB-3 first; TB-7 and TB-11 with Q12).
7. Spot-check the brief's code claims at the head (crates sim, lab, l1, tcpinfo; check.mjs, check.test.mjs; Cargo.toml).
8. Rule U4D2-Q1 to Q12, with exact words.
9. The placeholders P1 to P8, 7.2's launch steps, `<base>`, the runner, U4e merging first; the lanes beside U4e.
10. The main session's points (task item 6).
11. Verdict, conditions, Decisions needed, For Mustafa.

## Blocks (written as each finishes)

### Block 1. What I read, and the state it rests on (done)
- The brief whole (929 lines, sha256 e6bcefc8…, as the task names it); the check (u4d2-brief-check.md, 276 lines); the
  drafter's notes whole (u4d2-brief-notes.md, 356 lines: facts F1-F25, choices C1-C17, conflicts K1-K15, the
  "Revision for the check" with its process slips and each finding's outcome).
- The plan (u4-build-plan.md, sha256 686a583c…, 1,079 lines): §2's Features and C-U7-1 rows (:157, :159), §4.1
  (:178-203), §4.11 (:392-407), §4.14's hash-map row (:476), §4.15 (:478-504), §5's preamble (:507-512), U4d's D-2,
  D-7 and Bites (:612-614, :629, :640-642), §5 U4d2 (:666-702), U4r's -30 comparer and Evidence lines (:766-767,
  :790-792), Lanes (:840-875), §6 (:877-893), §7 (:895-918), Q9 (:1002-1005). plan.md (rev 3.3, 2bef127b…) §4.2
  (:432-450) and §4.5 (:527-551), its out-of-scope list (:324-329) and R-8 (:1401), its U7 row (:1247).
- My rulings: advisor-u4.md conditions 2, 7, 8, 10, 12 and the Q3 and Q9 rulings (:90-101, :130-149, :154-157,
  :198-199, :207); W1-PUB's U4d part and W1-TARGET (advisor-u4-wave1.md :387-416, :558-586); the U4d-F ruling's T4
  (advisor-u4df.md :459-470); the U4j round-2 ruling's B2.3 and B6 (advisor-u4j-r2.md :146-171, :283-298); the U4e brief
  check (advisor-u4e-brief.md, whole: the form followed here); the U4e round-1 ruling's R6, R8, R9, T2, T6 and "Before
  the merge" (advisor-u4e-r1.md :317-353, :386-493, :504-600, :681-730).
- u4s-carries.md (cdab0240…, 490 lines): "## U4d2 or the main session" (:407-447: entries 29, 41, 42, 52, 65, 66), the
  recordings section (:351-379: 67, 71, 72), entry 34 (:62-69), carry 39 (:169-179); every other mention of U4d2 (:68,
  :178: neither routes work to U4d2).
- ADR 0004's PM-0004-21, -22 and -74 rows (docs/adr/0004-simulation-core.md:1142, :1143, :1184).
- The runners: u4e-run.sh whole (98 lines, sha256 669f7c65…), main-run.sh :1-25; .github/workflows/gate.yml whole
  (the only workflow).
- Code at the main folder's head (f94b078 at my start; the saver's 3de696c since, notes only): spikes/netcode-proof
  equals 28a85e7 (`git diff --stat 28a85e7 HEAD -- spikes/netcode-proof` empty); adb1bab (U4d-F) and d50396a (U4j) are
  ancestors of HEAD (`git merge-base --is-ancestor`, exit 0 each). Read: replay.rs :285-289, :367-373, :608-706;
  hash.rs :10-34, :64-116; world.rs :585-636, :792-796, :930-1000, :1016-1023, :1214-1229, :1924-1928, :1955-1962,
  :2012-2022, :2139-2146; sim/tests/replay.rs :21-43, :296-340, :380-390, :512-516; l1/src/lib.rs :320-356;
  l1/tests/l1.rs :476-535; lab (Cargo.toml, lib.rs, main.rs :1-20, :60-215, probe.rs :15-30, :110-125);
  ops/runs/c-u2-pm-0001-3.mjs :1-80; check.mjs :25-35, :70-175, :1196-1290, :1395-1445, :1527-1663;
  check.test.mjs :1094-1105, :1617-1660 and its four "only npt-client-wasm and npt-tcpinfo may" pins (grep);
  Cargo.toml :1-25, :45-56, :70-115; crates/proto/Cargo.toml; crates/client-wasm/Cargo.toml; every crates/*/clippy.toml;
  rust-toolchain.toml; .cargo/config.toml; wip/u4e.patch's file list and its two clippy.toml hunks.
- Stamps now: brief e6bcefc8…; plan 686a583c…; plan.md 2bef127b…; carries cdab0240…; df-ruling 842ec092…; j-r2
  7efa9410…; e-r1 7c129309…; history c62ac831… (1,925 lines); the card 2c40717e… (613 lines: it moved since the
  brief's 6b36da10… stamp, as a living file does; the brief cites it by section only, so nothing in the brief breaks).
- Disk now: 7.8 GB free on /home/user; the shared target 6.8 GB (220 MB of it release); the main folder's own cache
  (.gate/netcode-proof-test/target) 6.7 GB.

### Block 2. Coverage of §5 U4d2 and of my conditions, testable as worded (done)
"Testable" means the brief's instruction ends in an exit code and a test runner's own summary line (check.mjs's step
line and exit code, libtest's or node --test's summary), not in a reading.

| Plan item or condition | Brief | Verdict |
|---|---|---|
| hash-wasm cdylib, npt-sim only, `bites` forwards (:667-670) | 2.1 manifest | present, testable (2.4 (d)'s deps lines) |
| the seven exports, `#[unsafe(no_mangle)] extern "C"`, integers only (:668-669) | 2.1; Q3, Q7, Q8 | present; `begin`'s arguments are Q3's (ruled below) |
| the Node loader, `WebAssembly.instantiate`, streaming (:670-671) | 2.2 | present, testable (2.4 (a), (d)) |
| `npt-lab hash`, release, two native legs in two processes, first tick and field diff (:670-672) | 2.3 | present, testable (h6, h1) |
| `HashCompare` on two per-tick lists, reused by U4r's -30 comparer (:672-673; condition 2) | 2.3, h4 | present, testable; it reads no file and knows no recording, which is what U4r needs |
| the `wasm` step: release build, flags, Node run (a script: the runner refusal stands, its note updated), the comparison, wall time, 5 min (:675-676) | 2.4 (a); Q2 | present, testable; the CI half of its timing names a run that does not exist (finding A1) |
| the features comparison, like for like, measured and bites apart (:677-678; A12) | 2.4 (b); Q6, Q12 (b) | present, testable; a shallow walk is not yet caught (finding A5) |
| the build-flag check, refusal in the sources, read-back that fails, self-test bite (:679-683; condition 7 (a)) | 2.4 (c); Q4 | present, testable; never only reports |
| hash-wasm's rules: its own `[lints]`, one allow per export, the scan, LEAF and WASM_CRATES, its dependency rule (:684-687) | 2.4 (d) | present, testable |
| the third exception: invariant, test, `UNSAFE_EXCEPTIONS`, plan.md §4.2 (:688-693; condition 7 (b), (c)) | 2.1, 2.2, 2.4 (d), (e); Q7, Q8 | present; plan.md §4.2 already names three (plan.md:436-447: read) |
| 0 mismatching ticks over 27,000, native twice and Wasm once (:694) | h1 | present, testable, exact counts |
| PM-0004-21's cross-target vectors (:694-695; T22) | h2; Q11 | present, testable |
| Bites: std sin (:696-697) | 2.4 (a) item 6; 2.7; Q5 | present, testable both ways the scan can fall |
| Bites: hash-map, "caught, or recorded as not catchable on wasm32" (:697-698) | 2.4 (a) item 6; 2.7 | present, testable (D-2's rule per pair; the step FAILs if no pair was exercised) |
| Bites: an extra feature (PM-0004-74), each flag, a stray `unsafe`, an export not on the list (:698-700) | 2.4 (b)-(d); 2.7 | present; several plant runs cannot show all their expected failures under the runner as it is (finding A2) |
| Reviewers (:701-702) | 7.2 item 7 | present (xhigh, counted: condition 10 (c)) |
| §5's preamble: deltas, clippy host and wasm32 both builds, fmt, Cargo.lock by the named dependencies only, pub items (:507-512) | sections 4, 5, 6; 2.5; 2.9 | present, testable |
| condition 8 (C-U7-1: wasm-opt is U7's) | 2 intro; 2.4 (a) item 1; 2.6's "not built" | present; plan.md's U7 row holds C-U7-1 (plan.md:1247: read) |
| condition 10 (b) (only U4d2 edits crates/lab) | section 4's files | present |
| W1-TARGET items 1-3 | section 4 rules 1-6 | present (the lock per call, provenance by the four release builds, the disk floors) |
| carries 29, 41, 42, 52, 65, 66 | 3.1 (P1) and its notes; 2.3-2.5; h7 | present; each has a home, a test or a "not applicable" with its reason |
| 67, 71, 72 do not apply | 3.2; 2.8 | agreed: U4d2 edits no crates/sim file, regenerates with the merged generator and pins what D-7 pins; every scratch plant is restored before the next call. The reading holds only while that stays true, which 2.8's stop rule keeps |

- **Beyond the plan:** nothing built that the plan, a carry or a ruling does not name. The scan data file, the vector
  file, the lab's clippy.toml and the exit-code form of carry 66 (h7 (ii)) each answer a plan item or a carry; the
  "plan silent" choices (C1-C17) are the least that makes an item testable, and none adds a feature for later.
- **Carry 42, checked myself:** entry 34 (carries :62-69) was met by U4a (u4a-build.md:125-126 lists the fixes; the
  check's C-rows and history.md:1335 agree), so carry 42's condition does not hold and the brief rightly carries no
  word of entry 34.
- **Carry 52, checked myself:** adb1bab is an ancestor of the head; `Recording::GENERATOR` is 2 (replay.rs:288) and
  D-7 pins it (sim/tests/replay.rs:382); the brief's 1.1 makes `<base>`'s ancestry a recorded first step.

### Block 3. The test-engineer's check against rev 1.1 (done)
Each finding against rev 1.1's text, and where the text cites code, against the code at the head.
- **TB-1 High (placeholders by line range in living files): answered as asked.** P1 by its heading, its entry set
  asserted; P2 by its first words in history.md (found once at :292, the line before it :291 "  USAGE-SAVINGS: (a), all
  three, with these limits.", the continuation :293 ending "before it changes anything.": checked); P3, P4, P5 and the
  new P8 from the closed rulings, each file's sha256 asserted (842ec092… and 7efa9410…: both hold now) and each span's
  first and last words named (df-ruling :462-464 and :465-468, j-r2 :168-171 and :298: each lands, read); every card
  line number gone (grep: only section and item cites remain); 7.2 item 8's re-stamp and plan re-check. Two gaps the
  answer leaves, both Low, with words in block 4: a placeholder that stands in several places (A4), and the carries
  cites that move if U4e merges first (A7).
- **TB-2 High (no verdict rule for the bites half): answered as asked, with one addition that holds.** Hash-map: D-2's
  rule per pair (sim/tests/replay.rs:300-308, read), three pairs, a FAIL when no pair is exercised, every digest
  printed. Std-sin: the scan's result committed as data (crates/lab/), the step's (R, T', F) rule, its FAIL naming the
  libc; the self-test's four outcomes per bite. The addition (the scan data names its libc; a moved T' names the
  step's) is test strength, not a feature. Two Lows ride it: which recording the step arms when several catch (A8),
  and the libc note's "CI" (A1).
- **TB-3 High (Q4 (a)'s property untested): answered as asked.** The two live runs that only the read-back can catch
  (`target-cpu=haswell` on x86-64, `target-cpu=bleeding-edge` on wasm32), the argv pin per source, and the clean
  read-back's non-vacuity (sse2, sign-ext). The facts it rests on (R24, notes F21) are the test-engineer's and the
  drafter's own `rustc --print cfg` runs; I re-ran none (block 10). As worded, each live run ends in check.mjs's exit
  code and its deps line naming the feature: testable.
- **TB-4 Medium (the word-size plant's site): answered as asked.** `write_state` (world.rs:1926) is the walk both
  `dump` (:1018-1022) and `state_hash` (:2141-2145) call (read). The expected outcome per pair needs the step to print
  every comparison on a FAIL and the runner to run past the first failing command (A2).
- **TB-5 Medium: answered as asked** (two Wasm legs in two Node processes; the std source cited at the toolchain's tag;
  the step decides from the digests it prints, not from the citation, so no verdict rests on a page I did not read).
- **TB-6 Medium: answered as asked** (exact 9,000 and 27,000; the six cases, and more).
- **TB-7 Medium: answered with a change that holds (U4D2-Q12 (a), ruled in block 5).** I read the four pins (:288,
  :486, :508, :1141: each quotes "only npt-client-wasm and npt-tcpinfo may", and `unsafeProblems` builds that message
  from `UNSAFE_EXCEPTIONS`), the deps fixture's summary pin (:1099: a pattern over "held: …" and "(6 crate files and 1
  cargo config file(s) read)") and the steps pin (:1617-1658: it pins each step's commands, builds its wasm32 line from
  `WASM_CRATES` and pins no step list). The drafter's reading is right on the code. `<crate>` is replaced in the copy,
  as asked, but its second use reads badly (A3), and the deps fixture's call of `depsStep` is a test the builder
  extends, like the steps pin (A10).
- **TB-8 Medium: answered as asked** (the four release builds before check.mjs in the same `-x` call; matching on the
  path).
- **TB-9 Medium: answered as asked, but its CI half cannot be met:** no CI job runs check.mjs (A1).
- **TB-10 Medium: answered as asked** (an `#[ignore]` test run by name: libtest's summary line). One Low rides it: B1's
  burst and where the test sits (A6).
- **TB-11 Medium: answered with a change that holds (U4D2-Q12 (b)), with one self-test case added (A5).**
- **TB-12 to TB-20 Low: each answered as asked.** Checked against the text: 2.1's id rule (TB-12), 7.2 item 7 and the
  header (TB-13), 7.2 item 1 (TB-14: main-run.sh:13-15 sets the shared target, read), h3 (TB-15), 2.4 (c)'s three
  controls (TB-16), 2.5's one lock-writing call (TB-17), h2's reason and text file (TB-18), 2.4 (f)'s three cases
  (TB-19), rule 2 (TB-20).
- **So:** rev 1.1 answers all twenty, the three Highs as asked. What remains is mine: three Mediums (A1, A2 in block
  4; A12 in block 7) and nine Lows (A3-A10 in block 4; A11 in block 6), none a High, none a security finding (A12 adds a
  security seat; it finds no defect). (Counts updated at block 7.)

### Block 4. My findings on rev 1.1, each with its exact words (done)
Severity on CLAUDE.md's scale. Every "old" phrase is the brief's (or the plan's) text read with its line breaks and the
next line's indent as one space; I counted each in the flattened file (python, read only): each occurs exactly once,
the one ordered pair aside (A1 (a) before (d)). The fold rewraps to 120.

**A1, Medium: no CI job runs check.mjs, so the timing's CI half and the infra-engineer's seat name a run that does
not exist.** The only workflow, .github/workflows/gate.yml, runs tools/gate.mjs and its smoke test, never
spikes/netcode-proof/check.mjs; and plan.md puts "a CI job for `spikes/` (R-8)" out of scope (plan.md:328; R-8 at
:1401: "approved; no CI job"). Plan :702's "infra-engineer (the step's time in CI)" and the brief's "The CI figure is
the infra-engineer's, from CI's run of the merged tree" therefore read a number nobody can produce; the reviewer seat
would come back with nothing, and the det-u4.mjs decision (plan :790-792) would lack its input. A CI job is .github/
work (risk tier 2, Mustafa's go) and out of the proof test's scope, so the reading changes, not the scope.
- 2.4 (a), Timing: replace "The CI figure is the infra-engineer's, from CI's run of the merged tree (plan :702:
  "infra-engineer (the step's time in CI)")." with "No CI job runs check.mjs (.github/workflows/gate.yml runs
  tools/gate.mjs only, and a CI job for spikes/ is out of scope: plan.md:328 and R-8), so plan :702's "the step's time
  in CI" is read as the step's time as a fresh build meets it: the infra-engineer reviews your cold and warm figures
  against the 5-minute budget, and says what a CI job would change (the advisor's U4d2 brief check, A1)."
- 2.4 (a) item 6, std-sin: replace "(CI's glibc may differ from this container's 2.39)" with "(another machine's
  glibc may differ from this container's 2.39)".
- Section 5 item 4: replace "against the 5-minute budget (plan :676); CI's figure is the infra-engineer's (2.4 (a))."
  with "against the 5-minute budget (plan :676); the infra-engineer reviews these figures (2.4 (a): no CI job runs
  check.mjs)."
- 7.2 item 7 (after the 2.4 (a) replacement, the phrase occurs once): replace "infra-engineer (the step's time in CI);"
  with "infra-engineer (the step's time against its budget, from your cold and warm figures: no CI job runs check.mjs,
  A1); security-auditor (the third `unsafe` exception, and check.mjs's unsafe and rustflags scans: A12);". (Edited at
  block 7 to carry A12.)
- **The plan** (the main session's commit): §5 U4d2's Reviewers line, replace "infra-engineer (the step's time in CI)."
  with "infra-engineer (the step's time against its budget; no CI job runs spikes/: plan.md R-8; the advisor's U4d2
  brief check, A1), security-auditor (the third `unsafe` exception and check.mjs's unsafe and rustflags scans: the same
  check, A12)." (Edited at block 7 to carry A12.)

**A2, Medium: several plant proofs cannot show all their expected failures under the runner as it is.** u4e-run.sh
stops a call at its first failing command (:74-88), and section 4 rule 2 forbids leaving a plant in place between
calls. 2.7's h2 row expects, in one call, the `wasm` step to fail and then the h2 unit test to fail; the stray-unsafe
row expects the deps step and rustc to refuse; the export row expects the loader and the deps scan to refuse; plant (1)
of the h2 row expects h2 (i) and the step's vector check to fail. In each, the second command never runs. And the h2
row's key evidence, leg 1 against leg 2 at 0 while native against Wasm fails, shows only if the `wasm` step prints
every comparison on a FAIL; as worded it may stop at its first failing sub-check. The builder would have to bend a
rule or stop.
- 7.2 item 2: replace "u4d2-run.sh: u4e-run.sh with the log check's folder `.gate/u4d2/` (u4e-run.sh:30-33) and its
  header naming U4d2; nothing else changes." with "u4d2-run.sh: u4e-run.sh with the log check's folder `.gate/u4d2/`
  (u4e-run.sh:30-33), its header naming U4d2, and one option more, `-k`, given after `-x` when both are: every command
  of the call runs even after one fails, each exit code logged as now, and the call exits with the first non-zero
  code; without `-k` it stops at the first failing command, as u4e-run.sh:74-88 does. Nothing else changes (the
  advisor's U4d2 brief check, A2)."
- 1.4: replace "`bash <runner> [-x] <worktree> <log> <command...> [:: <command...>]`" with "`bash <runner> [-x] [-k]
  <worktree> <log> <command...> [:: <command...>]`"; and replace "exit code and the provenance lines (u4e-run.sh:1-12,
  :38-39, :58-62, :91-97)." with "exit code and the provenance lines (u4e-run.sh:1-12, :38-39, :58-62, :91-97). It
  stops at the first command that fails; with `-k` it runs them all and exits with the first non-zero code (7.2 item
  2)."
- 2.4 (a), after "Its last line (the line main prints: :1646) gives the ticks compared per pair, the mismatches, each
  bite's outcome and each timed call's seconds." add: "On a FAIL the step still runs every comparison it has the legs
  for, and prints one line for each (its pair, the ticks compared, the mismatches, the first mismatch with its field)
  and for h2's vectors and each bite, within the last 40 lines main prints under a FAIL (:1649), so one run shows which
  pairs agree (2.7's h2 row reads leg 1 with leg 2 beside leg 1 with Wasm); a trap or a failed regeneration ends only
  the legs that need it."
- 2.7's lead: replace "its commands go in one runner call with `-x` (separated by `::`);" with "its commands go in one
  runner call with `-x -k` (separated by `::`), so that each expected failure shows;".
- 2.7's h2 row: replace "in one -x call" with "in one `-x -k` call".
- Section 4 rule 2: replace "a plant's commands go in one call," with "a plant's commands go in one call (`-x -k`: 7.2
  item 2),".

**A3, Low: the stop clause's second `<crate>` reads badly.** Rev 1.1's one string in both places gives "or in a test
that reads npt-hash-wasm, npt-lab, check.mjs's wasm and deps steps or check.test.mjs's files". My U4e round-1 ruling
(R8) named the crates once and wrote "their files"; the same here.
- 7.2 item 3's P4 bullet: replace "with both `<crate>` replaced by "npt-hash-wasm, npt-lab, check.mjs's wasm and deps
  steps or check.test.mjs" (the T4 item: "reads, with the unit's crate named"); the script asserts two replacements and
  no `<crate>` left." with "with the first `<crate>` replaced by "npt-hash-wasm or npt-lab, in check.mjs's deps or wasm
  step, or in check.test.mjs" and "<crate>'s files" by "their files" (the T4 item: "reads, with the unit's crate
  named"; the form of the advisor's U4e round-1 ruling, R8); the script asserts the two replacements and no `<crate>`
  left."

**A4, Low: five placeholders stand in more than one place, and 7.2 item 3 does not say where the copy goes or how a
wrapped source is copied.** P2 stands at 1.3, 1.4 and section 4; P3 at section 4 and section 5 item 3; P4 and P5 at 1.5
and section 4; P8 at 2.7 and section 4; and 7.2 item 3's own list names each again. "The placeholder gone" then
either copies Mustafa's words three times, two of them inside parentheses mid-sentence, or fails its own assert; and
P3, P4 and P5 wrap across indented lines in their sources.
- 1.3: replace "(the builders' limits, `<<P2>>`)" with "(the builders' limits: section 4)".
- 1.4: replace "never a grep (`<<P2>>`)." with "never a grep (the builders' limits: section 4)."
- 1.5: replace "(`<<P4>>`, with `<<P5>>`)" with "(section 4's stop clause)".
- 2.7's lead: replace "or its diff saved beside the run's output (`<<P8>>`), one at a time" with "or its diff saved
  beside the run's output (section 4's plant rule), one at a time".
- Section 5 item 3: replace "a diff is applied by Edit (`<<P3>>`)." with "a diff is applied by Edit (section 4's write
  rule)."
- 7.2 item 3's lead: replace "(the anchor found once, the copy's first and last words as named, the placeholder gone,
  no other byte of the brief changed):" with "(the anchor found once, the copy's first and last words as named, the
  placeholder gone from its one home outside this list, no other byte of the brief changed; this list keeps the names
  as the record; P6 and P7 are values, filled wherever they stand outside this list; a source span that wraps is
  copied with each line break and the next line's indent read as one space and rewrapped to 120, and the assert
  compares copy and source with every run of white space read as one space):".

**A5, Low: the features comparison's self-test does not catch a walk of npt-sim's direct dependencies alone.** Rev
1.1's non-vacuity (each list holds npt-sim and its manifest's direct dependencies) and its one self-test case (libm,
a direct dependency) both pass a walk that stops one level down, which would miss a feature difference on ppv-lite86
or zerocopy (notes F4). TB-11's point was the walk's reach; one case closes it without a pinned list.
- 2.4 (b)'s self-test: after "fail, naming libm and the feature;" insert " recorded outputs where only a transitive
  dependency differs (zerocopy, below rand_chacha through ppv-lite86) fail, naming it (a walk of npt-sim's direct
  dependencies alone would pass them);".

**A6, Low: h7 (ii)'s -3 arguments leave B1's burst and the test's home unsaid, and carry 66's summary form is open.**
B1's burst comes from main.rs's `profile` with `--burst-at-s` defaulting to 50 (main.rs:75, :89-93), and `probe` is a
module of the binary, not of the library (main.rs:13-14), so a test that calls `probe::run` sits in main.rs's or
probe.rs's test module. The summary line prints the whole log's hash as 16 hex digits (main.rs:182).
- h7 (ii): replace "(the default packet count and rate: main.rs:73-75, probe.rs:20-22; the -3 script's call, :65-72)"
  with "(the default packet count and rate: main.rs:73-75, probe.rs:20-22; B1's burst at the default 50 s, built as
  main.rs's `profile` builds it: :75, :89-93; the -3 script's call, :65-72; `probe` is a module of the binary,
  main.rs:13-14, so the test sits in main.rs's or probe.rs's test module)".
- 2.3's carry 66 bullet: replace "and in the summary line (main.rs:167-183)," with "and in the summary line
  (main.rs:167-183; as the first 16 hex digits, the whole-log hash's form there, :182, or whole: your report says
  which),".

**A7, Low: the carries cites that move if U4e merges first are not re-checked.** U4e's merge commit applies T2, which
inserts carries 73-77 at §U4r's end (above u4s-carries.md :323), so the recordings section (:351-379, cited in 1.6
and 3.2's heading) and §U4d2 (:407-447, cited in 1.6 and U4D2-Q9) move; 7.2 item 8 re-checks plan cites only. (T6
rewrites three plan rows in place, one line each: no plan line moves, only the plan's sha256.)
- 7.2 item 8: replace "re-check every plan cite of this brief and fold the new lines in the main session's commit;"
  with "re-check every plan cite and every u4s-carries.md cite of this brief (T2 inserts carries 73-77 above the
  recordings section and §U4d2; T6 moves no plan line) and fold the new lines in the main session's commit;".

**A8, Low: which recording the std-sin step arms when the scan catches on more than one.**
- 2.4 (a) item 6: replace "If it records a catch (recording R, tick T', field F):" with "If it records a catch
  (recording R, tick T', field F; with catches on more than one recording, the first in the manifest's order, the
  others recorded beside it in the data):".

**A9, Low: `feed` has no result, so Q7 (a)'s rule must allow none.**
- 2.4 (d): replace "By U4D2-Q7 (a), each listed export's parameters and result hold u32 and i32 only;" with "By
  U4D2-Q7 (a), each listed export's parameters, and its result if it has one, are u32 or i32 only (`feed` returns
  nothing, which Wasm lowers to no result);".

**A10, Low: the deps fixture test's call of `depsStep` is a test the builder extends.** The fixture test calls
`depsStep(ok.dir, ok.env, () => ({ ok: true, meta: ok.meta }))` (check.test.mjs:1096-1098); each new deps-step check
that runs a subprocess (`cargo tree`, `rustc --print cfg`) is injected as `metadata` is (notes C9), so that call
changes in the step that adds the check, or it runs real subprocesses on the fixture.
- Section 4's expected failures: after "The steps pin (:1617-1658) is one you extend (2.4 (g)); a failure there is not
  expected (U4D2-Q12 (a))." add " So is the deps fixture test's call of `depsStep` (:1096-1098): each new check's
  injected runner (the `cargo tree` and `rustc --print cfg` calls, passed as `metadata` is: notes C9) joins that call
  in the step that adds the check."

### Block 5. U4D2-Q1 to Q12 (done)
The brief is written to its recommendations; where I rule as recommended, its body stands and only the words below
change. Plan words are for the main session's own commit before the launch; each old phrase occurs once in the
flattened plan (checked), and my advisor-u4.md condition 12's list of allowed plan edits gains them by this ruling.

**U4D2-Q1 (the plan's moved check.mjs cites): (a).** The plan is a record approved at a hash; section 0's table names
every moved line, and U4d2 moves check.mjs again. The U4e brief's way (u4e-brief.md:127-130).
- Brief, section 0: replace "This brief uses the lines at the head (U4D2-Q1 asks whether the plan text is re-pointed):"
  with "This brief uses the lines at the head (U4D2-Q1 (a): the plan's text stays as approved, and this table is the
  reading):".

**U4D2-Q2 ("§4.5's flags"): (a).** The build plan's §4.5 (:292-304) holds no flag; plan.md §4.5 (:533-535) is the set
the rest of the row's checks come from, and wasm-opt is U7's (condition 8).
- Plan, §5 U4d2's `wasm` step: replace "the release build with §4.5's flags," with "the release build with plan.md
  §4.5's flags (baseline x86-64, no `+fma` or `target-cpu=native`, simd128 and relaxed-simd off; wasm-opt is U7's:
  C-U7-1; the advisor's U4D2-Q2 ruling),".

**U4D2-Q3 (`begin`'s seed and scene): (b), `begin(bite, arm_tick)`.** `Replayer` reads scene and seed from the
header (replay.rs:8-15, read; `Replayer::new` takes nothing: :645-648, read), so (a) would
put a second reader of the header's layout outside replay.rs and (c) a signature that says what it does not do. The
manifest's SHA-256 check binds each file to its entry before a word is fed, which is stricter than two fields, and
`HashCompare` names each entry's recording, so a leg fed the wrong file fails at its first tick.
- Plan, §5 U4d2 Builds: replace "`begin(seed_lo, seed_hi, scene, bite, arm_tick)`," with "`begin(bite, arm_tick)` (the
  recording's header carries the scene and the seed, and the manifest's SHA-256 binds each file to its entry: the
  advisor's U4D2-Q3 ruling),".
- Brief, 2.1: replace "(by U4D2-Q3 (b); under (a) the plan's `begin(seed_lo, seed_hi, scene, bite, arm_tick)`,
  failing at the header when scene or seed differs)" with "(U4D2-Q3 (b), the advisor's ruling: the header carries
  scene and seed)".

**U4D2-Q4 (the read-back's flags): (a), with the toolchain named.** A bare `rustc --print cfg` sees no flag cargo
adds, and `cargo rustc --print` is unstable on 1.99.1 (notes F2), so (c) can fail only on a toolchain change, against
condition 7 (a)'s "never only reports"; (b) re-implements cargo's choice of source and can be wrong in the same way
cargo's own rules change. (a) is a superset, fail-closed, and rev 1.1 proves it with runs only the read-back can catch.
rustup chooses the toolchain by the folder (rust-toolchain.toml pins 1.99.1 in spikes/netcode-proof), so the read-back
must run there to read the rustc cargo uses.
- Brief, 2.4 (c) (ii): replace "`rustc --print cfg --target <t>` for x86_64-unknown-linux-gnu and wasm32-unknown-unknown,
  given," with "`rustc --print cfg --target <t>` for x86_64-unknown-linux-gnu and wasm32-unknown-unknown, run from
  spikes/netcode-proof (so rust-toolchain.toml picks 1.99.1, the toolchain cargo uses; the step's line names `rustc
  --version`), given,".
- No plan words: "read back with `rustc --print cfg` per target" (plan :680-682) still says what is built.

**U4D2-Q5 (the std-sin bite): (a).** U4d's site takes its input from the vehicles' loops and `with_bite` takes only an
arming tick (world.rs:626-635, :940-966, :994-1000, :1214-1229: read), so the "scan for inputs" can only be a scan of
the loops' ticks, which every recording with vehicles meets within one period (at most 3,900 ticks: :794, read). If
the targets differ, the committed step holds a real sin catch at its tick; if they never differ, ADR 0004:1142 itself
says "recorded as not catchable on those targets and a 1-ulp planted change stands in", and the scratch stand-in shows
the step catching a change at that site at its arming tick. The step's catch-at-the-tick machinery is re-proved on
every run by the hash-map bite's native pair. (b) would need a crates/sim change by the main session with no wave-2
owner, a new pub item, and carries 67-72 checked, for a bite whose recurring value over (a) is small. Not looser:
plan.md §4.5:551's "caught at the first tick it changes" holds both ways (a bite that changes nothing has no tick to
catch), and no ADR or mark word changes.
- Brief, 2.4 (a) item 6: replace "and runs no std-sin leg; U4D2-Q5's ruling says what stands in." with "and runs no
  std-sin leg; the stand-in is 2.7's scratch plant, shown once (U4D2-Q5 (a))."
- Brief, 2.7's std-sin row: replace "the stand-in is U4D2-Q5's" with "the stand-in is the scratch plant in this row's
  last column (U4D2-Q5 (a))".
- No plan words: the plan's Bites line ("a 1-ulp planted change stands in") holds as it is.

**U4D2-Q6 (which builds the features comparison compares): (a), three builds.** The 27,000-tick result means "the
server's sim" only if the native legs' build (npt-lab's) resolves npt-sim's features as the server's does; ADR
0004:1184 asks it of "every build that runs the sim". One more `cargo tree` call, equal today (notes F4).
- Plan, §5 U4d2's features bullet: replace "npt-sim's resolved features under the server build against the hash-wasm
  build," with "npt-sim's resolved features under the server build and under npt-lab's (the native legs' build)
  against the hash-wasm build (the advisor's U4D2-Q6 ruling),".

**U4D2-Q7 (the invariant's second clause): (a).** The Rust signature is the Wasm signature's source, the scan already
finds the listed exports, and an f64 parameter is a bite it can fail on. (b) would read the type section in the loader
for the same fact; (c) leaves a clause no test can fail, which ADR 0002:264-265's "a test" does not allow. A9's words
let `feed`'s missing result through.
- Plan, §5 U4d2's third-exception bullet: replace "refuses any not on the list (its bite below);" with "refuses any not
  on the list (its bite below), and the deps scan refuses a listed export whose parameters or result are not u32 or
  i32 (its bite: an export taking an f64; the advisor's U4D2-Q7 ruling);".

**U4D2-Q8 (the names, and "no exported name collides"): (a).** The native build is never loaded as a library, glibc's
`step@GLIBC_2.2.5` is a compat version no new link binds (notes F5), and the names stay the ones U7's Chromium check
will load (plan :885-886). The invariant stays true as written, with what it was checked against recorded in the crate
doc (2.1). No words change.

**U4D2-Q9 (which manifest lines are U4d2's): (a), and crates/client-wasm/Cargo.toml's lints comment with them.** The
carries' own section names U4d2 and these files, no wave-2 lane holds them, and comment lines change nothing cargo
resolves. The client-wasm comment (:9-13: "one of the workspace's two unsafe_code exceptions") becomes false at the
very commit that adds the third exception, so the same lane makes it true (the main session's point, block 8); no
other text outside check.mjs and check.test.mjs says "two" (grep: only Cargo.toml:75 and client-wasm's :9).
- Brief, 2.5: after "- crates/proto/Cargo.toml: carry 41's comment lines (:13-14) only, by U4D2-Q9 (a); no feature or
  dependency change." add "\n- crates/client-wasm/Cargo.toml: its lints comment (:9-13, "one of the workspace's two
  unsafe_code exceptions") names three, npt-hash-wasm the third, in the workspace comment's form; comment lines only
  (U4D2-Q9 (a))."
- Brief, 2.4 (e): replace "Cargo.toml:73-92, in 2.5)" with "Cargo.toml:73-92 and crates/client-wasm/Cargo.toml:9-13,
  in 2.5)".
- Brief, section 4's files: replace "crates/proto/Cargo.toml's two comment lines (by U4D2-Q9)." with
  "crates/proto/Cargo.toml's two comment lines and crates/client-wasm/Cargo.toml's lints comment (:9-13), comments
  only (by U4D2-Q9 (a))."
- Brief, 3.1's carry 41 note: replace "and nothing else; under (b) or (c) your report gives it as not met (not your
  lane)." with "and nothing else (U4D2-Q9 (a))."
- Brief, return item 9: replace "texts outside your lane that still say "two" exceptions
  (crates/client-wasm/Cargo.toml:9-13's comment: "one of the workspace's two unsafe_code exceptions");" with "any text
  outside your lane that still says "two" exceptions (crates/client-wasm/Cargo.toml's comment is yours: 2.5);".
- Plan, Lanes' wave-2 line: replace "U4d2 owns check.mjs, Cargo.toml's members, Cargo.lock, crates/hash-wasm and
  npt-lab's `hash`;" with "U4d2 owns check.mjs and check.test.mjs, Cargo.toml's members and its lints and libm comments
  (carry 29), Cargo.lock, crates/hash-wasm, crates/lab (npt-lab's `hash`, carry 66's probe change and its
  clippy.toml), and two manifests' comment lines only: crates/proto/Cargo.toml's bites comment (carry 41) and
  crates/client-wasm/Cargo.toml's lints comment (the exceptions now three) (the advisor's U4D2-Q9 ruling);".

**U4D2-Q10 (npt-lab's clippy.toml): (a).** §4.11 and my Q9 (a) ruling give npt-lab both bans (plan :393-395,
:1002-1005); U4d2 is crates/lab's only wave-2 editor and adds the comparer of a determinism check, and lab/src holds no
`HashMap`, `HashSet`, `Instant` or `SystemTime` today (notes F12; the check's C9 row). The workspace's
`iter_over_hash_type = "deny"` (Cargo.toml) already refuses iterating a hash map in every crate; the type ban is the
stricter half. server/clippy.toml's missing type bans are block 8's.
- Plan, §5 U4d2 Builds: replace "on the two legs' per-tick server hashes (the advisor's condition 2)." with "on the two
  legs' per-tick server hashes (the advisor's condition 2). npt-lab gains its clippy.toml: §4.11's clock ban and Q9
  (a)'s HashMap and HashSet type bans (the advisor's U4D2-Q10 ruling)."

**U4D2-Q11 (h2's shown failure): (b).** Plant (1) shows the comparison against the file can fail; only plant (2) shows
the check catches what T22 exists for: native and Wasm differ while native and native agree (A2 makes that one run
show both). Both are scratch, restored and sha256-checked, and change no committed npt-sim byte, so carries 67-72 stay
out of reach.
- Plan, §5 U4d2 Bites: replace "an export not on the loader's list (the advisor's condition 7 (b))." with "an export not
  on the loader's list (the advisor's condition 7 (b)); and the vectors' shown failures: a word of the committed
  vector file changed, and a length written as `usize`'s bytes in npt-sim's walk `write_state` (ADR 0004:1142's
  word-size bite), both scratch plants, restored (the advisor's U4D2-Q11 ruling)."

**U4D2-Q12 (rev 1.1's two changes to the check's fixes): (a) and (a).**
- (a) TB-7's list: the drafter's reading is the code's (block 3): the four pins fail from the `UNSAFE_EXCEPTIONS` edit;
  :1099 fails only if the fixture's counts or `DEPS_CHECKED`'s "held:" start change; the steps pin is one the builder
  extends. Naming a pin as expected that does not fail would let a real failure of it pass; P4's own words also let a
  step's plan name any further pin it re-pins. A10 adds the deps fixture's `depsStep` call beside the steps pin.
- (b) TB-11's non-vacuity: the manifest-derived set (npt-sim and its shipped direct dependencies, read from the `cargo
  metadata` the deps step takes). Not `SIM_DEPENDENCIES` (check.mjs:77): the deps step holds npt-sim's shipped
  dependencies inside that list (:130-133, a subset check), not equal to it, so a lawful drop would fail a check that
  read the list. A fixed five would fail the step on a lawful lock change too; the equality across builds already
  catches a set that differs between them. A5 adds the transitive case TB-11's point needs.

**Section 8 and 7.2, for all twelve (brief words):**
- Section 8: after the heading "## 8. Questions for the advisor (U4D2-Q1 to Q12)" insert:
```
**Ruled at the advisor's check** (advisor-u4d2-brief.md, 2026-11-12; the questions below are kept as asked, for the record):

| Question | Ruling | Where it stands |
|---|---|---|
| U4D2-Q1 | (a): the plan's cites stay as approved; section 0's table is the reading | 0 |
| U4D2-Q2 | (a): plan.md §4.5's flags; wasm-opt is U7's; the plan's words changed | 2.4 (a) |
| U4D2-Q3 | (b): `begin(bite, arm_tick)`; the plan's words changed | 2.1 |
| U4D2-Q4 | (a), the read-back run from spikes/netcode-proof | 2.4 (c), 2.7 |
| U4D2-Q5 | (a): the scan's result committed as data; a catch judged at (R, T', F); none, the scratch stand-in once | 2.3, 2.4 (a), 2.7 |
| U4D2-Q6 | (a): three builds; the plan's words changed | 2.4 (b) |
| U4D2-Q7 | (a): the scan holds the signatures; the plan's words changed | 2.4 (d) |
| U4D2-Q8 | (a): the plan's names; the crate doc records what the invariant was checked against | 2.1 |
| U4D2-Q9 | (a), with crates/client-wasm/Cargo.toml's lints comment; the plan's Lanes words changed | 2.5, section 4 |
| U4D2-Q10 | (a); server/clippy.toml's type bans are the main session's, after U4e's merge | 2.5 |
| U4D2-Q11 | (b); the plan's Bites words changed | 2.6 h2, 2.7 |
| U4D2-Q12 | (a) and (a), with the check's A5 and A10 | 2.4 (b), section 4 |
```
- 7.2 item 4: replace "folded by script into the places marked "by U4D2-Qn"; any plan text a ruling allows (Q1's
  cites, Q2's "§4.5", Q3's `begin`, Q6's comparison line, Q8's names, Q10's clippy.toml line, Q11's bite line) in the
  main session's own commit, as the U4e brief ruling's L2 did." with "answered by the advisor's check
  (advisor-u4d2-brief.md, blocks 4 and 5) and folded into the places it names (done); the plan text it changes (Q2,
  Q3, Q6, Q7, Q9, Q10, Q11 and A1) is in the main session's own commit, as the U4e brief ruling's L2 did."
- 7.2 item 9: replace "next the advisor's check before the build (the card's "Next steps, in order", item 3)." with
  "the advisor's check (advisor-u4d2-brief.md) is done and folded here (rev 1.2)."
- The header: replace "Next: the main session's fold (section 7), then the advisor's check before the build (the
  card's "Next steps, in order", item 3: U4d2's builder brief, with the advisor)." with "Done since: the advisor's check
  (advisor-u4d2-brief.md, APPROVE on conditions) and its fold (rev 1.2). Next: section 7.2's launch steps."

### Block 6. The placeholders, 7.2's launch steps, `<base>`, the runner, and U4e merging first (done)
**The anchors, checked against the files now:**
| Placeholder | Source and anchor | Holds now | If U4e merges first |
|---|---|---|---|
| P1 | u4s-carries.md, the heading "## U4d2 or the main session" (once, :407) to the line before the next "## " (:448); entries at line start exactly 29, 41, 42, 52, 65, 66 | yes (read) | holds: T2 adds nothing to §U4d2 and inserts no "## " heading; the section moves down (A7) |
| P2 | history.md, the line beginning "  - Builders: pass or fail comes from the exit code" (once, :292) and :293; the line before is "  USAGE-SAVINGS: (a), all three, with these limits." (:291) | yes (grep, read) | holds (by text; a newer decision above it moves it, harmlessly) |
| P3 | advisor-u4df.md (842ec092…: holds) :462-464, from "Write files with the" to "whole with Write." | yes | unaffected (closed) |
| P4 | advisor-u4df.md :465-468, from "a failure in <crate>" to "and go on.", the `<crate>` replacements per A3 | yes | unaffected |
| P5 | advisor-u4j-r2.md (7efa9410…: holds) :168-171, from "a FAIL in check.mjs's test step" to "until it is explained." | yes | unaffected |
| P8 | advisor-u4j-r2.md :298, after "From now on: " to the line's end ("plants by Edit in place, or the planted file's diff saved beside the run's output.") | yes | unaffected |
| P6 | `<base>` and its counts (7.2 item 1) | see below | `<base>` holds U4e; a counted run gives the counts |
| P7 | the runner u4d2-run.sh (7.2 item 2, with A2's `-k`) | to write | unchanged |
- With A4's words, P1-P5 and P8 each stand once outside 7.2 item 3's list, and the copy rule for wrapped spans is
  stated. 7.2 item 3's checks (anchor once, first and last words, placeholder gone, no other byte changed) then make
  each copy testable by the script's own asserts.

**`<base>`:** the spike branch's head at launch. It must hold U4d-F's merge adb1bab (carry 52: the report names it),
and it does today: adb1bab, d50396a and 28a85e7 are ancestors of the head, and spikes/netcode-proof equals 28a85e7's
(read, block 1). 1.1's `git merge-base --is-ancestor adb1bab HEAD` in the worktree makes it the builder's first
recorded fact. If spikes/ equals 28a85e7's at launch, 7.2 item 1's counts are 399 / 446 / 52 of 52 / 10 steps (the
card's L3 entry: the counted run at 28a85e7, .gate/u4e-l3/l3-check.log); otherwise a counted run through main-run.sh
gives them (the shared target, the lock exclusive: main-run.sh:13-15, :19-20, read).

**The runner:** u4d2-run.sh as 7.2 item 2 says, with A2's `-k`. Read whole: u4e-run.sh's worktree and log checks
(:26-33), lock (:38-39), idle wait (:40-48), 4 GB check (:49-57), touch first and every 2 s (:58-64), the sequence
(:73-88) and provenance (:91-97) are what section 4 needs. One more call needs `-x` (A11 below).

**A11, Low: two kinds of run that give a result are missing from rule 2's `-x` list.** h7 (ii)'s `#[ignore]` test and
h7 (iii)'s probe runs give carry 66's evidence; a shared-lock run could meet U4e's copy's npt-lab test binary (U4e's
`cargo test --workspace` builds npt-lab from its own copy, without carry 66's test). Section 5 item 6 runs them
counted, but 2.6 does not say so.
- Section 4 rule 2: replace "each bite run, the std-sin scan, and every run with a scratch plant in place;" with "each
  bite run, the std-sin scan, h7 (ii) and (iii), every run whose output your report gives as a result, and every run
  with a scratch plant in place;". (Checked: the phrase occurs once.)

**If U4e merges first** (before launch, or while U4d2 builds): what changes, and what the brief must say.
- Before launch: `<base>` holds U4e's merge commit; 7.2 item 1's counted run gives the counts (R20 superseded); T2
  moves the carries lines the brief cites (A7: re-checked under 7.2 item 8, P1 unaffected); T6 changes the plan's
  sha256 and three §4.1 rows in place, moving no line the brief cites (7.2 item 8 re-stamps). The step count at
  `<base>` stays 10 (U4e adds no step). U4e's room and client clippy.toml then hold the hash-map bans (wip/u4e.patch);
  R16 still holds for lab and server.
- 1.2's sentence on U4e then becomes history. Brief words, used only if U4e has merged before launch: replace "U4d2 runs
  in wave 2 beside U4e's lane (plan :863-867; e-ruling L4, :335-336): two lanes, no shared file (U4e's patch touches
  crates/room, crates/client and crates/server only: wip/u4e.patch's file list; e-r1 R9 M1, :445-448)." with "U4e
  merged at `<U4e's merge sha>` before your launch (its merge commit's spikes/ files are under crates/room,
  crates/client and crates/server only, none in your lane), so `<base>` holds it and its counts include it." (The
  phrase occurs once; checked.)
- While U4d2 builds (the likely order: U4e's fix pass and round 2 are ahead): nothing in U4d2's worktree changes. The
  brief must say so to the builder. Brief words: in 1.2, after "Reviewers count toward the three-agent limit (plan
  :872-873)." add " If U4e merges while you build, your worktree does not change: never merge, pull or rebase; your
  counts stay deltas on `<base>`, and the main session applies your work onto the merged head and recomputes the
  counts (plan :870-871)." (The anchor occurs once; checked.)
- My U4e round-1 ruling's M5 says "All 10 steps pass" for U4e's merge. If U4d2 merges first, M5 reads "all 11 steps
  pass", the counts are U4d2's merged counts plus U4e's (as M5 already says), and the pre-run builds also hold the
  four release builds of section 4 rule 3 (their npt-lab and npt-hash-wasm lines naming the main folder), since the
  `wasm` step then builds U4e's crates in release inside npt-lab. That is my reading of my own M5; it changes no other
  M-condition.

### Block 7. The lanes beside U4e: are the merge order and W1-TARGET's lock enough? (done)
**Files: disjoint.** U4e's fix pass and round 2 hold crates/room, crates/server (not schedule.rs) and crates/client
(e-r1 R8's bounds: "no manifest, no Cargo.lock, no npt-proto, npt-sim, npt-link or npt-lab file"); wip/u4e.patch's 23
files are all there (read). U4d2 holds check.mjs, check.test.mjs, the workspace Cargo.toml's members and comments,
Cargo.lock's one entry, crates/hash-wasm, crates/lab, and two manifests' comment lines (proto, client-wasm: Q9). No
file is in both. U4d2's scratch plants in crates/sim (2.7's h2 and std-sin rows) live in its own worktree only and are
restored before its next call.

**The shared target: enough, on the rules as written.** Both lanes build the same package names into one folder, so
each can replace the other's artifacts (W1-TARGET's finding). The lock makes every claim safe: a counted run, a bite
run, a plant run and (A11) every run whose output is a result are `-x`, which no other lane's call can enter, and each
starts by touching its own crates/, so cargo rebuilds that copy's sources; the provenance lines name the copy's path.
An ordinary `-s` build may meet the other lane's artifacts, but it proves nothing and is never reported. Two cases
checked:
- U4d2's plant in npt-sim compiles into the shared target; U4e's next call touches its own crates/ (npt-sim's sources
  included) first, so its npt-sim is rebuilt from its own, unplanted copy.
- U4e's copy builds npt-lab from its own copy under `cargo test --workspace` (without carry 66's test); U4d2's h7
  results are therefore `-x` (A11), and a libtest summary of "0 passed" there would show the wrong binary ran.
- Cargo.lock and members differ between the copies (U4d2's has npt-hash-wasm); each copy's `--locked` reads its own
  lock, and an artifact's hash does not depend on the lock's other entries, so neither lane's lock check meets the
  other's.

**The merge order: enough, with the counts recomputed.** One lane at a time (plan :910-913), whichever is approved
first; the second's merge recomputes the counts on the merged tree (plan :870-871). U4e is ahead (its fix pass runs;
round 2 follows), so U4e merging during U4d2's build is the likely order; block 6 gives the brief's words for it, and
U4d2's own merge conditions come with my ruling on its review (U4e's M1-M9 form: the base still fits outside U4d2's
lane, the bytes merged are the bytes reviewed, check.mjs counted on the merged tree with 11 steps).

**One more limit, the three-agent one.** Today the session runs U4e's fix-pass builder and this check. Once U4d2's
builder launches, two agent slots are taken (U4e's builder, or its round-2 seats, and U4d2's builder), so U4e's round 2
cannot run my R8's batch 1 of three seats at once. Ruling: while U4d2's builder runs, U4e's round-2 seats run at most
two at a time (batch 1: netcode-architect and security-auditor; batch 2: engine-architect and the test-engineer;
batch 3: rust-reviewer), still one round (my role file's "Review budget": a round in batches is one round). U4d2's own
review round later runs the same way.

**What the brief must say about U4e's merge:** block 6's words (1.2's added sentence for the builder; 1.2's
replacement if U4e merged before launch; A7's carries re-check). Nothing else: the builder never merges, pulls or
rebases, and 7.2 item 5 already states the counts' rule.

**A12, Medium: the panel has no security seat, though U4d2 widens the spike's `unsafe` guard.** U4d2 adds the third
`unsafe_code` exception and changes the scan that holds the other two (`unsafeProblems`, `UNSAFE_EXCEPTIONS`,
`EXCEPTION_LEVELS`), and adds the rustflags read-back to the guard against lint-lowering flags. CLAUDE.md step 5 names
security-auditor for "deps" changes, and U3c's precedent is exact: the security seat reviewed npt-tcpinfo's exception
and check.mjs's scan and found that the scan missed relaxations rustc accepts (u3c-review-security.md S1). The plan's
U4d2 panel (:701-702) omits it. Fix: security-auditor joins U4d2's review round (read only, xhigh, counted toward the
limit). Its words are folded into A1's two Reviewers replacements (block 4, edited to carry them).

### Block 8. The main session's points (task item 6) (done)
Resumed after a usage cut: blocks 1-7 stand as written; the head is d526f91 (autosaves and notes since f94b078; the
brief, plan, plan.md, carries, df-ruling, j-r2 and e-r1 stamps re-hashed: all as block 1 gives them; spikes/netcode-proof
still equals 28a85e7).

**(a) The card's stamp of the plan: confirmed, and already corrected.** `git show 1e8c188^:` of u4-build-plan.md hashes
to d928358b…, the head's to 686a583c…; 1e8c188 (an autosave, "terminated") holds one changed line, L3's §4.1 row
(`git show --stat`: one file, 1+ 1-). The card's resume line (:16-17) now reads "686a583c…; it read d928358b… before
1e8c188", and the brief's section 0 stamps 686a583c… (:53). Nothing in the brief rested on the old stamp. The two other
files that name d928358b (u4d2-brief-notes.md, u4df-packet.md) are records of their day: no change. The lesson is in
the card already (next step 1: stop the saver before an edit toward a commit that must hold both).

**A13, Medium: npt-server's clippy.toml lacks the type bans my Q9 (a) ruling gave it; the gap is the plan's text.**
- What: the plan's Q9 (a) (:1002-1005), which I ruled (advisor-u4.md:207), reads "The HashMap and HashSet type bans in
  room, client, server, lab and bots; the clock bans in server, lab and bots". §4.11 (:393-395) wrote it narrower: the
  type bans join room's and client's files, and the three new files (server, lab, bots) get "the clock ban". U4b's
  Builds (:560: "crates/server/clippy.toml with the std and tokio clock bans (§4.11)") and U4b's file followed §4.11:
  crates/server/clippy.toml (read) bans the clocks only and cites §4.11 in its header. U4b did what the plan said.
- Reach: small. The workspace denies `iter_over_hash_type` for every crate (Cargo.toml's [workspace.lints.clippy],
  read), which refuses the order hazard itself; crates/server holds no HashMap or HashSet (src and tests: grep, none),
  nor do U4e's patch's added lines outside the room's and client's new bans (grep). The type ban is the stricter half
  Q9 (a) chose. npt-lab gets both bans by U4D2-Q10; npt-bots has no clippy.toml yet (a three-line library stub; its U4
  half, `Behaviour`, is U4r's: plan :746).
- Why neither U4d2 nor U4e: crates/server is U4e's lane until U4e merges (Lanes; e-r1 R8's bounds), its fix pass runs
  on fixed bounds, and its round 2 reviews what changed, which these two lines are not. check.mjs reads no
  server/clippy.toml (`CLIPPY_TWINS` holds only the sim and proto pair: check.mjs:765), so the change touches no file of
  U4d2's.
- Plan §4.11 (the main session's commit before U4d2's launch, with the other plan edits): replace "new clippy.toml files
  for npt-server and npt-lab, and for npt-bots (all three with the clock ban," with "new clippy.toml files for npt-server
  and npt-lab, and for npt-bots (all three with the same HashMap and HashSet type bans, by Q9 (a), and with the clock
  ban," (found once in the flattened plan). U4b's Builds line stays: it records what U4b built.
- The card's "Next steps", a new item that follows U4e's merge (the main session writes it): "**After U4e's merge,
  before U4r's brief (the advisor's U4d2 brief check, A13):** crates/server/clippy.toml gains the HashMap and HashSet
  type bans in the form U4e's room and client files use, and its header names them (Q9 (a); plan §4.11); proof: a
  scratch `std::collections::HashMap` in crates/server/src refused by clippy by name under the runner (-x), restored by
  Edit and its sha256 checked; then check.mjs counted, every step PASS and the counts unchanged; its own commit, which
  the advisor's next ruling on the spike branch checks. U4r's brief gives npt-bots its clippy.toml with both bans
  (§4.11)."
- The brief: nothing more. U4D2-Q10's row in section 8's table (block 5) already says the server's bans are the main
  session's, after U4e's merge.

**(c) crates/client-wasm/Cargo.toml:9-13 still says "two": settled by U4D2-Q9 (block 5).** Read now: "(one of the
workspace's two unsafe_code exceptions, with npt-tcpinfo's two functions; …)". The comment joins U4d2's lane, comment
lines only, so the third exception and every text that counts the exceptions change in one commit; the brief's return
item 9 is reworded to match (block 5).

**A14, Low: the drafter's process slips; recorded, nothing redone.**
- What: two `cp` copies of rev 1 (the brief and the notes) into the session scratchpad, and one `rustc --print cfg …
  2>` redirect into a scratch file there (notes :243-246). The sentence the drafter worked under (T4 item 1:
  "Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; …)") is broken by both:
  the redirect by its words, the copies by its first clause.
- Why no redo: neither wrote in the repo or in .gate/ (the working tree was clean at my start; git shows only the
  brief, the notes, the card and this file changing since f94b078); the brief I checked is the file whose sha256 the
  task names, read whole; and the one fact the redirect carried (rustc's warning for a cross-target flag, notes F21) I
  re-ran myself (block 10, where it proves incomplete: A15; the slip did not cause that, the reading did). Rev 1 was in
  git (3032b88 holds e5869dbf…, the notes' own stamp), so `git show` would have served.
- Brief, section 4's files bullet: after "then restored and the sha256 checked against the clean file." insert " A
  `cp`, `mv` or `tee`, and a `>` or `2>` into a file, are shell writes too, in any folder, the session scratchpad
  included (this brief's drafter slipped so: u4d2-brief-notes.md, "Own slips"): read an older revision with `git show
  <sha>:<path>`, and let the runner's log hold a command's output." (The anchor occurs once.)

### Block 9. An erratum to my block 4 (done)
**A7 undercounts T2's carries.** Read again (advisor-u4e-r1.md :504-597): T2 inserts carries 73-81, not 73-77: 73-77
after entry 69 at §U4r's end (above :323, so above the recordings section), 78 and 79 in §U4g, 80 in §U4f, 81 in §U4h,
every one above §U4d2 (:407). The drafter's notes F19 had it right. A7's point holds (both sections the brief cites
move; P1, found by its heading, does not break), but its words change, and they are the ones the fold uses:
- In A7's replacement for 7.2 item 8, read "(T2 inserts carries 73-81 above §U4d2, 73-77 of them above the recordings
  section; T6 moves no plan line)" in place of "(T2 inserts carries 73-77 above the recordings section and §U4d2; T6
  moves no plan line)". Everything else in A7 stands.
- So the next free carry number once T2 has landed is 82 (block 8's A13 uses none: its item lives in the card).

### Block 10. The read-back's facts, re-run (`rustc --print`, allowed in this run) (done)
Block 3 left TB-3's facts (R24, notes F21) as the drafter's and the test-engineer's runs. This run allows `rustc
--print`, so I ran them from spikes/netcode-proof (rust-toolchain.toml pins 1.99.1; `rustc --version`: "rustc 1.99.1
(58cf8f9c3 2026-10-13)"), output to the terminal only, nothing built:
| Given | x86_64-unknown-linux-gnu lists | wasm32-unknown-unknown lists | exit |
|---|---|---|---|
| nothing | fxsr, sse, sse2 | bulk-memory, multivalue, mutable-globals, nontrapping-fptoint, reference-types, sign-ext | 0 |
| `-C target-cpu=haswell` | `fma` among them | **no target_feature at all**, warning "not a recognized processor" | 0 |
| `-C target-cpu=x86-64` / `native` | not run here (F21: x86-64 lists fxsr, sse, sse2) | **none at all**, the same warning (native: one warning per host feature) | 0 |
| `-C target-feature=-fma` / `+sse2` | fxsr, sse, sse2 | unchanged, warning "unknown and unstable feature" | 0 |
| `-C target-cpu=bleeding-edge` / `mvp` | **sse, sse2 only** (fxsr gone), warning "target feature `x87` must be enabled …" | bleeding-edge: adds extended-const, relaxed-simd, simd128, tail-call | 0 |
| `-C target-feature=+simd128` | unchanged, warning "unknown and unstable feature" | not run (the name scan's case) | 0 |
- TB-3's answer holds: `haswell` lists fma on x86-64 and `bleeding-edge` lists simd128 and relaxed-simd on wasm32, and
  the name scan passes both, so each live run can fail only by the read-back. The clean lists are F1's.
- **A15, Medium: a CPU name of the other target's does not "only warn"; it empties wasm32's list.** The brief says "A
  flag meant for the other target only warns and exits 0 … so giving every flag to both read-backs adds no false
  failure" (2.4 (c) (ii)), and R24 "a cross-target flag only warns, exit 0". True for target features; false for CPU
  names. With the brief's own non-vacuity rule (the wasm32 read-back must list `sign-ext`), an x86-64 CPU name in any
  source fails the step on wasm32, even in `CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS`, which cargo gives to
  x86-64 builds alone; and TB-3's haswell run, through `RUSTFLAGS`, fails the wasm32 read-back too, a second FAIL line
  the brief does not expect. Medium, not High: no wrong pass (it fails closed), and the only flags it refuses are CPU
  names a lawful build never needs (plan.md §4.5's baseline x86-64 is the default CPU; wasm32's is the default too);
  but a builder meeting an unexplained wasm32 FAIL could loosen the non-vacuity rule to pass it, and the controls'
  "(whose read-backs list no fma)" invites a whole-step pass the step will not give. My Q4 (a) ruling stands: this is
  the superset's cost, it refuses no needed flag, and (b) would still be cargo's rules re-implemented.
- Brief, 2.4 (c) (ii): replace "A flag meant for the other target only warns and exits 0 (measured: `-C
  target-cpu=bleeding-edge` on x86-64 and `-C target-cpu=haswell` on wasm32; notes F21), so giving every flag to both
  read-backs adds no false failure." with "A flag meant for the other target warns and exits 0, and a target feature of
  the other target's (`-C target-feature=-fma` or `+sse2` on wasm32, `+simd128` on x86-64) leaves the list as it is;
  but a CPU name of the other target's changes it: given `-C target-cpu=haswell`, `x86-64` or `native`, the wasm32
  read-back lists no target_feature at all, and given `-C target-cpu=bleeding-edge` or `mvp`, the x86-64 one lists sse
  and sse2 only, warning that x87 must be enabled (rustc 1.99.1, run by the advisor's U4d2 brief check, A15; notes F21
  saw the warning and the exit code). So an x86-64 CPU name in any source fails the step on the wasm32 read-back's
  non-vacuity below, even in `CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS`, which cargo gives to x86-64 builds
  alone: the superset's cost under U4D2-Q4 (a), fail-closed, and it refuses no flag a lawful build needs (plan.md
  §4.5's baseline x86-64 is the default CPU, so a CPU name adds nothing). That FAIL names the flags the empty read-back
  was given."
- Brief, 2.4 (c)'s self-test controls: replace "`-C target-cpu=x86-64` (whose read-backs list no fma: notes F21);" with
  "`-C target-cpu=x86-64` (whose x86-64 read-backs list no fma: notes F21; the name scan passes all three, and the whole
  step passes the first two and FAILs the third on the wasm32 read-back's non-vacuity, as A15's sentence above says);".
- Brief, 2.7's flags row: replace "`RUSTFLAGS="-C target-cpu=haswell"` (the read-back: x86-64 lists `fma`, notes F21;
  the name scan passes it);" with "`RUSTFLAGS="-C target-cpu=haswell"` (the read-back: x86-64 lists `fma`, notes F21,
  and the wasm32 read-back lists nothing, so it fails its non-vacuity too: A15; the name scan passes it);".
- Brief, R24: replace "a cross-target flag only warns, exit 0 | notes F21 |" with "a cross-target flag warns, exit 0; a
  cross-target CPU name empties wasm32's list and cuts x86-64's to sse, sse2 | notes F21; the advisor's U4d2 brief
  check, A15 |".
- Each old phrase occurs once in the flattened brief (counted, read only). Section 8's U4D2-Q4 text stays as asked
  (block 5's table is the ruling).

### Block 11. The verdict (done)
**APPROVE for launch, on conditions L1 to L5.** The build may start once L1 to L4 are done; L5 is the launch.
- Why not REVISE: rev 1.1 answers all twenty findings of the test-engineer's check, the three Highs as asked and TB-7
  and TB-11 with changes that hold (block 3; U4D2-Q12, block 5); every item of §5 U4d2 and of my conditions 2, 7, 8
  and 10 (b) is in the brief and ends in an exit code and a runner's own summary line (block 2); the carries routed to
  U4d2 each have a home, and 67, 71 and 72 rightly stay out (blocks 2 and 6); nothing goes beyond the plan. My
  findings are five Mediums (A1, A2, A12, A13, A15) and ten Lows (A3 to A11, A14), none High and no security defect
  (A12 adds a seat; it finds no defect), each with exact words a script can fold; A13 lies outside the brief (plan text,
  and a main-session item after U4e's merge). The twelve questions are ruled with exact words (block 5). Nothing needs
  a second check unless the fold goes beyond those words.
- Risk tier 1 (spike code on PR #40's branch; check.mjs is the spike's own check, not one of tools/' gate files); this
  ruling is a notes-folder text. U4d2's panel: rust-reviewer, engine-architect, test-engineer and infra-engineer (the
  plan's) and security-auditor (A12), at xhigh, counted toward the three-agent limit (condition 10 (c)): five seats in
  batches of at most three, two while U4e's round 2 runs (block 7), one round.

**Conditions (the main session's):**
- **L1. Fold this ruling into the brief by script (rev 1.2):** block 4's A1 to A10 (A7 as block 9 corrects it; A1
  carrying A12's seat), block 5's words for U4D2-Q1 to Q12 with section 8's table, the header and 7.2 items 4 and 9,
  block 6's A11 and its 1.2 sentence for the builder (1.2's replacement only if U4e merged before launch), block 8's
  A14 and block 10's A15; and, in block 5's replacement for 7.2 item 4, "(advisor-u4d2-brief.md, blocks 4, 5, 8, 9 and
  10)" for "(advisor-u4d2-brief.md, blocks 4 and 5)" and "(Q2, Q3, Q6, Q7, Q9, Q10, Q11, A1 and A13)" for "(Q2, Q3, Q6,
  Q7, Q9, Q10, Q11 and A1)"; and, in 7.2 item 8, "if the plan's moved (the advisor's U4d2 brief check's plan edits, L2,
  move lines below §4.11; U4e's merge applies T6 to §4.1 in place)," for "if the plan's moved (U4e's merge applies T6
  to §4.1),". Each old phrase is matched with its line breaks and the next line's indent read as one space; I counted
  each in rev 1.1: once each (A1's ordered pair aside: block 4), and no two replacements share a sentence's anchor. The
  fold is recorded under the brief's title. If a phrase fails to match, or the fold must change any other word, it
  comes back to me; otherwise there is no second check.
- **L2. The plan text, in the main session's own commit, after U4e's fix pass returns and before U4d2's launch:**
  u4-build-plan.md's edits for U4D2-Q2, Q3, Q6, Q7, Q9 (the Lanes line), Q10 and Q11 (block 5), A1 with A12's seat
  (block 4) and A13's §4.11 (block 8), each old phrase found once (counted), allowed by my condition 12 through this
  ruling. They lengthen §4.11 and §5 U4d2 and so move every later plan line, §5 U4e's included: none lands while an
  agent that reads the plan by line is running (U4e's fix pass works from u4e-brief.md's 41 plan cites). From that
  commit on, u4e-brief.md's, e-r1's and every earlier ruling's plan cites read at 686a583c (`git show
  1e8c188:.checkpoints/netcode-proof-test/u4-build-plan.md`), and U4e's round-2 briefs say so. Then 7.2 item 8 runs on
  the brief: section 0's plan stamp re-set, and every `plan :N` cite whose line moved re-pointed by script (the cited
  line's text found at its new number; a cite inside an edited paragraph takes that paragraph's new span), each
  asserted. 7.2 item 6 (carry 65) as the brief says.
- **L3. The placeholders and the runner (7.2 items 1 to 3, as L1 leaves them):** P1 to P5 and P8 by their anchors and
  sha256 asserts (block 6: each holds now); P6, `<base>`: the spike branch's head at launch, with `git merge-base
  --is-ancestor adb1bab <base>` exit 0 recorded, and its counts 399 / 446 / 52 of 52 / 10 steps if `git diff --stat
  28a85e7 <base> -- spikes/netcode-proof` is empty, else from a counted run through main-run.sh; P7, u4d2-run.sh per
  A2: its diff against u4e-run.sh (669f7c65…) saved beside it, showing only the log folder, the header and `-k`; and
  `-k` shown once before launch in U4d2's worktree, in the runner's shared mode (no `-x`): `false :: true` with `-k`
  exits 1 with both exit codes in its log, and without `-k` its log holds the first only (both logs under
  .gate/u4d2/).
- **L4. U4e's order, said to the builder (block 6):** 1.2's added sentence ("If U4e merges while you build, your
  worktree does not change: …"); if U4e merged before launch, 1.2's replacement with its merge sha, `<base>` holding
  it, the counts from a counted run, and 7.2 item 8 run for T2's carries 73-81 (block 9) and T6.
- **L5. Launch as section 1 says:** one builder (general-purpose, opus, xhigh) in its own worktree from `<base>`, under
  W1-TARGET's lock; at most three agents at a time counting reviewers, so U4e's round-2 seats run at most two at a time
  while it builds (block 7). A13's server item waits for U4e's merge and is not U4d2's.

**For later briefs and rulings (no action now):**
- U4r's brief: npt-bots' clippy.toml with both bans (A13; §4.11 as L2 leaves it).
- U4d2's review round: the infra-engineer judges the step's cold and warm times against 5 minutes, with no CI figure
  (A1); the security seat reads the third exception and the scans (A12); my merge conditions will follow U4e's M1 to
  M9 form, with 11 steps.
- U4e's merge: if U4d2 merges first, e-r1's M5 reads "all 11 steps pass", and its pre-run builds hold section 4 rule
  3's four release builds (block 6).

## What I checked, and what I did not
- **Checked, by reading:** block 1's files and code cites; every old phrase this ruling replaces, counted in the
  flattened brief (blocks 4, 5, 6, 8, 10 and L1: once each, A1's ordered pair aside) or plan (once each), and the
  anchors that share an item checked for overlap (none); P1 to P5 and P8's anchors and the two closed rulings' sha256
  (block 6); the plan's stamp history by git (block 8); every crate's clippy.toml, crates/server and crates/lab for
  HashMap and HashSet, wip/u4e.patch's added lines and `CLIPPY_TWINS` (block 8); T2's carry numbers (block 9).
- **Checked, by running (read only, nothing written):** `rustc --print cfg` for both targets, clean and with nine
  flags (block 10); the sha256 of every stamped file; git ancestry and diffs.
- **Not checked:** I built, tested, planted and timed nothing (the shared target is U4e's fix pass's, and this run
  allows no build). So the 27,000-tick equality, the Wasm legs, the hash-map bite's outcome on wasm32, the std-sin
  scan's result, the cold and warm times, the Wasm size and notes F4's `cargo tree` output (the features comparison's
  inputs) are the builder's to show, each by an exit code. I fetched no web page (F20's std 1.99.1 source: the step
  decides from the digests it prints, block 3), did not re-run notes F5's `nm -D` (Q8 rests first on the native build
  never being loaded as a library), and ran neither `-C target-cpu=x86-64` nor `native` on x86-64 (block 10's table
  says so). U4e's fix pass may still change wip/u4e.patch within e-r1 R8's bounds; block 7's lane reading assumes it
  keeps them.

## Decisions needed (Mustafa)
None. Every point here is a technical or test choice inside the documented vision, a route between lanes, or the text
of my own ruling (Q9 (a)'s type bans put back into the plan's §4.11): mine to decide. No word or decision of
Mustafa's changes, and nothing he sees, hears or feels is touched.

## For Mustafa
Nothing. (PR #40 stays needs-mustafa for its existing reasons; this ruling adds no action for you.)
````

### ADR 0002: the build flags, wasm-opt and the unsafe exceptions (docs/adr/0002-code-structure-build-toolchain.md)
_`docs/adr/0002-code-structure-build-toolchain.md` lines 252-266 (sha256 33a5fe1cbb34), as they are:_

````text
   the target's default `-Cpanic=abort` (R1-BND-F19), so a trap ends the sim's instance, and the restart after a trap
   is ADR 0004's (part 20) with ADR 0001's resync path. A trap carries no panic message, so the Wasm build's panic
   hook (ADR 0004 part 20, S20) writes the message and location into a fixed buffer in linear memory, and when the
   instance traps the Worker's host reads that buffer, through a fresh view of the memory (part 5), and attaches it to
   the trap's report. The server is built for baseline x86-64, never with `+fma`,
   `-C target-cpu=native` or the `relaxed-simd` target feature: the gate reads the build flags of `sim`, `room`,
   `client-endpoint`, `sim-wasm` and `server` and fails if one appears (the rule is ADR 0004's, part 7; the check is
   this record's). If wasm-opt is kept (R5-CI-OQ4), the check reads its command line too, because wasm-opt optimizes
   the shipped Wasm after rustc: it fails on binaryen's `--fast-math` (`-ffm`, "Optimize floats without handling
   corner cases of NaNs, rounding, and negative zero" in binaryen's own options, read by rust-reviewer at the step-10
   review; not in the research) and on any feature flag beyond the target's defaults (R1-BND-F19's list, r1 lines
   1019–1025), `relaxed-simd` above all.
   `unsafe_code` is denied in every crate; an exception needs a written invariant, a test and an `#[allow]` with its
   reason. The Wasm size budget has no research number, and this record owns it (R1-RR-OQ5): the boundary spike and
   the sim core spike measure the `sim-wasm` file (raw and gzip, after wasm-opt and without it, `opt-level` "s" against
````

### ADR 0002: the determinism suite's tools (docs/adr/0002-code-structure-build-toolchain.md)
_`docs/adr/0002-code-structure-build-toolchain.md` lines 501-506 (sha256 33a5fe1cbb34), as they are:_

````text
   - **The determinism suite's tools:** a `devtools` replay runner, built as the server builds the sim, plays a
     recorded match and writes one [state hash](../research/GLOSSARY.md#state-hash) per tick; the gate runs it twice in
     two processes and once as Wasm in Node; Playwright opens a replay page that runs the `sim-wasm` Worker in Chromium,
     Firefox and WebKit and returns the hash stream; every Wasm run uses the exact file G6 ships and G7 measures (after
     wasm-bindgen and wasm-opt, checked by its hash); a comparer reports the first mismatching tick and a field-level
     diff. The gate's set is three short recordings (two teams, three teams, bots only); nightly adds a 10-minute bot
````

### ADR 0004: PM-0004-21 and PM-0004-22 (docs/adr/0004-simulation-core.md)
_`docs/adr/0004-simulation-core.md` lines 1142-1143 (sha256 e369eecf653a), as they are:_

````text
| PM-0004-21 | The suite catches a divergence at the tick it happens; the hash function is fixed (S7; part 9) | each injected divergence (one std `sin` call, one std `HashMap` iteration, one presentation random draw inside the sim) reported at its arming tick T with a field-level diff: 3 of 3; reported: the hash function picked, and its cost per tick (mean, p99); and neither the hash nor the serializer depends on the target's word size: the picked function gives the same digest for a committed byte string on native x86-64 and on wasm32, and one committed state serializes to the same bytes on both (proposed: `usize` is 32 bits on wasm32 and 64 bits on x86-64, so a length or index written as `usize`, or a hasher fed through `write_usize`, would make PM-0004-19 fail on its first tick for no game reason) | CI (native ×2, Node, the three engines); RS (the cost) | each injection armed at a known tick T with a value that differs between the two runs (proposed: an injection that changes no state proves nothing): the `sin` input taken from PM-0004-22's table where std's `sin` differs between native and Wasm (if none differs, the injection is recorded as not catchable on those targets and a 1-ulp planted change stands in); the `HashMap` holding at least 32 keys whose order decides an outcome, with its two orders logged | 1 run per injection; the digest and serialization vectors join the unit vectors (20 identical verdicts). Control: a presentation draw outside the sim passes (R5-GATE-REC3's G10 row, r5 line 1721). Bites (the word size): a planted `usize` field in the serialized state must change the bytes between the two targets | R1-DET-REC6 (lines 2222–2240); R5-TEST-REC2's mark (r5 lines 1427–1428); R5-GATE-REC3 (r5 line 1721) |
| PM-0004-22 | The maths library is one implementation everywhere (S7; part 7; decides R1-DET-OQ8 and R1-DET-OQ10) | for each `libm` function the sim uses, and for `sqrt`: at least 10,000 fixed inputs (a committed table of edge cases and seeded values: proposed) give bit-identical outputs (`to_bits`) on native x86-64 and in Wasm in Node and in each browser engine; `sqrt` also equals a correctly rounded reference; if part 7's `rem_euclid` or `div_euclid` is to join the allow-list (neither is on it, so this row's pass today does not wait on them), the method gets the same check, at least 10,000 fixed inputs with (3 + 2^-22, 1 + 2^-23) in `f32`, (3 + 2^-51, 1 + 2^-52) in `f64` and their sign mirrors among them, and its outputs also equal by value (`==`, which leaves a zero's sign to `to_bits`) a correctly rounded Euclidean reference, the exact quotient and remainder by integer or rational arithmetic rounded once to the type: for (a, b) and its mirrors (−a, b), (a, −b) and (−a, −b), the quotients 2, −3, −2 and 3 and the remainders 1, 2^-23, 1 and 2^-23 in `f32` and 1, 2^-52, 1 and 2^-52 in `f64` (verifiers fixcheck-d3a and skeptic-6 and the Rust and test reviews, 2026-11-03); part 7's `min`, `max` and `clamp` helpers give the same bits on every target for equal signed zeros, (+0, −0) and (−0, +0), and a clamp with its bounds the wrong way round returns a bound with no panic (part 7: one defined argument on equal inputs); reported: each function's largest error in units in the last place against a higher-precision reference (R1-DET-OQ8's "enough" read as PM-0004-30's 1 mm apex and PM-0004-54's 1 cm cursor marks passing with `libm`: proposed), and `libm`'s cost against std's, native and in Wasm | CI; MAC (Safari); RS and ML (the cost) | unit vectors in `cargo test` and wasm-bindgen-test (R5-TEST-REC1) | 20 identical verdicts. Bites: one output moved by one unit in the last place on one target; and, for the Euclidean clause when it runs, errors every target shares: a `div_euclid` planted as rustc 1.97.0's steps (`trunc(a / b)`, corrected by one only when `a % b < 0`: skeptic-6's disassembly, 2026-11-03), which gives 3 for (3 + 2^-22, 1 + 2^-23) where the exact quotient is 2, and one `rem_euclid` output moved by one unit in the last place on every target alike | R1-DET-REC2 (lines 2165–2168); R1-DET-OQ8 (lines 2704–2705); R1-DET-OQ10 (lines 2710–2712) |
````

### ADR 0004: PM-0004-74 (docs/adr/0004-simulation-core.md)
_`docs/adr/0004-simulation-core.md` lines 1184-1184 (sha256 e369eecf653a), as they are:_

````text
| PM-0004-74 | One feature set builds the sim on every target (S22; part 7; the id is ADR 0002's) | the resolved feature list of every dependency of `sim` (`cargo tree -e features`) identical for every build that runs the sim (`server`, `devtools`, `navbake` and `sim-wasm`), each resolved by the exact command CI and the release use: 0 differences | CI | `cargo tree -e features` run for `sim` in each build's target and features; the two lists sorted and compared by script | 1 run; the foundation re-proves it as a gate check (ADR 0002). Bites: a planted extra feature on a shared dependency in `server` makes the check fail (R5-LAYOUT-REC4's own); control: a dev-only feature on `devtools` that touches no `sim` dependency passes | R5-LAYOUT-REC4 (r5 lines 1216–1218); the draft's S22 |
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
