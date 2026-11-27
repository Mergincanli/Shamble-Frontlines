// The loader's own sentence for each hostile bundle of packages/e2e/tests/skeleton/content.spec.ts, read from the
// players' file under Node (the same constructor the Worker calls), so the spec pins the words the loader says, never
// typed guesses. Notes only. From the worktree's root, after a build of the players' file:
//   node .checkpoints/cs4b-content/hostile-sentences.mjs
import { readFileSync } from "node:fs";
import { initSync, Sim } from "../../packages/sim-worker/pkg/sim_wasm.js";

initSync({ module: readFileSync(new URL("../../packages/sim-worker/pkg/sim_wasm_bg.wasm", import.meta.url)) });
const built = readFileSync(new URL("../../.gate/dist-development/content.json", import.meta.url), "utf8");
// The spec's own cases, as it writes them.
const hostile = [
  ["a duplicate key", (text) => text.replace('"format": "shamble-content-bundle",', '"format": "shamble-content-bundle",\n  "format": "shamble-content-bundle",')],
  ["a __proto__ key", (text) => text.replace('{\n  "files": [', '{\n  "__proto__": {\n    "polluted": 1\n  },\n  "files": [')],
  ["a number out of every range", (text) => text.replace('"version": 1', '"version": 1e400')],
  ["an extra key in a file entry", (text) => text.replace('"path": "maps/test-scene.json",', '"extra": 1,\n      "path": "maps/test-scene.json",')],
  ["a source/ path", (text) => text.replace('"path": "maps/test-scene.json"', '"path": "source/test-scene.json"')],
  ["an unknown key named like an image tag", (text) => text.replace('{\n  "files": [', '{\n  "<img src=x onerror=alert(1)>": 1,\n  "files": [')],
  ["a key of 1 MiB", () => `{"${"k".repeat(1 << 20)}": 1}`],
];
for (const [name, make] of hostile) {
  const text = make(built);
  try {
    new Sim(new Uint8Array(32).fill(7), 0, 0, new TextEncoder().encode(text));
    console.log(`${name}: TAKEN (no refusal)`);
  } catch (error) {
    console.log(`${name}: ${JSON.stringify(error instanceof Error ? error.message : String(error))}`);
  }
}
