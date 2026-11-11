// Verifier (final, 2.1a): top-level declared names of tools/gate.mjs and tools/smoke.mjs at two refs; prints + and -.
// Read-only (git show). usage: node verifier-toplevel.mjs <baseRef> <headRef>
import { execFileSync } from 'node:child_process';
import { createRequire } from 'node:module';
const require = createRequire('/home/user/Shamble-Frontlines/tools/package.json');
const acorn = require('acorn');
const [A, B] = process.argv.slice(2);
const REPO = '/home/user/Shamble-Frontlines';
function names(ref, f) {
  const src = execFileSync('git', ['show', `${ref}:${f}`], { cwd: REPO, encoding: 'utf8' });
  const ast = acorn.parse(src.replace(/^#!.*/, ''), { ecmaVersion: 'latest', sourceType: 'module', allowHashBang: true });
  const out = new Set();
  const pat = p => { if (!p) return; if (p.type === 'Identifier') out.add(p.name); else if (p.type === 'ObjectPattern') p.properties.forEach(q => pat(q.value || q.argument)); else if (p.type === 'ArrayPattern') p.elements.forEach(pat); else if (p.type === 'AssignmentPattern') pat(p.left); else if (p.type === 'RestElement') pat(p.argument); };
  for (let n of ast.body) {
    if (n.type === 'ExportNamedDeclaration' || n.type === 'ExportDefaultDeclaration') n = n.declaration || n;
    if (!n) continue;
    if (n.type === 'FunctionDeclaration' || n.type === 'ClassDeclaration') out.add(n.id.name);
    else if (n.type === 'VariableDeclaration') n.declarations.forEach(d => pat(d.id));
    else if (n.type === 'ImportDeclaration') n.specifiers.forEach(s => out.add(s.local.name));
  }
  return out;
}
for (const f of ['tools/gate.mjs', 'tools/smoke.mjs']) {
  const a = names(A, f), b = names(B, f);
  console.log(f, 'count', a.size, '->', b.size, '| +[' + [...b].filter(x => !a.has(x)).join(', ') + '] -[' + [...a].filter(x => !b.has(x)).join(', ') + ']');
}
