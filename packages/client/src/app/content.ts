import { LOCAL_MAP, MAX_BUNDLE_BYTES } from "@shamble/sim-worker";
import type { MapDocument, Region, Shape, Spawn } from "@shamble/types";
import type { PictureContent } from "../picture.ts";

// The page's content (change-set 4b-2, step 9; the map's reading, change-set 4c): one fetch of the bundle the build put
// beside the page, a copy kept before its bytes go to the Worker, and the page's own reading of that copy only after the
// Worker said `ready`, which means the game's loader in the Wasm took the bundle whole: so `JSON.parse` never sees a
// bundle the loader refused. The page then reads the local match's map field by field into the generated
// `MapDocument` and hands it to the picture; it checks the shape the generated type names, never the loader's ranges,
// which a second copy here would let drift. Every sentence said here is fixed text, with at most a status number:
// never what the server sent, a content key or value, or a caught error's text (a parse error can quote its input).

/** Where the bundle lies: beside the page, on its own origin. Never taken from the address, a flag, storage or a message. */
export const CONTENT_URL = "./content.json";

/** The bundle as fetched: `bundle` goes to the Worker, transferred; `kept` is the page's own copy of its bytes. */
export interface FetchedContent {
  readonly bundle: ArrayBuffer;
  readonly kept: Uint8Array;
}

/** A value, or a problem in fixed words. */
export type Outcome<T> = { readonly ok: true; readonly value: T } | { readonly ok: false; readonly problem: string };

/**
 * A bundle's `format` and newest `version`, as the loader writes and reads them (crates/content/src/bundle.rs, tied
 * by `content-ties.test.mjs`).
 */
export const BUNDLE_FORMAT = "shamble-content-bundle";
export const BUNDLE_VERSION = 1;

/** What the page says when the bundle the loader took does not read as one on the page: the two are not one build. */
export const NOT_A_BUNDLE = "the content the game's loader took does not read as a bundle on the page: the page and the Wasm are from different builds";

/** What the page says when the local match's map does not read as a map on the page: the two are not one build. */
export const NOT_THE_MAP = "the map the game's loader took does not read as a map on the page: the page and the Wasm are from different builds";

/**
 * Each content kind's `format` and newest `version`, as the loader writes and reads them (crates/content/src/map.rs,
 * movement.rs; tied by `content-ties.test.mjs`), and the bundle's folder for each (bundle.rs, `FOLDERS`).
 */
export const MAP_FORMAT = "shamble-map";
export const MAP_VERSION = 1;
export const MOVEMENT_FORMAT = "shamble-movement";
export const MOVEMENT_VERSION = 1;
export const CONTENT_KINDS: ReadonlyMap<string, { readonly format: string; readonly newest: number }> = new Map([
  ["maps", { format: MAP_FORMAT, newest: MAP_VERSION }],
  ["movement", { format: MOVEMENT_FORMAT, newest: MOVEMENT_VERSION }],
]);

/** A file of the bundle as the readout names it: its path, its kind's format and its version. */
export interface ContentFile {
  readonly path: string;
  readonly format: string;
  readonly version: number;
}

/** What the page's reading gives: the picture's content, and the bundle's files for the readout. */
export interface ReadContent {
  readonly picture: PictureContent;
  readonly files: readonly ContentFile[];
}

/** One fetch of the bundle. A failure is a fixed sentence, and the page then starts no Worker. */
export async function fetchContent(fetchOnce: (url: string) => Promise<Response>): Promise<Outcome<FetchedContent>> {
  let bundle: ArrayBuffer;
  try {
    const response = await fetchOnce(CONTENT_URL);
    if (!response.ok) return { ok: false, problem: `the content could not be fetched: the server answered ${response.status}` };
    bundle = await response.arrayBuffer();
  } catch {
    return { ok: false, problem: "the content could not be fetched" };
  }
  if (bundle.byteLength === 0 || bundle.byteLength > MAX_BUNDLE_BYTES) {
    return { ok: false, problem: `the content is empty or more than the ${MAX_BUNDLE_BYTES} bytes the game's loader takes` };
  }
  // The copy first: the transfer to the Worker leaves the page's buffer empty.
  return { ok: true, value: { bundle, kept: new Uint8Array(bundle.slice(0)) } };
}

