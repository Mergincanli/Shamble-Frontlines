#!/usr/bin/env node
// tools/smoke.mjs: gate check 5a. Runs the real pages in headless Chromium (Playwright, SwiftShader), three r128 served from
// tools/node_modules, drives them with real clicks and keys, and FAILS on any page error or console error. Warnings are listed only.
//   index.html : menu -> Deploy (#mPlay) -> close the tutorial -> W held 1 s, two shots, ~5 s of play -> Esc (pause) -> Menu
//   editor.html: menu -> Map editor (#mEditor; fills the prefab list) -> Maps (#edMaps; fills the map list) -> close -> Menu
// Each step waits on game state with a generous timeout, never on exact timing, and checks that it did its job (the player
// moved, both clicks reached the canvas, the lists filled). Network requests and WebSockets are blocked and FAIL (except the
// three.js CDN URL, served locally), and so does reading a file outside the repo: the game must run offline from file://.
// Not seen: errors more than ~1 s after the last step, and code the script never reaches (5b covers names there).
// Usage (from anywhere): node tools/smoke.mjs [--json]   (gate.mjs runs it as check 5a)
// Exit code: 0 PASS, 1 FAIL, 2 setup error. Setup: cd tools && npm ci && npx --no -- playwright install chromium
import { createRequire } from 'node:module';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { dirname, resolve, join, sep } from 'node:path';
import { existsSync, realpathSync } from 'node:fs';

const TOOLS = dirname(fileURLToPath(import.meta.url));
const ROOT = resolve(TOOLS, '..');
const require = createRequire(import.meta.url);
const THREE_URL = 'https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js';
export const SMOKE_SETUP = 'cd tools && npm ci && npx --no -- playwright install chromium';
const STEP_TIMEOUT = 30000;   // per wait; a healthy one takes under 3 s on SwiftShader, ~10 s under heavy CPU load
const ROOT_URL = pathToFileURL(ROOT).href + '/';
const rel = s => String(s).split(ROOT_URL).join('');
const firstLine = s => String(s).split('\n')[0];
const frame = stack => { const l = String(stack || '').split('\n').find(x => /^\s+at /.test(x)); return l ? ' ' + rel(l.trim()) : ''; };
// Playwright's message, plus the last line of its call log (the reason, e.g. "element is not visible")
const why = e => { const l = String(e.message).split('\n').map(x => x.trim()).filter(Boolean); return l[0] + (l.length > 1 ? ' | ' + l[l.length - 1] : ''); };

// ---------- the two scripted sessions ----------
// step(name, fn): fn gets the page; a throw or timeout fails that step and ends the page's session.
const until = (page, fn, what, arg = null) => page.waitForFunction(fn, arg, { timeout: STEP_TIMEOUT, polling: 50 })
  .catch(() => { throw new Error('timed out waiting for ' + what); });
