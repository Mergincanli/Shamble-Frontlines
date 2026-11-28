// lib.ts: the page's logic, pure and Node-testable (u7-brief.md rev 1.1 Block T2 with the advisor's C1, C4, C5, C9):
// the link, the hash file, the stream framing, the WebSocket tag, the Wasm boundary's packing, the bounded inbox, the
// npt-log records and the transport chooser. It touches no global: the Worker and the page pass what it needs.
// Never put the token, a URL or the bind value into a status, a record or a console line.

export type Suite = "smoke" | "cert-14d" | "fallback" | "mds" | "wt-attempts";
export type Transport = "wt" | "ws";
export const SUITES: readonly Suite[] = ["smoke", "cert-14d", "fallback", "mds", "wt-attempts"];
export const TRANSPORT_NAME: Readonly<Record<Transport, string>> = { wt: "WebTransport", ws: "WebSocket" };

/** The status line's texts, byte for byte (U8's PAGE.playing reads "<name> · Playing", U+00B7). */
export const STATUS = {
  starting: "Starting",
  connecting: (t: Transport): string => `${TRANSPORT_NAME[t]} · Connecting`,
  joined: (t: Transport): string => `${TRANSPORT_NAME[t]} · Joined`,
  playing: (t: Transport): string => `${TRANSPORT_NAME[t]} · Playing`,
  ended: (why: string): string => `Ended: ${why}`,
  badLink: (why: string): string => `Bad link: ${why}`,
};

// ---- The numbers (catalogue docs/design/netcode-proof-messages.md; plan.md:909-910; C5) --------------------------

/** WebTransport's UDP port is this plus the room's NN; WebSocket's TCP port the next. */
export const WT_PORT_BASE = 30001;
export const WS_PORT_BASE = 31001;
export const HASH_BYTES = 32;
export const HASH_FILE_MAX = 1024;
const HEX_DIGITS_PER_BYTE = 2;
const HEX_RADIX = 16;
const ALL_BITS = 0xff;
/** Inbox record head: channel u8, arrival ns u64, length u32. Outbox record head: channel u8, length u32. */
const INBOX_HEAD = 13;
const INBOX_NS_AT = 1;
const INBOX_LEN_AT = 9;
const OUTBOX_HEAD = 5;
const OUTBOX_LEN_AT = 1;
const LIMITS_COUNT = 6;
const TAG_VALUES = 256;
/** A control message is at least its 2 B length and 1 B type; a full state its 4 B length and 1 B type. */
const CONTROL_HEAD = 2;
const FULL_STATE_HEAD = 4;
const MIN_TYPE = 1;
export const NS_PER_MS = 1e6;
export const NS_PER_S = 1e9;
const CLOCK_READS = 1000;
export const INBOX_MAX = 512;
export const RECORDS_MAX = 4096;
export const ATTEMPT_DEADLINE_MS = 10_000;

// ---- The link (U8's URL contract, wip/u8-build.patch:115-123; C4) ------------------------------------------------

export interface Link {
  readonly suite: Suite;
  /** NN, two digits, 00-31. */
  readonly room: string;
  readonly transport: Transport;
  readonly attempt: number;
  readonly token: string;
  /** C4's control: the room's 32 bytes each XOR 0xff (suite `wt-attempts` only). */
  readonly wrongHash: boolean;
  /** The armed page bite (the bites dist only), else null. */
  readonly bite: string | null;
}
export type Parsed<T> = { readonly ok: true; readonly value: T } | { readonly ok: false; readonly why: string };

