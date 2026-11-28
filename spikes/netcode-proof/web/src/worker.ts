// worker.ts: the module Worker (u7-brief.md rev 1.1 Block T3; C-U7-3; C1, C5, C9). It loads the Wasm client, makes a
// 16 B bind, connects (lib.ts's chooser), stamps every received message with performance.now() in its own read,
// queues it (at most 512) and hands the queue to the client at the next wake, set from next_due_ns; after each wake's
// sends it stamps the send time the next step gets. A trap ends the session ("Ended"): no restart, no rejoin.
// The token reaches the Wasm only; the bind never leaves this Worker; neither is posted, logged or recorded.
import {
  ATTEMPT_DEADLINE_MS, CH, NS_PER_MS, Recorder, STATUS, clockResNs, connect, controlFramer, counterIndex, countCsp,
  fullStateFramer, packInbox, readLimits, settles, unpackOutbox, wsUnwrap, wsWrap,
  InboxQueue, type Channel, type Limits, type Link, type Needed, type OutRecord, type StreamFramer, type Transport,
  type WsLike, type WtLike, type WtOptions,
} from "./lib.js";

export interface CoreClient {
  step(nowNs: bigint, prevSendNs: bigint, inbox: Uint8Array): Uint8Array;
  next_due_ns(): bigint;
  counters(): Uint32Array;
}
/** wasm-bindgen's `--target web` module of npt-client-wasm (C5's boundary). */
export interface CoreModule {
  default(): Promise<unknown>;
  Client: new (token: string, bind: Uint8Array) => CoreClient;
  counter_names(): string[];
  limits(): Uint32Array;
}
export interface StartMsg {
  readonly kind: "start";
  readonly link: Link;
  readonly host: string;
  readonly wtTimeoutMs: number | null;
  readonly mdsPeriodMs: number | null;
}
export type ToPage =
  | { readonly kind: "status"; readonly text: string }
  | { readonly kind: "records"; readonly lines: readonly string[] }
  | { readonly kind: "counters"; readonly names: readonly string[]; readonly values: readonly number[] }
  | { readonly kind: "clock"; readonly resNs: number };
export interface EngineDeps {
  now(): number;
  setTimer(fn: () => void, ms: number): unknown;
  clearTimer(id: unknown): void;
  fetchText(path: string): Promise<string>;
  webTransport(url: string, options: WtOptions): WtLike;
  webSocket(url: string): WsLike;
  randomBytes(n: number): Uint8Array;
  loadCore(): Promise<CoreModule>;
  post(msg: ToPage): void;
}

const BIND_BYTES = 16;
const SECOND_MS = 1000;
const WAKE_MAX_MS = 1000;
/** Writes in flight per writer, full-state streams open at once, and WebSocket bytes buffered: every queue bounded. */
const PENDING_MAX = 64;
const UNI_MAX = 4;
const WS_BUFFER_MAX = 1_048_576;
/** `prev_send_ns` at the first step, before anything was sent. */
const NO_SEND_NS = 0n;

/** Reads `r` chunk by chunk, each read starting the next (no loop, no promise chain); `onDone` once at the end. */
function pump<T>(r: ReadableStreamDefaultReader<T>, onChunk: (v: T) => boolean, onDone: (error: boolean) => void): void {
  void (async (): Promise<void> => {
    let res: ReadableStreamReadResult<T>;
    try {
      res = await r.read();
    } catch {
      onDone(true);
      return;
    }
    if (res.done) onDone(false);
    else if (onChunk(res.value)) pump(r, onChunk, onDone);
  })();
}

/** The page's own counters, beside the client's; `csp_violations` the Worker's, the page adds its own (U7R-NC-7). */
const PAGE_COUNTERS = [
  "inbox_dropped", "datagram_over_cap", "datagram_send_dropped", "send_failed", "uni_refused", "csp_violations",
] as const;

export class Engine {
  private readonly startMs: number;
  private ended = false;
  private playing = false;
  private shownJoined = false;
  private fellBack = false;
  private transport: Transport = "wt";
  private rec: Recorder | null = null;
  private client: CoreClient | null = null;
  private lim: Limits | null = null;
  private idx: Needed | null = null;
  private names: readonly string[] = [];
  private wt: WtLike | null = null;
  private ws: WsLike | null = null;
  private dgWriter: WritableStreamDefaultWriter<Uint8Array> | null = null;
  private ctlWriter: WritableStreamDefaultWriter<Uint8Array> | null = null;
  private readonly pending = { dg: 0, ctl: 0 };
  private openUnis = 0;
  private prevSendNs = NO_SEND_NS;
  private readonly inbox = new InboxQueue();
  private readonly pc: Record<(typeof PAGE_COUNTERS)[number], number> = {
    inbox_dropped: 0, datagram_over_cap: 0, datagram_send_dropped: 0, send_failed: 0, uni_refused: 0, csp_violations: 0,
  };
  private readonly timers: Record<"wake" | "second" | "mds" | "deadline", unknown> = {
    wake: null, second: null, mds: null, deadline: null,
  };

