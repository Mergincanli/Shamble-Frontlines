#!/usr/bin/env node
// Checkpoint saver (.claude/CHECKPOINTS.md): commits ONLY .checkpoints/ on the current cs/ or claude/ branch and pushes
// that branch (every local commit on it; never main, never forced).
//   node .claude/scripts/checkpoint.mjs save  <slug> -m <subject> [--trailer <line>]...
//   node .claude/scripts/checkpoint.mjs watch <slug> [--every <minutes>] [--for <minutes>] [--trailer <line>]...
// save:  stage .checkpoints/, commit only that path (anything else already staged stays staged), push the branch.
// watch: for long agent runs, run in the background with the Bash timeout at its maximum: every N minutes (default 10,
//        at least 0.01) save if .checkpoints/ changed; stops after --for minutes (default 110, at most 115), when the
//        branch changes, when the checkpoint file is gone, or on SIGINT/SIGTERM (after a last save). watch never
//        publishes code: when the branch holds unpushed commits outside .checkpoints/, it commits locally and skips
//        the push.
// Guards: only ^(cs|claude)/... branches (never main, never a detached head); .checkpoints/<slug>.md must exist (a
// finished task's last commit, which deletes it, is made by hand so CI runs); git runs without a shell; the branch is
// re-read right before each push; never a force push; a refused push is reported and left alone; staged notes that
// look like a secret (tokens, keys, credentials in URLs) are not committed. The script alone decides "[skip ci]": only
// when every commit it pushes touches .checkpoints/ alone; skip markers from the caller are refused.
// Run from the repository root. Node 22, no dependencies.
// Exit codes: 0 saved or nothing to save · 1 a git step failed or a secret was found · 2 usage error or refused by a guard.
import { execFileSync } from 'node:child_process';
import { existsSync, rmSync } from 'node:fs';
import { resolve } from 'node:path';

