// Tests for checkpoint.mjs: node --test .claude/scripts/checkpoint.test.mjs
// Each test builds a scratch repository with a bare "origin" in the system temp folder and deletes it afterwards.
// CHECKPOINT_SCRIPT points the tests at another copy of the script (used to prove the tests bite).
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync, spawn } from 'node:child_process';
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const SCRIPT = process.env.CHECKPOINT_SCRIPT || join(dirname(fileURLToPath(import.meta.url)), 'checkpoint.mjs');
const sh = (cwd, ...args) => {
  const r = spawnSync('git', args, { cwd, encoding: 'utf8' });
  if (r.status !== 0) throw new Error(`git ${args.join(' ')}: ${r.stderr}`);
  return r.stdout.trim();
};
const run = (cwd, ...args) => spawnSync(process.execPath, [SCRIPT, ...args], { cwd, encoding: 'utf8' });
const runEnv = (cwd, env, ...args) => spawnSync(process.execPath, [SCRIPT, ...args], { cwd, encoding: 'utf8', env: { ...process.env, ...env } });
const note = (repo, name = 'test.md', text = 'notes\n') => {
  mkdirSync(join(repo, '.checkpoints'), { recursive: true });
  writeFileSync(join(repo, '.checkpoints', name), text);
};

function fixture(t, branch = 'cs/test') {
  const root = mkdtempSync(join(tmpdir(), 'checkpoint-test-'));
  t.after(() => rmSync(root, { recursive: true, force: true }));
  const origin = join(root, 'origin.git'), repo = join(root, 'repo');
  sh(root, 'init', '--quiet', '--bare', '-b', 'main', origin);
  sh(root, 'init', '--quiet', '-b', 'main', repo);
  sh(repo, 'config', 'user.name', 'Test');
  sh(repo, 'config', 'user.email', 'test@example.invalid');
  sh(repo, 'config', 'commit.gpgsign', 'false');
  writeFileSync(join(repo, 'README.md'), 'one\n');
  sh(repo, 'add', 'README.md');
  sh(repo, 'commit', '--quiet', '-m', 'base');
  sh(repo, 'remote', 'add', 'origin', origin);
  sh(repo, 'push', '--quiet', 'origin', 'main');
  if (branch) { sh(repo, 'switch', '--quiet', '-c', branch); sh(repo, 'push', '--quiet', '-u', 'origin', branch); }
  return { root, origin, repo };
}

test('save commits only .checkpoints/, keeps other staged work staged, and pushes the branch', (t) => {
  const { repo, origin } = fixture(t);
  note(repo);
  writeFileSync(join(repo, 'README.md'), 'two\n');
  sh(repo, 'add', 'README.md');
  writeFileSync(join(repo, 'other.txt'), 'untracked\n');
  const r = run(repo, 'save', 'test', '-m', 'step 1', '--trailer', 'Co-Authored-By: X <x@example.invalid>');
  assert.equal(r.status, 0, r.stderr + r.stdout);
  assert.deepEqual(sh(repo, 'show', '--name-only', '--format=', 'HEAD').split('\n'), ['.checkpoints/test.md']);
  assert.equal(sh(repo, 'diff', '--cached', '--name-only'), 'README.md');
  assert.match(sh(repo, 'status', '--porcelain', '--', 'other.txt'), /^\?\? other\.txt$/);
  assert.equal(sh(origin, 'rev-parse', 'refs/heads/cs/test'), sh(repo, 'rev-parse', 'HEAD'));
  const msg = sh(repo, 'log', '-1', '--format=%B');
  assert.match(msg, /^checkpoint\(test\): step 1 \[skip ci\]/);
  assert.match(msg, /Co-Authored-By: X <x@example\.invalid>/);
});

test('nothing to save makes no commit', (t) => {
  const { repo } = fixture(t);
  const before = sh(repo, 'rev-parse', 'HEAD');
  note(repo);
  assert.equal(run(repo, 'save', 'test', '-m', 'a').status, 0);
  const mid = sh(repo, 'rev-parse', 'HEAD');
  assert.notEqual(mid, before);
  const r = run(repo, 'save', 'test', '-m', 'b');
  assert.equal(r.status, 0);
  assert.match(r.stdout, /nothing to save/);
  assert.equal(sh(repo, 'rev-parse', 'HEAD'), mid);
});

