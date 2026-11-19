// The build script's checks, each a small function with no side effect beyond reading the file it is given, so
// each is tested without a build (build-checks.test.mjs). The build itself (scripts/build.mjs) calls them.

import { createHash } from "node:crypto";
import { lstatSync, readFileSync } from "node:fs";
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
const DEV_MARKERS = ["Copy report", "__shambleReport", "ticks while hidden", "first page: report"];

/**
 * Two rules of the built page, one line per breach. The Wasm's glue is in the Worker's chunk and in no other (the
 * page never touches the Wasm); and a production build holds none of the development-only code, while a
 * development build holds all of it (so the markers themselves are known to be right).
 * @param {{ name: string, bytes: Uint8Array }[]} files
 */
export function bundleProblems(files, production) {
  const scripts = files.filter(({ name }) => name.endsWith(".js")).map(({ name, bytes }) => ({ name, text: Buffer.from(bytes).toString("utf8") }));
  const page = scripts.filter(({ name }) => !/(^|\/)worker-[^/]*\.js$/.test(name));
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
 * Refuses an output folder the build must not empty. Vite clears the folder it builds into, so the folder is either
 * the page's own `dist` or one whose name starts with `dist-`: a mistyped `--out-dir` cannot wipe sources or notes.
 */
export function checkOutDir(outDir, defaultDir) {
  if (outDir !== defaultDir && !/^dist-[\w.-]+$/.test(outDir.split(/[\\/]/).pop() ?? "")) {
    throw new Error(`--out-dir must be a folder named dist-<something>, because the build empties it: ${outDir}`);
  }
}

/**
 * Variables through which cargo's or the tool's settings can be changed from outside with no diff in the repository.
 * Not refused: the two that name the build folder, which the build's `--target-dir` overrides. Not covered: cargo's
 * config files outside the repository (the quality gate's flag check).
 */
export const CARGO_OVERRIDES = new RegExp(
  "^(?!CARGO_(BUILD_)?TARGET_DIR$)(RUSTFLAGS|CARGO_ENCODED_RUSTFLAGS|RUSTC|RUSTC_(WORKSPACE_)?WRAPPER|RUSTUP_TOOLCHAIN|" +
    "CARGO_(BUILD|PROFILE|TARGET)_.*|WASM_BINDGEN_.*|RUST_LOG)$",
);

/** The names in `env` that the Wasm build refuses, in the environment's order. Only names are read, never a value. */
export function refusedBuildSettings(env) {
  return Object.keys(env).filter((name) => CARGO_OVERRIDES.test(name));
}

