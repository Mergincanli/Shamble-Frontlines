// The tests of hosting/preview.mjs (step 2.6's pass mark M2), the lint of .github/workflows/preview.yml (G9) and the
// test of hosting/_headers (PHC-PREVIEW-NOINDEX). Run by the preview workflow's job `tests`: Node's built-ins only, no
// network (every call goes to a fake), all files in a fresh temporary folder. Each test pairs the good case with a
// planted defect that must turn it red.
import { after, describe, test } from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import * as fs from 'node:fs';
import { createServer } from 'node:net';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  BOT, COUNT_WARNING, MARKER, MAX_DELETIONS, MAX_FILE_BYTES, MAX_FILES, PATTERNS, Refusal, checkTree, commentBody, decide,
  judgeHeaders, run, servedPaths, setOutput, show,
} from './preview.mjs';

const ROOT = fileURLToPath(new URL('..', import.meta.url));
const WORKFLOW = fs.readFileSync(join(ROOT, '.github/workflows/preview.yml'), 'utf8');
const HEADERS = fs.readFileSync(join(ROOT, 'hosting/_headers'));
const scratch = fs.mkdtempSync(join(tmpdir(), 'pv-'));
after(() => fs.rmSync(scratch, { recursive: true, force: true }));
let made = 0;
const fresh = (label) => { made += 1; const d = join(scratch, `${label}${made}`); fs.mkdirSync(d, { recursive: true }); return d; };

const SHA = '69d9b5f36a603a3c6716050269c619ca7cabe85a';
const NEWER = 'b'.repeat(40);
const REPO = 'Mergincanli/Shamble-Frontlines';
const ACCOUNT = '0123456789abcdef0123456789abcdef';
const TOKEN = 'cfat_test-token-never-printed-0123456789';
const GH = 'ghs_test-github-token-never-printed-01234';
const SUB = 'shamble-previews-x1.pages.dev';
const LINK = `https://pr-12.${SUB}`;
// The real production build's shape (toolchain run 45686704452, 2026-11-25): index.html and the hashed assets/.
const BUILD = {
  'index.html': '<!doctype html><title>Shamble</title>\n',
  'assets/index-CYgNJTPa.css': 'body{}',
  'assets/index-zPOY-_Jj.js': 'console.log(1)',
  'assets/sim_wasm_bg-BiMaAlnD.wasm': '\0asm',
  'assets/worker-CyhXt1A5.js': 'self.onmessage=()=>{}',
};
function writeTree(dir, files) {
  for (const [path, content] of Object.entries(files)) {
    fs.mkdirSync(dirname(join(dir, path)), { recursive: true });
    fs.writeFileSync(join(dir, path), content);
  }
  return dir;
}
/** A checkout of main as the commands read it: hosting/_headers and the toolchain workflow. */
function mainCheckout(toolchain = 'name: toolchain\njobs:\n  toolchain:\n    steps:\n      - run: echo build\n') {
  const root = fresh('main');
  writeTree(root, { 'hosting/_headers': HEADERS, '.github/workflows/toolchain.yml': toolchain });
  return root;
}
function response({ status = 200, json, text, headers = {} } = {}) {
  return new Response(text ?? (json === undefined ? '' : JSON.stringify(json)), { status, headers });
}
/** A fake fetch: routes are [RegExp on "METHOD url", handler(match, init)]; every call is recorded in order. */
function fakeFetch(routes, events = []) {
  const calls = [];
  const f = async (url, init = {}) => {
    const method = init.method ?? 'GET';
    calls.push({ method, url, init });
    events.push(`call ${method} ${url}`);
    for (const [pattern, handler] of routes) {
      const m = pattern.exec(`${method} ${url}`);
      if (m) return response(await handler(m, init));
    }
    throw new Error(`no route for ${method} ${url}`);
  };
  f.calls = calls;
  return f;
}
function harness({ env = {}, root = ROOT, fetch = fakeFetch([]), fsx = fs, now = '2026-11-25T12:00:00Z', events = [] } = {}) {
  const lines = [];
  const output = join(fresh('out'), 'github-output');
  fs.writeFileSync(output, '');
  const io = {
    env: { GITHUB_OUTPUT: output, ...env }, fs: fsx, fetch, root, now: () => new Date(now),
    out: (line) => { lines.push(line); events.push(`line ${line}`); }, sleep: async () => {},
  };
  return { io, lines, events, output: () => fs.readFileSync(output, 'utf8'), fetch };
}
/** No line may begin with "::" except the script's own annotations, and every line is ASCII. */
function assertSafe(lines) {
  for (const l of lines) {
    assert.match(l, /^[\x20-\x7e]*$/, `a log line outside printable ASCII: ${JSON.stringify(l)}`);
    if (l.startsWith('::')) assert.match(l, /^::(?:notice|warning|error)::/, `a workflow command that is not ours: ${l}`);
  }
}
/** Records every fs call whose path argument starts with `prefix`. */
function watchedFs(prefix) {
  const touched = [];
  const fsx = new Proxy(fs, {
    get(target, prop) {
      const v = target[prop];
      if (typeof v !== 'function') return v;
      return (...args) => {
        for (const a of args) if (typeof a === 'string' && a.startsWith(prefix)) touched.push(`${String(prop)} ${a}`);
        return v.apply(target, args);
      };
    },
  });
  return { fsx, touched };
}

describe('the patterns', () => {
  const GOOD = {
    pr: '12', commit: SHA, repo: REPO, account: ACCOUNT, subdomain: SUB, link: LINK, branch: 'pr-12',
    deployment: '1f6487e4-0a85-43d1-8b1f-cd80c0e1d61c', segment: 'index-zPOY-_Jj.js', runId: '45686704452', go: 'true',
  };
  test('every pattern has a case here, and refuses its good value with a line break before or after', () => {
    assert.deepEqual(Object.keys(GOOD).sort(), Object.keys(PATTERNS).sort());
    for (const [name, value] of Object.entries(GOOD)) {
      assert.ok(PATTERNS[name].test(value), `${name} refuses its good value`);
      assert.ok(!PATTERNS[name].test(`${value}\n`), `${name} takes a trailing line break`);
      assert.ok(!PATTERNS[name].test(`\n${value}`), `${name} takes a leading line break`);
      assert.ok(!PATTERNS[name].test(`${value}\r`), `${name} takes a trailing carriage return`);
    }
  });
  test('the planted values are refused', () => {
    for (const v of ['0', '007', '1234567', '-1', '1e3', '12 ']) assert.ok(!PATTERNS.pr.test(v), v);
    for (const v of [SHA.toUpperCase(), SHA.slice(1), `${SHA}0`]) assert.ok(!PATTERNS.commit.test(v), v);
    for (const v of ['a/..', 'a/.', '../x', 'a/b/c', 'a b/c']) assert.ok(!PATTERNS.repo.test(v), v);
    for (const v of [`${LINK}/`, `${LINK}:443`, LINK.replace('https', 'http'), `${LINK}.evil.example`,
      'https://pr-12.evil.pages.dev.example.com', 'https://pr-012.x.pages.dev', 'https://1f6487e4.x.pages.dev']) {
      assert.ok(!PATTERNS.link.test(v), v);
    }
    assert.ok(!PATTERNS.subdomain.test('x.pages.dev.example.com'));
    assert.ok(!PATTERNS.deployment.test('f64788e9-fccd-4d4a-a28a-cb84f88f6'), 'the API example\'s short id');
    assert.ok(!PATTERNS.branch.test('pr-01'));
  });
  test('show() leaves no line break, nothing outside ASCII and no leading "::"', () => {
    for (const hostile of ['a\nb', '::error::x', 'x\r::set-env', ' ::warning::y', 'café', '%0A::error::z', '😀']) {
      const s = show(hostile);
      assert.match(s, /^[\x20-\x7e]*$/);
      assert.ok(!s.startsWith('::'));
      assert.equal(JSON.parse(s), hostile);
    }
  });
  test('an output is written only in its shape: a planted line break is refused and nothing is written', () => {
    const h = harness();
    setOutput(h.io, 'url', LINK, PATTERNS.link);
    assert.throws(() => setOutput(h.io, 'url', `${LINK}\ngo=true`, PATTERNS.link), Refusal);
    assert.equal(h.output(), `url=${LINK}\n`);
  });
  test('planted: a relative GITHUB_OUTPUT is refused', () => {
    const h = harness({ env: { GITHUB_OUTPUT: 'relative/github-output' } });
    assert.throws(() => setOutput(h.io, 'go', 'true', PATTERNS.go), Refusal);
  });
  test('an annotation escapes "%", so outside text cannot spell a line break for the runner', async () => {
    const h = harness({ env: { RUNNER_TEMP: fresh('t'), DOWNLOAD: '%0A::error::planted' }, root: mainCheckout() });
    assert.equal(await run('check-build', h.io), 0);
    const notice = h.lines.find((l) => l.startsWith('::notice::'));
    assert.ok(notice.includes('"%250A::error::planted"') && !notice.includes('"%0A'), notice);
  });
  test('an unknown command prints its usage and exits 2', async () => {
    const h = harness();
    assert.equal(await run('deploy-everything', h.io), 2);
    assert.match(h.lines[0], /^usage: /);
  });
});