function refused(t, setup) {
  const { repo } = fixture(t);
  const head = sh(repo, 'rev-parse', 'HEAD');
  setup(repo, head);
  note(repo);
  const r = run(repo, 'save', 'test', '-m', 'x');
  assert.equal(r.status, 2, r.stderr);
  assert.match(r.stderr, /refused/);
  assert.equal(sh(repo, 'rev-parse', 'HEAD'), head);
}
test('main is refused', (t) => refused(t, (repo) => sh(repo, 'switch', '--quiet', 'main')));
test('a detached head is refused', (t) => refused(t, (repo, head) => sh(repo, 'switch', '--quiet', '--detach', head)));
test('a branch outside cs/ and claude/ is refused', (t) =>
  refused(t, (repo, head) => sh(repo, 'switch', '--quiet', '-c', 'feature/x', head)));

test('a rejected push is reported and never forced', (t) => {
  const { root, origin, repo } = fixture(t);
  const other = join(root, 'other');
  sh(root, 'clone', '--quiet', '-b', 'cs/test', origin, other);
  sh(other, 'config', 'user.name', 'Other');
  sh(other, 'config', 'user.email', 'other@example.invalid');
  writeFileSync(join(other, 'theirs.txt'), 'theirs\n');
  sh(other, 'add', 'theirs.txt');
  sh(other, 'commit', '--quiet', '-m', 'theirs');
  sh(other, 'push', '--quiet', 'origin', 'cs/test');
  const theirs = sh(origin, 'rev-parse', 'refs/heads/cs/test');
  note(repo);
  const r = run(repo, 'save', 'test', '-m', 'mine');
  assert.equal(r.status, 1);
  assert.match(r.stdout, /refused|NOT pushed/);
  assert.equal(sh(origin, 'rev-parse', 'refs/heads/cs/test'), theirs);
  assert.match(sh(repo, 'log', '-1', '--format=%s'), /^checkpoint\(test\): mine/);
});

test('CI is skipped only when every pushed commit touches .checkpoints/ alone', (t) => {
  const { repo } = fixture(t);
  writeFileSync(join(repo, 'code.js'), '1;\n');
  sh(repo, 'add', 'code.js');
  sh(repo, 'commit', '--quiet', '-m', 'work, not pushed yet');
  note(repo);
  assert.equal(run(repo, 'save', 'test', '-m', 'with work').status, 0);
  assert.doesNotMatch(sh(repo, 'log', '-1', '--format=%s'), /skip ci/);
  note(repo, 'test.md', 'more\n');
  assert.equal(run(repo, 'save', 'test', '-m', 'notes only').status, 0);
  assert.match(sh(repo, 'log', '-1', '--format=%s'), /\[skip ci\]$/);
});

test('bad usage is refused', (t) => {
  const { repo } = fixture(t);
  assert.equal(run(repo, 'save', '../x', '-m', 'a').status, 2);
  assert.equal(run(repo, 'save', 'test').status, 2);
  assert.equal(run(repo, 'save', 'test', '-m', 'two\nlines').status, 2);
  assert.equal(run(repo, 'push', 'test').status, 2);
  assert.equal(run(join(repo, '.git'), 'save', 'test', '-m', 'a').status, 2);
});

test('watch saves changes and stops when the branch changes', async (t) => {
  const { repo, origin } = fixture(t);
  note(repo);
  const child = spawn(process.execPath, [SCRIPT, 'watch', 'test', '--every', '0.01', '--for', '1'], { cwd: repo });
  let out = '';
  child.stdout.on('data', (d) => { out += d; });
  const exited = new Promise((res) => child.on('exit', res));
  t.after(() => child.kill('SIGKILL'));
  const deadline = Date.now() + 15000;
  while (!/saved and pushed/.test(out) && Date.now() < deadline) await new Promise((r) => setTimeout(r, 200));
  assert.match(out, /saved and pushed/);
  assert.equal(sh(origin, 'rev-parse', 'refs/heads/cs/test'), sh(repo, 'rev-parse', 'HEAD'));
  sh(repo, 'switch', '--quiet', '-c', 'cs/elsewhere');
  const code = await Promise.race([exited, new Promise((r) => setTimeout(() => r('timeout'), 15000))]);
  assert.equal(code, 0);
  assert.match(out, /branch changed/);
});

