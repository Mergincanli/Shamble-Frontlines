// decisions-entries.mjs (foundation-restructure, step 2.1b, step 10; the advisor agent's C13): docs/DECISIONS.md gets
// one entry per message of Mustafa's in this session, newest first at the top of "Decisions, newest first", each quote
// copied by script from the report's Appendix A (docs/reports/2026-11-11-foundation-2-1b.md), never retyped; and his
// render-budget note, word for word from inside its quote marks in his task message (A2), as one item under "Queued".
// Additive: an entry whose event id is already in the file, and a Queued note already there, are skipped, so a later
// message (A7, ...) is added by a new ENTRIES line and a re-run, never by hand.
// Checks, after writing: every quoted line equals its Appendix A block byte for byte; the Queued note equals his words;
// removing the lines this run inserted gives back the file as it was byte for byte (nothing else changed).
//   node .gate/foundation-restructure/decisions-entries.mjs dry    -> .gate/foundation-restructure/DECISIONS.dry.md
//   node .gate/foundation-restructure/decisions-entries.mjs write  -> docs/DECISIONS.md
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const REPO = '/home/user/Shamble-Frontlines';
const REPORT = 'docs/reports/2026-11-11-foundation-2-1b.md';
const SESSION = 'session_019YNMUWdGJZoQFUuuvvBP1Y';
const mode = process.argv[2];
if (!['dry', 'write', 'regen-dry', 'regen'].includes(mode)) { console.error('usage: decisions-entries.mjs dry | write | regen-dry | regen'); process.exit(2); }
const md = readFileSync(join(REPO, REPORT), 'utf8');
const at = md.indexOf("## Appendix A. Mustafa's words for this change-set, in full");
if (at < 0) { console.error('no Appendix A in ' + REPORT); process.exit(2); }
const A = {};
for (const m of md.slice(at).matchAll(/\n### (A\d+)\. [^\n]*\(event ([0-9a-f-]{36})[^\n]*\n\n```text\n([\s\S]*?)\n```\n/g)) A[m[1]] = { event: m[2], text: m[3] };
// wraps the session's own text at 120 columns (never his quoted lines): first line after `first`, the rest after `rest`
const wrap = (text, first, rest, width = 120) => {
  const words = text.replace(/\s*\n\s*/g, ' ').split(' ');
  const out = []; let line = first;
  for (const w of words) {
    if (line.length > first.length && line !== rest && (line + ' ' + w).length > width) { out.push(line); line = rest + w; }
    else line = line === first || line === rest ? line + w : line + ' ' + w;
  }
  out.push(line);
  return out.join('\n');
};
const quote = t => t.split('\n').map(l => (l === '' ? '>' : '> ' + l)).join('\n');
const src = (id, note = '') => `(foundation 2.1b, PR #51; his message in that change-set's session, ${SESSION}, on his personal account, extracted by script from the session's events, event ${A[id].event}${note}). `;
const copied = id => `Copied by script from \`${REPORT}\`, Appendix ${id}, and checked byte for byte against it:`;
// newest first; a later message gets its own line at the top
const ENTRIES = [
  { id: 'A6', head: () => `- **Taken by Mustafa, 2026-11-11, 22:38:50 UTC: his second message on auto mode** ${src('A6')}The session's answer is in the report. ${copied('A6')}` },
  { id: 'A5', head: () => `- **Taken by Mustafa, 2026-11-11, 21:39:44 UTC: KIT-2.1B (a), E8-RUN (a), REG-TRUE (b), and the build go for steps 6 and 7** ${src('A5')}KIT-2.1B, E8-RUN and REG-TRUE are the session's three asks of that evening, in the report with the exact lines asked. His REG-TRUE (b) replaces D1 (a)'s "answered" wording and new count in the entry below. ${copied('A5')}` },
  { id: 'A4', head: () => `- **Taken by Mustafa, 2026-11-11, 21:10:58 UTC: his first message on auto mode** ${src('A4')}The session's answer is in the report. ${copied('A4')}` },
  { id: 'A3', head: () => `- **Taken by Mustafa, 2026-11-11, 20:44:29 UTC: D1 (a), his Windows run of the new gate, and the Node 24 finding for 2.3** ${src('A3', '; sent again byte-identical at 21:27:35 UTC, event 40ebaa1e-996c-42b2-b313-4198c6274017')}D1 is the session's stop of that evening: plan revision 3 named "the register's OQ-0002-2 row", which does not exist (the report). His REG-TRUE (b), above, replaces this entry's "answered" wording and new count; the rest of D1 (a) stands. ${copied('A3')}` },
  { id: 'A2', head: () => `- **Taken by Mustafa, 2026-11-11, 20:21:02 UTC: step 2.1b's task** ${src('A2', '; first sent at 20:18:13 UTC, event 6743399f-9d86-4e25-8c08-aee3542d31dc, byte-identical, and interrupted by him at 20:19:40 UTC')}Its render-budget note waits in "Queued", above. ${copied('A2')}` },
  { id: 'A1', head: () => `- **Taken by Mustafa, 2026-11-11, 20:20:36 UTC: his message before the task, in foundation 2.1b's session** ${src('A1')}Before it, at 20:19:47 UTC, he sent "hi", event 844bb920-ac8e-4c79-bc10-5682fbd74f53, and interrupted it at 20:20:24 UTC (Appendix A1). His task message, the entry above, followed 26 seconds later. ${copied('A1')}` },
  { id: 'A7', head: () => `- **Taken by Mustafa, 2026-11-11, 18:37:21 UTC: his go for #50** (foundation 2.1a, PR #50; his message in that change-set's session, session_019YNMUWLffMoE47SbvkqJ5P, on his personal account, extracted by script from that session's events, event ${A.A7.event}; merge commit 76b4f7a's message quotes it byte for byte, checked by script; recorded here by foundation 2.1b, PR #51, after the advisor agent's review). ${copied('A7')}` },
];
for (const e of ENTRIES) if (!A[e.id]) { console.error(`no ${e.id} block in the report`); process.exit(2); }
for (const id of Object.keys(A)) if (!ENTRIES.some(e => e.id === id)) { console.error(`Appendix ${id} has no ENTRIES line: add one at the top`); process.exit(2); }
// the render-budget note: the one line of A2 that opens with its quote mark, without the two quote marks
const noteLines = A.A2.text.split('\n').filter(l => l.startsWith('"A note for the render budget bench (ADR 0003), 2026-11-11.'));
if (noteLines.length !== 1 || !noteLines[0].endsWith('"')) { console.error(`the note line: ${noteLines.length} found`); process.exit(2); }
const NOTE = noteLines[0].slice(1, -1);
const QLEAD = `The render budget bench note (his task message of 2026-11-11, 20:21:02 UTC, in foundation 2.1b's session, under "Decisions, newest first": the entry "Taken by Mustafa, 2026-11-11, 20:21:02 UTC: step 2.1b's task"; he asked for it to be queued "in my words in docs/DECISIONS.md under "Queued""; copied by script from \`${REPORT}\`, Appendix A2, from inside its quote marks):`;

const path = join(REPO, 'docs/DECISIONS.md');
const orig = readFileSync(path, 'utf8');
const lines = orig.split('\n');
const iDec = lines.indexOf('## Decisions, newest first');
const iPtr = lines.indexOf('  - The entries below are pointers to items carried out.');
if (iDec < 0 || iPtr < 0 || iPtr > iDec) { console.error('anchors not found'); process.exit(2); }
const decSection = lines.slice(iDec).join('\n');
if (mode.startsWith('regen')) {
  const iNext = lines.findIndex((l, i) => i > iDec && l.startsWith('- **Taken by Mustafa, 2026-11-11, 16:54:54 UTC'));
  const old = lines.slice(iDec + 1, iNext);
  const heads = old.filter(l => l.startsWith('- **'));
  if (iNext < 0 || !heads.length || !old[0].startsWith('- **Taken by Mustafa, 2026-11-11, 22:38:50 UTC')) { console.error('regen: the block is not where it should be'); process.exit(2); }
  const ownText = old.join('\n');
  for (const h of heads) { const i = old.indexOf(h); const para = old.slice(i, i + 8).join(' ').replace(/\s+/g, ' '); if (!/PR #51/.test(para)) { console.error('regen: an entry not made by this script: ' + h.slice(0, 80)); process.exit(2); } }
  const fresh = ENTRIES.map(e => wrap(e.head().replace(/^- /, ''), '- ', '  ') + '\n' + quote(A[e.id].text)).join('\n').split('\n');
  const outR = [...lines.slice(0, iDec + 1), ...fresh, ...lines.slice(iNext)].join('\n');
  let okR = true; const chk = (n, p, d = '') => { if (!p) okR = false; console.log(`${p ? 'PASS' : 'FAIL'} ${n}${d ? ' :: ' + d : ''}`); };
  const nl = outR.split('\n');
  for (const e of ENTRIES) {
    const hi = nl.findIndex(l => l.startsWith(e.head().slice(0, 50)));
    let j = hi; while (j + 1 < nl.length && !(nl[j + 1] === '>' || nl[j + 1].startsWith('> '))) j++;
    const q = []; for (let k = j + 1; k < nl.length && (nl[k] === '>' || nl[k].startsWith('> ')); k++) q.push(nl[k] === '>' ? '' : nl[k].slice(2));
    chk(`${e.id}: the entry's quote equals Appendix ${e.id} byte for byte`, hi >= 0 && q.join('\n') === A[e.id].text, `${q.length} lines`);
  }
  chk('outside the regenerated block the file is unchanged', [...nl.slice(0, iDec + 1), ...nl.slice(iDec + 1 + fresh.length)].join('\n') === [...lines.slice(0, iDec + 1), ...lines.slice(iNext)].join('\n'));
  chk('the old block was only this script\'s entries', heads.length === 6, `${heads.length} entries replaced by ${ENTRIES.length}`);
  if (!okR) { console.error('regen: FAIL (nothing written)'); process.exit(1); }
  const tgt = mode === 'regen' ? path : join(REPO, '.gate/foundation-restructure/DECISIONS.dry.md');
  writeFileSync(tgt, outR); console.log(`regen: PASS; ${ENTRIES.length} entries written to ${tgt.replace(REPO + '/', '')}`);
  process.exit(0);
}
const todo = ENTRIES.filter(e => !decSection.includes(`event ${A[e.id].event}`));
const skipped = ENTRIES.filter(e => !todo.includes(e)).map(e => e.id);
const noteThere = lines.slice(0, iDec).includes('    > ' + NOTE);
const entryBlock = todo.length ? todo.map(e => wrap(e.head().replace(/^- /, ''), '- ', '  ') + '\n' + quote(A[e.id].text)).join('\n').split('\n') : [];
const queuedBlock = noteThere ? [] : [...wrap(QLEAD, '  - ', '    ').split('\n'), '    > ' + NOTE];
const out = [...lines.slice(0, iPtr), ...queuedBlock, ...lines.slice(iPtr, iDec + 1), ...entryBlock, ...lines.slice(iDec + 1)];
const text = out.join('\n');

// checks on the new text
let ok = true;
const check = (name, pass, detail = '') => { if (!pass) ok = false; console.log(`${pass ? 'PASS' : 'FAIL'} ${name}${detail ? ' :: ' + detail : ''}`); };
const nl = text.split('\n');
for (const e of ENTRIES) {
  const hi = nl.findIndex(l => l.startsWith(e.head().slice(0, 50)));
  if (hi < 0) { check(`${e.id}: the entry is in the file`, false); continue; }
  let j = hi; while (j + 1 < nl.length && !(nl[j + 1] === '>' || nl[j + 1].startsWith('> '))) j++;
  const q = []; for (let k = j + 1; k < nl.length && (nl[k] === '>' || nl[k].startsWith('> ')); k++) q.push(nl[k] === '>' ? '' : nl[k].slice(2));
  check(`${e.id}: the entry's quote equals Appendix ${e.id} byte for byte${skipped.includes(e.id) ? ' (already there)' : ''}`, q.join('\n') === A[e.id].text, `${q.length} lines, ${A[e.id].text.length} characters`);
}
const qi = nl.indexOf('    > ' + NOTE);
check('the Queued note is his words from inside the quote marks, on one line', qi > 0 && qi < nl.indexOf('## Decisions, newest first') && A.A2.text.includes('"' + NOTE + '"'), `${NOTE.length} characters`);
const stripped = [...nl.slice(0, iPtr), ...nl.slice(iPtr + queuedBlock.length, iDec + 1 + queuedBlock.length), ...nl.slice(iDec + 1 + queuedBlock.length + entryBlock.length)].join('\n');
check('removing the lines this run inserted gives back the file as it was, byte for byte', stripped === orig);
check('the newest entry comes first, right under the heading', nl[nl.indexOf('## Decisions, newest first') + 1].startsWith(ENTRIES[0].head().slice(0, 50)));
if (!ok) { console.error('decisions entries: FAIL (nothing written)'); process.exit(1); }
const target = mode === 'write' ? path : join(REPO, '.gate/foundation-restructure/DECISIONS.dry.md');
writeFileSync(target, text);
console.log(`decisions entries: PASS; ${todo.length} entries added${skipped.length ? ` (${skipped.join(', ')} already there)` : ''}, ${queuedBlock.length ? '1 Queued item added' : 'the Queued note already there'}; written to ${target.replace(REPO + '/', '')} (+${entryBlock.length + queuedBlock.length} lines)`);
