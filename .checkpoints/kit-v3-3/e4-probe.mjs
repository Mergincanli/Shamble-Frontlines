// E4 probe (kit-v3-3 step 3): which --output forms make git write a file, in a throwaway repo under the OS tmpdir.
import { spawnSync } from 'node:child_process';
import { existsSync, mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
const dir = mkdtempSync(join(tmpdir(), 'e4-'));
const env = { ...process.env, GIT_AUTHOR_NAME: 't', GIT_AUTHOR_EMAIL: 't@t', GIT_COMMITTER_NAME: 't', GIT_COMMITTER_EMAIL: 't@t', GIT_CONFIG_GLOBAL: '/dev/null', GIT_CONFIG_NOSYSTEM: '1' };
const g = (...a) => spawnSync('git', a, { cwd: dir, env, encoding: 'utf8' });
g('init', '-q', '-b', 'main'); g('commit', '-q', '--allow-empty', '-m', 'one');
const forms = [['rev-list', '--output=o1', 'HEAD'], ['rev-list', '--outp=o2', 'HEAD'], ['rev-list', '--outpu=o2b', 'HEAD'],
  ['log', '--output=o3', '-1'], ['log', '--outp=o4', '-1'], ['show', '--output=o5'], ['show', '--outp=o6'],
  ['diff', '--output=o7', 'HEAD', 'HEAD'], ['diff', '--outp=o8', 'HEAD', 'HEAD']];
for (const a of forms) {
  const r = g(...a); const f = a.find((x) => x.includes('=')).split('=')[1];
  console.log(`git ${a.join(' ')} -> exit ${r.status}, file ${existsSync(join(dir, f)) ? 'written' : 'not written'}${r.status ? ', ' + r.stderr.split('\n')[0].slice(0, 80) : ''}`);
}
rmSync(dir, { recursive: true, force: true });
