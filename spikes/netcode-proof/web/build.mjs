#!/usr/bin/env node
// build.mjs: U7's web build (u7-brief.md rev 1.1 Block T1; u8-build.md F14). RS runs it in web/ as
// `npm_config_ignore_scripts=true CARGO_NET_OFFLINE=true node build.mjs` after `npm ci --ignore-scripts`.
//   1. `wasm-bindgen --version` must print 0.2.130, else exit 2 (the binary: NPT_WASM_BINDGEN, else PATH's);
//   2. `cargo build --locked --offline --release --target wasm32-unknown-unknown -p npt-client-wasm` in its own target
//      folder under .gate/, with CARGO_HOME, the workspace and that target remapped, so RS and here make the same bytes;
//   3. `wasm-bindgen --target web --no-typescript` (no wasm-opt); the Wasm is scanned for absolute home paths;
//   4. `tsc -p .` into .gate/;
//   5. exactly seven files into a fresh dist folder, each SHA-256 printed.
// Options: `--bites` writes the bites dist (web/bites/hooks.js in place of hooks.js) to
// <checkout>/.gate/netcode-proof-test/dist-bites/ (C13); `--out <dir>` writes the measured dist under that .gate/
// folder instead of web/dist/ (a dist under web/ fails check.mjs's redact step: N-U7D-1). No shell, no npx.
import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { homedir } from "node:os";
import { dirname, join, relative, resolve, sep } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

export const WEB = dirname(fileURLToPath(import.meta.url));
export const SPIKE = resolve(WEB, "..");
export const ROOT = resolve(WEB, "..", "..", "..");
export const GATE = join(ROOT, ".gate", "netcode-proof-test");
export const WASM_BINDGEN_VERSION = "0.2.130";
/** The dist names U6c's page host compiles in (u6c-brief.md:241-246), in this order. */
export const DIST_NAMES = Object.freeze([
  "index.html",
  "main.js",
  "worker.js",
  "lib.js",
  "hooks.js",
  "npt_client_wasm.js",
  "npt_client_wasm_bg.wasm",
]);
const FROM_TSC = ["main.js", "worker.js", "lib.js", "hooks.js"];
const FROM_BINDGEN = ["npt_client_wasm.js", "npt_client_wasm_bg.wasm"];

/** `--bites` and `--out <dir>`; anything else is a usage error. */
export function parseArgs(argv) {
  const a = { bites: false, out: null };
  for (let i = 0; i < argv.length; i++) {
    if (argv[i] === "--bites") a.bites = true;
    else if (argv[i] === "--out" && i + 1 < argv.length) a.out = argv[++i];
    else throw new Error(`unknown argument ${JSON.stringify(argv[i])}`);
  }
  if (a.bites && a.out) throw new Error("--bites writes to its own folder; --out is for the measured dist");
  return a;
}

function inside(dir, parent) {
  const r = relative(parent, dir);
  return r !== "" && !r.startsWith("..") && !r.startsWith(sep) && resolve(parent, r) === dir;
}

/** The dist folder: web/dist (measured), .gate/netcode-proof-test/dist-bites (bites), or `--out` under that .gate/. */
export function outDirFor({ bites = false, out = null } = {}) {
  if (bites) return join(GATE, "dist-bites");
  if (out === null) return join(WEB, "dist");
  const d = resolve(out);
  if (!inside(d, GATE)) throw new Error("--out must name a folder under .gate/netcode-proof-test/");
  return d;
}

/** True when `wasm-bindgen --version` printed exactly the pinned version. */
export function wasmBindgenOk(text) {
  return String(text).trim() === `wasm-bindgen ${WASM_BINDGEN_VERSION}`;
}

/** CARGO_ENCODED_RUSTFLAGS: CARGO_HOME, the workspace and the Wasm's target folder mapped to fixed names. */
export function encodedRustFlags(cargoHome, spike, target) {
  return [
    `--remap-path-prefix=${cargoHome}=/cargo-home`,
    `--remap-path-prefix=${spike}=/npt`,
    `--remap-path-prefix=${target}=/npt-target`,
  ].join("\x1f");
}

/** The absolute home-like paths `bytes` holds: the generic home roots and each named folder. */
export function homePaths(bytes, folders = []) {
  const text = Buffer.from(bytes).toString("latin1");
  const needles = ["/home/", "/root/", "/Users/", "C:\\Users\\", ...folders.filter((f) => f && f.length > 1)];
  return needles.filter((n) => text.includes(n));
}

/**
 * What the seven assembled files must not hold (U7R-TE-9): `innerHTML` in any (dynamic text by textContent only), and a
 * bite id (`PM-0001-`) in the measured dist (only the bites dist's hooks.js lists them). `files` are [name, bytes].
 */