test('watch makes a last save when it is terminated', async (t) => {
  const { repo, origin } = fixture(t);
  note(repo);
  const child = spawn(process.execPath, [SCRIPT, 'watch', 'test', '--every', '5', '--for', '10'], { cwd: repo });
  let out = '';
  child.stdout.on('data', (d) => { out += d; });
  const exited = new Promise((res) => child.on('exit', res));
  t.after(() => child.kill('SIGKILL'));
  const until = Date.now() + 15000;
  while (!/watching/.test(out) && Date.now() < until) await new Promise((r) => setTimeout(r, 100));
  assert.match(out, /watching/);
  note(repo, 'test.md', 'changed after start\n');
  child.kill('SIGTERM');
  const code = await Promise.race([exited, new Promise((r) => setTimeout(() => r('timeout'), 15000))]);
  assert.equal(code, 0);
  assert.match(out, /saved and pushed/);
  assert.equal(sh(origin, 'rev-parse', 'refs/heads/cs/test'), sh(repo, 'rev-parse', 'HEAD'));
});

test('the task checkpoint file must exist: none yet, or deleted at the end', (t) => {
  const { repo } = fixture(t);
  note(repo, 'other.md');
  let r = run(repo, 'save', 'test', '-m', 'x');
  assert.equal(r.status, 2);
  assert.match(r.stderr, /does not exist/);
  note(repo);
  assert.equal(run(repo, 'save', 'test', '-m', 'first').status, 0);
  const head = sh(repo, 'rev-parse', 'HEAD');
  sh(repo, 'rm', '--quiet', '.checkpoints/test.md');
  r = run(repo, 'save', 'test', '-m', 'final delete');
  assert.equal(r.status, 2);
  assert.equal(sh(repo, 'rev-parse', 'HEAD'), head);
});

test('notes that look like a secret are not committed, and the secret is not printed', (t) => {
  const { repo, origin } = fixture(t);
  note(repo);
  assert.equal(run(repo, 'save', 'test', '-m', 'clean').status, 0);
  const head = sh(repo, 'rev-parse', 'HEAD');
  const token = 'ghp_' + 'A1b2C3d4E5'.repeat(4);
  for (const [name, text] of [['raw.md', `token ${token}\n`], ['proxy.md', 'https://user:hunter2@proxy.example.invalid:8080\n']]) {
    note(repo, name, text);
    const r = run(repo, 'save', 'test', '-m', 'leak');
    assert.equal(r.status, 1, r.stdout + r.stderr);
    assert.match(r.stdout, new RegExp(`NOT committed: \\.checkpoints/${name.replace('.', '\\.')}`));
    assert.doesNotMatch(r.stdout + r.stderr, /A1b2C3d4E5|hunter2/);
    assert.equal(sh(repo, 'rev-parse', 'HEAD'), head);
    assert.equal(sh(origin, 'rev-parse', 'refs/heads/cs/test'), head);
    assert.equal(sh(repo, 'diff', '--cached', '--name-only', '--', '.checkpoints/'), '');
    rmSync(join(repo, '.checkpoints', name));
  }
});

test('skip markers from the caller are refused', (t) => {
  const { repo } = fixture(t);
  note(repo);
  assert.equal(run(repo, 'save', 'test', '-m', 'step [ci skip]').status, 2);
  assert.equal(run(repo, 'save', 'test', '-m', 'step', '--trailer', 'skip-checks: true').status, 2);
});

test('--every and --for are bounded', (t) => {
  const { repo } = fixture(t);
  note(repo);
  assert.equal(run(repo, 'watch', 'test', '--every', '0.001').status, 2);
  assert.equal(run(repo, 'watch', 'test', '--for', '200').status, 2);
});

test('watch never publishes code commits', async (t) => {
  const { repo, origin } = fixture(t);
  const before = sh(origin, 'rev-parse', 'refs/heads/cs/test');
  writeFileSync(join(repo, 'code.js'), '1;\n');
  sh(repo, 'add', 'code.js');
  sh(repo, 'commit', '--quiet', '-m', 'code, not gated yet');
  note(repo);
  const child = spawn(process.execPath, [SCRIPT, 'watch', 'test', '--every', '0.01', '--for', '1'], { cwd: repo });
  let out = '';
  child.stdout.on('data', (d) => { out += d; });
  t.after(() => child.kill('SIGKILL'));
  const deadline = Date.now() + 15000;
  while (!/NOT pushed/.test(out) && Date.now() < deadline) await new Promise((r) => setTimeout(r, 200));
  assert.match(out, /saved locally, NOT pushed/);
  assert.equal(sh(origin, 'rev-parse', 'refs/heads/cs/test'), before);
  assert.match(sh(repo, 'log', '-1', '--format=%s'), /^checkpoint\(test\): autosave$/);
});

