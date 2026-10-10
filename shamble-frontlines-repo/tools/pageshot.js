// pageshot.js — render the REAL pages (DOM + CSS + HUD + WebGL) in headless Chromium. Toolkit file; never shipped.
// Fills the gap preview.js leaves: preview.js draws only the 3D scene; this draws everything the player sees.
//
// SETUP (fresh container): Playwright + Chromium are preinstalled (PLAYWRIGHT_BROWSERS_PATH=/opt/pw-browsers).
//   mkdir -p /home/claude/tools && cd /home/claude/tools && npm i three@0.128.0   (cdnjs is blocked; three is routed locally)
// USAGE: node pageshot.js <buildDir> <index.html|editor.html> <outPrefix> [steps...]
//   wait:<ms>  shot:<name>  eval:<js expr | Promise>  click:<css>  hover:<css>  key:<Key>  mouse:<x>,<y>
//   size:<w>x<h> (default 1600x900)   motion:reduce
//   e.g. node pageshot.js ./build index.html /tmp/s wait:1500 shot:menu "eval:deploy()" wait:900 "eval:(closeTutorial(),setPause(true))" shot:pause
// Prints every page error / console error. CAVEATS: no Arial Black / Courier New / Segoe Print in the container (substitutes);
// SwiftShader composites on the CPU, so animated DOM over the canvas runs at a few fps here — judge frame time in Brave
// (DEV panel, backtick), not here. Screenshots take ~3 s each while the 3D scene renders; eval "renderer.render=()=>{}"
// for fast DOM-only checks. deploy() opens the first-time tutorial asynchronously: wait before closeTutorial().
const path = require('path');
const pw = (() => { try { return require('playwright'); } catch (e) { return require('/home/claude/.npm-global/lib/node_modules/playwright'); } })();
const THREE_MIN = process.env.THREE_MIN || require.resolve('three/build/three.min.js', { paths: [__dirname, process.cwd(), '/home/claude/tools'] });
const [, , dir, pageName, out, ...steps] = process.argv;
if (!dir || !pageName || !out) { console.error('usage: node pageshot.js <buildDir> <page.html> <outPrefix> [steps...]'); process.exit(2); }
(async () => {
  const browser = await pw.chromium.launch({ args: ['--use-angle=swiftshader', '--enable-unsafe-swiftshader', '--ignore-gpu-blocklist'] });
  const opts = { viewport: { width: 1600, height: 900 }, deviceScaleFactor: 1 };
  const sz = steps.find(s => s.startsWith('size:'));
  if (sz) { const [w, h] = sz.slice(5).split('x').map(Number); opts.viewport = { width: w, height: h }; }
  if (steps.includes('motion:reduce')) opts.reducedMotion = 'reduce';
  const page = await (await browser.newContext(opts)).newPage();
  const errs = [];
  page.on('pageerror', e => errs.push('PAGEERROR ' + e.message));
  page.on('console', m => { if (m.type() === 'error' || m.type() === 'warning') errs.push(m.type().toUpperCase() + ' ' + m.text()); });
  await page.route('**/three.min.js', r => r.fulfill({ path: THREE_MIN, contentType: 'application/javascript' }));
  await page.goto('file://' + path.resolve(dir, pageName));
  await page.waitForTimeout(600);
  for (const s of steps) {
    const i = s.indexOf(':'), k = s.slice(0, i), v = s.slice(i + 1);
    if (k === 'wait') await page.waitForTimeout(+v);
    else if (k === 'shot') { await page.screenshot({ path: out + '_' + v + '.png' }); console.log('shot', out + '_' + v + '.png'); }
    else if (k === 'eval') { const r = await page.evaluate(v); if (r !== undefined) console.log('eval>', JSON.stringify(r)); }
    else if (k === 'click') await page.click(v);
    else if (k === 'hover') await page.hover(v);
    else if (k === 'key') await page.keyboard.press(v);
    else if (k === 'mouse') { const [x, y] = v.split(',').map(Number); await page.mouse.move(x, y); }
  }
  console.log(errs.length ? errs.join('\n') : 'no page errors');
  await browser.close();
})().catch(e => { console.error('PAGESHOT', e); process.exit(1); });
