// Tests for kept-logs.mjs: node --test .claude/scripts/kept-logs.test.mjs
// Each test builds a scratch repository with a bare "origin" in the system temp folder (a run-set folder of small files
// under .gate/, its results committed and pushed) and deletes it afterwards. KEPT_LOGS_SCRIPT points the tests at
// another copy of the script (used to prove the tests bite). KEPT_LOGS_DRY_RUN=<a real run-set folder's path> adds the
// dry run: a copy of that folder (only read) is listed, packed and removed in a scratch repository, and GNU tar's
// unpacking of the archive is compared file by file with the real folder.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { cpSync, createReadStream, existsSync, lstatSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, renameSync,
  rmSync, symlinkSync, utimesSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { basename, dirname, join } from 'node:path';
import { Readable } from 'node:stream';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { gzipSync } from 'node:zlib';

const HERE = dirname(fileURLToPath(import.meta.url));
const SCRIPT = process.env.KEPT_LOGS_SCRIPT || join(HERE, 'kept-logs.mjs');
const { header, readBack } = await import(pathToFileURL(SCRIPT).href);
const RUNS = '.gate/netcode-proof-test/runs';
const SET = 'u4r-P3-0123456789ab';
const RES = 'spikes/netcode-proof/results/u4r/0123456789ab/P3';
const ARCHIVE = `.gate/netcode-proof-test/kept-logs/${SET}.tar.gz`;
const FILES = { 'marks.json': '{}\n', 'l1-p3-s1/lab.jsonl': 'x'.repeat(1500), 'l1-p3-s1/room.jsonl': 'y'.repeat(512),
  'l1-p3-s2/empty.jsonl': '' };
const TAR = /GNU tar/.test(spawnSync('tar', ['--version'], { encoding: 'utf8' }).stdout ?? '');
const DRY = process.env.KEPT_LOGS_DRY_RUN;
const sha = (data) => createHash('sha256').update(data).digest('hex');
const fileSha = async (path) => {
  const hash = createHash('sha256');
  for await (const chunk of createReadStream(path)) hash.update(chunk);
  return hash.digest('hex');
};
const sh = (cwd, ...args) => {
  const r = spawnSync('git', args, { cwd, encoding: 'utf8' });
  if (r.status !== 0) throw new Error(`git ${args.join(' ')}: ${r.stderr}`);
  return r.stdout.trim();
};
const run = (cwd, args, role = '') => {
  const env = { ...process.env };
  delete env.SHAMBLE_ROLE;
  if (role) env.SHAMBLE_ROLE = role;
  return spawnSync(process.execPath, [SCRIPT, ...args], { cwd, encoding: 'utf8', env });
};
const list = (repo, set = SET, res = RES) => run(repo, ['list', set, '--results', res]);
const pack = (repo, set = SET, res = RES) => run(repo, ['pack', set, '--results', res]);
const refused = (r, re) => { assert.equal(r.status, 2, r.stderr + r.stdout); assert.match(r.stderr, re); };
const want = () => Object.keys(FILES).sort().map((p) => `${sha(FILES[p])} ${Buffer.byteLength(FILES[p])} ${p}`);
const plant = (set, files) => {
  for (const [path, text] of Object.entries(files)) {
    mkdirSync(dirname(join(set, path)), { recursive: true });
    writeFileSync(join(set, path), text);
  }
};
const commit = (repo, path, message, push = true) => {
  sh(repo, 'add', path);
  sh(repo, 'commit', '--quiet', '-m', message);
  if (push) sh(repo, 'push', '--quiet', 'origin', 'claude/test');
};
// Sets a folder and everything in it to an old time (deepest first), as the one old build cache's files are.
const age = (dir, when = new Date('2026-11-08T22:40:00Z')) => {
  for (const name of readdirSync(dir)) {
    const path = join(dir, name);
    if (lstatSync(path).isDirectory()) age(path, when);
    else utimesSync(path, when, when);
  }
  utimesSync(dir, when, when);
};

