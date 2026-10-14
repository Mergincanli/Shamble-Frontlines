#!/usr/bin/env node
// tools/gate.mjs: the SHAMBLE FRONTLINES verification gate. Checks the WORKING TREE against a base git ref.
//   [1] syntax         node --check on each engine file and on each page's inline <script>
//   [2] concatenation  node --check on each page's scripts joined in load order (read from its <script src> tags)
//   [3] symbols        top-level symbols (tools/battery.js), base vs head: nothing dropped, no duplicates; prints +added / -dropped
//   [4] player build   battery.js check 4: no reachable, unguarded reference from index.html's build to an editor-only symbol
//   [5a] smoke         tools/smoke.mjs: the real pages run in headless Chromium (menu, deploy, play, pause; editor, Maps); any
//                      page error or console error FAILS
//   [5b] names         eslint-scope over each page's scripts and inline DOM handlers: every reference resolves to a local, a
//                      top-level declaration of that page's build, an ECMAScript built-in or tools/gate-globals.json
//                      (typeof operands exempt; index.html's refs to editor-only symbols are check 4's); no assignment to a
//                      const; every THREE.<member> exists in three r128
//   [5c] layering      shamble-core.js references no game, editor or inline-script top-level symbol, guarded or not
//   [5d] script order  each page loads exactly: three r128 (cdnjs) -> core -> game [-> editor, editor.html] -> one inline script
// Usage (from anywhere): node tools/gate.mjs [baseRef] [--allow-drop name1,name2] [--no-smoke | --smoke-only] [--json]
//   baseRef      default: the merge-base of HEAD with main or origin/main, whichever is newer
//   --allow-drop symbols the approved plan removes on purpose (anything else dropped FAILS)
//   --no-smoke   skip 5a (quick local runs; CI runs it as its own job). The result says so.
//   --smoke-only run only 5a (the CI smoke job)
//   --json       machine-readable result on stdout
// Exit code: 0 PASS, 1 FAIL, 2 usage or setup error (incl. 5a without Playwright/Chromium when nothing else failed).
// Needs Node 22, git and tools/node_modules (cd tools && npm ci);
// 5a also needs Playwright's Chromium (npx --no -- playwright install chromium, inside tools/).
import { execFileSync, spawnSync } from 'node:child_process';
import { mkdtempSync, writeFileSync, rmSync, readFileSync, existsSync, realpathSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve, dirname, sep } from 'node:path';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';
import vm from 'node:vm';

const TOOLS = dirname(fileURLToPath(import.meta.url));
const ROOT = resolve(TOOLS, '..');
const FILES = ['shamble-core.js', 'shamble-game.js', 'shamble-editor.js', 'index.html', 'editor.html'];
const OLD_PREFIX = 'shamble-frontlines-repo/';   // where the files lived before CS-1 moved them to the root

const die = msg => { console.error('gate: ' + msg); process.exit(2); };
const firstLine = s => String(s).split('\n')[0];
const USAGE = 'usage: node tools/gate.mjs [baseRef] [--allow-drop name1,name2] [--no-smoke | --smoke-only] [--json]';

// ---------- arguments ----------
let baseArg = null, asJson = false, allowDrop = new Set(), noSmoke = false, smokeOnly = false;
const argv = process.argv.slice(2);
for (let i = 0; i < argv.length; i++) {
  const a = argv[i];
  if (a === '--json') asJson = true;
  else if (a === '--no-smoke') noSmoke = true;
  else if (a === '--smoke-only') smokeOnly = true;
  else if (a === '--allow-drop' || a.startsWith('--allow-drop=')) {
    const v = a.includes('=') ? a.slice(a.indexOf('=') + 1) : argv[++i];
    if (!v) die('--allow-drop needs a comma-separated list\n' + USAGE);
    allowDrop = new Set(v.split(',').map(s => s.trim()).filter(Boolean));
  } else if (a === '-h' || a === '--help') { console.log(USAGE); process.exit(0); }
  else if (a.startsWith('-')) die('unknown option ' + a + '\n' + USAGE);
  else if (baseArg === null) baseArg = a;
  else die('only one baseRef\n' + USAGE);
}
if (noSmoke && smokeOnly) die('--no-smoke and --smoke-only exclude each other\n' + USAGE);
if (smokeOnly && (baseArg !== null || allowDrop.size)) die('--smoke-only takes no baseRef or --allow-drop\n' + USAGE);