const DIR = '.checkpoints/';
const BRANCH_OK = /^(cs|claude)\/[A-Za-z0-9._-]+(\/[A-Za-z0-9._-]+)*$/;
const SLUG_OK = /^[a-z0-9][a-z0-9-]{0,63}$/;
const SKIP_MARK = /\[(skip ci|ci skip|no ci|skip actions|actions skip)\]|^\s*skip-checks\s*:/im;
const SECRETS = [
  ['GitHub token', /\b(gh[pousr]_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{20,})/],
  ['Anthropic key', /\bsk-ant-[A-Za-z0-9_-]{20,}/],
  ['API key', /\bsk-[A-Za-z0-9]{32,}/],
  ['AWS key', /\b(AKIA|ASIA)[0-9A-Z]{16}\b/],
  ['private key', /-----BEGIN [A-Z ]*PRIVATE KEY-----/],
  ['Slack token', /\bxox[abprs]-[A-Za-z0-9-]{10,}/],
  ['credentials in a URL', /[a-z][a-z0-9+.-]*:\/\/[^\s/:@]+:[^\s/@]+@/i],
];
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const log = (msg) => process.stdout.write(`${new Date().toISOString()} checkpoint: ${msg}\n`);
const fail = (code, msg) => { process.stderr.write(`checkpoint: ${msg}\n`); process.exit(code); };
const run = (env, args) => {
  try {
    const out = execFileSync('git', args, { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'], shell: false, maxBuffer: 64 << 20, env });
    return { ok: true, out: out.trim(), err: '' };
  } catch (e) {
    return { ok: false, out: String(e.stdout || '').trim(), err: String(e.stderr || e.message || '').trim() };
  }
};
const git = (...args) => run(process.env, args);
const currentBranch = () => git('symbolic-ref', '--quiet', '--short', 'HEAD');

function parse(argv) {
  const [cmd, slug, ...rest] = argv;
  const o = { cmd, slug, subject: null, trailers: [], every: 10, for: 110 };
  for (let i = 0; i < rest.length; i += 2) {
    const k = rest[i], v = rest[i + 1];
    if (v === undefined) fail(2, `missing value for ${k}`);
    if (k === '-m') o.subject = v;
    else if (k === '--trailer') o.trailers.push(v);
    else if (k === '--every') o.every = Number(v);
    else if (k === '--for') o.for = Number(v);
    else fail(2, `unknown option ${k}`);
  }
  if (cmd !== 'save' && cmd !== 'watch') fail(2, 'usage: checkpoint.mjs save|watch <slug> [options] (see the header)');
  if (!SLUG_OK.test(slug || '')) fail(2, `bad slug "${slug}": lowercase letters, digits and dashes`);
  if (cmd === 'save' && !o.subject) fail(2, 'save needs -m <subject>');
  for (const t of [o.subject || '', ...o.trailers]) {
    if (/[\r\n]/.test(t)) fail(2, 'subject and trailers are one line each');
    if (SKIP_MARK.test(t)) fail(2, 'refused: the script alone decides [skip ci]; drop the skip marker');
  }
  if (!(o.every >= 0.01) || !(o.for > 0 && o.for <= 115)) fail(2, '--every is at least 0.01 minutes; --for is above 0 and at most 115');
  return o;
}

const branchAllowed = (b) => BRANCH_OK.test(b) && !b.includes('..');
const checkpointFile = (o) => `${DIR}${o.slug}.md`;

function branchOrRefuse(o) {
  if (git('rev-parse', '--is-inside-work-tree').out !== 'true') fail(2, 'not inside a git working tree');
  if (git('rev-parse', '--show-prefix').out !== '') fail(2, 'run from the repository root');
  const b = currentBranch();
  if (!b.ok) fail(2, 'refused: detached HEAD');
  if (!branchAllowed(b.out)) fail(2, `refused: branch "${b.out}" is not a cs/ or claude/ branch`);
  if (!existsSync(checkpointFile(o))) fail(2, `refused: ${checkpointFile(o)} does not exist (task finished or not started; make a finished task's last commit by hand so CI runs)`);
  return b.out;
}

// Every commit the push would send touches only .checkpoints/ (so CI may be skipped for it).
function onlyCheckpointsAhead(branch) {
  const remote = git('rev-parse', '--verify', '--quiet', `refs/remotes/origin/${branch}`);
  if (!remote.ok) return false;
  const ahead = git('rev-list', `${remote.out}..HEAD`);
  if (!ahead.ok) return false;
  for (const c of ahead.out.split('\n').filter(Boolean)) {
    const files = git('diff-tree', '--no-commit-id', '--name-only', '-r', '--root', c);
    if (!files.ok || files.out.split('\n').some((f) => f && !f.startsWith(DIR))) return false;
  }
  return true;
}

// The notes staged in a private index that look like a secret: [path, kind] pairs. Names are read NUL-separated (never
// quoted); a note that can't be read counts as a hit. The secret itself is never printed.
function stagedSecrets(ig, head) {
  const hits = [];
  const names = ig('diff', '--cached', '-z', '--name-only', '--no-renames', '--diff-filter=d', head, '--', DIR);
  if (!names.ok) return [[DIR, 'list that could not be read']];
  for (const f of names.out.split('\0').filter(Boolean)) {
    const blob = ig('show', `:${f}`);
    if (!blob.ok) { hits.push([f, 'note that could not be read']); continue; }
    for (const [kind, re] of SECRETS) if (re.test(blob.out)) hits.push([f, kind]);
  }
  return hits;
}

async function withLockRetry(fn) {
  for (let i = 0; ; i++) {
    const r = fn();
    if (r.ok || !/index\.lock/.test(r.err) || i >= 5) return r;
    await sleep(2000);
  }
}

async function push(branch) {
  const waits = [2, 4, 8, 16];
  for (let i = 0; ; i++) {
    const now = currentBranch();
    if (now.out !== branch || !branchAllowed(now.out)) { log(`push skipped: the branch changed (${now.out || 'detached'})`); return false; }
    const r = git('push', 'origin', `HEAD:refs/heads/${branch}`);
    if (r.ok) return true;
    if (/rejected|non-fast-forward|fetch first|protected/i.test(r.err)) {
      log(`push refused, left alone (never forced): ${r.err.split('\n').pop()}`);
      return false;
    }
    if (i >= waits.length) { log(`push failed after ${i + 1} tries: ${r.err.split('\n').pop()}`); return false; }
    await sleep(waits[i] * 1000);
  }
}

// Commits exactly what was scanned: the notes are staged in a private index (HEAD + .checkpoints/), scanned there, and
// that index's tree becomes the commit. The session's own index is never staged into; only its .checkpoints/ entries
// are synced to the new HEAD afterwards, so anything else it staged stays staged.
async function save(o, branch, subject, background) {
  if (!existsSync(checkpointFile(o))) { log(`not saved: ${checkpointFile(o)} is gone`); return 2; }
  const head = git('rev-parse', 'HEAD');
  if (!head.ok) { log(`no HEAD: ${head.err}`); return 1; }
  const tmpIndex = resolve(git('rev-parse', '--git-dir').out, `checkpoint-index-${process.pid}`);
  const ig = (...args) => run({ ...process.env, GIT_INDEX_FILE: tmpIndex }, args);
  let commit, codeAhead;
  try {
    if (!ig('read-tree', head.out).ok) { log('could not read HEAD into a private index'); return 1; }
    const add = ig('add', '-A', '--', DIR);
    if (!add.ok) { log(`git add failed: ${add.err}`); return 1; }
    if (ig('diff', '--cached', '--quiet', head.out, '--', DIR).ok) { log('nothing to save'); return 0; }
    const secrets = stagedSecrets(ig, head.out);
    if (secrets.length) {
      for (const [f, kind] of secrets) log(`NOT committed: ${f} looks like a ${kind}; remove or fix it, then save again`);
      return 1;
    }
    const tree = ig('write-tree');
    if (!tree.ok) { log(`git write-tree failed: ${tree.err}`); return 1; }
    codeAhead = !onlyCheckpointsAhead(branch);
    const msg = ['-m', `checkpoint(${o.slug}): ${subject}${codeAhead ? '' : ' [skip ci]'}`];
    if (o.trailers.length) msg.push('-m', o.trailers.join('\n'));
    commit = git('commit-tree', tree.out, '-p', head.out, ...msg);
    if (!commit.ok) { log(`git commit-tree failed: ${commit.err}`); return 1; }
    const moved = await withLockRetry(() => git('update-ref', '-m', `checkpoint(${o.slug})`, `refs/heads/${branch}`, commit.out, head.out));
    if (!moved.ok) { log(`not saved: ${branch} moved while saving (${moved.err.split('\n').pop()})`); return 1; }
  } finally {
    rmSync(tmpIndex, { force: true });
  }
  const sync = await withLockRetry(() => git('reset', '--quiet', 'HEAD', '--', DIR));
  if (!sync.ok) log(`WARNING: the index's ${DIR} entries are behind HEAD; run: git reset -q HEAD -- ${DIR}`);
  const sha = commit.out.slice(0, 7);
  if (background && codeAhead) {
    log(`saved locally, NOT pushed ${sha} on ${branch}: the branch has unpushed commits outside ${DIR} (push them yourself)`);
    return 0;
  }
  const pushed = await push(branch);
  log(`${pushed ? 'saved and pushed' : 'saved locally, NOT pushed'} ${sha} on ${branch}`);
  return pushed ? 0 : 1;
}

async function watch(o, branch) {
  const end = Date.now() + o.for * 60000;
  let stopping = false;
  const last = async (why) => {
    if (stopping) return;
    stopping = true;
    if (currentBranch().out === branch && existsSync(checkpointFile(o))) await save(o, branch, `autosave (${why})`, true);
    log(`watch stopped: ${why}`);
    process.exit(0);
  };
  process.on('SIGINT', () => last('interrupted'));
  process.on('SIGTERM', () => last('terminated'));
  log(`watching ${DIR} on ${branch} every ${o.every} min for ${o.for} min`);
  while (!stopping) {
    await sleep(Math.min(o.every * 60000, Math.max(0, end - Date.now())));
    if (stopping) return;
    const now = currentBranch();
    if (now.out !== branch) { stopping = true; log(`watch stopped: the branch changed (${now.out || 'detached'})`); process.exit(0); }
    if (!existsSync(checkpointFile(o))) { stopping = true; log(`watch stopped: ${checkpointFile(o)} is gone (task finished)`); process.exit(0); }
    if (git('status', '--porcelain', '--', DIR).out) await save(o, branch, 'autosave', true);
    if (Date.now() >= end) await last('time is up');
  }
}

const o = parse(process.argv.slice(2));
const branch = branchOrRefuse(o);
if (o.cmd === 'save') process.exit(await save(o, branch, o.subject, false));
else await watch(o, branch);
