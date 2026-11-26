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
 * among them every `RUSTC_*`, and the names through which a stable cargo takes nightly-only features (`__CARGO_*`,
 * `CARGO_UNSTABLE_*`). Not refused: the two that name the build folder, which the build's `--target-dir` overrides.
 * Not covered: cargo's config files outside the repository, through which a profile setting (overflow checks among
 * them), a linker, a compiler wrapper or an `[env]` table can still be set with no diff here (the quality gate's
 * flag check); only their `rustflags` are shut out, by the build itself, which gives cargo its own (build.mjs,
 * `cargoEnv`).
 */
export const CARGO_OVERRIDES = new RegExp(
  "^(?!CARGO_(BUILD_)?TARGET_DIR$)(RUSTFLAGS|CARGO_ENCODED_RUSTFLAGS|RUSTC(_.*)?|RUSTUP_TOOLCHAIN|CARGO_INCREMENTAL|" +
    "__CARGO_.*|CARGO_(BUILD|PROFILE|RESOLVER|TARGET|UNSTABLE)_.*|WASM_BINDGEN_.*|RUST_LOG)$",
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
