// The docs pass of the CI-WASM pull request: writes Mustafa's chat messages since his go on #61 into docs/DECISIONS.md,
// newest first, and one dated pointer under the entry of 2026-11-19, 13:36:37 UTC. His words are never retyped: they
// come from the files saved by script from the session's transcript (words.mjs), each with its byte count and sha256.
// The file is always rebuilt from BEFORE (main's copy), so a second run writes the same bytes. From the repository root:
//   node .checkpoints/ci-wasm/decisions-3.mjs [--check] [--out <file>]
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
// The question VISION-NOTE and its options, copied by script from the chat's message of 14:29:51 UTC (never retyped).
import { OPTION_A, OPTION_B, QUESTION } from './vision-note.mjs';

const DEC = 'docs/DECISIONS.md';
const at = process.argv.indexOf('--out');
const OUT = at === -1 ? DEC : process.argv[at + 1];
const NOTES = '.checkpoints/ci-wasm/';
const BEFORE = 'f5030a33185b1ea1b15329174e82823f0215f993';
const SESSION = 'the build chat, session_019YNMUV9LFhASseJ9EP12kL, on his personal account';
const RECORD = 'docs/reports/2026-11-19-cs2-browser-check.md';
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
function saved(file) {
  const note = readFileSync(NOTES + file, 'utf8');
  const cut = note.indexOf('\n---\n');
  if (cut < 0) throw new Error(`no --- in ${file}`);
  const words = note.slice(cut + 5).replace(/\n$/, '');
  const said = note.slice(0, cut).match(/\((\d+) bytes, sha256 ([0-9a-f]{64})\)/);
  if (!said || Number(said[1]) !== Buffer.byteLength(words) || said[2] !== sha(words)) throw new Error(`${file}: the words differ from its own count or sha256`);
  return { words, bytes: Buffer.byteLength(words), sha256: sha(words) };
}
const by = (m) => `Copied by script (${m.bytes} bytes, sha256 ${m.sha256}):`;

// Newest first. `file` undefined: a pointer, with no words of his quoted under it.
const ENTRIES = [
  {
    key: '2026-11-19, 16:08:04 UTC: two notes for the work after #61',
    file: 'mustafa-2026-11-19-1638-two-notes.md',
    lead: (m) =>
      `- **Taken by Mustafa, 2026-11-19, 16:08:04 UTC: two notes for the work after #61: the docs pass that records the merge of #61 stays with the CI-WASM pull request, whose report keeps the workflow text apart from the docs; and for change-set 3's browser check the README asks him to keep the browser's window in front and uncovered during the first run and has one step with the window covered on purpose, so that BR-GAP is read and not guessed** (${SESSION}; his label: "My words of 2026-11-19, 16:38 (Amsterdam time)"; extracted by script from the session's transcript, message e02d3ad1-8f0c-4b6a-a30a-5b267f681630, whole: its first paragraph is his instruction to the chat after a compaction). BR-GAP is the one unexplained reading of his browser check: \`${RECORD}\`. ${by(m)}`,
  },
  {
    key: 'his browser check of the first page (#61), 2026-11-19, 14:46:25 to 15:09:29 UTC',
    lead: () =>
      `- _A pointer, not a decision: **his browser check of the first page (#61), 2026-11-19, 14:46:25 to 15:09:29 UTC.** Four chat messages (14:46:25, 14:54:19, 15:02:50 and 15:09:29 UTC) with eight "Copy report" texts, from his own browser (Brave 1.100.58) and from Firefox 160 on his Windows computer, and one picture. His go below says: "My browser reports are recorded with the next docs pass, not by a push to this head." They stand whole, copied by script, in \`${RECORD}\`, Appendix A, with the build chat's reading of them against the zip's README. His first Brave report read "last pause: none yet" where the README expects a recorded pause; by his go's condition ("If your reading of them shows a line that is not as your README expects, stop and tell me before you merge.") the chat told him at 14:50:49 UTC (its message 7e4ded94-23e4-459c-ad2a-6d1fdcb3d7d8 in the session's transcript), before any merge, and his re-check of 15:02:50 UTC showed the pause (45.0 s hidden, 0 ticks while hidden)._`,
  },
  {
    key: '2026-11-19, 14:31:30 UTC: his answer to the question VISION-NOTE',
    file: 'mustafa-2026-11-19T143130.md',
    lead: (m) =>
      `- **Taken by Mustafa, 2026-11-19, 14:31:30 UTC: his answer to the question VISION-NOTE ((a): the dated note for \`docs/VISION_V2.md\` is right as the chat wrote it)** (${SESSION}; extracted by script from the session's transcript, message 94cf4e6c-e46b-48a4-b48e-3270856870be, whole). The question, as the chat put it to him at 14:29:51 UTC under the note's exact text (the chat's message 699736ec-f365-4458-aceb-592bba842d59, copied by script): "${QUESTION}", with the options "${OPTION_A}" and "${OPTION_B}" The text it showed is the dated note now in \`docs/VISION_V2.md\` §4 and the sentence added to that file's "Updated on" lines, both written by this docs pass from that message, byte for byte. His second sentence asks for the instructions again; the chat answered with the browser check's steps at 14:31:56 UTC. ${by(m)}`,
  },
  {
    key: '2026-11-19, 14:28:25 UTC: his go on the merge of pull request #61',
    file: 'mustafa-2026-11-19-1516-go-61.md',
    lead: (m) =>
      `- **Taken by Mustafa, 2026-11-19, 14:28:25 UTC: his go on the merge of pull request #61 (change-set 2, the first page) into main by a merge commit at head 4cf30ce303b16e96b3ca5d5c0b20b928a4db6b86, what he accepts with that merge, his answer BROWSER-SET ((a), order only: Safari stays a target of v2; Chrome, the Chromium-based browsers and Firefox come first; Safari and Mac come later), and what comes after the merge (the CI-WASM pull request, its exact text to him first, with change-set 3's plan beside it; with change-set 3, one line in \`.gitignore\` for \`__pycache__/\`)** (${SESSION}; his label: "My words of 2026-11-19, 15:16 (Amsterdam time)"; extracted by script from the session's transcript, message 65801ff7-6e9d-4343-b384-16a67118bc87, whole; the same bytes stand in the message of the merge commit f5030a3). When these words came, no browser report was in the chat; the merge waited for his reports (the pointer above) and was made at 15:10:54 UTC, at that head, with \`gate\`, \`smoke\` and \`toolchain\` green on it (read at 15:10 UTC). The dated note he asks for stands in \`docs/VISION_V2.md\` §4 (his answer VISION-NOTE, the entry above). ${by(m)}`,
  },
];
// The dated pointer (REC-OLD: a line that records what he decided is never reworded). It goes under the quoted words
// of the entry of 13:36:37 UTC, that is, right before the entry of 11:07:48 UTC.
const ANCHOR = '- **Taken by Mustafa, 2026-11-19, 11:07:48 UTC: his answer to the question SIZE-STOP-2 of change-set 2';
const POINTER =
  wrap(
    `  _Dated pointer, 2026-11-19 (the docs pass after #61's merge; the advisor agent's wording point on head 4cf30ce): where the header of this entry says the lost night is "accepted as the report writes it down", his words above are "is accepted as you wrote it down, with your lesson."_`,
  ) + '\n';

