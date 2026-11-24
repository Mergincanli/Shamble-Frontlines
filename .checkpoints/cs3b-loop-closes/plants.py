#!/usr/bin/env python3
"""Planted defects for change-set 3a: each plant breaks one thing in a source file, runs the check that should catch
it, and puts the file back. A plant counts as caught only when the run fails AND its output holds the plant's `must`
pattern on a line that marks a failure (vitest's FAIL line of the named test, cargo's panic text, or Playwright's
code frame line with its '>' mark), never because some other check failed.

Run from the repository's root, after a clean build:  python3 -I -B .checkpoints/cs3a-rules-boundaries/plants.py [ids...]
It refuses to start on a tree whose tracked source differs from what it would restore to (it restores by content).
"""
import pathlib
import re
import subprocess
import sys
import time

ROOT = pathlib.Path.cwd()
WB = str(ROOT / ".gate/wb-tool/x/wasm-bindgen")
OUT = ROOT / ".gate/cs3a"
VITEST = ["node", "node_modules/vitest/vitest.mjs", "run"]
CLIENT = VITEST + ["--root", "packages/client"]
WORKER = VITEST + ["--root", "packages/sim-worker", "--dir", "packages/sim-worker/src"]
SCAN = ["cargo", "test", "-p", "sim", "--locked", "--test", "rules_scan"]
RNG = ["cargo", "test", "-p", "sim", "--locked", "--lib", "rng::"]
HOOK = ["cargo", "test", "-p", "sim-wasm", "--locked", "--lib", "panic::"]
WASM_ONLY = ["node", "packages/client/scripts/build.mjs", "--wasm-only"]
BOUNDARY = VITEST + ["--root", "packages/sim-worker", "--dir", "packages/sim-worker/boundary"]
STREAM = ["cargo", "test", "-p", "devtools", "--locked", "--test", "stream_vectors", "the_generator_gives"]
F32 = ["cargo", "test", "-p", "devtools", "--locked", "--test", "f32_vectors", "every_allowed"]
PINS = ["cargo", "test", "-p", "devtools", "--locked", "--test", "f32_vectors", "a_pinned_case"]
BUILD = ["node", "packages/client/scripts/build.mjs", "--out-dir", str(ROOT / ".gate/dist-plant")]
E2E = ["node", "node_modules/@playwright/test/cli.js", "test", "-c", "packages/e2e/playwright.config.ts", "--project=chromium",
       "--workers=1", "--retries=0", "--reporter=list", "--global-timeout=300000", "--trace", "off"]


def vitest_fail(title):
    """vitest prints ' FAIL  <file> > <title>' (and a line with a cross) for a failed test only."""
    return r"(FAIL|×).*" + re.escape(title)


def frame(source):
    """Playwright's code frame marks only the failing line with '>'."""
    return r"^\s*>\s*\d+ \|.*" + re.escape(source)


