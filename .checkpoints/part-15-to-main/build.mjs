// Part 15 to main: builds the Part 15 report (his answers, Part 15's text from commit ea851a8, his two messages) and
// writes his two messages into docs/DECISIONS.md with two dated pointers. His words are never retyped. Run from the
// repository root:   node .checkpoints/part-15-to-main/build.mjs [--check]
import { readFileSync, writeFileSync, existsSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
const P = '.checkpoints/part-15-to-main/';
const DEC = 'docs/DECISIONS.md', REP = 'docs/reports/2026-11-17-build-plan-slice-1-part-15.md';
const sha = (s) => createHash('sha256').update(s).digest('hex');
const words = (path) => { const t = readFileSync(path, 'utf8'); const i = t.indexOf('\n---\n'); if (i < 0) throw new Error('no --- in ' + path); return t.slice(i + 5).replace(/\n$/, ''); };
const quote = (s) => s.split('\n').map((l) => (l === '' ? '>' : `> ${l}`)).join('\n');
const unquote = (s) => s.split('\n').map((l) => (l === '>' ? '' : l.replace(/^> /, ''))).join('\n');
const wrap = (text, first, rest) => { const out = []; let line = first; let empty = true;
  for (const w of text.split(' ')) { if (!empty && line.length + 1 + w.length > 120) { out.push(line); line = rest + w; } else { line += (empty ? '' : ' ') + w; empty = false; } }
  out.push(line); return out.join('\n'); };
const part15 = execFileSync('git', ['show', 'ea851a8:.checkpoints/part-15/part-15-the-second-lane.md'], { maxBuffer: 1 << 26 }).toString('utf8');
const SESSION = 'the build chat, session_019YNMUV9LFhASseJ9EP12kL, on his personal account';
const TAG = 'Pointer, 2026-11-17, Part 15 to main (not his words):';
const msgs = [ // newest first
  { key: 'go-merge-56', file: P + 'mustafa-2026-11-17-2357-merge-56.md', id: 'P2',
    head: `- **Taken by Mustafa, 2026-11-17, 22:59:16 UTC: his go on the merge of #56 (change-set 0 of the build plan), what he accepts with it, and what comes after** (${SESSION}; his label: "My words of 2026-11-17, 23:57 (Amsterdam time)"; extracted by script from the session's transcript, message 987dac5c-6911-41b9-9d8e-2f803893370a, whole). The same words stand byte for byte in the message of merge commit d7dad976f1ea7170d597b51e3861f1ee74f980e4 (2026-11-17, 23:00:12 UTC; parents c94c81b and cd61b28). Before the merge the session read the head again (cd61b28350c4407d2f19b1b3603ea4beca8b4e30) and \`gate\`, \`smoke\` and \`toolchain\` on it: green. Copied by script and checked byte for byte against \`${REP}\`, P2:` },
  { key: 'answers-part-15', file: P + 'mustafa-2026-11-17-2306.md', id: 'P1',
    head: `- **Taken by Mustafa, 2026-11-17, 22:10:34 UTC: his answers to the six questions of the build plan's Part 15, "The second lane" (LANE-SPLIT, RECORD-FILES, VFX-ROUTE, VFX-FIX, VFX-MOMENTS, GUN-FILES: (a) each, with additions); the stop line of change-set 0; pull request #56** (${SESSION}; his label: "My words of 2026-11-17, 23:06 (Amsterdam time)"; extracted by script from the session's transcript, message c4edbe94-57f5-451a-8778-59897cfd0cdf, whole). The questions stand in Part 15 as he read it: \`${REP}\`, "Decisions needed". Its answer LANE-SPLIT changes "one pull request at a time" in his words of 2026-11-14 (the entry of 13:58:05 UTC, below) to one at a time per lane. Its answer VFX-ROUTE is also his word on a seventh package beside the six that ADR 0002 names (a dated pointer marks that list). Copied by script and checked byte for byte against \`${REP}\`, P1:` },
];
for (const m of msgs) m.head = wrap(m.head, '', '  ');
const pointers = [ // a sentence added to an older entry's header, before its "Copied by script" sentence
  { start: '- **Taken by Mustafa, 2026-11-14, 13:58:05 UTC: the game is built now', q: ['one pull request at a time per lane, in place of the plan\'s one at a time'],
    text: `${TAG} its "one pull request at a time" is changed by his answer LANE-SPLIT of 2026-11-17 (the entry of 22:10:34 UTC, above): "one pull request at a time per lane, in place of the plan's one at a time".` },
  { start: '- **Taken by Mustafa, 2026-11-17, 19:10:44 UTC: his go on the build plan', q: [],
    text: `${TAG} the plan part its CHANGE 2 asks for, and the answer its CHANGE 4 asks for, are Part 15, on main with his answers of 2026-11-17 (the entry of 22:10:34 UTC, above): \`${REP}\`.` },
];
const TOP = '## Decisions, newest first\n';
if (!process.argv.includes('--check')) {
  let t = readFileSync(DEC, 'utf8');
  if (t.includes('2026-11-17, 22:59:16 UTC: his go on the merge of #56')) throw new Error('already inserted');
  if (t.split(TOP).length !== 2) throw new Error('the list heading must stand once');
  for (const p of pointers) { // rebuild the header paragraph with the pointer sentence in it
    if (t.split(p.start).length !== 2) throw new Error('entry not found once: ' + p.start);
    const a = t.indexOf(p.start); const b = t.indexOf('\n>', a); const head = t.slice(a, b).split('\n').map((l) => l.trim()).join(' ');
    const C = ' Copied by script and checked byte for byte against'; if (head.split(C).length !== 2) throw new Error('no copy sentence in ' + p.start);
    t = t.slice(0, a) + wrap(head.replace(C, ' ' + p.text + C), '', '  ') + t.slice(b);
  }
  t = t.replace(TOP, TOP + msgs.map((m) => `${m.head}\n${quote(words(m.file))}\n`).join(''));
  writeFileSync(DEC, t);
  const block = (id, title, body) => `## ${id}. ${title}\n\`\`\`\`text\n${body}\n\`\`\`\`\nsha256 of the block: \`${sha(body)}\` (${Buffer.byteLength(body)} bytes).\n`;
  writeFileSync(REP, readFileSync(P + 'his-answers.md', 'utf8') + part15 + '\n---\n\n' +
    block('P1', 'His message of 2026-11-17, 22:10:34 UTC (his label: 23:06): his answers to Part 15\'s six questions', words(msgs[1].file)) + '\n' +
    block('P2', 'His message of 2026-11-17, 22:59:16 UTC (his label: 23:57): his go on the merge of #56', words(msgs[0].file)));
}
// checks
const t = readFileSync(DEC, 'utf8'); const r = existsSync(REP) ? readFileSync(REP, 'utf8') : ''; let ok = true;
for (const m of msgs) {
  const w = words(m.file); const i = t.indexOf(m.head);
  const got = i < 0 ? null : unquote(t.slice(i + m.head.length + 1, i + m.head.length + 1 + quote(w).length));
  const inRep = r.includes('````text\n' + w + '\n````\nsha256 of the block: `' + sha(w) + '`');
  console.log(m.key, Buffer.byteLength(w), 'bytes', sha(w).slice(0, 16), 'DECISIONS:', got === w ? 'equal' : 'DIFFERENT', '| report block:', inRep ? 'equal, sha256 right' : 'MISSING');
  if (got !== w || !inRep) ok = false;
}
const MARK = '## Part 15 as he read it (revision 2, commit ea851a8, copied by script)\n\n';
const a = r.indexOf(MARK); const copied = a < 0 ? '' : r.slice(a + MARK.length, a + MARK.length + part15.length);
console.log('Part 15 in the report equals commit ea851a8:', copied === part15 ? 'yes' : 'NO', '(' + part15.split('\n').length + ' lines, sha256 ' + sha(part15).slice(0, 16) + ')');
const np = pointers.filter((p) => t.includes(p.text.split(' ').slice(0, 9).join(' '))).length;
const long = t.split('\n').filter((l) => !l.startsWith('>') && l.length > 120 && /Part 15 to main|22:59:16 UTC|22:10:34 UTC/.test(l)).length;
console.log('pointers in DECISIONS:', np, 'of', pointers.length, '| header lines over 120:', long, '| CR:', (t + r).includes('\r') ? 'YES' : 'none');
process.exit(ok && copied === part15 && np === pointers.length && !long && !(t + r).includes('\r') ? 0 : 1);
