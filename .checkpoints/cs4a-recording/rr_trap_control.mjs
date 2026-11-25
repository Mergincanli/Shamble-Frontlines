// rust-reviewer r1 scratch: controls for rr_fuzz_wasm.mjs (a panic does throw in this harness) and R2's case.
// Usage: node rr_trap_control.mjs <module folder> <profile> <case: plant|rounds-ok|rounds-big>
import { readFileSync } from "node:fs";
import path from "node:path";
const [folder, profile, which] = process.argv.slice(2);
const glue = await import(path.join(folder, profile, "devtools.js"));
glue.initSync({ module: readFileSync(path.join(folder, profile, "devtools_bg.wasm")) });
try {
  if (which === "plant") console.log("returned", glue.plant_u16_add(65535, 1));
  if (which === "rounds-ok") console.log("returned", glue.fnv1a_64_rounds(138_547_332, 1).toString(16));
  if (which === "rounds-big") console.log("returned", glue.fnv1a_64_rounds(138_547_334, 1).toString(16));
} catch (error) {
  console.log(`${profile} ${which}: threw ${error.constructor.name}: ${String(error.message).split("\n")[0]}`);
}
