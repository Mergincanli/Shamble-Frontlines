// The build of the player's page, from the repository's root:
//
//   SHAMBLE_WASM_BINDGEN=<absolute path of the wasm-bindgen tool> node packages/client/scripts/build.mjs
//       [--wasm-only] [--production] [--out-dir <folder>]
//
// It builds the simulation as WebAssembly and writes its bindings into packages/sim-worker/pkg/ (build output,
// ignored by git), then builds the page with Vite into packages/client/dist/ (or --out-dir). Without --production
// the page is the development flavour: it has the readout and the copy report, and the zip's small server is
// copied beside it. Every command runs as a list of arguments, never through a shell.
//
// Every build but a --production one also makes the test-only WebAssembly module (crates/devtools, in its release
// and its dev profile) into .gate/rules-module/ at the repository's root, where the rule tests load it
// (boundary/rules.test.mjs, the e2e rules spec). That folder lies outside every package: no folder a server reads
// holds it, and no link in node_modules leads to it. It is built in a cargo call and a target folder of its own,
// nothing of the page or the Worker imports it, and a built page that held it would be refused (build-checks.mjs,
// wasmProblems).
//
// The wasm-bindgen tool is a ready-built program from the internet. Before it is run at all, `--version` included,
// its file must be the one wasm-bindgen-tool.mjs records (a plain file of that size and sha256), and its version
// must equal the wasm-bindgen library in Cargo.lock (PM-0002-18). The path that was checked is the path that is
// run: the tool is never looked up by name.
//
// The built-file check (build-checks.mjs explains each rule) reads what was built, on every build: cargo's own
// output with its names, in the release profile and in the dev profile, before the bindings tool runs (no fused
// multiply-add, no maths routine outside the allow-list, no vector instruction), and then the file players get (the
// instructions again, its imports, no path of this machine). A clean walk leaves a stamp beside that file, and no
// page is built from a file the stamp does not name. Nothing skips it.
//
// Every page carries the game's content as one bundle, `content.json` beside `index.html` (the sheet's row 9): the
// content tool (`crates/devtools`, `content_bundle`) reads `content/`, asks the game's own loader about each file and
// the whole, and writes the bundle only when the loader takes it. It runs in a cargo call and a target folder of its
// own, after Vite has built (and emptied) the page's folder; a page folder without the bundle is refused.

import { execFileSync } from "node:child_process";
import { copyFileSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { build as viteBuild } from "vite";
import {
  bundleProblems,
  checkCli,
  checkCliFile,
  checkOutDir,
  contentProblems,
  cspProblems,
  formatSizeTable,
  lockedWasmBindgenVersion,
  markupSinkProblems,
  missingOverflowOverrides,
  NATIVE_TESTS,
  plainFilesOf,
  productionProblems,
  refusedBuildSettings,
  registryRemapFlag,
  SCANNED_SOURCES,
  simFeatureDifferences,
  simFeatureList,
  sizeTable,
  sourceTexts,
  stampProblems,
  tooLarge,
  twinProblems,
  walkedStamp,
  wasmCodeOpcodes,
  wasmProblems,
  workerEntryChunk,
} from "./build-checks.mjs";
import { MATHS_ALLOWED } from "./maths-allowed.mjs";
import { WASM_BINDGEN_TOOL } from "./wasm-bindgen-tool.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../..");
const clientDir = path.join(root, "packages/client");
const pkgDir = path.join(root, "packages/sim-worker/pkg");
/** The Worker's entry module: the chunk the build makes from it is the only one that may hold the Wasm's glue. */
const workerSource = path.join(root, "packages/sim-worker/src/worker.ts");
const targetDir = path.join(root, "target");
const wasmFile = path.join(targetDir, "wasm32-unknown-unknown/release/sim_wasm.wasm");
/** The same crate in the dev profile: built only to be read, where no call is inlined away and every name stands. */
const devWasmFile = path.join(targetDir, "wasm32-unknown-unknown/debug/sim_wasm.wasm");
const wasmTarget = "wasm32-unknown-unknown";
/** The players' WebAssembly as the Worker loads it, and the stamp the built-file check leaves beside it. */
const playersWasmFile = path.join(pkgDir, "sim_wasm_bg.wasm");
const stampFile = path.join(pkgDir, "built-file-check.json");
/**
 * The environment of every cargo call this script makes. Its one addition: the compiler flags, which are the path
 * remap and nothing else (panic locations inside registry crates name one fixed place, not this machine's cargo
 * folder, so one commit gives the same Wasm bytes wherever it is built: RR-F5). Given this way, cargo takes its
 * compiler flags from here alone: `rustflags` in a cargo config file outside the repository do not reach the
 * compiler. What such a file can still set is not shut out here: a linker, a compiler wrapper, an `[env]` table,
 * a profile (the quality gate's flag check). The same name coming from outside is refused before any cargo call
 * (refuseSettings).
 */