/* eslint-disable ts-patterns/require-regex-example -- each has its examples above; no outside link in client code */
// U6a's slot token: "nptj_" then 22 base64url characters. Matches "nptj_" + 22 "A"; rejects "nptj_AAA".
const TOKEN_FORM = /^nptj_[A-Za-z0-9_-]{22}$/;
// A query value: 1-40 of letters, digits, "/", "_", "-". Matches "cert-14d"; rejects "a%20b" and "".
const VALUE_FORM = /^[A-Za-z0-9/_-]{1,40}$/;
// A room: two digits 00-31. Matches "07" and "31"; rejects "7", "32" and "007".
const ROOM_FORM = /^(0[0-9]|[12][0-9]|3[01])$/;
// An attempt: 0-99 without a leading zero. Matches "0" and "19"; rejects "01" and "100".
const ATTEMPT_FORM = /^(0|[1-9][0-9]?)$/;
// The hash: 64 lowercase hex digits. Matches "00ff" repeated 16 times; rejects any upper-case "A" or 63 digits.
const HEX64_FORM = /^[0-9a-f]{64}$/;
// A GREASE brand such as "Not)A;Brand" or "Not_A Brand". Matches "Not/A)Brand"; rejects "Chromium".
const GREASE_BRAND = /not.?a.?brand/i;
/* eslint-enable ts-patterns/require-regex-example */

/** Suites that try WebTransport only and never fall back (cert-14d's attempt rules: C4). */
export const NO_FALLBACK: readonly Suite[] = ["cert-14d", "wt-attempts"];
/** Suites whose measurement needs WebTransport first. */
const WT_ONLY: readonly Suite[] = ["cert-14d", "wt-attempts", "fallback", "mds"];
const HASH_PREFIX = "#t=";

function pair(part: string): readonly [string, string] {
  const eq = part.indexOf("=");
  return eq < 0 ? [part, ""] : [part.slice(0, eq), part.slice(eq + 1)];
}

/**
 * `?smoke=1&suite=<s>&room=<NN>&transport=<wt|ws>&attempt=<k>[&hash=wrong][&bite=<id>]` with `#t=<token>`. No other
 * key, no repeated key, no percent-encoding. `bite` is accepted only when `biteIds` (the bites dist's) names it.
 * A refusal's reason never holds the token or the link.
 */
export function parseLink(search: string, hash: string, biteIds: readonly string[] = []): Parsed<Link> {
  const bad = (why: string): Parsed<Link> => ({ ok: false, why });
  if (!search.startsWith("?")) return bad("no query");
  const keys = ["smoke", "suite", "room", "transport", "attempt", "hash", ...(biteIds.length > 0 ? ["bite"] : [])];
  const pairs = search.slice(1).split("&").map(pair);
  if (pairs.some(([k]) => !keys.includes(k))) return bad("an unknown query key");
  if (new Set(pairs.map(([k]) => k)).size !== pairs.length) return bad("a query key twice");
  const malformed = pairs.find(([, v]) => !VALUE_FORM.test(v));
  if (malformed !== undefined) return bad(`a malformed ${malformed[0]}`);
  const q = new Map(pairs);
  if (q.get("smoke") !== "1") return bad("smoke is not 1");
  const suite = SUITES.find((s) => s === q.get("suite"));
  if (suite === undefined) return bad("an unknown suite");
  const room = q.get("room") ?? "";
  if (!ROOM_FORM.test(room)) return bad("the room is not 00-31");
  const t = q.get("transport");
  if (t !== "wt" && t !== "ws") return bad("the transport is not wt or ws");
  if (t === "ws" && WT_ONLY.includes(suite)) return bad(`suite ${suite} runs on wt only`);
  const a = q.get("attempt") ?? "";
  if (!ATTEMPT_FORM.test(a)) return bad("the attempt is not 0-99");
  const h = q.get("hash");
  if (h !== undefined && (h !== "wrong" || suite !== "wt-attempts")) return bad("hash=wrong goes with suite wt-attempts only");
  const bite = q.get("bite") ?? null;
  if (bite !== null && !biteIds.includes(bite)) return bad("an unknown bite");
  if (!hash.startsWith(HASH_PREFIX)) return bad("no #t= fragment");
  const token = hash.slice(HASH_PREFIX.length);
  if (!TOKEN_FORM.test(token)) return bad("the token is malformed");
  return { ok: true, value: { suite, room, transport: t, attempt: Number(a), token, wrongHash: h === "wrong", bite } };
}