const require = createRequire(import.meta.url);
let battery, acorn, walk, escope;
try { battery = require('./battery.js'); acorn = require('acorn'); walk = require('acorn-walk'); escope = require('eslint-scope'); }
catch (e) { die('cannot load the gate\'s dependencies (did you run: cd tools && npm ci?)\n' + e.message); }

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
  // The newer of the two merge-bases: a stale local main must not silently give an older base.
  const mb = {};
  for (const m of ['main', 'origin/main']) { const sha = tryGit('merge-base', 'HEAD', m); if (sha) mb[m] = sha.trim(); }
  const L = mb['main'], O = mb['origin/main'];
  const pick = (m, note) => ({ ref: `merge-base(HEAD, ${m})`, sha: mb[m], note });
  if (L && O && L !== O) {
    if (tryGit('merge-base', '--is-ancestor', L, O) !== null) return pick('origin/main', 'local main is behind origin/main, so origin/main is used (update it: git fetch origin main:main)');
    if (tryGit('merge-base', '--is-ancestor', O, L) !== null) return pick('main', 'local main is ahead of origin/main, so local main is used');
    return pick('origin/main', 'local main and origin/main have diverged; origin/main is used');
  }
  if (L) return pick('main');
  if (O) return pick('origin/main');
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
const tmp = realpathSync(mkdtempSync(join(tmpdir(), 'shamble-gate-')));   // realpath: node --check reports real paths (macOS /var -> /private/var)
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
  for (const s of battery.scanHtml(html).scripts) {   // the browser's view: commented-out tags don't count, any attribute quoting
    const a = battery.attrsOf(s.attrs), kind = battery.scriptKind(a), module = kind === 'module';
    if (kind === 'data') continue;
    if (Object.hasOwn(a, 'src')) out.push({ src: a.src, module });
    else if (s.body.trim()) out.push({ label: `${page} inline#${++n}`, code: s.body, lineOffset: html.slice(0, s.bodyStart).split('\n').length - 1, module });
  }
  return out;
}

