// fix pass 4 no-looser harness (pattern: builder-pass3-day2-movers.mjs): old = the guard at 60ae124, new = the worktree's.
// (1) every row of every table, with its own io: kind and reason, old vs new; (2) every Bash line of every table, for the
// night shift and the PR Advisor with a plain fakeIo: kind, old vs new (no routine decision may get looser); (3) the
// routines' own lines (NIGHT_SHIFT.md, CHECKPOINTS.md, main's advisor.md): new must let each through.
// usage: node builder-pass4-movers.mjs <a3 root> <scratch dir>
import { readFileSync, writeFileSync, mkdirSync, rmSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { join, resolve } from 'node:path';
const [a3, dir] = process.argv.slice(2).map((x) => resolve(x));
rmSync(dir, { recursive: true, force: true }); mkdirSync(dir, { recursive: true });
writeFileSync(join(dir, 'routine-guard.mjs'), readFileSync(join(a3, '.claude/hooks/routine-guard.mjs')));
const old = spawnSync('git', ['show', '60ae124:.claude/hooks/routine-guard.mjs'], { cwd: a3, encoding: 'utf8' });
if (old.status !== 0) throw new Error('git show 60ae124 failed');
writeFileSync(join(dir, 'old.mjs'), old.stdout);
const extra = `
test('movers', async () => {
  const old = await import('./old.mjs'), cur = await import('./routine-guard.mjs');
  const rows = [];
  for (const [name, role, input, , opts] of CASES) rows.push(['CASES', name, input, () => fakeIo({ role, ...opts })]);
  for (const [name, input, , opts] of DAY_CASES) rows.push(['DAY_CASES', name, input, () => dayIo(opts)]);
  for (const [name, role, input, , opts] of ROUTINE_FIX_CASES) rows.push(['ROUTINE_FIX_CASES', name, input, () => dayIo({ ...opts, role })]);
  for (const [name, input, , opts] of NIGHT_SWITCH_CASES) rows.push(['NIGHT_SWITCH_CASES', name, input, () => nightIo(opts)]);
  for (const [name, input, , io] of PASS2_ROUTINE_CASES) rows.push(['PASS2_ROUTINE_CASES', name, input, io]);
  for (const [name, input, , io] of PASS3_ROUTINE_CASES) rows.push(['PASS3_ROUTINE_CASES', name, input, io]);
  for (const [name, input, , io] of PASS3B_COMMIT_CASES) rows.push(['PASS3B_COMMIT_CASES', name, input, io]);
  for (const [name, input, , opts] of PASS3_DAY_CASES) rows.push(['PASS3_DAY_CASES', name, input, () => dayIo(opts)]);
  for (const [name, input, , io] of PASS4_ROUTINE_CASES) rows.push(['PASS4_ROUTINE_CASES', name, input, io]);
  for (const [name, role, input, , opts] of PLAIN_BEHIND_CASES) rows.push(['PLAIN_BEHIND_CASES', name, input, () => fakeIo({ role, ...opts })]);
  const show = (d) => (d ? (d.ask !== undefined && d.deny === undefined ? 'ask: ' + d.ask : 'deny: ' + d.deny) : 'null');
  const rank = (k) => (k === 'deny' ? 2 : k === 'ask' ? 1 : 0);
  const out = [];
  let n = 0, looserRows = 0, nRole = 0, looser = 0, stricter = 0, dayMoved = 0;
  for (const [table, name, input, io] of rows) {
    const a = show(await old.decide(input, io())), b = show(await cur.decide(input, io()));
    n++;
    if (a !== b) {
      const ka = a.split(':')[0], kb = b.split(':')[0];
      if (rank(kb) < rank(ka)) looserRows++;
      if (!io().env.SHAMBLE_ROLE) dayMoved++;
      out.push([rank(kb) < rank(ka) ? 'ROW-LOOSER' : ka === kb ? 'ROW-REASON' : 'ROW-STRICTER', table, io().env.SHAMBLE_ROLE || 'day', name,
        a.slice(0, 120), '->', b.slice(0, 120)].join(' | '));
    }
  }
  const seen = new Set();
  for (const [table, name, input] of rows) {
    if (input?.tool_name !== 'Bash') continue;
    for (const role of ['night-shift', 'pr-advisor']) {
      const key = role + '\\0' + JSON.stringify(input.tool_input);
      if (seen.has(key)) continue;
      seen.add(key);
      const ka = kindOf(await old.decide(input, fakeIo({ role }))), kb = kindOf(await cur.decide(input, fakeIo({ role })));
      nRole++;
      if (ka && !kb) { looser++; out.push(['ROLE-LOOSER', role, table, name].join(' | ')); }
      else if (ka !== kb) { stricter++; out.push(['ROLE-STRICTER', role, table, name, JSON.stringify(input.tool_input.command), String(ka), '->', String(kb)].join(' | ')); }
    }
  }
  // the routines' own lines, placeholders filled (KIT_LINES of the C-PLAIN test, plus NIGHT_NPM and the gate)
  const own = [...KIT_LINES.map((l) => [l.startsWith('git fetch -q origin main &&') || l.startsWith('git diff --name-only') ? 'pr-advisor' : 'night-shift', l]),
    ['night-shift', NIGHT_NPM_CMD], ['night-shift', 'node tools/gate.mjs'], ['pr-advisor', 'TZ=Europe/Amsterdam date'],
    ['pr-advisor', \`git diff --name-only origin/main...\${SHA}\`], ['night-shift', \`git diff --name-only origin/main...\${SHA}\`]];
  let ownBad = 0;
  for (const [role, l] of own) {
    const ka = kindOf(await old.decide(sh(l), fakeIo({ role }))), kb = kindOf(await cur.decide(sh(l), fakeIo({ role })));
    if (kb !== null || ka !== kb) ownBad++;
    out.push(['OWN', role, JSON.stringify(l), String(ka), '->', String(kb)].join(' | '));
  }
  out.push('row decisions compared (each row with its own io): ' + n + '; kind looser: ' + looserRows + '; day rows moved: ' + dayMoved);
  out.push('role decisions compared (every Bash line x night-shift, pr-advisor; plain fakeIo): ' + nRole + '; looser: ' + looser + '; stricter: ' + stricter);
  out.push("routines' own lines: " + own.length + '; not null or moved: ' + ownBad);
  writeFileSync(join(HERE, 'movers.out'), out.join('\\n') + '\\n');
});
`;
writeFileSync(join(dir, 'routine-guard.test.mjs'), readFileSync(join(a3, '.claude/hooks/routine-guard.test.mjs'), 'utf8') + extra);
const r = spawnSync('node', ['--test', '--test-name-pattern=^movers$', 'routine-guard.test.mjs'], { cwd: dir, encoding: 'utf8' });
console.log(r.stdout.split('\n').filter((l) => /^# (pass|fail)|^not ok/.test(l)).join('\n'));
if (/^not ok/m.test(r.stdout)) console.log(r.stdout.split('\n').filter((l) => /Error|expected|actual|at /.test(l)).slice(0, 20).join('\n'));
console.log(readFileSync(join(dir, 'movers.out'), 'utf8'));
