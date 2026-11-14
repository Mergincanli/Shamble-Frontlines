// Counts the pass marks that Part 3 of plan.md places (sections 3.3 to 3.8: each table row's first cell, plus the
// ids in 3.5's heading) and Part 12's table, and compares them with the marks of the tests named in Mustafa's
// decision of 2026-11-14 (docs/SPIKE_PLAN.md's table, lines 147-160). Run: node count-marks.mjs plan.md
import { readFileSync } from 'node:fs';
const text = readFileSync(process.argv[2], 'utf8');
const between = (a, b) => text.slice(text.indexOf(a), text.indexOf(b));
const ids = (cell) => {
  const out = []; let adr = null, last = null;
  const re = /PM-(\d{4})-(\d+)|\bto\s+-(\d+)|(?:^|[\s,;(])-(\d+)/g; let m;
  while ((m = re.exec(cell))) {
    if (m[1]) { adr = m[1]; last = +m[2]; out.push(`${adr}-${last}`); }
    else if (m[3] && adr) { for (let n = last + 1; n <= +m[3]; n++) out.push(`${adr}-${n}`); last = +m[3]; }
    else if (m[4] && adr) { last = +m[4]; out.push(`${adr}-${last}`); }
  }
  return out;
};
const firstCells = (part) => part.split('\n').filter((l) => /^\| PM-/.test(l)).map((l) => l.split('|')[1]);
const part3 = between('### 3.3 ', '### 3.9 ');
const placed = new Map();
const add = (id, where) => placed.set(id, [...(placed.get(id) || []), where]);
for (const c of firstCells(part3)) for (const id of ids(c)) add(id, c.trim());
for (const l of part3.split('\n').filter((l) => l.startsWith('### 3.5 '))) for (const id of ids(l)) add(id, '3.5 heading');
const range = (adr, a, b) => Array.from({ length: b - a + 1 }, (_, i) => `${adr}-${a + i}`);
const expected = new Set([
  ...range('0002', 1, 34), '0002-77', '0002-78', ...range('0006', 28, 31),
  ...range('0004', 1, 75).filter((x) => x !== '0004-67'),
  ...range('0003', 1, 47), ...range('0005', 1, 37), '0001-39', '0001-40', '0001-63']);
const missing = [...expected].filter((x) => !placed.has(x));
const extra = [...placed.keys()].filter((x) => !expected.has(x));
const twice = [...placed].filter(([, w]) => w.length > 1).map(([k, w]) => `${k} (${w.length})`);
const per = {}; for (const x of expected) per[x.slice(0, 4)] = (per[x.slice(0, 4)] || 0) + 1;
console.log('expected', expected.size, JSON.stringify(per), 'placed', placed.size);
console.log('missing', missing.join(' ') || 'none'); console.log('extra', extra.join(' ') || 'none');
console.log('placed in more than one row', twice.join(' ') || 'none');
const part12 = between('## Part 12.', '## Part 13.');
const asset = new Set(firstCells(part12).flatMap(ids));
const wantAsset = range('0005', 38, 44);
console.log('asset spike rows', asset.size, 'missing', wantAsset.filter((x) => !asset.has(x)).join(' ') || 'none');
