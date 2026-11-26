// The static world's proof in the browser's own engine (PM-0004-32): the trees the test-only module builds and the
// rays it casts, in its release profile, against the native build's committed lines (its dev profile runs the same
// check under Node: packages/sim-worker/boundary/static-world.test.mjs). No server hands the module out: this spec
// answers a made-up address from the module's folder.
import path from "node:path";
import { fileURLToPath } from "node:url";
import { expect, test } from "@playwright/test";
import { readPlainFile } from "../../../sim-worker/boundary/plain-file.ts";

const root = fileURLToPath(new URL("../../../..", import.meta.url));
const ORIGIN = "http://rules.invalid";
const VECTOR = "static-world.txt";

test("the static world's trees and rays in the browser's WebAssembly are the native build's", async ({ page }) => {
  await page.route(`${ORIGIN}/**`, (route) => {
    const name = new URL(route.request().url()).pathname.slice(1);
    if (name === "") return route.fulfill({ contentType: "text/html", body: "<!doctype html><title>static world</title>" });
    if (name === "devtools.js" || name === "devtools_bg.wasm") {
      const contentType = name.endsWith(".js") ? "text/javascript" : "application/wasm";
      return route.fulfill({ contentType, path: path.join(root, ".gate/rules-module/release", name) });
    }
    // The vector is handed out only when it is a plain file in a plain folder (plain-file.ts), never through a link.
    if (name === VECTOR) {
      const body = new TextDecoder("utf-8", { fatal: true }).decode(readPlainFile(path.join(root, "crates/devtools/vectors"), VECTOR));
      return route.fulfill({ contentType: "text/plain", body });
    }
    return route.fulfill({ status: 404, body: "" });
  });
  await page.goto(`${ORIGIN}/`);
  const outcome = await page.evaluate(async (): Promise<{ same: string; changed: string; maps: number; tookMs: number }> => {
    const lines = await (await fetch("/static-world.txt")).text();
    // Not a name the type checker can follow: the module exists only once the build has made it.
    const address = "/devtools.js";
    const glue = await import(address);
    await glue.default({ module_or_path: "/devtools_bg.wasm" });
    const started = performance.now();
    const same: string = glue.static_world(lines);
    // The compare bites: the full world's ray digest with its last digit changed.
    const line = lines.split("\n")[8] ?? "";
    const changed: string = glue.static_world(lines.replace(line, `${line.slice(0, -1)}${line.endsWith("0") ? "1" : "0"}`));
    return { same, changed, maps: lines.split("\n").length - 2, tookMs: Math.round(performance.now() - started) };
  });
  console.log(`the static world's proof: ${JSON.stringify({ ...outcome, changed: outcome.changed.slice(0, 60) })}`);
  expect(outcome).toMatchObject({ same: "", maps: 12 });
  expect(outcome.changed).toMatch(/^this build gives\n {2}map 7 shapes 512 depth \d+ tree [0-9a-f]{16} rays \d+ [0-9a-f]{16}\nand the file has\n {2}map 7 /);
  expect(outcome.tookMs).toBeLessThan(60_000);
});
