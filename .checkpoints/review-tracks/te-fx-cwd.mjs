// track.mjs run from a subdirectory of the repo: are the per-path reads (diff -- path, check-ignore, ls-tree) root-relative?
import { execFileSync, spawnSync } from 'node:child_process';
import { mkdtempSync, writeFileSync, mkdirSync, rmSync } from 'node:fs';
import { join } from 'node:path';
const T = process.argv[2];
const dir = mkdtempSync(join(process.env.FXROOT, 'fx2-'));
const g = (...a) => execFileSync('git', a, { cwd: dir, encoding: 'utf8' });
const w = (p, s) => { mkdirSync(join(dir, p, '..'), { recursive: true }); writeFileSync(join(dir, p), s); };
try {
  g('init', '-q', '-b', 'main'); g('config', 'user.email', 'a@b'); g('config', 'user.name', 't'); g('config', 'core.ignorecase', 'false');
  w('docs/A.md', 'x\n'); w('tools/t.js', 'x\n'); w('.gitignore', '/dist/\n'); g('add', '-A'); g('commit', '-qm', 'b'); g('update-ref', 'refs/remotes/origin/main', 'HEAD');
  g('switch', '-qc', 'pr'); w('legacy/g.js', 'el.innerHTML = s\n'); w('docs/a.md', 'twin\n'); w('dist/x.js', 'x\n'); g('add', '-A'); g('add', '-f', 'dist/x.js'); g('commit', '-qm', 'p');
  for (const [sub, env] of [['', {}], ['tools', {}], ['', { GIT_CONFIG_COUNT: '1', GIT_CONFIG_KEY_0: 'color.ui', GIT_CONFIG_VALUE_0: 'always' }]]) {
    const r = spawnSync(process.execPath, [T, 'pr'], { cwd: join(dir, sub), encoding: 'utf8', env: { ...process.env, ...env } });
    const o = JSON.parse(r.stdout);
    console.log(`cwd=/${sub} ${JSON.stringify(env)}: ${o.track} floors=${JSON.stringify(o.goFloors)} picks=${o.reviewers.picks}`);
  }
} finally { rmSync(dir, { recursive: true, force: true }); }
