#!/usr/bin/env node
// Checkpoint saver (.claude/CHECKPOINTS.md): commits and pushes ONLY .checkpoints/, on the current cs/ or claude/ branch.
//   node .claude/scripts/checkpoint.mjs save  <slug> -m <subject> [--trailer <line>]...
//   node .claude/scripts/checkpoint.mjs watch <slug> [--every <minutes>] [--for <minutes>] [--trailer <line>]...
// save:  stage .checkpoints/, commit only that path (anything else already staged stays staged), push the branch.
// watch: for long agent runs, run in the background: every N minutes (default 10) save if .checkpoints/ changed;
//        stops after --for minutes (default 110), when the branch changes, or on SIGINT/SIGTERM (after a last save).
//        Minutes may be fractions (the tests use 0.01). Stop it before the pull request is marked ready.
// Guards: only ^(cs|claude)/... branches (never main, never a detached head); git runs without a shell; the branch is
// re-read right before each push; never a force push; a refused push is reported and left alone.
// A commit gets "[skip ci]" only when every commit it pushes touches .checkpoints/ alone; the final commit of a
// change-set (which deletes the checkpoint) always runs CI. Run from the repository root. Node 22, no dependencies.
// Exit codes: 0 saved or nothing to save · 1 a git step failed (commit or push) · 2 usage error or refused by a guard.
import { execFileSync } from 'node:child_process';

const DIR = '.checkpoints/';
const BRANCH_OK = /^(cs|claude)\/[A-Za-z0-9._-]+(\/[A-Za-z0-9._-]+)*$/;
const SLUG_OK = /^[a-z0-9][a-z0-9-]{0,63}$/;
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const log = (msg) => process.stdout.write(`${new Date().toISOString()} checkpoint: ${msg}\n`);
const fail = (code, msg) => { process.stderr.write(`checkpoint: ${msg}\n`); process.exit(code); };
const git = (...args) => {
  try {
    const out = execFileSync('git', args, { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'], shell: false });
    return { ok: true, out: out.trim(), err: '' };
  } catch (e) {
    return { ok: false, out: String(e.stdout || '').trim(), err: String(e.stderr || e.message || '').trim() };
  }
};
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
  for (const t of [o.subject || '', ...o.trailers]) if (/[\r\n]/.test(t)) fail(2, 'subject and trailers are one line each');
  if (!(o.every > 0) || !(o.for > 0)) fail(2, '--every and --for take a positive number of minutes');
  return o;
}

const branchAllowed = (b) => BRANCH_OK.test(b) && !b.includes('..');

function branchOrRefuse() {
  if (git('rev-parse', '--is-inside-work-tree').out !== 'true') fail(2, 'not inside a git working tree');
  if (git('rev-parse', '--show-prefix').out !== '') fail(2, 'run from the repository root');
  const b = currentBranch();
  if (!b.ok) fail(2, 'refused: detached HEAD');
  if (!branchAllowed(b.out)) fail(2, `refused: branch "${b.out}" is not a cs/ or claude/ branch`);
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

async function save(o, branch, subject) {
  const add = await withLockRetry(() => git('add', '-A', '--', DIR));
  if (!add.ok) { log(`git add failed: ${add.err}`); return 1; }
  if (git('diff', '--cached', '--quiet', '--', DIR).ok) { log('nothing to save'); return 0; }
  const skip = onlyCheckpointsAhead(branch);
  const msg = ['-m', `checkpoint(${o.slug}): ${subject}${skip ? ' [skip ci]' : ''}`];
  if (o.trailers.length) msg.push('-m', o.trailers.join('\n'));
  const commit = await withLockRetry(() => git('commit', '--quiet', '--only', ...msg, '--', DIR));
  if (!commit.ok) { log(`git commit failed: ${commit.err}`); return 1; }
  const sha = git('rev-parse', '--short', 'HEAD').out;
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
    if (currentBranch().out === branch) await save(o, branch, `autosave (${why})`);
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
    if (git('status', '--porcelain', '--', DIR).out) await save(o, branch, 'autosave');
    if (Date.now() >= end) await last('time is up');
  }
}

const o = parse(process.argv.slice(2));
const branch = branchOrRefuse();
if (o.cmd === 'save') process.exit(await save(o, branch, o.subject));
else await watch(o, branch);
