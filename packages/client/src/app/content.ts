import { MAX_BUNDLE_BYTES } from "@shamble/sim-worker";

// The page's content (change-set 4b-2, step 9): one fetch of the bundle the build put beside the page, a copy kept
// before its bytes go to the Worker, and the page's own reading of that copy only after the Worker said `ready`, which
// means the game's loader in the Wasm took the bundle whole: so `JSON.parse` never sees a bundle the loader refused.
// Every sentence said here is fixed text, with at most a status number: never what the server sent, a content key or
// value, or a caught error's text (a parse error can quote its input).

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
 * and let go then, or as soon as the match ends without it (a Worker that answered `error`). Until the page's scene
 * draws from the files (change-set 4c, which also brings the map's reading field by field), this reading is the
 * page's check that what the loader took is a bundle, and nothing of what it reads is kept.
 */
export class PageContent {
  #kept: Uint8Array | null = null;
  readonly #read: (kept: Uint8Array) => Outcome<ReadonlyMap<string, string>>;

  /** `read` is the page's reading (`readBundle`); a test passes one that counts. */
  constructor(read: (kept: Uint8Array) => Outcome<ReadonlyMap<string, string>> = readBundle) {
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
    return read.ok ? "" : read.problem;
  }
}
