// decisions-entries.mjs (foundation-restructure, step 2.1b, step 10; the advisor agent's C13): docs/DECISIONS.md gets
// one entry per message of Mustafa's in this session, newest first at the top of "Decisions, newest first", each quote
// copied by script from the report's Appendix A (docs/reports/2026-11-11-foundation-2-1b.md), never retyped; and his
// render-budget note, word for word from inside its quote marks in his task message (A2), as one item under "Queued".
// Checks, after writing: every quoted line equals its Appendix A block byte for byte; the Queued note equals his words;
// removing the inserted lines gives back the original file byte for byte (nothing else changed).
//   node .gate/foundation-restructure/decisions-entries.mjs dry    -> .gate/foundation-restructure/DECISIONS.dry.md
//   node .gate/foundation-restructure/decisions-entries.mjs write  -> docs/DECISIONS.md
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const REPO = '/home/user/Shamble-Frontlines';
const REPORT = 'docs/reports/2026-11-11-foundation-2-1b.md';
const SESSION = 'session_019YNMUWdGJZoQFUuuvvBP1Y';
const mode = process.argv[2];
if (!['dry', 'write'].includes(mode)) { console.error('usage: decisions-entries.mjs dry | write'); process.exit(2); }
const md = readFileSync(join(REPO, REPORT), 'utf8');
const at = md.indexOf("## Appendix A. Mustafa's words for this change-set, in full");
if (at < 0) { console.error('no Appendix A in ' + REPORT); process.exit(2); }
const A = {};
for (const m of md.slice(at).matchAll(/\n### (A\d+)\. [^\n]*\(event ([0-9a-f-]{36})[^\n]*\n\n```text\n([\s\S]*?)\n```\n/g)) A[m[1]] = { event: m[2], text: m[3] };
for (const id of ['A1', 'A2', 'A3', 'A4', 'A5', 'A6']) if (!A[id]) { console.error(`no ${id} block in the report`); process.exit(2); }
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
const src = (id, note = '') => `(foundation 2.1b, PR #51; his message in that change-set's session, ${SESSION}, on his
  personal account, extracted by script from the session's events, event ${A[id].event}${note}). `;
const copied = id => `Copied by script from \`${REPORT}\`, Appendix ${id}, and checked byte for byte against it:`;
// newest first
const ENTRIES = [
  { id: 'A6', head: `- **Taken by Mustafa, 2026-11-11, 22:38:50 UTC: his second message on auto mode** ${src('A6')}The session was
  already in Auto mode; a session cannot answer a prompt itself, and his words of 2026-11-11, 16:32:18 UTC, below, bar
  editing a settings file, hook or allow-list to get fewer prompts (the answer given in chat is in the report). ${copied('A6')}` },
  { id: 'A5', head: `- **Taken by Mustafa, 2026-11-11, 21:39:44 UTC: KIT-2.1B (a), E8-RUN (a), REG-TRUE (b), and the build go for
  steps 6 and 7** ${src('A5')}KIT-2.1B, E8-RUN and REG-TRUE are the session's three asks of that evening, in the report
  with the exact lines asked. Carried out by foundation 2.1b: the four kit lines (one commit), the rollback rehearsal
  E8, and REG-TRUE in the register and ADR 0002; his REG-TRUE (b) replaces D1 (a)'s "answered" wording and new count in
  the entry below. ${copied('A5')}` },
  { id: 'A4', head: `- **Taken by Mustafa, 2026-11-11, 21:10:58 UTC: his first message on auto mode** ${src('A4')}The session was
  already in Auto mode (the same answer as for his second message, above). ${copied('A4')}` },
  { id: 'A3', head: `- **Taken by Mustafa, 2026-11-11, 20:44:29 UTC: D1 (a), his Windows run of the new gate, and the Node 24 finding
  for 2.3** ${src('A3', `; sent again byte-identical at 21:27:35 UTC, event 40ebaa1e-996c-42b2-b313-4198c6274017`)}D1 is
  the session's stop of that evening: plan revision 3 named "the register's OQ-0002-2 row", which does not exist (the
  report). His REG-TRUE (b), above, replaces this entry's "answered" wording and new count; the rest of D1 (a) stands.
  ${copied('A3')}` },
  { id: 'A2', head: `- **Taken by Mustafa, 2026-11-11, 20:21:02 UTC: step 2.1b's task** ${src('A2')}Its render-budget note waits in
  "Queued", above. ${copied('A2')}` },
  { id: 'A1', head: `- **Taken by Mustafa, 2026-11-11, 20:20:36 UTC: his first message in foundation 2.1b's session** ${src('A1')}His task
  message, the entry above, followed 26 seconds later. ${copied('A1')}` },
];
// the render-budget note: the one line of A2 that opens with its quote mark, without the two quote marks
const noteLines = A.A2.text.split('\n').filter(l => l.startsWith('"A note for the render budget bench (ADR 0003), 2026-11-11.'));
if (noteLines.length !== 1 || !noteLines[0].endsWith('"')) { console.error(`the note line: ${noteLines.length} found`); process.exit(2); }
const NOTE = noteLines[0].slice(1, -1);
const QUEUED = `  - The render budget bench note (his task message of 2026-11-11, 20:21:02 UTC, in foundation 2.1b's session, under
    "Decisions, newest first": the entry "Taken by Mustafa, 2026-11-11, 20:21:02 UTC: step 2.1b's task"; he asked for it
    to be queued "in my words in docs/DECISIONS.md under "Queued""; copied by script from \`${REPORT}\`, Appendix A2,
    from inside its quote marks):
    > ${NOTE}`;

const path = join(REPO, 'docs/DECISIONS.md');
const orig = readFileSync(path, 'utf8');
if (orig.includes('foundation 2.1b, PR #51; his message')) { console.error('the entries are already in docs/DECISIONS.md'); process.exit(2); }
const lines = orig.split('\n');
const iDec = lines.indexOf('## Decisions, newest first');
const iPtr = lines.indexOf('  - The entries below are pointers to items carried out.');
if (iDec < 0 || iPtr < 0 || iPtr > iDec) { console.error('anchors not found'); process.exit(2); }
const entryBlock = ENTRIES.map(e => wrap(e.head.replace(/^- /, ''), '- ', '  ') + '\n' + quote(A[e.id].text)).join('\n').split('\n');
const qLead = QUEUED.slice(0, QUEUED.lastIndexOf('\n')).replace(/^  - /, '');
const queuedBlock = [...wrap(qLead, '  - ', '    ').split('\n'), '    > ' + NOTE];
const out = [...lines.slice(0, iPtr), ...queuedBlock, ...lines.slice(iPtr, iDec + 1), ...entryBlock, ...lines.slice(iDec + 1)];
const text = out.join('\n');

// checks on the new text
let ok = true;
const check = (name, pass, detail = '') => { if (!pass) ok = false; console.log(`${pass ? 'PASS' : 'FAIL'} ${name}${detail ? ' :: ' + detail : ''}`); };
const nl = text.split('\n');
for (const e of ENTRIES) {
  const hi = nl.findIndex(l => l.startsWith(e.head.slice(0, 50)));
  let j = hi; while (!nl[j].endsWith('checked byte for byte against it:')) j++;
  const q = []; for (let k = j + 1; k < nl.length && (nl[k] === '>' || nl[k].startsWith('> ')); k++) q.push(nl[k] === '>' ? '' : nl[k].slice(2));
  check(`${e.id}: the entry's quote equals Appendix ${e.id} byte for byte`, q.join('\n') === A[e.id].text, `${q.length} lines, ${A[e.id].text.length} characters`);
}
const qi = nl.indexOf('    > ' + NOTE);
check('the Queued note is his words from inside the quote marks, on one line', qi > 0 && qi < nl.indexOf('## Decisions, newest first') && A.A2.text.includes('"' + NOTE + '"'), `${NOTE.length} characters`);
const stripped = [...nl.slice(0, iPtr), ...nl.slice(iPtr + queuedBlock.length, iDec + 1 + queuedBlock.length), ...nl.slice(iDec + 1 + queuedBlock.length + entryBlock.length)].join('\n');
check('removing the inserted lines gives back the original file byte for byte', stripped === orig);
check('the newest entry comes first, right under the heading', nl[nl.indexOf('## Decisions, newest first') + 1].startsWith('- **Taken by Mustafa, 2026-11-11, 22:38:50 UTC'));
if (!ok) { console.error('decisions entries: FAIL (nothing written)'); process.exit(1); }
const target = mode === 'write' ? path : join(REPO, '.gate/foundation-restructure/DECISIONS.dry.md');
writeFileSync(target, text);
console.log(`decisions entries: PASS; ${ENTRIES.length} entries and 1 Queued item written to ${target.replace(REPO + '/', '')} (+${entryBlock.length + queuedBlock.length} lines)`);
