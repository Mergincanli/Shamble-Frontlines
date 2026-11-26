// The content reader's proofs in the browser's own engine (content items 1 and 2): the number corpus, the tree
// corpus, the verdicts on every class of broken file and on the fuzz run's first inputs against the native digests,
// and the inputs at the loader's caps, on the test-only module's release profile (its dev profile runs the same
// proofs under Node: packages/sim-worker/boundary/content.test.mjs). The page also asks its own engine for every
// number of the corpus, so the corpus, which Node's engine wrote, is held against each browser's JSON.parse and
// JSON.stringify too. No server hands the module out: this spec answers a made-up address from the module's folder.
import path from "node:path";
import { fileURLToPath } from "node:url";
import { expect, test } from "@playwright/test";
import { readPlainFile } from "../../../sim-worker/boundary/plain-file.ts";

const root = fileURLToPath(new URL("../../../..", import.meta.url));
const ORIGIN = "http://rules.invalid";
const VECTORS = ["content-corpus.txt", "content-trees.txt", "content-verdicts.txt"];
const vector = (name: string): string => new TextDecoder("utf-8", { fatal: true }).decode(readPlainFile(path.join(root, "crates/devtools/vectors"), name));

interface Outcome {
  corpus: string;
  corpusChanged: string;
  trees: string;
  verdicts: string;
  verdictsChanged: string;
  limits: string;
  numbers: number;
  notThisEngines: string[];
  memory: number;
  tookMs: number;
}

test("the content reader's proofs pass in the browser's WebAssembly, and the corpus is this browser's own", async ({ page }) => {
  await page.route(`${ORIGIN}/**`, (route) => {
    const name = new URL(route.request().url()).pathname.slice(1);
    if (name === "") return route.fulfill({ contentType: "text/html", body: "<!doctype html><title>content</title>" });
    if (name === "devtools.js" || name === "devtools_bg.wasm") {
      const contentType = name.endsWith(".js") ? "text/javascript" : "application/wasm";
      return route.fulfill({ contentType, path: path.join(root, ".gate/rules-module/release", name) });
    }
    // A vector is handed out only when it is a plain file in a plain folder (plain-file.ts), never through a link.
    if (VECTORS.includes(name)) return route.fulfill({ contentType: "text/plain", body: vector(name) });
    return route.fulfill({ status: 404, body: "" });
  });
  await page.goto(`${ORIGIN}/`);
  const outcome = await page.evaluate(async (): Promise<Outcome> => {
    const text = async (name: string): Promise<string> => (await fetch(`/${name}`)).text();
    // Not a name the type checker can follow: the module exists only once the build has made it.
    const address = "/devtools.js";
    const glue = await import(address);
    const wasm: { memory: WebAssembly.Memory } = await glue.default({ module_or_path: "/devtools_bg.wasm" });
    const corpus = await text("content-corpus.txt");
    const verdictsFile = await text("content-verdicts.txt");
    const started = performance.now();
    const view = new DataView(new ArrayBuffer(8));
    const lines = corpus.split("\n").slice(1, -1);
    const notThisEngines = lines.filter((line) => {
      const [bits, written, canonical] = line.split(" ");
      const value: unknown = JSON.parse(written ?? "");
      if (typeof value !== "number") return true;
      view.setFloat64(0, value);
      return view.getBigUint64(0).toString(16).padStart(16, "0") !== bits || JSON.stringify(value) !== canonical;
    });
    const verdicts: string = glue.content_verdicts(verdictsFile);
    const limits: string = glue.content_limits();
    return {
      corpus: glue.content_corpus(corpus),
      corpusChanged: glue.content_corpus(corpus.replace("\n402c000000000000 14 14\n", "\n402c000000000001 14 14\n")),
      trees: glue.content_trees(await text("content-trees.txt")),
      verdicts,
      verdictsChanged: glue.content_verdicts("shamble-content-verdicts 1 5 2\na valid file|2|0000000000000000\n"),
      limits,
      numbers: lines.length,
      notThisEngines: notThisEngines.slice(0, 5),
      memory: wasm.memory.buffer.byteLength,
      tookMs: Math.round(performance.now() - started),
    };
  });
  console.log(`the content reader's proofs: ${JSON.stringify({ ...outcome, corpusChanged: outcome.corpusChanged.slice(0, 80) })}`);
  expect(outcome).toMatchObject({ corpus: "", trees: "", verdicts: "", limits: "", numbers: 26_653, notThisEngines: [] });
  // The checks run in the Wasm and can fail: one changed bit of one number, and a verdicts file of another seed.
  expect(outcome.corpusChanged).toContain("the reader gives 402c000000000000");
  expect(outcome.verdictsChanged).toMatch(/^this build gives\n {2}a valid file\|2\|[0-9a-f]{16}\nand the file has\n {2}a valid file\|2\|0{16}$/);
  // The module's memory only grows; its heaviest input is a bundle at the 8 MiB cap, held three times at once. A
  // backstop against gross growth only: what the loader holds for each input is proven natively (content_fuzz.rs).
  expect(outcome.memory).toBeLessThan(64 * 1024 * 1024);
  // The corpus against this engine, the verdicts and the inputs at the caps, together: about ten times what they
  // took under Node when this was written (5.6 s), and under this project's own limit for one test.
  expect(outcome.tookMs).toBeLessThan(60_000);
});
