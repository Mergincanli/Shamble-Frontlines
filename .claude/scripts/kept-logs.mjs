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
//        and hashes it; walks the folder once more; and removes the folder only if every path, size and sha256 read
//        back equals the list and the folder still holds exactly its files and sizes. If one differs, the folder
//        stays, and so does the unfinished archive (<archive>.part), and the script exits 1: tell Mustafa.
// cache: removes .gate/u3c-review-target, the one old build cache (cargo refuses it: it has no CACHEDIR.TAG), only
//        while nothing in it is newer than his words (2026-11-15 07:20 UTC).
// pack and cache print free space before and after; the session copies both lines into its checkpoint.
// It removes nothing else: no other path, nothing given as a pattern, nothing through a symbolic link, no archive,
// finished or not. It refuses a routine (SHAMBLE_ROLE set), a linked worktree, a set folder that holds anything but
// plain files and folders, and an existing list, archive or unfinished archive. git runs without a shell and only
// reads. Node 22, no dependencies.
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
// The one OLD build cache: a folder there with anything changed after his words (07:20 UTC) is not that one.
const OLD_BEFORE = Date.parse('2026-11-15T07:20:00Z');

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
// Every file under the set folder as { path, bytes }, sorted by path. Anything but plain files and folders goes to
// `fail`: a refusal (exit 2) before any work; on pack's second walk, a mismatch (exit 1).
function walk(dir, set, fail = refuse) {
  const files = [];
  const visit = (rel) => {
    for (const name of readdirSync(join(dir, rel))) {
      const path = rel ? `${rel}/${name}` : name;
      if (!PATH_OK.test(path) || `${set}/${path}`.length > 99) {
        fail(`${set}/${path}: not a plain name of at most 99 characters`);
      }
      const st = lstatSync(join(dir, path));
      if (st.isDirectory()) visit(path);
      else if (st.isFile() && st.size < 8 ** 11) files.push({ path, bytes: st.size });
      else fail(`${set}/${path} is not a plain file under 8 GiB (a symbolic link, a device or a socket stays)`);
    }
  };
  visit('');
  if (!files.length) fail(`${set} holds no files`);
  return files.sort((a, b) => (a.path < b.path ? -1 : a.path > b.path ? 1 : 0));
}
// True when a walk's files or sizes are not exactly the list's.
const differs = (files, rows) => files.length !== rows.length ||
  files.some((f, i) => f.path !== rows[i].path || f.bytes !== rows[i].bytes);
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
  if (differs(now, rows)) {
    refuse(`the folder's files or sizes differ from ${listRel}: the folder stays; tell Mustafa (CHECKPOINTS.md §11)`);
  }
  mkdirSync(join(top, ARCHIVES), { recursive: true });
  const archive = join(realDir(top, ARCHIVES), `${set}.tar.gz`);
  const part = `${archive}.part`;
  if (exists(archive)) refuse(`${ARCHIVES}/${set}.tar.gz exists: an archive is never overwritten`);
  if (exists(part)) {
    refuse(`${ARCHIVES}/${set}.tar.gz.part exists: an earlier pack did not finish, and it stays: tell Mustafa`);
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
    // The second walk, the last check before the folder goes: it must still hold exactly the list's files and sizes.
    // Anything else is a mismatch: the folder and the unfinished archive stay, and the script exits 1.
    if (differs(walk(dir, set, (msg) => { throw new Mismatch(msg); }), rows)) {
      throw new Mismatch(`the folder changed while it was packed: its files or sizes now differ from ${listRel}`);
    }
    renameSync(part, archive);
  } catch (e) {
    process.stderr.write(`kept-logs: ${e instanceof Mismatch ? 'MISMATCH' : 'failed'}: ${e?.message ?? e}\n` +
      `kept-logs: the folder stays (${RUNS}/${set}), and so does any unfinished archive ` +
      `(${ARCHIVES}/${set}.tar.gz.part); tell Mustafa (CHECKPOINTS.md §11)\n`);
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

// The newest modification time of a folder and everything in it, never following a symbolic link.
const newest = (dir) => readdirSync(dir).reduce((max, name) => {
  const st = lstatSync(join(dir, name));
  return Math.max(max, st.isDirectory() ? newest(join(dir, name)) : st.mtimeMs);
}, lstatSync(dir).mtimeMs);

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
  if (newest(dir) >= OLD_BEFORE) {
    refuse(`${CACHE} holds something changed after 2026-11-15 07:20 UTC: not the one old build cache; it stays`);
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
