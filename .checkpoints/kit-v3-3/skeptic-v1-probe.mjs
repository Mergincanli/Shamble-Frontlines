// verifier delta probe (A3, ed3ea8c): (P1) day cd tracking vs a function, a DEBUG trap and an alias defined in the
// line; (P2) night: other folder movers; (P3) auto-5 #12 (a ref moved in the same line as the switch) and #13 (rev-parse
// lookup of refs/heads/<name>; git branch -m/-c names). Scratch repos under <a3>/.gate/kit-v3-3/vd-probe. Prints
// decisions (allow / ask / deny + the first 90 chars of the reason) and real bash/git outcomes (folder or changed names).
// Usage: node vd-probe.mjs <a3 worktree>
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
const A3 = process.argv[2];
const base = join(A3, '.gate/kit-v3-3/skv1-probe');
rmSync(base, { recursive: true, force: true });
mkdirSync(base, { recursive: true });
Object.assign(process.env, { GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t',
  GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' });
delete process.env.SHAMBLE_ROLE;
const g = (cwd, ...a) => { const r = spawnSync('git', a, { cwd, encoding: 'utf8' }); return { code: r.status, out: (r.stdout || '').trim(), err: (r.stderr || '').trim() }; };
const origin = join(base, 'origin.git'), proj = join(base, 'proj'), wt = join(base, 'wt');
g(base, 'init', '-q', '--bare', '-b', 'main', origin);
mkdirSync(join(proj, '.claude'), { recursive: true }); mkdirSync(join(proj, 'docs'));
g(base, 'init', '-q', '-b', 'main', proj);
writeFileSync(join(proj, 'CLAUDE.md'), 'rules\n'); writeFileSync(join(proj, '.claude/settings.json'), '{}\n');
writeFileSync(join(proj, 'docs/a.md'), 'a\n');
g(proj, 'add', '-A'); g(proj, 'commit', '-q', '-m', 'base'); g(proj, 'remote', 'add', 'origin', origin); g(proj, 'push', '-q', 'origin', 'main');
const mk = (br, file, text) => { g(proj, 'switch', '-q', '-c', br, 'main'); writeFileSync(join(proj, file), text); g(proj, 'commit', '-qam', br); g(proj, 'push', '-q', 'origin', br); g(proj, 'switch', '-q', 'main'); return g(proj, 'rev-parse', br).out; };
const ok = mk('cs/ok', 'docs/a.md', 'ok\n');
const evil = mk('cs/evil', '.claude/settings.json', '{"evil":1}\n');
g(proj, 'fetch', '-q', 'origin');
g(proj, 'worktree', 'add', '-q', wt, '-b', 'cs/wt', 'main');
process.env.CLAUDE_PROJECT_DIR = proj;
const { decide, realIo } = await import(join(A3, '.claude/hooks/routine-guard.mjs'));
const io = realIo();
const run = async (label, cmd, role = '', cwd = proj) => {
  io.env = { ...process.env, ...(role ? { SHAMBLE_ROLE: role } : {}) };
  const d = await decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command: cmd }, cwd, session_id: 's' }, io);
  const v = d === null ? 'allow' : d.ask ? 'ASK' : d.deny ? 'DENY' : JSON.stringify(d).slice(0, 60);
  console.log(`${label} [${role || 'day'}]: ${v}${d && (d.ask || d.deny) ? ' - ' + String(d.ask || d.deny).slice(0, 90) : ''}`);
};
const bashPwd = (label, line) => { const r = spawnSync('bash', ['-c', line], { cwd: proj, encoding: 'utf8' }); console.log(`  bash ${label}: ends in ${r.stdout.trim().split('\n').pop().replace(base, '<base>')}`); };
console.log('git', g(base, '--version').out, '| bash', spawnSync('bash', ['--version'], { encoding: 'utf8' }).stdout.split('\n')[0]);
// P1 day
console.log('bash', spawnSync('bash', ['--version'], { encoding: 'utf8' }).stdout.split('\n')[0]);
await run('C0 switch in main folder', 'git switch cs/evil');
await run('C1 cd <wt> && switch', `cd ${wt} && git switch cs/evil`);
await run('C1b cd <wt>; cd <proj>; switch', `cd ${wt}; cd ${proj}; git switch cs/evil`);
await run('C1c cd <wt>; true; cd <proj>; switch', `cd ${wt}; true; cd ${proj}; git switch cs/evil`);
await run('P1a function f {', `cd ${wt}; function f { cd ${proj}; }; f; git switch cs/evil`);
bashPwd('P1a', `cd ${wt}; function f { cd ${proj}; }; f; pwd`);
await run('P1a2 function f newline', `cd ${wt}\nfunction f {\n cd ${proj}\n}\nf\ngit switch cs/evil`);
await run('P1a3 f () { (space)', `cd ${wt}; f () { cd ${proj}; }; f; git switch cs/evil`);
await run('P1b trap DEBUG', `cd ${wt}; trap "cd ${proj}" DEBUG; git switch cs/evil`);
bashPwd('P1b', `cd ${wt}; trap "cd ${proj}" DEBUG; pwd`);
await run('P1c alias', `cd ${wt}; shopt -s expand_aliases; alias c=cd\nc ${proj}\ngit switch cs/evil`);
bashPwd('P1c', `cd ${wt}; shopt -s expand_aliases; alias c=cd\nc ${proj}\npwd`);
await run('P1d cd /; function; echo > CLAUDE.md', `cd /; function f { cd ${proj}; }; f; echo x > CLAUDE.md`);
await run('P1e cd /; trap; echo > CLAUDE.md', `cd /; trap "cd ${proj}" DEBUG; echo x > CLAUDE.md`);
await run('P1f control f() {', `cd /; f() { cd ${proj}; }; f; echo x > CLAUDE.md`);
await run('P1g agent cd /; function; echo > CLAUDE.md', `cd /; function f { cd ${proj}; }; f; echo x > CLAUDE.md`);
