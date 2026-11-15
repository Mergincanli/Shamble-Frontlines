# KEPT-LOGS kit rule: the draft of every kit line (a proposal; no kit file is edited)

Task: draft the narrow kit rule of Mustafa's KEPT-LOGS words of 2026-11-15, 08:20 Amsterdam (history.md:2088-2111).
Plan (blocks, in order): 1 read his words and the advisor's question · 2 read the kit (settings, guard, scripts,
CHECKPOINTS) · 3 find the folders and results · 4 check step 2.3's planned kit edits for overlap · 5 write (a)-(e) ·
6 check the code from stdin (syntax, an in-memory round trip through GNU tar), writing no other file.
Blocks 1-6 done (2026-11-15, main 4460ef5, this branch at 8df63d3). DONE is the last line.

## The shape in one paragraph

One new script, `.claude/scripts/kept-logs.mjs`, is the only way to remove the two things his words cover. Its test
file sits beside it. One new section, `.claude/CHECKPOINTS.md` §11, is appended after the file's last line and changes no
existing line. No guard, `settings.json` or `CLAUDE.md` line changes. Outside the kit, `docs/REPO_MAP.md` gets two rows
for the new files, which `repo-map-check.mjs` requires, plus one changed row.

Why this is the smallest safe shape:
- In a day session the guard reads only git, gh and kit writes. A `node` script is not inspected (routine-guard.mjs:
  768-770, "scripts, inline code and the other writers are not inspected"), so the script runs with no guard change.
  That keeps clear of step 2.3's guard work.
- In a routine the guard lets `node` run main's tracked scripts (`interpreter`, routine-guard.mjs:989-1006). So the
  script refuses a routine itself. That is one line, and it is tested.
- With no `settings.json` allow line, each run asks for permission (or the auto-mode classifier decides), so a human
  sees every removal. There are about 11 runs in all, so the asks are a fitting seatbelt.
- `rm -rf` stays denied. The script deletes with Node's `rmSync` on one path it has checked. It never runs a shell `rm`.

Overlap with step 2.3 (branch claude/sweet-mayer-grsukw at 5f7f682, `kit-texts.md`):
- Its CHECKPOINTS.md edits are K-7 (lines 219-224) and K-14 (line 128). §11 adds lines after line 229, so no hunk
  overlaps.
- Its guard edits (CS-D) and its `settings.json` allow rule (S-1) are not touched here.
- REPO_MAP is in its docs list. A conflict is possible only near rows 119-122, and it would be text-only.

KIT-FREEZE-2 matters here. docs/DECISIONS.md:29-37 records it as "The freeze stops new rules, new guard checks and kit
redesign". This is a new rule. His words of 2026-11-15 order it ("It is its own reviewed kit pull request to main"), so
the draft reads them as his own exception. The chat message that shows him the lines should ask him to confirm that
(KL-4 below).

## (a) The exact text of every kit line, file by file

### (a1) `.claude/CHECKPOINTS.md`: one new section appended after line 229 (main 4460ef5), no existing line changed

Line 229 today is "- `rm -rf` stays denied." The section is appended after it with one blank line between:

<!-- BEGIN checkpoints-11 -->
```
## 11. Kept raw logs and the old build cache (Mustafa, 2026-11-15, KEPT-LOGS: a narrow rule)
His words: "What the rule may cover, and nothing else: 1. The raw-log folder of a run set whose results are committed
and whose ruling is closed. 2. The one old build cache, .gate/u3c-review-target." It is the narrow kit rule of his
words of 2026-11-09 ("if it's ever needed, it becomes a narrow kit rule instead"). `rm -rf` stays denied: only
`node .claude/scripts/kept-logs.mjs` removes either, and its header gives the commands.
- Who and when: the main session of a day session, in the main folder (the script refuses a routine and a linked
  worktree), never while a run set or a build runs: packing takes a core from timed runs, the list's commit under
  `spikes/netcode-proof/` stops a running set, and a build's cache can fill the disk under the archive.
- A raw-log folder, one at a time: `list` writes its list (each file's sha256, size and path) beside the set's
  committed results; commit it, naming the set's closed ruling (file and line) in the message, and push it. `pack`
  then packs the folder into one compressed file under `.gate/netcode-proof-test/kept-logs/`, reads every file's hash
  back from it, and removes the folder only if every hash equals the list. If one differs, the folder stays: tell
  Mustafa, and pack nothing more until he answers.
- Free space before and after, as the script prints it, goes into the checkpoint. The archives stay: nothing here
  removes one.
```
<!-- END checkpoints-11 -->

### (a2) `.claude/scripts/kept-logs.mjs`: a new file, whole

