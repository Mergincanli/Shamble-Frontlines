// docs-check.mjs (foundation-restructure, step 2.1b, step 10): the scripted checks on the docs pass (the advisor tool's
// lenses 1 and 3), on the main folder's working tree against main 76b4f7a. Read-only; prints PASS/FAIL per check.
//   1. files: under docs/ only the files of plan-check §7 changed (DECISIONS.md, the 2.1b report, REPO_MAP, START_HERE,
//      MODULE_MAP, TECHNICAL, STATUS, STATUS_ARCHIVE, ROADMAP, ADR 0002, the register); nothing in the never-list
//      changed (spikes/, the tuning oracle files, the inventories, research, other reports, ADRs 0001 and 0003-0006,
//      SPIKE_PLAN); no PM row of any ADR changed.
//   2. ADR 0002 hunk allow-list: only :3 (Status), :7 (Signed off), the OQ-0002-2 entry, and the two rows
//      "| R5-LAYOUT-OQ5 |" and "| R5-GATE-OQ3 |" (each now "answered in part: OQ-0002-2").
//   3. the register hunk allow-list: only the ADR 0002 row (its count text still "8 open"), the status sentence (the
//      acceptance words added after "not yet accepted"), and one added row naming PM-0002-36 in the loose-ends
//      register; the Total paragraph unchanged.
//   4. the acceptance words are exactly "not yet accepted except ADR 0002's part 1, accepted by Mustafa on 2026-11-10
//      (SCOPE (a))" in ADR 0002 :7 and the register's status sentence.
//   5. quotes: every span of 20+ characters inside double quotes on an added docs line (DECISIONS.md and the report's
//      appendix excepted: they are byte-checked by their own scripts) is a substring of his messages A1-A6 or of a
//      docs file at main, whitespace folded.
//   6. START_HERE: every `tools/... legacy ...` command on an added line is one E6 ran (its command strings).
//   7. his render-budget note appears only in DECISIONS.md and the report (his words: "change no other file for it").
//   8. no added docs line points at a .checkpoints/ path (the folder is deleted before ready).
//   9. node .claude/scripts/repo-map-check.mjs exits 0.
// Run from the repo root: node .gate/foundation-restructure/docs-check.mjs
import { readFileSync, existsSync } from 'node:fs';
import { join } from 'node:path';
import { execFileSync, spawnSync } from 'node:child_process';

const REPO = '/home/user/Shamble-Frontlines';
const MAIN = '76b4f7a';
const git = (...a) => execFileSync('git', a, { cwd: REPO, encoding: 'utf8', maxBuffer: 256 << 20 });
const NOTES = join(REPO, '.checkpoints/foundation-restructure');
let ok = true;
const check = (name, pass, detail = '') => { if (!pass) ok = false; console.log(`${pass ? 'PASS' : 'FAIL'} ${name}${detail ? ' :: ' + detail : ''}`); };
const fold = s => s.replace(/\s+/g, ' ');

// ---------- 1. files ----------
const changed = git('diff', '--name-only', MAIN, '--', 'docs', 'spikes', 'README.md').trim().split('\n').filter(Boolean);
const untracked = git('ls-files', '--others', '--exclude-standard', '--', 'docs', 'spikes').trim().split('\n').filter(Boolean);
const ALLOWED = new Set(['docs/DECISIONS.md', 'docs/reports/2026-11-11-foundation-2-1b.md', 'docs/REPO_MAP.md', 'docs/START_HERE.md',
  'docs/MODULE_MAP.md', 'docs/TECHNICAL.md', 'docs/STATUS.md', 'docs/STATUS_ARCHIVE.md', 'docs/ROADMAP.md',
  'docs/adr/0002-code-structure-build-toolchain.md', 'docs/adr/README.md', 'README.md']);
