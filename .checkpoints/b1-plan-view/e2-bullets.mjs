// E2 (condition 4): EDITOR_V2's existing text, compared per bullet (wrapped lines joined), not per line. Read-only.
// Run from the repository root: node .checkpoints/b1-plan-view/e2-bullets.mjs [file] [baseRef] [pathAtBase]
// (pathAtBase defaults to file; pass docs/EDITOR_V2.md when file is a scratch copy, e.g. for a mutant.)
// A block is a heading, a list item (with its wrapped continuation lines) or a paragraph; its text is its lines
// joined, whitespace collapsed, list marker kept (so item numbers are compared too).
// Pass when: the headings (and so the section numbers 1-6) are identical; every base block appears, in order, either
// unchanged or as its old text plus exactly one bracketed pointer at its end that names "Decided 2026-11-05"; the
// intro paragraph may gain only the "Updated on 2026-11-05 ..." line; every other new block is one of the planned
// additions (the Principle pointer, the dated block and its nine [DECIDED] items, the §6 pointer).
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
const file = process.argv[2] || 'docs/EDITOR_V2.md';
const baseRef = process.argv[3] || 'af8393e';
const basePath = process.argv[4] || file;
function blocks(text) {
  const out = []; let cur = null;
  const flush = () => { if (cur) out.push({ ...cur, text: cur.lines.join(' ').replace(/\s+/g, ' ').trim() }); cur = null; };
  text.split('\n').forEach((l, i) => {
    if (!l.trim()) { flush(); return; }
    const ind = l.match(/^\s*/)[0].length;
    if (/^#/.test(l)) { flush(); out.push({ kind: 'heading', line: i + 1, text: l.trim() }); return; }
    if (/^\s*([-*+]|\d+\.)\s/.test(l)) { flush(); cur = { kind: 'item', line: i + 1, ind, lines: [l] }; return; }
    if (cur) { cur.lines.push(l); return; }
    cur = { kind: 'para', line: i + 1, ind, lines: [l] };
  });
  flush(); return out;
}
function onePointer(oldT, newT) {
  if (!newT.startsWith(oldT + ' (') || !newT.endsWith(')')) return null;
  const s = newT.slice(oldT.length + 1); let d = 0;
  for (let i = 0; i < s.length; i++) { if (s[i] === '(') d++; else if (s[i] === ')') { d--; if (d === 0 && i !== s.length - 1) return null; } if (d < 0) return null; }
  return d === 0 && s.includes('Decided 2026-11-05') ? s : null;
}
const ALLOWED = [
  t => t.startsWith('Principle (Mustafa, 2026-11-05): §3, Decided 2026-11-05.'),
  t => t.startsWith('- **Decided 2026-11-05 (Mustafa, after his talk with the Advisor).**'),
  t => t.startsWith('- `[DECIDED]` '),
  t => t.startsWith("Mustafa's tool order (§3, Decided 2026-11-05) ranks the tools; the architecture plan (step 2) maps it onto these slices, for his approval."),
];
const UPDATED = " Updated on 2026-11-05 with Mustafa's editor decisions";
const base = blocks(execFileSync('git', ['show', `${baseRef}:${basePath}`], { encoding: 'utf8' }));
const head = blocks(readFileSync(file, 'utf8'));
let fail = 0; const log = (c, m) => { if (!c) fail++; console.log(`${c ? 'ok  ' : 'FAIL'} ${m}`); };
const hb = base.filter(b => b.kind === 'heading').map(b => b.text), hh = head.filter(b => b.kind === 'heading').map(b => b.text);
log(JSON.stringify(hb) === JSON.stringify(hh), `headings identical (${hh.length}): ${hh.map(h => h.replace(/^#+ /, '').split(' ')[0]).join(' ')}`);
let j = 0, same = 0, ptr = 0, added = 0, decided = 0; const ptrTexts = [];
for (const b of head) {
  const o = base[j];
  if (o && b.text === o.text) { same++; j++; continue; }
  if (o && o.kind === b.kind) {
    const p = onePointer(o.text, b.text);
    if (p) { ptr++; j++; ptrTexts.push(p); console.log(`ptr  base :${o.line} -> :${b.line} ${o.text.slice(0, 40)}... + ${p}`); continue; }
    if (o.kind === 'para' && o.line === 3 && b.text.startsWith(o.text + UPDATED) && !b.text.slice(o.text.length + UPDATED.length).includes('\n')) {
      j++; console.log(`upd  base :${o.line} -> :${b.line} intro + "${b.text.slice(o.text.length + 1)}"`); continue;
    }
  }
  if (ALLOWED.some(f => f(b.text))) { added++; if (b.text.startsWith('- `[DECIDED]` ')) decided++; console.log(`new  :${b.line} ${b.text.slice(0, 70)}...`); continue; }
  log(false, `:${b.line} neither an old block (+ one pointer) nor a planned addition: ${b.text.slice(0, 90)}`);
}
log(j === base.length, `every base block found in order (${j}/${base.length}): ${same} unchanged, ${ptr} with one pointer`);
log(ptr <= 5, `pointers on old blocks: ${ptr} (planned 5: Select and transform, Test, Validation panel, §2 rule 4, [LATER])`);
log(decided === 9, `new [DECIDED] items in the dated block: ${decided} (planned 9)`);
log(![...ptrTexts, ...head.filter(b => ALLOWED.some(f => f(b.text))).map(b => b.text)].some(t => /interior/i.test(t)), 'no pointer or new block mentions interiors');
console.log(`blocks: base ${base.length}, head ${head.length}, new ${added}`);
console.log(fail ? `E2 FAIL (${fail})` : 'E2 PASS');
process.exit(fail ? 1 : 0);