<!-- BEGIN kept-logs.mjs -->
```js
#!/usr/bin/env node
// Kept raw logs and the one old build cache (.claude/CHECKPOINTS.md §11; KEPT-LOGS, Mustafa, 2026-11-15): the only way
// a session removes either. The main session of a day session runs it in the main folder, from the repository root:
//   node .claude/scripts/kept-logs.mjs list <set-folder> --results <results-folder>
//   node .claude/scripts/kept-logs.mjs pack <set-folder> --results <results-folder>
//   node .claude/scripts/kept-logs.mjs cache
// <set-folder>: a folder name under .gate/netcode-proof-test/runs/ that ends in the 12 hex digits of its run's commit.
// <results-folder>: the set's committed results under spikes/netcode-proof/results/, a path that holds those digits.
// list:  writes <results-folder>/raw-logs.sha256, one line per file of the set folder (sha256, bytes, path). The
//        session commits it, naming the set's closed ruling in the commit message, pushes it, then runs pack.
// pack:  checks that the list is committed and pushed and that the folder holds exactly its files and sizes; packs the
//        folder into .gate/netcode-proof-test/kept-logs/<set-folder>.tar.gz; reads every file back from that archive
//        and hashes it; and removes the folder only if every path, size and sha256 equals the list. If one differs,
//        the folder stays, the unfinished archive goes, and the script exits 1: tell Mustafa.
// cache: removes .gate/u3c-review-target (the old build cache that cargo refuses: it has no CACHEDIR.TAG).
// pack and cache print free space before and after; the session copies both lines into its checkpoint.
// It removes nothing else: no other path, nothing given as a pattern, nothing through a symbolic link; the archives
// stay. The one other file it may remove is its own unfinished archive (<archive>.part, a plain file). It refuses a
// routine (SHAMBLE_ROLE set), a linked worktree, a set folder that holds anything but plain files and folders, and an
// existing list or archive. git runs without a shell and only reads. Node 22, no dependencies.
// Exit codes: 0 done · 1 a check failed while working (the folder stays) · 2 usage error, or refused before any work.
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { closeSync, createReadStream, createWriteStream, fsyncSync, lstatSync, mkdirSync, openSync, readdirSync,
  readFileSync, realpathSync, renameSync, rmSync, statfsSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { pipeline as chain } from 'node:stream';
import { pipeline } from 'node:stream/promises';
import { fileURLToPath } from 'node:url';
import { createGunzip, createGzip } from 'node:zlib';

const RUNS = '.gate/netcode-proof-test/runs';
const ARCHIVES = '.gate/netcode-proof-test/kept-logs';
const CACHE = '.gate/u3c-review-target';
const RESULTS = 'spikes/netcode-proof/results/';
const LIST = 'raw-logs.sha256';
const SET_OK = /^[A-Za-z0-9][A-Za-z0-9-]{0,63}-([0-9a-f]{12})$/;
// A relative path of plain names: letters, digits, . _ and -, and no name that starts with a dot (so no . or ..).
const PATH_OK = /^[A-Za-z0-9_-][A-Za-z0-9._-]*(\/[A-Za-z0-9_-][A-Za-z0-9._-]*)*$/;
const MIB = 1048576;

export class Mismatch extends Error {}
const say = (msg) => process.stdout.write(`kept-logs: ${msg}\n`);
const refuse = (msg) => { process.stderr.write(`kept-logs: refused: ${msg}\n`); process.exit(2); };
const git = (...args) => {
  try {
    const out = execFileSync('git', args, { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'], shell: false,
      maxBuffer: 64 << 20 });
    return { ok: true, out: out.trim() };
  } catch { return { ok: false, out: '' }; }
};
const exists = (p) => { try { lstatSync(p); return true; } catch { return false; } };
const freeMiB = (top) => { const s = statfsSync(top); return Math.floor((s.bavail * s.bsize) / MIB); };
const total = (rows) => rows.reduce((n, r) => n + r.bytes, 0);

// The repository root of the main folder (never a linked worktree), in a day session.
function root() {
  if (process.env.SHAMBLE_ROLE) refuse('a routine never removes kept logs: day sessions only (CHECKPOINTS.md §11)');
  if (git('rev-parse', '--is-inside-work-tree').out !== 'true' || git('rev-parse', '--show-prefix').out !== '') {
    refuse('run from the repository root');
  }
  const real = (p) => realpathSync(resolve(p));
  if (real(git('rev-parse', '--git-dir').out) !== real(git('rev-parse', '--git-common-dir').out)) {
    refuse('run in the main folder: a linked worktree holds no kept logs');
  }
  return realpathSync(process.cwd());
}
// A folder under top, reached with no symbolic link on the way (its real path is the plain join).
function realDir(top, rel) {
  const abs = join(top, rel);
  let real = '';
  try { real = realpathSync(abs); } catch { refuse(`${rel} does not exist`); }
  if (real !== abs) refuse(`${rel} goes through a symbolic link (${real})`);
  if (!lstatSync(abs).isDirectory()) refuse(`${rel} is not a folder`);
  return abs;
}
// Every file under the set folder as { path, bytes }, sorted by path. Anything but plain files and folders refuses.
function walk(dir, set) {
  const files = [];
  const visit = (rel) => {
    for (const name of readdirSync(join(dir, rel))) {
      const path = rel ? `${rel}/${name}` : name;
      if (!PATH_OK.test(path) || `${set}/${path}`.length > 99) {
        refuse(`${set}/${path}: not a plain name of at most 99 characters`);
      }
      const st = lstatSync(join(dir, path));
      if (st.isDirectory()) visit(path);
      else if (st.isFile() && st.size < 8 ** 11) files.push({ path, bytes: st.size });
      else refuse(`${set}/${path} is not a plain file under 8 GiB (a symbolic link, a device or a socket stays)`);
    }
  };
  visit('');
  if (!files.length) refuse(`${set} holds no files`);
  return files.sort((a, b) => (a.path < b.path ? -1 : a.path > b.path ? 1 : 0));
}
const listText = (set, rows) => `# ${RUNS}/${set}/: ${rows.length} files, ${total(rows)} bytes (sha256, bytes, path; ` +
  `.claude/scripts/kept-logs.mjs)\n${rows.map((r) => `${r.sha} ${r.bytes} ${r.path}\n`).join('')}`;
// The committed list, read only if it is exactly what list writes for this set.
function parseList(text, set) {
  const rows = text.split('\n').slice(1, -1).map((line) => {
    const m = /^([0-9a-f]{64}) (\d{1,15}) (\S+)$/.exec(line);
    if (!m) refuse(`${LIST}: "${line.slice(0, 80)}" is not "<sha256> <bytes> <path>"`);
    return { sha: m[1], bytes: Number(m[2]), path: m[3] };
  });
  if (listText(set, rows) !== text) refuse(`${LIST} is not the list this script writes for ${set}`);
  return rows;
}
// The set folder and its results folder, checked: results committed in ref (HEAD for list, the pushed branch for
// pack) and nothing uncommitted there. Returns the set folder's path and the list's path.
function checkSet(top, set, results, ref) {
  const m = SET_OK.exec(set ?? '');
  if (!m) refuse(`"${set}" is not a run-set folder name (<set>-<the 12 hex digits of its commit>)`);
  const res = String(results ?? '').replace(/\/$/, '');
  if (!res.startsWith(RESULTS) || !PATH_OK.test(res) || !res.split('/').includes(m[1])) {
    refuse(`--results must be the set's results folder under ${RESULTS}, a path that holds ${m[1]}`);
  }
  realDir(top, res);
  const listRel = `${res}/${LIST}`;
  if (!git('ls-tree', '-r', '--name-only', ref, `${res}/`).out.split('\n').some((f) => f && f !== listRel)) {
    refuse(`${res} holds no results committed in ${ref}`);
  }
  const status = git('status', '--porcelain', '--', res);
  if (!status.ok || status.out !== '') refuse(`${res} has changes that are not committed`);
  return { dir: realDir(top, `${RUNS}/${set}`), listRel };
}

