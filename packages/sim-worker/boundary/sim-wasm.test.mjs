// The boundary tests: the built Wasm and its glue, loaded in Node (ADR 0002 part 8). They need the build
// (`node packages/client/scripts/build.mjs --wasm-only`, which makes the players' Wasm and, for rules.test.mjs
// beside this file, the test-only module; a `--production` build makes the players' Wasm alone), so the package's
// `test` script leaves this folder out and CI runs it after its builds. Plain JavaScript on purpose: they read the
// Wasm with Node's own module, and Node's types are not a dependency, so a TypeScript file here would never be
// type-checked.
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { BLOCK_BYTES, BLOCK_LAYOUT, checkLayoutHash, LAYOUT_HASH } from "@shamble/types";
import { beforeAll, expect, test } from "vitest";
import * as glue from "../pkg/sim_wasm.js";
import { initSync, panic_buffer, Sim, tick_hz } from "../pkg/sim_wasm.js";
import { SimHost } from "../src/host.ts";
import { MAX_TICKS_PER_WAKE } from "../src/pacing.ts";
import { PANIC_TEXT_ROOM, readPanicMessage } from "../src/panic-message.ts";
import { copyBlock } from "../src/read-block.ts";

const SEED = new Uint8Array(32).fill(7);
const PERIOD = 1000 / 30;
let wasm;

beforeAll(() => {
  wasm = initSync({ module: readFileSync(new URL("../pkg/sim_wasm_bg.wasm", import.meta.url)) });
});

/** The block as the host copies it: through a fresh view of the memory. */
function blockOf(sim) {
  const into = new ArrayBuffer(sim.block_len());
  copyBlock(wasm.memory, sim.block_ptr(), sim.block_len(), into);
  return into;
}
const tickOf = (buffer) => new DataView(buffer).getUint32(0, true);
/** One section of a block as its words, by the generated layout's own offsets. */
function wordsOf(buffer, section) {
  const { offset, fields } = BLOCK_LAYOUT[section];
  const view = new DataView(buffer);
  return Object.fromEntries(Object.entries(fields).map(([name, field]) => [name, view.getUint32(offset + field.offset, true)]));
}

test("a block written by Rust passes the page's layout check, from tick 0 on", () => {
  const sim = new Sim(SEED, 0, 0);
  expect(tick_hz()).toBe(30);
  // The host's cap is a quarter of a second of the simulation's own tick rate (ADR 0004 part 5).
  expect(Math.floor(0.25 * tick_hz())).toBe(MAX_TICKS_PER_WAKE);
  // The generated layout and the Rust writer agree on the block's length: the last section's end.
  expect(sim.block_len()).toBe(44);
  expect([BLOCK_BYTES, BLOCK_LAYOUT.link.offset + BLOCK_LAYOUT.link.stride]).toEqual([44, 44]);
  expect(checkLayoutHash(blockOf(sim))).toEqual({ ok: true });
  expect(tickOf(blockOf(sim))).toBe(0);
  sim.step(0);
  expect(checkLayoutHash(blockOf(sim))).toEqual({ ok: true });
  expect(new DataView(blockOf(sim)).getUint32(4, true)).toBe(LAYOUT_HASH);
});

test("two steps advance the tick by two, and each returns the next due time", () => {
  const sim = new Sim(SEED, 0, 0);
  // The host's first rebase says when the first tick is due.
  expect(sim.rebase(1000)).toBeCloseTo(1000 + PERIOD, 9);
  expect(sim.step(1000 + PERIOD)).toBeCloseTo(1000 + 2 * PERIOD, 9);
  expect(sim.step(1000 + 2 * PERIOD)).toBeCloseTo(1000 + 3 * PERIOD, 9);
  expect(tickOf(blockOf(sim))).toBe(2);
  expect(sim.rebase(60_000)).toBeCloseTo(60_000 + PERIOD, 9);
});

