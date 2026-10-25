// R7: coverage of the final rows. Every top-level declaration of the three engine files (acorn, as the coverage
// critic counted them), matched against the final rows' Where fields (fixes2.json applied).
// Run from the repo root: node .checkpoints/r7-legacy-inventory/coverage-final.cjs  → coverage-final.json
'use strict';
const fs = require('fs');
const path = require('path');
const acorn = require(path.resolve('tools/node_modules/acorn'));
const D = '.checkpoints/r7-legacy-inventory';
const CODES = ['CD', 'CL', 'CC', 'CM', 'CS', 'GW', 'GA', 'GC', 'GN', 'GE', 'GP', 'ED', 'PG', 'TL', 'KT'];
const fixes = fs.existsSync(`${D}/fixes2.json`) ? JSON.parse(fs.readFileSync(`${D}/fixes2.json`, 'utf8')) : {};
const rows = [];
const first = JSON.parse(fs.readFileSync(`${D}/rows-and-checks.json`, 'utf8'));
for (const c of CODES) {
  // R7_FIRST=1: the readers' first-pass rows, to compare with the coverage critic's count
  const file = fs.existsSync(`${D}/after-${c}.json`) ? `${D}/after-${c}.json` : `${D}/final-${c}.json`;
  const j = process.env.R7_FIRST ? first[c].rows : JSON.parse(fs.readFileSync(file, 'utf8'));
  for (const r of (Array.isArray(j) ? j : j.rows)) {
    const f = fixes[r.id];
    if (f && f._remove) continue;
    rows.push(Object.assign({}, r, f || {}));
  }
}
function names(p, o) {
  if (!p) return;
  if (p.type === 'Identifier') o.push(p.name);
  else if (p.type === 'ObjectPattern') for (const q of p.properties) names(q.type === 'RestElement' ? q.argument : q.value, o);
  else if (p.type === 'ArrayPattern') for (const e of p.elements) names(e, o);
  else if (p.type === 'RestElement') names(p.argument, o);
  else if (p.type === 'AssignmentPattern') names(p.left, o);
}
const decls = {};
const nlines = {};
for (const f of ['core', 'game', 'editor']) {
  const src = fs.readFileSync(`shamble-${f}.js`, 'utf8');
  nlines[f] = src.split('\n').length;
  const L = [];
  for (const st of acorn.parse(src, { ecmaVersion: 'latest', locations: true }).body) {
    if ((st.type === 'FunctionDeclaration' || st.type === 'ClassDeclaration') && st.id) L.push({ n: st.id.name, l: st.loc.start.line });
    else if (st.type === 'VariableDeclaration') for (const d of st.declarations) { const ns = []; names(d.id, ns); for (const n of ns) L.push({ n, l: d.loc.start.line }); }
  }
  decls[f] = L;
}
// line ranges a Where cites (a bare ":12-20" after a file counts too; the final review found an earlier version
// that skipped them): "shamble-game.js:10-20, 31" or "game:10-20"; bare numbers belong to the last file named
for (const r of rows) {
  r.refs = [];
  let cur = null;
  const re = /\b(?:shamble-)?(core|game|editor)(?:\.js)? ?:(\d{1,4})(?:\s*[-–]\s*(\d{1,4}))?|(index\.html|editor\.html|tools\/[\w./-]+|[\w-]+\.(?:md|mjs|json|py|yml))|(?<![\w.#x/])(\d{1,4})(?:\s*[-–]\s*(\d{1,4}))?(?![\w.%]|\.\d)/g;
  let m;
  while ((m = re.exec(r.where))) {
    if (m[1]) { cur = m[1]; r.refs.push({ f: cur, a: +m[2], b: m[3] ? +m[3] : +m[2] }); continue; }
    if (m[4]) { cur = null; continue; }
    if (!cur) continue;
    const a = +m[5], b = m[6] ? +m[6] : a;
    if (a >= 1 && b <= nlines[cur] && b >= a) r.refs.push({ f: cur, a, b });
  }
}
const out = {};
for (const f of ['core', 'game', 'editor']) {
  let named = 0, inRange = 0;
  const unnamed = [];
  for (const d of decls[f]) {
    const word = new RegExp('(^|[^\\w$])' + d.n.replace(/\$/g, '\\$') + '($|[^\\w$])');
    if (rows.some((r) => word.test(r.where))) { named++; continue; }
    if (rows.some((r) => r.refs.some((x) => x.f === f && d.l >= x.a && d.l <= x.b))) { inRange++; continue; }
    unnamed.push(d);
  }
  out[f] = { total: decls[f].length, named, in_range: inRange, unnamed };
  console.log(f, 'total', decls[f].length, 'named', named, 'in a cited range only', inRange, 'neither', unnamed.length);
}
if (!process.env.R7_FIRST) fs.writeFileSync(`${D}/coverage-final.json`, JSON.stringify(out, null, 1) + '\n');