  constructor(private readonly d: EngineDeps) {
    this.startMs = d.now();
    d.post({ kind: "clock", resNs: clockResNs(d.now) });
  }

  /** Nanoseconds since this Worker started: the records' clock (C1). */
  private ns(): number {
    return (this.d.now() - this.startMs) * NS_PER_MS;
  }
  /** The client's clock: performance.now() as integer ns. */
  private stamp(): bigint {
    return BigInt(Math.round(this.d.now() * NS_PER_MS));
  }
  private status(text: string): void {
    this.d.post({ kind: "status", text });
  }
  private arm(name: keyof Engine["timers"], fn: () => void, ms: number): void {
    this.timers[name] = this.d.setTimer(() => {
      this.timers[name] = null;
      if (!this.ended) fn();
    }, ms);
  }
  /** The time to the next whole second on the Worker clock's grid from its start, so `second` never drifts (U7R-NC-6). */
  private toNextSecond(): number {
    return SECOND_MS - ((this.d.now() - this.startMs) % SECOND_MS);
  }
  /** One `securitypolicyviolation` in this Worker (U7R-NC-7). */
  cspViolation(): void {
    this.pc.csp_violations++;
  }

  async start(m: StartMsg): Promise<void> {
    const core = (async (): Promise<CoreModule | null> => {
      try {
        const mod = await this.d.loadCore();
        await mod.default();
        return mod;
      } catch {
        return null;
      }
    })();
    const rec = new Recorder((line) => this.d.post({ kind: "records", lines: [line] }), this.ns());
    this.rec = rec;
    this.arm("deadline", () => rec.attempt(this.ns(), 0), ATTEMPT_DEADLINE_MS);
    this.arm("second", this.everySecond, this.toNextSecond());
    const chosen = await connect({
      link: m.link, host: m.host, timeoutMs: m.wtTimeoutMs, fetchText: this.d.fetchText, webTransport: this.d.webTransport,
      webSocket: this.d.webSocket, setTimer: this.d.setTimer, clearTimer: this.d.clearTimer, onStatus: (s) => this.status(s),
    });
    if (chosen.kind === "failed") return this.end(chosen.why);
    if (chosen.kind === "wt") this.watchWt(chosen.wt);
    else this.watchWs(chosen.ws, chosen.fellBack);
    const mod = await core;
    if (this.ended) return undefined;
    if (mod === null) return this.end("the client did not load");
    const lim = readLimits(mod.limits());
    const names = mod.counter_names();
    const idx = counterIndex(names);
    if (!lim.ok) return this.end(lim.why);
    if (!idx.ok) return this.end(idx.why);
    [this.lim, this.idx, this.names] = [lim.value, idx.value, names];
    try {
      this.client = new mod.Client(m.link.token, this.d.randomBytes(BIND_BYTES));
    } catch {
      return this.end("the client refused the link");
    }
    if (this.wt !== null) await this.setupWt(this.wt, lim.value, m.mdsPeriodMs);
    else this.setupWs(lim.value);
    if (!this.ended) this.wake();
    return undefined;
  }

  /** The chosen transport's end is watched at once, before the client has loaded. */
  private watchWt(wt: WtLike): void {
    this.wt = wt;
    void (async (): Promise<void> => {
      await settles(wt.closed);
      this.end("the WebTransport session closed");
    })();
  }
  private watchWs(ws: WsLike, fellBack: boolean): void {
    [this.ws, this.transport, this.fellBack] = [ws, "ws", fellBack];
    this.ws.onclose = this.ws.onerror = (): void => this.end("the WebSocket closed");
  }

