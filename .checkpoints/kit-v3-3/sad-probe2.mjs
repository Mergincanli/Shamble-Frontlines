// security-auditor delta probe 2: routine writes into .git (could forge refs/remotes/origin/main). Fake-free, real io.
// usage: node sad-probe2.mjs <guard.mjs> <scratch dir>
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync, realpathSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const [guardPath, scratch] = process.argv.slice(2);
const { decide, realIo } = await import(pathToFileURL(guardPath).href);
const R = join(scratch, 'sad-probe2'); rmSync(R, { recursive: true, force: true }); mkdirSync(R, { recursive: true });
const P = join(realpathSync(R), 'proj'); mkdirSync(P);
const env = { ...process.env, GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1', GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t' };
spawnSync('git', ['init', '-q', '-b', 'main'], { cwd: P, env }); writeFileSync(join(P, 'a'), 'a\n');
spawnSync('git', ['add', '-A'], { cwd: P, env }); spawnSync('git', ['commit', '-q', '-m', 'b'], { cwd: P, env });
process.env.CLAUDE_PROJECT_DIR = P;
const kind = (d) => (d ? (d.ask !== undefined && d.deny === undefined ? 'ask' : 'deny') : 'null');
const io = (role) => ({ ...realIo(), env: role ? { SHAMBLE_ROLE: role } : {} });
const ev = (tool, ti) => ({ hook_event_name: 'PreToolUse', tool_name: tool, tool_input: ti, cwd: P, scratchpad_dir: join(R, 's') });
for (const role of ['night-shift', 'pr-advisor']) {
  console.log(role, 'Bash echo > .git/refs/remotes/origin/main:', kind(await decide(ev('Bash', { command: 'echo 0 > .git/refs/remotes/origin/main' }), io(role))));
  console.log(role, 'Bash cp a .git/packed-refs:', kind(await decide(ev('Bash', { command: 'cp a .git/packed-refs' }), io(role))));
  console.log(role, 'Write .git/refs/remotes/origin/main:', kind(await decide(ev('Write', { file_path: join(P, '.git/refs/remotes/origin/main'), content: '0' }), io(role))));
  console.log(role, 'Write .git/config:', kind(await decide(ev('Write', { file_path: join(P, '.git/config'), content: '0' }), io(role))));
}
rmSync(R, { recursive: true, force: true });
