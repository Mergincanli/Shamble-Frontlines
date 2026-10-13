// pageshot.js — render the REAL pages (DOM + CSS + HUD + WebGL) in Chromium. Toolkit file; never shipped.
// Fills the gap preview.js leaves: preview.js draws only the 3D scene; this draws everything the player sees.
//
// SETUP: cd tools && npm ci && npx playwright install chromium   (three r128 is served from tools/node_modules, so no CDN needed)
// USAGE (from the repo root): node tools/pageshot.js <buildDir> <index.html|editor.html> <outPrefix> [--headed] [steps...]
//   wait:<ms>  shot:<name>  eval:<js expr | Promise>  click:<css>  hover:<css>  key:<Key>  mouse:<x>,<y>
//   size:<w>x<h> (default 1600x900)   motion:reduce
//   e.g. node tools/pageshot.js . index.html .gate/s wait:1500 shot:menu "eval:deploy()" wait:900 "eval:(closeTutorial(),setPause(true))" shot:pause
// --headed: a visible browser window on the real GPU (Linux needs a display: xvfb-run -a). Default: headless SwiftShader (CPU).
// Every run prints the WebGL renderer string, and every shot line repeats it, so each screenshot says which GPU drew it.
// Prints every page error / console error. CAVEATS: no Arial Black / Courier New / Segoe Print in the Linux container (substitutes);
// SwiftShader composites on the CPU, so animated DOM over the canvas runs at a few fps there — judge frame time in Brave
// (DEV panel, backtick) or --headed on the GPU. Screenshots take ~3 s each while the 3D scene renders; eval "renderer.render=()=>{}"
// for fast DOM-only checks. deploy() opens the first-time tutorial asynchronously: wait before closeTutorial().
const path = require('path'), { pathToFileURL } = require('url');
const pw = require('playwright');
const THREE_MIN = process.env.THREE_MIN || require.resolve('three/build/three.min.js', { paths: [__dirname] });
const HEADED = process.argv.includes('--headed');
const [dir, pageName, out, ...steps] = process.argv.slice(2).filter(a => a !== '--headed');
if (!dir || !pageName || !out) { console.error('usage: node tools/pageshot.js <buildDir> <page.html> <outPrefix> [--headed] [steps...]'); process.exit(2); }
(async () => {
  const browser = await pw.chromium.launch(HEADED
    ? { headless: false, args: ['--ignore-gpu-blocklist'] }
    : { args: ['--use-angle=swiftshader', '--enable-unsafe-swiftshader', '--ignore-gpu-blocklist'] });
  const opts = { viewport: { width: 1600, height: 900 }, deviceScaleFactor: 1 };
  const sz = steps.find(s => s.startsWith('size:'));
  if (sz) { const [w, h] = sz.slice(5).split('x').map(Number); opts.viewport = { width: w, height: h }; }
  if (steps.includes('motion:reduce')) opts.reducedMotion = 'reduce';
  const page = await (await browser.newContext(opts)).newPage();
  const errs = [];
  page.on('pageerror', e => errs.push('PAGEERROR ' + e.message));
  page.on('console', m => { if (m.type() === 'error' || m.type() === 'warning') errs.push(m.type().toUpperCase() + ' ' + m.text()); });
  await page.route('**/three.min.js', r => r.fulfill({ path: THREE_MIN, contentType: 'application/javascript' }));
  await page.goto(pathToFileURL(path.resolve(dir, pageName)).href);
  await page.waitForTimeout(600);
  const gpu = await page.evaluate(() => {   // the game's own context when it exists, else a probe canvas in the same GPU process
    const g = (typeof renderer !== 'undefined' && renderer.getContext && renderer.getContext()) || document.createElement('canvas').getContext('webgl');
    if (!g) return 'no WebGL';
    const e = g.getExtension('WEBGL_debug_renderer_info');
    return e ? g.getParameter(e.UNMASKED_RENDERER_WEBGL) : g.getParameter(g.RENDERER);
  });
  console.log('renderer', gpu, HEADED ? '(headed)' : '(headless)');
  for (const s of steps) {
    const i = s.indexOf(':'), k = s.slice(0, i), v = s.slice(i + 1);
    if (k === 'wait') await page.waitForTimeout(+v);
    else if (k === 'shot') { await page.screenshot({ path: out + '_' + v + '.png' }); console.log('shot', out + '_' + v + '.png', '[' + gpu + ']'); }
    else if (k === 'eval') { const r = await page.evaluate(v); if (r !== undefined) console.log('eval>', JSON.stringify(r)); }
    else if (k === 'click') await page.click(v);
    else if (k === 'hover') await page.hover(v);
    else if (k === 'key') await page.keyboard.press(v);
    else if (k === 'mouse') { const [x, y] = v.split(',').map(Number); await page.mouse.move(x, y); }
  }
  console.log(errs.length ? errs.join('\n') : 'no page errors');
  await browser.close();
})().catch(e => { console.error('PAGESHOT', e); process.exit(1); });
