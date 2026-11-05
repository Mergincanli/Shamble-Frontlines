// fix pass 4: for each in-suite mutant (PASS4_ROUTINE_MUTANTS, and PLAIN_BEHIND's S3 mutant with R-PLAIN off), the rows
// that change (kind or reason) in a flat copy. usage: node builder-pass4-mut.mjs <a3 root> <scratch dir>
import { readFileSync, writeFileSync, mkdirSync, rmSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { join, resolve } from 'node:path';
const [a3, dir] = process.argv.slice(2).map((x) => resolve(x));
rmSync(dir, { recursive: true, force: true }); mkdirSync(dir, { recursive: true });
const guard = readFileSync(join(a3, '.claude/hooks/routine-guard.mjs'), 'utf8');
const head = readFileSync(join(a3, '.claude/hooks/routine-guard.test.mjs'), 'utf8');
const dump = join(dir, 'muts.json');
writeFileSync(join(dir, 'routine-guard.test.mjs'), `${head}\nif (process.env.DUMP_MUTS) writeFileSync(process.env.DUMP_MUTS, ` +
  'JSON.stringify({ pass4: PASS4_ROUTINE_MUTANTS, behind: PLAIN_BEHIND_MUTANTS.slice(-1) }));\n');
const run = (src, pattern, extraEnv = {}) => {
  writeFileSync(join(dir, 'routine-guard.mjs'), src);
  const r = spawnSync('node', ['--test', `--test-name-pattern=${pattern}`, 'routine-guard.test.mjs'], { cwd: dir, encoding: 'utf8', env: { ...process.env, ...extraEnv } });
  return [...r.stdout.matchAll(/^not ok \d+ - (.*)$/gm)].map((m) => m[1]);
};
run(guard, '^NOTHING$', { DUMP_MUTS: dump });
const { pass4, behind } = JSON.parse(readFileSync(dump, 'utf8'));
console.log('unmutated, fix pass 4 rows failing:', run(guard, '^routine \\(fix pass 4\\)').length);
for (const [label, from, to] of pass4) {
  if (guard.split(from).length !== 2) { console.log(label, 'TARGET NOT UNIQUE'); continue; }
  const f = run(guard.replace(from, () => to), '^routine \\(fix pass 4\\)').map((n) => n.replace(/^routine \(fix pass 4\) /, ''));
  console.log(`== ${label}: ${f.length} rows move`);
  for (const n of f) console.log(`   ${n}`);
}
const PLAIN_OFF = 'if (role) { const found = plainForm(command); if (found) return deny(plainReason(role, found)); }';
const off = guard.replace(PLAIN_OFF, () => '');
const lite = (role) => ({ env: { SHAMBLE_ROLE: role }, projectDir: '/repo', home: '/home/u', gitLongOptions: () => [],
  git: (args) => (args.join(' ') === 'rev-parse --show-toplevel' ? { code: 0, out: '/repo\n' } : { code: 0, out: '' }),
  gh: () => ({ code: 1, out: '' }), exists: () => true, realpath: (p) => p, readFile: () => '{}' });
const ev = (command) => ({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command }, cwd: '/repo', scratchpad_dir: '/tmp/s' });
for (const [label, from, to] of behind) {
  writeFileSync(join(dir, 'off.mjs'), off); writeFileSync(join(dir, 'offm.mjs'), off.replace(from, () => to));
  const a = await import(join(dir, 'off.mjs')), b = await import(join(dir, 'offm.mjs'));
  console.log(`== (R-PLAIN off) ${label}`);
  for (const [role, line] of [['night-shift', '>CLAUDE.md'], ['pr-advisor', 'TZ=x >.claude/settings.json'],
    ['night-shift', 'git status; >.claude/settings.json'], ['night-shift', '>docs/x.md']]) {
    const k = (d) => (d ? `deny: ${String(d.deny).slice(0, 80)}` : 'null');
    console.log(`   [${role}] ${JSON.stringify(line)}: R-PLAIN off ${k(await a.decide(ev(line), lite(role)))} | + mutant ${k(await b.decide(ev(line), lite(role)))}`);
  }
}
