// Writes Mustafa's message of 2026-11-26, 18:16:35 UTC, to Builder V4 into docs/DECISIONS.md, from the chat's saved
// message file (.checkpoints/cs4-walk/mustafa-2026-11-26-take-in-76.md, copied by script from the chat's
// transcript): no word of his is typed here. The script of 4b-1's docs pass (decisions-4b1.mjs, in the notes at
// 0b0933a), with this one message. From the worktree's root:
//   node .checkpoints/cs4-walk/decisions-take-in-76.mjs write <commit at which the saved file is readable>
//   node .checkpoints/cs4-walk/decisions-take-in-76.mjs check
// `write` refuses a message that is already in the file. `check` reads the file back and compares every quoted line
// with the saved message, byte for byte, and the saved message with the byte count and sha256 its own header gives.
// The text in bold and the brackets after it are this chat's, written for the file.
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

const NOTES = '.checkpoints/cs4-walk/';
const DECISIONS = 'docs/DECISIONS.md';
const CHAT = 'the builder chat he calls Builder V4, lane 1 of the build, session_019YNMUVZR6ociHFWjByxkrq, a Claude Code chat';
// The file, and what he decided in it, in this chat's words (the bold line of the entry).
const ENTRIES = [
  ['mustafa-2026-11-26-take-in-76.md',
    'main moved under pull request #75 (to 2671595fc823750a83c58f5b5b122128eb500387, the Guard fix chat\'s #76), so it cannot merge at 2e282176: the chat takes main in once, with no rebase and no force push; in the four clashing docs files both sides are kept, main\'s text stays unchanged where a line belongs to another chat, and STATUS ends at 120 lines or fewer; the advisor agent reviews what the take-in changed; a report in three lines; his go follows gate and smoke green on the new head; SIGHT-FLAG (a): not in 4b, with one follow-up line so that Slice 2\'s plan adds the mark (which things carry it is still open in his rule: fences, and whether shots follow sight); DUR-1 (C): the table value is the rate for now, tuned by play in Testlines, a tuned value reaching the game through his Export; STATUS-LEN (a); WATCH-75: yes; the advisor agent\'s four conditions for 4b-2 stand, and 4b-2 starts right after the merge with no new word from him'],
];
// The Export's own text is not repeated in the register: his words before it are quoted, and the entry says where
// the Export is, whole.
const EXPORT = 'no-export-in-this-message';
const EXPORT_NOTE = ' The Export follows these lines in the same message and is not repeated here: it is saved whole by script as `.checkpoints/cs4b-content/testlines-export-30.json` (15,508 bytes, sha256 a9d47785ed8f70d895d10480d180c9bba5a67f7e955e0c460c0c1bef7718c4ab; step rate 30, no warning), with its manifest beside it.';

const sha = (text) => createHash('sha256').update(text).digest('hex');
function saved(file) {
  const all = readFileSync(NOTES + file, 'utf8');
  const cut = all.indexOf('\n---\n');
  const note = all.slice(0, cut);
  const text = all.slice(cut + 5).replace(/\n$/, '');
  const m = note.match(/(?:the user message|the message he sent while the chat was working \(stored as a queued command\)) of (\d{4}-\d{2}-\d{2})T(\d{2}:\d{2}:\d{2})\.\d+Z \(uuid ([0-9a-f-]{36})\), whole \((\d+) bytes, sha256 ([0-9a-f]{64})\)\. What it answers: (.*)_\s*$/s);
  if (!m) throw new Error(`${file}: its header does not read as a saved message's`);
  const [, date, time, uuid, bytes, digest, answers] = m;
  if (Buffer.byteLength(text) !== Number(bytes) || sha(text) !== digest) throw new Error(`${file}: the message is not the ${bytes} bytes with sha256 ${digest} its header gives`);
  // What the register quotes: the whole message, or, for the Export's message, his own lines before the Export.
  let shown = text;
  if (file === EXPORT) {
    const at = text.indexOf('\n{\n');
    if (at < 0) throw new Error(`${file}: the Export's first line was not found`);
    shown = text.slice(0, at).replace(/\s+$/, '');
  }
  return { date, time, uuid, bytes, digest, answers: answers.trim().replace(/\.$/, ''), text, shown };
}
function wrap(paragraph, indent = '  ', width = 118) {
  const lines = [];
  let line = '';
  for (const word of paragraph.split(' ')) {
    const next = line === '' ? word : `${line} ${word}`;
    if (next.length > width - (lines.length === 0 ? 0 : indent.length) && line !== '') { lines.push(line); line = word; } else line = next;
  }
  lines.push(line);
  return lines.map((l, i) => (i === 0 ? l : `${indent}${l}`)).join('\n');
}
const quoted = (text) => text.split('\n').map((l) => (l === '' ? '>' : `> ${l}`)).join('\n');
// How an entry's bracket ends, right before his quoted words: found again by `check`.
const closing = (s, file) => (file === EXPORT
  ? `Copied by script (his own lines before the Export, ${Buffer.byteLength(s.shown)} bytes of the message's ${s.bytes}, whose sha256 is ${s.digest}):`
  : `Copied by script (${s.bytes} bytes, sha256 ${s.digest}):`);