function fixture(t, files = FILES) {
  const root = mkdtempSync(join(tmpdir(), 'kept-logs-test-'));
  t.after(() => rmSync(root, { recursive: true, force: true }));
  const origin = join(root, 'origin.git'), repo = join(root, 'repo');
  sh(root, 'init', '--quiet', '--bare', '-b', 'main', origin);
  sh(root, 'init', '--quiet', '-b', 'main', repo);
  for (const [k, v] of [['user.name', 'Test'], ['user.email', 'test@example.invalid'], ['commit.gpgsign', 'false']]) {
    sh(repo, 'config', k, v);
  }
  writeFileSync(join(repo, '.gitignore'), '.gate/\n');
  mkdirSync(join(repo, RES), { recursive: true });
  writeFileSync(join(repo, RES, 'verdict.json'), '{"verdict":"PASS"}\n');
  sh(repo, 'add', '.gitignore', RES);
  sh(repo, 'commit', '--quiet', '-m', 'results');
  sh(repo, 'remote', 'add', 'origin', origin);
  sh(repo, 'switch', '--quiet', '-c', 'claude/test');
  sh(repo, 'push', '--quiet', '-u', 'origin', 'claude/test');
  const set = join(repo, RUNS, SET);
  plant(set, files);
  return { root, repo, set };
}

test('list, then pack: the list is right, every file is read back, the folder goes, free space is printed',
  async (t) => {
  const { repo, set } = fixture(t);
  const l = list(repo);
  assert.equal(l.status, 0, l.stderr);
  const lines = readFileSync(join(repo, RES, 'raw-logs.sha256'), 'utf8').trimEnd().split('\n');
  assert.equal(lines[0], `# ${RUNS}/${SET}/: 4 files, 2015 bytes (sha256, bytes, path; .claude/scripts/kept-logs.mjs)`);
  assert.deepEqual(lines.slice(1), want());
  commit(repo, `${RES}/raw-logs.sha256`, 'raw-log list (ruling: a test)');
  refused(list(repo), /never overwritten/);
  const p = pack(repo);
  assert.equal(p.status, 0, p.stderr + p.stdout);
  assert.match(p.stdout, /free before: \d+ MiB[\s\S]*read back: 4 of 4 files[\s\S]*removed: [\s\S]*free after: \d+/);
  assert.equal(existsSync(set), false, 'the folder is removed');
  const got = await readBack(Readable.from([readFileSync(join(repo, ARCHIVE))]));
  assert.deepEqual(got.map((e) => `${e.sha} ${e.bytes} ${e.path.slice(SET.length + 1)}`), want());
  plant(set, FILES);
  refused(pack(repo), /never overwritten/);
});

test('GNU tar unpacks the archive to the same bytes, and readBack reads an archive GNU tar wrote',
  { skip: !TAR && 'GNU tar is missing' }, async (t) => {
    const { root, repo } = fixture(t);
    assert.equal(list(repo).status, 0);
    commit(repo, `${RES}/raw-logs.sha256`, 'raw-log list');
    assert.equal(pack(repo).status, 0);
    const out = join(root, 'unpacked');
    mkdirSync(out);
    const x = spawnSync('tar', ['-xzf', join(repo, ARCHIVE), '-C', out], { encoding: 'utf8' });
    assert.equal(x.status, 0, x.stderr);
    for (const [path, text] of Object.entries(FILES)) assert.equal(readFileSync(join(out, SET, path), 'utf8'), text);
    const made = spawnSync('tar', ['--format=ustar', '-czf', '-', '-C', join(out, SET), ...Object.keys(FILES).sort()]);
    assert.equal(made.status, 0, String(made.stderr));
    const got = await readBack(Readable.from([made.stdout]));
    assert.deepEqual(got.map((e) => `${e.sha} ${e.bytes} ${e.path}`), want());
  });

test('pack refuses a list that is not committed, or committed but not pushed; the folder stays, no archive', (t) => {
  const { repo, set } = fixture(t);
  assert.equal(list(repo).status, 0);
  refused(pack(repo), /not committed/);
  commit(repo, `${RES}/raw-logs.sha256`, 'raw-log list', false);
  refused(pack(repo), /not in refs\/remotes\/origin\/claude\/test/);
  assert.ok(existsSync(join(set, 'marks.json')));
  assert.equal(existsSync(join(repo, ARCHIVE)), false);
});

