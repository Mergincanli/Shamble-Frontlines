// Verifier: rebuilds the Part 15 report from its sources and compares it whole; checks the DECISIONS quote blocks end
// where his words end. Run from the repository root. Prints only lengths, hashes and verdicts.
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
const sha = (s) => createHash('sha256').update(s).digest('hex');
const P = '.checkpoints/part-15-to-main/';
const words = (path) => { const t = readFileSync(path, 'utf8'); const i = t.indexOf('\n---\n'); return t.slice(i + 5).replace(/\n$/, ''); };
const part15 = execFileSync('git', ['show', 'ea851a8:.checkpoints/part-15/part-15-the-second-lane.md']).toString('utf8');
const notesCopy = readFileSync(P + 'part-15-the-second-lane.md', 'utf8');
console.log('notes copy of Part 15 == ea851a8:', notesCopy === part15);
const rep = readFileSync('docs/reports/2026-11-17-build-plan-slice-1-part-15.md', 'utf8');
const w1 = words(P + 'mustafa-2026-11-17-2306.md'), w2 = words(P + 'mustafa-2026-11-17-2357-merge-56.md');
const block = (id, title, body) => `## ${id}. ${title}\n\`\`\`\`text\n${body}\n\`\`\`\`\nsha256 of the block: \`${sha(body)}\` (${Buffer.byteLength(body)} bytes).\n`;
const MARK = '## Part 15 as he read it (revision 2, commit ea851a8, copied by script)\n\n';
const iMark = rep.indexOf(MARK);
console.log('mark count:', rep.split(MARK).length - 1, 'at line', rep.slice(0, iMark).split('\n').length);
const top = rep.slice(0, iMark + MARK.length);
const tail = block('P1', "His message of 2026-11-17, 22:10:34 UTC (his label: 23:06): his answers to Part 15's six questions", w1) + '\n' +
  block('P2', 'His message of 2026-11-17, 22:59:16 UTC (his label: 23:57): his go on the merge of #56', w2);
const rebuilt = top + part15 + '\n---\n\n' + tail;
console.log('report == top + ea851a8 + --- + P1 + P2 (whole file):', rebuilt === rep, 'len', rep.length, rebuilt.length);
const ha = readFileSync(P + 'his-answers.md', 'utf8');
console.log('report top == notes his-answers.md:', top === ha, '(top ends with mark:', top.endsWith(MARK), ')');
console.log('w1', Buffer.byteLength(w1), sha(w1)); console.log('w2', Buffer.byteLength(w2), sha(w2));
// DECISIONS: the quote block after each new header ends where his words end
const dec = readFileSync('docs/DECISIONS.md', 'utf8');
const quote = (s) => s.split('\n').map((l) => (l === '' ? '>' : `> ${l}`)).join('\n');
for (const [k, w] of [['P1', w1], ['P2', w2]]) {
  const q = quote(w); const n = dec.split(q).length - 1; const i = dec.indexOf(q);
  const after = dec.slice(i + q.length, i + q.length + 40).split('\n').slice(0, 3);
  console.log(k, 'quote block occurrences:', n, '| next lines after block:', JSON.stringify(after.map((l) => l.slice(0, 30))));
}
