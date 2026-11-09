// node run.mjs <u4s_xtarget.wasm> [shift]: the grid hash computed inside wasm32, printed as native's main.rs prints it.
import { readFileSync } from "node:fs";

const bytes = readFileSync(process.argv[2]);
const shift = Number(process.argv[3] ?? 8);
const { instance } = await WebAssembly.instantiate(bytes, {});
const imports = WebAssembly.Module.imports(new WebAssembly.Module(bytes));
const h = BigInt.asUintN(64, instance.exports.xt_run(shift));
const n = BigInt.asUintN(64, instance.exports.xt_count(shift));
console.log(`wasm32 shift ${shift}: hash ${h.toString(16).padStart(16, "0")} over ${n} values (imports: ${imports.length})`);
