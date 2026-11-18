// The cost of the six overflow overrides (register row OVERFLOW-ALL: "with its cost measured"): the time of one
// step in two builds of the same Wasm, one with the lines and one without. A reading in Node on this container,
// never judged.   node overflow-cost.mjs <pkg folder with the lines> <pkg folder without>
import { readFileSync } from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";

const STEPS = 2_000_000;
const RUNS = 7;
async function load(pkg) {
  const glue = await import(pathToFileURL(path.resolve(pkg, "sim_wasm.js")).href);
  glue.initSync({ module: readFileSync(path.resolve(pkg, "sim_wasm_bg.wasm")) });
  return glue.Sim;
}
function nsPerStep(Sim) {
  const sim = new Sim(new Uint8Array(32), 0, 0, 0);
  const start = process.hrtime.bigint();
  for (let i = 0; i < STEPS; i += 1) {
    sim.input(i);
    sim.step();
  }
  const ns = Number(process.hrtime.bigint() - start) / STEPS;
  sim.free();
  return ns;
}
const [withLines, without] = [await load(process.argv[2]), await load(process.argv[3])];
const samples = { with: [], without: [] };
for (let run = 0; run < RUNS + 1; run += 1) {
  const a = nsPerStep(withLines);
  const b = nsPerStep(without);
  if (run > 0) { samples.with.push(a); samples.without.push(b); } // the first pair warms up
}
const median = (list) => [...list].sort((x, y) => x - y)[Math.floor(list.length / 2)];
for (const [name, list] of Object.entries(samples)) {
  console.log(`${name.padEnd(8)} median ${median(list).toFixed(2)} ns per tick (input + step), min ${Math.min(...list).toFixed(2)}, max ${Math.max(...list).toFixed(2)}, ${RUNS} runs of ${STEPS} ticks`);
}
