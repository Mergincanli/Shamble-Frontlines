// The security seat's round-2 probe of the loader's refusals (u4d2-r2-review-security.md): FX-2 (the target_features
// section), FX-4 (imports) and FX-5 (export kinds), on the copy's crates/hash-wasm/loader.mjs. Part A reads the .wasm
// files given on the command line; parts B to D build modules in memory, each with the seven listed function exports
// and `memory` unless a case changes that, so that only the case's own bytes can be refused. Writes nothing.
// (Second form: `kinds` is read with Object.hasOwn, as `kinds["__proto__"]` had read Object.prototype.)
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const { moduleProblems, instantiate, HASH_WASM_EXPORTS } = await import(pathToFileURL(join(process.cwd(), "crates", "hash-wasm", "loader.mjs")).href);

const leb = (n) => {
  const out = [];
  do {
    let b = n & 0x7f;
    n >>>= 7;
    if (n) b |= 0x80;
    out.push(b);
  } while (n);
  return out;
};
const name = (s) => {
  const b = typeof s === "string" ? [...Buffer.from(s)] : s;
  return [...leb(b.length), ...b];
};
const section = (id, body) => [id, ...leb(body.length), ...body];
const entry = (prefix, n) => [prefix.charCodeAt(0), ...name(n)];
const body = (entries) => [...leb(entries.length), ...entries.flat()];
const BUILT = ["bulk-memory", "bulk-memory-opt", "call-indirect-overlong", "multivalue", "mutable-globals", "nontrapping-fptoint", "reference-types", "sign-ext"];
const CLEAN = body(BUILT.map((n) => entry("+", n)));

/** A module: the listed exports (`kinds` changes one's kind), `imports` as [module, field, kind], `sections` the
 * target_features bodies (CLEAN by default), `customName` that section's name. */
function mod({ kinds = {}, imports = [], sections = [CLEAN], customName = "target_features", extra = [] } = {}) {
  const names = [...HASH_WASM_EXPORTS, "memory", ...extra];
  const kindOf = (x) => (Object.hasOwn(kinds, x) ? kinds[x] : x === "memory" ? "memory" : "function");
  const fns = names.filter((x) => kindOf(x) === "function");
  const globals = names.filter((x) => kindOf(x) === "global");
  const tables = names.filter((x) => kindOf(x) === "table");
  const memImported = imports.some((i) => i[2] === "memory");
  const tabImported = imports.some((i) => i[2] === "table");
  const fnImports = imports.filter((i) => i[2] === "function").length;
  const globImports = imports.filter((i) => i[2] === "global").length;
  const desc = { function: [0x00, 0], memory: [0x02, 0x00, 1], table: [0x01, 0x70, 0x00, 1], global: [0x03, 0x7f, 0x00] };
  const exp = (x) => {
    const k = kindOf(x);
    if (k === "memory") return [...name(x), 0x02, 0];
    if (k === "table") return [...name(x), 0x01, 0];
    if (k === "global") return [...name(x), 0x03, ...leb(globImports + globals.indexOf(x))];
    return [...name(x), 0x00, ...leb(fnImports + fns.indexOf(x))];
  };
  const needMem = names.some((x) => kindOf(x) === "memory") && !memImported;
  return new Uint8Array([
    0x00, 0x61, 0x73, 0x6d, 1, 0, 0, 0,
    ...section(1, [1, 0x60, 0, 1, 0x7f]),
    ...(imports.length ? section(2, [...leb(imports.length), ...imports.flatMap(([m, f, k]) => [...name(m), ...name(f), ...desc[k]])]) : []),
    ...section(3, [...leb(fns.length), ...fns.map(() => 0)]),
    ...(tables.length && !tabImported ? section(4, [1, 0x70, 0x00, 1]) : []),
    ...(needMem ? section(5, [1, 0x00, 1]) : []),
    ...(globals.length ? section(6, [...leb(globals.length), ...globals.flatMap(() => [0x7f, 0x00, 0x41, 0, 0x0b])]) : []),
    ...section(7, [...leb(names.length), ...names.flatMap(exp)]),
    ...section(10, [...leb(fns.length), ...fns.flatMap(() => [4, 0x00, 0x41, 0x00, 0x0b])]),
    ...sections.flatMap((b) => section(0, [...name(customName), ...b])),
  ]);
}

const report = (label, bytes) => {
  let m;
  try {
    m = new WebAssembly.Module(bytes);
  } catch (e) {
    console.log(`  ${label}: not a module: ${e.message}`);
    return;
  }
  const p = moduleProblems(m);
  let inst;
  try {
    inst = `instantiated (${typeof instantiate(m).begin})`;
  } catch (e) {
    inst = e.message.startsWith("refused: ") ? "instantiate refused" : `instantiate threw: ${e.message}`;
  }
  console.log(`  ${label}: ${p.length} problem(s); ${inst}`);
  for (const l of p) console.log(`      ${l}`);
};

