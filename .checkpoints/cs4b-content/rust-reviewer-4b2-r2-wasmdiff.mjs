// rust-reviewer round 2: compare two wasm files section by section, and function bodies of the code section.
import { readFileSync } from 'node:fs';
const [a, b] = process.argv.slice(2).map((p) => readFileSync(p));
function leb(buf, at) { let r = 0, s = 0, x; do { x = buf[at++]; r |= (x & 0x7f) << s; s += 7; } while (x & 0x80); return [r >>> 0, at]; }
function sections(buf) {
  const out = []; let at = 8;
  while (at < buf.length) {
    const id = buf[at++]; let size; [size, at] = leb(buf, at);
    let name = String(id);
    if (id === 0) { let [n, p] = leb(buf, at); name = 'custom:' + buf.subarray(p, p + n).toString(); }
    out.push({ id, name, body: buf.subarray(at, at + size) }); at += size;
  }
  return out;
}
function bodies(code) { let [n, at] = leb(code, 0); const out = []; for (let i = 0; i < n; i++) { let size; [size, at] = leb(code, at); out.push(code.subarray(at, at + size)); at += size; } return out; }
const sa = sections(a), sb = sections(b);
for (let i = 0; i < Math.max(sa.length, sb.length); i++) {
  const x = sa[i], y = sb[i];
  const same = x && y && Buffer.compare(x.body, y.body) === 0;
  console.log(x?.name, y?.name, x?.body.length, y?.body.length, same ? 'same' : 'DIFF');
  if (!same && x?.id === 10) {
    const ba = bodies(x.body), bb = bodies(y.body);
    ba.forEach((f, k) => { if (Buffer.compare(f, bb[k]) !== 0) console.log('  fn', k, f.length, bb[k].length); });
  }
  if (!same && x?.id === 11) {
    let d = 0; for (let k = 0; k < Math.min(x.body.length, y.body.length); k++) if (x.body[k] !== y.body[k]) { if (d < 12) console.log('  data byte', k, x.body[k], y.body[k]); d++; }
    console.log('  data bytes differing', d);
  }
}
