// Writes Mustafa's words of 2026-11-26 to Builder V4 into docs/DECISIONS.md, from the chat's saved message files
// (.checkpoints/cs4-walk/mustafa-2026-11-26-*.md, each copied by script from the chat's transcript): no word of his
// is typed here. 4a's script (decisions-4a.mjs, in the notes at c17a054), with the day's messages. From the
// worktree's root:
//   node .checkpoints/cs4b-content/decisions-4b1.mjs write <commit at which the saved files are readable>
//   node .checkpoints/cs4b-content/decisions-4b1.mjs check
// `write` refuses a message that is already in the file. `check` reads the file back and compares every quoted line
// with the saved message, byte for byte, and each saved message with the byte count and sha256 its own header gives.
// The text in bold and the brackets after it are this chat's, written for the file.
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

const NOTES = '.checkpoints/cs4-walk/';
const DECISIONS = 'docs/DECISIONS.md';
const CHAT = 'the builder chat he calls Builder V4, lane 1 of the build, session_019YNMUVZR6ociHFWjByxkrq, a Claude Code chat';
// The file, and what he decided in it, in this chat's words (the bold line of the entry).
const ENTRIES = [
  ['mustafa-2026-11-26-go-merge-70.md',
    'his go for the merge of pull request #70 (change-set 4a) at head 420b16b35225c41f1978b177822f2e24386a2f95, by a merge commit with that id as the expected head; BOARD-SLIP (a): the history stays as it is; CONTENT-SOURCE (c): the content reader is all our own, kept to what the content files need, everything else refused, duplicate keys included, and fuzzed before the first value rests on it; DEPART-4 (a): all fourteen departures stand, with two notes (the 48 is one named constant; until the aim codec widens, the loader refuses a playable map larger than its reach, and STATUS names the change-set that widens it); EARLY-LOOK-ORDER (b): inside 4c only; FIREFOX-REPLAY (a): the CI Builder gets it from him, and a red comes to him first; LINKS-CHECK (c): about 15 lines in 4b; one question to the chat (one repository only, and has a guard prompt ever reached him from it); 4b starts after the advisor agent\'s re-read, and his Export at 30 steps comes before 4b\'s step 6'],
  ['mustafa-2026-11-26-source-go.md',
    'SOURCE-GO (a): the rule that a change under content/source/ needs his go is written into the review-tracks rule change the Guard fix chat has queued; it is no condition for 4b\'s merge and this chat writes no kit file for it; until that rule is merged, main\'s rule that game feel and balance go to him covers a change to his values, every pull request that changes anything under content/source/ says so in the first lines of its report, and his Advisor\'s merge check lists that folder; the Export and the reader: he agrees with the chat\'s proposal if the advisor agent approves it (a tool script reads his Export with the platform\'s own JSON reader, the game\'s reader stays as narrow as proposed, the Rust tool proves the written content file canonical, no new package comes in, the game never loads the Export itself); security-auditor\'s twelve points go into the test list before step 1; his Export at 30 steps comes before step 6'],
  ['mustafa-2026-11-26-size-4b.md',
    'SIZE-4B (b), in engine-architect\'s cut: change-set 4b is two pull requests and the collision library goes wholly into 4b-2; stop lines 7,450 for 4b-1 and 8,450 for 4b-2, each on its own; before step 1 of each half the checkpoint names what leaves first, and he gets one sentence at two thirds of the line; 4b-1 starts at once, with no outside package, no wait for his Export and no browser check; FEATURE-COMPARE (a): two named exceptions, each pinned to exact entries and versions, with tests and planted mistakes, under four conditions, recorded as a dated pointer for ADR 0002\'s next revision; 4b-1\'s report says in its first lines what moved 4b from 5,410 to about 7,050 lines and gives a new estimate for the first build he can move a soldier in; his Export at 30 steps comes before the data step (step 8), in 4b-2'],
  ['mustafa-2026-11-26-export-30.md',
    'his Testlines Export at 30 steps, handed over for the data step of 4b-2'],
  ['mustafa-2026-11-26-vision-rule-v2.md',
    'VISION-RULE, version 2: a player sees an enemy only when their own soldier has a clear line of sight to that enemy (the character\'s sight, not the camera\'s view): his answer to the open design questions R2-INT-OQ1 and R4-LOS-OQ2; the rule in ten points (a white vision cone from the soldier, pointing where the soldier looks; its width starts at 190 degrees and is tuned later; an enemy is drawn only while some part of their body is inside the cone with nothing solid in between, in the open behind the player\'s back too; no maximum reach; the player\'s own eyes only; what blocks sight: walls, buildings, static objects, vehicles, hedges and smoke, never soldiers, and not the player\'s own vehicle while inside it; an enemy that leaves sight is gone at once; projectiles and their traces are always drawn; sounds are not hidden; the ground, walls and buildings always stay visible); left open by him: the cone\'s final width, how the cone is drawn, whether fences block sight, whether shots are blocked by the same things as sight; no code for it now and not part of Slice 1 (the sight check with Slice 2\'s bots, the hiding over the network in Slice 3; the cone tried in Testlines first); recorded at 4b-1\'s docs pass with pointers in docs/adr/README.md and one follow-up line in STATUS, no ADR text rewritten; one question to the chat (a "blocks sight" flag in 4b), not to be built without his word; the chat\'s main folder brought up to main by a fast-forward'],
];
// The Export's own text is not repeated in the register: his words before it are quoted, and the entry says where
// the Export is, whole.
const EXPORT = 'mustafa-2026-11-26-export-30.md';
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
} else throw new Error('usage: decisions-4b1.mjs write <commit> | check');
