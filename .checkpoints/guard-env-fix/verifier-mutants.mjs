// Verifier scratch (guard-env-fix): independent check of the new DAY_MUTANTS rows and the new DAY_CASES rows.
// Calls decide() only (fake io from the test file); runs no shell command. Copies go under .gate/guard-env-fix/.
// Usage: node .checkpoints/guard-env-fix/verifier-mutants.mjs <repoDir> [<mainGuardFile>]
import { mkdirSync, readFileSync, writeFileSync, copyFileSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const repo = process.argv[2], mainGuard = process.argv[3];
const out = join(repo, '.gate/guard-env-fix/vmut');
mkdirSync(out, { recursive: true });
const H = join(repo, '.claude/hooks');
copyFileSync(join(H, 'guard.mjs'), join(out, 'guard.mjs'));
const guardSrc = readFileSync(join(H, 'routine-guard.mjs'), 'utf8');
writeFileSync(join(out, 'routine-guard.mjs'), guardSrc);
let t = readFileSync(join(H, 'routine-guard.test.mjs'), 'utf8');
t = t.replace("import { test } from 'node:test';", 'const test = () => {};');
t += '\nexport { dayIo as __dayIo, kindOf as __kindOf, DAY_MUTANTS as __MUT };\n';
writeFileSync(join(out, 'routine-guard.test.mjs'), t);
const T = await import(pathToFileURL(join(out, 'routine-guard.test.mjs')).href);
const { DAY_CASES, __dayIo: dayIo, __kindOf: kindOf, __MUT: MUT } = T;
const isNew = (s) => /^(SEC-U7B-1|SA-R1-|SA-R2-)/.test(s);
console.log(`DAY_CASES ${DAY_CASES.length}, new ${DAY_CASES.filter((c) => isNew(c[0])).length}; DAY_MUTANTS ${MUT.length}, new ${MUT.filter((m) => isNew(m[0])).length}`);
async function run(file, label) {
  const m = await import(pathToFileURL(file).href);
  const caught = [];
  for (const [name, input, want, opts] of DAY_CASES) {
    let d; try { d = await m.decide(input, dayIo(opts)); } catch { d = 'threw'; }
    const k = d === 'threw' ? 'threw' : kindOf(d);
    if (k !== want) caught.push(`${name} (want ${want}, got ${k})`);
  }
  return caught;
}
let i = 0;
for (const [label, from, to] of MUT) {
  if (!isNew(label)) continue;
  const n = guardSrc.split(from).length - 1;
  const f = join(out, `m${i++}.mjs`);
  writeFileSync(f, guardSrc.replace(from, to));
  const c = await run(f, label);
  console.log(`MUTANT ${label} | occurrences ${n} | cases failing ${c.length} | first: ${c[0] ?? 'NONE (survives)'}`);
}
const base = await run(join(out, 'routine-guard.mjs'), 'head');
console.log(`head guard: cases failing ${base.length}`);
if (mainGuard) {
  const f = join(out, 'main-guard.mjs');
  copyFileSync(mainGuard, f);
  const c = await run(f, 'main');
  const newFail = c.filter((s) => isNew(s));
  console.log(`main guard with head's DAY_CASES: failing ${c.length} (new rows ${newFail.length}, old rows ${c.length - newFail.length})`);
  for (const s of c.filter((x) => !isNew(x))) console.log(`  OLD ROW FAILS ON MAIN: ${s}`);
  const kinds = {}; for (const s of newFail) { const k = s.replace(/.*\(want (\S+), got (\S+)\)$/, '$1->$2'); kinds[k] = (kinds[k] ?? 0) + 1; }
  console.log('  new rows failing on main by want->got:', JSON.stringify(kinds));
}
