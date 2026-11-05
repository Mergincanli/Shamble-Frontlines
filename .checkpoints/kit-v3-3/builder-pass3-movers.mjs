// Movers and no-looser check (builder pass 3, routine side A). In a flat copy: the new guard, the guard at 01b158c
// (old.mjs) and the test file with one extra test that decides every existing row of every table with both guards (the
// row's own io), and every routine Bash row once more under each role (night-shift, pr-advisor) with a plain fake io.
// Prints each row whose kind or reason moved, and any routine line the new guard lets through where the old refused.
// usage: node builder-pass3-movers.mjs <a3 root> <scratch dir>
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { join } from 'node:path';
const [a3, dir] = process.argv.slice(2);
mkdirSync(dir, { recursive: true });
writeFileSync(join(dir, 'routine-guard.mjs'), readFileSync(join(a3, '.claude/hooks/routine-guard.mjs')));
const old = spawnSync('git', ['show', '01b158c:.claude/hooks/routine-guard.mjs'], { cwd: a3, encoding: 'utf8' });
if (old.status !== 0) throw new Error('git show 01b158c failed');
writeFileSync(join(dir, 'old.mjs'), old.stdout);
const extra = `
test('movers', async () => {
  const old = await import('./old.mjs');
  const rows = [];
  for (const [name, role, input, , opts] of CASES) rows.push(['CASES', name, input, () => fakeIo({ role, ...opts })]);
  for (const [name, input, , opts] of DAY_CASES) rows.push(['DAY_CASES', name, input, () => dayIo(opts)]);
  for (const [name, role, input, , opts] of ROUTINE_FIX_CASES) rows.push(['ROUTINE_FIX_CASES', name, input, () => dayIo({ ...opts, role })]);
  for (const [name, input, , opts] of NIGHT_SWITCH_CASES) rows.push(['NIGHT_SWITCH_CASES', name, input, () => nightIo(opts)]);
  for (const [name, input, , io] of PASS2_ROUTINE_CASES) rows.push(['PASS2_ROUTINE_CASES', name, input, io]);
  const show = (d) => (d ? (d.ask !== undefined && d.deny === undefined ? 'ask: ' + d.ask : 'deny: ' + d.deny) : 'null');
  const out = [];
  let n = 0, looser = 0;
  for (const [table, name, input, io] of rows) {
    const a = show(await old.decide(input, io())), b = show(await decide(input, io()));
    n++;
    if (a !== b) out.push(['MOVED', table, name, a.slice(0, 160), '->', b.slice(0, 160)].join(' | '));
  }
  const seen = new Set();
  for (const [table, name, input] of rows) {
    if (input?.tool_name !== 'Bash') continue;
    for (const role of ['night-shift', 'pr-advisor']) {
      const key = role + '\\0' + JSON.stringify(input.tool_input);
      if (seen.has(key)) continue;
      seen.add(key);
      const ka = kindOf(await old.decide(input, fakeIo({ role }))), kb = kindOf(await decide(input, fakeIo({ role })));
      n++;
      if (ka && !kb) { looser++; out.push(['LOOSER', role, table, name].join(' | ')); }
      else if (ka !== kb) out.push(['STRICTER', role, table, name, String(ka), '->', String(kb)].join(' | '));
    }
  }
  writeFileSync(join(HERE, 'movers.out'), out.join('\\n') + '\\n' + 'decisions compared: ' + n + ', looser: ' + looser + '\\n');
});
`;
writeFileSync(join(dir, 'routine-guard.test.mjs'), readFileSync(join(a3, '.claude/hooks/routine-guard.test.mjs'), 'utf8') + extra);
const r = spawnSync('node', ['--test', '--test-name-pattern=^movers$', 'routine-guard.test.mjs'], { cwd: dir, encoding: 'utf8' });
console.log(r.stdout.split('\n').filter((l) => /^# (pass|fail)|^not ok/.test(l)).join('\n'));
console.log(readFileSync(join(dir, 'movers.out'), 'utf8'));
