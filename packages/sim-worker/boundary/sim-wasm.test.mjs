// The boundary tests: the built Wasm and its glue, loaded in Node (ADR 0002 part 8). They need the build
// (`node packages/client/scripts/build.mjs --wasm-only`), so the package's `test` script, which CI runs, leaves this
// folder out. Plain JavaScript on purpose: they read the Wasm with Node's own module, and Node's types are not a
// dependency, so a TypeScript file here would never be type-checked.
import { readFileSync } from "node:fs";
import { checkLayoutHash, LAYOUT_HASH } from "@shamble/types";
import { beforeAll, expect, test } from "vitest";
import { initSync, panic_buffer, Sim, tick_hz } from "../pkg/sim_wasm.js";
import { SimHost } from "../src/host.ts";
import { MAX_TICKS_PER_WAKE } from "../src/pacing.ts";
import { readPanicMessage } from "../src/panic-message.ts";
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

test("a block written by Rust passes the page's layout check, from tick 0 on", () => {
  const sim = new Sim(SEED, 0, 0, 0);
  expect(tick_hz()).toBe(30);
  // The host's cap is a quarter of a second of the simulation's own tick rate (ADR 0004 part 5).
  expect(Math.floor(0.25 * tick_hz())).toBe(MAX_TICKS_PER_WAKE);
  expect(sim.block_len()).toBe(8);
  expect(checkLayoutHash(blockOf(sim))).toEqual({ ok: true });
  expect(tickOf(blockOf(sim))).toBe(0);
  sim.step();
  expect(checkLayoutHash(blockOf(sim))).toEqual({ ok: true });
  expect(new DataView(blockOf(sim)).getUint32(4, true)).toBe(LAYOUT_HASH);
});

test("two steps advance the tick by two, and each returns the next due time", () => {
  const sim = new Sim(SEED, 0, 0, 1000);
  expect(sim.step()).toBeCloseTo(1000 + 2 * PERIOD, 9);
  expect(sim.step()).toBeCloseTo(1000 + 3 * PERIOD, 9);
  expect(tickOf(blockOf(sim))).toBe(2);
  expect(sim.rebase(60_000)).toBeCloseTo(60_000 + PERIOD, 9);
  sim.input(42);
  expect(sim.newest_input()).toBe(42);
});

test("after the memory grows between two steps, the next block is the block of a run without the growth", () => {
  const grown = new Sim(SEED, 0, 1024, 0);
  const plain = new Sim(SEED, 0, 1024, 0);
  grown.step();
  plain.step();
  // One copy before the growth, so a host that keeps its first view is caught here whatever ran before.
  expect(tickOf(blockOf(grown))).toBe(1);
  const kept = new Uint8Array(wasm.memory.buffer, grown.block_ptr(), grown.block_len());
  wasm.memory.grow(1);
  grown.step();
  plain.step();
  // A view kept from before the growth reads nothing: the host must make a fresh one on every copy.
  expect(kept.byteLength).toBe(0);
  expect(new Uint8Array(blockOf(grown))).toEqual(new Uint8Array(blockOf(plain)));
  expect(tickOf(blockOf(grown))).toBe(2);
});

test("padding makes the block longer and it still passes the check", () => {
  const sim = new Sim(SEED, 0, 65536, 0);
  expect(sim.block_len()).toBe(8 + 65536);
  sim.step();
  expect(checkLayoutHash(blockOf(sim))).toEqual({ ok: true });
});

test("a bad seed, mode or padding is refused with a plain reason", () => {
  expect(() => new Sim(new Uint8Array(31), 0, 0, 0)).toThrow("the match seed must be 32 bytes and is 31");
  expect(() => new Sim(SEED, 1, 0, 0)).toThrow("only local play (0)");
  // The whole number counts: 256 must not be cut down to 0, which is local play.
  expect(() => new Sim(SEED, 256, 0, 0)).toThrow("mode 256 does not exist");
  expect(() => new Sim(SEED, 0, 0xffff_ffff, 0)).toThrow("above the writer's cap");
});

test("the panic hook's buffer stands from the module's start: no text, and its room said", () => {
  const ptr = panic_buffer();
  const header = new DataView(wasm.memory.buffer, ptr, 8);
  expect(header.getUint32(0, true)).toBe(0);
  // The room is written by the module's start function, which the glue runs as it instantiates the module: so
  // the hook is installed before any other export can run. (A trap's text itself: rules.test.mjs.)
  expect(header.getUint32(4, true)).toBe(1016);
  expect(readPanicMessage(wasm.memory, ptr)).toBe("");
});

test("in the glue, each per-tick call reaches exactly one export", () => {
  const glue = readFileSync(new URL("../pkg/sim_wasm.js", import.meta.url), "utf8");
  for (const [method, exported] of [["input(sequence)", "sim_input"], ["step()", "sim_step"]]) {
    const from = glue.indexOf(`    ${method} {`);
    expect(from, method).toBeGreaterThan(-1);
    const body = glue.slice(from, glue.indexOf("\n    }", from));
    expect(body.match(/wasm\.\w+/g), method).toEqual([`wasm.${exported}`]);
  }
});

test("the host on the real Wasm: two calls per tick, one transferred block per tick, each passing the check", () => {
  const sim = new Sim(SEED, 0, 0, 1000);
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
  host.onMessage({ type: "input", sequence: 9, returned: [] });
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
  expect(sim.newest_input()).toBe(9);
  expect(host.stats()).toMatchObject({ ticks: 30, undetached: 0, maxTicksInOneWake: 1, drops: 0 });
});
