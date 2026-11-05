// skeptic SD-5 probe 2: proposed fix variants; the project root as clone source (false-ask check)
import { mkdirSync, rmSync, writeFileSync, realpathSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const [guardPath, scratch] = process.argv.slice(2);
const { decide, realIo } = await import(pathToFileURL(guardPath).href);
const root = join(scratch, 'p2'); rmSync(root, { recursive: true, force: true }); mkdirSync(root, { recursive: true });
const R = realpathSync(root), P = join(R, 'proj');
const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
mkdirSync(join(P, '.claude'), { recursive: true }); writeFileSync(join(P, '.claude', 'settings.json'), '{}\n'); writeFileSync(join(P, 'CLAUDE.md'), 'r\n');
spawnSync('git', ['init', '-q', '-b', 'main'], { cwd: P, env }); spawnSync('git', ['add', '-A'], { cwd: P, env }); spawnSync('git', ['commit', '-q', '-m', 'b'], { cwd: P, env });
process.env.CLAUDE_PROJECT_DIR = P;
const kind = (d) => (d ? (d.ask !== undefined && d.deny === undefined ? 'ask' : 'deny') : 'null');
const run = async (c) => kind(await decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command: c }, cwd: P, scratchpad_dir: join(R, 's') }, { ...realIo(), env: {} }));
const U = 'https://example.invalid/x/repo.git';
for (const c of [`git clone ${U} .claude/sub`, `git clone --revision main ${U} .claude/sub`, `git clone --revision main ${U} CLAUDE.md`, `git clone --x val ${U} .claude/sub`,
  `git clone --revision main ${U}`, `git clone ${U} ${R}/elsewhere`, `git clone ${P} ${R}/elsewhere`, `git clone . ${R}/elsewhere`, `git clone -b main ${P} ${R}/elsewhere`])
  console.log(c.replace(U, '<url>').replaceAll(R, '<R>'), '->', await run(c));