export function distProblems(files, bites) {
  const p = [];
  for (const [name, bytes] of files) {
    const text = Buffer.from(bytes).toString("latin1");
    if (text.includes("innerHTML")) p.push(`${name} holds innerHTML`);
    if (!bites && text.includes("PM-0001-")) p.push(`${name} holds a bite id in the measured dist`);
  }
  return p;
}

function sha256(file) {
  return createHash("sha256").update(readFileSync(file)).digest("hex");
}

/**
 * Copies the seven files into a fresh `out` (removed first; only web/dist or a folder under .gate/netcode-proof-test/)
 * and returns each name with its SHA-256. `jsDir` holds tsc's four files, `bindgenDir` wasm-bindgen's two; `hooks`
 * replaces tsc's hooks.js when given.
 */
export function assemble({ out, index, jsDir, bindgenDir, hooks = null }) {
  const d = resolve(out);
  if (d !== join(WEB, "dist") && !inside(d, GATE)) throw new Error("a dist goes to web/dist or under .gate/ only");
  const from = new Map([["index.html", index]]);
  for (const n of FROM_TSC) from.set(n, join(jsDir, n));
  for (const n of FROM_BINDGEN) from.set(n, join(bindgenDir, n));
  if (hooks !== null) from.set("hooks.js", hooks);
  for (const [n, f] of from) if (!existsSync(f)) throw new Error(`missing ${n}`);
  rmSync(d, { recursive: true, force: true });
  mkdirSync(d, { recursive: true });
  const sums = [];
  for (const n of DIST_NAMES) {
    copyFileSync(from.get(n), join(d, n));
    sums.push({ name: n, sha256: sha256(join(d, n)) });
  }
  const got = readdirSync(d).sort();
  if (got.join("\n") !== [...DIST_NAMES].sort().join("\n")) throw new Error("the dist is not the seven names");
  return sums;
}

function run(cmd, args, env = process.env, cwd = SPIKE) {
  return execFileSync(cmd, args, { cwd, env, encoding: "utf8", stdio: ["ignore", "pipe", "inherit"] });
}

function main() {
  let args;
  try {
    args = parseArgs(process.argv.slice(2));
  } catch (e) {
    console.error(`build.mjs: ${e.message}; usage: node build.mjs [--bites | --out <dir under .gate/netcode-proof-test>]`);
    process.exit(2);
  }
  const out = outDirFor(args);
  const bindgen = process.env.NPT_WASM_BINDGEN || "wasm-bindgen";
  let version = "";
  try {
    version = run(bindgen, ["--version"]);
  } catch {
    version = "";
  }
  if (!wasmBindgenOk(version)) {
    console.error(`build.mjs: wasm-bindgen ${WASM_BINDGEN_VERSION} is required (read: ${version.trim().slice(0, 60) || "none"})`);
    process.exit(2);
  }
  const target = join(GATE, "wasm-target");
  const work = join(GATE, args.bites ? "web-build-bites" : "web-build");
  const cargoHome = resolve(process.env.CARGO_HOME || join(homedir(), ".cargo"));
  const env = { ...process.env, CARGO_NET_OFFLINE: "true", CARGO_ENCODED_RUSTFLAGS: encodedRustFlags(cargoHome, SPIKE, target) };
  delete env.RUSTFLAGS;
  run("cargo", ["build", "--locked", "--offline", "--release", "--target", "wasm32-unknown-unknown", "-p", "npt-client-wasm",
    "--target-dir", target], env);
  rmSync(work, { recursive: true, force: true });
  const bindgenDir = join(work, "bindgen");
  const jsDir = join(work, "js");
  run(bindgen, ["--target", "web", "--no-typescript", "--out-dir", bindgenDir,
    join(target, "wasm32-unknown-unknown", "release", "npt_client_wasm.wasm")]);
  const found = homePaths(readFileSync(join(bindgenDir, "npt_client_wasm_bg.wasm")), [cargoHome, SPIKE, ROOT, homedir()]);
  if (found.length > 0) {
    console.error(`build.mjs: the Wasm holds ${found.length} absolute home path(s); the remap missed one`);
    process.exit(1);
  }
  run(process.execPath, [join(WEB, "node_modules", "typescript", "bin", "tsc"), "-p", WEB, "--outDir", jsDir], process.env, WEB);
  const sums = assemble({ out, index: join(WEB, "index.html"), jsDir, bindgenDir,
    hooks: args.bites ? join(WEB, "bites", "hooks.js") : null });
  const problems = distProblems(DIST_NAMES.map((n) => [n, readFileSync(join(out, n))]), args.bites);
  if (problems.length > 0) {
    console.error(`build.mjs: ${problems.join("; ")}`);
    process.exit(1);
  }
  console.log(`build.mjs: ${args.bites ? "bites" : "measured"} dist, ${sums.length} files`);
  for (const s of sums) console.log(`${s.sha256}  ${s.name}`);
}

if (process.argv[1] && pathToFileURL(resolve(process.argv[1])).href === import.meta.url) main();
