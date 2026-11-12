// e3-bases-2-1b.mjs (foundation-restructure, step 2.1b): E3's second part after the move, "the edited gate against three
// bases" (plan 1.5), on 2.1b's real head (the main folder's working tree: files in legacy/, main's gate unchanged).
// Each run: `node tools/gate.mjs <base> --no-smoke --json`; recorded: exit, pass, the base prefix the gate chose, the build
// folder, and check 3's summary. Expected (2.1a's e3-bases.md, there on a local test commit, here on the pushed head):
//   44446a5 (OLD, root) PASS "" +0 -0 · 8e58937 (CS-1, shamble-frontlines-repo/) PASS +34 -0 · 76b4f7a (A, root) PASS ""
//   · c090843 (the move, legacy/) PASS "legacy/" · b6b3b81 (own head, legacy/) PASS "legacy/" +0 -0.
// Run from the repo root: node .gate/foundation-restructure/e3-bases-2-1b.mjs
import { writeFileSync } from 'node:fs';
import { spawnSync, execFileSync } from 'node:child_process';

const REPO = '/home/user/Shamble-Frontlines';
const BASES = [
  { sha: '44446a5', what: 'OLD (root layout)', prefix: '' },
  { sha: '8e58937', what: "CS-1's shamble-frontlines-repo/ layout", prefix: 'shamble-frontlines-repo/' },
  { sha: '76b4f7a', what: 'A, main with 2.1a (root layout)', prefix: '' },
  { sha: 'c090843', what: "2.1b's move commit (legacy/)", prefix: 'legacy/' },
  { sha: 'b6b3b81', what: "2.1b's head when run (legacy/)", prefix: 'legacy/' },
];
const head = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: REPO, encoding: 'utf8' }).trim();
const dirty = execFileSync('git', ['status', '--porcelain', '--', 'legacy', 'tools'], { cwd: REPO, encoding: 'utf8' }).trim();
const res = { head, legacyAndToolsClean: dirty === '', rows: [] };
let ok = dirty === '';
console.log(`head ${head.slice(0, 7)}; legacy/ and tools/ clean: ${dirty === ''}`);
for (const b of BASES) {
  const p = spawnSync(process.execPath, ['tools/gate.mjs', b.sha, '--no-smoke', '--json'], { cwd: REPO, encoding: 'utf8', maxBuffer: 64 << 20 });
  let j = null; try { j = JSON.parse(p.stdout); } catch { /* died */ }
  const c3 = j && j.checks.find(x => String(x.id) === '3');
  const row = { base: b.sha, what: b.what, exit: p.status, pass: j ? j.pass : false, build: j && j.build, basePrefix: j && j.base ? j.base.prefix : null,
    check3: c3 ? c3.summary : null, err: (p.stderr || '').split('\n')[0] };
  row.ok = p.status === 0 && row.pass === true && row.basePrefix === b.prefix && row.build === 'legacy/';
  if (!row.ok) ok = false;
  res.rows.push(row);
  console.log(`${row.ok ? 'PASS' : 'FAIL'} base ${b.sha} (${b.what}): exit ${p.status}, prefix "${row.basePrefix}", build ${row.build} :: ${row.check3 || row.err}`);
}
res.verdict = ok ? 'PASS' : 'FAIL';
console.log('E3 bases: ' + res.verdict);
writeFileSync(`${REPO}/.checkpoints/foundation-restructure/e3-bases-2-1b-result.json`, JSON.stringify(res, null, 1) + '\n');
process.exit(ok ? 0 : 1);