describe('the refuse list (G3)', () => {
  const io = { fs };
  const tree = (extra = {}) => writeTree(writeTree(fresh('b'), BUILD), extra);
  test('the real build\'s shape passes', () => {
    const r = checkTree(io, tree());
    assert.deepEqual(r.refusals, []);
    assert.equal(r.files.length, 5);
  });
  const planted = [
    ['.env', 'dot'], ['assets/.hidden.js', 'dot'], ['.well-known/x.txt', 'dot'],
    ['a b.js', 'outside letters'], ['café.js', 'outside letters'], ['x:y.js', 'outside letters'],
    [`${'x'.repeat(129)}.js`, 'outside letters'], ['-rf.js', 'starting with "-"'], ['a..b.js', 'holding ".."'],
    ['_worker.js', 'host acts'], ['assets/_worker.js', 'host acts'], ['_worker.js/index.js', 'host acts'],
    ['_routes.json', 'host acts'], ['_redirects', 'host acts'], ['_headers', 'host acts'], ['_Headers', 'host acts'],
    ['functions/api.js', 'host acts'], ['FUNCTIONS/api.js', 'host acts'], ['wrangler.toml', 'host acts'],
    ['wrangler.json', 'host acts'], ['wrangler.jsonc', 'host acts'],
  ];
  for (const [name, why] of planted) {
    test(`planted ${JSON.stringify(name)} is refused (${why})`, () => {
      const r = checkTree(io, tree({ [name]: 'x' }));
      assert.ok(r.refusals.some((x) => x.includes(why === 'dot' ? 'starting with a dot' : why === 'host acts' ? 'the host acts on' : why)),
        JSON.stringify(r.refusals));
    });
  }
  test('a link and a socket are refused (neither a plain file nor a folder)', async () => {
    const dir = tree();
    fs.symlinkSync(join(dir, 'index.html'), join(dir, 'assets', 'link.js'));
    assert.ok(checkTree(io, dir).refusals.some((x) => x.includes('"assets/link.js": neither')));
    const sockDir = tree();
    const server = createServer();
    await new Promise((resolve) => { server.listen(join(sockDir, 's'), resolve); });
    try { assert.ok(checkTree(io, sockDir).refusals.some((x) => x.includes('"s": neither'))); } finally { server.close(); }
  });
  test('a file of 25 MiB is refused; one byte less passes', () => {
    const dir = tree();
    fs.writeFileSync(join(dir, 'assets', 'big.bin'), '');
    fs.truncateSync(join(dir, 'assets', 'big.bin'), MAX_FILE_BYTES - 1);
    assert.deepEqual(checkTree(io, dir).refusals, []);
    fs.truncateSync(join(dir, 'assets', 'big.bin'), MAX_FILE_BYTES);
    assert.ok(checkTree(io, dir).refusals.some((x) => x.includes('25 MiB or more')));
  });
  test('20,000 files pass; one more is refused', () => {
    const dir = writeTree(fresh('many'), { 'index.html': 'x' });
    fs.mkdirSync(join(dir, 'assets'));
    for (let i = 1; i < MAX_FILES; i += 1) fs.writeFileSync(join(dir, 'assets', `f${i}.js`), '');
    assert.deepEqual(checkTree(io, dir).refusals, []);
    fs.writeFileSync(join(dir, 'assets', 'one-more.js'), '');
    assert.ok(checkTree(io, dir).refusals.includes(`more than ${MAX_FILES} files`));
  });
  test('a build without index.html at the top is refused', () => {
    const dir = writeTree(fresh('b'), { 'assets/x.js': 'x', 'sub/index.html': 'x' });
    assert.ok(checkTree(io, dir).refusals.includes('no index.html at the top'));
  });
  test('with main\'s header file: exactly one _headers, at the top, byte for byte', () => {
    assert.deepEqual(checkTree(io, tree({ _headers: HEADERS }), { headers: HEADERS }).refusals, []);
    const changed = Buffer.from(HEADERS); changed[changed.length - 2] ^= 1;
    assert.ok(checkTree(io, tree({ _headers: changed }), { headers: HEADERS }).refusals.some((x) => x.includes('differs')));
    assert.ok(checkTree(io, tree(), { headers: HEADERS }).refusals.includes('no _headers at the top'));
    assert.ok(checkTree(io, tree({ _headers: HEADERS, 'assets/_headers': HEADERS }), { headers: HEADERS }).refusals
      .some((x) => x.includes('"assets/_headers"')));
  });
});

describe('check-build', () => {
  const env = (t, download) => ({ RUNNER_TEMP: t, DOWNLOAD: download });
  const withHandOver = 'jobs:\n  toolchain:\n    steps:\n      - uses: actions/upload-artifact@x\n        with:\n          name: preview-site\n          path: .gate/dist-production/\n';
  for (const outcome of ['failure', 'cancelled', 'skipped', '', 'Success', 'success\n']) {
    test(`download outcome ${JSON.stringify(outcome)}: the fetched folder is never read, go=false`, async () => {
      const t = fresh('t');
      writeTree(join(t, 'site-raw'), { ...BUILD, '_worker.js': 'planted' });
      const watched = watchedFs(join(t, 'site-raw'));
      const h = harness({ env: env(t, outcome), root: mainCheckout(), fsx: watched.fsx });
      assert.equal(await run('check-build', h.io), 0);
      assert.equal(h.output(), 'go=false\n');
      assert.deepEqual(watched.touched, []);
      assert.ok(!fs.existsSync(join(t, 'site-checked')));
      assert.ok(h.lines.some((l) => l.startsWith('::notice::no build to preview')));
      assertSafe(h.lines);
    });
  }
  test('no download while main\'s toolchain hands a build over: a visible failure, still without reading', async () => {
    const t = fresh('t');
    writeTree(join(t, 'site-raw'), BUILD);
    const watched = watchedFs(join(t, 'site-raw'));
    const h = harness({ env: env(t, 'failure'), root: mainCheckout(withHandOver), fsx: watched.fsx });
    assert.equal(await run('check-build', h.io), 1);
    assert.equal(h.output(), 'go=false\n');
    assert.deepEqual(watched.touched, []);
    assert.ok(h.lines.some((l) => l.startsWith('::error::no build to preview')));
  });
  test('planted: an artifact named preview-site-checked, or a mismatched quote, is not the hand-over step', async () => {
    for (const name of ['name: preview-site-checked', 'name: "preview-site\'', 'name: preview-sites']) {
      const h = harness({ env: env(fresh('t'), 'failure'), root: mainCheckout(withHandOver.replace('name: preview-site', name)) });
      assert.equal(await run('check-build', h.io), 0, name);
    }
  });
  test('the hand-over step is found quoted or with a comment after it', async () => {
    for (const name of ['name: "preview-site"', 'name: \'preview-site\'', 'name: preview-site # the hand-over', 'name:   preview-site  ']) {
      const h = harness({ env: env(fresh('t'), 'failure'), root: mainCheckout(withHandOver.replace('name: preview-site', name)) });
      assert.equal(await run('check-build', h.io), 1, name);
    }
  });
  test('a clean build: checked, copied, main\'s header file added, go=true and nothing else written', async () => {
    const t = fresh('t');
    writeTree(join(t, 'site-raw'), BUILD);
    const h = harness({ env: env(t, 'success'), root: mainCheckout() });
    assert.equal(await run('check-build', h.io), 0);
    assert.equal(h.output(), 'go=true\n');
    for (const [p, c] of Object.entries(BUILD)) assert.equal(fs.readFileSync(join(t, 'site-checked', p), 'utf8'), c);
    assert.ok(fs.readFileSync(join(t, 'site-checked', '_headers')).equals(HEADERS));
    assertSafe(h.lines);
  });
  test('planted: 20,000 files pass the first look, but not with main\'s header file added: refused, go=false', async () => {
    const t = fresh('t');
    const raw = join(t, 'site-raw');
    writeTree(raw, { 'index.html': 'x' });
    fs.mkdirSync(join(raw, 'assets'));
    for (let i = 1; i < MAX_FILES; i += 1) fs.writeFileSync(join(raw, 'assets', `f${i}.js`), '');
    const h = harness({ env: env(t, 'success'), root: mainCheckout() });
    assert.equal(await run('check-build', h.io), 1);
    assert.equal(h.output(), 'go=false\n');
  });
  for (const planted of ['_headers', '_worker.js', 'functions/index.js', '.env']) {
    test(`planted ${planted} in the build: refused, go=false, nothing copied`, async () => {
      const t = fresh('t');
      writeTree(join(t, 'site-raw'), { ...BUILD, [planted]: 'planted' });
      const h = harness({ env: env(t, 'success'), root: mainCheckout() });
      assert.equal(await run('check-build', h.io), 1);
      assert.equal(h.output(), 'go=false\n');
      assert.ok(!fs.existsSync(join(t, 'site-checked')));
      assertSafe(h.lines);
    });
  }
});

describe('recheck', () => {
  const site = (extra = {}, headers = HEADERS) => { const t = fresh('t'); writeTree(join(t, 'site'), { ...BUILD, _headers: headers, ...extra }); return t; };
  test('the checked build passes again', async () => {
    const h = harness({ env: { RUNNER_TEMP: site() }, root: mainCheckout() });
    assert.equal(await run('recheck', h.io), 0);
  });
  test('planted: a header file one byte off, an extra _redirects, a missing header file', async () => {
    const off = Buffer.from(HEADERS); off[0] ^= 1;
    for (const t of [site({}, off), site({ _redirects: '/* https://evil.example 302' })]) {
      const h = harness({ env: { RUNNER_TEMP: t }, root: mainCheckout() });
      assert.equal(await run('recheck', h.io), 1);
    }
    const t = fresh('t'); writeTree(join(t, 'site'), BUILD);
    assert.equal(await run('recheck', harness({ env: { RUNNER_TEMP: t }, root: mainCheckout() }).io), 1);
  });
});

