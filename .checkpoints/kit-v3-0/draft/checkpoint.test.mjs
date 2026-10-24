// Tests for checkpoint.mjs: node --test .claude/scripts/
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
const note = (repo, name = 't.md', text = 'notes\n') => {
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
  assert.deepEqual(sh(repo, 'show', '--name-only', '--format=', 'HEAD').split('\n'), ['.checkpoints/t.md']);
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

test('main, a detached head and a branch outside cs/ and claude/ are refused', (t) => {
  const { repo } = fixture(t);
  const head = sh(repo, 'rev-parse', 'HEAD');
  for (const setup of [
    () => sh(repo, 'switch', '--quiet', 'main'),
    () => sh(repo, 'switch', '--quiet', '--detach', head),
    () => sh(repo, 'switch', '--quiet', '-c', 'feature/x', head),
    () => sh(repo, 'switch', '--quiet', '-c', 'cs/../main', head),
  ]) {
    try { setup(); } catch { continue; } // git itself refuses some names: nothing to test then
    note(repo, 'refuse.md', String(Math.random()));
    const r = run(repo, 'save', 'test', '-m', 'x');
    assert.equal(r.status, 2, r.stderr);
    assert.match(r.stderr, /refused/);
    assert.equal(sh(repo, 'rev-parse', 'HEAD'), head);
  }
});

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
  note(repo, 't.md', 'more\n');
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
  const child = spawn(process.execPath, [SCRIPT, 'watch', 'test', '--every', '0.01', '--for', '1'], { cwd: repo });
  let out = '';
  child.stdout.on('data', (d) => { out += d; });
  const exited = new Promise((res) => child.on('exit', res));
  t.after(() => child.kill('SIGKILL'));
  note(repo);
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
  const child = spawn(process.execPath, [SCRIPT, 'watch', 'test', '--every', '5', '--for', '10'], { cwd: repo });
  let out = '';
  child.stdout.on('data', (d) => { out += d; });
  const exited = new Promise((res) => child.on('exit', res));
  t.after(() => child.kill('SIGKILL'));
  while (!/watching/.test(out)) await new Promise((r) => setTimeout(r, 100));
  note(repo);
  child.kill('SIGTERM');
  const code = await Promise.race([exited, new Promise((r) => setTimeout(() => r('timeout'), 15000))]);
  assert.equal(code, 0);
  assert.match(out, /saved and pushed/);
  assert.equal(sh(origin, 'rev-parse', 'refs/heads/cs/test'), sh(repo, 'rev-parse', 'HEAD'));
});
