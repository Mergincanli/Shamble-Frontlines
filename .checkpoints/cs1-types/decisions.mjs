// Change-set 1's docs pass: writes Mustafa's message of 2026-11-18, 11:08:25 UTC (his go on the merge of #57, his
// answer OLD-LINES) into docs/DECISIONS.md, with one dated pointer in the entry that holds his answer RECORD-FILES.
// His words are never retyped: they come from the file saved by script from the transcript, and are checked byte for
// byte against the message of merge commit 22c6afa. Run from the repository root:
//   node .checkpoints/cs1-types/decisions.mjs [--check]
import { readFileSync, writeFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
const DEC = 'docs/DECISIONS.md';
const FILE = '.checkpoints/cs1-types/mustafa-2026-11-18-1131-merge-57.md';
const MERGE = '22c6afa0341c36c40479f671736a569adfdb2663';
const sha = (s) => createHash('sha256').update(s).digest('hex');
const words = (() => { const t = readFileSync(FILE, 'utf8'); const i = t.indexOf('\n---\n'); if (i < 0) throw new Error('no --- in ' + FILE); return t.slice(i + 5).replace(/\n$/, ''); })();
const quote = (s) => s.split('\n').map((l) => (l === '' ? '>' : `> ${l}`)).join('\n');
const unquote = (s) => s.split('\n').map((l) => (l === '>' ? '' : l.replace(/^> /, ''))).join('\n');
const wrap = (text, first, rest) => { const out = []; let line = first; let empty = true;
  for (const w of text.split(' ')) { if (!empty && line.length + 1 + w.length > 120) { out.push(line); line = rest + w; } else { line += (empty ? '' : ' ') + w; empty = false; } }
  out.push(line); return out.join('\n'); };
const SESSION = 'the build chat, session_019YNMUV9LFhASseJ9EP12kL, on his personal account';
const head = wrap(`- **Taken by Mustafa, 2026-11-18, 11:08:25 UTC: his go on the merge of #57 (Part 15 of the build plan, "The second lane", with his six answers), what he accepts with it, his answer OLD-LINES, and what comes after** (${SESSION}; his label: "My words of 2026-11-18, 11:31 (Amsterdam time)"; extracted by script from the session's transcript, message 16653b82-ec18-4cb3-acb1-73f6a294381e, whole). The same words stand byte for byte in the message of merge commit ${MERGE} (2026-11-18, 11:09:31 UTC; parents d7dad97 and f26ac5b). Before the merge the session read the head again (f26ac5b85396f4b280ba21e57f2a1ea177382d80) and \`gate\`, \`smoke\` and \`toolchain\` on it: green. The question OLD-LINES stands in \`docs/reports/2026-11-17-part-15-second-lane.md\`, "Decisions needed"; his answer adds to his answer RECORD-FILES (the entry of 2026-11-17, 22:10:34 UTC, below; a dated pointer marks it). Copied by script and checked byte for byte against that merge commit's message:`, '', '  ');
const POINTER_START = '- **Taken by Mustafa, 2026-11-17, 22:10:34 UTC: his answers to the six questions of the build plan\'s Part 15';
const POINTER_QUOTE = '(a), for lines about merged pull requests.';
const POINTER = `Pointer, 2026-11-18, change-set 1 (not his words): his answer OLD-LINES of 2026-11-18 (the entry of 11:08:25 UTC, above) adds to its answer RECORD-FILES: "${POINTER_QUOTE}"`;
const TOP = '## Decisions, newest first\n';
const COPY = ' Copied by script and checked byte for byte against';
if (!process.argv.includes('--check')) {
  let t = readFileSync(DEC, 'utf8');
  if (t.includes('2026-11-18, 11:08:25 UTC: his go on the merge of #57')) throw new Error('already inserted');
  if (t.split(TOP).length !== 2) throw new Error('the list heading must stand once');
  if (t.split(POINTER_START).length !== 2) throw new Error('the RECORD-FILES entry is not found once');
  const a = t.indexOf(POINTER_START); const b = t.indexOf('\n>', a);
  const old = t.slice(a, b).split('\n').map((l) => l.trim()).join(' ');
  if (old.split(COPY).length !== 2) throw new Error('no copy sentence in the RECORD-FILES entry');
  t = t.slice(0, a) + wrap(old.replace(COPY, ' ' + POINTER + COPY), '', '  ') + t.slice(b);
  t = t.replace(TOP, TOP + `${head}\n${quote(words)}\n`);
  writeFileSync(DEC, t);
}
// checks
const t = readFileSync(DEC, 'utf8');
const i = t.indexOf(head);
const got = i < 0 ? null : unquote(t.slice(i + head.length + 1, i + head.length + 1 + quote(words).length));
const merge = execFileSync('git', ['log', '-1', '--format=%B', MERGE]).toString('utf8');
const before = execFileSync('git', ['show', 'origin/main:' + DEC], { maxBuffer: 1 << 26 }).toString('utf8');
// Every line of main's file is still there, in order, apart from the one entry header that gained the pointer.
const a0 = before.indexOf(POINTER_START); const b0 = before.indexOf('\n>', a0);
const a1 = t.indexOf(POINTER_START); const b1 = t.indexOf('\n>', a1);
const flat = (s) => s.split('\n').map((l) => l.trim()).join(' ');
const rest = t.slice(0, a1).replace(`${head}\n${quote(words)}\n`, '') + '<entry header>' + t.slice(b1);
const restBefore = before.slice(0, a0) + '<entry header>' + before.slice(b0);
const headerOk = a0 >= 0 && a1 >= 0 && flat(t.slice(a1, b1)) === flat(before.slice(a0, b0)).replace(COPY, ' ' + POINTER + COPY);
const long = t.slice(i, i + head.length).split('\n').concat(t.slice(a1, b1).split('\n')).filter((l) => l.length > 120).length;
console.log('his words:', Buffer.byteLength(words), 'bytes, sha256', sha(words).slice(0, 16));
console.log('in DECISIONS, unquoted:', got === words ? 'equal' : 'DIFFERENT');
console.log('in the message of merge commit ' + MERGE.slice(0, 7) + ':', merge.includes(words) ? 'equal (a whole part of it)' : 'NOT FOUND');
console.log('the pointer\'s quote is in his words:', words.includes(POINTER_QUOTE) ? 'yes' : 'NO');
console.log('the RECORD-FILES entry\'s header: main\'s text plus the pointer sentence:', headerOk ? 'yes' : 'NO');
console.log('every other byte of main\'s file unchanged:', rest === restBefore ? 'yes' : 'NO');
console.log('header lines over 120:', long, '| CR:', t.includes('\r') ? 'YES' : 'none');
process.exit(got === words && merge.includes(words) && words.includes(POINTER_QUOTE) && headerOk && rest === restBefore && !long && !t.includes('\r') ? 0 : 1);
