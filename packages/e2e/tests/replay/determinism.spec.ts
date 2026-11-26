// The recorded matches in the browser's own WebAssembly engine (ADR 0004's milestone; PM-0004-19): every committed
// recording replays, in a Worker, on the file players get, to its hashes file's shared column. The Worker and its
// made-up address exist in this spec alone: no server hands them out, and nothing a player gets holds them. The file
// under test is packages/sim-worker/pkg's, which the spec first shows to be, byte for byte, the one in the built page.
import { readdirSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { expect, test } from "@playwright/test";
import { checkReplayable, firstDifference, readHashes, readRecording } from "../../../sim-worker/src/recording.ts";

const root = fileURLToPath(new URL("../../../..", import.meta.url));
const recordings = path.join(root, "crates/devtools/recordings");
const pkg = path.join(root, "packages/sim-worker/pkg");
const ORIGIN = "http://replay.invalid";

/** The Worker: the bindings' own calls, as the live host makes them, and the shared hash after each step. */
const WORKER = `import init, { Sim, tick_hz } from "/sim_wasm.js";
onmessage = async ({ data }) => {
  await init({ module_or_path: "/sim_wasm_bg.wasm" });
  const sim = new Sim(Uint8Array.from(data.seed), 0, 0);
  if (data.steps.length > 0) sim.rebase(data.steps[0][0]);
  const hashes = data.steps.map(([now, post, held, p0, p1, p2, p3]) => {
    sim.input(post, held, p0, p1, p2, p3);
    sim.step(now);
    return sim.shared_hash().toString(16);
  });
  postMessage({ tickHz: tick_hz(), hashes });
};`;

type Step = readonly [number, number, number, number, number, number, number];
interface Outcome {
  tickHz: number;
  hashes: string[];
}

test("every recording replays in the browser's WebAssembly, on the file players get, to its hashes file", async ({ page }) => {
  const dist = process.env["SHAMBLE_DIST"];
  if (dist === undefined) throw new Error("SHAMBLE_DIST names no built page");
  const built = readdirSync(path.join(dist, "assets")).filter((name) => /^sim_wasm_bg-.*\.wasm$/.test(name));
  expect(built).toHaveLength(1);
  const [inPage, inPkg] = [readFileSync(path.join(dist, "assets", built[0] ?? "")), readFileSync(path.join(pkg, "sim_wasm_bg.wasm"))];
  expect(inPage.length === inPkg.length && inPage.every((byte, at) => byte === inPkg[at])).toBe(true);

  await page.route(`${ORIGIN}/**`, (route) => {
    const name = new URL(route.request().url()).pathname.slice(1);
    if (name === "") return route.fulfill({ contentType: "text/html", body: "<!doctype html><title>replay</title>" });
    if (name === "replay-worker.js") return route.fulfill({ contentType: "text/javascript", body: WORKER });
    if (name === "sim_wasm.js") return route.fulfill({ contentType: "text/javascript", path: path.join(pkg, name) });
    if (name === "sim_wasm_bg.wasm") return route.fulfill({ contentType: "application/wasm", path: path.join(pkg, name) });
    return route.fulfill({ status: 404, body: "" });
  });
  await page.goto(`${ORIGIN}/`);
  const replay = (seed: number[], steps: Step[]): Promise<Outcome> =>
    page.evaluate(
      (data) =>
        new Promise<Outcome>((resolve, reject) => {
          const worker = new Worker("/replay-worker.js", { type: "module" });
          worker.onmessage = (event: MessageEvent<Outcome>): void => resolve(event.data);
          worker.onerror = (event): void => reject(new Error(event.message));
          worker.postMessage(data);
        }),
      { seed, steps },
    );

  const names = readdirSync(recordings).filter((name) => name.endsWith(".rec"));
  expect(names).toContain("first-loop.rec");
  for (const name of names) {
    const recording = readRecording(readFileSync(path.join(recordings, name)), 30);
    checkReplayable(recording);
    const hashes = readHashes(readFileSync(path.join(recordings, name.replace(/\.rec$/, ".hashes")), "utf8"));
    // The hashes file names its recording by digest: it is this recording's.
    const digest = new Uint8Array(await crypto.subtle.digest("SHA-256", Uint8Array.from(readFileSync(path.join(recordings, name)))));
    expect(hashes.recordingSha256).toBe(Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join(""));
    const expected = hashes.lines;
    const steps = recording.steps.map((step): Step => [step.hostMs, step.post, step.held, ...step.presses]);
    const got = await replay([...recording.seed], steps);
    expect(got.tickHz).toBe(recording.tickRate);
    expect(got.hashes).toHaveLength(expected.length);
    expect(firstDifference(expected, got.hashes.map((hash) => BigInt(`0x${hash}`)))).toBeNull();
    // The check can fail, and it names the step: one held button changed in step 46's post is found at step 46.
    const changed = steps.map((step, index): Step => (index === 45 ? [step[0], step[1], step[2] ^ 1, step[3], step[4], step[5], step[6]] : step));
    const other = await replay([...recording.seed], changed);
    expect(firstDifference(expected, other.hashes.map((hash) => BigInt(`0x${hash}`)))).toBe(46);
    console.log(`${name}: ${got.hashes.length} steps, every shared hash as in the file, in ${await page.evaluate(() => navigator.userAgent)}`);
  }
});
