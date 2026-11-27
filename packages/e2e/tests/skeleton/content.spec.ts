// The page's content in a real browser, on the built development page (change-set 4b-2, step 9): one fetch of the
// bundle beside the page, its bytes handed to the Worker, whose loader judges them, and the page's own reading only
// after the Worker's `ready`. A bundle the loader refuses shows the loader's sentence as text and nothing else; a
// failed fetch shows the page's fixed sentence and starts no Worker.
import { readFileSync } from "node:fs";
import path from "node:path";
import { expect, type Page, test } from "@playwright/test";
import { needsBuild, report, running } from "./report.ts";

const baseline = JSON.parse(readFileSync(new URL("./render-counts.json", import.meta.url), "utf8")) as Record<string, unknown>;
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

// Bundles that JSON.parse takes (but for the number) and the loader refuses, each shown only as the loader's sentence.
const hostile: [string, (text: string) => string][] = [
  ["a duplicate key", (text) => text.replace('"format": "shamble-content-bundle",', '"format": "shamble-content-bundle",\n  "format": "shamble-content-bundle",')],
  ["a __proto__ key", (text) => text.replace('{\n  "files": [', '{\n  "__proto__": {\n    "polluted": 1\n  },\n  "files": [')],
  ["a number out of every range", (text) => text.replace('"version": 1', '"version": 1e400')],
  ["an extra key in a file entry", (text) => text.replace('"path": "maps/test-scene.json",', '"extra": 1,\n      "path": "maps/test-scene.json",')],
  ["a source/ path", (text) => text.replace('"path": "maps/test-scene.json"', '"path": "source/test-scene.json"')],
  ["an unknown key named like an image tag", (text) => text.replace('{\n  "files": [', '{\n  "<img src=x onerror=alert(1)>": 1,\n  "files": [')],
  ["a key of 1 MiB", () => `{"${"k".repeat(1 << 20)}": 1}`],
];

for (const [name, make] of hostile) {
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
    await expect(notice).toContainText("The match stopped: the content is refused: ");
    // The loader's sentence, whole and within its bound, as plain text: no element came of it.
    const said = (await notice.textContent()) ?? "";
    expect(said.length - "The match stopped: ".length).toBeLessThanOrEqual(512);
    expect(await notice.locator("*").count()).toBe(0);
    expect(await page.locator("img").count()).toBe(0);
    expect(dialogs).toEqual([]);
    // The page itself carries on with its empty scene; only the match stopped.
    const data = await report(page);
    expect(data.errors).toBe(1);
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