const cargoEnv = () => ({ ...process.env, CARGO_ENCODED_RUSTFLAGS: registryRemapFlag(process.env["CARGO_HOME"], os.homedir(), root) });
/**
 * The test-only module: its bindings' folder, outside every package (so outside what a server reads and what
 * node_modules links to), and a target folder that is never the players' Wasm's.
 */
const rulesDir = path.join(root, ".gate/rules-module");
const rulesTargetDir = path.join(root, ".gate/target-rules");
/** The content tool's target folder: its own, so building a page never rebuilds what the tests built, or the reverse. */
const contentTargetDir = path.join(root, ".gate/target-content");
/**
 * Where the development server's bundle is written (vite.config.mjs): outside every package and every folder a
 * server serves, and outside `content/`, as the content tool asks.
 */
export const DEV_CONTENT_DIR = path.join(root, ".gate/dev-content");
/** The tool's absolute path, after its file was compared with the recorded values. */
function checkedTool() {
  const tool = process.env["SHAMBLE_WASM_BINDGEN"];
  if (tool === undefined || !path.isAbsolute(tool)) {
    throw new Error("SHAMBLE_WASM_BINDGEN must be the absolute path of the wasm-bindgen tool");
  }
  checkCliFile(tool, WASM_BINDGEN_TOOL.program.sha256, WASM_BINDGEN_TOOL.program.bytes);
  return tool;
}

/** Stops the build when the environment holds a setting of cargo's or the tool's (build-checks.mjs names them). */
function refuseSettings() {
  const overrides = refusedBuildSettings(process.env);
  if (overrides.length > 0) throw new Error(`build settings from the environment are refused: unset ${overrides.join(", ")}`);
}

/**
 * Two refusals read from cargo's own resolution, before anything is compiled (build-checks.mjs explains each): a
 * third-party crate that would be linked into the players' Wasm with overflow checks off, and a build that resolves
 * the simulation's features differently from another build that runs it.
 */
function checkCrates() {
  const tree = (...args) => execFileSync("cargo", ["tree", "--locked", ...args], { cwd: root, encoding: "utf8", env: cargoEnv() });
  const unchecked = missingOverflowOverrides(
    tree("-p", "sim-wasm", "--target", wasmTarget, "-e", "normal,no-proc-macro", "--prefix", "none"),
    readFileSync(path.join(root, "Cargo.toml"), "utf8"),
    root,
  );
  if (unchecked.length > 0) {
    throw new Error(
      `the players' Wasm would hold these crates with overflow checks off: ${unchecked.join(", ")}. ` +
        "Give each its [profile.release.package.<name>] block with overflow-checks = true in the root Cargo.toml",
    );
  }
  const features = (edges, ...args) => tree(...args, "-e", `features,${edges},no-proc-macro`, "--prefix", "depth", "--no-dedupe");
  const differences = simFeatureDifferences({
    "the players' Wasm": simFeatureList(features("normal", "-p", "sim-wasm", "--target", wasmTarget)),
    "the test-only module": simFeatureList(features("normal", "-p", "devtools", "--target", wasmTarget)),
    // As `cargo test --workspace` builds it: dev-dependencies' features count, the simulation's own test helpers do not.
    [NATIVE_TESTS]: simFeatureList(features("normal,dev", "--workspace"), features("normal", "--workspace")),
  });
  if (differences.length > 0) throw new Error(differences.join("\n"));
}

/**
 * Builds the test-only module in both profiles and writes each one's bindings into its own folder under
 * .gate/rules-module/. The same refusals and the same checked tool as the players' Wasm; its own cargo
 * call and target folder, so nothing it compiles can stand where the players' Wasm is read from.
 */