// visible, then a real mouse click at its centre. No wait for 'stable', so an animated control still gets clicked; the state
// check after each click proves it landed (a covering overlay would take the click instead, and that check fails).
const press = async (page, sel) => {
  await page.locator(sel).waitFor({ state: 'visible', timeout: STEP_TIMEOUT });
  await page.click(sel, { timeout: STEP_TIMEOUT, force: true });
};
const SESSIONS = {
  'index.html': async (step, page, info) => {
    await step('menu is up', async () => {
      await until(page, () => typeof mode !== 'undefined' && mode === 'menu', "mode === 'menu'");
      await page.locator('#mPlay').waitFor({ state: 'visible', timeout: STEP_TIMEOUT });
    });
    let t0 = 0;
    await step('click Deploy', async () => {
      await press(page, '#mPlay');
      await until(page, () => mode === 'game' && started === true, "mode === 'game' after Deploy");
      t0 = Date.now();
    });
    await step('close the tutorial', async () => {
      if (await page.evaluate(() => tutorialOpen)) { await page.keyboard.press('Enter'); await until(page, () => !tutorialOpen, 'the tutorial to close'); }
    });
    const snap = () => page.evaluate(() => ({ x: player.position.x, z: player.position.z, clicks: window.__smokeClicks }));
    let before = null;
    await step('hold W: the player moves', async () => {
      await page.evaluate(() => {   // probes only: count canvas clicks and rendered frames
        window.__smokeClicks = 0; renderer.domElement.addEventListener('mousedown', () => { window.__smokeClicks++; });
        window.__smokeFrames = 0; (function tick() { window.__smokeFrames++; requestAnimationFrame(tick); })();
      });
      before = await snap();
      await page.mouse.move(640, 300);
      await page.keyboard.down('KeyW');
      const f0 = await page.evaluate(() => window.__smokeFrames);
      await page.waitForTimeout(1000);   // at least 1 s and 3 frames (SwiftShader can run the scene at ~2 fps), and 0.1 m moved
      await until(page, ([f, x, z]) => window.__smokeFrames >= f + 3 && Math.hypot(player.position.x - x, player.position.z - z) >= 0.1,
        'the player to move 0.1 m with W held', [f0, before.x, before.z]);
      await page.keyboard.up('KeyW');
    });
    await step('fire two shots', async () => {
      for (let i = 0; i < 2; i++) { await page.mouse.down(); await page.waitForTimeout(80); await page.mouse.up(); await page.waitForTimeout(450); }
      await until(page, () => window.__smokeClicks === 2, 'both clicks to reach the game canvas');
    });
    await step('play to ~5 s', async () => {
      await page.waitForTimeout(Math.max(0, 5000 - (Date.now() - t0)));
      const after = await snap();
      info.clicks = after.clicks;   // left clicks that reached the game canvas (the shots; rounds fired depend on the weapon's mag)
      info.moved = Math.hypot(after.x - before.x, after.z - before.z);
    });
    await step('Esc pauses', async () => {
      await page.keyboard.press('Escape');
      await until(page, () => paused === true, 'paused === true after Esc');
    });
    await step('pause menu -> Menu', async () => {
      await press(page, '#pmMenu');
      await until(page, () => mode === 'menu' && !started, "mode === 'menu' after Menu");
    });
  },
  'editor.html': async (step, page) => {
    await step('menu is up', async () => {
      await until(page, () => typeof mode !== 'undefined' && mode === 'menu', "mode === 'menu'");
      await page.locator('#mEditor').waitFor({ state: 'visible', timeout: STEP_TIMEOUT });
    });
    await step('click Map editor (fills the prefab list)', async () => {
      await press(page, '#mEditor');
      await until(page, () => mode === 'editor', "mode === 'editor'");
      await until(page, () => document.getElementById('edPrefabList').children.length > 0, 'the prefab list to fill');
    });
    await step('open Maps (fills the map list)', async () => {
      await press(page, '#edMaps');
      await until(page, () => getComputedStyle(document.getElementById('edMapsModal')).display !== 'none' &&
        document.getElementById('edMapList').children.length > 0, 'the Maps window and its list');
    });
    await step('close Maps', async () => {
      await press(page, '#edMapsClose');
      await until(page, () => getComputedStyle(document.getElementById('edMapsModal')).display === 'none', 'the Maps window to close');
    });
    await step('editor -> Menu', async () => {
      await press(page, '#edMenu');
      await until(page, () => mode === 'menu', "mode === 'menu' after leaving the editor");
    });
  },
};

