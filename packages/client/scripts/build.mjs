// The build of the player's page, from the repository's root:
//
//   SHAMBLE_WASM_BINDGEN=<absolute path of the wasm-bindgen tool> node packages/client/scripts/build.mjs [--wasm-only]
//
// It builds the simulation as WebAssembly and writes its bindings into packages/sim-worker/pkg/ (build output,
// ignored by git). Every command runs as a list of arguments, never through a shell.
//
// The wasm-bindgen tool is a ready-built program from the internet. Before it is run at all, `--version` included,
// its file must be the one wasm-bindgen-tool.mjs records (a plain file of that size and sha256), and its version
// must equal the wasm-bindgen library in Cargo.lock (PM-0002-18). The path that was checked is the path that is
// run: the tool is never looked up by name.

import { execFileSync } from "node:child_process";
import { mkdirSync, readdirSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { checkCli, checkCliFile, formatSizeTable, lockedWasmBindgenVersion, sizeTable, tooLarge } from "./build-checks.mjs";
import { WASM_BINDGEN_TOOL } from "./wasm-bindgen-tool.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../..");
const pkgDir = path.join(root, "packages/sim-worker/pkg");
const wasmFile = path.join(root, "target/wasm32-unknown-unknown/release/sim_wasm.wasm");

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
  execFileSync("cargo", ["build", "--locked", "--release", "--target", "wasm32-unknown-unknown", "-p", "sim-wasm"], {
    cwd: root,
    stdio: "inherit",
  });
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

function main() {
  const args = process.argv.slice(2);
  const unknown = args.filter((arg) => arg !== "--wasm-only");
  if (unknown.length > 0) throw new Error(`unknown option: ${unknown.join(" ")}`);
  console.log(formatSizeTable(buildWasm()));
  if (!args.includes("--wasm-only")) {
    throw new Error("the page's build joins this script with the page (step 6): use --wasm-only until then");
  }
}

if (process.argv[1] !== undefined && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    main();
  } catch (error) {
    console.error(`build: ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  }
}