# id, what is planted, [(file, old, new)], commands (each a list; a build first for the browser ones), must
PLANTS = [
    ("f8-unknown", "the link takes a message it does not know as nothing",
     [("packages/client/src/app/worker-link.ts",
       'if (!isWorkerToPage(data)) return this.#fail("the Worker sent a message the page does not know");',
       'if (!isWorkerToPage(data)) return "";')],
     [CLIENT + ["src/app/worker-link.test.ts"]], vitest_fail("a block of the wrong length, an error from the Worker and an unknown message")),
    ("f8-stats-field", "the statistics check leaves its first field out",
     [("packages/sim-worker/src/messages.ts", "for (let i = 0; i < STATS_KINDS.length; i += 1) {", "for (let i = 1; i < STATS_KINDS.length; i += 1) {")],
     [WORKER + ["messages.test.ts"]], vitest_fail("the statistics are refused when one field is missing")),
    ("f8-done-at", "a block whose time is not a number is taken",
     [("packages/sim-worker/src/messages.ts", ' && Number.isFinite(value["doneAt"]);', ";")],
     [WORKER + ["messages.test.ts"]], vitest_fail("anything else from the Worker is refused")),
    ("n3-no-terminate", "the end of the match does not stop the Worker",
     [("packages/client/src/app/worker-link.ts", "    this.#port.terminate();\n", "")],
     [CLIENT + ["src/app/worker-link.test.ts"]], vitest_fail("the match ends once, whoever ends it first")),
    ("n3-ends-twice", "a second cause ends the match again",
     [("packages/client/src/app/worker-link.ts", '    if (this.#failure !== "") return false;\n', "")],
     [CLIENT + ["src/app/worker-link.test.ts"]], vitest_fail("the match ends once, whoever ends it first")),
    ("n3-posts-after", "the link still posts the tab's visibility after the end",
     [("packages/client/src/app/worker-link.ts", '    if (this.#failure !== "") return;\n    this.#port.postMessage({ type: "visibility", hidden }, []);',
       '    this.#port.postMessage({ type: "visibility", hidden }, []);')],
     [CLIENT + ["src/app/worker-link.test.ts"]], vitest_fail("the match ends once, whoever ends it first")),
    ("f13-writable", "a field of the link is writable from outside again",
     [("packages/client/src/app/worker-link.ts", "  get posts(): number {\n    return this.#posts;\n  }\n",
       "  get posts(): number {\n    return this.#posts;\n  }\n\n  set posts(value: number) {\n    this.#posts = value;\n  }\n")],
     [CLIENT + ["src/app/worker-link.test.ts"]], vitest_fail("what the link knows can be read and not written from outside")),
    ("fps-median", "frames a second from the usual gap again",
     [("packages/client/src/harness/frame-times.ts", "const mean = this.#deltas.mean();", "const mean = this.#deltas.percentile(50);")],
     [CLIENT + ["src/harness/frame-times.test.ts"]], vitest_fail("frames a second is a count over time")),
    ("mean-capacity", "the ring's mean divides by its capacity",
     [("packages/sim-worker/src/stats.ts", "sum / this.#count;", "sum / this.#values.length;")],
     [WORKER + ["stats.test.ts"]], vitest_fail("the mean is over the samples held, and only those")),
    ("neg-zero", "a value that rounds to zero keeps its minus sign",
     [("packages/client/src/overlay/report.ts", 'return Number(text) === 0 ? text.replace("-", "") : text;', "return text;")],
     [CLIENT + ["src/overlay/report.test.ts"]], vitest_fail("a value that rounds to zero prints as zero")),
    ("nt3-control", "the error text keeps its control characters",
     [("packages/client/src/app/error-hook.ts", '.replace(/[\\u0000-\\u001f\\u007f]+/g, " ")', "")],
     [CLIENT + ["src/app/error-hook.test.ts"]], vitest_fail("the newest error's text is kept as one line of bounded length")),
    ("nt3-length", "the error text is not cut",
     [("packages/client/src/app/error-hook.ts", ".slice(0, 300)", "")],
     [CLIENT + ["src/app/error-hook.test.ts"]], vitest_fail("the newest error's text is kept as one line of bounded length")),
    ("nt4-after-copy", "the hand-off's time is read after the copy",
     [("packages/sim-worker/src/host.ts", "this.#env.post({ type: \"block\", buffer, doneAt: this.#env.timeOrigin + before }, [buffer]);",
       "this.#env.post({ type: \"block\", buffer, doneAt: this.#env.timeOrigin + this.#env.now() }, [buffer]);")],
     [WORKER + ["host.test.ts"]], vitest_fail("the hand-off's time is read when the step returned, before the copy")),
    ("favicon", "the icon request falls through to the files",
     [("packages/client/serve.mjs", "if ((request.url ?? '').split(/[?#]/, 1)[0] === '/favicon.ico') {", "if (false) {")],
     [CLIENT + ["serve.test.mjs"]], vitest_fail("the browser's own request for an icon")),
    ("l4-refusal-policy", "a refusal goes out without the policy header",
     [("packages/client/serve.mjs", "  response.writeHead(status, {\n    ...COMMON_HEADERS,",
       "  response.writeHead(status, {\n    'X-Content-Type-Options': 'nosniff',\n    'Cache-Control': 'no-store',\n    'Cross-Origin-Resource-Policy': 'same-origin',")],
     [CLIENT + ["serve.test.mjs"]], vitest_fail("every method but GET and HEAD")),
    ("tel-lexical", "a path that leaves the folder reaches the file system",
     [("packages/client/serve.mjs", "if (fromRoot === '' || fromRoot.startsWith('..') || path.isAbsolute(fromRoot)) return { status: 404 };",
       "if (fromRoot === '' || path.isAbsolute(fromRoot)) return { status: 404 };")],
     [CLIENT + ["serve.test.mjs"]], vitest_fail("a path that leaves the folder, or names the folder itself, before the file system is asked")),
    ("tel-empty", "the folder itself reaches the file system",
     [("packages/client/serve.mjs", "if (fromRoot === '' || fromRoot.startsWith('..') || path.isAbsolute(fromRoot)) return { status: 404 };",
       "if (fromRoot.startsWith('..') || path.isAbsolute(fromRoot)) return { status: 404 };")],
     [CLIENT + ["serve.test.mjs"]], vitest_fail("a path that leaves the folder, or names the folder itself, before the file system is asked")),
    ("tel-slash", "an address without a leading slash is looked up",
     [("packages/client/serve.mjs", "  if (!rawPath.startsWith('/')) return { status: 400 };\n", "")],
     [CLIENT + ["serve.test.mjs"]], vitest_fail("an address that does not start with a slash")),
    ("tel-not-a-file", "a folder with a served ending is answered as a file",
     [("packages/client/serve.mjs", " || !info.isFile()) return { status: 404 };", ") return { status: 404 };")],
     [CLIENT + ["serve.test.mjs"]], vitest_fail("a folder whose name has a served ending")),
    ("n6-prefix", "the page's chunks are told from the Worker's by a name prefix again",
     [("packages/client/scripts/build-checks.mjs", "const page = scripts.filter(({ name }) => name !== workerEntry);",
       "const page = scripts.filter(({ name }) => !/(^|\\/)worker-[^/]*\\.js$/.test(name));")],
     [CLIENT + ["scripts/build-checks.test.mjs"]], vitest_fail("the glue stays in the Worker's chunk")),
    ("n6-not-entry", "a chunk that is no entry counts as the Worker's",
     [("packages/client/scripts/build-checks.mjs", "chunk.isEntry === true && ", "")],
     [CLIENT + ["scripts/build-checks.test.mjs"]], vitest_fail("the Worker's chunk is the one entry chunk the build made")),
    ("alloc-every-call", "the state hash frees and takes its buffer on every call",
     [("crates/sim/src/world.rs", "        out.clear();\n        out.reserve(STATE_BYTES);", "        out.clear();\n        out.shrink_to_fit();\n        out.reserve(STATE_BYTES);")],
     [["cargo", "test", "-p", "sim", "--locked", "--test", "state_hash_alloc"]], r"a later call took memory"),
    # --- step 2: the rule scan on the simulation crate itself, and the planted counter
    ("scan-hash", "a std hash map enters the simulation crate",
     [("crates/sim/src/world.rs", "use crate::state_hash::fnv1a_64;\n", "use crate::state_hash::fnv1a_64;\n#[allow(unused_imports)]\nuse std::collections::HashMap;\n")],
     [SCAN], r"world\.rs:6: `HashMap`"),
    ("scan-float", "a float method outside the list enters the simulation crate",
     [("crates/sim/src/world.rs", "/// The length of a match seed in bytes.", "pub fn planted(x: f32) -> f32 {\n    x.sin()\n}\n\n/// The length of a match seed in bytes.")],
     [SCAN], r"world\.rs:\d+: `sin`"),
    ("scan-clock", "a clock enters the simulation crate",
     [("crates/sim/src/world.rs", "/// The length of a match seed in bytes.", "pub fn planted() -> bool {\n    Instant::now().elapsed().is_zero()\n}\nuse std::time::Instant;\n\n/// The length of a match seed in bytes.")],
     [SCAN], r"world\.rs:\d+: `Instant`"),
    ("scan-method-form", "the tick's bytes are taken in the method form again",
     [("crates/sim/src/world.rs", "u32::to_le_bytes(*tick)", "tick.to_le_bytes()")],
     [SCAN], r"world\.rs:\d+: `to_le_bytes`"),
    ("overflow-off", "the release profile's overflow checks are turned off (by a cargo setting on the command line; no file is edited)",
     [], [["cargo", "test", "--release", "-p", "sim", "--locked", "--lib", "--config", "profile.release.overflow-checks=false"]],
     r"overflow_checks_are_on - should panic \.\.\. FAILED"),
    # --- step 4: our own ChaCha8, against the published vectors (sim) and against the vector file (devtools)
    ("rng-rotation", "the last rotation of the quarter round is by 8, not 7",
     [("crates/sim/src/rng.rs", "x[b] = (x[b] ^ x[c]).rotate_left(7);", "x[b] = (x[b] ^ x[c]).rotate_left(8);")],
     [RNG], r"TC1 differs"),
    ("rng-rounds", "ten rounds instead of eight",
     [("crates/sim/src/rng.rs", "        for _ in 0..4 {\n            quarter_round(&mut x, 0, 4, 8, 12);", "        for _ in 0..5 {\n            quarter_round(&mut x, 0, 4, 8, 12);")],
     [RNG], r"TC1 differs"),
    ("rng-stream-high", "the stream id's high half is not used",
     [("crates/sim/src/rng.rs", "            half(stream, 4),\n", "            0,\n")],
     [RNG], r"TC3 differs|TC4 differs"),
    ("rng-stream-high-file", "the same, read by the vector file's check",
     [("crates/sim/src/rng.rs", "            half(stream, 4),\n", "            0,\n")],
     [STREAM], r"stream 4294967296, word 0"),
    ("rng-carry", "the block counter's high word stays 0",
     [("crates/sim/src/rng.rs", "            half(number, 4),\n", "            0,\n")],
     [STREAM], r"across the carry at block 2\^32, word 48"),
    ("rng-pairs", "64 bits are drawn high word first",
     [("crates/sim/src/rng.rs", "        let low = u64::from(self.next_u32());\n        low | (u64::from(self.next_u32()) << 32)",
       "        let high = u64::from(self.next_u32());\n        u64::from(self.next_u32()) | (high << 32)")],
     [STREAM], r"stream 0 drawn as 64 bits, draw 0"),
    ("rng-stale-block", "a moved position still reads the block it held",
     [("crates/sim/src/rng.rs", "Some((held, block)) if held == number => block,", "Some((_, block)) => block,")],
     [STREAM], r"stream 0, word 16:"),
    ("rng-position", "set_position moves one word too far",
     [("crates/sim/src/rng.rs", "        self.position = position;\n    }", "        self.position = position + 1;\n    }")],
     [STREAM], r"stream 0 restored at word 1001, word 1001"),
    # --- step 5: the float operations against the exact reference
    ("f32-round", "round goes to the even neighbour on a half",
     [("crates/devtools/src/rules/f32_ops.rs", "f(a.round())", "f(a.round_ties_even())")], [F32], r"round 4 [0-9a-f]{16} \(the reference"),
    ("f32-mul-add", "mul_add is a product, then a sum (two roundings)",
     [("crates/devtools/src/rules/f32_ops.rs", "f(a.mul_add(b, c))", "f(a * b + c)")], [F32], r"mul_add 4 [0-9a-f]{16} \(the reference"),
    ("f32-total-cmp", "total_cmp treats the two zeros as equal",
     [("crates/devtools/src/rules/f32_ops.rs", "a.total_cmp(&b) as i8 as u8 as u64", "a.partial_cmp(&b).map_or(0, |order| order as i8 as u8 as u64)")],
     [F32], r"total_cmp 1 [0-9a-f]{16} \(the reference"),
    ("f32-fract", "fract is taken from the floor",
     [("crates/devtools/src/rules/f32_ops.rs", "f(a.fract())", "f(a - a.floor())")], [F32], r"fract 4 [0-9a-f]{16} \(the reference"),
    ("f32-cast", "a cast to i8 goes through i32 and wraps",
     [("crates/devtools/src/rules/f32_ops.rs", '("to_i8", 1, |a, _, _| a as i8 as u8 as u64)', '("to_i8", 1, |a, _, _| a as i32 as u8 as u64)')],
     [F32], r"to_i8 1 [0-9a-f]{16} \(the reference"),
    ("f32-sqrt", "sqrt is one unit in the last place off for some inputs (taken through f64 of a product)",
     [("crates/devtools/src/rules/f32_ops.rs", '("sqrt_abs", 4, |a, _, _| f(a.abs().sqrt()))', '("sqrt_abs", 4, |a, _, _| f((f64::from(a.abs()) * 1.000_000_01).sqrt() as f32))')],
     [F32], r"sqrt_abs 4 [0-9a-f]{16} \(the reference"),
    ("f32-nan", "a NaN result keeps the bits the machine gave it",
     [("crates/devtools/src/rules/f32_ops.rs", "    u64::from(if value.is_nan() {\n        0x7FC0_0000\n    } else {\n        value.to_bits()\n    })", "    u64::from(value.to_bits())")],
     [F32], r"(sub|mul|div|sqrt) 4 [0-9a-f]{16} \(the reference"),
    # --- step 6: the panic hook, its reader, and the host's guard
    ("hook-not-installed", "the module's start does not install the hook",
     [("crates/sim-wasm/src/panic.rs", '    std::panic::set_hook(Box::new(|info| record(&BUFFER, format_args!("{info}"))));\n', "")],
     [HOOK], r"the_hook_writes_a_panic_s_place_and_message_into_the_module_s_buffer \.\.\. FAILED"),
    ("hook-cut", "a text that is too long is cut inside a character",
     [("crates/sim-wasm/src/panic.rs", "        while !text.is_char_boundary(take) {\n            take -= 1;\n        }\n", "")],
     [HOOK], r"a_text_that_is_too_long_is_cut_at_a_character_s_end_and_nothing_follows_the_cut \.\.\. FAILED"),
    ("hook-goes-on", "after a piece did not fit, a later and shorter piece is still taken",
     [("crates/sim-wasm/src/panic.rs", "let mut take = if self.full { 0 } else { room.min(text.len()) };", "let mut take = room.min(text.len());")],
     [HOOK], r"a_text_that_is_too_long_is_cut_at_a_character_s_end_and_nothing_follows_the_cut \.\.\. FAILED"),
    ("hook-room", "the start does not write the buffer's room (seen on the built Wasm)",
     [("crates/sim-wasm/src/panic.rs", "    store_word(&BUFFER[4..HEADER_BYTES], BUFFER_BYTES - HEADER_BYTES);\n    std::panic", "    std::panic")],
     [WASM_ONLY, BOUNDARY], vitest_fail("the panic hook's buffer stands from the module's start")),
    ("guard-resume", "the resume after a hidden tab calls the Wasm outside the guard",
     [("packages/sim-worker/src/host.ts", "this.#call(this.#resume) !== undefined", "this.#resume() !== undefined")],
     [WORKER + ["host.test.ts"]], vitest_fail("a trap on the resume after a hidden tab is caught as one in a tick is")),
    ("guard-again", "a trapped instance is called again",
     [("packages/sim-worker/src/host.ts", "    if (this.#failed) return undefined;\n    try {", "    try {")],
     [WORKER + ["host.test.ts"]], vitest_fail("a trap's own message reaches the page, and the Wasm is never called again")),
    ("guard-no-text", "the host does not read the hook's text",
     [("packages/sim-worker/src/host.ts", "message: trapMessage(this.#env.memory, this.#env.panicBuffer, error)", "message: error instanceof Error ? error.message : String(error)")],
     [WORKER + ["host.test.ts"]], vitest_fail("a trap's own message reaches the page, and the Wasm is never called again")),
    ("reader-bounds", "the reader trusts a length that is above the room or ends outside the memory",
     [("packages/sim-worker/src/panic-message.ts", 'if (length === 0 || room !== PANIC_TEXT_ROOM || length > room || ptr + 8 + length > bytes) return "";', 'if (length === 0 || room !== PANIC_TEXT_ROOM) return "";')],
     [WORKER + ["panic-message.test.ts"]], vitest_fail("a buffer that makes no sense reads as no text and throws nothing")),
    ("reader-room", "the reader takes the whole room, not the text's length",
     [("packages/sim-worker/src/panic-message.ts", "new Uint8Array(memory.buffer, ptr + 8, length)", "new Uint8Array(memory.buffer, ptr + 8, Math.min(room, bytes - ptr - 8))")],
     [WORKER + ["panic-message.test.ts"]], vitest_fail("the text the hook left is read whole")),
    # --- step 7: the test-only module (each plant builds both Wasm modules, then runs the boundary folder)
    ("wasm-foreign", "the build no longer tells a foreign WebAssembly file from the players' own",
     [("packages/client/scripts/build-checks.mjs", "const others = modules.filter(({ bytes }) => !Buffer.from(bytes).equals(Buffer.from(playersWasm)));", "const others = modules.filter(() => false);")],
     [CLIENT + ["scripts/build-checks.test.mjs"]], vitest_fail("a built page holds the players' WebAssembly and no other")),
    ("module-wraps", "the planted counter wraps instead of overflowing",
     [("crates/devtools/src/wasm.rs", "    black_box(counter) + black_box(step)\n", "    black_box(counter).wrapping_add(black_box(step))\n")],
     [WASM_ONLY, BOUNDARY], r"FAIL.*release profile > a counter that overflows traps"),
    ("module-release-unchecked", "the release profile's overflow checks are off (the root manifest edited for this run only, restored after it)",
     [("Cargo.toml", "[profile.release]\noverflow-checks = true", "[profile.release]\noverflow-checks = false")],
     [WASM_ONLY, BOUNDARY], r"FAIL.*release profile > a counter that overflows traps"),
    ("module-no-hook", "the module does not link the players' panic hook",
     [("crates/devtools/src/wasm.rs", "use sim_wasm as _;\n", "")],
     [WASM_ONLY, BOUNDARY], r"FAIL.*release profile > a counter that overflows traps"),
    ("module-vectors-blind", "the module's float check answers \"all equal\" without looking",
     [("crates/devtools/src/wasm.rs", "    f32_ops::check(inputs, expected).err().unwrap_or_default()\n", "    let _ = (inputs, expected, f32_ops::CASES);\n    String::new()\n")],
     [WASM_ONLY, BOUNDARY], r"FAIL.*release profile > every allowed float operation gives the exact reference's results"),
    ("players-clock", "the players' Wasm imports a clock",
     [("crates/sim-wasm/src/lib.rs", "use wasm_bindgen::prelude::*;\n", "use wasm_bindgen::prelude::*;\n\n#[wasm_bindgen]\nextern \"C\" {\n    #[wasm_bindgen(js_namespace = Date, js_name = now)]\n    fn planted_now() -> f64;\n}\n"),
      ("crates/sim-wasm/src/lib.rs", "        self.schedule.ticked();\n", "        self.schedule.ticked();\n        let _ = planted_now();\n")],
     [WASM_ONLY, BOUNDARY], r"FAIL.*the Wasm players get imports nothing but its own glue"),
    # --- step 8: the build's two refusals, the path remap, the wider refusal of settings
    ("override-removed", "a third-party crate of the players' Wasm loses its overflow override (the root manifest edited for this run only)",
     [("Cargo.toml", "[profile.release.package.cfg-if]\noverflow-checks = true\n\n", "")],
     [WASM_ONLY], r"build: the players' Wasm would hold these crates with overflow checks off: cfg-if"),
    ("override-blind", "the override check finds nothing missing, whatever the tree holds",
     [("packages/client/scripts/build-checks.mjs", "return runtimeCrates(treeText, root).filter((name) => !overridden.has(name));", "return runtimeCrates(treeText, root).filter((name) => overridden.has(name) && false);")],
     [CLIENT + ["scripts/build-checks.test.mjs"]], vitest_fail("a crate that joins the tree, an override that is removed or turned off")),
    ("features-differ", "the test-only module turns on a feature of the simulation that the players' Wasm does not (two manifests edited for this run only)",
     [("crates/sim/Cargo.toml", "[lints]\nworkspace = true\n", "[features]\nplanted = []\n\n[lints]\nworkspace = true\n"),
      ("crates/devtools/Cargo.toml", 'sim = { path = "../sim" }', 'sim = { path = "../sim", features = ["planted"] }')],
     [WASM_ONLY], r"build: the simulation's features differ between the players' Wasm and the test-only module"),
    ("features-blind", "the feature compare reports no difference, whatever the lists hold",
     [("packages/client/scripts/build-checks.mjs", ".filter(([, list]) => JSON.stringify(list) !== JSON.stringify(first))", ".filter(() => false)")],
     [CLIENT + ["scripts/build-checks.test.mjs"]], vitest_fail("builds that differ outside the simulation, or in a feature of the simulation itself")),
    ("remap-off", "the players' Wasm is built without the path remap",
     [("packages/client/scripts/build.mjs", 'const cargoEnv = () => ({ ...process.env, CARGO_ENCODED_RUSTFLAGS: registryRemapFlag(process.env["CARGO_HOME"], os.homedir(), root) });', "const cargoEnv = () => ({ ...process.env });")],
     [WASM_ONLY, BOUNDARY], r"FAIL.*no Wasm names the cargo folder of the machine that built it"),
    ("channel", "the names through which cargo takes nightly-only features are let through",
     [("packages/client/scripts/build-checks.mjs", '"__CARGO_.*|CARGO_(BUILD|PROFILE|RESOLVER|TARGET|UNSTABLE)_.*', '"CARGO_(BUILD|PROFILE|RESOLVER|TARGET)_.*')],
     [CLIENT + ["scripts/build-checks.test.mjs"]], vitest_fail("the Wasm build refuses cargo's and the tool's settings from the environment")),
    # --- round 1's fix pass: the wider scan, mul_add held back and pinned, the build checks on real cargo output
    ("scan-alias", "a banned float method enters the simulation under an alias of f32",
     [("crates/sim/src/world.rs", "/// The length of a match seed in bytes.", "type Real = f32;\npub fn planted(x: Real) -> Real {\n    Real::sin(x)\n}\n\n/// The length of a match seed in bytes.")],
     [SCAN], r"world\.rs:\d+: `sin`"),
    ("scan-held", "mul_add enters the simulation",
     [("crates/sim/src/world.rs", "/// The length of a match seed in bytes.", "pub fn planted(x: f32) -> f32 {\n    x.mul_add(2.0, 1.0)\n}\n\n/// The length of a match seed in bytes.")],
     [SCAN], r"world\.rs:\d+: `mul_add` is on ADR 0004 part 7's list and held back"),
    ("scan-hash-name", "a hashed map under another crate's name enters the simulation",
     [("crates/sim/src/world.rs", "/// The length of a match seed in bytes.", "#[allow(dead_code)]\ntype FxHashMap = [u32; 4];\n\n/// The length of a match seed in bytes.")],
     [SCAN], r"world\.rs:\d+: `FxHashMap`"),
    ("scan-target-feature", "code for one processor feature enters the simulation",
     [("crates/sim/src/world.rs", "/// The length of a match seed in bytes.", "#[cfg(target_feature = \"simd128\")]\npub fn planted() {}\n\n/// The length of a match seed in bytes.")],
     [SCAN], r"world\.rs:\d+: `target_feature`"),
    ("scan-escape-fn", "part 9's escape is put on a whole function",
     [("crates/sim/src/world.rs", "/// The length of a match seed in bytes.", "#[allow(clippy::disallowed_types, reason = \"planted\")]\npub fn planted() -> usize {\n    std::collections::HashSet::<u32>::new().len()\n}\n\n/// The length of a match seed in bytes.")],
     [SCAN], r"world\.rs:\d+: `HashSet`"),
    ("scan-path-narrow", "the scan counts a banned name only after f32 or f64 again",
     [("crates/sim/tests/rules_scan.rs", 'let on_a_path = before == "::" && !NOT_FLOATS.contains(&before_2);', 'let on_a_path = before == "::" && matches!(before_2, "f32" | "f64");')],
     [SCAN], r"each_planted_breach_is_found_once_and_by_name \.\.\. FAILED"),
    ("scan-held-blind", "the scan lets mul_add through",
     [("crates/sim/tests/rules_scan.rs", "HELD.iter().find(|(held, _)| float_call && *held == name)", "HELD.iter().find(|(held, _)| float_call && held.is_empty() && *held == name)")],
     [SCAN], r"each_planted_breach_is_found_once_and_by_name \.\.\. FAILED"),
    ("f32-mul-add-f64", "mul_add is worked out through f64 (two roundings), natively",
     [("crates/devtools/src/rules/f32_ops.rs", "f(a.mul_add(b, c))", "f((f64::from(a) * f64::from(b) + f64::from(c)) as f32)")], [F32], r"mul_add 4 [0-9a-f]{16} \(the reference"),
    ("f32-mul-add-f64-wasm", "the same, in WebAssembly (the pinned cases agree with it; the other constructed cases do not)",
     [("crates/devtools/src/rules/f32_ops.rs", "f(a.mul_add(b, c))", "f((f64::from(a) * f64::from(b) + f64::from(c)) as f32)")],
     [WASM_ONLY, BOUNDARY], r"FAIL.*release profile > every allowed float operation gives the exact reference's results"),
    ("f32-from-f64", "the cast from f64 is fed an f32 product again",
     [("crates/devtools/src/rules/f32_ops.rs", "f(f64::from_bits(wide(a, b)) as f32)", "f((f64::from(a) * f64::from(b)) as f32)")], [F32], r"from_f64 4 [0-9a-f]{16} \(the reference"),
    ("pins-ignored-wasm", "the WebAssembly check counts no pin: mul_add there must then differ from the reference",
     [("crates/devtools/src/rules/f32_ops.rs", 'check_as(inputs, expected, cfg!(target_arch = "wasm32"))', "check_as(inputs, expected, false)")],
     [WASM_ONLY, BOUNDARY], r"FAIL.*release profile > every allowed float operation gives the exact reference's results"),
    ("pins-not-compared", "a pinned case is no longer compared with its measured value",
     [("crates/devtools/src/rules/f32_ops.rs", "if bits != u64::from(pin.measured) {", "if bits != u64::from(pin.measured) && case > CASES {")],
     [PINS], r"a_pinned_case_that_no_longer_gives_its_measured_value_fails_by_name \.\.\. FAILED"),
    ("rng-pairs-odd", "64 bits drawn from a block's last word repeat that word as their high half",
     [("crates/sim/src/rng.rs", "        low | (u64::from(self.next_u32()) << 32)\n", "        let across = self.position % BLOCK_WORDS == 0;\n        let high = u64::from(self.next_u32());\n        low | ((if across { low } else { high }) << 32)\n")],
     [RNG], r"sixty_four_bits_are_two_words_the_first_one_low \.\.\. FAILED"),
    ("hook-start-exported", "the hook's start is an export of the glue again",
     [("crates/sim-wasm/src/panic.rs", "#[wasm_bindgen(start, private)]", "#[wasm_bindgen(start)]")],
     [WASM_ONLY, BOUNDARY], vitest_fail("the glue exports what the Worker calls and nothing more")),
    ("room-drift", "the page's number for the hook's room is not the Wasm's",
     [("packages/sim-worker/src/panic-message.ts", "export const PANIC_TEXT_ROOM = 1016;", "export const PANIC_TEXT_ROOM = 1000;")],
     [BOUNDARY], vitest_fail("the panic hook's buffer stands from the module's start")),
    ("reader-other-room", "the reader takes a buffer whose room is not the hook's",
     [("packages/sim-worker/src/panic-message.ts", "length === 0 || room !== PANIC_TEXT_ROOM || length > room", "length === 0 || length > room")],
     [WORKER + ["panic-message.test.ts"]], vitest_fail("a header whose room is not the hook's is not read at all")),
    ("trap-sentence", "the Worker's sentence after a throw leaves the panic's text out",
     [("packages/sim-worker/src/panic-message.ts", '  if (panic !== "") return `the simulation stopped: ${panic}`;\n', "")],
     [WORKER + ["panic-message.test.ts"]], vitest_fail("the Worker's sentence after a throw")),
    ("foreign-source", "a crate from git, another registry or a folder elsewhere counts as ours again",
     [("packages/client/scripts/build-checks.mjs", 'if (!(entry[3] ?? "").replaceAll("\\\\", "/").startsWith(ours)) names.add(entry[1]);', "if (entry[3] === undefined) names.add(entry[1]);")],
     [CLIENT + ["scripts/build-checks.test.mjs"]], vitest_fail("only a folder under the repository's crates/ is ours")),
    ("features-own-lines", "the feature list holds only the lines cargo prints under the simulation itself",
     [("packages/client/scripts/build-checks.mjs", "if (closure.has(name) && (text.includes(' feature \"') || name !== \"sim\"))", "if (name === \"sim\" && text.includes(' feature \"'))")],
     [CLIENT + ["scripts/build-checks.test.mjs"]], vitest_fail("a feature another package turns on for a dependency of the simulation is found and named")),
    ("features-dev-dep", "a dev-dependency turns on a feature of the simulation for the native tests only (two manifests edited for this run only)",
     [("crates/sim/Cargo.toml", "[lints]\nworkspace = true\n", "[features]\nplanted = []\n\n[lints]\nworkspace = true\n"),
      ("crates/sim-wasm/Cargo.toml", "[lints]\nworkspace = true\n", "[dev-dependencies]\nsim = { path = \"../sim\", features = [\"planted\"] }\n\n[lints]\nworkspace = true\n")],
     [WASM_ONLY], r"build: the simulation's features differ between the players' Wasm and the native tests"),
    ("tree-any-root", "the overflow check reads a tree that is not the players' Wasm's",
     [("packages/client/scripts/build-checks.mjs", "if (!(lines[0] ?? \"\").startsWith(`${rootCrate} v`)) throw new Error(`cargo tree did not start at ${rootCrate}: the overflow check has nothing to read`);\n", "")],
     [CLIENT + ["scripts/build-checks.test.mjs"]], vitest_fail("a line the check cannot place, or a tree that is not the players' Wasm's, stops it")),
    ("remap-empty-home", "an empty CARGO_HOME is taken as a folder, so the remap names nothing",
     [("packages/client/scripts/build-checks.mjs", 'cargoHome === undefined || cargoHome === ""', "cargoHome === undefined")],
     [CLIENT + ["scripts/build-checks.test.mjs"]], vitest_fail("the path remap names the registry folder under cargo's home")),
    ("env-lower-case", "a refused setting written in lower case gets through",
     [("packages/client/scripts/build-checks.mjs", "CARGO_OVERRIDES.test(name.toUpperCase())", "CARGO_OVERRIDES.test(name)")],
     [CLIENT + ["scripts/build-checks.test.mjs"]], vitest_fail("the Wasm build refuses cargo's and the tool's settings from the environment")),
    ("devserver-open", "the dev server hands out the whole repository, the test-only module's folder included",
     [("packages/client/vite.config.mjs", 'folder("../sim-worker/src"), folder("../sim-worker/pkg"), ', 'folder("../.."), ')],
     [BUILD, E2E + ["-g", "with the flag on under Vite's dev server"]], frame('expect((await page.request.get(at(".gate/rules-module/release/devtools_bg.wasm"))).status()).toBe(403);')),
    ("devserver-link", "the dev server reads node_modules as a whole again, and with it npm's link to the Worker's package",
     [("packages/client/vite.config.mjs", 'folder("../../node_modules/three")', 'folder("../../node_modules")')],
     [BUILD, E2E + ["-g", "with the flag on under Vite's dev server"]], frame('expect((await page.request.get(at("node_modules/@shamble/sim-worker/package.json"))).status()).toBe(403);')),
    # --- round 2's fix pass
    ("scan-tuple-field", "a banned float method is called on a tuple's field",
     [("crates/sim/src/world.rs", "/// The length of a match seed in bytes.", "pub fn planted(p: (f32, f32)) -> f32 {\n    p.0.sin()\n}\n\n/// The length of a match seed in bytes.")],
     [SCAN], r"world\.rs:\d+: `sin`"),
    ("scan-range-dots", "the scan reads a range's two dots as two single ones again",
     [("crates/sim/tests/rules_scan.rs", "let pair = (c == ':' || c == '.') && next == c;", "let pair = c == ':' && next == c;")],
     [SCAN], r"what_the_rules_allow_passes \.\.\. FAILED"),
    ("scan-arch-nested", "per-target code is brought in through a nested use list",
     [("crates/sim/src/world.rs", "/// The length of a match seed in bytes.", "#[cfg(target_arch = \"wasm32\")]\npub use core::{arch::wasm32::v128 as PlantedLane};\n\n/// The length of a match seed in bytes.")],
     [SCAN], r"world\.rs:\d+: `arch`"),
    ("remap-control-char", "a cargo home with a control character in its path is put into the flags variable",
     [("packages/client/scripts/build-checks.mjs", "  if (/[\\u0000-\\u001f\\u007f]/.test(home)) throw new Error(\"cargo's home folder has a control character in its path: the build cannot name it in a compiler flag\");\n", "")],
     [CLIENT + ["scripts/build-checks.test.mjs"]], vitest_fail("the path remap names the registry folder under cargo's home")),
    ("remap-relative-cwd", "a relative cargo home is resolved against node's own folder, not the one cargo runs in",
     [("packages/client/scripts/build-checks.mjs", "const home = path.resolve(base, cargoHome", "const home = path.resolve(cargoHome")],
     [CLIENT + ["scripts/build-checks.test.mjs"]], vitest_fail("the path remap names the registry folder under cargo's home")),
    # --- in the browser: a build of the planted tree, then one spec
    ("nt5-stale", "the Worker runs on while hidden and the page asks for no report while hidden",
     [("packages/sim-worker/src/host.ts", "        if (data.hidden) {\n          this.#pacer.pause();\n          this.#clearTimer();\n        } else if",
       "        if (data.hidden) {\n          // planted: nothing\n        } else if"),
      ("packages/client/src/app/boot.ts", "      link.requestReport();\n", "      if (!document.hidden) link.requestReport();\n")],
     [BUILD, E2E + ["-g", "a hidden tab pauses the match"]], frame("expect(stillHidden.tick).toBe(atRead.tick);")),
    ("te-104", "a resume runs 90 ticks at once and counts no drop",
     [("packages/sim-worker/src/pacing.ts", "      this.#pauses += 1;\n      this.#nextDueMs = this.#sim.rebase(now);",
       "      this.#pauses += 1;\n      for (let i = 0; i < 90; i += 1) {\n        this.#sim.step();\n        this.#ticks += 1;\n      }\n      this.#nextDueMs = this.#sim.rebase(now);")],
     [BUILD, E2E + ["-g", "a hidden tab pauses the match"]], frame("- hidden).toBeLessThanOrEqual(most);")),
    ("two-workers", "the page starts a second Worker beside the first",
     [("packages/client/src/app/boot.ts", "  const worker = new SimWorker();\n", "  const worker = new SimWorker();\n  new SimWorker();\n")],
     [BUILD, E2E + ["-g", "the first page on the default backend"]], frame("expect(page.workers()).toHaveLength(1);")),
]