// A page's classic scripts in load order as [{ label, code, lineOffset }]: local <script src> files read from the working tree,
// inline scripts as they are. External and module scripts are left out; broken srcs go to errs (check 2 reports them).
function classicParts(page, html, errs = [], external = new Set()) {
  const parts = [];
  for (const s of pageScripts(page, html)) {
    if (s.module) continue;   // module scripts get their own scope: not part of the classic-script build
    if (s.code) { parts.push(s); continue; }
    if (/^([a-z]+:)?\/\//i.test(s.src)) { external.add(s.src.split('/').pop()); continue; }
    const name = s.src.replace(/^\.\//, ''), abs = resolve(ROOT, name);
    if (!abs.startsWith(ROOT + sep)) { errs.push(`${page}: <script src="${s.src}"> points outside the repo`); continue; }
    const code = LOCAL[name] ?? (existsSync(abs) ? headFile(name) : null);
    if (code === null) { errs.push(`${page}: <script src="${s.src}"> does not exist`); continue; }
    parts.push({ label: name, code });
  }
  return parts;
}

// ---------- scope analysis for 5b and 5c (eslint-scope) ----------
// One classic script (or a DOM handler wrapped in a function): its top-level names (and which are const), its free
// references (every reference not resolved inside it; fine when some script of the same page declares the name at top level),
// assignments to its own constants, and the THREE.<member> names it uses.
function scopeOf(code) {
  const ast = acorn.parse(code, { ecmaVersion: 'latest', sourceType: 'script', ranges: true, locations: true });
  const typeofOps = new Set(), members = [], threeSet = new Set();
  const isThree = n => n.type === 'MemberExpression' && !n.computed && n.object.type === 'Identifier' && n.object.name === 'THREE' && n.property.type === 'Identifier';
  walk.simple(ast, {
    UnaryExpression(n) { if (n.operator === 'typeof') typeofOps.add(n.argument); },
    MemberExpression(n) { if (isThree(n)) members.push(n); },
    AssignmentExpression(n) { if (isThree(n.left)) threeSet.add(n.left.property.name); },
  });
  const sm = escope.analyze(ast, { ecmaVersion: 2025, sourceType: 'script' }), g = sm.globalScope;
  const isConst = v => v.defs.some(d => d.type === 'Variable' && d.parent.kind === 'const');
  const constWrites = sm.scopes.flatMap(sc => sc.references.filter(r => r.resolved && r.isWrite() && !r.init && isConst(r.resolved)))
    .map(r => ({ name: r.identifier.name, line: r.identifier.loc.start.line }));
  const freeThree = new Set(g.through.filter(r => r.identifier.name === 'THREE').map(r => r.identifier));
  return { top: new Set(g.variables.map(v => v.name)), topConst: new Set(g.variables.filter(isConst).map(v => v.name)), constWrites,
           free: g.through.map(r => ({ name: r.identifier.name, line: r.identifier.loc.start.line, typeofOp: typeofOps.has(r.identifier),
                                       write: r.isWrite() && !r.isRead(), assign: r.isWrite() })),
           three: members.filter(n => freeThree.has(n.object) && !typeofOps.has(n)).map(n => ({ name: n.property.name, line: n.property.loc.start.line })),
           threeSet };
}
const scopeCache = new Map();   // core and game are analysed once for both pages
const scoped = code => { if (!scopeCache.has(code)) scopeCache.set(code, scopeOf(code)); return scopeCache.get(code); };

// ---------- 5a smoke, and the result ----------
async function smokeCheck() {
  try {
    const { runSmoke } = await import('./smoke.mjs');
    const r = await runSmoke({ root: ROOT });
    add('5a', 'smoke', r.pass, r.summary, r.details, { pages: r.pages, setup: r.setup });
  } catch (e) { add('5a', 'smoke', false, 'tools/smoke.mjs could not run', [firstLine(e.message)]); }
}
const ORDER = ['1', '2', '3', '4', '5a', '5b', '5c', '5d'];
function finish() {
  checks.sort((a, b) => ORDER.indexOf(String(a.id)) - ORDER.indexOf(String(b.id)));
  const pass = checks.every(c => c.pass);
  const scope = smokeOnly ? 'smoke only' : noSmoke ? 'smoke skipped' : 'full';
  const tail = smokeOnly ? ' (smoke only: checks 1-4 and 5b-5d not run)' : noSmoke ? ' (5a smoke skipped: --no-smoke)' : '';
  if (asJson) {
    console.log(JSON.stringify({ pass, scope, base: base && { ref: base.ref, sha: base.sha, note: base.note }, head: 'working tree', checks }, null, 1));
  } else {
    console.log(base ? `SHAMBLE FRONTLINES gate | base ${base.ref} = ${base.sha.slice(0, 7)} | head = working tree`
                     : 'SHAMBLE FRONTLINES gate | smoke only | head = working tree');
    if (base && base.note) console.log('note: ' + base.note);
    for (const c of checks) {
      console.log(`${`[${c.id}]`.padEnd(4)} ${c.name.padEnd(14)} ${c.skipped ? 'SKIP' : c.pass ? 'PASS' : 'FAIL'}  ${c.summary}`);
      for (const d of c.details) console.log('      ' + d);
    }
    console.log((pass ? 'GATE PASS' : 'GATE FAIL') + tail);
  }
  // exit 2 when the only failure is a missing smoke setup (Playwright or Chromium): the code was not judged
  process.exit(pass ? 0 : checks.every(c => c.pass || c.setup) ? 2 : 1);
}

// ---------- run ----------
const checks = [];
const add = (id, name, pass, summary, details = [], extra = {}) => checks.push({ id, name, pass, summary, details, ...extra });
const base = smokeOnly ? null : resolveBase(baseArg);
if (smokeOnly) { await smokeCheck(); finish(); }
const head = toBuild(headFile);
const LOCAL = { 'shamble-core.js': head.core, 'shamble-game.js': head.game, 'shamble-editor.js': head.editor };
const PAGES = [['index.html', head.indexHtml], ['editor.html', head.editorHtml]];

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
  const summ = [], errs = [], external = new Set();
  for (const [page, html] of PAGES) {
    const parts = classicParts(page, html, errs, external);
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

// [5b] names: every free reference of each page's scripts and inline DOM handlers resolves, nothing assigns to a const,
// and every THREE.<member> exists in three r128 (tools/node_modules/three: the same build the pages load)
const BUILTINS = new Set(Object.getOwnPropertyNames(vm.runInNewContext('globalThis')));   // pristine V8: ECMAScript only
let LISTED, THREE_EXTRA, R128;
try { const j = JSON.parse(readFileSync(join(TOOLS, 'gate-globals.json'), 'utf8')); LISTED = new Set([...(j.browser || []), ...(j.library || [])]); THREE_EXTRA = j.three || []; }
catch (e) { die('cannot read tools/gate-globals.json: ' + e.message); }
try { const T = require('three'); if (String(T.REVISION) !== '128') throw new Error('found r' + T.REVISION); R128 = Object.keys(T); }
catch (e) { die('cannot load three r128 from tools/node_modules (cd tools && npm ci): ' + firstLine(e.message)); }
{
  const problems = [], summ = [];
  // index.html's references to editor-only symbols are check 4's job (it proves each one is guarded): same set as battery.js
  const indexTop = new Set(), editorOnly = new Set();
  try {
    for (const p of classicParts('index.html', head.indexHtml)) scoped(p.code).top.forEach(n => indexTop.add(n));
    for (const n of scoped(head.editor).top) if (!indexTop.has(n)) editorOnly.add(n);
  } catch { /* a syntax error: check 1 reports it, and the loop below names the file */ }
  let anyUndefined = false;
  for (const [page, html] of PAGES) {
    const units = [], broken = [], declared = new Set(), consts = new Set(), three = new Set([...R128, ...THREE_EXTRA]);
    for (const p of classicParts(page, html)) {
      try { const r = scoped(p.code); units.push([p.label, p.lineOffset || 0, r]); r.top.forEach(n => declared.add(n)); r.topConst.forEach(n => consts.add(n)); r.threeSet.forEach(n => three.add(n)); }
      catch (e) { broken.push(p.label); problems.push(`${p.label}: cannot analyse (${firstLine(e.message)}; see check 1)`); }
    }
    const handlers = battery.handlersOf(html);
    for (const h of handlers) {
      try { units.push([`${page} ${h.attr}="${h.code.slice(0, 40)}"`, h.line - 2, scopeOf(`(function (event) {\n${h.code}\n});`), true]); }
      catch (e) { problems.push(`${page} line ${h.line}: the ${h.attr} handler does not parse: "${h.code.slice(0, 60)}"`); }
    }
    const exempt = page === 'index.html' ? editorOnly : new Set();
    const ok = f => f.typeofOp || declared.has(f.name) || BUILTINS.has(f.name) || LISTED.has(f.name) || exempt.has(f.name);
    const found = { undef: new Map(), constw: new Map(), three: new Map() };
    const note = (kind, name, where) => { const m = found[kind]; if (!m.has(name)) m.set(name, []); m.get(name).push(where); };
    for (const [label, off, r, isHandler] of units) {
      const at = line => `${label} line ${line + off}`;
      for (const f of r.free) {
        if (!ok(f)) note('undef', f.name, at(f.line) + (!f.write ? '' : isHandler ? ' (handler code sees the page globals only: write this.x or document.x)' : ' (an assignment: it would create an implicit global)'));
        else if (f.assign && consts.has(f.name)) note('constw', f.name, at(f.line));
      }
      for (const w of r.constWrites) note('constw', w.name, at(w.line));
      for (const m of r.three) if (!three.has(m.name)) note('three', m.name, at(m.line));
    }
    const list = (m, say) => { for (const [name, where] of m) problems.push(`${page}: ${say(name)} (${where[0]}${where.length > 1 ? `; +${where.length - 1} more` : ''})`); };
    if (broken.length) problems.push(`${page}: unresolved names not listed, because ${broken.join(', ')} could not be analysed (its declarations are unknown)`);
    else { list(found.undef, n => `${n} is not defined`); anyUndefined ||= found.undef.size > 0; }
    list(found.constw, n => `assignment to const ${n}: throws when it runs`);
    list(found.three, n => `THREE.${n} does not exist in three r128`);
    summ.push(`${page}: ${units.length - handlers.length} scripts + ${handlers.length} DOM handlers, ${found.undef.size + found.constw.size + found.three.size} problems`);
  }
  if (anyUndefined) problems.push('a browser API the code really needs: add its name to tools/gate-globals.json (reviewed data, not a gate-tool change)');
  add('5b', 'names', !problems.length, summ.join(' | ') +
      ` | allowed: ${BUILTINS.size} built-ins (Node ${process.version}) + ${LISTED.size} in gate-globals.json + ${R128.length} THREE members (r128)`, problems);
}

// [5c] layering: shamble-core.js stands alone (no reference into game, editor or inline-script symbols, typeof included)
try {
  const core = scoped(head.core), upper = new Map();
  const above = [['shamble-game.js', head.game], ['shamble-editor.js', head.editor],
                 ...PAGES.flatMap(([page, html]) => classicParts(page, html).filter(p => !LOCAL[p.label]).map(p => [p.label, p.code]))];
  for (const [label, code] of above) { try { for (const n of scoped(code).top) if (!core.top.has(n) && !upper.has(n)) upper.set(n, label); } catch { /* check 1 */ } }
  const hits = core.free.filter(f => upper.has(f.name)), outside = new Set(core.free.filter(f => !core.top.has(f.name)).map(f => f.name));
  add('5c', 'layering', !hits.length, `shamble-core.js uses ${outside.size} names it does not declare; ${hits.length} references into game/editor/inline symbols`,
      hits.map(f => `shamble-core.js line ${f.line}: ${f.typeofOp ? 'typeof ' : ''}${f.name} (declared in ${upper.get(f.name)})`));
} catch (e) { add('5c', 'layering', false, 'cannot analyse shamble-core.js (see check 1)', [firstLine(e.message)]); }

// [5d] script order: exactly the expected <script> tags, in order, with no attribute that changes when they run
{
  const THREE_URL = 'https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js';
  const WANT = { 'index.html': [THREE_URL, 'shamble-core.js', 'shamble-game.js', 'inline'],
                 'editor.html': [THREE_URL, 'shamble-core.js', 'shamble-game.js', 'shamble-editor.js', 'inline'] };
  const short = s => s === THREE_URL ? 'three r128 (cdnjs)' : s;
  const problems = [], summ = [];
  for (const [page, html] of PAGES) {
    const got = [];
    for (const sc of battery.scanHtml(html).scripts) {   // commented-out tags don't count; attributes in any quoting
      const a = battery.attrsOf(sc.attrs), kind = battery.scriptKind(a);
      if (kind === 'data') continue;                     // a data block (type="application/json" etc.) never runs
      if (!Object.hasOwn(a, 'src') && !sc.body.trim()) continue;      // an empty inline script runs nothing
      const flags = [kind === 'module' ? 'type=module' : '', ...['async', 'defer', 'nomodule'].filter(f => Object.hasOwn(a, f))].filter(Boolean);
      got.push((Object.hasOwn(a, 'src') ? a.src.replace(/^\.\//, '') : 'inline') + (flags.length ? ` [${flags.join(' ')}]` : ''));
    }
    const want = WANT[page], same = got.length === want.length && got.every((g, i) => g === want[i]);
    if (!same) problems.push(`${page}: expected ${want.map(short).join(' -> ')}; found ${got.map(short).join(' -> ') || 'no scripts'}`);
    summ.push(`${page}: ${got.map(short).join(' -> ')}`);
  }
  add('5d', 'script order', !problems.length, summ.join(' | '), problems);
}

// [5a] smoke (slowest, so last)
if (noSmoke) add('5a', 'smoke', true, 'skipped (--no-smoke): the full gate or CI\'s smoke job runs it', [], { skipped: true });
else await smokeCheck();
finish();