// ---- The hash file (u6b-brief.md condition 4; advisor-u6c-brief.md C1 (e), C9) ---------------------------------

export const hashPath = (room: string): string => `/npt/cert-hash-${room}.json`;

/** `{"algorithm":"sha-256","value_hex":<64 lowercase hex>,"validity_ns":<integer>}` exactly, as its 32 bytes. */
export function parseHashFile(text: string): Parsed<Uint8Array> {
  const bad = (why: string): Parsed<Uint8Array> => ({ ok: false, why: `the hash file ${why}` });
  if (text.length > HASH_FILE_MAX) return bad("is over 1 KiB");
  let o: unknown;
  try {
    o = JSON.parse(text);
  } catch {
    return bad("is not JSON");
  }
  if (typeof o !== "object" || o === null || Array.isArray(o)) return bad("is not an object");
  const rec = o as Record<string, unknown>;
  if (Object.keys(rec).sort().join(",") !== "algorithm,validity_ns,value_hex") return bad("has other keys");
  if (rec.algorithm !== "sha-256") return bad("is not sha-256");
  const hex = rec.value_hex;
  if (typeof hex !== "string" || !HEX64_FORM.test(hex)) return bad("value is not 64 lowercase hex digits");
  if (!Number.isSafeInteger(rec.validity_ns) || (rec.validity_ns as number) < 0) return bad("validity is not an integer");
  const at = (i: number): string => hex.slice(HEX_DIGITS_PER_BYTE * i, HEX_DIGITS_PER_BYTE * (i + 1));
  return { ok: true, value: Uint8Array.from({ length: HASH_BYTES }, (_, i) => parseInt(at(i), HEX_RADIX)) };
}

/** C4's `hash=wrong`: each byte XOR 0xff, so the pin can never match. */
export function wrongHash(bytes: Uint8Array): Uint8Array {
  return bytes.map((b) => b ^ ALL_BITS);
}

// ---- Addresses ----------------------------------------------------------------------------------------------------

export const wtUrl = (host: string, room: string): string => `https://${host}:${WT_PORT_BASE + Number(room)}/npt`;
export const wsUrl = (host: string, room: string): string => `wss://${host}:${WS_PORT_BASE + Number(room)}/`;

// ---- The Wasm boundary (C5: every integer little-endian) ---------------------------------------------------------

// eslint-disable-next-line ts-patterns/no-magic-numbers -- C5's channel numbers, named by this table
export const CH = { datagram: 0, control: 1, fullState: 2 } as const;
export type Channel = 0 | 1 | 2;

export interface Limits {
  /** The largest control message with its 2 B length. */
  readonly control: number;
  /** The largest full state with its 4 B length. */
  readonly fullState: number;
  readonly datagram: number;
  /** The WebSocket tag of each channel, by channel number. */
  readonly tags: readonly [number, number, number];
}

/** `limits()` in C5's order: control, full state, datagram, then the three WebSocket tags. */
export function readLimits(v: ArrayLike<number>): Parsed<Limits> {
  const n = Array.from(v);
  const [control = 0, fullState = 0, datagram = 0, td = -1, tc = -1, tf = -1] = n;
  const tags: [number, number, number] = [td, tc, tf];
  const okTags = tags.every((t) => Number.isInteger(t) && t >= 0 && t < TAG_VALUES) && new Set(tags).size === tags.length;
  const okCaps = control >= CONTROL_HEAD + MIN_TYPE && fullState >= FULL_STATE_HEAD + MIN_TYPE && datagram >= MIN_TYPE;
  if (n.length !== LIMITS_COUNT || !okCaps || !okTags) return { ok: false, why: "the client's limits are malformed" };
  return { ok: true, value: { control, fullState, datagram, tags } };
}