  private async setupWt(wt: WtLike, lim: Limits, mdsPeriodMs: number | null): Promise<void> {
    const dg = wt.datagrams as WtLike["datagrams"] & { createWritable?: () => WritableStream<Uint8Array> };
    this.dgWriter = (typeof dg.createWritable === "function" ? dg.createWritable() : dg.writable).getWriter();
    try {
      const bidi = await wt.createBidirectionalStream();
      this.ctlWriter = bidi.writable.getWriter();
      this.readStream(bidi.readable, controlFramer(lim), CH.control, () => this.end("the control stream ended"));
    } catch {
      return this.end("the control stream did not open");
    }
    pump(wt.datagrams.readable.getReader(), (v) => this.onDatagram(v, wt, lim), () => undefined);
    pump(wt.incomingUnidirectionalStreams.getReader(), (s) => this.onUni(s, lim), () => undefined);
    const read = (): void => {
      this.rec?.mdsRead(this.ns(), wt.datagrams.maxDatagramSize);
      if (mdsPeriodMs !== null) this.arm("mds", read, mdsPeriodMs);
    };
    read();
    return undefined;
  }

  private onDatagram(v: Uint8Array, wt: WtLike, lim: Limits): boolean {
    const at = this.stamp();
    if (this.ended) return false;
    this.rec?.datagram(v.byteLength, wt.datagrams.maxDatagramSize);
    if (v.byteLength > lim.datagram) this.pc.datagram_over_cap++;
    else if (!this.inbox.push({ channel: CH.datagram, arrivalNs: at, bytes: v })) this.pc.inbox_dropped++;
    return true;
  }

  private onUni(s: ReadableStream<Uint8Array>, lim: Limits): boolean {
    if (this.ended) return false;
    if (this.openUnis >= UNI_MAX) {
      this.pc.uni_refused++;
      void settles(s.cancel());
      return true;
    }
    this.openUnis++;
    this.readStream(s, fullStateFramer(lim), CH.fullState, () => this.openUnis--);
    return true;
  }

  /** One stream's messages, reassembled and capped; a message over its cap or cut short ends the session. */
  private readStream(s: ReadableStream<Uint8Array>, framer: StreamFramer, channel: Channel, onEnd: () => void): void {
    pump(s.getReader(), (v) => {
      const at = this.stamp();
      if (this.ended) return false;
      let frames: Uint8Array[];
      try {
        frames = framer.push(v);
      } catch {
        this.end("a stream message over its cap");
        return false;
      }
      frames.forEach((bytes) => {
        if (!this.inbox.push({ channel, arrivalNs: at, bytes })) this.pc.inbox_dropped++;
      });
      return true;
    }, (error) => {
      if (framer.partial || error) this.end("a stream ended inside a message");
      else onEnd();
    });
  }

  private setupWs(lim: Limits): void {
    const ws = this.ws;
    if (ws === null) return;
    ws.onmessage = (e): void => {
      const at = this.stamp();
      if (this.ended) return;
      if (!(e.data instanceof ArrayBuffer)) return this.end("a WebSocket message that is not binary");
      const u = wsUnwrap(new Uint8Array(e.data), lim);
      if (!u.ok && u.close) return this.end(u.why);
      if (!u.ok) this.pc.datagram_over_cap++;
      else if (!this.inbox.push({ channel: u.channel, arrivalNs: at, bytes: u.bytes })) this.pc.inbox_dropped++;
      return undefined;
    };
  }

  private send(r: OutRecord): void {
    if (this.ws !== null && this.lim !== null) {
      if (this.ws.bufferedAmount <= WS_BUFFER_MAX) this.ws.send(wsWrap(this.lim.tags[r.channel], r.bytes));
      else if (r.channel === CH.datagram) this.pc.datagram_send_dropped++;
      else this.end("the WebSocket's send buffer is full");
      return;
    }
    const isDg = r.channel === CH.datagram;
    const w = isDg ? this.dgWriter : this.ctlWriter;
    const key = isDg ? "dg" : "ctl";
    if (w === null) return;
    if (this.pending[key] >= PENDING_MAX) {
      if (isDg) this.pc.datagram_send_dropped++;
      else this.end("the control stream is stuck");
      return;
    }
    this.pending[key]++;
    void (async (): Promise<void> => {
      if (!(await settles(w.write(r.bytes)))) this.pc.send_failed++;
      this.pending[key]--;
    })();
  }