const extra = [...changed, ...untracked].filter(f => !ALLOWED.has(f));
check('only plan-check §7\'s docs changed (and the root README from step 7)', extra.length === 0, extra.join(', ') || `${changed.length} changed`);
const NEVER = /^(spikes\/|docs\/legacy-tuning\.json$|docs\/LEGACY_TUNING\.md$|docs\/LEGACY_INVENTORY\.md$|docs\/EDITOR_V1_INVENTORY\.md$|docs\/research\/|docs\/SPIKE_PLAN\.md$|docs\/adr\/000[13456]-)/;
check('nothing in the never-list changed', ![...changed, ...untracked].some(f => NEVER.test(f)));
const adrDiff = git('diff', '-U0', MAIN, '--', 'docs/adr');
check('no PM row of any ADR changed', !adrDiff.split('\n').some(l => /^[-+]\| PM-/.test(l)));

// hunks of one file: [{oldStart, oldLines: [...], newStart, newLines: [...]}]
function hunks(file) {
  const out = []; let h = null;
  for (const l of git('diff', '-U0', MAIN, '--', file).split('\n')) {
    const m = l.match(/^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@/);
    if (m) { h = { oldStart: +m[1], oldCount: m[2] === undefined ? 1 : +m[2], newStart: +m[3], old: [], add: [] }; out.push(h); continue; }
    if (!h || l.startsWith('---') || l.startsWith('+++')) continue;
    if (l.startsWith('-')) h.old.push(l.slice(1)); else if (l.startsWith('+')) h.add.push(l.slice(1));
  }
  return out;
}
const mainLines = f => git('show', `${MAIN}:${f}`).split('\n');

// ---------- 2. ADR 0002 ----------
{
  const F = 'docs/adr/0002-code-structure-build-toolchain.md';
  const L = mainLines(F);
  const oq2 = L.findIndex(l => l.startsWith('- **OQ-0002-2.')) + 1, oq3 = L.findIndex(l => l.startsWith('- **OQ-0002-3.')) + 1;
  const rows = L.map((l, i) => [l, i + 1]).filter(([l]) => /^\| R5-(LAYOUT-OQ5|GATE-OQ3) \| open: OQ-0002-2 \|/.test(l)).map(([, n]) => n);
  check('ADR 0002 at main: the OQ-0002-2 entry and the two rows found by their text', oq2 > 0 && oq3 > oq2 && rows.length === 2, `entry :${oq2}-:${oq3 - 1}, rows :${rows.join(', :')}`);
  const allowedOld = new Set([3, 7, ...rows]); for (let n = oq2; n < oq3; n++) allowedOld.add(n);
  const bad = [];
  for (const h of hunks(F)) {
    const olds = Array.from({ length: h.oldCount }, (_, k) => h.oldStart + k);
    const at = h.oldCount ? olds : [h.oldStart, h.oldStart + 1];
    if (!(h.oldCount ? olds.every(n => allowedOld.has(n)) : at.some(n => n >= oq2 && n <= oq3))) bad.push(`hunk at main :${h.oldStart} (${h.oldCount} old lines)`);
  }
  check('ADR 0002: every change is on :3, :7, the OQ-0002-2 entry or the two rows', bad.length === 0, bad.join('; ') || `${hunks(F).length} hunks`);
  const head = readFileSync(join(REPO, F), 'utf8').split('\n');
  check('ADR 0002: both rows now read "answered in part: OQ-0002-2"', head.filter(l => /^\| R5-(LAYOUT-OQ5|GATE-OQ3) \| answered in part: OQ-0002-2 \|/.test(l)).length === 2 && !head.some(l => l.includes('open: OQ-0002-2')));
  check('ADR 0002 :7 carries the acceptance words', head[6] === "**Signed off by Mustafa:** as the basis for the spikes (2026-11-03, SPIKE-ORDER (b)); not yet accepted except ADR 0002's part 1, accepted by Mustafa on 2026-11-10 (SCOPE (a))", head[6]);
  const entry = head.slice(head.findIndex(l => l.startsWith('- **OQ-0002-2.')), head.findIndex(l => l.startsWith('- **OQ-0002-3.'))).join('\n');
  check('ADR 0002: the OQ-0002-2 entry says "answered in part", names tools/, docs/ and step 2.3, and points to the 2.1b report', /answered in part/i.test(entry) && entry.includes('`tools/`') && entry.includes('`docs/`') && /2\.3/.test(entry) && entry.includes('2026-11-11-foundation-2-1b.md'));
}

