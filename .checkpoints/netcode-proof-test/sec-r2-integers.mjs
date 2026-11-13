// The security seat's round-2 probe (u4d2-r2-review-security.md): SEC-U4D2-3's limit as the comment beside
// EXPORT_INTEGERS states it (check.mjs:217-225), against the copy's unsafeProblems. npt-hash-wasm's lib.rs swapped in
// memory only: a module-local `type u32 = f64;`, a `use core::primitive::f64 as u32;`, and a control (feed's word
// typed f64 by its own token). Writes nothing. Run from the copy's spikes/netcode-proof.
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const here = process.cwd();
const { unsafeProblems, crateFiles } = await import(pathToFileURL(join(here, "check.mjs")).href);
const LIB = "crates/hash-wasm/src/lib.rs";
const real = crateFiles(join(here, "crates")).files;
const disk = (f) => readFileSync(join(here, f), "utf8");
const lib = disk(LIB);
const FEED = 'pub extern "C" fn feed(word: u32) {';
if (!lib.includes(FEED)) throw new Error("feed's signature not found");
for (const [label, text] of [
  ["`type u32 = f64;` (the comment: passes)", `type u32 = f64;\n${lib}`],
  ["`use core::primitive::f64 as u32;` (the comment: passes)", `use core::primitive::f64 as u32;\n${lib}`],
  ["control: feed(word: f64) (refused)", lib.replace(FEED, 'pub extern "C" fn feed(word: f64) {')],
]) {
  const p = unsafeProblems(real, (f) => (f === LIB ? text : disk(f)));
  console.log(`--- ${label}: ${p.length} problem(s)`);
  for (const l of p) console.log(`    ${l.slice(0, 240)}`);
}