test('watch stops when the checkpoint file is gone', async (t) => {
  const { repo } = fixture(t);
  note(repo);
  const child = spawn(process.execPath, [SCRIPT, 'watch', 'test', '--every', '0.01', '--for', '1'], { cwd: repo });
  let out = '';
  child.stdout.on('data', (d) => { out += d; });
  const exited = new Promise((res) => child.on('exit', res));
  t.after(() => child.kill('SIGKILL'));
  const until = Date.now() + 15000;
  while (!/watching/.test(out) && Date.now() < until) await new Promise((r) => setTimeout(r, 100));
  assert.match(out, /watching/);
  rmSync(join(repo, '.checkpoints', 'test.md'));
  const code = await Promise.race([exited, new Promise((r) => setTimeout(() => r('timeout'), 15000))]);
  assert.equal(code, 0);
  assert.match(out, /is gone/);
});

test('a trailer with a newline is refused, and nothing runs through a shell', (t) => {
  const { repo } = fixture(t);
  note(repo);
  assert.equal(run(repo, 'save', 'test', '-m', 'x', '--trailer', 'A: b\nC: d').status, 2);
  const r = run(repo, 'save', 'test', '-m', 'x $(touch pwned) `touch pwned2`; touch pwned3');
  assert.equal(r.status, 0, r.stderr + r.stdout);
  assert.match(sh(repo, 'log', '-1', '--format=%s'), /\$\(touch pwned\) `touch pwned2`; touch pwned3/);
  for (const f of ['pwned', 'pwned2', 'pwned3']) assert.equal(spawnSync('test', ['-e', join(repo, f)]).status, 1);
});

test('a note whose name git would quote is still scanned, and an unreadable note is refused', (t) => {
  const { repo, origin } = fixture(t);
  note(repo);
  assert.equal(run(repo, 'save', 'test', '-m', 'clean').status, 0);
  const head = sh(repo, 'rev-parse', 'HEAD');
  note(repo, 'ü "odd" name.md', 'ghp_' + 'Z9y8X7w6V5'.repeat(4) + '\n');
  const r = run(repo, 'save', 'test', '-m', 'odd name');
  assert.equal(r.status, 1, r.stdout + r.stderr);
  assert.match(r.stdout, /NOT committed/);
  assert.equal(sh(repo, 'rev-parse', 'HEAD'), head);
  assert.equal(sh(origin, 'rev-parse', 'refs/heads/cs/test'), head);
});

test('the commit holds exactly the scanned notes and leaves the index in step with HEAD', (t) => {
  const { repo } = fixture(t);
  note(repo, 'test.md', 'one\n');
  note(repo, 'a.md', 'alpha\n');
  assert.equal(run(repo, 'save', 'test', '-m', 'two notes').status, 0);
  assert.equal(sh(repo, 'show', 'HEAD:.checkpoints/a.md'), 'alpha');
  assert.equal(sh(repo, 'status', '--porcelain', '--', '.checkpoints/'), '');
  assert.equal(sh(repo, 'diff', '--cached', '--name-only'), '');
});

test('the value of a secret environment variable is refused, and only its name is printed', (t) => {
  const { repo } = fixture(t);
  note(repo);
  assert.equal(run(repo, 'save', 'test', '-m', 'clean').status, 0);
  const head = sh(repo, 'rev-parse', 'HEAD');
  const value = '0123456789abcdef0123456789abcdef';
  note(repo, 'env.md', `pasted: ${value}\n`);
  const r = runEnv(repo, { FAKE_TOKEN: value }, 'save', 'test', '-m', 'env paste');
  assert.equal(r.status, 1, r.stdout + r.stderr);
  assert.match(r.stdout, /value of \$FAKE_TOKEN/);
  assert.doesNotMatch(r.stdout + r.stderr, new RegExp(value));
  assert.equal(sh(repo, 'rev-parse', 'HEAD'), head);
});

test('more token shapes are refused: URL-encoded GitHub, JWT, npm, Google', (t) => {
  const { repo } = fixture(t);
  note(repo);
  assert.equal(run(repo, 'save', 'test', '-m', 'clean').status, 0);
  const head = sh(repo, 'rev-parse', 'HEAD');
  const samples = [
    'x=a%3Dghp_' + 'Q'.repeat(36),
    'eyJ' + 'a'.repeat(20) + '.eyJ' + 'b'.repeat(20) + '.' + 'c'.repeat(20),
    'npm_' + 'N'.repeat(36),
    'AIza' + 'G'.repeat(35),
  ];
  for (const [i, text] of samples.entries()) {
    note(repo, `s${i}.md`, text + '\n');
    assert.equal(run(repo, 'save', 'test', '-m', 'sample').status, 1, text.slice(0, 12));
    assert.equal(sh(repo, 'rev-parse', 'HEAD'), head);
    rmSync(join(repo, '.checkpoints', `s${i}.md`));
  }
});

