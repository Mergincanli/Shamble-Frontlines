// skeptic V-3 probe (A3, ed3ea8c guard): a tag refs/tags/refs/heads/cs/evil pushed to origin at a clean commit; a fresh
// clone (no local cs/evil) fetches it; does the night guard allow `git switch cs/evil`, and where does real git land?
// Scratch repos under <a3>/.gate/kit-v3-3/skv3. Usage: node skeptic-V3-probe.mjs <a3 worktree>
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
const A3 = process.argv[2];
const base = join(A3, '.gate/kit-v3-3/skv3');
rmSync(base, { recursive: true, force: true }); mkdirSync(base, { recursive: true });
Object.assign(process.env, { GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t',
  GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' });
delete process.env.SHAMBLE_ROLE;
const g = (cwd, ...a) => { const r = spawnSync('git', a, { cwd, encoding: 'utf8' }); return { code: r.status, out: (r.stdout || '').trim(), err: (r.stderr || '').trim() }; };
const origin = join(base, 'origin.git'), seed = join(base, 'seed'), proj = join(base, 'proj');
g(base, 'init', '-q', '--bare', '-b', 'main', origin);
mkdirSync(join(seed, '.claude'), { recursive: true }); mkdirSync(join(seed, 'docs'));
g(base, 'init', '-q', '-b', 'main', seed);
writeFileSync(join(seed, 'CLAUDE.md'), 'rules\n'); writeFileSync(join(seed, '.claude/settings.json'), '{}\n'); writeFileSync(join(seed, 'docs/a.md'), 'a\n');
g(seed, 'add', '-A'); g(seed, 'commit', '-q', '-m', 'base'); g(seed, 'remote', 'add', 'origin', origin); g(seed, 'push', '-q', 'origin', 'main');
const mk = (br, file, text) => { g(seed, 'switch', '-q', '-c', br, 'main'); writeFileSync(join(seed, file), text); g(seed, 'commit', '-qam', br); g(seed, 'push', '-q', 'origin', br); g(seed, 'switch', '-q', 'main'); return g(seed, 'rev-parse', br).out; };
const ok = mk('cs/ok', 'docs/a.md', 'ok\n');
const evil = mk('cs/evil', '.claude/settings.json', '{"evil":1}\n');
g(base, 'clone', '-q', origin, proj); // proj: only main local
process.env.CLAUDE_PROJECT_DIR = proj;
const { decide, realIo } = await import(join(A3, '.claude/hooks/routine-guard.mjs'));
const io = realIo();
const run = async (label, cmd) => {
  io.env = { ...process.env, SHAMBLE_ROLE: 'night-shift' };
  const d = await decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command: cmd }, cwd: proj, session_id: 's' }, io);
  console.log(`${label}: ${d === null ? 'allow' : d.ask ? 'ASK - ' + String(d.ask).slice(0, 100) : d.deny ? 'DENY - ' + String(d.deny).slice(0, 100) : JSON.stringify(d).slice(0, 80)}`);
};
console.log('git', g(base, '--version').out);
await run('S0 control: switch cs/evil, no tag', 'git switch cs/evil');
await run('S1 control: switch cs/ok, no tag', 'git switch cs/ok');
// attacker with push rights to origin: push the tag
console.log('push tag to origin:', g(seed, 'push', '-q', 'origin', `${ok}:refs/tags/refs/heads/cs/evil`).code);
await run('F1 guard on plain fetch', 'git fetch origin');
const f1 = g(proj, 'fetch', '-q', 'origin');
console.log(`  real plain fetch exit ${f1.code}; tag present after plain fetch: ${g(proj, 'show-ref', '--verify', '-q', 'refs/tags/refs/heads/cs/evil').code === 0}`);
await run('F2 guard on fetch --tags', 'git fetch origin --tags');
g(proj, 'fetch', '-q', 'origin', '--tags');
console.log(`  tag present: ${g(proj, 'show-ref', '--verify', '-q', 'refs/tags/refs/heads/cs/evil').code === 0}; local branch cs/evil: ${g(proj, 'show-ref', '--verify', '-q', 'refs/heads/cs/evil').code === 0}`);
console.log(`  rev-parse refs/heads/cs/evil^{commit} = ${g(proj, 'rev-parse', '--verify', 'refs/heads/cs/evil^{commit}').out === ok ? 'ok (the tag)' : 'other'}`);
await run('S2 guard on switch cs/evil, tag present', 'git switch cs/evil');
const r = g(proj, 'switch', '-q', 'cs/evil');
console.log(`  real switch exit ${r.code}; HEAD = ${g(proj, 'rev-parse', 'HEAD').out === evil ? 'evil commit' : 'other'}; changes vs origin/main: ${g(proj, 'diff', '--name-only', 'origin/main', 'HEAD').out || '(none)'}`);
// the proposed fix's exact lookup
g(proj, 'switch', '-q', 'main'); g(proj, 'branch', '-q', '-D', 'cs/evil');
const sr = g(proj, 'show-ref', '--verify', '--hash', 'refs/heads/cs/evil');
console.log(`fix check: show-ref --verify refs/heads/cs/evil exit ${sr.code} (no branch); refs/remotes/origin/cs/evil = ${g(proj, 'show-ref', '--verify', '--hash', 'refs/remotes/origin/cs/evil').out === evil ? 'evil' : 'other'}`);