test('a folder that changed after its list: an extra file or a new size refuses (2), the same size with other bytes ' +
  'fails on read-back (1); the folder and the unfinished archive stay, and the next pack refuses', (t) => {
  const { repo, set } = fixture(t);
  assert.equal(list(repo).status, 0);
  commit(repo, `${RES}/raw-logs.sha256`, 'raw-log list');
  writeFileSync(join(set, 'extra.log'), 'late\n');
  refused(pack(repo), /differ from[\s\S]*tell Mustafa/);
  rmSync(join(set, 'extra.log'));
  writeFileSync(join(set, 'marks.json'), '{}\n\n');
  refused(pack(repo), /differ from/);
  writeFileSync(join(set, 'marks.json'), '[]\n');
  const c = pack(repo);
  assert.equal(c.status, 1, c.stderr + c.stdout);
  assert.match(c.stderr, /MISMATCH[\s\S]*1 of 4 files differ[\s\S]*marks\.json[\s\S]*the folder stays/);
  assert.equal(readFileSync(join(set, 'marks.json'), 'utf8'), '[]\n', 'the folder stays');
  assert.equal(existsSync(join(repo, ARCHIVE)), false);
  assert.ok(existsSync(join(repo, `${ARCHIVE}.part`)), 'the unfinished archive stays for Mustafa to see');
  refused(pack(repo), /did not finish/);
  assert.ok(existsSync(join(repo, `${ARCHIVE}.part`)), 'and the next pack leaves it');
});

test('a folder that changes after the read-back, before it would go: the second walk fails (1) on a late file, a ' +
  'late link and a grown file; the folder and the unfinished archive stay', (t) => {
  // For the child process only: when pack opens the unfinished archive to read it back, `late` runs in the repository.
  // The archive holds exactly the list, so the read-back passes and only the second walk can see the change.
  const packWith = (late) => {
    const { root, repo, set } = fixture(t);
    assert.equal(list(repo).status, 0);
    commit(repo, `${RES}/raw-logs.sha256`, 'raw-log list');
    const hook = join(root, 'late.mjs');
    writeFileSync(hook, "import fs from 'node:fs';\nimport { syncBuiltinESMExports } from 'node:module';\n" +
      'const open = fs.createReadStream;\n' +
      `fs.createReadStream = (path, ...rest) => { if (String(path).endsWith('.tar.gz.part')) { ${late}; } ` +
      'return open(path, ...rest); };\nsyncBuiltinESMExports();\n');
    const env = { ...process.env };
    delete env.SHAMBLE_ROLE;
    const p = spawnSync(process.execPath, ['--import', pathToFileURL(hook).href, SCRIPT, 'pack', SET, '--results', RES],
      { cwd: repo, encoding: 'utf8', env });
    assert.equal(p.status, 1, p.stderr + p.stdout);
    assert.ok(existsSync(join(set, 'marks.json')), 'the folder stays');
    assert.equal(existsSync(join(repo, ARCHIVE)), false);
    assert.ok(existsSync(join(repo, `${ARCHIVE}.part`)), 'the unfinished archive stays for Mustafa to see');
    return { p, set };
  };
  const a = packWith(`fs.writeFileSync(${JSON.stringify(`${RUNS}/${SET}/late.log`)}, 'late')`);
  assert.match(a.p.stderr, /MISMATCH[\s\S]*changed while it was packed[\s\S]*the folder stays/);
  assert.equal(readFileSync(join(a.set, 'late.log'), 'utf8'), 'late', 'the late file stays too');
  const b = packWith(`fs.symlinkSync('marks.json', ${JSON.stringify(`${RUNS}/${SET}/late-link`)})`);
  assert.match(b.p.stderr, /MISMATCH[\s\S]*late-link is not a plain file[\s\S]*the folder stays/);
  assert.ok(lstatSync(join(b.set, 'late-link')).isSymbolicLink(), 'the link stays, never followed');
  const c = packWith(`fs.appendFileSync(${JSON.stringify(`${RUNS}/${SET}/marks.json`)}, 'late')`);
  assert.match(c.p.stderr, /MISMATCH[\s\S]*changed while it was packed[\s\S]*the folder stays/);
  assert.equal(readFileSync(join(c.set, 'marks.json'), 'utf8'), '{}\nlate', 'the grown file stays as it is');
});