function isObject(value: unknown): value is object {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

/** An own property's value, or undefined: nothing a prototype holds is ever read. */
function own(object: object, key: string): unknown {
  return Object.hasOwn(object, key) ? (object as Readonly<Record<string, unknown>>)[key] : undefined;
}

/**
 * The page's reading of its copy, once the Worker said `ready`: the bundle's files, path to text, read field by field
 * from own properties and put into nothing else. A bundle the loader took always reads; one that does not is a fault
 * of the build, said in fixed words, and nothing of it is kept.
 */
export function readBundle(kept: Uint8Array): Outcome<ReadonlyMap<string, string>> {
  const fault = { ok: false, problem: NOT_A_BUNDLE } as const;
  let tree: unknown;
  try {
    tree = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(kept));
  } catch {
    return fault;
  }
  if (!isObject(tree) || own(tree, "format") !== BUNDLE_FORMAT || own(tree, "version") !== BUNDLE_VERSION) return fault;
  const files = own(tree, "files");
  if (!Array.isArray(files)) return fault;
  const byPath = new Map<string, string>();
  for (const entry of files) {
    if (!isObject(entry)) return fault;
    const path = own(entry, "path");
    const text = own(entry, "text");
    if (typeof path !== "string" || typeof text !== "string" || byPath.has(path)) return fault;
    byPath.set(path, text);
  }
  return { ok: true, value: byPath };
}

/** Whether `object`'s own keys are exactly `keys`: no key missing, none more (a `__proto__` key is an own key too). */
function closed(object: object, keys: readonly string[]): boolean {
  return Object.keys(object).length === keys.length && keys.every((key) => Object.hasOwn(object, key));
}

/** An array of exactly `n` finite numbers, copied (`JSON.parse` turns 1e400 into Infinity), or null. */
function corner(value: unknown, n: 2): [number, number] | null;
function corner(value: unknown, n: 3): [number, number, number] | null;
function corner(value: unknown, n: 2 | 3): number[] | null {
  if (!Array.isArray(value) || value.length !== n) return null;
  const numbers: number[] = [];
  for (const part of value as unknown[]) {
    if (typeof part !== "number" || !Number.isFinite(part)) return null;
    numbers.push(part);
  }
  return numbers;
}

/** One of `words`, or null. */
function word<W extends string>(value: unknown, words: readonly W[]): W | null {
  return typeof value === "string" && (words as readonly string[]).includes(value) ? (value as W) : null;
}

/** A string, or null (an id's grammar is the loader's). */
function text(value: unknown): string | null {
  return typeof value === "string" ? value : null;
}

/** The objects of an array, or null when it is no array or holds anything but objects. */
function objects(value: unknown): object[] | null {
  return Array.isArray(value) && (value as unknown[]).every(isObject) ? (value as object[]) : null;
}

/**
 * The local match's map, read field by field from the parsed tree of its file into the generated `MapDocument`: each
 * object by its exact key set, each member by its kind, from own properties only, nothing cast and nothing spread.
 * A tree the loader took always reads; one that does not is a fault of the build, said in fixed words.
 */
export function readMap(tree: unknown): Outcome<MapDocument> {
  const fault = { ok: false, problem: NOT_THE_MAP } as const;
  if (!isObject(tree) || !closed(tree, ["format", "version", "id", "bounds", "spawns", "shapes", "regions"])) return fault;
  if (own(tree, "format") !== MAP_FORMAT || own(tree, "version") !== MAP_VERSION) return fault;
  const id = text(own(tree, "id"));
  const boundsTree = own(tree, "bounds");
  if (id === null || !isObject(boundsTree) || !closed(boundsTree, ["min", "max"])) return fault;
  const boundsMin = corner(own(boundsTree, "min"), 3);
  const boundsMax = corner(own(boundsTree, "max"), 3);
  const spawnTrees = objects(own(tree, "spawns"));
  const shapeTrees = objects(own(tree, "shapes"));
  const regionTrees = objects(own(tree, "regions"));
  if (boundsMin === null || boundsMax === null || spawnTrees === null || shapeTrees === null || regionTrees === null) return fault;
  const spawns: Spawn[] = [];
  for (const entry of spawnTrees) {
    const spawnId = text(own(entry, "id"));
    const at = corner(own(entry, "at"), 3);
    const facing = own(entry, "facing_deg");
    if (!closed(entry, ["id", "at", "facing_deg"]) || spawnId === null || at === null) return fault;
    if (typeof facing !== "number" || !Number.isFinite(facing)) return fault;
    spawns.push({ id: spawnId, at, facing_deg: facing });
  }
  const shapes: Shape[] = [];
  for (const entry of shapeTrees) {
    const shapeId = text(own(entry, "id"));
    const min = corner(own(entry, "min"), 3);
    const max = corner(own(entry, "max"), 3);
    const type = word(own(entry, "type"), ["box", "ramp"] as const);
    if (shapeId === null || min === null || max === null || type === null) return fault;
    if (type === "box") {
      if (!closed(entry, ["id", "min", "max", "type"])) return fault;
      shapes.push({ id: shapeId, min, max, type: "box" });
    } else {
      const toward = word(own(entry, "rises_toward"), ["+x", "-x", "+z", "-z"] as const);
      if (!closed(entry, ["id", "min", "max", "type", "rises_toward"]) || toward === null) return fault;
      shapes.push({ id: shapeId, min, max, type: "ramp", rises_toward: toward });
    }
  }
  const regions: Region[] = [];
  for (const entry of regionTrees) {
    const regionId = text(own(entry, "id"));
    const kind = word(own(entry, "kind"), ["street", "grass"] as const);
    const min = corner(own(entry, "min"), 2);
    const max = corner(own(entry, "max"), 2);
    if (!closed(entry, ["id", "kind", "min", "max"]) || regionId === null || kind === null || min === null || max === null) return fault;
    regions.push({ id: regionId, kind, min, max });
  }
  return { ok: true, value: { id, bounds: { min: boundsMin, max: boundsMax }, spawns, shapes, regions } };
}