export interface InboxRecord {
  readonly channel: Channel;
  readonly arrivalNs: bigint;
  readonly bytes: Uint8Array;
}
export interface OutRecord {
  readonly channel: 0 | 1;
  readonly bytes: Uint8Array;
}

/** The inbox: channel u8, arrival ns u64, length u32, bytes, per record. */
export function packInbox(recs: readonly InboxRecord[]): Uint8Array {
  const out = new Uint8Array(recs.reduce((n, r) => n + INBOX_HEAD + r.bytes.length, 0));
  const v = new DataView(out.buffer);
  recs.reduce((o, r) => {
    v.setUint8(o, r.channel);
    v.setBigUint64(o + INBOX_NS_AT, r.arrivalNs, true);
    v.setUint32(o + INBOX_LEN_AT, r.bytes.length, true);
    out.set(r.bytes, o + INBOX_HEAD);
    return o + INBOX_HEAD + r.bytes.length;
  }, 0);
  return out;
}

/** The outbox: channel u8 (0 or 1), length u32, bytes, per record. A malformed outbox throws (a core fault). */
export function unpackOutbox(b: Uint8Array): OutRecord[] {
  const v = new DataView(b.buffer, b.byteOffset, b.byteLength);
  const out: OutRecord[] = [];
  let o = 0;
  // eslint-disable-next-line ts-patterns/no-loop-statements -- a walk over byte offsets, bounded by the buffer
  while (o < b.length) {
    if (o + OUTBOX_HEAD > b.length) throw new Error("the outbox ends inside a record header");
    const ch = v.getUint8(o);
    const len = v.getUint32(o + OUTBOX_LEN_AT, true);
    if (ch !== CH.datagram && ch !== CH.control) throw new Error("the outbox names an unknown channel");
    if (o + OUTBOX_HEAD + len > b.length) throw new Error("the outbox ends inside a record");
    out.push({ channel: ch, bytes: b.subarray(o + OUTBOX_HEAD, o + OUTBOX_HEAD + len) });
    o += OUTBOX_HEAD + len;
  }
  return out;
}

/** Received messages waiting for the next wake: at most `cap`, more dropped and counted. */
export class InboxQueue {
  private q: InboxRecord[] = [];
  dropped = 0;
  constructor(readonly cap: number = INBOX_MAX) {}
  push(r: InboxRecord): boolean {
    if (this.q.length >= this.cap) {
      this.dropped++;
      return false;
    }
    this.q.push(r);
    return true;
  }
  drain(): InboxRecord[] {
    const q = this.q;
    this.q = [];
    return q;
  }
  get length(): number {
    return this.q.length;
  }
}

// ---- Stream framing (catalogue docs/design/netcode-proof-messages.md:68-76) ------------------------------------

export class FrameError extends Error {}

/**
 * Reassembles length-prefixed messages (a `head`-byte little-endian length of what follows) across any chunk split.
 * Each message, its length included, is at most `cap` bytes; a longer one throws FrameError (the session closes).
 */
export class StreamFramer {
  private readonly buf: Uint8Array;
  private have = 0;
  private total = 0;
  constructor(private readonly head: typeof CONTROL_HEAD | typeof FULL_STATE_HEAD, private readonly cap: number) {
    this.buf = new Uint8Array(cap);
  }
  push(chunk: Uint8Array): Uint8Array[] {
    const out: Uint8Array[] = [];
    let i = 0;
    // eslint-disable-next-line ts-patterns/no-loop-statements -- a walk over byte offsets, bounded by the chunk
    while (i < chunk.length) {
      const need = (this.total === 0 ? this.head : this.total) - this.have;
      const n = Math.min(need, chunk.length - i);
      this.buf.set(chunk.subarray(i, i + n), this.have);
      this.have += n;
      i += n;
      if (this.total === 0 && this.have === this.head) {
        const v = new DataView(this.buf.buffer, 0, this.head);
        const len = this.head === CONTROL_HEAD ? v.getUint16(0, true) : v.getUint32(0, true);
        if (this.head + len > this.cap) throw new FrameError(`a stream message over its ${this.cap} B cap`);
        this.total = this.head + len;
      }
      if (this.total !== 0 && this.have === this.total) {
        out.push(this.buf.slice(0, this.total));
        this.have = 0;
        this.total = 0;
      }
    }
    return out;
  }
  /** True while a message is cut short (the stream must not end here). */
  get partial(): boolean {
    return this.have > 0;
  }
}
export const controlFramer = (lim: Limits): StreamFramer => new StreamFramer(CONTROL_HEAD, lim.control);
export const fullStateFramer = (lim: Limits): StreamFramer => new StreamFramer(FULL_STATE_HEAD, lim.fullState);