async function list(top, set, results) {
  const { dir, listRel } = checkSet(top, set, results, 'HEAD');
  if (exists(join(top, listRel))) refuse(`${listRel} exists: a list is never overwritten`);
  const rows = [];
  for (const f of walk(dir, set)) {
    const hash = createHash('sha256');
    for await (const chunk of createReadStream(join(dir, f.path))) hash.update(chunk);
    rows.push({ ...f, sha: hash.digest('hex') });
  }
  writeFileSync(join(top, listRel), listText(set, rows), { flag: 'wx' });
  say(`list written: ${listRel} (${rows.length} files, ${total(rows)} bytes)`);
  say("next: commit it, naming the set's closed ruling (file and line) in the message; push it; then run pack");
}

// A POSIX ustar header for a plain file: name (at most 99 bytes), mode 644, owner 0, size and mtime in octal.
export function header(name, size, mtime) {
  const h = Buffer.alloc(512);
  const put = (text, at) => h.write(text, at, 'latin1');
  const oct = (n, width) => `${n.toString(8).padStart(width, '0')}\0`;
  put(name, 0); put(oct(0o644, 7), 100); put(oct(0, 7), 108); put(oct(0, 7), 116);
  put(oct(size, 11), 124); put(oct(mtime, 11), 136); put('        ', 148); put('0', 156);
  put('ustar\0', 257); put('00', 263);
  put(`${oct(h.reduce((n, b) => n + b, 0), 6)} `, 148);
  return h;
}
// Reads back a gzip'd ustar stream as pack writes it: plain-file entries, then zero blocks to the end. Returns
// [{ path, bytes, sha }]; anything else throws Mismatch.
export async function readBack(gz) {
  const got = [];
  let buf = Buffer.alloc(0), cur = null, pad = 0, zeros = 0;
  const field = (h, a, b) => h.subarray(a, b).toString('latin1').replace(/\0[\s\S]*$/, '');
  for await (const chunk of chain(gz, createGunzip(), () => {})) {
    buf = Buffer.concat([buf, chunk]);
    for (;;) {
      if (cur) {
        const n = Math.min(cur.left, buf.length);
        cur.hash.update(buf.subarray(0, n));
        buf = buf.subarray(n);
        cur.left -= n;
        if (cur.left) break;
        got.push({ path: cur.path, bytes: cur.bytes, sha: cur.hash.digest('hex') });
        pad = (512 - (cur.bytes % 512)) % 512;
        cur = null;
      }
      if (pad) {
        const n = Math.min(pad, buf.length);
        buf = buf.subarray(n);
        pad -= n;
        if (pad) break;
      }
      if (buf.length < 512) break;
      const h = buf.subarray(0, 512);
      buf = buf.subarray(512);
      if (h.every((b) => b === 0)) { zeros++; continue; }
      if (zeros) throw new Mismatch('data after the end of the archive');
      const sum = h.reduce((n, b, i) => n + (i >= 148 && i < 156 ? 32 : b), 0);
      if (parseInt(field(h, 148, 156).trim(), 8) !== sum) throw new Mismatch('a header checksum is wrong');
      if (field(h, 257, 263) !== 'ustar' || ![0x30, 0].includes(h[156]) || field(h, 345, 500) !== '') {
        throw new Mismatch(`an entry that is not a plain ustar file (${field(h, 0, 100)})`);
      }
      const bytes = parseInt(field(h, 124, 136).trim(), 8);
      if (!Number.isSafeInteger(bytes)) throw new Mismatch('a header size is not a number');
      cur = { path: field(h, 0, 100), bytes, left: bytes, hash: createHash('sha256') };
    }
  }
  if (cur || pad || zeros < 2 || buf.length) throw new Mismatch('the archive ends early');
  return got;
}

