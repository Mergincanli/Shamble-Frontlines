// security-auditor delta probe 3: try to break the fix round's day rules on a real repo (decision kinds only).
// usage: node sad-probe3.mjs <guard.mjs> <scratch dir>
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync, realpathSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const [guardPath, scratch] = process.argv.slice(2);
const { decide, realIo } = await import(pathToFileURL(guardPath).href);
const R0 = join(scratch, 'sad-probe3'); rmSync(R0, { recursive: true, force: true }); mkdirSync(R0, { recursive: true });
const R = realpathSync(R0), P = join(R, 'proj');
mkdirSync(join(P, '.claude'), { recursive: true }); mkdirSync(join(P, 'docs', 'sub', 'x'), { recursive: true }); mkdirSync(join(P, 'tools'));
writeFileSync(join(P, 'CLAUDE.md'), 'r\n'); writeFileSync(join(P, '.claude', 'settings.json'), '{}\n'); writeFileSync(join(P, 'docs', 'sub', 'x', 'CLAUDE.md'), 'n\n'); writeFileSync(join(P, 'docs', 'a.md'), 'a\n');
const env = { ...process.env, GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1', GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t' };
for (const a of [['init', '-q', '-b', 'main'], ['add', '-A'], ['commit', '-q', '-m', 'b']]) spawnSync('git', a, { cwd: P, env });
process.env.CLAUDE_PROJECT_DIR = P;
const kind = (d) => (d ? (d.ask !== undefined && d.deny === undefined ? 'ask' : 'deny') : 'null');
const run = async (command, cwd = P) => kind(await decide({ hook_event_name: 'PreToolUse', tool_name: 'Bash', tool_input: { command }, cwd, scratchpad_dir: join(R, 's') }, { ...realIo(), env: {} }));
const rows = [
  ['SA-1', '(cd .claude && sed -i s/a/b/ settings.json)'],
  ['SA-1', 'cd .claude && sed -i s/a/b/ settings.json && git status | cat'],
  ['R-b', 'cd .claude && echo x > settings.json'],
  ['R-b', `cd ${P}/.claude && echo x > settings.json`],
  ['R-b control', `cd ${P}/docs && echo x > a.md`],
  ['R-e', `source /dev/null; echo x > CLAUDE.md`],
  ['SA-5', 'sed --in-pla=.b s/a/b/ CLAUDE.md'],
  ['SA-5', 'sed -s -i s/a/b/ CLAUDE.md'],
  ['SA-5', "sed -E -i'' -es/a/b/ CLAUDE.md"],
  ['SA-5', 'sed --expr s/a/b/ -i CLAUDE.md'],
  ['SA-8', 'export GIT_DIR=/x; git status'],
  ['SA-8', 'env GIT_WORK_TREE=/x git status'],
  ['SA-8', 'declare -x GIT_INDEX_FILE=/x; git add a'],
  ['F-b', 'git worktree add .claude/wt'],
  ['F-b', 'git worktree add --lock --reason r -b cs/q .claude/wt2'],
  ['F-b', `git clone ${P} tools/x`],
  ['F-b', `git clone ${P} .claude`],
  ['F-b control', `git clone ${P} ${R}/elsewhere`],
  ['SA-6', 'rm -rf docs'],
  ['SA-6', `rm -rf ${R}`],
  ['SA-6', 'mv docs /tmp/'],
  ['SA-6 control', 'rm -rf docs/a.md'],
  ['C7 glob', "sed -i 's/a/b/' *.md"],
  ['C7 glob', 'rm -f CLAUDE.m?'],
  ['C7 glob', 'cp docs/a.md .claud?/settings.json'],
  ['C7 brace', 'tee CLAUDE.{md,x} < docs/a.md'],
  ['C7 quoted control', "sed -i 's/a/b/' 'CLAUDE.md'"],
];
for (const [k, c] of rows) console.log(k, '|', c.replaceAll(R, '<R>'), '->', await run(c));
rmSync(R0, { recursive: true, force: true });