test("what the host hands in comes back in the block: each button's count in its own byte, through the real messages", () => {
  const sim = new Sim(SEED, 0, 0);
  sim.step(0);
  const counts = new Uint8Array(16);
  for (let button = 0; button < 16; button += 1) {
    // One press of this button on top of the ones before it, with only its own bit held (and bits above the
    // sixteenth, which are ignored).
    counts[button] = 1;
    const [p0, p1, p2, p3] = new Uint32Array(counts.buffer);
    sim.input(button + 1, (0xffff_0000 | (1 << button)) >>> 0, p0, p1, p2, p3);
    sim.step(0);
    const block = blockOf(sim);
    expect(wordsOf(block, "echo"), `button ${button}`).toEqual({
      held: 1 << button,
      presses_0_3: p0,
      presses_4_7: p1,
      presses_8_11: p2,
      presses_12_15: p3,
    });
    // Sent and received climb with the ticks, nothing is refused, and this tick's command was built from this post.
    expect(wordsOf(block, "link")).toEqual({ sent: button + 1, received: button + 2, refused: 0, post_used: button + 1 });
  }
  // Any numbers are taken: nothing in `input` can trap on what a page sent. The last post handed in before a
  // step counts, whatever its number: which post is the newest is the host's to say (host.ts), not the Wasm's.
  sim.input(17, 0, 0, 0, 0, 0);
  sim.input(0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff);
  sim.step(0);
  expect(wordsOf(blockOf(sim), "echo").held).toBe(0xffff);
  expect(wordsOf(blockOf(sim), "link")).toMatchObject({ post_used: 0xffff_ffff, refused: 0 });
});

test("after the memory grows between two steps, the next block is the block of a run without the growth", () => {
  const grown = new Sim(SEED, 0, 1024);
  const plain = new Sim(SEED, 0, 1024);
  grown.step(0);
  plain.step(0);
  // One copy before the growth, so a host that keeps its first view is caught here whatever ran before.
  expect(tickOf(blockOf(grown))).toBe(1);
  const kept = new Uint8Array(wasm.memory.buffer, grown.block_ptr(), grown.block_len());
  wasm.memory.grow(1);
  grown.step(0);
  plain.step(0);
  // A view kept from before the growth reads nothing: the host must make a fresh one on every copy.
  expect(kept.byteLength).toBe(0);
  expect(new Uint8Array(blockOf(grown))).toEqual(new Uint8Array(blockOf(plain)));
  expect(tickOf(blockOf(grown))).toBe(2);
});

test("padding makes the block longer and it still passes the check", () => {
  const sim = new Sim(SEED, 0, 65536);
  expect(sim.block_len()).toBe(44 + 65536);
  sim.step(0);
  expect(checkLayoutHash(blockOf(sim))).toEqual({ ok: true });
});

test("a bad seed, mode or padding is refused with a plain reason", () => {
  expect(() => new Sim(new Uint8Array(31), 0, 0)).toThrow("the match seed must be 32 bytes and is 31");
  expect(() => new Sim(SEED, 1, 0)).toThrow("only local play (0)");
  // The whole number counts: 256 must not be cut down to 0, which is local play.
  expect(() => new Sim(SEED, 256, 0)).toThrow("mode 256 does not exist");
  expect(() => new Sim(SEED, 0, 0xffff_ffff)).toThrow("above the writer's cap");
});

test("the panic hook's buffer stands from the module's start: no text, and its room said", () => {
  const ptr = panic_buffer();
  const header = new DataView(wasm.memory.buffer, ptr, 8);
  expect(header.getUint32(0, true)).toBe(0);
  // The room is written by the module's start function, which the glue runs as it instantiates the module: so
  // the hook is installed before any other export can run. (A trap's text itself: rules.test.mjs.)
  expect(header.getUint32(4, true)).toBe(1016);
  // The one number the Worker's reader holds a copy of.
  expect(PANIC_TEXT_ROOM).toBe(1016);
  expect(readPanicMessage(wasm.memory, ptr)).toBe("");
});

test("the players' Wasm is the file the built-file check walked, and it asks the host for the bindings' three calls only", () => {
  const bytes = readFileSync(new URL("../pkg/sim_wasm_bg.wasm", import.meta.url));
  // The stamp the build leaves after a clean walk of this file (build-checks.mjs, walkedStamp).
  const stamp = JSON.parse(readFileSync(new URL("../pkg/built-file-check.json", import.meta.url), "utf8"));
  expect(stamp.production).toBe(createHash("sha256").update(bytes).digest("hex"));
  // Node's own reading of the imports, beside the build's reader: an error's text, a throw, the table's set-up. No
  // clock, no randomness, no maths function of the host.
  const imports = WebAssembly.Module.imports(new WebAssembly.Module(bytes)).map(({ module, name, kind }) => `${kind} ${module} ${name}`);
  expect(imports.sort()).toEqual([
    "function ./sim_wasm_bg.js __wbg_Error_208446807d0d1e30",
    "function ./sim_wasm_bg.js __wbg___wbindgen_throw_4c88d8bf264fd984",
    "function ./sim_wasm_bg.js __wbindgen_init_externref_table",
  ]);
});

