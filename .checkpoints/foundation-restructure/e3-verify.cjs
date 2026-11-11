// Verifies e3-mutations.json against the 44446a5 files (git show; nothing in the working tree is touched):
// every find occurs exactly once in its file at 44446a5 AND once in the file as edited so far (edits apply in order);
// every edit changes the file; the result parses (JS files with acorn, each page's inline scripts and handlers), except P3.
const path = require('path'), fs = require('fs'), cp = require('child_process');
const ROOT = path.resolve(__dirname, '../..'), T = path.join(ROOT, 'tools');
const acorn = require(require.resolve('acorn', { paths: [T] })), battery = require(path.join(T, 'battery.js'));
const show = (rev, f) => cp.execFileSync('git', ['-C', ROOT, 'show', `${rev}:${f}`], { encoding: 'utf8', maxBuffer: 64 << 20 });
const FIVE = ['shamble-core.js', 'shamble-game.js', 'shamble-editor.js', 'index.html', 'editor.html'];
const BASE = Object.fromEntries(FIVE.map(f => [f, show('44446a5', f)]));
const count = (s, x) => { let n = 0, i = 0; while ((i = s.indexOf(x, i)) >= 0) { n++; i += x.length; } return n; };
const parses = (f, s) => { try {
  if (f.endsWith('.js')) acorn.parse(s, { ecmaVersion: 'latest', sourceType: 'script' });
  else { for (const sc of battery.scanHtml(s).scripts) { const a = battery.attrsOf(sc.attrs); if (!Object.hasOwn(a, 'src') && battery.scriptKind(a) === 'classic') acorn.parse(sc.body, { ecmaVersion: 'latest', sourceType: 'script' }); }
         for (const h of battery.handlersOf(s)) acorn.parse(`(function (event) {\n${h.code}\n});`, { ecmaVersion: 'latest', sourceType: 'script' }); }
  return true; } catch (e) { return e.message; } };
const M = JSON.parse(fs.readFileSync(path.join(__dirname, 'e3-mutations.json'), 'utf8'));
const IDS = ['P1','P2','P3','P4',...Array.from({length:22},(_, i)=>'M'+(i+1)),'C1','C2','C3','PC1','PC2'];
let bad = 0; const fail = m => { bad++; console.log('FAIL ' + m); };
const ids = M.map(m => m.id); if (ids.join() !== IDS.join()) fail('ids/order: ' + ids.join());
const CHECKS = new Set(['1','2','3','4','5a','5b','5c','5d']);
let nEdits = 0;
for (const m of M) {
  if (!Array.isArray(m.expected) || !m.expected.every(c => CHECKS.has(c))) fail(m.id + ' expected');
  if (/^(C|PC)/.test(m.id) ? m.expected.length : !m.expected.length) fail(m.id + ' expected emptiness');
  if (m.flags && m.id !== 'PC1') fail(m.id + ' has flags');
  const cur = { ...BASE }, touched = new Set();
  for (const e of m.edits) {
    nEdits++;
    if (!FIVE.includes(e.file)) { fail(`${m.id} file ${e.file}`); continue; }
    if (e.from_commit) { const s = show(e.from_commit, e.file); if (s === cur[e.file]) fail(m.id + ' from_commit equals base'); cur[e.file] = s; touched.add(e.file); continue; }
    const nb = count(BASE[e.file], e.find), nc = count(cur[e.file], e.find);
    if (nb !== 1) fail(`${m.id} find occurs ${nb}x in ${e.file} at 44446a5: ${JSON.stringify(e.find.slice(0, 60))}`);
    if (nc !== 1) fail(`${m.id} find occurs ${nc}x in ${e.file} after earlier edits`);
    if (e.find === e.replace) fail(m.id + ' no-op edit');
    cur[e.file] = cur[e.file].replace(e.find, () => e.replace); touched.add(e.file);
  }
  for (const f of touched) {
    if (cur[f] === BASE[f]) fail(`${m.id} leaves ${f} unchanged`);
    const p = parses(f, cur[f]);
    if (m.id === 'P3' ? p === true : p !== true) fail(`${m.id} ${f} parse: ${p}`);
  }
  console.log(`ok-so-far ${m.id}: ${m.edits.length} edit(s) on ${[...touched].join(', ')}; changed bytes ${[...touched].map(f => cur[f].length - BASE[f].length).join(',')}`);
}
// PC2 must be a pure move: the moved text appears once in core and zero times in game afterwards
console.log(`${M.length} entries, ${nEdits} edits; ${bad ? bad + ' FAILURES' : 'ALL CHECKS PASS'}`);
process.exit(bad ? 1 : 0);
