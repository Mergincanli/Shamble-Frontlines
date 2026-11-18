// Probe: does the hold release a changed manifest in a root folder named ":" when main already holds that path
// with the root file's content? (the line that reads the push resolves ":/Cargo.lock" as git's "top" pattern)
import { spawnSync } from 'node:child_process';
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
const SCRIPT = process.argv[2];
const sh = (cwd, ...a) => { const r = spawnSync('git', a, { cwd, encoding: 'utf8' }); if (r.status !== 0) throw new Error(`git ${a.join(' ')}: ${r.stderr}`); return r.stdout.trim(); };
const root = mkdtempSync(join(tmpdir(), 'probe-'));
try {
  const origin = join(root, 'origin.git'), repo = join(root, 'repo');
  sh(root, 'init', '--quiet', '--bare', '-b', 'main', origin);
  sh(root, 'init', '--quiet', '-b', 'main', repo);
  sh(repo, 'config', 'user.name', 'Test'); sh(repo, 'config', 'user.email', 'test@example.invalid'); sh(repo, 'config', 'commit.gpgsign', 'false');
  mkdirSync(join(repo, ':'));
  writeFileSync(join(repo, 'Cargo.lock'), 'B\n');
  writeFileSync(join(repo, ':', 'Cargo.lock'), 'B\n');
  sh(repo, 'add', '--', 'Cargo.lock', ':(literal):/Cargo.lock');
  sh(repo, 'commit', '--quiet', '-m', 'main holds both, same content');
  sh(repo, 'remote', 'add', 'origin', origin);
  sh(repo, 'push', '--quiet', 'origin', 'main');
  sh(repo, 'switch', '--quiet', '-c', 'cs/test'); sh(repo, 'push', '--quiet', '-u', 'origin', 'cs/test');
  const before = sh(origin, 'rev-parse', 'refs/heads/cs/test');
  writeFileSync(join(repo, ':', 'Cargo.lock'), 'C: not on main\n');
  sh(repo, 'add', '--', ':(literal):/Cargo.lock');
  sh(repo, 'commit', '--quiet', '-m', 'the file in the folder named : changes');
  mkdirSync(join(repo, '.checkpoints'), { recursive: true });
  writeFileSync(join(repo, '.checkpoints', 'test.md'), 'notes\n');
  const r = spawnSync(process.execPath, [SCRIPT, 'save', 'test', '-m', 'probe'], { cwd: repo, encoding: 'utf8' });
  const pushed = sh(origin, 'rev-parse', 'refs/heads/cs/test') !== before;
  console.log(`${SCRIPT.split('/').slice(-2).join('/')}: exit ${r.status}; ${pushed ? 'RELEASED (pushed)' : 'held (not pushed)'}; ${r.stdout.trim().slice(25, 140)}`);
} finally { rmSync(root, { recursive: true, force: true }); }
