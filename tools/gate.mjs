#!/usr/bin/env node
// tools/gate.mjs: the SHAMBLE FRONTLINES verification gate. Checks the WORKING TREE against a base git ref.
//   [1] syntax         node --check on each engine file and on each page's inline <script>
//   [2] concatenation  node --check on each page's scripts joined in load order (read from its <script src> tags)
//   [3] symbols        top-level symbols (tools/battery.js), base vs head: nothing dropped, no duplicates; prints +added / -dropped
//   [4] player build   battery.js check 4: no reachable, unguarded reference from index.html's build to an editor-only symbol
// Usage (from anywhere): node tools/gate.mjs [baseRef] [--allow-drop name1,name2] [--json]
//   baseRef      default: the merge-base of HEAD with main (origin/main when there is no local main)
//   --allow-drop symbols the approved plan removes on purpose (anything else dropped FAILS)
//   --json       machine-readable result on stdout
// Exit code: 0 PASS, 1 FAIL, 2 usage or setup error. Needs Node 22, git and tools/node_modules (cd tools && npm ci).
import { execFileSync, spawnSync } from 'node:child_process';
import { mkdtempSync, writeFileSync, rmSync, readFileSync, existsSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve, dirname, sep } from 'node:path';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';

const TOOLS = dirname(fileURLToPath(import.meta.url));
const ROOT = resolve(TOOLS, '..');
const FILES = ['shamble-core.js', 'shamble-game.js', 'shamble-editor.js', 'index.html', 'editor.html'];
const OLD_PREFIX = 'shamble-frontlines-repo/';   // where the files lived before CS-1 moved them to the root

const die = msg => { console.error('gate: ' + msg); process.exit(2); };
const USAGE = 'usage: node tools/gate.mjs [baseRef] [--allow-drop name1,name2] [--json]';

// ---------- arguments ----------
let baseArg = null, asJson = false, allowDrop = new Set();
const argv = process.argv.slice(2);
for (let i = 0; i < argv.length; i++) {
  const a = argv[i];
  if (a === '--json') asJson = true;
  else if (a === '--allow-drop' || a.startsWith('--allow-drop=')) {
    const v = a.includes('=') ? a.slice(a.indexOf('=') + 1) : argv[++i];
    if (!v) die('--allow-drop needs a comma-separated list\n' + USAGE);
    allowDrop = new Set(v.split(',').map(s => s.trim()).filter(Boolean));
  } else if (a === '-h' || a === '--help') { console.log(USAGE); process.exit(0); }
  else if (a.startsWith('-')) die('unknown option ' + a + '\n' + USAGE);
  else if (baseArg === null) baseArg = a;
  else die('only one baseRef\n' + USAGE);
}

const require = createRequire(import.meta.url);
let battery;
try { battery = require('./battery.js'); } catch (e) { die('cannot load tools/battery.js (did you run: cd tools && npm ci?)\n' + e.message); }

// ---------- git ----------
const tryGit = (...a) => {
  try { return execFileSync('git', ['-C', ROOT, ...a], { encoding: 'utf8', maxBuffer: 64 << 20, stdio: ['ignore', 'pipe', 'pipe'] }); }
  catch { return null; }
};
function resolveBase(arg) {
  if (arg) {
    const sha = tryGit('rev-parse', '--verify', '--quiet', arg + '^{commit}');
    if (!sha) die(`base ref not found: ${arg}`);
    return { ref: arg, sha: sha.trim() };
  }
  for (const m of ['main', 'origin/main']) {
    const sha = tryGit('merge-base', 'HEAD', m);
    if (sha) return { ref: `merge-base(HEAD, ${m})`, sha: sha.trim() };
  }
  die('no baseRef given and no merge-base with main or origin/main (a shallow clone needs full history)');
}
const lf = s => s.replace(/\r\n/g, '\n');
function baseFile(sha, f) {
  for (const p of [f, OLD_PREFIX + f]) { const s = tryGit('show', `${sha}:${p}`); if (s !== null) return lf(s); }
  die(`${f} not found at base ${sha.slice(0, 7)}`);
}
const headFile = f => {
  const p = join(ROOT, f);
  if (!existsSync(p)) die(`${f} is missing from the working tree`);
  return lf(readFileSync(p, 'utf8'));
};
const toBuild = get => ({ core: get('shamble-core.js'), game: get('shamble-game.js'), editor: get('shamble-editor.js'),
                          indexHtml: get('index.html'), editorHtml: get('editor.html') });

