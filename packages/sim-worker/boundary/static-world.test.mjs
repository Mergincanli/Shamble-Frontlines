// The static world's proof in WebAssembly, under Node (PM-0004-32): the check of
// crates/devtools/tests/static_world.rs, run by the test-only module in its release and its dev profile. The
// committed lines are the native build's: a module whose tree for one map, or whose answer to one ray, differs by a
// bit gives another line, and the check names the map. A missing module fails this file at its imports.
import { readFileSync } from "node:fs";
import { describe, expect, test } from "vitest";
import * as dev from "../../../.gate/rules-module/dev/devtools.js";
import * as release from "../../../.gate/rules-module/release/devtools.js";
import { readPlainFile } from "./plain-file.ts";

// The committed vector, read only when it is a plain file in a plain folder (plain-file.ts).
const lines = new TextDecoder("utf-8", { fatal: true }).decode(readPlainFile(new URL("../../../crates/devtools/vectors/", import.meta.url), "static-world.txt"));

describe.each([["release", release], ["dev", dev]])("the static world's proof, %s profile", (profile, glue) => {
  glue.initSync({ module: readFileSync(new URL(`../../../.gate/rules-module/${profile}/devtools_bg.wasm`, import.meta.url)) });

  test("every map's tree and every ray's answer are the native build's", { timeout: 120_000 }, () => {
    expect(lines.split("\n")).toHaveLength(14);
    const started = performance.now();
    expect(glue.static_world(lines)).toBe("");
    console.log(`the static world's proof, ${profile} profile: 12 maps and 24,000 rays in ${Math.round(performance.now() - started)} ms`);
    // The compare bites: one digit of one ray digest changed is found, with its map.
    const line = lines.split("\n")[8];
    const changed = lines.replace(line, `${line.slice(0, -1)}${line.endsWith("0") ? "1" : "0"}`);
    expect(glue.static_world(changed)).toBe(`this build gives\n  ${line}\nand the file has\n  ${line.slice(0, -1)}${line.endsWith("0") ? "1" : "0"}`);
  });
});