// ---------- run ----------
// Returns { pass, setup, summary, details, renderer, pages: [...] }. setup: true means Playwright or Chromium is missing.
export async function runSmoke({ root = ROOT } = {}) {
  const setupFail = why => ({ pass: false, setup: true, summary: why, details: ['set up the smoke test with: ' + SMOKE_SETUP], pages: [] });
  let pw, threeMin;
  try { pw = require('playwright'); } catch { return setupFail('Playwright is not installed (an optional dependency of tools/)'); }
  try { threeMin = require.resolve('three/build/three.min.js'); } catch { return setupFail('three r128 is not installed in tools/node_modules'); }
  let browser;
  try { browser = await pw.chromium.launch({ args: ['--use-angle=swiftshader', '--enable-unsafe-swiftshader', '--ignore-gpu-blocklist'] }); }
  catch (e) { return setupFail('Chromium for Playwright could not start: ' + firstLine(e.message)); }
  const pages = [];
  let renderer = '';
  try {
    for (const [name, session] of Object.entries(SESSIONS)) {
      const file = join(root, name);
      const r = { page: name, pass: false, steps: 0, of: 0, failedStep: null, errors: [], warnings: new Map(), blocked: [], info: {}, ms: {} };
      pages.push(r);
      if (!existsSync(file)) { r.errors.push(`${name} is missing`); continue; }
      const ctx = await browser.newContext({ viewport: { width: 1280, height: 720 } });   // fresh storage per page
      const page = await ctx.newPage();
      let cur = 'load';
      const err = msg => r.errors.push(`[${cur}] ${msg}`);
      page.on('pageerror', e => err((/eval at (predicate|evaluate)/.test(String(e.stack)) ? 'error in a smoke probe (the page did not load?): ' : 'page error: ') +
        firstLine(e.message) + frame(e.stack)));
      page.on('console', m => {
        const t = m.type(), loc = m.location(), at = loc && loc.url ? ` (${rel(loc.url)}:${loc.lineNumber + 1})` : '';
        if (t === 'error') err('console.error: ' + firstLine(m.text()) + at);
        else if (t === 'warning') { const k = firstLine(m.text()).slice(0, 160); r.warnings.set(k, (r.warnings.get(k) || 0) + 1); }
      });
      page.on('crash', () => err('the page crashed'));
      page.on('dialog', d => d.dismiss().catch(() => {}));
      const inRepo = u => { try { const f = resolve(fileURLToPath(u.split(/[?#]/)[0])), r = resolve(root) + sep;
        return process.platform === 'win32' ? f.toLowerCase().startsWith(r.toLowerCase()) : f.startsWith(r); } catch { return false; } };
      page.on('request', q => { if (/^file:/i.test(q.url()) && !inRepo(q.url())) err('read a file outside the repo: ' + q.url().slice(0, 120)); });
      await page.routeWebSocket(/.*/, ws => { r.blocked.push(ws.url()); err('blocked a WebSocket: ' + ws.url().slice(0, 120)); ws.close(); });
      await page.route('**/*', route => {
        const u = route.request().url();
        if (u === THREE_URL) return route.fulfill({ path: threeMin, contentType: 'application/javascript' });
        if (/^(file|data|blob):/i.test(u)) return route.continue();
        r.blocked.push(u); err('blocked a network request: ' + u.slice(0, 120));
        return route.abort();
      });
      let ok = true;
      const step = async (stepName, fn) => {
        r.of++;
        if (!ok) return;
        cur = stepName;
        const t = Date.now();
        try { await fn(); r.steps++; r.ms[stepName] = Date.now() - t; } catch (e) { ok = false; r.failedStep = stepName; err('step failed: ' + why(e)); }
      };
      try {
        const tl = Date.now();
        await page.goto(pathToFileURL(file).href, { waitUntil: 'load', timeout: 30000 });
        r.ms.load = Date.now() - tl;
        if (!renderer) renderer = await page.evaluate(() => {
          const gl = document.createElement('canvas').getContext('webgl'); if (!gl) return 'no WebGL';
          const x = gl.getExtension('WEBGL_debug_renderer_info'); return x ? gl.getParameter(x.UNMASKED_RENDERER_WEBGL) : gl.getParameter(gl.RENDERER);
        }).catch(() => 'unknown');
        await session(step, page, r.info);
        cur = 'settle';
        await page.waitForTimeout(1000);   // errors within ~1 s of the last step (storage promises, timers) still count
      } catch (e) { ok = false; err('session failed: ' + firstLine(e.message)); }
      r.pass = ok && !r.errors.length && r.steps === r.of;
      await ctx.close();
    }
  } finally { await browser.close(); }
  const pass = pages.length > 0 && pages.every(p => p.pass);
  const nWarn = pages.reduce((s, p) => s + [...p.warnings.values()].reduce((a, b) => a + b, 0), 0);
  const fmt = p => {
    const extra = 'clicks' in p.info ? ` (${p.info.clicks}/2 shots reached the canvas, moved ${p.info.moved.toFixed(1)} m)` : '';
    return `${p.page} ${p.steps}/${p.of} steps, ${p.errors.length} errors${extra}`;
  };
  const details = [];
  for (const p of pages) for (const e of p.errors) details.push(`${p.page} ${e}`);
  for (const p of pages) for (const [w, n] of p.warnings) details.push(`warning (does not fail): ${p.page}${n > 1 ? ` ×${n}` : ''}: ${w}`);
  return { pass, setup: false, summary: pages.map(fmt).join(' | ') + ` | ${nWarn} warnings | renderer ${renderer}`, details, renderer,
           pages: pages.map(p => ({ ...p, warnings: Object.fromEntries(p.warnings) })) };
}

// ---------- CLI ----------
const isMain = () => { try { return realpathSync(resolve(process.argv[1])) === realpathSync(fileURLToPath(import.meta.url)); } catch { return false; } };
if (process.argv[1] && isMain()) {   // realpath on both sides: a symlinked or junctioned checkout must still run the CLI
  const asJson = process.argv.includes('--json');
  const bad = process.argv.slice(2).filter(a => a !== '--json');
  if (bad.length) { console.error('usage: node tools/smoke.mjs [--json]'); process.exit(2); }
  const res = await runSmoke();
  if (asJson) console.log(JSON.stringify(res, null, 1));
  else {
    console.log(`[5a] smoke ${res.pass ? 'PASS' : 'FAIL'}  ${res.summary}`);
    for (const d of res.details) console.log('      ' + d);
  }
  process.exit(res.pass ? 0 : res.setup ? 2 : 1);
}
