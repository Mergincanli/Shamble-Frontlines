// docs-keeper pass for #56's merge and Part 15 to main (#57). Moves #56's STATUS row into the archive by line range,
// copies its open items into the archive's live Follow-ups by script, and writes the new lines from
// docs-keeper-newtext.txt. Asserts every line it replaces; refuses to run twice. Usage: node <this> [--dry]
import fs from 'node:fs';
import path from 'node:path';

const ROOT = '/home/user/Shamble-Frontlines';
const DRY = process.argv.includes('--dry');
const here = path.dirname(new URL(import.meta.url).pathname);
const rd = (p) => fs.readFileSync(path.join(ROOT, p), 'utf8');
const fail = (m) => { console.error('ASSERT FAILED: ' + m); process.exit(1); };

// ---- new text sections
const nt = fs.readFileSync(path.join(here, 'docs-keeper-newtext.txt'), 'utf8').split('\n');
const S = {}; let cur = null;
for (const l of nt) {
  if (l.startsWith('@@')) { cur = l.slice(2); if (cur !== 'END') S[cur] = []; continue; }
  if (cur && cur !== 'END') S[cur].push(l);
}
const one = (k) => { if (S[k].length !== 1) fail(k + ' should be one line'); return S[k][0]; };
const joinPara = (k) => S[k].join(' ');

// ---- STATUS
const st = rd('docs/STATUS.md').split('\n');
if (st.some((l) => l.startsWith('| #57 |'))) fail('STATUS already has a #57 row: already applied');
const L = (n) => st[n - 1];
const expect = (n, s) => { if (L(n) !== s) fail(`STATUS L${n} differs:\n  have: ${L(n)}\n  want: ${s}`); };
expect(5, '_Last updated: 2026-11-17 (change-set 0 of the build plan, PR #56; main c94c81b)._');
expect(13, '- **Last merged:** DOCS-54 (PR #55, c94c81b, 2026-11-17 17:09:33 UTC by `merged_at`, on his go); every merge: ROADMAP.');
if (!L(28).startsWith('| #40 |')) fail('STATUS L28 is not #40');
if (!L(29).startsWith('| #56 | change-set 0 of the build plan (')) fail('STATUS L29 is not #56');
expect(35, '2. v2: the netcode proof test (#40); the foundation (his order of 2026-11-11): #49, #50, #51, #48 and #52 merged; step');
expect(36, '   2.3: "The 2.3 chat stays stopped for now: I pass it nothing until its plan has been checked." Then the build plan\'s');
expect(37, '   change-sets (#56 first); "Slice 3, online, still waits for the netcode proof test\'s result."');
expect(40, '- His go for #56 (change-set 0; risk tier 1, decision docs; no Brave check), once the advisor approves its final head.');
expect(41, '- Part 15, "The second lane": answered 2026-11-17, 22:10 UTC, all six (a) (his words: 023e6f3, `.checkpoints/part-15/`');
expect(42, '  `mustafa-2026-11-17-2306.md`; text: ea851a8). It goes to main next, in a docs change of its own before change-set 1.');
expect(43, '- The netcode proof test\'s items (its plan in #40; the older list: the archive\'s "Phase", "Next (v2)"): OK each run\'s');
expect(100, '  each a CI edit that asks him); OVERFLOW-PANIC-RULE (with the first code in the room crate or a server crate).');
expect(101, '');
if (!L(102).startsWith('## Open risks')) fail('STATUS L102 is not Open risks');

// #56's row: the line range 29..29, moved (cells kept by script)
const row56 = L(29);
const cells = row56.replace(/^\| /, '').replace(/ \|$/, '').split(' | ');
if (cells.length !== 5) fail('#56 row has ' + cells.length + ' cells');
const [pr, work, branch, state, waiting] = cells;
if (pr !== '#56') fail('first cell ' + pr);
if (!state.startsWith('ready for review; ')) fail('state cell prefix');
if (!waiting.startsWith('his go')) fail('waiting cell');
const OPEN = 'Open after it, for "Follow-ups" once merged: ';
const oi = work.indexOf(OPEN);
if (oi < 0) fail('open items marker not found');
const openText = work.slice(oi + OPEN.length);
// split at top-level "; " (outside parentheses)
const items = []; let depth = 0, start = 0;
for (let i = 0; i < openText.length; i++) {
  const c = openText[i];
  if (c === '(') depth++; else if (c === ')') depth--;
  else if (c === ';' && depth === 0 && openText[i + 1] === ' ') { items.push(openText.slice(start, i)); start = i + 2; }
}
items.push(openText.slice(start));
if (items.length !== 5) fail('expected 5 open items, got ' + items.length + ': ' + JSON.stringify(items));

