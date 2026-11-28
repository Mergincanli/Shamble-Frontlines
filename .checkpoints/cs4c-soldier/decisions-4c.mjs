// Writes Mustafa's words to lane 1's builder chat at 4b-2's close into docs/DECISIONS.md, at 4c's first docs commit
// (his message of 2026-11-27, 23:59 Amsterdam, item 4, and his design notes' item 6), from the chat's saved message
// files (.checkpoints/cs4-walk/mustafa-*.md, each copied by script from the chat's transcript): no word of his is
// typed here. 4b-2's script (decisions-4b2.mjs, readable at 59f6cc7), with these messages. From the worktree's root:
//   node .checkpoints/cs4c-soldier/decisions-4c.mjs write <commit at which the saved files are readable>
//   node .checkpoints/cs4c-soldier/decisions-4c.mjs check
// `write` refuses a message that is already in the file (STEP12-GO is there since 4b-2's docs pass). `check` reads the
// file back and compares every quoted line with the saved message, byte for byte, and each saved message with the
// byte count and sha256 its own header gives. The text in bold and the brackets after it are this chat's, written
// for the file. His message of 23:59 Amsterdam joins ENTRIES only as he answers BOARD-WORDS: under (a) saved whole
// (chat-words.mjs `words`); under (b) saved by chat-words.mjs `less` with its item 3 left out (the line that starts
// "3. ", left out because it names his board), which `saved` below reads; under (c) not at all.
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

const NOTES = '.checkpoints/cs4-walk/';
const DECISIONS = 'docs/DECISIONS.md';
const BRANCH = 'claude/ecstatic-wozniak-5cbdzs';
const V6 = 'the builder chat he calls Builder V6, lane 1 of the build, session_01WLuqnt1v3UNwSqbJsJre7u, a Claude Code chat on his second personal account';
// The file, the chat it was sent to, and what he decided in it, in this chat's words (the bold line of the entry).
const ENTRIES = [
  ['mustafa-2026-11-27-go-merge-79.md', V6,
    'his go for the merge of change-set 4b-2, "The world", at head b7145273fdaff2b2ee6eccafaf5e358840bc3572 (his Advisor then opened it as pull request #79 and merged it by merge commit at that head)'],
  ['mustafa-2026-11-27-may-merge.md', V6,
    'his word that the chat may merge 4b-2 itself; the chat could not, its GitHub access being read-only, and his Advisor merged it'],
  ['mustafa-2026-11-27-design-notes.md', V6,
    'his design direction for the economy and base repair: resource points and repair bots, with decided and open lines and every number a placeholder until playtests, nothing of it built in Slice 1; and where it goes in the docs (LOOP_V2\'s new section 14 with his text, dated pointers at its lines on the open fields, one sentence in its header, one line in the ADR register)'],
];

const sha = (text) => createHash('sha256').update(text).digest('hex');
// A saved message is whole, or (chat-words.mjs `less`, for BOARD-WORDS (b)) shown without one line: then the header
// gives the left-out line's number, why it is left out, and the shown text's bytes and sha256, which is what is quoted
// and checked here. The whole message's bytes and sha256 are in no file of the repository (a digest and a length of
// the whole beside the shown text would let a reader confirm a guess of the left-out line): the chat gives them to
// him, and chat-words.mjs `less` run again into a scratch file checks the saved file against the transcript.
function saved(file) {
  const all = readFileSync(NOTES + file, 'utf8');
  const cut = all.indexOf('\n---\n');
  const note = all.slice(0, cut);
  const text = all.slice(cut + 5).replace(/\n$/, '');
  const m = note.match(/(?:the user message|the message he sent while the chat was working \(stored as a queued command\)) of (\d{4}-\d{2}-\d{2})T(\d{2}:\d{2}:\d{2})\.\d+Z \(uuid ([0-9a-f-]{36})\), (?:whole|shown here without its line (\d+) of (\d+), which is left out because (.*?)) \((\d+) bytes, sha256 ([0-9a-f]{64})\)\. What it answers: (.*)_\s*$/s);
  if (!m) throw new Error(`${file}: its header does not read as a saved message's`);
  const [, date, time, uuid, line, of, why, bytes, digest, answers] = m;
  const left = line === undefined ? null : { line, of, why };
  if (Buffer.byteLength(text) !== Number(bytes) || sha(text) !== digest) throw new Error(`${file}: the ${left ? 'shown text' : 'message'} is not the ${bytes} bytes with sha256 ${digest} its header gives`);
  if (left && text.split('\n').length !== Number(of) - 1) throw new Error(`${file}: the shown text is not the message less one line`);
  return { date, time, uuid, bytes, digest, left, answers: answers.trim().replace(/\.$/, ''), text, shown: text };
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
const closing = (s) => `Copied by script (${s.bytes} bytes, sha256 ${s.digest}):`;
function entry(file, chat, summary, commit) {
  const s = saved(file);
  const how = s.left
    ? `without its line ${s.left.line} of ${s.left.of}, left out because ${s.left.why} (the whole message is in the chat's transcript, by this id)`
    : 'whole';
  const head = `- **Taken by Mustafa, ${s.date}, ${s.time} UTC: ${summary}** (${chat}; message ${s.uuid}, ${how}; copied by script from the chat's saved file \`${NOTES}${file}\` (readable at commit ${commit} of the branch \`${BRANCH}\`), which the chat copied by script from its transcript). What it answers, in the chat's words (not his): ${s.answers}. ${closing(s)}`;
  return { ...s, block: `${wrap(head)}\n${quoted(s.shown)}\n` };
}

const [mode, commit] = process.argv.slice(2);
if (mode === 'write') {
  if (!/^[0-9a-f]{40}$/.test(commit ?? '')) throw new Error('write needs the full commit id at which the saved files are readable');
  let text = readFileSync(DECISIONS, 'utf8');
  for (const [file, chat, summary] of ENTRIES) {
    const e = entry(file, chat, summary, commit);
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
    const less = s.left ? ` (the message less its line ${s.left.line} of ${s.left.of})` : '';
    console.log(`${file}: ${Buffer.byteLength(s.shown)} of ${s.bytes} bytes quoted, sha256 ${s.digest.slice(0, 12)}...${less}, in ${DECISIONS} word for word`);
  }
} else throw new Error('usage: decisions-4c.mjs write <commit> | check');
