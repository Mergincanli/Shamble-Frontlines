// Scratch probe (A3 reader-routines): today's guard decisions for a few inputs, fake io, no network.
import { spawnSync } from 'node:child_process';
const { decide } = await import(process.argv[2]);
const io = (role) => ({ env: { SHAMBLE_ROLE: role }, projectDir: '/repo', home: '/home/u',
  gitLongOptions: (s) => (spawnSync('git', [s, '--git-completion-helper'], { encoding: 'utf8' }).stdout || '').split(/\s+/).filter((o) => o.startsWith('--')),
  git: (a) => ({ code: 0, out: a.join(' ') === 'rev-parse --show-toplevel' ? '/repo\n' : '' }), gh: () => ({ code: 1, out: '' }),
  exists: () => true, realpath: (p) => p, readFile: () => '{}' });
const ev = (tool_name, tool_input) => ({ hook_event_name: 'PreToolUse', tool_name, tool_input, cwd: '/repo', scratchpad_dir: '/tmp/s' });
const sh = (c) => ev('Bash', { command: c });
const R = { owner: 'Mergincanli', repo: 'Shamble-Frontlines' };
const cases = [
  ['pr-advisor', sh('git log --output=CLAUDE.md -1')], ['pr-advisor', sh('git diff --output=CLAUDE.md')],
  ['pr-advisor', sh('git rev-list --count origin/main..HEAD')], ['night-shift', sh('git rev-list --count HEAD')],
  ['pr-advisor', ev('mcp__github__actions_get', { ...R, method: 'get_workflow_run', resource_id: '1' })],
  ['pr-advisor', ev('mcp__github__actions_list', { ...R, method: 'list_workflow_runs' })],
  ['pr-advisor', ev('mcp__github__actions_run_trigger', { ...R, method: 'rerun_workflow_run', run_id: 1 })],
  ['pr-advisor', ev('mcp__github__get_job_logs', { ...R, job_id: 1 })], ['pr-advisor', ev('mcp__github__get_check_run', { ...R, check_run_id: 1 })],
  ['pr-advisor', ev('mcp__github__get_job_logs', { owner: 'x', repo: 'y', job_id: 1 })],
  ['night-shift', ev('mcp__github__actions_get', { ...R, method: 'get_workflow_run', resource_id: '1' })],
  ['night-shift', sh('git switch cs/other')], ['night-shift', sh('git switch --detach 0123abc')], ['pr-advisor', sh('git switch --detach origin/main')],
  ['night-shift', sh('python3 -I tools/x.py')], ['night-shift', sh('ruby x.rb')],
];
for (const [role, e] of cases) { const d = await decide(e, io(role)); console.log(`${role} | ${e.tool_name === 'Bash' ? e.tool_input.command : e.tool_name + ' ' + (e.tool_input.method ?? '')} | ${d ? 'REFUSED: ' + d.deny : 'goes on'}`); }
