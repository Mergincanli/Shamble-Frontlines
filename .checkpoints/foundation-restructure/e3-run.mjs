// e3-run.mjs (foundation-restructure, 2.1a): evidence E3, the edited gate bites exactly like OLD's.
// Plants each entry of e3-mutations.json in three throwaway worktrees built from commits, runs the full gate (smoke
// included) against the base 44446a5, records the failing check ids and the exit code, and restores the file.
//   old    = .gate/fdn-old     at 44446a5 (OLD's own gate; files at the root)
//   new    = .gate/fdn-new     at the 2.1a commit (the edited gate; files at the root)
//   newleg = .gate/fdn-newleg  at the 2.1a commit plus a local commit of the move (the edited gate; files in legacy/)
// Run from the repo root (resumable: entries already in e3-results.json are skipped):
//   node .checkpoints/foundation-restructure/e3-run.mjs
import { readFileSync, writeFileSync, existsSync } from 'node:fs';
import { join } from 'node:path';
import { spawnSync, execFileSync } from 'node:child_process';

const REPO = '/home/user/Shamble-Frontlines';
const NOTES = join(REPO, '.checkpoints/foundation-restructure');
const BASE = '44446a5';
const COLS = [
  { id: 'old', dir: join(REPO, '.gate/fdn-old'), build: '' },
  { id: 'new', dir: join(REPO, '.gate/fdn-new'), build: '' },
  { id: 'newleg', dir: join(REPO, '.gate/fdn-newleg'), build: 'legacy/' },
];
const env = { ...process.env, PLAYWRIGHT_BROWSERS_PATH: join(REPO, '.gate/pw-browsers') };
const table = JSON.parse(readFileSync(join(NOTES, 'e3-mutations.json'), 'utf8'));
const OUT = join(NOTES, 'e3-results.json');
const results = existsSync(OUT) ? JSON.parse(readFileSync(OUT, 'utf8')) : {};
const git = (cwd, ...a) => execFileSync('git', a, { cwd, encoding: 'utf8', maxBuffer: 64 << 20 });
const FIVE = ['shamble-core.js', 'shamble-game.js', 'shamble-editor.js', 'index.html', 'editor.html'];

// the clean state of each column: every file equals the worktree's committed blob (old/new: at the root; newleg: legacy/)
const clean = {};
for (const c of COLS) {
  clean[c.id] = {};
  for (const f of FIVE) {
    const disk = readFileSync(join(c.dir, c.build + f), 'utf8');
    const blob = git(c.dir, 'show', `HEAD:${c.build}${f}`);
    if (disk !== blob) throw new Error(`${c.id}: ${c.build}${f} is not clean; fix the worktree first`);
    clean[c.id][f] = disk;
  }
}

function plant(c, m) {
  for (const e of m.edits) {
    const p = join(c.dir, c.build + e.file);
    if (e.from_commit) { writeFileSync(p, git(c.dir, 'show', `${e.from_commit}:${e.file}`)); continue; }
    const s = readFileSync(p, 'utf8');
    const n = s.split(e.find).length - 1;
    if (n !== 1) throw new Error(`${m.id} on ${c.id}: find occurs ${n} times in ${e.file}`);
    writeFileSync(p, s.replace(e.find, () => e.replace));
  }
}
function restore(c) {
  for (const f of FIVE) writeFileSync(join(c.dir, c.build + f), clean[c.id][f]);
  for (const f of FIVE) if (readFileSync(join(c.dir, c.build + f), 'utf8') !== clean[c.id][f]) throw new Error(`${c.id}: restore failed for ${f}`);
}
function runGate(c, flags) {
  const p = spawnSync(process.execPath, ['tools/gate.mjs', BASE, '--json', ...flags], { cwd: c.dir, env, encoding: 'utf8', maxBuffer: 64 << 20 });
  let j = null; try { j = JSON.parse(p.stdout); } catch { /* died before the result */ }
  const failing = j ? j.checks.filter(x => !x.pass && !x.skipped).map(x => String(x.id)).sort() : ['(died: ' + (p.stderr || '').split('\n')[0] + ')'];
  const lines = j ? Object.fromEntries(j.checks.map(x => [String(x.id), (x.pass ? 'PASS ' : 'FAIL ') + x.summary + (x.details && x.details.length ? ' || ' + x.details.slice(0, 3).join(' | ') : '')])) : {};
  return { exit: p.status, failing, lines, build: j && j.build, basePrefix: j && j.base && j.base.prefix };
}

for (const m of table) {
  if (results[m.id]) continue;
  const r = { id: m.id, what: m.what, expected: m.expected, flags: m.flags || [] };
  for (const c of COLS) {
    try { plant(c, m); r[c.id] = runGate(c, m.flags || []); }
    catch (e) { r[c.id] = { exit: null, failing: ['(plant error: ' + e.message + ')'], lines: {} }; }
    finally { restore(c); }
  }
  const same = JSON.stringify(r.old.failing) === JSON.stringify(r.new.failing) && JSON.stringify(r.new.failing) === JSON.stringify(r.newleg.failing);
  const isControl = /^(C\d|PC\d)$/.test(m.id);
  if (m.id === 'PC1') r.verdict = same && ['old', 'new', 'newleg'].every(k => (r[k].lines['3'] || '').startsWith('PASS')) ? 'PASS (judged on check 3, as the Phase 0 gate had checks 1-4 only)' : 'FAIL';
  else if (isControl) r.verdict = ['old', 'new', 'newleg'].every(k => r[k].exit === 0) ? 'PASS' : 'FAIL';
  else r.verdict = same && r.old.failing.length > 0 && !r.old.failing[0].startsWith('(') ? 'PASS' : 'FAIL';
  r.reportChecksInOld = m.expected.every(x => r.old.failing.includes(x));
  results[m.id] = r;
  writeFileSync(OUT, JSON.stringify(results, null, 1));
  console.log(`${r.verdict.padEnd(5)} ${m.id.padEnd(4)} old ${JSON.stringify(r.old.failing)}/${r.old.exit} new ${JSON.stringify(r.new.failing)}/${r.new.exit} newleg ${JSON.stringify(r.newleg.failing)}/${r.newleg.exit} report ${JSON.stringify(m.expected)} in-old ${r.reportChecksInOld}`);
}
const all = Object.values(results);
console.log(`E3: ${all.filter(r => r.verdict.startsWith('PASS')).length} of ${all.length} PASS (table has ${table.length})`);
