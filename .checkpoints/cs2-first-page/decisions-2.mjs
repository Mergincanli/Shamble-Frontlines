// The last docs pass of change-set 2 (the first page): writes Mustafa's chat message of 2026-11-19, 13:36:37 UTC into
// docs/DECISIONS.md as the newest entry. His words are never retyped: they come from the file saved by script from the
// session's transcript (words.mjs), with its byte count and sha256. Run from the repository root:
//   node .checkpoints/cs2-first-page/decisions-2.mjs [--check]
// BEFORE is the commit whose docs/DECISIONS.md this entry is added to: every other byte of that file must stay.
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

const DEC = 'docs/DECISIONS.md';
const NOTES = '.checkpoints/cs2-first-page/';
const FILE = 'mustafa-2026-11-19-1200-four-things.md';
const REPORT = 'docs/reports/2026-11-19-cs2-first-page.md';
const BEFORE = '5172dfba8a423869f2e0b7b67c5384f738db7b31';
const SESSION = 'the build chat, session_019YNMUV9LFhASseJ9EP12kL, on his personal account';
const KEY = '2026-11-19, 13:36:37 UTC: his answer to SIZE-STOP-2 in his own words';
const sha = (s) => createHash('sha256').update(s).digest('hex');
const quote = (s) => s.split('\n').map((l) => (l === '' ? '>' : `> ${l}`)).join('\n');
const unquote = (s) => s.split('\n').map((l) => (l === '>' ? '' : l.replace(/^> /, ''))).join('\n');
const wrap = (text, rest = '  ') => {
  const out = [];
  let line = '';
  let empty = true;
  for (const w of text.split(' ')) {
    if (!empty && line.length + 1 + w.length > 120) {
      out.push(line);
      line = rest + w;
    } else {
      line += (empty ? '' : ' ') + w;
      empty = false;
    }
  }
  out.push(line);
  return out.join('\n');
};

const note = readFileSync(NOTES + FILE, 'utf8');
const cut = note.indexOf('\n---\n');
if (cut < 0) throw new Error('no --- in ' + FILE);
const words = note.slice(cut + 5).replace(/\n$/, '');
const bytes = Buffer.byteLength(words);

const lead = wrap(
  `- **Taken by Mustafa, ${KEY} ((a): the stop line of change-set 2 moves to 4,700 added lines, and it is the last move of that line), the night lost to an approval prompt accepted as the report writes it down, and four things for the change-sets after this one: SLICE-FEEL (aiming down sights and the sprint-to-fire delay are game rules of Slice 1; both are priced and placed in change-set 4's plan), PROJ-DRAW ((a): in change-set 9 lane 1 draws grenades and arrows in flight plainly, and they pass to lane 2 when change-set 9 is merged), lane 2's first list of needs on the "Lane requests" issue #59 (answered row by row in change-set 4's plan; its wishes W1 and W4 are now needs), and his browser check of the first page (his own browser and once Firefox; the game has to work in Chrome, the Chromium-based browsers and Firefox; Safari and Mac come later)** (${SESSION}; his label: "My words of 2026-11-19, 12:00 (Amsterdam time)"; extracted by script from the session's transcript, message d436904b-c45e-4767-95c0-efc1882ec346, whole). Its first paragraph is about the chat's compaction: the resume card it names had by then been folded into \`${REPORT}\` and deleted (commit 29ab052), and stays readable at commit 929b146. His one letter of 11:07:48 UTC, the entry below, was read as (a) on SIZE-STOP-2; these words give that answer themselves. When they came the change-set stood at 4,477 added lines, with both review rounds and the verifier done. The question PROJ-DRAW as lane 2 put it, with its options, is not in this lane's records; the comment he names on #59 (issue comment 6994614989; a comment there is never his word) asks under its gap 4 who draws projectiles in flight after change-set 9. What this lane did with point 4 stands in \`${REPORT}\`. Copied by script (${bytes} bytes, sha256 ${sha(words)}):`,
);
const block = `${lead}\n${quote(words)}\n`;
const TOP = '## Decisions, newest first\n';

if (!process.argv.includes('--check')) {
  const t = readFileSync(DEC, 'utf8');
  if (t.includes(KEY)) throw new Error('already inserted');
  if (t.split(TOP).length !== 2) throw new Error('the list heading must stand once');
  writeFileSync(DEC, t.replace(TOP, TOP + block));
}

// checks
const t = readFileSync(DEC, 'utf8');
const before = execFileSync('git', ['show', `${BEFORE}:${DEC}`], { maxBuffer: 1 << 26 }).toString('utf8');
const i = t.indexOf(lead);
const got = i < 0 ? null : unquote(t.slice(i + lead.length + 1, i + lead.length + 1 + quote(words).length));
const ok = {
  'in DECISIONS, unquoted, equal to the saved words': got === words,
  "the notes file's own count and sha256 equal the words": note.includes(`whole (${bytes} bytes, sha256 ${sha(words)})`),
  'the lead holds its key': lead.replace(/\n {2}/g, ' ').includes(KEY),
  'the entry stands at the top of the list': t.includes(TOP + lead),
  [`every other byte of the file at ${BEFORE.slice(0, 7)} is unchanged`]: t.replace(block, '') === before,
  'no lead line over 120 characters': lead.split('\n').every((l) => l.length <= 120),
  'no carriage return': !t.includes('\r'),
};
console.log(`${FILE}: ${bytes} bytes, sha256 ${sha(words)}`);
for (const [k, v] of Object.entries(ok)) console.log(`${v ? 'yes' : 'NO '}  ${k}`);
process.exit(Object.values(ok).every(Boolean) ? 0 : 1);
