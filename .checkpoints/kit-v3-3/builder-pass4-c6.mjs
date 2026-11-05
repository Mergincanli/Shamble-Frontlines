// fix pass 4, C6 proof for the six rows moved out of main's mutation table (advisor-pass4 §4): byte-identical with main
// 12d7961; deleted from head's copy of main's table; with R-PLAIN and kitPath on, no CASES row kills them; with R-PLAIN
// off (four rows) or the kitPath checks off (two rows), CASES rows kill them; plainForm refuses each R-PLAIN-off killer,
// and the head guard refuses each kitPath-off killer. usage: node builder-pass4-c6.mjs <a3 root> <scratch dir>
import { readFileSync, writeFileSync, mkdirSync, rmSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';
const [a3, dir] = process.argv.slice(2).map((x) => resolve(x));
rmSync(dir, { recursive: true, force: true }); mkdirSync(dir, { recursive: true });
const guard = readFileSync(join(a3, '.claude/hooks/routine-guard.mjs'), 'utf8');
const head = readFileSync(join(a3, '.claude/hooks/routine-guard.test.mjs'), 'utf8');
const main = spawnSync('git', ['show', '12d7961:.claude/hooks/routine-guard.test.mjs'], { cwd: a3, encoding: 'utf8' }).stdout;
const dumpFile = join(dir, 'cases.json');
writeFileSync(join(dir, 'routine-guard.test.mjs'), `${head}\nif (process.env.DUMP_CASES) writeFileSync(process.env.DUMP_CASES, ` +
  'JSON.stringify(CASES.map(([n, r, i, ref]) => [n, r, i, ref])));\n');
const run = (src, pattern, extraEnv = {}) => {
  writeFileSync(join(dir, 'routine-guard.mjs'), src);
  const r = spawnSync('node', ['--test', `--test-name-pattern=${pattern}`, 'routine-guard.test.mjs'], { cwd: dir, encoding: 'utf8', env: { ...process.env, ...extraEnv } });
  return [...r.stdout.matchAll(/^not ok \d+ - (.*)$/gm)].map((m) => m[1]);
};
run(guard, '^NOTHING$', { DUMP_CASES: dumpFile });
const CASES = JSON.parse(readFileSync(dumpFile, 'utf8'));
const byName = new Map(CASES.map((c) => [c[0], c]));
const PLAIN_OFF = 'if (role) { const found = plainForm(command); if (found) return deny(plainReason(role, found)); }';
const KIT_OFF = [['const k = kitPath(p, cwd, io);', 'const k = null;'], ['const kit = kitPath(p, vcwd, io);', 'const kit = null;']];
const plainOff = guard.replace(PLAIN_OFF, () => '');
const kitOff = KIT_OFF.reduce((s, [f, t]) => s.replace(f, () => t), guard);
const rowLine = (src, label) => src.split('\n').find((l) => l.startsWith(`  ['${label}', `));
const { plainForm, decide } = await import(pathToFileURL(join(a3, '.claude/hooks/routine-guard.mjs')).href);
const moved = [['the fixed shell set (catch-all)', 'PLAIN'], ['a program by a path', 'PLAIN'], ['environment set on a command', 'PLAIN'],
  ['re-exec wrappers', 'PLAIN'], ['protected writes (Edit/Write tool)', 'KIT'], ['protected redirect targets', 'KIT']];
const CASE_PAT = '^(refused|goes on): ';
for (const [label, where] of moved) {
  const mLine = rowLine(main, label);
  const hLines = head.split('\n').filter((l) => l === mLine);
  // the row is a line of main's own test file (this repository's tracked source), read as the array literal it is
  const row = eval(`[${mLine.trim().replace(/,$/, '')}]`)[0];
  const [, from, to] = row;
  console.log(`== ${label} (${where === 'PLAIN' ? 'PLAIN_BEHIND_MUTANTS' : 'KIT_BEHIND_MUTANTS'})`);
  console.log('  byte-identical lines in head:', hLines.length, '(the moved row; 1 = present once, deleted from MUTANTS)');
  const mutsStart = head.indexOf('const MUTANTS = ['), mutsEnd = head.indexOf('];', mutsStart);
  console.log('  in head\'s copy of main\'s MUTANTS:', head.slice(mutsStart, mutsEnd).includes(mLine) ? 'STILL THERE' : 'deleted',
    '| comment naming it:', head.slice(mutsStart, mutsEnd).split('\n').some((l) => l.trim().startsWith('//') && l.includes(`'${label}'`)) ? 'yes' : 'no');
  const on = run(guard.replace(from, to), CASE_PAT).filter((n) => byName.has(n.replace(/^(refused|goes on): /, '')));
  console.log('  CASES rows that kill it with R-PLAIN and kitPath on:', on.length);
  const base = where === 'PLAIN' ? plainOff : kitOff;
  const offFails = run(base, CASE_PAT).filter((n) => byName.has(n.replace(/^(refused|goes on): /, '')));
  const killers = run(base.replace(from, to), CASE_PAT).map((n) => n.replace(/^(refused|goes on): /, '')).filter((n) => byName.has(n));
  console.log(`  ${where === 'PLAIN' ? 'R-PLAIN' : 'kitPath'} off, unmutated CASES failures:`, offFails.length, `| killers: ${killers.length}`);
  for (const k of killers) {
    const [, role, input] = byName.get(k);
    const cmd = input.tool_name === 'Bash' ? input.tool_input.command : `${input.tool_name} ${input.tool_input.file_path ?? ''}`;
    let why;
    if (where === 'PLAIN') { const f = plainForm(cmd); why = f === null ? 'plainForm ACCEPTS' : `plainForm refuses: ${f.prog ? `R-PROG (${f.prog})` : f}`; }
    else { const d = await decide(input, { ...fakeIoLite(role) }); why = d?.deny ? `head refuses: ${d.deny.slice(0, 110)}` : 'head LETS IT THROUGH'; }
    console.log(`    killer [${role || 'day'}] ${JSON.stringify(cmd)}: ${why}`);
  }
}
// a minimal stand-in of the test file's fakeIo for the KIT killers (Write/Edit and redirects on a kit path in /repo)
function fakeIoLite(role) {
  return { env: role ? { SHAMBLE_ROLE: role } : {}, projectDir: '/repo', home: '/home/u', gitLongOptions: () => [],
    git: (args) => (args.join(' ') === 'rev-parse --show-toplevel' ? { code: 0, out: '/repo\n' } : { code: 0, out: '' }),
    gh: () => ({ code: 1, out: '' }), exists: () => true, realpath: (p) => p, readFile: () => '{}' };
}