  /** One wake: the queue to the client, its outbox sent, the send time stamped (C-U7-3), the next wake set. */
  private readonly wake = (): void => {
    const client = this.client;
    if (this.ended || client === null) return;
    let out: OutRecord[];
    let dueNs: bigint;
    try {
      out = unpackOutbox(client.step(this.stamp(), this.prevSendNs, packInbox(this.inbox.drain())));
      out.forEach((r) => this.send(r));
      this.prevSendNs = this.stamp();
      this.observe(client.counters());
      dueNs = client.next_due_ns();
    } catch {
      return this.end("the client trapped");
    }
    const ms = Number(dueNs - this.stamp()) / NS_PER_MS;
    if (!this.ended) this.arm("wake", this.wake, Math.min(WAKE_MAX_MS, Math.max(0, ms)));
    return undefined;
  };

  private observe(c: Uint32Array): void {
    const idx = this.idx;
    if (idx === null) return;
    const [joined, samples, own] = [c[idx.joined] ?? 0, c[idx.clock_samples] ?? 0, c[idx.own_soldier] ?? 0];
    const ns = this.ns();
    if (joined > 0 && samples > 0) this.rec?.attempt(ns, 1);
    if (this.playing || joined === 0) return;
    if (own > 0) {
      this.playing = true;
      this.status(STATUS.playing(this.transport));
      if (this.fellBack) this.rec?.fallback(ns);
    } else if (!this.shownJoined) {
      this.shownJoined = true;
      this.status(STATUS.joined(this.transport));
    }
  }

  private postCounters(): void {
    const core = this.client === null ? [] : Array.from(this.client.counters());
    this.d.post({
      kind: "counters",
      names: [...this.names, ...PAGE_COUNTERS.map((n) => `page_${n}`)],
      values: [...core, ...PAGE_COUNTERS.map((n) => (n === "inbox_dropped" ? this.inbox.dropped : this.pc[n]))],
    });
  }

  private readonly everySecond = (): void => {
    if (this.wt !== null && this.client !== null) this.rec?.second(this.ns());
    this.postCounters();
    this.arm("second", this.everySecond, this.toNextSecond());
  };

  /** Ends the session once: timers cleared, `attempt.ok` 0 if not yet written, the transport closed. No restart. */
  private end(why: string): void {
    if (this.ended) return;
    this.ended = true;
    Object.values(this.timers).forEach((t) => (t === null ? undefined : this.d.clearTimer(t)));
    this.rec?.attempt(this.ns(), 0);
    if (this.wt !== null && this.client !== null) this.rec?.final(this.ns());
    this.status(STATUS.ended(why));
    try {
      this.postCounters();
    } catch {
      // a trapped client may not answer
    }
    try {
      this.wt?.close();
      this.ws?.close();
    } catch {
      // already closed
    }
  }
}

/** The start message's shape, checked (it comes from the page, which parsed the link). */
export function readStart(x: unknown): StartMsg | null {
  const m = x as Partial<StartMsg> | null;
  const ms = (v: unknown): boolean => v === null || (typeof v === "number" && v > 0);
  const ok = m !== null && typeof m === "object" && m.kind === "start" && typeof m.host === "string"
    && typeof m.link === "object" && m.link !== null && typeof m.link.token === "string"
    && ms(m.wtTimeoutMs) && ms(m.mdsPeriodMs);
  return ok ? (m as StartMsg) : null;
}

/** wasm-bindgen's glue, beside this file in the dist (a string, so tsc does not resolve it). */
const CORE_URL: string = "./npt_client_wasm.js";

interface WorkerScope {
  postMessage(m: unknown): void;
  onmessage: ((e: MessageEvent) => void) | null;
}

if ("WorkerGlobalScope" in globalThis) {
  const scope = globalThis as unknown as WorkerScope;
  const engine = new Engine({
    now: () => performance.now(),
    setTimer: (fn, ms) => setTimeout(fn, ms),
    clearTimer: (id) => clearTimeout(id as ReturnType<typeof setTimeout>),
    fetchText: async (path) => {
      const r = await fetch(path, { cache: "no-store", credentials: "omit", redirect: "error" });
      if (!r.ok) throw new Error("not found");
      return r.text();
    },
    webTransport: (url, options) => new WebTransport(url, options as WebTransportOptions) as unknown as WtLike,
    webSocket: (url) => new WebSocket(url) as unknown as WsLike,
    randomBytes: (n) => crypto.getRandomValues(new Uint8Array(n)),
    loadCore: async () => (await import(CORE_URL)) as CoreModule,
    post: (msg) => scope.postMessage(msg),
  });
  countCsp(globalThis as unknown as Parameters<typeof countCsp>[0], () => engine.cspViolation());
  let started = false;
  scope.onmessage = (e): void => {
    const m = readStart(e.data);
    if (started || m === null) return;
    started = true;
    void engine.start(m);
  };
}
