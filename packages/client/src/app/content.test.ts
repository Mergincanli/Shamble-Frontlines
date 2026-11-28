import { expect, test } from "vitest";
import { CONTENT_URL, type ContentLink, fetchContent, type MessageLink, NOT_A_BUNDLE, PageContent, readBundle, takeWorkerError, takeWorkerMessage } from "./content.ts";
import { WorkerLink } from "./worker-link.ts";

/** A bundle's text as the loader writes one: its files in path order, each file's own text inside as a string. */
const BUNDLE_TEXT = `${JSON.stringify(
  {
    files: [
      { path: "maps/test-scene.json", text: '{\n  "format": "shamble-map",\n  "id": "test-scene"\n}\n' },
      { path: "movement/soldier.json", text: '{\n  "format": "shamble-movement",\n  "id": "soldier"\n}\n' },
    ],
    format: "shamble-content-bundle",
    version: 1,
  },
  null,
  2,
)}\n`;
const encode = (text: string): Uint8Array<ArrayBuffer> => new TextEncoder().encode(text);

/** A fetch that answers every call with `answer` and counts the addresses it was asked for. */
function fetcher(answer: () => Promise<Response>) {
  const asked: string[] = [];
  return { asked, fetchOnce: (url: string): Promise<Response> => (asked.push(url), answer()) };
}

test("one fetch of the bundle beside the page; the copy kept is the bytes fetched, still whole after the transfer", async () => {
  const { asked, fetchOnce } = fetcher(() => Promise.resolve(new Response(encode(BUNDLE_TEXT))));
  const fetched = await fetchContent(fetchOnce);
  expect(asked).toEqual([CONTENT_URL]);
  expect(CONTENT_URL).toBe("./content.json");
  if (!fetched.ok) throw new Error(fetched.problem);
  const sent = new Uint8Array(fetched.value.bundle.slice(0));
  expect(sent).toEqual(encode(BUNDLE_TEXT));
  // The transfer to the Worker empties the page's buffer; the kept copy is its own.
  structuredClone(fetched.value.bundle, { transfer: [fetched.value.bundle] });
  expect(fetched.value.bundle.byteLength).toBe(0);
  expect(fetched.value.kept).toEqual(sent);
});

test("what the page says of a failed fetch is fixed text, with at most the status number: never what the server or an error said", async () => {
  const marked = '<img src=x onerror="alert(1)">';
  const cases: [() => Promise<Response>, string][] = [
    [() => Promise.resolve(new Response(marked, { status: 404, statusText: marked })), "the content could not be fetched: the server answered 404"],
    [() => Promise.resolve(new Response(marked, { status: 500 })), "the content could not be fetched: the server answered 500"],
    [() => Promise.reject(new TypeError(marked)), "the content could not be fetched"],
    [() => Promise.resolve(new Response(new Uint8Array(0))), "the content is empty or more than the 8388608 bytes the game's loader takes"],
    [() => Promise.resolve(new Response(new Uint8Array(8 * 1024 * 1024 + 1))), "the content is empty or more than the 8388608 bytes the game's loader takes"],
  ];
  for (const [answer, said] of cases) {
    const { asked, fetchOnce } = fetcher(answer);
    const fetched = await fetchContent(fetchOnce);
    expect(fetched).toEqual({ ok: false, problem: said });
    expect(asked).toHaveLength(1);
  }
  // A bundle at the loader's cap is fetched (what it holds is the loader's to judge).
  expect((await fetchContent(fetcher(() => Promise.resolve(new Response(new Uint8Array(8 * 1024 * 1024)))).fetchOnce)).ok).toBe(true);
});

