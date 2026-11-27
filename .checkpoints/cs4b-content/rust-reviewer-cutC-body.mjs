// rust-reviewer, cut C: prints one function's body bytes (hex) and its size, by name, from a built twin.
// Usage: node rust-reviewer-cutC-body.mjs <file.wasm> <name>
import { readFileSync } from "node:fs";
const [file, want] = process.argv.slice(2);
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
const i = names.indexOf(want);
const body = bodies[i - imports];
console.log(`${want}: function ${i}, ${body.length} bytes: ${Buffer.from(body).toString("hex").match(/../g).join(" ")}`);
