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
import { mkdirSync, writeFileSync, existsSync, readdirSync, readFileSync } from 'node:fs';
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

// run 4 (the advisor agent's C7): both console lines of saves-console-lines.md (read from that file, never retyped), in
// their own fresh profile, on the real legacy/index.html and legacy/editor.html, the pages Mustafa will paste them on.
// DevTools' copy() does not exist in a page, so a stub records what it was given. The game's own keys count too.
{
  const md = readFileSync(join(REPO, '.checkpoints/foundation-restructure/saves-console-lines.md'), 'utf8');
  const lines = [...md.matchAll(/```js\n(.*)\n```/g)].map(m => m[1]);
  if (lines.length !== 2) throw new Error(`expected 2 console lines in saves-console-lines.md, found ${lines.length}`);
  const [BACKUP, RESTORE] = lines;
  const PROFILE2 = join(W, 'profile-lines');
  mkdirSync(PROFILE2, { recursive: true });
  const SAVES = { shamble_prog: JSON.stringify({ xp: 1234, rank: 3, name: 'Müstafa "the" tester' }),
    'vmap:Cabana copy': JSON.stringify({ boxes: [[1, 2, 3]], note: 'line1\nline2\ttab   sep' }), 'vmap:big': 'x'.repeat(300000) };
  const cl = out.consoleLines = { profile: PROFILE2, checks: [] };
  const ck = (name, pass, detail = '') => { cl.checks.push({ name, pass: !!pass, detail }); console.log(`${pass ? 'PASS' : 'FAIL'} console lines: ${name}${detail ? ' :: ' + detail : ''}`); };
  const snap = p => p.evaluate(() => Object.fromEntries(Object.keys(localStorage).map(k => [k, localStorage.getItem(k)])));
  const stub = p => p.evaluate(() => { window.copy = t => { window.__copied = t; }; });
  const parse = t => { try { return JSON.parse(t); } catch { return null; } };
  ctx = await chromium.launchPersistentContext(PROFILE2, { headless: true, args: ARGS });
  page = await open(ctx, join(SITE, 'legacy', 'index.html'));
  await page.waitForFunction(() => typeof KVStore !== 'undefined', null, { timeout: 30000 });
  await page.evaluate(async (s) => { for (const [k, v] of Object.entries(s)) await KVStore.set(k, v); }, SAVES);
  await stub(page);
  const s1 = await snap(page);
  const said1 = await page.evaluate(BACKUP);
  const b1 = parse(await page.evaluate(() => window.__copied));
  ck('backup on legacy/index.html answers with the key count', said1 === `${Object.keys(s1).length} keys copied: paste them into a text file now`, said1);
  ck('its text holds every key, values byte for byte', b1 && JSON.stringify(Object.keys(b1).sort()) === JSON.stringify(Object.keys(s1).sort()) && Object.entries(s1).every(([k, v]) => b1[k] === v), `${Object.keys(s1).length} keys: ${Object.keys(s1).sort().join(', ').slice(0, 160)}`);
  ck('the seeded saves are in it (quotes, newlines, non-ASCII, 300 KB)', b1 && Object.entries(SAVES).every(([k, v]) => b1[k] === v));
  await page.close();
  page = await open(ctx, join(SITE, 'legacy', 'editor.html'));
  await page.waitForFunction(() => typeof KVStore !== 'undefined', null, { timeout: 30000 });
  await stub(page);
  const said2 = await page.evaluate(BACKUP);
  const b2 = parse(await page.evaluate(() => window.__copied));
  ck('backup on legacy/editor.html holds the same saves', b2 && Object.entries(SAVES).every(([k, v]) => b2[k] === v), said2);
  // one save lost and one changed since the backup; restore on legacy/index.html must bring back the lost one only
  await page.evaluate(() => { localStorage.removeItem('vmap:Cabana copy'); localStorage.setItem('shamble_prog', 'NEWER'); });
  await page.close();
  page = await open(ctx, join(SITE, 'legacy', 'index.html'));
  await page.waitForFunction(() => typeof KVStore !== 'undefined', null, { timeout: 30000 });
  const pre = await snap(page);
  ck('before the restore the lost save is missing', !('vmap:Cabana copy' in pre));
  const said3 = await page.evaluate(RESTORE.replace('PASTE_HERE', () => JSON.stringify(b1)));
  const missing = Object.keys(b1).filter(k => !(k in pre)).length;
  ck('restore answers with its counts', said3 === `${missing} keys restored, ${Object.keys(b1).length - missing} already there (left as they are)`, said3);
  const post = await snap(page);
  ck('the lost save is back, byte for byte', post['vmap:Cabana copy'] === SAVES['vmap:Cabana copy']);
  ck('an existing save is never overwritten', post.shamble_prog === pre.shamble_prog, `shamble_prog: ${String(pre.shamble_prog).slice(0, 20)}`);
  ck('nothing else changed', Object.entries(pre).every(([k, v]) => post[k] === v) && Object.keys(post).every(k => k in pre || k in b1));
  await page.reload({ waitUntil: 'load' });
  await page.waitForFunction(() => typeof KVStore !== 'undefined', null, { timeout: 30000 });
  ck('after a reload the game reads the restored save through KVStore', ((await page.evaluate(async () => KVStore.get('vmap:Cabana copy'))) || {}).value === SAVES['vmap:Cabana copy']);
  await ctx.close();
  cl.verdict = cl.checks.every(c => c.pass) ? 'PASS' : 'FAIL';
  console.log('E5 console lines: ' + cl.verdict);
}

const r2 = out.runs.filter(r => r.run === 2);
const control = r2[0].read === 'written-by-root-index';
const all = r2.every(r => r.read === 'written-by-root-index');
const sameFolder = r2[1].read === 'written-by-root-index';
out.verdict = !control ? 'NO RESULT: the control failed (the write did not survive a relaunch)'
  : all && out.runs.at(-1).read === 'written-by-legacy-index' ? 'SHARED: every file:// page, in any folder, reads the same storage'
  : sameFolder ? 'PER FOLDER' : 'PER FILE';
console.log('E5 verdict: ' + out.verdict);
writeFileSync(join(REPO, '.checkpoints/foundation-restructure/e5-result.json'), JSON.stringify(out, null, 1) + '\n');
