// The page's content in a real browser, on the built development page (change-set 4b-2, step 9): one fetch of the
// bundle beside the page, its bytes handed to the Worker, whose loader judges them, and the page's own reading only
// after the Worker's `ready`. A bundle the loader refuses shows the loader's sentence as text and nothing else; a
// failed fetch shows the page's fixed sentence and starts no Worker.
import { readFileSync } from "node:fs";
import path from "node:path";
import { expect, type Page, test } from "@playwright/test";
import { needsBuild, report, running, scene } from "./report.ts";

const counts = JSON.parse(readFileSync(new URL("./render-counts.json", import.meta.url), "utf8")) as Record<string, Record<string, unknown>>;
/** The empty floor's counts: what a refused bundle leaves on screen. */
const baseline = counts["empty"] ?? {};
/** The built page's bundle: the content tool's output, as the build wrote it beside index.html. */
const built = (): string => readFileSync(path.join(needsBuild(), "content.json"), "utf8");

test.beforeAll(() => void needsBuild());

/** Serves `body` (with `status`) for the page's content address, and counts the page's requests for it. */
async function serveContent(page: Page, body: string | null, status = 200): Promise<{ readonly asked: number }> {
  const count = { asked: 0 };
  page.on("request", (request) => {
    if (new URL(request.url()).pathname === "/content.json") count.asked += 1;
  });
  if (body !== null) await page.route("**/content.json", (route) => route.fulfill({ status, contentType: "application/json", body }));
  return count;
}

test("the page fetches the bundle beside it once, and the served bytes are the built file's", async ({ page }) => {
  const count = await serveContent(page, null);
  const answered = page.waitForResponse((response) => new URL(response.url()).pathname === "/content.json");
  await page.goto("/");
  const response = await answered;
  expect(new Uint8Array(await response.body())).toEqual(new Uint8Array(readFileSync(path.join(needsBuild(), "content.json"))));
  await running(page, 30);
  expect((await report(page)).errors).toBe(0);
  expect(count.asked).toBe(1);
});

test("the Worker's loader reads the bytes the page was served: one changed value is refused at its own file and line", async ({ page }) => {
  // The map's id written with a capital letter: the id grammar refuses it, at the id's line of the map's own text.
  const bundle = JSON.parse(built()) as { files: { path: string; text: string }[] };
  const map = bundle.files.find((file) => file.path === "maps/test-scene.json");
  if (map === undefined) throw new Error("the built bundle holds no test scene");
  const line = map.text.split("\n").findIndex((text) => text === '  "id": "test-scene",') + 1;
  expect(line).toBeGreaterThan(0);
  const changed = built().replace('\\"id\\": \\"test-scene\\"', '\\"id\\": \\"Test-scene\\"');
  expect(changed).not.toBe(built());
  await serveContent(page, changed);
  await page.goto("/");
  await expect(page.locator("#notice")).toContainText(`The match stopped: the content is refused: `);
  await expect(page.locator("#notice")).toContainText(`, in maps/test-scene.json, line ${line} column `);
});

// Bundles that JSON.parse takes (but for the number) and the loader refuses, each shown only as the loader's own
// sentence for its class (read from the players' file under Node by the notes' hostile-sentences.mjs, never typed
// as a guess; test-engineer's round of 2026-11-27).
const hostile: [string, (text: string) => string, string][] = [
  ["a duplicate key", (text) => text.replace('"format": "shamble-content-bundle",', '"format": "shamble-content-bundle",\n  "format": "shamble-content-bundle",'), 'the key "format" is there twice, line 13 column 3'],
  ["a __proto__ key", (text) => text.replace('{\n  "files": [', '{\n  "__proto__": {\n    "polluted": 1\n  },\n  "files": ['), 'the key "__proto__" is not a lower-case name of 1 to 64 characters, line 2 column 3'],
  ["a number out of every range", (text) => text.replace('"version": 1', '"version": 1e400'), "the number 1e400 is too large, line 13 column 14, at version"],
  ["an extra key in a file entry", (text) => text.replace('"path": "maps/test-scene.json",', '"extra": 1,\n      "path": "maps/test-scene.json",'), 'the key "extra" is not known, line 4 column 7, at files[0]'],
  ["a source/ path", (text) => text.replace('"path": "maps/test-scene.json"', '"path": "source/test-scene.json"'), '"source/test-scene.json" is not a content file\'s path'],
  ["an unknown key named like an image tag", (text) => text.replace('{\n  "files": [', '{\n  "<img src=x onerror=alert(1)>": 1,\n  "files": ['), 'the key "<img src=x onerror=alert(1)>" is not a lower-case name of 1 to 64 characters, line 2 column 3'],
  ["a key of 1 MiB", () => `{"${"k".repeat(1 << 20)}": 1}`, `the key "${"k".repeat(64)}..." is not a lower-case name of 1 to 64 characters, line 1 column 2`],
];

for (const [name, make, said] of hostile) {
  test(`a bundle with ${name} is refused by the loader, said as text, and the scene stays empty`, async ({ page }) => {
    const text = make(built());
    expect(text).not.toBe(built());
    await serveContent(page, text);
    const dialogs: string[] = [];
    page.on("dialog", (dialog) => {
      dialogs.push(dialog.message());
      void dialog.dismiss();
    });
    await page.goto("/");
    const notice = page.locator("#notice");
    // The loader's own sentence for this class, whole, within its bound, and as plain text: no element came of it.
    await expect(notice).toHaveText(`The match stopped: the content is refused: ${said}`);
    const shown = (await notice.textContent()) ?? "";
    expect(shown.length - "The match stopped: ".length).toBeLessThanOrEqual(512);
    expect(await notice.locator("*").count()).toBe(0);
    expect(await page.locator("img").count()).toBe(0);
    expect(dialogs).toEqual([]);
    // The page itself carries on with its empty floor, drawn alone; only the match stopped, and nothing was read.
    const data = await report(page);
    expect(data.errors).toBe(1);
    expect([data.content, data.scene]).toEqual(["none", null]);
    expect((await scene(page)).map((object) => [object.id, object.kind])).toEqual([["ground", "ground"]]);
    if (baseline[data.picture.backend] !== null && baseline[data.picture.backend] !== undefined) {
      await expect.poll(async () => (await report(page)).counters).toEqual(baseline[data.picture.backend]);
    }
  });
}

test("a failed fetch of the bundle is said in the page's fixed words, and no Worker is started", async ({ page }) => {
  const count = await serveContent(page, '<img src=x onerror="alert(1)">', 404);
  await page.goto("/");
  const notice = page.locator("#notice");
  await expect(notice).toHaveText("The match did not start: the content could not be fetched: the server answered 404");
  expect(await page.locator("img").count()).toBe(0);
  expect(page.workers()).toHaveLength(0);
  expect(count.asked).toBe(1);
});
