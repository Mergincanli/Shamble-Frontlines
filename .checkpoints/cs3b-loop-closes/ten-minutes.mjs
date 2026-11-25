// The plan's one 10-minute run (section 9; PM-0002-5's reading): the built development page in Chromium for ten
// minutes, from a script of its own, never a spec that CI picks up. It serves the build with the build's own small
// server on a port of its own, taps a key every two seconds, and prints what the page's report says at the start
// and at the end. Run from the repository's root:
//   node .checkpoints/cs3b-loop-closes/ten-minutes.mjs <built folder> [minutes]
// It exits 0 when the run had 0 errors, no message refused, no press lost or doubled, no tick on older or wrong
// keys, and the ticks of the time it ran (30 a second, less at most one second's worth).
import { spawn } from 'node:child_process';
import path from 'node:path';
import { chromium } from '@playwright/test';

const dist = path.resolve(process.argv[2] ?? '');
const minutes = Number(process.argv[3] ?? 10);
const PORT = 4180;
const server = spawn('node', ['serve.mjs', String(PORT)], { cwd: dist, stdio: 'ignore' });
const stop = () => server.kill();
process.on('exit', stop);
await new Promise((resolve) => setTimeout(resolve, 1000));
const browser = await chromium.launch({ executablePath: '/opt/pw-browsers/chromium' });
const page = await browser.newPage({ viewport: { width: 1280, height: 720 } });
await page.goto(`http://127.0.0.1:${PORT}/`);
const report = () => page.evaluate(() => window.__shambleReport());
await page.waitForFunction(() => {
  const data = window.__shambleReport?.();
  return data !== undefined && data.worker !== null && data.blocks >= 90;
}, undefined, { timeout: 60_000 });
const began = Date.now();
const start = await report();
const codes = ['KeyW', 'KeyA', 'KeyS', 'KeyD', 'Space', 'ShiftLeft', 'ShiftRight'];
let taps = 0;
while (Date.now() - began < minutes * 60_000) {
  await page.keyboard.press(codes[taps % codes.length]);
  taps += 1;
  await page.waitForTimeout(2000);
}
await page.waitForTimeout(1500);
const end = await report();
const seconds = (Date.now() - began) / 1000;
const ticks = end.tick - start.tick;
const reading = {
  commit: end.header.commit,
  backend: end.picture.backend,
  seconds: Number(seconds.toFixed(1)),
  ticks,
  ticksExpectedAtLeast: Math.floor(30 * (seconds - 1)),
  frames: end.frames - start.frames,
  errors: end.errors,
  messages: end.messages,
  presses: end.presses,
  releases: end.releases,
  olderPosts: end.olderPosts,
  wrongKeys: end.wrongKeys,
  taps,
  keyToTickP50: Number(end.keyToTickP50.toFixed(1)),
  keyToTickP95: Number(end.keyToTickP95.toFixed(1)),
  worker: { ticks: end.worker?.ticks, drops: end.worker?.drops, pauses: end.worker?.pauses, stalePosts: end.worker?.stalePosts, poolMisses: end.worker?.poolMisses, maxTicksInOneWake: end.worker?.maxTicksInOneWake },
};
console.log(JSON.stringify(reading));
await browser.close();
stop();
const sound =
  end.errors === 0 && end.messages.refused === 0 && end.presses.lost === 0 && end.presses.doubled === 0 &&
  end.presses.made === end.presses.echoed && end.releases.doubled === 0 && end.olderPosts === 0 && end.wrongKeys === 0 &&
  ticks >= reading.ticksExpectedAtLeast - 30 * (end.worker?.drops ?? 0) && ticks <= Math.ceil(30 * seconds) + 1;
console.log(sound ? 'TEN MINUTES: sound' : 'TEN MINUTES: NOT sound');
process.exit(sound ? 0 : 1);
