// LOCAL PROOF ONLY: shows that check-headers.mjs passes the good modes and fails every planted mistake, and that
// probe.mjs sorts every kind of answer as the plan says, against serve-headers.mjs (this chat's reading of Cloudflare's
// _headers rules). It proves the scripts, not Cloudflare.
//   node proof.mjs <test-repository folder> <scratch folder>
import { execFile } from 'node:child_process';
import { mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
import { promisify } from 'node:util';
import { startServer } from './serve-headers.mjs';

const run = promisify(execFile);
const [repo, scratch] = process.argv.slice(2);
const shim = pathToFileURL(new URL('./fetch-shim.mjs', import.meta.url).pathname).href;
const P = 'shamble-preview-check.pages.dev';
const M = '0123456789abcdef0123456789abcdef01234567';
const builds = join(scratch, 'builds');

async function buildAll() {
  for (const mode of ['production', 'preview', 'preview-isolated', 'bite-coep', 'bite-noindex', 'bite-nocache']) {
    await run(process.execPath, [join(repo, 'tools/build.mjs'), '--mode', mode, '--marker', M, '--out', mode], { cwd: builds });
  }
}

async function script(port, name, args) {
  try {
    const { stdout } = await run(process.execPath, ['--import', shim, join(repo, 'tools', name), ...args],
      { env: { ...process.env, LOCAL_PROOF_PORT: String(port) }, maxBuffer: 1 << 24 });
    return { code: 0, stdout };
  } catch (err) {
    return { code: err.code, stdout: err.stdout || '', stderr: err.stderr || '' };
  }
}

const results = [];
const record = (o) => ({ marker: M, made: '2026-11-15T00:00:00Z', ...o });

// wantPaths(forms) -> true when every failure is the planted one (TE-2: a bite counts only on its planted response).
async function headerCase(server, label, rec, wantCode, wantPaths = () => true) {
  const dir = join(scratch, 'proof', label);
  rmSync(dir, { recursive: true, force: true });
  mkdirSync(join(dir, 'records'), { recursive: true });
  writeFileSync(join(dir, 'records', 'r.json'), JSON.stringify(rec));
  const r = await script(server.address().port, 'check-headers.mjs', ['--records', join(dir, 'records'), '--out', join(dir, 'headers.json')]);
  const out = JSON.parse(readFileSync(join(dir, 'headers.json'), 'utf8'));
  const why = out.records.flatMap((x) => x.forms.flatMap((f) => [`${f.form}:${f.result}(X-Check-Rule ${f.xCheckRule})`,
    ...f.responses.flatMap((resp) => resp.failures.map((w) => `  ${f.form} ${resp.path}: ${w}`))]));
  const forms = out.records.flatMap((x) => x.forms);
  const pathsOk = wantPaths(forms);
  results.push({ check: `check-headers ${label}`, want: `exit ${wantCode}, planted failures only`, got: `exit ${r.code}, paths ${pathsOk ? 'as planted' : 'NOT as planted'}`, ok: r.code === wantCode && pathsOk, detail: why });
}
const onlyOn = (path, re) => (forms) => forms.length > 0 && forms.every((f) => f.result === 'fail' && f.failure_paths.length === 1
  && f.failure_paths[0].path === path && f.failure_paths[0].failures.length === 1 && re.test(f.failure_paths[0].failures[0]));
const everyFailure = (re) => (forms) => forms.some((f) => f.result === 'fail') && forms.every((f) => f.failure_paths.every((x) => x.failures.every((w) => re.test(w))));

async function main() {
  mkdirSync(builds, { recursive: true });
  await buildAll();
  const b = (m) => ({ kind: 'build', dir: join(builds, m) });

  // Run 1: one route per link, the host's default noindex not emulated.
  let server = await startServer({
    [P]: b('production'), [`aaaa1111.${P}`]: b('production'),
    [`iso-off.${P}`]: b('preview'), [`bbbb2222.${P}`]: b('preview'),
    [`iso-on.${P}`]: b('preview-isolated'), [`cccc3333.${P}`]: b('preview-isolated'),
    [`bite-coep.${P}`]: b('bite-coep'), [`dddd4444.${P}`]: b('bite-coep'),
    [`bite-nocache.${P}`]: b('bite-nocache'), [`eeee5555.${P}`]: b('bite-nocache'),
  });
  await headerCase(server, 'production', record({ deployment_id: 'p', url: `https://aaaa1111.${P}`, alias: null, environment: 'production', branch: 'main', mode: 'production', expect: 'production', leg: 'production' }), 0);
  await headerCase(server, 'preview', record({ deployment_id: 'v', url: `https://bbbb2222.${P}`, alias: `https://iso-off.${P}`, environment: 'preview', branch: 'iso-off', mode: 'preview', expect: 'preview', leg: 'iso-off' }), 0);
  await headerCase(server, 'preview-isolated', record({ deployment_id: 'i', url: `https://cccc3333.${P}`, alias: `https://iso-on.${P}`, environment: 'preview', branch: 'iso-on', mode: 'preview-isolated', expect: 'preview-isolated', leg: 'iso-on' }), 0);
  await headerCase(server, 'bite-coep (must fail, on /worker.js COEP only)', record({ deployment_id: 'c', url: `https://dddd4444.${P}`, alias: `https://bite-coep.${P}`, environment: 'preview', branch: 'bite-coep', mode: 'bite-coep', expect: 'preview-isolated', leg: 'bite-coep' }), 1, onlyOn('/worker.js', /^COEP ""/));
  await headerCase(server, 'bite-nocache (must fail, on the content file only)', record({ deployment_id: 'n', url: `https://eeee5555.${P}`, alias: `https://bite-nocache.${P}`, environment: 'preview', branch: 'bite-nocache', mode: 'bite-nocache', expect: 'preview', leg: 'bite-nocache' }), 1, onlyOn('/content.ef463692.bin', /immutable/));
  server.close();

  // A stale stable link (it serves another mode than the record): no reading, never a pass or a failure (TE-6).
  server = await startServer({ [`iso-off.${P}`]: b('preview-isolated'), [`bbbb2222.${P}`]: b('preview') });
  await headerCase(server, 'stale stable link (must be no reading)', record({ deployment_id: 's', url: `https://bbbb2222.${P}`, alias: `https://iso-off.${P}`, environment: 'preview', branch: 'iso-off', mode: 'preview', expect: 'preview', leg: 'iso-off' }), 3,
    (forms) => forms.find((f) => f.form === 'stable')?.result === 'no-reading' && forms.find((f) => f.form === 'one-off')?.result === 'pass');
  server.close();

  // No record at all (the "revoked" run): exit 3, never a green pass (TE-5).
  {
    const dir = join(scratch, 'proof', 'no-records');
    rmSync(dir, { recursive: true, force: true });
    mkdirSync(join(dir, 'records'), { recursive: true });
    const r = await script(1, 'check-headers.mjs', ['--records', join(dir, 'records'), '--out', join(dir, 'headers.json')]);
    results.push({ check: 'check-headers with no record', want: 'exit 3', got: `exit ${r.code}`, ok: r.code === 3 });
  }

  // Run 2: production carries the planted noindex.
  server = await startServer({ [P]: b('bite-noindex'), [`ffff6666.${P}`]: b('bite-noindex') });
  await headerCase(server, 'bite-noindex (must fail, on noindex only)', record({ deployment_id: 'x', url: `https://ffff6666.${P}`, alias: null, environment: 'production', branch: 'main', mode: 'bite-noindex', expect: 'production', leg: 'bite-noindex' }), 1, everyFailure(/noindex/));
  server.close();

  // Run 3: the host's own noindex emulated on every link with a label before the project (finding A5): production's
  // one-off link must then fail while its own address passes.
  server = await startServer({ [P]: b('production'), [`aaaa1111.${P}`]: b('production') }, { emulateHostNoindex: true });
  await headerCase(server, 'production, host noindex on one-off links (one-off must fail)', record({ deployment_id: 'p2', url: `https://aaaa1111.${P}`, alias: null, environment: 'production', branch: 'main', mode: 'production', expect: 'production', leg: 'production' }), 1,
    (forms) => forms.find((f) => f.form === 'stable')?.result === 'pass' && everyFailure(/noindex/)(forms.filter((f) => f.form === 'one-off')));
  server.close();

  // The probe: one route per kind of answer.
  const pdir = join(scratch, 'proof', 'probe');
  rmSync(pdir, { recursive: true, force: true });
  mkdirSync(join(pdir, 'records', '1'), { recursive: true });
  const recs = [
    record({ deployment_id: 'a1', url: `https://gggg7777.${P}`, alias: `https://cs-pd-one.${P}`, environment: 'preview', branch: 'cs/pd-one', mode: 'preview', expect: 'preview' }),
    record({ deployment_id: 'c1', url: `https://hhhh8888.${P}`, alias: `https://cs-pd-open.${P}`, environment: 'preview', branch: 'cs/pd-open', mode: 'preview', expect: 'preview' }),
    record({ deployment_id: 'd1', url: `https://iiii9999.${P}`, alias: `https://cs-pd-off.${P}`, environment: 'preview', branch: 'cs/pd-off', mode: 'preview', expect: 'preview' }),
    record({ deployment_id: 't1', url: `https://jjjj0000.${P}`, alias: `https://cs-pdt-one.${P}`, environment: 'preview', branch: 'cs/pdt-one', mode: 'preview', expect: 'preview' }),
    record({ deployment_id: 't2', url: `https://kkkk1111.${P}`, alias: `https://cs-pdt-one.${P}`, environment: 'preview', branch: 'cs/pdt-one', marker: '', mode: 'tombstone', expect: 'none' }),
    record({ deployment_id: 'b1', url: `https://llll2222.${P}`, alias: `https://blocked.${P}`, environment: 'preview', branch: 'blocked', mode: 'preview', expect: 'preview' }),
    record({ deployment_id: 'e1', url: `https://mmmm3333.${P}`, alias: `https://broken.${P}`, environment: 'preview', branch: 'broken', mode: 'preview', expect: 'preview' }),
    record({ deployment_id: 'r1', url: `https://nnnn4444.${P}`, alias: `https://ratelimited.${P}`, environment: 'preview', branch: 'ratelimited', mode: 'preview', expect: 'preview' }),
    record({ deployment_id: 'h1', url: `https://oooo5555.${P}`, alias: `https://challenged.${P}`, environment: 'preview', branch: 'challenged', mode: 'preview', expect: 'preview' }),
    record({ deployment_id: 'u1', url: `https://pppp6666.${P}`, alias: `https://cs-pd-401.${P}`, environment: 'preview', branch: 'cs/pd-401', mode: 'preview', expect: 'preview' }),
  ];
  recs.forEach((r, i) => writeFileSync(join(pdir, 'records', '1', `r${i}.json`), JSON.stringify(r)));
  const closed = '2026-11-14T00:00:00Z';
  writeFileSync(join(pdir, 'pulls.ndjson'), [
    { number: 1, head: 'cs/pd-one', state: 'closed', closed_at: closed, merged_at: null },
    { number: 2, head: 'cs/pd-open', state: 'open', closed_at: null, merged_at: null },
    { number: 3, head: 'cs/pd-off', state: 'closed', closed_at: closed, merged_at: null },
    { number: 4, head: 'cs/pdt-one', state: 'closed', closed_at: closed, merged_at: null },
    { number: 5, head: 'cs/pd-401', state: 'closed', closed_at: closed, merged_at: null },
  ].map((x) => JSON.stringify(x)).join('\n'));
  server = await startServer({
    [`gggg7777.${P}`]: { kind: '404' }, [`cs-pd-one.${P}`]: { kind: 'tombstone' },
    [`hhhh8888.${P}`]: b('preview'), [`cs-pd-open.${P}`]: b('preview'),
    [`iiii9999.${P}`]: b('preview'), [`cs-pd-off.${P}`]: b('preview'),
    [`jjjj0000.${P}`]: { kind: '404' }, [`cs-pdt-one.${P}`]: { kind: 'wall' }, [`kkkk1111.${P}`]: { kind: 'tombstone' },
    [`llll2222.${P}`]: { kind: 'block' }, [`blocked.${P}`]: { kind: 'block' },
    [`mmmm3333.${P}`]: { kind: 'error' }, [`broken.${P}`]: { kind: 'error' },
    [`nnnn4444.${P}`]: { kind: '429' }, [`ratelimited.${P}`]: { kind: '429' },
    [`oooo5555.${P}`]: { kind: 'challenge' }, [`challenged.${P}`]: { kind: 'challenge' },
    [`pppp6666.${P}`]: { kind: '401' }, [`cs-pd-401.${P}`]: { kind: '401' },
  });
  const pr = await script(server.address().port, 'probe.mjs', ['--records', join(pdir, 'records'), '--pulls', join(pdir, 'pulls.ndjson'), '--purpose', 'deletion', '--out', join(pdir, 'probe.json')]);
  server.close();
  results.push({ check: 'probe exit code with no-reading rows', want: 'exit 3', got: `exit ${pr.code}`, ok: pr.code === 3 });
  const rows = JSON.parse(readFileSync(join(pdir, 'probe.json'), 'utf8')).rows;
  const want = {
    [`https://gggg7777.${P}/`]: ['gone', true], [`https://cs-pd-one.${P}/`]: ['gone', true],
    [`https://hhhh8888.${P}/`]: ['serves', true], [`https://cs-pd-open.${P}/`]: ['serves', true],
    [`https://iiii9999.${P}/`]: ['serves', true], [`https://cs-pd-off.${P}/`]: ['serves', true],
    [`https://jjjj0000.${P}/`]: ['gone', true], [`https://cs-pdt-one.${P}/`]: ['wall', true], [`https://kkkk1111.${P}/`]: ['gone', true],
    [`https://llll2222.${P}/`]: ['no-reading', null], [`https://blocked.${P}/`]: ['no-reading', null],
    [`https://mmmm3333.${P}/`]: ['no-reading', null], [`https://broken.${P}/`]: ['no-reading', null],
    [`https://nnnn4444.${P}/`]: ['no-reading', null], [`https://ratelimited.${P}/`]: ['no-reading', null],
    [`https://oooo5555.${P}/`]: ['no-reading', null], [`https://challenged.${P}/`]: ['no-reading', null],
    [`https://pppp6666.${P}/`]: ['other', true], [`https://cs-pd-401.${P}/`]: ['other', true],
  };
  for (const [url, [outcome, asExpected]] of Object.entries(want)) {
    const row = rows.find((x) => x.url === url);
    const got = row ? `${row.outcome}/${row.as_expected}` : 'missing';
    results.push({ check: `probe ${url.replace(`.${P}/`, '')}`, want: `${outcome}/${asExpected}`, got, ok: got === `${outcome}/${asExpected}` });
  }

  for (const r of results) {
    console.log(`${r.ok ? 'OK  ' : 'FAIL'} ${r.check}: want ${r.want}, got ${r.got}`);
    for (const d of r.detail || []) console.log(`       ${d}`);
  }
  const failed = results.filter((r) => !r.ok).length;
  console.log(`local proof: ${results.length - failed} of ${results.length} as expected`);
  process.exit(failed ? 1 : 0);
}

main().catch((err) => { console.error(err); process.exit(2); });
