// The docs pass of the CI-WASM pull request: writes docs/reports/2026-11-19-cs2-browser-check.md, the record of
// Mustafa's browser check of the first page (#61). His four chat messages are never retyped: they come from the files
// saved by script from the session's transcript (each names its uuid, byte count and sha256), and the reading comes
// from browser-reports-reading.md beside them. Run from the repository root:
//   node .checkpoints/ci-wasm/browser-check-record.mjs [--check] [--out <file>]
// Without --check it writes the file; with --check it only compares the file on disk with what it would write.
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

const NOTES = '.checkpoints/ci-wasm/';
const at = process.argv.indexOf('--out');
const OUT = at === -1 ? 'docs/reports/2026-11-19-cs2-browser-check.md' : process.argv[at + 1];
const MESSAGES = [
  ['mustafa-2026-11-19T144625.md', 'Brave: four reports and the version line'],
  ['mustafa-2026-11-19T145419.md', 'Firefox: the page opened as a file (a screenshot), and the served page\'s report'],
  ['mustafa-2026-11-19T150250.md', 'Brave: two re-checks of the hidden tab'],
  ['mustafa-2026-11-19T150929.md', 'Firefox: the report of `?backend=webgl`'],
];
const sha = (s) => createHash('sha256').update(s).digest('hex');
const quote = (s) => s.split('\n').map((l) => (l === '' ? '>' : `> ${l}`)).join('\n');
const unquote = (s) => s.split('\n').map((l) => (l === '>' ? '' : l.replace(/^> /, ''))).join('\n');

/** One saved message: its words, and the time and uuid its own first line gives. */
function saved(file) {
  const note = readFileSync(NOTES + file, 'utf8');
  const cut = note.indexOf('\n---\n');
  if (cut < 0) throw new Error(`no --- in ${file}`);
  const head = note.slice(0, cut);
  const words = note.slice(cut + 5).replace(/\n$/, '');
  const time = head.match(/chat message of (\S+Z)/)?.[1];
  const uuid = head.match(/uuid ([0-9a-f-]{36})/)?.[1];
  const said = head.match(/\((\d+) bytes, sha256 ([0-9a-f]{64})\)/);
  if (!time || !uuid || !said) throw new Error(`${file}: its first line names no time, uuid or sha256`);
  if (Number(said[1]) !== Buffer.byteLength(words) || said[2] !== sha(words)) throw new Error(`${file}: the words differ from its own count or sha256`);
  return { file, words, time, uuid, bytes: Buffer.byteLength(words), sha256: sha(words), image: /other parts: image/.test(head) };
}

// The reading: the note's body from its first "## " heading on, with the three places that name the notes folder
// rewritten for a file that outlives it.
const reading = readFileSync(NOTES + 'browser-reports-reading.md', 'utf8');
const from = reading.indexOf('\n## What came, in order\n');
if (from < 0) throw new Error('the reading has no "What came, in order"');
const swaps = [
  ['| His message (UTC) | File | What it holds |', '| His message (UTC) | Where | What it holds |'],
  ['| 14:28:25 | `mustafa-2026-11-19-1516-go-61.md` |', '| 14:28:25 | `docs/DECISIONS.md` |'],
  ['| 14:31:30 | `mustafa-2026-11-19T143130.md` |', '| 14:31:30 | `docs/DECISIONS.md` |'],
  ['| 14:46:25 | `mustafa-2026-11-19T144625.md` |', '| 14:46:25 | Appendix A, message 1 |'],
  [
    '| 14:54:19 | `mustafa-2026-11-19T145419.md` | Firefox: a screenshot of the page opened as a file (the picture is not in the notes; a copy lies in `.gate/cs2-first-page/merge/`), and the served page\'s report. |',
    '| 14:54:19 | Appendix A, message 2 | Firefox: a screenshot of the page opened as a file (the picture is not in the repository: it shows a folder path of his computer), and the served page\'s report. |',
  ],
  ['| 15:02:50 | `mustafa-2026-11-19T150250.md` |', '| 15:02:50 | Appendix A, message 3 |'],
  ['| 15:09:29 | `mustafa-2026-11-19T150929.md` |', '| 15:09:29 | Appendix A, message 4 |'],
  ['## Observations for the follow-ups (ids are this note\'s; none is an error count above 0)', '## Observations for the follow-ups (the ids are this record\'s; none is an error count above 0)'],
];
let body = reading.slice(from + 1).replace(/\n$/, '');
for (const [a, b] of swaps) {
  if (body.split(a).length !== 2) throw new Error(`the reading must hold once: ${a.slice(0, 60)}`);
  body = body.replace(a, b);
}
if (body.includes('.checkpoints/') || body.includes('this note')) throw new Error('the reading still names the notes');

