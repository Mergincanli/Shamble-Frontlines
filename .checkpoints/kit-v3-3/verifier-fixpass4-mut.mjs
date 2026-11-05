// verifier fix pass 4: each PASS4 mutant on its own key row, decision only (no test harness), lite io.
// usage: node verifier-fixpass4-mut.mjs <a3 root> <scratch dir>
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';
const [a3, dir] = process.argv.slice(2).map((x) => resolve(x));
mkdirSync(dir, { recursive: true });
const guard = readFileSync(join(a3, '.claude/hooks/routine-guard.mjs'), 'utf8');
const t = readFileSync(join(a3, '.claude/hooks/routine-guard.test.mjs'), 'utf8');
const block = t.slice(t.indexOf('const PASS4_ROUTINE_MUTANTS = ['), t.indexOf("test('mutation runs (fix pass 4, R-PROG"));
const MUTS = eval(block.replace('const PASS4_ROUTINE_MUTANTS = ', '(').replace(/;\s*$/, ')'));
const lite = (role) => ({ env: { SHAMBLE_ROLE: role }, projectDir: '/repo', home: '/home/u', gitLongOptions: () => [],
  git: (args) => (args.join(' ') === 'rev-parse --show-toplevel' ? { code: 0, out: '/repo\n' } : { code: 0, out: '' }),
  gh: () => ({ code: 1, out: '' }), exists: () => false, realpath: (p) => p, readFile: () => '{}' });
const ev = (command) => ({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command }, cwd: '/repo', scratchpad_dir: '/tmp/s' });
const key = ['docs/do git status', 'TZ=x', '>docs/x.md git status', '2>&1 git status', 'cd docs && git status', '"git" status',
  'timeout 5 git status', 'PATH=x git status', 'TZ=x\ngit status', 'TZ=x; git status', 'xvfb-run -a git status', "git status > 'a*b'"];
const k = (d) => (d ? `deny: ${String(d.deny).replace(/^routine \(night-shift\): /, '').slice(0, 70)}` : 'null');
writeFileSync(join(dir, 'm-base.mjs'), guard);
const base = await import(pathToFileURL(join(dir, 'm-base.mjs')).href);
console.log('MUTANTS', MUTS.length);
for (const [i, [label, from, to]] of MUTS.entries()) {
  const n = guard.split(from).length - 1;
  writeFileSync(join(dir, `m${i}.mjs`), guard.replace(from, () => to));
  const m = await import(pathToFileURL(join(dir, `m${i}.mjs`)).href);
  const moved = [];
  for (const l of key) {
    let a, b; a = k(await base.decide(ev(l), lite('night-shift')));
    try { b = k(await m.decide(ev(l), lite('night-shift'))); } catch (e) { b = 'THREW ' + e.message; }
    if (a !== b) moved.push(`   ${JSON.stringify(l)}: ${a} -> ${b}`);
  }
  console.log(`== ${label} (target x${n}): ${moved.length} key rows move`); for (const x of moved) console.log(x);
}