// ---------- 3. the register ----------
{
  const F = 'docs/adr/README.md';
  const L = mainLines(F);
  const row = L.findIndex(l => l.startsWith('| [0002](0002-')) + 1;
  const tot = L.findIndex(l => l.startsWith('Total: 119 numbered open questions')) + 1;
  const sent = L.findIndex(l => l.includes('Status of every draft: Proposed.')) + 1;
  const sentEnd = L.findIndex((l, i) => i >= sent - 1 && l.includes('not yet accepted. His words')) + 1;
  const le = L.findIndex(l => l === '## Loose-ends register') + 1, leEnd = L.findIndex(l => l.startsWith('## Needs outside VISION_V2')) + 1;
  check('the register at main: the ADR 0002 row, the Total line, the status sentence and the loose-ends section found', row > 0 && tot > 0 && sent > tot && sentEnd >= sent && le > 0 && leEnd > le, `row :${row}, Total :${tot}, sentence :${sent}-:${sentEnd}, loose ends :${le}-:${leEnd}`);
  const allowedOld = new Set([row]); for (let n = sent; n <= sentEnd; n++) allowedOld.add(n);
  const bad = []; let pmRows = 0;
  for (const h of hunks(F)) {
    const olds = Array.from({ length: h.oldCount }, (_, k) => h.oldStart + k);
    if (h.oldCount) { if (!olds.every(n => allowedOld.has(n))) bad.push(`hunk at main :${h.oldStart}`); }
    else if (h.oldStart >= le && h.oldStart < leEnd && h.add.length === 1 && h.add[0].startsWith('| ') && h.add[0].includes('PM-0002-36')) pmRows++;
    else bad.push(`addition after main :${h.oldStart} (${h.add.length} lines)`);
  }
  check('the register: every change is the ADR 0002 row, the status sentence, or one added PM-0002-36 row in the loose ends', bad.length === 0 && pmRows === 1, bad.join('; ') || `PM-0002-36 rows added: ${pmRows}`);
  const head = readFileSync(join(REPO, F), 'utf8');
  const hrow = head.split('\n').find(l => l.startsWith('| [0002](0002-')) || '';
  check('the register: the ADR 0002 row still says "8 open" and carries the acceptance words', hrow.includes('8 open') && hrow.includes("not yet accepted except ADR 0002's part 1, accepted by Mustafa on 2026-11-10 (SCOPE (a)"));
  // D1 (a) via plan-check §3 as sent (119b71d: "-2 answered by the restructure plan" in this cell), with REG-TRUE (b)'s
  // two substitutions (the count stays 8; "answered" → "answered in part"), and the register's own rule ("marked")
  check('the register: the ADR 0002 row marks -2 "answered in part" (the register\'s rule) and keeps the count at 8',
    / 8 open: -8 answered at the stop \(2026-11-03\); -2 answered in part by the restructure plan \(2026-11-11\) \|$/.test(hrow));
  check('the register: the status sentence carries the acceptance words', fold(head).includes("not yet accepted except ADR 0002's part 1, accepted by Mustafa on 2026-11-10 (SCOPE (a)). His words"));
  check('the register: the Total line unchanged', head.includes(L[tot - 1]));
  const pm = head.split('\n').find(l => l.startsWith('| ') && l.includes('PM-0002-36')) || '';
  check("the register: the PM-0002-36 row calls the reading the advisor agent's (Q-A1, 2026-11-10), recorded on his D1 (a)", /advisor agent/.test(pm) && pm.includes('Q-A1') && pm.includes('2026-11-10') && /D1 \(a\)/.test(pm), pm.slice(0, 140));
}

// ---------- 5. quotes on added lines ----------
{
  const A = ['hi', 'task', 'd1', 'permission', 'answers', 'auto'].map(n => readFileSync(join(NOTES, `mustafa-2026-11-11-${n}.md`), 'utf8')).join('\n');
  const corpus = fold(A + '\n' + git('ls-tree', '-r', '--name-only', MAIN, 'docs').trim().split('\n').filter(f => /\.(md|json|html)$/.test(f)).map(f => git('show', `${MAIN}:${f}`)).join('\n'));
  const files = changed.filter(f => f !== 'docs/DECISIONS.md' && f !== 'docs/reports/2026-11-11-foundation-2-1b.md');
  const misses = [];
  for (const f of files) for (const l of git('diff', '-U0', MAIN, '--', f).split('\n').filter(l => l.startsWith('+') && !l.startsWith('+++'))) {
    for (const m of l.matchAll(/"([^"]{20,})"/g)) if (!corpus.includes(fold(m[1]))) misses.push(`${f}: "${m[1].slice(0, 70)}"`);
  }
  check('every quoted span (20+ characters) on an added docs line is his words (A1-A6) or a docs line at main', misses.length === 0, misses.slice(0, 6).join(' | ') || 'none outside');
}