const blocks = ENTRIES.map((e) => {
  const m = e.file === undefined ? undefined : saved(e.file);
  const lead = wrap(e.lead(m));
  return { ...e, m, lead, block: m === undefined ? `${lead}\n` : `${lead}\n${quote(m.words)}\n` };
});
const TOP = '## Decisions, newest first\n';
const before = execFileSync('git', ['show', `${BEFORE}:${DEC}`], { maxBuffer: 1 << 26 }).toString('utf8');
if (before.split(TOP).length !== 2) throw new Error('the list heading must stand once');
if (before.split(ANCHOR).length !== 2) throw new Error('the anchor entry must stand once');
const text = before.replace(TOP, TOP + blocks.map((b) => b.block).join('')).replace(ANCHOR, POINTER + ANCHOR);

if (!process.argv.includes('--check')) writeFileSync(OUT, text);
const t = readFileSync(OUT, 'utf8');
const ok = {
  'the file on disk is what the script writes': t === text,
  [`with the new blocks and the pointer taken out, every byte equals the file at ${BEFORE.slice(0, 7)}`]:
    blocks.reduce((s, b) => s.replace(b.block, ''), t).replace(POINTER, '') === before,
  'the newest entry stands at the top of the list': t.includes(TOP + blocks[0].lead),
  'no new line over 120 characters outside his quoted words': [...blocks.map((b) => b.lead), POINTER].every((s) => s.split('\n').every((l) => l.length <= 120)),
  'no carriage return': !t.includes('\r'),
};
for (const b of blocks) {
  ok[`the lead holds its key: ${b.key.slice(0, 60)}`] = b.lead.replace(/\n {2}/g, ' ').includes(b.key);
  if (b.m === undefined) continue;
  const i = t.indexOf(b.lead);
  ok[`${b.file}: in DECISIONS, unquoted, equal to the saved words (${b.m.bytes} bytes, sha256 ${b.m.sha256.slice(0, 12)})`] =
    i >= 0 && unquote(t.slice(i + b.lead.length + 1, i + b.lead.length + 1 + quote(b.m.words).length)) === b.m.words;
}
for (const [k, v] of Object.entries(ok)) console.log(`${v ? 'yes' : 'NO '}  ${k}`);
process.exit(Object.values(ok).every(Boolean) ? 0 : 1);
