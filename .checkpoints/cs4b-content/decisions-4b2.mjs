// Writes Mustafa's words to lane 1's builder chats during change-set 4b-2 into docs/DECISIONS.md, from the chats'
// saved message files (.checkpoints/cs4-walk/mustafa-*.md, each copied by script from its chat's transcript): no word
// of his is typed here. 4b-1's script (decisions-4b1.mjs, beside this one), with 4b-2's messages and two chats. From
// the worktree's root:
//   node .checkpoints/cs4b-content/decisions-4b2.mjs write <commit at which the saved files are readable>
//   node .checkpoints/cs4b-content/decisions-4b2.mjs check
// `write` refuses a message that is already in the file. `check` reads the file back and compares every quoted line
// with the saved message, byte for byte, and each saved message with the byte count and sha256 its own header gives.
// The text in bold and the brackets after it are this chat's, written for the file.
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

const NOTES = '.checkpoints/cs4-walk/';
const DECISIONS = 'docs/DECISIONS.md';
const BRANCH = 'claude/ecstatic-wozniak-5cbdzs';
const V4 = 'the builder chat he calls Builder V4, lane 1 of the build, session_019YNMUVZR6ociHFWjByxkrq, a Claude Code chat';
const V6 = 'the builder chat he calls Builder V6, lane 1 of the build, session_01WLuqnt1v3UNwSqbJsJre7u, a Claude Code chat on his second personal account';
// The file, the chat it was sent to, and what he decided in it, in this chat's words (the bold line of the entry).
const ENTRIES = [
  ['mustafa-2026-11-26-go-merge-75.md', V4,
    'his go for the merge of pull request #75 (change-set 4b-1, "Content") at head 2542b1a0c109e54c514cf63d1c2e2de8932a1a7b'],
  ['mustafa-2026-11-26-grey-colours.md', V4,
    'GREY-COLOURS (a): the default greys of the first scene stand (he gives no other colours)'],
  ['mustafa-2026-11-27-save-every-25-minutes.md', V4,
    'a standing instruction: the chat\'s own work and its agents\' work are committed and pushed at least every 25 minutes, so that his other account can continue it; the work is not cut to save usage'],
  ['mustafa-2026-11-27-builder-v6.md', V6,
    'the seat of change-set 4b-2 moves to Builder V6, on his second personal account (all three of his Claude accounts are his and count the same); the guard check while he watches; the work from `claude/cool-pasteur-oiqk1r` at 7f5ba3966def8c36e2c3863c6b2fda7b4b9a9169 on a branch of the chat\'s own, Builder V4 pushing nothing more there; DIFF-LIST (a): the list as printed from his Export at 30 steps is his, its movement values become the game\'s starting movement table, and a later Export replaces them the same way; CI-TIME: main taken in once after #78, never a test cut or thinned to fit the limit; VISION-RULE, an addition that sharpens point 2c of version 2 (sight starts at the soldier\'s eyes at the height of the face; an enemy is seen only when a straight line from those eyes to the enemy\'s body is free, height counting at both ends; what is seen follows where the eyes are at that moment, a jump over a hedge included), with the exact eye height and the body points the line is tested against left open, and no code now; saving at least every 25 minutes, the agents\' work too; a merge only on his go for the exact head after the advisor agent approved it, his Advisor opening and merging the pull request when the chat cannot; git and gh always start their own command, and every file a shell command writes is named by its full path'],
  ['mustafa-2026-11-27-guard-retry.md', V6,
    'GUARD-RETRY (a): the guard check run once more while he watched, and he denied it; then the takeover and the first report'],
  ['mustafa-2026-11-27-step12-go.md', V6,
    'STEP12-GO (a): the page\'s scene (4b-2\'s step 12), with the map\'s reading field by field, moves to the start of change-set 4c, and 4b-2 closes with its steps 6 to 11'],
];

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
  return { date, time, uuid, bytes, digest, answers: answers.trim().replace(/\.$/, ''), text, shown: text };
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
  const head = `- **Taken by Mustafa, ${s.date}, ${s.time} UTC: ${summary}** (${chat}; message ${s.uuid}, whole; copied by script from the chat's saved file \`${NOTES}${file}\` (readable at commit ${commit} of the branch \`${BRANCH}\`), which the chat copied by script from its transcript). What it answers, in the chat's words (not his): ${s.answers}. ${closing(s)}`;
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
    console.log(`${file}: ${Buffer.byteLength(s.shown)} of ${s.bytes} bytes quoted, sha256 ${s.digest.slice(0, 12)}..., in ${DECISIONS} word for word`);
  }
} else throw new Error('usage: decisions-4b2.mjs write <commit> | check');
