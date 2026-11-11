// e5-saves.mjs (foundation-restructure, step 2.1b): evidence E5, do saves carry over when the game moves into legacy/?
// One persistent Chromium profile (the tools' Playwright 1.56.1, the pinned Chromium 1194), file:// pages, as a player
// opens them by double-click. Run 1: <site>/index.html writes a KVStore key (backend asserted 'localStorage').
// Run 2 (the browser relaunched on the same profile): <site>/index.html reads it back (the control: without it the test
// proves nothing), then <site>/editor.html, <site>/legacy/index.html, <site>/legacy/editor.html and a page in an
// unrelated folder. Run 3: the reverse, legacy/index.html writes a second key and <site>/index.html reads it.
// The site: the root layout from A (76b4f7a) beside legacy/ from 2.1b's move commit (the five blobs are the same).
// Headless Chromium on Linux is not Brave on Windows: Mustafa's backup step stays whatever this says.
// Run from the repo root: node .gate/foundation-restructure/e5-saves.mjs <move-commit>
import { createRequire } from 'node:module';
import { mkdirSync, writeFileSync, existsSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
import { execFileSync } from 'node:child_process';

const REPO = '/home/user/Shamble-Frontlines';
const MOVE = process.argv[2];
if (!MOVE) { console.error('usage: e5-saves.mjs <move-commit>'); process.exit(2); }
const A = '76b4f7a';
const W = join(REPO, '.gate/foundation-restructure/e5');
const SITE = join(W, 'site'), OTHER = join(W, 'other'), PROFILE = join(W, 'profile');
if (existsSync(PROFILE) && readdirSync(PROFILE).length) { console.error('the profile folder is not empty: use a fresh .gate/foundation-restructure/e5'); process.exit(2); }
const require = createRequire(join(REPO, 'tools/package.json'));
const { chromium } = require('playwright');
const threeMin = require.resolve('three/build/three.min.js');
const THREE_URL = 'https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js';
const FIVE = ['shamble-core.js', 'shamble-game.js', 'shamble-editor.js', 'index.html', 'editor.html'];
const show = (ref) => execFileSync('git', ['show', ref], { cwd: REPO, maxBuffer: 64 << 20 });

mkdirSync(join(SITE, 'legacy'), { recursive: true }); mkdirSync(OTHER, { recursive: true }); mkdirSync(PROFILE, { recursive: true });
for (const f of FIVE) {
  writeFileSync(join(SITE, f), show(`${A}:${f}`));
  writeFileSync(join(SITE, 'legacy', f), show(`${MOVE}:legacy/${f}`));
}
writeFileSync(join(OTHER, 'probe.html'), '<!doctype html><meta charset="utf-8"><title>probe</title><script>window.__probe=1</script>\n');

const ARGS = ['--use-angle=swiftshader', '--enable-unsafe-swiftshader', '--ignore-gpu-blocklist'];
const url = p => pathToFileURL(p).href;
async function open(ctx, file) {
  const page = await ctx.newPage();
  await page.route('**/*', route => {
    const u = route.request().url();
    if (u === THREE_URL) return route.fulfill({ path: threeMin, contentType: 'application/javascript' });
    if (/^(file|data|blob):/i.test(u)) return route.continue();
    return route.abort();
  });
  await page.goto(url(file), { waitUntil: 'load', timeout: 30000 });
  return page;
}
const out = { chromium: chromium.executablePath(), move: MOVE, site: SITE, runs: [] };
const log = (o) => { out.runs.push(o); console.log(JSON.stringify(o)); };

// run 1: the root index.html writes
let ctx = await chromium.launchPersistentContext(PROFILE, { headless: true, args: ARGS });
let page = await open(ctx, join(SITE, 'index.html'));
await page.waitForFunction(() => typeof KVStore !== 'undefined', null, { timeout: 30000 });
log({ run: 1, page: 'index.html', backend: await page.evaluate(() => KVStore.backend),
  set: await page.evaluate(async () => { await KVStore.set('e5_probe', 'written-by-root-index'); return localStorage.getItem('e5_probe'); }) });
await ctx.close();

// run 2: relaunch on the same profile; every page reads
ctx = await chromium.launchPersistentContext(PROFILE, { headless: true, args: ARGS });
for (const [name, file, kv] of [
  ['index.html (control)', join(SITE, 'index.html'), true],
  ['editor.html', join(SITE, 'editor.html'), true],
  ['legacy/index.html', join(SITE, 'legacy', 'index.html'), true],
  ['legacy/editor.html', join(SITE, 'legacy', 'editor.html'), true],
  ['other/probe.html (unrelated folder)', join(OTHER, 'probe.html'), false],
]) {
  page = await open(ctx, file);
  if (kv) await page.waitForFunction(() => typeof KVStore !== 'undefined', null, { timeout: 30000 });
  const got = await page.evaluate(async (kv) => kv ? ((await KVStore.get('e5_probe')) || {}).value || null : localStorage.getItem('e5_probe'), kv);
  log({ run: 2, page: name, origin: await page.evaluate(() => location.origin), read: got });
  await page.close();
}
await ctx.close();

// run 3: the reverse, legacy/index.html writes; the root index.html reads after a relaunch
ctx = await chromium.launchPersistentContext(PROFILE, { headless: true, args: ARGS });
page = await open(ctx, join(SITE, 'legacy', 'index.html'));
await page.waitForFunction(() => typeof KVStore !== 'undefined', null, { timeout: 30000 });
log({ run: 3, page: 'legacy/index.html', set: await page.evaluate(async () => { await KVStore.set('e5_probe2', 'written-by-legacy-index'); return localStorage.getItem('e5_probe2'); }) });
await ctx.close();
ctx = await chromium.launchPersistentContext(PROFILE, { headless: true, args: ARGS });
page = await open(ctx, join(SITE, 'index.html'));
await page.waitForFunction(() => typeof KVStore !== 'undefined', null, { timeout: 30000 });
log({ run: 3, page: 'index.html after relaunch', read: await page.evaluate(async () => ((await KVStore.get('e5_probe2')) || {}).value || null) });
await ctx.close();

const r2 = out.runs.filter(r => r.run === 2);
const control = r2[0].read === 'written-by-root-index';
const all = r2.every(r => r.read === 'written-by-root-index');
const sameFolder = r2[1].read === 'written-by-root-index';
out.verdict = !control ? 'NO RESULT: the control failed (the write did not survive a relaunch)'
  : all && out.runs.at(-1).read === 'written-by-legacy-index' ? 'SHARED: every file:// page, in any folder, reads the same storage'
  : sameFolder ? 'PER FOLDER' : 'PER FILE';
console.log('E5 verdict: ' + out.verdict);
writeFileSync(join(REPO, '.checkpoints/foundation-restructure/e5-result.json'), JSON.stringify(out, null, 1) + '\n');
