// battery.js: AST analysis of the five-file build. The authority for gate checks 3-4 (tools/gate.mjs wraps it).
// CLI: node tools/battery.js <dirWithAllFiveFiles>  ->  prints JSON { symbols, intraDupes, check4 }.
// Module: const { analyze, readBuild, handlersOf } = require('./battery.js'); analyze({ core, game, editor, indexHtml, editorHtml }).
// Setup: cd tools && npm ci (acorn + acorn-walk resolve from tools/node_modules).
// Check 3 input: top-level declared symbols per file (the shared-global surface).
// Check 4: every reference in the PLAYER build (core + game + index inline + index DOM handlers)
// to a symbol that exists ONLY in shamble-editor.js, classified as
//   guarded  - inside a mode==='editor'/'charlab' branch, an && short-circuit on such a test,
//              a typeof-guard on an editor symbol, or itself a typeof operand;
//   dead     - inside a top-level function never reachable from load-time code;
//   FLAGGED  - unguarded on a reachable path (would throw in index.html).
// AST-based on purpose: a per-line "is there a mode=== here" scan is what let placeCursor slip.
const fs = require('fs'), path = require('path');
const acorn = require('acorn');
const walk = require('acorn-walk');

// every inline classic <script> of a page (no src; no type, or a JavaScript type), joined in page order
const inlineOf = html => [...html.matchAll(/<script\b([^>]*)>([\s\S]*?)<\/script>/gi)]
  .filter(m => !/\bsrc\s*=/i.test(m[1]) && (!/\btype\s*=/i.test(m[1]) || /\btype\s*=\s*["']?(text|application)\/javascript\b/i.test(m[1])))
  .map(m => m[2]).join('\n');
const unentity = s => s.replace(/&quot;/g, '"').replace(/&#0*39;|&apos;/g, "'").replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&amp;/g, '&');
// a page's inline DOM handlers (on…="…", either quote style), entities decoded: [{ attr, code, line }]. Also used by gate check 5b.
const handlersOf = html => [...html.matchAll(/\s(on[a-z]+)\s*=\s*(?:"([^"]*)"|'([^']*)')/gi)]
  .map(m => ({ attr: m[1].toLowerCase(), code: unentity(m[2] ?? m[3]), line: html.slice(0, m.index + 1).split('\n').length }));
function readBuild(dir) {
  const rd = f => fs.readFileSync(path.join(dir, f), 'utf8');
  return { core: rd('shamble-core.js'), game: rd('shamble-game.js'), editor: rd('shamble-editor.js'),
           indexHtml: rd('index.html'), editorHtml: rd('editor.html') };
}

function analyze(build) {
  const indexHtml = build.indexHtml;
  const src = {
    core: build.core, game: build.game, editor: build.editor,
    indexInline: inlineOf(indexHtml), editorInline: inlineOf(build.editorHtml),
  };
  const ast = {};
  for (const k in src) ast[k] = acorn.parse(src[k], { ecmaVersion: 'latest', sourceType: 'script', locations: true });

  function patternNames(p, out) {
    if (!p) return;
    switch (p.type) {
      case 'Identifier': out.push(p.name); break;
      case 'ObjectPattern': for (const pr of p.properties) patternNames(pr.type === 'RestElement' ? pr.argument : pr.value, out); break;
      case 'ArrayPattern': for (const e of p.elements) patternNames(e, out); break;
      case 'RestElement': patternNames(p.argument, out); break;
      case 'AssignmentPattern': patternNames(p.left, out); break;
    }
  }
  function topDecls(prog) {
    const out = [];
    for (const st of prog.body) {
      if ((st.type === 'FunctionDeclaration' || st.type === 'ClassDeclaration') && st.id) out.push({ name: st.id.name, node: st, kind: 'fn' });
      else if (st.type === 'VariableDeclaration') for (const d of st.declarations) { const ns = []; patternNames(d.id, ns); for (const n of ns) out.push({ name: n, node: d, kind: 'var' }); }
    }
    return out;
  }
  const decls = {}; for (const k in ast) decls[k] = topDecls(ast[k]);
  const symbols = {}, intraDupes = {};
  for (const k in decls) {
    symbols[k] = decls[k].map(d => d.name).sort();
    const seen = new Set(), dup = new Set();
    for (const n of symbols[k]) { if (seen.has(n)) dup.add(n); seen.add(n); }
    intraDupes[k] = [...dup];
  }

  // ---- check 4 ----
  const PLAYER = ['core', 'game', 'indexInline'];
  const playerSet = new Set(PLAYER.flatMap(k => symbols[k]));
  const editorOnly = new Set(symbols.editor.filter(n => !playerSet.has(n)));

  // locals declared inside a function (params + var/let/const/function/class/catch), not descending into nested functions
  const localsCache = new WeakMap();
  const isFn = n => n && (n.type === 'FunctionDeclaration' || n.type === 'FunctionExpression' || n.type === 'ArrowFunctionExpression');
  function localsOf(fn) {
    if (localsCache.has(fn)) return localsCache.get(fn);
    const names = new Set(); const tmp = [];
    for (const p of fn.params) patternNames(p, tmp);
    if (fn.type === 'FunctionExpression' && fn.id) tmp.push(fn.id.name);
    (function scan(n) {
      if (!n || typeof n.type !== 'string') return;
      if (n !== fn && isFn(n)) { if (n.type === 'FunctionDeclaration' && n.id) tmp.push(n.id.name); return; }
      if (n.type === 'VariableDeclaration') for (const d of n.declarations) patternNames(d.id, tmp);
      if (n.type === 'ClassDeclaration' && n.id) tmp.push(n.id.name);
      if (n.type === 'CatchClause' && n.param) patternNames(n.param, tmp);
      for (const key in n) {
        if (key === 'loc' || key === 'start' || key === 'end') continue;
        const v = n[key];
        if (Array.isArray(v)) v.forEach(scan); else if (v && typeof v.type === 'string') scan(v);
      }
    })(fn.body);
    tmp.forEach(x => names.add(x)); localsCache.set(fn, names); return names;
  }
  const shadowed = (name, anc) => anc.some(a => isFn(a) && localsOf(a).has(name));

  const isModeTest = (e, ops) => e && e.type === 'BinaryExpression' && ops.includes(e.operator) &&
    ((e.left.type === 'Identifier' && e.left.name === 'mode' && e.right.type === 'Literal' && (e.right.value === 'editor' || e.right.value === 'charlab')) ||
     (e.right.type === 'Identifier' && e.right.name === 'mode' && e.left.type === 'Literal' && (e.left.value === 'editor' || e.left.value === 'charlab')));
  const isTypeofGuard = e => e && e.type === 'BinaryExpression' && ['!==', '!=', '===', '=='].includes(e.operator) &&
    [[e.left, e.right], [e.right, e.left]].some(([u, l]) => u.type === 'UnaryExpression' && u.operator === 'typeof' &&
      u.argument.type === 'Identifier' && editorOnly.has(u.argument.name) && l.type === 'Literal' &&
      ((['!==', '!='].includes(e.operator) && l.value === 'undefined') || (['===', '=='].includes(e.operator) && l.value === 'function')));
  function isGuard(e) {
    if (!e) return false;
    if (isModeTest(e, ['===', '==']) || isTypeofGuard(e)) return true;
    if (e.type === 'LogicalExpression' && e.operator === '&&') return isGuard(e.left) || isGuard(e.right);
    return false;
  }
  const isNegGuard = e => isModeTest(e, ['!==', '!=']);
  function guardOf(anc) {
    for (let i = anc.length - 2; i >= 0; i--) {
      const p = anc[i], c = anc[i + 1];
      if (p.type === 'UnaryExpression' && p.operator === 'typeof' && i === anc.length - 2) return 'typeof-operand';
      if ((p.type === 'IfStatement' || p.type === 'ConditionalExpression')) {
        if (c === p.consequent && isGuard(p.test)) return 'mode/typeof branch';
        if (c === p.alternate && isNegGuard(p.test)) return 'negated-mode else';
      }
      if (p.type === 'LogicalExpression' && p.operator === '&&' && c === p.right && isGuard(p.left)) return '&& short-circuit';
      if (p.type === 'SwitchCase' && p.test && p.test.type === 'Literal' && (p.test.value === 'editor' || p.test.value === 'charlab')) {
        const sw = anc[i - 1]; if (sw && sw.type === 'SwitchStatement' && sw.discriminant.type === 'Identifier' && sw.discriminant.name === 'mode') return 'switch(mode) case';
      }
    }
    return null;
  }

  // names referenced inside a subtree (expression-context identifiers + compound-assignment targets)
  function refsIn(node) {
    const out = new Set();
    walk.ancestor(node, {
      Identifier(n) { out.add(n.name); },
      AssignmentExpression(n) { if (n.operator !== '=' && n.left.type === 'Identifier') out.add(n.left.name); },
    });
    return out;
  }
  // reachability over the player build: roots = every top-level statement that runs on load
  // (non-function statements + ALL var initialisers, conservatively) + identifiers in index.html's inline DOM handlers
  const declByName = new Map();
  for (const k of PLAYER) for (const d of decls[k]) { if (!declByName.has(d.name)) declByName.set(d.name, []); declByName.get(d.name).push(d); }
  const reachable = new Set(), queue = [];
  const visitNames = names => { for (const n of names) if (declByName.has(n) && !reachable.has(n)) { reachable.add(n); queue.push(n); } };
  for (const k of PLAYER) for (const st of ast[k].body) if (st.type !== 'FunctionDeclaration' && st.type !== 'ClassDeclaration') visitNames(refsIn(st));
  const domHandlers = [];
  for (const { code: h } of handlersOf(indexHtml)) {   // inline DOM handlers, either quote style
    try { const names = refsIn(acorn.parse(h, { ecmaVersion: 'latest', allowReturnOutsideFunction: true })); visitNames(names); for (const n of names) if (editorOnly.has(n)) domHandlers.push({ handler: h.slice(0, 80), editorOnlyRef: n }); } catch (e) { domHandlers.push({ handler: h.slice(0, 80), parseError: String(e.message) }); }
  }
  while (queue.length) { const n = queue.shift(); for (const d of declByName.get(n)) visitNames(refsIn(d.node)); }

  const refs = [];
  for (const k of PLAYER) {
    walk.ancestor(ast[k], {
      Identifier(n, anc) {
        if (!editorOnly.has(n.name) || shadowed(n.name, anc)) return;
        const top = anc[1]; let owner = '<top-level>', live = true;
        if (top && top.type === 'FunctionDeclaration') { owner = top.id.name; live = reachable.has(owner); }
        else if (top && top.type === 'VariableDeclaration') { const ns = []; top.declarations.forEach(d => patternNames(d.id, ns)); owner = ns.join(','); }
        const g = guardOf(anc);
        refs.push({ file: k, line: n.loc.start.line, name: n.name, owner, status: g ? 'guarded' : (live ? 'FLAGGED' : 'dead'), guard: g || undefined });
      },
    });
  }
  const tally = s => refs.filter(r => r.status === s).length;
  return {
    symbols, intraDupes,
    check4: { editorOnlySymbols: editorOnly.size, refsToEditorOnly: refs.length, guarded: tally('guarded'), dead: tally('dead'),
              flagged: refs.filter(r => r.status === 'FLAGGED'), domHandlerRefs: domHandlers, reachableTopLevel: reachable.size, refs },
  };
}

module.exports = { analyze, readBuild, handlersOf };

if (require.main === module) {
  const dir = process.argv[2];
  if (!dir) { console.error('usage: node tools/battery.js <dirWithAllFiveFiles>'); process.exit(2); }
  console.log(JSON.stringify(analyze(readBuild(dir))));
}
