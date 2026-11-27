// Fixture repos for track.mjs main(): case twins, gitignore, empty diff, DECISIONS edits. Usage: node fx.mjs <track.mjs>
import { execFileSync, spawnSync } from 'node:child_process';
import { mkdtempSync, writeFileSync, mkdirSync, rmSync, symlinkSync } from 'node:fs';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
const T = process.argv[2];
const dir = mkdtempSync(join(process.env.FXROOT || tmpdir(), 'fx-'));
const g = (...a) => execFileSync('git', a, { cwd: dir, encoding: 'utf8' });
const w = (p, s) => { mkdirSync(join(dir, p, '..'), { recursive: true }); writeFileSync(join(dir, p), s); };
const run = (ref, label) => {
  const r = spawnSync(process.execPath, [T, ref], { cwd: dir, encoding: 'utf8' });
  if (r.status !== 0) { console.log(`== ${label}: exit ${r.status} ${r.stderr.trim()}`); return; }
  const o = JSON.parse(r.stdout);
  console.log(`== ${label}: ${o.track} why=${JSON.stringify(o.why)} floors=${JSON.stringify(o.goFloors)} mc=${JSON.stringify(o.forMergeCheck)} words=${JSON.stringify(o.wordsRecord)} rev=${JSON.stringify(o.reviewers.picks)}`);
};
try {
  g('init', '-q', '-b', 'main'); g('config', 'user.email', 'a@b'); g('config', 'user.name', 't'); g('config', 'core.ignorecase', 'false');
  w('docs/A.md', 'base\n'); w('.gitignore', 'dist/\n');
  w('docs/DECISIONS.md', '# D\n\n- **Taken by Mustafa, message 11111111-aaaa** (2026-11-01). Copied by script (5 bytes, sha256 ' +
    '2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824):\n> hello\n');
  g('add', '-A'); g('commit', '-qm', 'base'); g('update-ref', 'refs/remotes/origin/main', 'HEAD');
  g('switch', '-qc', 'pr1'); w('docs/a.md', 'twin\n'); g('add', '-A'); g('commit', '-qm', 'twin');
  run('pr1', 'new docs/a.md beside existing docs/A.md');
  g('switch', '-qc', 'pr3'); w('docs/A.md', 'edit\n'); g('commit', '-qam', 'a');
  run('pr3', 'twins in tree (from pr1), PR changes docs/a.md and docs/A.md');
  g('switch', '-q', 'main'); g('switch', '-qc', 'pr5'); w('docs/DECISIONS.md', '# D\n\n- **Taken by Mustafa, message 11111111-aaaa** (2026-11-01). Copied by script (5 bytes, sha256 ' +
    '2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824):\n> HELLO\n'); g('commit', '-qam', 'q');
  run('pr5', 'existing entry: quote changed, header kept');
  g('switch', '-q', 'main'); g('switch', '-qc', 'pr6'); w('docs/DECISIONS.md', '# D\n\nsomething else\n'); g('commit', '-qam', 'q');
  run('pr6', 'existing entry removed');
  g('switch', '-q', 'main'); g('switch', '-qc', 'pr7'); w('dist/x.js', 'x\n'); g('add', '-f', 'dist/x.js'); g('commit', '-qm', 'ign');
  run('pr7', 'force-added gitignored file');
  g('switch', '-q', 'main'); g('switch', '-qc', 'pr8'); symlinkSync('A.md', join(dir, 'docs/link')); g('add', '-A'); g('commit', '-qm', 'ln');
  run('pr8', 'symlink');
  g('switch', '-q', 'main'); g('switch', '-qc', 'pr9'); w('docs/s.md', 'fetch(x) localStorage\n'); g('add', '-A'); g('commit', '-qm', 's');
  run('pr9', 'docs text naming fetch( and localStorage');
  g('switch', '-q', 'main'); g('switch', '-qc', 'pr10'); w('legacy/g.js', 'new XMLHttpRequest()\n'); g('add', '-A'); g('commit', '-qm', 'x');
  run('pr10', 'legacy adds XMLHttpRequest');
  g('switch', '-q', 'main'); g('switch', '-qc', 'pr11'); w('docs/x.md', 'see below\n> a quote of him\n[DECIDED] y\n'); g('add', '-A'); g('commit', '-qm', 'x');
  run('pr11', 'docs adds a quote and a DECIDED line');
  g('switch', '-q', 'main'); g('switch', '-qc', 'pr12'); g('rm', '-q', 'docs/A.md'); g('commit', '-qm', 'del');
  run('pr12', 'delete a file');
  g('switch', '-q', 'main'); g('switch', '-qc', 'pr13'); w('spikes/s/src/main.rs', 'fn main(){}\n'); g('add', '-A'); g('commit', '-qm', 'x');
  run('pr13', 'a spike .rs file');
  run('main', 'empty diff');
} finally { rmSync(dir, { recursive: true, force: true }); }
