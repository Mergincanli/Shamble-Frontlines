// skeptic P3-5 repro (kit-v3-3 pass 3): night commit with a positional pathspec. usage: node skp3-5.mjs <guard> <scratch>
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync, realpathSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const [gp, scratch] = process.argv.slice(2);
const { decide, realIo } = await import(pathToFileURL(gp).href);
const root = join(scratch, 'skp3-5b'); rmSync(root, { recursive: true, force: true }); mkdirSync(root, { recursive: true });
const R = realpathSync(root), O = join(R, 'o.git'), P = join(R, 'p');
const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
const g = (...a) => spawnSync('git', a, { cwd: P, env, encoding: 'utf8' });
spawnSync('git', ['init', '-q', '--bare', '-b', 'main', O], { env });
mkdirSync(join(P, 'docs'), { recursive: true });
writeFileSync(join(P, 'CLAUDE.md'), 'r\n'); writeFileSync(join(P, 'docs', 'a.md'), 'a\n');
g('init', '-q', '-b', 'main'); g('add', '-A'); g('commit', '-q', '-m', 'b'); g('remote', 'add', 'origin', O); g('push', '-q', 'origin', 'main'); g('fetch', '-q', 'origin');
g('switch', '-q', '-c', 'cs/w', 'origin/main');
process.env.CLAUDE_PROJECT_DIR = P;
const dec = (c) => decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command: c }, cwd: P, scratchpad_dir: join(R, 's') }, { ...realIo(), env: { SHAMBLE_ROLE: 'night-shift' } });
const k = (d) => (d ? (d.deny !== undefined ? 'deny' : 'ask') : 'null');
for (const [label, line] of [['dash-dash dot', 'git commit -q -m m -- .'], ['bare dot', 'git commit -q -m m .'], ['dash-dash glob', "git commit -q -m m -- 'CLAUDE.*'"], ['dash-dash top magic', "git commit -q -m m -- ':(top)CLAUDE.md'"], ['dash-dash star md', "git commit -q -m m -- '*.md'"]]) {
  writeFileSync(join(P, 'CLAUDE.md'), 'x\n'); writeFileSync(join(P, 'docs', 'a.md'), 'y\n');
  const d = await dec(line); console.log(label, '->', k(d));
  if (!d) { spawnSync('bash', ['--norc', '-c', line], { cwd: P, env }); console.log('  commit holds:', g('show', '--name-only', '--format=', 'HEAD').stdout.trim().split('\n').join(','));
    const p = await dec('git push origin cs/w'); console.log('  then night push ->', k(p), p ? String(p.deny).slice(0, 90) : ''); g('reset', '-q', '--hard', 'origin/main'); }
  g('checkout', '-q', '--', '.');
}
rmSync(root, { recursive: true, force: true });