test("the page reads a bundle the loader took as its files by path, from own fields only", () => {
  const read = readBundle(encode(BUNDLE_TEXT));
  if (!read.ok) throw new Error(read.problem);
  expect([...read.value]).toEqual([
    ["maps/test-scene.json", '{\n  "format": "shamble-map",\n  "id": "test-scene"\n}\n'],
    ["movement/soldier.json", '{\n  "format": "shamble-movement",\n  "id": "soldier"\n}\n'],
  ]);
  // A key named `__proto__` is an own field like any other after JSON.parse: it reaches no prototype and is not read.
  const withProto = BUNDLE_TEXT.replace('"files": [', '"__proto__": { "files": 7 },\n  "files": [');
  expect(readBundle(encode(withProto)).ok).toBe(true);
  expect(({} as Record<string, unknown>)["files"]).toBeUndefined();
});

test("what the page cannot read as a bundle is said in fixed words, and nothing of it is kept", () => {
  const faults = [
    "not json at all: <img src=x>",
    '"a bundle"',
    "[]",
    BUNDLE_TEXT.replace('"shamble-content-bundle"', '"shamble-map"'),
    BUNDLE_TEXT.replace('"version": 1', '"version": 2'),
    BUNDLE_TEXT.replace('"version": 1', '"version": "1"'),
    '{"files": {}, "format": "shamble-content-bundle", "version": 1}',
    '{"files": ["maps/test-scene.json"], "format": "shamble-content-bundle", "version": 1}',
    BUNDLE_TEXT.replace('"path": "movement/soldier.json"', '"path": "maps/test-scene.json"'),
    BUNDLE_TEXT.replace('"text": "{\\n  \\"format\\": \\"shamble-map\\"', '"text": 7, "x": "{\\n  \\"format\\": \\"shamble-map\\"'),
  ];
  for (const text of faults) expect(readBundle(encode(text)), text).toEqual({ ok: false, problem: NOT_A_BUNDLE });
  // Bytes that are no UTF-8 at all.
  expect(readBundle(new Uint8Array([0xff, 0xfe]))).toEqual({ ok: false, problem: NOT_A_BUNDLE });
  // The fixed sentence quotes nothing it read.
  expect(NOT_A_BUNDLE).not.toMatch(/[<>"{}]/);
});

/** The link with a stand-in Worker, and a count of the page's readings of its copy. */
function started(answer: () => Promise<Response> = () => Promise.resolve(new Response(encode(BUNDLE_TEXT)))) {
  const sent: unknown[] = [];
  const link = new WorkerLink({ postMessage: (message) => sent.push(message), terminate: () => undefined }, () => 0);
  const readings: ReturnType<typeof readBundle>[] = [];
  const content = new PageContent((kept) => {
    const read = readBundle(kept);
    readings.push(read);
    return read;
  });
  const { asked, fetchOnce } = fetcher(answer);
  return { link, sent, readings, content, asked, fetchOnce };
}

test("a Worker that answers error: the copy is never read, and the bundle was fetched once", async () => {
  const { link, sent, readings, content, asked, fetchOnce } = started();
  const fetched = await fetchContent(fetchOnce);
  if (!fetched.ok) throw new Error(fetched.problem);
  content.start(fetched.value, link, 0);
  expect(sent).toHaveLength(1);
  expect(link.onMessage({ type: "error", message: "the content is refused: a key that is not a field of this kind" })).not.toBe("");
  expect(content.afterMessage(link)).toBe("");
  // A ready that comes after the end is ignored by the link, and the copy is gone.
  link.onMessage({ type: "ready", blockBytes: 44, tickHz: 30 });
  expect(content.afterMessage(link)).toBe("");
  expect([readings, asked]).toEqual([[], [CONTENT_URL]]);
});

test("a Worker that answers ready: the copy is read once, and its files are the bundle's by path", async () => {
  const { link, readings, content, fetchOnce } = started();
  const fetched = await fetchContent(fetchOnce);
  if (!fetched.ok) throw new Error(fetched.problem);
  content.start(fetched.value, link, 0);
  // Before the answer, nothing is read.
  expect(content.afterMessage(link)).toBe("");
  expect(readings).toHaveLength(0);
  expect(link.onMessage({ type: "ready", blockBytes: 44, tickHz: 30 })).toBe("");
  expect(content.afterMessage(link)).toBe("");
  expect(content.afterMessage(link)).toBe("");
  expect(readings).toHaveLength(1);
  const [read] = readings;
  if (read === undefined || !read.ok) throw new Error("the copy was not read as a bundle");
  expect([...read.value.keys()]).toEqual(["maps/test-scene.json", "movement/soldier.json"]);
});

test("a bundle the page cannot read after the loader took it ends the match in fixed words", async () => {
  const { link, content, fetchOnce } = started(() => Promise.resolve(new Response(encode('{"files": [], "format": "x", "version": 1}'))));
  const fetched = await fetchContent(fetchOnce);
  if (!fetched.ok) throw new Error(fetched.problem);
  content.start(fetched.value, link, 0);
  link.onMessage({ type: "ready", blockBytes: 44, tickHz: 30 });
  expect(content.afterMessage(link)).toBe(NOT_A_BUNDLE);
});

/** The page's own handling of the Worker (boot.ts calls the same two functions), on a link and its copy. */
async function wired() {
  const { link, readings, content, fetchOnce } = started();
  const fetched = await fetchContent(fetchOnce);
  if (!fetched.ok) throw new Error(fetched.problem);
  content.start(fetched.value, link, 0);
  return { link, readings, content };
}

test("however the match ends before the Worker's ready, the page lets its copy of the bundle go (engine-architect's round of 2026-11-27)", async () => {
  // The loader refused: the Worker's error message ends the match, and the copy goes with it, never read.
  const refused = await wired();
  expect(refused.content.holding).toBe(true);
  const said = "the content is refused: a key that is not a field of this kind";
  expect(takeWorkerMessage(refused.link, refused.content, { data: { type: "error", message: said } })).toBe(said);
  expect([refused.content.holding, refused.readings.length]).toEqual([false, 0]);
  // The Worker's own error event (a script that did not load): the same.
  const failed = await wired();
  expect(takeWorkerError(failed.link, failed.content, undefined)).toBe("the Worker failed");
  expect([failed.content.holding, failed.readings.length]).toEqual([false, 0]);
  // A message the page does not know.
  const unknown = await wired();
  expect(takeWorkerMessage(unknown.link, unknown.content, { data: { type: "surprise" } })).toBe("the Worker sent a message the page does not know");
  expect(unknown.content.holding).toBe(false);
});

test("on the Worker's ready the page reads its copy once and lets it go; a copy that reads as no bundle ends the match", async () => {
  const ready = await wired();
  expect(takeWorkerMessage(ready.link, ready.content, { data: { type: "ready", blockBytes: 44, tickHz: 30 } })).toBe("");
  expect([ready.content.holding, ready.readings.length, ready.link.failure]).toEqual([false, 1, ""]);
  // Later messages read nothing again.
  takeWorkerMessage(ready.link, ready.content, { data: { type: "ready", blockBytes: 44, tickHz: 30 } });
  expect(ready.readings).toHaveLength(1);
  // The loader took it, the page cannot read it as a bundle: the page says so, and the match ends.
  const { link, content, fetchOnce } = started(() => Promise.resolve(new Response(encode('{"files": [], "format": "x", "version": 1}'))));
  const fetched = await fetchContent(fetchOnce);
  if (!fetched.ok) throw new Error(fetched.problem);
  content.start(fetched.value, link, 0);
  expect(takeWorkerMessage(link, content, { data: { type: "ready", blockBytes: 44, tickHz: 30 } })).toBe(NOT_A_BUNDLE);
  expect([content.holding, link.failure]).toEqual([false, NOT_A_BUNDLE]);
});

test("the link the content starts is the page's own link type", () => {
  // `WorkerLink` gives what `PageContent` and the page's handling need of a link: this does not compile if they drift.
  const link: ContentLink = new WorkerLink({ postMessage: () => undefined, terminate: () => undefined }, () => 0);
  const handled: MessageLink = new WorkerLink({ postMessage: () => undefined, terminate: () => undefined }, () => 0);
  expect([link.ready, link.failure, handled.failure]).toEqual([false, "", ""]);
});
