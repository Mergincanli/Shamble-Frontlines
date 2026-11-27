// rust-reviewer, 4b-2 review: a twin's code bytes by crate (from the name section's mangled names), its data
// segment bytes, and the build's own twin rules with the committed allow-list.
// Usage: node rust-reviewer-4b2-sizes.mjs <build-checks.mjs> <maths-allowed.mjs> <release-twin.wasm> [<dev-twin.wasm>]
import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";
const [checks, allowedFile, release, debug] = process.argv.slice(2);
const { twinProblems } = await import(pathToFileURL(checks).href);
const { MATHS_ALLOWED } = await import(pathToFileURL(allowedFile).href);
const b = readFileSync(release);
const u32 = (p) => { let v = 0, k = 0; for (;;) { const x = b[p.at++]; v += (x & 0x7f) * 2 ** (7 * k++); if (!(x & 0x80)) return v; } };
const P = { at: 8 }; const secs = [];
while (P.at < b.length) { const id = b[P.at++]; const len = u32(P); secs.push({ id, start: P.at, len }); P.at += len; }
let imports = 0; const names = []; const bodies = []; let data = 0;
for (const s of secs) {
  const p = { at: s.start };
  if (s.id === 2) for (let n = u32(p); n > 0; n--) { for (let t = 0; t < 2; t++) { const l = u32(p); p.at += l; } const k = b[p.at++]; if (k === 0) { imports++; u32(p); } else throw new Error("non-function import"); }
  if (s.id === 0) { const l = u32(p); const nm = b.subarray(p.at, p.at + l).toString(); p.at += l; if (nm === "name") for (;;) { const id = b[p.at++]; const len = u32(p); const end = p.at + len; if (id === 1) { for (let n = u32(p); n > 0; n--) { const i = u32(p); const tl = u32(p); names[i] = b.subarray(p.at, p.at + tl).toString(); p.at += tl; } break; } p.at = end; } }
  if (s.id === 10) for (let n = u32(p); n > 0; n--) { const len = u32(p); bodies.push(len); p.at += len; }
  if (s.id === 11) data += s.len;
}
// The crate of a v0 or legacy mangled name: the first identifier after the crate-root marker.
const crateOf = (name) => {
  if (name === undefined) return "?";
  const v0 = name.match(/^_R.*?C[s0-9A-Za-z_]*?_(\d+)([a-z_][a-z0-9_]*)/);
  if (name.startsWith("_R")) { const m = name.match(/Cs[0-9A-Za-z_]+_(\d+)/); if (m) { const len = Number(m[1]); const at = name.indexOf(m[0]) + m[0].length; return name.slice(at, at + len); } return "v0?"; }
  const legacy = name.match(/^_ZN(\d+)/); if (legacy) return name.slice(legacy.index + legacy[0].length, legacy.index + legacy[0].length + Number(legacy[1]));
  void v0; return "C:" + name.slice(0, 40);
};
const byCrate = new Map(); let total = 0;
bodies.forEach((len, k) => { const c = crateOf(names[k + imports]); byCrate.set(c, (byCrate.get(c) ?? 0) + len); total += len; });
console.log(`file ${b.length} bytes, ${bodies.length} functions, code ${total} bytes, data section ${data} bytes`);
for (const [c, n] of [...byCrate].sort((x, y) => y[1] - x[1]).slice(0, 25)) console.log(`  ${c}: ${n}`);
const lines = [...twinProblems(b, "the release twin", MATHS_ALLOWED), ...(debug ? twinProblems(readFileSync(debug), "the dev-profile twin", MATHS_ALLOWED) : [])];
console.log(`twin rules with the committed allow-list: ${lines.length} lines`); for (const l of lines) console.log(`  ${l}`);
const plain = names.filter((n, i) => i >= imports && n && !n.startsWith("_R") && !n.startsWith("_ZN"));
console.log(`plain names: ${plain.length}: ${plain.filter((n) => !n.startsWith("__wbg") && !n.startsWith("__wbindgen") && !n.startsWith("sim_") && !n.startsWith("tick_hz")).join(" ")}`);