/**
 * The page's reading of its copy of the bundle: its files by path (`readBundle`), each file's own `format` and
 * `version` against its folder's kind, and the local match's map read field by field (`readMap`), for the picture.
 * Each file is parsed once.
 */
export function readContent(kept: Uint8Array): Outcome<ReadContent> {
  const bundle = readBundle(kept);
  if (!bundle.ok) return bundle;
  const fault = { ok: false, problem: NOT_A_BUNDLE } as const;
  const files: ContentFile[] = [];
  let map: MapDocument | null = null;
  for (const [path, fileText] of bundle.value) {
    const slash = path.indexOf("/");
    const kind = slash > 0 ? CONTENT_KINDS.get(path.slice(0, slash)) : undefined;
    if (kind === undefined) return fault;
    let tree: unknown;
    try {
      tree = JSON.parse(fileText);
    } catch {
      return fault;
    }
    const format = isObject(tree) ? own(tree, "format") : undefined;
    const version = isObject(tree) ? own(tree, "version") : undefined;
    if (format !== kind.format || typeof version !== "number" || !Number.isInteger(version) || version < 1 || version > kind.newest) return fault;
    files.push({ path, format: kind.format, version });
    if (path === `maps/${LOCAL_MAP}.json`) {
      const read = readMap(tree);
      if (!read.ok) return read;
      map = read.value;
    }
  }
  if (map === null) return { ok: false, problem: NOT_THE_MAP };
  return { ok: true, value: { picture: { map }, files } };
}

/** What the content needs of the link: `WorkerLink`, or a stand-in in tests. */
export interface ContentLink {
  start(padBytes: number, bundle: ArrayBuffer): void;
  readonly ready: boolean;
  readonly failure: string;
}

/** What the page's handling of the Worker needs of the link: `WorkerLink`, or a stand-in in tests. */
export interface MessageLink extends ContentLink {
  receive(event: { readonly data: unknown }): string;
  workerError(message: string | undefined): string;
  end(reason: string): boolean;
}

/**
 * One message of the Worker, as the page takes it: the link first, then the content, which reads its copy on the
 * Worker's `ready` and lets it go as soon as the match has ended, however it ended. Returns the problem that ended
 * the match (the page says it), or "".
 */
export function takeWorkerMessage(link: MessageLink, content: PageContent, event: { readonly data: unknown }): string {
  const problem = link.receive(event);
  const read = content.afterMessage(link);
  if (problem !== "") return problem;
  return read !== "" && link.end(read) ? read : "";
}

/** The Worker's own error event, as the page takes it: the link ends the match, and the content lets its copy go. */
export function takeWorkerError(link: MessageLink, content: PageContent, message: string | undefined): string {
  const problem = link.workerError(message);
  content.afterMessage(link);
  return problem;
}

/**
 * The page's copy of the bundle from the Worker's start to its `ready`: read once, the first time the link is ready,
 * and let go then, or as soon as the match ends without it (a Worker that answered `error`). What the reading gives
 * is handed on once, to `deliver` (the page gives it to the picture), and nothing of it is kept here.
 */
export class PageContent {
  #kept: Uint8Array | null = null;
  readonly #deliver: (read: ReadContent) => void;
  readonly #read: (kept: Uint8Array) => Outcome<ReadContent>;

  /** `deliver` takes what the reading gives, once; `read` is the page's reading (`readContent`), a test's may count. */
  constructor(deliver: (read: ReadContent) => void, read: (kept: Uint8Array) => Outcome<ReadContent> = readContent) {
    this.#deliver = deliver;
    this.#read = read;
  }

  /** Whether the page still holds its copy of the bundle: from the start to the Worker's first answer. */
  get holding(): boolean {
    return this.#kept !== null;
  }

  /** Starts the Worker on the fetched bundle: its bytes are transferred, and the page's copy stays here. */
  start(fetched: FetchedContent, link: ContentLink, padBytes: number): void {
    this.#kept = fetched.kept;
    link.start(padBytes, fetched.bundle);
  }

  /** After each message of the Worker: the problem the reading found (the page then ends the match), or "". */
  afterMessage(link: ContentLink): string {
    if (this.#kept === null) return "";
    if (link.failure !== "") {
      this.#kept = null;
      return "";
    }
    if (!link.ready) return "";
    const kept = this.#kept;
    this.#kept = null;
    const read = this.#read(kept);
    if (!read.ok) return read.problem;
    this.#deliver(read.value);
    return "";
  }
}