// ---------- node --check on a temp copy ----------
const tmp = mkdtempSync(join(tmpdir(), 'shamble-gate-'));
process.on('exit', () => rmSync(tmp, { recursive: true, force: true }));
let nTmp = 0;
// parts: [{ label, code }]. Returns null on success, else the first error lines with the line mapped back to its part.
function nodeCheck(parts) {
  const starts = []; let line = 1, code = '';
  for (const [i, p] of parts.entries()) {
    if (i) { code += '\n;\n'; line += 2; }
    starts.push(line); code += p.code; line += p.code.split('\n').length - 1;
  }
  const file = join(tmp, `${++nTmp}.${parts.length === 1 && parts[0].module ? 'mjs' : 'cjs'}`);
  writeFileSync(file, code);
  const r = spawnSync(process.execPath, ['--check', file], { encoding: 'utf8' });
  if (r.status === 0) return null;
  const out = (r.stderr || r.stdout || 'syntax error').trim().split('\n');
  const m = out[0].startsWith(file) && out[0].slice(file.length).match(/^:(\d+)/);
  if (m) {
    const L = +m[1]; let k = 0;
    while (k + 1 < starts.length && starts[k + 1] <= L) k++;
    out[0] = `${parts[k].label} line ${L - starts[k] + 1 + (parts[k].lineOffset || 0)}`;
  } else out[0] = `${parts.map(p => p.label).join(' + ')}: ${out[0]}`;
  return out.filter(l => l.trim() && !l.startsWith('Node.js v') && !/^\s+at /.test(l)).slice(0, 4).join('\n      ');
}