export function buildRulesModule() {
  for (const [profile, flags, folder] of [["release", ["--release"], "release"], ["dev", [], "debug"]]) {
    refuseSettings();
    execFileSync(
      "cargo",
      ["build", "--locked", ...flags, "--target", wasmTarget, "-p", "devtools", "--lib", "--target-dir", rulesTargetDir],
      { cwd: root, stdio: "inherit", env: cargoEnv() },
    );
    const outDir = path.join(rulesDir, profile);
    rmSync(outDir, { recursive: true, force: true });
    mkdirSync(outDir, { recursive: true });
    const built = path.join(rulesTargetDir, wasmTarget, folder, "devtools.wasm");
    execFileSync(checkedTool(), ["--target", "web", "--out-dir", outDir, built], { cwd: root, stdio: "inherit" });
  }
}

/** Builds the Wasm and writes its bindings. Returns the size table of what the Worker loads. */
export function buildWasm() {
  const locked = lockedWasmBindgenVersion(readFileSync(path.join(root, "Cargo.lock"), "utf8"));
  if (locked !== WASM_BINDGEN_TOOL.version) {
    throw new Error(`Cargo.lock records wasm-bindgen ${locked} and wasm-bindgen-tool.mjs the tool ${WASM_BINDGEN_TOOL.version}`);
  }
  checkCli(execFileSync(checkedTool(), ["--version"], { encoding: "utf8" }), locked);
  // The Wasm players get is built with no setting from the environment (the overflow checks, the optimisation level,
  // the target's features), and into this repository's own target folder, which is where it is read from below.
  refuseSettings();
  checkCrates();
  // An earlier walk's stamp goes first: until this walk is clean, no page can be built from what pkg/ holds.
  rmSync(stampFile, { force: true });
  const refuse = (problems) => {
    if (problems.length > 0) throw new Error(`the built-file check refuses the players' WebAssembly:\n  ${problems.join("\n  ")}`);
  };
  for (const profile of [["--release"], []]) {
    execFileSync(
      "cargo",
      ["build", "--locked", ...profile, "--target", wasmTarget, "-p", "sim-wasm", "--target-dir", targetDir],
      { cwd: root, stdio: "inherit", env: cargoEnv() },
    );
  }
  // The name rules read the twins before the bindings tool: a file the tool itself would refuse fails here, by its rule.
  const twin = readFileSync(wasmFile);
  const devTwin = readFileSync(devWasmFile);
  refuse([...twinProblems(twin, "the release twin", MATHS_ALLOWED), ...twinProblems(devTwin, "the dev-profile twin", MATHS_ALLOWED)]);
  mkdirSync(pkgDir, { recursive: true });
  // Checked again: the cargo build above ran build scripts as this same user.
  execFileSync(checkedTool(), ["--target", "web", "--out-dir", pkgDir, wasmFile], { cwd: root, stdio: "inherit" });
  const production = readFileSync(playersWasmFile);
  const cargoHome = process.env["CARGO_HOME"];
  refuse(productionProblems(production, [os.homedir(), path.resolve(root, cargoHome === undefined || cargoHome === "" ? path.join(os.homedir(), ".cargo") : cargoHome), root]));
  writeFileSync(stampFile, walkedStamp({ twin, devTwin, production }));
  console.log(`the built-file check: ${wasmCodeOpcodes(production).instructions} instructions of the players' WebAssembly walked, and both twins' names read\n`);
  const rows = sizeTable(
    readdirSync(pkgDir)
      .filter((name) => name.endsWith(".wasm") || name.endsWith(".js"))
      .sort()
      .map((name) => ({ name: `pkg/${name}`, bytes: readFileSync(path.join(pkgDir, name)) })),
  );
  const large = tooLarge(rows);
  if (large.length > 0) throw new Error(`a served file reaches 25 MiB: ${large.map((row) => row.name).join(", ")}`);
  return rows;
}

/**
 * Writes the content bundle into `outDir` (the built page's folder, or the development server's): the content tool
 * asks the game's own loader about every file under `content/` and the whole, and writes `content.json` only when the
 * loader takes them (it refuses a folder inside `content/`). The same refusals of settings from the environment as
 * every cargo call here, and the same call, environment and target folder for both callers, so a page built once
 * leaves the development server nothing to compile.
 */
export function writeContentBundle(outDir) {
  refuseSettings();
  execFileSync(
    "cargo",
    ["run", "--quiet", "--locked", "-p", "devtools", "--bin", "content_bundle", "--target-dir", contentTargetDir, "--", outDir],
    { cwd: root, stdio: "inherit", env: cargoEnv() },
  );
}

