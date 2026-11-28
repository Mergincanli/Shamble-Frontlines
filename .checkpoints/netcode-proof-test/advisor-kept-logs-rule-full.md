# Advisor: the plan review of the KEPT-LOGS kit rule's drafted lines (workflow wf_0f69bce5-688), 2026-11-15

Written by script, word for word, from the workflow's journal. The draft: kept-logs-rule-draft.md; the advisor's notes: advisor-kept-logs-rule.md. Nothing below is the main session's.

## Verdict

APPROVE WITH CONDITIONS

## Summary

I checked the KEPT-LOGS kit-rule draft against Mustafa's words of 2026-11-15, 08:20 (history.md:2088-2111), using advisor.md from origin/main (4460ef5). Verdict: APPROVE WITH CONDITIONS. All the conditions are already written into the final text (notes: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-kept-logs-rule.md, blocks between BEGIN/END markers).

The shape is right. There is one appended section, CHECKPOINTS.md §11, and no existing kit line changes. The rest is one new script, .claude/scripts/kept-logs.mjs, plus its tests. It makes no change to the guard, settings.json, CLAUDE.md or any agent file, so it stays clear of step 2.3. Step 2.3 (origin/claude/sweet-mayer-grsukw at 5f7f682) edits CHECKPOINTS.md only at lines 128 and 219-224, with unchanged lines 225-229 in between. His three steps and the stay-and-tell rule are all present: the list is committed and pushed; the folder is packed into one .tar.gz, every file is read back and hashed against the list, and the folder goes only if all of them match; free space is printed before and after.

Facts I verified:
- the guard does not inspect node scripts in day sessions (routine-guard.mjs:767-770);
- both routines carry SHAMBLE_ROLE, so the script's own refusal of routines holds;
- no allow line in settings.json covers the script, and the `rm -rf` deny stays;
- all 10 set folders, 195 files and 11,684 MiB pass SET_OK and PATH_OK; the longest name is 63 characters;
- the runner refuses to write into an existing set (c-u4r-runs.mjs:419);
- redact-check does not flag hex hashes;
- the cache is 4,051 MiB and nothing in it is newer than 2026-11-08 22:40 UTC.

Findings:
- F1 Medium: the draft deleted its own unfinished .part archive, which is a third thing outside his two items. On a mismatch it also destroyed the evidence he asked to be told about. Fix: the .part always stays.
- F2 Medium: `cache` would remain a standing delete of any later folder at that path, but his item is "the one old build cache". Fix: it refuses anything changed after 2026-11-15 07:20 UTC. I ran the same code read-only on the real cache: 17,873 entries, newest 2026-11-08T22:40Z, passes, 77 ms.
- F3 Low: a folder that no longer matches its list now says "tell Mustafa".
- F4 Low: the change to the existing CHECKPOINTS row in REPO_MAP is dropped.
- F5 Low: §11 now names the netcode folder, and the ruling and free space reach the report.
- F6 Low: KIT-FREEZE-2 becomes a statement his yes confirms, not a re-ask. His own 08:20 words order this kit PR.

Final size, measured: §11 is 18 lines, the script 313, the tests 292 (widest 120), plus 2 REPO_MAP rows. Both code blocks pass `node --check`. A diff against the draft's blocks shows only the condition edits.

Risk tier is 2 (kit). Removing a raw-log folder is reversible, because the hash-checked archive holds every byte. Removing the cache is not reversible, but it is rebuildable build output and his words name it. So his decision of the act is in, and no extra tier-3 step is needed.

Not run here, because they write files: the test suite, the 11 mutation runs and the dry run. They belong to the build step (C7).

My slip: I wrote one probe file by shell redirect into the session scratchpad (outside the repo, one line) before re-reading my limits. Nothing in the repository changed except my notes file. The session's background saver appears to have committed an early version of that notes file: git shows it as tracked and modified.

Answer to his last line (5e563b95): it was covered by the advisor's carry-62 (h) review (advisor-u4r-c62h C1, C14), after the r2-run ruling that ordered it, and by check.mjs (all 11 steps). No reviewer seat reviewed it in a round. The session's task list shows a test-engineer read of it in progress (C62H-REVIEW), so the session must sync that paragraph before sending.

