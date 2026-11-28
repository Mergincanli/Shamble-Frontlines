// fake-core.mjs: a stand-in for wasm-bindgen's npt_client_wasm.js with the Wasm class's shape (C5's boundary): a
// default `init`, `new Client(token, bind)`, `step(now_ns: bigint, prev_send_ns: bigint, inbox: Uint8Array)` returning
// the outbox, `next_due_ns(): bigint`, `counters(): Uint32Array`, and free `counter_names()` and `limits()`. Its
// packing is written here from C5's text, apart from lib.ts's, so the golden file checks both.
// Script: the first step sends the Join on Control; each due step sends an Input datagram, every 5th a Ping. A control
// message of type 8 (JoinAccepted) sets `joined`, a datagram of type 6 (Pong) counts a clock sample, a full state
// after the join sets `own_soldier`.
export const TYPE = { input: 1, snapshot: 2, fullState: 3, ping: 5, pong: 6, join: 7, joinAccepted: 8 };
export const TICK_NS = 33_333_333n;
/** Steps a call at most (the real core's `npt_sim::LOOP_CATCHUP`). */
export const CATCHUP = 7;
export const NAMES = ["joined", "clock_samples", "own_soldier", "steps", "inbox_records", "inbox_refused"];

/** C5's inbox, read with Buffer: records, or the rest refused at the first malformed one. */
export function fakeUnpackInbox(bytes, caps = [1150, 256, 65541]) {
  const b = Buffer.from(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const out = [];
  let o = 0;
  let refused = 0;
  while (o < b.length && out.length < 512) {
    if (o + 13 > b.length) {
      refused++;
      break;
    }
    const channel = b.readUInt8(o);
    const arrivalNs = b.readBigUInt64LE(o + 1);
    const len = b.readUInt32LE(o + 9);
    if (channel > 2 || len > caps[channel] || o + 13 + len > b.length) {
      refused++;
      break;
    }
    out.push({ channel, arrivalNs, bytes: Uint8Array.from(b.subarray(o + 13, o + 13 + len)) });
    o += 13 + len;
  }
  return { records: out, refused };
}

/** C5's outbox, written with Buffer. */
export function fakePackOutbox(records) {
  return new Uint8Array(Buffer.concat(records.flatMap((r) => {
    const h = Buffer.alloc(5);
    h.writeUInt8(r.channel, 0);
    h.writeUInt32LE(r.bytes.length, 1);
    return [h, Buffer.from(r.bytes)];
  })));
}

/** A core module; `opts.limits` replaces the six numbers, `opts.names` the counter names, `opts.trapAt` traps there. */
export function makeCore(opts = {}) {
  const log = { inits: 0, clients: [], steps: [] };
  const limits = opts.limits ?? [256, 65541, 1150, 0, 1, 2];
  class Client {
    constructor(token, bind) {
      if (typeof token !== "string" || !/^nptj_[A-Za-z0-9_-]{22}$/.test(token)) throw new Error("bad token");
      if (!(bind instanceof Uint8Array) || bind.length !== 16) throw new Error("bad bind");
      log.clients.push({ token, bind: Uint8Array.from(bind) });
      this.c = { joined: 0, clock_samples: 0, own_soldier: 0, steps: 0, inbox_records: 0, inbox_refused: 0 };
      this.due = 0n;
      this.sentJoin = false;
    }
    step(nowNs, prevSendNs, inbox) {
      if (typeof nowNs !== "bigint" || typeof prevSendNs !== "bigint" || !(inbox instanceof Uint8Array)) {
        throw new TypeError("step(bigint, bigint, Uint8Array)");
      }
      const { records, refused } = fakeUnpackInbox(inbox, [limits[2], limits[0], limits[1]]);
      const entry = { nowNs, prevSendNs, records, sent: 0 };
      log.steps.push(entry);
      this.c.inbox_records += records.length;
      this.c.inbox_refused += refused;
      for (const r of records) {
        if (r.channel === 1 && r.bytes[2] === TYPE.joinAccepted) this.c.joined = 1;
        if (r.channel === 0 && r.bytes[0] === TYPE.pong && this.c.joined) this.c.clock_samples++;
        if (r.channel === 2 && this.c.joined) this.c.own_soldier = 1;
      }
      const out = [];
      if (!this.sentJoin) {
        this.sentJoin = true;
        out.push({ channel: 1, bytes: [3, 0, TYPE.join, 0xa1, 0xa2] });
      }
      if (opts.trapAt !== undefined && this.c.steps >= opts.trapAt) throw new WebAssembly.RuntimeError("unreachable");
      for (let k = 0; k < CATCHUP && this.due <= nowNs; k++) {
        this.c.steps++;
        out.push({ channel: 0, bytes: [TYPE.input, this.c.steps & 0xff] });
        if (this.c.steps % 5 === 0) out.push({ channel: 0, bytes: [TYPE.ping, this.c.steps & 0xff] });
        this.due = (this.due === 0n ? nowNs : this.due) + TICK_NS;
      }
      entry.sent = out.length;
      return fakePackOutbox(out);
    }
    next_due_ns() {
      return this.due;
    }
    counters() {
      return Uint32Array.from(NAMES.map((n) => this.c[n]));
    }
    free() {}
  }
  return {
    log,
    default: async () => {
      log.inits++;
    },
    Client,
    counter_names: () => (opts.names ?? NAMES).slice(),
    limits: () => Uint32Array.from(limits),
  };
}