// A page's scripts in order: { src } for <script src>, { label, code, module } for inline JavaScript.
// Inline scripts of another type (JSON data, templates) are not JavaScript and are skipped, as the syntax hook does.
function pageScripts(page, html) {
  const out = []; let n = 0;
  for (const m of html.matchAll(/<script\b([^>]*)>([\s\S]*?)<\/script>/gi)) {
    const src = (m[1].match(/\bsrc\s*=\s*["']([^"']+)["']/i) || [])[1];
    const type = (m[1].match(/\btype\s*=\s*["']?([^"'\s>]+)/i) || [])[1];
    if (type && !/^(module|text\/javascript|application\/javascript)$/i.test(type)) continue;
    const startLine = html.slice(0, m.index + m[0].indexOf('>') + 1).split('\n').length;
    if (src) out.push({ src, module: /^module$/i.test(type || '') });
    else if (m[2].trim()) out.push({ label: `${page} inline#${++n}`, code: m[2], lineOffset: startLine - 1, module: /^module$/i.test(type || '') });
  }
  return out;
}

// ---------- run ----------
const base = resolveBase(baseArg);
const head = toBuild(headFile);
const checks = [];
const add = (id, name, pass, summary, details = []) => checks.push({ id, name, pass, summary, details });

// [1] syntax
{
  const units = [
    { label: 'shamble-core.js', code: head.core }, { label: 'shamble-game.js', code: head.game },
    { label: 'shamble-editor.js', code: head.editor },
    ...pageScripts('index.html', head.indexHtml).filter(s => s.code), ...pageScripts('editor.html', head.editorHtml).filter(s => s.code),
  ];
  const errs = units.map(u => [u.label, nodeCheck([u])]).filter(([, e]) => e);
  add(1, 'syntax', !errs.length, `${units.length - errs.length}/${units.length} sources parse (${units.map(u => u.label).join(', ')})`,
      errs.map(([, e]) => e));
}

// [2] load-order concatenation per page
{
  const local = { 'shamble-core.js': head.core, 'shamble-game.js': head.game, 'shamble-editor.js': head.editor };
  const summ = [], errs = [], external = new Set();
  for (const [page, html] of [['index.html', head.indexHtml], ['editor.html', head.editorHtml]]) {
    const parts = [];
    for (const s of pageScripts(page, html)) {
      if (s.module) continue;   // module scripts get their own scope: not part of the classic-script concatenation
      if (s.code) { parts.push(s); continue; }
      if (/^([a-z]+:)?\/\//i.test(s.src)) { external.add(s.src.split('/').pop()); continue; }
      const name = s.src.replace(/^\.\//, ''), abs = resolve(ROOT, name);
      if (!abs.startsWith(ROOT + sep)) { errs.push(`${page}: <script src="${s.src}"> points outside the repo`); continue; }
      const code = local[name] ?? (existsSync(abs) ? headFile(name) : null);
      if (code === null) { errs.push(`${page}: <script src="${s.src}"> does not exist`); continue; }
      parts.push({ label: name, code });
    }
    summ.push(`${page}: ${parts.map(p => p.label.replace(/^.*? (inline#\d+)$/, '$1')).join(' + ')}`);
    const e = nodeCheck(parts);
    if (e) errs.push(`${page} concatenation: ${e}`);
  }
  add(2, 'concatenation', !errs.length, summ.join(' | ') + (external.size ? ` (external, skipped: ${[...external].join(', ')})` : ''), errs);
}

// [3] symbols and [4] player build, both from battery.js
let H = null, B = null;
try { H = battery.analyze(head); } catch (e) { add(3, 'symbols', false, 'battery.js could not analyse the working tree', [String(e.message)]); }
try { B = battery.analyze(toBuild(f => baseFile(base.sha, f))); } catch (e) { if (H) add(3, 'symbols', false, 'battery.js could not analyse the base', [String(e.message)]); }
if (H && B) {
  const KEYS = Object.keys(H.symbols);
  const where = syms => { const m = new Map(); for (const k of KEYS) for (const n of syms[k] || []) m.set(n, [...(m.get(n) || []), k]); return m; };
  const hw = where(H.symbols), bw = where(B.symbols);
  const added = [...hw.keys()].filter(n => !bw.has(n)).sort();
  const droppedAll = [...bw.keys()].filter(n => !hw.has(n)).sort();
  const dropped = droppedAll.filter(n => !allowDrop.has(n)), allowed = droppedAll.filter(n => allowDrop.has(n));
  const moved = [...hw.keys()].filter(n => bw.has(n) && hw.get(n).length === 1 && hw.get(n).join() !== bw.get(n).join()).sort();
  const cross = [...hw.entries()].filter(([, f]) => new Set(f).size > 1).map(([n, f]) => `${n} (${[...new Set(f)].join(', ')})`);
  const intra = KEYS.flatMap(k => (H.intraDupes[k] || []).map(n => `${n} (twice in ${k})`));
  const counts = KEYS.map(k => `${k} ${H.symbols[k].length}`).join(' / ');
  const details = [];
  if (added.length) details.push('+ added: ' + added.join(', '));
  if (dropped.length) details.push('- DROPPED: ' + dropped.join(', '));
  if (allowed.length) details.push('- dropped (allowed by --allow-drop): ' + allowed.join(', '));
  const unusedAllow = [...allowDrop].filter(n => !droppedAll.includes(n));
  if (unusedAllow.length) details.push('note: --allow-drop names that were not dropped: ' + unusedAllow.join(', '));
  if (moved.length) details.push('~ moved: ' + moved.map(n => `${n} (${bw.get(n).join(',')} -> ${hw.get(n).join(',')})`).join(', '));
  if (cross.length) details.push('! CROSS-FILE DUPLICATES: ' + cross.join(', '));
  if (intra.length) details.push('! DUPLICATES WITHIN A FILE: ' + intra.join(', '));
  add(3, 'symbols', !dropped.length && !cross.length && !intra.length,
      `${counts} | +${added.length} -${droppedAll.length} | cross-file duplicates ${cross.length}, within-file ${intra.length}`, details);
}
if (H) {
  const c4 = H.check4, flagged = c4.flagged || [], dom = c4.domHandlerRefs || [];
  add(4, 'player build', !flagged.length && !dom.length,
      `${c4.refsToEditorOnly} refs to editor-only symbols: ${c4.guarded} guarded, ${c4.dead} dead, ${flagged.length} flagged` +
      (dom.length ? `, ${dom.length} DOM-handler problems` : ''),
      [...flagged.map(r => `FLAGGED ${r.name} at ${r.file} line ${r.line} (in ${r.owner}): unguarded on a reachable path, throws in index.html`),
       ...dom.map(d => d.parseError ? `index.html handler does not parse: "${d.handler}"` : `index.html handler uses editor-only ${d.editorOnlyRef}: "${d.handler}"`)]);
} else if (!checks.some(c => c.id === 4)) add(4, 'player build', false, 'not run: battery.js could not analyse the working tree');

checks.sort((a, b) => a.id - b.id);
const pass = checks.every(c => c.pass);
if (asJson) {
  console.log(JSON.stringify({ pass, base: { ref: base.ref, sha: base.sha }, head: 'working tree', checks }, null, 1));
} else {
  console.log(`SHAMBLE FRONTLINES gate | base ${base.ref} = ${base.sha.slice(0, 7)} | head = working tree`);
  for (const c of checks) {
    console.log(`[${c.id}] ${c.name.padEnd(14)} ${c.pass ? 'PASS' : 'FAIL'}  ${c.summary}`);
    for (const d of c.details) console.log('      ' + d);
  }
  console.log(pass ? 'GATE PASS' : 'GATE FAIL');
}
process.exit(pass ? 0 : 1);