## Conditions

All are already applied in the final blocks of /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-kept-logs-rule.md (markers checkpoints-11, kept-logs.mjs, kept-logs.test.mjs, repo-map-rows, final-message). The session pastes those, not the draft's.

C1 (F1, Medium): no .part is ever removed.
- In pack, the stale-.part block becomes:
  `if (exists(part)) { refuse(`${ARCHIVES}/${set}.tar.gz.part exists: an earlier pack did not finish, and it stays: tell Mustafa`); }`
- The catch drops `rmSync(part, { force: true });` and writes:
  "kept-logs: the folder stays (${RUNS}/${set}), and so does any unfinished archive (${ARCHIVES}/${set}.tar.gz.part); tell Mustafa (CHECKPOINTS.md §11)"
- The header says the archive "stays", and "no archive, finished or not".
- Test 4 asserts that the .part stays, and that a second pack is refused (/did not finish/).
- New mutations: M9 (the stale-.part refusal removed) and M10 (the catch removes the .part). Each must fail test 4.

C2 (F2, Medium): `cache` removes only the one old cache.
- Add `const OLD_BEFORE = Date.parse('2026-11-15T07:20:00Z');`.
- Add `newest(dir)`, an lstat walk that never follows a link.
- As the last check before the removal: `if (newest(dir) >= OLD_BEFORE) { refuse(`${CACHE} holds something changed after 2026-11-15 07:20 UTC: not the one old build cache; it stays`); }`
- Test 7 refuses a freshly planted cache, then removes it after an `age()` helper sets every entry to 2026-11-08 22:40 UTC with utimesSync. The test imports gain lstatSync, readdirSync and utimesSync.
- Mutation M11 (the check removed) must fail test 7.
- Read-only build evidence: `find .gate/u3c-review-target -newermt '2026-11-15 07:20 UTC' | head -1` prints nothing.

C3 (F3, Low): the pre-pack mismatch refusal reads:
- "the folder's files or sizes differ from ${listRel}: the folder stays; tell Mustafa (CHECKPOINTS.md §11)".
- Test 4 matches /differ from[\s\S]*tell Mustafa/.
- §11 reads "If one differs, or the folder no longer matches its list, the folder stays: tell Mustafa ...".

C4 (F4, Low): REPO_MAP gets the two new rows only, and the CHECKPOINTS row is left as it is.

C5 (F5, Low): §11 is the final 18-line text. It names `.gate/netcode-proof-test/runs/`, the cache's date bound, and "or the folder no longer matches its list"; the ruling goes into the checkpoint with the free-space lines and on into the report; "The archives stay, finished or not".

C6 (F6, the message): the final text is what goes to Mustafa.
- KIT-FREEZE-2 is one statement that his yes confirms.
- It says plainly that the tests pack and delete copies only, in a temp folder.
- It lists the three narrowings so he can strike them.
- It names 5e563b95 and the review that covered it. Before sending, check that paragraph against the session's C62H-REVIEW task and update it if that read has reported.

C7 (build evidence, after his yes):
- Work in `git worktree add .gate/kept-logs-wt -b <branch> origin/main`. Never switch the main folder. The `.claude/` writes ask him first, in Accept edits mode.
- Run the draft's (c) steps 1-6 with the mutation table M1-M11, recording each mutation's failing test names.
- The kit tests' pass counts must be equal on main 4460ef5 and on the PR head: routine-guard.test.mjs, settings.test.mjs, checkpoint.test.mjs.
- repo-map-check and `node tools/gate.mjs` must PASS.
- Extract the three kit files by script from the text he said yes to, compare them byte for byte (cmp), and measure their line counts.
- The dry run's real-folder fingerprints must be equal before and after.

C8 (the PR report): it carries the tier-2 table.
- No existing rule sentence changes.
- §11 is looser than §10's "`rm -rf` stays denied" for exactly his two items, with his 2026-11-15 words quoted.
- §11 is narrower than his words in three places: nothing more is packed after a mismatch; the unfinished archive stays; the cache has a date bound.

