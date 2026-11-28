import { expect, test } from "vitest";
import {
  CONTENT_URL,
  type ContentLink,
  fetchContent,
  type MessageLink,
  NOT_A_BUNDLE,
  NOT_THE_MAP,
  type Outcome,
  PageContent,
  type ReadContent,
  readBundle,
  readContent,
  readMap,
  takeWorkerError,
  takeWorkerMessage,
} from "./content.ts";
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

/** A whole map document, as a map file holds one: every object kind, a ramp, a patch of ground. */
const MAP = {
  bounds: { max: [10, 5, 10], min: [-10, 0, -10] },
  format: "shamble-map",
  id: "test-scene",
  regions: [{ id: "lot", kind: "street", max: [10, 10], min: [-10, -10] }],
  shapes: [
    { id: "crate", max: [1, 1, 1], min: [0, 0, 0], type: "box" },
    { id: "slope", max: [4, 1, 2], min: [2, 0, 0], rises_toward: "+x", type: "ramp" },
  ],
  spawns: [{ at: [0, 0, -5], facing_deg: 90, id: "one" }],
  version: 1,
};
/** What `readMap` gives for `MAP`: the document less its `format` and `version`. */
const READ_MAP = { id: MAP.id, bounds: MAP.bounds, spawns: MAP.spawns, shapes: MAP.shapes, regions: MAP.regions };
/** A copy of `MAP`'s tree, to change one thing in. */
const tree = (): Record<string, unknown> => JSON.parse(JSON.stringify(MAP)) as Record<string, unknown>;
/** A movement file's head, all the page reads of it. */
const MOVEMENT_TEXT = '{\n  "format": "shamble-movement",\n  "id": "soldier",\n  "version": 1\n}\n';
/** A whole bundle the loader would take, from its files in path order. */
const bundleOf = (files: readonly { path: string; text: string }[]): string =>
  `${JSON.stringify({ files, format: "shamble-content-bundle", version: 1 }, null, 2)}\n`;
const FULL_BUNDLE = bundleOf([
  { path: "maps/test-scene.json", text: `${JSON.stringify(MAP, null, 2)}\n` },
  { path: "movement/soldier.json", text: MOVEMENT_TEXT },
]);

/** The link with a stand-in Worker, a count of the page's readings of its copy, and what the content delivered. */
function started(answer: () => Promise<Response> = () => Promise.resolve(new Response(encode(FULL_BUNDLE)))) {
  const sent: unknown[] = [];
  const link = new WorkerLink({ postMessage: (message) => sent.push(message), terminate: () => undefined }, () => 0);
  const readings: Outcome<ReadContent>[] = [];
  const delivered: ReadContent[] = [];
  const content = new PageContent(
    (read) => delivered.push(read),
    (kept) => {
      const read = readContent(kept);
      readings.push(read);
      return read;
    },
  );
  const { asked, fetchOnce } = fetcher(answer);
  return { link, sent, readings, delivered, content, asked, fetchOnce };
}

test("a Worker that answers error: the copy is never read, and the bundle was fetched once", async () => {
  const { link, sent, readings, delivered, content, asked, fetchOnce } = started();
  const fetched = await fetchContent(fetchOnce);
  if (!fetched.ok) throw new Error(fetched.problem);
  content.start(fetched.value, link, 0);
  expect(sent).toHaveLength(1);
  expect(link.onMessage({ type: "error", message: "the content is refused: a key that is not a field of this kind" })).not.toBe("");
  expect(content.afterMessage(link)).toBe("");
  // A ready that comes after the end is ignored by the link, and the copy is gone.
  link.onMessage({ type: "ready", blockBytes: 44, tickHz: 30 });
  expect(content.afterMessage(link)).toBe("");
  expect([readings, delivered, asked]).toEqual([[], [], [CONTENT_URL]]);
});