const messages = MESSAGES.map(([file, what]) => ({ ...saved(file), what }));
const appendix = messages
  .map(
    (m, i) =>
      `### Message ${i + 1}: ${m.what}\n` +
      `_Chat message of ${m.time} (uuid ${m.uuid}), its text whole` +
      `${m.image ? ';\nit also carried one picture, which is not in the repository' : ''}.\nCopied by script ` +
      `(${m.bytes} bytes, sha256 ${m.sha256}):_\n${quote(m.words)}\n`,
  )
  .join('\n');

const text = `# Mustafa's browser check of the first page (pull request #61): his reports, word for word, and their reading

_What this is: the record his go on #61 asks for ("My browser reports are recorded with the next docs pass, not by a
push to this head.": \`docs/DECISIONS.md\`, the entry of 2026-11-19, 14:28:25 UTC). He ran the zip of change-set 2
(\`shamble-v2-02-first-page.zip\`, 245,953 bytes, sha256
c49d8a7f9b599bc5d9855fca0c782650a796b0f54ec4cf284badb5f2a920c5f4, built from commit 7d8e2de2e) in his own browser
(Brave) and in Firefox on his Windows computer, and pasted what the page's "Copy report" button gave him into the build
chat (session_019YNMUV9LFhASseJ9EP12kL). Appendix A holds those four messages whole, copied by script from the
session's transcript and never retyped. The reading above it is the build chat's, written at 15:12 UTC the same day;
the merge (f5030a3, 15:10:54 UTC) followed his last report. The change-set's own report:
\`docs/reports/2026-11-19-cs2-first-page.md\`._

_In short: errors 0 in all eight reports; WebGPU starts in both browsers on his machine, and the forced WebGL 2
fallback works in both; the pause on a hidden tab works in both (0 ticks while hidden). One thing is not explained
(BR-GAP below), and three things were not shown (NOT-SHOWN below). The follow-ups stand in
\`docs/STATUS_ARCHIVE.md\`, "Follow-ups", "From change-set 2", under these ids._

${body}

## Appendix A. His four messages, word for word
_Each block below is one chat message's text, quoted line for line ("> " before each line). The first lines of his
third message quote the build chat's re-check steps back, each with his "Done:"._

${appendix}`;

if (!process.argv.includes('--check')) writeFileSync(OUT, text);
const disk = readFileSync(OUT, 'utf8');
const ok = {
  'the file on disk is what the script writes': disk === text,
  'no line over 120 characters outside the tables and his quoted words': text
    .split('\n')
    .every((l) => l.length <= 120 || l.startsWith('|') || l.startsWith('>')),
  'no carriage return': !text.includes('\r'),
};
for (const m of messages) {
  const lead = `(uuid ${m.uuid}), its text whole`;
  const i = disk.indexOf(lead);
  const start = disk.indexOf(':_\n', i) + 3;
  ok[`message of ${m.time}: quoted whole, ${m.bytes} bytes, sha256 ${m.sha256.slice(0, 12)}`] =
    i >= 0 && unquote(disk.slice(start, start + quote(m.words).length)) === m.words;
}
for (const [k, v] of Object.entries(ok)) console.log(`${v ? 'yes' : 'NO '}  ${k}`);
process.exit(Object.values(ok).every(Boolean) ? 0 : 1);
