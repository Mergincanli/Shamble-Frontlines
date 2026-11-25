// rust-reviewer scratch for PM-0004-21: times the scratch module's FNV-1a 64 and XXH64 in WebAssembly under Node.
// Run: node <this file> <path to bench.wasm>. Block mean: one timer pair around 20,000 calls inside Wasm (as the
// project's boundary test does). Per-call mean and p99: process.hrtime.bigint() around each of 20,000 single calls
// from JavaScript, so they include the timer and one JS-to-Wasm call.
import { readFileSync } from "node:fs";

const bytes = readFileSync(process.argv[2]);
const module = new WebAssembly.Module(bytes);
console.log("imports:", JSON.stringify(WebAssembly.Module.imports(module)));
const { exports: x } = new WebAssembly.Instance(module, {});
console.log("digest_check (FNV and XXH64 give the native digests on wasm32):", x.digest_check());
const rounds = 20_000;
console.log(`WebAssembly in Node ${process.version}, scratch module (rustc opt-level 3, no wasm-opt), ${rounds} calls per figure`);
for (const length of [22, 54, 3_076, 4_096, 16_384, 65_536]) {
  x.fill(length);
  const line = [];
  for (const [name, once, many] of [["fnv1a_64", x.fnv_once, x.fnv_rounds], ["xxh64", x.xxh_once, x.xxh_rounds]]) {
    many(200);
    const start = process.hrtime.bigint();
    many(rounds);
    const block = Number(process.hrtime.bigint() - start) / rounds;
    const nanos = new Float64Array(rounds);
    let folded = 0n;
    for (let i = 0; i < rounds; i++) {
      const t0 = process.hrtime.bigint();
      folded ^= once();
      nanos[i] = Number(process.hrtime.bigint() - t0);
    }
    nanos.sort();
    const mean = nanos.reduce((a, b) => a + b, 0) / rounds;
    line.push(`${name} block ${block.toFixed(1).padStart(8)} ns, per-call ${mean.toFixed(1).padStart(8)} / p99 ${String(nanos[Math.floor(rounds * 0.99)]).padStart(6)} ns`);
    if (folded === 1n) console.log("(never)");
  }
  console.log(`  ${String(length).padStart(6)} B  ${line.join("   ")}`);
}
