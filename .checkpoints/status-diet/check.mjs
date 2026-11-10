// STATUS-DIET's checks (E1, E2, E3 in part, Q2): run from the repository root.
// Usage: node .checkpoints/status-diet/check.mjs [--dir <dir with the three files>]   (default .gate/status-diet/out)
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { oldLines, build, readTemplate, FILES, RA, RB, RC, RM, MANUAL, BASE } from './split.mjs';

const HERE = path.dirname(fileURLToPath(import.meta.url));
// --rev <commit>: check the three files as that commit holds them (E1 runs on the pure-move commit: condition 11).
const di = process.argv.indexOf('--dir'), ri = process.argv.indexOf('--rev');
let dir = di > 0 ? process.argv[di + 1] : '.gate/status-diet/out';
if (ri > 0) {
  const { execFileSync } = await import('node:child_process');
  const rev = process.argv[ri + 1];
  dir = `.gate/status-diet/rev-${rev}`;
  fs.mkdirSync(dir, { recursive: true });
  for (const f of FILES) fs.writeFileSync(path.join(dir, f), execFileSync('git', ['show', `${rev}:docs/${f}`], { encoding: 'utf8', maxBuffer: 1 << 26 }));
  console.log(`checking the three files as ${rev} holds them`);
}
let fails = 0;
const fail = (m) => { fails++; console.log('FAIL ' + m); };
const ok = (m) => console.log('ok   ' + m);
const lines = oldLines(), N = lines.length;
const NOT_MOVED = [1, 28, 29, 140, 141, 186, 187, 196, 197, 947, 948, 1009, 1010, 1218, 1219];

