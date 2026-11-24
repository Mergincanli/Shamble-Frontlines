// The rule tests in WebAssembly, in the browser's own engine: the float vectors (PM-0004-22), the random stream's
// vectors (PM-0004-27) and the planted counter overflow with the panic hook's text (PM-0002-53), on the test-only
// module in its release and its dev profile. No server hands the module out: this spec answers a made-up address
// from the module's own folder, so nothing here can reach a page players get. A missing module fails the test.
import path from "node:path";
import { fileURLToPath } from "node:url";
import { expect, test } from "@playwright/test";

const root = fileURLToPath(new URL("../../../..", import.meta.url));
const ORIGIN = "http://rules.invalid";
const VECTORS = ["f32-inputs.bin", "f32-expected.txt", "chacha8-words.bin"];

interface Outcome {
  f32: string;
  f32Changed: string;
  stream: string;
  wrapped: number;
  fits: number;
  thrown: string;
  panic: string;
}

for (const profile of ["release", "dev"] as const) {
  test(`the rule tests pass in the browser's WebAssembly, ${profile} profile`, async ({ page }) => {
    await page.route(`${ORIGIN}/**`, (route) => {
      const name = new URL(route.request().url()).pathname.slice(1);
      if (name === "") return route.fulfill({ contentType: "text/html", body: "<!doctype html><title>rules</title>" });
      if (name === "devtools.js" || name === "devtools_bg.wasm") {
        const contentType = name.endsWith(".js") ? "text/javascript" : "application/wasm";
        return route.fulfill({ contentType, path: path.join(root, ".gate/rules-module", profile, name) });
      }
      if (VECTORS.includes(name)) {
        return route.fulfill({ contentType: "application/octet-stream", path: path.join(root, "crates/devtools/vectors", name) });
      }
      return route.fulfill({ status: 404, body: "" });
    });
    await page.goto(`${ORIGIN}/`);
    const outcome = await page.evaluate(async (): Promise<Outcome> => {
      const bytes = async (name: string): Promise<Uint8Array> => new Uint8Array(await (await fetch(`/${name}`)).arrayBuffer());
      // Not a name the type checker can follow: the module exists only once the build has made it.
      const address = "/devtools.js";
      const glue = await import(address);
      const wasm: { memory: WebAssembly.Memory } = await glue.default({ module_or_path: "/devtools_bg.wasm" });
      const inputs = await bytes("f32-inputs.bin");
      const expected = new TextDecoder().decode(await bytes("f32-expected.txt"));
      const f32: string = glue.f32_vectors(inputs, expected);
      inputs[1] = (inputs[1] ?? 0) ^ 0x40;
      const f32Changed: string = glue.f32_vectors(inputs, expected);
      const stream: string = glue.chacha8_vectors(await bytes("chacha8-words.bin"));
      const buffer: number = glue.panic_buffer();
      const wrapped: number = glue.plant_u16_wrapping_add(65_535, 1);
      const fits: number = glue.plant_u16_add(65_534, 1);
      let thrown = "nothing was thrown";
      try {
        glue.plant_u16_add(65_535, 1);
      } catch (error) {
        thrown = error instanceof WebAssembly.RuntimeError ? "WebAssembly.RuntimeError" : String(error);
      }
      // The hook's text, read through the memory alone: no call into a trapped instance.
      const length = new DataView(wasm.memory.buffer, buffer, 8).getUint32(0, true);
      const panic = new TextDecoder().decode(new Uint8Array(wasm.memory.buffer, buffer + 8, length));
      return { f32, f32Changed: f32Changed.slice(0, 41), stream, wrapped, fits, thrown, panic };
    });
    console.log(`the rule tests, ${profile} profile: ${JSON.stringify(outcome)}`);
    expect(outcome).toMatchObject({ f32: "", stream: "", wrapped: 0, fits: 65_535, thrown: "WebAssembly.RuntimeError" });
    // The check runs in the Wasm and can fail: one changed bit of one input was found.
    expect(outcome.f32Changed).toBe("not the exact reference's results: add 4 ");
    expect(outcome.panic).toMatch(/^panicked at crates\/devtools\/src\/wasm\.rs:\d+:\d+:\nattempt to add with overflow$/);
  });
}
