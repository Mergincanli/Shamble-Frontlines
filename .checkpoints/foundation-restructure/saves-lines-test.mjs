// saves-lines-test.mjs (foundation-restructure, step 2.1b): proves the two console lines in saves-console-lines.md do
// what the report says, in headless Chromium (the tools' Playwright, pinned 1194) on a file:// page. DevTools' copy()
// does not exist in a page, so a stub records what it was given. The lines are read from the notes file, never retyped.
//   backup: every key, as one JSON text, values byte for byte (quotes, newlines, non-ASCII, a large value)
//   restore: puts back only missing keys and never overwrites an existing one
// Run from the repo root: node .gate/foundation-restructure/saves-lines-test.mjs
import { createRequire } from 'node:module';
import { mkdirSync, writeFileSync, readFileSync, existsSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';

const REPO = '/home/user/Shamble-Frontlines';
const W = join(REPO, '.gate/foundation-restructure/saves-lines');
const PROFILE = join(W, 'profile');
if (existsSync(PROFILE) && readdirSync(PROFILE).length) { console.error('use a fresh .gate/foundation-restructure/saves-lines'); process.exit(2); }
mkdirSync(PROFILE, { recursive: true });
const md = readFileSync(join(REPO, '.checkpoints/foundation-restructure/saves-console-lines.md'), 'utf8');
const blocks = [...md.matchAll(/```js\n(.*)\n```/g)].map(m => m[1]);
if (blocks.length !== 2) { console.error(`expected 2 js lines, found ${blocks.length}`); process.exit(2); }
const [BACKUP, RESTORE] = blocks;
const require = createRequire(join(REPO, 'tools/package.json'));
const { chromium } = require('playwright');
writeFileSync(join(W, 'page.html'), '<!doctype html><meta charset="utf-8"><title>saves</title>\n');
const ctx = await chromium.launchPersistentContext(PROFILE, { headless: true });
const page = await ctx.newPage();
await page.goto(pathToFileURL(join(W, 'page.html')).href);
const SAVES = {
  shamble_prog: JSON.stringify({ xp: 1234, rank: 3, name: 'Müstafa "the" tester' }),
  'vmap:Cabana copy': JSON.stringify({ boxes: [[1, 2, 3]], note: 'line1\nline2\ttab   sep' }),
  'vmap:big': 'x'.repeat(300000),
  shamble_settings: '{"vol":0.8}',
};
await page.evaluate(s => { localStorage.clear(); for (const [k, v] of Object.entries(s)) localStorage.setItem(k, v); window.copy = t => { window.__copied = t; }; }, SAVES);
const res = { chromium: chromium.executablePath(), checks: [] };
const check = (name, pass, detail = '') => { res.checks.push({ name, pass, detail }); console.log(`${pass ? 'PASS' : 'FAIL'} ${name}${detail ? ' :: ' + detail : ''}`); };

// backup
const said = await page.evaluate(BACKUP);
const copied = await page.evaluate(() => window.__copied);
check('backup answers with the key count', said === '4 keys copied: paste them into a text file now', said);
const parsed = JSON.parse(copied);
check('backup text starts with {', copied.startsWith('{'));
check('backup holds every key, values byte for byte', JSON.stringify(Object.keys(parsed).sort()) === JSON.stringify(Object.keys(SAVES).sort()) && Object.entries(SAVES).every(([k, v]) => parsed[k] === v));

// restore: one key missing, one key changed since the backup; the line must restore the missing one only
await page.evaluate(() => { localStorage.removeItem('vmap:Cabana copy'); localStorage.setItem('shamble_prog', 'NEWER'); });
const line = RESTORE.replace('PASTE_HERE', () => copied);
const said2 = await page.evaluate(line);
check('restore answers with its counts', said2 === '1 keys restored, 3 already there (left as they are)', said2);
const after = await page.evaluate(() => Object.fromEntries(Object.keys(localStorage).map(k => [k, localStorage.getItem(k)])));
check('the missing key is back, byte for byte', after['vmap:Cabana copy'] === SAVES['vmap:Cabana copy']);
check('an existing key is never overwritten', after.shamble_prog === 'NEWER');
check('nothing else changed', after['vmap:big'] === SAVES['vmap:big'] && after.shamble_settings === SAVES.shamble_settings && Object.keys(after).length === 4);
await ctx.close();
res.verdict = res.checks.every(c => c.pass) ? 'PASS' : 'FAIL';
console.log('console lines: ' + res.verdict);
writeFileSync(join(REPO, '.checkpoints/foundation-restructure/saves-lines-result.json'), JSON.stringify(res, null, 1) + '\n');
