// verifier E4: real git --output / --outp in a scratch repo, and the head guard's decision on each. Usage: node vr-e4.mjs <worktree>
import { spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const WT = process.argv[2];
const R = join(WT, '.gate', 'kit-v3-3', 'vr-e4');
rmSync(R, { recursive: true, force: true }); mkdirSync(R, { recursive: true });
const g = (...a) => spawnSync('git', a, { cwd: R, encoding: 'utf8' });
g('init', '-q'); g('config', 'user.email', 'v@x'); g('config', 'user.name', 'v');
writeFileSync(join(R, 'a.txt'), 'a\n'); g('add', 'a.txt'); g('commit', '-qm', 'one');
writeFileSync(join(R, 'a.txt'), 'b\n'); g('commit', '-qam', 'two');
const { decide, realIo } = await import(pathToFileURL(join(WT, '.claude/hooks/routine-guard.mjs')).href);
console.log(spawnSync('git', ['--version'], { encoding: 'utf8' }).stdout.trim());
const forms = [['rev-list', 'HEAD'], ['log', '-1'], ['show', 'HEAD'], ['diff', 'HEAD~1']];
for (const [sub, extra] of forms) for (const opt of ['--output', '--outp']) {
  const f = `${sub}-${opt.slice(2)}.out`;
  const r = g(sub, `${opt}=${f}`, extra);
  const io = realIo(); io.projectDir = R;
  const out = {};
  for (const role of ['', 'pr-advisor', 'night-shift']) {
    io.env = { ...process.env, SHAMBLE_ROLE: role };
    const d = await decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command: `git ${sub} ${opt}=${f} ${extra}` }, cwd: R }, io);
    out[role || 'day'] = d ? (d.deny !== undefined ? 'deny' : d.ask !== undefined ? 'ask' : 'other') : 'null';
  }
  console.log(`git ${sub} ${opt}=: exit ${r.status}, file written ${existsSync(join(R, f))}, guard ${JSON.stringify(out)}`);
}