test("a Worker that answers ready: the copy is read once and delivered once, the map field by field and the files by path", async () => {
  const { link, readings, delivered, content, fetchOnce } = started();
  const fetched = await fetchContent(fetchOnce);
  if (!fetched.ok) throw new Error(fetched.problem);
  content.start(fetched.value, link, 0);
  // Before the answer, nothing is read.
  expect(content.afterMessage(link)).toBe("");
  expect(readings).toHaveLength(0);
  expect(link.onMessage({ type: "ready", blockBytes: 44, tickHz: 30 })).toBe("");
  expect(content.afterMessage(link)).toBe("");
  expect(content.afterMessage(link)).toBe("");
  expect([readings.length, delivered.length]).toEqual([1, 1]);
  expect(delivered[0]).toStrictEqual({
    picture: { map: READ_MAP },
    files: [
      { path: "maps/test-scene.json", format: "shamble-map", version: 1 },
      { path: "movement/soldier.json", format: "shamble-movement", version: 1 },
    ],
  });
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
  const { link, readings, delivered, content, fetchOnce } = started();
  const fetched = await fetchContent(fetchOnce);
  if (!fetched.ok) throw new Error(fetched.problem);
  content.start(fetched.value, link, 0);
  return { link, readings, delivered, content };
}

test("however the match ends before the Worker's ready, the page lets its copy of the bundle go (engine-architect's round of 2026-11-27)", async () => {
  // The loader refused: the Worker's error message ends the match, and the copy goes with it, never read.
  const refused = await wired();
  expect(refused.content.holding).toBe(true);
  const said = "the content is refused: a key that is not a field of this kind";
  expect(takeWorkerMessage(refused.link, refused.content, { data: { type: "error", message: said } })).toBe(said);
  expect([refused.content.holding, refused.readings.length, refused.delivered.length]).toEqual([false, 0, 0]);
  // The Worker's own error event (a script that did not load): the same.
  const failed = await wired();
  expect(takeWorkerError(failed.link, failed.content, undefined)).toBe("the Worker failed");
  expect([failed.content.holding, failed.readings.length, failed.delivered.length]).toEqual([false, 0, 0]);
  // A message the page does not know.
  const unknown = await wired();
  expect(takeWorkerMessage(unknown.link, unknown.content, { data: { type: "surprise" } })).toBe("the Worker sent a message the page does not know");
  expect([unknown.content.holding, unknown.delivered.length]).toEqual([false, 0]);
});

test("on the Worker's ready the page reads its copy once and lets it go; a copy that reads as no bundle ends the match", async () => {
  const ready = await wired();
  expect(takeWorkerMessage(ready.link, ready.content, { data: { type: "ready", blockBytes: 44, tickHz: 30 } })).toBe("");
  expect([ready.content.holding, ready.readings.length, ready.delivered.length, ready.link.failure]).toEqual([false, 1, 1, ""]);
  // Later messages read and deliver nothing again.
  takeWorkerMessage(ready.link, ready.content, { data: { type: "ready", blockBytes: 44, tickHz: 30 } });
  expect([ready.readings.length, ready.delivered.length]).toEqual([1, 1]);
  // The loader took it, the page cannot read it as a bundle: the page says so, and the match ends.
  const { link, content, fetchOnce } = started(() => Promise.resolve(new Response(encode('{"files": [], "format": "x", "version": 1}'))));
  const fetched = await fetchContent(fetchOnce);
  if (!fetched.ok) throw new Error(fetched.problem);
  content.start(fetched.value, link, 0);
  expect(takeWorkerMessage(link, content, { data: { type: "ready", blockBytes: 44, tickHz: 30 } })).toBe(NOT_A_BUNDLE);
  expect([content.holding, link.failure]).toEqual([false, NOT_A_BUNDLE]);
  // A bundle whose map does not read: the match ends in the map's fixed words, and nothing is delivered.
  const noMap = started(() => Promise.resolve(new Response(encode(bundleOf([{ path: "movement/soldier.json", text: MOVEMENT_TEXT }])))));
  const noMapFetched = await fetchContent(noMap.fetchOnce);
  if (!noMapFetched.ok) throw new Error(noMapFetched.problem);
  noMap.content.start(noMapFetched.value, noMap.link, 0);
  expect(takeWorkerMessage(noMap.link, noMap.content, { data: { type: "ready", blockBytes: 44, tickHz: 30 } })).toBe(NOT_THE_MAP);
  expect([noMap.content.holding, noMap.delivered.length, noMap.link.failure]).toEqual([false, 0, NOT_THE_MAP]);
});

test("the map reads field by field from own properties into the generated type, every object kind", () => {
  expect(readMap(tree())).toStrictEqual({ ok: true, value: READ_MAP });
  // A tree that `JSON.parse` made: the same.
  expect(readMap(JSON.parse(JSON.stringify(MAP)))).toStrictEqual({ ok: true, value: READ_MAP });
});

test("one key more in any object of the map gives the map's fixed sentence", () => {
  const extra: ((map: Record<string, unknown>) => void)[] = [
    (map) => (map["extra"] = 1),
    (map) => ((map["bounds"] as Record<string, unknown>)["extra"] = 1),
    (map) => ((map["spawns"] as Record<string, unknown>[])[0]!["extra"] = 1),
    (map) => ((map["shapes"] as Record<string, unknown>[])[0]!["extra"] = 1),
    (map) => ((map["shapes"] as Record<string, unknown>[])[1]!["extra"] = 1),
    (map) => ((map["regions"] as Record<string, unknown>[])[0]!["extra"] = 1),
  ];
  for (const add of extra) {
    const map = tree();
    add(map);
    expect(readMap(map)).toEqual({ ok: false, problem: NOT_THE_MAP });
  }
  // And one key less.
  const less = tree();
  delete less["regions"];
  expect(readMap(less)).toEqual({ ok: false, problem: NOT_THE_MAP });
});

test("a member of the wrong kind gives the map's fixed sentence", () => {
  const wrong: ((map: Record<string, unknown>) => void)[] = [
    (map) => (map["id"] = 7),
    (map) => (map["format"] = "shamble-maps"),
    (map) => (map["version"] = 2),
    (map) => ((map["bounds"] as Record<string, unknown>)["min"] = [-10, 0]),
    (map) => ((map["bounds"] as Record<string, unknown>)["max"] = [10, "5", 10]),
    (map) => ((map["bounds"] as Record<string, unknown>)["max"] = [10, Number.POSITIVE_INFINITY, 10]),
    (map) => ((map["spawns"] as Record<string, unknown>[])[0]!["facing_deg"] = "90"),
    (map) => ((map["spawns"] as Record<string, unknown>[])[0]!["id"] = 1),
    (map) => ((map["shapes"] as Record<string, unknown>[])[0]!["type"] = "wedge"),
    (map) => ((map["shapes"] as Record<string, unknown>[])[1]!["rises_toward"] = "up"),
    (map) => ((map["regions"] as Record<string, unknown>[])[0]!["kind"] = "water"),
    (map) => ((map["regions"] as Record<string, unknown>[])[0]!["min"] = [-10, -10, 0]),
    (map) => (map["shapes"] = {}),
    (map) => (map["spawns"] = [7]),
  ];
  for (const change of wrong) {
    const map = tree();
    change(map);
    expect(readMap(map)).toEqual({ ok: false, problem: NOT_THE_MAP });
  }
  // A number out of every range reaches the page as Infinity from `JSON.parse`: refused too.
  const parsed: unknown = JSON.parse(JSON.stringify(MAP).replace('"facing_deg":90', '"facing_deg":1e400'));
  expect(readMap(parsed)).toEqual({ ok: false, problem: NOT_THE_MAP });
  // Not an object at all.
  for (const value of [null, [], "map", 7]) expect(readMap(value)).toEqual({ ok: false, problem: NOT_THE_MAP });
});

test("rises_toward belongs to a ramp alone: a box with it, and a ramp without it, are refused", () => {
  const box = tree();
  (box["shapes"] as Record<string, unknown>[])[0]!["rises_toward"] = "+x";
  expect(readMap(box)).toEqual({ ok: false, problem: NOT_THE_MAP });
  const ramp = tree();
  delete (ramp["shapes"] as Record<string, unknown>[])[1]!["rises_toward"];
  expect(readMap(ramp)).toEqual({ ok: false, problem: NOT_THE_MAP });
});

test("nothing is read through a prototype: a map without its own regions is refused while Object.prototype has some", () => {
  const map = tree();
  delete map["regions"];
  const proto = Object.prototype as Record<string, unknown>;
  try {
    proto["regions"] = [];
    expect(readMap(map)).toEqual({ ok: false, problem: NOT_THE_MAP });
  } finally {
    delete proto["regions"];
  }
});

test("every refusal is one of the two fixed sentences, which quote nothing they read", () => {
  const marked = "<img src=x onerror=alert(1)>";
  const inputs: unknown[] = [{ ...tree(), [marked]: marked }, { ...tree(), id: [marked] }, marked];
  for (const input of inputs) {
    const read = readMap(input);
    expect(read).toEqual({ ok: false, problem: NOT_THE_MAP });
  }
  const bundle = readContent(encode(bundleOf([{ path: `maps/${marked}.json`, text: marked }])));
  expect(bundle).toEqual({ ok: false, problem: NOT_A_BUNDLE });
  for (const sentence of [NOT_A_BUNDLE, NOT_THE_MAP]) expect(sentence).not.toMatch(/[<>"{}]/);
});

test("the bundle's files: each folder's kind, format and version, and the local match's map by its path", () => {
  const mapText = `${JSON.stringify(MAP, null, 2)}\n`;
  // The local map is taken by its path, whatever stands before it.
  const other = `${JSON.stringify({ ...MAP, id: "aaa", bounds: { max: [1, 1, 1], min: [0, 0, 0] } }, null, 2)}\n`;
  const two = readContent(encode(bundleOf([{ path: "maps/aaa.json", text: other }, { path: "maps/test-scene.json", text: mapText }])));
  if (!two.ok) throw new Error(two.problem);
  expect(two.value.picture.map).toStrictEqual(READ_MAP);
  expect(two.value.files.map((file) => file.path)).toEqual(["maps/aaa.json", "maps/test-scene.json"]);
  // No local map; a folder no kind has; a file whose format is not its folder's; a version newer than the page reads.
  expect(readContent(encode(bundleOf([{ path: "maps/aaa.json", text: other }])))).toEqual({ ok: false, problem: NOT_THE_MAP });
  expect(readContent(encode(bundleOf([{ path: "sounds/test-scene.json", text: mapText }])))).toEqual({ ok: false, problem: NOT_A_BUNDLE });
  expect(readContent(encode(bundleOf([{ path: "movement/test-scene.json", text: mapText }])))).toEqual({ ok: false, problem: NOT_A_BUNDLE });
  const newer = mapText.replace('"version": 1', '"version": 2');
  expect(readContent(encode(bundleOf([{ path: "maps/test-scene.json", text: newer }])))).toEqual({ ok: false, problem: NOT_A_BUNDLE });
  expect(readContent(encode(bundleOf([{ path: "maps/test-scene.json", text: "not json" }])))).toEqual({ ok: false, problem: NOT_A_BUNDLE });
});

test("the link the content starts is the page's own link type", () => {
  // `WorkerLink` gives what `PageContent` and the page's handling need of a link: this does not compile if they drift.
  const link: ContentLink = new WorkerLink({ postMessage: () => undefined, terminate: () => undefined }, () => 0);
  const handled: MessageLink = new WorkerLink({ postMessage: () => undefined, terminate: () => undefined }, () => 0);
  expect([link.ready, link.failure, handled.failure]).toEqual([false, "", ""]);
});