/** A WebSocket message: the channel's 1 B tag, then the channel's bytes. */
export function wsWrap(tag: number, bytes: Uint8Array): Uint8Array {
  const out = new Uint8Array(bytes.length + 1);
  out[0] = tag;
  out.set(bytes, 1);
  return out;
}

export type Unwrapped =
  | { readonly ok: true; readonly channel: Channel; readonly bytes: Uint8Array }
  | { readonly ok: false; readonly close: boolean; readonly why: string };

/** Strips the tag by `limits`' table and applies the channel's cap: a datagram over it is dropped, a stream closes. */
export function wsUnwrap(data: Uint8Array, lim: Limits): Unwrapped {
  const channel = lim.tags.indexOf(data[0] ?? -1);
  if (data.length === 0 || channel < 0) return { ok: false, close: true, why: "an unknown WebSocket tag" };
  const bytes = data.subarray(1);
  const caps = [lim.datagram, lim.control, lim.fullState];
  if (bytes.length > (caps[channel] ?? 0)) return { ok: false, close: channel !== CH.datagram, why: "a message over its cap" };
  return { ok: true, channel: channel as Channel, bytes };
}

// ---- Records (npt-log's `event` and `second` forms: crates/log/src/lib.rs:195-233; C1) ---------------------------

export function eventLine(ev: string, ns: number, f: Readonly<Record<string, number>>): string {
  return JSON.stringify({ k: "event", ev, ph: "measured", ns: Math.max(0, Math.round(ns)), f });
}
export function secondLine(sec: number, v: Readonly<Record<string, number>>): string {
  return JSON.stringify({ k: "second", sec, ph: "measured", v });
}

/**
 * The suites' records, from times in ns since the Worker's start: `attempt.ok` once, `fallback.playing_ns` once
 * (from the connect sequence's start), `mds_read.size_bytes` per read, and per second `recv_over_max_count`.
 */
export class Recorder {
  private attemptDone = false;
  private fallbackDone = false;
  private overMax = 0;
  private lastSec = -1;
  constructor(private readonly emit: (line: string) => void, private readonly connectStartNs: number) {}
  attempt(ns: number, ok: 0 | 1): void {
    if (this.attemptDone) return;
    this.attemptDone = true;
    this.emit(eventLine("attempt", ns, { ok }));
  }
  get attemptWritten(): boolean {
    return this.attemptDone;
  }
  fallback(ns: number): void {
    if (this.fallbackDone) return;
    this.fallbackDone = true;
    this.emit(eventLine("fallback", ns, { playing_ns: Math.round(ns - this.connectStartNs) }));
  }
  mdsRead(ns: number, size: number): void {
    this.emit(eventLine("mds_read", ns, { size_bytes: size }));
  }
  datagram(size: number, maxDatagramSize: number): void {
    if (size > maxDatagramSize) this.overMax++;
  }
  /** Called each second: one record for the second just completed, with the datagrams counted since the last. */
  second(ns: number): void {
    const sec = Math.floor(ns / NS_PER_S) - 1;
    if (sec < 0 || sec <= this.lastSec) return;
    this.emit(secondLine(sec, { recv_over_max_count: this.overMax }));
    this.overMax = 0;
    this.lastSec = sec;
  }
  /** At the session's end: the partial second's record, so a datagram counted in it is never lost (U7R-NC-6). */
  final(ns: number): void {
    const sec = Math.floor(ns / NS_PER_S);
    if (sec <= this.lastSec) return;
    this.emit(secondLine(sec, { recv_over_max_count: this.overMax }));
    this.overMax = 0;
    this.lastSec = sec;
  }
}

