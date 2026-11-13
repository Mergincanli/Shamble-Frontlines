#!/usr/bin/env node
// npt-hash-wasm's Node loader (u4-build-plan.md §5 U4d2; u4d2-brief.md 2.2): runs one recording through the wasm32
// build of npt-hash-wasm and writes its per-tick hashes in the line form npt-lab's legs write, for check.mjs's `wasm`
// step to compare with the native legs (npt-lab's `hash compare`, HashCompare).
//   node crates/hash-wasm/loader.mjs --wasm <file.wasm> --rec <file.rec> --name <recording> --first-tick <tick>
//        --out <lines file> [--bite <index> --arm <tick>] [--dump-at <tick|end> --dump-out <dump file>]
// - Before instantiating, it refuses the module (moduleProblems), naming each reason: an export whose name is not on
//   LOADER_EXPORTS or whose kind is not the listed one, a listed name the module lacks (the spike's third unsafe
//   exception's test: the advisor's condition 7 (b)); every import, by module and name (WebAssembly.Module.imports),
//   so no host clock or random source can reach the Wasm leg (the advisor's U4d2 round-1 ruling, FX-4: an import
//   object `{}` would have given a module importing `constructor.constructor` JS's Function); and a target_features
//   section that lists simd128 or relaxed-simd as used or required, or none, or more than one, or one that does not
//   parse (FX-2: plan.md §4.5's "simd128 and relaxed-simd off", read from the module itself, so a source-level
//   `#[target_feature]` or a flag reaching rustc by a route check.mjs does not read is refused too). It instantiates
//   with `Object.create(null)`, an import object with nothing in it, not even Object.prototype's names.
// - It streams the recording in: the file's bytes as little-endian 32-bit words, the last zero-padded, fed in chunks
//   of 64 KiB with `step` called until NEED_MORE between them, never the whole file before the first step.
// - Per tick it writes `h <recording> <tick> <hash>` (the hash as 16 hex digits, from hash_hi and hash_lo), the ticks
//   numbered from --first-tick as the recording's header does (`step` gives a status only; the world's own tick is in
//   the hashed state, so a leg that stepped other ticks mismatches by hash), and at the end `stepped <n>`.
// - With --dump-at it writes the dump's words at that tick (or after the last, `end`) through dump_len and dump_word:
//   `dump <recording> <tick|end>`, `hash <16 hex>`, then one word a line as 8 hex digits (npt-lab's dump form).
// - A bite comes in as its index in World::BITES, from 1 (`npt-lab hash bites` lists them), with its arming tick.
// - A trap (WebAssembly.RuntimeError: the wasm32 build aborts on a panic), a FAILED status, a refused `begin`, a file
//   that ends inside a tick or a dump tick never stepped ends it with exit 1, naming the recording and the last tick
//   stepped; a usage error exits 2. Each run is its own Node process (check.mjs runs it through process.execPath).

