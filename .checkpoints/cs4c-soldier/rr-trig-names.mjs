// rust-reviewer's scratch for block 7 (RR4): the build check's own readers on the scratch wasm file: its function
// names, what libmProblems says against today's reviewed list, and the fused or banned names. Read-only.
import { readFileSync } from "node:fs";
import { wasmFunctionNames, libmProblems, fusedMultiplyAddProblems, bannedMathsProblems } from "../../packages/client/scripts/build-checks.mjs";
import { MATHS_ALLOWED } from "../../packages/client/scripts/maths-allowed.mjs";
const bytes = readFileSync(process.argv[2]);
const names = wasmFunctionNames(bytes);
console.log(`${names.length} function names`);
for (const n of names) console.log(`name ${n}`);
for (const p of libmProblems(names, MATHS_ALLOWED)) console.log(`libm ${p}`);
for (const p of fusedMultiplyAddProblems(names)) console.log(`fused ${p}`);
for (const p of bannedMathsProblems(names)) console.log(`banned ${p}`);
