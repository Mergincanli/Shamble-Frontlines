// The security seat's round-2 probe (u4d2-r2-review-security.md), a companion of sec-r2-scan.mjs: near forms of FX-6
// and pathProblems that the first probe did not try. The copy's unsafeProblems on crateFiles' real list with
// npt-hash-wasm's lib.rs swapped, in memory only. Each case prints its problem count and lines. Writes nothing.
// Run from the copy's spikes/netcode-proof.
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const here = process.cwd();
const { unsafeProblems, crateFiles } = await import(pathToFileURL(join(here, "check.mjs")).href);
const LIB = "crates/hash-wasm/src/lib.rs";
const real = crateFiles(join(here, "crates")).files;
const disk = (f) => readFileSync(join(here, f), "utf8");
const lib = disk(LIB);
const BEGIN_BODY = "    let Some(replayer) = replayer_for(bite, arm_tick) else {\n";
if (!lib.includes(BEGIN_BODY)) throw new Error("anchor not found in begin's body");
const inBegin = (line) => lib.replace(BEGIN_BODY, `${line}\n${BEGIN_BODY}`);
const meta = "macro_rules! m {\n    ($a:meta) => {\n        #[$a]\n        mod g;\n    };\n}\n";
for (const [label, text] of [
  ['#[$a] with m!(path = "x.txt") in begin\'s body', `${meta}${inBegin('    m!(path = "x.txt");')}`],
  ['#[$a] with m!(path = "../../../gen.rs") at the crate level', `${lib}${meta}m!(path = "../../../gen.rs");\n`],
  ['a plain #[path = "x.txt"] mod g; in begin\'s body (control: refused)', inBegin('    #[path = "x.txt"]\n    mod g;')],
]) {
  const p = unsafeProblems(real, (f) => (f === LIB ? text : disk(f)));
  console.log(`--- ${label}: ${p.length} problem(s)`);
  for (const l of p) console.log(`    ${l.slice(0, 240)}`);
}