test('a flagged note is left out, the clean notes are still saved', (t) => {
  const { repo, origin } = fixture(t);
  note(repo);
  assert.equal(run(repo, 'save', 'test', '-m', 'clean').status, 0);
  note(repo, 'test.md', 'step 2 done\n');
  note(repo, 'example.md', 'postgres://app:changeme@localhost/db\n');
  const r = run(repo, 'save', 'test', '-m', 'step 2');
  assert.equal(r.status, 1, r.stdout + r.stderr);
  assert.match(r.stdout, /flagged notes left out/);
  assert.equal(sh(repo, 'show', 'HEAD:.checkpoints/test.md'), 'step 2 done');
  assert.throws(() => sh(repo, 'show', 'HEAD:.checkpoints/example.md'));
  assert.equal(sh(origin, 'rev-parse', 'refs/heads/cs/test'), sh(repo, 'rev-parse', 'HEAD'));
});

test('a renamed note with a token added is refused', (t) => {
  const { repo, origin } = fixture(t);
  note(repo);
  const body = Array.from({ length: 30 }, (_, i) => `line ${i}: a long enough clean note for rename detection`).join('\n') + '\n';
  note(repo, 'a.md', body);
  assert.equal(run(repo, 'save', 'test', '-m', 'clean').status, 0);
  rmSync(join(repo, '.checkpoints', 'a.md'));
  note(repo, 'b.md', body + 'ghp_' + 'R'.repeat(36) + '\n');
  assert.equal(run(repo, 'save', 'test', '-m', 'rename').status, 1);
  assert.throws(() => sh(repo, 'show', 'HEAD:.checkpoints/b.md'));
  assert.throws(() => sh(origin, 'show', 'refs/heads/cs/test:.checkpoints/b.md'));
});

test('code added in a merge commit is never marked [skip ci]', (t) => {
  const { repo } = fixture(t);
  note(repo);
  assert.equal(run(repo, 'save', 'test', '-m', 'clean').status, 0);
  sh(repo, 'switch', '--quiet', '-c', 'side');
  note(repo, 'side.md', 'side\n');
  sh(repo, 'add', '.checkpoints/side.md');
  sh(repo, 'commit', '--quiet', '-m', 'side: notes only');
  sh(repo, 'switch', '--quiet', 'cs/test');
  sh(repo, 'merge', '--quiet', '--no-ff', '--no-commit', 'side');
  writeFileSync(join(repo, 'evil.js'), 'evil();\n');
  sh(repo, 'add', 'evil.js');
  sh(repo, 'commit', '--quiet', '-m', 'merge side');
  note(repo, 'test.md', 'after the merge\n');
  assert.equal(run(repo, 'save', 'test', '-m', 'after merge').status, 0);
  assert.doesNotMatch(sh(repo, 'log', '-1', '--format=%s'), /skip ci/);
});

test('URL-encoded JWTs and sk- keys are refused; long kebab-case words are not', (t) => {
  const { repo } = fixture(t);
  note(repo);
  assert.equal(run(repo, 'save', 'test', '-m', 'clean').status, 0);
  const head = sh(repo, 'rev-parse', 'HEAD');
  const jwt = 'eyJ' + 'a'.repeat(20) + '.eyJ' + 'b'.repeat(20) + '.' + 'c'.repeat(20);
  for (const [i, text] of [`token%3D${jwt}`, `key%3Dsk-proj-${'K'.repeat(40)}`].entries()) {
    note(repo, `u${i}.md`, text + '\n');
    assert.equal(run(repo, 'save', 'test', '-m', 'encoded').status, 1, text.slice(0, 16));
    assert.equal(sh(repo, 'rev-parse', 'HEAD'), head);
    rmSync(join(repo, '.checkpoints', `u${i}.md`));
  }
  note(repo, 'slug.md', 'branch cs/task-legacy-inventory-and-tuning-tables-for-v2-research\n');
  assert.equal(run(repo, 'save', 'test', '-m', 'kebab').status, 0);
});