/** Counts `securitypolicyviolation` events at `target`: the page's document, the Worker's scope (U7R-NC-7). */
export function countCsp(target: { addEventListener(type: string, fn: () => void): void }, onCount: () => void): void {
  target.addEventListener("securitypolicyviolation", () => onCount());
}

/** After a good parse: the token's fragment dropped from the address bar and history, path and query kept (U7R-NC-5). */
export function forgetFragment(
  loc: { readonly pathname: string; readonly search: string },
  hist: { replaceState(data: null, unused: string, url: string): void },
): void {
  hist.replaceState(null, "", `${loc.pathname}${loc.search}`);
}

/** The Worker clock's step (C1): the smallest non-zero difference over `reads` consecutive reads, integer ns. */
export function clockResNs(now: () => number, reads = CLOCK_READS): number {
  const t = Array.from({ length: reads }, () => now());
  const best = t.reduce((m, x, i) => (i > 0 && x > (t[i - 1] ?? x) ? Math.min(m, x - (t[i - 1] ?? x)) : m), Infinity);
  return best === Infinity ? 0 : Math.max(1, Math.round(best * NS_PER_MS));
}

/** The counters the page reads by name (C5: names, never positions): Playing is joined plus the own soldier. */
export const NEEDED_COUNTERS = ["joined", "clock_samples", "own_soldier"] as const;
export type Needed = Record<(typeof NEEDED_COUNTERS)[number], number>;
export function counterIndex(names: readonly string[]): Parsed<Needed> {
  const missing = NEEDED_COUNTERS.find((n) => !names.includes(n));
  if (missing !== undefined) return { ok: false, why: `the client has no counter ${missing}` };
  const [joined, clock_samples, own_soldier] = NEEDED_COUNTERS.map((n) => names.indexOf(n));
  return { ok: true, value: { joined: joined ?? 0, clock_samples: clock_samples ?? 0, own_soldier: own_soldier ?? 0 } };
}

/** The browser's brand versions, else its user-agent string (no GREASE brand). */
export function versionText(nav: { userAgent?: string; userAgentData?: { brands?: { brand: string; version: string }[] } }): string {
  const brands = (nav.userAgentData?.brands ?? []).filter((b) => !GREASE_BRAND.test(b.brand));
  return brands.length > 0 ? brands.map((b) => `${b.brand} ${b.version}`).join("; ") : (nav.userAgent ?? "");
}

// ---- The transport chooser (plan.md:496-497; C9) ----------------------------------------------------------------

export interface Bidi {
  readonly readable: ReadableStream<Uint8Array>;
  readonly writable: WritableStream<Uint8Array>;
}
export interface WtLike {
  readonly ready: Promise<unknown>;
  readonly closed: Promise<unknown>;
  close(): void;
  readonly datagrams: {
    readonly readable: ReadableStream<Uint8Array>;
    readonly writable: WritableStream<Uint8Array>;
    readonly maxDatagramSize: number;
  };
  createBidirectionalStream(): Promise<Bidi>;
  readonly incomingUnidirectionalStreams: ReadableStream<ReadableStream<Uint8Array>>;
}
export interface WsLike {
  binaryType: string;
  readonly bufferedAmount: number;
  onopen: ((e: unknown) => void) | null;
  onclose: ((e: unknown) => void) | null;
  onerror: ((e: unknown) => void) | null;
  onmessage: ((e: { data: unknown }) => void) | null;
  send(data: Uint8Array): void;
  close(): void;
}
export interface WtOptions {
  readonly allowPooling: false;
  readonly requireUnreliable: true;
  readonly serverCertificateHashes: { algorithm: "sha-256"; value: Uint8Array }[];
}
export interface ConnectDeps {
  readonly link: Link;
  /** The page's own `location.hostname`. */
  readonly host: string;
  /** WebTransport's wait before the fallback; null waits for ever (a bite). */
  readonly timeoutMs: number | null;
  fetchText(path: string): Promise<string>;
  webTransport(url: string, options: WtOptions): WtLike;
  webSocket(url: string): WsLike;
  setTimer(fn: () => void, ms: number): unknown;
  clearTimer(id: unknown): void;
  onStatus(text: string): void;
}
export type Chosen =
  | { readonly kind: "wt"; readonly wt: WtLike }
  | { readonly kind: "ws"; readonly ws: WsLike; readonly fellBack: boolean }
  | { readonly kind: "failed"; readonly why: string };