describe('find-pr', () => {
  const pull = (number, sha, headRepo = REPO, baseRepo = REPO) => ({ number, state: 'open', head: { sha, repo: { full_name: headRepo } }, base: { repo: { full_name: baseRepo } } });
  const githubWith = (pages) => fakeFetch([[/^GET https:\/\/api\.github\.com\/repos\/Mergincanli\/Shamble-Frontlines\/pulls\?state=open&per_page=100&page=(\d+)$/,
    (m) => ({ json: pages[Number(m[1]) - 1] ?? [] })]]);
  const env = (sha = SHA) => ({ GH_TOKEN: GH, REPO, HEAD_SHA: sha });
  test('one open pull request from this repository with that newest commit: pr and sha written', async () => {
    const filler = Array.from({ length: 100 }, (_, i) => pull(100 + i, NEWER));
    const h = harness({ env: env(), fetch: githubWith([filler, [pull(12, SHA), pull(13, SHA, 'someone/fork'), pull(14, NEWER)]]) });
    assert.equal(await run('find-pr', h.io), 0);
    assert.equal(h.output(), `pr=12\nsha=${SHA}\n`);
    assert.equal(h.fetch.calls[0].init.headers.authorization, `Bearer ${GH}`);
    assert.ok(!h.lines.join('\n').includes(GH));
    assertSafe(h.lines);
  });
  test('planted: a commit id with a trailing line break is refused before any call', async () => {
    const h = harness({ env: env(`${SHA}\n`), fetch: githubWith([[pull(12, SHA)]]) });
    assert.equal(await run('find-pr', h.io), 1);
    assert.equal(h.fetch.calls.length, 0);
    assert.equal(h.output(), '');
  });
  test('planted: only a fork, or a pull request into another repository, has the commit: nothing written', async () => {
    for (const page of [[pull(13, SHA, 'someone/fork')], [pull(15, SHA, REPO, 'someone/other')]]) {
      const h = harness({ env: env(), fetch: githubWith([page]) });
      assert.equal(await run('find-pr', h.io), 0);
      assert.equal(h.output(), '');
    }
  });
  test('planted: a listed pull request whose state is not open is ignored', async () => {
    const h = harness({ env: env(), fetch: githubWith([[{ ...pull(12, SHA), state: 'closed' }]]) });
    assert.equal(await run('find-pr', h.io), 0);
    assert.equal(h.output(), '');
  });
  test('planted: more than 10 full pages of pull requests: a visible failure, nothing written', async () => {
    const full = Array.from({ length: 100 }, (_, i) => pull(1000 + i, NEWER));
    const h = harness({ env: env(), fetch: githubWith(Array.from({ length: 11 }, () => full)) });
    assert.equal(await run('find-pr', h.io), 1);
    assert.equal(h.fetch.calls.length, 10);
    assert.equal(h.output(), '');
  });
  test('planted: two pull requests share the commit: nothing written', async () => {
    const h = harness({ env: env(), fetch: githubWith([[pull(12, SHA), pull(16, SHA)]]) });
    assert.equal(await run('find-pr', h.io), 0);
    assert.equal(h.output(), '');
  });
  test('planted: GitHub answers 500: a visible failure, nothing written', async () => {
    const h = harness({ env: env(), fetch: fakeFetch([[/pulls/, () => ({ status: 500, json: { message: 'x' } })]]) });
    assert.equal(await run('find-pr', h.io), 1);
    assert.equal(h.output(), '');
  });
});

describe('verify-token', () => {
  const cloudflareWith = ({ status = 'active', expires = '2027-02-23T23:00:00Z', record = {}, verifyHttp = 200, verifyOk = true,
    projectHttp = 200, projectOk = true } = {}) => fakeFetch([
    [/^GET https:\/\/api\.cloudflare\.com\/client\/v4\/accounts\/0123456789abcdef0123456789abcdef\/tokens\/verify$/,
      () => ({ status: verifyHttp, json: { success: verifyOk, errors: [], result: { id: 'x', status, not_before: '2026-11-26T00:00:00Z', expires_on: expires } } })],
    [/^GET https:\/\/api\.cloudflare\.com\/client\/v4\/accounts\/0123456789abcdef0123456789abcdef\/pages\/projects\/shamble-previews$/,
      () => ({ status: projectHttp, json: { success: projectOk, errors: [], result: { name: 'shamble-previews', subdomain: SUB, production_branch: 'main', ...record } } })],
  ]);
  test('planted: an answer with HTTP 401 or success false is refused, though its body says "active"', async () => {
    for (const planted of [{ verifyHttp: 401 }, { verifyOk: false }, { projectHttp: 404 }, { projectOk: false }]) {
      const t = fresh('t');
      assert.equal(await run('verify-token', harness({ env: env(t), fetch: cloudflareWith(planted) }).io), 1, JSON.stringify(planted));
      assert.ok(!fs.existsSync(join(t, 'preview-subdomain')));
    }
  });
  const env = (t, extra = {}) => ({ RUNNER_TEMP: t, PROJECT: 'shamble-previews', CLOUDFLARE_API_TOKEN: TOKEN, CLOUDFLARE_ACCOUNT_ID: ACCOUNT, ...extra });
  test('an active token: its status and dates printed, never the token or the account; the address saved', async () => {
    const t = fresh('t');
    const h = harness({ env: env(t), fetch: cloudflareWith() });
    assert.equal(await run('verify-token', h.io), 0);
    assert.equal(fs.readFileSync(join(t, 'preview-subdomain'), 'utf8'), SUB);
    const log = h.lines.join('\n');
    assert.ok(log.includes('status "active"') && log.includes('expires_on "2027-02-23T23:00:00Z"'));
    assert.ok(!log.includes(TOKEN) && !log.includes(ACCOUNT));
    assert.ok(h.fetch.calls.every((c) => c.url.startsWith('https://api.cloudflare.com/client/v4/accounts/')));
    assert.ok(!h.lines.some((l) => l.startsWith('::warning::')));
    assertSafe(h.lines);
  });
  test('planted: an expired token fails visibly and saves no address', async () => {
    const t = fresh('t');
    assert.equal(await run('verify-token', harness({ env: env(t), fetch: cloudflareWith({ status: 'expired' }) }).io), 1);
    assert.ok(!fs.existsSync(join(t, 'preview-subdomain')));
  });
  test('planted: an account ID of the wrong shape is refused before any call, and not printed', async () => {
    const t = fresh('t');
    const h = harness({ env: env(t, { CLOUDFLARE_ACCOUNT_ID: ACCOUNT.slice(1) }), fetch: cloudflareWith() });
    assert.equal(await run('verify-token', h.io), 1);
    assert.equal(h.fetch.calls.length, 0);
    assert.ok(!h.lines.join('\n').includes(ACCOUNT.slice(1)));
  });
  test('planted: an address of the wrong shape, a pr- production branch, another project', async () => {
    for (const record of [{ subdomain: 'evil.example.com' }, { production_branch: 'pr-12' }, { name: 'other' }]) {
      const t = fresh('t');
      assert.equal(await run('verify-token', harness({ env: env(t), fetch: cloudflareWith({ record }) }).io), 1, JSON.stringify(record));
      assert.ok(!fs.existsSync(join(t, 'preview-subdomain')));
    }
  });
  test('a token ending within 14 days gives a warning', async () => {
    const h = harness({ env: env(fresh('t')), fetch: cloudflareWith({ expires: '2026-12-01T00:00:00Z' }) });
    assert.equal(await run('verify-token', h.io), 0);
    assert.ok(h.lines.some((l) => l.startsWith('::warning::the deploy token ends')));
  });
});

describe('read-url', () => {
  // As Wrangler 4.164.0 writes them (wrangler-dist/cli.js: "pages-deploy" then "pages-deploy-detailed").
  const detailed = (extra = {}) => JSON.stringify({ type: 'pages-deploy-detailed', version: 1, pages_project: 'shamble-previews',
    deployment_id: '1f6487e4-0a85-43d1-8b1f-cd80c0e1d61c', url: `https://1f6487e4.${SUB}`, alias: LINK, environment: 'preview',
    production_branch: 'main', deployment_trigger: { metadata: { commit_hash: SHA } }, timestamp: '2026-11-25T20:00:00.000Z', ...extra });
  const record = (lines, sub = SUB) => {
    const t = fresh('t');
    fs.writeFileSync(join(t, 'preview-subdomain'), sub);
    fs.writeFileSync(join(t, 'wrangler-output.ndjson'), `${[JSON.stringify({ type: 'wrangler-session', version: 1 }),
      JSON.stringify({ type: 'pages-deploy', version: 1, pages_project: 'shamble-previews' }), ...lines].join('\n')}\n`);
    return t;
  };
  const env = (t) => ({ RUNNER_TEMP: t, PROJECT: 'shamble-previews' });
  test('the one upload entry, a preview on the project\'s address: url written', async () => {
    const h = harness({ env: env(record([detailed()])) });
    assert.equal(await run('read-url', h.io), 0);
    assert.equal(h.output(), `url=${LINK}\n`);
  });
  const planted = {
    'another project\'s address': [detailed({ alias: 'https://pr-12.evil.pages.dev' })],
    'production': [detailed({ environment: 'production' })],
    'two upload entries': [detailed(), detailed()],
    'no link': [detailed({ alias: undefined })],
    'a link with a path': [detailed({ alias: `${LINK}/x` })],
    'a link with a line break': [detailed({ alias: `${LINK}\n` })],
    'another project': [detailed({ pages_project: 'other' })],
    'a one-off address elsewhere': [detailed({ url: 'https://1f6487e4.evil.pages.dev' })],
    'a line that is not JSON': [detailed(), '::error::not json'],
  };
  for (const [name, lines] of Object.entries(planted)) {
    test(`planted: ${name}: refused, nothing written`, async () => {
      const h = harness({ env: env(record(lines)) });
      assert.equal(await run('read-url', h.io), 1);
      assert.equal(h.output(), '');
      assertSafe(h.lines);
    });
  }
  test('planted: an upload id of the wrong shape, even when the one-off address matches it', async () => {
    const id = 'ZZZZZZZZ-0a85-43d1-8b1f-cd80c0e1d61c';
    const h = harness({ env: env(record([detailed({ deployment_id: id, url: `https://ZZZZZZZZ.${SUB}` })])) });
    assert.equal(await run('read-url', h.io), 1);
    assert.equal(h.output(), '');
  });
  test('planted: a saved address of the wrong shape', async () => {
    assert.equal(await run('read-url', harness({ env: env(record([detailed()], 'evil.example.com')) }).io), 1);
  });
});