async function pack(top, set, results) {
  const branch = git('symbolic-ref', '--quiet', '--short', 'HEAD');
  if (!branch.ok) refuse("detached HEAD: pack runs on the branch that holds the set's results");
  const pushed = `refs/remotes/origin/${branch.out}`;
  const { dir, listRel } = checkSet(top, set, results, pushed);
  const onDisk = git('hash-object', '--', listRel).out;
  for (const ref of ['HEAD', pushed]) {
    const blob = git('rev-parse', '--verify', '--quiet', `${ref}:${listRel}`);
    if (!onDisk || !blob.ok || blob.out !== onDisk) {
      refuse(`${listRel} is not in ${ref} as it is on disk: commit it and push it first`);
    }
  }
  const rows = parseList(readFileSync(join(top, listRel), 'utf8'), set);
  const now = walk(dir, set);
  if (now.length !== rows.length || now.some((f, i) => f.path !== rows[i].path || f.bytes !== rows[i].bytes)) {
    refuse(`the folder's files or sizes differ from ${listRel}: the folder stays`);
  }
  mkdirSync(join(top, ARCHIVES), { recursive: true });
  const archive = join(realDir(top, ARCHIVES), `${set}.tar.gz`);
  const part = `${archive}.part`;
  if (exists(archive)) refuse(`${ARCHIVES}/${set}.tar.gz exists: an archive is never overwritten`);
  if (exists(part)) {
    if (!lstatSync(part).isFile()) refuse(`${ARCHIVES}/${set}.tar.gz.part is not a plain file`);
    rmSync(part);
    say(`removed this script's unfinished archive from a run that did not finish: ${ARCHIVES}/${set}.tar.gz.part`);
  }
  const before = freeMiB(top);
  say(`free before: ${before} MiB`);
  if (before * MIB < total(rows) + 1024 * MIB) {
    refuse(`${before} MiB free is less than the folder's ${Math.ceil(total(rows) / MIB)} MiB plus 1024 MiB`);
  }
  try {
    await pipeline(async function* entries() {
      for (const r of rows) {
        const st = lstatSync(join(dir, r.path));
        if (!st.isFile()) throw new Mismatch(`${r.path} is no longer a plain file`);
        yield header(`${set}/${r.path}`, r.bytes, Math.floor(st.mtimeMs / 1000));
        let n = 0;
        for await (const chunk of createReadStream(join(dir, r.path))) {
          n += chunk.length;
          if (n > r.bytes) break;
          yield chunk;
        }
        if (n !== r.bytes) throw new Mismatch(`${r.path} changed size while it was packed`);
        if (r.bytes % 512) yield Buffer.alloc(512 - (r.bytes % 512));
      }
      yield Buffer.alloc(1024);
    }, createGzip(), createWriteStream(part, { flags: 'wx' }));
    const fd = openSync(part, 'r');
    try { fsyncSync(fd); } finally { closeSync(fd); }
    const got = await readBack(createReadStream(part));
    const bad = rows.filter((r, i) => got[i]?.path !== `${set}/${r.path}` || got[i].bytes !== r.bytes ||
      got[i].sha !== r.sha);
    if (bad.length || got.length !== rows.length) {
      throw new Mismatch(`read back from the archive: ${bad.length} of ${rows.length} files differ from the list ` +
        `(${got.length} entries): ${bad.map((r) => r.path).join(', ')}`);
    }
    renameSync(part, archive);
  } catch (e) {
    rmSync(part, { force: true });
    process.stderr.write(`kept-logs: ${e instanceof Mismatch ? 'MISMATCH' : 'failed'}: ${e?.message ?? e}\n` +
      `kept-logs: the folder stays (${RUNS}/${set}) and the unfinished archive is removed; tell Mustafa ` +
      '(CHECKPOINTS.md §11)\n');
    process.exit(1);
  }
  say(`archive: ${ARCHIVES}/${set}.tar.gz, ${lstatSync(archive).size} bytes from ${total(rows)}; read back: ` +
    `${rows.length} of ${rows.length} files equal the list (path, bytes, sha256)`);
  try { rmSync(dir, { recursive: true }); } catch (e) {
    process.stderr.write(`kept-logs: failed while removing ${RUNS}/${set} (${e.message}): the archive is complete ` +
      'and checked, and the folder may be partly removed; tell Mustafa\n');
    process.exit(1);
  }
  say(`removed: ${RUNS}/${set}`);
  say(`free after: ${freeMiB(top)} MiB`);
}

function cache(top) {
  const dir = realDir(top, CACHE);
  const tracked = git('ls-files', '--', CACHE);
  if (!tracked.ok || tracked.out !== '') refuse(`${CACHE} holds tracked files`);
  const wts = git('worktree', 'list', '--porcelain', '-z');
  if (!wts.ok) refuse('git could not list the worktrees');
  for (const w of wts.out.split('\0').filter((l) => l.startsWith('worktree ')).map((l) => l.slice(9))) {
    let real = w;
    try { real = realpathSync(w); } catch { /* a worktree that is gone keeps its path */ }
    if (real === dir || real.startsWith(`${dir}/`)) refuse(`${CACHE} holds the worktree ${w}`);
  }
  say(`free before: ${freeMiB(top)} MiB`);
  rmSync(dir, { recursive: true });
  say(`removed: ${CACHE}`);
  say(`free after: ${freeMiB(top)} MiB`);
}

