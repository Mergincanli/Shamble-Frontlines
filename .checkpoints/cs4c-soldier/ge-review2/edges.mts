// Round 2 probe: overview's ending and frameCamera at edge aspects (graphics-engineer, step S round 2).
import * as THREE from "three/webgpu";
import { overview, frameCamera } from "../../../packages/client/src/render/scene.ts";
const MAP = { min: [-32, 0, -12], max: [80, 4.7, 12] } as any;
const LARGEST = { min: [-327.67, 0, -327.67], max: [327.67, 655.35, 327.67] } as any;
for (const b of [["map", MAP], ["largest", LARGEST]] as const) {
  for (const a of [0, NaN, Infinity, -1, 1e-300, 1e300, 1 / 4320, 0.0015, 0.002, 1 / 1080, 1, 1920]) {
    const t0 = performance.now();
    const v = overview(b[1], a);
    const ms = performance.now() - t0;
    console.log(b[0], a, v === null ? "null" : `d=${new THREE.Vector3(...v.position).distanceTo(new THREE.Vector3(...v.target)).toFixed(2)} pos=${v.position.map((x) => x.toFixed(1))}`, `${ms.toFixed(1)}ms`);
  }
}
const cam = new THREE.PerspectiveCamera(50, 1 / 1080, 0.1, 500);
console.log("frame 1x1080 largest", frameCamera(cam, LARGEST), cam.far);
