// verifier scratch: where does a byte offset of a .wasm file fall, and if it is in a data segment that holds a
// panic Location {file ptr, file len, line, col}, which file and line. Read-only.
// usage: node verifier-wasm-diff.mjs <file.wasm> <0-based offset>
import { readFileSync } from 'node:fs';

const [file, offArg] = process.argv.slice(2);
const buf = readFileSync(file);
const target = Number(offArg);
let p = 8;
const leb = () => { let r = 0, s = 0, b; do { b = buf[p++]; r |= (b & 0x7f) << s; s += 7; } while (b & 0x80); return r >>> 0; };
const names = { 0: 'custom', 1: 'type', 2: 'import', 3: 'function', 4: 'table', 5: 'memory', 6: 'global', 7: 'export', 8: 'start', 9: 'element', 10: 'code', 11: 'data', 12: 'datacount' };
const segments = [];
while (p < buf.length) {
  const id = buf[p++];
  const size = leb();
  const start = p;
  let label = names[id] ?? `id${id}`;
  if (id === 0) { const save = p; const n = leb(); label += `:${buf.subarray(p, p + n).toString('utf8')}`; p = save; }
  if (id === 11) {
    const count = leb();
    for (let i = 0; i < count; i += 1) {
      const flags = leb();
      let mem = null;
      if (flags === 0) { if (buf[p++] !== 0x41) throw new Error('expected i32.const'); let r = 0, s = 0, b; do { b = buf[p++]; r |= (b & 0x7f) << s; s += 7; } while (b & 0x80); if (s < 32 && (b & 0x40)) r |= -1 << s; mem = r >>> 0; if (buf[p++] !== 0x0b) throw new Error('expected end'); }
      else if (flags === 2) { leb(); throw new Error('memidx form not handled'); }
      const len = leb();
      segments.push({ i, mem, fileStart: p, len });
      p += len;
    }
  }
  if (target >= start && target < start + size) console.log(`offset ${target} is in section ${label} (file bytes ${start}..${start + size})`);
  p = start + size;
}
const seg = segments.find((s) => target >= s.fileStart && target < s.fileStart + s.len);
if (seg) {
  const memAddr = seg.mem + (target - seg.fileStart);
  console.log(`data segment ${seg.i}: memory ${seg.mem}, ${seg.len} bytes; the byte is at memory address ${memAddr}`);
  const at = (addr) => { const s = segments.find((x) => addr >= x.mem && addr < x.mem + x.len); return s ? s.fileStart + (addr - s.mem) : -1; };
  // Try each alignment: a Location is 16 bytes, its line at +8.
  for (const back of [8, 9, 10, 11]) {
    const base = target - back;
    const ptr = buf.readUInt32LE(base), len = buf.readUInt32LE(base + 4), line = buf.readUInt32LE(base + 8), col = buf.readUInt32LE(base + 12);
    const fo = at(ptr);
    if (fo >= 0 && len > 0 && len < 300) {
      const name = buf.subarray(fo, fo + len).toString('utf8');
      if (/^[\x20-\x7e]+$/.test(name)) console.log(`as a Location (line at +${back - 8} into the line field): file "${name}", line ${line}, column ${col}`);
    }
  }
  console.log(`bytes around: ${buf.subarray(target - 12, target + 8).toString('hex')}`);
}
