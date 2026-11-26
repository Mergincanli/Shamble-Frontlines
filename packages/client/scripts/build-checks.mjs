// The build script's checks, each a small function with no side effect beyond reading the file it is given, so
// each is tested without a build (build-checks.test.mjs). The build itself (scripts/build.mjs) calls them.

import { createHash } from "node:crypto";
import { lstatSync, readFileSync } from "node:fs";
import path from "node:path";
import { brotliCompressSync, constants, gzipSync } from "node:zlib";

/** No served file may reach 25 MiB (PM-0002-29's gated clause; a static host's per-file limit). */
export const MAX_SERVED_BYTES = 26_214_400;

/**
 * The version of the `wasm-bindgen` crate that `Cargo.lock` records. The command-line tool that writes the glue
 * must be exactly this version (PM-0002-18; ADR 0002 part 2).
 */
export function lockedWasmBindgenVersion(lockText) {
  const versions = [];
  for (const block of lockText.split(/^\[\[package\]\]$/m).slice(1)) {
    if (/^name = "wasm-bindgen"$/m.test(block)) {
      const version = /^version = "([^"]+)"$/m.exec(block);
      if (version === null) throw new Error("Cargo.lock: the wasm-bindgen entry has no version line");
      versions.push(version[1]);
    }
  }
  if (versions.length !== 1) {
    throw new Error(`Cargo.lock: expected exactly one wasm-bindgen entry, found ${versions.length}`);
  }
  return versions[0];
}

/**
 * Compares what the tool printed for `--version` with the version in `Cargo.lock`. Throws, with both versions in
 * the message, unless they are equal; the build then stops before any binding is written.
 */
export function checkCli(versionOutput, locked) {
  const printed = /^wasm-bindgen (\d+\.\d+\.\d+)$/.exec(versionOutput.trim());
  if (printed === null) {
    throw new Error(`the wasm-bindgen tool printed no version this script can read: ${JSON.stringify(versionOutput)}`);
  }
  if (printed[1] !== locked) {
    throw new Error(
      `the wasm-bindgen tool is version ${printed[1]}, and Cargo.lock records the library at ${locked}: ` +
        "the two must be equal, so no binding is written",
    );
  }
}

/**
 * Checks the tool's file before it is run at all, `--version` included: a plain file (no link) of the recorded size
 * and sha256. The tool is a program from the internet that no lockfile covers, and anything that runs as this user
 * could have replaced it since it was checked. Throws when the expected values are missing: the build never runs a
 * tool it cannot compare.
 */
export function checkCliFile(file, expectedSha256, expectedSize) {
  if (typeof expectedSha256 !== "string" || !/^[0-9a-f]{64}$/.test(expectedSha256)) {
    throw new Error("the wasm-bindgen tool's expected sha256 is not given (64 hex digits): the tool is not run");
  }
  if (!Number.isInteger(expectedSize) || expectedSize < 1) {
    throw new Error("the wasm-bindgen tool's expected size is not given: the tool is not run");
  }
  const info = lstatSync(file);
  if (!info.isFile()) throw new Error(`the wasm-bindgen tool at ${file} is not a plain file: it is not run`);
  if (info.size !== expectedSize) {
    throw new Error(`the wasm-bindgen tool at ${file} is ${info.size} bytes, not the recorded ${expectedSize}: it is not run`);
  }
  const found = createHash("sha256").update(readFileSync(file)).digest("hex");
  if (found !== expectedSha256) {
    throw new Error(`the wasm-bindgen tool at ${file} has sha256 ${found}, not the recorded one: it is not run`);
  }
}

/**
 * One row per file: its size raw, with gzip at level 9 and with Brotli at quality 11 (PM-0002-29's settings).
 * @param {{ name: string, bytes: Uint8Array }[]} files
 */
export function sizeTable(files) {
  return files.map(({ name, bytes }) => ({
    name,
    raw: bytes.length,
    gzip: gzipSync(bytes, { level: 9 }).length,
    brotli: brotliCompressSync(bytes, {
      params: { [constants.BROTLI_PARAM_QUALITY]: 11, [constants.BROTLI_PARAM_SIZE_HINT]: bytes.length },
    }).length,
  }));
}

/** The rows of a size table whose file reaches the limit for a served file. */
export function tooLarge(rows) {
  return rows.filter((row) => row.raw >= MAX_SERVED_BYTES);
}

/** The size table as text, for the build's output and the report. */
export function formatSizeTable(rows) {
  const width = Math.max(4, ...rows.map((row) => row.name.length));
  const line = (name, raw, gzip, brotli) =>
    `${name.padEnd(width)}  ${String(raw).padStart(10)}  ${String(gzip).padStart(10)}  ${String(brotli).padStart(10)}`;
  return [line("file", "raw", "gzip -9", "brotli 11"), ...rows.map((row) => line(row.name, row.raw, row.gzip, row.brotli))].join(
    "\n",
  );
}

/**
 * What in a built page would stop a strict content security policy (ADR 0002 part 7: "the built HTML holds no
 * inline script or style, and the Worker and the Wasm load as files, never from `blob:` or `data:` URLs"). It
 * stands in for the gate's check (G8) until the gate v2 has one. Returns one line per problem.
 * @param {{ name: string, bytes: Uint8Array }[]} files
 */