describe('check-headers', () => {
  const site = (files = BUILD) => { const t = fresh('t'); writeTree(join(t, 'site'), { ...files, _headers: HEADERS }); return t; };
  const preview = (path) => ({
    'x-robots-tag': 'noindex',
    'cache-control': path.startsWith('/assets/') ? 'public, max-age=31536000, immutable' : 'public, max-age=0, must-revalidate',
    'content-type': path.endsWith('.wasm') ? 'application/wasm' : 'text/plain',
  });
  const host = (change = (path, h) => h) => fakeFetch([[/^GET https:\/\/pr-12\.shamble-previews-x1\.pages\.dev(\/.*)$/,
    (m) => { const r = change(m[1], { status: 200, headers: preview(m[1]) }); return r; }]]);
  test('the served paths: index.html as /, other .html without the ending, never _headers or 404.html', () => {
    assert.deepEqual(servedPaths([{ path: '_headers' }, { path: 'index.html' }, { path: 'about.html' }, { path: 'docs/index.html' },
      { path: '404.html' }, { path: 'assets/a-1.js' }]), ['/', '/about', '/docs/', '/assets/a-1.js']);
  });
  test('the preview set on every file passes, and both bites bite', async () => {
    const h = harness({ env: { RUNNER_TEMP: site(), URL: LINK }, root: mainCheckout(), fetch: host() });
    assert.equal(await run('check-headers', h.io), 0);
    assert.equal(h.fetch.calls.length, 5);
    for (const c of h.fetch.calls) {
      assert.equal(c.init.redirect, 'manual');
      assert.ok(c.init.headers['user-agent'].startsWith('shamble-frontlines-preview-workflow/'));
      assert.ok(!('cookie' in c.init.headers));
    }
    assert.ok(h.lines.some((l) => /^the bite: with "immutable" removed in memory, [1-9]/.test(l)));
    assertSafe(h.lines);
  });
  const planted = {
    'index.html cached for good': (p, r) => (p === '/' ? { ...r, headers: { ...r.headers, 'cache-control': 'public, max-age=31536000, immutable' } } : r),
    'the Wasm as octet-stream': (p, r) => (p.endsWith('.wasm') ? { ...r, headers: { ...r.headers, 'content-type': 'application/octet-stream' } } : r),
    'one hashed file without immutable': (p, r) => (p.endsWith('.css') ? { ...r, headers: { ...r.headers, 'cache-control': 'no-cache' } } : r),
    'one file without noindex': (p, r) => (p.endsWith('.js') ? { ...r, headers: { ...r.headers, 'x-robots-tag': 'nosnippet' } } : r),
    'COOP sent': (p, r) => ({ ...r, headers: { ...r.headers, 'cross-origin-opener-policy': 'same-origin' } }),
    'a redirect on /': (p, r) => (p === '/' ? { status: 308, headers: { location: 'https://evil.example/' } } : r),
    'a 404 that stays': (p, r) => (p.endsWith('.css') ? { status: 404, headers: {} } : r),
  };
  for (const [name, change] of Object.entries(planted)) {
    test(`planted: ${name}: a visible failure`, async () => {
      const h = harness({ env: { RUNNER_TEMP: site(), URL: LINK }, root: mainCheckout(), fetch: host(change) });
      assert.equal(await run('check-headers', h.io), 1);
      assert.ok(h.fetch.calls.every((c) => c.url.startsWith(LINK)), 'a redirect was followed');
      assertSafe(h.lines);
    });
  }
  test('a 404 right after the upload is asked again, then passes', async () => {
    let first = true;
    const h = harness({ env: { RUNNER_TEMP: site(), URL: LINK }, root: mainCheckout(),
      fetch: host((p, r) => { if (p === '/' && first) { first = false; return { status: 404, headers: {} }; } return r; }) });
    assert.equal(await run('check-headers', h.io), 0);
    assert.equal(h.fetch.calls.length, 6);
  });
  test('a 503, then a 403 with error 1010, are asked again; the third answer is judged', async () => {
    let n = 0;
    const h = harness({ env: { RUNNER_TEMP: site(), URL: LINK }, root: mainCheckout(), fetch: host((p, r) => {
      if (p !== '/') return r;
      n += 1;
      return n === 1 ? { status: 503, headers: {} } : n === 2 ? { status: 403, headers: {}, text: 'error code: 1010' } : r;
    }) });
    assert.equal(await run('check-headers', h.io), 0);
    assert.equal(h.fetch.calls.filter((c) => c.url === `${LINK}/`).length, 3);
  });
  test('planted: a 503 that stays is asked five times, then fails', async () => {
    const h = harness({ env: { RUNNER_TEMP: site(), URL: LINK }, root: mainCheckout(),
      fetch: host((p, r) => (p === '/' ? { status: 503, headers: {} } : r)) });
    assert.equal(await run('check-headers', h.io), 1);
    assert.equal(h.fetch.calls.filter((c) => c.url === `${LINK}/`).length, 5);
  });
  test('planted: the build\'s file list holds a refused file: refused before any call', async () => {
    const h = harness({ env: { RUNNER_TEMP: site({ ...BUILD, _redirects: '/* https://evil.example 302' }), URL: LINK }, root: mainCheckout(), fetch: host() });
    assert.equal(await run('check-headers', h.io), 1);
    assert.equal(h.fetch.calls.length, 0);
  });
  test('planted: a link of the wrong shape is refused before any call', async () => {
    const h = harness({ env: { RUNNER_TEMP: site(), URL: 'https://pr-12.evil.example' }, root: mainCheckout(), fetch: host() });
    assert.equal(await run('check-headers', h.io), 1);
    assert.equal(h.fetch.calls.length, 0);
  });
  test('planted: a build with no hashed file leaves the cache bite nothing to remove: a visible failure', async () => {
    const h = harness({ env: { RUNNER_TEMP: site({ 'index.html': 'x' }), URL: LINK }, root: mainCheckout(), fetch: host() });
    assert.equal(await run('check-headers', h.io), 1);
    assert.ok(h.lines.some((l) => l.includes('the bite did not bite')));
  });
  test('judgeHeaders on its own: the preview set passes, each planted header fails', () => {
    const good = ['/', '/assets/a-1.js', '/assets/b-2.wasm'].map((path) => ({ path, status: 200, headers: preview(path) }));
    assert.deepEqual(judgeHeaders(good), []);
    assert.equal(judgeHeaders([{ ...good[1], headers: { ...good[1].headers, 'cross-origin-embedder-policy': 'require-corp' } }]).length, 1);
  });
});

describe('comment', () => {
  const env = (extra = {}) => ({ GH_TOKEN: GH, REPO, PR: '12', SHA, URL: LINK, DEPLOY: 'success', HEADERS: 'success', GITHUB_RUN_ID: '45690000001', ...extra });
  const marked = (id, login, type, body = `${MARKER}\nold`) => ({ id, user: { login, type }, body });
  function github({ head = SHA, comments = [], deployments = [], write = 200, post = 201, pullAnswer = null, listStatus = 200,
    countStatus = 200 } = {}) {
    // Any pull request number is open with that head, so a planted value can only be refused by the check it targets.
    return fakeFetch([
      [/^GET https:\/\/api\.github\.com\/repos\/Mergincanli\/Shamble-Frontlines\/pulls\/(\d+)$/,
        (m) => pullAnswer ?? { json: { number: Number(m[1]), state: 'open', head: { sha: head, repo: { full_name: REPO } } } }],
      [/^GET https:\/\/api\.github\.com\/repos\/Mergincanli\/Shamble-Frontlines\/deployments\?environment=preview&per_page=100&page=(\d+)$/,
        (m) => (countStatus === 200 ? { json: deployments[Number(m[1]) - 1] ?? [] } : { status: countStatus, json: { message: 'x' } })],
      [/^GET https:\/\/api\.github\.com\/repos\/Mergincanli\/Shamble-Frontlines\/issues\/\d+\/comments\?per_page=100&page=1$/,
        () => (listStatus === 200 ? { json: comments } : { status: listStatus, json: { message: 'x' } })],
      [/^PATCH https:\/\/api\.github\.com\/repos\/Mergincanli\/Shamble-Frontlines\/issues\/comments\/(\d+)$/, () => ({ status: write, json: {} })],
      [/^POST https:\/\/api\.github\.com\/repos\/Mergincanli\/Shamble-Frontlines\/issues\/\d+\/comments$/, () => ({ status: post, json: {} })],
    ]);
  }
  test('planted: GitHub refuses the write (403 on an update, 500 on a new comment): the step turns red', async () => {
    assert.equal(await run('comment', harness({ env: env(), fetch: github({ comments: [marked(777, BOT, 'Bot')], write: 403 }) }).io), 1);
    assert.equal(await run('comment', harness({ env: env(), fetch: github({ post: 500 }) }).io), 1);
  });
  test('planted: the pull request cannot be read (500): the step turns red and writes nothing', async () => {
    const f = github({ pullAnswer: { status: 500, json: { message: 'x' } } });
    assert.equal(await run('comment', harness({ env: env(), fetch: f }).io), 1);
    assert.equal(writes(f).length, 0);
  });
  test('planted: the pull request is closed, or its head is a fork: nothing written', async () => {
    for (const json of [{ number: 12, state: 'closed', head: { sha: SHA, repo: { full_name: REPO } } },
      { number: 12, state: 'open', head: { sha: SHA, repo: { full_name: 'someone/fork' } } }]) {
      const f = github({ pullAnswer: { json } });
      assert.equal(await run('comment', harness({ env: env(), fetch: f }).io), 0);
      assert.equal(writes(f).length, 0);
    }
  });
  test('planted: the comments cannot be read: the step turns red, and no second comment is written', async () => {
    const f = github({ listStatus: 500 });
    assert.equal(await run('comment', harness({ env: env(), fetch: f }).io), 1);
    assert.equal(writes(f).length, 0);
  });
  test('planted: an unreadable count is "not counted", never 0', async () => {
    const f = github({ countStatus: 500 });
    const h = harness({ env: env(), fetch: f });
    assert.equal(await run('comment', h.io), 0);
    assert.ok(JSON.parse(writes(f)[0].init.body).body.includes('not counted (see the run)'));
    assert.ok(h.lines.some((l) => l.endsWith(': not counted')));
  });
  const writes = (f) => f.calls.filter((c) => c.method !== 'GET');
  test('the body: the marker first, the link, the commit, fixed words only', () => {
    const body = commentBody({ pr: '12', sha: SHA, link: LINK, deploy: 'success', headers: 'success', count: 3, month: '2026-11', runUrl: 'https://github.com/x/y/actions/runs/1' });
    assert.ok(body.startsWith(`${MARKER}\n**Preview of pull request 12:** ${LINK}`));
    assert.ok(body.includes(SHA) && body.includes('(2026-11, UTC): 3.'));
    const failed = commentBody({ pr: '12', sha: SHA, link: null, deploy: 'failure', headers: 'skipped', count: null, month: '2026-11', runUrl: null });
    assert.ok(failed.includes('no new link this time: the upload failed') && !failed.includes('https://pr-'));
    assert.ok(commentBody({ pr: '12', sha: SHA, link: LINK, deploy: 'success', headers: 'success', count: COUNT_WARNING, month: '2026-11', runUrl: null })
      .includes('400 or more'));
  });
  test('the bot\'s marked comment is updated, never another user\'s; the month is counted', async () => {
    const deployments = [[{ created_at: '2026-11-25T10:00:00Z' }, { created_at: '2026-11-02T10:00:00Z' }, { created_at: '2026-10-30T10:00:00Z' }]];
    const f = github({ comments: [marked(5, 'Mergincanli', 'User'), marked(777, BOT, 'Bot')], deployments });
    const h = harness({ env: env(), fetch: f });
    assert.equal(await run('comment', h.io), 0);
    const w = writes(f);
    assert.equal(w.length, 1);
    assert.ok(w[0].url.endsWith('/issues/comments/777') && w[0].method === 'PATCH');
    const body = JSON.parse(w[0].init.body).body;
    assert.ok(body.includes(LINK) && body.includes('(2026-11, UTC): 2.'));
    assert.ok(body.includes('https://github.com/Mergincanli/Shamble-Frontlines/actions/runs/45690000001'));
    assertSafe(h.lines);
  });
  test('planted: no marked comment of the bot (only another user\'s, or the bot\'s without the marker): a new one', async () => {
    const f = github({ comments: [marked(5, 'Mergincanli', 'User'), { id: 6, user: { login: BOT, type: 'Bot' }, body: 'unrelated' }] });
    assert.equal(await run('comment', harness({ env: env(), fetch: f }).io), 0);
    assert.deepEqual(writes(f).map((c) => c.method), ['POST']);
  });
  test('planted: the pull request has a newer commit: nothing written', async () => {
    const f = github({ head: NEWER });
    assert.equal(await run('comment', harness({ env: env(), fetch: f }).io), 0);
    assert.equal(writes(f).length, 0);
  });
  test('planted: another pull request\'s link, an outside link, a result GitHub never gives: refused, nothing written', async () => {
    for (const extra of [{ URL: `https://pr-13.${SUB}` }, { PR: '1' }, { URL: 'https://pr-12.evil.example' }, { DEPLOY: 'success\n' },
      { HEADERS: 'passed' }, { PR: '12\n' }, { SHA: SHA.toUpperCase() }]) {
      const f = github();
      assert.equal(await run('comment', harness({ env: env(extra), fetch: f }).io), 1, JSON.stringify(extra));
      assert.equal(writes(f).length, 0);
    }
  });
  test('a failed upload: the comment says so, with no link', async () => {
    const f = github({ comments: [marked(777, BOT, 'Bot')] });
    assert.equal(await run('comment', harness({ env: env({ DEPLOY: 'failure', URL: '', HEADERS: '' }), fetch: f }).io), 0);
    const body = JSON.parse(writes(f)[0].init.body).body;
    assert.ok(body.includes('the upload failed') && !body.includes('https://pr-'));
  });
  test('the count stops at a page wholly before the month only while the records come newest first', async () => {
    // Each record a minute apart: falling within a page (newest first) or rising (oldest first).
    const page = (date, step, n = 100) => Array.from({ length: n }, (_, i) => ({ created_at: new Date(Date.parse(date) + step * i * 60000).toISOString() }));
    const sorted = github({ deployments: [page('2026-11-20T00:00:00Z', -1), page('2026-10-01T00:00:00Z', -1), page('2026-11-21T00:00:00Z', -1)] });
    await run('comment', harness({ env: env(), fetch: sorted }).io);
    assert.equal(sorted.calls.filter((c) => c.url.includes('/deployments?')).length, 2);
    const unsorted = github({ deployments: [page('2026-10-01T00:00:00Z', 1), page('2026-11-21T00:00:00Z', 1), page('2026-11-22T00:00:00Z', 1, 5)] });
    const h = harness({ env: env(), fetch: unsorted });
    await run('comment', h.io);
    assert.equal(unsorted.calls.filter((c) => c.url.includes('/deployments?')).length, 3);
    assert.ok(h.lines.includes('preview deploy attempts this month (2026-11, UTC), an upper bound on uploads: 105'));
  });
});

