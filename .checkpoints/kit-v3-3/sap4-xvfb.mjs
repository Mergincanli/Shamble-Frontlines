// pass 4 probe F5: xvfb-run's -e/--error-file and -f/--auth-file write files; unwrapRole skips them with their value.
// usage: node sap4-xvfb.mjs <guard.mjs> <scratch dir>
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync, readFileSync, realpathSync, statSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const [guardPath, scratch] = process.argv.slice(2);
const { decide, realIo } = await import(pathToFileURL(guardPath).href);
const root = join(scratch, 'sap4-xvfb');
rmSync(root, { recursive: true, force: true }); mkdirSync(root, { recursive: true });
const R = realpathSync(root), P = join(R, 'proj');
const env = { ...process.env, GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1', GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t' };
mkdirSync(join(P, '.claude'), { recursive: true });
const reset = () => { writeFileSync(join(P, 'CLAUDE.md'), 'rules\n'); writeFileSync(join(P, '.claude', 'settings.json'), '{"a":1}\n'); };
reset();
spawnSync('git', ['init', '-q', '-b', 'main'], { cwd: P, env }); spawnSync('git', ['add', '-A'], { cwd: P, env }); spawnSync('git', ['commit', '-q', '-m', 'b'], { cwd: P, env });
process.env.CLAUDE_PROJECT_DIR = P;
const kind = (d) => (d ? (d.ask !== undefined && d.deny === undefined ? 'ask' : 'deny') : 'null');
for (const role of ['night-shift', 'pr-advisor']) for (const line of ['xvfb-run -a -f .claude/settings.json git status', 'xvfb-run -a -e CLAUDE.md git status']) {
  reset();
  const d = await decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command: line }, cwd: P, scratchpad_dir: join(R, 's') }, { ...realIo(), env: { SHAMBLE_ROLE: role } });
  let eff = '';
  if (!d) { const r = spawnSync('bash', ['--norc', '--noprofile', '-c', line], { cwd: P, env, encoding: 'utf8', timeout: 20000 }); eff = `bash exit ${r.status}; CLAUDE.md ${statSync(join(P, 'CLAUDE.md')).size} bytes (was 6), first ${JSON.stringify(readFileSync(join(P, 'CLAUDE.md'), 'utf8').slice(0, 12))}; settings.json ${statSync(join(P, '.claude/settings.json')).size} bytes (was 8), JSON ${(() => { try { JSON.parse(readFileSync(join(P, '.claude/settings.json'), 'utf8')); return 'ok'; } catch { return 'broken'; } })()}`; }
  console.log(role, JSON.stringify(line), '->', kind(d), d ? String(d.deny ?? d.ask).slice(0, 90) : eff);
}