function entry(file, summary, commit) {
  const s = saved(file);
  const label = s.text.split('\n')[0];
  const extra = file === EXPORT ? EXPORT_NOTE : '';
  const head = `- **Taken by Mustafa, ${s.date}, ${s.time} UTC: ${summary}** (${CHAT}; his label: "${label}"; message ${s.uuid}, whole; copied by script from the chat's saved file \`${NOTES}${file}\` (readable at commit ${commit} of the chat's branch \`claude/cool-pasteur-oiqk1r\`), which the chat copied by script from its transcript). What it answers, in the chat's words (not his): ${s.answers}.${extra} ${closing(s, file)}`;
  return { ...s, block: `${wrap(head)}\n${quoted(s.shown)}\n` };
}

const [mode, commit] = process.argv.slice(2);
if (mode === 'write') {
  if (!/^[0-9a-f]{40}$/.test(commit ?? '')) throw new Error('write needs the full commit id at which the saved files are readable');
  let text = readFileSync(DECISIONS, 'utf8');
  for (const [file, summary] of ENTRIES) {
    const e = entry(file, summary, commit);
    if (text.includes(e.uuid)) throw new Error(`${file} is already in ${DECISIONS}`);
    // Newest first: before the first entry of the list that is older than this one.
    const lines = text.split('\n');
    const from = lines.findIndex((l) => l === '## Decisions, newest first') + 1;
    let at = lines.findIndex((l, i) => i >= from && l.startsWith('## '));
    for (let i = from; i < at; i += 1) {
      const m = lines[i].match(/^- \*\*Taken by [^,]+, (\d{4}-\d{2}-\d{2})(?:, (\d{2}:\d{2}:\d{2}) UTC)?/);
      if (m && `${m[1]} ${m[2] ?? '00:00:00'}` < `${e.date} ${e.time}`) { at = i; break; }
    }
    lines.splice(at, 0, ...e.block.replace(/\n$/, '').split('\n'));
    text = lines.join('\n');
    console.log(`${file}: ${e.date} ${e.time} UTC, before line ${at + 1}`);
  }
  writeFileSync(DECISIONS, text);
} else if (mode === 'check') {
  const text = readFileSync(DECISIONS, 'utf8');
  for (const [file] of ENTRIES) {
    const s = saved(file);
    // The entry's bracket is wrapped, so it is found by the message's id, and its end by the digest that closes it.
    const at = text.indexOf(s.uuid);
    const block = quoted(s.shown) + '\n';
    const end = `${s.digest}):\n`;
    const start = text.indexOf(end, at);
    if (at < 0 || start < 0 || text.slice(start + end.length).slice(0, block.length) !== block) throw new Error(`${file}: its words in ${DECISIONS} are not the saved message's, byte for byte`);
    // And the quoted block ends there: the next line is no line of a quote.
    if (text.slice(start + end.length + block.length).startsWith('>')) throw new Error(`${file}: more quoted lines follow its words in ${DECISIONS}`);
    console.log(`${file}: ${Buffer.byteLength(s.shown)} of ${s.bytes} bytes quoted, sha256 ${s.digest.slice(0, 12)}..., in ${DECISIONS} word for word`);
  }
} else throw new Error('usage: decisions-take-in-76.mjs write <commit> | check');