// E1a: every old line is placed exactly once, or is one of the structure lines listed.
const owner = new Array(N + 1).fill(null);
for (const f of FILES) for (const t of readTemplate(f)) {
  const m = t.match(/^<<MOVE (\d+)-(\d+)>>$/);
  if (m) for (let i = +m[1]; i <= +m[2]; i++) { if (owner[i]) fail(`line ${i} placed twice (${owner[i]}, ${f})`); owner[i] = f; }
}
for (const i of NOT_MOVED) {
  if (owner[i]) fail(`structure line ${i} is also moved`);
  if (!(lines[i - 1] === '' || /^#{1,2} /.test(lines[i - 1]))) fail(`structure line ${i} is not a heading or blank: ${lines[i - 1]}`);
}
const unplaced = [];
for (let i = 1; i <= N; i++) if (!owner[i] && !NOT_MOVED.includes(i)) unplaced.push(i);
if (unplaced.length) fail(`lines not placed: ${unplaced.join(', ')}`);
const per = Object.fromEntries(FILES.map(f => [f, owner.filter(x => x === f).length]));
ok(`E1a partition: ${N} old lines = ${N - NOT_MOVED.length} moved (${FILES.map(f => `${f} ${per[f]}`).join(', ')}) + ${NOT_MOVED.length} structure lines (${NOT_MOVED.map(i => lines[i - 1] || '(blank)').filter((v, k, a) => a.indexOf(v) === k).join(' | ')})`);

// E1b: the files on disk are exactly the script's output (moved lines byte for byte, in the manifest's order).
const { rows, text } = build(lines);
for (const f of FILES) {
  const p = path.join(dir, f);
  if (!fs.existsSync(p)) { fail(`${p} missing`); continue; }
  if (fs.readFileSync(p, 'utf8') !== text[f]) fail(`${p} differs from the script's output`);
  else ok(`E1b ${p} equals the script's output (${text[f].split('\n').length - 1} lines, ${Buffer.byteLength(text[f])} bytes)`);
}
// E1c, independent of the manifest: each moved old line's text occurs in its file at least as often as it was moved there.
for (const f of FILES) {
  const have = new Map(); for (const t of fs.readFileSync(path.join(dir, f), 'utf8').split('\n')) have.set(t, (have.get(t) || 0) + 1);
  const need = new Map(); for (let i = 1; i <= N; i++) if (owner[i] === f) need.set(lines[i - 1], (need.get(lines[i - 1]) || 0) + 1);
  let short = 0; for (const [t, c] of need) if ((have.get(t) || 0) < c) { short++; if (short < 4) fail(`${f} lacks a moved line: ${t.slice(0, 80)}`); }
  if (!short) ok(`E1c ${f}: all ${[...need.values()].reduce((a, b) => a + b, 0)} moved lines present`);
}

// E2: decision ids before (STATUS at BASE) and after (the three files), three nets.
const nets = { 'a, answered codes "CODE (x)"': [RA, 'g'], 'b, hyphenated capital codes': [RB, 'g'], 'c, codes that lead a list entry': [RC, 'l'] };
const ids = (texts, [re, kind]) => {
  const all = [];
  for (const t of texts) if (kind === 'g') for (const m of t.matchAll(re)) all.push(m[0]); else { const m = t.match(re); if (m) all.push(m[1]); }
  return all;
};
for (const [name, net] of Object.entries(nets)) {
  const before = ids(lines, net), after = FILES.flatMap(f => ids(fs.readFileSync(path.join(dir, f), 'utf8').split('\n'), net));
  const moved = FILES.flatMap(f => ids(rows[f].filter(r => r.old).map(r => r.text), net));
  const nb = new Set(before), na = new Set(after);
  const lost = [...nb].filter(x => !na.has(x)), added = [...na].filter(x => !nb.has(x));
  const perFile = FILES.map(f => `${f} ${new Set(ids(fs.readFileSync(path.join(dir, f), 'utf8').split('\n'), net)).size}`).join(', ');
  const line = `E2 (${name}): before ${nb.size} unique / ${before.length} mentions; after ${na.size} unique (${perFile}); ` +
    `mentions in moved text ${moved.length}, in new text ${after.length - moved.length}; lost ${lost.length}; new ${added.length}${added.length ? ' (' + added.join(', ') + ')' : ''}`;
  if (lost.length || moved.length !== before.length) fail(line + (lost.length ? ` LOST: ${lost.join(', ')}` : ' (moved mentions differ from before)')); else ok(line);
}
// E2 (condition 1): lines that name him with a date or record his go or approval, before and after; each one outside
// DECISIONS.md is listed in its index.
const rmBefore = lines.map((t, k) => k + 1).filter(i => RM.test(lines[i - 1]) || MANUAL[i]);
const rmAfter = FILES.flatMap(f => rows[f].filter(r => r.old && (RM.test(r.text) || MANUAL[r.old])).map(r => r.old));
const idx = fs.readFileSync(path.join(dir, 'DECISIONS.md'), 'utf8').split('## Also recorded elsewhere')[1] || '';
const listed = new Set([...idx.matchAll(/\d+/g)].map(m => +m[0]));
const outside = rmAfter.filter(i => owner[i] !== 'DECISIONS.md'), unlisted = outside.filter(i => !listed.has(i));
const rmLine = `E2 (his dated lines and approvals): before ${rmBefore.length}, after ${rmAfter.length} (in DECISIONS.md ${rmAfter.length - outside.length}, elsewhere ${outside.length}, each in the index)`;
if (rmBefore.length !== rmAfter.length || unlisted.length) fail(rmLine + (unlisted.length ? ` NOT IN THE INDEX: ${unlisted.join(', ')}` : '')); else ok(rmLine);

// E3 in part: STATUS's length.
const sl = fs.readFileSync(path.join(dir, 'STATUS.md'), 'utf8').split('\n').length - 1;
if (sl > 120) fail(`STATUS.md has ${sl} lines (more than 120)`); else ok(`E3 STATUS.md: ${sl} lines (at most 120)`);

// Q2 (the advisor's ruling): every open follow-up names itself in STATUS's "Follow-ups" section; no entry is missed.
const st = fs.readFileSync(path.join(dir, 'STATUS.md'), 'utf8');
const fu = (st.split('## Follow-ups')[1] || '').split('\n## ')[0].replace(/\s+/g, ' ');
const tsv = fs.readFileSync(path.join(HERE, 'followups.tsv'), 'utf8').split('\n').filter(l => l && !l.startsWith('#')).map(l => l.split('\t'));
const known = new Set(tsv.map(r => +r[0]));
const entries = [];
for (let i = 421; i <= 879; i++) if ((i <= 673 || i >= 862) && /^(  |    )?- /.test(lines[i - 1]) && !(i >= 674 && i <= 861)) {
  if (i >= 862 || /^ {2,4}- /.test(lines[i - 1])) entries.push(i);
}
const missed = entries.filter(i => !known.has(i));
const absent = tsv.filter(r => r[1] === 'open' && !fu.includes(r[2].replace(/\s+/g, ' ')));
const q2 = `Q2 follow-ups: ${entries.length} entries in the archive's Follow-ups, ${tsv.filter(r => r[1] === 'open').length} open, ` +
  `${tsv.filter(r => r[1] === 'closed').length} closed, ${tsv.filter(r => r[1] === 'group').length} group headers`;
if (missed.length || absent.length) fail(q2 + (missed.length ? `; NOT IN followups.tsv: ${missed.join(', ')}` : '') + (absent.length ? `; NOT NAMED IN STATUS: ${absent.map(r => r[0] + ' ' + r[2]).join('; ')}` : ''));
else ok(q2 + '; every open one named in STATUS');

console.log(fails ? `${fails} FAIL` : 'ALL PASS');
process.exit(fails ? 1 : 0);