test('refused before any work: a routine, the usage, a bad set name or results path, results not committed, not the ' +
  'root, a linked worktree', (t) => {
  const { root, repo, set } = fixture(t);
  refused(run(repo, ['list', SET, '--results', RES], 'night-shift'), /routine/);
  refused(run(repo, ['cache'], 'pr-advisor'), /routine/);
  for (const args of [[], ['pack', SET], ['list', SET, '--results', RES, 'more'], ['remove', SET, '--results', RES]]) {
    refused(run(repo, args), /usage/);
  }
  for (const bad of ['..', `${SET}/..`, 'u4r-P3', 'u4r-P3-0123456789AB', `../runs/${SET}`, '-rf']) {
    refused(list(repo, bad), /not a run-set folder/);
  }
  for (const res of ['spikes/netcode-proof/results/u4r/ffffffffffff/P3', '.gate/x/0123456789ab',
    `spikes/netcode-proof/results/../../../${RUNS}/0123456789ab`]) refused(list(repo, SET, res), /--results/);
  mkdirSync(join(repo, 'spikes/netcode-proof/results/u4r/0123456789ab/P2'));
  refused(list(repo, SET, 'spikes/netcode-proof/results/u4r/0123456789ab/P2'), /no results committed/);
  writeFileSync(join(repo, RES, 'verdict.json'), '{"verdict":"FAIL"}\n');
  refused(list(repo), /not committed/);
  writeFileSync(join(repo, RES, 'verdict.json'), '{"verdict":"PASS"}\n');
  refused(run(join(repo, 'spikes'), ['list', SET, '--results', RES]), /repository root/);
  sh(repo, 'worktree', 'add', '--quiet', '-b', 'claude/wt', join(root, 'wt'));
  refused(run(join(root, 'wt'), ['list', SET, '--results', RES]), /main folder/);
  assert.ok(existsSync(join(set, 'marks.json')));
  assert.equal(existsSync(join(repo, RES, 'raw-logs.sha256')), false, 'no list was written');
});

test('odd entries refuse and stay: a symbolic link anywhere, a pipe, a long, spaced or hidden name', (t) => {
  const { root, repo, set } = fixture(t);
  symlinkSync(join(set, 'marks.json'), join(set, 'link.json'));
  refused(list(repo), /not a plain file/);
  rmSync(join(set, 'link.json'));
  symlinkSync(join(set, 'l1-p3-s1'), join(set, 'l1-p3-s1-link'));
  refused(list(repo), /not a plain file/);
  rmSync(join(set, 'l1-p3-s1-link'));
  if (spawnSync('mkfifo', [join(set, 'pipe')]).status === 0) {
    refused(list(repo), /not a plain file/);
    rmSync(join(set, 'pipe'));
  }
  for (const name of ['a'.repeat(90), 'a b.log', '.hidden']) {
    writeFileSync(join(set, name), 'x');
    refused(list(repo), /plain name/);
    rmSync(join(set, name));
  }
  const away = join(root, 'away');
  renameSync(set, away);
  symlinkSync(away, set);
  refused(list(repo), /symbolic link/);
  assert.ok(existsSync(join(away, 'marks.json')), 'the link target stays');
  rmSync(set);
  renameSync(away, set);
  assert.equal(list(repo).status, 0, 'the folder as it was lists');
});

test('cache removes the one old .gate/u3c-review-target and nothing else; it refuses a link, a tracked file, a ' +
  'worktree inside, anything changed after his words, more words', (t) => {
  const { root, repo, set } = fixture(t);
  const rel = '.gate/u3c-review-target', cache = join(repo, rel);
  refused(run(repo, ['cache']), /does not exist/);
  plant(cache, { 'debug/libx.rlib': 'x' });
  refused(run(repo, ['cache', 'now']), /usage/);
  sh(repo, 'worktree', 'add', '--quiet', '-b', 'claude/in-cache', join(cache, 'wt'));
  refused(run(repo, ['cache']), /holds the worktree/);
  sh(repo, 'worktree', 'remove', join(cache, 'wt'));
  sh(repo, 'add', '--force', `${rel}/debug/libx.rlib`);
  refused(run(repo, ['cache']), /tracked files/);
  sh(repo, 'rm', '--cached', '--quiet', `${rel}/debug/libx.rlib`);
  refused(run(repo, ['cache']), /not the one old build cache/);
  assert.ok(existsSync(join(cache, 'debug/libx.rlib')), 'a newer cache stays');
  age(cache);
  const ok = run(repo, ['cache']);
  assert.equal(ok.status, 0, ok.stderr);
  assert.match(ok.stdout, /free before: \d+ MiB[\s\S]*removed: \.gate\/u3c-review-target[\s\S]*free after: \d+ MiB/);
  assert.equal(existsSync(cache), false);
  assert.ok(existsSync(join(set, 'marks.json')), 'the run-set folder is untouched');
  const away = join(root, 'away');
  plant(away, { keep: 'x' });
  symlinkSync(away, cache);
  refused(run(repo, ['cache']), /symbolic link/);
  assert.ok(existsSync(join(away, 'keep')), 'the link target stays');
});