/** The commit the page is built from, marked when the tree holds changes that are not committed. */
function commitName() {
  const git = (...args) => execFileSync("git", args, { cwd: root, encoding: "utf8" }).trim();
  const changed = git("status", "--porcelain", "--", "crates", "packages", "Cargo.toml", "Cargo.lock", "package.json", "package-lock.json");
  return git("rev-parse", "--short=9", "HEAD") + (changed === "" ? "" : "+changes");
}

/** The stamp file's text, or nothing when the built-file check left none. */
function stampText() {
  try {
    return readFileSync(stampFile, "utf8");
  } catch {
    return undefined;
  }
}

/** Builds the page into `outDir` and checks every served file. Returns their size table. */
export async function buildPage({ production, outDir }) {
  // Before anything is built: no source of the page or the Worker turns a text into markup or code.
  const sinks = markupSinkProblems(sourceTexts(root, SCANNED_SOURCES));
  if (sinks.length > 0) throw new Error(`the page's sources are refused:\n  ${sinks.join("\n  ")}`);
  process.env["VITE_SHAMBLE_COMMIT"] = commitName();
  if (production) delete process.env["VITE_SHAMBLE_DEVTOOLS"];
  else process.env["VITE_SHAMBLE_DEVTOOLS"] = "1";
  // The Worker's bundle reports its own chunks: Vite builds it apart from the page's, with these plugins beside the
  // settings of vite.config.mjs.
  const workerChunks = [];
  await viteBuild({
    root: clientDir,
    configFile: path.join(clientDir, "vite.config.mjs"),
    logLevel: "warn",
    build: { outDir, emptyOutDir: true },
    worker: {
      plugins: () => [
        {
          name: "shamble-worker-chunks",
          generateBundle(_options, bundle) {
            for (const chunk of Object.values(bundle)) if (chunk.type === "chunk") workerChunks.push(chunk);
          },
        },
      ],
    },
  });
  const workerEntry = workerEntryChunk(workerChunks, workerSource);
  // The zip's small server is a development tool: it is not part of a production build.
  if (!production) copyFileSync(path.join(clientDir, "serve.mjs"), path.join(outDir, "serve.mjs"));
  // The game's content, judged by its own loader, beside index.html.
  writeContentBundle(outDir);
  // The built page, file by file: a folder that holds a link or anything else that is no plain file stops here.
  const files = plainFilesOf(outDir)
    .filter((name) => name !== "serve.mjs")
    .map((name) => ({ name, bytes: readFileSync(path.join(outDir, name)) }));
  const rows = sizeTable(files);
  const playersWasm = readFileSync(playersWasmFile);
  const problems = [
    ...tooLarge(rows).map((row) => `${row.name}: a served file reaches 25 MiB`),
    ...cspProblems(files),
    ...bundleProblems(files, production, workerEntry),
    // The page's WebAssembly is the Worker's file, and the Worker's file is the one the built-file check walked.
    ...stampProblems(stampText(), playersWasm),
    ...wasmProblems(files, playersWasm),
    ...contentProblems(files),
  ];
  if (problems.length > 0) throw new Error(`the built page is refused:\n  ${problems.join("\n  ")}`);
  return rows;
}

async function main() {
  const args = process.argv.slice(2);
  const at = args.indexOf("--out-dir");
  const outDir = at === -1 ? path.join(clientDir, "dist") : path.resolve(args[at + 1] ?? "");
  const unknown = args.filter((arg, i) => !["--wasm-only", "--production", "--out-dir"].includes(arg) && i !== at + 1);
  if (unknown.length > 0 || (at !== -1 && args[at + 1] === undefined)) throw new Error(`unknown or unfinished option: ${args.join(" ")}`);
  checkOutDir(outDir, path.join(clientDir, "dist"));
  console.log(formatSizeTable(buildWasm()));
  const production = args.includes("--production");
  if (!production) {
    buildRulesModule();
    console.log(`\nthe test-only module (never shipped) in ${path.relative(root, rulesDir)}: release and dev`);
  }
  if (args.includes("--wasm-only")) return;
  console.log(`\n${production ? "production" : "development"} page in ${path.relative(root, outDir)}:`);
  console.log(formatSizeTable(await buildPage({ production, outDir })));
}

if (process.argv[1] !== undefined && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  main().catch((error) => {
    console.error(`build: ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  });
}