const newSt = [];
for (let n = 1; n <= st.length; n++) {
  if (n === 5) { newSt.push(one('STATUS_L5')); continue; }
  if (n === 13) { newSt.push(one('STATUS_L13')); continue; }
  if (n === 29) { newSt.push(one('STATUS_ROW57')); continue; }
  if (n === 36) { newSt.push(...S.STATUS_NEXT); continue; }
  if (n === 37) continue;
  if (n === 40) { newSt.push(...S.STATUS_WAITING); continue; }
  if (n === 41 || n === 42) continue;
  newSt.push(L(n));
  if (n === 100) newSt.push(...S.STATUS_FU);
}

// ---- ARCHIVE
const ar = rd('docs/STATUS_ARCHIVE.md').split('\n');
const A = (n) => ar[n - 1];
if (A(156) !== '## In flight') fail('archive L156');
if (A(157) !== '| Work | Branch | State | Waiting on |') fail('archive L157');
if (A(158) !== '|---|---|---|---|') fail('archive L158');
if (ar.some((l) => l.includes('merged: PR #56 ('))) fail('archive already has #56: already applied');
const fuEndIdx = ar.indexOf('## Decisions pending: the rest');
if (fuEndIdx < 0) fail('no Decisions pending: the rest');
if (ar[fuEndIdx - 1] !== '') fail('no blank line before Decisions pending');
const lastFu = fuEndIdx - 1; // 1-based line number of the last Follow-ups line = fuEndIdx - 1
if (!A(lastFu).startsWith('    - The other five are follow-ups. ADV-NS-L2')) fail('archive last Follow-ups line: ' + A(lastFu).slice(0, 60));

if (!work.startsWith('change-set 0 of the build plan (')) fail('work cell start');
const workB = '**change-set 0 of the build plan** (' + work.slice('change-set 0 of the build plan ('.length);
const mergedCell = joinPara('ARCH_MERGED_CELL').replace('{BRANCH}', branch);
const doneCell = 'done: ' + state.slice('ready for review; '.length) + joinPara('ARCH_DONE_TAIL');
const arRow = '| ' + workB + ' | ' + mergedCell + ' | ' + doneCell + ' | none |';

