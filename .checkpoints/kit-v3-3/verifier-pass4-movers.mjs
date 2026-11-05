// (verifier pass 4 copy, with the C-PLAIN random lines and extra probe lines added to the role check)
// Movers, no-looser and reader-equality check (builder pass 3, day side 2: D-ASK and P3-7). In a flat copy: the new
// guard, the guard at 01b158c (old.mjs), the guard before the day side (pre.mjs: routine sides A and B done) and the
// test file with one extra test. It decides every row of every table with each guard (the row's own io); every Bash row
// once more under each role (night-shift, pr-advisor) with a plain fake io, old vs new; and parseShell's fields other
// than unreadable, pre vs new, on every Bash line of every table.
// usage: node builder-pass3-day2-movers.mjs <a3 root> <scratch dir> <pre.mjs>
import { readFileSync, writeFileSync, mkdirSync, copyFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { join } from 'node:path';
const [a3, dir, pre] = process.argv.slice(2);
mkdirSync(dir, { recursive: true });
writeFileSync(join(dir, 'routine-guard.mjs'), readFileSync(join(a3, '.claude/hooks/routine-guard.mjs')));
copyFileSync(pre, join(dir, 'pre.mjs'));
const old = spawnSync('git', ['show', '01b158c:.claude/hooks/routine-guard.mjs'], { cwd: a3, encoding: 'utf8' });
if (old.status !== 0) throw new Error('git show 01b158c failed');
writeFileSync(join(dir, 'old.mjs'), old.stdout);
const extra = `
test('movers', async () => {
  const old = await import('./old.mjs'), pre = await import('./pre.mjs'), cur = await import('./routine-guard.mjs');
  const rows = [];
  for (const [name, role, input, , opts] of CASES) rows.push(['CASES', name, input, () => fakeIo({ role, ...opts })]);
  for (const [name, input, , opts] of DAY_CASES) rows.push(['DAY_CASES', name, input, () => dayIo(opts)]);
  for (const [name, role, input, , opts] of ROUTINE_FIX_CASES) rows.push(['ROUTINE_FIX_CASES', name, input, () => dayIo({ ...opts, role })]);
  for (const [name, input, , opts] of NIGHT_SWITCH_CASES) rows.push(['NIGHT_SWITCH_CASES', name, input, () => nightIo(opts)]);
  for (const [name, input, , io] of PASS2_ROUTINE_CASES) rows.push(['PASS2_ROUTINE_CASES', name, input, io]);
  for (const [name, input, , io] of PASS3_ROUTINE_CASES) rows.push(['PASS3_ROUTINE_CASES', name, input, io]);
  for (const [name, input, , io] of PASS3B_COMMIT_CASES) rows.push(['PASS3B_COMMIT_CASES', name, input, io]);
  for (const [name, input, , opts] of PASS3_DAY_CASES) rows.push(['PASS3_DAY_CASES', name, input, () => dayIo(opts)]);
  const oio = (io) => ({ ...io, git: (args, cwd) => io.git(args.flatMap((a) => (a === '--no-relative' ? [a, '--ignore-submodules=none'] : [a])), cwd) });
  const show = (d) => (d ? (d.ask !== undefined && d.deny === undefined ? 'ask: ' + d.ask : 'deny: ' + d.deny) : 'null');
  const rank = (k) => (k === 'deny' ? 2 : k === 'ask' ? 1 : 0);
  const out = [];
  let n = 0, nRole = 0, looser = 0, dayLooser = 0;
  for (const [table, name, input, io] of rows) {
    const a = show(await old.decide(input, oio(io()))), p = show(await pre.decide(input, io())), b = show(await cur.decide(input, io()));
    n++;
    if (p !== b) out.push(['DAY2-MOVED', table, name, p.slice(0, 150), '->', b.slice(0, 150)].join(' | '));
    if (a !== b) {
      const ka = a.split(':')[0], kb = b.split(':')[0];
      if (rank(kb) < rank(ka)) dayLooser++;
      out.push([rank(kb) < rank(ka) ? 'PASS3-LOOSER' : 'PASS3-MOVED', table, name, a.slice(0, 150), '->', b.slice(0, 150)].join(' | '));
    }
  }
  const seen = new Set();
  for (const [table, name, input] of rows) {
    if (input?.tool_name !== 'Bash') continue;
    for (const role of ['night-shift', 'pr-advisor']) {
      const key = role + '\\0' + JSON.stringify(input.tool_input);
      if (seen.has(key)) continue;
      seen.add(key);
      const ka = kindOf(await old.decide(input, oio(fakeIo({ role })))), kb = kindOf(await cur.decide(input, fakeIo({ role })));
      nRole++;
      if (ka && !kb) { looser++; out.push(['ROLE-LOOSER', role, table, name].join(' | ')); }
      else if (ka !== kb) out.push(['ROLE-STRICTER', role, table, name, String(ka), '->', String(kb)].join(' | '));
    }
  }
  const r = seeded(20261105), r2 = seeded(4);
  const extraLines = [...Array.from({ length: 2000 }, (_, k) => plainLine(r, k).line), ...Array.from({ length: 1000 }, (_, k) => plainLine(r2, k).line),
    "git status > 'a*b'", 'git log -1 > docs/x.md', 'git commit -q -m m', 'git commit -am m', 'git push origin HEAD', 'git switch cs/ok', 'git fetch -q origin cs/ok',
    'date +%s', "date '+%H %M'", 'node .claude/scripts/checkpoint.mjs save s -m "x" --trailer "y"', 'gh pr view 38', 'gh api repos/o/r/pulls/38'];
  let nExtra = 0, extraLooser = 0;
  for (const l of extraLines) {
    for (const role of ['night-shift', 'pr-advisor']) {
      const input = ev('Bash', { command: l });
      const ka = kindOf(await old.decide(input, oio(fakeIo({ role })))), kb = kindOf(await cur.decide(input, fakeIo({ role })));
      nExtra++;
      if (ka && !kb) { extraLooser++; out.push(['EXTRA-LOOSER', role, JSON.stringify(l)].join(' | ')); }
    }
  }
  out.push('extra role decisions (C-PLAIN random lines + probes, old vs new): ' + nExtra + '; looser: ' + extraLooser);
  let nParse = 0, differ = 0;
  const lines = new Set(rows.filter(([, , i]) => i?.tool_name === 'Bash' && typeof i.tool_input?.command === 'string').map(([, , i]) => i.tool_input.command));
  for (const l of lines) {
    const { unreadable, ...b } = cur.parseShell(l), a = pre.parseShell(l);
    nParse++;
    if (JSON.stringify(a) !== JSON.stringify(b)) { differ++; out.push(['PARSE-DIFFERS', JSON.stringify(l)].join(' | ')); }
  }
  out.push('decisions compared (row io, 3 guards): ' + n + '; kind looser vs 01b158c: ' + dayLooser);
  out.push('role decisions compared (old vs new): ' + nRole + '; looser: ' + looser);
  out.push('parseShell lines compared (pre vs new, fields but unreadable): ' + nParse + '; differ: ' + differ);
  writeFileSync(join(HERE, 'movers.out'), out.join('\\n') + '\\n');
});
`;
writeFileSync(join(dir, 'routine-guard.test.mjs'), readFileSync(join(a3, '.claude/hooks/routine-guard.test.mjs'), 'utf8') + extra);
const r = spawnSync('node', ['--test', '--test-name-pattern=^movers$', 'routine-guard.test.mjs'], { cwd: dir, encoding: 'utf8' });
console.log(r.stdout.split('\n').filter((l) => /^# (pass|fail)|^not ok/.test(l)).join('\n'));
if (/^not ok/m.test(r.stdout)) console.log(r.stdout.split('\n').filter((l) => /Error|expected|actual|at /.test(l)).slice(0, 20).join('\n'));
console.log(readFileSync(join(dir, 'movers.out'), 'utf8'));
