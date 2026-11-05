// step 9 probe: what `git diff --name-only` prints for a renamed and a non-ASCII kit file, with and without
// -z / --no-renames, and under diff.relative. Scratch repo under .gate/kit-v3-3/. Prints names only.
import { spawnSync } from 'node:child_process';
import { mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
const root = join(process.argv[2], 'step9-probe');
rmSync(root, { recursive: true, force: true });
mkdirSync(join(root, '.claude', 'agents'), { recursive: true });
mkdirSync(join(root, 'docs'));
const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
const g = (cwd, ...a) => spawnSync('git', a, { cwd, env, encoding: 'utf8' });
writeFileSync(join(root, '.claude', 'agents', 'x.md'), 'agent\n'.repeat(20));
writeFileSync(join(root, 'docs', 'a.md'), 'a\n');
g(root, 'init', '-q', '-b', 'main'); g(root, 'add', '-A'); g(root, 'commit', '-q', '-m', 'base');
g(root, 'switch', '-q', '-c', 'ren'); g(root, 'mv', '.claude/agents/x.md', 'docs/x.md'); g(root, 'commit', '-q', '-m', 'ren');
g(root, 'switch', '-q', 'main'); g(root, 'switch', '-q', '-c', 'uni');
writeFileSync(join(root, '.claude', 'agents', 'é.md'), 'new\n'); g(root, 'add', '-A'); g(root, 'commit', '-q', '-m', 'uni');
g(root, 'switch', '-q', 'main');
const show = (label, cwd, args) => console.log(label, JSON.stringify(g(cwd, 'diff', '--name-only', ...args).stdout));
show('rename, plain:', root, ['main', 'ren', '--']);
show('rename, --no-renames:', root, ['--no-renames', 'main', 'ren', '--']);
show('non-ASCII, plain:', root, ['main', 'uni', '--']);
show('non-ASCII, -z:', root, ['-z', 'main', 'uni', '--']);
g(root, 'config', 'diff.relative', 'true');
show('diff.relative, from docs/, plain:', join(root, 'docs'), ['main', 'uni', '--']);
show('diff.relative, from docs/, --no-relative:', join(root, 'docs'), ['--no-relative', '-z', 'main', 'uni', '--']);
console.log('git', g(root, '--version').stdout.trim());
rmSync(root, { recursive: true, force: true });