describe('reconcile', () => {
  let ids = 0;
  const preview = (branch, createdOn, { env = 'preview', stage = 'success', id } = {}) => {
    ids += 1;
    return { id: id ?? `${String(ids).padStart(8, '0')}-0000-4000-8000-000000000000`, environment: env, created_on: createdOn,
      latest_stage: { name: 'deploy', status: stage }, deployment_trigger: { type: 'ad_hoc', metadata: { branch } } };
  };
  test('decide: closed loses all; open keeps its newest successful upload and anything newer; the rest untouched', () => {
    const records = [
      preview('pr-1', '2026-11-01T00:00:00Z'), preview('pr-1', '2026-11-02T00:00:00Z'),
      preview('pr-2', '2026-11-03T00:00:00Z'), preview('pr-2', '2026-11-04T00:00:00Z'), preview('pr-2', '2026-11-05T00:00:00Z', { stage: 'failure' }),
      preview('pr-3', '2026-11-01T00:00:00Z'), preview('main', '2026-11-01T00:00:00Z'), preview('pr-01', '2026-11-01T00:00:00Z'),
      preview('pr-4', '2026-11-01T00:00:00Z', { env: 'production' }), preview('pr-5', 'not a date'), preview('pr-6', '2026-11-01T00:00:00Z', { id: '../x' }),
    ];
    const plan = decide(records, new Map([['1', 'closed'], ['2', 'open']]));
    assert.deepEqual(plan.now.map((c) => [c.pr, c.d.created_on]),
      [['1', '2026-11-01T00:00:00Z'], ['1', '2026-11-02T00:00:00Z'], ['2', '2026-11-03T00:00:00Z']]);
    assert.equal(plan.skipped, 5);
  });
  test('decide: an open pull request with no successful upload keeps everything', () => {
    const plan = decide([preview('pr-2', '2026-11-03T00:00:00Z', { stage: 'failure' }), preview('pr-2', '2026-11-04T00:00:00Z', { stage: 'active' })], new Map([['2', 'open']]));
    assert.equal(plan.now.length, 0);
  });
  test('decide: at most 20 now, oldest first, the rest left for the next run', () => {
    const records = Array.from({ length: 30 }, (_, i) => preview('pr-1', new Date(Date.UTC(2026, 10, 1, i)).toISOString()));
    const plan = decide(records, new Map([['1', 'closed']]));
    assert.equal(plan.now.length, MAX_DELETIONS);
    assert.equal(plan.later, 10);
    assert.equal(plan.now[0].d.created_on, '2026-11-01T00:00:00.000Z');
  });
  const base = 'https:\\/\\/api\\.cloudflare\\.com\\/client\\/v4\\/accounts\\/0123456789abcdef0123456789abcdef\\/pages\\/projects\\/shamble-previews\\/deployments';
  function world({ pages, states, failPage = 0, failDelete = '', falseDelete = '' }) {
    const events = [];
    const f = fakeFetch([
      [new RegExp(`^GET ${base}\\?env=preview&page=(\\d+)&per_page=25$`), (m) => {
        const page = Number(m[1]);
        if (page === failPage) return { status: 500, json: { success: false, errors: [{ code: 1, message: 'x' }] } };
        return { json: { success: true, errors: [], result: pages[page - 1] ?? [], result_info: { page, per_page: 25, total_pages: pages.length } } };
      }],
      [/^GET https:\/\/api\.github\.com\/repos\/Mergincanli\/Shamble-Frontlines\/pulls\/(\d+)$/, (m) => states[m[1]] ?? { status: 404, json: {} }],
      [new RegExp(`^DELETE ${base}\\/([0-9a-f-]+)\\?force=true$`), (m) => (m[1] === failDelete ? { status: 500, json: { success: false } }
        : m[1] === falseDelete ? { json: { success: false, errors: [{ code: 8000000, message: 'x' }] } } : { json: { success: true, errors: [], result: {} } })],
    ], events);
    return { f, events };
  }
  const env = { CLOUDFLARE_API_TOKEN: TOKEN, CLOUDFLARE_ACCOUNT_ID: ACCOUNT, GH_TOKEN: GH, REPO, PROJECT: 'shamble-previews' };
  const open = (n) => ({ json: { number: n, state: 'open' } });
  const closed = (n) => ({ json: { number: n, state: 'closed' } });
  test('looks every pull request up first, logs each deletion before making it, touches nothing it could not look up', async () => {
    const page1 = Array.from({ length: 25 }, (_, i) => preview('pr-7', new Date(Date.UTC(2026, 10, 1, i)).toISOString()));
    const page2 = [preview('pr-1', '2026-11-02T00:00:00Z'), preview('pr-3', '2026-11-01T00:00:00Z'), preview('pr-4', '2026-11-01T00:00:00Z'),
      preview('pr-8', '2026-11-01T00:00:00Z'), preview('pr-9', '2026-11-01T00:00:00Z')];
    const { f, events } = world({ pages: [page1, page2], states: { 7: open(7), 1: closed(1), 3: { status: 500, json: {} },
      8: { json: { number: 8, state: 'Closed' } }, 9: { json: { number: 90, state: 'closed' } } } });
    const h = harness({ env, fetch: f, events });
    assert.equal(await run('reconcile', h.io), 0);
    const deletes = f.calls.filter((c) => c.method === 'DELETE');
    // pr-7 is open: 24 older uploads, oldest first; pr-1 closed: 1. At most 20 now. pr-3 (500), pr-4 (404),
    // pr-8 ("Closed") and pr-9 (another number) are left alone.
    assert.equal(deletes.length, MAX_DELETIONS);
    const firstDelete = events.findIndex((e) => e.startsWith('call DELETE'));
    const lastLookup = events.map((e) => /^call GET https:\/\/api\.github\.com\//.test(e)).lastIndexOf(true);
    assert.ok(lastLookup < firstDelete, 'a deletion came before every lookup was done');
    for (const c of deletes) {
      const id = /deployments\/([0-9a-f-]+)\?/.exec(c.url)[1];
      const i = events.indexOf(`call DELETE ${c.url}`);
      assert.ok(events[i - 1].startsWith(`line deleting "${id}"`), `no log line before deleting ${id}`);
    }
    assert.ok(!deletes.some((c) => [page2[1], page2[2], page2[3], page2[4]].some((d) => c.url.includes(d.id))));
    assert.ok(h.lines.some((l) => l.includes('left for the next run: 5')));
    assert.ok(!h.lines.join('\n').includes(TOKEN) && !h.lines.join('\n').includes(ACCOUNT));
    assertSafe(h.lines);
  });
  test('planted: a listing page fails: nothing looked up, nothing deleted, a visible failure', async () => {
    const page1 = Array.from({ length: 25 }, (_, i) => preview('pr-1', new Date(Date.UTC(2026, 10, 1, i)).toISOString()));
    const { f } = world({ pages: [page1, [preview('pr-1', '2026-11-03T00:00:00Z')]], states: { 1: closed(1) }, failPage: 2 });
    assert.equal(await run('reconcile', harness({ env, fetch: f }).io), 1);
    assert.equal(f.calls.filter((c) => c.method === 'DELETE').length, 0);
    assert.equal(f.calls.filter((c) => c.url.startsWith('https://api.github.com/')).length, 0);
  });
  test('a record listed on two pages (an upload made while the list was read) is deleted once', async () => {
    const twice = preview('pr-1', '2026-11-01T00:00:00Z');
    const page1 = [...Array.from({ length: 24 }, (_, i) => preview('pr-1', new Date(Date.UTC(2026, 10, 2, i)).toISOString())), twice];
    const { f } = world({ pages: [page1, [twice]], states: { 1: closed(1) } });
    assert.equal(await run('reconcile', harness({ env, fetch: f }).io), 0);
    const deletes = f.calls.filter((c) => c.method === 'DELETE').map((c) => c.url);
    assert.equal(deletes.filter((u) => u.includes(twice.id)).length, 1);
    assert.equal(new Set(deletes).size, deletes.length);
  });
  test('planted: one deletion fails: the others still run, and the step turns red', async () => {
    const records = [preview('pr-1', '2026-11-01T00:00:00Z'), preview('pr-1', '2026-11-02T00:00:00Z')];
    const { f } = world({ pages: [records], states: { 1: closed(1) }, failDelete: records[0].id });
    assert.equal(await run('reconcile', harness({ env, fetch: f }).io), 1);
    assert.equal(f.calls.filter((c) => c.method === 'DELETE').length, 2);
  });
  test('planted: a delete answered HTTP 200 with success false counts as failed', async () => {
    const records = [preview('pr-1', '2026-11-01T00:00:00Z')];
    const { f } = world({ pages: [records], states: { 1: closed(1) }, falseDelete: records[0].id });
    const h = harness({ env, fetch: f });
    assert.equal(await run('reconcile', h.io), 1);
    assert.ok(h.lines.some((l) => l.startsWith('  not deleted: HTTP 200')));
  });
  test('planted: more than 40 full pages of previews: incomplete, nothing deleted', async () => {
    const pages = Array.from({ length: 41 }, (_, p) => Array.from({ length: 25 }, (_, i) => preview('pr-1', new Date(Date.UTC(2026, 10, 1, 0, p * 25 + i)).toISOString())));
    const { f } = world({ pages, states: { 1: closed(1) } });
    assert.equal(await run('reconcile', harness({ env, fetch: f }).io), 1);
    assert.equal(f.calls.filter((c) => c.method === 'DELETE').length, 0);
    assert.equal(f.calls.filter((c) => c.url.includes('/deployments?')).length, 40);
  });
  test('planted: an account ID of the wrong shape: no call at all', async () => {
    const { f } = world({ pages: [[]], states: {} });
    assert.equal(await run('reconcile', harness({ env: { ...env, CLOUDFLARE_ACCOUNT_ID: 'x' }, fetch: f }).io), 1);
    assert.equal(f.calls.length, 0);
  });
});

describe('the command line (the workflow runs `node hosting/preview.mjs <command>`)', () => {
  const SCRIPT = join(ROOT, 'hosting/preview.mjs');
  const cli = (args, env = {}) => spawnSync(process.execPath, [SCRIPT, ...args], { encoding: 'utf8', env: { PATH: process.env.PATH ?? '', ...env } });
  test('no command: the usage line and exit 2', () => {
    const r = cli([]);
    assert.equal(r.status, 2, r.stdout + r.stderr);
    assert.match(r.stdout, /^usage: node hosting\/preview\.mjs </);
  });
  test('recheck with the real files: main\'s header file passes, a changed one is refused', () => {
    const t = fresh('cli');
    writeTree(join(t, 'site'), { ...BUILD, _headers: HEADERS });
    const ok = cli(['recheck'], { RUNNER_TEMP: t });
    assert.equal(ok.status, 0, ok.stdout + ok.stderr);
    assert.match(ok.stdout, /passes the refuse list again/);
    const t2 = fresh('cli');
    const off = Buffer.from(HEADERS); off[0] ^= 1;
    writeTree(join(t2, 'site'), { ...BUILD, _headers: off });
    const bad = cli(['recheck'], { RUNNER_TEMP: t2 });
    assert.equal(bad.status, 1, bad.stdout + bad.stderr);
    assert.match(bad.stdout, /^::error::recheck: /m);
  });
  test('find-pr refuses a malformed commit before any call, and writes nothing', () => {
    const out = join(fresh('cli'), 'github-output');
    fs.writeFileSync(out, '');
    const r = cli(['find-pr'], { HEAD_SHA: 'not-a-commit', REPO, GH_TOKEN: GH, GITHUB_OUTPUT: out });
    assert.equal(r.status, 1, r.stdout + r.stderr);
    assert.match(r.stdout, /^::error::find-pr: the run's commit is not a commit id/m);
    assert.equal(fs.readFileSync(out, 'utf8'), '');
  });
});

// ---------- G9: the lint of .github/workflows/preview.yml ----------
// A reader for this repository's workflow layout (2-space indents, block style): each line with its place (top key,
// job, job key, step, step key). Block scalars (run: |) are content: a "#" inside one is not a YAML comment.
function workflowLines(text) {
  const records = [];
  const keyOf = (s) => (/^([A-Za-z0-9_-]+):(?:\s|$)/.exec(s) ?? [])[1] ?? null;
  const ctx = { top: null, job: null, jobKey: null, step: -1, stepKey: null };
  let steps = -1;
  let block = null;
  text.split('\n').forEach((raw, i) => {
    const line = raw.replace(/\s+$/, '');
    const indent = line.length - line.trimStart().length;
    if (block !== null) {
      if (line === '' || indent > block) { records.push({ n: i + 1, line, indent, ...ctx, key: null, inBlock: true }); return; }
      block = null;
    }
    const body = line.trimStart();
    if (body === '' || body.startsWith('#')) return;
    const item = body.startsWith('- ');
    const key = keyOf(item ? body.slice(2) : body);
    if (indent === 0) Object.assign(ctx, { top: key, job: null, jobKey: null, step: -1, stepKey: null });
    else if (indent === 2 && ctx.top === 'jobs') Object.assign(ctx, { job: key, jobKey: null, step: -1, stepKey: null });
    else if (indent === 4 && ctx.job) Object.assign(ctx, { jobKey: key, step: -1, stepKey: null });
    else if (indent === 6 && ctx.jobKey === 'steps' && item) { steps += 1; Object.assign(ctx, { step: steps, stepKey: key }); }
    else if (indent === 8 && ctx.step >= 0) ctx.stepKey = key;
    const value = key === null ? '' : (item ? body.slice(2) : body).slice(key.length + 1).trim();
    if (blockHeader(value)) block = item ? indent + 2 : indent;
    records.push({ n: i + 1, line, indent, ...ctx, key, value, item, inBlock: false });
  });
  return records;
}
/** A block scalar's header (`|`, `>-`, `|2+`), with or without a trailing YAML comment (`| # note`). */
function blockHeader(value) {
  return /^[|>][0-9+-]*(?:\s+#.*)?$/.test(value);
}
/** The shell command a record carries: a run block's line, a one-line run's value, or '' (a block header, not a run). */
function commandOf(r) {
  if (!(r.step >= 0 && r.stepKey === 'run')) return '';
  if (r.inBlock) return r.line.trim();
  return r.key === 'run' && !blockHeader(r.value) ? r.value : '';
}
function lintWorkflow(text) {
  const problems = [];
  const flag = (rule, r, what) => problems.push(`${rule} line ${r ? r.n : '-'}: ${what}`);
  if (text.includes('\t')) flag('L0', null, 'a tab character');
  const records = workflowLines(text);
  for (const r of records) if (!r.inBlock && r.indent % 2 !== 0) flag('L0', r, 'an odd indent outside a block');
  // L0: shapes this reader does not follow (a quoted key, a flow-mapping step, a space before a key's colon).
  for (const r of records) {
    if (!r.inBlock && /^\s*(?:- )?(?:["'][^"']*["']\s*:|\{)/.test(r.line)) flag('L0', r, 'a quoted key or a flow mapping');
    if (!r.inBlock && /^\s*(?:- )?[A-Za-z0-9_-]+\s+:(?:\s|$)/.test(r.line)) flag('L0', r, 'a space before a key\'s colon');
  }
  // L1 (G1): no ${{ }} inside any shell block.
  for (const r of records) if (r.step >= 0 && r.stepKey === 'run' && r.line.includes('${{')) flag('L1', r, '${{ inside run');
  const stepsOf = new Map();
  for (const r of records) if (r.step >= 0) { if (!stepsOf.has(r.step)) stepsOf.set(r.step, []); stepsOf.get(r.step).push(r); }
  // L2 (G6): every line naming secrets (in any case and spelling: secrets.X, secrets['X'], toJSON(secrets)) is one of
  // the two planned env lines, in a step's env: never job-wide or workflow-wide, never an action input.
  const SECRET_LINES = ['CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}', 'CLOUDFLARE_ACCOUNT_ID: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}'];
  const namesSecrets = (r) => /\bsecrets\b/i.test(r.line);
  for (const r of records.filter(namesSecrets)) {
    if (!(r.step >= 0 && r.stepKey === 'env' && r.indent === 10 && !r.inBlock)) flag('L2', r, 'a secret outside a step env');
    else if (!SECRET_LINES.includes(r.line.trim())) flag('L2', r, 'a secret line other than the two planned ones');
  }
  // ... and the steps naming secrets are exactly the three planned ones, in order, each known by its whole command (not
  // by its job, and not by a command it merely holds): deploy's token check, deploy's upload, reconcile; each names
  // both secrets once. A change to one of these three commands changes this list in the same reviewed pull request.
  const PLACES = [
    ['deploy', 'node hosting/preview.mjs verify-token'],
    ['deploy', [
      '[[ "$PR" =~ ^[0-9]{1,6}$ ]] && [[ "$SHA" =~ ^[0-9a-f]{40}$ ]] || exit 1',
      '"$RUNNER_TEMP/wrangler/node_modules/.bin/wrangler" pages deploy "$RUNNER_TEMP/site" \\',
      '--project-name "$PROJECT" --branch "pr-$PR" --commit-hash "$SHA" \\',
      '--commit-message "pull request $PR" --commit-dirty=true',
    ].join('\n')],
    ['reconcile', 'node hosting/preview.mjs reconcile'],
  ];
  const places = [...stepsOf.values()].filter((rs) => rs.some(namesSecrets)).map((rs) => {
    const run = rs.map(commandOf).filter((s) => s !== '').join('\n');
    const at = PLACES.findIndex(([job, command]) => rs[0].job === job && run === command);
    const both = SECRET_LINES.every((s) => rs.filter((r) => r.line.trim() === s).length === 1);
    return at >= 0 && both ? `planned ${at + 1}` : `${rs[0].job} line ${rs[0].n} (${run.split('\n')[0] || 'no run'})`;
  });
  if (places.join(', ') !== 'planned 1, planned 2, planned 3') flag('L2', null, `secrets in [${places.join(', ')}]; planned: deploy's token check and upload, and reconcile`);
  // L3 (G5): npm only as one whole `npm ci --ignore-scripts` line whose other flags are --no-audit and --no-fund:
  // nothing chained after it or continued onto the next line, and no flag that turns install scripts back on (npm 10
  // reads `--no-ignore-scripts`, `--ignore-scripts=false` and `--ignore-scripts false` as on again, measured); no npm
  // configuration on any line (npm_config_* in any case, an .npmrc); no other installer.
  const NPM_CI = /^npm ci(?: --(?:ignore-scripts|no-audit|no-fund))+$/;
  const plainNpmCi = (r) => NPM_CI.test(commandOf(r)) && commandOf(r).split(' ').includes('--ignore-scripts');
  for (const r of records) {
    if (/npm_config_|npmrc/i.test(r.line)) flag('L3', r, 'npm configuration (npm_config_*, an .npmrc)');
    if (!(r.step >= 0 && r.stepKey === 'run')) continue;
    if (/\bnpx\b|\byarn\b|\bpnpm\b|\bcorepack\b|\bbunx?\b/i.test(r.line)) flag('L3', r, 'an installer other than npm ci');
    if (/\bnpm\b/i.test(r.line) && !plainNpmCi(r)) flag('L3', r, 'npm other than one whole "npm ci --ignore-scripts" line');
  }
  // L4 (G8): no cache in any job.
  const jobs = [...new Set(records.filter((r) => r.top === 'jobs' && r.indent === 2).map((r) => r.job))];
  for (const job of jobs) {
    if (!records.some((r) => r.job === job && r.indent === 4 && r.line.trim() === 'cache-mode: none')) flag('L4', null, `job ${job} without cache-mode: none`);
  }
  for (const [, rs] of stepsOf) {
    const uses = rs.find((r) => r.key === 'uses' && (r.indent === 6 || r.indent === 8));
    const withLine = (text2) => rs.some((r) => r.stepKey === 'with' && r.indent === 10 && r.line.trim() === text2);
    if (uses && /^actions\/cache[/@]/.test(uses.value)) flag('L4', uses, 'actions/cache');
    if (uses && uses.value.startsWith('actions/setup-node@') && !withLine('package-manager-cache: false')) flag('L4', uses, 'setup-node with its cache on');
    for (const r of rs) if (r.stepKey === 'with' && r.indent === 10 && r.key === 'cache') flag('L4', r, 'a cache input');
    // L5 (G1): every checkout without its credentials; every action pinned to a full commit.
    if (uses && uses.value.startsWith('actions/checkout@') && !withLine('persist-credentials: false')) flag('L5', uses, 'a checkout that keeps its credentials');
    if (uses && uses.value.startsWith('actions/checkout@') && rs.some((r) => r.stepKey === 'with' && r.indent === 10 && (r.key === 'ref' || r.key === 'repository'))) {
      flag('L5', uses, 'a checkout of another ref or repository (every checkout is main\'s)');
    }
    if (uses && !/^[A-Za-z0-9_.-]+\/[A-Za-z0-9_./-]+@[0-9a-f]{40}(?:\s+#.*)?$/.test(uses.value)) flag('L5', uses, 'an action not pinned to a full commit');
  }
  if (!records.some((r) => r.indent === 0 && r.line === 'permissions: {}')) flag('L5', null, 'no top-level permissions: {}');
  if (/pull_request_target/.test(text)) flag('L5', null, 'pull_request_target');
  return problems;
}
/** Replaces the nth (1-based) of exactly `count` occurrences, so a planted test never silently plants nothing. */
function plant(text, find, replacement, { count = 1, nth = 1 } = {}) {
  const parts = text.split(find);
  assert.equal(parts.length - 1, count, `expected ${count} occurrences of ${JSON.stringify(find)}`);
  return parts.slice(0, nth).join(find) + replacement + parts.slice(nth).join(find);
}

describe('G9: the lint of preview.yml', () => {
  test('the workflow as it stands passes, and the reader sees its whole shape', () => {
    assert.deepEqual(lintWorkflow(WORKFLOW), []);
    const rs = workflowLines(WORKFLOW);
    assert.deepEqual([...new Set(rs.filter((r) => r.top === 'jobs' && r.indent === 2).map((r) => r.job))], ['tests', 'check', 'deploy', 'after', 'reconcile']);
    assert.ok(new Set(rs.filter((r) => r.step >= 0 && r.key === 'run').map((r) => r.step)).size >= 12);
  });
  test('YAML comments are not read: a comment naming secrets, npm install or npm_config_ raises nothing', () => {
    assert.deepEqual(lintWorkflow('# no secrets, no npm install, no npm_config_ignore_scripts here\n' + WORKFLOW), []);
  });
  test('a block header with a trailing comment opens a block, and is not read as part of the command', () => {
    const header = '          WRANGLER_OUTPUT_FILE_PATH: ${{ runner.temp }}/wrangler-output.ndjson\n        run: |\n';
    const text = plant(WORKFLOW, header, header.replace('run: |\n', 'run: | # the upload\n'));
    assert.deepEqual(lintWorkflow(text), []);
    assert.equal(workflowLines(text).find((r) => r.line.trim().startsWith('"$RUNNER_TEMP/wrangler/node_modules/.bin/wrangler"')).inBlock, true);
  });
  const NPM_LINE = 'npm ci --ignore-scripts --no-audit --no-fund';
  const METRICS = '  WRANGLER_SEND_METRICS: "false"\n';
  const TOKEN = '          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}\n';
  const ACCOUNT = '          CLOUDFLARE_ACCOUNT_ID: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}\n';
  const VERIFY_NAME_END = '(prints its status and dates, never the token)\n';
  const UPLOAD_END = '            --commit-message "pull request $PR" --commit-dirty=true\n';
  const INSTALL = '      - name: Install Wrangler from main\'s lockfile into a fresh folder, install scripts off\n        run: |\n';
  /** Gives the install step (no secrets, no env today) an env with these lines. */
  const onInstall = (envLines, base = WORKFLOW) => plant(base, INSTALL, INSTALL.replace('        run: |\n', `        env:\n${envLines}        run: |\n`));
  /** The token check without its env: both secrets taken off it. */
  const verifyBare = () => plant(WORKFLOW, `${VERIFY_NAME_END}        env:\n${TOKEN}${ACCOUNT}`, VERIFY_NAME_END);
  const planted = [
    ['L1', '${{ }} in a one-line run', () => plant(WORKFLOW, 'run: node hosting/preview.mjs find-pr', 'run: node hosting/preview.mjs find-pr "${{ github.event.workflow_run.head_branch }}"')],
    ['L1', '${{ }} in a run block', () => plant(WORKFLOW, '          cd "$RUNNER_TEMP/wrangler"\n', '          cd "$RUNNER_TEMP/wrangler"\n          echo "${{ needs.check.outputs.pr }}"\n')],
    ['L1', '${{ }} in a shell comment inside a run block', () => plant(WORKFLOW, '          cd "$RUNNER_TEMP/wrangler"\n', '          cd "$RUNNER_TEMP/wrangler"\n          # ${{ github.event.workflow_run.head_branch }}\n')],
    ['L2', 'the token job-wide', () => plant(WORKFLOW, '    needs: check\n    if: needs.check.outputs.go == \'true\'\n', '    needs: check\n    if: needs.check.outputs.go == \'true\'\n    env:\n      CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}\n')],
    ['L2', 'the token workflow-wide', () => plant(WORKFLOW, '  WRANGLER_SEND_METRICS: "false"\n', '  WRANGLER_SEND_METRICS: "false"\n  CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}\n')],
    ['L2', 'a secret as an action input', () => plant(WORKFLOW, 'github-token: ${{ github.token }}', 'github-token: ${{ secrets.CLOUDFLARE_API_TOKEN }}')],
    ['L2', 'every secret workflow-wide through toJSON', () => plant(WORKFLOW, '  WRANGLER_SEND_METRICS: "false"\n', '  WRANGLER_SEND_METRICS: "false"\n  ALL: ${{ toJSON(secrets) }}\n')],
    ['L2', 'the token by index, job-wide', () => plant(WORKFLOW, '    needs: check\n    if: needs.check.outputs.go == \'true\'\n', '    needs: check\n    if: needs.check.outputs.go == \'true\'\n    env:\n      T: ${{ secrets[\'CLOUDFLARE_API_TOKEN\'] }}\n')],
    ['L2', 'the token on a third step', () => plant(WORKFLOW, '          DEPLOY: ${{ needs.deploy.result }}\n', '          DEPLOY: ${{ needs.deploy.result }}\n          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}\n')],
    ['L3', 'npm ci with install scripts on', () => plant(WORKFLOW, 'npm ci --ignore-scripts --no-audit --no-fund', 'npm ci --no-audit --no-fund')],
    ['L3', 'npm ci with install scripts turned back on', () => plant(WORKFLOW, 'npm ci --ignore-scripts --no-audit --no-fund', 'npm ci --ignore-scripts=false --no-audit --no-fund')],
    ['L3', 'npm rebuild', () => plant(WORKFLOW, '          cd "$RUNNER_TEMP/wrangler"\n', '          cd "$RUNNER_TEMP/wrangler"\n          npm rebuild\n')],
    ['L3', 'npm install by another spelling', () => plant(WORKFLOW, '          cd "$RUNNER_TEMP/wrangler"\n', '          cd "$RUNNER_TEMP/wrangler"\n          npm --prefix . install\n')],
    ['L5', 'a checkout of the pull request\'s commit', () => plant(WORKFLOW, '          persist-credentials: false\n', '          persist-credentials: false\n          ref: ${{ github.event.workflow_run.head_sha }}\n', { count: 5, nth: 2 })],
    ['L0', 'a quoted key', () => plant(WORKFLOW, '        run: node hosting/preview.mjs find-pr', '        "run": node hosting/preview.mjs find-pr')],
    ['L0', 'a step as a flow mapping', () => plant(WORKFLOW, '    steps:\n', '    steps:\n      - { name: x, run: echo hi }\n', { count: 5, nth: 1 })],
    ['L0', 'a space before a key\'s colon', () => plant(WORKFLOW, '        run: node hosting/preview.mjs find-pr', '        run : node hosting/preview.mjs find-pr')],
    ['L3', 'npx', () => plant(WORKFLOW, '          cd "$RUNNER_TEMP/wrangler"\n', '          cd "$RUNNER_TEMP/wrangler"\n          npx wrangler --version\n')],
    ['L4', 'a job without cache-mode: none', () => plant(WORKFLOW, '    if: github.event_name == \'pull_request\'\n    runs-on: ubuntu-24.04\n    timeout-minutes: 10\n    cache-mode: none\n', '    if: github.event_name == \'pull_request\'\n    runs-on: ubuntu-24.04\n    timeout-minutes: 10\n')],
    ['L4', 'setup-node with its cache on', () => plant(WORKFLOW, 'package-manager-cache: false', 'package-manager-cache: true', { count: 5, nth: 3 })],
    ['L4', 'a cache input', () => plant(WORKFLOW, '          package-manager-cache: false\n', '          package-manager-cache: false\n          cache: npm\n', { count: 5, nth: 2 })],
    ['L4', 'actions/cache', () => plant(WORKFLOW, '    steps:\n', `    steps:\n      - uses: actions/cache@${'0'.repeat(40)} # v4\n        with:\n          path: x\n          key: y\n`, { count: 5, nth: 4 })],
    ['L5', 'a checkout that keeps its credentials', () => plant(WORKFLOW, 'persist-credentials: false', 'persist-credentials: true', { count: 5, nth: 1 })],
    ['L5', 'an action pinned to a tag', () => plant(WORKFLOW, 'actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1', 'actions/checkout@v7', { count: 5, nth: 2 })],
    ['L5', 'permissions widened', () => plant(WORKFLOW, '\npermissions: {}\n', '\npermissions: write-all\n')],
    ['L0', 'a tab', () => plant(WORKFLOW, '    timeout-minutes: 10\n', '\ttimeout-minutes: 10\n', { count: 5, nth: 1 })],
    // Review round 2 (the test-engineer's N1 to N3): a command chained after npm ci, install scripts switched back on,
    // npm configuration, the token's places by step, and a block header carrying a comment.
    ['L3', 'npm install chained after npm ci', () => plant(WORKFLOW, NPM_LINE, `${NPM_LINE}; npm install left-pad`)],
    ['L3', 'npm exec chained after npm ci', () => plant(WORKFLOW, NPM_LINE, `${NPM_LINE} && npm exec wrangler -- --version`)],
    ['L3', 'npm rebuild chained after npm ci', () => plant(WORKFLOW, NPM_LINE, `${NPM_LINE} && npm rebuild`)],
    ['L3', 'the flag switched off again by =false', () => plant(WORKFLOW, NPM_LINE, 'npm ci --ignore-scripts --ignore-scripts=false --no-audit --no-fund')],
    ['L3', 'the flag switched off again by its negation', () => plant(WORKFLOW, NPM_LINE, 'npm ci --ignore-scripts --no-ignore-scripts --no-audit --no-fund')],
    ['L3', 'the flag switched off again by the value false', () => plant(WORKFLOW, NPM_LINE, 'npm ci --ignore-scripts false --no-audit --no-fund')],
    ['L3', 'npm ci continued onto a second line', () => plant(WORKFLOW, NPM_LINE, `${NPM_LINE} \\\n            --no-ignore-scripts`)],
    ['L3', 'npm_config_ignore_scripts on the install step', () => onInstall('          npm_config_ignore_scripts: \'false\'\n')],
    ['L3', 'NPM_CONFIG_IGNORE_SCRIPTS workflow-wide', () => plant(WORKFLOW, METRICS, `${METRICS}  NPM_CONFIG_IGNORE_SCRIPTS: "false"\n`)],
    ['L3', 'an .npmrc written before npm ci', () => plant(WORKFLOW, `          ${NPM_LINE}\n`, `          echo "ignore-scripts=false" > .npmrc\n          ${NPM_LINE}\n`)],
    ['L2', 'the token moved from the token check to the install step', () => onInstall(TOKEN, plant(WORKFLOW, TOKEN, '', { count: 3, nth: 1 }))],
    ['L2', 'both secrets moved from the token check to the install step', () => onInstall(TOKEN + ACCOUNT, verifyBare())],
    ['L2', 'both secrets moved to the install step, with verify-token appended to it', () => plant(onInstall(TOKEN + ACCOUNT, verifyBare()), `          ${NPM_LINE}\n`, `          ${NPM_LINE}\n          node hosting/preview.mjs verify-token\n`)],
    ['L2', 'the token by index on a third step', () => onInstall('          T: ${{ secrets[\'CLOUDFLARE_API_TOKEN\'] }}\n')],
    ['L2', 'every secret on a third step through toJSON', () => onInstall('          ALL: ${{ toJSON(secrets) }}\n')],
    ['L2', 'the token in capitals on a third step', () => onInstall('          T: ${{ SECRETS.CLOUDFLARE_API_TOKEN }}\n')],
    ['L2', 'the token in capitals on a planned step', () => plant(WORKFLOW, TOKEN, '          CLOUDFLARE_API_TOKEN: ${{ SECRETS.CLOUDFLARE_API_TOKEN }}\n', { count: 3, nth: 1 })],
    ['L2', 'a command appended to the upload', () => plant(WORKFLOW, UPLOAD_END, `${UPLOAD_END}          printenv CLOUDFLARE_ACCOUNT_ID\n`)],
    ['L2', 'every secret on a planned step through toJSON', () => plant(WORKFLOW, ACCOUNT, ACCOUNT + '          ALL: ${{ toJSON(secrets) }}\n', { count: 3, nth: 3 })],
    ['L2', 'the token taken off the upload step', () => plant(WORKFLOW, TOKEN, '', { count: 3, nth: 2 })],
    ['L1', '${{ }} in a shell comment under a block header that carries a comment', () => plant(WORKFLOW, '        run: |\n          mkdir -p "$RUNNER_TEMP/wrangler"', '        run: | # c\n          # ${{ github.event.workflow_run.head_branch }}\n          mkdir -p "$RUNNER_TEMP/wrangler"')],
  ];
  for (const [rule, name, make] of planted) {
    test(`planted: ${name} (${rule})`, () => {
      const problems = lintWorkflow(make());
      assert.ok(problems.some((p) => p.startsWith(`${rule} `)), `${rule} not caught: ${JSON.stringify(problems)}`);
    });
  }
});

// ---------- PHC-PREVIEW-NOINDEX: the header file ----------
function headerRules(text) {
  const rules = [];
  let current = null;
  for (const raw of text.split('\n')) {
    if (raw.trim() === '' || raw.trimStart().startsWith('#')) continue;
    if (!/^\s/.test(raw)) { current = { pattern: raw.trim(), headers: [] }; rules.push(current); continue; }
    const m = /^\s+([A-Za-z0-9-]+):\s*(.*)$/.exec(raw);
    assert.ok(m && current, `an unreadable header line: ${JSON.stringify(raw)}`);
    current.headers.push([m[1].toLowerCase(), m[2].trim()]);
  }
  return rules;
}
function headerProblems(text) {
  const rules = headerRules(text);
  const problems = [];
  const has = (pattern, name, word) => rules.some((r) => r.pattern === pattern && r.headers.some(([n, v]) => n === name && new RegExp(`\\b${word}\\b`, 'i').test(v)));
  if (!has('/*', 'x-robots-tag', 'noindex')) problems.push('no X-Robots-Tag noindex on /*');
  if (!has('/assets/*', 'cache-control', 'immutable')) problems.push('no immutable Cache-Control on /assets/*');
  for (const r of rules) {
    if (r.pattern !== '/assets/*' && r.headers.some(([n, v]) => n === 'cache-control' && /\bimmutable\b/i.test(v))) problems.push(`immutable on ${r.pattern}`);
    if (r.headers.some(([n]) => n === 'cross-origin-opener-policy' || n === 'cross-origin-embedder-policy')) problems.push(`COOP or COEP on ${r.pattern}`);
  }
  if (rules.length > 100) problems.push('more than 100 rules (the host\'s limit)');
  if (text.split('\n').some((l) => l.length > 2000)) problems.push('a line longer than 2,000 characters (the host\'s limit)');
  return problems;
}
describe('the header file (PHC-PREVIEW-NOINDEX)', () => {
  const text = HEADERS.toString('utf8');
  test('hosting/_headers: noindex on every path, immutable only on the hashed files', () => {
    assert.deepEqual(headerProblems(text), []);
  });
  test('planted: the noindex line removed', () => {
    assert.ok(headerProblems(plant(text, '  X-Robots-Tag: noindex\n', '')).includes('no X-Robots-Tag noindex on /*'));
  });
  test('planted: every path cached for good', () => {
    assert.ok(headerProblems(plant(text, '  X-Robots-Tag: noindex\n', '  X-Robots-Tag: noindex\n  Cache-Control: public, max-age=31536000, immutable\n'))
      .includes('immutable on /*'));
  });
  test('planted: COEP on every path', () => {
    assert.ok(headerProblems(plant(text, '  X-Robots-Tag: noindex\n', '  X-Robots-Tag: noindex\n  Cross-Origin-Embedder-Policy: require-corp\n'))
      .includes('COOP or COEP on /*'));
  });
});