const entry = (name, body) => Buffer.concat([header(name, body.length, 0), body,
  Buffer.alloc((512 - (body.length % 512)) % 512)]);
const tarGz = (...blocks) => Readable.from([gzipSync(Buffer.concat(blocks))]);
test('readBack reads a plain archive and refuses a wrong checksum, another entry type, a short end and data after ' +
  'the end', async () => {
  const body = Buffer.from('hello\n'), end = Buffer.alloc(1024);
  assert.deepEqual(await readBack(tarGz(entry('s/a.txt', body), end)), [{ path: 's/a.txt', bytes: 6, sha: sha(body) }]);
  const flipped = entry('s/a.txt', body);
  flipped[0] ^= 1;
  await assert.rejects(readBack(tarGz(flipped, end)), /checksum/);
  const folder = entry('s/d/', Buffer.alloc(0));
  folder[156] = 0x35;
  folder.fill(32, 148, 156);
  folder.write(`${folder.reduce((n, b) => n + b, 0).toString(8).padStart(6, '0')}\0 `, 148, 'latin1');
  await assert.rejects(readBack(tarGz(folder, end)), /not a plain ustar file/);
  await assert.rejects(readBack(tarGz(entry('s/a.txt', body))), /ends early/);
  await assert.rejects(readBack(tarGz(entry('s/a.txt', body), end, entry('s/b.txt', body))), /after the end/);
});

test('dry run on a copy of a real run-set folder: listed, packed, read back, removed; GNU tar unpacks the same bytes',
  { skip: (!DRY || !TAR) && 'set KEPT_LOGS_DRY_RUN=<a real run-set folder> (GNU tar needed)' }, async (t) => {
    const name = basename(DRY), commit12 = /-([0-9a-f]{12})$/.exec(name)?.[1];
    assert.ok(commit12 && existsSync(DRY), `${DRY} is a run-set folder`);
    const { root, repo } = fixture(t, {});
    const res = `spikes/netcode-proof/results/dry-run/${commit12}`;
    plant(join(repo, res), { 'verdict.json': '{}\n' });
    commit(repo, res, 'dry-run results');
    cpSync(DRY, join(repo, RUNS, name), { recursive: true, preserveTimestamps: true });
    const t0 = Date.now();
    const l = list(repo, name, res);
    const t1 = Date.now();
    assert.equal(l.status, 0, l.stderr);
    commit(repo, `${res}/raw-logs.sha256`, 'dry-run list');
    const t2 = Date.now();
    const p = pack(repo, name, res);
    const t3 = Date.now();
    assert.equal(p.status, 0, p.stderr);
    assert.equal(existsSync(join(repo, RUNS, name)), false, 'the copy is removed');
    const out = join(root, 'unpacked');
    mkdirSync(out);
    const x = spawnSync('tar', ['-xzf', join(repo, `.gate/netcode-proof-test/kept-logs/${name}.tar.gz`), '-C', out]);
    assert.equal(x.status, 0, String(x.stderr));
    const rows = readFileSync(join(repo, res, 'raw-logs.sha256'), 'utf8').trimEnd().split('\n').slice(1);
    for (const row of rows) {
      const [hash, , path] = row.split(' ');
      assert.equal(await fileSha(join(DRY, path)), hash, `the real ${path} equals the list`);
      assert.equal(await fileSha(join(out, name, path)), hash, `GNU tar's ${path} equals the list`);
    }
    t.diagnostic(`${name}: ${rows.length} files; list ${t1 - t0} ms, pack ${t3 - t2} ms`);
    t.diagnostic(p.stdout.trim());
  });
