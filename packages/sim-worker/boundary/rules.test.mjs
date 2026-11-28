// The rule tests in WebAssembly, under Node: the float vectors and the maths routines' (PM-0004-22), the random stream's vectors
// (PM-0004-27), the planted counter overflow with the panic hook's text (PM-0002-53), and the imports of the Wasm
// players get (PM-0004-10: no clock). They load the test-only module (crates/devtools), which every build but a
// production one makes in its release and its dev profile (`node packages/client/scripts/build.mjs --wasm-only`),
// into .gate/rules-module/ at the repository's root: outside every package, so that no server can hand it out.
// A missing module fails this file at its imports: these tests never skip.
import { readFileSync } from "node:fs";
import { homedir } from "node:os";
import path from "node:path";
import { describe, expect, test } from "vitest";
import * as dev from "../../../.gate/rules-module/dev/devtools.js";
import * as release from "../../../.gate/rules-module/release/devtools.js";
import { readPanicMessage } from "../src/panic-message.ts";

const vectors = (name) => readFileSync(new URL(`../../../crates/devtools/vectors/${name}`, import.meta.url));
const inputs = vectors("f32-inputs.bin");
const expected = vectors("f32-expected.txt").toString("utf8");
const words = vectors("chacha8-words.bin");
const libmInputs = vectors("libm-inputs.bin");
const libmExpected = vectors("libm-expected.txt").toString("utf8");
// The dev profile is not optimised: its own limit, far above what it took here (under 0.1 s).
const LIMIT_MS = 60_000;

describe.each([["release", release], ["dev", dev]])("the test-only module, %s profile", (profile, glue) => {
  // One instance per profile, and the order matters: the trap comes last, and a trapped instance is not called again.
  const wasm = glue.initSync({ module: readFileSync(new URL(`../../../.gate/rules-module/${profile}/devtools_bg.wasm`, import.meta.url)) });

  test("every allowed float operation gives the exact reference's results, but for mul_add's pinned cases, which give their measured values", { timeout: LIMIT_MS }, () => {
    expect(glue.f32_vectors(inputs, expected)).toBe("");
    // The check runs in the Wasm and can fail: one changed bit of one input is found.
    const changed = Uint8Array.from(inputs);
    changed[1] ^= 0x40;
    expect(glue.f32_vectors(changed, expected)).toContain("not the exact reference's results: add 4 ");
  });

  test("libm's exponential and sine-and-cosine give the outputs the native build recorded (PM-0004-22)", { timeout: LIMIT_MS }, () => {
    expect(glue.libm_vectors(libmInputs, libmExpected)).toBe("");
    // The check runs in the Wasm and can fail: one changed byte of the inputs is named by the expected file's header.
    const changed = Uint8Array.from(libmInputs);
    changed[1] ^= 0x40;
    expect(glue.libm_vectors(changed, libmExpected)).toContain("the inputs are not the ones the expected file's header names: ");
  });

  test("the random stream gives the vector file's words", { timeout: LIMIT_MS }, () => {
    expect(glue.chacha8_vectors(words)).toBe("");
    const changed = Uint8Array.from(words);
    changed[4 * 9_999] ^= 1;
    expect(glue.chacha8_vectors(changed)).toContain("stream 0, word 9999:");
  });

  test("a counter that overflows traps, and the hook keeps the panic's place and message", () => {
    const buffer = glue.panic_buffer();
    expect(readPanicMessage(wasm.memory, buffer)).toBe("");
    // Where wrapping is written it wraps, and a sum that fits is a sum.
    expect(glue.plant_u16_wrapping_add(65_535, 1)).toBe(0);
    expect(glue.plant_u16_add(65_534, 1)).toBe(65_535);
    expect(() => glue.plant_u16_add(65_535, 1)).toThrow(WebAssembly.RuntimeError);
    // Read through the memory alone, as the Worker's host does after a trap.
    expect(readPanicMessage(wasm.memory, buffer)).toMatch(/^panicked at crates\/devtools\/src\/wasm\.rs:\d+:\d+:\nattempt to add with overflow$/);
  });
});

test("the Wasm players get imports nothing but its own glue: no clock, no random source", () => {
  const imports = WebAssembly.Module.imports(new WebAssembly.Module(readFileSync(new URL("../pkg/sim_wasm_bg.wasm", import.meta.url))));
  expect(imports.length).toBeGreaterThan(0);
  for (const entry of imports) {
    expect(entry, entry.name).toMatchObject({ module: "./sim_wasm_bg.js", kind: "function" });
    expect(entry.name).toMatch(/^__wbg_(__wbindgen_throw|Error)_[0-9a-f]{16}$|^__wbindgen_init_externref_table$/);
  }
  // And the two modules are two files: the test-only one is not the players' under another name.
  expect(readFileSync(new URL("../../../.gate/rules-module/release/devtools_bg.wasm", import.meta.url)).equals(readFileSync(new URL("../pkg/sim_wasm_bg.wasm", import.meta.url)))).toBe(false);
});

test("no Wasm names the cargo folder of the machine that built it", () => {
  // A panic's place inside a registry crate is a path. The build maps cargo's own folder to one fixed place, so
  // one commit gives the same bytes on every machine (RR-F5); here: the fixed place is there, and this machine's is not.
  const registry = path.join(process.env["CARGO_HOME"] ?? path.join(homedir(), ".cargo"), "registry");
  for (const file of ["../pkg/sim_wasm_bg.wasm", "../../../.gate/rules-module/release/devtools_bg.wasm", "../../../.gate/rules-module/dev/devtools_bg.wasm"]) {
    const text = readFileSync(new URL(file, import.meta.url)).toString("latin1");
    expect(text.includes("/cargo/registry/src/"), file).toBe(true);
    expect(text.includes(registry), file).toBe(false);
  }
});
