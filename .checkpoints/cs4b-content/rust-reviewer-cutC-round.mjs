// rust-reviewer, cut C: runs the twin's own `round` body in this engine (V8) over edge cases and xorshift bit
// patterns, against an exact reference of round-half-away-from-zero, and prints a hash of the result bits.
// Usage: node rust-reviewer-cutC-round.mjs <file.wasm> <count>
import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
const [file, countText] = process.argv.slice(2);
const b = readFileSync(file);
const u32 = (p) => { let v = 0, k = 0; for (;;) { const x = b[p.at++]; v += (x & 0x7f) * 2 ** (7 * k++); if (!(x & 0x80)) return v; } };
const P = { at: 8 }; const secs = [];
while (P.at < b.length) { const id = b[P.at++]; const len = u32(P); secs.push({ id, start: P.at }); P.at += len; }
let imports = 0; const names = []; const bodies = [];
for (const s of secs) {
  const p = { at: s.start };
  if (s.id === 2) for (let n = u32(p); n > 0; n--) { for (let t = 0; t < 2; t++) { const l = u32(p); p.at += l; } const k = b[p.at++]; if (k === 0) { imports++; u32(p); } else throw new Error("non-function import"); }
  if (s.id === 0) { const l = u32(p); const nm = b.subarray(p.at, p.at + l).toString(); p.at += l; if (nm === "name") for (;;) { const id = b[p.at++]; const len = u32(p); const end = p.at + len; if (id === 1) { for (let n = u32(p); n > 0; n--) { const i = u32(p); const tl = u32(p); names[i] = b.subarray(p.at, p.at + tl).toString(); p.at += tl; } break; } p.at = end; } }
  if (s.id === 10) for (let n = u32(p); n > 0; n--) { const len = u32(p); bodies.push(b.subarray(p.at, p.at + len)); p.at += len; }
}
const body = bodies[names.indexOf("round") - imports];
const leb = (n) => { const o = []; do { let x = n & 0x7f; n >>>= 7; if (n) x |= 0x80; o.push(x); } while (n); return o; };
const sec = (id, bytes) => [id, ...leb(bytes.length), ...bytes];
const module = new Uint8Array([0, 0x61, 0x73, 0x6d, 1, 0, 0, 0,
  ...sec(1, [1, 0x60, 1, 0x7c, 1, 0x7c]), ...sec(3, [1, 0]),
  ...sec(7, [1, 5, ..."round".split("").map((c) => c.charCodeAt(0)), 0, 0]),
  ...sec(10, [1, ...leb(body.length), ...body])]);
const round = new WebAssembly.Instance(new WebAssembly.Module(module)).exports.round;
const reference = (x) => { if (!Number.isFinite(x)) return x; const t = Math.trunc(x); return Math.abs(x - t) >= 0.5 ? t + Math.sign(x) : t; };
const f = new Float64Array(1); const u = new BigUint64Array(f.buffer);
const hash = createHash("sha256"); let wrong = 0; let n = 0;
const check = (x) => { const r = round(x); const e = reference(x); if (!Object.is(r, e)) { if (wrong++ < 5) console.log("differs", x, r, e); } f[0] = r; hash.update(Buffer.from(f.buffer)); n++; };
const edges = [0, -0, 0.5, -0.5, 0.49999999999999994, -0.49999999999999994, 1.5, 2.5, -2.5, 2 ** 52, 2 ** 52 + 1, 2 ** 52 - 0.5, 2 ** 51 + 0.5, 2 ** 53, 1e300, -1e300, 5e-324, Number.MAX_VALUE, Infinity, -Infinity, 4503599627370495.5, 0.5000000000000001];
for (const x of edges) for (const s of [x, -x]) check(s);
for (let k = -2000; k <= 2000; k++) { check(k / 4); check(k / 8 + 1e-9); }
let s = 0x9e3779b97f4a7c15n; const m = (1n << 64n) - 1n;
for (let k = 0; k < Number(countText); k++) {
  s ^= (s << 13n) & m; s ^= s >> 7n; s ^= (s << 17n) & m;
  u[0] = s; const x = f[0];
  if (Number.isNaN(x)) continue; check(x);
  // The same pattern pulled into the range where rounding happens (|x| below 2^53).
  u[0] = (s & 0x800fffffffffffffn) | (BigInt(0x3fe + Number((s >> 52n) % 54n)) << 52n); check(f[0]);
}
console.log(`checked ${n}, differs from exact reference: ${wrong}, sha256 of result bits ${hash.digest("hex")}`);