export function cspProblems(files) {
  const problems = [];
  for (const { name, bytes } of files) {
    if (!/\.(html|js|css)$/.test(name)) continue;
    const text = Buffer.from(bytes).toString("utf8");
    if (name.endsWith(".html")) {
      if (/<script(?![^>]*\ssrc=)[^>]*>/i.test(text)) problems.push(`${name}: an inline script`);
      if (/<style[\s>]/i.test(text)) problems.push(`${name}: an inline style element`);
      if (/<[^>]+\sstyle\s*=/i.test(text)) problems.push(`${name}: a style attribute`);
      if (/<[^>]+\son[a-z]+\s*=/i.test(text)) problems.push(`${name}: an inline event handler`);
    }
    if (/data:(application\/wasm|text\/javascript|application\/javascript)/i.test(text)) {
      problems.push(`${name}: a script or the Wasm in a data: URL`);
    }
    if (name.endsWith(".js")) {
      // None of these is in the page today; each would need a looser policy than the one serve.mjs sends.
      for (const [what, pattern] of [["eval", /\beval\(/], ["new Function", /\bnew Function\(/], ["a blob: URL", /createObjectURL|blob:/]]) {
        if (pattern.test(text)) problems.push(`${name}: ${what}`);
      }
    }
  }
  return problems;
}

/** Text that only the development readout, its report and the tests' hook hold. */
const DEV_MARKERS = ["Copy report", "__shambleReport", "ticks while hidden", "first page: report", "key-diagram"];

/**
 * The one chunk the build made from the Worker's own source module, by the name the build gave it. `chunks` is what
 * the Worker's bundle holds (Rollup's output: `fileName`, `isEntry`, `facadeModuleId`); `workerSource` is the
 * absolute path of the Worker's entry module. Throws unless exactly one entry chunk comes from that module: the
 * check below must know the Worker's chunk from the build, never guess it from a file name.
 * @param {{ fileName: string, isEntry?: boolean, facadeModuleId?: string | null }[]} chunks
 */
export function workerEntryChunk(chunks, workerSource) {
  const source = workerSource.replaceAll("\\", "/");
  const entries = chunks
    .filter((chunk) => chunk.isEntry === true && (chunk.facadeModuleId ?? "").split("?", 1)[0].replaceAll("\\", "/") === source)
    .map((chunk) => chunk.fileName);
  if (entries.length !== 1) {
    throw new Error(`the build made ${entries.length} entry chunks from the Worker's source (${workerSource}), not exactly one`);
  }
  return entries[0];
}

/**
 * Two rules of the built page, one line per breach. The Wasm's glue is in the Worker's chunk and in no other (the
 * page never touches the Wasm); and a production build holds none of the development-only code, while a
 * development build holds all of it (so the markers themselves are known to be right). `workerEntry` is the
 * Worker's chunk as `workerEntryChunk` named it: every other script is the page's, whatever its file is called.
 * @param {{ name: string, bytes: Uint8Array }[]} files
 * @param {boolean} production
 * @param {string} workerEntry
 */
export function bundleProblems(files, production, workerEntry) {
  const scripts = files.filter(({ name }) => name.endsWith(".js")).map(({ name, bytes }) => ({ name, text: Buffer.from(bytes).toString("utf8") }));
  if (typeof workerEntry !== "string" || !scripts.some(({ name }) => name === workerEntry)) {
    return [`the Worker's chunk (${String(workerEntry)}) is not among the built scripts: the glue check cannot tell the page's code from the Worker's`];
  }
  const page = scripts.filter(({ name }) => name !== workerEntry);
  const problems = [
    ...page.filter(({ text }) => /__wbindgen|__wbg_/.test(text)).map(({ name }) => `${name}: the Wasm's glue outside the Worker's chunk`),
    // The page never touches WebAssembly (PM-0002-2): its chunks do not even name it, wherever a call would sit.
    ...page.filter(({ text }) => text.includes("WebAssembly")).map(({ name }) => `${name}: the page's code names WebAssembly`),
  ];
  for (const marker of DEV_MARKERS) {
    const found = scripts.filter(({ text }) => text.includes(marker)).map(({ name }) => name);
    if (production && found.length > 0) problems.push(`${found.join(", ")}: development code in a production build (${JSON.stringify(marker)})`);
    if (!production && found.length === 0) problems.push(`a development build without ${JSON.stringify(marker)}: the marker list is out of date`);
  }
  return problems;
}

/**
 * The only WebAssembly a built page may hold is the players' own. One line for every file among `files` that is a
 * WebAssembly module (by its name's ending or by its first four bytes) and is not byte for byte `playersWasm`, the
 * file the Worker's bindings were just written for; and one line when there is none at all. The test-only module
 * is built beside the players' Wasm: this keeps it, and anything else, out of what a server hands out.
 * @param {{ name: string, bytes: Uint8Array }[]} files
 * @param {Uint8Array} playersWasm
 */
export function wasmProblems(files, playersWasm) {
  const magic = Buffer.from([0x00, 0x61, 0x73, 0x6d]);
  const modules = files.filter(({ name, bytes }) => name.endsWith(".wasm") || Buffer.from(bytes.subarray(0, 4)).equals(magic));
  const others = modules.filter(({ bytes }) => !Buffer.from(bytes).equals(Buffer.from(playersWasm)));
  return [
    ...others.map(({ name }) => `${name}: a WebAssembly file that is not the players' own`),
    ...(modules.length === others.length ? ["no file is the players' WebAssembly: the Worker would have nothing to load"] : []),
  ];
}

/**
 * Refuses an output folder the build must not empty. Vite clears the folder it builds into, so the folder is either
 * the page's own `dist` or one whose name starts with `dist-`: a mistyped `--out-dir` cannot wipe sources or notes.
 */
export function checkOutDir(outDir, defaultDir) {
  if (outDir !== defaultDir && !/^dist-[\w.-]+$/.test(outDir.split(/[\\/]/).pop() ?? "")) {
    throw new Error(`--out-dir must be a folder named dist-<something>, because the build empties it: ${outDir}`);
  }
}

/**
 * Variables through which cargo's or the tool's settings can be changed from outside with no diff in the repository:
 * among them every `RUSTC_*`, the names through which a stable cargo takes nightly-only features (`__CARGO_*`,
 * `CARGO_UNSTABLE_*`), and the one variable a build script of the tree reads (`ENSURE_NO_PANIC`, the `libm` crate's:
 * it changes how that crate is compiled). Not refused: the two that name the build folder, which the build's `--target-dir` overrides.
 * Not covered: cargo's config files outside the repository, through which a profile setting (overflow checks among
 * them), a linker, a compiler wrapper or an `[env]` table can still be set with no diff here (the quality gate's
 * flag check); only their `rustflags` are shut out, by the build itself, which gives cargo its own (build.mjs,
 * `cargoEnv`).
 */
export const CARGO_OVERRIDES = new RegExp(
  "^(?!CARGO_(BUILD_)?TARGET_DIR$)(RUSTFLAGS|CARGO_ENCODED_RUSTFLAGS|RUSTC(_.*)?|RUSTUP_TOOLCHAIN|CARGO_INCREMENTAL|" +
    "__CARGO_.*|CARGO_(BUILD|PROFILE|RESOLVER|TARGET|UNSTABLE)_.*|WASM_BINDGEN_.*|RUST_LOG|ENSURE_NO_PANIC)$",
);

/**
 * The names in `env` that the Wasm build refuses, in the environment's order. Only names are read, never a value;
 * they are compared in upper case, because Windows takes a variable's name without regard to case.
 */
export function refusedBuildSettings(env) {
  return Object.keys(env).filter((name) => CARGO_OVERRIDES.test(name.toUpperCase()));
}


/**
 * The crates in `cargo tree`'s flat list (`-e normal,no-proc-macro --prefix none`) that are not ours: what is
 * compiled for the target and linked into the Wasm beside the workspace's own crates. Ours is a crate whose source
 * is a folder under `<root>/crates/` (`[profile.release]` covers it); a crate from the registry, from git, from
 * another registry or from a folder elsewhere all count, whatever cargo prints after its version. A line this
 * cannot read, a proc-macro, or a tree that does not start at `rootCrate` is a refusal: the list must be exact.
 */
export function runtimeCrates(treeText, root, rootCrate = "sim-wasm") {
  const ours = `${root.replaceAll("\\", "/").replace(/\/+$/, "")}/crates/`;
  const names = new Set();
  const lines = treeText.split("\n").filter((text) => text.trim() !== "");
  if (!(lines[0] ?? "").startsWith(`${rootCrate} v`)) throw new Error(`cargo tree did not start at ${rootCrate}: the overflow check has nothing to read`);
  for (const line of lines) {
    const entry = /^([A-Za-z0-9_-]+) v[0-9][^ ]*( \(([^)]*)\))?( \(\*\))?$/.exec(line.trim());
    if (entry === null || entry[3] === "proc-macro") throw new Error(`cargo tree printed a line the overflow check cannot place: ${line}`);
    if (!(entry[3] ?? "").replaceAll("\\", "/").startsWith(ours)) names.add(entry[1]);
  }
  return [...names].sort();
}

/** The crates the root `Cargo.toml` turns overflow checks on for by name: `[profile.release.package.<name>]`. */
export function overriddenCrates(cargoToml) {
  const names = [];
  for (const block of cargoToml.split(/^(?=\[)/m)) {
    const header = /^\[profile\.release\.package\.("?)([A-Za-z0-9_-]+)\1\]\s*$/m.exec(block.split("\n", 1)[0]);
    if (header !== null && /^overflow-checks\s*=\s*true\s*(#.*)?$/m.test(block)) names.push(header[2]);
  }
  return names.sort();
}

/**
 * The runtime crates of a build that would ship with overflow checks off: in the tree, and without a named override
 * (ADR 0002 part 4: the workspace's `"*"` turns them off for every crate that is not named). Empty when all is well.
 */
export function missingOverflowOverrides(treeText, cargoToml, root) {
  const overridden = new Set(overriddenCrates(cargoToml));
  return runtimeCrates(treeText, root).filter((name) => !overridden.has(name));
}

/**
 * What one build resolves for the simulation, from `cargo tree -e features,normal,no-proc-macro --prefix depth
 * --no-dedupe`: every feature of `sim` and of every package `sim` depends on, with those packages' versions,
 * sorted. Cargo prints a feature where it is asked for: a server that turns a feature on for a dependency it shares
 * with `sim` shows it under the server, so the whole tree is read, not `sim`'s branch alone. ADR 0002 part 3 wants
 * this list identical in every build that runs the simulation (PM-0004-74), because cargo unifies features over
 * everything one command builds.
 *
 * `closureTree` says which packages are `sim`'s (the tree on normal edges); `treeText` may be the same build's tree
 * with `dev` edges too, as the tests are built: a feature a dev-dependency turns on is then seen, and `sim`'s own
 * dev-dependencies do not count as its dependencies. Not modelled: a build made with other `--features` flags or a
 * narrower `-p` list than the one the tree was asked for (CI's release test line is one). Throws without a `sim`.
 *
 * A known clash, kept loud: ADR 0002 part 5 plans a feature that only dev-dependencies turn on (the generated
 * types). The day it reaches `sim` or one of its dependencies this compare refuses it and every build stops; the
 * step that first nests generated types decides which of the two gives way.
 */
export function simFeatureList(treeText, closureTree = treeText) {
  const parse = (text) => text.split("\n").map((line) => /^(\d+)(.*?)( \(command-line\))?$/.exec(line)).filter((line) => line !== null);
  const nameOf = (text) => /^([A-Za-z0-9_-]+) (v[0-9]|feature ")/.exec(text)?.[1];
  const closure = new Set(["sim"]);
  let seen = false;
  let under = -1;
  for (const [, depthText, text] of parse(closureTree)) {
    const depth = Number(depthText);
    if (under !== -1 && depth > under) closure.add(nameOf(text));
    else under = /^sim v[0-9]/.test(text) ? depth : -1;
    seen ||= under !== -1;
  }
  if (!seen) throw new Error("cargo tree printed no sim crate: the feature check has nothing to read");
  const found = new Set();
  for (const [, , text] of parse(treeText)) {
    const name = nameOf(text);
    if (closure.has(name) && (text.includes(' feature "') || name !== "sim")) found.add(text.replace(/ \([^)]*\)$/, ""));
  }
  return [...found].sort();
}

/** The name `build.mjs` gives the list of the tests as `cargo test --workspace` builds them. */
export const NATIVE_TESTS = "the native tests";

/**
 * The two named exceptions to "the same features in every build" (Mustafa's FEATURE-COMPARE (a), 2026-11-26: "Two
 * named exceptions, each pinned to exact entries and versions ... any other difference still stops every build").
 * A change of either package's version needs his yes again: its entry no longer matches, and every build stops.
 *
 * 1. `safe_arch` exists only where the target is x86: `wide` (which the collision library's `simba` always turns
 *    on) names it under a target condition, so the native tests hold these three entries and no WebAssembly build
 *    may hold any entry of that package.
 * 2. `thiserror`'s `std` is turned on by the type writer, a dev-dependency: the native tests hold these two
 *    entries, and only while the players' list holds exactly this version of the package.
 *
 * Maps, so that a package named like a property of every object cannot be found by accident.
 */
export const TARGET_ONLY_ENTRIES = new Map([["safe_arch", ["safe_arch v1.2.0", 'safe_arch feature "bytemuck"', 'safe_arch feature "default"']]]);
export const TEST_ONLY_FEATURES = new Map([["thiserror v2.0.21", ['thiserror feature "default"', 'thiserror feature "std"']]]);

/**
 * Where the simulation's feature lists differ between builds. `lists` maps a build's name to its
 * `simFeatureList`; the first is the players' Wasm, which every other list is held against.
 *
 * Refused, one line each: a list that holds two versions of one package (also when every list agrees); any entry
 * of a target-only package in a list other than the native tests' (also when every list agrees); an entry only
 * the players' list has; an entry another list has beyond the players', unless that list is the native tests'
 * and the entry is, to the letter, one of the two named exceptions' (the second only while the players' list
 * holds the exception's own version entry). Nothing is excused in any other list.
 */
export function simFeatureDifferences(lists, targetOnly = TARGET_ONLY_ENTRIES, testOnly = TEST_ONLY_FEATURES) {
  const nameOf = (entry) => /^([A-Za-z0-9_-]+) (v[0-9]|feature ")/.exec(entry)?.[1];
  const [[firstName, first], ...rest] = Object.entries(lists);
  const problems = [];
  for (const [name, list] of Object.entries(lists)) {
    const versions = list.filter((entry) => /^[A-Za-z0-9_-]+ v[0-9]/.test(entry));
    for (const entry of versions) {
      const twin = versions.find((other) => other !== entry && nameOf(other) === nameOf(entry));
      if (twin !== undefined && entry < twin) problems.push(`${name} holds two versions of one package the simulation runs on: ${entry} and ${twin}`);
    }
    const strays = name === NATIVE_TESTS ? [] : list.filter((entry) => targetOnly.has(nameOf(entry)));
    if (strays.length > 0) problems.push(`${name} holds ${JSON.stringify(strays)}, which only ${NATIVE_TESTS} may hold (a package of one target)`);
  }
  for (const [name, list] of rest) {
    const excused = (entry) =>
      name === NATIVE_TESTS &&
      ((targetOnly.get(nameOf(entry)) ?? []).includes(entry) || [...testOnly].some(([version, features]) => features.includes(entry) && first.includes(version)));
    const more = list.filter((entry) => !first.includes(entry) && !excused(entry));
    const fewer = first.filter((entry) => !list.includes(entry));
    if (more.length > 0 || fewer.length > 0) {
      problems.push(`the simulation's features differ between ${firstName} and ${name}: only ${name} has ${JSON.stringify(more)}; only ${firstName} has ${JSON.stringify(fewer)}`);
    }
  }
  return problems;
}

/**
 * The compiler flag that writes one fixed place instead of the builder's own cargo folder into panic locations of
 * registry crates, so one commit gives the same Wasm bytes on every machine (RR-F5). `cargoHome` is `CARGO_HOME`,
 * or nothing: cargo reads an empty value as unset, and then its home is `.cargo` under the user's home folder; a
 * relative value is resolved against `base`, the folder cargo is run in, because the compiler matches the flag
 * against absolute paths. A path with a control character in it is refused: the flags travel in one variable that
 * is split at such a character, so the rest of the path would arrive as a flag of its own.
 */
export function registryRemapFlag(cargoHome, homeDir, base) {
  const home = path.resolve(base, cargoHome === undefined || cargoHome === "" ? path.join(homeDir, ".cargo") : cargoHome);
  if (/[\u0000-\u001f\u007f]/.test(home)) throw new Error("cargo's home folder has a control character in its path: the build cannot name it in a compiler flag");
  return `--remap-path-prefix=${home.replaceAll("\\", "/").replace(/\/+$/, "")}/registry/src=/cargo/registry/src`;
}

// The built-file check: what the players' WebAssembly may hold, read from the file itself. Mustafa's MUL-ADD (a)
// of 2026-11-24 ("The fused multiply-add stays banned in the simulation; it writes a * b + c") asked for it in the
// pull request that brings the first outside crate; PM-0004-23 asks for a build check that fails on a maths call
// whose result is not the same on every target; PM-0004-25 keeps vector instructions out. The functions below read
// bytes and return what they found; `twinProblems` and `productionProblems` are what the build calls, and the build
// throws on any line of theirs. There is no switch, variable or list entry that skips a rule.

/** A reader over bytes: a read past the end, or a number longer than its type allows, throws with `what` named. */
function reader(bytes, what) {
  let at = 0;
  const fail = (why) => {
    throw new Error(`${what}: ${why} (at byte ${at} of ${bytes.length})`);
  };
  const self = {
    fail,
    done: () => at >= bytes.length,
    rest: () => bytes.subarray(at),
    byte: () => (at < bytes.length ? bytes[at++] : fail("it ends in the middle of a value")),
    /** An unsigned number of at most 32 bits; the padded five-byte form a linker writes is read like the short one. */
    u32() {
      let value = 0;
      for (let k = 0; k < 5; k += 1) {
        const b = self.byte();
        value += (b & 0x7f) * 2 ** (7 * k);
        if ((b & 0x80) === 0) return value <= 0xffff_ffff ? value : fail("a number over 32 bits");
      }
      return fail("a number longer than five bytes");
    },
    /** Steps over one number of at most `most` bytes, whatever its sign. */
    skipNumber(most) {
      for (let k = 0; k < most; k += 1) if ((self.byte() & 0x80) === 0) return;
      fail(`a number longer than ${most} bytes`);
    },
    take(length) {
      if (length > bytes.length - at) fail("it ends in the middle of a value");
      at += length;
      return bytes.subarray(at - length, at);
    },
    text: () => Buffer.from(self.take(self.u32())).toString("utf8"),
  };
  return self;
}

/** A WebAssembly file's sections in its own order: `id`, a custom section's `name`, and `bytes` (what follows). */
function wasmSections(bytes, what) {
  if (bytes.length < 8 || [0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00].some((b, i) => bytes[i] !== b)) {
    throw new Error(`${what}: not a WebAssembly module of version 1`);
  }
  const file = reader(bytes.subarray(8), what);
  const sections = [];
  while (!file.done()) {
    const id = file.byte();
    const content = reader(file.take(file.u32()), `${what}, section ${id}`);
    const name = id === 0 ? content.text() : undefined;
    sections.push({ id, name, bytes: content.rest() });
  }
  return sections;
}

const IMPORT_KINDS = ["function", "table", "memory", "global", "tag"];

/** Every import of a WebAssembly file, in its order: `module`, `name` and `kind` (one of `IMPORT_KINDS`). */
export function wasmImports(bytes, what = "the WebAssembly file") {
  const imports = [];
  for (const section of wasmSections(bytes, what).filter(({ id }) => id === 2)) {
    const r = reader(section.bytes, `${what}, its imports`);
    for (let left = r.u32(); left > 0; left -= 1) {
      const [module, name, kind] = [r.text(), r.text(), r.byte()];
      if (kind === 0) r.u32();
      else if (kind === 1 || kind === 2) {
        if (kind === 1) r.byte();
        const flags = r.byte();
        r.skipNumber(10);
        if ((flags & 1) === 1) r.skipNumber(10);
      } else if (kind === 3) r.take(2);
      else if (kind === 4) {
        r.take(1);
        r.u32();
      } else r.fail(`an import of kind ${kind}`);
      imports.push({ module, name, kind: IMPORT_KINDS[kind] });
    }
    if (!r.done()) r.fail("bytes after the last import");
  }
  return imports;
}

/**
 * The name of every function of a WebAssembly file, by its index, from the file's `name` section. Throws when the
 * file has not exactly one such section or when a function has no name in it: a rule that reads names must not pass
 * because the names are gone.
 */
export function wasmFunctionNames(bytes, what = "the WebAssembly file") {
  const sections = wasmSections(bytes, what);
  const named = sections.filter(({ id, name }) => id === 0 && name === "name");
  if (named.length !== 1) throw new Error(`${what}: ${named.length} name sections, not one: the name rules have nothing to read`);
  const functions =
    wasmImports(bytes, what).filter(({ kind }) => kind === "function").length +
    sections.filter(({ id }) => id === 3).reduce((sum, section) => sum + reader(section.bytes, what).u32(), 0);
  const names = new Array(functions).fill(undefined);
  const r = reader(named[0].bytes, `${what}, its name section`);
  while (!r.done()) {
    const id = r.byte();
    const part = reader(r.take(r.u32()), `${what}, its function names`);
    if (id !== 1) continue;
    for (let left = part.u32(); left > 0; left -= 1) {
      const index = part.u32();
      if (index >= functions) part.fail(`a name for function ${index} of ${functions}`);
      names[index] = part.text();
    }
  }
  const nameless = names.filter((name) => name === undefined || name === "").length;
  if (functions === 0 || nameless > 0) throw new Error(`${what}: ${nameless} of its ${functions} functions have no name: the name rules cannot see them`);
  return names;
}

/** The entries of a WebAssembly file's `target_features` section as the tools write them (`+simd128`). Throws without one. */
export function wasmTargetFeatures(bytes, what = "the WebAssembly file") {
  const found = wasmSections(bytes, what).filter(({ id, name }) => id === 0 && name === "target_features");
  if (found.length !== 1) throw new Error(`${what}: ${found.length} target_features sections, not one`);
  const r = reader(found[0].bytes, `${what}, its target features`);
  const features = [];
  for (let left = r.u32(); left > 0; left -= 1) features.push(String.fromCharCode(r.byte()) + r.text());
  if (!r.done()) r.fail("bytes after the last feature");
  return features;
}

/** The value types a local or a block may have: the four numbers, the vector, and the two reference types. */
const VALUE_TYPES = new Set([0x7f, 0x7e, 0x7d, 0x7c, 0x7b, 0x70, 0x6f]);

/**
 * What follows each instruction this check knows, by opcode: `u` an index, `m` a load's or store's alignment and
 * offset, `i` and `I` a 32-bit and a 64-bit constant, `f` and `F` four and eight bytes of a float, `t` a branch
 * table, `s` a typed select's types, `h` a reference's type. Here are the instructions of WebAssembly 1.0 and of
 * the features `TARGET_FEATURES_ALLOWED` names; an opcode that is not here stops the build.
 */
const IMMEDIATES = new Map([
  ...[0x00, 0x01, 0x05, 0x0f, 0x1a, 0x1b, 0xd1].map((op) => [op, ""]),
  ...[0x0c, 0x0d, 0x10, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x3f, 0x40, 0xd2].map((op) => [op, "u"]),
  ...Array.from({ length: 0x3e - 0x28 + 1 }, (_, k) => [0x28 + k, "m"]),
  ...Array.from({ length: 0xc4 - 0x45 + 1 }, (_, k) => [0x45 + k, ""]),
  ...[[0x0e, "t"], [0x11, "uu"], [0x1c, "s"], [0x41, "i"], [0x42, "I"], [0x43, "f"], [0x44, "F"], [0xd0, "h"]],
]);
/** The same for the instructions behind the prefix 0xFC: the saturating conversions, and the bulk memory and table ones. */
const IMMEDIATES_FC = ["", "", "", "", "", "", "", "", "uu", "u", "uu", "u", "uu", "u", "uu", "u", "u", "u"];

/**
 * Walks every function body of a WebAssembly file, instruction by instruction, and returns how many `bodies` and
 * `instructions` it read and `vector`: for each body that holds a vector instruction (the prefix 0xFD: SIMD, the
 * relaxed kind and its four fused multiply-adds among them), the body's place and the first such instruction's
 * number. The walk reads no further in such a body. A constant's bytes are stepped over, never read as opcodes.
 *
 * Throws, so that nothing unread can pass: an opcode it does not know, a body that does not end exactly at its
 * declared size with every block closed, a type it does not know.
 */
export function wasmCodeOpcodes(bytes, what = "the WebAssembly file") {
  const result = { bodies: 0, instructions: 0, vector: [] };
  for (const section of wasmSections(bytes, what).filter(({ id }) => id === 10)) {
    const code = reader(section.bytes, `${what}, its code`);
    for (let left = code.u32(); left > 0; left -= 1) {
      const place = result.bodies;
      result.bodies += 1;
      const body = reader(code.take(code.u32()), `${what}, function body ${place}`);
      const valueType = (b) => (VALUE_TYPES.has(b) ? b : body.fail(`a type this check does not know (0x${b.toString(16)})`));
      for (let groups = body.u32(); groups > 0; groups -= 1) {
        body.u32();
        valueType(body.byte());
      }
      let vector = false;
      for (let depth = 0; depth >= 0 && !vector; ) {
        const op = body.byte();
        result.instructions += 1;
        if (op === 0x0b) depth -= 1;
        else if (op === 0x02 || op === 0x03 || op === 0x04) {
          depth += 1;
          const type = body.byte();
          if ((type & 0x80) !== 0) body.skipNumber(4);
          else if (type >= 0x40 && type !== 0x40) valueType(type);
        } else if (op === 0xfd) {
          vector = true;
          result.vector.push({ body: place, instruction: body.u32() });
        } else {
          const sub = op === 0xfc ? body.u32() : undefined;
          const follows = sub === undefined ? IMMEDIATES.get(op) : IMMEDIATES_FC[sub];
          if (follows === undefined) body.fail(`an instruction this check does not know (0x${op.toString(16)}${sub === undefined ? "" : ` ${sub}`})`);
          for (const kind of follows) {
            if (kind === "u") body.u32();
            else if (kind === "m") {
              if (body.u32() >= 64) body.fail("a load or store that names a memory");
              body.skipNumber(10);
            }
            else if (kind === "i" || kind === "h") body.skipNumber(5);
            else if (kind === "I") body.skipNumber(10);
            else if (kind === "f" || kind === "F") body.take(kind === "f" ? 4 : 8);
            else if (kind === "t") for (let targets = body.u32() + 1; targets > 0; targets -= 1) body.u32();
            else for (let types = body.u32(); types > 0; types -= 1) valueType(body.byte());
          }
        }
      }
      if (!vector && !body.done()) body.fail("bytes after the function's last end");
    }
    if (!code.done()) code.fail("bytes after the last function body");
  }
  return result;
}

/**
 * Every reading of a function's name as `<length><text>`, at any place in it, and the whole name. Both of Rust's
 * naming schemes write each step of a path that way (`4libm4math4fmaf4fmaf`), so the set holds every step of the
 * path, wherever it stands (a generic function's name ends in the crate that used it, not in its own last step),
 * and some readings that are no step at all: a rule that refuses a step can then refuse too much, never too little.
 */
export function nameSteps(name) {
  const steps = new Set([name]);
  for (let at = 0; at < name.length; at += 1) {
    for (let digits = 1; digits <= 3 && /^[1-9][0-9]*$/.test(name.slice(at, at + digits)) && at + digits <= name.length; digits += 1) {
      const length = Number(name.slice(at, at + digits));
      // The newer scheme writes a `_` between the length and a text that itself starts with `_` or a digit.
      for (const start of name[at + digits] === "_" ? [at + digits, at + digits + 1] : [at + digits]) {
        if (start + length <= name.length) steps.add(name.slice(start, start + length));
      }
    }
  }
  return steps;
}

/** The steps a fused multiply-add is called by: C's names, and the Rust methods and helpers of the crates in the tree. */
export const FUSED_STEPS = ["fma", "fmaf", "fmaf16", "fmaf128", "fmal", "fma_round", "fma_wide_round", "mul_add", "simd_mul_add", "mul_add_assign"];

/**
 * One line for every function whose name holds a fused multiply-add's step (Mustafa's MUL-ADD): the toolchain's
 * `fmaf` and `fma`, the `libm` crate's, and any `mul_add` of any crate. Nothing lifts this rule, the allow-list of
 * maths routines included.
 */
export function fusedMultiplyAddProblems(names) {
  return names.flatMap((name) => {
    const steps = nameSteps(name);
    const found = FUSED_STEPS.filter((step) => steps.has(step));
    return found.length === 0 ? [] : [`${name}: a fused multiply-add (${found.join(", ")}): its bits differ between the native build and WebAssembly, so the simulation writes a * b + c (Mustafa's MUL-ADD)`];
  });
}

/**
 * The C names that std's float methods become in WebAssembly and whose results ADR 0004 part 7 does not promise
 * the same on every target: the transcendental functions, the powers, the float remainder.
 */
// prettier-ignore
export const BANNED_MATHS = [
  "sin", "cos", "tan", "sincos", "asin", "acos", "atan", "atan2", "sinh", "cosh", "tanh", "asinh", "acosh", "atanh", "exp", "exp2",
  "exp10", "expm1", "log", "log2", "log10", "log1p", "pow", "cbrt", "hypot", "erf", "erfc", "tgamma", "lgamma", "lgamma_r", "fmod",
  "remainder",
].flatMap((name) => (name === "lgamma_r" ? [name, "lgammaf_r"] : [name, `${name}f`])).concat(["__powisf2", "__powidf2"]);

/** One line for every function that is one of `BANNED_MATHS` by its whole name (PM-0004-23's build check). */
export function bannedMathsProblems(names) {
  return names.filter((name) => BANNED_MATHS.includes(name)).map((name) => `${name}: a maths function whose result is not the same on every target (ADR 0004 part 7)`);
}

/** The paths a name spells inside `crate`: for each place the crate's own step stands, the steps written right after it. */
function pathsInside(name, crate) {
  const key = `${crate.length}${crate}`;
  const paths = [];
  for (let from = name.indexOf(key); from !== -1; from = name.indexOf(key, from + 1)) {
    if (/[0-9]/.test(name[from - 1] ?? "")) continue;
    const steps = [];
    for (let at = from + key.length; ; ) {
      const length = /^[1-9][0-9]{0,2}/.exec(name.slice(at))?.[0];
      if (length === undefined) break;
      // The newer scheme's `_` between a length and a text that starts with `_` or a digit (`9___powisf2`).
      const start = at + length.length + (name.startsWith("_R") && name[at + length.length] === "_" ? 1 : 0);
      if (start + Number(length) > name.length) break;
      steps.push(name.slice(start, start + Number(length)));
      at = start + Number(length);
    }
    // The older scheme ends a name in a hash, written as one more step.
    if (/^h[0-9a-f]{16}$/.test(steps.at(-1) ?? "")) steps.pop();
    if (steps.length > 0) paths.push(steps.join("::"));
  }
  return paths;
}

/**
 * What a name without a path may be without being a maths routine: the bindings' own names, an export of ours (the
 * bindings end each in sixteen hex digits), and memory's routines.
 */
const PLAIN_NAMES = /^(__wbg_|__wbindgen_|__externref_)|_[0-9a-f]{16}$|^(memcpy|memmove|memset|memcmp|bcmp)$/;

/**
 * The maths routines in `names` that are not on the allow-list, one line each. A maths routine is: any function
 * of the `libm` crate; any function under `math` or `float` of the toolchain's `compiler_builtins`; and any
 * function with a plain name (C's naming) that is neither the bindings' nor one of memory's four. With the
 * collision library's `std` off, its sines, arc cosines and logarithms would arrive as the first kind, under
 * names `BANNED_MATHS` does not hold. `allowed` is `MATHS_ALLOWED` (maths-allowed.mjs): each entry names a routine
 * as `libm::math::sqrtf`, `compiler_builtins::math::libm_math::round` or `C::round`, and covers the steps below it.
 */
export function libmProblems(names, allowed) {
  const routines = allowed.map(({ routine }) => routine);
  const listed = (routine) => routines.some((entry) => routine === entry || routine.startsWith(`${entry}::`));
  return names.flatMap((name) => {
    const mangled = name.startsWith("_R") || name.startsWith("_ZN");
    const found = [
      ...pathsInside(name, "libm").map((path) => `libm::${path}`),
      ...pathsInside(name, "compiler_builtins").filter((path) => /^(math|float)(::|$)/.test(path)).map((path) => `compiler_builtins::${path}`),
      // A banned function and a fused multiply-add are refused by their own rules, which no list lifts.
      ...(mangled || PLAIN_NAMES.test(name) || BANNED_MATHS.includes(name) || FUSED_STEPS.includes(name) ? [] : [`C::${name}`]),
    ];
    return found.filter((routine) => !listed(routine)).map((routine) => `${name}: ${routine} is a maths routine outside the allow-list (packages/client/scripts/maths-allowed.mjs)`);
  });
}

/** The target features the players' WebAssembly is built with on the pinned toolchain: these, each switched on, and no other. */
// prettier-ignore
export const TARGET_FEATURES_ALLOWED = [
  "+bulk-memory", "+bulk-memory-opt", "+call-indirect-overlong", "+multivalue", "+mutable-globals", "+nontrapping-fptoint",
  "+reference-types", "+sign-ext",
];

/** What `wasmCodeOpcodes` and `wasmTargetFeatures` hold against a file: a vector instruction, a feature off the list. */
function instructionProblems(bytes, what, names) {
  const walked = wasmCodeOpcodes(bytes, what);
  const first = names === undefined ? 0 : names.length - walked.bodies;
  return [
    ...(walked.instructions > 0 ? [] : [`${what}: no instruction was read`]),
    ...walked.vector.map(({ body, instruction }) => {
      const fused = instruction >= 261 && instruction <= 264 ? ", a fused multiply-add of the relaxed kind (Mustafa's MUL-ADD)" : "";
      return `${what}: function body ${body}${names === undefined ? "" : ` (${names[first + body]})`} holds vector instruction 0xFD ${instruction}${fused}: the players' file is built without SIMD (PM-0004-25)`;
    }),
    ...wasmTargetFeatures(bytes, what).filter((feature) => !TARGET_FEATURES_ALLOWED.includes(feature)).map((feature) => `${what}: the target feature ${feature} is not one the players' file is built with`),
  ];
}

/** The steps that show a twin's names are whole: our own crate's and the standard library's. */
const SENTINEL_STEPS = ["sim_wasm", "core"];

/**
 * Every rule that reads the twin: cargo's own output for the players' WebAssembly, with its names, before the
 * bindings tool turns it into the file players get (the tool removes and renames, and cannot add a routine).
 * `what` names the twin in each line: the release one, and the dev-profile one, where nothing is inlined away.
 */
export function twinProblems(bytes, what, allowed) {
  const names = wasmFunctionNames(bytes, what);
  const steps = new Set(names.flatMap((name) => [...nameSteps(name)]));
  return [
    ...SENTINEL_STEPS.filter((step) => !steps.has(step)).map((step) => `${what}: no function's name holds \`${step}\`: these are not the names the rules read`),
    ...fusedMultiplyAddProblems(names).map((line) => `${what}: ${line}`),
    ...bannedMathsProblems(names).map((line) => `${what}: ${line}`),
    ...libmProblems(names, allowed).map((line) => `${what}: ${line}`),
    ...instructionProblems(bytes, what, names),
  ];
}

/** The players' imports, to the letter: the two calls the bindings make (an error's text, a throw) and their table's set-up. */
export const PLAYERS_IMPORTS = ["__wbg_Error_208446807d0d1e30", "__wbg___wbindgen_throw_4c88d8bf264fd984", "__wbindgen_init_externref_table"].map((name) => ({
  module: "./sim_wasm_bg.js",
  name,
  kind: "function",
}));

/**
 * Every rule that reads the file players get, after the bindings tool: the instruction walk again (what ships is
 * what is walked); its imports, which are `PLAYERS_IMPORTS` and nothing else, so that no crate can add a call to
 * the host unseen (a clock, randomness, a maths function of JavaScript's); and no path of the machine that built
 * it (`machinePaths`: the builder's home, cargo's folder, the repository), which also shows the path remap held.
 */
export function productionProblems(bytes, machinePaths) {
  const what = "the players' WebAssembly";
  const same = (a, b) => a.module === b.module && a.name === b.name && a.kind === b.kind;
  const imports = wasmImports(bytes, what);
  const text = Buffer.from(bytes).toString("latin1");
  const paths = ["/home/", "/root/", "/Users/", "\\Users\\", ...machinePaths.filter((entry) => entry.length > 1)];
  return [
    ...instructionProblems(bytes, what),
    ...imports.filter((entry) => !PLAYERS_IMPORTS.some((known) => same(known, entry))).map((entry) => `${what} imports the ${entry.kind} ${entry.module} ${entry.name}, which is not one of the bindings' own ${PLAYERS_IMPORTS.length}`),
    ...PLAYERS_IMPORTS.filter((known) => imports.filter((entry) => same(known, entry)).length !== 1).map((known) => `${what} does not import ${known.name} exactly once`),
    ...[...new Set(paths)].filter((entry) => text.includes(entry)).map((entry) => `${what} holds a path of the machine that built it (${entry})`),
  ];
}

/** The stamp of one clean walk: the sha256 of each file the rules read, the players' own among them. */
export function walkedStamp({ twin, devTwin, production }) {
  const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");
  return `${JSON.stringify({ twin: sha256(twin), devTwin: sha256(devTwin), production: sha256(production) })}\n`;
}

/**
 * Why a page may not be built from `playersWasm`: it is not the file the stamp says the built-file check walked
 * (`stampText` is the stamp file's text, or nothing when there is no stamp). Empty when the stamp is this file's.
 */
export function stampProblems(stampText, playersWasm) {
  let walked;
  try {
    walked = JSON.parse(stampText ?? "null")?.production;
  } catch {
    walked = undefined;
  }
  if (typeof walked !== "string") return ["the built-file check left no stamp: the players' WebAssembly was not walked"];
  const found = createHash("sha256").update(playersWasm).digest("hex");
  return found === walked ? [] : [`the players' WebAssembly (sha256 ${found}) is not the file the built-file check walked (${walked})`];
}
