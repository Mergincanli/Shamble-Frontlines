// security-auditor scratch (4b-2 step 6): the three feature lists build.mjs reads, resolved from the batch in a
// scratch copy (cargo tree --offline --locked: nothing compiled, nothing fetched), held against the committed vector
// crates/devtools/vectors/sim-feature-lists.json and against the compare with its two named exceptions.
// Usage: node security-auditor-deps-4b2-lists.mjs <scratch workspace> <repo root>
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";

const [ws, repo] = process.argv.slice(2);
const { simFeatureList, simFeatureDifferences, missingOverflowOverrides, NATIVE_TESTS } = await import(
  pathToFileURL(path.join(repo, "packages/client/scripts/build-checks.mjs")).href
);
const tree = (...args) => execFileSync("cargo", ["tree", "--offline", "--locked", ...args], { cwd: ws, encoding: "utf8" });
const features = (edges, ...args) => tree(...args, "-e", `features,${edges},no-proc-macro`, "--prefix", "depth", "--no-dedupe");
const target = "wasm32-unknown-unknown";
const lists = {
  "the players' Wasm": simFeatureList(features("normal", "-p", "sim-wasm", "--target", target)),
  "the test-only module": simFeatureList(features("normal", "-p", "devtools", "--target", target)),
  [NATIVE_TESTS]: simFeatureList(features("normal,dev", "--workspace"), features("normal", "--workspace")),
};
for (const [name, list] of Object.entries(lists)) console.log(`${name}: ${list.length} entries`);
const vector = JSON.parse(readFileSync(path.join(repo, "crates/devtools/vectors/sim-feature-lists.json"), "utf8"));
for (const [name, list] of Object.entries(lists)) {
  const v = vector[name] ?? [];
  const same = v.length === list.length && v.every((entry, i) => entry === list[i]);
  console.log(`${name}: equal to the committed vector: ${same}`);
  if (!same) console.log("  only resolved:", list.filter((e) => !v.includes(e)), "only vector:", v.filter((e) => !list.includes(e)));
}
console.log("compare with the two exceptions:", JSON.stringify(simFeatureDifferences(lists)));
console.log("compare without them:", JSON.stringify(simFeatureDifferences(lists, new Map(), new Map())));
const overflow = missingOverflowOverrides(
  tree("-p", "sim-wasm", "--target", target, "-e", "normal,no-proc-macro", "--prefix", "none"),
  readFileSync(path.join(ws, "Cargo.toml"), "utf8"),
  ws,
);
console.log("players' Wasm crates without an overflow override:", JSON.stringify(overflow));
console.log("players' list:", JSON.stringify(lists["the players' Wasm"]));
