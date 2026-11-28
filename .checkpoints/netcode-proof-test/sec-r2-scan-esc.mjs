// The security seat's round-2 probe (u4d2-r2-review-security.md), a companion of sec-r2-scan.mjs: C2R-2's escaped
// literal in round 1's form (`.\x2e/`, which rustc reads as `../`), in an include! in begin's body and in a #[path],
// in npt-hash-wasm. A literal that starts with a backslash is read by pathProblems as an absolute path and refused
// (sec-r2-scan.mjs's two cases), so these start with a dot. In memory only; writes nothing.
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
const esc = ".\\x2e/.\\x2e/.\\x2e/gen.rs"; // rustc: ../../../gen.rs, the spike's root from crates/hash-wasm/src
for (const [label, text] of [
  [`include!("${esc}") in begin's body`, lib.replace(BEGIN_BODY, `    include!("${esc}");\n${BEGIN_BODY}`)],
  [`#[path = "${esc}"] mod g;`, `${lib}#[path = "${esc}"]\nmod g;\n`],
]) {
  const p = unsafeProblems(real, (f) => (f === LIB ? text : disk(f)));
  console.log(`--- ${label}: ${p.length} problem(s)`);
  for (const l of p) console.log(`    ${l.slice(0, 240)}`);
}
