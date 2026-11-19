// Scratch (test-engineer, CI-WASM followups, block 5): a draft of the check "every third-party runtime crate of the
// shipped Wasm has its overflow override in the root Cargo.toml", tried on today's tree and on planted Cargo.toml texts.
// Runs `cargo tree` read-only (no build, no target/ write); prints names only.
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";

/** Third-party crates in `cargo tree --prefix none` output (workspace members carry a path; `(*)` repeats one). */
function runtimeCrates(treeText) {
  const names = new Set();
  for (const line of treeText.split("\n")) {
    const m = /^([A-Za-z0-9_-]+) v\S+( \((?<rest>[^)]*)\))?/.exec(line.trim());
    if (m === null) continue;
    const rest = m.groups?.rest ?? "";
    if (rest.startsWith("/") || /^[A-Za-z]:\\/.test(rest)) continue; // a workspace member (a local path)
    if (rest === "proc-macro") throw new Error(`a proc-macro crate in the tree (${m[1]}): run cargo tree with -e normal,no-proc-macro`);
    names.add(m[1]);
  }
  return [...names].sort();
}

/** Crates with `[profile.release.package.<name>]` followed by `overflow-checks = true` ("*" is not a name). */
function overriddenCrates(cargoToml) {
  const names = new Set();
  for (const m of cargoToml.matchAll(/^\[profile\.release\.package\.("?)([A-Za-z0-9_-]+)\1\]\r?\n((?:(?!\[).*\r?\n)*)/gm)) {
    if (/^overflow-checks\s*=\s*true\s*$/m.test(m[3])) names.add(m[2]);
  }
  return names;
}

const missing = (tree, toml) => runtimeCrates(tree).filter((name) => !overriddenCrates(toml).has(name));

const t0 = Date.now();
const tree = execFileSync("cargo", ["tree", "--locked", "--offline", "-p", "sim-wasm", "--target", "wasm32-unknown-unknown", "-e", "normal,no-proc-macro", "--prefix", "none"], { encoding: "utf8" });
const ms = Date.now() - t0;
const toml = readFileSync(new URL("../../Cargo.toml", import.meta.url), "utf8");
console.log(`cargo tree: ${ms} ms; runtime crates: ${runtimeCrates(tree).join(", ")}`);
console.log(`overridden: ${[...overriddenCrates(toml)].sort().join(", ")}`);
console.log(`today: missing = [${missing(tree, toml).join(", ")}]`);

const plants = {
  "the wasm-bindgen-shared override removed": toml.replace("[profile.release.package.wasm-bindgen-shared]\noverflow-checks = true\n", ""),
  "cfg-if's override set to false": toml.replace("[profile.release.package.cfg-if]\noverflow-checks = true", "[profile.release.package.cfg-if]\noverflow-checks = false"),
  "a new runtime crate in the tree (rand_core)": null,
};
for (const [what, planted] of Object.entries(plants)) {
  const result = planted === null ? missing(`${tree}rand_core v0.9.3\n`, toml) : missing(tree, planted);
  console.log(`plant "${what}": missing = [${result.join(", ")}] -> ${result.length > 0 ? "caught" : "PASSED UNSEEN"}`);
}
try {
  runtimeCrates("wasm-bindgen-macro v0.2.130 (proc-macro)\n");
  console.log("a proc-macro line: NOT refused");
} catch (error) {
  console.log(`a proc-macro line: refused (${error.message.slice(0, 60)}...)`);
}

const t1 = Date.now();
execFileSync("cargo", ["metadata", "--locked", "--offline", "--format-version", "1", "--filter-platform", "wasm32-unknown-unknown"], { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });
console.log(`cargo metadata (for comparison): ${Date.now() - t1} ms`);
