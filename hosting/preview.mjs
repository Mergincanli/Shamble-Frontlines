#!/usr/bin/env node
// Step 2.6 (lane 2): the one script of the preview workflow (.github/workflows/preview.yml), one command per step.
// Node 22's built-ins only, so no job installs anything to run it.
//   find-pr        the open pull request from this repository whose newest commit is the toolchain run's commit
//   check-build    the build that run handed over: the refuse list, then main's header file added
//   recheck        the deploy job's own copy of the checked build: the refuse list again, the header file byte for byte
//   verify-token   asks Cloudflare whether the token is valid (prints its status and dates only) and reads the
//                  project's real address
//   read-url       the link from Wrangler's record, checked against the project's real address
//   check-headers  the served headers of every file of the build, then the bite
//   comment        writes or updates the one link comment on the pull request, from checked values only
//   reconcile      deletes the previews of closed pull requests and the older uploads of open ones (at most 20 a run)
// What it trusts: main's checkout (this file, hosting/_headers, .github/workflows/toolchain.yml) and the runner's own
// variables. Everything else (the event's values, another job's outputs, the build's files, GitHub's and Cloudflare's
// answers) is checked before use, and printed only as JSON in ASCII, so no line of its log can begin with "::" (a
// workflow command). The token is never printed. The plan: step 2.6's plan, revision 2, section 2.8; when something
// fails: docs/runbooks/previews.md.
// Exit codes: 0 done, or nothing to do; 1 refused or failed (the step turns red); 2 usage error.
import * as realFs from 'node:fs';
import { dirname, isAbsolute, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

export const PROJECT = 'shamble-previews';
export const GITHUB_API = 'https://api.github.com';
export const CLOUDFLARE_API = 'https://api.cloudflare.com/client/v4';
export const USER_AGENT = 'shamble-frontlines-preview-workflow/1';
export const MARKER = '<!-- shamble-preview -->';
export const BOT = 'github-actions[bot]';
export const MAX_FILES = 20000;
export const MAX_FILE_BYTES = 25 * 1024 * 1024;
export const MAX_DELETIONS = 20;
export const COUNT_WARNING = 400;
const MAX_PAGES = 10;
const CLOUDFLARE_PAGE = 25;
const CLOUDFLARE_MAX_PAGES = 40;

// Every value from outside is matched whole: JavaScript's `$` never matches before a trailing line break.
export const PATTERNS = {
  pr: /^[1-9][0-9]{0,5}$/,
  commit: /^[0-9a-f]{40}$/,
  repo: /^[A-Za-z0-9][A-Za-z0-9-]{0,38}\/(?!\.+$)[A-Za-z0-9._-]{1,100}$/,
  account: /^[0-9a-f]{32}$/,
  subdomain: /^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.pages\.dev$/,
  link: /^https:\/\/pr-[1-9][0-9]{0,5}\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.pages\.dev$/,
  branch: /^pr-[1-9][0-9]{0,5}$/,
  deployment: /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/,
  segment: /^[A-Za-z0-9_][A-Za-z0-9._-]{0,127}$/,
  runId: /^[1-9][0-9]{0,19}$/,
  go: /^(?:true|false)$/,
};
const RESULTS = new Set(['success', 'failure', 'cancelled', 'skipped']);
// Names the host acts on instead of serving (Pages' Functions, routing, redirects, headers, Wrangler's settings).
const HOST_FILES = new Set(['_worker.js', '_routes.json', '_redirects', '_headers', 'functions', 'wrangler.toml',
  'wrangler.json', 'wrangler.jsonc']);
const HEADER_NAMES = ['content-type', 'cache-control', 'x-robots-tag', 'cross-origin-opener-policy',
  'cross-origin-embedder-policy', 'location'];

/** A refusal: the step turns red with this message (the script's own words and show() values only). */
export class Refusal extends Error {}

// ---------- printing ----------
/** Outside text as JSON in ASCII: no raw line break, nothing outside ASCII, so it can never begin a log line. */
export function show(value) {
  const json = JSON.stringify(value === undefined ? null : value) ?? 'null';
  return json.replace(/[\u007f-￿]/g, (c) => `\\u${c.charCodeAt(0).toString(16).padStart(4, '0')}`);
}
function say(io, message) {
  if (message.startsWith('::')) throw new Error('a log line may not begin with "::"');
  io.out(message);
}
/** A workflow annotation (notice, warning or error): the runner reads its text after "::kind::". */
function annotate(io, kind, message) {
  io.out(`::${kind}::${message.replace(/%/g, '%25').replace(/\r/g, '%0D').replace(/\n/g, '%0A')}`);
}
const errorName = (err) => String(err?.cause?.code ?? err?.name ?? 'error').slice(0, 60);

// ---------- inputs and outputs ----------
export function setOutput(io, name, value, pattern) {
  if (!pattern.test(value)) throw new Refusal(`the output ${name} would not have its shape: ${show(value)}`);
  const file = io.env.GITHUB_OUTPUT;
  if (!file || !isAbsolute(file)) throw new Refusal('GITHUB_OUTPUT is not set');
  io.fs.appendFileSync(file, `${name}=${value}\n`);
}
function temp(io) {
  const t = io.env.RUNNER_TEMP;
  if (!t || !isAbsolute(t)) throw new Refusal('RUNNER_TEMP is not set');
  return t;
}
function repo(io) {
  const r = io.env.REPO ?? '';
  if (!PATTERNS.repo.test(r)) throw new Refusal(`the repository name does not have its shape: ${show(r)}`);
  return r;
}
function githubToken(io) {
  const t = io.env.GH_TOKEN ?? '';
  if (!/^[\x21-\x7e]{20,255}$/.test(t)) throw new Refusal('GH_TOKEN is empty or not a token (its value is not printed)');
  return t;
}
function project(io) {
  if (io.env.PROJECT !== PROJECT) throw new Refusal(`the workflow's project name is not ${show(PROJECT)}`);
  return PROJECT;
}
function cloudflareSecrets(io) {
  const account = io.env.CLOUDFLARE_ACCOUNT_ID ?? '';
  const token = io.env.CLOUDFLARE_API_TOKEN ?? '';
  if (!PATTERNS.account.test(account)) {
    throw new Refusal('the secret CLOUDFLARE_ACCOUNT_ID is not 32 hexadecimal digits (its value is not printed)');
  }
  if (!/^[\x21-\x7e]{20,255}$/.test(token)) {
    throw new Refusal('the secret CLOUDFLARE_API_TOKEN is empty or not a token (its value is not printed)');
  }
  return { account, token };
}
const mainFile = (io, rel) => join(io.root, rel);

// ---------- calls ----------
async function request(io, url, { method = 'GET', headers = {}, body } = {}) {
  let res;
  try {
    res = await io.fetch(url, {
      method,
      redirect: 'manual',
      headers: { 'user-agent': USER_AGENT, ...headers, ...(body === undefined ? {} : { 'content-type': 'application/json' }) },
      body: body === undefined ? undefined : JSON.stringify(body),
      signal: AbortSignal.timeout(30_000),
    });
  } catch (err) {
    return { status: 0, json: null, error: errorName(err) };
  }
  let json = null;
  try { json = JSON.parse(await res.text()); } catch { json = null; }
  return { status: res.status, json };
}
function github(io, path, options = {}) {
  return request(io, `${GITHUB_API}${path}`, {
    ...options,
    headers: { authorization: `Bearer ${githubToken(io)}`, accept: 'application/vnd.github+json',
      'x-github-api-version': '2022-11-28' },
  });
}
function cloudflare(io, token, path, options = {}) {
  return request(io, `${CLOUDFLARE_API}${path}`, { ...options, headers: { authorization: `Bearer ${token}` } });
}
/** Every item of a GitHub list, 100 a page; `ok` is false when a page fails or the list is longer than 10 pages. */
async function githubList(io, path) {
  const items = [];
  for (let page = 1; page <= MAX_PAGES; page += 1) {
    const r = await github(io, `${path}${path.includes('?') ? '&' : '?'}per_page=100&page=${page}`);
    if (r.status !== 200 || !Array.isArray(r.json)) return { ok: false, items, why: `HTTP ${show(r.status)}` };
    items.push(...r.json);
    if (r.json.length < 100) return { ok: true, items };
  }
  return { ok: false, items, why: `longer than ${MAX_PAGES} pages` };
}
const cloudflareErrors = (json) => (Array.isArray(json?.errors) ? json.errors.slice(0, 3).map((e) => ({
  code: e?.code ?? null, message: typeof e?.message === 'string' ? e.message.slice(0, 200) : null })) : null);

// ---------- the refuse list (the security-auditor's G3) ----------
function refuseName(name, headersAllowedHere) {
  if (name.startsWith('.')) return 'a name starting with a dot';
  if (name.startsWith('-')) return 'a name starting with "-"';
  if (name.includes('..')) return 'a name holding ".."';
  if (!PATTERNS.segment.test(name)) return 'a name outside letters, digits, ".", "_" and "-" (or longer than 128)';
  if (headersAllowedHere && name === '_headers') return null;
  if (HOST_FILES.has(name.toLowerCase())) return 'a file or folder the host acts on instead of serving';
  return null;
}

/**
 * Walks a build folder without following links. `headers` null: no `_headers` anywhere (check-build's first look);
 * a Buffer: exactly one `_headers`, at the top, equal to it byte for byte. Returns the plain files and the refusals.
 */
export function checkTree(io, root, { headers = null } = {}) {
  const files = [];
  const refusals = [];
  let rootStat;
  try { rootStat = io.fs.lstatSync(root); } catch { return { files, refusals: ['the folder is missing'] }; }
  if (!rootStat.isDirectory()) return { files, refusals: ['it is not a folder'] };
  let stop = false;
  const walk = (dir, rel) => {
    for (const name of io.fs.readdirSync(dir).sort()) {
      if (stop) return;
      const path = rel === '' ? name : `${rel}/${name}`;
      const why = refuseName(name, headers !== null && rel === '');
      if (why) { refusals.push(`${show(path)}: ${why}`); continue; }
      const st = io.fs.lstatSync(join(dir, name));
      if (st.isDirectory()) { walk(join(dir, name), path); continue; }
      if (!st.isFile()) { refusals.push(`${show(path)}: neither a plain file nor a folder`); continue; }
      if (st.size >= MAX_FILE_BYTES) { refusals.push(`${show(path)}: 25 MiB or more (${st.size} bytes)`); continue; }
      files.push({ path, size: st.size });
      if (files.length > MAX_FILES) { refusals.push(`more than ${MAX_FILES} files`); stop = true; }
    }
  };
  walk(root, '');
  if (!files.some((f) => f.path === 'index.html')) refusals.push('no index.html at the top');
  if (headers !== null) {
    const found = files.find((f) => f.path === '_headers');
    if (!found) refusals.push('no _headers at the top');
    else if (!io.fs.readFileSync(join(root, '_headers')).equals(headers)) {
      refusals.push('_headers differs from main\'s hosting/_headers');
    }
  }
  return { files, refusals };
}
function reportRefusals(io, what, refusals) {
  for (const r of refusals.slice(0, 50)) say(io, `refused: ${r}`);
  if (refusals.length > 50) say(io, `refused: ${refusals.length - 50} more`);
  throw new Refusal(`${what} is refused (${refusals.length} reasons above)`);
}

// ---------- find-pr ----------
/** Writes `pr` and `sha` only for exactly one open pull request from this repository whose newest commit is HEAD_SHA. */
export async function findPr(io) {
  const sha = io.env.HEAD_SHA ?? '';
  if (!PATTERNS.commit.test(sha)) throw new Refusal(`the run's commit is not a commit id: ${show(sha)}`);
  const name = repo(io);
  githubToken(io);
  const list = await githubList(io, `/repos/${name}/pulls?state=open`);
  if (!list.ok) throw new Refusal(`the open pull requests could not be read (${list.why})`);
  const matches = list.items.filter((p) => p?.state === 'open' && p.head?.sha === sha && p.head?.repo?.full_name === name
    && p.base?.repo?.full_name === name && Number.isSafeInteger(p.number) && PATTERNS.pr.test(String(p.number)));
  if (matches.length !== 1) {
    say(io, `nothing to deploy: ${matches.length} open pull requests from this repository have ${show(sha)} as their newest commit (one is needed)`);
    return 0;
  }
  const pr = String(matches[0].number);
  setOutput(io, 'pr', pr, PATTERNS.pr);
  setOutput(io, 'sha', sha, PATTERNS.commit);
  say(io, `pull request ${pr}: its newest commit is the run's commit ${show(sha)}`);
  return 0;
}

// ---------- check-build ----------
/** Writes only go=true or go=false. Reads the fetched build only when the download's outcome is exactly `success`. */
export async function checkBuild(io) {
  const t = temp(io);
  const outcome = io.env.DOWNLOAD ?? '';
  if (outcome !== 'success') {
    const toolchain = io.fs.readFileSync(mainFile(io, '.github/workflows/toolchain.yml'), 'utf8');
    // The hand-over step's artifact name, plain or quoted, with or without a comment after it.
    const handsOver = /^[ \t]*name:[ \t]*(["']?)preview-site\1[ \t]*(?:#.*)?$/m.test(toolchain);
    setOutput(io, 'go', 'false', PATTERNS.go);
    if (handsOver) {
      annotate(io, 'error', `no build to preview: main's toolchain workflow hands one over, but this run's could not be fetched (download outcome ${show(outcome)})`);
      return 1;
    }
    annotate(io, 'notice', `no build to preview: main's toolchain workflow does not hand one over yet (download outcome ${show(outcome)})`);
    return 0;
  }
  const raw = join(t, 'site-raw');
  const checked = join(t, 'site-checked');
  const first = checkTree(io, raw, { headers: null });
  if (first.refusals.length > 0) { setOutput(io, 'go', 'false', PATTERNS.go); reportRefusals(io, 'the build', first.refusals); }
  io.fs.mkdirSync(checked);
  for (const f of first.files) {
    const from = join(raw, f.path);
    if (!io.fs.lstatSync(from).isFile()) { setOutput(io, 'go', 'false', PATTERNS.go); throw new Refusal(`${show(f.path)} changed while it was copied`); }
    io.fs.mkdirSync(dirname(join(checked, f.path)), { recursive: true });
    io.fs.copyFileSync(from, join(checked, f.path), io.fs.constants.COPYFILE_EXCL);
  }
  const headers = io.fs.readFileSync(mainFile(io, 'hosting/_headers'));
  io.fs.writeFileSync(join(checked, '_headers'), headers, { flag: 'wx' });
  const second = checkTree(io, checked, { headers });
  if (second.refusals.length > 0) { setOutput(io, 'go', 'false', PATTERNS.go); reportRefusals(io, 'the checked copy', second.refusals); }
  setOutput(io, 'go', 'true', PATTERNS.go);
  say(io, `the build passes the refuse list: ${first.files.length} files, plus main's header file`);
  return 0;
}

// ---------- recheck ----------
export async function recheck(io) {
  const site = join(temp(io), 'site');
  const r = checkTree(io, site, { headers: io.fs.readFileSync(mainFile(io, 'hosting/_headers')) });
  if (r.refusals.length > 0) reportRefusals(io, 'the deploy job\'s copy of the build', r.refusals);
  say(io, `the deploy job's copy passes the refuse list again: ${r.files.length} files, its _headers equal to main's`);
  return 0;
}

// ---------- verify-token ----------
/** Prints only the token's status and dates; saves the project's real address for read-url. */
export async function verifyToken(io) {
  const { account, token } = cloudflareSecrets(io);
  const name = project(io);
  const t = temp(io);
  const v = await cloudflare(io, token, `/accounts/${account}/tokens/verify`);
  const result = v.json?.result ?? {};
  say(io, `Cloudflare's answer on the token: HTTP ${show(v.status)}, status ${show(result.status ?? null)}, not_before ${show(result.not_before ?? null)}, expires_on ${show(result.expires_on ?? null)}`);
  if (v.status !== 200 || v.json?.success !== true || result.status !== 'active') {
    if (v.error) say(io, `the call did not complete: ${show(v.error)}`);
    else say(io, `Cloudflare's errors: ${show(cloudflareErrors(v.json))}`);
    throw new Refusal('the token is not active (the runbook: a new token by clicks 6 and 8)');
  }
  const ends = Date.parse(result.expires_on ?? '');
  if (!Number.isNaN(ends) && ends - io.now().getTime() < 14 * 24 * 3600 * 1000) {
    annotate(io, 'warning', `the deploy token ends on ${show(result.expires_on)}: renew it (the runbook: clicks 6 and 8)`);
  }
  const p = await cloudflare(io, token, `/accounts/${account}/pages/projects/${name}`);
  const record = p.json?.result ?? {};
  if (p.status !== 200 || p.json?.success !== true || record.name !== name) {
    say(io, `Cloudflare's errors: ${show(cloudflareErrors(p.json))}`);
    throw new Refusal(`the project ${show(name)} could not be read (HTTP ${show(p.status)})`);
  }
  if (typeof record.subdomain !== 'string' || !PATTERNS.subdomain.test(record.subdomain)) {
    throw new Refusal(`the project's address does not have its shape: ${show(record.subdomain ?? null)}`);
  }
  if (typeof record.production_branch !== 'string' || PATTERNS.branch.test(record.production_branch)) {
    throw new Refusal(`the project's production branch is ${show(record.production_branch ?? null)}: an upload as pr-<number> could become production`);
  }
  io.fs.writeFileSync(join(t, 'preview-subdomain'), record.subdomain, { mode: 0o600 });
  say(io, `the project's address: ${show(record.subdomain)}; its production branch: ${show(record.production_branch)}`);
  return 0;
}

// ---------- read-url ----------
const literal = (s) => s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
/** The link from Wrangler's record: exactly one upload entry, a preview, on the address verify-token read. */
export async function readUrl(io) {
  const name = project(io);
  const t = temp(io);
  const sub = io.fs.readFileSync(join(t, 'preview-subdomain'), 'utf8');
  if (!PATTERNS.subdomain.test(sub)) throw new Refusal(`the saved project address does not have its shape: ${show(sub)}`);
  const lines = io.fs.readFileSync(join(t, 'wrangler-output.ndjson'), 'utf8').split('\n').filter((l) => l.trim() !== '');
  const entries = lines.map((l) => { try { return JSON.parse(l); } catch { return undefined; } });
  if (entries.includes(undefined)) throw new Refusal('a line of Wrangler\'s record is not JSON');
  const uploads = entries.filter((e) => e?.type === 'pages-deploy-detailed');
  if (uploads.length !== 1) throw new Refusal(`Wrangler's record holds ${uploads.length} upload entries (one is needed)`);
  const d = uploads[0];
  if (d.pages_project !== name) throw new Refusal(`the upload went to project ${show(d.pages_project ?? null)}`);
  if (d.environment !== 'preview') throw new Refusal(`the upload is not a preview: ${show(d.environment ?? null)}`);
  if (typeof d.deployment_id !== 'string' || !PATTERNS.deployment.test(d.deployment_id)) {
    throw new Refusal(`the upload's id does not have its shape: ${show(d.deployment_id ?? null)}`);
  }
  if (d.url !== `https://${d.deployment_id.slice(0, 8)}.${sub}`) {
    throw new Refusal(`the upload's own address ${show(d.url ?? null)} is not on ${show(sub)}`);
  }
  const alias = typeof d.alias === 'string' ? d.alias : '';
  if (!new RegExp(`^https://pr-[1-9][0-9]{0,5}\\.${literal(sub)}$`).test(alias) || !PATTERNS.link.test(alias)) {
    throw new Refusal(`the link ${show(alias)} is not https://pr-<number>.${sub}`);
  }
  setOutput(io, 'url', alias, PATTERNS.link);
  say(io, `the link: ${show(alias)} (upload ${show(d.deployment_id)})`);
  return 0;
}

// ---------- check-headers ----------
/** The address each file is served at: index.html as its folder, other .html without the ending (Pages' rule). */
export function servedPaths(files) {
  return files.filter((f) => f.path !== '_headers' && f.path !== '404.html').map((f) => {
    if (f.path === 'index.html') return '/';
    if (f.path.endsWith('/index.html')) return `/${f.path.slice(0, -'index.html'.length)}`;
    if (f.path.endsWith('.html')) return `/${f.path.slice(0, -'.html'.length)}`;
    return `/${f.path}`;
  });
}
/** The preview header set, judged on fetched answers ({path, status, headers}); returns the problems found. */
export function judgeHeaders(answers) {
  const problems = [];
  for (const a of answers) {
    const where = show(a.path);
    const h = a.headers ?? {};
    if (a.status !== 200) { problems.push(`${where}: HTTP ${show(a.status)}, expected 200`); continue; }
    if (!/(?:^|[\s,])noindex(?:$|[\s,])/i.test(h['x-robots-tag'] ?? '')) problems.push(`${where}: no X-Robots-Tag noindex`);
    const cache = h['cache-control'] ?? '';
    const immutable = /(?:^|[\s,])immutable(?:$|[\s,])/i.test(cache);
    const hashed = a.path.startsWith('/assets/');
    if (hashed && !immutable) problems.push(`${where}: Cache-Control ${show(cache)}, expected "immutable"`);
    if (!hashed && immutable) problems.push(`${where}: Cache-Control ${show(cache)} caches it for good outside /assets/`);
    if (a.path.endsWith('.wasm') && (h['content-type'] ?? '').split(';')[0].trim().toLowerCase() !== 'application/wasm') {
      problems.push(`${where}: Content-Type ${show(h['content-type'] ?? null)}, expected "application/wasm"`);
    }
    if (h['cross-origin-opener-policy'] !== undefined || h['cross-origin-embedder-policy'] !== undefined) {
      problems.push(`${where}: COOP or COEP sent (previews are built with the isolation flag off)`);
    }
  }
  return problems;
}
const without = (answers, header, word) => answers.map((a) => ({ ...a, headers: { ...a.headers,
  [header]: (a.headers?.[header] ?? '').split(',').map((s) => s.trim()).filter((s) => s.toLowerCase() !== word).join(', ') } }));

async function fetchServed(io, url) {
  let last = null;
  for (let attempt = 1; attempt <= 5; attempt += 1) {
    last = { status: 0, headers: {}, error: null };
    try {
      const res = await io.fetch(url, { method: 'GET', redirect: 'manual', headers: { 'user-agent': USER_AGENT, accept: '*/*' },
        signal: AbortSignal.timeout(30_000) });
      for (const name of HEADER_NAMES) { const v = res.headers.get(name); if (v !== null) last.headers[name] = v; }
      last.status = res.status;
      const blocked = res.status === 403 && /1010/.test((await res.text()).slice(0, 4000));
      if (res.status !== 403) await res.body?.cancel?.();
      // An upload seconds old may not have reached every edge yet (404); 1010 is Pages' answer to a scripted client.
      if (!(res.status >= 500 || res.status === 404 || blocked)) return last;
    } catch (err) {
      last.error = errorName(err);
    }
    if (attempt < 5) await io.sleep(2000 * 2 ** (attempt - 1));
  }
  return last;
}

export async function checkHeaders(io) {
  const url = io.env.URL ?? '';
  if (!PATTERNS.link.test(url)) throw new Refusal(`the link does not have its shape: ${show(url)}`);
  const tree = checkTree(io, join(temp(io), 'site'), { headers: io.fs.readFileSync(mainFile(io, 'hosting/_headers')) });
  if (tree.refusals.length > 0) reportRefusals(io, 'the build\'s file list', tree.refusals);
  const answers = [];
  for (const path of servedPaths(tree.files)) {
    const a = await fetchServed(io, `${url}${path}`);
    answers.push({ path, status: a.status, headers: a.headers });
    say(io, `${show(path)}: ${show({ status: a.status, ...a.headers, ...(a.error ? { error: a.error } : {}) })}`);
  }
  const problems = judgeHeaders(answers);
  for (const p of problems) say(io, `header check: ${p}`);
  const cacheBite = judgeHeaders(without(answers, 'cache-control', 'immutable'));
  const noindexBite = judgeHeaders(without(answers, 'x-robots-tag', 'noindex'));
  say(io, `the bite: with "immutable" removed in memory, ${cacheBite.length} problems; with "noindex" removed, ${noindexBite.length}`);
  if (problems.length > 0) throw new Refusal(`the served headers are not the preview set (${problems.length} problems above)`);
  if (cacheBite.length === 0 || noindexBite.length === 0) throw new Refusal('the bite did not bite: the check would pass a missing rule');
  say(io, `the served headers are the preview set on all ${answers.length} files`);
  return 0;
}

// ---------- comment ----------
/** The comment, from checked values only (digits, hex, the checked link, fixed words). */
export function commentBody({ pr, sha, link, deploy, headers, count, month, runUrl }) {
  const lines = [MARKER];
  if (deploy === 'success') lines.push(`**Preview of pull request ${pr}:** ${link}`);
  else {
    const what = deploy === 'cancelled' ? 'was cancelled' : deploy === 'skipped' ? 'did not run' : 'failed';
    lines.push(`**Preview of pull request ${pr}:** no new link this time: the upload ${what} (see the run).`);
  }
  lines.push('');
  lines.push(`- Commit \`${sha}\`, this pull request's newest commit when the preview was made. The page is the build `
    + `this commit's tests ran on: the branch merged with main, so the page's corner shows that merge's commit, not `
    + `\`${sha.slice(0, 7)}\`.`);
  if (deploy === 'success') {
    lines.push(headers === 'success' ? '- Headers: checked on every file (noindex everywhere, the hashed files cached for good).'
      : headers === 'failure' ? '- Headers: the check failed (see the run). The link works, but a header is not as planned.'
        : '- Headers: not checked this time (see the run).');
  } else {
    lines.push('- An earlier link of this pull request, if there was one, still shows the earlier build.');
  }
  lines.push(`- Preview deploy attempts this month (${month}, UTC): ${count === null ? 'not counted (see the run)' : count}. `
    + 'Every attempt counts, so this is an upper bound on uploads; Cloudflare\'s free plan may count each upload against '
    + 'its 500 builds a month.'
    + (count !== null && count >= COUNT_WARNING ? ' **400 or more: see docs/runbooks/previews.md.**' : ''));
  if (runUrl) lines.push(`- The run: ${runUrl}`);
  lines.push('', 'Written by the preview workflow (docs/runbooks/previews.md).');
  return lines.join('\n');
}
/**
 * GitHub's deployment records for `preview` made this month (UTC); null when they cannot be read. GitHub's docs do not
 * state the list's order: a page made wholly before this month ends the count only while every record read so far came
 * newest first; otherwise every page is read (at most 10).
 */
export async function monthCount(io, name, start) {
  let n = 0;
  let previous = Infinity;
  let newestFirst = true;
  for (let page = 1; page <= MAX_PAGES; page += 1) {
    const r = await github(io, `/repos/${name}/deployments?environment=preview&per_page=100&page=${page}`);
    if (r.status !== 200 || !Array.isArray(r.json)) return null;
    let wholePageOlder = r.json.length > 0;
    for (const d of r.json) {
      const at = Date.parse(d?.created_at ?? '');
      if (Number.isNaN(at)) return null;
      if (at > previous) newestFirst = false;
      previous = at;
      if (at >= start) { n += 1; wholePageOlder = false; }
    }
    if (r.json.length < 100 || (newestFirst && wholePageOlder)) return n;
  }
  return null;
}

export async function comment(io) {
  const name = repo(io);
  const pr = io.env.PR ?? '';
  const sha = io.env.SHA ?? '';
  if (!PATTERNS.pr.test(pr)) throw new Refusal(`not a pull request number: ${show(pr)}`);
  if (!PATTERNS.commit.test(sha)) throw new Refusal(`not a commit id: ${show(sha)}`);
  const deploy = io.env.DEPLOY ?? '';
  if (!RESULTS.has(deploy)) throw new Refusal(`the deploy job's result is not one GitHub gives: ${show(deploy)}`);
  const headers = io.env.HEADERS ? io.env.HEADERS : 'skipped';
  if (!RESULTS.has(headers)) throw new Refusal(`the header check's outcome is not one GitHub gives: ${show(headers)}`);
  const link = io.env.URL ?? '';
  if (deploy === 'success' && (!PATTERNS.link.test(link) || !link.startsWith(`https://pr-${pr}.`))) {
    throw new Refusal(`the link ${show(link)} is not this pull request's (pr-${pr})`);
  }
  const runId = io.env.GITHUB_RUN_ID ?? '';
  const runUrl = PATTERNS.runId.test(runId) ? `https://github.com/${name}/actions/runs/${runId}` : null;
  const p = await github(io, `/repos/${name}/pulls/${pr}`);
  if (p.status !== 200) throw new Refusal(`pull request ${pr} could not be read (HTTP ${show(p.status)})`);
  if (p.json?.state !== 'open' || p.json?.head?.sha !== sha || p.json?.head?.repo?.full_name !== name) {
    say(io, `nothing written: ${show(sha)} is no longer the newest commit of open pull request ${pr} (a newer run writes the comment)`);
    return 0;
  }
  const now = io.now();
  const start = Date.UTC(now.getUTCFullYear(), now.getUTCMonth(), 1);
  const month = new Date(start).toISOString().slice(0, 7);
  const count = await monthCount(io, name, start);
  say(io, `preview deploy attempts this month (${month}, UTC), an upper bound on uploads: ${count === null ? 'not counted' : count}`);
  if (count !== null && count >= COUNT_WARNING) {
    annotate(io, 'warning', `${count} preview deploy attempts this month: the free plan's limit is 500 builds if uploads count`);
  }
  const body = commentBody({ pr, sha, link: deploy === 'success' ? link : null, deploy, headers, count, month, runUrl });
  const list = await githubList(io, `/repos/${name}/issues/${pr}/comments`);
  if (!list.ok) throw new Refusal(`the pull request's comments could not be read (${list.why})`);
  const mine = list.items.find((c) => c?.user?.login === BOT && c?.user?.type === 'Bot' && typeof c.body === 'string'
    && c.body.startsWith(MARKER) && Number.isSafeInteger(c.id));
  const w = mine
    ? await github(io, `/repos/${name}/issues/comments/${mine.id}`, { method: 'PATCH', body: { body } })
    : await github(io, `/repos/${name}/issues/${pr}/comments`, { method: 'POST', body: { body } });
  if (w.status !== (mine ? 200 : 201)) throw new Refusal(`the comment could not be written (HTTP ${show(w.status)})`);
  say(io, `${mine ? 'updated' : 'wrote'} the link comment on pull request ${pr}`);
  return 0;
}

// ---------- reconcile ----------
const madeAt = (d) => Date.parse(d.created_on);
/**
 * Which previews to delete, given Cloudflare's records and each pull request's state as looked up ('open', 'closed';
 * anything else touches nothing). Only previews whose branch is pr-<number>. A closed pull request loses all its
 * previews; an open one keeps its newest successful upload and anything newer. Oldest first, at most `max`.
 */
export function decide(deployments, states, max = MAX_DELETIONS) {
  const groups = new Map();
  let skipped = 0;
  for (const d of deployments) {
    const branch = d?.deployment_trigger?.metadata?.branch;
    if (d?.environment !== 'preview' || typeof branch !== 'string' || !PATTERNS.branch.test(branch)
      || typeof d.id !== 'string' || !PATTERNS.deployment.test(d.id) || Number.isNaN(madeAt(d))) { skipped += 1; continue; }
    const pr = branch.slice(3);
    if (!groups.has(pr)) groups.set(pr, []);
    groups.get(pr).push(d);
  }
  const candidates = [];
  for (const [pr, list] of groups) {
    const state = states.get(pr);
    if (state === 'closed') {
      for (const d of list) candidates.push({ d, pr, why: 'its pull request is closed' });
    } else if (state === 'open') {
      // Wrangler's own test of a finished upload: its latest stage is "deploy" with status "success".
      const newest = list.filter((d) => d.latest_stage?.name === 'deploy' && d.latest_stage?.status === 'success')
        .sort((a, b) => madeAt(a) - madeAt(b)).at(-1);
      if (newest) {
        for (const d of list) if (madeAt(d) < madeAt(newest)) candidates.push({ d, pr, why: 'an older upload of an open pull request' });
      }
    }
  }
  candidates.sort((a, b) => madeAt(a.d) - madeAt(b.d));
  return { now: candidates.slice(0, max), later: Math.max(0, candidates.length - max), groups, skipped };
}
/** Every preview record of the project, or `complete: false` (and then nothing is deleted). */
async function listPreviews(io, account, token, name) {
  const items = [];
  for (let page = 1; page <= CLOUDFLARE_MAX_PAGES; page += 1) {
    const r = await cloudflare(io, token, `/accounts/${account}/pages/projects/${name}/deployments?env=preview&page=${page}&per_page=${CLOUDFLARE_PAGE}`);
    if (r.status !== 200 || r.json?.success !== true || !Array.isArray(r.json.result)) {
      return { complete: false, items, why: `page ${page}: HTTP ${show(r.status)}${r.error ? ` ${show(r.error)}` : ''}` };
    }
    items.push(...r.json.result);
    // The docs give no page size or maximum and mark result_info optional: the end is an empty page, the last page
    // result_info names, or a page shorter than the size result_info gives.
    const info = r.json.result_info ?? {};
    if (r.json.result.length === 0 || (Number.isSafeInteger(info.total_pages) && page >= info.total_pages)
      || (Number.isSafeInteger(info.per_page) && info.per_page > 0 && r.json.result.length < info.per_page)) {
      // An upload made while the list is read can push a record onto the next page too: each id once.
      const ids = new Set();
      return { complete: true, items: items.filter((d) => typeof d?.id !== 'string' || (!ids.has(d.id) && ids.add(d.id))) };
    }
  }
  return { complete: false, items, why: `more than ${CLOUDFLARE_MAX_PAGES} pages` };
}

export async function reconcile(io) {
  const { account, token } = cloudflareSecrets(io);
  const name = project(io);
  const r = repo(io);
  githubToken(io);
  const listing = await listPreviews(io, account, token, name);
  if (!listing.complete) throw new Refusal(`the project's previews could not all be read (${listing.why}): nothing deleted`);
  const first = decide(listing.items, new Map());
  const states = new Map();
  for (const pr of first.groups.keys()) {
    const p = await github(io, `/repos/${r}/pulls/${pr}`);
    const state = p.status === 200 && p.json?.number === Number(pr) && (p.json.state === 'open' || p.json.state === 'closed')
      ? p.json.state : null;
    if (state) states.set(pr, state);
    say(io, `pull request ${pr}: ${state ?? `not read (HTTP ${show(p.status)}): its previews are left alone`}, ${first.groups.get(pr).length} previews`);
  }
  const plan = decide(listing.items, states);
  say(io, `previews listed: ${listing.items.length}; not ours to touch (not a pr-<number> preview): ${plan.skipped}; to delete now: ${plan.now.length}; left for the next run: ${plan.later}`);
  let failed = 0;
  for (const c of plan.now) {
    say(io, `deleting ${show(c.d.id)} (branch ${show(c.d.deployment_trigger.metadata.branch)}, made ${show(c.d.created_on)}): ${c.why}`);
    const del = await cloudflare(io, token, `/accounts/${account}/pages/projects/${name}/deployments/${c.d.id}?force=true`, { method: 'DELETE' });
    const ok = del.status === 200 && del.json?.success === true;
    if (!ok) failed += 1;
    say(io, ok ? '  deleted' : `  not deleted: HTTP ${show(del.status)} ${show(cloudflareErrors(del.json) ?? del.error ?? null)}`);
  }
  if (failed > 0) throw new Refusal(`${failed} deletions failed`);
  return 0;
}

// ---------- the command line ----------
export const COMMANDS = {
  'find-pr': findPr, 'check-build': checkBuild, recheck, 'verify-token': verifyToken, 'read-url': readUrl,
  'check-headers': checkHeaders, comment, reconcile,
};
export function defaultIo() {
  return {
    env: process.env, fs: realFs, fetch: globalThis.fetch, root: fileURLToPath(new URL('..', import.meta.url)),
    out: (line) => process.stdout.write(`${line}\n`), now: () => new Date(),
    sleep: (ms) => new Promise((resolve) => { setTimeout(resolve, ms); }),
  };
}
export async function run(name, io) {
  if (!Object.hasOwn(COMMANDS, name)) {
    io.out(`usage: node hosting/preview.mjs <${Object.keys(COMMANDS).join('|')}>`);
    return 2;
  }
  try {
    return await COMMANDS[name](io);
  } catch (err) {
    if (err instanceof Refusal) annotate(io, 'error', `${name}: ${err.message}`);
    else annotate(io, 'error', `${name}: an unexpected error: ${show(String(err?.message ?? err).slice(0, 300))}`);
    return 1;
  }
}
if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  process.exitCode = await run(process.argv[2] ?? '', defaultIo());
}
