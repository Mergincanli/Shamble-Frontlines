// rust-reviewer, cut C: runs the twin's own `__multi3` body in this engine (V8) on xorshift operand pairs and edge
// values, against BigInt's product modulo 2^128. Usage: node rust-reviewer-cutC-multi3.mjs <file.wasm> <count>
import { readFileSync } from "node:fs";
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
const body = bodies[names.indexOf("__multi3") - imports];
const leb = (n) => { const o = []; do { let x = n & 0x7f; n >>>= 7; if (n) x |= 0x80; o.push(x); } while (n); return o; };
const sec = (id, bytes) => [id, ...leb(bytes.length), ...bytes];
const name = [..."m"].map((c) => c.charCodeAt(0));
const module = new Uint8Array([0, 0x61, 0x73, 0x6d, 1, 0, 0, 0,
  ...sec(1, [1, 0x60, 5, 0x7f, 0x7e, 0x7e, 0x7e, 0x7e, 0]), ...sec(3, [1, 0]), ...sec(5, [1, 0, 1]),
  ...sec(7, [2, 1, ...name, 0, 0, 3, ..."mem".split("").map((c) => c.charCodeAt(0)), 2, 0]),
  ...sec(10, [1, ...leb(body.length), ...body])]);
const { m, mem } = new WebAssembly.Instance(new WebAssembly.Module(module)).exports;
const view = new BigUint64Array(mem.buffer, 0, 2);
const M64 = (1n << 64n) - 1n; const M128 = (1n << 128n) - 1n;
const signed = (x) => BigInt.asIntN(64, x);
let wrong = 0, n = 0;
const check = (a, c) => {
  m(0, signed(a & M64), signed(a >> 64n), signed(c & M64), signed(c >> 64n));
  const got = view[0] | (view[1] << 64n); const want = (a * c) & M128; n++;
  if (got !== want && wrong++ < 5) console.log("differs", a, c, got, want);
};
const edges = [0n, 1n, 2n, 5n, M64, 1n << 64n, M128, 1n << 127n, (1n << 64n) + 1n, 5n ** 54n, 5n ** 55n, (1n << 53n) - 1n];
for (const a of edges) for (const c of edges) check(a, c);
let s = 0x2545f4914f6cdd1dn; const next = () => { s ^= (s << 13n) & M64; s ^= s >> 7n; s ^= (s << 17n) & M64; return s; };
for (let k = 0; k < Number(countText); k++) { const a = (next() << 64n) | next(); const c = (next() << 64n) | next(); check(a, c); check(a >> 64n, c); check(a & ((1n << 53n) - 1n), 5n ** BigInt(k % 56)); }
console.log(`checked ${n}, differs from BigInt: ${wrong}`);
