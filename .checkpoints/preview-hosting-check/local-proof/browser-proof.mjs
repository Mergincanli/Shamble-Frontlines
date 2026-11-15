// LOCAL PROOF ONLY: opens each built mode in the preinstalled Chromium (Playwright), served by serve-headers.mjs on
// 127.0.0.1 (a secure context, so isolation can switch on). It shows the page finishes, its worker reports, the
// WebAssembly runs, crossOriginIsolated follows the mode, and the content file's second visit transfers 0 bytes when
// cached for good and more than 0 with no-cache. It proves the page and the scripts' logic, not Cloudflare and not the
// branded browsers of GitHub's machines (the browser workflow does that).
//   node browser-proof.mjs <scratch folder>
import { createRequire } from 'node:module';
import { join } from 'node:path';
import { startServer } from './serve-headers.mjs';

const { chromium } = createRequire(import.meta.url)('playwright');
const builds = join(process.argv[2], 'builds');
const WANT = { production: 'false/false', preview: 'false/false', 'preview-isolated': 'true/true', 'bite-nocache': 'false/false' };

const routes = {};
const server = await startServer(routes);
const host = `127.0.0.1:${server.address().port}`;
const browser = await chromium.launch();
const results = [];

async function open(context, url) {
  const page = await context.newPage();
  await page.goto(url);
  await page.waitForFunction(() => document.documentElement.dataset.done, null, { timeout: 15000 });
  return page;
}

for (const mode of ['production', 'preview', 'preview-isolated', 'bite-coep', 'bite-nocache']) {
  routes[host] = { kind: 'build', dir: join(builds, mode) };
  const context = await browser.newContext();
  const url = `http://${host}/`;
  const page = await open(context, url);
  const seen = await page.evaluate(() => ({
    page: document.getElementById('coi-page').textContent, worker: document.getElementById('coi-worker').textContent,
    wasm: document.getElementById('wasm').textContent, status: document.getElementById('status').textContent,
    resources: JSON.parse(document.getElementById('resources').textContent || '{}'),
  }));
  const row = { mode, coi: `${seen.page}/${seen.worker}`, wasm: seen.wasm, status: seen.status,
    resources: [...(seen.resources.page || []), ...(seen.resources.worker || [])].join(' ') };
  if (mode === 'production' || mode === 'bite-nocache') {
    const second = await context.newPage();
    await page.close();
    await second.goto(url);
    await second.waitForFunction(() => document.documentElement.dataset.done, null, { timeout: 15000 });
    row.secondVisitTransfer = await second.evaluate(() => {
      const e = performance.getEntriesByType('resource').find((r) => /\/content\.[0-9a-f]{8}\.bin$/.test(new URL(r.name).pathname));
      return e ? e.transferSize : null;
    });
  }
  if (mode === 'bite-coep') row.ok = row.coi !== 'true/true';
  else row.ok = row.coi === WANT[mode] && row.wasm.startsWith('ran (2 + 3 = 5)') && row.status === 'done'
    && (mode !== 'production' || row.secondVisitTransfer === 0) && (mode !== 'bite-nocache' || row.secondVisitTransfer > 0);
  results.push(row);
  await context.close();
}

await browser.close();
server.close();
for (const r of results) console.log(`${r.ok ? 'OK  ' : 'FAIL'} ${r.mode}: crossOriginIsolated ${r.coi}; wasm ${r.wasm}; status ${r.status}${'secondVisitTransfer' in r ? `; second visit transferSize ${r.secondVisitTransfer}` : ''}\n       resources: ${r.resources}`);
console.log(`browser proof (Chromium ${browser.version()}): ${results.filter((r) => r.ok).length} of ${results.length} as expected`);
process.exit(results.every((r) => r.ok) ? 0 : 1);
