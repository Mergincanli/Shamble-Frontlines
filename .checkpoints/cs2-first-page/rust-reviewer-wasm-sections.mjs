// rust-reviewer's scratch reader (review 1 of cs2-first-page): the sections of a .wasm file with their sizes, the
// largest function bodies by name (from the `name` section when present), the exports and imports, and the file's
// size raw, gzip -9 and Brotli 11. Reads only.   node rust-reviewer-wasm-sections.mjs <file.wasm> [topN]
import { readFileSync } from "node:fs";
import { brotliCompressSync, constants, gzipSync } from "node:zlib";

const file = process.argv[2];
const topN = Number(process.argv[3] ?? 25);
const buf = readFileSync(file);
let pos = 8;
const u32 = () => {
  let result = 0, shift = 0, byte;
  do { byte = buf[pos++]; result += (byte & 0x7f) * 2 ** shift; shift += 7; } while (byte & 0x80);
  return result;
};
const str = () => { const n = u32(); const s = buf.subarray(pos, pos + n).toString("utf8"); pos += n; return s; };
const NAMES = { 1: "type", 2: "import", 3: "function", 4: "table", 5: "memory", 6: "global", 7: "export", 8: "start",
  9: "element", 10: "code", 11: "data", 12: "datacount", 13: "tag" };
const sections = [];
let importedFuncs = 0, bodies = [], funcNames = new Map(), exportsList = [], importsList = [], dataSegs = [];
while (pos < buf.length) {
  const start = pos;
  const id = buf[pos++];
  const size = u32();
  const payloadStart = pos;
  const end = pos + size;
  let name = NAMES[id] ?? `id${id}`;
  if (id === 0) {
    name = `custom:${str()}`;
    if (name === "custom:name") {
      while (pos < end) {
        const sub = buf[pos++]; const subSize = u32(); const subEnd = pos + subSize;
        if (sub === 1) { const n = u32(); for (let i = 0; i < n; i++) { const idx = u32(); funcNames.set(idx, str()); } }
        pos = subEnd;
      }
    }
  } else if (id === 2) {
    const n = u32();
    for (let i = 0; i < n; i++) {
      const mod = str(), field = str(); const kind = buf[pos++];
      importsList.push(`${mod}.${field} (kind ${kind})`);
      if (kind === 0) { u32(); importedFuncs++; }
      else if (kind === 1) { pos++; const fl = u32(); u32(); if (fl & 1) u32(); }
      else if (kind === 2) { const fl = u32(); u32(); if (fl & 1) u32(); }
      else if (kind === 3) { pos += 2; }
      else if (kind === 4) { pos++; u32(); }
    }
  } else if (id === 7) {
    const n = u32();
    for (let i = 0; i < n; i++) { const nm = str(); const kind = buf[pos++]; const idx = u32(); exportsList.push(`${nm} (kind ${kind}, index ${idx})`); }
  } else if (id === 10) {
    const n = u32();
    for (let i = 0; i < n; i++) { const bs = u32(); bodies.push({ index: importedFuncs + i, size: bs }); pos += bs; }
  } else if (id === 11) {
    const n = u32();
    for (let i = 0; i < n; i++) {
      const flag = u32(); if (flag === 2) u32();
      if (flag !== 1) { while (buf[pos] !== 0x0b) pos++; pos++; }
      const len = u32(); dataSegs.push(len); pos += len;
    }
  }
  sections.push({ name, total: end - start, payload: size });
  pos = end;
}
const gz = gzipSync(buf, { level: 9 }).length;
const br = brotliCompressSync(buf, { params: { [constants.BROTLI_PARAM_QUALITY]: 11, [constants.BROTLI_PARAM_SIZE_HINT]: buf.length } }).length;
console.log(`${file}: ${buf.length} bytes raw, ${gz} gzip -9, ${br} brotli 11`);
console.log("sections (bytes with header):");
for (const s of sections) console.log(`  ${s.name.padEnd(36)} ${String(s.total).padStart(7)}`);
console.log(`functions: ${importedFuncs} imported, ${bodies.length} defined; code bytes in bodies ${bodies.reduce((a, b) => a + b.size, 0)}`);
console.log(`data segments: ${dataSegs.length}, bytes ${dataSegs.reduce((a, b) => a + b, 0)}`);
console.log(`imports (${importsList.length}):`); for (const i of importsList) console.log(`  ${i}`);
console.log(`exports (${exportsList.length}):`); for (const e of exportsList) console.log(`  ${e}`);
console.log(`largest ${topN} function bodies:`);
for (const b of [...bodies].sort((x, y) => y.size - x.size).slice(0, topN)) {
  console.log(`  ${String(b.size).padStart(6)}  #${b.index} ${funcNames.get(b.index) ?? "(no name)"}`);
}