console.log("--- A. the built and probe modules (files)");
for (const f of process.argv.slice(2)) {
  const bytes = readFileSync(f);
  report(f.split("/").slice(-2).join("/"), bytes);
  // The code section's bytes, and whether f32x4.relaxed_madd's opcode (0xfd 0x105: fd 85 02) is in it.
  let p = 8;
  const u = () => {
    let r = 0;
    let s = 0;
    for (;;) {
      const b = bytes[p++];
      r += (b & 0x7f) * 2 ** s;
      s += 7;
      if (!(b & 0x80)) return r;
    }
  };
  while (p < bytes.length) {
    const id = bytes[p++];
    const size = u();
    if (id === 10) {
      const code = bytes.subarray(p, p + size);
      let hits = 0;
      for (let i = 0; i + 2 < code.length; i++) if (code[i] === 0xfd && code[i + 1] === 0x85 && code[i + 2] === 0x02) hits++;
      console.log(`      code section ${size} B, fd 85 02 (f32x4.relaxed_madd) found ${hits} time(s)`);
    }
    p += size;
  }
}

console.log("--- B. FX-2: target_features bytes (every other part of the module clean)");
const B = {
  "the built list (control)": [CLEAN],
  "count 0, no entries": [[0]],
  "empty body": [[]],
  "count's LEB over 5 bytes": [[0x80, 0x80, 0x80, 0x80, 0x80, 0x00]],
  "count 2^32-1, one entry": [[0xff, 0xff, 0xff, 0xff, 0x0f, ...entry("+", "sign-ext")]],
  "count 2^35-1 (5th byte 0x7f), nothing after": [[0xff, 0xff, 0xff, 0xff, 0x7f]],
  "name length past the end": [[1, 0x2b, 0x7f, 0x61, 0x62]],
  "name length 2^32-1": [[1, 0x2b, 0xff, 0xff, 0xff, 0xff, 0x0f, 0x61]],
  "prefix 0x00": [[1, 0x00, ...name("simd128")]],
  "prefix '!'": [[1, 0x21, ...name("simd128")]],
  "prefix 0xff": [[1, 0xff, ...name("simd128")]],
  "name overlong UTF-8 (c0 af)": [[1, 0x2b, ...name([0xc0, 0xaf])]],
  "name a lone surrogate (ed a0 80)": [[1, 0x2b, ...name([0xed, 0xa0, 0x80])]],
  "name byte ff": [[1, 0x2b, ...name([0xff])]],
  "two sections, both clean": [CLEAN, CLEAN],
  "two sections, one clean, one +simd128": [CLEAN, body([entry("+", "simd128")])],
  "a byte after the last entry": [[...CLEAN, 0]],
  "+simd128": [body([...BUILT.map((n) => entry("+", n)), entry("+", "simd128")])],
  "=relaxed-simd": [body([entry("=", "relaxed-simd")])],
  "-simd128 -relaxed-simd (disallowed: passes)": [body([entry("-", "simd128"), entry("-", "relaxed-simd")])],
  "-simd128 then +simd128": [body([entry("-", "simd128"), entry("+", "simd128")])],
  "'SIMD128' (not a name wasm-ld writes)": [body([entry("+", "SIMD128")])],
  "'simd128\\0'": [body([entry("+", "simd128\0")])],
};
for (const [label, sections] of Object.entries(B)) report(label, mod({ sections }));
report("the section under another name ('target_featureS'), none named target_features", mod({ customName: "target_featureS" }));

console.log("--- C. FX-4: one import each, beside a clean target_features section (so FX-2 accepts the module)");
for (const [m, f, k] of [
  ["constructor", "constructor", "function"],
  ["__proto__", "valueOf", "function"],
  ["constructor", "keys", "function"],
  ["env", "now", "function"],
  ["", "", "function"],
  ["env", "memory", "memory"],
  ["env", "table", "table"],
  ["env", "g", "global"],
]) {
  report(`imports ${m}.${f} (${k})`, mod({ imports: [[m, f, k]] }));
}

console.log("--- D. FX-5: listed names of another kind; names off the list");
for (const [label, kinds, extra] of [
  ["begin as a global", { begin: "global" }, []],
  ["memory as a function", { memory: "function" }, []],
  ["memory as a global", { memory: "global" }, []],
  ["memory as a table", { memory: "table" }, []],
  ["step as a table", { step: "table" }, []],
  ["an extra export __proto__", {}, ["__proto__"]],
  ["an extra export constructor", {}, ["constructor"]],
  ["an extra export hasOwnProperty", {}, ["hasOwnProperty"]],
]) {
  report(label, mod({ kinds, extra }));
}
