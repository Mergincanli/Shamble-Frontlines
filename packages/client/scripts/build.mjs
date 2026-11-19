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
// The wasm-bindgen tool is a ready-built program from the internet. Before it is run at all, `--version` included,
// its file must be the one wasm-bindgen-tool.mjs records (a plain file of that size and sha256), and its version
// must equal the wasm-bindgen library in Cargo.lock (PM-0002-18). The path that was checked is the path that is
// run: the tool is never looked up by name.

import { execFileSync } from "node:child_process";
import { copyFileSync, mkdirSync, readdirSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { build as viteBuild } from "vite";
import { checkCli, checkCliFile, cspProblems, formatSizeTable, lockedWasmBindgenVersion, sizeTable, tooLarge } from "./build-checks.mjs";
import { WASM_BINDGEN_TOOL } from "./wasm-bindgen-tool.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../..");
const clientDir = path.join(root, "packages/client");
const pkgDir = path.join(root, "packages/sim-worker/pkg");
const targetDir = path.join(root, "target");
const wasmFile = path.join(targetDir, "wasm32-unknown-unknown/release/sim_wasm.wasm");
/** Variables through which cargo's settings can be changed from outside with no diff in the repository. */
const CARGO_OVERRIDES = /^(RUSTFLAGS|CARGO_ENCODED_RUSTFLAGS|RUSTC_WRAPPER|RUSTC|CARGO_(BUILD|PROFILE|TARGET)_.*)$/;

/** The tool's absolute path, after its file was compared with the recorded values. */
function checkedTool() {
  const tool = process.env["SHAMBLE_WASM_BINDGEN"];
  if (tool === undefined || !path.isAbsolute(tool)) {
    throw new Error("SHAMBLE_WASM_BINDGEN must be the absolute path of the wasm-bindgen tool");
  }
  checkCliFile(tool, WASM_BINDGEN_TOOL.program.sha256, WASM_BINDGEN_TOOL.program.bytes);
  return tool;
}

/** Builds the Wasm and writes its bindings. Returns the size table of what the Worker loads. */
export function buildWasm() {
  const locked = lockedWasmBindgenVersion(readFileSync(path.join(root, "Cargo.lock"), "utf8"));
  if (locked !== WASM_BINDGEN_TOOL.version) {
    throw new Error(`Cargo.lock records wasm-bindgen ${locked} and wasm-bindgen-tool.mjs the tool ${WASM_BINDGEN_TOOL.version}`);
  }
  checkCli(execFileSync(checkedTool(), ["--version"], { encoding: "utf8" }), locked);
  // The Wasm players get is built with the manifests' settings alone (the overflow checks, the optimisation level,
  // the target's features), and into this repository's own target folder, which is where it is read from below.
  const overrides = Object.keys(process.env).filter((name) => CARGO_OVERRIDES.test(name));
  if (overrides.length > 0) throw new Error(`cargo settings from the environment are refused: unset ${overrides.join(", ")}`);
  execFileSync(
    "cargo",
    ["build", "--locked", "--release", "--target", "wasm32-unknown-unknown", "-p", "sim-wasm", "--target-dir", targetDir],
    { cwd: root, stdio: "inherit" },
  );
  mkdirSync(pkgDir, { recursive: true });
  // Checked again: the cargo build above ran build scripts as this same user.
  execFileSync(checkedTool(), ["--target", "web", "--out-dir", pkgDir, wasmFile], { cwd: root, stdio: "inherit" });
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

/** The commit the page is built from, marked when the tree holds changes that are not committed. */
function commitName() {
  const git = (...args) => execFileSync("git", args, { cwd: root, encoding: "utf8" }).trim();
  const changed = git("status", "--porcelain", "--", "crates", "packages", "Cargo.toml", "Cargo.lock", "package.json", "package-lock.json");
  return git("rev-parse", "--short=9", "HEAD") + (changed === "" ? "" : "+changes");
}

/** Every file of a folder, as paths from it with forward slashes. */
function filesOf(folder, prefix = "") {
  return readdirSync(folder, { withFileTypes: true })
    .sort((a, b) => (a.name < b.name ? -1 : 1))
    .flatMap((entry) =>
      entry.isDirectory() ? filesOf(path.join(folder, entry.name), `${prefix}${entry.name}/`) : [`${prefix}${entry.name}`],
    );
}

/** Builds the page into `outDir` and checks every served file. Returns their size table. */
export async function buildPage({ production, outDir }) {
  process.env["VITE_SHAMBLE_COMMIT"] = commitName();
  if (production) delete process.env["VITE_SHAMBLE_DEVTOOLS"];
  else process.env["VITE_SHAMBLE_DEVTOOLS"] = "1";
  await viteBuild({
    root: clientDir,
    configFile: path.join(clientDir, "vite.config.mjs"),
    logLevel: "warn",
    build: { outDir, emptyOutDir: true },
  });
  // The zip's small server is a development tool: it is not part of a production build.
  if (!production) copyFileSync(path.join(clientDir, "serve.mjs"), path.join(outDir, "serve.mjs"));
  const files = filesOf(outDir)
    .filter((name) => name !== "serve.mjs")
    .map((name) => ({ name, bytes: readFileSync(path.join(outDir, name)) }));
  const rows = sizeTable(files);
  const problems = [...tooLarge(rows).map((row) => `${row.name}: a served file reaches 25 MiB`), ...cspProblems(files)];
  if (problems.length > 0) throw new Error(`the built page is refused:\n  ${problems.join("\n  ")}`);
  return rows;
}

async function main() {
  const args = process.argv.slice(2);
  const at = args.indexOf("--out-dir");
  const outDir = at === -1 ? path.join(clientDir, "dist") : path.resolve(args[at + 1] ?? "");
  const unknown = args.filter((arg, i) => !["--wasm-only", "--production", "--out-dir"].includes(arg) && i !== at + 1);
  if (unknown.length > 0 || (at !== -1 && args[at + 1] === undefined)) throw new Error(`unknown or unfinished option: ${args.join(" ")}`);
  console.log(formatSizeTable(buildWasm()));
  if (args.includes("--wasm-only")) return;
  const production = args.includes("--production");
  console.log(`\n${production ? "production" : "development"} page in ${path.relative(root, outDir)}:`);
  console.log(formatSizeTable(await buildPage({ production, outDir })));
}

if (process.argv[1] !== undefined && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  main().catch((error) => {
    console.error(`build: ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  });
}
