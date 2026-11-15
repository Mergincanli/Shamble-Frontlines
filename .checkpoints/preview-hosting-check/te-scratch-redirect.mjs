// test-engineer scratch: does check-headers fail when the host 308s *.html, and does probe call 429 'gone'?
// usage: node te-scratch-redirect.mjs <test repo> <builds dir>
import { createServer } from 'node:http';
import { execFile } from 'node:child_process';
import { promisify } from 'node:util';
import { mkdirSync, writeFileSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { startServer } from './local-proof/serve-headers.mjs';
const run = promisify(execFile);
const [repo, scratch] = process.argv.slice(2);
const P = 'shamble-preview-check.pages.dev';
const shim = new URL('./local-proof/fetch-shim.mjs', import.meta.url).href;
const inner = await startServer({ [`iso-off.${P}`]: { kind: 'build', dir: join(scratch, 'builds', 'preview') }, [`bbbb2222.${P}`]: { kind: 'build', dir: join(scratch, 'builds', 'preview') } });
const ip = inner.address().port;
const outer = createServer(async (req, res) => {
  const host = req.headers['x-test-host'];
  const path = new URL(req.url, 'http://x').pathname;
  if (host.startsWith('rl.')) { res.writeHead(429, { 'content-type': 'text/plain' }); res.end('rate limited'); return; }
  if (path.endsWith('.html')) { res.writeHead(308, { location: path.replace(/(index)?\.html$/, '') }); res.end(); return; }
  const r = await fetch(`http://127.0.0.1:${ip}${path}`, { headers: { 'x-test-host': host } });
  const h = {}; r.headers.forEach((v, k) => { h[k] = v; });
  res.writeHead(r.status, h); res.end(Buffer.from(await r.arrayBuffer()));
});
await new Promise((ok) => outer.listen(0, '127.0.0.1', ok));
const env = { ...process.env, LOCAL_PROOF_PORT: String(outer.address().port) };
const d = join(scratch, 'te'); mkdirSync(join(d, 'rec'), { recursive: true });
const rec = { deployment_id: 'v', url: `https://bbbb2222.${P}`, alias: `https://iso-off.${P}`, environment: 'preview', branch: 'iso-off', mode: 'preview', expect: 'preview', marker: 'm1', leg: 'iso-off' };
writeFileSync(join(d, 'rec', 'r.json'), JSON.stringify(rec));
const r1 = await run(process.execPath, ['--import', shim, join(repo, 'tools/check-headers.mjs'), '--records', join(d, 'rec'), '--out', join(d, 'h.json')], { env }).catch((e) => e);
console.log('check-headers with .html 308: exit', r1.code ?? 0); console.log((r1.stdout || '').split('\n').filter((l) => /FAIL|PASS|html|summary/.test(l)).join('\n'));
mkdirSync(join(d, 'prec'), { recursive: true });
writeFileSync(join(d, 'prec', 'r.json'), JSON.stringify({ ...rec, url: `https://rl1.${P}`, alias: `https://rl.${P}`, branch: 'cs/pd-one' }));
writeFileSync(join(d, 'pulls.ndjson'), JSON.stringify({ number: 1, head: 'cs/pd-one', state: 'closed', closed_at: '2026-11-14T00:00:00Z', merged_at: null }));
const r2 = await run(process.execPath, ['--import', shim, join(repo, 'tools/probe.mjs'), '--records', join(d, 'prec'), '--pulls', join(d, 'pulls.ndjson'), '--purpose', 'deletion', '--out', join(d, 'p.json')], { env }).catch((e) => e);
console.log('probe exit', r2.code ?? 0); for (const row of JSON.parse(readFileSync(join(d, 'p.json'), 'utf8')).rows) console.log(row.url, row.status, row.outcome, row.as_expected);
inner.close(); outer.close();