def run(command):
    env = dict(__import__("os").environ)
    env["SHAMBLE_WASM_BINDGEN"] = WB
    env["SHAMBLE_DIST"] = str(ROOT / ".gate/dist-plant")
    env["SHAMBLE_CHROMIUM"] = "/opt/pw-browsers/chromium"
    done = subprocess.run(command, cwd=ROOT, env=env, capture_output=True, text=True, timeout=600)
    return done.returncode, done.stdout + done.stderr


def main():
    wanted = set(sys.argv[1:])
    OUT.mkdir(parents=True, exist_ok=True)
    rows = []
    for plant_id, what, edits, commands, must in PLANTS:
        if wanted and plant_id not in wanted:
            continue
        originals = {}
        for name, old, new in edits:
            text = originals.get(name) or (ROOT / name).read_text()
            originals.setdefault(name, text)
        try:
            current = dict(originals)
            for name, old, new in edits:
                assert current[name].count(old) == 1, f"{plant_id}: the text to replace is not there exactly once in {name}"
                current[name] = current[name].replace(old, new)
                (ROOT / name).write_text(current[name])
            output, code = "", 0
            for command in commands:
                code, text = run(command)
                output += f"$ {' '.join(command)}\n{text}\n"
                if code != 0:
                    break
        finally:
            for name, text in originals.items():
                (ROOT / name).write_text(text)
        (OUT / f"plant-{plant_id}.out").write_text(output)
        hit = [line for line in output.splitlines() if re.search(must, line)]
        caught = code != 0 and bool(hit)
        rows.append((plant_id, what, "CAUGHT" if caught else "NOT CAUGHT", (hit[0].strip() if hit else f"exit {code}, no line matches")[:150]))
        print(f"{rows[-1][2]:10} {plant_id:18} {rows[-1][3]}", flush=True)
    stamp = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
    print(f"{stamp}: {sum(1 for row in rows if row[2] == 'CAUGHT')} of {len(rows)} caught")
    return 0 if all(row[2] == "CAUGHT" for row in rows) else 1


if __name__ == "__main__":
    sys.exit(main())
