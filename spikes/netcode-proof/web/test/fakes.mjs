// fakes.mjs: a fake clock with timers, a fake WebTransport and WebSocket (Node's own web streams), for the chooser and
// the Worker's tests. Each fake records what the page did to it; nothing touches a network.

/** Lets every pending promise callback run. */
export async function flush(rounds = 4) {
  for (let i = 0; i < rounds; i++) await new Promise((r) => setImmediate(r));
}

/**
 * Time in ms from 0; timers fire only inside `advance`, in time order, with promises flushed after each, each `lateMs`
 * after its due time (a busy thread). Time never moves back: a test that advances `t` inside a callback (a write that
 * takes time) holds.
 */
export class FakeClock {
  t = 0;
  seq = 0;
  lateMs = 0;
  timers = new Map();
  now = () => this.t;
  setTimer = (fn, ms) => {
    const id = ++this.seq;
    this.timers.set(id, { at: this.t + Math.max(0, ms) + this.lateMs, fn });
    return id;
  };
  clearTimer = (id) => {
    this.timers.delete(id);
  };
  async advance(ms) {
    const end = this.t + ms;
    await flush();
    for (;;) {
      const next = [...this.timers.entries()].filter(([, x]) => x.at <= end).sort((a, b) => a[1].at - b[1].at || a[0] - b[0])[0];
      if (!next) break;
      this.timers.delete(next[0]);
      this.t = Math.max(this.t, next[1].at);
      next[1].fn();
      await flush();
    }
    this.t = Math.max(this.t, end);
    await flush();
  }
}

/**
 * A writable whose writer's `write` runs at once: `onWrite()` first (a write that takes time), then the bytes kept in
 * `written`; `stuck` writes never settle (a stalled stream).
 */
export function hookedWritable(written, { onWrite = () => {}, stuck = false } = {}) {
  const write = (b) => {
    onWrite();
    written.push(Uint8Array.from(b));
    return stuck ? new Promise(() => {}) : Promise.resolve();
  };
  return { getWriter: () => ({ write }) };
}

function deferred() {
  let resolve;
  let reject;
  const promise = new Promise((a, b) => {
    resolve = a;
    reject = b;
  });
  return { promise, resolve, reject };
}

/** A readable stream whose chunks the test pushes, and a writable one that keeps what the page wrote. */
function pipePair() {
  let ctl;
  const readable = new ReadableStream({ start: (c) => (ctl = c) });
  const written = [];
  const writable = new WritableStream({ write: (chunk) => void written.push(Uint8Array.from(chunk)) });
  return { readable, writable, written, push: (b) => ctl.enqueue(Uint8Array.from(b)), end: () => ctl.close() };
}

export class FakeWt {
  static made = [];
  constructor(url, options) {
    this.url = url;
    this.options = options;
    this.closeCalls = 0;
    this.bidis = [];
    this._ready = deferred();
    this._closed = deferred();
    this.ready = this._ready.promise;
    this.closed = this._closed.promise;
    this.ready.catch(() => {});
    this.dg = pipePair();
    this.datagrams = { readable: this.dg.readable, writable: this.dg.writable, maxDatagramSize: 1024 };
    let uctl;
    this.incomingUnidirectionalStreams = new ReadableStream({ start: (c) => (uctl = c) });
    this.pushUni = (chunks) => {
      let sc;
      const s = new ReadableStream({ start: (c) => (sc = c) });
      uctl.enqueue(s);
      for (const ch of chunks) sc.enqueue(Uint8Array.from(ch));
      return { end: () => sc.close(), push: (b) => sc.enqueue(Uint8Array.from(b)) };
    };
    FakeWt.made.push(this);
  }
  open() {
    this._ready.resolve();
  }
  fail() {
    this._ready.reject(new Error("refused"));
    this._closed.reject(new Error("refused"));
  }
  close() {
    this.closeCalls++;
    this._closed.resolve();
  }
  /** Every writer from now on is a `hookedWritable` with `hook` (before the session opens). */
  useHook(hook) {
    this.hook = hook;
    this.datagrams.writable = hookedWritable(this.dg.written, hook);
  }
  async createBidirectionalStream() {
    const p = pipePair();
    if (this.hook) p.writable = hookedWritable(p.written, this.hook);
    this.bidis.push(p);
    return p;
  }
}

export class FakeWs {
  static made = [];
  constructor(url) {
    this.url = url;
    this.binaryType = "blob";
    this.bufferedAmount = 0;
    this.sent = [];
    this.closeCalls = 0;
    this.onopen = this.onclose = this.onerror = this.onmessage = null;
    FakeWs.made.push(this);
  }
  open() {
    this.onopen?.({});
  }
  fail() {
    this.onerror?.({});
    this.onclose?.({});
  }
  message(bytes) {
    this.onmessage?.({ data: Uint8Array.from(bytes).buffer });
  }
  send(d) {
    this.sent.push(Uint8Array.from(d));
  }
  close() {
    this.closeCalls++;
  }
}

/** The hash file U6b's room writes, for a 32-byte value that is no palindrome (bytes 0x00..0x1f). */
export const HASH_BYTES = Uint8Array.from({ length: 32 }, (_, i) => i);
export const hashFile = (hex = Buffer.from(HASH_BYTES).toString("hex")) =>
  JSON.stringify({ algorithm: "sha-256", value_hex: hex, validity_ns: 13 * 86400 * 1e9 });

/** ConnectDeps over the fakes; `fetched` lists every path asked for. */
export function connectDeps(link, clock, { timeoutMs = 1500, hashText = hashFile(), fetchFails = false } = {}) {
  FakeWt.made = [];
  FakeWs.made = [];
  const d = {
    link,
    host: "127.0.0.1",
    timeoutMs,
    fetched: [],
    statuses: [],
    fetchText: async (path) => {
      d.fetched.push(path);
      if (fetchFails) throw new Error("404");
      return hashText;
    },
    webTransport: (url, options) => new FakeWt(url, options),
    webSocket: (url) => new FakeWs(url),
    setTimer: clock.setTimer,
    clearTimer: clock.clearTimer,
    onStatus: (s) => d.statuses.push(s),
  };
  return d;
}