async function main(args) {
  const [cmd, set, flag, results, ...extra] = args;
  if (cmd === 'cache' && args.length === 1) return cache(root());
  if ((cmd === 'list' || cmd === 'pack') && flag === '--results' && results && !extra.length) {
    const top = root();
    return cmd === 'list' ? list(top, set, results) : pack(top, set, results);
  }
  return refuse('usage: kept-logs.mjs list|pack <set-folder> --results <results-folder> | kept-logs.mjs cache');
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try { await main(process.argv.slice(2)); } catch (e) {
    process.stderr.write(`kept-logs: failed: ${e?.message ?? e}\n`);
    process.exit(1);
  }
}
```
<!-- END kept-logs.mjs -->

### (a3) `.claude/scripts/kept-logs.test.mjs`: a new file, whole

<!-- BEGIN kept-logs.test.mjs -->
```js
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
import { cpSync, createReadStream, existsSync, mkdirSync, mkdtempSync, readFileSync, renameSync, rmSync, symlinkSync,
  writeFileSync } from 'node:fs';
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
  'fails on read-back (1); the folder stays, no archive, no .part', (t) => {
  const { repo, set } = fixture(t);
  assert.equal(list(repo).status, 0);
  commit(repo, `${RES}/raw-logs.sha256`, 'raw-log list');
  writeFileSync(join(set, 'extra.log'), 'late\n');
  refused(pack(repo), /differ from/);
  rmSync(join(set, 'extra.log'));
  writeFileSync(join(set, 'marks.json'), '{}\n\n');
  refused(pack(repo), /differ from/);
  writeFileSync(join(set, 'marks.json'), '[]\n');
  const c = pack(repo);
  assert.equal(c.status, 1, c.stderr + c.stdout);
  assert.match(c.stderr, /MISMATCH[\s\S]*1 of 4 files differ[\s\S]*marks\.json[\s\S]*the folder stays/);
  assert.equal(readFileSync(join(set, 'marks.json'), 'utf8'), '[]\n', 'the folder stays');
  assert.equal(existsSync(join(repo, ARCHIVE)), false);
  assert.equal(existsSync(join(repo, `${ARCHIVE}.part`)), false);
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

test('cache removes .gate/u3c-review-target and nothing else; it refuses a link, a tracked file, a worktree inside, ' +
  'more words', (t) => {
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
```
<!-- END kept-logs.test.mjs -->

### (a4) Not kit lines, but in the same pull request: `docs/REPO_MAP.md` (tier 0)

`repo-map-check.mjs` lists every `.claude/scripts/*` file (repo-map-check.mjs:30-31), so two rows are needed. Insert them
after line 121 (the `checkpoint.test.mjs` row), keeping the map's name order:

<!-- BEGIN repo-map-rows -->
```
| `.claude/scripts/kept-logs.mjs` | KEPT-LOGS (`CHECKPOINTS.md` §11): `list` writes a closed run set's raw-log list beside its results; `pack` packs the folder, reads every file's hash back and removes the folder only if all equal the committed list; `cache` removes `.gate/u3c-review-target`; it refuses everything else |
| `.claude/scripts/kept-logs.test.mjs` | Its tests: `node --test .claude/scripts/kept-logs.test.mjs`; `KEPT_LOGS_DRY_RUN=<a real run-set folder>` adds the dry run on a copy |
```
<!-- END repo-map-rows -->

The `CHECKPOINTS.md` row (line 82) changes only at its end, so sessions that read the map find §11:
- Before: `...; the background saver checked at every update while agents run, and restarted if it has stopped (§3, §5); disk hygiene (§10) |`
- After: `...; the background saver checked at every update while agents run, and restarted if it has stopped (§3, §5); disk hygiene (§10); kept raw logs and the old build cache, the one narrow delete rule (§11) |`

Not in this pull request:
- `settings.json`
- the guard
- `CLAUDE.md`
- the agent files
- `docs/DECISIONS.md`: this is KL-5. §11 quotes his scope words itself, so the rule stands without a register entry.

Size: §11 is 15 lines, plus the one blank line before it. The script is 304 lines and the test file 278 lines, both
measured from the blocks above (`awk`, widest line 120). REPO_MAP gets 2 new rows and 1 changed row. That is 600 lines
added and 1 changed.

## (b) Why each line is needed, and nothing more

### §11 (CHECKPOINTS.md)

- **Where it goes:** §10 is "Disk hygiene (Mustafa, 2026-11-08; guidance, no rule change)", and its last line is "rm -rf
  stays denied".
  - A rule bullet inside §10 would make that heading untrue, and changing the heading is one more changed line.
  - A new section after §10 changes no existing line, and it keeps clear of step 2.3's K-7 (lines 219-224).
- **The heading** names his decision, the date and the label (KEPT-LOGS), and says it is a narrow rule, not guidance.
- **"His words: ..."** quotes his two scope items. Each line break is joined to a space, the way step 2.3's kit texts
  check quotes ("each found in DECISIONS.md once whitespace is joined").
  - The scope is his own text, so it cannot drift wider than his words.
  - No register entry is needed for the rule to stand.
  - At build time a script must check the quote byte for byte against history.md:2095-2097. An apostrophe check on
    `it's` was done here: it is ASCII 0x27 in history.md.
- **"It is the narrow kit rule of his words of 2026-11-09 (...)"** gives the rule its origin.
  - The clause is the one he quoted himself on 2026-11-15 (history.md:2105), so it can be checked byte for byte too.
- **"`rm -rf` stays denied: only `node .claude/scripts/kept-logs.mjs` removes either"** says this adds one method and
  opens no general delete.
  - The deny list (settings.json:247) stays as it is.
- **"Who and when"** has three parts:
  - **Main session of a day session, main folder:** the kept logs exist only in the main folder's `.gate/`. Routines
    run on other machines. An agent is not one of the watched hands his words address ("you tell me").
  - **The script refuses a routine and a linked worktree:** the reader learns that this half is enforced, not just
    asked for.
  - **Never while a run set or a build runs**, for three reasons:
    1. gzip takes a whole core, and the runs are timed. Ticks-per-second and wall-clock checks: c-u4r-runs.mjs:93-94.
    2. Any commit under `spikes/netcode-proof/` stops a running set: c-u4r-runs.mjs:78-82.
    3. A shared-target refill of about 9 GiB could fill the disk under a half-written archive.
- **"A raw-log folder, one at a time"** gives his steps in his order: list committed, then packed, then hashes read
  back, then removal only when every hash is equal. Three details go beyond the bare step:
  - **The list sits beside the set's committed results:** it reaches main with PR #40's squash, while `.checkpoints/` is
    deleted from the final tree (SQUASH-40). This is KL-1.
  - **The closed ruling is named in the commit message:** the script cannot check that a ruling is closed. Naming the
    ruling's file and line makes the session's claim part of the record.
  - **"Pack nothing more until he answers":** this is my addition, not his words. It only narrows the rule: a mismatch
    means a bug or a disk fault, and the next folder should not go before he has looked (KL-3).
- **"Free space before and after ... goes into the checkpoint"** is his third bullet. The script prints the two lines,
  and the checkpoint is the committed record.
- **"The archives stay: nothing here removes one"** makes it plain that the rule does not cover deleting an archive.

### The script, block by block

- **The header** is the rule's procedure in one place. It gives the commands, what each does, what it refuses, and the
  exit codes. This follows `checkpoint.mjs`'s header style.
- **The fixed paths:**
  - `RUNS`, `CACHE` and the archive folder are the two things his words name, plus where the archives go.
  - `RESULTS` scopes the results check to this spike. `LIST` is the list's one name.
  - The rule is narrow on purpose: another spike's logs would need their own rule.
- **`SET_OK`:**
  - It matches the 10 folders under `runs/` today: `u4r-P3-0f066338dd8d`, `c-u2-pm-0001-3-81c6d01e371b`,
    `u4r-precheck-30-faf42aeb034c` and the rest.
  - It refuses `..`, `/` and a leading `-`.
  - It captures the commit's 12 hex digits, so the results path can be paired with the set.
- **`PATH_OK`** allows plain names only. It rejects `.` and `..`, hidden names, spaces and newlines, so a list line
  splits safely on spaces.
  - All 195 files (242 entries with the folders) under `runs/` pass it today. A `find` with that pattern printed
    nothing.
- **`root()`** has three checks:
  - It refuses a routine. The guard would let a routine run main's tracked script (routine-guard.mjs:989-1006), so the
    script's own check is the only stop there.
  - It requires the repository root and the main worktree: the git dir must equal the common dir. A linked worktree's
    `.gate/` holds no kept logs.
- **`realDir()`:** a path whose real path is not the plain join goes through a symbolic link, so it is refused. This
  covers `.gate`, `netcode-proof-test`, `runs`, the set folder itself and the cache.
- **`walk()`:**
  - It lists the folder with `lstat`, so a symbolic link is never followed.
  - It refuses a symbolic link, a pipe, a device or a socket: those cannot be archived faithfully, and their targets
    must stay.
  - It refuses a file of 8 GiB or more (the ustar size field's limit) and a name over 99 bytes (the ustar name field).
    Today's longest is 63 and the biggest file is 573 MB.
  - It refuses an empty folder.
  - It sorts by path, so the list and the archive have one order.
- **`listText()` / `parseList()`:**
  - One fixed format: a header line, then `<sha256> <bytes> <path>` per file.
  - `parseList` rebuilds the text from the rows it read and requires it to equal the committed file byte for byte. A
    hand-edited, re-ordered or padded list is refused.
- **`checkSet()`** is shared by `list` and `pack`:
  - The results path must sit under `spikes/netcode-proof/results/` and hold the set's 12 digits.
  - At least one file other than the list must be committed there. That is "a run set whose results are committed",
    checked against HEAD for `list` and against the pushed branch for `pack`.
  - Nothing uncommitted may be there.
- **`list`:**
  - It refuses an existing list. The `wx` flag makes the refusal hold under a race too, and a symbolic link at that
    path is never followed.
  - It hashes each file and writes the list. It prints the next step.
  - It never commits: git writes stay with the session, so the script only reads git.
- **`header()` / `readBack()`:**
  - A minimal POSIX ustar writer and reader, so the script has no dependency and no shelled-out `tar` with its options.
  - The reader accepts only what the writer makes: plain ustar files with correct checksums, a short name, then zero
    blocks to the end.
  - It uses `stream.pipeline`, so a read error ends the loop instead of hanging it.
  - The two are tested against GNU tar in both directions, so a fault shared by the writer and the reader would show
    (test 2, and the dry run).
- **`pack`:**
  1. The list must be on disk, in HEAD and in `origin/<branch>`, all three equal. That is "committed", and pushed, so a
     container loss cannot take the only record.
  2. The folder now must equal the list's paths and sizes. That catches an added file, which would otherwise be deleted
     unarchived.
  3. The archive is never overwritten. A stale `.part` that is a plain file is the script's own unfinished output, so it
     is removed. Anything else refuses.
  4. Free space is printed. Packing needs the folder's size plus 1 GiB free, the worst case with no compression, so
     packing can never fill the disk.
  5. It writes the archive as `.part` (`wx`), then fsyncs it.
  6. It reads every entry back from the `.part` file and compares path, size and sha256 with the committed list. This is
     his "every file's hash read back from that archive equals the list".
  7. Only then: rename to `.tar.gz`, remove the folder, print free space after.
  - On any mismatch or error before step 7, the `.part` is removed, the folder stays, the script exits 1 and says to tell
    Mustafa.
  - If the removal fails part way, it says the archive is complete and checked, and the folder may be partly removed.
- **`cache`:** removes the one fixed path. It refuses a symbolic link, any tracked file inside, and any registered
  worktree inside. It prints free space before and after.
- **`main`:**
  - The usage is exact. Extra words refuse, and so does any other command name.
  - The script runs only as a program, so the tests can import `header` and `readBack`.

### The test file

Each test proves one rule line, and the mutation runs in (c) show that each one bites.
1. **The happy path:**
   - The exact list text.
   - The commit, then a second `list` refused.
   - `pack` prints free space before and after.
   - The folder is gone.
   - The archive reads back to the list.
   - A second `pack` with the folder re-made is refused, because the archive exists.
2. **GNU tar in both directions:** it unpacks our archive to the same bytes, and our reader reads GNU tar's archive.
3. **The list is not committed / not pushed:** both refused, and the folder stays.
4. **The folder changed after its list:**
   - An extra file is refused (2).
   - A new size is refused (2).
   - Same size, other bytes: refused on read-back (1), with "MISMATCH" and the file named. The folder stays, with no
     archive and no `.part`.
   - This is his "If one hash differs, the folder stays".
5. **Refused before any work:**
   - a routine
   - the usage
   - six bad set names
   - three bad results paths
   - results with nothing committed
   - uncommitted results
   - not the root
   - a linked worktree
6. **Odd entries:**
   - a symbolic link to a file
   - a symbolic link to a folder
   - a pipe
   - a long, a spaced and a hidden name
   - the set folder reached through a link, whose target stays
7. **`cache`:**
   - missing
   - more words
   - a worktree inside
   - a tracked file inside
   - success, with the run-set folder untouched
   - a link in its place, whose target stays
8. **`readBack`'s refusals:** a wrong checksum, a folder entry, a short end, and data after the end.
9. **The dry run:** opt-in, see (c).

## (c) How it is tested

All of this happens at build time, after Mustafa's yes to the lines. It runs in a worktree from origin/main
(`git worktree add .gate/kept-logs-wt -b cs/kept-logs origin/main`; or a `claude/` stand-in branch if the cloud session
is limited to one). The main folder never switches branch. The `.claude/` writes ask him first, and auto mode may refuse
them before the prompt, so they are made in Accept edits or Manual mode (CLAUDE.md, protocol step 3).

1. **Its own tests:** `node --test .claude/scripts/kept-logs.test.mjs`. Expected: 8 pass, 1 skipped (the dry run).
2. **The tests bite** (`KEPT_LOGS_SCRIPT=<mutated copy>`; each copy is made under `.gate/kept-logs-rule/` by a small
   script that applies one exact replacement and checks that it matched once):

   | Mutation | Expected to fail |
   |---|---|
   | M1: read-back compare removed (`bad` always empty) | test 4 |
   | M2: the folder removed before the read-back | test 4 |
   | M3: the routine refusal removed | test 5 |
   | M4: the folder-equals-list check removed | test 4, extra-file case |
   | M5: `realDir`'s link check removed | tests 6 and 7 |
   | M6: the pushed-ref check removed | test 3 |
   | M7: `cache`'s worktree check removed | test 7 |
   | M8: `readBack`'s checksum check removed | test 8 |

   The evidence is each mutation's failing test names.
3. **The dry run on copies**, two real sets:
   - `u4r-precheck-30-faf42aeb034c` (17 MiB, 2 files)
   - `u4r-P3-fd4612050cd6` (291 MiB, 11 files in 3 folders)
   - Each runs as `KEPT_LOGS_DRY_RUN=/home/user/Shamble-Frontlines/.gate/netcode-proof-test/runs/<set>`
     `node --test --test-name-pattern='dry run' .claude/scripts/kept-logs.test.mjs`.
   - The test copies the real folder into a scratch repository in the temp folder, then runs `list`, commit and push,
     and `pack`. GNU tar unpacks the archive, and every file's sha256 is compared with the real folder and with the
     list. The scratch repository deletes itself.
   - Before and after each dry run, a fingerprint of the real folder must be equal: `find <real> -type f -printf
     '%P %s %T@\n' | sort | sha256sum`. That shows the real folder was only read.
   - Recorded: the times, the archive size and the ratio. P0's time (3.4 GB) is estimated from P3's MB/s.
   - Peak disk is about 2.3 times the folder: 0.7 GiB for P3.
4. **Nothing else moved:**
   - `node --test .claude/scripts/checkpoint.test.mjs`
   - `node --test .claude/hooks/settings.test.mjs`
   - `node --test .claude/hooks/routine-guard.test.mjs`
   - `node .claude/scripts/repo-map-check.mjs`
   - `node tools/gate.mjs` (PASS; no game code changes)
5. **The guard needs no change:** the real hook is fed the day-session command, the way settings.test.mjs:102-107 runs
   it. The guard should print nothing (no decision). The command is `node .claude/scripts/kept-logs.mjs pack
   u4r-P3-fd4612050cd6 --results spikes/netcode-proof/results/u4r/fd4612050cd6/P3`.
6. **The quotes:** a script checks §11's two quotes, whitespace joined, byte for byte against history.md:2095-2097 and
   :2105.
7. **Reviews:**
   - This is risk tier 2: the advisor (the plan, then every pushed head) and security-auditor.
   - test-engineer is recommended for the proof.
   - It merges on Mustafa's go in a day session. Nothing is packed or deleted before the PR is on main and the netcode
     branch has taken main in (his words).

Done while drafting (read-only, no file written): both code blocks pass `node --input-type=module --check` from stdin.
An in-memory round trip (also from stdin) checked three things:
- `header()` and `readBack()` round-trip: the entries are named, sized and hashed right. These are a file whose size is
  not a multiple of 512, a file of exactly 512 bytes, an empty file and one in a subfolder.
- GNU tar 1.35 lists the same archive from stdin with no warning and extracts each file to the same sha256.
- `readBack` refuses a flipped name byte (wrong checksum) and a missing end. Data that is not gzip ends in an error
  (Z_DATA_ERROR), not a hang.
- `listText` then `parseList` gives back the same rows, and `SET_OK` takes today's folder names and refuses `..`,
  `u4r-P3`, `-rf` and upper-case hex.

The archive's stream shape was copied into the harness from `pack`. It is not `pack` itself, which needs files on disk.

Not done here:
- the test suite
- the mutation runs
- the dry run

All three need files written, and they belong to the build step.

## (d) What it can never touch

- Any path but these three:
  - `.gate/netcode-proof-test/runs/<name>-<12 hex>`: a direct child, a real folder, with plain files only.
  - `.gate/u3c-review-target`.
  - Its own `.gate/netcode-proof-test/kept-logs/<set>.tar.gz.part`.
- So these are out of reach:
  - `runs/` itself.
  - The other folders in `.gate/netcode-proof-test/` (`bin`, `det-u4`, `r2-e59ece9ecafc`, the three `c62h-5e563b9589d3*`
    folders, `target`).
  - `.gate/shared-target` and every other `.gate/` folder.
  - Worktrees and `.checkpoints/`.
  - Every tracked file, the results, and every archive once made.
- Nothing through a symbolic link: not on the way to the folder, not inside it, not the cache.
- Nothing given as a pattern. It removes one named folder per call, and no glob ever reaches `rmSync`.
- No folder whose list is missing, not committed, not pushed, malformed, or unequal to the folder's files and sizes now.
- No folder whose archive does not read back equal to the list, path, size and sha256 for every file.
- No list or archive is ever overwritten.
- No git write: it runs `rev-parse`, `ls-tree`, `ls-files`, `status`, `hash-object` (without `-w`), `symbolic-ref` and
  `worktree list`. The session commits and pushes.
- Never in a routine, a linked worktree, or a subfolder of the repository.
- The archives: no command removes one.

## (e) The risks

1. **"Whose ruling is closed" is not machine-checked.** The script cannot tell. To keep it honest:
   - The list's commit message names the ruling (file and line).
   - The checkpoint records it.
   - The advisor sees the commits.
   - Today's candidates are below, and the session cites each ruling before `list`.
2. **Read-back reads from the page cache.** The check proves the archive as the system serves it right after writing. A
   later disk fault is not covered.
   - The committed list allows a later re-check: `tar -xzOf <archive> <path> | sha256sum`.
   - The raw logs live only in this container today anyway, so this is no new loss.
3. **A fault shared by our tar writer and reader** would pass read-back. GNU tar is used in both directions in test 2
   and in the dry run, which compares with the real files. The session may also run `tar -tzf` on each real archive
   (optional).
4. **The only stop in a routine is the script's own check.** The guard allows main's tracked scripts. Test 5 and
   mutation M3 cover it, and the routines' containers hold none of these folders.
5. **An agent could run it.** The script cannot tell an agent from the main session, and the guard does not see `node`
   scripts (its stated limits). §11 says the main session runs it. Each run also asks for permission, since there is no
   allow line (KL-2).
6. **Permission prompts.** About 11 removal runs and about 10 list runs each ask, or the auto-mode classifier decides.
   If the classifier refuses, the session asks Mustafa. That is intended.
7. **KIT-FREEZE-2:** this is a new rule during the freeze. It rests on his 2026-11-15 words (KL-4).
8. **Overlap with step 2.3:**
   - CHECKPOINTS.md: no shared hunk (K-7 at 219-224 and K-14 at 128, against lines appended after 229).
   - REPO_MAP: possible text-only conflict near rows 119-122. Check with `git merge-tree` at merge time.
9. **Runner interplay:** a list commit under `spikes/netcode-proof/` stops a set that is running (c-u4r-runs.mjs:81-82).
   §11's "never while a run set ... runs" covers it.
10. **The free-space check is conservative:** folder size plus 1 GiB. Today that is at most 3,393 + 1,024 = 4,417 MiB
    against 7,711 MiB free. A refusal is a stop, never a loss.
11. **A removal that fails part way:** the archive is complete and checked, the folder is partly gone, and the script
    says so (exit 1). The next `pack` refuses (the archive exists) until Mustafa decides.
12. **PR #40 grows:** the lists add 10 files and 205 lines (195 file lines and 10 headers) under `spikes/netcode-proof/results/`, as data,
    not code. A list is text (64-hex hashes, sizes, plain names). redact-check scans the spike folder
    (check.mjs:2630-2633), and runs.txt already carries sha256s.
13. **`.gate/u3c-review-target`** also holds `build.log` (1.9 KB), an empty `c2fm/` and `lintprobe/`.
    - The advisor judged it "no evidence" (advisor-brawl1-plan-full.md:119).
    - LEFTOVER-DELETE recorded it as a leftover (history.md:406).
    - His words name the folder whole.
14. **Timing:** gzip-6 on one core takes an estimated 1-2 minutes for a P0 set (3.4 GB). The dry run's MB/s on P3 gives
    the figure before the real act.

### Candidate sets for after the merge (not kit lines; the session confirms each ruling)

`du -sm` at 2026-11-15, nothing running, free 7,711 MiB:

| Set folder (under `.gate/netcode-proof-test/runs/`) | MiB | Files | Results folder (under `spikes/netcode-proof/results/`) |
|---|---|---|---|
| `u4r-precheck-30-faf42aeb034c` | 17 | 2 | `u4r-precheck-30/faf42aeb034c` |
| `u4r-P3-fd4612050cd6` (BRAWL-1 re-run) | 291 | 11 | `u4r/fd4612050cd6/P3` |
| `u4r-P3-0f066338dd8d` | 519 | 11 | `u4r/0f066338dd8d/P3` |
| `u4r-P2-fc411116c1f1` (BRAWL-1 re-run) | 623 | 11 | `u4r/fc411116c1f1/P2` |
| `u4r-P2-ab10f058a828` | 779 | 11 | `u4r/ab10f058a828/P2` |
| `u4r-B1-64f3307d5c7f` | 821 | 11 | `u4r/64f3307d5c7f/B1` |
| `c-u2-pm-0001-3-65869918809f` | 998 | 39 | `c-u2-pm-0001-3/65869918809f` |
| `c-u2-pm-0001-3-81c6d01e371b` | 998 | 39 | `c-u2-pm-0001-3/81c6d01e371b` |
| `u4r-P0-051995db9cea` | 3,249 | 30 | `u4r/051995db9cea/P0` |
| `u4r-P0-93509dfd7fd1` (BRAWL-1 re-run) | 3,393 | 30 | `u4r/93509dfd7fd1/P0` |

- **Raw logs:** 11,688 MiB in all.
- **Archives:** at the advisor's 3.55 times (one P3 run, zlib-6), about 3.3 GiB. That frees about 8.2 GiB.
- **The cache:** 4,051 MiB more.
- **Free after everything:** about 20 GiB (an estimate; the ratio was measured on one run).

## Decisions for the advisor and Mustafa

- **KL-1, where the committed list goes.**
  - **Why it matters:** the list must outlive the folder, and `.checkpoints/` leaves PR #40's final tree (SQUASH-40).
  - **Options:** (a) beside the set's committed results (`<results>/raw-logs.sha256`), so it reaches main with the
    spike; (b) in `.checkpoints/netcode-proof-test/kept-logs/`, so it lives in the branch history only.
  - **Recommendation:** (a).
- **KL-2, an allow line in `settings.json` for the script.**
  - **Why it matters:** without one, each run asks; with one, a second kit file changes, near step 2.3's S-1.
  - **Options:** (a) none: each run asks, or the classifier decides; (b) `Bash(node .claude/scripts/kept-logs.mjs *)`.
  - **Recommendation:** (a).
- **KL-3, "pack nothing more until he answers"** after a mismatch.
  - **Why it matters:** it is the draft's addition to his words.
  - **Options:** (a) keep it, which only narrows the rule; (b) drop it.
  - **Recommendation:** (a).
- **KL-4, KIT-FREEZE-2.**
  - **Why it matters:** the freeze stops new rules (DECISIONS.md:33).
  - **The ask:** read his 2026-11-15 words as his own exception, and ask him to confirm that in the same message that
    shows him the lines.
  - **Recommendation:** ask, in one line.
- **KL-5, his KEPT-LOGS words in `docs/DECISIONS.md`.**
  - **Why it matters:** it is the register of his words; LEFTOVER-DELETE is not there either.
  - **Options:** (a) docs-keeper's next pass; (b) in this PR, by script.
  - **Recommendation:** (a). §11 quotes his scope itself, and the kit PR stays in one place.

What the session shows him in chat:
1. §11 whole.
2. The script whole.
3. The test file whole.
4. The REPO_MAP rows, marked as not kit.
5. KL-4's one-line question.

That is about 600 lines. He asked for "the exact text of every kit line", so the whole of it goes in.

DONE