// wrap: width 120, quoted spans kept whole
function wrap(text, first, rest, width = 120) {
  const raw = text.split(' ');
  const toks = [];
  for (let i = 0; i < raw.length; i++) {
    let t = raw[i];
    const opens = (t.match(/"/g) || []).length;
    if (opens % 2 === 1) { // a quote opens here and closes later
      while (i + 1 < raw.length) { i++; t += ' ' + raw[i]; if ((raw[i].match(/"/g) || []).length % 2 === 1) break; }
    }
    toks.push(t);
  }
  const out = []; let line = first;
  for (const t of toks) {
    const cand = (line === first || line === rest) ? line + t : line + ' ' + t;
    if (cand.length > width && line !== first && line !== rest) { out.push(line); line = rest + t; }
    else line = cand;
  }
  out.push(line);
  return out;
}
const src = [joinPara('ARCH_FU_SRC1'), joinPara('ARCH_FU_SRC2'), joinPara('ARCH_FU_SRC3'), joinPara('ARCH_FU_SRC4'), joinPara('ARCH_FU_SRC5')];
const fuBlock = [...S.ARCH_FU_HEADER];
items.forEach((it, i) => fuBlock.push(...wrap(it + '. ' + src[i], '  - ', '    ')));
fuBlock.push(...wrap(joinPara('ARCH_FU_ITEM6'), '  - ', '    '));

const newAr = [];
for (let n = 1; n <= ar.length; n++) {
  newAr.push(A(n));
  if (n === 158) newAr.push(arRow);
  if (n === lastFu) newAr.push(...fuBlock);
}

// ---- ROADMAP
const rm = rd('docs/ROADMAP.md').split('\n');
const R = (n) => rm[n - 1];
if (R(122) !== '## Recent work') fail('ROADMAP L122');
if (R(124) !== '') fail('ROADMAP L124 not blank');
if (R(125) !== '**Change-set 0 of the build plan [2026-11-17, PR #56: ready for review]** — cs0-model-rules, on his go of 2026-11-17 (docs only):') fail('ROADMAP L125: ' + R(125));
if (R(128) !== '(open questions 98 to 96). Report `docs/reports/2026-11-17-cs0-model-rules.md`. Two review rounds done (the verifier: PASS); then his go.') fail('ROADMAP L128');
if (R(129) !== '') fail('ROADMAP L129 not blank');
if (!R(130).startsWith("**Main's docs after #52, #53 and #54 [2026-11-17, MERGED]**")) fail('ROADMAP L130');
const newRm = [];
for (let n = 1; n <= rm.length; n++) {
  if (n === 125) { newRm.push(...S.ROADMAP_NEW); newRm.push(one('ROADMAP_L125')); continue; }
  if (n === 128) { newRm.push(...S.ROADMAP_L128); continue; }
  newRm.push(R(n));
}

// ---- REPO_MAP
const mp = rd('docs/REPO_MAP.md').split('\n');
const M = (n) => mp[n - 1];
if (M(80) !== '| `docs/reports/2026-11-17-cs0-model-rules.md` | The report of change-set 0 of the build plan (PR #56) |') fail('REPO_MAP L80');
if (mp.some((l) => l.includes('2026-11-17-build-plan-slice-1-part-15.md'))) fail('REPO_MAP already has Part 15');
const newMp = [];
for (let n = 1; n <= mp.length; n++) { newMp.push(M(n)); if (n === 80) newMp.push(...S.REPOMAP_ROWS); }

// ---- checks
const dec = rd('docs/DECISIONS.md');
const quotes = [
  'It is not waived. The way out is my call, with the plan for the first-model change-set.',
  'also my word on a seventh package beside the six the structure draft names',
];
for (const q of quotes) if (!dec.includes(q)) fail('quote not in DECISIONS: ' + q);
for (const q of quotes) if (!newAr.some((l) => l.includes(q))) fail('quote broken across lines in the archive: ' + q);
const added = (oldA, newA) => { const s = new Set(oldA); return newA.filter((l) => !s.has(l)); };
let wide = 0;
for (const [name, o, nw] of [['STATUS', st, newSt], ['ARCHIVE', ar, newAr], ['ROADMAP', rm, newRm], ['REPO_MAP', mp, newMp]]) {
  for (const l of added(o, nw)) if (!l.startsWith('|') && [...l].length > 120) { wide++; console.log(`WIDE ${name} ${[...l].length}: ${l}`); }
}
const nl = (a) => a.length - (a[a.length - 1] === '' ? 1 : 0); // lines, not counting the split after the final newline
console.log('STATUS lines: ' + nl(st) + ' -> ' + nl(newSt));
console.log('ARCHIVE lines: ' + nl(ar) + ' -> ' + nl(newAr) + '; ROADMAP: ' + nl(rm) + ' -> ' + nl(newRm) + '; REPO_MAP: ' + nl(mp) + ' -> ' + nl(newMp));
console.log('open items:'); items.forEach((it, i) => console.log('  ' + (i + 1) + '. ' + it));
console.log('wide added lines (non-table, >120): ' + wide);
if (nl(newSt) > 120) fail('STATUS over 120 lines: ' + nl(newSt));
if (wide) fail('wide lines');
if (DRY) { console.log('--- archive Follow-ups block:'); fuBlock.forEach((l) => console.log(l)); console.log('DRY: nothing written'); process.exit(0); }
fs.writeFileSync(path.join(ROOT, 'docs/STATUS.md'), newSt.join('\n'));
fs.writeFileSync(path.join(ROOT, 'docs/STATUS_ARCHIVE.md'), newAr.join('\n'));
fs.writeFileSync(path.join(ROOT, 'docs/ROADMAP.md'), newRm.join('\n'));
fs.writeFileSync(path.join(ROOT, 'docs/REPO_MAP.md'), newMp.join('\n'));
console.log('WRITTEN: STATUS, STATUS_ARCHIVE, ROADMAP, REPO_MAP');