import { closeSync, openSync, readFileSync, readSync, writeFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

/** npt-hash-wasm's seven exports (crates/hash-wasm/src/lib.rs), each a function: the third exception's list. */
export const HASH_WASM_EXPORTS = ["begin", "feed", "step", "hash_lo", "hash_hi", "dump_len", "dump_word"];
/**
 * The linker's own exports in the release build of a wasm32-unknown-unknown cdylib: `memory`, the module's linear
 * memory, which wasm-ld exports by default (rustc 1.99.1's release build of npt-hash-wasm exports it and nothing else
 * beside the seven; it imports nothing).
 */
export const LINKER_EXPORTS = { memory: "memory" };
/** Every export the loader accepts, by name, with its kind. */
export const LOADER_EXPORTS = { ...Object.fromEntries(HASH_WASM_EXPORTS.map((n) => [n, "function"])), ...LINKER_EXPORTS };

/** `begin`'s results (crates/hash-wasm/src/lib.rs). */
export const BEGIN = { BEGUN: 0, REFUSED: 1 };
/** `step`'s results, one per npt_sim::replay::Step (crates/hash-wasm/src/lib.rs). */
export const STEP = { NEED_MORE: 0, STEPPED: 1, DONE: 2, FAILED: 3 };
/** The bytes read from the recording between steps. */
const BLOCK_BYTES = 1 << 16;

/** The target features plan.md §4.5 leaves off on wasm32, refused in a module's target_features section as `+` or `=`. */
const REFUSED_FEATURES = ["simd128", "relaxed-simd"];

/**
 * A target_features custom section's body read (the WebAssembly tool conventions' Linking.md, "Target Features
 * Section", which wasm-ld writes: a LEB128 count, then per entry a prefix byte, `+` used, `-` disallowed or `=`
 * required, and the feature's name as a LEB128 length and UTF-8 bytes): { features: [{ prefix, name }] }, or
 * { error } naming why it does not parse (a length past the end, a prefix of another byte, a name that is no UTF-8, or
 * bytes after the last entry).
 */
function targetFeatures(bytes) {
  let at = 0;
  const leb = (what) => {
    let n = 0;
    for (let shift = 0; shift < 35; shift += 7) {
      if (at >= bytes.length) throw new Error(`it ends inside ${what}`);
      const b = bytes[at++];
      n += (b & 0x7f) * 2 ** shift;
      if (!(b & 0x80)) return n;
    }
    throw new Error(`${what} runs past 5 bytes`);
  };
  try {
    const count = leb("its entry count");
    const features = [];
    for (let k = 0; k < count; k++) {
      if (at >= bytes.length) throw new Error(`it ends before entry ${k + 1} of ${count}`);
      const prefix = String.fromCharCode(bytes[at++]);
      if (!["+", "-", "="].includes(prefix)) throw new Error(`entry ${k + 1}'s prefix is byte ${prefix.charCodeAt(0)}, not +, - or =`);
      const length = leb(`entry ${k + 1}'s name length`);
      if (at + length > bytes.length) throw new Error(`entry ${k + 1}'s name runs past its end`);
      const name = new TextDecoder("utf-8", { fatal: true }).decode(bytes.subarray(at, at + length));
      at += length;
      features.push({ prefix, name });
    }
    if (at !== bytes.length) throw new Error(`${bytes.length - at} byte(s) follow its last entry`);
    return { features };
  } catch (e) {
    return { error: e.message };
  }
}

/**
 * The loader's refusal of a compiled module, as sentences (empty when it may be instantiated): every export whose name
 * is not on LOADER_EXPORTS, or whose kind is not the listed one, by name, and every listed name it lacks; every import,
 * by module and name; and its target_features section (targetFeatures) when it lists simd128 or relaxed-simd with `+`
 * or `=`, or when the module has none, or more than one, or one that does not parse.
 */
export function moduleProblems(module) {
  const problems = [];
  const exports = WebAssembly.Module.exports(module);
  for (const { name, kind } of exports) {
    if (!Object.hasOwn(LOADER_EXPORTS, name)) problems.push(`the module exports ${JSON.stringify(name)} (a ${kind}), which is not on the loader's list`);
    else if (LOADER_EXPORTS[name] !== kind) problems.push(`the module exports ${JSON.stringify(name)} as a ${kind}, not a ${LOADER_EXPORTS[name]}`);
  }
  for (const name of Object.keys(LOADER_EXPORTS)) {
    if (!exports.some((e) => e.name === name)) problems.push(`the module does not export ${JSON.stringify(name)}`);
  }
  for (const { module: from, name, kind } of WebAssembly.Module.imports(module)) {
    problems.push(`the module imports ${JSON.stringify(`${from}.${name}`)} (a ${kind}): the loader gives a module nothing to import`);
  }
  const sections = WebAssembly.Module.customSections(module, "target_features");
  if (sections.length !== 1) {
    problems.push(`the module has ${sections.length} target_features sections, not one, so its target features cannot be read (wasm-ld writes one)`);
  } else {
    const read = targetFeatures(new Uint8Array(sections[0]));
    if (read.error) problems.push(`the module's target_features section does not parse: ${read.error}`);
    for (const { prefix, name } of read.features ?? []) {
      if (REFUSED_FEATURES.includes(name) && prefix !== "-") {
        problems.push(`the module's target_features section lists ${prefix}${name}, which plan.md §4.5 leaves off (simd128 and relaxed-simd)`);
      }
    }
  }
  return problems;
}

/** The loader's refusal of a module's bytes (moduleProblems); bytes that are no WebAssembly module are refused too. */
export function exportProblems(bytes) {
  let module;
  try {
    module = new WebAssembly.Module(bytes);
  } catch (e) {
    return [`not a WebAssembly module: ${e.message}`];
  }
  return moduleProblems(module);
}

/**
 * A module the loader accepts (moduleProblems: no import among the rest, refused by name before this point), instantiated
 * with `Object.create(null)`, an import object with no names at all: its exports. A module that still fails to
 * instantiate is refused with the engine's message.
 */
export function instantiate(module) {
  const problems = moduleProblems(module);
  if (problems.length) throw new Error(`refused: ${problems.join("; ")}`);
  try {
    return new WebAssembly.Instance(module, Object.create(null)).exports;
  } catch (e) {
    throw new Error(`refused: the module does not instantiate: ${e.message}`);
  }
}

/**
 * Calls `onChunk(words)` with the file's bytes as little-endian 32-bit words, BLOCK_BYTES at a time, the last word
 * zero-padded; stops early when `onChunk` returns false.
 */
export function forEachChunk(path, onChunk) {
  const fd = openSync(path, "r");
  const buf = Buffer.alloc(BLOCK_BYTES + 3);
  let carry = 0;
  try {
    for (;;) {
      const n = readSync(fd, buf, carry, BLOCK_BYTES, null);
      const have = carry + n;
      if (n === 0) {
        if (carry > 0) {
          buf.fill(0, carry, 4);
          onChunk([buf.readUInt32LE(0)]);
        }
        return;
      }
      const whole = have - (have % 4);
      const words = new Array(whole / 4);
      for (let i = 0; i < words.length; i++) words[i] = buf.readUInt32LE(i * 4);
      carry = have - whole;
      buf.copy(buf, 0, whole, have);
      if (onChunk(words) === false) return;
    }
  } finally {
    closeSync(fd);
  }
}

const hex = (n, digits) => (n >>> 0).toString(16).padStart(digits, "0");

/**
 * One recording through an instance's exports: { ok, lines, stepped, dump, error }. `lines` are the per-tick lines and
 * the closing `stepped <n>` (or `failed <why>`); `dump` the requested dump's text, if it was taken.
 */
export function replay(exports, { path, name, firstTick, bite = 0, arm = 0, dumpAt = null }) {
  const lines = [];
  let stepped = 0;
  let status = STEP.NEED_MORE;
  let dump = null;
  let error = null;
  const tickOf = (i) => (firstTick + i) >>> 0;
  const last = () => (stepped ? `tick ${tickOf(stepped - 1)}` : "no tick");
  const hash = () => `${hex(exports.hash_hi(), 8)}${hex(exports.hash_lo(), 8)}`;
  const takeDump = (at) => {
    const n = exports.dump_len() >>> 0;
    const words = new Array(n);
    for (let i = 0; i < n; i++) words[i] = `${hex(exports.dump_word(i), 8)}\n`;
    return `dump ${name} ${at}\nhash ${hash()}\n${words.join("")}`;
  };
  const begun = exports.begin(bite, arm) >>> 0;
  if (begun !== BEGIN.BEGUN) {
    error = `${name}: begin(${bite}, ${arm}) returned ${begun}, not BEGUN: the module refused the bite (a measured build, or an index outside World::BITES)`;
  } else {
    try {
      forEachChunk(path, (words) => {
        for (const w of words) exports.feed(w);
        for (;;) {
          status = exports.step() >>> 0;
          if (status !== STEP.STEPPED) break;
          const tick = tickOf(stepped);
          stepped++;
          lines.push(`h ${name} ${tick} ${hash()}`);
          if (dumpAt === tick) dump = takeDump(tick);
        }
        return status === STEP.NEED_MORE;
      });
      if (status === STEP.FAILED) error = `${name}: step() returned FAILED after ${last()}`;
      else if (status === STEP.NEED_MORE) error = `${name}: the file ends inside a tick, after ${last()}`;
      else if (status !== STEP.DONE) error = `${name}: step() returned ${status}, no status it gives, after ${last()}`;
      else if (dumpAt === "end") dump = takeDump("end");
      else if (dumpAt !== null && dump === null) error = `${name}: no tick ${dumpAt} was stepped, so no dump was taken (after ${last()})`;
    } catch (e) {
      if (!(e instanceof WebAssembly.RuntimeError)) throw e;
      error = `${name}: the Wasm leg trapped after ${last()}: ${e.message}`;
    }
  }
  lines.push(error ? `failed ${error}` : `stepped ${stepped}`);
  return { ok: !error, lines, stepped, dump, error };
}

const USAGE =
  "usage: node loader.mjs --wasm <file.wasm> --rec <file.rec> --name <recording> --first-tick <tick> --out <lines file> " +
  "[--bite <index> --arm <tick>] [--dump-at <tick|end> --dump-out <dump file>]";

/** The command's options: each `--name value` once. */
export function parse(argv) {
  const opts = {};
  for (let i = 0; i < argv.length; i += 2) {
    const flag = argv[i];
    if (!flag.startsWith("--") || i + 1 >= argv.length) throw new Error(`${flag}: every option is --name value`);
    const k = flag.slice(2);
    if (Object.hasOwn(opts, k)) throw new Error(`${flag} given twice`);
    opts[k] = argv[i + 1];
  }
  const known = ["wasm", "rec", "name", "first-tick", "out", "bite", "arm", "dump-at", "dump-out"];
  for (const k of Object.keys(opts)) if (!known.includes(k)) throw new Error(`unknown option --${k}`);
  for (const k of ["wasm", "rec", "name", "first-tick", "out"]) if (!opts[k]) throw new Error(`--${k} is required`);
  const u32 = (k, v) => {
    if (!/^\d+$/.test(v) || Number(v) > 0xffff_ffff) throw new Error(`--${k} ${JSON.stringify(v)} is not a whole number below 2^32`);
    return Number(v);
  };
  if (("bite" in opts) !== ("arm" in opts)) throw new Error("--bite and --arm go together");
  if (("dump-at" in opts) !== ("dump-out" in opts)) throw new Error("--dump-at and --dump-out go together");
  const bite = "bite" in opts ? u32("bite", opts.bite) : 0;
  if ("bite" in opts && bite === 0) throw new Error("--bite is World::BITES's index from 1");
  return {
    wasm: opts.wasm,
    path: opts.rec,
    name: opts.name,
    firstTick: u32("first-tick", opts["first-tick"]),
    out: opts.out,
    bite,
    arm: "arm" in opts ? u32("arm", opts.arm) : 0,
    dumpAt: !("dump-at" in opts) ? null : opts["dump-at"] === "end" ? "end" : u32("dump-at", opts["dump-at"]),
    dumpOut: opts["dump-out"] ?? null,
  };
}

/** The command: 0 on a recording replayed, 1 on a refusal or a failed replay, 2 on a usage error. */
export function main(argv) {
  let o;
  try {
    o = parse(argv);
  } catch (e) {
    console.error(`loader: ${e.message}\n${USAGE}`);
    return 2;
  }
  let exports;
  try {
    exports = instantiate(new WebAssembly.Module(readFileSync(o.wasm)));
  } catch (e) {
    console.error(`loader: ${o.wasm}: ${e.message}`);
    return 1;
  }
  const r = replay(exports, o);
  writeFileSync(o.out, `${r.lines.join("\n")}\n`);
  if (r.dump !== null) writeFileSync(o.dumpOut, r.dump);
  if (!r.ok) {
    console.error(`loader: ${r.error}`);
    return 1;
  }
  const armed = o.bite ? `, bite ${o.bite} armed at tick ${o.arm}` : "";
  const dumped = r.dump !== null ? `, its dump at ${o.dumpAt} written` : "";
  console.log(`wasm32 leg: ${o.name}: ${r.stepped} ticks stepped${armed}${dumped}`);
  return 0;
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  process.exitCode = main(process.argv.slice(2));
}
