// rust-reviewer r1 scratch: runs rr_sha_diff's wasm32 build in Node (no imports) and prints its two numbers.
import { readFileSync } from "node:fs";
const bytes = readFileSync(process.argv[2]);
const { instance } = await WebAssembly.instantiate(bytes, {});
const hex = (v) => BigInt.asUintN(64, v).toString(16).padStart(16, "0");
console.log(hex(instance.exports.fold_all()), hex(instance.exports.long_a((1 << 29) + 1)));