C9 (reviews and merge):
- security-auditor and test-engineer review, in one round of one batch of two. The advisor reviews every pushed head.
- If review changes a kit line, the merge message shows him the before and after.
- It merges from a day session on his go for that head. Nothing is packed or deleted until the PR is on main and the netcode branch has taken main in.
- The list commits later are data on PR #40's branch, in their own commits, outside the bot fix's line count.

Decided here, as technical choices inside his words:
- KL-1 (a): the list goes beside the set's results.
- KL-2 (a): no allow line in settings.json.
- KL-3 (a): kept, and shown to him as a narrowing.
- KL-4: a statement, not a question.
- KL-5 (a): docs-keeper records his words in its next pass.

## The final text for Mustafa

**KEPT-LOGS: every kit line, for your yes.** Nothing is built, packed or deleted yet.

This is the narrow kit rule of your words of 2026-11-15, 08:20. It covers your two items and nothing else:
- the raw-log folder of a run set whose results are committed and whose ruling is closed;
- the one old build cache, `.gate/u3c-review-target`.

Where it goes: one new section at the end of `.claude/CHECKPOINTS.md` (§11, 18 lines, no existing line changed) and one
new script (313 lines) with its tests (292 lines). The rule itself is short. The script makes "nothing else" and
"every hash read back" hold by code, not by care. No change to the guard, `settings.json`, `CLAUDE.md` or any agent
file, so it stays clear of step 2.3's guard change. `rm -rf` stays denied.

Your three steps, in the script's order (`list`, then `pack`):
1. `list` writes the folder's list (sha256, size, path per file) beside the set's committed results. I commit it,
   naming the closed ruling, and push it.
2. `pack` refuses unless that list is committed and pushed and the folder still matches it. It packs the folder into
   one `.tar.gz`, reads every file back out of that archive, hashes it, and compares it with the list.
3. It prints free space before and after; I copy both lines into the checkpoint, and from there into the report.

If one hash differs, the folder stays, the unfinished archive stays too (for you to look at), and I tell you.

Three places where the rule is narrower than your words (so you can strike them):
- after a mismatch, nothing more is packed until you answer;
- an unfinished archive is never deleted;
- the cache is removed only while nothing in it is newer than your words of 08:20; a later folder at that path stays.

