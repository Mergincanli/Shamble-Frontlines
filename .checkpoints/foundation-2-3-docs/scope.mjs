// E5, the scope check: every path this change touches against main's guard (protectedRel, mergeAllowed), imported from
// a copy of origin/main's .claude/hooks/routine-guard.mjs written under .gate/ (never the branch's own copy). Every path
// must be under docs/ (or .checkpoints/ until the last commit) and none may be protected.
//   node .checkpoints/foundation-2-3-docs/scope.mjs [<head>]
import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const BASE = '4d21c1a7a4d39613bffce5d5f9bbc92f2f88a300';
const head = process.argv[2] ?? 'HEAD';
const dir = join(process.cwd(), '.gate/fdn23-docs/guard-main');
mkdirSync(dir, { recursive: true });
const src = execFileSync('git', ['show', 'origin/main:.claude/hooks/routine-guard.mjs'], { encoding: 'utf8' });
const main = execFileSync('git', ['rev-parse', 'origin/main'], { encoding: 'utf8' }).trim();
writeFileSync(join(dir, 'routine-guard.mjs'), src);
const { protectedRel, mergeAllowed } = await import(join(dir, 'routine-guard.mjs'));
const paths = execFileSync('git', ['diff', '--name-only', '--no-renames', `${BASE}...${head}`], { encoding: 'utf8' }).split('\n').filter(Boolean);
const raw = execFileSync('git', ['diff', '--raw', '--no-renames', `${BASE}...${head}`], { encoding: 'utf8' }).split('\n').filter(Boolean);
const problems = [];
for (const l of raw) { const m = l.split(/\s+/); if (m[1] && !['100644', '000000'].includes(m[1])) problems.push(`mode ${m[1]}: ${l}`); }
const rows = paths.map((p) => ({ p, prot: !!protectedRel(p), merge: !!mergeAllowed(p) }));
for (const r of rows) {
  if (!r.p.startsWith('docs/') && !r.p.startsWith('.checkpoints/')) problems.push(`outside docs/: ${r.p}`);
  if (r.prot) problems.push(`protected by main's guard: ${r.p}`);
}
console.log(`main's guard from origin/main ${main.slice(0, 7)}; ${paths.length} paths changed ${BASE.slice(0, 7)}...${head}`);
for (const r of rows.filter((x) => !x.p.startsWith('.checkpoints/'))) console.log(`  ${r.p}: protectedRel ${r.prot}, mergeAllowed ${r.merge}`);
const cp = rows.filter((x) => x.p.startsWith('.checkpoints/')).length;
if (cp) console.log(`  (.checkpoints/: ${cp} paths, deleted in the last commit before ready)`);
console.log(`${problems.length ? 'FAIL' : 'OK'}: no protected path, every path under docs/${cp ? ' or .checkpoints/' : ''}, file modes plain`);
for (const p of problems) console.log(`  ${p}`);
process.exit(problems.length ? 1 : 0);
