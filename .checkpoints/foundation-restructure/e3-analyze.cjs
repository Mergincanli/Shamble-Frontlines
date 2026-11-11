// E3 helper analysis (read-only): symbol deltas for M6, editor free names in the index build (M20/M22), PC2 candidates, wrType free names.
const path = require('path'), fs = require('fs'), cp = require('child_process');
const ROOT = path.resolve(__dirname, '../..'), T = path.join(ROOT, 'tools');
const req = m => require(require.resolve(m, { paths: [T] }));
const acorn = req('acorn'), escope = req('eslint-scope'), battery = require(path.join(T, 'battery.js'));
const vm = require('vm');
const show = (rev, f) => cp.execFileSync('git', ['-C', ROOT, 'show', `${rev}:${f}`], { encoding: 'utf8', maxBuffer: 64 << 20 });
const F = ['shamble-core.js', 'shamble-game.js', 'shamble-editor.js', 'index.html', 'editor.html'];
const B = Object.fromEntries(F.map(f => [f, show('44446a5', f)]));
const parse = c => acorn.parse(c, { ecmaVersion: 'latest', sourceType: 'script', ranges: true, locations: true });
const scope = c => { const ast = parse(c), g = escope.analyze(ast, { ecmaVersion: 2025, sourceType: 'script' }).globalScope;
  return { ast, top: new Set(g.variables.map(v => v.name)), free: g.through.map(r => r.identifier.name) }; };
const BUILT = new Set(Object.getOwnPropertyNames(vm.runInNewContext('globalThis')));
const gg = JSON.parse(fs.readFileSync(path.join(T, 'gate-globals.json'), 'utf8')); const LIST = new Set([...gg.browser, ...gg.library]);
const inl = h => battery.scanHtml(h).scripts.filter(s => !/src/.test(s.attrs) && s.body.trim()).map(s => s.body);
const core = scope(B['shamble-core.js']), game = scope(B['shamble-game.js']), ed = scope(B['shamble-editor.js']);
const idxInl = inl(B['index.html']).map(scope), edInl = inl(B['editor.html']).map(scope);
// 1. M6
const oldEd = show('5858864', 'shamble-editor.js');
const build = ed => ({ core: B['shamble-core.js'], game: B['shamble-game.js'], editor: ed, indexHtml: B['index.html'], editorHtml: B['editor.html'] });
const a = battery.analyze(build(B['shamble-editor.js'])), b = battery.analyze(build(oldEd));
const sa = new Set(a.symbols.editor), sb = new Set(b.symbols.editor);
console.log('M6 editor symbols 44446a5', sa.size, 'old', sb.size);
console.log('M6 dropped (in 44446a5, not in old):', [...sa].filter(n => !sb.has(n)).join(', '));
console.log('M6 added (in old only):', [...sb].filter(n => !sa.has(n)).join(', '));
console.log('M6 check4 flagged old:', (b.check4.flagged || []).length, 'dom', (b.check4.domHandlerRefs || []).length);
const oldS = scope(oldEd), decl = new Set([...core.top, ...game.top, ...oldS.top, ...edInl.flatMap(s => [...s.top])]);
console.log('M6 old editor free names unresolved in editor.html build:', [...new Set(oldS.free.filter(n => !decl.has(n) && !BUILT.has(n) && !LIST.has(n)))].join(', '));
// 2. M20/M22: editor in the index build
const idecl = new Set([...core.top, ...game.top, ...ed.top, ...idxInl.flatMap(s => [...s.top])]);
console.log('M20/M22 editor free names unresolved in index.html build + editor:', [...new Set(ed.free.filter(n => !idecl.has(n) && !BUILT.has(n) && !LIST.has(n)))].join(', ') || '(none)');
// 3. PC2 candidates: game top-level function declarations whose free names (incl. inner) all resolve to core tops/builtins/listed, not referenced by core
const fnFree = (code, node) => { const c = code.slice(node.start, node.end); return scope(c).free; };
const cands = [];
for (const st of game.ast.body) if (st.type === 'FunctionDeclaration' && st.id) {
  const fr = fnFree(B['shamble-game.js'], st);
  if (fr.every(n => core.top.has(n) || BUILT.has(n) || LIST.has(n))) cands.push(`${st.id.name} (line ${st.loc.start.line}, ${st.end - st.start} chars)`);
}
console.log('PC2 candidates:', cands.join('; '));
// 4. wrType
const w = game.ast.body.find(s => s.type === 'FunctionDeclaration' && s.id && s.id.name === 'wrType');
const wf = [...new Set(fnFree(B['shamble-game.js'], w))];
console.log('wrType lines', w.loc.start.line, '-', w.loc.end.line, 'free:', wf.join(', '));
console.log('wrType free names that are game/editor/inline tops (5c):', wf.filter(n => !core.top.has(n) && (game.top.has(n) || ed.top.has(n))).join(', '));
