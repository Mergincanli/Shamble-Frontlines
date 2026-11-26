// security-auditor scratch (heapless advisories): reads the twins already on disk with build-checks.mjs's own readers
// (nothing built, nothing written) and reports which mangling scheme their names use, whether any name mentions
// heapless or rstar in any spelling, and what nameSteps / unlinkedCrateProblems find for legacy and v0 spellings.
// Usage: node security-auditor-advisories-names.mjs <repo root>
import { readFileSync } from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";

const [repo] = process.argv.slice(2);
const checks = await import(pathToFileURL(path.join(repo, "packages/client/scripts/build-checks.mjs")).href);
const { wasmFunctionNames, nameSteps, unlinkedCrateProblems } = checks;

for (const [label, rel] of [
  ["release twin", "target/wasm32-unknown-unknown/release/sim_wasm.wasm"],
  ["dev-profile twin", "target/wasm32-unknown-unknown/debug/sim_wasm.wasm"],
]) {
  const names = wasmFunctionNames(readFileSync(path.join(repo, rel)), label);
  const v0 = names.filter((n) => n.startsWith("_R")).length;
  const legacy = names.filter((n) => n.startsWith("_ZN")).length;
  const mention = names.filter((n) => /heapless|rstar/.test(n));
  const traitImplsV0 = names.filter((n) => n.startsWith("_R") && /^_RNvX/.test(n)).length;
  console.log(`${label}: ${names.length} names, v0 (_R) ${v0}, legacy (_ZN) ${legacy}, other ${names.length - v0 - legacy}; v0 trait impls ${traitImplsV0}`);
  console.log(`  names mentioning heapless/rstar in any spelling: ${mention.length}; rule lines: ${unlinkedCrateProblems(names).length}`);
  console.log(`  sample other-scheme names: ${JSON.stringify(names.filter((n) => !n.startsWith("_R") && !n.startsWith("_ZN")).slice(0, 6))}`);
}

// The same functions in the two schemes (hand-written spellings, hash parts made up).
const samples = {
  "legacy inherent (BinaryHeap::pop)": "_ZN8heapless11binary_heap28BinaryHeap$LT$T$C$K$C$_$GT$3pop17h0123456789abcdefE",
  "legacy trait impl (<heapless::vec::IntoIter as Drop>::drop)": "_ZN104_$LT$heapless..vec..IntoIter$LT$T$C$_$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h0123456789abcdefE",
  "legacy drop glue (drop_in_place<rstar::..::SmallHeap>)": "_ZN4core3ptr85drop_in_place$LT$rstar..algorithm..nearest_neighbor..SmallHeap$LT$u32$GT$$GT$17h0123456789abcdefE",
  "v0 trait impl (from the test)": "_RNvXs_NtNtCs9aB3cD4eF5g_5rstar9algorithm16nearest_neighborINtB4_24NearestNeighborIteratormENtNtNtNtCs4NIexwV1FGX_4core4iter6traits8iterator8Iterator4next",
};
for (const [label, name] of Object.entries(samples)) {
  const steps = nameSteps(name);
  console.log(`${label}: heapless step ${steps.has("heapless")}, rstar step ${steps.has("rstar")}, rule lines ${unlinkedCrateProblems([name]).length}`);
}