KIT-FREEZE-2: this is a new rule during the freeze. I read your words of 08:20 ("It is its own reviewed kit pull
request to main") as your exception for it; your yes to these lines confirms that.

How it is proven, after your yes:
- its tests: 8, plus a dry run;
- 11 planted bugs, each of which must make a test fail;
- the dry run on copies of two real run-set folders in a temp folder: the tests pack and delete those copies only, and
  each real folder must be unchanged afterwards (its fingerprint, before and after);
- the kit's own tests with the same counts as on main; the gate.

Reviews: the advisor (this plan, then every pushed head), security-auditor and test-engineer. Then I stop for your go on
the merge. Nothing real is packed or deleted before the pull request is on main and this branch has taken main in.

Your last line: the pass-bar file's change of 2026-11-14, 16:14 UTC, was commit 5e563b95 ("carry 62 (h)"). It added one
check and one planted bug (whole-as-lost) to PM-0001-17 in `marks.json`. Two things covered it:
- the advisor's carry-62 (h) review (advisor-u4r-c62h: "right as committed"), after the r2-run ruling that ordered it;
- `check.mjs`, all 11 steps.
No reviewer seat reviewed it in a round. A test-engineer read of those lines is under way now; its result comes in its
own report.

### 1. `.claude/CHECKPOINTS.md`: appended after line 229, one blank line before it

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
- A raw-log folder (the netcode proof test's, under `.gate/netcode-proof-test/runs/`), one at a time: `list` writes
  its list (each file's sha256, size and path) beside the set's committed results; commit it, naming the set's closed
  ruling (file and line) in the message, and push it. `pack` then packs the folder into one compressed file under
  `.gate/netcode-proof-test/kept-logs/`, reads every file's hash back from it, and removes the folder only if every
  hash equals the list. If one differs, or the folder no longer matches its list, the folder stays: tell Mustafa, and
  pack nothing more until he answers.
- The cache: `cache` removes it only while nothing in it is newer than his words (2026-11-15 07:20 UTC); a later
  folder at that path is not the old cache and stays.
- Free space before and after, as the script prints it, goes into the checkpoint with the folder's ruling, and from
  there into the report. The archives stay, finished or not: nothing here removes one.
```
<!-- END checkpoints-11 -->

### 2. `.claude/scripts/kept-logs.mjs`: a new file, whole

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
//        the folder stays, and so does the unfinished archive (<archive>.part), and the script exits 1: tell Mustafa.
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
```
<!-- END kept-logs.mjs -->

### 3. `.claude/scripts/kept-logs.test.mjs`: a new file, whole

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
```
<!-- END kept-logs.test.mjs -->

### Not kit, in the same pull request: two new rows in `docs/REPO_MAP.md`, after the `checkpoint.test.mjs` row

<!-- BEGIN repo-map-rows -->
```
| `.claude/scripts/kept-logs.mjs` | KEPT-LOGS (`CHECKPOINTS.md` §11): `list` writes a closed run set's raw-log list beside its results; `pack` packs the folder, reads every file's hash back and removes the folder only if all equal the committed list; `cache` removes the one old build cache, `.gate/u3c-review-target`; it refuses everything else |
| `.claude/scripts/kept-logs.test.mjs` | Its tests: `node --test .claude/scripts/kept-logs.test.mjs`; `KEPT_LOGS_DRY_RUN=<a real run-set folder>` adds the dry run on a copy |
```
<!-- END repo-map-rows -->

Not in this pull request: `settings.json`, the guard, `CLAUDE.md`, the agent files, `docs/DECISIONS.md` (docs-keeper
records these words in its next pass; §11 quotes your scope itself).

**Decisions needed:**
- **KL-YES: do these kit lines go in exactly as shown?** Why it matters: you asked to see every kit line and to say yes
  before anything is built. Options: (a) yes, as shown; (b) yes, with changes you name (for example, strike one of the
  three narrowings above); (c) no. Recommendation: (a). The advisor approved them with conditions, and all of those
  conditions are already in the text above.

**For Mustafa:**
1. Read the three kit blocks (§11, the script, the tests) and the two map rows.
2. Say "yes" (or what to change) in this chat. Nothing is built before that, and nothing is packed or deleted before
   the pull request is on main and this branch has taken main in.

## The PR's reviewers

This is risk tier 2, because it changes kit files: .claude/CHECKPOINTS.md and .claude/scripts/. docs/REPO_MAP.md rides along.

Reviewers:
- security-auditor: required at risk tier 2, since this is a new delete path in the kit and a script the guard does not inspect;
- test-engineer: for the proof, meaning the 8 tests plus the dry run, mutations M1-M11 each biting, and the dry run's real-folder fingerprints.

They run as one round, in one batch of two agents (Opus 5.5, high). The advisor reviewed this plan and reviews every pushed head. A day-session advisor runs the gate and the kit tests at the same counts as main.

It is needs-mustafa. It merges from a day session on his go for the reviewed head, with the gate and smoke green. If review changes a kit line, the merge message shows him the before and after.

No art-director, perf, netcode, engine or graphics review is needed: no game code, runtime or rendering is touched.

## For Mustafa

Now: the session shows him final_text_for_mustafa, whole, in chat, and waits for his yes. That is the KEPT-LOGS lines: §11, the script, the tests, plus the two REPO_MAP rows marked as not kit. The canonical copy is between the "BEGIN final-message" and "END final-message" markers in /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/advisor-kept-logs-rule.md. Extract it by script rather than retyping it, and build the PR's kit files byte for byte from the blocks he says yes to.

Before sending, sync the 5e563b95 paragraph with the session's own C62H-REVIEW task. The task list shows a test-engineer read of those lines in progress; if it has reported, state its result instead of "under way".

After his yes: build per C7 in a worktree from origin/main, run the reviews per C9, and stop for his merge go. Nothing real is packed or deleted until the PR is on main and the netcode branch has taken main in.

## The drafter's result

The draft is ready: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/kept-logs-rule-draft.md (1,036 lines, ends with DONE). It is the only file I changed: `git status` shows just that one untracked file. Nothing is committed, and no kit file was edited.

**What the rule changes.** It touches one place and avoids the guard, `settings.json` and `CLAUDE.md`:
- **`.claude/CHECKPOINTS.md`:** a new §11 of 15 lines, added after line 229. No existing line changes.
  - It quotes Mustafa's two scope items. A check confirmed the quote equals history.md:2095-2097 byte for byte once line breaks are joined.
  - It says only the new script removes either thing, and `rm -rf` stays denied.
  - It says who may run it: the main session of a day session, never while a run set or a build runs.
  - It gives his steps in his order: the list is committed and pushed, the folder is packed, every hash is read back, and the folder goes only if every hash is equal. If one differs, the folder stays and he is told. Free space before and after goes into the checkpoint.
- **`.claude/scripts/kept-logs.mjs`:** a new script, 304 lines, Node only, with no dependencies and no shell `rm`.
  - `list <set> --results <dir>` writes `raw-logs.sha256` (sha256, size and path per file) beside the set's committed results.
  - `pack <set> --results <dir>` needs that list on disk, in HEAD and on the pushed branch, all equal, and the folder must still match it file for file. It writes a `.tar.gz` and reads every file back from it. Only when path, size and sha256 all match does it remove the folder. On any mismatch it exits 1, the folder stays, and its unfinished archive is removed.
  - `cache` removes `.gate/u3c-review-target`.
  - It refuses a routine, a linked worktree, any symbolic link, odd names, and an existing list or archive. It only reads git; the session commits and pushes.
- **`.claude/scripts/kept-logs.test.mjs`:** a new test file, 278 lines, 9 tests. One of them is an opt-in dry run on a copy of a real run-set folder, which GNU tar unpacks and compares file by file with the real folder.
- **`docs/REPO_MAP.md`** (not a kit file): 2 new rows, which `repo-map-check.mjs` requires, and 1 changed row.

That comes to 600 lines added and 1 changed.

**Why no guard change is needed.**
- In a day session the guard does not inspect `node` scripts (routine-guard.mjs:768-770).
- In a routine the guard would let main's tracked scripts run (routine-guard.mjs:989-1006). So the script refuses when `SHAMBLE_ROLE` is set, and a test covers that.
- With no `settings.json` allow line, every run asks for permission.

**Overlap with step 2.3** (its kit texts at 5f7f682):
- Its CHECKPOINTS.md edits are at lines 219-224 and 128, so no change overlaps §11.
- Its guard and `settings.json` changes are untouched.
- REPO_MAP could conflict in text only, near rows 119-122.

**What I checked, without writing any other file:**
- Both code blocks pass `node --check` read from stdin.
- An in-memory round trip with the script's real `header()` and `readBack()` worked. GNU tar 1.35 listed the archive with no warning and extracted every file to the same sha256.
- `readBack` refused a broken checksum and a missing end, and data that is not gzip gave an error instead of a hang.
- The list text and the set-name pattern behave as intended.
- The quotes in §11 and the line numbers cited in the draft were checked against their sources.

**What I did not run:** the test suite, the 8 mutation runs that show the tests catch faults, and the dry run on two real sets (17 MiB and 291 MiB). All three need files written, so they belong to the build step. The draft lists their commands and expected results.

**Points the draft raises (KL-1 to KL-5):**
- **KIT-FREEZE-2 (KL-4):** docs/DECISIONS.md:33 says the freeze "stops new rules". This is a new rule. I read his 2026-11-15 words as his own exception, and the draft recommends asking him to confirm that in one line, in the same message that shows him the kit lines.
- **KL-1:** put the list beside the set's committed results, so it reaches main with PR #40's squash. In `.checkpoints/` it would be deleted from the final tree.
- **KL-2:** add no allow line, so every run asks for permission.
- **KL-3:** keep my addition "pack nothing more until he answers" after a mismatch. It only narrows his words.
- **KL-5:** add his KEPT-LOGS words to DECISIONS.md in docs-keeper's next pass, not in this PR.

**After the merge.** The draft lists the 10 candidate run-set folders: 11,688 MiB in all, with their results paths. The session must cite each one's closed ruling first, because the script cannot check that.
- Packing them should free about 8.2 GiB. That rests on the advisor's 3.55-times compression, measured on one run.
- The old build cache frees another 4,051 MiB.
- That would take free space from 7,711 MiB to about 20 GiB.

Showing him every kit line in chat means about 600 lines.
