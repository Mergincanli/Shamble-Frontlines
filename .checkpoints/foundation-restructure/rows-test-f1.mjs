// rows-test-f1.mjs (foundation-restructure, 2.1a, fix pass for the security review's F1): the build-folder rule as a table,
// now with the folder-move rows. The original rows-test.mjs stays as the record of 2026-11-10; this one takes its inputs:
//   node .checkpoints/foundation-restructure/rows-test-f1.mjs <scratch> <out> <old|f1> <legacyBase>
//   <scratch>    a plain folder holding tools/ (a gate to test) and the five files (e.g. .gate/fdn-scratch = the unfixed
//                gate, dd12aeb's blobs; .gate/fdn-scratch-f1 = the fixed gate)
//   <out>        a fresh folder for the row folders (refused if it exists)
//   <old|f1>     which expectations to judge by: the rows that differ show the hole (old) or the stop (f1)
//   <legacyBase> a commit with the five files in legacy/: 44446a5 with them moved, built by plumbing, never on a branch:
//     T=$(git ls-tree 44446a5 | grep -P '\t(shamble-core\.js|shamble-game\.js|shamble-editor\.js|index\.html|editor\.html)$' | git mktree)
//     R=$( (git ls-tree 44446a5 | grep -vP '\t(shamble-core\.js|shamble-game\.js|shamble-editor\.js|index\.html|editor\.html)$'; printf '040000 tree %s\tlegacy\n' "$T") | git mktree)
//     GIT_AUTHOR_NAME=rows-test-f1 GIT_AUTHOR_EMAIL=rows-test-f1@localhost GIT_AUTHOR_DATE=2026-11-11T00:00:00Z \
//     GIT_COMMITTER_NAME=rows-test-f1 GIT_COMMITTER_EMAIL=rows-test-f1@localhost GIT_COMMITTER_DATE=2026-11-11T00:00:00Z \
//     git commit-tree "$R" -p 44446a5 -m "rows-test-f1: 44446a5 with the five legacy files in legacy/ (never on a branch)"
// Each row is a plain folder <out>/<row> inside the repo (so git finds the repo for the base). Prints one line per row
// and exits 1 if any row misses its expectation.
import { cpSync, mkdirSync, symlinkSync, existsSync, copyFileSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { spawnSync } from 'node:child_process';

const REPO = '/home/user/Shamble-Frontlines';
const [S0, OUT0, MODE, LEGBASE] = process.argv.slice(2);
if (!S0 || !OUT0 || !['old', 'f1'].includes(MODE) || !LEGBASE) { console.error('usage: rows-test-f1.mjs <scratch> <out> <old|f1> <legacyBase>'); process.exit(2); }
const S = resolve(REPO, S0), OUT = resolve(REPO, OUT0);
if (existsSync(OUT)) { console.error(`${OUT} exists: give a fresh folder`); process.exit(2); }
const FIVE = ['shamble-core.js', 'shamble-game.js', 'shamble-editor.js', 'index.html', 'editor.html'];
const env = { ...process.env, PLAYWRIGHT_BROWSERS_PATH: join(REPO, '.gate/pw-browsers') };
const OLDBASE = '44446a5';
const moved = /^gate: the build folder moved from /;
// want: { exit, build?, prefix?, msg? } for both modes, or { old: {...}, f1: {...} } where they differ
const rows = [
  { id: 'a_root_only', root: FIVE, legacy: [], smoke: true, base: OLDBASE, want: { exit: 0, build: '.', prefix: '' } },
  { id: 'b_legacy_only', root: [], legacy: FIVE, smoke: true, base: OLDBASE, want: { exit: 0, build: 'legacy/', prefix: '' } },
  { id: 'c_both_complete', root: FIVE, legacy: FIVE, base: OLDBASE, want: { exit: 2, msg: /two places/ } },
  { id: 'd_neither', root: [], legacy: [], base: OLDBASE, want: { exit: 2, msg: /^gate: shamble-core\.js is missing from the working tree/ } },
  { id: 'e_partial_legacy', root: [], legacy: ['shamble-core.js', 'shamble-game.js', 'index.html', 'editor.html'], base: OLDBASE,
    want: { exit: 2, msg: /^gate: legacy\/shamble-editor\.js is missing from the working tree/ } },
  { id: 'f_root_plus_stray_legacy_js', root: FIVE, legacy: ['shamble-game.js'], base: OLDBASE, want: { exit: 2, msg: /two places/ } },
  { id: 'g_partial_root', root: ['shamble-core.js', 'shamble-game.js', 'index.html', 'editor.html'], legacy: [], base: OLDBASE,
    want: { exit: 2, msg: /^gate: shamble-editor\.js is missing from the working tree/ } },
  // the move with a page left behind at the root (an accidental copy): old passes, f1 stops
  { id: 'h_legacy_plus_stray_root_page', root: ['index.html'], legacy: FIVE, base: OLDBASE,
    want: { old: { exit: 0, build: 'legacy/', prefix: '' }, f1: { exit: 2, msg: moved } } },
  // the decoy (security review F1): clean copies in legacy/, root pages wired to a broken script
  { id: 'i_decoy', root: ['index.html', 'editor.html'], legacy: FIVE, extra: { 'broken-core.js': 'this is not javascript (\n' }, base: OLDBASE,
    want: { old: { exit: 0, build: 'legacy/', prefix: '' }, f1: { exit: 2, msg: /still holds .*index\.html/ } } },
  // a letter-case variant at the root: invisible to an exact-name check on Linux
  { id: 'j_letter_case', root: [], rename: { 'index.html': 'Index.html' }, legacy: FIVE, base: OLDBASE,
    want: { old: { exit: 0, build: 'legacy/', prefix: '' }, f1: { exit: 2, msg: /still holds Index\.html/ } } },
  // no move (a legacy/ base and head): a root page is allowed, as for a future v2 index.html
  { id: 'k_no_move_root_page', root: ['index.html'], legacy: FIVE, base: LEGBASE, want: { exit: 0, build: 'legacy/', prefix: 'legacy/' } },
  // the revert of the move with a page left in legacy/: old passes, f1 stops
  { id: 'l_revert_leftover_page', root: FIVE, legacy: ['index.html'], base: LEGBASE,
    want: { old: { exit: 0, build: '.', prefix: 'legacy/' }, f1: { exit: 2, msg: /moved from legacy\/ \(base\) to the repo root \(head\), but legacy\/ still holds index\.html/ } } },
  // the clean revert
  { id: 'm_revert_clean', root: FIVE, legacy: [], base: LEGBASE, want: { exit: 0, build: '.', prefix: 'legacy/' } },
];
let bad = 0;
mkdirSync(OUT, { recursive: true });
for (const r of rows) {
  const want = r.want[MODE] || r.want;
  const dir = join(OUT, r.id);
  mkdirSync(join(dir, 'legacy'), { recursive: true });
  cpSync(join(S, 'tools'), join(dir, 'tools'), { recursive: true, filter: src => !src.endsWith('/node_modules') });
  symlinkSync(join(REPO, 'tools/node_modules'), join(dir, 'tools/node_modules'));
  for (const f of r.root) copyFileSync(join(S, f), join(dir, f));
  for (const [f, to] of Object.entries(r.rename || {})) copyFileSync(join(S, f), join(dir, to));
  for (const [f, text] of Object.entries(r.extra || {})) writeFileSync(join(dir, f), text);
  for (const f of r.legacy) copyFileSync(join(S, f), join(dir, 'legacy', f));
  const args = ['tools/gate.mjs', r.base, '--json', ...(r.smoke ? [] : ['--no-smoke'])];
  const p = spawnSync(process.execPath, args, { cwd: dir, env, encoding: 'utf8', maxBuffer: 64 << 20 });
  let build = null, prefix = null, pass = null;
  try { const j = JSON.parse(p.stdout); build = j.build; prefix = j.base && j.base.prefix; pass = j.pass; } catch { /* died before the result */ }
  const err = (p.stderr || '').split('\n')[0];
  let ok = p.status === want.exit;
  if (want.build !== undefined) ok = ok && build === want.build;
  if (want.prefix !== undefined) ok = ok && prefix === want.prefix;
  if (want.msg) ok = ok && want.msg.test(err);
  if (!ok) bad++;
  console.log(`${ok ? 'PASS' : 'FAIL'} ${r.id}: exit ${p.status}; build ${build}; base prefix ${JSON.stringify(prefix)}; gate pass ${pass}; stderr: ${err || '(none)'}`);
}
for (const [id, want] of [['b_legacy_only', 0], ['c_both_complete', 2]]) {
  const p = spawnSync(process.execPath, ['tools/smoke.mjs', '--json'], { cwd: join(OUT, id), env, encoding: 'utf8', maxBuffer: 64 << 20 });
  let summary = ''; try { summary = JSON.parse(p.stdout).summary; } catch { summary = (p.stderr || '').split('\n')[0]; }
  let ok = p.status === want; if (id === 'c_both_complete') ok = ok && /two places/.test(summary);
  if (!ok) bad++;
  console.log(`${ok ? 'PASS' : 'FAIL'} smoke CLI ${id}: exit ${p.status}; ${summary.slice(0, 160)}`);
}
console.log(bad ? `ROWS (${MODE}): ${bad} FAILED` : `ROWS (${MODE}): ALL PASS`);
process.exit(bad ? 1 : 0);
