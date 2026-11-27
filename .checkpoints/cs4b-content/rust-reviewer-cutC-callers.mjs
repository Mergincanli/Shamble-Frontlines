// rust-reviewer, cut C: who calls each named routine in a built twin (direct callers, then the chain up to an export).
// Usage: node rust-reviewer-cutC-callers.mjs <file.wasm> <name>...   Reads the file only; prints names, no bytes.
import { readFileSync } from "node:fs";
const [file, ...targets] = process.argv.slice(2);
const bytes = readFileSync(file);
let at = 8;
const u32 = (b, p) => { let v = 0, k = 0; for (;;) { const x = b[p.at++]; v += (x & 0x7f) * 2 ** (7 * k++); if (!(x & 0x80)) return v; } };
const skip = (b, p) => { while (b[p.at++] & 0x80); };
const sections = [];
const P = { at };
while (P.at < bytes.length) { const id = bytes[P.at++]; const len = u32(bytes, P); sections.push({ id, start: P.at, end: P.at + len }); P.at += len; }
let imports = 0, names = [], bodies = [];
for (const s of sections) {
  const p = { at: s.start };
  if (s.id === 2) { for (let n = u32(bytes, p); n > 0; n--) { for (let t = 0; t < 2; t++) { const l = u32(bytes, p); p.at += l; } const kind = bytes[p.at++]; if (kind === 0) { imports++; u32(bytes, p); } else if (kind === 3) p.at += 2; else if (kind === 1 || kind === 2) { if (kind === 1) p.at++; const f = bytes[p.at++]; skip(bytes, p); if (f & 1) skip(bytes, p); } else { p.at++; u32(bytes, p); } } }
  if (s.id === 0) { const l = u32(bytes, p); const nm = Buffer.from(bytes.subarray(p.at, p.at + l)).toString(); p.at += l; if (nm === "name") { while (p.at < s.end) { const id = bytes[p.at++]; const len = u32(bytes, p); const end = p.at + len; if (id === 1) { for (let n = u32(bytes, p); n > 0; n--) { const i = u32(bytes, p); const tl = u32(bytes, p); names[i] = Buffer.from(bytes.subarray(p.at, p.at + tl)).toString(); p.at += tl; } } p.at = end; } } }
  if (s.id === 10) { for (let n = u32(bytes, p); n > 0; n--) { const len = u32(bytes, p); bodies.push({ start: p.at, end: p.at + len }); p.at += len; } }
}
const IMM = new Map([
  ...[0x00, 0x01, 0x05, 0x0f, 0x1a, 0x1b, 0xd1, 0x0b].map((op) => [op, ""]),
  ...[0x0c, 0x0d, 0x10, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x3f, 0x40, 0xd2].map((op) => [op, "u"]),
  ...Array.from({ length: 0x3e - 0x28 + 1 }, (_, k) => [0x28 + k, "m"]),
  ...Array.from({ length: 0xc4 - 0x45 + 1 }, (_, k) => [0x45 + k, ""]),
  [0x0e, "t"], [0x11, "uu"], [0x1c, "s"], [0x41, "i"], [0x42, "I"], [0x43, "f"], [0x44, "F"], [0xd0, "i"],
]);
const FC = ["", "", "", "", "", "", "", "", "uu", "u", "uu", "u", "uu", "u", "uu", "u", "u", "u"];
const calls = bodies.map((b, k) => {
  const p = { at: b.start }; const out = new Set();
  for (let g = u32(bytes, p); g > 0; g--) { u32(bytes, p); p.at++; }
  while (p.at < b.end) {
    const op = bytes[p.at++];
    if (op === 0x02 || op === 0x03 || op === 0x04) { const t = bytes[p.at]; if (t & 0x80) skip(bytes, p); else p.at++; continue; }
    const sub = op === 0xfc ? u32(bytes, p) : undefined;
    const f = sub === undefined ? IMM.get(op) : FC[sub];
    if (f === undefined) throw new Error(`unknown op ${op.toString(16)} in body ${k}`);
    for (const c of f) {
      if (c === "u") { const v = u32(bytes, p); if (op === 0x10 && c === "u") out.add(v); }
      else if (c === "m") { u32(bytes, p); skip(bytes, p); }
      else if (c === "i" || c === "I") skip(bytes, p);
      else if (c === "f") p.at += 4; else if (c === "F") p.at += 8;
      else if (c === "t") { for (let n = u32(bytes, p) + 1; n > 0; n--) u32(bytes, p); }
      else if (c === "s") { for (let n = u32(bytes, p); n > 0; n--) p.at++; }
    }
  }
  return out;
});
const callers = new Map();
calls.forEach((set, k) => { for (const t of set) { if (!callers.has(t)) callers.set(t, new Set()); callers.get(t).add(k + imports); } });
for (const target of targets) {
  const idx = names.findIndex((n) => n === target);
  console.log(`== ${target} (function ${idx}) calls: ${idx < 0 ? "-" : [...(calls[idx - imports] ?? [])].map((c) => names[c]).join(", ") || "nothing"}`);
  // Breadth-first up the callers, each function once, with its depth.
  const seen = new Map([[idx, 0]]); const queue = [idx];
  while (queue.length) { const f = queue.shift(); for (const c of callers.get(f) ?? []) if (!seen.has(c)) { seen.set(c, seen.get(f) + 1); queue.push(c); } }
  for (const [f, d] of [...seen].sort((a, b) => a[1] - b[1])) if (d > 0 && d <= Number(process.env.DEPTH ?? 3)) console.log(`  depth ${d}: ${names[f]}`);
}