/** True when `p` resolves, false when it rejects; never throws. */
export async function settles(p: Promise<unknown>): Promise<boolean> {
  try {
    await p;
    return true;
  } catch {
    return false;
  }
}

function shut(wt: WtLike): void {
  try {
    wt.close();
  } catch {
    // already closed
  }
}

/**
 * `ws`: WebSocket at once. `wt`: the room's hash file first (anything wrong stops here, before any connect), then
 * WebTransport; on failure WebSocket at once, on silence WebSocket after `timeoutMs`. Suites in NO_FALLBACK never fall
 * back and arm no timer. An abandoned WebTransport is closed, and closed again if it turns ready later (C9): the
 * caller sends its `Join` on the one transport this returns.
 */
export async function connect(d: ConnectDeps): Promise<Chosen> {
  const { link } = d;
  if (link.transport === "ws") return openWs(d, false);
  let text: string;
  try {
    text = await d.fetchText(hashPath(link.room));
  } catch {
    return { kind: "failed", why: "the hash file could not be fetched" };
  }
  const h = parseHashFile(text);
  if (!h.ok) return { kind: "failed", why: h.why };
  const value = link.wrongHash ? wrongHash(h.value) : h.value;
  const noFallback = NO_FALLBACK.includes(link.suite);
  d.onStatus(STATUS.connecting("wt"));
  let wt: WtLike;
  try {
    wt = d.webTransport(wtUrl(d.host, link.room), {
      allowPooling: false,
      requireUnreliable: true,
      serverCertificateHashes: [{ algorithm: "sha-256", value }],
    });
  } catch {
    return noFallback ? { kind: "failed", why: "WebTransport did not start" } : openWs(d, true);
  }
  void settles(wt.closed);
  const ready = settles(wt.ready);
  let timer: unknown = null;
  const silence = new Promise<"timeout">((done) => {
    if (!noFallback && d.timeoutMs !== null) timer = d.setTimer(() => done("timeout"), d.timeoutMs);
  });
  const outcome = await Promise.race([ready, silence]);
  if (timer !== null) d.clearTimer(timer);
  if (outcome === true) return { kind: "wt", wt };
  shut(wt);
  void (async (): Promise<void> => {
    if (await ready) shut(wt);
  })();
  if (noFallback) return { kind: "failed", why: "the WebTransport session did not open" };
  return openWs(d, true);
}

function openWs(d: ConnectDeps, fellBack: boolean): Promise<Chosen> {
  d.onStatus(STATUS.connecting("ws"));
  let ws: WsLike;
  try {
    ws = d.webSocket(wsUrl(d.host, d.link.room));
  } catch {
    return Promise.resolve({ kind: "failed", why: "the WebSocket did not start" });
  }
  ws.binaryType = "arraybuffer";
  return new Promise((done) => {
    ws.onopen = (): void => done({ kind: "ws", ws, fellBack });
    ws.onerror = ws.onclose = (): void => done({ kind: "failed", why: "the WebSocket did not open" });
  });
}
