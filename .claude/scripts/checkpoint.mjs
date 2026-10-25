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
import { existsSync } from 'node:fs';

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
const git = (...args) => {
  try {
    const out = execFileSync('git', args, { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'], shell: false, maxBuffer: 64 << 20 });
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

// The staged notes that look like a secret: [path, kind] pairs. The secret itself is never printed.
function stagedSecrets() {
  const hits = [];
  const names = git('diff', '--cached', '--name-only', '--diff-filter=AM', '--', DIR).out.split('\n').filter(Boolean);
  for (const f of names) {
    const blob = git('show', `:${f}`);
    if (!blob.ok) continue;
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

async function save(o, branch, subject, background) {
  if (!existsSync(checkpointFile(o))) { log(`not saved: ${checkpointFile(o)} is gone`); return 2; }
  const add = await withLockRetry(() => git('add', '-A', '--', DIR));
  if (!add.ok) { log(`git add failed: ${add.err}`); return 1; }
  if (git('diff', '--cached', '--quiet', '--', DIR).ok) { log('nothing to save'); return 0; }
  const secrets = stagedSecrets();
  if (secrets.length) {
    git('reset', '--quiet', '--', DIR);
    for (const [f, kind] of secrets) log(`NOT committed: ${f} looks like it holds a ${kind}; remove it, then save again`);
    return 1;
  }
  const codeAhead = !onlyCheckpointsAhead(branch);
  const msg = ['-m', `checkpoint(${o.slug}): ${subject}${codeAhead ? '' : ' [skip ci]'}`];
  if (o.trailers.length) msg.push('-m', o.trailers.join('\n'));
  const commit = await withLockRetry(() => git('commit', '--quiet', '--only', ...msg, '--', DIR));
  if (!commit.ok) { log(`git commit failed: ${commit.err}`); return 1; }
  const sha = git('rev-parse', '--short', 'HEAD').out;
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