test("the glue exports what the Worker calls and nothing more", () => {
  // The two ways to load the module, the one type, and two calls asked once. The start function that installs the
  // panic hook is not among them: the glue runs it itself.
  expect(Object.keys(glue).sort()).toEqual(["Sim", "default", "initSync", "panic_buffer", "tick_hz"]);
  // The one type's own calls: a method added to the bindings shows here. `__destroy_into_raw` and `free` are
  // wasm-bindgen's own; only its dispose symbol is left out.
  const calls = Reflect.ownKeys(Sim.prototype).filter((name) => typeof name === "string");
  // `shared_hash` is the one read-only number a replay of a recorded match compares (his DEPART-3); no live tick
  // calls it (host.test.ts pins a live tick's calls).
  expect(calls.sort()).toEqual(["__destroy_into_raw", "block_len", "block_ptr", "constructor", "free", "input", "rebase", "shared_hash", "step"]);
  expect(Object.getOwnPropertyNames(Sim).filter((name) => !["length", "name", "prototype"].includes(name))).toEqual([]);
});

test("in the glue, each per-tick call reaches exactly one export", () => {
  const glue = readFileSync(new URL("../pkg/sim_wasm.js", import.meta.url), "utf8");
  // The replay's one number too: no argument, one export, so its shape is pinned and not only its name.
  for (const [method, exported] of [["input(post, held, p0, p1, p2, p3)", "sim_input"], ["step(now_ms)", "sim_step"], ["shared_hash()", "sim_shared_hash"]]) {
    const from = glue.indexOf(`    ${method} {`);
    expect(from, method).toBeGreaterThan(-1);
    const body = glue.slice(from, glue.indexOf("\n    }", from));
    expect(body.match(/wasm\.\w+/g), method).toEqual([`wasm.${exported}`]);
  }
});

test("the host on the real Wasm: two calls per tick, one transferred block per tick, each passing the check", () => {
  const sim = new Sim(SEED, 0, 0);
  const calls = { input: 0, step: 0, rebase: 0, block_ptr: 0, block_len: 0 };
  const counted = Object.fromEntries(
    ["input", "step", "rebase", "block_ptr", "block_len"].map((name) => [
      name,
      (...args) => {
        if (name in calls) calls[name] += 1;
        return sim[name](...args);
      },
    ]),
  );
  const clock = { now: 1000 };
  const posts = [];
  let timer;
  const host = new SimHost(
    counted,
    {
      memory: wasm.memory,
      panicBuffer: panic_buffer(),
      now: () => clock.now,
      timeOrigin: 0,
      post: (message, transfer) => posts.push(structuredClone(message, { transfer })),
      setTimer: (run, delayMs) => ((timer = { run, at: clock.now + delayMs }), 1),
      clearTimer: () => (timer = undefined),
      hardwareConcurrency: 1,
      crossOriginIsolated: false,
    },
    tick_hz(),
  );
  host.start();
  host.onMessage({ type: "input", post: 9, held: 0x21, p0: 1, p1: 0, p2: 0, p3: 0, returned: [] });
  for (let i = 0; i < 30; i += 1) {
    // Half-way the memory grows: the host itself must copy through a fresh view from then on (PM-0002-6).
    if (i === 15) wasm.memory.grow(1);
    clock.now = timer.at;
    timer.run();
  }
  expect(posts.filter((post) => post.type === "error")).toEqual([]);
  const blocks = posts.filter((post) => post.type === "block");
  expect(blocks.map((block) => tickOf(block.buffer))).toEqual(Array.from({ length: 30 }, (_, i) => i + 1));
  for (const block of blocks) expect(checkLayoutHash(block.buffer)).toEqual({ ok: true });
  expect(calls).toEqual({ input: 30, step: 30, rebase: 1, block_ptr: 1, block_len: 1 });
  // The post reached the local server through the real messages and came back: held, pressed once, and used.
  const last = blocks.at(-1).buffer;
  expect(wordsOf(last, "echo")).toMatchObject({ held: 0x21, presses_0_3: 1 });
  expect(wordsOf(last, "link")).toEqual({ sent: 29, received: 30, refused: 0, post_used: 9 });
  expect(host.stats()).toMatchObject({ ticks: 30, undetached: 0, maxTicksInOneWake: 1, drops: 0 });
});
