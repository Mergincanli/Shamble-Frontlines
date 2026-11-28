// perf-auditor scratch harness for step S (read-only on the code): the real scene.ts in Node, no renderer.
// Run from the worktree root: node --expose-gc .checkpoints/cs4c-soldier/perf-s-harness.mjs <part>
import * as THREE from "three/webgpu";
import { buildMapFloor, disposeFloor, overview } from "../../packages/client/src/render/scene.ts";
import { readFile } from "node:fs/promises";

const part = process.argv[2] ?? "all";
const R = 327.67;
function worstMap() {
  const shapes = [];
  for (let i = 0; i < 512; i += 1) {
    const x = -300 + (i % 32) * 18;
    const z = -300 + Math.floor(i / 32) * 36;
    const top = 0.5 + (i % 128) * 0.25; // many distinct tops: many greys
    const ramp = i % 4 === 0;
    shapes.push(ramp ? { id: `s${i}`, type: "ramp", rises_toward: "+x", min: [x, 0, z], max: [x + 4, top, z + 4] } : { id: `s${i}`, type: "box", min: [x, 0, z], max: [x + 4, top, z + 4] });
  }
  const regions = [];
  for (let i = 0; i < 64; i += 1) regions.push({ id: `r${i}`, kind: i % 2 ? "street" : "grass", min: [-320 + i * 10, -320], max: [-312 + i * 10, 320] });
  return { id: "worst", bounds: { min: [-R, 0, -R], max: [R, 60, R] }, spawns: [{ id: "a", at: [0, 0, 0], facing_deg: 0 }], shapes, regions };
}
function stats(floor) {
  const mats = new Set();
  let tris = 0, lineVerts = 0, meshes = 0, lines = 0;
  for (const o of floor.children) {
    mats.add(o.material);
    const n = o.geometry.getAttribute("position").count;
    if (o.isLineSegments) { lines += 1; lineVerts += n; } else { meshes += 1; tris += n / 3; }
  }
  return { objects: floor.children.length, meshes, lines, materials: mats.size, triangles: tris, lineSegments: lineVerts / 2 };
}

if (part === "all" || part === "counts") {
  const test = JSON.parse(await readFile(new URL("../../content/maps/test-scene.json", import.meta.url), "utf8"));
  console.log("test-scene", JSON.stringify(stats(buildMapFloor(test))));
  const times = [];
  const worst = worstMap();
  for (let k = 0; k < 7; k += 1) {
    const t = performance.now();
    const f = buildMapFloor(worst);
    times.push(performance.now() - t);
    if (k === 0) console.log("worst", JSON.stringify(stats(f)));
    disposeFloor(f);
  }
  times.sort((a, b) => a - b);
  console.log("worst build ms (7 runs, sorted)", times.map((t) => t.toFixed(2)).join(" "));
}

if (part === "all" || part === "dispose") {
  const worst = worstMap();
  const f = buildMapFloor(worst);
  let g = 0, m = 0;
  const seenG = new Set(), seenM = new Set();
  for (const o of f.children) {
    if (!seenG.has(o.geometry)) { seenG.add(o.geometry); o.geometry.addEventListener("dispose", () => { g += 1; }); }
    if (!seenM.has(o.material)) { seenM.add(o.material); o.material.addEventListener("dispose", () => { m += 1; }); }
  }
  disposeFloor(f);
  console.log(`dispose: geometries ${g} of ${seenG.size}, materials ${m} of ${seenM.size}`);
}

if (part === "all" || part === "overview") {
  for (const [name, bounds] of [["test-scene", { min: [-32, 0, -24], max: [80, 20, 24] }], ["worst", worstMap().bounds]]) {
    for (const aspect of [16 / 9, 0.3]) {
      const N = 2000;
      for (let i = 0; i < 200; i += 1) overview(bounds, aspect);
      globalThis.gc?.();
      const h0 = process.memoryUsage().heapUsed;
      const t = performance.now();
      for (let i = 0; i < N; i += 1) overview(bounds, aspect);
      const us = ((performance.now() - t) / N) * 1000;
      console.log(`overview ${name} aspect ${aspect.toFixed(2)}: ${us.toFixed(1)} us a call (${N} calls), heap growth before gc ${((process.memoryUsage().heapUsed - h0) / N).toFixed(0)} B a call`);
    }
  }
}

if (part === "aspect0") {
  // Does the fit terminate when the window is 0 wide (aspect 0)? Count the doubling steps by wrapping nothing: run with a timeout.
  const t = performance.now();
  const v = overview({ min: [-32, 0, -24], max: [80, 20, 24] }, 0);
  console.log("aspect 0 returned", JSON.stringify(v), (performance.now() - t).toFixed(1), "ms");
}
if (part === "aspectnan") {
  const v = overview({ min: [-32, 0, -24], max: [80, 20, 24] }, NaN);
  console.log("aspect NaN returned", JSON.stringify(v));
}
