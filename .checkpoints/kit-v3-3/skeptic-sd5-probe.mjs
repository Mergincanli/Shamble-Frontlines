// skeptic SD-5 probe: guard decision on git clone forms (no env values printed). usage: node skeptic-sd5-probe.mjs <guard> <scratch>
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync, realpathSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const [guardPath, scratch] = process.argv.slice(2);
const { decide, realIo } = await import(pathToFileURL(guardPath).href);
const root = join(scratch, 'p'); rmSync(root, { recursive: true, force: true }); mkdirSync(root, { recursive: true });
const R = realpathSync(root), P = join(R, 'proj');
const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
const g = (...a) => spawnSync('git', a, { cwd: P, env, encoding: 'utf8' });
mkdirSync(join(P, '.claude'), { recursive: true }); writeFileSync(join(P, '.claude', 'settings.json'), '{}\n'); writeFileSync(join(P, 'CLAUDE.md'), 'r\n');
g('init', '-q', '-b', 'main'); g('add', '-A'); g('commit', '-q', '-m', 'b');
process.env.CLAUDE_PROJECT_DIR = P;
const kind = (d) => (d ? (d.ask !== undefined && d.deny === undefined ? 'ask' : 'deny') : 'null');
const run = async (command, extra = {}) => kind(await decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command }, cwd: P, scratchpad_dir: join(R, 's'), ...extra }, { ...realIo(), env: {} }));
const U = 'https://example.invalid/x/repo.git';
for (const c of [`git clone ${U} .claude/sub`, `git clone --revision main ${U} .claude/sub`, `git clone --revision=main ${U} .claude/sub`,
  `git clone --revision main ${U} .claude`, `git clone --revision main ${U} CLAUDE.md`, `git clone --x val ${U} .claude/sub`, `git clone --revision main ${U} elsewhere`]) {
  console.log(c.replace(U, '<url>'), '-> main', await run(c), '| agent', await run(c, { agent_id: 'a1', agent_type: 'general' }));
}