// ---------- 6. START_HERE's commands ----------
{
  const ran = JSON.parse(readFileSync(join(NOTES, 'e6-commands-result.json'), 'utf8')).results.map(r => r.command.replace(/^xvfb-run -a /, ''));
  const cmds = [];
  for (const l of git('diff', '-U0', MAIN, '--', 'docs/START_HERE.md').split('\n').filter(l => l.startsWith('+') && !l.startsWith('+++')))
    for (const m of l.matchAll(/`((?:node|python)[^`]*\blegacy\b[^`]*)`/g)) cmds.push(m[1]);
  // a command's core: the program and its flags up to the folder `legacy`, then the subcommand (and, after `eval`, the
  // script); placeholders (<...>) and optional parts ([...]) and later flags may differ from E6's literal runs
  const core = c => {
    const t = c.trim().split(/\s+/), i = t.indexOf('legacy');
    if (i < 0) return c.trim();
    const end = t[i + 1] === 'eval' ? i + 3 : i + 2;
    return t.slice(0, end).filter(x => !/^[<\[]/.test(x)).join(' ');
  };
  const notRun = cmds.filter(c => !ran.some(r => r.startsWith(core(c))));
  check("START_HERE: every added `legacy` command's core is one E6 ran", notRun.length === 0, notRun.join(' | ') || `${cmds.length} commands`);
}

// ---------- 7. the render-budget note ----------
{
  const KEY = 'A note for the render budget bench (ADR 0003), 2026-11-11.';
  const where = git('grep', '-l', '-F', KEY, '--', '.').trim().split('\n').filter(Boolean).filter(f => !f.startsWith('.checkpoints/'));
  const wt = spawnSync('grep', ['-rlF', KEY, 'docs', 'CLAUDE.md', 'README.md', 'tools'], { cwd: REPO, encoding: 'utf8' }).stdout.trim().split('\n').filter(Boolean);
  const all = [...new Set([...where, ...wt])];
  check('his render-budget note is only in DECISIONS.md and the 2.1b report', all.every(f => f === 'docs/DECISIONS.md' || f === 'docs/reports/2026-11-11-foundation-2-1b.md') && all.includes('docs/DECISIONS.md'), all.join(', '));
}

// ---------- 8. no .checkpoints/ pointers on added docs lines ----------
{
  const hits = [];
  // DECISIONS.md's new lines are his words, byte-checked; a path he names there is his, not a pointer of ours
  for (const f of changed.filter(f => f.startsWith('docs/') && f !== 'docs/reports/2026-11-11-foundation-2-1b.md' && f !== 'docs/DECISIONS.md'))
    for (const l of git('diff', '-U0', MAIN, '--', f).split('\n').filter(l => l.startsWith('+') && !l.startsWith('+++')))
      if (/\.checkpoints\//.test(l)) hits.push(`${f}: ${l.slice(0, 100)}`);
  check('no added docs line points at a .checkpoints/ path', hits.length === 0, hits.slice(0, 4).join(' | '));
}

// ---------- 9. repo-map-check ----------
{
  const p = spawnSync(process.execPath, ['.claude/scripts/repo-map-check.mjs'], { cwd: REPO, encoding: 'utf8' });
  check('node .claude/scripts/repo-map-check.mjs exits 0', p.status === 0, `exit ${p.status}: ${((p.stdout || '') + (p.stderr || '')).trim().split('\n').slice(-2).join(' | ').slice(0, 300)}`);
}
console.log('docs check: ' + (ok ? 'PASS' : 'FAIL'));
process.exit(ok ? 0 : 1);
